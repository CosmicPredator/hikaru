use log::{error, info};
use std::{collections::HashMap, path::Path};

use crate::clients::TorrentClient;

pub enum QBittorrentError {
    LoginFailedError,
    DownloadQueueError(String),
}

pub struct QBittorrent {
    id: String,
    client_type: String,
    url: String,
    username: String,
    password: String,
    http_client: reqwest::Client,
}

impl QBittorrent {
    const QBIT_LOGIN_EP: &str = "/api/v2/auth/login";
    const QBIT_ADD_TORR_EP: &str = "/api/v2/torrents/add";

    pub fn new(
        id: &str,
        client_type: &str,
        url: &str,
        username: &str,
        password: &str,
        http_client: reqwest::Client,
    ) -> Self {
        Self {
            id: id.to_string(),
            client_type: client_type.to_string(),
            url: url.to_string(),
            username: username.to_string(),
            password: password.to_string(),
            http_client,
        }
    }

    async fn post_form(
        &self,
        endpoint: &str,
        form_data: HashMap<&str, &str>,
    ) -> Result<String, reqwest::Error> {
        let url = format!("{}{}", self.url, endpoint);
        let resp = self
            .http_client
            .post(url)
            .header("User-Agent", "toru-client")
            .header("Accept", "*/*")
            .form(&form_data)
            .send()
            .await?;

        resp.text().await
    }
}

impl TorrentClient for QBittorrent {
    type Err = QBittorrentError;

    fn id(&self) -> &str {
        &self.id
    }

    fn client_type(&self) -> &str {
        &self.client_type
    }

    fn url(&self) -> &str {
        &self.url
    }

    fn username(&self) -> &str {
        &self.username
    }

    fn password(&self) -> &str {
        &self.password
    }

    async fn init(&self) -> Result<(), Self::Err> {
        info!("initializing {}: type = {}", self.id(), self.client_type());
        let url = format!("{}{}", self.url(), Self::QBIT_LOGIN_EP);
        let mut form_data = HashMap::new();
        form_data.insert("username", self.username());
        form_data.insert("password", self.password());

        let response_text = self
            .post_form(&url, form_data)
            .await
            .map_err(|_| QBittorrentError::LoginFailedError)?;

        if response_text.ne("Ok.") {
            error!("login failed. invalid credentials");
            return Err(QBittorrentError::LoginFailedError);
        }

        Ok(())
    }

    async fn download(
        &self,
        title: &str,
        torrent_url: &str,
        save_path: &Path,
    ) -> Result<(), Self::Err> {
        info!("adding torrent '{title}' to qbittorrent");
        let url = format!("{}{}", self.url(), Self::QBIT_ADD_TORR_EP);
        let mut form_data = HashMap::new();
        form_data.insert("urls", torrent_url);
        form_data.insert("savepath", save_path.to_str().unwrap());

        let response_text = self.post_form(&url, form_data).await.map_err(|_| {
            QBittorrentError::DownloadQueueError("unable to send request to qbit".to_string())
        })?;

        if response_text.ne("Ok.") {
            error!("qbit refused the torrent queue request");
            return Err(QBittorrentError::DownloadQueueError(
                "qbit refused enqueue request".to_string(),
            ));
        }
        Ok(())
    }
}
