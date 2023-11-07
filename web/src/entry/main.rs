use cali_core::config::CaliConfig;
use cali_derive::setup_server;
use lightweight_web::config::Config;
use std::{error::Error, str::FromStr, sync::Arc};

#[derive(Debug, Clone)]
pub struct ServerContext {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server_config = CaliConfig::new(Arc::new(ServerContext {}), true, |s| s);
    setup_server!("lightweight", "0.1.1", server_config);
    Ok(())
}
