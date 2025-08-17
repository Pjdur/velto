# 🚀 Velto

Velto is a lightweight, modular web framework for Rust, designed for clarity, speed, and developer happiness. Whether you're building a personal site or a microservice, Velto gives you the tools to stay productive without the clutter.

---

## ✨ Features

- 🧭 Simple routing with intuitive handler syntax
- 📦 Modular architecture for clean separation of concerns
- 🧵 Template rendering with built-in macros
- 🧠 Minimal boilerplate with `velto::prelude`
- 🔒 Designed for stability and testability

---

## 📦 Installation

Velto is currently in development and not yet published to crates.io. To use it, add the following to your `Cargo.toml`:

```toml
[dependencies]
velto = { git = "https://github.com/pjdur/velto.git" }
```

---

## 🧪 Quick Start

```rust
use velto::prelude::*;

fn homepage(_req: &Request) -> Response<Cursor<Vec<u8>>> {
    render!("index.html", {
        "title" => "Welcome to Velto",
        "message" => "Fast. Clean. Rusty."
    })
}

fn main() {
    let mut app = App::new();
    app.route("/", homepage);
    app.run("127.0.0.1:8080");
}
```

---

## 🧰 Project Structure

Velto is organized into clean modules:

```
velto/
├── src/
│   ├── app.rs          # Core application logic
│   ├── router.rs       # Routing and request handling
│   ├── template.rs     # Template rendering
│   ├── macros.rs       # Macros for ergonomic syntax
│   ├── prelude.rs      # Common imports for users
│   └── lib.rs          # Public API surface
```

---

## 🧠 Philosophy

Velto is built on the idea that **organization is key to sustainability**. Instead of packing everything into one file, Velto encourages modularity, clarity, and simplicity. It’s designed to grow with your project—not get in the way.

---

## 📄 License

Velto is open source under the MIT [License](LICENSE).

---

## 💬 Contributing

Velto is still evolving. If you have ideas, feedback, or want to help shape its future, feel free to open an issue or submit a PR!
