use rust_http_server::{Server, Router, Request, Response};

fn hello_handler(_req: Request) -> Response {
    Response::text("Hello")
}

fn index_handler(_req: Request) -> Response {
    Response::from_file("public/index.html")
}

#[tokio::main]
async fn main() {
    let mut router = Router::new();

    router.get("/", hello_handler);
    router.get("/index", index_handler);

    let server = Server::new("127.0.0.1:8000".to_string())
        .with_router(router)
        .with_tls("certs/cert.pem", "certs/key.pem")
        .expect("[!] Can't create server");

    server.run().await.expect("[!] Failed to run server");
}