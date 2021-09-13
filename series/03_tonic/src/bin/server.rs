use tonic::async_trait;
use tonic_example::echo_server::{Echo, EchoServer};
use tonic_example::{EchoReply, EchoRequest};

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
async fn main() -> anyhow::Result<()> {
    let addr = ([0, 0, 0, 0], 3000).into();

    tonic::transport::Server::builder()
        .add_service(EchoServer::new(MyEcho))
        .serve(addr)
        .await?;

    Ok(())
}
