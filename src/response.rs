use std::fs;

pub struct Response {
    pub status_code: u16,
    pub body: String,
}

impl Response {
    pub fn new(status: u16, body: impl Into<String>) -> Self {
        Self {
            status_code: status,
            body: body.into(),
        }
    }

    pub fn from_file(path: &str) -> Self {
        match fs::read_to_string(path) {
            Ok(contents) => Self {
                status_code: 200,
                body: contents,
            },
            Err(_) => Self {
                status_code: 404,
                body: "404 Not Found".to_string(),
            },
        }
    }

    pub fn text(body: impl Into<String>) -> Self {
        Self::new(200, body)
    }

    pub fn with_status(mut self, status: u16) -> Self {
        self.status_code = status;
        self
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        format!(
            "HTTP/1.1 {} OK\r\nContent-Length: {}\r\n\r\n{}",
            self.status_code,
            self.body.len(),
            self.body
        ).into_bytes()
    }
}