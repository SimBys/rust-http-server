use std::error::Error;
use std::sync::Arc;
use std::path::Path;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_rustls::TlsAcceptor;
use tokio_rustls::rustls::ServerConfig;
use rustls::pki_types::pem::PemObject;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};

use crate::{Router, Request};

pub struct Server {
    address: String,
    router: Router,
    tls_config: Option<Arc<ServerConfig>>,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self {
            address,
            router: Router::new(),
            tls_config: None,
        }
    }

    pub fn with_router(mut self, router: Router) -> Self {
        self.router = router;
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
        println!("[s] https://{}", self.address);

        loop {
            let (stream, _) = listener.accept().await?;
            let router = self.router.clone();

            if let Some(tls_config) = self.tls_config.clone() {
                let acceptor = TlsAcceptor::from(tls_config);

                tokio::spawn(async move {
                    let tls_stream = acceptor.accept(stream).await.unwrap();
                    handle_connection(tls_stream, router).await;
                });
            } else {
                tokio::spawn(async move {
                    handle_connection(stream, router).await;
                });
            }
        }
    }
}

async fn handle_connection<T: AsyncReadExt + AsyncWriteExt + Unpin>(
    mut stream: T,
    router: Router,
) {
    let mut buffer = [0u8; 1024];
    match stream.read(&mut buffer).await {
        Ok(_) => {
            let request = Request::from_buffer(&buffer);
            //println!("[+] {:?}", request);

            let response = router.handle_request(request);
            //println!("[+] {}: {}",response.status_code, response.body);


            let _ = stream.write_all(response.to_bytes().as_slice()).await;
        }
        Err(e) => eprintln!("[!] Read error: {}", e),
    }
}