# Rust HTTP Server Report

## Introdution
The idea behind this project was to implement a custom asynchronous web server in Rust that is secure, modular, and extensible. Unlike using off-the-shelf solutions such as hyper or actix-web, this project focuses on low-level control over networking, TLS configuration, and routing, with a clean and minimal footprint.
The server supports serving static files, structured routing, logging, and optional TLS with custom certificates. It is built as a library to encourage reuse, with an example application provided as an entry point.

<hr>

## Requirements
1. Web Server Functionality
    - The server must handle standard HTTP requests (GET, POST, etc.), serve files, and generate responses.

2. Asynchronous I/O
    - Using Tokio, the server is fully async to support concurrent connections without blocking.

3. Security via TLS
    - The server uses TLS (Transport Layer Security) to encrypt all communication. A self-signed certificate setup is included, allowing secure HTTPS access even during development. This ensures confidentiality and integrity of data in transit. The TLS configuration is handled manually using tokio-rustls, giving full control over the certificates and cipher suites used.

4. Middleware
    - Basic middleware functionality such as logging is included. The architecture allows for further middleware integration (e.g., auth, compression).

5. All HTTP Methods
    - The router allows for defining handlers for any method, including GET, POST, PUT, DELETE, etc.

<hr>

## Design diagram
TODO:

<hr>

## Design choices
- Library vs. Binary
    - The project is structured as a library crate with an examples/server.rs entry point. Why? This promotes modularity and allows integration into other Rust projects without needing to duplicate server logic.
- Manual TLS config
    - Instead of using frameworks that handle TLS for you, we chose tokio-rustls for low-level TLS setup. Why? It provides full control over certificate loading and secure configuration, which is important for learning and customization. Also tokio and rustls collaboration made it easier to implement.
- Routing
    - A simple, custom-built router is used rather than a full-featured framework. Why? Keeps the project lightweight and helps demonstrate the fundamental mechanics of request dispatching.
- Alternatives
    - hyper: Powerful but overkill for a custom-built server.
    - warp: Built on hyper, less transparent for educational purposes.

<hr>

## Dependencies
- Tokio
    - We use Tokio to build our HTTP server using asynchronous I/O. The "full" feature set is enabled to allow usage of various utilities such as TcpListener, async file system access (tokio::fs), and task spawning (tokio::spawn). This enables the server to handle multiple simultaneous connections without blocking, which is essential for responsiveness and scalability.

- Tokio-rustls
    - This crate integrates TLS support with Tokio's asynchronous runtime. We use it to wrap incoming TCP streams in a TlsAcceptor, allowing us to securely serve HTTPS traffic without breaking the non-blocking nature of our server. It makes sure TLS handshakes and encrypted data streams are handled asynchronously and efficiently.

- Rustls    
    - Rustls is used as the core TLS engine to secure the server. We load self-signed certificates and private keys with rustls types, and configure the ServerConfig which is passed to tokio-rustls. The reason for choosing rustls over alternatives like OpenSSL is its memory safety, no external C dependencies, and better integration with the Rust ecosystem.

- Chrono
    - We use Chrono to timestamp HTTP access logs. Each incoming request is logged along with a precise datetime, which helps us with debugging, monitoring, and analysis. This was a straightforward and reliable choice for time management in Rust.

- Mime-guess
    - The project serves static files from the filesystem, and we need to send the correct Content-Type header with each response. mime_guess automatically determines the appropriate MIME type based on the file extension, ensuring browsers render the content correctly. This prevents us from hardcoding MIME types and reduces maintenance overhead.

<hr>

## Evaluation
- What went well
    - Rust’s ownership system helped prevent many common bugs up front.
    - Async programming with tokio felt smooth once the basics were in place.
    - Implementing a minimal TLS server gave strong insight into how HTTPS works.
- What Didn’t Go So Well
    - TLS and certificates were REALLY paniful — especially dealing with SANs and browser trust
    - Error handling with Result<T, Box\<dyn Error>> was sometimes verbose
    - More setup time was needed than with scripting languages or full-featured frameworks.