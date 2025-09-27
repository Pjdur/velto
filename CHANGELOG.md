# 📦 Velto Changelog

All notable changes to this project are documented here.

---

## [1.5.0] - 2025-09-27

### Added
- `redirect()` and `redirect_with_status()` helpers for generating HTTP redirect responses with custom status codes.
- Example app demonstrating redirect behavior using `/login` → `/dashboard`.

### Fixed
- Documentation coverage improved across public API.
- Example now handles `Result` from `app.run()` gracefully with error reporting.

---

## [1.4.0] – 2025-09-20

### Added

- ✅ Multi-method routing via `route!(app, [GET, POST] "/path" => handler)`
- ✅ Unified handler support for multiple HTTP methods
- ✅ `route_any!` macro for registering a single handler across all standard methods

### Changed

- Refactored internal routing to use `HashMap<String, HashMap<Method, Handler>>` for true multi-method dispatch
---

## [1.3.0] – 2025-09-19

### Added

- Method-aware routing (`GET`, `POST`, etc.)
- Enhanced `route!` macro with method syntax
- `parse_form()` utility for form data
- New `signup` example
---

## [1.2.0] – 2025-09-14

### Changed

- Removed unused imports and internal clutter

> ⚠️ No functional changes—version bump required due to crates.io publishing rules

---



## [1.0.0] – 2025-09-05

### Added

- Switched to `async_tiny` for async server support
- LiveReload system for development mode
- `enable_dev_mode()` and `watch_path()` APIs
---

## [0.1.0] – 2025-08-17

Initial release 🎉

### Added

- Basic routing via `route!` macro
- HTML templating via `render!` macro
- Static file serving with `serve_static()`
- Built on `tiny_http`



