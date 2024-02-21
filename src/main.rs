use nmd::{NmdCli, NmdCliError};


fn main() -> Result<(), NmdCliError> {

    let cli = NmdCli::new();

    cli.parse()
}