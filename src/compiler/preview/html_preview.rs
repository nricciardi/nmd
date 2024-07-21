use std::{path::PathBuf, sync::RwLock, time::{Instant, SystemTime}};
use chrono::{DateTime, Local};
use getset::{Getters, Setters};
use once_cell::sync::Lazy;
use thiserror::Error;
use tokio::{fs::File, io::AsyncReadExt, task::JoinHandle};
use warp::Filter;
use tokio::sync::RwLock as TokioRwLock;

use super::{Preview, PreviewError};

pub const CHECK_PREVIEW_UPDATE_ROUTE: &str = "check-preview-updates";

pub static LAST_UPDATE: Lazy<RwLock<DateTime<Local>>> = Lazy::new(|| RwLock::new(chrono::offset::Local::now()));

#[derive(Error, Debug)]
pub enum HtmlPreviewError {
}


const PREVIEW_PORT: u16 = 1234;


#[derive(Debug, Getters, Setters)]
pub struct HtmlPreview {
    
    #[getset(get = "pub", set = "pub")]
    src: PathBuf,

    server_thread_handle: Option<JoinHandle<()>>
}

impl HtmlPreview {
    pub fn new(src: PathBuf) -> Self {
        Self {
            src,
            server_thread_handle: None
        }
    }
}

impl Preview for HtmlPreview {

    async fn start(&mut self) -> Result<(), PreviewError> {

        let src = self.src.clone();

        let original_log_max_level = log::max_level();

        log::set_max_level(log::LevelFilter::Warn);

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

            let check_preview_update_route = warp::path(CHECK_PREVIEW_UPDATE_ROUTE)
                                                        .map(|| {
                                                            warp::reply::json(&LAST_UPDATE.read().unwrap().timestamp())
                                                        });
    
            log::info!("html preview will be running on: http://127.0.0.1:{}", PREVIEW_PORT);

            warp::serve(
                preview_route_implicite
                .or(preview_route_explicit)
                .or(check_preview_update_route)
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

        *LAST_UPDATE.write().unwrap() = now;

        log::info!("html preview updated (new last update: {})", now);

        Ok(())
    }
    
    async fn stop(&mut self) -> Result<(), PreviewError> {
        
        if let Some(j) = self.server_thread_handle.take() {
            j.await?;
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