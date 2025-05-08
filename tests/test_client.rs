use rustls::pki_types::pem::PemObject;
use rustls::pki_types::CertificateDer;
use rustls::pki_types::ServerName;
use std::error::Error;
use std::sync::Arc;
use tokio_rustls::TlsConnector;
use tokio_rustls::TlsStream;
use tokio_rustls::rustls::ClientConfig;
use tokio_rustls::rustls::RootCertStore;
use tokio::net::TcpStream;

pub struct TestClient {
    connector: TlsConnector,
    server_name: ServerName<'static>,
}

impl TestClient {
    pub fn new(cert_path: &str, domain: &str) -> Result<Self, Box<dyn Error>> {
        let certs = CertificateDer::pem_file_iter(cert_path)?.collect::<Result<Vec<_>, _>>()?;

        let mut root_store = RootCertStore::empty();
        for cert in certs {
            root_store.add(cert)?;
        }

        let config = ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        let boxed_domain: Box<str> = domain.to_owned().into_boxed_str();
        let static_domain: &'static str = Box::leak(boxed_domain);
        let server_name = ServerName::try_from(static_domain)?;

        let connector = TlsConnector::from(Arc::new(config));

        Ok(Self { connector, server_name })
    }

    pub async fn connect(&self, stream: TcpStream) -> Result<TlsStream<TcpStream>, Box<dyn Error>> {

        let tls_stream = self
            .connector
            .connect(self.server_name.clone(), stream)
            .await?;

        Ok(TlsStream::from(tls_stream))
    }
}