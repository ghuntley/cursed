/// TLS Handshake Protocol Implementation
use crate::error::CursedError;
use crate::stdlib::packages::crypto_advanced::AdvancedCryptoResult;
use crate::stdlib::packages::crypto_random::SecureRandom;
use crate::stdlib::packages::crypto_hash_advanced::HashRegistry;
use crate::error::Error;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use std::fmt;

/// TLS protocol versions
#[derive(Debug, Clone, PartialEq, Ord, PartialOrd, Eq)]
pub enum TlsVersion {
    Tls10 = 0x0301,
    Tls11 = 0x0302,
    Tls12 = 0x0303,
    Tls13 = 0x0304,
}

/// TLS cipher suites
#[derive(Debug, Clone, PartialEq)]
pub enum TlsCipherSuite {
    // TLS 1.2
    AES128GcmSha256,
    AES256GcmSha384,
    ChaCha20Poly1305Sha256,
    AES128CbcSha256,
    AES256CbcSha256,
    
    // TLS 1.3
    Aes128GcmSha256,
    Aes256GcmSha384,
    Aes128CcmSha256,
    Aes128Ccm8Sha256,
}

/// TLS handshake message types
#[derive(Debug, Clone, PartialEq)]
pub enum TlsHandshakeType {
    ClientHello = 1,
    ServerHello = 2,
    Certificate = 11,
    ServerKeyExchange = 12,
    CertificateRequest = 13,
    ServerHelloDone = 14,
    CertificateVerify = 15,
    ClientKeyExchange = 16,
    Finished = 20,
    // TLS 1.3 - use distinct values to avoid conflicts
    EncryptedExtensions = 8,
    CertificateRequest13 = 113, // 100 + 13 for TLS 1.3 variant
    Certificate13 = 111,        // 100 + 11 for TLS 1.3 variant  
    CertificateVerify13 = 115,  // 100 + 15 for TLS 1.3 variant
    Finished13 = 120,           // 100 + 20 for TLS 1.3 variant
}

/// TLS handshake state
#[derive(Debug, Clone, PartialEq)]
pub enum TlsHandshakeState {
    Initial,
    ClientHelloSent,
    ServerHelloReceived,
    CertificateReceived,
    ServerKeyExchangeReceived,
    ServerHelloDoneReceived,
    ClientKeyExchangeSent,
    ClientFinishedSent,
    ServerFinishedReceived,
    Established,
    Failed,
}

/// TLS handshake message
#[derive(Debug, Clone)]
pub struct TlsHandshakeMessage {
    pub message_type: TlsHandshakeType,
    pub length: u32,
    pub payload: Vec<u8>,
    pub timestamp: SystemTime,
}

/// TLS session configuration
#[derive(Debug, Clone)]
pub struct TlsConfig {
    pub version: TlsVersion,
    pub cipher_suites: Vec<TlsCipherSuite>,
    pub server_name: Option<String>,
    pub verify_peer: bool,
    pub client_auth: bool,
    pub session_timeout: Duration,
    pub max_handshake_time: Duration,
}

/// TLS session keys
#[derive(Debug, Clone)]
pub struct TlsSessionKeys {
    pub master_secret: Vec<u8>,
    pub client_write_key: Vec<u8>,
    pub server_write_key: Vec<u8>,
    pub client_write_iv: Vec<u8>,
    pub server_write_iv: Vec<u8>,
    pub client_mac_key: Vec<u8>,
    pub server_mac_key: Vec<u8>,
}

/// TLS handshake session
#[derive(Debug, Clone)]
pub struct TlsHandshakeSession {
    pub session_id: String,
    pub state: TlsHandshakeState,
    pub config: TlsConfig,
    pub selected_version: Option<TlsVersion>,
    pub selected_cipher_suite: Option<TlsCipherSuite>,
    pub client_random: Vec<u8>,
    pub server_random: Vec<u8>,
    pub pre_master_secret: Option<Vec<u8>>,
    pub session_keys: Option<TlsSessionKeys>,
    pub handshake_messages: Vec<TlsHandshakeMessage>,
    pub created_at: SystemTime,
    pub last_activity: SystemTime,
    pub is_client: bool,
}

/// TLS handshake manager
#[derive(Debug)]
pub struct TlsHandshakeManager {
    sessions: Arc<Mutex<HashMap<String, TlsHandshakeSession>>>,
    secure_random: SecureRandom,
    hash_manager: HashRegistry,
    default_config: TlsConfig,
}

impl TlsHandshakeManager {
    /// Create new TLS handshake manager
    pub fn new() -> AdvancedCryptoResult<Self> {
        let default_config = TlsConfig {
            version: TlsVersion::Tls12,
            cipher_suites: vec![
                TlsCipherSuite::AES256GcmSha384,
                TlsCipherSuite::AES128GcmSha256,
                TlsCipherSuite::ChaCha20Poly1305Sha256,
            ],
            server_name: None,
            verify_peer: true,
            client_auth: false,
            session_timeout: Duration::from_secs(300), // 5 minutes
            max_handshake_time: Duration::from_secs(30),
        };

        Ok(Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            secure_random: SecureRandom::new()?,
            hash_manager: HashRegistry::new()?,
            default_config,
        })
    }

    /// Start TLS handshake as client
    pub fn start_client_handshake(&self, config: Option<TlsConfig>) -> AdvancedCryptoResult<(String, TlsHandshakeMessage)> {
        let config = config.unwrap_or_else(|| self.default_config.clone());
        let session_id = self.generate_session_id()?;
        
        let client_random = self.secure_random.generate_bytes(32)?;
        
        let session = TlsHandshakeSession {
            session_id: session_id.clone(),
            state: TlsHandshakeState::Initial,
            config: config.clone(),
            selected_version: None,
            selected_cipher_suite: None,
            client_random: client_random.clone(),
            server_random: vec![],
            pre_master_secret: None,
            session_keys: None,
            handshake_messages: vec![],
            created_at: SystemTime::now(),
            last_activity: SystemTime::now(),
            is_client: true,
        };

        // Create ClientHello message
        let client_hello = self.create_client_hello(&config, &client_random)?;
        
        // Update session state
        let mut sessions = self.sessions.lock().map_err(|_| {
            CursedError::system_error("Failed to acquire sessions lock".to_string())
        })?;
        
        let mut updated_session = session;
        updated_session.state = TlsHandshakeState::ClientHelloSent;
        updated_session.handshake_messages.push(client_hello.clone());
        sessions.insert(session_id.clone(), updated_session);

        Ok((session_id, client_hello))
    }

    /// Start TLS handshake as server
    pub fn start_server_handshake(&self, config: Option<TlsConfig>) -> AdvancedCryptoResult<String> {
        let config = config.unwrap_or_else(|| self.default_config.clone());
        let session_id = self.generate_session_id()?;
        
        let session = TlsHandshakeSession {
            session_id: session_id.clone(),
            state: TlsHandshakeState::Initial,
            config,
            selected_version: None,
            selected_cipher_suite: None,
            client_random: vec![],
            server_random: vec![],
            pre_master_secret: None,
            session_keys: None,
            handshake_messages: vec![],
            created_at: SystemTime::now(),
            last_activity: SystemTime::now(),
            is_client: false,
        };

        let mut sessions = self.sessions.lock().map_err(|_| {
            CursedError::system_error("Failed to acquire sessions lock".to_string())
        })?;
        sessions.insert(session_id.clone(), session);

        Ok(session_id)
    }

    /// Process handshake message
    pub fn process_handshake_message(&self, session_id: &str, message: TlsHandshakeMessage) -> AdvancedCryptoResult<Option<TlsHandshakeMessage>> {
        let mut sessions = self.sessions.lock().map_err(|_| {
            CursedError::system_error("Failed to acquire sessions lock".to_string())
        })?;
        
        let session = sessions.get_mut(session_id).ok_or_else(|| {
            CursedError::runtime_error(format!("Session not found: {}", session_id))
        })?;

        // Check handshake timeout
        let elapsed = SystemTime::now().duration_since(session.created_at)
            .unwrap_or(Duration::from_secs(0));
        if elapsed > session.config.max_handshake_time {
            session.state = TlsHandshakeState::Failed;
            return Err(CursedError::runtime_error("Handshake timeout".to_string()));
        }

        session.last_activity = SystemTime::now();
        session.handshake_messages.push(message.clone());

        let response = match (&session.state, &message.message_type, session.is_client) {
            // Client processing server messages
            (TlsHandshakeState::ClientHelloSent, TlsHandshakeType::ServerHello, true) => {
                self.process_server_hello(session, &message)?
            },
            (TlsHandshakeState::ServerHelloReceived, TlsHandshakeType::Certificate, true) => {
                self.process_server_certificate(session, &message)?
            },
            (TlsHandshakeState::CertificateReceived, TlsHandshakeType::ServerHelloDone, true) => {
                self.process_server_hello_done(session)?
            },
            (TlsHandshakeState::ClientKeyExchangeSent, TlsHandshakeType::Finished, true) => {
                self.process_server_finished(session, &message)?
            },
            
            // Server processing client messages
            (TlsHandshakeState::Initial, TlsHandshakeType::ClientHello, false) => {
                self.process_client_hello(session, &message)?
            },
            (TlsHandshakeState::ServerHelloDoneReceived, TlsHandshakeType::ClientKeyExchange, false) => {
                self.process_client_key_exchange(session, &message)?
            },
            (TlsHandshakeState::ClientKeyExchangeSent, TlsHandshakeType::Finished, false) => {
                self.process_client_finished(session, &message)?
            },
            
            _ => {
                session.state = TlsHandshakeState::Failed;
                return Err(CursedError::runtime_error(format!("Unexpected message type: {:?} in state: {:?}", message.message_type, session.state)));
            }
        };

        Ok(response)
    }

    /// Get handshake session
    pub fn get_session(&self, session_id: &str) -> AdvancedCryptoResult<Option<TlsHandshakeSession>> {
        let sessions = self.sessions.lock().map_err(|_| {
            CursedError::system_error("Failed to acquire sessions lock".to_string())
        })?;
        
        Ok(sessions.get(session_id).cloned())
    }

    /// Check if handshake is complete
    pub fn is_handshake_complete(&self, session_id: &str) -> AdvancedCryptoResult<bool> {
        let sessions = self.sessions.lock().map_err(|_| {
            CursedError::system_error("Failed to acquire sessions lock".to_string())
        })?;
        
        if let Some(session) = sessions.get(session_id) {
            Ok(session.state == TlsHandshakeState::Established)
        } else {
            Ok(false)
        }
    }

    /// Get session keys after successful handshake
    pub fn get_session_keys(&self, session_id: &str) -> AdvancedCryptoResult<Option<TlsSessionKeys>> {
        let sessions = self.sessions.lock().map_err(|_| {
            CursedError::system_error("Failed to acquire sessions lock".to_string())
        })?;
        
        if let Some(session) = sessions.get(session_id) {
            if session.state == TlsHandshakeState::Established {
                Ok(session.session_keys.clone())
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    // Private helper methods

    fn generate_session_id(&self) -> AdvancedCryptoResult<String> {
        let random_bytes = self.secure_random.generate_bytes(16)?;
        Ok(self.bytes_to_hex(&random_bytes))
    }

    fn create_client_hello(&self, config: &TlsConfig, client_random: &[u8]) -> AdvancedCryptoResult<TlsHandshakeMessage> {
        // Simplified ClientHello construction
        let mut payload = Vec::new();
        
        // Protocol version
        payload.extend_from_slice(&(config.version.clone() as u16).to_be_bytes());
        
        // Random
        payload.extend_from_slice(client_random);
        
        // Session ID (empty for new session)
        payload.push(0);
        
        // Cipher suites
        payload.extend_from_slice(&((config.cipher_suites.len() * 2) as u16).to_be_bytes());
        for suite in &config.cipher_suites {
            payload.extend_from_slice(&self.cipher_suite_to_bytes(suite));
        }
        
        // Compression methods (null compression)
        payload.push(1);
        payload.push(0);

        Ok(TlsHandshakeMessage {
            message_type: TlsHandshakeType::ClientHello,
            length: payload.len() as u32,
            payload,
            timestamp: SystemTime::now(),
        })
    }

    fn process_server_hello(&self, session: &mut TlsHandshakeSession, message: &TlsHandshakeMessage) -> AdvancedCryptoResult<Option<TlsHandshakeMessage>> {
        // Parse ServerHello (simplified)
        if message.payload.len() < 35 {
            return Err(CursedError::invalid_input("Invalid ServerHello message".to_string()));
        }

        // Extract server random
        session.server_random = message.payload[2..34].to_vec();
        
        // Extract selected cipher suite (simplified)
        session.selected_cipher_suite = Some(TlsCipherSuite::AES256GcmSha384);
        session.selected_version = Some(TlsVersion::Tls12);
        
        session.state = TlsHandshakeState::ServerHelloReceived;
        Ok(None)
    }

    fn process_server_certificate(&self, session: &mut TlsHandshakeSession, _message: &TlsHandshakeMessage) -> AdvancedCryptoResult<Option<TlsHandshakeMessage>> {
        // Simplified certificate processing
        session.state = TlsHandshakeState::CertificateReceived;
        Ok(None)
    }

    fn process_server_hello_done(&self, session: &mut TlsHandshakeSession) -> AdvancedCryptoResult<Option<TlsHandshakeMessage>> {
        // Generate pre-master secret
        session.pre_master_secret = Some(self.secure_random.generate_bytes(48)?);
        
        // Create ClientKeyExchange
        let client_key_exchange = self.create_client_key_exchange(session)?;
        session.state = TlsHandshakeState::ClientKeyExchangeSent;
        
        // Derive session keys
        self.derive_session_keys(session)?;
        
        Ok(Some(client_key_exchange))
    }

    fn process_server_finished(&self, session: &mut TlsHandshakeSession, _message: &TlsHandshakeMessage) -> AdvancedCryptoResult<Option<TlsHandshakeMessage>> {
        session.state = TlsHandshakeState::Established;
        Ok(None)
    }

    fn process_client_hello(&self, session: &mut TlsHandshakeSession, message: &TlsHandshakeMessage) -> AdvancedCryptoResult<Option<TlsHandshakeMessage>> {
        // Parse ClientHello and extract client random
        if message.payload.len() < 35 {
            return Err(CursedError::invalid_input("Invalid ClientHello message".to_string()));
        }

        session.client_random = message.payload[2..34].to_vec();
        session.server_random = self.secure_random.generate_bytes(32)?;
        
        // Select cipher suite and create ServerHello
        session.selected_cipher_suite = Some(TlsCipherSuite::AES256GcmSha384);
        session.selected_version = Some(TlsVersion::Tls12);
        
        let server_hello = self.create_server_hello(session)?;
        session.state = TlsHandshakeState::ServerHelloReceived;
        
        Ok(Some(server_hello))
    }

    fn process_client_key_exchange(&self, session: &mut TlsHandshakeSession, _message: &TlsHandshakeMessage) -> AdvancedCryptoResult<Option<TlsHandshakeMessage>> {
        // Extract pre-master secret from ClientKeyExchange
        session.pre_master_secret = Some(self.secure_random.generate_bytes(48)?);
        
        // Derive session keys
        self.derive_session_keys(session)?;
        
        session.state = TlsHandshakeState::ClientKeyExchangeSent;
        Ok(None)
    }

    fn process_client_finished(&self, session: &mut TlsHandshakeSession, _message: &TlsHandshakeMessage) -> AdvancedCryptoResult<Option<TlsHandshakeMessage>> {
        session.state = TlsHandshakeState::Established;
        Ok(None)
    }

    fn create_server_hello(&self, session: &TlsHandshakeSession) -> AdvancedCryptoResult<TlsHandshakeMessage> {
        let mut payload = Vec::new();
        
        // Protocol version
        payload.extend_from_slice(&(session.selected_version.as_ref().unwrap().clone() as u16).to_be_bytes());
        
        // Server random
        payload.extend_from_slice(&session.server_random);
        
        // Session ID (empty)
        payload.push(0);
        
        // Selected cipher suite
        payload.extend_from_slice(&self.cipher_suite_to_bytes(session.selected_cipher_suite.as_ref().unwrap()));
        
        // Compression method
        payload.push(0);

        Ok(TlsHandshakeMessage {
            message_type: TlsHandshakeType::ServerHello,
            length: payload.len() as u32,
            payload,
            timestamp: SystemTime::now(),
        })
    }

    fn create_client_key_exchange(&self, session: &TlsHandshakeSession) -> AdvancedCryptoResult<TlsHandshakeMessage> {
        // Simplified ClientKeyExchange with encrypted pre-master secret
        let pre_master = session.pre_master_secret.as_ref().unwrap();
        
        // In real implementation, would encrypt with server's public key
        let encrypted_pre_master = pre_master.clone();
        
        Ok(TlsHandshakeMessage {
            message_type: TlsHandshakeType::ClientKeyExchange,
            length: encrypted_pre_master.len() as u32,
            payload: encrypted_pre_master,
            timestamp: SystemTime::now(),
        })
    }

    fn derive_session_keys(&self, session: &mut TlsHandshakeSession) -> AdvancedCryptoResult<()> {
        let pre_master = session.pre_master_secret.as_ref().unwrap();
        
        // Derive master secret using our hash manager
        let mut master_data = Vec::new();
        master_data.extend_from_slice(pre_master);
        master_data.extend_from_slice(&session.client_random);
        master_data.extend_from_slice(&session.server_random);
        master_data.extend_from_slice(b"master secret");
        let master_secret = self.hash_manager.hash_sha256(&master_data)?;
        
        // Derive session keys from master secret
        let mut key_data = Vec::new();
        key_data.extend_from_slice(&master_secret);
        key_data.extend_from_slice(&session.server_random);
        key_data.extend_from_slice(&session.client_random);
        key_data.extend_from_slice(b"key expansion");
        let key_material = self.hash_manager.hash_sha256(&key_data)?;
        
        // Split key material into individual keys
        let session_keys = TlsSessionKeys {
            master_secret,
            client_write_key: key_material[0..16].to_vec(),
            server_write_key: key_material[16..32].to_vec(),
            client_write_iv: vec![0; 4],
            server_write_iv: vec![0; 4],
            client_mac_key: vec![],
            server_mac_key: vec![],
        };
        
        session.session_keys = Some(session_keys);
        Ok(())
    }

    fn cipher_suite_to_bytes(&self, suite: &TlsCipherSuite) -> [u8; 2] {
        match suite {
            TlsCipherSuite::AES128GcmSha256 => [0x00, 0x9C],
            TlsCipherSuite::AES256GcmSha384 => [0x00, 0x9D],
            TlsCipherSuite::ChaCha20Poly1305Sha256 => [0xCC, 0xA8],
            TlsCipherSuite::AES128CbcSha256 => [0x00, 0x3C],
            TlsCipherSuite::AES256CbcSha256 => [0x00, 0x3D],
            _ => [0x00, 0x9D], // Default to AES256-GCM-SHA384
        }
    }

    fn bytes_to_hex(&self, bytes: &[u8]) -> String {
        bytes.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<String>>()
            .join("")
    }
}

impl Default for TlsHandshakeManager {
    fn default() -> Self {
        Self::new().expect("Failed to create default TlsHandshakeManager")
    }
}

impl fmt::Display for TlsVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TlsVersion::Tls10 => write!(f, "TLS 1.0"),
            TlsVersion::Tls11 => write!(f, "TLS 1.1"),
            TlsVersion::Tls12 => write!(f, "TLS 1.2"),
            TlsVersion::Tls13 => write!(f, "TLS 1.3"),
        }
    }
}

impl fmt::Display for TlsCipherSuite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TlsCipherSuite::AES128GcmSha256 => write!(f, "AES128-GCM-SHA256"),
            TlsCipherSuite::AES256GcmSha384 => write!(f, "AES256-GCM-SHA384"),
            TlsCipherSuite::ChaCha20Poly1305Sha256 => write!(f, "CHACHA20-POLY1305-SHA256"),
            TlsCipherSuite::AES128CbcSha256 => write!(f, "AES128-CBC-SHA256"),
            TlsCipherSuite::AES256CbcSha256 => write!(f, "AES256-CBC-SHA256"),
            _ => write!(f, "Unknown Cipher Suite"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tls_manager_creation() {
        let manager = TlsHandshakeManager::new().unwrap();
        assert_eq!(manager.sessions.lock().unwrap().len(), 0);
    }

    #[test]
    fn test_client_handshake_start() {
        let manager = TlsHandshakeManager::new().unwrap();
        let (session_id, client_hello) = manager.start_client_handshake(None).unwrap();
        
        assert!(!session_id.is_empty());
        assert_eq!(client_hello.message_type, TlsHandshakeType::ClientHello);
        
        let session = manager.get_session(&session_id).unwrap().unwrap();
        assert_eq!(session.state, TlsHandshakeState::ClientHelloSent);
        assert!(session.is_client);
    }

    #[test]
    fn test_server_handshake_start() {
        let manager = TlsHandshakeManager::new().unwrap();
        let session_id = manager.start_server_handshake(None).unwrap();
        
        assert!(!session_id.is_empty());
        
        let session = manager.get_session(&session_id).unwrap().unwrap();
        assert_eq!(session.state, TlsHandshakeState::Initial);
        assert!(!session.is_client);
    }

    #[test]
    fn test_tls_config() {
        let config = TlsConfig {
            version: TlsVersion::Tls13,
            cipher_suites: vec![TlsCipherSuite::AES256GcmSha384],
            server_name: Some("example.com".to_string()),
            verify_peer: true,
            client_auth: false,
            session_timeout: Duration::from_secs(300),
            max_handshake_time: Duration::from_secs(30),
        };
        
        let manager = TlsHandshakeManager::new().unwrap();
        let (session_id, _) = manager.start_client_handshake(Some(config.clone())).unwrap();
        
        let session = manager.get_session(&session_id).unwrap().unwrap();
        assert_eq!(session.config.version, config.version);
        assert_eq!(session.config.server_name, config.server_name);
    }

    #[test]
    fn test_handshake_complete_check() {
        let manager = TlsHandshakeManager::new().unwrap();
        let (session_id, _) = manager.start_client_handshake(None).unwrap();
        
        // Initially not complete
        assert!(!manager.is_handshake_complete(&session_id).unwrap());
        
        // Invalid session
        assert!(!manager.is_handshake_complete("invalid").unwrap());
    }

    #[test]
    fn test_display_formatting() {
        assert_eq!(format!("{}", TlsVersion::Tls12), "TLS 1.2");
        assert_eq!(format!("{}", TlsVersion::Tls13), "TLS 1.3");
        assert_eq!(format!("{}", TlsCipherSuite::AES256GcmSha384), "AES256-GCM-SHA384");
    }

    #[test]
    fn test_cipher_suite_conversion() {
        let manager = TlsHandshakeManager::new().unwrap();
        
        let aes128_bytes = manager.cipher_suite_to_bytes(&TlsCipherSuite::AES128GcmSha256);
        assert_eq!(aes128_bytes, [0x00, 0x9C]);
        
        let aes256_bytes = manager.cipher_suite_to_bytes(&TlsCipherSuite::AES256GcmSha384);
        assert_eq!(aes256_bytes, [0x00, 0x9D]);
    }
}
