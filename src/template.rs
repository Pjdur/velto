use std::collections::HashMap;
use std::fs;
use regex::Regex;

/// Loads and renders an HTML template from the `templates/` directory,
/// replacing `{{ variable }}` placeholders with values from the context.
pub fn render_template(file: &str, context: &HashMap<&str, &str>) -> String {
    let path = format!("templates/{}", file);
    let mut contents = fs::read_to_string(path).unwrap_or_else(|_| "<h1>Template not found</h1>".to_string());

    let re = Regex::new(r"\{\{\s*(\w+)\s*\}\}").unwrap();
    contents = re.replace_all(&contents, |caps: &regex::Captures| {
        let key = &caps[1];
        context.get(key).copied().unwrap_or("")
    }).to_string();

    contents
}
