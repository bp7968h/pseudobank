use std::error;
use api_gateway::gateway::Gateway;
use api_gateway::get_settings;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let configuration = get_settings().unwrap();
    let listening_addr = format!(
        "{}:{}",
        configuration.gateway.host, configuration.gateway.port
    );

    let listener = TcpListener::bind(&listening_addr).await?;
    let gateway = Gateway::new(configuration);

    gateway.run(listener).await
}