extern crate futures;
extern crate tokio;

use futures::future::lazy;
use futures::sync::mpsc;
use futures::{stream, Future, Sink, Stream};

fn main() {
    tokio::run(lazy(|| {
        let (tx, rx) = mpsc::channel(1_024);

        tokio::spawn(lazy(|| {
            stream::iter_ok(0..10).fold(tx, |tx, i| {
                tx.send(format!("Message {} from spawned task", i))
                    .map_err(|e| println!("error = {:?}", e))
            })
            .map(|_| ()) // drop the tx handle
        }));

        rx.for_each(|msg| {
            println!("Got `{}`", msg);
            Ok(())
        })
    }));
}
