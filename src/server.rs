extern crate futures;
extern crate grpcio;
extern crate protos;

use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::sync::oneshot;
use futures::Future;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

use protos::calculate::{CalcReply, CalcRequest};
use protos::calculate_grpc::{self, Calculator};


#[derive(Clone)]
struct CalculateService;

impl Calculator for CalculateService {
    fn plus(&mut self, ctx: RpcContext, Request: CalcRequest, sink: UnarySink<CalcReply>) {

        println!("Received Request {{ {:?} }}", Request);

        let mut CalcReply = CalcReply::new();

        CalcReply.set_result(    
            Request.get_number_a() + Request.get_number_b()
        );

        let f = sink
            .success(CalcReply.clone())
            .map(move |_| println!("Responded reply {{ {:?} }}", CalcReply))
            .map_err(move |err| eprintln!("Failed to reply: {:?}", err));

        ctx.spawn(f)
    }
}


fn main() {
    let env = Arc::new(Environment::new(1));
    let service = calculate_grpc::create_calculator(CalculateService);

    let mut server = ServerBuilder::new(env)
    .register_service(service)
    .bind("127.0.0.1", 5001)
    .build()
    .unwrap();

    server.start();

    for &(ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }

    let (tx, rx) = oneshot::channel();

    thread::spawn(move || {
        println!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });

    println!("Hello, world!");
    let _ = rx.wait();
    let _ = server.shutdown().wait();
}
