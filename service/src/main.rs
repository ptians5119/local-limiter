#[macro_use]
extern crate serde;

mod config;
pub(crate) mod arg;
mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::parse().await?;
    service::grpc_run(&config).await
}
