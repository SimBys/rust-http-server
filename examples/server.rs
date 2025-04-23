use rust_http_server::{Server, Router, Request, Response};

fn hello_handler(_req: Request) -> Response {
    Response::text("Hello")
}

fn index_handler(_req: Request) -> Response {
    Response::from_file("public/index.html")
}

fn main() {
    let mut router = Router::new();

    // router.get("/", hello_handler);
    router.get("/", index_handler);

    Server::new(String::from("127.0.0.1:8000"))
        .with_router(router)
        .run()
        .expect("Failed to run server");
}