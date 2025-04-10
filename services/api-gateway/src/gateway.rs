use std::{sync::Arc, time::Duration};

use hyper::{server::conn::http1, service::service_fn};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use crate::{Router, Settings};


pub struct Gateway {
    config: Settings,
    router: Arc<Router>,
}

impl Gateway {
    pub fn new(config: Settings) -> Self {
        let health_check_interval = Duration::from_secs(30);
        let router = Arc::new(Router::new(health_check_interval));
        
        Router::start_health_check(Arc::clone(&router));

        Gateway { config, router }
    }

    pub async fn run(&self, listener: TcpListener) -> Result<(), Box<dyn std::error::Error>> {
        println!(
            "API Gateway running on `http://{}:{}`",
            self.config.gateway.host, self.config.gateway.port
        );

        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);
            let router = Arc::clone(&self.router);
    
            tokio::task::spawn(async move {
                if let Err(e) = http1::Builder::new()
                    .preserve_header_case(true)
                    .title_case_headers(true)
                    .serve_connection(io, service_fn(move |req| {
                        let router = Arc::clone(&router);
                        async move { router.handle_request(req).await }
                    }))
                    .await {
                        println!("Error serving connection: {:?}", e);
                    }
            });
        }
    }
    
}

#[cfg(test)]
mod tests {
    use crate::configuration::GatewaySettings;
    use super::*;
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn test_gateway_accepts_connections() {
        let config = Settings {
            gateway: GatewaySettings {
                host: "127.0.0.1".to_string(),
                port: 0,
            }
        };
        
        let listener = TcpListener::bind(format!(
            "{}:{}", 
            config.gateway.host, config.gateway.port))
            .await.unwrap();
        let addr = listener.local_addr().unwrap();
    
        let gateway = Gateway::new(config);
        
        let _ = tokio::spawn(async move {
            gateway.run(listener).await.unwrap();
        });
        
        let response = reqwest::get(format!("http://{}:{}/health", addr.ip(), addr.port())).await.unwrap();
        assert_eq!(response.status().as_u16(), 200);
    }
}