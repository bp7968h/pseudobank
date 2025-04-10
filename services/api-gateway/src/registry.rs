use std::{collections::HashMap, sync::{Arc, RwLock}, time::{Duration, Instant}};

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

    pub fn check_services_health(&self, timeout: Duration) {
        if let Ok(mut services) = self.services.write() {
            for (_, instance) in services.iter_mut() {
                if instance.last_heartbeat.elapsed() > timeout {
                    instance.status = ServiceStatus::Down;
                }
            }
        }
    }
}