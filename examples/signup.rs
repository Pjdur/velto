use velto::prelude::*;

/// Unified handler for both GET and POST requests to `/signup`.
/// - GET: renders the signup form
/// - POST: parses form data and returns confirmation
fn signup(req: &Request) -> Response {
    match req.method().as_str() {
        "POST" => {
            let form = parse_form(std::str::from_utf8(req.body()).unwrap_or(""));
            let username = form.get("username").map(|s| s.as_str()).unwrap_or("");
            let password = form.get("password").map(|s| s.as_str()).unwrap_or("");

            let reply = format!("Signed up as: {}\nPassword: {}", username, password);
            Response::from_string(reply).with_content_type("text/plain")
        }
        _ => {
            render!("signup.html", {
                "title" => "Sign Up"
            })
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut app = App::new();

    // Register both GET and POST methods for "/signup" using one handler
    route!(app, [GET, POST] "/signup" => signup);

    // Serve static files from the "static/" directory
    app.serve_static("static");

    // Start the server on localhost:8080
    app.run("127.0.0.1:8080").await
}
