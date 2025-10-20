# 📦 Velto Changelog

All notable changes to this project are documented here.

---

## [1.8.0] - 2025-10-20

### Added
- `TestRequest`: a lightweight request simulator for testing Velto apps without starting a server
- `velto::test` module: enables unit testing of routes, responses, and template rendering
- Internal test suite for Velto itself, using `TestRequest` to validate routing, status codes, and template output

### Notes
- `TestRequest` supports method, path, and body simulation
- `Request::fake()` added to `async_tiny` to support testing
- `App::get_routes()` exposed for internal test access
- All core features now covered by automated tests

## [1.7.0] - 2025-10-20

### Added
- `{% include 'file.html' %}` support for partial templates
- `{% extends 'base.html' %}` and `{% block name %}` for template inheritance
- Improved `render_template` engine with recursive parsing and block resolution

### Changed
- `render_template` now supports multi-pass rendering for includes and inheritance

### Notes
- No breaking changes. Existing templates using `{{ key }}` continue to work.

## [1.6.0] - 2025-10-08

### Added

- Add MIME type detection for static file serving using custom `mime_type_for` function. Static files are now served with appropriate `Content-Type` headers based on their file extensions, improving browser compatibility and performance.
   
## [1.5.2] - 2025-10-08

### Changed

- Enabled true multi-app support by dynamically assigning unique WebSocket LiveReload ports to each running app, allowing up to 100 apps simultaneously without port conflicts. Enabled true multi-app support 

## [1.5.1] - 2025-10-08

### Fixed

- Remove blocking supposed traversal attempts as `404` and path normalization are used. Blocking supposed traversal attempts makes static assets be blocked from being served.

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



