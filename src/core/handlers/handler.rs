use std::net::TcpStream;

pub trait Handler {
    fn handle(&self, stream: TcpStream) -> String;
}
