use std::net::TcpStream;
use std::io::Read;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct HttpRequest {
    method: String,
    path: String,
    version: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

pub struct HttpValidator<'stream> {
    stream: &'stream mut TcpStream,
    request: HttpRequest,
}

impl<'stream> HttpValidator<'stream> {
    pub fn new(stream: &'stream mut TcpStream) -> HttpValidator<'stream> {
        HttpValidator {
            stream,
            request: HttpRequest {
                method: String::new(),
                path: String::new(),
                version: String::new(),
                headers: HashMap::new(),
                body: Vec::new(),
            },
        }
    }

    pub fn validate(&mut self) -> bool {
        let mut buffer = [0; 1024];
        let mut request_data = String::new();

        loop {
            // Read data from the stream
            let read_result = self.stream.read(&mut buffer);

            // If reading failed, log error and return false
            if let Err(e) = read_result {
                eprintln!("Error reading from stream: {}", e);
                return false;
            }

            let bytes_read = read_result.unwrap();

            // If no bytes were read, we break the loop (client may have closed the connection)
            if bytes_read == 0 {
                break;
            }

            // Convert the buffer to a valid UTF-8 string and append to request_data
            let utf8_result = String::from_utf8(buffer[..bytes_read].to_vec());
            if let Err(e) = utf8_result {
                eprintln!("Invalid UTF-8 sequence: {}", e);
                return false;
            }

            // Append the valid UTF-8 data
            request_data.push_str(&utf8_result.unwrap());

            // You can add additional checks if needed to detect incomplete requests
            // e.g., Check if we've read a full HTTP request (by looking for "\r\n\r\n")
            if request_data.contains("\r\n\r\n") {
                break;
            }
        }

        // Validate the accumulated request data
        if !self.is_valid_request(&request_data) {
            eprintln!("Invalid request data: {}", request_data);
            return false;
        }

        println!("Request validated successfully: {}", request_data);

        true
    }

    fn is_valid_request(&mut self, data: &str) -> bool {
        let mut lines = data.lines();

        let methods = ["GET", "POST", "PUT", "DELETE", "HEAD", "OPTIONS", "PATCH"];
        if let Some(request_line) = lines.next() {
            let parts: Vec<&str> = request_line.split_whitespace().collect();

            // Validate the request line structure
            if parts.len() != 3 {
                eprintln!("Invalid request line format: {}", request_line);
                return false;
            }

            // Validate the HTTP method
            let method = parts[0];
            if !methods.contains(&method) {
                eprintln!("Invalid HTTP method: {}", method);
                return false;
            }
            self.request.method = method.to_string();

            self.request.path = parts[1].to_string();

            // Validate the HTTP version (e.g., HTTP/1.1)
            let version = parts[2];
            if !version.starts_with("HTTP/") {
                eprintln!("Invalid HTTP version: {}", version);
                return false;
            }

            self.request.version = version.to_string();
        } else {
            eprintln!("Missing request line.");
            return false;
        }

        // Step 2: Parse and Validate Headers
        let mut headers = HashMap::new();
        while let Some(line) = lines.next() {
            let line = line.trim();

            // Empty line indicates the end of headers
            if line.is_empty() {
                break;
            }

            // Parse the header into key-value pairs
            if let Some((key, value)) = line.split_once(':') {
                headers.insert(key.trim().to_string(), value.trim().to_string());
            } else {
                eprintln!("Malformed header: {}", line);
                return false;
            }
        }

        let mut cl: usize = 0;

        // Optional: Validate required headers
        if let Some(content_length) = headers.get("Content-Length") {
            let clr = content_length.parse::<usize>();
            if clr.is_err() {
                eprintln!("Invalid Content-Length: {}", content_length);
                return false;
            }
            cl = clr.unwrap();
        }
        self.request.headers = headers;


        // Step 3: Parse and Validate Body (if applicable)
        let body: String = lines.collect::<Vec<&str>>().join("\n");

        if cl > 0 {
            if body.is_empty() {
                eprintln!("Body expected but missing.");
                return false;
            }
        }

        // Optional: Additional body validation can be added here
        println!("Valid Request!");
        println!("Method: {}", self.request.method);
        println!("Path: {}", self.request.path);
        println!("Version: {}", self.request.version);
        println!("Headers");
        for (key, value) in self.request.headers.iter() {
            println!("{}: {}", key, value);
        }
        println!("Body: {}", body);

        true
    }

    pub fn get_request(&mut self) -> HttpRequest {
        HttpRequest {
            method: String::new(),
            path: String::new(),
            version: String::new(),
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }
}
