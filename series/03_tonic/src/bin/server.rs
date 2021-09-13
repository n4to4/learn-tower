use tonic::async_trait;
use tonic_example::echo_server::{Echo, EchoServer};
use tonic_example::{EchoReply, EchoRequest};

use hyper::service::make_service_fn;
use std::convert::Infallible;

pub struct MyEcho;

#[async_trait]
impl Echo for MyEcho {
    async fn echo(
        &self,
        request: tonic::Request<EchoRequest>,
    ) -> Result<tonic::Response<EchoReply>, tonic::Status> {
        Ok(tonic::Response::new(EchoReply {
            message: format!("Echoing back: {}", request.get_ref().message),
        }))
    }
}

#[tokio::main]
async fn _main() -> anyhow::Result<()> {
    let addr = ([0, 0, 0, 0], 3000).into();

    tonic::transport::Server::builder()
        .add_service(EchoServer::new(MyEcho))
        .serve(addr)
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = ([0, 0, 0, 0], 3000).into();

    let grpc_service = tonic::transport::Server::builder()
        .add_service(EchoServer::new(MyEcho))
        .into_service();

    let make_grpc_service = make_service_fn(move |_conn| {
        let grpc_service = grpc_service.clone();
        async { Ok::<_, Infallible>(grpc_service) }
    });

    let server = hyper::Server::bind(&addr).serve(make_grpc_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}
