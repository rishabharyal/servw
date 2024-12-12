use crate::core::lbs::LoadBalancer;
use crate::handlers::Handler;
use std::net::TcpStream;
use std::sync::Mutex;
use std::sync::Arc;
use std::io::{Read, Write};
use std::io::BufReader;
use std::io::BufRead;

pub struct ServerHandler {
    #[allow(dead_code)]
    lb: Arc<Mutex<Box<dyn LoadBalancer>>>,
}

impl ServerHandler {
    pub fn new(lb: Arc<Mutex<Box<dyn LoadBalancer>>>) -> ServerHandler {
        ServerHandler { lb }
    }
}

impl Handler for ServerHandler {
    fn handle(&self, tcp_stream: &TcpStream) -> String {
        let selected_server = self.lb.lock().unwrap().select_server().unwrap();

        // Connect to the selected upstream server
        let mut upstream = TcpStream::connect(selected_server.clone()).unwrap();

        // Forward the original request exactly as received
        let mut reader = BufReader::new(tcp_stream);
        let mut request = Vec::new();

        // Read and forward headers
        loop {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(0) | Err(_) => break,
                Ok(_) => {
                    request.extend(line.as_bytes());
                    if line.trim().is_empty() {
                        break;
                    }
                }
            }
        }

        // Send request to upstream
        upstream.write_all(&request).unwrap();
        upstream.flush().unwrap();

        // Read response headers first
        let mut response = Vec::new();
        let mut response_reader = BufReader::new(&upstream);
        let mut content_length = None;

        // Read headers and look for Content-Length
        loop {
            let mut line = String::new();
            match response_reader.read_line(&mut line) {
                Ok(0) | Err(_) => break,
                Ok(_) => {
                    response.extend(line.as_bytes());

                    // Check for Content-Length header
                    if line.to_lowercase().starts_with("content-length:") {
                        content_length = line.split(':').nth(1)
                            .and_then(|s| s.trim().parse::<usize>().ok());
                    }

                    // End of headers
                    if line.trim().is_empty() {
                        break;
                    }
                }
            }
        }

        // If we have a Content-Length, read exactly that many bytes
        if let Some(length) = content_length {
            let mut body = vec![0; length];
            response_reader.read_exact(&mut body).unwrap();
            response.extend(body);
        } else {
            // If no Content-Length, read until connection closes
            let mut buffer = [0; 4096];
            loop {
                match upstream.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(n) => response.extend_from_slice(&buffer[..n]),
                    Err(_) => break,
                }
            }
        }

        String::from_utf8_lossy(&response).to_string()
    }
}