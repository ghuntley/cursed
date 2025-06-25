//! Channel implementation

#[derive(Debug)]
pub enum ChannelError {
pub type SendResult<T> = Result<(), ChannelError>;
pub type ReceiveResult<T> = Result<T, ChannelError>;
