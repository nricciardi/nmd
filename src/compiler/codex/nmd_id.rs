const PREFIX_VALUE_SEPARATOR: &str = "-";
const SPACE_REPLACER: char = '-';



pub struct NmdId {
    prefix: Option<String>,
    value: String,
}

impl NmdId {
    pub fn new(value: &str) -> Self {
        Self {
            prefix: None,
            value: String::from(value)
        }
    }

    pub fn new_with_prefix(prefix: &str, value: &str) -> Self {
        Self {
            prefix: Some(String::from(prefix)),
            value: String::from(value)
        }
    }

    pub fn prefix(&self) -> &Option<String> {
        &self.prefix
    }

    pub fn value(&self) -> &String {
        &self.value
    }

    pub fn set_prefix(&mut self, prefix: Option<String>) {
        self.prefix = prefix
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value
    }

    pub fn build(&self) -> String {

        if let Some(prefix) = self.prefix.as_ref() {
            return format!("{}{}{}", Self::parse_str(prefix), PREFIX_VALUE_SEPARATOR, Self::parse_str(&self.value))
        }

        format!("{}", Self::parse_str(&self.value))
    }

    fn parse_str(s: &str) -> String {

        let allowed_chars = s.chars().filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-' || *c == ' ').map(|c| {
            if c == ' ' {
                return SPACE_REPLACER;
            }

            c
        });

        allowed_chars.collect()
    }
}