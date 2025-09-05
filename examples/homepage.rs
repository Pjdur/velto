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
    route!(app, "/" => homepage);
    app.enable_dev_mode();
    app.serve_static("static");
    app.run("127.0.0.1:8080").await
}
