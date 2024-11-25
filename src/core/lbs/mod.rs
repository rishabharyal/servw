pub mod lc;
pub mod rr;
pub mod none;

pub trait LoadBalancer: Send + Sync {
    fn select_server(&mut self) -> Option<String>;
    fn request_complete(&mut self, _server: String) {}
}

pub use self::none::*;
pub use self::lc::*;
pub use self::rr::*;
