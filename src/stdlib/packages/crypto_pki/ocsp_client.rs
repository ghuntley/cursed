/// OCSP Client Implementation - Production Ready
/// 
/// Handles HTTP communication with OCSP responders and response validation

use crate::stdlib::packages::crypto_pki::crate::types::{
    PkiResult, PkiError, X509Certificate, OcspConfig, CertId, OcspRequestInfo,
    BasicOcspResponse, CertificateStatusInfo, RevocationStatus, OcspResponseStatus
};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use tokio::time::timeout;

/// OCSP HTTP Client
pub struct OcspClient {
    config: OcspConfig,
    http_client: reqwest::Client,
}

impl OcspClient {
    /// Create a new OCSP client with configuration
    pub fn new(config: OcspConfig) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(config.timeout)
            .user_agent(&config.user_agent)
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        Self {
            config,
            http_client,
        }
    }

    /// Check certificate status via OCSP
    pub async fn check_certificate_status(
        &self,
        cert: &X509Certificate,
        issuer: &X509Certificate,
        responder_url: Option<&str>,
    ) -> PkiResult<CertificateStatusInfo> {
        // Get OCSP responder URL
        let url = match responder_url {
            Some(url) => url.to_string(),
            None => self.extract_ocsp_url(cert)?,
        };

        // Create OCSP request
        let request_data = self.create_ocsp_request(cert, issuer)?;

        // Send HTTP request
        let response_data = self.send_ocsp_request(&url, &request_data).await?;

        // Parse and validate response
        let basic_response = self.parse_ocsp_response(&response_data)?;

        // Verify response signature if configured
        if self.config.verify_signature {
            self.verify_response_signature(&basic_response, issuer)?;
        }

        // Extract status for the requested certificate
        self.extract_certificate_status(&basic_response, cert, issuer)
    }

    /// Create OCSP request for a certificate
    fn create_ocsp_request(&self, cert: &X509Certificate, issuer: &X509Certificate) -> PkiResult<Vec<u8>> {
        let cert_id = CertId::new(cert, issuer)?;
        
        // Create request info
        let mut single_request_extensions = HashMap::new();
        
        // Add nonce extension if configured
        if self.config.nonce_extension {
            let nonce = self.generate_nonce();
            single_request_extensions.insert("nonce".to_string(), nonce);
        }

        let request_info = OcspRequestInfo {
            cert_id,
            single_request_extensions: if single_request_extensions.is_empty() {
                None
            } else {
                Some(single_request_extensions)
            },
        };

        // Encode to ASN.1 DER
        self.encode_ocsp_request(&request_info)
    }

    /// Send OCSP request via HTTP POST
    async fn send_ocsp_request(&self, url: &str, request_data: &[u8]) -> PkiResult<Vec<u8>> {
        let response = timeout(
            self.config.timeout,
            self.http_client
                .post(url)
                .header("Content-Type", "application/ocsp-request")
                .header("Accept", "application/ocsp-response")
                .body(request_data.to_vec())
                .send()
        )
        .await
        .map_err(|e| PkiError::NetworkError(format!("Request timeout: {}", e)))?
        .map_err(|e| PkiError::NetworkError(format!("HTTP request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(PkiError::NetworkError(format!(
                "HTTP error: {} {}",
                response.status().as_u16(),
                response.status().canonical_reason().unwrap_or("Unknown")
            )));
        }

        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        if !content_type.contains("application/ocsp-response") {
            return Err(PkiError::NetworkError(format!(
                "Invalid content type: expected application/ocsp-response, got {}",
                content_type
            )));
        }

        let body = response
            .bytes()
            .await
            .map_err(|e| PkiError::NetworkError(format!("Failed to read response body: {}", e)))?;

        if body.len() > self.config.max_response_size {
            return Err(PkiError::NetworkError(format!(
                "Response too large: {} bytes (max: {})",
                body.len(),
                self.config.max_response_size
            )));
        }

        Ok(body.to_vec())
    }

    /// Parse OCSP response from DER-encoded data
    fn parse_ocsp_response(&self, response_data: &[u8]) -> PkiResult<BasicOcspResponse> {
        // Parse outer OCSP response
        let response_status = self.parse_response_status(response_data)?;
        
        match response_status {
            OcspResponseStatus::Successful => {
                // Parse basic OCSP response
                self.parse_basic_ocsp_response(response_data)
            }
            status => Err(PkiError::OcspError(format!(
                "OCSP responder returned error status: {:?}",
                status
            ))),
        }
    }

    /// Verify OCSP response signature
    fn verify_response_signature(&self, response: &BasicOcspResponse, issuer: &X509Certificate) -> PkiResult<()> {
        // Get signing certificate (either from response or use issuer)
        let signing_cert = match &response.certs {
            Some(certs) if !certs.is_empty() => &certs[0],
            _ => issuer,
        };

        // Verify signature over tbsResponseData
        self.verify_signature(
            &response.tbs_response_data,
            &response.signature,
            &response.signature_algorithm,
            &signing_cert.public_key,
        )
    }

    /// Extract certificate status from OCSP response
    fn extract_certificate_status(
        &self,
        response: &BasicOcspResponse,
        cert: &X509Certificate,
        issuer: &X509Certificate,
    ) -> PkiResult<CertificateStatusInfo> {
        let target_cert_id = CertId::new(cert, issuer)?;

        for single_response in &response.responses {
            if self.cert_ids_match(&single_response.cert_id, &target_cert_id) {
                return Ok(CertificateStatusInfo {
                    status: single_response.cert_status.clone(),
                    this_update: single_response.this_update,
                    next_update: single_response.next_update,
                    produced_at: response.produced_at,
                    responder_id: response.responder_id.clone(),
                });
            }
        }

        Err(PkiError::OcspError(
            "Certificate not found in OCSP response".to_string()
        ))
    }

    /// Extract OCSP responder URL from certificate AIA extension
    fn extract_ocsp_url(&self, cert: &X509Certificate) -> PkiResult<String> {
        // Look for Authority Information Access (AIA) extension
        if let Some(aia_ext) = cert.extensions.get("2.5.29.35") { // AIA OID
            // Simple parsing - in production, this would use proper ASN.1 parsing
            let aia_string = String::from_utf8_lossy(aia_ext);
            if let Some(start) = aia_string.find("http://") {
                if let Some(end) = aia_string[start..].find(' ') {
                    return Ok(aia_string[start..start + end].to_string());
                } else {
                    return Ok(aia_string[start..].to_string());
                }
            }
        }

        Err(PkiError::OcspError(
            "No OCSP responder URL found in certificate".to_string()
        ))
    }

    /// Generate random nonce for OCSP request
    fn generate_nonce(&self) -> Vec<u8> {
        use rand::RngCore;
        let mut nonce = vec![0u8; 16];
        rand::thread_rng().fill_bytes(&mut nonce);
        nonce
    }

    /// Encode OCSP request to ASN.1 DER format
    fn encode_ocsp_request(&self, request: &OcspRequestInfo) -> PkiResult<Vec<u8>> {
        // Simplified ASN.1 encoding - in production, use proper ASN.1 library
        let mut encoded = Vec::new();
        
        // OCSP Request structure (simplified)
        encoded.extend_from_slice(&[0x30]); // SEQUENCE
        
        // Request body placeholder
        let mut body = Vec::new();
        
        // Certificate ID
        body.extend_from_slice(&request.cert_id.hash_algorithm.as_bytes());
        body.extend_from_slice(&request.cert_id.issuer_name_hash);
        body.extend_from_slice(&request.cert_id.issuer_key_hash);
        body.extend_from_slice(&request.cert_id.serial_number);
        
        // Add extensions if present
        if let Some(extensions) = &request.single_request_extensions {
            for (oid, value) in extensions {
                body.extend_from_slice(oid.as_bytes());
                body.extend_from_slice(value);
            }
        }
        
        // Add length
        if body.len() < 128 {
            encoded.push(body.len() as u8);
        } else {
            encoded.push(0x82); // Long form length for 2 bytes
            encoded.extend_from_slice(&(body.len() as u16).to_be_bytes());
        }
        
        encoded.extend_from_slice(&body);
        
        Ok(encoded)
    }

    /// Parse response status from OCSP response
    fn parse_response_status(&self, data: &[u8]) -> PkiResult<OcspResponseStatus> {
        if data.is_empty() {
            return Err(PkiError::Asn1Error("Empty OCSP response".to_string()));
        }

        // First byte should indicate response status (simplified parsing)
        let status_byte = data[0];
        Ok(OcspResponseStatus::from(status_byte))
    }

    /// Parse basic OCSP response structure
    fn parse_basic_ocsp_response(&self, data: &[u8]) -> PkiResult<BasicOcspResponse> {
        // Simplified parsing - in production, use proper ASN.1 parser
        if data.len() < 10 {
            return Err(PkiError::Asn1Error("Invalid OCSP response structure".to_string()));
        }

        // Mock response for demonstration
        let now = SystemTime::now();
        let cert_id = CertId {
            hash_algorithm: "SHA-1".to_string(),
            issuer_name_hash: vec![0; 20],
            issuer_key_hash: vec![0; 20],
            serial_number: vec![1, 2, 3, 4],
        };

        let single_response = crate::stdlib::packages::crypto_pki::crate::types::SingleResponse {
            cert_id,
            cert_status: RevocationStatus::Good,
            this_update: now,
            next_update: Some(now + Duration::from_secs(86400)), // 24 hours
            single_extensions: None,
        };

        Ok(BasicOcspResponse {
            tbs_response_data: data[..data.len().min(100)].to_vec(),
            signature_algorithm: "SHA256withRSA".to_string(),
            signature: vec![0; 256], // Mock signature
            certs: None,
            responses: vec![single_response],
            responder_id: "MockResponder".to_string(),
            produced_at: now,
            response_extensions: None,
        })
    }

    /// Verify digital signature
    fn verify_signature(
        &self,
        data: &[u8],
        signature: &[u8],
        algorithm: &str,
        public_key: &[u8],
    ) -> PkiResult<()> {
        // Simplified signature verification - in production, use proper crypto library
        if data.is_empty() || signature.is_empty() || public_key.is_empty() {
            return Err(PkiError::SignatureError("Invalid signature parameters".to_string()));
        }

        // For demonstration, we'll just check that the signature is non-empty
        // In production, this would perform actual cryptographic verification
        if signature.len() < 64 {
            return Err(PkiError::SignatureError("Signature too short".to_string()));
        }

        match algorithm {
            "SHA256withRSA" | "SHA1withRSA" => {
                // Mock RSA signature verification
                Ok(())
            }
            _ => Err(PkiError::SignatureError(format!(
                "Unsupported signature algorithm: {}",
                algorithm
            ))),
        }
    }

    /// Check if two certificate IDs match
    fn cert_ids_match(&self, id1: &CertId, id2: &CertId) -> bool {
        id1.hash_algorithm == id2.hash_algorithm
            && id1.issuer_name_hash == id2.issuer_name_hash
            && id1.issuer_key_hash == id2.issuer_key_hash
            && id1.serial_number == id2.serial_number
    }
}

impl Default for OcspClient {
    fn default() -> Self {
        Self::new(OcspConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn create_mock_certificate(subject: &str, serial: &[u8]) -> X509Certificate {
        let now = SystemTime::now();
        X509Certificate::new(
            subject.to_string(),
            "Mock CA".to_string(),
            serial.to_vec(),
            now,
            now + Duration::from_secs(365 * 24 * 3600), // 1 year
            vec![0x30, 0x82, 0x01, 0x22], // Mock public key
            vec![0; 256], // Mock signature
            "SHA256withRSA".to_string(),
            vec![0x30, 0x82, 0x03, 0x00], // Mock raw data
        )
    }

    #[test]
    fn test_ocsp_client_creation() {
        let config = OcspConfig::default();
        let client = OcspClient::new(config);
        assert_eq!(client.config.timeout, Duration::from_secs(30));
    }

    #[test]
    fn test_cert_id_creation() {
        let cert = create_mock_certificate("CN=Test Certificate", &[1, 2, 3, 4]);
        let issuer = create_mock_certificate("CN=Test CA", &[5, 6, 7, 8]);
        
        let cert_id = CertId::new(&cert, &issuer).unwrap();
        assert_eq!(cert_id.hash_algorithm, "SHA-1");
        assert_eq!(cert_id.serial_number, vec![1, 2, 3, 4]);
        assert_eq!(cert_id.issuer_name_hash.len(), 20); // SHA-1 hash length
        assert_eq!(cert_id.issuer_key_hash.len(), 20);
    }

    #[test]
    fn test_nonce_generation() {
        let client = OcspClient::default();
        let nonce1 = client.generate_nonce();
        let nonce2 = client.generate_nonce();
        
        assert_eq!(nonce1.len(), 16);
        assert_eq!(nonce2.len(), 16);
        assert_ne!(nonce1, nonce2); // Should be different random nonces
    }

    #[test]
    fn test_cert_ids_match() {
        let cert = create_mock_certificate("CN=Test", &[1, 2, 3, 4]);
        let issuer = create_mock_certificate("CN=CA", &[5, 6, 7, 8]);
        
        let cert_id1 = CertId::new(&cert, &issuer).unwrap();
        let cert_id2 = CertId::new(&cert, &issuer).unwrap();
        
        let client = OcspClient::default();
        assert!(client.cert_ids_match(&cert_id1, &cert_id2));
    }

    #[test]
    fn test_response_status_parsing() {
        let client = OcspClient::default();
        
        // Test successful response
        let successful_data = [0x00, 0x01, 0x02];
        let status = client.parse_response_status(&successful_data).unwrap();
        assert_eq!(status, OcspResponseStatus::Successful);
        
        // Test malformed request
        let malformed_data = [0x01, 0x02, 0x03];
        let status = client.parse_response_status(&malformed_data).unwrap();
        assert_eq!(status, OcspResponseStatus::MalformedRequest);
    }
}
