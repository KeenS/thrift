extern crate futures;
extern crate tokio_core as tokio;
extern crate tokio_service as service;
extern crate tokio_proto as proto;
extern crate thrift;

use std::io;
use futures::{Future,done};
use tokio::reactor::Core;
use tokio::net::TcpStream;
use thrift::thrift::*;
use thrift::client::FlockClient;
use thrift::service::serve;

#[derive(Clone)]
struct FlockServer;

impl FlockService for FlockServer {
    fn isLoggedIn(&self, token: String) -> Box<Future<Item = bool, Error = io::Error>> {
        println!("GOT: {:?}", token);
        if &token == "123" {
            Box::new(done(Ok(true)))
        } else {
            Box::new(done(Ok(false)))
        }
    }
}

pub fn main() {
    let mut core = Core::new().unwrap();

    // This brings up our server.
    let addr = "127.0.0.1:12345".parse().unwrap();

    serve(
        core.handle(),
        addr,
        FlockServer).unwrap();

    // Now our client. We use the same reactor as for the server - usually though this would be
    // done in a separate program most likely on a separate machine.
    let handle = core.handle().clone();
    let stream = core.run(TcpStream::connect(&addr, &handle.clone())).expect("connection failed");
    let client = FlockClient::new(&handle.clone(), stream);

    // The connect call returns us a ClientHandle that allows us to use the 'Service' as a function
    // - one that returns a future that we can 'await' on.
    let resp = client.isLoggedIn("123".to_string());
    let resp = core.run(resp).expect("rpc failed");
    println!("RESPONSE: {:?}", resp);
    let resp = client.isLoggedIn("1234".to_string());
    let resp = core.run(resp).expect("rpc failed");
    println!("RESPONSE: {:?}", resp);

}
