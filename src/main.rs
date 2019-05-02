extern crate tokio;

use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

// after starting, run in another terminal:
// ncat localhost 6142

fn main() {
    let addr = "127.0.0.1:6142".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    // convert TcpListener to a stream of incoming connections
    let server = listener
        .incoming()
        .for_each(|socket| {
            let (reader, writer) = socket.split();
            let amount = io::copy(reader, writer);

            let msg = amount.then(|result| {
                match result {
                    Ok((amount, _, _)) => println!("wrote {} bytes", amount),
                    Err(e) => println!("error: {}", e),
                }

                Ok(())
            });

            tokio::spawn(msg);
            Ok(())
        })
        .map_err(|err| {
            println!("accept error = {:?}", err);
        });

    println!("server running on localhost:6142");

    // start the server
    tokio::run(server);
}
