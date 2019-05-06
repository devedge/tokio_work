extern crate futures;
extern crate tokio;

use futures::{stream, Stream};

fn fibonacci() -> impl Stream<Item = u64, Error = ()> {
    stream::unfold((1, 1), |(curr, next)| {
        let new_next = curr + next;

        Some(Ok((curr, (next, new_next))))
    })
}

fn main() {
    tokio::run(fibonacci().take(10).for_each(|num| {
        println!("{}", num);
        Ok(())
    }));
}
