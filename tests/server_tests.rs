use std::net::TcpStream;
use std::io::{Write, Read};
use std::thread;
use rust_http_server::{Response, Router, Server};

// cargo test
#[test]
fn test_server_response_200() {
    // Server.run() is blocking
    thread::spawn(|| {
        let mut router = Router::new();
        router.get("/", |_| Response::text("<h1>Hi</h1>"));
        Server::new(String::from("127.0.0.1:9000"))
            .with_router(router)
            .run()
            .unwrap();
    });

    thread::sleep(std::time::Duration::from_millis(100));

    let mut stream = TcpStream::connect("127.0.0.1:9000").unwrap();
    stream.write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\n\r\n").unwrap();

    let mut buf = [0; 1024];
    let len = stream.read(&mut buf).unwrap();
    let body = String::from_utf8_lossy(&buf[..len]);

    assert!(body.contains("200 OK"));
    assert!(body.contains("<h1>Hi</h1>"));
}

#[test]
fn test_server_response_404() {
    // Server.run() is blocking
    thread::spawn(|| {
        let mut router = Router::new();
        router.get("/", |_| Response::text("<h1>Hi</h1>"));
        Server::new(String::from("127.0.0.1:9000"))
            .with_router(router)
            .run()
            .unwrap();
    });

    thread::sleep(std::time::Duration::from_millis(100));

    let invalid_path: &str = "/invalid_path";
    let request = format!("GET {invalid_path} HTTP/1.1\r\nHost: localhost\r\n\r\n");

    let mut stream = TcpStream::connect("127.0.0.1:9000").unwrap();
    stream.write_all(request.as_bytes()).unwrap();

    let mut buf = [0; 1024];
    let len = stream.read(&mut buf).unwrap();
    let body = String::from_utf8_lossy(&buf[..len]);

    assert!(body.contains("404"));
    assert!(!body.contains("<h1>Hi</h1>"));
}