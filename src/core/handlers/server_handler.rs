use crate::core::lbs::LoadBalancer;
use crate::handlers::Handler;

pub struct ServerHandler {
    lb: Box<dyn LoadBalancer>,
}

impl ServerHandler {
    pub fn new(lb: Box<dyn LoadBalancer>) -> ServerHandler {
        ServerHandler { lb }
    }
}

impl Handler for ServerHandler {
    fn handle(&self) -> String {
        "done".to_string()
    }
}
