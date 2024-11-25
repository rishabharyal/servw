use crate::handlers::Handler;
use std::net::TcpStream;

pub struct CgiHandler {}

impl CgiHandler {
    pub fn new() -> Self {
        Self {}
    }
}

impl Handler for CgiHandler {
    fn handle(&self, _stream: &TcpStream) -> String {
        "done".to_string()
    }
}
