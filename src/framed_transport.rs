use std::io::{self, Cursor};
use tokio_core::io::Io;
use tokio_core::easy::{Parse, Serialize, EasyBuf, EasyFramed};
use tokio_proto::pipeline;
use thrust::protocol::{ThriftDeserializer, ThriftSerializer, Serializer, Deserializer,Deserialize as De, Serialize as Se, Error};
use thrust::transport::*;
use thrust::binary_protocol::BinaryProtocol;
use futures::{Poll, Async};
use std::marker::PhantomData;
use thrift::Flock_isLoggedIn_Args;

pub struct TTransport<T> {
    phantom: PhantomData<T>
}
impl <T>TTransport<T> {
    pub fn new() -> Self {
        TTransport {
            phantom: PhantomData
        }
    }
}

// pub struct TTransport;

// impl TTransport {
//     pub fn new() -> Self {
//         TTransport
//     }
// }

//impl <D>Parse for TTransport<D>
//    where D: Deserializer + ThriftDeserializer + From<ReadTransport>,
impl Parse for TTransport<Flock_isLoggedIn_Args>
{
    type Out = Flock_isLoggedIn_Args;

    fn parse(&mut self, buf: &mut EasyBuf) -> Poll<Self::Out, io::Error> {
        println!("parsig {:?}", buf.as_ref());
        let mut protocol = BinaryProtocol::from(Cursor::new(buf));
        match De::deserialize(&mut protocol) {
            Ok(res) => Ok(Async::Ready(res)),
            Err(Error::Byteorder(_)) => Ok(Async::NotReady),
            Err(Error::Io(e)) => Err(e),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "failed to parse thrift data"))
        }
    }
}

//impl <D>Parse for TTransport<D>
//    where D: Deserializer + ThriftDeserializer + From<ReadTransport>,
impl Parse for TTransport<bool>
{
    type Out = bool;

    fn parse(&mut self, buf: &mut EasyBuf) -> Poll<Self::Out, io::Error> {
        println!("parsing {:?}", buf.as_ref());
        let mut protocol = BinaryProtocol::from(Cursor::new(buf));
        match De::deserialize(&mut protocol) {
            Ok(res) => Ok(Async::Ready(res)),
            Err(Error::Byteorder(_)) => Ok(Async::NotReady),
            Err(Error::Io(e)) => Err(e),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "failed to parse thrift data"))
        }
    }
}


//impl <S>Serialize for TTransport<S>
//    where S: Serializer + ThriftSerializer + From<WriteTransport>,
impl Serialize for TTransport<bool>
{
    type In = bool;

    fn serialize(&mut self, frame: Self::In, buf: &mut Vec<u8>) {
        println!("serializing {:?}", frame);
        let mut protocol = BinaryProtocol::from(buf);
        let _ = frame.serialize(&mut protocol);
    }
}

//impl <S>Serialize for TTransport<S>
//    where S: Serializer + ThriftSerializer + From<WriteTransport>,
impl Serialize for TTransport<Flock_isLoggedIn_Args>
{
    type In = Flock_isLoggedIn_Args;

    fn serialize(&mut self, frame: Self::In, buf: &mut Vec<u8>) {
        println!("serializing {:?}", frame.token);
        let mut protocol = BinaryProtocol::from(buf);
        let _ = frame.serialize(&mut protocol);
    }
}

pub type FramedThriftTransport<T, D, S> = EasyFramed<T, D, S>;

pub fn new_thrift_transport<T, D, S>(inner: T, de: D, se: S) -> FramedThriftTransport<T, D, S>
    where T: Io,
          D: Parse,
          S: Serialize,
          // D: Deserializer + ThriftDeserializer + Parse,
          // S: Serializer + ThriftSerializer + Serialize,
{
  EasyFramed::new(inner,
              de,
              se)
}
