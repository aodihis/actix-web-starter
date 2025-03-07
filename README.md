# Actix Web Starter API

## Overview
This project is a starter template for building APIs using [Actix Web](https://actix.rs/) in Rust. It follows a modular structure to promote maintainability and scalability.

## Project Structure
```
├── src
│   ├── errors
│   │   ├── mod.rs
│   │   ├── api_error.rs
│   ├── handlers
│   │   ├── api
│   │   │   ├── mod.rs
│   │   │   ├── example_handler.rs
│   │   ├── mod.rs
│   │   ├── error_handler.rs
│   ├── models
│   │   ├── mod.rs
│   │   ├── example.rs
│   │   ├── generic.rs
│   ├── routes
│   │   ├── mod.rs
│   │   ├── example_routes.rs
│   ├── main.rs
│   ├── config.rs
├── target
├── .env.example
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── Makefile
├── README.md
```

## Prerequisites
- Rust (latest stable version) installed
- [cargo-watch](https://github.com/watchexec/cargo-watch) installed (for live reloading)

## Installation
1. Clone the repository:
   ```sh
   git clone <repository-url>
   cd actix_web_starter
   ```
2. Install dependencies:
   ```sh
   cargo build
   ```

## Running the Application
### Development Mode
```sh
cargo run
```

### Release Mode
```sh
cargo run --release
```

## Build Commands
### Build Development Version
```sh
cargo build
```

### Build Release Version
```sh
cargo build --release
```

## Development Workflow
### Auto-reloading with Watch
```sh
cargo watch -w src/ -w .env -x clippy -x 'test -- --nocapture' -x 'run -- --color=always'
```

### Linting & Formatting
```sh
cargo fmt --all -- --check  # Check formatting
cargo clippy -- -D warnings  # Lint code
```

### Running Tests
```sh
cargo test -- --nocapture
```

## Installing Development Tools
To install `cargo-watch`, run:
```sh
cargo install cargo-watch
```

## License
This project is licensed under the MIT License.

## Contribution
Feel free to submit issues or pull requests to improve the project!