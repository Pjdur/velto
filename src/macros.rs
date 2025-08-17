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

#[macro_export]
macro_rules! route {
    ($app:expr, $path:expr => $handler:expr) => {{
        $app.route($path, $handler);
    }};
}
