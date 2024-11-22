pub mod lc;
pub mod rr;
mod none;

trait LoadBalancer {
    fn select_server(&mut self) -> Option<String>;
}