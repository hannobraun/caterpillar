use std::pin::Pin;

use futures::Stream;

pub struct Chars {
    pub inner: Pin<Box<dyn Stream<Item = char>>>,
}
