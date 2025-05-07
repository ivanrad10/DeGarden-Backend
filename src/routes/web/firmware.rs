use axum::{
    body::StreamBody,
    http::{header, HeaderValue, StatusCode},
    response::IntoResponse,
};
use std::{env, path::PathBuf, process::Command};
use tokio::fs::File;
use tokio_util::io::ReaderStream;

use super::types::FirmwareRequest;

pub async fn moisture(payload: FirmwareRequest) -> impl IntoResponse {
    let base_dir = env::var("BASE_DIR_MOISTURE").expect("BASE_DIR_MOISTURE not set in .env file");

    build_and_stream_firmware(&base_dir, &payload).await
}

pub async fn flowmeter(payload: FirmwareRequest) -> impl IntoResponse {
    let base_dir = env::var("BASE_DIR_FLOWMETER").expect("BASE_DIR_FLOWMETER not set in .env file");

    build_and_stream_firmware(&base_dir, &payload).await
}

async fn build_and_stream_firmware(base_dir: &str, payload: &FirmwareRequest) -> impl IntoResponse {
    let board = payload.board.as_str();
    let ssid = payload.ssid.as_str();
    let password = payload.password.as_str();
    let key = payload.key.as_str();

    let supported_boards = env::var("SUPPORTED_BOARDS_MOISTURE")
        .expect("SUPPORTED_BOARDS_MOISTURE not set in .env file");

    let supported: Vec<&str> = supported_boards.split(',').map(str::trim).collect();

    match supported.iter().any(|&b| b == board) {
        true => {}
        false => {
            return (
                StatusCode::BAD_REQUEST,
                format!("Unsupported board: {}", board),
            )
                .into_response()
        }
    };

    let project_dir = format!("{}/{}", base_dir, board);

    let mut command = Command::new("cargo");
    command
        .arg("build")
        .arg("--release")
        .env("DEVICE_ID", key)
        .env("WIFI_SSID", ssid)
        .env("WIFI_PASSWORD", password)
        .current_dir(&project_dir);

    let output = command.output();

    match output {
        Ok(output) if output.status.success() => {
            // proceed to stream the firmware file
        }
        Ok(output) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Build failed:\n{}", String::from_utf8_lossy(&output.stderr)),
            )
                .into_response();
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to run build command: {}", e),
            )
                .into_response();
        }
    }

    let firmware_path: PathBuf = match board {
        "esp32c6" => format!(
            "{}/target/riscv32imc-unknown-none-elf/release/firmware",
            &project_dir
        )
        .into(),
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                format!("Unsupported board: {}", board),
            )
                .into_response();
        }
    };

    match File::open(&firmware_path).await {
        Ok(file) => {
            let stream = ReaderStream::new(file);
            let body = StreamBody::new(stream);

            let content_disposition =
                HeaderValue::from_str(&format!("attachment; filename=\"{}.bin\"", board))
                    .unwrap_or_else(|_| HeaderValue::from_static("attachment"));

            (
                [
                    (
                        header::CONTENT_TYPE,
                        HeaderValue::from_static("application/octet-stream"),
                    ),
                    (header::CONTENT_DISPOSITION, content_disposition),
                ],
                body,
            )
                .into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Firmware file not found: {}", e),
        )
            .into_response(),
    }
}
