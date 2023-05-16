use std::pin::Pin;

use futures::{Stream, StreamExt};

pub struct Chars {
    pub inner: Pin<Box<dyn Stream<Item = char>>>,
}

impl Chars {
    pub async fn next(&mut self) -> Option<char> {
        self.inner.next().await
    }
}
