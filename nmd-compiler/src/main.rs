use nmd_compiler::{CompilerCli, CompilerCliError};


fn main() -> Result<(), CompilerCliError> {

    let cli = CompilerCli::new();

    cli.parse()
}