extern crate futures;
extern crate tokio;

use futures::future::lazy;
use futures::sync::oneshot;
use futures::Future;

fn main() {
    tokio::run(lazy(|| {
        let (tx, rx) = oneshot::channel();

        tokio::spawn(lazy(|| {
            tx.send("hello from spawned task");
            Ok(())
        }));

        rx.and_then(|msg| {
            println!("Got `{}`", msg);
            Ok(())
        })
        .map_err(|e| println!("error = {:?}", e))
    }));
}
