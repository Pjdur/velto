use velto::prelude::*;

fn homepage(_req: &Request) -> Response<Cursor<Vec<u8>>> {
    render!("index.html", {
        "title" => "Welcome",
        "message" => "Hello, Velto!"
    })
}


fn main() {
    let mut app = App::new();
    route!(app, "/" => homepage);
    app.serve_static("static");
    app.run("127.0.0.1:8080");
}