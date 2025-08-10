use anyhow::anyhow;
use log::info;

mod helpers;
mod clients;
mod task;
mod feeds;
mod filters;
mod actions;
mod hconf_parser;

type AnyErr = anyhow::Error;

fn init_logger() -> Result<(), AnyErr> {
    simple_logger::SimpleLogger::new()
        .with_colors(true)
        .with_level(log::LevelFilter::Info)
        .without_timestamps()
        .init()
        .map_err(|_| anyhow!("failed to initialize logger"))
}

#[tokio::main]
async fn main() -> Result<(), AnyErr> {
    init_logger()?;
    info!("starting application");
    Ok(())
}
