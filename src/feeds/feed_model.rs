use serde::Deserialize;
use std::{fmt::Display, path::PathBuf, str::FromStr};

#[derive(Debug, Deserialize)]
#[serde(rename = "rss")]
pub struct RssFeedModel {
    pub channel: Channel,
}

#[derive(Debug, Deserialize)]
pub struct Channel {
    #[serde(rename = "item")]
    pub items: Vec<Item>,
}

#[derive(Debug, Deserialize)]
pub struct Item {
    pub title: Option<String>,
    pub link: Option<String>,
    pub linktype: Option<String>,
    pub size: Option<String>,
    #[serde(rename = "pubDate")]
    pub pub_date: Option<String>,
    #[serde(rename = "infohash")]
    pub info_hash: Option<String>,
}

#[derive(Debug)]
pub struct RssFeedParseError(String);

impl Display for RssFeedParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<quick_xml::DeError> for RssFeedParseError {
    fn from(value: quick_xml::DeError) -> Self {
        RssFeedParseError(format!("xml deseralization error: {}", value))
    }
}

impl FromStr for RssFeedModel {
    type Err = RssFeedParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rss = quick_xml::de::from_str::<Self>(s)?;
        Ok(rss)
    }
}

impl TryFrom<PathBuf> for RssFeedModel {
    type Error = RssFeedParseError;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let fs = std::fs::read_to_string(value)
            .map_err(|_| RssFeedParseError("file not found".into()))?;
        let rss = quick_xml::de::from_str::<Self>(&fs)?;
        Ok(rss)
    }
}

impl RssFeedModel {
    pub async fn from_url(feed_url: &str, http_client: reqwest::Client,) -> Result<Self, RssFeedParseError> {
        let request = http_client
            .get(feed_url)
            .header("User-Agent", "hikaru-client")
            .header("Accept", "*/*")
            .send()
            .await
            .map_err(|_| RssFeedParseError("unable to fetch rss feed from url".into()))?;
        let response_text = request
            .text()
            .await
            .map_err(|_| RssFeedParseError("invalid feed content".into()))?;
        let rss = quick_xml::de::from_str::<RssFeedModel>(&response_text)?;
        Ok(rss)
    }

    pub fn get_items(&self) -> &Vec<Item> {
        &self.channel.items
    }
}
