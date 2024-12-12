use crate::handlers::Handler;
use std::net::TcpStream;

pub struct CgiHandler {
    config: crate::config::Config
}

impl CgiHandler {
    pub fn new(config: crate::config::Config) -> Self {
        Self {
            config
        }
    }
}

impl Handler for CgiHandler {
    fn handle(&self, _stream: &TcpStream) -> String {
        println!("Handling CGI request");
        // print config
        println!("Config: {:?}", self.config);
        "HTTP/1.1 200 OK\r\n\
        Content-Type: text/plain\r\n\
        Content-Length: 12\r\n\
        Connection: close\r\n\
        \r\n\
        Hello World!\n\
        ".to_string()
    }
}
