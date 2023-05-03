use std::{pin::Pin, task::Poll};

use futures::{stream, Stream};

pub struct Tokenizer {
    buf: String,
}

impl Tokenizer {
    pub fn new() -> Self {
        Self { buf: String::new() }
    }

    pub async fn tokenize<'r>(
        &'r mut self,
        mut ch: Pin<&'r mut dyn Stream<Item = char>>,
    ) -> impl Stream<Item = String> + 'r {
        stream::poll_fn(move |cx| loop {
            let ch = match ch.as_mut().poll_next(cx) {
                Poll::Ready(Some(ch)) => ch,
                Poll::Ready(None) => {
                    if self.buf.is_empty() {
                        return Poll::Ready(None);
                    }

                    let token = self.buf.clone();
                    self.buf.clear();
                    return Poll::Ready(Some(token));
                }
                Poll::Pending => return Poll::Pending,
            };

            if ch.is_whitespace() {
                let token = self.buf.clone();
                self.buf.clear();
                return Poll::Ready(Some(token));
            }

            self.buf.push(ch);
        })
    }
}
