use velto::http_method::Method;
use velto::prelude::*;

/// A unified handler that inspects the HTTP method and responds accordingly.
fn method_handler(req: &Request) -> Response {
    match Method::from_hyper(req.method()) {
        Method::GET => Response::from_string("You sent a GET request."),
        Method::POST => Response::from_string("You sent a POST request."),
        Method::PUT => Response::from_string("You sent a PUT request."),
        Method::DELETE => Response::from_string("You sent a DELETE request."),
        _ => Response::from_string("You sent some other request."),
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut app = App::new();

    // Register the same handler for multiple methods
    route!(app, [GET, POST, PUT, DELETE] "/method" => method_handler);

    app.run("127.0.0.1:8080").await
}
