use crate::handlers::Handler;

pub struct CgiHandler {

}

impl CgiHandler {
    pub fn new() -> Self {
        Self {}
    }
}

impl Handler for CgiHandler {
    fn handle(&self) -> String {
        "done".to_string()
    }
}