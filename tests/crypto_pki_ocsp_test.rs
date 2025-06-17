/// Comprehensive OCSP (Online Certificate Status Protocol) Tests
/// 
/// This test suite validates the complete OCSP implementation including:
/// - OCSP client functionality
/// - Request creation and parsing
/// - Response validation
/// - Caching mechanisms
/// - Error handling
/// - Security features

use cursed::stdlib::packages::crypto_pki::{
    types::{
        X509Certificate, OcspConfig, CertId, RevocationStatus, 
        CertificateStatusInfo, PkiResult
    },
    ocsp::{
        OcspRequest, OcspResponse, OcspCache, OcspValidator, OcspError, OcspResult,
        create_ocsp_request, parse_ocsp_response, check_ocsp_status
    },
    ocsp_client::OcspClient,
};
use std::time::{Duration, SystemTime};

/// Create a mock certificate for testing
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

#[cfg(test)]
mod ocsp_tests {
    use super::*;

    #[test]
    fn test_ocsp_config_creation() {
        let config = OcspConfig::default();
        assert_eq!(config.timeout, Duration::from_secs(30));
        assert_eq!(config.max_response_size, 1024 * 1024);
        assert_eq!(config.user_agent, "CURSED-PKI-OCSP/1.0");
        assert!(config.verify_signature);
        assert!(config.cache_responses);
        assert!(config.nonce_extension);
    }

    #[test]
    fn test_ocsp_client_creation() {
        let config = OcspConfig::default();
        let client = OcspClient::new(config);
        // Client should be created successfully
        // Note: We can't test internal fields as they're private
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
    fn test_ocsp_request_creation() {
        let cert = create_mock_certificate("CN=Test Certificate", &[1, 2, 3, 4]);
        let issuer = create_mock_certificate("CN=Test CA", &[5, 6, 7, 8]);
        
        let mut request = OcspRequest::new();
        assert!(request.add_certificate(&cert, &issuer).is_ok());
        assert_eq!(request.request_list.len(), 1);
        assert!(request.nonce.is_none());
    }

    #[test]
    fn test_ocsp_request_with_nonce() {
        let mut request = OcspRequest::new();
        let nonce = vec![1, 2, 3, 4, 5, 6, 7, 8];
        request.set_nonce(nonce.clone());
        assert_eq!(request.nonce, Some(nonce));
    }

    #[test]
    fn test_ocsp_request_with_extensions() {
        let mut request = OcspRequest::new();
        request.add_extension("1.2.3.4".to_string(), vec![0x01, 0x02]);
        assert!(request.request_extensions.is_some());
        
        let extensions = request.request_extensions.unwrap();
        assert_eq!(extensions.get("1.2.3.4"), Some(&vec![0x01, 0x02]));
    }

    #[test]
    fn test_create_ocsp_request_function() {
        let cert = create_mock_certificate("CN=Test Certificate", &[1, 2, 3, 4]);
        let issuer = create_mock_certificate("CN=Test CA", &[5, 6, 7, 8]);
        
        let request = create_ocsp_request(&cert, &issuer).unwrap();
        assert_eq!(request.request_list.len(), 1);
        assert!(request.nonce.is_none());
    }

    #[test]
    fn test_parse_ocsp_response_successful() {
        // Test successful response
        let success_data = vec![0x00, 0x30, 0x82, 0x01, 0x00]; // Mock DER data with successful status
        let response = parse_ocsp_response(&success_data).unwrap();
        assert!(response.is_successful());
        assert!(response.response_bytes.is_some());
        
        if let Some(basic_response) = &response.response_bytes {
            assert!(!basic_response.responses.is_empty());
            assert_eq!(basic_response.signature_algorithm, "SHA256withRSA");
            assert_eq!(basic_response.responder_id, "MockResponder");
        }
    }

    #[test]
    fn test_parse_ocsp_response_error() {
        // Test error response
        let error_data = vec![0x01]; // Non-zero status indicates error
        let response = parse_ocsp_response(&error_data).unwrap();
        assert!(!response.is_successful());
        assert!(response.response_bytes.is_none());
    }

    #[test]
    fn test_parse_ocsp_response_empty() {
        // Test empty data
        let empty_data = vec![];
        let result = parse_ocsp_response(&empty_data);
        assert!(result.is_err());
        
        match result.unwrap_err() {
            OcspError::InvalidResponse(msg) => {
                assert!(msg.contains("Empty response data"));
            }
            _ => panic!("Expected InvalidResponse error"),
        }
    }

    #[test]
    fn test_ocsp_response_get_single_response() {
        let success_data = vec![0x00, 0x30, 0x82, 0x01, 0x00];
        let response = parse_ocsp_response(&success_data).unwrap();
        
        // Create a matching cert ID
        let cert_id = CertId {
            hash_algorithm: "SHA-1".to_string(),
            issuer_name_hash: vec![0; 20],
            issuer_key_hash: vec![0; 20],
            serial_number: vec![1, 2, 3, 4],
        };
        
        let single_response = response.get_single_response(&cert_id);
        assert!(single_response.is_some());
        
        if let Some(sr) = single_response {
            assert_eq!(sr.cert_status, RevocationStatus::Good);
            assert!(sr.next_update.is_some());
        }
    }

    #[test]
    fn test_ocsp_cache_creation() {
        let cache = OcspCache::new(100, Duration::from_secs(3600));
        let stats = cache.stats();
        assert_eq!(stats.total_entries, 0);
        assert_eq!(stats.valid_entries, 0);
        assert_eq!(stats.max_size, 100);
    }

    #[test]
    fn test_ocsp_cache_operations() {
        let cache = OcspCache::new(10, Duration::from_secs(3600));
        let cert = create_mock_certificate("CN=Test", &[1, 2, 3, 4]);
        let issuer = create_mock_certificate("CN=CA", &[5, 6, 7, 8]);
        
        let cache_key = cache.generate_cache_key(&cert, &issuer);
        
        // Initially empty
        assert!(cache.get(&cache_key).is_none());
        
        // Add entry
        let status_info = CertificateStatusInfo {
            status: RevocationStatus::Good,
            this_update: SystemTime::now(),
            next_update: Some(SystemTime::now() + Duration::from_secs(3600)),
            produced_at: SystemTime::now(),
            responder_id: "Test".to_string(),
        };
        
        cache.put(cache_key.clone(), status_info.clone());
        
        // Should be cached
        let cached = cache.get(&cache_key);
        assert!(cached.is_some());
        
        let cached_info = cached.unwrap();
        assert_eq!(cached_info.status, RevocationStatus::Good);
        assert_eq!(cached_info.responder_id, "Test");
        
        let stats = cache.stats();
        assert_eq!(stats.total_entries, 1);
        assert_eq!(stats.valid_entries, 1);
    }

    #[test]
    fn test_ocsp_cache_expiration() {
        let cache = OcspCache::new(10, Duration::from_secs(1)); // Short TTL for testing
        let cert = create_mock_certificate("CN=Test", &[1, 2, 3, 4]);
        let issuer = create_mock_certificate("CN=CA", &[5, 6, 7, 8]);
        
        let cache_key = cache.generate_cache_key(&cert, &issuer);
        
        // Add entry with past expiration
        let status_info = CertificateStatusInfo {
            status: RevocationStatus::Good,
            this_update: SystemTime::now(),
            next_update: Some(SystemTime::now() - Duration::from_secs(1)), // Expired
            produced_at: SystemTime::now(),
            responder_id: "Test".to_string(),
        };
        
        cache.put(cache_key.clone(), status_info);
        
        // Should not return expired entry
        assert!(cache.get(&cache_key).is_none());
    }

    #[test]
    fn test_ocsp_cache_clear() {
        let cache = OcspCache::new(10, Duration::from_secs(3600));
        let cert = create_mock_certificate("CN=Test", &[1, 2, 3, 4]);
        let issuer = create_mock_certificate("CN=CA", &[5, 6, 7, 8]);
        
        let cache_key = cache.generate_cache_key(&cert, &issuer);
        let status_info = CertificateStatusInfo {
            status: RevocationStatus::Good,
            this_update: SystemTime::now(),
            next_update: Some(SystemTime::now() + Duration::from_secs(3600)),
            produced_at: SystemTime::now(),
            responder_id: "Test".to_string(),
        };
        
        cache.put(cache_key.clone(), status_info);
        assert!(cache.get(&cache_key).is_some());
        
        cache.clear();
        assert!(cache.get(&cache_key).is_none());
        
        let stats = cache.stats();
        assert_eq!(stats.total_entries, 0);
    }

    #[test]
    fn test_ocsp_validator_creation() {
        let config = OcspConfig::default();
        let validator = OcspValidator::new(config);
        
        // Should have cache enabled by default
        let stats = validator.cache_stats();
        assert!(stats.is_some());
        
        if let Some(cache_stats) = stats {
            assert_eq!(cache_stats.total_entries, 0);
            assert_eq!(cache_stats.max_size, 1000); // Default cache size
        }
    }

    #[test]
    fn test_ocsp_validator_with_custom_cache() {
        let config = OcspConfig::default();
        let custom_cache = OcspCache::new(50, Duration::from_secs(1800));
        let validator = OcspValidator::with_cache(config, custom_cache);
        
        let stats = validator.cache_stats();
        assert!(stats.is_some());
        
        if let Some(cache_stats) = stats {
            assert_eq!(cache_stats.max_size, 50); // Custom cache size
        }
    }

    #[test]
    fn test_ocsp_validator_clear_cache() {
        let config = OcspConfig::default();
        let validator = OcspValidator::new(config);
        
        // Clear should not panic even with empty cache
        validator.clear_cache();
        
        let stats = validator.cache_stats();
        assert!(stats.is_some());
    }

    #[test]
    fn test_ocsp_error_types() {
        // Test different OCSP error types
        let network_error = OcspError::NetworkError("Connection failed".to_string());
        assert!(network_error.to_string().contains("OCSP network error"));
        
        let invalid_request = OcspError::InvalidRequest("Bad request".to_string());
        assert!(invalid_request.to_string().contains("Invalid OCSP request"));
        
        let invalid_response = OcspError::InvalidResponse("Bad response".to_string());
        assert!(invalid_response.to_string().contains("Invalid OCSP response"));
        
        let signature_error = OcspError::SignatureVerificationFailed("Signature invalid".to_string());
        assert!(signature_error.to_string().contains("OCSP signature verification failed"));
    }

    #[test]
    fn test_ocsp_error_conversion() {
        use cursed::stdlib::packages::crypto_pki::types::PkiError;
        
        let ocsp_error = OcspError::NetworkError("Test error".to_string());
        let pki_error: PkiError = ocsp_error.into();
        
        match pki_error {
            PkiError::OcspError(msg) => {
                assert!(msg.contains("OCSP network error"));
                assert!(msg.contains("Test error"));
            }
            _ => panic!("Expected OcspError variant"),
        }
    }

    #[test]
    fn test_revocation_status_variants() {
        // Test Good status
        let good_status = RevocationStatus::Good;
        assert_eq!(good_status, RevocationStatus::Good);
        
        // Test Unknown status
        let unknown_status = RevocationStatus::Unknown;
        assert_eq!(unknown_status, RevocationStatus::Unknown);
        
        // Test Revoked status
        let revoked_status = RevocationStatus::Revoked {
            reason: Some(cursed::stdlib::packages::crypto_pki::types::RevocationReason::KeyCompromise),
            revocation_time: SystemTime::now(),
        };
        
        match revoked_status {
            RevocationStatus::Revoked { reason, .. } => {
                assert!(reason.is_some());
            }
            _ => panic!("Expected Revoked status"),
        }
    }

    #[test]
    fn test_certificate_status_info() {
        let now = SystemTime::now();
        let status_info = CertificateStatusInfo {
            status: RevocationStatus::Good,
            this_update: now,
            next_update: Some(now + Duration::from_secs(86400)),
            produced_at: now,
            responder_id: "test-responder".to_string(),
        };
        
        assert_eq!(status_info.status, RevocationStatus::Good);
        assert_eq!(status_info.responder_id, "test-responder");
        assert!(status_info.next_update.is_some());
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_full_ocsp_workflow() {
        // Create certificates
        let cert = create_mock_certificate("CN=Test Certificate", &[1, 2, 3, 4]);
        let issuer = create_mock_certificate("CN=Test CA", &[5, 6, 7, 8]);
        
        // Create OCSP request
        let request = create_ocsp_request(&cert, &issuer).unwrap();
        assert!(!request.request_list.is_empty());
        
        // Simulate response parsing
        let response_data = vec![0x00, 0x30, 0x82, 0x01, 0x00];
        let response = parse_ocsp_response(&response_data).unwrap();
        assert!(response.is_successful());
        
        // Verify we can extract status
        if let Some(basic_response) = response.response_bytes {
            assert!(!basic_response.responses.is_empty());
            let single_response = &basic_response.responses[0];
            assert_eq!(single_response.cert_status, RevocationStatus::Good);
        }
    }

    #[test]
    fn test_ocsp_validator_workflow() {
        let config = OcspConfig::default();
        let validator = OcspValidator::new(config);
        
        // Test that validator is created properly
        assert!(validator.cache_stats().is_some());
        
        // Test cache operations
        validator.clear_cache();
        let stats = validator.cache_stats().unwrap();
        assert_eq!(stats.total_entries, 0);
    }

    #[test]
    fn test_certificate_operations() {
        let cert = create_mock_certificate("CN=Test", &[1, 2, 3, 4]);
        
        // Test certificate properties
        assert_eq!(cert.subject, "CN=Test");
        assert_eq!(cert.issuer, "Mock CA");
        assert_eq!(cert.serial_number, vec![1, 2, 3, 4]);
        assert_eq!(cert.signature_algorithm, "SHA256withRSA");
        
        // Test certificate validity
        assert!(cert.is_currently_valid());
        
        // Test fingerprint
        let fingerprint = cert.fingerprint();
        assert_eq!(fingerprint.len(), 32); // SHA-256 hash
        
        // Test serial hex
        let serial_hex = cert.serial_hex();
        assert_eq!(serial_hex, "01020304");
    }

    #[test]
    fn test_error_handling_chain() {
        // Test that errors propagate correctly through the system
        let empty_data = vec![];
        let result = parse_ocsp_response(&empty_data);
        
        assert!(result.is_err());
        match result.unwrap_err() {
            OcspError::InvalidResponse(msg) => {
                assert!(msg.contains("Empty response data"));
            }
            _ => panic!("Expected InvalidResponse error"),
        }
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;

    #[test]
    fn test_cache_performance() {
        let cache = OcspCache::new(1000, Duration::from_secs(3600));
        
        // Add many entries
        for i in 0..100 {
            let cert = create_mock_certificate(&format!("CN=Test{}", i), &[i as u8]);
            let issuer = create_mock_certificate("CN=CA", &[5, 6, 7, 8]);
            let cache_key = cache.generate_cache_key(&cert, &issuer);
            
            let status_info = CertificateStatusInfo {
                status: RevocationStatus::Good,
                this_update: SystemTime::now(),
                next_update: Some(SystemTime::now() + Duration::from_secs(3600)),
                produced_at: SystemTime::now(),
                responder_id: format!("responder-{}", i),
            };
            
            cache.put(cache_key, status_info);
        }
        
        let stats = cache.stats();
        assert_eq!(stats.total_entries, 100);
        assert_eq!(stats.valid_entries, 100);
    }

    #[test]
    fn test_cert_id_generation_performance() {
        let cert = create_mock_certificate("CN=Test", &[1, 2, 3, 4]);
        let issuer = create_mock_certificate("CN=CA", &[5, 6, 7, 8]);
        
        // Generate many cert IDs
        for _ in 0..100 {
            let cert_id = CertId::new(&cert, &issuer).unwrap();
            assert_eq!(cert_id.hash_algorithm, "SHA-1");
            assert_eq!(cert_id.serial_number.len(), 4);
        }
    }

    #[test]
    fn test_response_parsing_performance() {
        let response_data = vec![0x00, 0x30, 0x82, 0x01, 0x00];
        
        // Parse many responses
        for _ in 0..100 {
            let response = parse_ocsp_response(&response_data).unwrap();
            assert!(response.is_successful());
        }
    }
}
