use anyhow::*;
use clap::Clap;
use tonic::transport::Endpoint;
use tonic_example::{echo_client::EchoClient, EchoRequest};

#[derive(Clap)]
struct Opt {
    #[clap(long, default_value = "http://localhost:3000")]
    server: String,
    message: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::parse();

    let endpoint: Endpoint = opt.server.parse().context("Invalid endpoint")?;
    let mut grpc = EchoClient::connect(endpoint)
        .await
        .context("Unable to establish connection")?;
    let res = grpc
        .echo(EchoRequest {
            message: opt.message,
        })
        .await
        .context("Unable to send echo request")?;

    println!("{:?}", res);
    Ok(())
}
