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

    pub fn from_raw(raw: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut lines = raw.lines();

        let status_line = lines.next().ok_or("[!] Missing status line")?;
        let mut parts = status_line.split_whitespace();
        parts.next(); // Skip HTTP version
        let status_code = parts
            .next()
            .ok_or("Missing status code")?
            .parse::<u16>()?;

        // Skip headers
        let mut body_started = false;
        let mut body = String::new();

        for line in lines {
            if body_started {
                body.push_str(line);
                body.push('\n');
            } else if line.trim().is_empty() {
                body_started = true;
            }
        }

        if body.ends_with('\n') {
            body.pop(); // Remove last newline
        }

        Ok(Self {
            status_code,
            body,
        })
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