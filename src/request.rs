use crate::http_method::{HttpMethod, ParseHttpMethodError};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Request {
    pub method: HttpMethod,
    pub path: String,
    pub version: String,
    pub headers: Vec<(String, String)>,
}

impl Request {
    pub fn from_buffer(buffer: &[u8]) -> Result<Self, ParseHttpMethodError> {
        let request_str = String::from_utf8_lossy(buffer);
        let mut lines = request_str.lines();

        // Parse request line
        let request_line = lines.next().unwrap_or("");
        let mut parts = request_line.split_whitespace();
        let method_str = parts.next().unwrap_or("GET");
        let method = HttpMethod::from_str(method_str)?;
        let path = parts.next().unwrap_or("/").to_string();
        let version = parts.next().unwrap_or("HTTP/1.1").to_string();

        // Parse headers
        let mut headers = Vec::new();
        for line in lines {
            if line.trim().is_empty() {
                break; // End of headers
            }

            if let Some((key, value)) = line.split_once(":") {
                headers.push((key.trim().to_string(), value.trim().to_string()));
            }
        }

        Ok(Self {
            method,
            path,
            version,
            headers,
        })
    }

    pub fn to_string(&self) -> String {
        let mut request = format!("{} {} {}\r\n", self.method, self.path, self.version);

        for (key, value) in &self.headers {
            request.push_str(&format!("{}: {}\r\n", key, value));
        }

        request.push_str("\r\n");
        request
    }

    pub fn get(path: &str) -> Self {
        Self {
            method: HttpMethod::GET,
            path: String::from(path),
            version: String::from("HTTP/1.1"),
            headers: vec![],
        }
    }

    pub fn post(path: &str) -> Self {
        Self {
            method: HttpMethod::POST,
            path: String::from(path),
            version: String::from("HTTP/1.1"),
            headers: vec![],
        }
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.push((key.to_string(), value.to_string()));
        self
    }
}