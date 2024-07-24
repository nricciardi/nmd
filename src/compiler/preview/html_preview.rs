use std::{path::PathBuf, sync::RwLock};
use chrono::{DateTime, Local};
use getset::{Getters, Setters};
use once_cell::sync::Lazy;
use serde::Serialize;
use thiserror::Error;
use tokio::{fs::File, io::AsyncReadExt, task::JoinHandle};
use warp::Filter;

use super::{Preview, PreviewError};

pub const PREVIEW_STATE_INFO_ROUTE: &str = "preview-state-info";
const DEFAULT_SCRAPE_INTERVAL: u32 = 2000; 


pub static LAST_UPDATE: Lazy<RwLock<Option<DateTime<Local>>>> = Lazy::new(|| RwLock::new(None));
pub static LAST_SEEN: Lazy<RwLock<Option<DateTime<Local>>>> = Lazy::new(|| RwLock::new(None));


#[derive(Debug, Serialize)]
struct PreviewStateInfo {
    last_update_timestamp: Option<i64>,
    last_seen_timestamp: Option<i64>,
    scrape_interval: Option<u32>,
}


#[derive(Error, Debug)]
pub enum HtmlPreviewError {
}


pub const PREVIEW_PORT: u16 = 1234;
pub const PREVIEW_URL: &str = "http://127.0.0.1:1234";      // change if PREVIEW_PORT changes


#[derive(Debug, Getters, Setters)]
pub struct HtmlPreview {
    
    #[getset(get = "pub", set = "pub")]
    src: PathBuf,

    server_thread_handle: Option<JoinHandle<()>>,
    
    client_preview_scraping_interval: u32,
}

impl HtmlPreview {
    pub fn new(src: PathBuf, client_preview_scraping_interval: Option<u32>) -> Self {
        Self {
            src,
            server_thread_handle: None,
            client_preview_scraping_interval: client_preview_scraping_interval.unwrap_or(DEFAULT_SCRAPE_INTERVAL)
        }
    }
}

impl Preview for HtmlPreview {

    async fn start(&mut self) -> Result<(), PreviewError> {

        let src = self.src.clone();

        let original_log_max_level = log::max_level();

        log::set_max_level(log::LevelFilter::Warn);

        let client_preview_scraping_interval = self.client_preview_scraping_interval;

        self.server_thread_handle = Some(tokio::spawn(async move {

            let show_preview = move || {
                let src = src.clone();

                log::info!("serving preview...");

                serve_preview(src)
            };

            let preview_route_implicite = warp::path::end()
                                .and_then(show_preview.clone());

            let preview_route_explicit = warp::path!("preview")
                                .and_then(show_preview);

            let preview_state_info_route = warp::path(PREVIEW_STATE_INFO_ROUTE)
                                        .map(move || {

                                            let last_update_timestamp: Option<i64>;
                                            let last_seen_timestamp: Option<i64>;

                                            if let Some(l) = *LAST_UPDATE.read().unwrap() {

                                                last_update_timestamp = Some(l.timestamp());
                                            
                                            } else {

                                                last_update_timestamp = None;
                                            }

                                            if let Some(l) = *LAST_SEEN.read().unwrap() {

                                                last_seen_timestamp = Some(l.timestamp());
                                            
                                            } else {

                                                last_seen_timestamp = None;
                                            }

                                            let response = PreviewStateInfo {
                                                last_update_timestamp,
                                                last_seen_timestamp,
                                                scrape_interval: Some(client_preview_scraping_interval)
                                            };

                                            let now = chrono::offset::Local::now();

                                            log::debug!("html preview seen (new last seen: {})", now);

                                            *LAST_SEEN.write().unwrap() = Some(now);

                                            warp::reply::json(&response)
                                        });
    
            log::info!("html preview will be running on: {}", PREVIEW_URL);

            warp::serve(
                preview_route_implicite
                .or(preview_route_explicit)
                .or(preview_state_info_route)
            )
            .run(([127, 0, 0, 1], PREVIEW_PORT))
            .await
        }));

        log::set_max_level(original_log_max_level);
        
        Ok(())
    }

    async fn render(&mut self) -> Result<(), PreviewError> {

        log::info!("html preview rendered");

        Ok(())
    }

    async fn update(&mut self) -> Result<(), PreviewError> {

        let now = chrono::offset::Local::now();

        *LAST_UPDATE.write().unwrap() = Some(now);

        log::info!("html preview updated (new last update: {})", now);

        Ok(())
    }
    
    async fn stop(&mut self) -> Result<(), PreviewError> {

        if let Some(j) = self.server_thread_handle.take() {
            j.await?;       // need Ctrl + C to terminate
        }

        log::info!("html preview stop");

        Ok(())
    }
}

async fn serve_preview(file_path: PathBuf) -> Result<impl warp::Reply, warp::Rejection> {

    let mut file = File::open(file_path.clone()).await.map_err(|err| {

        log::error!("error occurs during preview file opening: {} ({:?})", err.to_string(), file_path);

        warp::reject()
    })?;

    let mut contents = String::new();

    file.read_to_string(&mut contents).await.map_err(|err| {

        log::error!("error occurs during preview file reading: {} ({:?})", err.to_string(), file_path);

        warp::reject()
    })?;

    Ok(warp::reply::html(contents))
}