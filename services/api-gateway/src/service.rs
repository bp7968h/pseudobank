use hyper::{Request, Response, StatusCode, Method};
use hyper::body::{Incoming, Bytes};
use http_body_util::Full;

pub async fn handle_request(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, hyper::Error> {
    println!("Received request: {} {}", req.method(), req.uri());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/health") => {
            let response = Response::builder()
                .status(StatusCode::OK)
                .body(Full::new(Bytes::from(r#"{"status":"ok"}"#)))
                .expect("Failed to create response");

            Ok(response)
        },
        _ => {
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Full::new(Bytes::from(r#"{"error":"Not Found"}"#)))
                .expect("Failed to create response");
            
            Ok(response)
        }
    }
}