#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    OPTIONS,
    HEAD,
}

impl Method {
    pub fn from_hyper(method: &http::Method) -> Self {
        match *method {
            http::Method::GET => Method::GET,
            http::Method::POST => Method::POST,
            http::Method::PUT => Method::PUT,
            http::Method::DELETE => Method::DELETE,
            http::Method::PATCH => Method::PATCH,
            http::Method::OPTIONS => Method::OPTIONS,
            http::Method::HEAD => Method::HEAD,
            _ => Method::GET, // fallback
        }
    }
}
