/// WebSocket message types

// use crate::stdlib::net::error::{NetError, NetResult, websocket_error};
// use crate::stdlib::net::websocket::{WebSocketFrame, Opcode};
use crate::error::CursedError;

/// WebSocket message types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    Text,
    Binary,
    Close,
    Ping,
    Pong,
}

/// WebSocket message
#[derive(Debug, Clone)]
pub struct WebSocketMessage {
    pub message_type: MessageType,
    pub data: Vec<u8>,
}

impl WebSocketMessage {
    /// Create a text message
    pub fn text(text: String) -> Self {
        Self {
            message_type: MessageType::Text,
            data: text.into_bytes(),
        }
    }
    
    /// Create a binary message
    pub fn binary(data: Vec<u8>) -> Self {
        Self {
            message_type: MessageType::Binary,
            data,
        }
    }
    
    /// Create a ping message
    pub fn ping(data: Vec<u8>) -> Self {
        Self {
            message_type: MessageType::Ping,
            data,
        }
    }
    
    /// Create a pong message
    pub fn pong(data: Vec<u8>) -> Self {
        Self {
            message_type: MessageType::Pong,
            data,
        }
    }
    
    /// Get message as text
    pub fn as_text(&self) -> NetResult<String> {
        if self.message_type != MessageType::Text {
            return Err(websocket_error("Message is not text", None, None));
        }
        
        String::from_utf8(self.data.clone())
            .map_err(|e| websocket_error(&format!("Invalid UTF-8: {}", e), None, None))
    }
    
    /// Get message as binary data
    pub fn as_binary(&self) -> &[u8] {
        &self.data
    }
    
    /// Check if message is text
    pub fn is_text(&self) -> bool {
        self.message_type == MessageType::Text
    }
    
    /// Check if message is binary
    pub fn is_binary(&self) -> bool {
        self.message_type == MessageType::Binary
    }
    
    /// Check if message is a control message
    pub fn is_control(&self) -> bool {
        matches!(self.message_type, MessageType::Close | MessageType::Ping | MessageType::Pong)
    }
    
    /// Convert message to WebSocket frame
    pub fn to_frame(&self) -> NetResult<WebSocketFrame> {
        let opcode = match self.message_type {
            MessageType::Text => Opcode::Text,
            MessageType::Binary => Opcode::Binary,
            MessageType::Ping => Opcode::Ping,
            MessageType::Pong => Opcode::Pong,
            MessageType::Close => Opcode::Close,
        };
        
        Ok(WebSocketFrame {
            fin: true,
            opcode,
            masked: true,
            payload: self.data.clone(),
        })
    }
    
    /// Create message from WebSocket frame
    pub fn from_frame(frame: WebSocketFrame) -> NetResult<Self> {
        let message_type = match frame.opcode {
            Opcode::Text => MessageType::Text,
            Opcode::Binary => MessageType::Binary,
            Opcode::Ping => MessageType::Ping,
            Opcode::Pong => MessageType::Pong,
            Opcode::Close => MessageType::Close,
            _ => return Err(websocket_error("Unsupported frame opcode", None, None)),
        };
        
        Ok(Self {
            message_type,
            data: frame.payload,
        })
    }
    
    /// Get message size in bytes
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    /// Check if message is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

