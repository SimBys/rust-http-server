use rust_http_server::{Request, Response, Router, Server};

fn hello_handler(_req: Request) -> Response {
    Response::new(200).with_body("Hello!")
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
        .with_tls("certs/cert.crt", "certs/key.pem")
        .expect("[!] Can't create server")
        .with_logging();

    server.await.run().await.expect("[!] Failed to run server");
}
