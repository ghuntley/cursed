/// WebSocket message types

use crate::stdlib::net::error::{NetError, NetResult, websocket_error};
use crate::stdlib::net::websocket::{WebSocketFrame, Opcode};
use crate::error::Error;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_message() {
        let message = WebSocketMessage::text("Hello, WebSocket!".to_string());
        assert_eq!(message.message_type, MessageType::Text);
        assert!(message.is_text());
        assert!(!message.is_binary());
        assert!(!message.is_control());
        
        let text = message.as_text().unwrap();
        assert_eq!(text, "Hello, WebSocket!");
    }

    #[test]
    fn test_binary_message() {
        let data = vec![0x01, 0x02, 0x03, 0x04];
        let message = WebSocketMessage::binary(data.clone());
        assert_eq!(message.message_type, MessageType::Binary);
        assert!(!message.is_text());
        assert!(message.is_binary());
        assert!(!message.is_control());
        
        assert_eq!(message.as_binary(), &data);
    }

    #[test]
    fn test_control_messages() {
        let ping = WebSocketMessage::ping(vec![1, 2, 3]);
        assert!(ping.is_control());
        assert_eq!(ping.message_type, MessageType::Ping);
        
        let pong = WebSocketMessage::pong(vec![4, 5, 6]);
        assert!(pong.is_control());
        assert_eq!(pong.message_type, MessageType::Pong);
    }

    #[test]
    fn test_message_size() {
        let message = WebSocketMessage::text("Hello".to_string());
        assert_eq!(message.len(), 5);
        assert!(!message.is_empty());
        
        let empty_message = WebSocketMessage::binary(vec![]);
        assert_eq!(empty_message.len(), 0);
        assert!(empty_message.is_empty());
    }

    #[test]
    fn test_frame_conversion() {
        let message = WebSocketMessage::text("Test".to_string());
        let frame = message.to_frame().unwrap();
        assert_eq!(frame.opcode, Opcode::Text);
        assert_eq!(frame.payload, b"Test");
        
        let message2 = WebSocketMessage::from_frame(frame).unwrap();
        assert_eq!(message2.message_type, MessageType::Text);
        assert_eq!(message2.data, b"Test");
    }

    #[test]
    fn test_invalid_text_conversion() {
        let invalid_utf8 = vec![0xFF, 0xFE, 0xFD];
        let message = WebSocketMessage {
            message_type: MessageType::Text,
            data: invalid_utf8,
        };
        
        assert!(message.as_text().is_err());
    }

    #[test]
    fn test_text_from_binary_error() {
        let binary_message = WebSocketMessage::binary(vec![1, 2, 3]);
        assert!(binary_message.as_text().is_err());
    }
}
