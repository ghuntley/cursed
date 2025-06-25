// Inter-process communication for CURSED
use std::collections::HashMap;
use std::sync::mpsc;
use crate::error::CursedError;

/// IPC channel for communication between processes
#[derive(Debug)]
pub struct IpcChannel {
/// IPC message
#[derive(Debug, Clone)]
pub struct IpcMessage {
/// Type of IPC message
#[derive(Debug, Clone, PartialEq)]
pub enum IpcMessageType {
/// IPC error types
#[derive(Debug, Clone)]
pub enum IpcError {
// impl std::fmt::Display for IpcError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             IpcError::ChannelClosed => write!(f, "IPC channel is closed"),
//             IpcError::SendTimeout => write!(f, "Send operation timed out"),
//             IpcError::ReceiveTimeout => write!(f, "Receive operation timed out"),
//             IpcError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
//             IpcError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
//             IpcError::PermissionDenied => write!(f, "Permission denied"),
//             IpcError::InvalidMessage => write!(f, "Invalid message format"),
//         }
//     }
// }

// impl std::error::CursedError for IpcError {}
// 
impl IpcChannel {
    /// Create a new IPC channel
    pub fn new(channel_id: String) -> (Self, mpsc::Sender<IpcMessage>) {
        let (tx, rx) = mpsc::channel();
        let channel = Self {
        (channel, tx)
    /// Send a message through the channel
    pub fn send(&self, message: IpcMessage) -> Result<(), IpcError> {
        self.sender.send(message)
            .map_err(|_| IpcError::ChannelClosed)
    /// Receive a message from the channel
    pub fn receive(&self) -> Result<IpcMessage, IpcError> {
        self.receiver.recv()
            .map_err(|_| IpcError::ChannelClosed)
    /// Try to receive a message without blocking
    pub fn try_receive(&self) -> Result<Option<IpcMessage>, IpcError> {
        match self.receiver.try_recv() {
        }
    }
    
    /// Close the channel
    pub fn close(&mut self) {
        self.connected = false;
    /// Check if channel is connected
    pub fn is_connected(&self) -> bool {
        self.connected
    }
}

impl IpcMessage {
    /// Create a new IPC message
    pub fn new<S: Into<String>>(
        data: Vec<u8>
    ) -> Self {
        Self {
        }
    }
    
    /// Create a request message
    pub fn request<S: Into<String>>(source: S, destination: S, data: Vec<u8>) -> Self {
        Self::new(source, destination, IpcMessageType::Request, data)
    /// Create a response message
    pub fn response<S: Into<String>>(source: S, destination: S, data: Vec<u8>) -> Self {
        Self::new(source, destination, IpcMessageType::Response, data)
    /// Create a notification message
    pub fn notification<S: Into<String>>(source: S, destination: S, data: Vec<u8>) -> Self {
        Self::new(source, destination, IpcMessageType::Notification, data)
    /// Create an error message
    pub fn error<S: Into<String>>(source: S, destination: S, data: Vec<u8>) -> Self {
        Self::new(source, destination, IpcMessageType::CursedError, data)
    /// Get message as string
    pub fn as_string(&self) -> Result<String, IpcError> {
        String::from_utf8(self.data.clone())
            .map_err(|e| IpcError::SerializationError(e.to_string()))
    /// Set message data from string
    pub fn from_string<S: Into<String>>(mut self, data: S) -> Self {
        self.data = data.into().into_bytes();
        self
    /// Get message age
    pub fn age(&self) -> std::time::Duration {
        self.timestamp.elapsed().unwrap_or_default()
    }
}
