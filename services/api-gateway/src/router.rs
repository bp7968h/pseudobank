use crate::Registry;

pub struct Router {
    registry: Registry,
}

impl Router {
    pub fn new() -> Self {
        Router { 
            registry: Registry::new(),
        }
    }
}