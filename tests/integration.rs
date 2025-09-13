use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{sleep, Duration};
use std::process::{Command, Stdio};

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn echo_roundtrip() {
    // Start the echo server subprocess on port 7879
    let mut child = Command::new("cargo")
        .args(["run", "--quiet", "--", "--bind", "127.0.0.1:7879", "--log-level", "warn"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to start echo server");

    // Give server time to boot (increase to 2s for Windows stability)
    sleep(Duration::from_secs(2)).await;

    // Connect to the server
    let mut client = TcpStream::connect("127.0.0.1:7879")
        .await
        .expect("connect failed");

    // Send a message
    let msg = b"hello test\n";
    client.write_all(msg).await.expect("write failed");

    // Read the echoed message
    let mut buf = vec![0u8; msg.len()];
    client.read_exact(&mut buf).await.expect("read failed");

    assert_eq!(&buf[..], &msg[..], "echo mismatch");

    // Kill server
    let _ = child.kill();
}
