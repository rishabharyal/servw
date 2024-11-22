use crate::core::lbs::LoadBalancer;

pub struct ServerHandler {
    lb: LoadBalancer,
}

impl ServerHandler {
    pub fn new(lb: LoadBalancer) -> Self {
        Self {
            lb,
        }
    }

    pub fn handle(&self) -> String {
        // TODO: Implement
        //
        // first, get the required server from the load balancer
        // seocnd, make the reques to the obtained server

        return "HTTP/1.1 200 OK\n\n".to_string();
    }
    
}
