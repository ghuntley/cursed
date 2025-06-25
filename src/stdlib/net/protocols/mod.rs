use crate::error::CursedError;
/// Protocol implementations for CURSED networking
/// 
/// This module provides implementations for common network protocols
/// including SMTP, FTP, SSH, and TLS/SSL functionality.

pub mod smtp;
pub mod ftp;
pub mod ssh;
pub mod tls;

// Re-export main types
pub use smtp::{SmtpClient, EmailMessage, SmtpConfig};
pub use ftp::{
    FtpEntry, FtpState, TransferProgress, AsyncFtpClient
// };
pub use ssh::{SshClient, SshCommand, SshKey, SshConfig};
pub use tls::{TlsConfig, TlsVersion, CipherSuite};

/// Common protocol error types
#[derive(Debug, Clone)]
pub enum ProtocolError {
// impl std::fmt::Display for ProtocolError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             ProtocolError::Authentication(msg) => write!(f, "Authentication error: {}", msg),
//             ProtocolError::Connection(msg) => write!(f, "Connection error: {}", msg),
//             ProtocolError::Protocol(msg) => write!(f, "Protocol error: {}", msg),
//             ProtocolError::Timeout(msg) => write!(f, "Timeout error: {}", msg),
//             ProtocolError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
//         }
//     }
// }

// impl std::error::CursedError for ProtocolError {}
// 
pub type ProtocolResult<T> = std::result::Result<T, ProtocolError>;

