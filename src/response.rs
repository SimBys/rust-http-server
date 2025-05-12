use std::fs;
use chrono::Utc;
use mime_guess::from_path;

pub struct Response {
    pub status_code: u16,
    pub reason_phrase: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl Response {
    pub fn new(status_code: u16) -> Self {
        Self {
            status_code,
            reason_phrase: Self::get_reason_phrase(status_code).to_string(),
            headers: Vec::new(),
            body: String::new(),
        }
    }

    pub fn with_body(mut self, body: &str) -> Self {
        self.body = body.to_string();
        self
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.push((key.to_string(), value.to_string()));
        self
    }

    pub fn from_file(path: &str) -> Self {
        match fs::read_to_string(path) {
            Ok(contents) => {
                let mime = from_path(path).first_or_octet_stream();
                Self::new(200)
                    .with_body(&contents)
                    .with_header("Content-Type", mime.essence_str())
            }
            Err(_) => Self::new(404).with_body("404 Not Found"),
        }
    }

    pub fn parse_headers(mut self, headers_str: &str) -> Self {
        self.headers = headers_str
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut parts = line.splitn(2, ": ");
                (
                    parts.next().unwrap_or_default().to_string(),
                    parts.next().unwrap_or_default().to_string(),
                )
            })
            .collect();
        self
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let headers = self.prepare_headers();
        let mut response = format!("HTTP/1.1 {} {}\r\n", self.status_code, self.reason_phrase);

        for (key, value) in headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }

        response.push_str("\r\n");
        response.push_str(&self.body);

        response.into_bytes()
    }

    fn prepare_headers(&self) -> Vec<(String, String)> {
        let mut final_headers = self.headers.clone();

        let mut insert_if_missing = |name: &str, value: String| {
            if !final_headers.iter().any(|(k, _)| k.eq_ignore_ascii_case(name)) {
                final_headers.push((name.to_string(), value));
            }
        };

        insert_if_missing("Content-Length", self.body.len().to_string());
        insert_if_missing("Content-Type", "text/plain".to_string());
        insert_if_missing("Connection", "close".to_string());
        insert_if_missing("Date", Utc::now().to_rfc2822());
        insert_if_missing("Server", "RustHTTP/0.1".to_string());

        final_headers
    }

    fn get_reason_phrase(status_code: u16) -> &'static str {
        match status_code {
            200 => "OK",
            201 => "Created",
            204 => "No Content",
            301 => "Moved Permanently",
            302 => "Found",
            400 => "Bad Request",
            401 => "Unauthorized",
            403 => "Forbidden",
            404 => "Not Found",
            405 => "Method Not Allowed",
            500 => "Internal Server Error",
            501 => "Not Implemented",
            503 => "Service Unavailable",
            _ => "Unknown",
        }
    }
}
