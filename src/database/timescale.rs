use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_postgres::{Client, NoTls};

pub async fn connect() -> Arc<Mutex<Client>> {
    // Extract credentials
    dotenv().ok();
    let db_host = env::var("DB_HOST").expect("DB_HOST not set in .env");
    let db_port = env::var("DB_PORT").expect("DB_PORT not set in .env");
    let db_user = env::var("DB_USER").expect("DB_USER not set in .env");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD not set in .env");
    let db_name = env::var("DB_NAME").expect("DB_NAME not set in .env");

    // Connect to the database
    let (client, connection) = tokio_postgres::connect(
        &format!(
            "host={} port={} user={} password={} dbname={}",
            db_host, db_port, db_user, db_password, db_name
        ),
        NoTls,
    )
    .await
    .expect("Failed to connect to TimescaleDB");

    // Run the connection in a background task
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    // Return the client wrapped in Arc<Mutex> for thread-safety
    Arc::new(Mutex::new(client))
}
