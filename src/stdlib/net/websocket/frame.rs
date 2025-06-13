/// WebSocket frame implementation

use crate::stdlib::net::error::{NetError, NetResult, websocket_error};
use crate::stdlib::net::socket::TcpSocket;
use crate::stdlib::net::websocket::CloseCode;

/// WebSocket frame opcodes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    Continuation = 0x0,
    Text = 0x1,
    Binary = 0x2,
    Close = 0x8,
    Ping = 0x9,
    Pong = 0xA,
}

/// WebSocket frame types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameType {
    Text,
    Binary,
    Close,
    Ping,
    Pong,
    Continuation,
}

/// WebSocket frame
#[derive(Debug, Clone)]
pub struct WebSocketFrame {
    pub fin: bool,
    pub opcode: Opcode,
    pub masked: bool,
    pub payload: Vec<u8>,
}

impl WebSocketFrame {
    pub fn text(data: String) -> Self {
        Self {
            fin: true,
            opcode: Opcode::Text,
            masked: true,
            payload: data.into_bytes(),
        }
    }
    
    pub fn binary(data: Vec<u8>) -> Self {
        Self {
            fin: true,
            opcode: Opcode::Binary,
            masked: true,
            payload: data,
        }
    }
    
    pub fn close(code: CloseCode, reason: &str) -> Self {
        let mut payload = Vec::new();
        payload.extend_from_slice(&code.as_u16().to_be_bytes());
        payload.extend_from_slice(reason.as_bytes());
        
        Self {
            fin: true,
            opcode: Opcode::Close,
            masked: true,
            payload,
        }
    }
    
    pub fn ping(data: Vec<u8>) -> Self {
        Self {
            fin: true,
            opcode: Opcode::Ping,
            masked: true,
            payload: data,
        }
    }
    
    pub fn pong(data: Vec<u8>) -> Self {
        Self {
            fin: true,
            opcode: Opcode::Pong,
            masked: true,
            payload: data,
        }
    }
    
    pub fn to_bytes(&self) -> NetResult<Vec<u8>> {
        let mut frame = Vec::new();
        
        // First byte: FIN + RSV + Opcode
        let mut first_byte = self.opcode as u8;
        if self.fin {
            first_byte |= 0x80;
        }
        frame.push(first_byte);
        
        // Second byte: MASK + Payload length
        let payload_len = self.payload.len();
        let mut second_byte = 0u8;
        if self.masked {
            second_byte |= 0x80;
        }
        
        if payload_len < 126 {
            second_byte |= payload_len as u8;
            frame.push(second_byte);
        } else if payload_len <= 65535 {
            second_byte |= 126;
            frame.push(second_byte);
            frame.extend_from_slice(&(payload_len as u16).to_be_bytes());
        } else {
            second_byte |= 127;
            frame.push(second_byte);
            frame.extend_from_slice(&(payload_len as u64).to_be_bytes());
        }
        
        // Masking key and payload
        if self.masked {
            let mask = [0x12, 0x34, 0x56, 0x78]; // Simplified mask
            frame.extend_from_slice(&mask);
            
            // Apply mask to payload
            for (i, &byte) in self.payload.iter().enumerate() {
                frame.push(byte ^ mask[i % 4]);
            }
        } else {
            frame.extend_from_slice(&self.payload);
        }
        
        Ok(frame)
    }
    
    pub fn from_socket(socket: &TcpSocket) -> NetResult<Self> {
        // Read frame header
        let mut header = [0u8; 2];
        socket.read_exact(&mut header)?;
        
        let fin = (header[0] & 0x80) != 0;
        let opcode = match header[0] & 0x0F {
            0x0 => Opcode::Continuation,
            0x1 => Opcode::Text,
            0x2 => Opcode::Binary,
            0x8 => Opcode::Close,
            0x9 => Opcode::Ping,
            0xA => Opcode::Pong,
            _ => return Err(websocket_error("Invalid opcode", None, None)),
        };
        
        let masked = (header[1] & 0x80) != 0;
        let mut payload_len = (header[1] & 0x7F) as u64;
        
        // Extended payload length
        if payload_len == 126 {
            let mut len_bytes = [0u8; 2];
            socket.read_exact(&mut len_bytes)?;
            payload_len = u16::from_be_bytes(len_bytes) as u64;
        } else if payload_len == 127 {
            let mut len_bytes = [0u8; 8];
            socket.read_exact(&mut len_bytes)?;
            payload_len = u64::from_be_bytes(len_bytes);
        }
        
        // Masking key
        let mask = if masked {
            let mut mask_bytes = [0u8; 4];
            socket.read_exact(&mut mask_bytes)?;
            Some(mask_bytes)
        } else {
            None
        };
        
        // Payload
        let mut payload = vec![0u8; payload_len as usize];
        socket.read_exact(&mut payload)?;
        
        // Unmask payload if needed
        if let Some(mask) = mask {
            for (i, byte) in payload.iter_mut().enumerate() {
                *byte ^= mask[i % 4];
            }
        }
        
        Ok(Self {
            fin,
            opcode,
            masked,
            payload,
        })
    }
    
    pub fn frame_type(&self) -> FrameType {
        match self.opcode {
            Opcode::Text => FrameType::Text,
            Opcode::Binary => FrameType::Binary,
            Opcode::Close => FrameType::Close,
            Opcode::Ping => FrameType::Ping,
            Opcode::Pong => FrameType::Pong,
            Opcode::Continuation => FrameType::Continuation,
        }
    }
    
    pub fn is_control_frame(&self) -> bool {
        matches!(self.opcode, Opcode::Close | Opcode::Ping | Opcode::Pong)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_creation() {
        let frame = WebSocketFrame::text("Hello".to_string());
        assert_eq!(frame.opcode, Opcode::Text);
        assert!(frame.fin);
        assert!(frame.masked);
        assert_eq!(frame.payload, b"Hello");
    }

    #[test]
    fn test_frame_types() {
        let text_frame = WebSocketFrame::text("test".to_string());
        assert_eq!(text_frame.frame_type(), FrameType::Text);
        assert!(!text_frame.is_control_frame());
        
        let ping_frame = WebSocketFrame::ping(vec![1, 2, 3]);
        assert_eq!(ping_frame.frame_type(), FrameType::Ping);
        assert!(ping_frame.is_control_frame());
    }

    #[test]
    fn test_close_frame() {
        let close_frame = WebSocketFrame::close(CloseCode::NORMAL, "Goodbye");
        assert_eq!(close_frame.opcode, Opcode::Close);
        assert!(close_frame.payload.len() >= 2); // At least close code
    }
}
