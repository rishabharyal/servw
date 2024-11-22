use std::collections::HashMap;

struct LeastConn {
    servers: Vec<String>,
    servers_and_count: HashMap<String, usize>,
}

impl LeastConn {
    pub fn new() -> Self {
        Self {
            servers: Vec::new(),
            servers_and_count: HashMap::new(),
        }
    }
}
