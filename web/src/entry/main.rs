use cali_core::config::CaliConfig;
use cali_derive::setup_server;
use lightweight_web::config::Config;
use std::time::Duration;
use std::{error::Error, str::FromStr, sync::Arc};
use tower::timeout::TimeoutLayer;

#[derive(Debug, Clone)]
pub struct ServerContext {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server_config = CaliConfig::new(Arc::new(ServerContext {}), true, |s| {
        s.layer(TimeoutLayer::new(Duration::from_secs(30)))
    });
    setup_server!("lightweight", "0.1.1", server_config);
    Ok(())
}
