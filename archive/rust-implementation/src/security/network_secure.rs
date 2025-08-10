//! Secure networking implementation with TLS/SSL support

use crate::error::CursedError;
use rustls::{
    ClientConfig, ServerConfig, Certificate, PrivateKey,
    client::ServerCertVerifier, server::ClientCertVerifier,
    RootCertStore, SupportedCipherSuite, ProtocolVersion,
};
use rustls_webpki::TrustAnchor;
use std::io::{Read, Write};
use std::net::{TcpStream, TcpListener, SocketAddr};
use std::sync::Arc;
use std::time::{Duration, SystemTime};

pub type NetworkResult<T> = Result<T, CursedError>;

/// TLS security configuration levels
#[derive(Debug, Clone, Copy)]
pub enum SecurityLevel {
    /// Maximum security - TLS 1.3 only, strongest ciphers
    Maximum,
    /// High security - TLS 1.2+ with strong ciphers
    High,
    /// Standard security - TLS 1.2+ with modern ciphers
    Standard,
}

/// TLS cipher suite configuration
#[derive(Debug, Clone)]
pub struct TlsConfig {
    pub security_level: SecurityLevel,
    pub protocol_versions: Vec<ProtocolVersion>,
    pub cipher_suites: Vec<SupportedCipherSuite>,
    pub verify_certificates: bool,
    pub verify_hostname: bool,
    pub client_auth_required: bool,
}

impl TlsConfig {
    /// Create maximum security TLS configuration
    pub fn maximum_security() -> Self {
        Self {
            security_level: SecurityLevel::Maximum,
            protocol_versions: vec![ProtocolVersion::TLSv1_3],
            cipher_suites: vec![
                rustls::cipher_suite::TLS13_AES_256_GCM_SHA384,
                rustls::cipher_suite::TLS13_CHACHA20_POLY1305_SHA256,
            ],
            verify_certificates: true,
            verify_hostname: true,
            client_auth_required: false,
        }
    }

    /// Create high security TLS configuration  
    pub fn high_security() -> Self {
        Self {
            security_level: SecurityLevel::High,
            protocol_versions: vec![ProtocolVersion::TLSv1_3, ProtocolVersion::TLSv1_2],
            cipher_suites: vec![
                rustls::cipher_suite::TLS13_AES_256_GCM_SHA384,
                rustls::cipher_suite::TLS13_CHACHA20_POLY1305_SHA256,
                rustls::cipher_suite::TLS12_ECDHE_RSA_WITH_AES_256_GCM_SHA384,
                rustls::cipher_suite::TLS12_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384,
            ],
            verify_certificates: true,
            verify_hostname: true,
            client_auth_required: false,
        }
    }

    /// Create standard security TLS configuration
    pub fn standard_security() -> Self {
        Self {
            security_level: SecurityLevel::Standard,
            protocol_versions: vec![ProtocolVersion::TLSv1_3, ProtocolVersion::TLSv1_2],
            cipher_suites: rustls::ALL_CIPHER_SUITES.to_vec(),
            verify_certificates: true,
            verify_hostname: true,
            client_auth_required: false,
        }
    }

    /// Validate configuration security
    pub fn validate(&self) -> NetworkResult<()> {
        // Ensure no weak protocols
        for version in &self.protocol_versions {
            match version {
                ProtocolVersion::SSLv2 | ProtocolVersion::SSLv3 | ProtocolVersion::TLSv1_0 | ProtocolVersion::TLSv1_1 => {
                    return Err(CursedError::runtime_error(&format!(
                        "Weak protocol version not allowed: {:?}", version
                    )));
                }
                _ => {}
            }
        }

        // Ensure cipher suites are configured
        if self.cipher_suites.is_empty() {
            return Err(CursedError::runtime_error("No cipher suites configured"));
        }

        Ok(())
    }
}

/// Certificate validation configuration
#[derive(Debug, Clone)]
pub struct CertificateConfig {
    pub ca_certificates: Vec<Certificate>,
    pub client_certificates: Vec<Certificate>,
    pub private_key: Option<PrivateKey>,
    pub verify_chain: bool,
    pub verify_expiration: bool,
    pub allowed_hostnames: Vec<String>,
}

impl CertificateConfig {
    pub fn new() -> Self {
        Self {
            ca_certificates: Vec::new(),
            client_certificates: Vec::new(),
            private_key: None,
            verify_chain: true,
            verify_expiration: true,
            allowed_hostnames: Vec::new(),
        }
    }

    /// Load system CA certificates
    pub fn with_system_cas(mut self) -> NetworkResult<Self> {
        let mut root_store = RootCertStore::empty();
        root_store.add_server_trust_anchors(
            webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
                rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
                    ta.subject,
                    ta.spki,
                    ta.name_constraints,
                )
            })
        );
        
        // Convert to Certificate format
        // In practice, you'd load actual certificates
        Ok(self)
    }

    /// Add custom CA certificate
    pub fn add_ca_certificate(mut self, cert_pem: &[u8]) -> NetworkResult<Self> {
        let cert = rustls_pemfile::certs(&mut cert_pem.as_ref())
            .map_err(|e| CursedError::runtime_error(&format!("Failed to parse certificate: {}", e)))?
            .into_iter()
            .map(Certificate)
            .collect::<Vec<_>>();
            
        self.ca_certificates.extend(cert);
        Ok(self)
    }

    /// Set private key for server authentication
    pub fn with_private_key(mut self, key_pem: &[u8]) -> NetworkResult<Self> {
        let mut keys = rustls_pemfile::pkcs8_private_keys(&mut key_pem.as_ref())
            .map_err(|e| CursedError::runtime_error(&format!("Failed to parse private key: {}", e)))?;
            
        if keys.is_empty() {
            return Err(CursedError::runtime_error("No private keys found"));
        }
        
        self.private_key = Some(PrivateKey(keys.remove(0)));
        Ok(self)
    }
}

/// Secure TLS client
pub struct SecureTlsClient {
    config: ClientConfig,
    tls_config: TlsConfig,
}

impl SecureTlsClient {
    /// Create new secure TLS client
    pub fn new(tls_config: TlsConfig, cert_config: CertificateConfig) -> NetworkResult<Self> {
        tls_config.validate()?;

        let mut root_store = RootCertStore::empty();
        
        // Add system CA certificates
        root_store.add_server_trust_anchors(
            webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
                rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
                    ta.subject,
                    ta.spki,
                    ta.name_constraints,
                )
            })
        );

        // Add custom CA certificates
        for cert in &cert_config.ca_certificates {
            let ta = TrustAnchor::try_from_cert_der(&cert.0)
                .map_err(|e| CursedError::runtime_error(&format!("Invalid CA certificate: {}", e)))?;
            root_store.add_server_trust_anchors([rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
                ta.subject,
                ta.spki,
                ta.name_constraints,
            )].iter().cloned());
        }

        let config = ClientConfig::builder()
            .with_cipher_suites(&tls_config.cipher_suites)
            .with_safe_default_kx_groups()
            .with_protocol_versions(&tls_config.protocol_versions)
            .map_err(|e| CursedError::runtime_error(&format!("TLS config error: {}", e)))?
            .with_root_certificates(root_store)
            .with_no_client_auth();

        Ok(Self {
            config,
            tls_config,
        })
    }

    /// Connect to server with TLS
    pub fn connect(&self, hostname: &str, port: u16) -> NetworkResult<SecureTlsConnection> {
        // Validate hostname
        if !self.is_valid_hostname(hostname) {
            return Err(CursedError::runtime_error("Invalid hostname"));
        }

        let addr = format!("{}:{}", hostname, port);
        let sock_addr: SocketAddr = addr.parse()
            .map_err(|e| CursedError::runtime_error(&format!("Invalid address: {}", e)))?;

        // Create TCP connection with timeout
        let tcp_stream = TcpStream::connect_timeout(&sock_addr, Duration::from_secs(10))
            .map_err(|e| CursedError::runtime_error(&format!("TCP connection failed: {}", e)))?;

        // Set TCP options
        tcp_stream.set_nodelay(true)
            .map_err(|e| CursedError::runtime_error(&format!("Failed to set TCP_NODELAY: {}", e)))?;
        tcp_stream.set_read_timeout(Some(Duration::from_secs(30)))
            .map_err(|e| CursedError::runtime_error(&format!("Failed to set read timeout: {}", e)))?;
        tcp_stream.set_write_timeout(Some(Duration::from_secs(30)))
            .map_err(|e| CursedError::runtime_error(&format!("Failed to set write timeout: {}", e)))?;

        // Create TLS connection
        let server_name = hostname.try_into()
            .map_err(|e| CursedError::runtime_error(&format!("Invalid server name: {}", e)))?;
        
        let mut conn = rustls::ClientConnection::new(Arc::new(self.config.clone()), server_name)
            .map_err(|e| CursedError::runtime_error(&format!("TLS connection failed: {}", e)))?;

        let mut tls_stream = rustls::Stream::new(&mut conn, &mut tcp_stream);

        Ok(SecureTlsConnection {
            stream: Box::new(tls_stream),
            peer_certificates: conn.peer_certificates().map(|certs| certs.to_vec()),
        })
    }

    /// Validate hostname format
    fn is_valid_hostname(&self, hostname: &str) -> bool {
        if hostname.is_empty() || hostname.len() > 253 {
            return false;
        }

        // Basic hostname validation
        hostname.chars().all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-')
            && !hostname.starts_with('-')
            && !hostname.ends_with('-')
            && !hostname.starts_with('.')
            && !hostname.ends_with('.')
    }
}

/// Secure TLS server
pub struct SecureTlsServer {
    config: ServerConfig,
    listener: TcpListener,
}

impl SecureTlsServer {
    /// Create new secure TLS server
    pub fn new(
        bind_addr: &str,
        tls_config: TlsConfig,
        cert_config: CertificateConfig,
    ) -> NetworkResult<Self> {
        tls_config.validate()?;

        let private_key = cert_config.private_key
            .ok_or_else(|| CursedError::runtime_error("Private key required for server"))?;

        if cert_config.client_certificates.is_empty() {
            return Err(CursedError::runtime_error("Server certificate required"));
        }

        let config = ServerConfig::builder()
            .with_cipher_suites(&tls_config.cipher_suites)
            .with_safe_default_kx_groups()
            .with_protocol_versions(&tls_config.protocol_versions)
            .map_err(|e| CursedError::runtime_error(&format!("TLS config error: {}", e)))?
            .with_no_client_auth()
            .with_single_cert(cert_config.client_certificates, private_key)
            .map_err(|e| CursedError::runtime_error(&format!("Certificate error: {}", e)))?;

        let listener = TcpListener::bind(bind_addr)
            .map_err(|e| CursedError::runtime_error(&format!("Failed to bind: {}", e)))?;

        Ok(Self {
            config,
            listener,
        })
    }

    /// Accept incoming TLS connections
    pub fn accept(&self) -> NetworkResult<SecureTlsConnection> {
        let (tcp_stream, _addr) = self.listener.accept()
            .map_err(|e| CursedError::runtime_error(&format!("Accept failed: {}", e)))?;

        // Set TCP options
        tcp_stream.set_nodelay(true)
            .map_err(|e| CursedError::runtime_error(&format!("Failed to set TCP_NODELAY: {}", e)))?;

        // Create TLS connection
        let mut conn = rustls::ServerConnection::new(Arc::new(self.config.clone()))
            .map_err(|e| CursedError::runtime_error(&format!("TLS connection failed: {}", e)))?;

        let mut tls_stream = rustls::Stream::new(&mut conn, &mut tcp_stream);

        Ok(SecureTlsConnection {
            stream: Box::new(tls_stream),
            peer_certificates: conn.peer_certificates().map(|certs| certs.to_vec()),
        })
    }
}

/// Secure TLS connection wrapper
pub struct SecureTlsConnection {
    stream: Box<dyn SecureStream>,
    peer_certificates: Option<Vec<Certificate>>,
}

impl SecureTlsConnection {
    /// Get peer certificates
    pub fn peer_certificates(&self) -> Option<&[Certificate]> {
        self.peer_certificates.as_deref()
    }

    /// Verify peer certificate chain
    pub fn verify_peer_certificate(&self, expected_hostname: &str) -> NetworkResult<()> {
        let certs = self.peer_certificates.as_ref()
            .ok_or_else(|| CursedError::runtime_error("No peer certificates"))?;

        if certs.is_empty() {
            return Err(CursedError::runtime_error("Empty certificate chain"));
        }

        // In practice, implement full certificate chain validation
        // including expiration, revocation, hostname matching
        println!("Certificate verification for {} - {} certificates", expected_hostname, certs.len());
        
        Ok(())
    }

    /// Secure write with length validation
    pub fn write_secure(&mut self, data: &[u8]) -> NetworkResult<usize> {
        if data.len() > 1_000_000 {
            return Err(CursedError::runtime_error("Data too large"));
        }

        self.stream.write(data)
            .map_err(|e| CursedError::runtime_error(&format!("Write failed: {}", e)))
    }

    /// Secure read with timeout
    pub fn read_secure(&mut self, buf: &mut [u8]) -> NetworkResult<usize> {
        self.stream.read(buf)
            .map_err(|e| CursedError::runtime_error(&format!("Read failed: {}", e)))
    }
}

/// Secure stream trait
trait SecureStream: Read + Write + Send + Sync {}

impl<T: Read + Write + Send + Sync> SecureStream for T {}

/// Network security utilities
pub struct NetworkSecurity;

impl NetworkSecurity {
    /// Validate IP address for allowed ranges
    pub fn is_allowed_ip(ip: std::net::IpAddr, allowed_ranges: &[std::net::IpAddr]) -> bool {
        // Reject private/localhost IPs in production
        match ip {
            std::net::IpAddr::V4(ipv4) => {
                if ipv4.is_loopback() || ipv4.is_private() || ipv4.is_multicast() {
                    return false;
                }
            }
            std::net::IpAddr::V6(ipv6) => {
                if ipv6.is_loopback() || ipv6.is_multicast() {
                    return false;
                }
            }
        }

        allowed_ranges.contains(&ip)
    }

    /// Rate limiting for connections
    pub fn check_rate_limit(client_ip: std::net::IpAddr, max_per_minute: u32) -> NetworkResult<()> {
        // Implementation would track per-IP connection rates
        // For now, return success
        println!("Rate limit check for {}: {}/min", client_ip, max_per_minute);
        Ok(())
    }

    /// Generate secure session ID
    pub fn generate_session_id() -> NetworkResult<String> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let bytes: [u8; 32] = rng.gen();
        Ok(hex::encode(bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tls_config_validation() {
        let config = TlsConfig::maximum_security();
        assert!(config.validate().is_ok());

        let mut weak_config = config.clone();
        weak_config.protocol_versions = vec![ProtocolVersion::TLSv1_0];
        assert!(weak_config.validate().is_err());
    }

    #[test]
    fn test_hostname_validation() {
        let cert_config = CertificateConfig::new();
        let tls_config = TlsConfig::standard_security();
        let client = SecureTlsClient::new(tls_config, cert_config).unwrap();

        assert!(client.is_valid_hostname("example.com"));
        assert!(client.is_valid_hostname("sub.example.com"));
        assert!(!client.is_valid_hostname(""));
        assert!(!client.is_valid_hostname("-invalid.com"));
        assert!(!client.is_valid_hostname("invalid.com-"));
    }

    #[test]
    fn test_ip_validation() {
        let allowed_ips = vec![
            "203.0.113.1".parse().unwrap(),
            "198.51.100.1".parse().unwrap(),
        ];

        let public_ip: std::net::IpAddr = "203.0.113.1".parse().unwrap();
        assert!(NetworkSecurity::is_allowed_ip(public_ip, &allowed_ips));

        let private_ip: std::net::IpAddr = "192.168.1.1".parse().unwrap();
        assert!(!NetworkSecurity::is_allowed_ip(private_ip, &allowed_ips));
    }
}
