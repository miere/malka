use crate::conf::SubscriptionConfig;
use crate::manager::SubscriptionManager;
use std::{env, fs};
use std::env::Args;

mod error;
mod kafka;
mod aws;
mod conf;
pub mod manager;

#[tokio::main]
async fn main() -> error::Result<()> {
    let args = env::args();

    return match args.len() {
        0 | 1 => Err(error::KnownHandledErrors::InvalidParameters),
        _ => run_consumer(args).await
    }
}

async fn run_consumer(args: Args) -> error::Result<()> {
    env_logger::init();

    let mut manager = SubscriptionManager::default();

    args.skip(1)
        .map(|file_name| fs::read_to_string(file_name).unwrap())
        .flat_map(|file_content| {
            println!("file_content: {}", &file_content);
            serde_json::from_str::<Vec<SubscriptionConfig>>(&file_content).unwrap()
        })
        .for_each(|subscription| manager.subscribe(subscription).unwrap());

    manager.await_termination().await?;

    Ok(())
}