use crate::core::lbs::LoadBalancer;

pub struct RoundRobin {
    servers: Vec<String>,
    last_index: usize,
    slen: usize,
}

impl RoundRobin {
    pub fn new(servers: Vec<String>) -> Self {
        Self {
            slen: servers.len(),
            servers,
            last_index: 0,
        }
    }
}

impl LoadBalancer for RoundRobin {
    fn select_server(&mut self) -> Option<String> {
        if self.servers.is_empty() {
            return None;
        }

        let current = self.last_index;
        self.last_index = (self.last_index+1) % self.slen;
        Some(self.servers[current].clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_servers() {
        let mut lb = RoundRobin::new(vec![]);
        assert_eq!(lb.select_server(), None);
    }

    #[test]
    fn test_single_server() {
        let mut lb = RoundRobin::new(vec!["server1".to_string()]);
        assert_eq!(lb.select_server(), Some("server1".to_string()));
        assert_eq!(lb.select_server(), Some("server1".to_string()));
    }

    #[test]
    fn test_round_robin_order() {
        let servers = vec![
            "server1".to_string(),
            "server2".to_string(),
            "server3".to_string(),
        ];
        let mut lb = RoundRobin::new(servers);

        assert_eq!(lb.select_server(), Some("server1".to_string()));
        assert_eq!(lb.select_server(), Some("server2".to_string()));
        assert_eq!(lb.select_server(), Some("server3".to_string()));
        // Should wrap around to beginning
        assert_eq!(lb.select_server(), Some("server1".to_string()));
    }

    #[test]
    fn test_multiple_cycles() {
        let servers = vec![
            "server1".to_string(),
            "server2".to_string(),
        ];
        let mut lb = RoundRobin::new(servers);

        // First cycle
        assert_eq!(lb.select_server(), Some("server1".to_string()));
        assert_eq!(lb.select_server(), Some("server2".to_string()));

        // Second cycle
        assert_eq!(lb.select_server(), Some("server1".to_string()));
        assert_eq!(lb.select_server(), Some("server2".to_string()));
    }
}