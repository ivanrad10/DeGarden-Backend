use std::{path::PathBuf, process::Command};

use axum::{
    body::StreamBody,
    http::{header, HeaderValue, StatusCode},
    response::IntoResponse,
};
use tokio::fs::File;
use tokio_util::io::ReaderStream;

pub async fn moisture(
    board: String,
    ssid: String,
    password: String,
    key: String,
) -> impl IntoResponse {
    let project_dir = "/home/alex/Documents/work/DeGarden-Firmware-Moisture/";
    build_and_stream_firmware(project_dir, &board, &ssid, &password, &key).await
}

pub async fn flowmeter(
    board: String,
    ssid: String,
    password: String,
    key: String,
) -> impl IntoResponse {
    let project_dir = "/home/alex/Documents/work/DeGarden-Firmware-Flowmeter/";
    build_and_stream_firmware(project_dir, &board, &ssid, &password, &key).await
}

async fn build_and_stream_firmware(
    project_dir: &str,
    board: &str,
    ssid: &str,
    password: &str,
    key: &str,
) -> impl IntoResponse {
    let mut command = Command::new("cargo");
    command
        .arg("build")
        .arg("--release")
        .env("DEVICE_ID", key)
        .env("WIFI_SSID", ssid)
        .env("WIFI_PASSWORD", password)
        .current_dir(project_dir);

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

    let firmware_path: PathBuf = format!(
        "{}/target/riscv32imc-unknown-none-elf/release/firmware",
        project_dir
    )
    .into();

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
