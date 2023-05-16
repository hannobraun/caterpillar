use std::pin::Pin;

use futures::Stream;

pub type Chars = Pin<Box<dyn Stream<Item = char>>>;
