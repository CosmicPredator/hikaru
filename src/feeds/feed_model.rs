use serde::Deserialize;
use std::{path::PathBuf, str::FromStr};

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
    pub title: String,
    pub link: String,
    pub lintype: String,
    pub size: String,
    #[serde(rename = "pubDate")]
    pub pub_date: String,
    #[serde(rename = "infohash")]
    pub info_hash: String,
}

#[derive(Debug)]
pub struct RssFeedParseError(String);

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
