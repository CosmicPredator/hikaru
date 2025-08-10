use std::path::PathBuf;
use crate::{actions::{Action, ExecError}, clients::TorrentClient};

pub struct DownloadAction<T: TorrentClient + Send + Sync> {
    client: T,
    torrent_link: String,
    save_path: PathBuf
}

impl<T: TorrentClient + Send + Sync> DownloadAction<T> {

    /// Creates new instance of DownloadAction<T>
    pub fn new(client: T, torrent_link: String, save_path: PathBuf) -> Self {
        Self { client, torrent_link, save_path }
    }
}

impl<T: TorrentClient + Send + Sync> Action<()> for DownloadAction<T> {        
    async fn execute(&mut self) -> Result<(), ExecError> {
        self
            .client
            .download("".into(), &self.torrent_link, &self.save_path)
            .await
            .map_err(|_| ExecError("client refused to enqueue".into()))
    }
}

