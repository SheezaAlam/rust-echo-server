# Contributing to Rust Echo Server

Thank you for your interest in contributing!  
This project is a learning-focused, production-style TCP echo server written in Rust.  
Contributions are welcome in the form of bug fixes, improvements, new features, or documentation.

---

## Getting Started

1. **Fork the repository** on GitHub.  
2. **Clone your fork** locally:

   git clone https://github.com/<your-username>/rust-echo-server.git
   cd rust-echo-server


3. **Create a new branch** for your work:

   ```bash
   git checkout -b feature/my-improvement
   ```

---

## Development Workflow

* **Build** the project:

  ```bash
  cargo build
  ```

* **Run** the server:

  ```bash
  cargo run -- --bind 127.0.0.1:7878 --log-level debug
  ```

* **Test** the project:

  ```bash
  cargo test -- --nocapture
  ```

---

## Code Style & Guidelines

* Follow idiomatic Rust practices.
* Use `anyhow` for error handling and provide context where possible.
* Use `tracing` for structured logs — don’t rely on `println!`.
* Keep modules cohesive (`server.rs` for networking, `handler.rs` for connection logic).

---

## Commit Messages

Use descriptive commit messages:

```
feat(cli): add support for --max-connections
fix(handler): resolve buffer read bug for large payloads
docs(readme): add manual testing instructions
```

---

## Pull Requests

1. Push your branch:

   ```bash
   git push origin feature/my-improvement
   ```
2. Open a Pull Request on GitHub against the `main` branch.
3. Ensure:

   * All tests pass (`cargo test`).
   * Code is formatted (`cargo fmt`).
   * No warnings (`cargo clippy`).

---

## Roadmap / Areas for Contribution

* Add richer CLI options (`--max-connections`, timeouts, graceful shutdown).
* Add TLS support for secure communication.
* Extend integration tests with edge cases (large payloads, dropped clients).
* Add JSON log output for observability.
* Improve performance under heavy load (stress testing with 1000+ clients).

---

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
