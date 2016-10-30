extern crate futures;
extern crate tokio_core as tokio;
extern crate tokio_service as service;
extern crate tokio_proto as proto;
extern crate thrift;

use tokio::reactor::Core;
use tokio::net::TcpStream;
use service::Service;
use thrift::thrift::Flock_isLoggedIn_Args;
use thrift::framed_transport::{new_thrift_transport};
use proto::easy::pipeline::connect;
use futures::Future;

pub fn main() {
    let mut core = Core::new().unwrap();

    // This brings up our server.
    let addr = "127.0.0.1:12345".parse().unwrap();

    thrift::service::serve(
        &core.handle(),
        addr,
        service::simple_service(|msg: Flock_isLoggedIn_Args| {
            println!("GOT: {:?}", msg.token);
            if &msg.token == "123" {
                Ok(true)
            } else {
                Ok(false)
            }
        })).unwrap();

    // Now our client. We use the same reactor as for the server - usually though this would be
    // done in a separate program most likely on a separate machine.
    println!("connecting");
    let stream = TcpStream::connect(&addr, &core.handle());
    println!("connected");
    let stream = core.run(stream).expect("failed to connect");
    let frame = new_thrift_transport::<_, bool, Flock_isLoggedIn_Args>(stream);
    let client = connect(frame, &core.handle());

    // The connect call returns us a ClientHandle that allows us to use the 'Service' as a function
    // - one that returns a future that we can 'await' on.
    let resp = client.call(Flock_isLoggedIn_Args{token: "123".to_string()});
    let resp = core.run(resp).expect("rpc failed");
    println!("RESPONSE: {:?}", resp);
}
