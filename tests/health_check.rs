use std::net::{SocketAddr, TcpListener};

use anyhow::{Error, Result};
use glowing_telegram::run_server;
use reqwest::Client;

#[tokio::test]
async fn health_check_works() -> Result<(), Error> {
    let base_address = run_application()
        .await
        .expect("Unable to start application");
    let client = Client::new();
    let endpoint = format!("{}/health_check", base_address);
    let response = client
        .get(endpoint)
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());
    assert!(
        response
            .text()
            .await
            .is_ok_and(|text| "Healthy".eq(text.as_str()))
    );

    Ok(())
}

async fn run_application() -> Result<String, Error> {
    let address = SocketAddr::from(([127, 0, 0, 1], 0));
    let listener = TcpListener::bind(address)?;
    let port = listener.local_addr().unwrap().port();
    let _ = tokio::spawn(run_server(listener));
    Ok(format!("http://127.0.0.1:{}", port))
}
