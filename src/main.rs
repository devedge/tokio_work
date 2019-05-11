extern crate futures;
extern crate tokio;

use futures::{Future, Stream};
use tokio::io;
use tokio::net::TcpListener;

fn main() {
    let addr = "127.0.0.1:1234".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();
    tokio::run({
        listener
            .incoming()
            .for_each(|socket| {
                // inbound socket has been received
                //
                // spawn new task to process a socket
                tokio::spawn({
                    // write to socket
                    io::write_all(socket, "hello world")
                        // drop the socket
                        .map(|_| ())
                        // write any error to STDOUT
                        .map_err(|e| println!("socket error = {:?}", e))
                });

                // receive next socket
                Ok(())
            })
            .map_err(|e| println!("listener error = {:?}", e))
    });
}
