struct LeastConn {}

impl LeastConn {
    pub fn new() -> Self {
        Self {}
    }
}

impl LoadBalancer for LeastConn {
    fn select_server(&self, servers: &Vec<Server>) -> Option<Server> {
        Some(servers[0].clone())
    }
}
