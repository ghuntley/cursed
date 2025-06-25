/// Integration tests for certificate renewal functionality
/// 
/// Comprehensive integration testing covering:
/// - End-to-end certificate renewal workflows
/// - ACME protocol integration scenarios
/// - Certificate monitoring and expiration detection
/// - Automated renewal scheduling and execution
/// - Error handling and recovery in real scenarios
/// - Certificate backup and rollback operations
/// - Notification system integration
/// - Performance testing under load

use cursed::stdlib::packages::crypto_pki::certificate_renewal::*;
use cursed::stdlib::packages::crypto_pki::{
    types::*,
    error::{PkiError, CertificateErrorCode},
    key_management::{KeyManager, KeyGenerationConfig, KeyPair},
};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::path::PathBuf;
use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;
use tempfile::TempDir;

/// Integration test for complete certificate renewal manager lifecycle
#[test]
fn test_certificate_renewal_manager_lifecycle() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create comprehensive test configuration
    let config = create_test_renewal_config(&temp_dir);
    
    // Create and initialize renewal manager
    let mut manager = CertificateRenewalManager::new(config);
    let init_result = manager.initialize();
    assert!(init_result.is_ok(), "Manager initialization should succeed");
    
    // Verify initialization created necessary components
    assert!(manager.acme_clients.contains_key("default"));
    assert!(temp_dir.path().exists());
    
    // Test statistics initial state
    let stats = manager.get_renewal_statistics().expect("Should get statistics");
    assert_eq!(stats.total_renewal_attempts, 0);
    assert_eq!(stats.successful_renewals, 0);
    assert_eq!(stats.monitored_certificates_count, 0);
    
    // Clean up
    let shutdown_result = manager.shutdown();
    assert!(shutdown_result.is_ok(), "Manager shutdown should succeed");
}

/// Integration test for certificate monitoring and status detection
#[test]
fn test_certificate_monitoring_integration() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config = create_test_renewal_config(&temp_dir);
    let mut manager = CertificateRenewalManager::new(config);
    manager.initialize().expect("Manager initialization should succeed");
    
    // Create test certificate files
    let cert_path = temp_dir.path().join("test_cert.crt");
    let key_path = temp_dir.path().join("test_cert.key");
    create_test_certificate_files(&cert_path, &key_path);
    
    // Create test certificate
    let test_certificate = create_test_certificate(Duration::from_secs(30 * 24 * 3600)); // 30 days validity
    
    // Create monitoring configuration
    let monitoring_config = CertificateMonitoringConfig {
        renewal_days_before_expiry: 15,
        renewal_method: RenewalMethod::Acme {
            client_id: "default".to_string(),
            challenge_method: AcmeChallenge::Http01 {
                webroot_path: temp_dir.path().to_path_buf(),
            },
        },
        auto_renewal_enabled: true,
        notification_preferences: NotificationPreferences {
            email_enabled: false,
            webhook_enabled: false,
            custom_endpoints: Vec::new(),
            frequency_limits: NotificationFrequencyLimits {
                max_per_hour: 5,
                max_per_day: 20,
                duplicate_cooldown_minutes: 15,
            },
        },
        validation_requirements: ValidationRequirements::default(),
    };
    
    // Add certificate to monitoring
    let add_result = manager.add_certificate_to_monitoring(
        "test_cert_001".to_string(),
        test_certificate,
        cert_path,
        key_path,
        monitoring_config,
    );
    assert!(add_result.is_ok(), "Adding certificate to monitoring should succeed");
    
    // Verify certificate was added
    let monitored_certs = manager.get_monitored_certificates().expect("Should get monitored certificates");
    assert_eq!(monitored_certs.len(), 1);
    assert!(monitored_certs.contains(&"test_cert_001".to_string()));
    
    // Check certificate status
    let cert_status = manager.get_certificate_status("test_cert_001").expect("Should get certificate status");
    assert_eq!(cert_status, CertificateStatus::Valid);
    
    // Test statistics update
    let stats = manager.get_renewal_statistics().expect("Should get statistics");
    assert_eq!(stats.monitored_certificates_count, 1);
    
    // Remove certificate from monitoring
    let remove_result = manager.remove_certificate_from_monitoring("test_cert_001");
    assert!(remove_result.is_ok(), "Removing certificate from monitoring should succeed");
    
    // Verify certificate was removed
    let monitored_certs_after = manager.get_monitored_certificates().expect("Should get monitored certificates");
    assert_eq!(monitored_certs_after.len(), 0);
    
    manager.shutdown().expect("Manager shutdown should succeed");
}

/// Integration test for manual certificate renewal workflow
#[test]
fn test_manual_certificate_renewal_workflow() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config = create_test_renewal_config(&temp_dir);
    let mut manager = CertificateRenewalManager::new(config);
    manager.initialize().expect("Manager initialization should succeed");
    
    // Create and add test certificate
    let cert_path = temp_dir.path().join("manual_cert.crt");
    let key_path = temp_dir.path().join("manual_cert.key");
    create_test_certificate_files(&cert_path, &key_path);
    
    let test_certificate = create_test_certificate(Duration::from_secs(10 * 24 * 3600)); // 10 days validity
    let monitoring_config = create_test_monitoring_config(&temp_dir);
    
    manager.add_certificate_to_monitoring(
        "manual_cert_001".to_string(),
        test_certificate,
        cert_path,
        key_path,
        monitoring_config,
    ).expect("Adding certificate should succeed");
    
    // Trigger manual renewal
    let renewal_method = RenewalMethod::Manual {
        instructions: "Contact CA support for manual certificate renewal".to_string(),
    };
    
    let task_id = manager.trigger_manual_renewal("manual_cert_001", Some(renewal_method))
        .expect("Manual renewal trigger should succeed");
    
    // Verify task was created
    assert!(!task_id.is_empty());
    assert!(task_id.starts_with("manual_renewal_"));
    
    // Check task status (should be pending manual intervention)
    thread::sleep(Duration::from_millis(100)); // Allow time for processing
    let task_status = manager.get_renewal_task_status(&task_id);
    
    // Task might be in different states depending on processing speed
    assert!(task_status.is_ok(), "Should be able to get task status");
    
    // Verify statistics were updated
    let stats = manager.get_renewal_statistics().expect("Should get statistics");
    assert!(stats.total_renewal_attempts >= 1);
    
    manager.shutdown().expect("Manager shutdown should succeed");
}

/// Integration test for ACME client initialization and configuration
#[test]
fn test_acme_client_integration() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create ACME configuration with test settings
    let acme_config = AcmeConfig {
        directory_url: "https://acme-staging-v02.api.letsencrypt.org/directory".to_string(),
        contact_email: "test@example.com".to_string(),
        terms_of_service_agreed: true,
        supported_challenges: vec![
            AcmeChallenge::Http01 {
                webroot_path: temp_dir.path().to_path_buf(),
            },
            AcmeChallenge::Dns01 {
                dns_provider: DnsProvider {
                    provider_type: "manual".to_string(),
                    credentials: HashMap::new(),
                    record_ttl: 300,
                    propagation_wait_seconds: 60,
                },
            },
        ],
        challenge_timeout_seconds: 300,
        account_key_pair: None,
        external_account_binding: None,
    };
    
    // Create ACME client
    let mut client = AcmeClient::new("test_client".to_string(), acme_config.clone())
        .expect("ACME client creation should succeed");
    
    // Test client properties
    assert_eq!(client.client_id, "test_client");
    assert_eq!(client.config.directory_url, acme_config.directory_url);
    assert_eq!(client.config.contact_email, acme_config.contact_email);
    assert_eq!(client.config.supported_challenges.len(), 2);
    assert_eq!(client.statistics.total_certificate_requests, 0);
    
    // Test account initialization (would create mock account)
    let init_result = client.initialize_account();
    assert!(init_result.is_ok(), "Account initialization should succeed");
    
    // Verify account was created
    assert!(client.account.is_some());
    if let Some(ref account) = client.account {
        assert!(!account.account_url.is_empty());
        assert_eq!(account.status, AcmeAccountStatus::Valid);
        assert!(account.terms_of_service_agreed);
    }
    
    // Test order creation
    let domain_names = vec!["test.example.com".to_string(), "www.test.example.com".to_string()];
    let order_result = client.create_order(&domain_names);
    assert!(order_result.is_ok(), "Order creation should succeed");
    
    let order = order_result.unwrap();
    assert_eq!(order.status, AcmeOrderStatus::Pending);
    assert_eq!(order.identifiers.len(), 2);
    assert_eq!(order.identifiers[0].value, "test.example.com");
    assert_eq!(order.identifiers[1].value, "www.test.example.com");
    
    // Verify statistics were updated
    assert_eq!(client.statistics.total_certificate_requests, 1);
}

/// Integration test for certificate storage operations
#[test]
fn test_certificate_storage_integration() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create storage configuration
    let storage_config = StorageConfig {
        base_directory: temp_dir.path().to_path_buf(),
        certificate_filename_pattern: "cert_{id}.pem".to_string(),
        private_key_filename_pattern: "key_{id}.pem".to_string(),
        chain_filename_pattern: "chain_{id}.pem".to_string(),
        certificate_permissions: 0o644,
        private_key_permissions: 0o600,
        atomic_operations: true,
        backup_retention_days: 30,
    };
    
    // Create storage manager
    let mut storage_manager = CertificateStorageManager::new(storage_config);
    let init_result = storage_manager.initialize();
    assert!(init_result.is_ok(), "Storage manager initialization should succeed");
    
    // Create test certificate and key
    let test_certificate = create_test_certificate(Duration::from_secs(90 * 24 * 3600));
    let test_private_key = b"-----BEGIN PRIVATE KEY-----\nMIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQC7VJTUt9Us8cKB\n-----END PRIVATE KEY-----\n";
    
    // Store certificate
    let store_result = storage_manager.store_certificate(
        "integration_test_001",
        &test_certificate,
        test_private_key,
    );
    assert!(store_result.is_ok(), "Certificate storage should succeed");
    
    // Verify files were created
    let cert_path = temp_dir.path().join("cert_integration_test_001.pem");
    let key_path = temp_dir.path().join("key_integration_test_001.pem");
    
    assert!(cert_path.exists(), "Certificate file should exist");
    assert!(key_path.exists(), "Private key file should exist");
    
    // Verify file contents
    let stored_cert_data = fs::read(&cert_path).expect("Should be able to read certificate file");
    let stored_key_data = fs::read(&key_path).expect("Should be able to read private key file");
    
    assert_eq!(stored_cert_data, test_certificate.raw_data);
    assert_eq!(stored_key_data, test_private_key);
    
    // Test backup functionality
    let monitored_cert = create_test_monitored_certificate(&cert_path, &key_path);
    let backup_result = storage_manager.backup_certificate(&monitored_cert);
    assert!(backup_result.is_ok(), "Certificate backup should succeed");
    
    // Verify backup was created
    let backup_dir = temp_dir.path().join("backups").join("test_cert_001");
    assert!(backup_dir.exists(), "Backup directory should exist");
    
    // Verify statistics were updated
    assert_eq!(storage_manager.statistics.certificates_stored, 1);
    assert_eq!(storage_manager.statistics.total_operations, 1);
}

/// Integration test for notification system
#[test]
fn test_notification_system_integration() {
    let notification_config = NotificationConfig {
        email_config: Some(EmailConfig {
            smtp_server: "smtp.example.com".to_string(),
            smtp_port: 587,
            smtp_auth: Some(SmtpAuth {
                username: "noreply@example.com".to_string(),
                password: "test_password".to_string(),
                mechanism: SmtpAuthMechanism::Plain,
            }),
            from_address: "noreply@example.com".to_string(),
            to_addresses: vec!["admin@example.com".to_string()],
            subject_prefix: "[CURSED PKI TEST]".to_string(),
        }),
        webhook_config: Some(WebhookConfig {
            url: "https://hooks.slack.com/test".to_string(),
            method: "POST".to_string(),
            headers: {
                let mut headers = HashMap::new();
                headers.insert("Content-Type".to_string(), "application/json".to_string());
                headers
            },
            auth: Some(WebhookAuth::BearerToken("test_token".to_string())),
            timeout_seconds: 30,
            retry_policy: RetryPolicy::default(),
        }),
        enable_log_notifications: true,
        notification_thresholds: NotificationThresholds {
            expiration_warning_days: 30,
            expiration_critical_days: 7,
            notify_on_success: true,
            notify_on_failure: true,
            notify_on_config_change: false,
        },
    };
    
    // Create notification manager
    let mut notification_manager = RenewalNotificationManager::new(notification_config.clone());
    let init_result = notification_manager.initialize();
    assert!(init_result.is_ok(), "Notification manager initialization should succeed");
    
    // Test manual renewal notification
    let notification_result = notification_manager.send_manual_renewal_notification(
        "test_cert_001",
        "Manual renewal required. Please contact CA support.",
    );
    assert!(notification_result.is_ok(), "Manual renewal notification should succeed");
    
    // Verify notification was recorded
    if let Ok(history) = notification_manager.notification_history.lock() {
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].notification_type, NotificationType::RenewalFailure);
        assert!(history[0].content.contains("Manual renewal required"));
    }
    
    // Verify statistics were updated
    assert_eq!(notification_manager.statistics.total_notifications, 1);
}

/// Integration test for retry policy and error handling
#[test]
fn test_retry_policy_integration() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create configuration with aggressive retry policy for testing
    let mut config = create_test_renewal_config(&temp_dir);
    config.retry_policy = RetryPolicy {
        max_attempts: 3,
        initial_delay_seconds: 1, // Short delay for testing
        backoff_multiplier: 2.0,
        max_delay_seconds: 10,
        jitter_factor: 0.1,
    };
    
    let manager = CertificateRenewalManager::new(config);
    
    // Test retry policy calculations
    let retry_policy = &manager.config.retry_policy;
    assert_eq!(retry_policy.max_attempts, 3);
    assert_eq!(retry_policy.initial_delay_seconds, 1);
    assert_eq!(retry_policy.backoff_multiplier, 2.0);
    
    // Test backoff calculation simulation
    let mut delay = retry_policy.initial_delay_seconds;
    let mut calculated_delays = Vec::new();
    
    for attempt in 1..=retry_policy.max_attempts {
        calculated_delays.push(delay);
        if attempt < retry_policy.max_attempts {
            delay = ((delay as f64) * retry_policy.backoff_multiplier) as u64;
            delay = delay.min(retry_policy.max_delay_seconds);
        }
    }
    
    assert_eq!(calculated_delays, vec![1, 2, 4]);
}

/// Integration test for certificate expiration detection
#[test]
fn test_certificate_expiration_detection() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config = create_test_renewal_config(&temp_dir);
    let mut manager = CertificateRenewalManager::new(config);
    manager.initialize().expect("Manager initialization should succeed");
    
    // Create certificates with different expiration times
    let near_expiry_cert = create_test_certificate(Duration::from_secs(5 * 24 * 3600)); // 5 days
    let valid_cert = create_test_certificate(Duration::from_secs(60 * 24 * 3600)); // 60 days
    let expired_cert = create_expired_test_certificate(); // Already expired
    
    // Test expiration calculation for near expiry certificate
    let days_until_expiry = CertificateRenewalManager::calculate_days_until_expiry(&near_expiry_cert)
        .expect("Should calculate days until expiry");
    assert!(days_until_expiry <= 5, "Should detect certificate is near expiry");
    
    // Test expiration calculation for valid certificate
    let days_until_expiry_valid = CertificateRenewalManager::calculate_days_until_expiry(&valid_cert)
        .expect("Should calculate days until expiry");
    assert!(days_until_expiry_valid >= 50, "Should detect certificate is still valid");
    
    // Test expiration calculation for expired certificate
    let days_until_expiry_expired = CertificateRenewalManager::calculate_days_until_expiry(&expired_cert)
        .expect("Should handle expired certificate");
    assert_eq!(days_until_expiry_expired, 0, "Should detect certificate is expired");
    
    manager.shutdown().expect("Manager shutdown should succeed");
}

/// Integration test for configuration validation and edge cases
#[test]
fn test_configuration_validation_integration() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Test configuration with extreme values
    let extreme_config = RenewalConfig {
        default_renewal_days_before_expiry: 365, // Very long renewal period
        max_concurrent_renewals: 100, // High concurrency
        retry_policy: RetryPolicy {
            max_attempts: 10,
            initial_delay_seconds: 1,
            backoff_multiplier: 3.0,
            max_delay_seconds: 86400, // 24 hours
            jitter_factor: 0.5,
        },
        acme_config: AcmeConfig {
            directory_url: "https://acme-staging-v02.api.letsencrypt.org/directory".to_string(),
            contact_email: "test@example.com".to_string(),
            terms_of_service_agreed: true,
            supported_challenges: vec![
                AcmeChallenge::Http01 {
                    webroot_path: temp_dir.path().to_path_buf(),
                },
                AcmeChallenge::Dns01 {
                    dns_provider: DnsProvider {
                        provider_type: "manual".to_string(),
                        credentials: HashMap::new(),
                        record_ttl: 1,
                        propagation_wait_seconds: 0,
                    },
                },
                AcmeChallenge::TlsAlpn01 { port: 443 },
            ],
            challenge_timeout_seconds: 86400, // 24 hours
            account_key_pair: None,
            external_account_binding: None,
        },
        storage_config: StorageConfig {
            base_directory: temp_dir.path().to_path_buf(),
            certificate_filename_pattern: "cert_{id}_{timestamp}.pem".to_string(),
            private_key_filename_pattern: "key_{id}_{timestamp}.pem".to_string(),
            chain_filename_pattern: "chain_{id}_{timestamp}.pem".to_string(),
            certificate_permissions: 0o600,
            private_key_permissions: 0o400, // Very restrictive
            atomic_operations: true,
            backup_retention_days: 3650, // 10 years
        },
        notification_config: NotificationConfig {
            email_config: None,
            webhook_config: None,
            enable_log_notifications: true,
            notification_thresholds: NotificationThresholds {
                expiration_warning_days: 180, // 6 months
                expiration_critical_days: 90, // 3 months
                notify_on_success: true,
                notify_on_failure: true,
                notify_on_config_change: true,
            },
        },
        validation_requirements: ValidationRequirements {
            validate_chain: true,
            validate_ocsp: true,
            validate_crl: true,
            custom_policies: vec![
                "strict_validation".to_string(),
                "require_ct_logs".to_string(),
                "check_weak_keys".to_string(),
            ],
            validation_timeout_seconds: 300,
            rollback_on_validation_failure: true,
        },
        backup_config: BackupConfig {
            enable_auto_backup: true,
            backup_directory: temp_dir.path().join("comprehensive_backups"),
            max_backup_versions: 100,
            enable_compression: true,
            verify_backups: true,
            auto_cleanup_enabled: true,
        },
        monitoring_config: MonitoringConfig {
            scan_interval_hours: 1, // Frequent scanning
            enable_proactive_monitoring: true,
            monitor_ct_logs: true,
            health_check_config: Some(HealthCheckConfig {
                port: 8080,
                path: "/health/certificates".to_string(),
                include_certificate_details: true,
            }),
            metrics_config: Some(MetricsConfig {
                format: "prometheus".to_string(),
                endpoint: "http://localhost:9090/metrics".to_string(),
                export_interval_seconds: 30,
            }),
        },
    };
    
    // Test that extreme configuration is accepted
    let manager = CertificateRenewalManager::new(extreme_config.clone());
    assert_eq!(manager.config.default_renewal_days_before_expiry, 365);
    assert_eq!(manager.config.max_concurrent_renewals, 100);
    assert_eq!(manager.config.retry_policy.max_attempts, 10);
    assert_eq!(manager.config.acme_config.supported_challenges.len(), 3);
    assert_eq!(manager.config.validation_requirements.custom_policies.len(), 3);
    assert_eq!(manager.config.backup_config.max_backup_versions, 100);
}

/// Integration test for concurrent operations
#[test]
fn test_concurrent_operations_integration() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config = create_test_renewal_config(&temp_dir);
    let manager = Arc::new(Mutex::new(CertificateRenewalManager::new(config)));
    
    // Initialize manager
    manager.lock().unwrap().initialize().expect("Manager initialization should succeed");
    
    // Test concurrent certificate additions
    let handles: Vec<_> = (0..5).map(|i| {
        let manager_clone = Arc::clone(&manager);
        let temp_dir_path = temp_dir.path().to_path_buf();
        
        thread::spawn(move || {
            let cert_id = format!("concurrent_cert_{:03}", i);
            let cert_path = temp_dir_path.join(format!("cert_{}.crt", i));
            let key_path = temp_dir_path.join(format!("key_{}.key", i));
            
            // Create test files
            fs::write(&cert_path, b"test certificate data").expect("Should write cert file");
            fs::write(&key_path, b"test private key data").expect("Should write key file");
            
            let test_certificate = create_test_certificate(Duration::from_secs(30 * 24 * 3600));
            let monitoring_config = create_test_monitoring_config(&temp_dir_path);
            
            let result = manager_clone.lock().unwrap().add_certificate_to_monitoring(
                cert_id,
                test_certificate,
                cert_path,
                key_path,
                monitoring_config,
            );
            
            assert!(result.is_ok(), "Concurrent certificate addition should succeed");
        })
    }).collect();
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread should complete successfully");
    }
    
    // Verify all certificates were added
    let monitored_certs = manager.lock().unwrap().get_monitored_certificates()
        .expect("Should get monitored certificates");
    assert_eq!(monitored_certs.len(), 5);
    
    // Verify statistics
    let stats = manager.lock().unwrap().get_renewal_statistics()
        .expect("Should get statistics");
    assert_eq!(stats.monitored_certificates_count, 5);
    
    // Cleanup
    manager.lock().unwrap().shutdown().expect("Manager shutdown should succeed");
}

/// Performance integration test for renewal system under load
#[test]
fn test_renewal_system_performance() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config = create_test_renewal_config(&temp_dir);
    let mut manager = CertificateRenewalManager::new(config);
    manager.initialize().expect("Manager initialization should succeed");
    
    let start_time = SystemTime::now();
    
    // Add multiple certificates for monitoring
    for i in 0..50 {
        let cert_id = format!("perf_test_cert_{:03}", i);
        let cert_path = temp_dir.path().join(format!("perf_cert_{}.crt", i));
        let key_path = temp_dir.path().join(format!("perf_key_{}.key", i));
        
        create_test_certificate_files(&cert_path, &key_path);
        
        let test_certificate = create_test_certificate(Duration::from_secs((30 + i) * 24 * 3600));
        let monitoring_config = create_test_monitoring_config(&temp_dir);
        
        let result = manager.add_certificate_to_monitoring(
            cert_id,
            test_certificate,
            cert_path,
            key_path,
            monitoring_config,
        );
        assert!(result.is_ok(), "Certificate addition should succeed");
    }
    
    let add_duration = start_time.elapsed().expect("Should get elapsed time");
    println!("Added 50 certificates in {:?}", add_duration);
    
    // Verify all certificates were added efficiently
    let monitored_certs = manager.get_monitored_certificates()
        .expect("Should get monitored certificates");
    assert_eq!(monitored_certs.len(), 50);
    
    // Test bulk status retrieval performance
    let status_start = SystemTime::now();
    for cert_id in &monitored_certs {
        let _status = manager.get_certificate_status(cert_id)
            .expect("Should get certificate status");
    }
    let status_duration = status_start.elapsed().expect("Should get elapsed time");
    println!("Retrieved 50 certificate statuses in {:?}", status_duration);
    
    // Performance assertions
    assert!(add_duration < Duration::from_secs(5), "Adding certificates should be fast");
    assert!(status_duration < Duration::from_secs(2), "Status retrieval should be fast");
    
    manager.shutdown().expect("Manager shutdown should succeed");
}

// Helper functions for integration tests

fn create_test_renewal_config(temp_dir: &TempDir) -> RenewalConfig {
    RenewalConfig {
        default_renewal_days_before_expiry: 30,
        max_concurrent_renewals: 5,
        retry_policy: RetryPolicy::default(),
        acme_config: AcmeConfig {
            directory_url: "https://acme-staging-v02.api.letsencrypt.org/directory".to_string(),
            contact_email: "test@example.com".to_string(),
            terms_of_service_agreed: true,
            supported_challenges: vec![AcmeChallenge::Http01 {
                webroot_path: temp_dir.path().to_path_buf(),
            }],
            challenge_timeout_seconds: 300,
            account_key_pair: None,
            external_account_binding: None,
        },
        storage_config: StorageConfig {
            base_directory: temp_dir.path().to_path_buf(),
            certificate_filename_pattern: "{id}.crt".to_string(),
            private_key_filename_pattern: "{id}.key".to_string(),
            chain_filename_pattern: "{id}_chain.crt".to_string(),
            certificate_permissions: 0o644,
            private_key_permissions: 0o600,
            atomic_operations: true,
            backup_retention_days: 90,
        },
        notification_config: NotificationConfig {
            email_config: None,
            webhook_config: None,
            enable_log_notifications: true,
            notification_thresholds: NotificationThresholds::default(),
        },
        validation_requirements: ValidationRequirements::default(),
        backup_config: BackupConfig::default(),
        monitoring_config: MonitoringConfig::default(),
    }
}

fn create_test_monitoring_config(temp_dir: &TempDir) -> CertificateMonitoringConfig {
    CertificateMonitoringConfig {
        renewal_days_before_expiry: 30,
        renewal_method: RenewalMethod::Acme {
            client_id: "default".to_string(),
            challenge_method: AcmeChallenge::Http01 {
                webroot_path: temp_dir.path().to_path_buf(),
            },
        },
        auto_renewal_enabled: true,
        notification_preferences: NotificationPreferences {
            email_enabled: false,
            webhook_enabled: false,
            custom_endpoints: Vec::new(),
            frequency_limits: NotificationFrequencyLimits {
                max_per_hour: 5,
                max_per_day: 20,
                duplicate_cooldown_minutes: 15,
            },
        },
        validation_requirements: ValidationRequirements::default(),
    }
}

fn create_test_certificate(validity_duration: Duration) -> X509Certificate {
    let now = SystemTime::now();
    
    X509Certificate {
        version: 3,
        serial_number: SerialNumber::from_big_int(12345),
        signature_algorithm: SignatureAlgorithm::RsaWithSha256,
        issuer: DistinguishedName {
            common_name: Some("Test CA".to_string()),
            organization: Some("Test Organization".to_string()),
            organizational_unit: None,
            country: Some("US".to_string()),
            state_or_province: None,
            locality: None,
            email_address: None,
            additional_attributes: HashMap::new(),
        },
        validity: Validity {
            not_before: now,
            not_after: now + validity_duration,
        },
        subject: DistinguishedName {
            common_name: Some("test.example.com".to_string()),
            organization: Some("Test Organization".to_string()),
            organizational_unit: None,
            country: Some("US".to_string()),
            state_or_province: None,
            locality: None,
            email_address: None,
            additional_attributes: HashMap::new(),
        },
        subject_public_key_info: SubjectPublicKeyInfo {
            algorithm: PublicKeyAlgorithm::Rsa { key_size: 2048 },
            public_key: vec![0x30, 0x82, 0x01, 0x22], // Mock RSA public key
            parameters: None,
        },
        extensions: Vec::new(),
        raw_data: b"-----BEGIN CERTIFICATE-----\nMIIDXTCCAkWgAwIBAgIJAKoK/hU7hCiRMA0GCSqGSIb3DQEBCwUAMEUxCzAJBgNV\n-----END CERTIFICATE-----\n".to_vec(),
        fingerprint: None,
        key_usage: KeyUsage::default(),
        extended_key_usage: ExtendedKeyUsage::default(),
    }
}

fn create_expired_test_certificate() -> X509Certificate {
    let now = SystemTime::now();
    let past_time = now - Duration::from_secs(30 * 24 * 3600); // 30 days ago
    
    X509Certificate {
        version: 3,
        serial_number: SerialNumber::from_big_int(54321),
        signature_algorithm: SignatureAlgorithm::RsaWithSha256,
        issuer: DistinguishedName {
            common_name: Some("Test CA".to_string()),
            organization: Some("Test Organization".to_string()),
            organizational_unit: None,
            country: Some("US".to_string()),
            state_or_province: None,
            locality: None,
            email_address: None,
            additional_attributes: HashMap::new(),
        },
        validity: Validity {
            not_before: past_time - Duration::from_secs(365 * 24 * 3600), // 1 year ago
            not_after: past_time, // 30 days ago (expired)
        },
        subject: DistinguishedName {
            common_name: Some("expired.example.com".to_string()),
            organization: Some("Test Organization".to_string()),
            organizational_unit: None,
            country: Some("US".to_string()),
            state_or_province: None,
            locality: None,
            email_address: None,
            additional_attributes: HashMap::new(),
        },
        subject_public_key_info: SubjectPublicKeyInfo {
            algorithm: PublicKeyAlgorithm::Rsa { key_size: 2048 },
            public_key: vec![0x30, 0x82, 0x01, 0x22],
            parameters: None,
        },
        extensions: Vec::new(),
        raw_data: b"-----BEGIN CERTIFICATE-----\nMIIDXTCCAkWgAwIBAgIJAKoK/hU7hCiRMA0GCSqGSIb3DQEBCwUAMEUxCzAJBgNV\n-----END CERTIFICATE-----\n".to_vec(),
        fingerprint: None,
        key_usage: KeyUsage::default(),
        extended_key_usage: ExtendedKeyUsage::default(),
    }
}

fn create_test_certificate_files(cert_path: &PathBuf, key_path: &PathBuf) {
    let cert_content = b"-----BEGIN CERTIFICATE-----\nMIIDXTCCAkWgAwIBAgIJAKoK/hU7hCiRMA0GCSqGSIb3DQEBCwUAMEUxCzAJBgNV\n-----END CERTIFICATE-----\n";
    let key_content = b"-----BEGIN PRIVATE KEY-----\nMIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQC7VJTUt9Us8cKB\n-----END PRIVATE KEY-----\n";
    
    fs::write(cert_path, cert_content).expect("Should write certificate file");
    fs::write(key_path, key_content).expect("Should write private key file");
}

fn create_test_monitored_certificate(cert_path: &PathBuf, key_path: &PathBuf) -> MonitoredCertificate {
    MonitoredCertificate {
        certificate_id: "test_cert_001".to_string(),
        certificate: create_test_certificate(Duration::from_secs(30 * 24 * 3600)),
        certificate_path: cert_path.clone(),
        private_key_path: key_path.clone(),
        monitoring_config: CertificateMonitoringConfig {
            renewal_days_before_expiry: 30,
            renewal_method: RenewalMethod::Manual {
                instructions: "Test renewal".to_string(),
            },
            auto_renewal_enabled: false,
            notification_preferences: NotificationPreferences {
                email_enabled: false,
                webhook_enabled: false,
                custom_endpoints: Vec::new(),
                frequency_limits: NotificationFrequencyLimits {
                    max_per_hour: 5,
                    max_per_day: 20,
                    duplicate_cooldown_minutes: 15,
                },
            },
            validation_requirements: ValidationRequirements::default(),
        },
        last_checked: SystemTime::now(),
        status: CertificateStatus::Valid,
        renewal_history: Vec::new(),
        next_renewal_time: None,
    }
}
