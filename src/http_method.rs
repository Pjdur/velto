/// Represents the set of supported HTTP methods.
///
/// This enum provides a simplified and strongly typed representation
/// of common HTTP request methods.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Method {
    /// The HTTP GET method.
    GET,
    /// The HTTP POST method.
    POST,
    /// The HTTP PUT method.
    PUT,
    /// The HTTP DELETE method.
    DELETE,
    /// The HTTP PATCH method.
    PATCH,
    /// The HTTP OPTIONS method.
    OPTIONS,
    /// The HTTP HEAD method.
    HEAD,
}

impl Method {
    /// Converts a reference to an `http::Method` into a corresponding `Method` variant.
    ///
    /// # Arguments
    ///
    /// * `method` - A reference to an `http::Method` from the `http` crate.
    ///
    /// # Returns
    ///
    /// A `Method` variant matching the input. If the method is not recognized,
    /// it defaults to `Method::GET`.
    ///
    /// # Example
    ///
    /// ```
    /// use http::Method as HyperMethod;
    /// use velto::http_method::Method;
    ///
    /// let method = Method::from_hyper(&HyperMethod::POST);
    /// assert_eq!(method, Method::POST);
    /// ```
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
