use env_logger;
use anyhow::Result;
use nmd_compiler::Compiler;
use nmd_compiler::compiler::CompilerConfiguration;
use regex::Regex;

fn main() -> Result<()> {

    let content = "#1 Titolo del paragrafo 1";

    let regex = Regex::new(r"#(\d+)\s*(.*)").expect("Errore nella compilazione della regex");

    let parsed_content = regex.replace_all(&content, r#"<h$1>$2</h$1>"#).to_string();

    println!("{}", parsed_content);

    Ok(())

    /* env_logger::init();

    let compiler_configuration = CompilerConfiguration::new( ".", "html")?;

    let compiler = Compiler::new(compiler_configuration)?;

    Ok(compiler.compile()?) */
}