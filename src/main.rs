pub mod cli;
pub mod preview;
pub mod watcher;
pub mod dossier_manager;
pub mod generator;
pub mod builder;
pub mod constants;


use cli::{NmdCli, NmdCliError};
use tokio;


#[tokio::main]
async fn main() -> Result<(), NmdCliError> {

    let cli = NmdCli::new();

    cli.serve().await
}