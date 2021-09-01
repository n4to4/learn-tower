use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::task::Poll;
use tower::{Service, ServiceExt};

struct Request {
    path_and_query: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

#[derive(Debug)]
struct Response {
    status: u32,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

async fn run<App>(mut app: App)
where
    App: Service<Request, Response = Response>,
    App::Error: std::fmt::Debug,
    App::Future: Send + 'static,
{
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        let req = Request {
            path_and_query: "/fake/path?page=1".to_owned(),
            headers: HashMap::new(),
            body: Vec::new(),
        };

        let app = match app.ready().await {
            Err(e) => {
                eprintln!("Service not able to accept requests: {:?}", e);
                continue;
            }
            Ok(app) => app,
        };

        let future = app.call(req);
        tokio::spawn(async move {
            match future.await {
                Ok(res) => println!("Successful response: {:?}", res),
                Err(e) => eprintln!("Error occurred: {:?}", e),
            }
        });
    }
}

#[derive(Default)]
struct DemoApp {
    counter: Arc<AtomicUsize>,
}

impl Service<Request> for DemoApp {
    type Response = Response;
    type Error = anyhow::Error;
    #[allow(clippy::type_complexity)]
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: Request) -> Self::Future {
        let counter = Arc::clone(&self.counter);
        Box::pin(async move {
            println!("Handling a request for {}", req.path_and_query);
            let counter = counter.fetch_add(1, Ordering::SeqCst);
            anyhow::ensure!(counter % 4 != 2, "Failing 25% of the time, just for fun");
            req.headers
                .insert("X-Counter".to_owned(), counter.to_string());
            let res = Response {
                status: 200,
                headers: req.headers,
                body: req.body,
            };
            Ok::<_, anyhow::Error>(res)
        })
    }
}

#[tokio::main]
async fn main() {
    run(DemoApp::default()).await;
}
