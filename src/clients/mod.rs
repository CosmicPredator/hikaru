use std::path::Path;

pub mod qbittorrent;

pub trait TorrentClient {
    type Err;

    fn id(&self) -> &str;
    fn client_type(&self) -> &str;
    fn url(&self) -> &str;
    fn username(&self) -> &str;
    fn password(&self) -> &str;

    async fn init(&self) -> Result<(), Self::Err>
    where
        Self: Sized;
    async fn download(&self, title: &str, torrent_url: &str, save_path: &Path) -> Result<(), Self::Err>;
}