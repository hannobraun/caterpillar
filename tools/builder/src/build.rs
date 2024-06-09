use tokio_stream::{Stream, StreamExt};

pub async fn build(mut changes: impl Stream<Item = ()> + Unpin) {
    while let Some(event) = changes.next().await {
        dbg!(event);
    }
}
