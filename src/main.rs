use std::path::PathBuf;

use anyhow::anyhow;

use crate::{hconf_parser::model::HConf, task::task_pipeline::TaskPipeline};

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
    let hconf = HConf::try_from(PathBuf::from("tasks.hconf"))
        .map_err(|_| anyhow!("error"))?;
    let pipeline = TaskPipeline::try_from(hconf)
        .map_err(|_|anyhow!("error"))?;
    pipeline.run()
        .await
        .map_err(|_|anyhow!("error"))?;
    Ok(())
}
