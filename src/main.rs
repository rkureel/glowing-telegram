use std::net::{SocketAddr, TcpListener};

use anyhow::{Error, Result};
use glowing_telegram::run_server;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let listener = TcpListener::bind(addr)?;

    run_server(listener).await?;
    Ok(())
}
