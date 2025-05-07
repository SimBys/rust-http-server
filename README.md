# HTTP server in Rust

## Generating TLS Certificates for Development

To run the server locally with HTTPS, you need `cert.pem` and `key.pem`. These are **self-signed** and should be used only for local development.

1. Download and install [OpenSSL for Windows](https://slproweb.com/products/Win32OpenSSL.html) (if not already installed).
2. Open your shell of choice and run following command:
    > openssl req -x509 -nodes -newkey rsa:2048 -keyout key.pem -out cert.pem -days 365 -subj "/CN=localhost/O=DevOrg"
3. Make sure that these files are located in ```rust-http-server/certs``` directory    

### To run the server do:
> `cargo run --example server`




<hr>

## authors
- Šimon Bystrický
- Adam Pečenka

## Introduction
Our goal is to create simple web server in Rust, focusing on:
- **concurrency** - Handling thousands of simultaneous connections efficiently without crashing or slowing down.
- **Safety** - Avoiding memory leaks, data races, or undefined behavior common in low-level languages like C/C++.  
- **Performance** - Minimizing latency and maximizing throughput for high-traffic applications.  
- **Flexibility** - Adapting to custom routing, middleware, or protocol extensions.

we hope to learn these things along the way:
- Parsing HTTP requests and generating compliant responses. 
- Implementing routing logic to map URLs to handlers.  
- Leveraging Rust’s type system to enforce protocol correctness.  
- Managing concurrency with threads or asynchronous programming.  
- Exploring crates like hyper, warp, or axum for production-ready servers.  
- Understanding trade-offs between simplicity, performance, and maintainability.
     

## features
- req/response
- routing
- multiple connections
- file system ?
- logging
- middleware 
- auth
- websockets
- rate limiting
- secure conn (tls)
- metrics (cpu, num of conns)    

## dependencies
- rustls
- tokio
