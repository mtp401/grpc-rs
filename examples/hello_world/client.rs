// Copyright 2017 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.


extern crate futures;
extern crate grpc;
extern crate grpc_proto;
extern crate protobuf;

use std::sync::Arc;

use grpc::{ChannelBuilder, Environment};
use grpc_proto::example::helloworld::HelloRequest;
use grpc_proto::example::helloworld_grpc::GreeterClient;

fn main() {
    let env = Arc::new(Environment::new(1));
    let ch = ChannelBuilder::new(env).connect("localhost:50051");
    let client = GreeterClient::new(ch);

    let mut req = HelloRequest::new();
    req.set_name("world".to_owned());
    let reply = client.say_hello(req).expect("rpc");
    println!("Greeter received: {}", reply.get_message());
}
