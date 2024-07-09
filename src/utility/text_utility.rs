use regex::Regex;



pub fn replace(content: &str, replacements: &Vec<(Regex, String)>) -> String {
    let mut result = String::from(content);

    for (regex, rep) in replacements {
        result = regex.replace_all(&result, rep).to_string();
    }

    result
}