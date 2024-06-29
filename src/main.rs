use std::sync::Arc;

use nmd::cli::{NmdCli, NmdCliError};



fn main() -> Result<(), NmdCliError> {

    let cli = NmdCli::new();

    cli.parse()
}