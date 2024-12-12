use std::net::TcpStream;

pub trait Handler: Send + Sync {
    fn handle(&self, stream: &TcpStream) -> String;
}
