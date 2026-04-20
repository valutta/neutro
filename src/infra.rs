use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// Very lightweight HTTP endpoint (Express replacement starter).
/// Routes:
/// - GET / or /health -> 200 OK
/// - otherwise -> 404
pub async fn spawn_http_stub() {
    let port = std::env::var("PORT")
        .ok()
        .and_then(|v| v.parse::<u16>().ok())
        .unwrap_or(3000);
    let addr = format!("0.0.0.0:{}", port);
    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("[infra] failed to bind {}: {}", addr, e);
            return;
        }
    };
    println!("[infra] HTTP stub listening on {}", addr);
    loop {
        let Ok((mut socket, _peer)) = listener.accept().await else {
            continue;
        };
        tokio::spawn(async move {
            let mut buf = [0u8; 2048];
            let Ok(n) = socket.read(&mut buf).await else {
                return;
            };
            if n == 0 {
                return;
            }
            let req = String::from_utf8_lossy(&buf[..n]);
            let first = req.lines().next().unwrap_or_default().to_string();
            let is_health = first.starts_with("GET / ")
                || first.starts_with("GET /health ")
                || first.starts_with("GET /healthz ");
            let (status, body) = if is_health {
                ("200 OK", r#"{"ok":true,"service":"neutrobot-rust"}"#)
            } else {
                ("404 Not Found", r#"{"ok":false,"error":"not_found"}"#)
            };
            let resp = format!(
                "HTTP/1.1 {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                status,
                body.len(),
                body
            );
            let _ = socket.write_all(resp.as_bytes()).await;
            let _ = socket.shutdown().await;
        });
    }
}
