use std::error;
use api_gateway::get_settings;

use tokio::net::TcpListener;
use hyper_util::rt::TokioIo;
use hyper::server::conn::http1;
use hyper::service::service_fn;

use api_gateway::handle_request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let configuration = get_settings().unwrap();
    let listening_addr = format!(
        "{}:{}",
        configuration.gateway.host, configuration.gateway.port
    );

    let listener = TcpListener::bind(&listening_addr).await?;
    println!("Server running on `http://{}`", listening_addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(e) = http1::Builder::new()
                .preserve_header_case(true)
                .title_case_headers(true)
                .serve_connection(io, service_fn(handle_request))
                .await {
                    println!("Error serving connection: {:?}", e);
                }
        });
    }
}