use rand::Rng;
use crate::core::lbs::LoadBalancer;

pub struct None {
    servers: Vec<String>,
    slen: usize,
}

impl None {
    pub fn new(servers: Vec<String>) -> Self {
        Self {
            slen: servers.len(),
            servers,
        }
    }
}

impl LoadBalancer for None{
    fn select_server(&mut self) -> Option<String> {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.slen);

        Some(self.servers[index].clone())
    }
}