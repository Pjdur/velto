use velto::prelude::*;

fn page(_req: &Request) -> Response {
    render!("page.html", {
        "title" => "Included Header"
    })
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut app = App::new();
    route!(app, "/" => page);
    app.run("127.0.0.1:8080").await
}
