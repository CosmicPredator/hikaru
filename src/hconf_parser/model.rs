use std::{collections::HashMap, fmt::{self, Display}, path::PathBuf};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ClientType {
    QBittorrent,
    Deluge,
    RQBit
}

impl fmt::Display for ClientType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ClientType::QBittorrent => "qbittorrent",
            ClientType::RQBit => "rqbit",
            ClientType::Deluge => "deluge",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Deserialize)]
pub struct TorrentClient {
    pub id: String,
    #[serde(rename = "type")]
    pub client_type: ClientType,
    pub url: String,
    pub username: String,
    pub password: String
}

#[derive(Debug, Deserialize, Clone)]
pub struct Task {
    pub schedule: String,
    pub max_retries: u32,
    pub rss_feed: RssFeed,
    #[serde(rename = "action")]
    pub actions: HashMap<String, FeedAction>
}

#[derive(Debug, Deserialize, Clone)]
pub struct RssFeed {
    pub url: String,
    pub indexer: Option<String>,
    pub filter: Option<Filter>
}

#[derive(Debug, Deserialize, Clone)]
pub struct Filter {
    pub include: Vec<String>,
    pub exclude: Vec<String>,
    pub min_size: String,
    pub max_size: String
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum FeedAction {
    #[serde(rename = "exec")]
    Exec {
        command: String,
        args: Vec<String>
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

    #[serde(rename = "task")]
    pub tasks: HashMap<String, Task>
}

#[derive(Debug)]
pub enum HConfParseError {
    FileReadError(String),
    ParseError(String)
}

impl Display for HConfParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            HConfParseError::FileReadError(msg) => write!(f, "{}", msg),
            HConfParseError::ParseError(msg) => write!(f, "{}", msg)
        }
    }
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