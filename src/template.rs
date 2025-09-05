use crate::is_dev_mode;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

pub fn render_template(file: &str, context: &HashMap<&str, &str>) -> String {
    let path = format!("templates/{}", file);
    let mut contents =
        fs::read_to_string(path).unwrap_or_else(|_| "<h1>Template not found</h1>".to_string());

    let re = Regex::new(r"\{\{\s*(\w+)\s*\}\}").unwrap();
    contents = re
        .replace_all(&contents, |caps: &regex::Captures| {
            let key = &caps[1];
            context.get(key).copied().unwrap_or("")
        })
        .to_string();

    if is_dev_mode() {
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
