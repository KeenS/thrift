use tokio_proto::{self, server};
use tokio_proto::easy::pipeline;
use tokio_service::{Service, NewService};
use tokio_core::reactor::Handle;
use futures::{Async, Future};
use std::io;
use std::net::SocketAddr;
use framed_transport::{TTransport, new_thrift_transport};
use thrift::*;


pub struct FlockServer<T>{
    inner: T
}

impl <T>Service for FlockServer<T>
    where T: Service<Request = Flock_isLoggedIn_Args, Response = bool, Error = io::Error>,
          T::Future: 'static,
{
    type Request = Flock_isLoggedIn_Args;
    type Response = bool;
    type Error = io::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;


    fn call(&self, req: Self::Request) -> Self::Future {
        Box::new(self.inner.call(req))
    }

    fn poll_ready(&self) -> Async<()> {
        Async::Ready(())
    }
}

/// Serve a service up. Secret sauce here is 'NewService', a helper that must be able to create a
/// new 'Service' for each connection that we receive.
pub fn serve<T>(handle: &Handle,  addr: SocketAddr, new_service: T)
                -> io::Result<()>
    where T: NewService<Request = Flock_isLoggedIn_Args, Response = bool, Error = io::Error> + Send + 'static,
{
    try!(server::listen(handle, addr, move |stream| {
        // Initialize the pipeline dispatch with the service and the line
        // transport
        let service = FlockServer { inner: try!(new_service.new_service()) };
        Ok(pipeline::EasyServer::new(service,
                                  new_thrift_transport::<_, Flock_isLoggedIn_Args, bool>(stream)))
    }));
    Ok(())
}
