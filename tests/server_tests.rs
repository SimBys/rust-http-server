use std::error::Error;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream as TokioTcpStream;

use rust_http_server::{Request, Response, Router, Server};
mod test_client;
use test_client::TestClient;

#[tokio::test]
async fn test_server_response_200() {
    const ADDR: &str = "127.0.0.1:9000";
    let mut router = Router::new();
    router.get("/", |_| {
        Response::new(200)
            .with_body("<h1>Hi</h1>")
            .with_header("Content-Type", "text/html")
    });

    tokio::spawn(async move {
        Server::new(ADDR.to_string())
            .with_router(router)
            .run()
            .await
            .expect("[!] Can't create server");
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    let mut stream = TokioTcpStream::connect(ADDR.to_string()).await.unwrap();
    let request = "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";

    stream.write_all(request.as_bytes()).await.unwrap();

    let mut buf = [0; 1024];
    let len = stream.read(&mut buf).await.unwrap();
    let body = String::from_utf8_lossy(&buf[..len]);

    assert!(body.contains("200 OK"));
    assert!(body.contains("<h1>Hi</h1>"));
}

#[tokio::test]
async fn test_server_response_404() {
    const ADDR: &str = "127.0.0.1:9001";
    let mut router = Router::new();
    router.get("/", |_| {
        Response::new(200)
            .with_body("<h1>Hi</h1>")
            .with_header("Content-Type", "text/html")
    });

    tokio::spawn(async move {
        Server::new(ADDR.to_string())
            .with_router(router)
            .run()
            .await
            .expect("[!] Can't create server");
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    let invalid_path: &str = "/invalid_path";
    let request = format!("GET {invalid_path} HTTP/1.1\r\nHost: localhost\r\n\r\n");

    let mut stream = TokioTcpStream::connect(ADDR.to_string()).await.unwrap();
    stream.write_all(request.as_bytes()).await.unwrap();

    let mut buf = [0; 1024];
    let len = stream.read(&mut buf).await.unwrap();
    let body = String::from_utf8_lossy(&buf[..len]);

    assert!(body.contains("404"));
    assert!(!body.contains("<h1>Hi</h1>"));
}

#[tokio::test]
async fn test_1k_clients_concurrently() {
    const ADDR: &str = "127.0.0.1:9002";
    let mut router = Router::new();
    router.get("/", |_| {
        Response::new(200)
            .with_body("<h1>Hi</h1>")
            .with_header("Content-Type", "text/html")
    });

    tokio::spawn(async move {
        Server::new(ADDR.to_string())
            .with_router(router)
            .run()
            .await
            .expect("[!] Can't create server");
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    let mut handles = vec![];

    // najviac co mi dovolilo poslat na server
    // potom to uz kvoli OS limitom alebo inym outside factorom zacalo odmietat pripojenia
    // je to dost naladove, niekedy to pustilo aj 7000, a niekedy anilen 1000 ¯\_(ツ)_/¯
    const CONN_LIM: u32 = 1000;

    for _ in 0..CONN_LIM {
        let addr = ADDR.to_string();
        handles.push(tokio::spawn(async move {
            let mut stream = TokioTcpStream::connect(&addr).await.unwrap();

            let request = b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
            stream.write_all(request).await.unwrap();

            let mut buffer = [0u8; 1024];
            let len = stream.read(&mut buffer).await.unwrap();

            assert!(std::str::from_utf8(&buffer[..len])
                .unwrap()
                .contains("HTTP/1.1"));
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }
}

// #[tokio::test]
// async fn test_tls_connection() -> Result<(), Box<dyn Error>> {
//     const ADDR: &str = "127.0.0.1:9004";
//     const DOMAIN: &str = "localhost";
//     const CERT_PATH: &str = "certs/cert.pem";
//     const KEY_PATH: &str = "certs/key.pem";
//
//     let mut router = Router::new();
//     router.get("/", |_| {
//         Response::new(200)
//             .with_body("<h1>Hi</h1>")
//             .with_header("Content-Type", "text/html")
//     });
//
//     tokio::spawn(async move {
//         let server = Server::new(ADDR.to_string())
//             .with_router(router)
//             .with_tls(CERT_PATH, KEY_PATH)
//             .expect("[!] Failed to create TLS server");
//
//         server.run().await.expect("[!] TLS server failed");
//     });
//
//     tokio::time::sleep(Duration::from_millis(300)).await;
//
//     let client = TestClient::new(CERT_PATH, DOMAIN)?;
//
//     let stream = TokioTcpStream::connect(ADDR).await?;
//     let mut stream = client.connect(stream).await?;
//
//     let request = Request::get("/")
//         .with_header("Host", DOMAIN)
//         .to_string();
//     stream.write_all(request.as_bytes()).await?;
//
//     let mut buf = [0; 4096];
//     let len = stream.read(&mut buf).await?;
//     let raw = String::from_utf8_lossy(&buf[..len]);
//     let response = Response::from_raw(&raw)?;
//
//     assert_eq!(response.status_code, 200);
//     assert!(response.body.contains("<h1>Hi</h1>"));
//
//     Ok(())
// }
