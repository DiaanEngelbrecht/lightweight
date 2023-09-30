use flair_derive::setup_server;
use lightweight_web::config::Config;
use std::{error::Error, str::FromStr, sync::Arc};

#[derive(Debug, Clone)]
pub struct ServerContext {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let my_context = Arc::new(ServerContext {});
    setup_server!("lightweight", "0.1.1", my_context);
    Ok(())
}
