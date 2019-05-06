extern crate futures;
extern crate tokio;

use futures::{stream, Stream};
use std::time::Duration;
use tokio::timer::Interval;

struct Fibonacci {
    curr: u64,
    next: u64,
}

fn main() {
    let mut fib = Fibonacci { curr: 1, next: 1 };

    let future = Interval::new_interval(Duration::from_secs(1)).map(move |_| {
        let curr = fib.curr;
        let next = curr + fib.next;

        fib.curr = fib.next;
        fib.next = next;

        curr
    });

    tokio::run(future.take(10).map_err(|_| ()).for_each(|num| {
        println!("{}", num);
        Ok(())
    }));

    // use concrete streams to convert values and iterators info streams
    let values = vec!["one", "two", "three"];

    tokio::run(stream::iter_ok(values).for_each(|value| {
        println!("{}", value);
        Ok(())
    }))
}
