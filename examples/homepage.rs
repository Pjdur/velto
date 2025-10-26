use velto::middleware::logger;
use velto::prelude::*;

fn homepage(_req: &Request) -> Response {
    render!("index.html", {
        "title" => "Welcome",
        "message" => "Hello, Velto!"
    })
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut app = App::new();
    app.use_middleware(logger);
    route!(app, "/" => homepage);
    app.run("127.0.0.1:8080").await
}
