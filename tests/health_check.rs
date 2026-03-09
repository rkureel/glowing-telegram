use anyhow::{Error, Result};
use glowing_telegram::run_server;
use reqwest::Client;

#[tokio::test]
async fn health_check_works() -> Result<(), Error> {
    run_application()
        .await
        .expect("Unable to start application");
    let client = Client::new();
    let response = client
        .get("http://localhost:8000/health_check")
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

async fn run_application() -> Result<(), Error> {
    let _ = tokio::spawn(run_server());
    Ok(())
}
