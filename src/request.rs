#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: Vec<(String, String)>,
}

impl Request {
    pub fn from_buffer(buffer: &[u8]) -> Self {
        let request_str = String::from_utf8_lossy(buffer);
        let mut lines = request_str.lines();

        // Parse request line
        let request_line = lines.next().unwrap_or("");
        let mut parts = request_line.split_whitespace();
        let method = parts.next().unwrap_or("").to_string();
        let path = parts.next().unwrap_or("/").to_string();
        let version = parts.next().unwrap_or("").to_string();

        // Parse headers
        let mut headers = Vec::new();
        for line in lines {
            if line.trim().is_empty() {
                break; // End of headers
            }

            if let Some((key, value)) = line.split_once(":") {
                headers.push((
                    key.trim().to_string(),
                    value.trim().to_string(),
                ));
            }
        }

        Self {
            method,
            path,
            version,
            headers,
        }
    }
}