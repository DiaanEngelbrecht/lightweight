use cali_derive::setup_server;
use cali_core::config::CaliConfig;
use lightweight_web::config::Config;
use std::{error::Error, str::FromStr};

#[derive(Debug, Clone)]
pub struct ServerContext {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let my_context = ServerContext {};

    let server_config = CaliConfig::new()
        .enable_database()
        .add_global_context(my_context)
        .add_middleware(move |server| {
            server
        });
    setup_server!("lightweight", "0.1.1", server_config);
    Ok(())
}
