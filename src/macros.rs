/// render! macro allowing for easy templating
/// Example:
/// ```
/// use velto::{render, Response};
///
/// let _ = render!("index.html", {
///     "title" => "Welcome",
///     "message" => "Hello, Velto!"
/// });
/// ```
#[macro_export]
macro_rules! render {
    ($file:expr, { $($key:expr => $val:expr),* $(,)? }) => {{
        let mut ctx = std::collections::HashMap::new();
        $(ctx.insert($key, $val);)*
        let html = $crate::render_template($file, &ctx);
        $crate::Response::from_data(html.into_bytes())
            .with_header("Content-Type: text/html".parse::<$crate::Header>().unwrap())
    }};
}

/// Route macro for defining routes
/// Supports single method, multiple methods, or default GET.
/// Example:
/// ```
/// use velto::{App, Response, route};
///
/// let mut app = App::new();
/// route!(app, "/hello" => |_req| {
///     Response::from_string("Hello!")
/// });
///
/// route!(app, [GET, POST] "/signup" => |req| {
///     match req.method().as_str() {
///         "POST" => Response::from_string("Signed up!"),
///         _ => Response::from_string("Sign up form"),
///     }
/// });
/// ```
#[macro_export]
macro_rules! route {
    // Multiple methods → same handler
    ($app:expr, [$($method:ident),+] $path:expr => $handler:expr) => {{
        $( $app.route($crate::http_method::Method::$method, $path, $handler); )+
    }};

    // Single method
    ($app:expr, $method:ident $path:expr => $handler:expr) => {{
        $app.route($crate::http_method::Method::$method, $path, $handler);
    }};

    // Default to GET
    ($app:expr, $path:expr => $handler:expr) => {{
        $app.route($crate::http_method::Method::GET, $path, $handler);
    }};
}

/// route_any! macro for registering a handler across all standard HTTP methods
/// Example:
/// ```
/// use velto::{App, Response, route_any};
///
/// let mut app = App::new();
/// route_any!(app, "/echo" => |req| {
///     Response::from_string(format!("Method: {}", req.method()))
/// });
/// ```
#[macro_export]
macro_rules! route_any {
    ($app:expr, $path:expr => $handler:expr) => {{
        use $crate::http_method::Method::*;
        for method in [GET, POST, PUT, DELETE, PATCH, OPTIONS] {
            $app.route(method, $path, $handler);
        }
    }};
}
