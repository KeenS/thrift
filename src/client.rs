use futures::{Async, Future};
use std::io;
use tokio_service::Service;
use tokio_proto::easy;
use tokio_core::reactor::Handle;
use tokio_core::net::TcpStream;
use thrust_tokio::framed_transport::*;
use thrift::*;

pub struct FlockClient {
    inner: easy::EasyClient<FlockServiceMethodArgs, FlockServiceMethodReturn>,
}

impl FlockClient {
    pub fn new(handle:&Handle, stream: TcpStream) -> Self {
        let transport = new_thrift_client_transport::<_, FlockServiceMethods, FlockServiceMethodArgs>(stream);
        let easy_client = easy::pipeline::connect(transport, &handle);

        FlockClient { inner: easy_client }
    }
}

impl Service for FlockClient {
    type Request = FlockServiceMethodArgs;
    type Response = FlockServiceMethodReturn;
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
    fn isLoggedIn(&self, token: String) -> Box<Future<Item = bool, Error = io::Error>> {
        use thrift::FlockServiceMethodArgs::*;
        use thrift::FlockServiceMethodReturn::*;
        let ret = self.call(AisLoggedIn(Flock_isLoggedIn_Args{token: token})).map(|r| {
            if let RisLoggedIn(ret) = r {
                ret
            } else {
                unreachable!("logic flaw");
            }
        });
        Box::new(ret)
    }
    fn isLoggedOut(&self, token: String) -> Box<Future<Item = bool, Error = io::Error>> {
        use thrift::FlockServiceMethodArgs::*;
        use thrift::FlockServiceMethodReturn::*;
        let ret = self.call(AisLoggedOut(Flock_isLoggedOut_Args{token: token})).map(|r| {
            if let RisLoggedOut(ret) = r {
                ret
            } else {
                unreachable!("logic flaw");
            }
        });
        Box::new(ret)
    }
}
