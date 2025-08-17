pub use std::io::Cursor;
pub use tiny_http::{Request, Response, Header};

pub type Handler = fn(&Request) -> Response<Cursor<Vec<u8>>>;
