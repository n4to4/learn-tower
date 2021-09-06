use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::{convert::Infallible, net::SocketAddr};

async fn handle(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World!".into()))
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

use hyper::service::Service;
use std::future::Ready;
use std::sync::{atomic::AtomicUsize, Arc};
use std::task::Poll;

struct DemoApp {
    counter: Arc<AtomicUsize>,
}

impl Service<Request<Body>> for DemoApp {
    type Response = Response<Body>;
    type Error = hyper::http::Error;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _req: Request<Body>) -> Self::Future {
        let counter = self
            .counter
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let res = Response::builder()
            .status(200)
            .header("Context-Type", "text/plain; charset=utf-8")
            .body(format!("Counter is at: {}", counter).into());
        std::future::ready(res)
    }
}

use hyper::server::conn::AddrStream;

struct DemoAppFactory {
    counter: Arc<AtomicUsize>,
}

impl Service<&AddrStream> for DemoAppFactory {
    type Response = DemoApp;
    type Error = Infallible;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, conn: &AddrStream) -> Self::Future {
        println!("Accepting a new connection from {:?}", conn);
        std::future::ready(Ok(DemoApp {
            counter: Arc::clone(&self.counter),
        }))
    }
}
