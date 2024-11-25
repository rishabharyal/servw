use crate::core::lbs::LoadBalancer;
use crate::handlers::Handler;
use std::net::TcpStream;
use std::sync::Mutex;
use std::sync::Arc;
use rand::Rng;

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
    fn handle(&self, _tcp_stream: &TcpStream) -> String {
        let selected_server = self.lb.lock().unwrap().select_server().unwrap();
        // sleep for five seconds
        let random_seconds = rand::thread_rng().gen_range(0..30);
        std::thread::sleep(std::time::Duration::from_secs(random_seconds));
        self.lb.lock().unwrap().request_complete(selected_server.clone());
        // return a json response
        
        let body = format!("{{\"server\": \"{}\"}}", selected_server);

        format!(
            "HTTP/1.1 200 OK\r\n\
            Content-Length: {}\r\n\
            Content-Type: application/json\r\n\
            Connection: close\r\n\
            \r\n\
            {}",
            body.len(),
            body
        )

    }
}
