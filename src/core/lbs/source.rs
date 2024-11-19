struct Source {}

impl Source {
    pub fn new() -> Self {
        Self {}
    }
}

impl LoadBalancer for Source {
    fn select_server(&self, servers: &Vec<Server>) -> Option<Server> {
        Some(servers[0].clone())
    }
}
