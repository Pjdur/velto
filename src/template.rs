use regex::Regex;
use std::collections::HashMap;
use std::fs;

/// Renders an HTML template with context interpolation, includes, and inheritance.
///
/// This function loads a template file from the `templates/` directory and processes it
/// using a lightweight templating engine. It supports:
///
/// - `{{ key }}`: Variable interpolation from the provided context.
/// - `{% include 'file.html' %}`: Includes and renders another template inline.
/// - `{% extends 'base.html' %}`: Inherits from a base template.
/// - `{% block name %}...{% endblock %}`: Defines content blocks for overriding in child templates.
///
/// # Arguments
///
/// * `file` - The name of the template file to render (relative to the `templates/` directory).
/// * `context` - A map of key-value pairs used for variable substitution in the template.
///
/// # Returns
///
/// A `String` containing the fully rendered HTML output.
///
/// # Example
///
/// ```rust
/// let mut context = HashMap::new();
/// context.insert("title", "Welcome");
/// context.insert("message", "Hello, Velto!");
///
/// let html = render_template("index.html", &context);
/// ```
///
/// # Template Example
///
/// `templates/base.html`:
/// ```html
/// <html>
///   <body>
///     {% block content %}{% endblock %}
///   </body>
/// </html>
/// ```
///
/// `templates/home.html`:
/// ```html
/// {% extends 'base.html' %}
///
/// {% block content %}
///   {% include 'header.html' %}
///   <p>{{ message }}</p>
/// {% endblock %}
/// ```
///
/// `templates/header.html`:
/// ```html
/// <header><h1>{{ title }}</h1></header>
/// ```
///
/// # Notes
///
/// - If development mode is enabled (`App::enable_dev_mode()`), a LiveReload script is injected
///   before the closing `</body>` tag to enable automatic browser refresh on file changes.
/// - Missing templates or includes will render a fallback error message.
pub fn render_template(file: &str, context: &HashMap<&str, &str>) -> String {
    let mut contents = load_template(file);

    // Handle {% extends 'base.html' %}
    let extends_re = Regex::new(r"\{%\s*extends\s*'([^']+)'\s*%\}").unwrap();
    if let Some(caps) = extends_re.captures(&contents) {
        let base_file = &caps[1];
        let base_contents = load_template(base_file);

        // Extract blocks from child
        let block_re = Regex::new(r"\{%\s*block\s+(\w+)\s*%\}(?s)(.*?)\{%\s*endblock\s*%\}").unwrap();
        let mut blocks = HashMap::new();
        for caps in block_re.captures_iter(&contents) {
            blocks.insert(caps[1].to_string(), caps[2].to_string());
        }

        // Replace blocks in base
        let base_block_re = Regex::new(r"\{%\s*block\s+(\w+)\s*%\}(?s)(.*?)\{%\s*endblock\s*%\}").unwrap();
        contents = base_block_re
            .replace_all(&base_contents, |caps: &regex::Captures| {
                blocks.get(&caps[1]).cloned().unwrap_or_else(|| caps[2].to_string())
            })
            .to_string();
    }

    // Handle {% include 'header.html' %}
    let include_re = Regex::new(r"\{%\s*include\s*'([^']+)'\s*%\}").unwrap();
    contents = include_re
        .replace_all(&contents, |caps: &regex::Captures| {
            load_template(&caps[1])
        })
        .to_string();

    // Handle {{ key }} interpolation
    let var_re = Regex::new(r"\{\{\s*(\w+)\s*\}\}").unwrap();
    contents = var_re
        .replace_all(&contents, |caps: &regex::Captures| {
            context.get(&caps[1]).copied().unwrap_or("")
        })
        .to_string();

    // Inject LiveReload if in dev mode
    if crate::is_dev_mode() {
        let reload_script = r#"
<script>
const ws = new WebSocket("ws://localhost:35729");
ws.onopen = () => console.log("✅ LiveReload connected");
ws.onmessage = () => {
    console.log("🔁 Reloading...");
    location.reload();
};
ws.onerror = (e) => console.error("❌ WebSocket error", e);
</script>
"#;
        if contents.contains("</body>") {
            contents = contents.replace("</body>", &format!("{}{}", reload_script, "</body>"));
        } else {
            contents.push_str(reload_script);
        }
    }

    contents
}

fn load_template(file: &str) -> String {
    let path = format!("templates/{}", file);
    fs::read_to_string(path).unwrap_or_else(|_| format!("<h1>Template '{}' not found</h1>", file))
}
