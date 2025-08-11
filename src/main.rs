use std::path::PathBuf;

use anyhow::anyhow;

use crate::hconf_parser::{model::HConf};

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
        .with_level(log::LevelFilter::Debug)
        .without_timestamps()
        .init()
        .map_err(|_| anyhow!("failed to initialize logger"))
}

#[tokio::main]
async fn main() -> Result<(), AnyErr> {
    init_logger()?;

    // let validator = HConfValidator::try_from(PathBuf::from_str("tasks.hconf").unwrap())?;
    // validator.validate();
    let hconf = HConf::try_from(PathBuf::from("tasks.hconf"))
        .map_err(|err| anyhow!("{err}"))?;
    println!("{hconf:#?}");

    Ok(())
}
