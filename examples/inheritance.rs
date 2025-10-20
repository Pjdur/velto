use velto::prelude::*;

fn child(_req: &Request) -> Response {
    render!("child.html", {
        "message" => "Hello from child template!"
    })
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut app = App::new();
    route!(app, "/" => child);
    app.run("127.0.0.1:8080").await
}
