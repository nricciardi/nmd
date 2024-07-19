use html_preview::HtmlPreviewError;
use thiserror::Error;
use tokio::task::JoinError;

pub mod html_preview;


#[derive(Error, Debug)]
pub enum PreviewError {

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    JoinError(#[from] JoinError),

    #[error(transparent)]
    HtmlPreviewError(#[from] HtmlPreviewError),
}


pub trait Preview {

    async fn start(&mut self) -> Result<(), PreviewError>;
    
    async fn render(&mut self) -> Result<(), PreviewError>;

    async fn update(&mut self) -> Result<(), PreviewError>;

    async fn stop(&mut self) -> Result<(), PreviewError>;
}