use std::path::PathBuf;

use hcl::{
    eval::{Context, Evaluate},
    expr::TemplateExpr,
};
use log::info;
use quick_xml::{name, se};

use crate::{
    actions::{Action, exec::ExecAction},
    clients::{TorrentClient, qbittorrent::QBittorrent},
    feeds::feed_model::RssFeedModel,
    hconf_parser::model::{FeedAction, HConf},
};

pub struct TaskPipeline {
    task_config: HConf,
    http_client: reqwest::Client,
}

#[derive(Debug)]
pub enum TaskPipelineError {
    HttpClientError(String),
    InitError,
}

impl From<reqwest::Error> for TaskPipelineError {
    fn from(value: reqwest::Error) -> Self {
        Self::HttpClientError(format!("unable to initialize http client: {}", value))
    }
}

impl TryFrom<HConf> for TaskPipeline {
    type Error = TaskPipelineError;

    fn try_from(value: HConf) -> Result<Self, Self::Error> {
        let http_client = reqwest::Client::builder().cookie_store(true).build()?;
        Ok(Self {
            task_config: value,
            http_client,
        })
    }
}

impl TaskPipeline {
    pub async fn run(&self) -> Result<(), TaskPipelineError> {
        let torrent_client = QBittorrent::new(
            &self.task_config.torrent_client.id,
            &self.task_config.torrent_client.client_type,
            &self.task_config.torrent_client.url,
            &self.task_config.torrent_client.username,
            &self.task_config.torrent_client.password,
            self.http_client.clone(),
        );

        torrent_client
            .init()
            .await
            .map_err(|_| TaskPipelineError::InitError)?;

        let mut var_context = Context::new();
        for (_, action) in &self.task_config.task["erai_feed"].actions {
            if let FeedAction::Exec {
                command,
                args,
                output_as,
            } = action
            {
                let mut exec_action = ExecAction::new(
                    command,
                    args.iter().map(|s| s.as_str()).collect(),
                    output_as,
                );
                exec_action
                    .execute()
                    .await
                    .map_err(|_| TaskPipelineError::InitError)?;
                var_context.declare_var(output_as.as_ref(), exec_action.output_as);
            }
        }

        let rss_feed = RssFeedModel::from_url(
            &self.task_config.task["erai_feed"].rss_feed.url,
            self.http_client.clone(),
        )
        .await
        .map_err(|_| TaskPipelineError::InitError)?;

        for (_, action) in &self.task_config.task["erai_feed"].actions {
            if let FeedAction::Download { save_path, .. } = action {
                for feed in rss_feed.get_items() {
                    info!("processing: {}", feed.title);
                    let expr = TemplateExpr::from(save_path.clone());
                    let evaluated_path = expr
                        .evaluate(&var_context)
                        .map_err(|_| TaskPipelineError::InitError)?;
                    torrent_client.download(
                        &feed.title,
                        &feed.link,
                        &PathBuf::from(evaluated_path.as_str().unwrap()),
                    ).await
                    .map_err(|_| TaskPipelineError::InitError)?;
                }
            }
        }

        Ok(())
    }
}
