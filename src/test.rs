use crate::http_method::Method;
use crate::{App, Request, Response};
use std::str::FromStr;

pub struct TestRequest {
    method: Method,
    path: String,
    body: Vec<u8>,
}

impl TestRequest {
    pub fn new(method: &str, path: &str) -> Self {
        Self {
            method: Method::from_hyper(
                &http::Method::from_str(method).unwrap_or(http::Method::GET),
            ),
            path: path.to_string(),
            body: Vec::new(),
        }
    }

    pub fn with_body(mut self, body: &str) -> Self {
        self.body = body.as_bytes().to_vec();
        self
    }

    pub fn send(&self, app: &App) -> Response {
        let http_method =
            http::Method::from_str(&format!("{:?}", self.method)).unwrap_or(http::Method::GET);
        let req = Request::fake(&http_method, &self.path, &self.body);
        let routes = app.get_routes();

        routes
            .get(&self.path)
            .and_then(|m| m.get(&self.method))
            .map(|handler| handler(&req))
            .unwrap_or_else(|| Response::from_string("404 Not Found").with_status_code(404))
    }
}
