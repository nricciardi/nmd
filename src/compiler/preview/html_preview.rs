use std::path::PathBuf;
use getset::{Getters, Setters};
use thiserror::Error;
use tokio::{fs::File, io::AsyncReadExt, task::JoinHandle};
use warp::Filter;

use super::{Preview, PreviewError};

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

        // TODO:
        // log::set_max_level(log::LevelFilter::Warn);

        // let server = rocket::build()
        //     .mount("/", FileServer::from(src))
        //     .configure(rocket::Config {
        //         port: PREVIEW_PORT,
        //         ..rocket::Config::default()
        //     });

        self.server_thread_handle = Some(tokio::spawn(async move {

            let preview_route = warp::path::end()
            .and_then(move || {
                let src = src.clone();

                log::info!("serving preview...");

                serve_preview(src)
            });
    
            log::info!("html preview will be running local (127.0.0.1) on port: {}", PREVIEW_PORT);

            warp::serve(preview_route)
            .run(([127, 0, 0, 1], PREVIEW_PORT))
            .await
        }));

        // log::set_max_level(original_log_max_level);

        // self.server_thread_handle = Some(tokio::spawn(async {
        // }));
        
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