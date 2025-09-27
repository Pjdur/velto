use crate::{Header, HeaderName, HeaderValue, Response};

/// Creates a 302 Found redirect response to the specified location.
///
/// This is a convenience wrapper around [`redirect_with_status`] for the common case.
///
/// # Example
/// ```
/// let response = redirect("/login");
/// ```
pub fn redirect(to: &str) -> Response {
    redirect_with_status(to, 302)
}

/// Creates a redirect response with a custom status code.
///
/// This sets the `Location` header and returns an empty body. Common status codes include:
/// - `301` (Moved Permanently)
/// - `302` (Found)
/// - `303` (See Other)
/// - `307` (Temporary Redirect)
/// - `308` (Permanent Redirect)
///
/// # Example
/// ```
/// let response = redirect_with_status("/new-url", 301);
/// ```
pub fn redirect_with_status(to: &str, status: u16) -> Response {
    Response::empty(status).with_header(Header(
        HeaderName::from_static("location"),
        HeaderValue::from_str(to).expect("valid Location header"),
    ))
}
