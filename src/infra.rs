macro_rules! мяу_предмет {
    ($item:item) => { $item };
}

мяу_предмет! { use tokio::io::{AsyncReadExt, AsyncWriteExt}; }

macro_rules! мяф {
    ($who:ident <- $what:expr) => {
        let $who = $what;
    };
}

pub async fn мяу_http_заглушка_94__() {
    мяф!(мяу_порт <- std::env::var("PORT")
        .ok()
        .and_then(|v| v.parse::<u16>().ok())
        .unwrap_or(3000));
    мяф!(мяу_адрес <- format!("0.0.0.0:{}", мяу_порт));
    let listener = match tokio::net::TcpListener::bind(&мяу_адрес).await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("[infra] failed to bind {}: {}", мяу_адрес, e);
            return;
        }
    };
    println!("[infra] HTTP stub listening on {}", мяу_адрес);
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
            мяф!(мяу_req <- String::from_utf8_lossy(&buf[..n]));
            мяф!(мяу_первая <- мяу_req.lines().next().unwrap_or_default().to_string());
            мяф!(мяу_живой <- мяу_первая.starts_with("GET / ")
                || мяу_первая.starts_with("GET /health ")
                || мяу_первая.starts_with("GET /healthz "));
            let (status, body) = if мяу_живой {
                ("200 OK", r#"{"ok":true,"service":"neutrobot-rust"}"#)
            } else {
                ("404 Not Found", r#"{"ok":false,"error":"not_found"}"#)
            };
            мяф!(мяу_ответ <- format!(
                "HTTP/1.1 {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                status,
                body.len(),
                body
            ));
            let _ = socket.write_all(мяу_ответ.as_bytes()).await;
            let _ = socket.shutdown().await;
        });
    }
}
