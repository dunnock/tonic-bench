use hello_bench::greeter_client::GreeterClient;
use hello_bench::{Empty, Something};
use futures::future::try_join_all;
use tonic::Request;
use structopt::StructOpt;

pub mod hello_bench {
    tonic::include_proto!("hellobench");
}

#[derive(StructOpt)]
struct Opt {
    /// Number of parallel client connections
    #[structopt(short="c", long="connections", default_value="1")]
    connections: usize,
    /// Send amount of messages in every connection
    /// multiplied by num of connections
    #[structopt(short="m", long="messages", default_value="10000")]
    messages: usize,
    #[structopt(short="r", long="request", default_value="Empty")]
    request: RequestOption
}

#[derive(Clone)]
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
/*    fn create<T>(&self) -> Request<T> {
        match self {
            Self::Empty => Request::new(Empty{}),
            Self::Something => Request::new(Something { text: "some request string".to_owned() })
        }
    }
*/
    async fn send<T>(&self, client: &mut GreeterClient<T>) -> Result<(),tonic::Status> 
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
    let mut clients = try_join_all((0..conns).map(|_| GreeterClient::connect("http://[::1]:50051"))).await?;

    const LOOPS: usize = 100000;    

    //let mut requests: Vec<Request<_>> = (0..LOOPS)
    //    .map(|_| tonic::Request::new(opt.request.create())).collect();

    //const CAP: usize = 100;
    
    let time = std::time::Instant::now();
    let batch_size: usize = LOOPS/conns;
//    let mut batches = Vec::with_capacity(conns);
//    for _ in 0..conns {
//        let batch: Vec<Request<_>> = requests.drain(0..batch_size).collect();
//        batches.push(batch);
//    }

    let mut handles = Vec::with_capacity(conns);
    for mut client in clients.drain(..) {
        //let batch = batches.pop().unwrap();
        let request = opt.request.clone();
        let num = opt.messages;
        handles.push(
            tokio::spawn(async move {
                for _m in 0..num {
                    request.send(&mut client).await.expect("Error from server");
                }
            })
        )
    }
    try_join_all(handles).await?;
    let elps = time.elapsed();
    println!("Elapsed: {}ms", elps.as_millis());
    let total = opt.messages * opt.connections;
    println!("processed {} with {:.0} rps", total, (total) as f64 / elps.as_millis() as f64 * 1000.0);

    Ok(())
}
