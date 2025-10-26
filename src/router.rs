pub use async_tiny::{Header, Request, Response};

/// A route handler takes a reference to an async_tiny::Request
/// and returns an async_tiny::Response.
pub type Handler = Box<dyn Fn(&Request) -> Response + Send + Sync>;
