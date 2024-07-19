use std::path::PathBuf;
use getset::{Getters, Setters};
use rocket::{config::Shutdown, fs::FileServer, Ignite, Rocket};
use thiserror::Error;
use tokio::task::JoinHandle;

use super::{Preview, PreviewError};

#[derive(Error, Debug)]
pub enum HtmlPreviewError {

    #[error(transparent)]
    WebServerStartError(#[from] rocket::Error),
}


const PREVIEW_PORT: u16 = 1234;


#[derive(Debug, Getters, Setters)]
pub struct HtmlPreview {
    
    #[getset(get = "pub", set = "pub")]
    src: PathBuf,

    server_thread_handle: Option<JoinHandle<Result<Rocket<Ignite>, rocket::Error>>>
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

        // TODO:
        // log::set_max_level(log::LevelFilter::Warn);

        let server = rocket::build()
            .mount("/", FileServer::from(src))
            .configure(rocket::Config {
                port: PREVIEW_PORT,
                ..rocket::Config::default()
            });

        log::set_max_level(original_log_max_level);

        self.server_thread_handle = Some(tokio::spawn(async {

            log::info!("html preview will be running on port: {}", PREVIEW_PORT);

            server.launch().await       // TODO: do not start
        }));
        
        Ok(())
    }

    async fn render(&mut self) -> Result<(), PreviewError> {

        log::info!("html preview rendered");

        Ok(())
    }

    async fn update(&mut self) -> Result<(), PreviewError> {

        log::info!("html preview updated");

        Ok(())
    }
    
    async fn stop(&mut self) -> Result<(), PreviewError> {
        
        if let Some(j) = self.server_thread_handle.take() {
            let r = j.await?;

            if let Ok(rocket) = r {
                rocket.shutdown().await;
            } else {

                let err = r.err().unwrap();

                log::error!("error occurs: {}", err);

                return Err(PreviewError::HtmlPreviewError(HtmlPreviewError::WebServerStartError(err)));
            }
        }

        log::info!("html preview stop");

        Ok(())
    }
}