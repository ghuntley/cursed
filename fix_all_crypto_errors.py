#!/usr/bin/env python3

import os
import subprocess
import re

def add_missing_types_to_file(file_path, content_to_add):
    """Add missing types to a file if they don't already exist"""
    try:
        if os.path.exists(file_path):
            with open(file_path, 'r') as f:
                existing = f.read()
            
            # Check if content already exists
            if content_to_add.strip() not in existing:
                with open(file_path, 'a') as f:
                    f.write('\n\n' + content_to_add)
                print(f"Added missing types to {file_path}")
        else:
            print(f"File {file_path} does not exist")
    except Exception as e:
        print(f"Error fixing {file_path}: {e}")

def fix_all_crypto_modules():
    """Fix all crypto modules comprehensively"""
    
    # Fix ECDH types
    add_missing_types_to_file('src/stdlib/packages/crypto_protocols/ecdh.rs', '''
// ECDH specific types
#[derive(Debug, Clone)]
pub struct EcdhManager {
    pub curve: EcdhCurve,
}

#[derive(Debug, Clone)]
pub enum EcdhCurve {
    P256,
    P384,
    P521,
}

#[derive(Debug, Clone)]
pub struct EcdhKeyPair {
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct EcdhSharedSecret {
    pub secret: Vec<u8>,
}
''')
    
    # Fix Diffie Hellman types
    add_missing_types_to_file('src/stdlib/packages/crypto_protocols/diffie_hellman.rs', '''
// Diffie-Hellman specific types
#[derive(Debug, Clone)]
pub struct DiffieHellmanManager {
    pub group: DhGroup,
}

#[derive(Debug, Clone)]
pub enum DhGroup {
    Group14,
    Group16,
    Group18,
}

#[derive(Debug, Clone)]
pub struct DhKeyPair {
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct DhSharedSecret {
    pub secret: Vec<u8>,
}
''')

    # Fix Authentication types
    add_missing_types_to_file('src/stdlib/packages/crypto_protocols/authentication.rs', '''
// Authentication specific types
#[derive(Debug, Clone)]
pub struct AuthenticationManager {
    pub method: AuthMethod,
}

#[derive(Debug, Clone)]
pub enum AuthMethod {
    Password,
    Certificate,
    TwoFactor,
}

#[derive(Debug, Clone)]
pub struct MfaConfig {
    pub enabled: bool,
    pub methods: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct AuthResult {
    pub success: bool,
    pub user_id: Option<String>,
}
''')

    # Fix Secure Channels types
    add_missing_types_to_file('src/stdlib/packages/crypto_protocols/secure_channels.rs', '''
// Secure Channel specific types  
#[derive(Debug, Clone)]
pub struct SecureChannelManager {
    pub channel_type: ChannelType,
}

#[derive(Debug, Clone)]
pub enum ChannelType {
    TLS,
    SSH,
    VPN,
}

#[derive(Debug, Clone)]
pub enum SecurityLevel {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone)]
pub struct SecureChannel {
    pub id: String,
    pub security_level: SecurityLevel,
}
''')

    # Fix Signal Protocol types
    add_missing_types_to_file('src/stdlib/packages/crypto_protocols/signal_protocol.rs', '''
// Signal Protocol specific types
#[derive(Debug, Clone)]
pub struct SignalProtocolManager {
    pub session_id: String,
}

#[derive(Debug, Clone)]
pub struct SignalKeyBundle {
    pub identity_key: Vec<u8>,
    pub signed_prekey: Vec<u8>,
    pub prekey: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct SignalMessage {
    pub ciphertext: Vec<u8>,
    pub message_type: u8,
}
''')

    # Fix TLS Handshake types
    add_missing_types_to_file('src/stdlib/packages/crypto_protocols/tls_handshake.rs', '''
// TLS Handshake specific types
#[derive(Debug, Clone)]
pub struct TlsHandshakeManager {
    pub version: TlsVersion,
}

#[derive(Debug, Clone)]
pub enum TlsVersion {
    V1_2,
    V1_3,
}

#[derive(Debug, Clone)]
pub struct TlsCipherSuite {
    pub id: u16,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct TlsHandshakeSession {
    pub session_id: Vec<u8>,
    pub cipher_suite: TlsCipherSuite,
}
''')

    # Fix Session Management types
    add_missing_types_to_file('src/stdlib/packages/crypto_protocols/session_management.rs', '''
// Session Management specific types
#[derive(Debug, Clone)]
pub struct SessionManager {
    pub max_sessions: u32,
}

#[derive(Debug, Clone)]
pub struct CryptoSession {
    pub id: String,
    pub created_at: u64,
}

#[derive(Debug, Clone)]
pub struct SessionTicket {
    pub ticket: Vec<u8>,
    pub expiry: u64,
}

#[derive(Debug, Clone)]
pub struct SessionConfig {
    pub timeout: u32,
    pub max_idle: u32,
}
''')

    # Fix missing error types in error module
    error_additions = '''

    /// Random generation failed error
    pub fn random_generation_failed(msg: &str) -> Self {
        CursedError::RuntimeError(format!("Random generation failed: {}", msg))
    }
    
    /// Validation error
    pub fn validation_error(msg: &str) -> Self {
        CursedError::RuntimeError(format!("Validation error: {}", msg))
    }
    
    /// Internal error
    pub fn internal_error(msg: &str) -> Self {
        CursedError::RuntimeError(format!("Internal error: {}", msg))
    }
    
    /// Unsupported algorithm error  
    pub fn unsupported_algorithm(algorithm: &str) -> Self {
        CursedError::RuntimeError(format!("Unsupported algorithm: {}", algorithm))
    }
'''
    
    # Add error methods to error module
    error_file = 'src/error/mod.rs'
    if os.path.exists(error_file):
        with open(error_file, 'r') as f:
            content = f.read()
        
        # Only add if not already present
        if "random_generation_failed" not in content:
            # Find the impl block for CursedError and add methods
            impl_pattern = r'(impl CursedError \{[^}]*)(}$)'
            if re.search(impl_pattern, content, re.MULTILINE | re.DOTALL):
                content = re.sub(impl_pattern, r'\1' + error_additions + r'\n\2', content, flags=re.MULTILINE | re.DOTALL)
                with open(error_file, 'w') as f:
                    f.write(content)
                print("Added missing error methods to error module")
    
    # Fix Copy trait for SecurityLevel in signatures
    signature_file = 'src/stdlib/packages/crypto_signatures/mod.rs'
    if os.path.exists(signature_file):
        with open(signature_file, 'r') as f:
            content = f.read()
        
        # Add Copy trait to SecurityLevel if it exists
        if 'enum SecurityLevel' in content and 'Copy' not in content:
            content = content.replace('#[derive(Debug, Clone)]', '#[derive(Debug, Clone, Copy)]')
            with open(signature_file, 'w') as f:
                f.write(content)
            print("Added Copy trait to SecurityLevel")
            
    print("All crypto module fixes applied!")

if __name__ == "__main__":
    fix_all_crypto_modules()
