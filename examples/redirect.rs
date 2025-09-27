use velto::prelude::*;

fn login_handler(_req: &Request) -> Response {
    redirect("/dashboard")
}

fn dashboard_handler(_req: &Request) -> Response {
    Response::from_string("Welcome to your dashboard!")
}

#[tokio::main]
async fn main() {
    let mut app = App::new();

    route!(app, [GET] "/login" => login_handler);
    route!(app, [GET] "/dashboard" => dashboard_handler);

    if let Err(e) = app.run("127.0.0.1:8080").await {
        eprintln!("Server failed to start: {}", e);
    }
}
