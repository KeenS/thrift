use futures::{Async, Future};
use std::io;
use tokio_service::Service;
use tokio_proto::{self};
use tokio_core::reactor::Handle;
use tokio_core::net::TcpStream;
use framed_transport::new_thrift_transport;
use thrift::*;

/// And the client handle.
pub struct FlockClient {
    inner: tokio_proto::easy::EasyClient<Flock_isLoggedIn_Args, bool>,
}

impl FlockClient {
    pub fn new(handle:&Handle, stream: TcpStream) -> FlockClient {
        let transport = new_thrift_transport(stream);
        let easy_client = tokio_proto::easy::pipeline::connect(transport, &handle);

        FlockClient { inner: easy_client }
    }
}

impl Service for FlockClient {
    type Request = Flock_isLoggedIn_Args;
    type Response = bool;
    type Error = io::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, arg: Self::Request) -> Self::Future {
        // Make sure that the request does not include any new lines
        self.inner.call(arg)
            .boxed()
    }

    fn poll_ready(&self) -> Async<()> {
        Async::Ready(())
    }
}

impl FlockService for FlockClient {
    type F = <FlockClient as Service>::Future;

    fn isLoggedIn(&self, token: String) -> Self::F {
        self.call(Flock_isLoggedIn_Args{token: token})
    }
}
