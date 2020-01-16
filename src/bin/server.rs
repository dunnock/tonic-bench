use tonic::{transport::Server, Request, Response, Status};

use hello_bench::greeter_server::{Greeter, GreeterServer};
use hello_bench::{Empty, Something};

pub mod hello_bench {
    tonic::include_proto!("hellobench");
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_empty(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<Empty>, Status> {
//        println!("Got a request from {:?}", request.remote_addr());

        let reply = Empty {};
        Ok(Response::new(reply))
    }
    async fn say_something(
        &self,
        request: Request<Something>,
    ) -> Result<Response<Something>, Status> {
//        println!("Got a request from {:?}", request.remote_addr());

        let reply = Something {
            text: "some reply string".to_owned(),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
