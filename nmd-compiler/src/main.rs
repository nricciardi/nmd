use nmd_compiler::Compiler;
use nmd_compiler::compiler::CompilerConfiguration;

fn main() {

    let compiler_configuration = CompilerConfiguration::new("html");

    let compiler = Compiler::new(compiler_configuration);

    compiler.compile()
}