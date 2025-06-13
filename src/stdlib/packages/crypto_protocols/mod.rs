/// Cryptographic Protocols Package - Production Implementation
pub mod key_exchange;
pub mod ecdh;
pub mod diffie_hellman;
pub mod key_agreement;
pub mod authentication;
pub mod secure_channels;
pub mod signal_protocol;
pub mod noise_protocol;
pub mod tls_handshake;
pub mod session_management;
pub mod forward_secrecy;
pub mod attack_resistance;
pub mod side_channel_protection;
pub mod protocol_verification;
pub mod key_derivation;

// Re-export main types and functions
pub use key_exchange::{KeyExchangeManager, KeyExchangeProtocol, KeyExchangeResult};
pub use ecdh::{EcdhManager, EcdhCurve, EcdhKeyPair, EcdhSharedSecret};
pub use diffie_hellman::{DiffieHellmanManager, DhGroup, DhKeyPair, DhSharedSecret};
pub use authentication::{AuthenticationManager, AuthMethod, MfaConfig, AuthResult};
pub use secure_channels::{SecureChannelManager, ChannelType, SecurityLevel, SecureChannel};
pub use signal_protocol::{SignalProtocolManager, SignalKeyBundle, SignalMessage};
pub use tls_handshake::{TlsHandshakeManager, TlsVersion, TlsCipherSuite, TlsHandshakeSession};
pub use session_management::{SessionManager, CryptoSession, SessionTicket, SessionConfig};

use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;

/// Cryptographic protocols package initialization
pub fn init_crypto_protocols() -> AdvancedCryptoResult<()> {
    println!("🔐 Cryptographic protocols package initialized - secure communications ready!");
    Ok(())
}

/// Create a comprehensive cryptographic protocols suite
pub fn create_protocol_suite() -> AdvancedCryptoResult<CryptoProtocolSuite> {
    Ok(CryptoProtocolSuite {
        key_exchange: KeyExchangeManager::new()?,
        ecdh: EcdhManager::new()?,
        diffie_hellman: DiffieHellmanManager::new()?,
        authentication: AuthenticationManager::new()?,
        secure_channels: SecureChannelManager::new()?,
        signal_protocol: SignalProtocolManager::new()?,
        tls_handshake: TlsHandshakeManager::new()?,
        session_management: SessionManager::new()?,
    })
}

/// Comprehensive cryptographic protocols suite
#[derive(Debug)]
pub struct CryptoProtocolSuite {
    pub key_exchange: KeyExchangeManager,
    pub ecdh: EcdhManager,
    pub diffie_hellman: DiffieHellmanManager,
    pub authentication: AuthenticationManager,
    pub secure_channels: SecureChannelManager,
    pub signal_protocol: SignalProtocolManager,
    pub tls_handshake: TlsHandshakeManager,
    pub session_management: SessionManager,
}
