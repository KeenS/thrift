use tokio_proto::server;
use tokio_proto::easy::pipeline;
use tokio_service::{Service, NewService, simple_service};
use tokio_core::reactor::Handle;
use futures::{Async, Future};
use std::io;
use std::net::SocketAddr;
use framed_transport::new_thrift_transport;
use thrift::*;


struct FlockServer<T>{
    inner: T
}

impl <T>Service for FlockServer<T>
    // where T: Service<Request = Flock_isLoggedIn_Args, Response = bool, Error = io::Error>,
    //       T::Future: 'static,
    where T: FlockService
{
    type Request = Flock_isLoggedIn_Args;
    type Response = bool;
    type Error = io::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;


    fn call(&self, req: Self::Request) -> Self::Future {
        self.inner.isLoggedIn(req.token)
    }

    fn poll_ready(&self) -> Async<()> {
        Async::Ready(())
    }
}

/// Serve a service up. Secret sauce here is 'NewService', a helper that must be able to create a
/// new 'Service' for each connection that we receive.
pub fn serve<T>(handle: Handle,  addr: SocketAddr, flock_service: T)
                -> io::Result<()>
    where T: FlockService+Clone+'static
{
    try!(server::listen(&handle, addr, move |stream| {
        // Initialize the pipeline dispatch with the service and the line
        // transport
        let service = FlockServer { inner: flock_service.clone() };
        Ok(pipeline::EasyServer::new(service,
                                  new_thrift_transport::<_, Flock_isLoggedIn_Args, bool>(stream)))
    }));
    Ok(())
}
