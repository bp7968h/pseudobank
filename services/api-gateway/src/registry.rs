use std::{collections::HashMap, sync::{Arc, RwLock}, time::Instant};

use hyper::Method;

pub struct ServiceRegistration {
    service_name: String,
    base_url: String,
    routes: Vec<RouteDefinition>,
    health_path: String,
    timeout_ms: u64,
}

pub struct RouteDefinition {
    path: String,
    methods: Vec<Method>,
    rate_limit: Option<u32>,
    auth_required: bool,
}

pub enum ServiceStatus {
    Up,
    Down,
}

pub struct ServiceInstance {
    registration: ServiceRegistration,
    last_heartbeat: Instant,
    status: ServiceStatus,
}

pub struct Registry {
    services: Arc<RwLock<HashMap<String, ServiceInstance>>>,
}

impl Registry {
    pub fn new() -> Self {
        Registry { 
            services: Arc::new(RwLock::new(HashMap::new())), 
        }
    }
}