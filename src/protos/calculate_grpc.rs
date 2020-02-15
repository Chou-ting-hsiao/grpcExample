// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_CALCULATOR_PLUS: ::grpcio::Method<super::calculate::CalcRequest, super::calculate::CalcReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/calculate.Calculator/Plus",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct CalculatorClient {
    client: ::grpcio::Client,
}

impl CalculatorClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        CalculatorClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn plus_opt(&self, req: &super::calculate::CalcRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::calculate::CalcReply> {
        self.client.unary_call(&METHOD_CALCULATOR_PLUS, req, opt)
    }

    pub fn plus(&self, req: &super::calculate::CalcRequest) -> ::grpcio::Result<super::calculate::CalcReply> {
        self.plus_opt(req, ::grpcio::CallOption::default())
    }

    pub fn plus_async_opt(&self, req: &super::calculate::CalcRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::calculate::CalcReply>> {
        self.client.unary_call_async(&METHOD_CALCULATOR_PLUS, req, opt)
    }

    pub fn plus_async(&self, req: &super::calculate::CalcRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::calculate::CalcReply>> {
        self.plus_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Calculator {
    fn plus(&mut self, ctx: ::grpcio::RpcContext, req: super::calculate::CalcRequest, sink: ::grpcio::UnarySink<super::calculate::CalcReply>);
}

pub fn create_calculator<S: Calculator + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_CALCULATOR_PLUS, move |ctx, req, resp| {
        instance.plus(ctx, req, resp)
    });
    builder.build()
}
