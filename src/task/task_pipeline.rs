use std::{path::PathBuf, sync::Arc};

use anyhow::anyhow;
use hcl::eval::Context;
use log::{debug, info};
use quick_xml::se;
use reqwest::Error;

use crate::{
    actions::{Action, download::DownloadAction},
    clients::{TorrentClient, qbittorrent::QBittorrent},
    feeds::feed_model::RssFeedModel,
    hconf_parser::model::{ClientType, FeedAction, HConf},
    task::task_schedule::TaskSchedule,
};

pub struct TaskPipeline {
    task_config: HConf,
    http_client: reqwest::Client,
}

impl TryFrom<HConf> for TaskPipeline {
    type Error = anyhow::Error;

    fn try_from(value: HConf) -> Result<Self, Self::Error> {
        let http_client = reqwest::Client::builder().cookie_store(true).build()?;
        Ok(Self {
            task_config: value,
            http_client,
        })
    }
}

impl TaskPipeline {
    async fn prepare_torr_client(&self) -> Result<impl TorrentClient, anyhow::Error> {
        if let ClientType::QBittorrent = &self.task_config.torrent_client.client_type {
            let client = QBittorrent::new(
                &self.task_config.torrent_client.id,
                &self.task_config.torrent_client.client_type.to_string(),
                &self.task_config.torrent_client.url,
                &self.task_config.torrent_client.username,
                &self.task_config.torrent_client.password,
                self.http_client.clone(),
            );

            client.init().await.map_err(|err| match err {
                crate::clients::qbittorrent::QBittorrentError::LoginFailedError => {
                    anyhow!("failed to initiazlize qbit client")
                }
                crate::clients::qbittorrent::QBittorrentError::DownloadQueueError(msg) => {
                    anyhow!("{}", msg)
                }
            })?;

            return Ok(client);
        }
        Err(anyhow!("invalid torrent client"))
    }

    async fn run_feed(&self) -> Result<(), anyhow::Error> {
        for (task_name, task) in &self.task_config.tasks {
            let task_name = Arc::new(task_name.clone());
            let task = Arc::new(task.clone());
            let http_client = self.http_client.clone();

            let torr_client = &self.prepare_torr_client().await.unwrap();

            tokio::spawn(async move {
                let task = Arc::clone(&task);
                let task_name = Arc::clone(&task_name);
                info!("spawning thread for task {}", task_name.clone());

                // parsing schedule interval
                let schedule = TaskSchedule::try_from(task.schedule.as_str()).unwrap();

                // saving hconf contexts
                let mut var_context = Context::new();
                for (name, action) in &task.actions {
                    if let FeedAction::Exec { command, args } = action {
                        let mut exec_action = crate::actions::exec::ExecAction::new(
                            &command,
                            args.iter().map(|s| s.as_str()).collect(),
                        );
                        let output = exec_action.execute().await.unwrap();
                        let output_var = format!("{}.output", name);
                        debug!("adding context: {} -> {}", &output_var, &output);
                        var_context.declare_var(output_var, output);
                    }
                }

                // requesting rss feed from internet
                let feeds = RssFeedModel::from_url(&task.rss_feed.url, http_client)
                    .await
                    .unwrap();

                for item in feeds.get_items() {
                    for (_, action) in &task.actions {
                        if let FeedAction::Download { save_path, .. } = action {
                            // let dl_action = DownloadAction::new(
                            //     torr_client,
                            //     item.link.unwrap(),
                            //     PathBuf::from(save_path.as_ref())
                            // );
                        }
                    }
                }

                loop {
                    tokio::time::sleep(schedule.to_seconds()).await;
                }
            });
        }
        Ok(())
    }
}
