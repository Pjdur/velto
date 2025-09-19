use velto::prelude::*;

/// GET handler for the signup page.
/// Renders the `signup.html` template from the `templates/` directory.
fn show_signup(_req: &Request) -> Response {
    render!("signup.html", {
        "title" => "Sign Up"
    })
}

/// POST handler for form submission.
/// Parses the form body and responds with a confirmation message.
fn handle_signup(req: &Request) -> Response {
    // Convert the raw request body (Bytes) into a UTF-8 string
    let form = parse_form(std::str::from_utf8(req.body()).unwrap_or(""));

    // Extract form fields safely using `.get(...)`
    let username = form.get("username").map(|s| s.as_str()).unwrap_or("");
    let password = form.get("password").map(|s| s.as_str()).unwrap_or("");

    // Respond with a simple text confirmation
    let reply = format!("Signed up as: {}\nPassword: {}", username, password);
    Response::from_string(reply).with_content_type("text/plain")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Create a new Velto app instance
    let mut app = App::new();

    // Register GET and POST routes for "/signup"
    // GET shows the form, POST handles submission
    route!(app, GET "/signup" => show_signup);
    route!(app, POST "/signup" => handle_signup);

    // Serve static files from the "static/" directory
    app.serve_static("static");

    // Start the server on localhost:8080
    app.run("127.0.0.1:8080").await
}
