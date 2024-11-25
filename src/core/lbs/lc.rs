use crate::core::lbs::LoadBalancer;

pub struct LeastConn {
    servers: Vec<String>,
    connections: Vec<usize>,
}

impl LeastConn {
    pub fn new(servers: Vec<String>) -> Self {
        Self {
            connections: vec![0; servers.len()],
            servers,
        }
    }
}

impl LoadBalancer for LeastConn {
    fn select_server(&mut self) -> Option<String> {
        let mut least_connection_index  = 0;
        for i in 1..self.connections.len() {
            if self.connections[i] == 0 {
                least_connection_index = i;
                break;
            }

            if self.connections[i] < self.connections[least_connection_index] {
                least_connection_index = i;
            }
        }

        self.connections[least_connection_index] += 1;
        Some(self.servers[least_connection_index].clone())
        
    }

    fn request_complete(&mut self, server: String) {
        self.connections[self.servers.iter().position(|s| s == &server).unwrap()] -= 1;
    }
}
