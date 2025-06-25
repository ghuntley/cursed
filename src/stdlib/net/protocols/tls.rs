/// TLS/SSL configuration and utilities

/// TLS protocol versions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TlsVersion {
/// Cipher suites
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CipherSuite {
/// TLS configuration
#[derive(Debug, Clone)]
pub struct TlsConfig {
impl Default for TlsConfig {
    fn default() -> Self {
        Self {
            cipher_suites: vec![
        }
    }
impl TlsConfig {
    pub fn new() -> Self {
        Self::default()
    pub fn min_version(mut self, version: TlsVersion) -> Self {
        self.min_version = version;
        self
    pub fn max_version(mut self, version: TlsVersion) -> Self {
        self.max_version = version;
        self
    pub fn verify_certificates(mut self, verify: bool) -> Self {
        self.verify_certificates = verify;
        self
    pub fn ca_file<P: Into<String>>(mut self, path: P) -> Self {
        self.ca_file = Some(path.into());
        self
    pub fn cert_file<P: Into<String>>(mut self, path: P) -> Self {
        self.cert_file = Some(path.into());
        self
    pub fn key_file<P: Into<String>>(mut self, path: P) -> Self {
        self.key_file = Some(path.into());
        self
    }
}

impl std::fmt::Display for TlsVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
        }
    }
impl std::fmt::Display for CipherSuite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
        }
    }
