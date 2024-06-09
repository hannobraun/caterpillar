use tokio_stream::wrappers::UnboundedReceiverStream;

pub type Changes = UnboundedReceiverStream<()>;
