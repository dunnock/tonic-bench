use hello_bench::greeter_client::GreeterClient;
use hello_bench::{Empty, Something};
use futures::future::{select_all, try_join_all, FutureExt};
use tonic::Request;
use structopt::StructOpt;

pub mod hello_bench {
    tonic::include_proto!("hellobench");
}

#[derive(StructOpt)]
struct Opt {
    /// Number of parallel client connections
    #[structopt(long="connections", default_value="1")]
    connections: usize,
    /// Number of parallel requests per connection
    #[structopt(short="c", long="concurency", default_value="10")]
    concurency: usize,
    /// Send amount of messages in every connection
    /// multiplied by num of connections
    #[structopt(short="m", long="messages", default_value="10000")]
    messages: usize,
    /// Type of request Empty or Something
    #[structopt(short="r", long="request", default_value="Empty")]
    request: RequestOption,
    /// Port where server is running
    #[structopt(short="p", long="port", default_value="50051")]
    port: u32
}

#[derive(Clone, Copy)]
enum RequestOption {
    Empty,
    Something
}
impl std::str::FromStr for RequestOption {
    type Err = String;
    fn from_str(s: &str) -> Result<Self,Self::Err> {
        match s {
            "Empty" => Ok(RequestOption::Empty),
            "Something" => Ok(RequestOption::Something),
            s => Err(format!("allowed options: {}", s))
        }
    }
}
use tonic::codegen::*;
impl RequestOption {
    async fn send<T>(&self,  mut client: GreeterClient<T>) -> Result<(),tonic::Status> 
        where 
            T: tonic::client::GrpcService<tonic::body::BoxBody>,
            T::ResponseBody: Body + HttpBody + Send + 'static,
            T::Error: Into<StdError>,
            <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,

    {
        match self {
            Self::Empty => {
                client.say_empty(Request::new(Empty{})).await?;
            },
            Self::Something => {
                client.say_something(
                    Request::new(Something { text: "some request string".to_owned() })
                ).await?;
            }
        };
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let conns: usize = opt.connections;
    let addr = format!("http://[::1]:{}", opt.port);
    let clients = (0..conns).map(|_| GreeterClient::connect(addr.clone()));
    let mut clients = try_join_all(clients).await?;

    let time = std::time::Instant::now();

    let mut handles = Vec::with_capacity(conns);
    for client in clients.drain(..) {
        let request = opt.request.clone();
        // number of concurrent stream for single connection
        let concur = opt.concurency;
        // number of messages should be sent in every stream
        let msgs = opt.messages;
        let make_request = move |_| {
            let request = request.clone();
            let client = client.clone();
            async move {
                request.send(client.clone()).await
            }.boxed()
        };
        let mut futures = (0..concur).map(make_request.clone()).collect();
        
        handles.push(
            tokio::spawn(async move {
                let (mut ok, mut fail) = (0,0);
                // send messages concurrently for single client
                for msg in 0..msgs {
                    let (res, _, f) = select_all(futures).await;
                    match res {
                        Ok(_) => ok += 1,
                        Err(_) => fail += 1
                    };
                    futures = f;
                    futures.push(make_request(msg));
                };
                (ok, fail)
            })
        )
    }
    let res: Vec<(usize, usize)> = try_join_all(handles).await?;
    let elps = time.elapsed();
    println!("Elapsed: {}ms", elps.as_millis());
    let total = opt.messages * opt.connections;
    println!("processed {} with {:.0} rps", total, (total) as f64 / elps.as_millis() as f64 * 1000.0);

    let res = res.iter().fold((0,0), |acc, (ok, fail)| (acc.0+ok, acc.1+fail));
    println!("successful {} failed {} requests", res.0, res.1);

    Ok(())
}
