use env_logger;
use anyhow::Result;
use nmd_compiler::Compiler;
use nmd_compiler::compiler::CompilerConfiguration;

fn main() -> Result<()> {

    env_logger::init();

    let compiler_configuration = CompilerConfiguration::new("html")?;

    let compiler = Compiler::new(compiler_configuration);

    Ok(compiler.compile()?)
}