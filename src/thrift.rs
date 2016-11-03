// DO NOT EDIT: autogenerated by thrust
#![allow(dead_code, unused_imports, non_snake_case)]
use futures::{Async, Future};
use thrust::protocol::{ThriftDeserializer, ThriftSerializer, ThriftMessageType};
use thrust::protocol::{Error, ThriftType};
use thrust::protocol::{Serializer, Deserializer};
use thrust::protocol::{Deserialize, Serialize};
use thrust_tokio::framed_transport::*;
use tokio_core::reactor::Handle;
use tokio_core::net::TcpStream;
use tokio_proto::server;
use tokio_proto::easy::{self, pipeline};
use tokio_service::Service;

use std::io;
use std::net::SocketAddr;
use std::str::FromStr;

pub trait FlockService: Send {
    fn isLoggedIn(&self, token: String) -> Box<Future<Item = bool, Error = io::Error>>;
    fn isLoggedOut(&self, token: String) -> Box<Future<Item = bool, Error = io::Error>>;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FlockServiceMethods {
    MisLoggedIn,
    MisLoggedOut,
}

impl FromStr for FlockServiceMethods {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::FlockServiceMethods::*;
        match s {
            "isLoggedIn" => Ok(MisLoggedIn),
            "isLoggedOut" => Ok(MisLoggedOut),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "failed to parse thrift method data"))
        }
    }
}


impl ParseThrift for FlockServiceMethods {
    type Args = FlockServiceMethodArgs;
    type Ret = FlockServiceMethodReturn;
    fn parse_args<D: Deserializer + ThriftDeserializer>(&self, proto: &mut D) -> Result<Self::Args, Error> {
        use self::FlockServiceMethodArgs::*;
        use self::FlockServiceMethods::*;
        match self {
            &MisLoggedIn => FlockisLoggedInArgs::deserialize(proto).map(AisLoggedIn),
            &MisLoggedOut => FlockisLoggedOutArgs::deserialize(proto).map(AisLoggedOut),
        }
    }

    fn parse_ret<D: Deserializer + ThriftDeserializer>(&self, proto: &mut D) -> Result<Self::Ret, Error> {
        use self::FlockServiceMethodReturn::*;
        use self::FlockServiceMethods::*;
        match self {
            &MisLoggedIn => bool::deserialize(proto).map(RisLoggedIn),
            &MisLoggedOut => bool::deserialize(proto).map(RisLoggedOut),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FlockServiceMethodArgs {
    AisLoggedIn(FlockisLoggedInArgs),
    AisLoggedOut(FlockisLoggedOutArgs),
}


impl Serialize for FlockServiceMethodArgs {
    fn serialize<S>(&self, s: &mut S) -> Result<(), Error>
        where S: Serializer + ThriftSerializer
    {
        use self::FlockServiceMethodArgs::*;
        match self {
            &AisLoggedIn(ref b) => {
                try!(s.write_message_begin("isLoggedIn", ThriftMessageType::Call));
                try!(b.serialize(s));
                try!(s.write_message_end());
            },
            &AisLoggedOut(ref b) => {
                try!(s.write_message_begin("isLoggedOut", ThriftMessageType::Call));
                try!(b.serialize(s));
                try!(s.write_message_end());
            },
        };
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FlockServiceMethodReturn {
    RisLoggedIn(bool),
    RisLoggedOut(bool),

}


impl Serialize for FlockServiceMethodReturn {
    fn serialize<S>(&self, s: &mut S) -> Result<(), Error>
        where S: Serializer + ThriftSerializer
    {
        use self::FlockServiceMethodReturn::*;
        match self {
            &RisLoggedIn(ref b) => {
                try!(s.write_message_begin("isLoggedIn", ThriftMessageType::Reply));
                try!(b.serialize(s));
                try!(s.write_message_end());
            },
            &RisLoggedOut(ref b) => {
                try!(s.write_message_begin("isLoggedOut", ThriftMessageType::Reply));
                try!(b.serialize(s));
                try!(s.write_message_end());
            },
        };
        Ok(())
    }
}



#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FlockisLoggedInArgs {
    pub token: String
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FlockisLoggedOutArgs {
    pub token: String
}
impl Serialize for FlockisLoggedInArgs {
    fn serialize<S>(&self, s: &mut S) -> Result<(), Error>
        where S: Serializer + ThriftSerializer
    {
        try!(s.write_struct_begin("Flock_isLoggedIn_Args"));
        try!(s.write_field_begin("token", ThriftType::String, 1));
        try!(self.token.serialize(s));
        try!(s.write_field_end());
        try!(s.write_field_stop());
        try!(s.write_struct_end());
        Ok(())
    }
}
impl Serialize for FlockisLoggedOutArgs {
    fn serialize<S>(&self, s: &mut S) -> Result<(), Error>
        where S: Serializer + ThriftSerializer
    {
        try!(s.write_struct_begin("Flock_isLoggedOut_Args"));
        try!(s.write_field_begin("token", ThriftType::String, 1));
        try!(self.token.serialize(s));
        try!(s.write_field_end());
        try!(s.write_field_stop());
        try!(s.write_struct_end());
        Ok(())
    }
}




impl Deserialize for FlockisLoggedInArgs {
    fn deserialize<D>(de: &mut D) -> Result<Self, Error>
        where D: Deserializer + ThriftDeserializer,
    {
        try!(de.read_struct_begin());
        let mut token = None;
        loop {
            let scheme_field = try!(de.read_field_begin());
            if scheme_field.ty == ThriftType::Stop {
                break;
            };
            match scheme_field.seq {
                1 => {
                    if scheme_field.ty == ThriftType::String {
                        token = Some(try!(de.deserialize_str()));
                    } else {
                        // skip
                    }
                },
                _ => (),// skip
            }
            try!(de.read_field_end());
        };
        try!(de.read_struct_end());
        let args = FlockisLoggedInArgs {
            token: token.unwrap(),
        };
        Ok(args)
    }
}

impl Deserialize for FlockisLoggedOutArgs {
    fn deserialize<D>(de: &mut D) -> Result<Self, Error>
        where D: Deserializer + ThriftDeserializer,
    {
        try!(de.read_struct_begin());
        let mut token = None;
        loop {
            let scheme_field = try!(de.read_field_begin());
            if scheme_field.ty == ThriftType::Stop {
                break;
            };
            match scheme_field.seq {
                1 => {
                    if scheme_field.ty == ThriftType::String {
                        token = Some(try!(de.deserialize_str()));
                    } else {
                        // skip
                    }
                },
                _ => (),// skip
            }
            try!(de.read_field_end());
        };
        try!(de.read_struct_end());
        let args = FlockisLoggedOutArgs {
            token: token.unwrap(),
        };
        Ok(args)
    }
}


pub struct FlockClient {
    inner: easy::EasyClient<FlockServiceMethodArgs, FlockServiceMethodReturn>,
}

impl FlockClient {
    pub fn new(handle: &Handle, stream: TcpStream) -> Self {
        let transport = new_thrift_client_transport::<_, FlockServiceMethods, FlockServiceMethodArgs>(stream);
        let easy_client = easy::pipeline::connect(transport, handle);

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
        let args = FlockisLoggedInArgs {
            token: token,
        };
        let ret = self.call(AisLoggedIn(args)).map(|r| {
            if let RisLoggedIn(ret) = r {
                ret
            } else {
                unreachable!("generated code error. map be a bug.");
            }
        });
        Box::new(ret)
    }
    fn isLoggedOut(&self, token: String) -> Box<Future<Item = bool, Error = io::Error>> {
        use thrift::FlockServiceMethodArgs::*;
        use thrift::FlockServiceMethodReturn::*;
        let args = FlockisLoggedOutArgs {
            token: token,
        };
        let ret = self.call(AisLoggedOut(args)).map(|r| {
            if let RisLoggedOut(ret) = r {
                ret
            } else {
                unreachable!("generated code error. map be a bug.");
            }
        });
        Box::new(ret)
    }
}
#[derive(Clone)]
pub struct FlockServer<T>
{
    inner: T,
}

impl <T: FlockService>FlockServer<T>
{
    pub fn new(inner: T) -> Self
    {
        FlockServer {
            inner: inner
        }
    }

    pub fn serve(self, handle: &Handle,  addr: SocketAddr)
                    -> io::Result<server::ServerHandle>
        where T: FlockService+Clone+'static
    {
        server::listen(handle, addr, move |stream| {
            Ok(pipeline::EasyServer::new(self.clone(),
                                         new_thrift_server_transport::<_, FlockServiceMethods, FlockServiceMethodReturn>(stream)))
        })
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
            AisLoggedOut(args)  => Box::new(self.inner.isLoggedOut(args.token).map(RisLoggedOut)),
        }
    }

    fn poll_ready(&self) -> Async<()> {
        Async::Ready(())
    }
}
