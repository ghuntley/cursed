use crate::error::CursedError;
/// WebSocket implementation for CURSED networking
/// 
/// This module provides WebSocket client and server functionality with support
/// for real-time bidirectional communication, frame handling, compression,
/// and connection management.

pub mod client;
pub mod server;
pub mod frame;
pub mod message;
pub mod config;

// Re-export main types
pub use client::{WebSocketClient, WebSocketClientBuilder};
pub use server::{WebSocketServer, WebSocketListener};
pub use frame::{WebSocketFrame, FrameType, Opcode};
pub use message::{WebSocketMessage, MessageType};
pub use config::{WebSocketConfig, CompressionConfig};

/// WebSocket close codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CloseCode(pub u16);

impl CloseCode {
    pub const NORMAL: CloseCode = CloseCode(1000);
    pub const GOING_AWAY: CloseCode = CloseCode(1001);
    pub const PROTOCOL_ERROR: CloseCode = CloseCode(1002);
    pub const UNSUPPORTED_DATA: CloseCode = CloseCode(1003);
    pub const NO_STATUS_RECEIVED: CloseCode = CloseCode(1005);
    pub const ABNORMAL_CLOSURE: CloseCode = CloseCode(1006);
    pub const INVALID_FRAME_PAYLOAD_DATA: CloseCode = CloseCode(1007);
    pub const POLICY_VIOLATION: CloseCode = CloseCode(1008);
    pub const MESSAGE_TOO_BIG: CloseCode = CloseCode(1009);
    pub const MANDATORY_EXTENSION: CloseCode = CloseCode(1010);
    pub const INTERNAL_ERROR: CloseCode = CloseCode(1011);
    pub const SERVICE_RESTART: CloseCode = CloseCode(1012);
    pub const TRY_AGAIN_LATER: CloseCode = CloseCode(1013);
    pub const BAD_GATEWAY: CloseCode = CloseCode(1014);
    pub const TLS_HANDSHAKE: CloseCode = CloseCode(1015);
    
    pub fn as_u16(&self) -> u16 {
        self.0
    }
    
    pub fn reason(&self) -> &'static str {
        match self.0 {
            1000 => "Normal Closure",
            1001 => "Going Away",
            1002 => "Protocol Error",
            1003 => "Unsupported Data",
            1005 => "No Status Received",
            1006 => "Abnormal Closure",
            1007 => "Invalid Frame Payload Data",
            1008 => "Policy Violation",
            1009 => "Message Too Big",
            1010 => "Mandatory Extension",
            1011 => "Internal Error",
            1012 => "Service Restart",
            1013 => "Try Again Later",
            1014 => "Bad Gateway",
            1015 => "TLS Handshake",
            _ => "Unknown",
        }
    }
}

impl std::fmt::Display for CloseCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0, self.reason())
    }
}

/// WebSocket connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Connecting,
    Open,
    Closing,
    Closed,
}
