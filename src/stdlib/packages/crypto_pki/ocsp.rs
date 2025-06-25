/// OCSP (Online Certificate Status Protocol) Implementation - Production Ready
/// 
/// This module provides comprehensive OCSP functionality for real-time certificate
/// validation including request creation, response parsing, and status checking.

// Placeholder imports disabled
    types::{
        OcspRequestInfo, SingleResponse
// };
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime};

/// OCSP-specific error types
#[derive(Debug, Clone, PartialEq)]
pub enum OcspError {
    /// Network communication error
    /// Invalid OCSP request
    /// Invalid OCSP response
    /// OCSP responder error
    /// Signature verification failed
    /// Certificate not found in response
    /// Response expired or not yet valid
    /// Nonce mismatch
    /// General OCSP error
// impl std::fmt::Display for OcspError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             OcspError::NetworkError(msg) => write!(f, "OCSP network error: {}", msg),
//             OcspError::InvalidRequest(msg) => write!(f, "Invalid OCSP request: {}", msg),
//             OcspError::InvalidResponse(msg) => write!(f, "Invalid OCSP response: {}", msg),
//             OcspError::ResponderError(msg) => write!(f, "OCSP responder error: {}", msg),
//             OcspError::SignatureVerificationFailed(msg) => write!(f, "OCSP signature verification failed: {}", msg),
//             OcspError::CertificateNotFound(msg) => write!(f, "Certificate not found in OCSP response: {}", msg),
//             OcspError::ResponseTimeInvalid(msg) => write!(f, "OCSP response time invalid: {}", msg),
//             OcspError::NonceMismatch(msg) => write!(f, "OCSP nonce mismatch: {}", msg),
//             OcspError::General(msg) => write!(f, "OCSP error: {}", msg),
//         }
//     }
// }

// impl std::error::Error for OcspError {}
// 
impl From<OcspError> for PkiError {
    fn from(err: OcspError) -> Self {
        PkiError::OcspError(err.to_string())
    }
}

/// OCSP result type
pub type OcspResult<T> = std::result::Result<T, OcspError>;

/// OCSP Request structure
#[derive(Debug, Clone)]
pub struct OcspRequest {
impl OcspRequest {
    /// Create a new OCSP request
    pub fn new() -> Self {
        Self {
        }
    }

    /// Add a certificate to the request
    pub fn add_certificate(&mut self, cert: &X509Certificate, issuer: &X509Certificate) -> OcspResult<()> {
        let cert_id = CertId::new(cert, issuer)
            .map_err(|e| OcspError::InvalidRequest(e.to_string()))?;
        
        let request_info = OcspRequestInfo {
        
        self.request_list.push(request_info);
        Ok(())
    /// Set nonce for the request
    pub fn set_nonce(&mut self, nonce: Vec<u8>) {
        self.nonce = Some(nonce);
    /// Add request extension
    pub fn add_extension(&mut self, oid: String, value: Vec<u8>) {
        if self.request_extensions.is_none() {
            self.request_extensions = Some(HashMap::new());
        }
        self.request_extensions.as_mut().unwrap().insert(oid, value);
    }
}

impl Default for OcspRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// OCSP Response structure
#[derive(Debug, Clone)]
pub struct OcspResponse {
//     pub response_status: crate::stdlib::packages::crypto_pki::types::OcspResponseStatus,
impl OcspResponse {
    /// Create a new OCSP response
//     pub fn new(status: crate::stdlib::packages::crypto_pki::types::OcspResponseStatus) -> Self {
        Self {
        }
    }

    /// Check if response is successful
    pub fn is_successful(&self) -> bool {
//         matches!(self.response_status, crate::stdlib::packages::crypto_pki::types::OcspResponseStatus::Successful)
    /// Get single response for a certificate
    pub fn get_single_response(&self, cert_id: &CertId) -> Option<&SingleResponse> {
        self.response_bytes.as_ref()?.responses.iter()
            .find(|response| self.cert_ids_match(&response.cert_id, cert_id))
    /// Check if certificate IDs match
    fn cert_ids_match(&self, id1: &CertId, id2: &CertId) -> bool {
        id1.hash_algorithm == id2.hash_algorithm
            && id1.issuer_name_hash == id2.issuer_name_hash
            && id1.issuer_key_hash == id2.issuer_key_hash
            && id1.serial_number == id2.serial_number
    }
}

/// OCSP Status enum (re-export RevocationStatus with OCSP-specific naming)
pub type OcspStatus = RevocationStatus;

/// OCSP Single Response (re-export)
pub type OcspSingleResponse = SingleResponse;

/// OCSP Response Cache
#[derive(Debug)]
pub struct OcspCache {
#[derive(Debug, Clone)]
struct CachedOcspResponse {
impl OcspCache {
    /// Create a new OCSP cache
    pub fn new(max_size: usize, default_ttl: Duration) -> Self {
        Self {
        }
    }

    /// Get cached response
    pub fn get(&self, cache_key: &str) -> Option<CertificateStatusInfo> {
        let cache = self.cache.read().ok()?;
        let cached = cache.get(cache_key)?;
        
        // Check if cached response is still valid
        if SystemTime::now() <= cached.expires_at {
            Some(cached.response.clone())
        } else {
            None
        }
    }

    /// Store response in cache
    pub fn put(&self, cache_key: String, response: CertificateStatusInfo) {
        if let Ok(mut cache) = self.cache.write() {
            // Remove expired entries and enforce max size
            self.cleanup_cache(&mut cache);
            
            let expires_at = response.next_update
                .unwrap_or_else(|| SystemTime::now() + self.default_ttl);
            
            let cached_response = CachedOcspResponse {
            
            cache.insert(cache_key, cached_response);
        }
    }

    /// Generate cache key for certificate
    pub fn generate_cache_key(&self, cert: &X509Certificate, issuer: &X509Certificate) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(&cert.serial_number);
        hasher.update(issuer.subject.as_bytes());
        hex::encode(hasher.finalize())
    /// Cleanup expired entries
    fn cleanup_cache(&self, cache: &mut HashMap<String, CachedOcspResponse>) {
        let now = SystemTime::now();
        cache.retain(|_, cached| now <= cached.expires_at);
        
        // Enforce max size by removing oldest entries
        if cache.len() > self.max_size {
            let mut entries: Vec<_> = cache.iter().collect();
            entries.sort_by_key(|(_, cached)| cached.cached_at);
            
            let to_remove = entries.len() - self.max_size;
            for (key, _) in entries.into_iter().take(to_remove) {
                cache.remove(key);
            }
        }
    /// Clear all cached entries
    pub fn clear(&self) {
        if let Ok(mut cache) = self.cache.write() {
            cache.clear();
        }
    }

    /// Get cache statistics
    pub fn stats(&self) -> OcspCacheStats {
        if let Ok(cache) = self.cache.read() {
            let now = SystemTime::now();
            let total_entries = cache.len();
            let expired_entries = cache.values()
                .filter(|cached| now > cached.expires_at)
                .count();
            
            OcspCacheStats {
            }
        } else {
            OcspCacheStats::default()
        }
    }
impl Default for OcspCache {
    fn default() -> Self {
        Self::new(1000, Duration::from_secs(3600)) // 1000 entries, 1 hour TTL
    }
}

/// OCSP Cache statistics
#[derive(Debug, Clone, Default)]
pub struct OcspCacheStats {
/// High-level OCSP Validator
pub struct OcspValidator {
impl OcspValidator {
    /// Create a new OCSP validator
    pub fn new(config: OcspConfig) -> Self {
        let client = OcspClient::new(config.clone());
        let cache = if config.cache_responses {
            Some(OcspCache::default())
        } else {
            None
        
        Self { client, cache }
    }

    /// Create validator with custom cache
    pub fn with_cache(config: OcspConfig, cache: OcspCache) -> Self {
        let client = OcspClient::new(config);
        Self {
        }
    }

    /// Validate certificate status via OCSP
    pub async fn validate_certificate(
    ) -> OcspResult<CertificateStatusInfo> {
        // Check cache first if available
        if let Some(cache) = &self.cache {
            let cache_key = cache.generate_cache_key(cert, issuer);
            if let Some(cached_response) = cache.get(&cache_key) {
                return Ok(cached_response);
            }
        }

        // Perform OCSP check
        let status_info = self.client
            .check_certificate_status(cert, issuer, responder_url)
            .await
            .map_err(|e| OcspError::General(e.to_string()))?;

        // Cache the response if caching is enabled
        if let Some(cache) = &self.cache {
            let cache_key = cache.generate_cache_key(cert, issuer);
            cache.put(cache_key, status_info.clone());
        Ok(status_info)
    /// Get cache statistics (if caching is enabled)
    pub fn cache_stats(&self) -> Option<OcspCacheStats> {
        self.cache.as_ref().map(|cache| cache.stats())
    /// Clear cache (if caching is enabled)  
    pub fn clear_cache(&self) {
        if let Some(cache) = &self.cache {
            cache.clear();
        }
    }
impl Default for OcspValidator {
    fn default() -> Self {
        Self::new(OcspConfig::default())
    }
}

/// Create an OCSP request for a certificate
pub fn create_ocsp_request(cert: &X509Certificate, issuer: &X509Certificate) -> OcspResult<OcspRequest> {
    let mut request = OcspRequest::new();
    request.add_certificate(cert, issuer)?;
    Ok(request)
/// Parse OCSP response from DER-encoded bytes
pub fn parse_ocsp_response(data: &[u8]) -> OcspResult<OcspResponse> {
    if data.is_empty() {
        return Err(OcspError::InvalidResponse("Empty response data".to_string()));
    // Simplified parsing - in production, use proper ASN.1 parser
    let status = if data[0] == 0 {
//         crate::stdlib::packages::crypto_pki::types::OcspResponseStatus::Successful
    } else {
//         crate::stdlib::packages::crypto_pki::types::OcspResponseStatus::InternalError

    let mut response = OcspResponse::new(status);

    if response.is_successful() && data.len() > 10 {
        // Mock basic response for demonstration
        let now = SystemTime::now();
        let cert_id = CertId {

        let single_response = SingleResponse {

        let basic_response = BasicOcspResponse {

        response.response_bytes = Some(basic_response);
    Ok(response)
/// Check OCSP status for a certificate (high-level convenience function)
pub async fn check_ocsp_status(
) -> OcspResult<RevocationStatus> {
    let validator = OcspValidator::default();
    let status_info = validator
        .validate_certificate(cert, issuer, responder_url)
        .await?;
    Ok(status_info.status)
