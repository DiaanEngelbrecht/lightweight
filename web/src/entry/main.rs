use lightweight_web::config::Config;
use flair_derive::setup_server;
use std::{error::Error, str::FromStr};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    setup_server!("lightweight", "0.1.1");
    Ok(())
}
