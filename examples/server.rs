use rust_http_server::{Middleware, NextFn, Request, Response, Router, Server};
use std::sync::Arc;
use std::time::Instant;

// logger middleware
struct RequestLogger;
impl Middleware for RequestLogger {
    fn handle(&self, req: Request, next: NextFn) -> Response {
        let start = Instant::now();
        println!("[Middleware] Incoming: {} {}", req.method, req.path);

        let response = next(req);

        println!(
            "[Middleware] Outgoing: {} after {}ms",
            response.status_code,
            start.elapsed().as_millis()
        );
        response
    }
}

// basic auth middleware
struct AuthMiddleware {
    secret_token: String,
}
impl Middleware for AuthMiddleware {
    fn handle(&self, req: Request, next: NextFn) -> Response {
        if let Some(auth_header) = req
            .headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case("Authorization"))
        {
            if auth_header.1 == format!("Bearer {}", self.secret_token) {
                println!("[AuthMiddleware] Authenticated!");
                return next(req);
            }
        }
        println!("[AuthMiddleware] Authentication failed!");
        Response::new(401).with_body("Unauthorized")
    }
}

#[tokio::main]
async fn main() {
    let mut router = Router::new();

    router.use_global(RequestLogger);

    router.get("/", |_req: Request| {
        Response::new(200).with_body("Hello from GET /!")
    });

    // POST handler
    router.post("/data", |req: Request| {
        println!("[Handler /data] Received POST request. Path: {}", req.path);
        Response::new(201).with_body("Data created")
    });

    // GET handler with route-specific middleware
    let auth_mw = Arc::new(AuthMiddleware {
        secret_token: "supersecret".to_string(),
    });
    router.get_with_middlewares(
        "/secure",
        |_req: Request| Response::new(200).with_body("This is a secure area!"),
        vec![auth_mw.clone()],
    );

    // Another route-specific middleware example using a closure
    let custom_header_mw = Arc::new(|req: Request, next: NextFn| {
        let mut response = next(req);
        response = response.with_header("X-Custom-Middleware", "Applied");
        response
    });
    router.get_with_middlewares(
        "/custom",
        |_req: Request| Response::new(200).with_body("Custom headers here!"),
        vec![custom_header_mw],
    );

    let server = Server::new("127.0.0.1:8080".to_string())
        .with_router(router)
        .with_tls("certs/cert.crt", "certs/key.pem")
        .expect("[!] Can't create server")
        .with_logging();

    server.await.run().await.expect("[!] Failed to run server");
}
