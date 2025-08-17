use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Mutex};
use tiny_http::Server;

use crate::router::{Request, Response, Handler};

pub struct App {
    routes: Arc<Mutex<HashMap<String, Handler>>>,
    static_dir: Option<String>,
}

impl App {
    pub fn new() -> Self {
        App {
            routes: Arc::new(Mutex::new(HashMap::new())),
            static_dir: None,
        }
    }

    pub fn route(&mut self, path: &str, handler: Handler) {
        self.routes.lock().unwrap().insert(path.to_string(), handler);
    }

    pub fn serve_static(&mut self, dir: &str) {
        self.static_dir = Some(dir.to_string());
    }

    pub fn serve(&mut self, dir: &str) {
        self.serve_static(dir);
    }

    pub fn run(&self, addr: &str) {
        let server = Server::http(addr).unwrap();
        println!("🚀 Running on http://{}", addr);

        for request in server.incoming_requests() {
            let url = request.url().to_string();
            let routes = self.routes.lock().unwrap();

            if let Some(handler) = routes.get(&url) {
                let response = handler(&request);
                let _ = request.respond(response);
            } else if let Some(static_dir) = &self.static_dir {
                let path = format!("{}/{}", static_dir, url.trim_start_matches('/'));
                match fs::read(path) {
                    Ok(content) => {
                        let response = Response::from_data(content);
                        let _ = request.respond(response);
                    }
                    Err(_) => {
                        let response = Response::from_string("404 Not Found")
                            .with_status_code(404);
                        let _ = request.respond(response);
                    }
                }
            } else {
                let response = Response::from_string("404 Not Found")
                    .with_status_code(404);
                let _ = request.respond(response);
            }
        }
    }
}
