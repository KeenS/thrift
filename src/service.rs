use tokio_proto::server;
use tokio_proto::easy::pipeline;
use tokio_service::Service;
use tokio_core::reactor::Handle;
use futures::Async;
use std::io;
use std::net::SocketAddr;
use thrust_tokio::framed_transport::new_thrift_transport;
use thrift::*;


struct FlockServer<T>{
    inner: T
}

impl <T>Service for FlockServer<T>
    where T: FlockService
{
    type Request = Flock_isLoggedIn_Args;
    type Response = bool;
    type Error = io::Error;
    type Future = T::F;


    fn call(&self, req: Self::Request) -> Self::Future {
        self.inner.isLoggedIn(req.token)
    }

    fn poll_ready(&self) -> Async<()> {
        Async::Ready(())
    }
}

pub fn serve<T>(handle: Handle,  addr: SocketAddr, flock_service: T)
                -> io::Result<()>
    where T: FlockService+Clone+'static
{
    try!(server::listen(&handle, addr, move |stream| {
        let service = FlockServer { inner: flock_service.clone() };
        Ok(pipeline::EasyServer::new(service,
                                  new_thrift_transport::<_, Flock_isLoggedIn_Args, bool>(stream)))
    }));
    Ok(())
}
