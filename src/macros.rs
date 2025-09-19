/// render! macro allowing for easy templating
/// Example:
/// ```
/// render!("index.html", {
///     "title" => "Welcome",
///     "message" => "Hello, Velto!"
/// })
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
/// Example:
/// ```
/// route!(app, "/hello" => |req| {
///     render!("hello.html", {
///         "name" => "World"
///     })
/// });
/// ```
#[macro_export]
macro_rules! route {
    ($app:expr, $method:ident $path:expr => $handler:expr) => {{
        $app.route($crate::http_method::Method::$method, $path, $handler);
    }};
    ($app:expr, $path:expr => $handler:expr) => {{
        $app.route($crate::http_method::Method::GET, $path, $handler);
    }};
}
