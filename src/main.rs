use tokio;
use nmd::cli::{NmdCli, NmdCliError};


#[tokio::main]
async fn main() -> Result<(), NmdCliError> {

    let cli = NmdCli::new();

    cli.serve().await
}