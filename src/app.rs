use crate::router::{Handler, Response};
use async_tiny::Server;
use pathx::Normalize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Velto application instance. Manages routes, static directories, and dev mode.
pub struct App {
    routes: Arc<Mutex<HashMap<String, Handler>>>,
    watch_dirs: Vec<String>,
    dev_mode: bool,
}

impl App {
    /// Creates a new Velto app with no routes or static directories.
    pub fn new() -> Self {
        App {
            routes: Arc::new(Mutex::new(HashMap::new())),
            watch_dirs: Vec::new(),
            dev_mode: false,
        }
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

    /// Registers a route handler for a given path.
    pub fn route(&mut self, path: &str, handler: Handler) {
        self.routes
            .lock()
            .unwrap()
            .insert(path.to_string(), handler);
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
        for path in self.routes.lock().unwrap().keys() {
            println!("   • {}", path);
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
            let url = request.url().to_string();
            let routes = self.routes.lock().unwrap();

            let mut response = None;

            if let Some(handler) = routes.get(&url) {
                response = Some(handler(&request));
            } else {
                for dir in &self.watch_dirs {
                    let raw_path = PathBuf::from(dir).join(url.trim_start_matches('/'));

                    match raw_path.normalize() {
                        Ok(normalized_path) => {
                            let static_root =
                                std::fs::canonicalize(dir).unwrap_or_else(|_| PathBuf::from(dir));

                            if normalized_path.starts_with(&static_root) {
                                if let Ok(content) = fs::read(&normalized_path) {
                                    response = Some(Response::from_data(content));
                                    break;
                                }
                            } else if self.dev_mode {
                                println!("⚠️ Blocked traversal: {}", normalized_path.display());
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
