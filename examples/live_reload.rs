use velto::prelude::*;

/// A simple homepage route that renders `index.html` with dynamic data.
fn homepage(_req: &Request) -> Response {
    render!("index.html", {
        "title" => "Welcome",
        "message" => "Hello, Velto!"
    })
}

/// Starts a Velto app in development mode with LiveReload enabled.
/// Serves static files from the `static/` directory and mounts the homepage at `/`.
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut app = App::new();
    app.enable_dev_mode();
    route!(app, "/" => homepage);
    app.serve_static("static");
    app.run("127.0.0.1:8080").await
}
