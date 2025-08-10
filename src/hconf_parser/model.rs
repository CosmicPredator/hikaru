use std::{collections::HashMap, path::PathBuf};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TorrentClient {
    pub id: String,
    #[serde(rename = "type")]
    pub client_type: String,
    pub url: String,
    pub username: String,
    pub password: String
}

#[derive(Debug, Deserialize)]
pub struct Task {
    pub schedule: String,
    pub max_retries: u32,
    pub rss_feed: RssFeed,
    #[serde(rename = "action")]
    pub actions: HashMap<String, FeedAction>
}

#[derive(Debug, Deserialize)]
pub struct RssFeed {
    pub url: String,
    pub indexer: Option<String>,
    pub filter: Option<Filter>
}

#[derive(Debug, Deserialize)]
pub struct Filter {
    pub include: Vec<String>,
    pub exclude: Vec<String>,
    pub min_size: String,
    pub max_size: String
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum FeedAction {
    #[serde(rename = "exec")]
    Exec {
        command: String,
        args: Vec<String>,
        output_as: String
    },
    #[serde(rename = "download")]
    Download {
        client: String,
        save_path: String
    }
}

#[derive(Debug, Deserialize)]
pub struct HConf {
    pub torrent_client: TorrentClient,
    pub task: HashMap<String, Task>
}

#[derive(Debug)]
pub enum HConfParseError {
    FileReadError(String),
    ParseError(String)
}

impl From<std::io::Error> for HConfParseError {
    fn from(value: std::io::Error) -> Self {
        HConfParseError::FileReadError(
            format!("unable to read hconf file: {}", value)
        )
    }
}

impl From<hcl::Error> for HConfParseError {
    fn from(value: hcl::Error) -> Self {
        HConfParseError::ParseError(
            format!("unable to parse hconf: {}", value)
        )
    }
}

impl TryFrom<PathBuf> for HConf {
    type Error = HConfParseError;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let fs = std::fs::read_to_string(value)?;
        let parsed = hcl::from_str::<HConf>(&fs)?;
        Ok(parsed)
    }
}