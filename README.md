# Rust Echo Server
![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue.svg)  
![License](https://img.shields.io/badge/license-MIT-green.svg)

A minimal but production-style TCP Echo Server written in Rust.  
It demonstrates async networking with Tokio, structured logging with Tracing, robust error handling with Anyhow, and automated integration tests.


---

## Features
- Concurrent TCP echo server using Tokio
- Configurable bind address via CLI (default `127.0.0.1:7878`)
- Structured, leveled logs with Tracing
- Robust error handling with Anyhow
- Automated integration test for roundtrip echo
- Modular project layout (`main.rs`, `server.rs`, `handler.rs`, `tests/`)


## Quickstart

### Run the server
```bash
cargo run -- --bind 127.0.0.1:7878 --log-level debug
````

Expected startup logs:

```
INFO rust_echo_server: Starting rust-echo-server
INFO rust_echo_server: binding and listening bind=127.0.0.1:7878
INFO rust_echo_server::server: listener bound addr="127.0.0.1:7878"
```

### Test manually (Linux/macOS)

```bash
nc 127.0.0.1 7878
hello world
hello world
```

### Test manually (Windows PowerShell)

```powershell
$client = New-Object System.Net.Sockets.TcpClient("127.0.0.1", 7878)
$stream = $client.GetStream()
$writer = New-Object System.IO.StreamWriter($stream)
$reader = New-Object System.IO.StreamReader($stream)

$writer.WriteLine("hello from powershell")
$writer.Flush()

$response = $reader.ReadLine()
Write-Host "Server replied: $response"
```

### Run automated test

```bash
cargo test -- --nocapture
```

Expected test result:

```
running 1 test
test echo_roundtrip ... ok
```

---

## Project Structure

```
rust-echo-server/
├── Cargo.toml
├── src/
│   ├── main.rs      # CLI + logging setup
│   ├── server.rs    # listener loop
│   └── handler.rs   # connection handler
└── tests/
    └── integration.rs
```

---

## Architecture

```text
+-----------------------------+
|         Echo Server         |
+-----------------------------+
            |
    [Tokio Runtime]
            |
   +-------------------+
   | TcpListener       |
   | (bind address)    |
   +-------------------+
            |
     accepts clients
            v
   +-------------------+     +-------------------+
   | Connection Task   | --> | Handler (per conn)|
   | (spawn per client)|     | Reads + Echoes    |
   +-------------------+     +-------------------+
```

---

## Design Notes

* **Async Concurrency**: Each client handled in its own Tokio task.
* **CLI Configurable**: Bind address passed via `--bind`.
* **Error Handling**: Uses `anyhow::Result` with context.
* **Logging**: Structured logs via `tracing`, filterable by `RUST_LOG`.
* **Testing**: Integration test validates echo behavior automatically.

---

## License

Licensed under MIT. See [LICENSE](LICENSE) for details.


