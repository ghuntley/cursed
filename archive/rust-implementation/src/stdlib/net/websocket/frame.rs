//! WebSocket frame handling

/// WebSocket frame structure
#[derive(Debug, Clone)]
pub struct WebSocketFrame {
    pub opcode: Opcode,
    pub payload: Vec<u8>,
    pub is_final: bool,
    pub is_masked: bool,
}

impl WebSocketFrame {
    pub fn new_text(text: &str) -> Self {
        Self {
            opcode: Opcode::Text,
            payload: text.as_bytes().to_vec(),
            is_final: true,
            is_masked: false,
        }
    }
    
    pub fn new_binary(data: &[u8]) -> Self {
        Self {
            opcode: Opcode::Binary,
            payload: data.to_vec(),
            is_final: true,
            is_masked: false,
        }
    }
    
    pub fn new_close() -> Self {
        Self {
            opcode: Opcode::Close,
            payload: Vec::new(),
            is_final: true,
            is_masked: false,
        }
    }
    
    pub fn new_ping(data: &[u8]) -> Self {
        Self {
            opcode: Opcode::Ping,
            payload: data.to_vec(),
            is_final: true,
            is_masked: false,
        }
    }
    
    pub fn new_pong(data: &[u8]) -> Self {
        Self {
            opcode: Opcode::Pong,
            payload: data.to_vec(),
            is_final: true,
            is_masked: false,
        }
    }
}

/// WebSocket opcodes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    Continuation = 0x0,
    Text = 0x1,
    Binary = 0x2,
    Close = 0x8,
    Ping = 0x9,
    Pong = 0xA,
}

impl From<u8> for Opcode {
    fn from(byte: u8) -> Self {
        match byte & 0x0F {
            0x0 => Opcode::Continuation,
            0x1 => Opcode::Text,
            0x2 => Opcode::Binary,
            0x8 => Opcode::Close,
            0x9 => Opcode::Ping,
            0xA => Opcode::Pong,
            _ => Opcode::Close, // Default to close for unknown opcodes
        }
    }
}

/// Frame type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameType {
    Text,
    Binary,
    Close,
    Ping,
    Pong,
    Continuation,
}

impl From<Opcode> for FrameType {
    fn from(opcode: Opcode) -> Self {
        match opcode {
            Opcode::Text => FrameType::Text,
            Opcode::Binary => FrameType::Binary,
            Opcode::Close => FrameType::Close,
            Opcode::Ping => FrameType::Ping,
            Opcode::Pong => FrameType::Pong,
            Opcode::Continuation => FrameType::Continuation,
        }
    }
}
