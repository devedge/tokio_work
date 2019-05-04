extern crate futures;
extern crate tokio;

use futures::Future;
use tokio::io;
use tokio::net::TcpStream;

fn main() {
    // test by running this in anothe terminal:
    // ncat -l 127.0.0.1 1234
    let addr = "127.0.0.1:1234".parse().unwrap();

    let future = TcpStream::connect(&addr)
        .and_then(|socket| io::write_all(socket, b"hello world"))
        .map(|_| println!("write complete"))
        .map_err(|_| println!("failed"));

    tokio::run(future);
}
