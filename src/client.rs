use futures::{Async, Future};
use std::io;
use tokio_service::Service;
use tokio_proto::easy;
use tokio_core::reactor::Handle;
use tokio_core::net::TcpStream;
use thrust_tokio::framed_transport::new_thrift_transport;
use thrift::*;

pub struct FlockClient {
    inner: easy::EasyClient<Flock_isLoggedIn_Args, bool>,
}

impl FlockClient {
    pub fn new(handle:&Handle, stream: TcpStream) -> Self {
        let transport = new_thrift_transport(stream);
        let easy_client = easy::pipeline::connect(transport, &handle);

        FlockClient { inner: easy_client }
    }
}

impl Service for FlockClient {
    type Request = Flock_isLoggedIn_Args;
    type Response = bool;
    type Error = io::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, arg: Self::Request) -> Self::Future {
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
