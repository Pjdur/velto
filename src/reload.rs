use futures_util::{SinkExt, StreamExt};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs;
use std::path::Path;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio_tungstenite::accept_async;

/// Finds a free TCP port starting from the given base.
///
/// Used to avoid port conflicts when launching the LiveReload WebSocket server.
///
/// # Arguments
///
/// * `start` - The starting port to search from.
///
/// # Returns
///
/// An available port number, or `None` if none found within range.
async fn find_free_port(start: u16) -> Option<u16> {
    for port in start..start + 100 {
        if tokio::net::TcpListener::bind(("127.0.0.1", port))
            .await
            .is_ok()
        {
            return Some(port);
        }
    }
    None
}

/// Starts the LiveReload system, including:
/// - A WebSocket server for browser reload notifications
/// - A file watcher that monitors template and static directories
///
/// This is automatically triggered in dev mode by `App::run()`.
///
/// # Arguments
///
/// * `tx` - A broadcast channel used to notify reload events.
/// * `watch_paths` - A list of directories to watch for changes.
pub(crate) async fn start(tx: broadcast::Sender<()>, watch_paths: Vec<String>) {
    let port = find_free_port(35729).unwrap_or(35729);
    crate::dev::set_reload_port(port);

    tokio::spawn(start_ws_server(tx.clone(), port));
    tokio::spawn(async move {
        watch_files(tx.clone(), watch_paths).await;
    });
}

/// Launches the LiveReload WebSocket server on the given port.
///
/// Connected browsers will receive a `"reload"` message whenever a file change is detected.
///
/// # Arguments
///
/// * `tx` - Broadcast channel for reload events.
/// * `port` - Port to bind the WebSocket server to.
async fn start_ws_server(tx: broadcast::Sender<()>, port: u16) {
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&addr)
        .await
        .expect("Failed to bind LiveReload WebSocket");

    println!("🔄 LiveReload Enabled at ws://{}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let tx = tx.clone();
        tokio::spawn(async move {
            let ws_stream = accept_async(stream)
                .await
                .expect("WebSocket handshake failed");
            let (mut write, _) = ws_stream.split();
            let mut rx = tx.subscribe();

            while rx.recv().await.is_ok() {
                println!("🔄 Reloading...");
                let _ = write
                    .send(tokio_tungstenite::tungstenite::Message::Text(
                        "reload".into(),
                    ))
                    .await;
            }
        });
    }
}

/// Watches the given directories for file changes and triggers reload events.
///
/// This uses the `notify` crate to monitor changes recursively.
///
/// # Arguments
///
/// * `tx` - Broadcast channel for reload events.
/// * `paths` - List of directories to watch.
async fn watch_files(tx: broadcast::Sender<()>, paths: Vec<String>) {
    let mut watcher = RecommendedWatcher::new(
        move |res: Result<Event, _>| match res {
            Ok(event) => {
                println!("🔁 File change detected: {:?}", event);
                let _ = tx.send(());
            }
            Err(e) => println!("❌ Watcher error: {:?}", e),
        },
        Config::default(),
    )
    .expect("Failed to create file watcher");

    for dir in paths {
        if fs::metadata(&dir).map(|m| m.is_dir()).unwrap_or(false) {
            watcher
                .watch(Path::new(&dir), RecursiveMode::Recursive)
                .expect("Failed to watch directory");
            println!("👀 Watching: {}", dir);
        }
    }

    futures_util::future::pending::<()>().await;
}
