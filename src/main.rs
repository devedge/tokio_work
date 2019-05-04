extern crate bytes;
#[macro_use]
extern crate futures;
extern crate tokio;

use bytes::{Buf, Bytes};
use futures::{Async, Future, Poll};
use std::io::{self, Cursor};
use tokio::io::AsyncWrite;
use tokio::net::{tcp::ConnectFuture, TcpStream};

enum HelloWorld {
    Connecting(ConnectFuture),
    Connected(TcpStream, Cursor<Bytes>),
}

impl Future for HelloWorld {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(), io::Error> {
        use self::HelloWorld::*;

        loop {
            match self {
                Connecting(ref mut f) => {
                    let socket = try_ready!(f.poll());
                    let data = Cursor::new(Bytes::from_static(b"hello world"));
                    *self = Connected(socket, data);
                }
                Connected(ref mut socket, ref mut data) => {
                    // Keep trying to write the buffer to the socket as long as the
                    // buffer has more bytes available for consumption
                    while data.has_remaining() {
                        try_ready!(socket.write_buf(data));
                    }
                    return Ok(Async::Ready(()));
                }
            }
        }
    }
}

fn main() {
    let addr = "127.0.0.1:1234".parse().unwrap();
    let connect_future = TcpStream::connect(&addr);
    let hello_world = HelloWorld::Connecting(connect_future);

    // Run it, here we map the error since tokio::run expects a Future<Item=(), Error=()>
    tokio::run(hello_world.map_err(|e| println!("{0}", e)))
}
