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

    fn start(&mut self) -> impl std::future::Future<Output = Result<(), PreviewError>> + Send;
    
    fn render(&mut self) -> impl std::future::Future<Output = Result<(), PreviewError>> + Send;

    fn update(&mut self) -> impl std::future::Future<Output = Result<(), PreviewError>> + Send;

    fn stop(&mut self) -> impl std::future::Future<Output = Result<(), PreviewError>> + Send;
}