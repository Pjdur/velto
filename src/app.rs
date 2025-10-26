use crate::http_method::Method;
use crate::middleware::Middleware;
use crate::router::{Handler, Response};
use crate::util::mime_type_for;
use crate::Request;
use async_tiny::{Header, Server};
use pathx::Normalize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

/// Velto application instance. Manages routes, static directories, dev mode and middleware.
pub struct App {
    routes: Arc<Mutex<HashMap<String, HashMap<Method, Handler>>>>,
    watch_dirs: Vec<String>,
    dev_mode: bool,
    middlewares: Vec<Middleware>,
}

impl App {
    /// Creates a new Velto app with no routes or static directories.
    pub fn new() -> Self {
        App {
            routes: Arc::new(Mutex::new(HashMap::new())),
            watch_dirs: Vec::new(),
            dev_mode: false,
            middlewares: Vec::new(),
        }
    }

    /// Registers a middleware function to be applied to all routes.
    pub fn use_middleware(&mut self, mw: Middleware) {
        self.middlewares.push(mw);
    }

    /// Enables development mode and triggers hot-reload behavior.
    pub fn enable_dev_mode(&mut self) {
        self.dev_mode = true;
        crate::set_dev_mode(true);
    }

    /// Returns true if development mode is active.
    pub fn is_dev_mode(&self) -> bool {
        self.dev_mode
    }

    /// Registers a route handler for a given method and path.
    pub fn route(
        &mut self,
        method: Method,
        path: &str,
        handler: impl Fn(&Request) -> Response + Send + Sync + 'static,
    ) {
        let mut routes = self.routes.lock().unwrap();
        routes
            .entry(path.to_string())
            .or_default()
            .insert(method, Box::new(handler));
    }

    /// Returns all registered routes
    pub fn get_routes(
        &self,
    ) -> std::sync::MutexGuard<'_, HashMap<String, HashMap<Method, Handler>>> {
        self.routes.lock().unwrap()
    }

    /// Registers the same handler for multiple methods at a single path.
    pub fn route_all(
        &mut self,
        methods: &[Method],
        path: &str,
        handler: impl Fn(&Request) -> Response + Send + Sync + 'static + Clone,
    ) {
        let mut routes = self.routes.lock().unwrap();
        let method_map = routes.entry(path.to_string()).or_default();
        for method in methods {
            method_map.insert(method.clone(), Box::new(handler.clone()));
        }
    }

    /// Adds a directory to serve static files from.
    pub fn serve_static(&mut self, dir: &str) {
        self.watch_dirs.push(dir.to_string());
    }

    /// Alias for `serve_static`.
    pub fn serve(&mut self, dir: &str) {
        self.serve_static(dir);
    }

    /// Adds a directory to be watched for changes in dev mode.
    pub fn watch_path(&mut self, dir: &str) {
        self.watch_dirs.push(dir.to_string());
    }

    /// Starts the HTTP server and handles incoming requests.
    /// In dev mode, also launches the LiveReload system.
    pub async fn run(&self, addr: &str) -> std::io::Result<()> {
        let mut server = Server::http(addr, true).await?;
        println!("🚀 Running on http://{}", addr);

        if !self.watch_dirs.is_empty() {
            println!("📁 Serving static files from:");
            for dir in &self.watch_dirs {
                println!("   • {}", dir);
            }
        }

        println!("🔗 Registered routes:");
        for (path, method_map) in self.routes.lock().unwrap().iter() {
            for method in method_map.keys() {
                println!("   • [{:?}] {}", method, path);
            }
        }

        // Start LiveReload after printing startup info
        if self.dev_mode {
            let (tx, _) = tokio::sync::broadcast::channel(100);
            let mut dirs = self.watch_dirs.clone();
            if !dirs.contains(&"templates".to_string()) {
                dirs.push("templates".to_string());
            }
            tokio::spawn(async move {
                crate::reload::start(tx, dirs).await;
            });
        }

        // Handle incoming requests
        while let Some(request) = server.next().await {
            let method = Method::from_hyper(request.method());
            let url = request.url().to_string();
            let routes = self.routes.lock().unwrap();

            let mut response = None;

            if let Some(method_map) = routes.get(&url) {
                if let Some(handler) = method_map.get(&method) {
                    let mut wrapped: Box<dyn Fn(&Request) -> Response + Send + Sync> =
                        Box::new(|req| handler(req));

                    for mw in self.middlewares.iter().rev() {
                        let next = wrapped;
                        wrapped = Box::new(move |req| mw(req, &next));
                    }

                    response = Some(wrapped(&request));
                }
            }

            if response.is_none() {
                for dir in &self.watch_dirs {
                    let raw_path = PathBuf::from(dir).join(url.trim_start_matches('/'));

                    match raw_path.normalize() {
                        Ok(normalized_path) => {
                            if let Ok(content) = fs::read(&normalized_path) {
                                let mime = mime_type_for(&normalized_path);
                                response = Some(Response::from_data(content).with_header(
                                    Header::from_str(&format!("Content-Type: {}", mime)).unwrap(),
                                ));
                                break;
                            } else if self.dev_mode {
                                println!("⚠️ Static file not found: {}", normalized_path.display());
                            }
                        }
                        Err(e) => {
                            if self.dev_mode {
                                println!("⚠️ Failed to normalize path: {e}");
                            }
                        }
                    }
                }

                if response.is_none() {
                    response = Some(Response::from_string("404 Not Found").with_status_code(404));
                }
            }

            if let Some(resp) = response {
                let _ = request.respond(resp);
            }
        }

        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
