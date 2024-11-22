use std::collections::HashMap;

pub struct LeastConn {
    servers: Vec<String>,
    servers_and_count: HashMap<String, usize>,
}

impl LeastConn {
    pub fn new(servers: Vec<String>) -> Self {
        Self {
            servers,
            servers_and_count: HashMap::new(),
        }
    }
}
