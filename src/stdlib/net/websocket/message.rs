//! WebSocket message handling

/// WebSocket message
#[derive(Debug, Clone)]
pub struct WebSocketMessage {
    pub message_type: MessageType,
    pub data: Vec<u8>,
}

impl WebSocketMessage {
    pub fn text(text: &str) -> Self {
        Self {
            message_type: MessageType::Text,
            data: text.as_bytes().to_vec(),
        }
    }
    
    pub fn binary(data: &[u8]) -> Self {
        Self {
            message_type: MessageType::Binary,
            data: data.to_vec(),
        }
    }
    
    pub fn close() -> Self {
        Self {
            message_type: MessageType::Close,
            data: Vec::new(),
        }
    }
    
    pub fn as_text(&self) -> Option<String> {
        if self.message_type == MessageType::Text {
            String::from_utf8(self.data.clone()).ok()
        } else {
            None
        }
    }
    
    pub fn as_binary(&self) -> Option<&[u8]> {
        if self.message_type == MessageType::Binary {
            Some(&self.data)
        } else {
            None
        }
    }
}

/// WebSocket message type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    Text,
    Binary,
    Close,
    Ping,
    Pong,
}

impl std::fmt::Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageType::Text => write!(f, "Text"),
            MessageType::Binary => write!(f, "Binary"),
            MessageType::Close => write!(f, "Close"),
            MessageType::Ping => write!(f, "Ping"),
            MessageType::Pong => write!(f, "Pong"),
        }
    }
}
