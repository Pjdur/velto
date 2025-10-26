use crate::{Request, Response};

/// A middleware function takes a request and the next handler, and returns a response.
pub type Middleware = fn(&Request, &dyn Fn(&Request) -> Response) -> Response;

/// Logs the request method and URL before and after handling.
pub fn logger(req: &Request, next: &dyn Fn(&Request) -> Response) -> Response {
    println!("📥 {} {}", req.method(), req.url());
    let res = next(req);
    println!("📤 Responded with {}", res.status_code());
    res
}
