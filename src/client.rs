extern crate grpcio;
extern crate protos;

use std::env;
use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};

use protos::calculate::{CalcReply, CalcRequest};
use protos::calculate_grpc::CalculatorClient;

fn main() {

    let port = 5001;

    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect(format!("localhost:{}", port).as_str());

    let client = CalculatorClient::new(ch);

    let mut Request = CalcRequest::new();

    Request.set_number_a(1);
    Request.set_number_b(1);
   
    let Reply = client.plus(&Request).expect("RPC Failed!");

    println!("Plus {:?} and got {:.2}", Request, Reply.get_result());
}
