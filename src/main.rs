use std::io::Error;

use glowing_telegram::run_server;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run_server().await
}
