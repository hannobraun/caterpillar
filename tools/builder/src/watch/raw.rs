use tokio_stream::wrappers::UnboundedReceiverStream;

pub type RawChanges = UnboundedReceiverStream<()>;
