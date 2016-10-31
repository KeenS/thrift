use tokio_proto::server;
use tokio_proto::easy::pipeline;
use tokio_service::{Service, simple_service};
use tokio_core::reactor::Handle;
use futures::{Async, Future};
use std::io;
use std::net::SocketAddr;
use thrust_tokio::framed_transport::*;
use thrift::*;

struct FlockServer<T>
{
    inner: T,
}

impl <T: FlockService>FlockServer<T>
{
    fn new(inner: T) -> Self
    {
        FlockServer {
            inner: inner
        }
    }
}

impl <T>Service for FlockServer<T>
    where T: FlockService
{
    type Request = FlockServiceMethodArgs;
    type Response = FlockServiceMethodReturn;
    type Error = io::Error;
    type Future = Box<Future<Item = FlockServiceMethodReturn, Error = io::Error>>;


    fn call(&self, req: Self::Request) -> Self::Future {
        use thrift::FlockServiceMethodArgs::*;
        use thrift::FlockServiceMethodReturn::*;
        match req {
            AisLoggedIn(args)  => Box::new(self.inner.isLoggedIn(args.token).map(RisLoggedIn)),
            AisLoggedOut(args) => Box::new(self.inner.isLoggedOut(args.token).map(RisLoggedOut)),
        }
    }

    fn poll_ready(&self) -> Async<()> {
        Async::Ready(())
    }
}

pub fn serve<T>(handle: Handle,  addr: SocketAddr, flock_service: T)
                -> io::Result<server::ServerHandle>
    where T: FlockService+Clone+'static
{
    server::listen(&handle, addr, move |stream| {
        let service = FlockServer { inner: flock_service.clone() };
        Ok(pipeline::EasyServer::new(service,
                                  new_thrift_server_transport::<_, FlockServiceMethods, FlockServiceMethodReturn>(stream)))
    })
}
