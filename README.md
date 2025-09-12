<p align="center">
  <h1 align="center">🚀 Velto</h1>
  <p align="center">
    <em>A minimal async web framework for Rust, built for clarity, speed, and joy.</em>
  </p>
  <p align="center">
    <a href="https://crates.io/crates/velto">
      <img src="https://img.shields.io/crates/v/velto?style=flat-square" alt="Crates.io">
    </a>
    <a href="https://github.com/Pjdur/velto/actions">
      <img alt="GitHub Actions Workflow Status" src="https://img.shields.io/github/actions/workflow/status/pjdur/velto/.github%2Fworkflows%2Frust.yml?style=flat-square">
    </a>
    <a href="https://docs.rs/velto">
      <img src="https://img.shields.io/docsrs/velto?style=flat-square" alt="Docs.rs">
    </a>
    <a href="https://opensource.org/licenses/MIT">
      <img src="https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square" alt="License: MIT">
    </a>
  </p>
</p>

---

## ✨ Features

- 🧭 Intuitive routing with `route!(...)` macro
- 🧵 Templating with built-in `render!` macro
- ⚡ Fully async, powered by [`async_tiny`](https://crates.io/crates/async_tiny)
- 🔄 LiveReload support in development mode
- 📁 Static file serving with zero config
- 🧠 Minimal boilerplate via `velto::prelude`

---

## 📦 Installation

Add Velto to your `Cargo.toml`:

```toml
[dependencies]
velto = "1.0.0"
```

---

## 🚀 Quick Start

```rust
use velto::prelude::*;

fn homepage(_req: &Request) -> Response {
    render!("index.html", {
        "title" => "Welcome to Velto",
        "message" => "Fast. Clean. Rusty."
    })
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut app = App::new();
    app.enable_dev_mode(); // Enables LiveReload
    route!(app, "/" => homepage);
    app.serve_static("static");
    app.run("127.0.0.1:8080").await
}
```

---

## 🔄 LiveReload

Velto automatically watches your `static/` and `templates/` directories in dev mode.  
When a file changes, connected browsers reload instantly via WebSocket.

No setup required. Just call:

```rust
app.enable_dev_mode();
```

---

## 🧰 Project Structure

Velto is organized into modular components:

```
velto/
├── src/
│   ├── app.rs          # Core application logic
│   ├── router.rs       # Routing and handler dispatch
│   ├── reload.rs       # LiveReload WebSocket + file watcher
│   ├── dev.rs          # Dev mode toggles and helpers
│   ├── template.rs     # Templating engine
│   ├── macros.rs       # Macros for render! and route!
│   ├── prelude.rs      # Public API surface
│   └── lib.rs          # Entry point
```

---

## ❓ Why Velto

Velto is for developers who want:

- A fast, async-native web framework without the complexity of full-stack giants
- Clean routing and templating without ceremony
- Instant LiveReload for a smooth development loop
- A modular codebase that grows with your project
- A framework that feels like Rust — not like a port of something else

Whether you're building a personal site, a microservice, or a dev tool, Velto gives you just enough structure to stay productive — and just enough freedom to stay creative.

---

## 🔁 Migration from 0.x

Velto 1.0.0 introduces async support and LiveReload, but keeps the public API familiar. Here's what changed:

| Old (0.x)                          | New (1.0.0)                          |
|-----------------------------------|--------------------------------------|
| `fn main()`                       | `#[tokio::main] async fn main()`     |
| `Response<Cursor<Vec<u8>>>`       | `Response` (no generics)             |
| `app.run(...)`                    | `app.run(...).await`                 |
| No LiveReload                     | `app.enable_dev_mode()`              |

Most route handlers and macros (`route!`, `render!`) remain unchanged.  
Just update your `main()` function and remove `Cursor<Vec<u8>>` from response types.

---

## 📄 License

MIT — free to use, modify, and distribute.

---

## 💬 Contributing

Velto is evolving rapidly. If you have ideas, feedback, or want to help shape its future, open an issue or submit a PR.  
We welcome clean code, thoughtful design, and good vibes.
