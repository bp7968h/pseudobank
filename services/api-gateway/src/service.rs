use hyper::{Request, Response, StatusCode, Method};
use hyper::body::{Body, Bytes};
use http_body_util::Full;

pub async fn handle_request(req: Request<impl Body>) -> Result<Response<Full<Bytes>>, hyper::Error> {
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

#[cfg(test)]
mod tests {
    use super::handle_request;
    use http_body_util::BodyExt;
    use hyper::{body::{Body, Bytes}, Method, Request, StatusCode};

    struct MockBody {
        data: &'static [u8],
    }

    impl MockBody {
        fn _new(data: &'static [u8]) -> Self {
            MockBody { data }
        }

        fn new_empty() -> Self {
            MockBody { data: &[] }
        }
    }

    impl Body for MockBody {
        type Data = Bytes;
        type Error = hyper::Error;

        fn poll_frame(
                mut self: std::pin::Pin<&mut Self>,
                _cx: &mut std::task::Context<'_>,
            ) -> std::task::Poll<Option<Result<hyper::body::Frame<Self::Data>, Self::Error>>> {
            if self.data.is_empty() {
                std::task::Poll::Ready(None)
            } else {
                let data = self.data;
                self.data = &[];
                std::task::Poll::Ready(Some(Ok(hyper::body::Frame::data(Bytes::from(data)))))
            }
        }
    }

    #[tokio::test]
    async fn test_health_endpoint() {
        let request = Request::builder()
            .method(Method::GET)
            .uri("/health")
            .body(MockBody::new_empty())
            .unwrap();

        let response = handle_request(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.body().to_owned().frame().await.unwrap().unwrap().into_data().unwrap(), r#"{"status":"ok"}"#);
    }

    #[tokio::test]
    async fn test_unknown_endpoint() {
        let request = Request::builder()
            .method(Method::GET)
            .uri("/unknown")
            .body(MockBody::new_empty())
            .unwrap();

        let response = handle_request(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        assert_eq!(response.body().to_owned().frame().await.unwrap().unwrap().into_data().unwrap(), r#"{"error":"Not Found"}"#);
    }
}