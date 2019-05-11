extern crate futures;
extern crate tokio;

use futures::future::lazy;
use futures::sync::mpsc;
use futures::{future, stream, Future, Sink, Stream};
use std::time::Duration;
use tokio::io;
use tokio::net::TcpListener;
use tokio::timer::Interval;

fn bg_task(rx: mpsc::Receiver<usize>) -> impl Future<Item = (), Error = ()> {
    #[derive(Eq, PartialEq)]
    enum Item {
        Value(usize),
        Tick,
        Done,
    }

    let tick_dur = Duration::from_secs(30);

    let interval = Interval::new_interval(tick_dur)
        .map(|_| Item::Tick)
        .map_err(|_| ());

    let items = rx
        .map(Item::Value)
        .chain(stream::once(Ok(Item::Done)))
        .select(interval)
        .take_while(|item| future::ok(*item != Item::Done));

    items
        .fold(0, |num, item| {
            match item {
                Item::Value(v) => future::ok(num + v),
                Item::Tick => {
                    println!("bytes read = {}", num);

                    // reset the byte counter
                    future::ok(0)
                }
                _ => unreachable!(),
            }
        })
        .map(|_| ())
}

fn main() {
    tokio::run(lazy(|| {
        let addr = "127.0.0.1:1234".parse().unwrap();
        let listener = TcpListener::bind(&addr).unwrap();

        let (tx, rx) = mpsc::channel(1_024);

        tokio::spawn(bg_task(rx));

        listener
            .incoming()
            .for_each(move |socket| {
                // inbound socket has been received
                //
                // spawn new task to process a socket
                tokio::spawn({
                    let tx = tx.clone();

                    // write to socket
                    io::read_to_end(socket, vec![])
                        // drop the socket
                        .and_then(move |(_, buf)| {
                            tx.send(buf.len()).map_err(|_| io::ErrorKind::Other.into())
                        })
                        .map(|_| ())
                        // write any error to STDOUT
                        .map_err(|e| println!("socket error = {:?}", e))
                });

                // receive the next inbound socket
                Ok(())
            })
            .map_err(|e| println!("listener error = {:?}", e))
    }));
}
