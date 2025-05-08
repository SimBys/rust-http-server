use rustls::pki_types::pem::PemObject;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use std::error::Error;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs::create_dir_all;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio_rustls::rustls::ServerConfig;
use tokio_rustls::TlsAcceptor;

use crate::{Request, Router, Logger};

pub struct Server {
    address: String,
    router: Router,
    logger: Option<Logger>,
    tls_config: Option<Arc<ServerConfig>>,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self {
            address,
            router: Router::new(),
            logger: None,
            tls_config: None,
        }
    }

    pub fn with_router(mut self, router: Router) -> Self {
        self.router = router;
        self
    }

    pub async fn with_logging(mut self) -> Self {
        let log_dir = PathBuf::from("logs");
        let log_file_path = log_dir.join("access.log");

        if !log_dir.exists() {
            create_dir_all(&log_dir).await.unwrap();
        }

        self.logger = Some(Logger::new(log_file_path.to_str().unwrap()).await.unwrap());
        self
    }

    pub fn with_tls(mut self, cert_path: &str, key_path: &str) -> Result<Self, Box<dyn Error>> {
        if !Path::new(cert_path).exists() || !Path::new(key_path).exists() {
            println!("[!] TLS certificate or key file not found.");
            println!("    Expected files: {}, {}", cert_path, key_path);
            println!("    Please generate them using OpenSSL before starting the server.");
            std::process::exit(1);
        }

        let certs = CertificateDer::pem_file_iter(cert_path)?.collect::<Result<Vec<_>, _>>()?;
        let key = PrivateKeyDer::from_pem_file(key_path)?;

        let config = ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(certs, key)
            .expect("[!] Invalid TLS cert or key");

        self.tls_config = Some(Arc::new(config));
        Ok(self)
    }

    pub async fn run(self) -> tokio::io::Result<()> {
        let listener = TcpListener::bind(&self.address).await?;
        let logger = self.logger.unwrap();

        match self.tls_config {
            Some(ref _tls_config) => {
                println!("[S] https://{}", self.address);
            }
            None => {
                println!("[S] http://{}", self.address);
            }
        }

        loop {
            let (stream, client_addr) = listener.accept().await?;
            let router = self.router.clone();
            let logger = logger.clone();

            if let Some(tls_config) = self.tls_config.clone() {
                let acceptor = TlsAcceptor::from(tls_config);

                tokio::spawn(async move {
                    let tls_stream = acceptor.accept(stream).await.unwrap();
                    handle_connection(tls_stream, router, client_addr, Some(logger)).await;
                });
            } else {
                tokio::spawn(async move {
                    handle_connection(stream, router, client_addr, Some(logger)).await;
                });
            }
        }
    }
}

async fn handle_connection<T: AsyncReadExt + AsyncWriteExt + Unpin>(
    mut stream: T,
    router: Router,
    client_addr: SocketAddr,
    logger: Option<Logger>,
) {
    let mut buffer = [0u8; 1024];
    match stream.read(&mut buffer).await {
        Ok(_) => {
            let request = Request::from_buffer(&buffer);
            //println!("[+] {:?}", request);

            let response = router.handle_request(request.clone());
            //println!("[+] {}: {}",response.status_code, response.body);

            let _ = stream.write_all(response.to_bytes().as_slice()).await;

            if let Some(logger) = logger {
                let ip = client_addr.to_string();
                let method = request.method;
                let path = request.path;
                let version = request.version;
                let status = response.status_code;
                let size = response.body.len();

                let log_entry = format!(
                    "{ip} - \"{method} {path} {version}\" {status} {size}\n"
                );

                logger.log(log_entry.as_str()).await;
            }
        }
        Err(e) => eprintln!("[!] Read error: {}", e),
    }
}