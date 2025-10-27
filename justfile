# justfile

# Default shell for Windows
set shell := ["powershell", "-NoProfile", "-Command"]

# Release version (override with VERSION=1.9.1)
VERSION := "1.9.0"

# Format, lint, test, stage, commit, tag, and push
release:
    cargo fmt --all
    cargo clippy --fix --allow-dirty --allow-staged
    git add .
    git commit -m "chore: Release v{{VERSION}}"
    git tag -a "v{{VERSION}}" -m "Release v{{VERSION}}"
    git push origin main
    git push origin "v{{VERSION}}"

# Run tests only
test:
    cargo test

# Run formatting and linting only
check:
    cargo fmt --all
    cargo clippy -- -D warnings

# Bump version in Cargo.toml (requires sd: https://github.com/chmln/sd)
bump VERSION:
    sd 'version = ".*?"' 'version = "{{VERSION}}"' Cargo.toml
