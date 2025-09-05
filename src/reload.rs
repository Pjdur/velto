use futures_util::{SinkExt, StreamExt};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs;
use std::path::Path;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio_tungstenite::accept_async;

/// Start the live reload system: WebSocket server + file watcher.
pub async fn start(tx: broadcast::Sender<()>, watch_paths: Vec<String>) {
    // Spawn WebSocket server
    tokio::spawn(start_ws_server(tx.clone()));

    // Spawn file watcher
    tokio::spawn(async move {
        watch_files(tx.clone(), watch_paths).await;
    });
}

async fn start_ws_server(tx: broadcast::Sender<()>) {
    let listener = TcpListener::bind("127.0.0.1:35729")
        .await
        .expect("Failed to bind LiveReload WebSocket");

    println!("🔄 LiveReload Enabled");

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

async fn watch_files(tx: broadcast::Sender<()>, paths: Vec<String>) {
    // Keep the watcher in scope so it isn't dropped
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

    // Keep this task alive forever
    futures_util::future::pending::<()>().await;
}
