pub struct RoundRobin {
    servers: Vec<string>,
    index: usize,
}

impl RoundRobin {
    pub fn new() -> Self {
        Self {}
    }
}

impl LoadBalancer for RoundRobin {
    fn select_server(&self, servers: &Vec<Server>) -> Option<Server> {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..servers.len());

        Some(servers[index].clone())
    }
}
