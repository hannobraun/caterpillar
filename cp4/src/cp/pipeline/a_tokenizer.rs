use std::pin::Pin;

use futures::{Stream, StreamExt};

pub struct Tokenizer {
    buf: String,
}

impl Tokenizer {
    pub fn new() -> Self {
        Self { buf: String::new() }
    }

    pub async fn tokenize(
        &mut self,
        mut chars: Pin<&mut dyn Stream<Item = char>>,
    ) -> Option<String> {
        loop {
            let ch = match chars.next().await {
                Some(ch) => ch,
                None => {
                    if self.buf.is_empty() {
                        return None;
                    }

                    let token = self.buf.clone();
                    self.buf.clear();
                    return Some(token);
                }
            };

            if ch.is_whitespace() {
                let token = self.buf.clone();
                self.buf.clear();
                return Some(token);
            }

            self.buf.push(ch);
        }
    }
}
