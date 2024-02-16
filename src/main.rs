use nmd::{NmdCli, NmdCliError};


fn main() -> Result<(), NmdCliError> {

    let cli = NmdCli::new();

    cli.parse()

//     let regex = regex::Regex::new(r"::: (\w+)\n(?s:(.*))\n:::").unwrap();

//     let text = r#"

// # title 1

// ::: warning
// new
// warning

// multiline
// :::

// "#.trim();

//     let r = regex.find(text);

//     Ok(())
}