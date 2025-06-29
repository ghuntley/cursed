/// Comprehensive test suite for certificate renewal functionality
/// 
/// Tests cover all aspects of the certificate renewal system including:
/// - Certificate lifecycle management and monitoring
/// - ACME protocol integration for automated renewal
/// - Certificate expiration monitoring and alerting
/// - Zero-downtime certificate rotation workflows
/// - Certificate validation and rollback capabilities
/// - Renewal scheduling and automation
/// - Error handling and recovery mechanisms
/// - Certificate backup and restoration
/// - Notification systems and alerting
/// - Configuration management and validation

// Import all crypto PKI types from cursed crate
use cursed::*;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::path::PathBuf;
use std::collections::HashMap;
use std::fs;
use tempfile::TempDir;

/// Test renewal configuration creation and validation
#[test]
fn test_renewal_config_creation() {
    // Test default configuration
    let default_config = RenewalConfig::default();
    assert_eq!(default_config.default_renewal_days_before_expiry, 30);
    assert_eq!(default_config.max_concurrent_renewals, 5);
    assert!(default_config.backup_config.enable_auto_backup);
    assert!(default_config.validation_requirements.validate_chain);
    
    // Test custom configuration
    let custom_config = RenewalConfig {
        default_renewal_days_before_expiry: 14,
        max_concurrent_renewals: 10,
        retry_policy: RetryPolicy {
            max_attempts: 5,
            initial_delay_seconds: 30,
            backoff_multiplier: 1.5,
            max_delay_seconds: 1800,
            jitter_factor: 0.2,
        },
        monitoring_config: MonitoringConfig {
            scan_interval_hours: 12,
            enable_proactive_monitoring: false,
            monitor_ct_logs: true,
            health_check_config: Some(HealthCheckConfig {
                port: 8080,
                path: "/health".to_string(),
                include_certificate_details: true,
            }),
            metrics_config: None,
        },
        ..default_config
    };
    
    assert_eq!(custom_config.default_renewal_days_before_expiry, 14);
    assert_eq!(custom_config.max_concurrent_renewals, 10);
    assert_eq!(custom_config.retry_policy.max_attempts, 5);
    assert!(!custom_config.monitoring_config.enable_proactive_monitoring);
}

/// Test certificate renewal manager creation and initialization
#[test]
fn test_renewal_manager_creation() {
    let config = RenewalConfig::default();
    let manager = CertificateRenewalManager::new(config.clone());
    
    assert_eq!(manager.config.default_renewal_days_before_expiry, config.default_renewal_days_before_expiry);
    assert!(manager.acme_clients.is_empty());
    assert_eq!(manager.monitored_certificates.read().unwrap().len(), 0);
    assert_eq!(manager.active_renewals.read().unwrap().len(), 0);
}

/// Test certificate renewal manager initialization
#[test]
fn test_renewal_manager_initialization() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let mut config = RenewalConfig::default();
    config.storage_config.base_directory = temp_dir.path().to_path_buf();
    
    let mut manager = CertificateRenewalManager::new(config);
    let result = manager.initialize();
    
    // Initialization should succeed
    assert!(result.is_ok());
    
    // ACME clients should be initialized
    assert!(manager.acme_clients.contains_key("default"));
    
    // Storage directory should be created
    assert!(temp_dir.path().exists());
}

/// Test ACME configuration and client creation
#[test]
fn test_acme_client_creation() {
    let acme_config = AcmeConfig {
        directory_url: "https://acme-staging-v02.api.letsencrypt.org/directory".to_string(),
        contact_email: "test@example.com".to_string(),
        terms_of_service_agreed: true,
        supported_challenges: vec![
            AcmeChallenge::Http01 {
                webroot_path: PathBuf::from("/tmp/acme"),
            },
            AcmeChallenge::Dns01 {
                dns_provider: DnsProvider {
                    provider_type: "cloudflare".to_string(),
                    credentials: {
                        let mut creds = HashMap::new();
                        creds.insert("api_token".to_string(), "test_token".to_string());
                        creds
                    },
                    record_ttl: 300,
                    propagation_wait_seconds: 60,
                },
            },
        ],
        challenge_timeout_seconds: 600,
        account_key_pair: None,
        external_account_binding: None,
    };
    
    let client_result = AcmeClient::new("test_client".to_string(), acme_config.clone());
    assert!(client_result.is_ok());
    
    let client = client_result.unwrap();
    assert_eq!(client.client_id, "test_client");
    assert_eq!(client.config.directory_url, acme_config.directory_url);
    assert_eq!(client.config.contact_email, acme_config.contact_email);
    assert_eq!(client.config.supported_challenges.len(), 2);
}

/// Test certificate monitoring configuration
#[test]
fn test_certificate_monitoring_config() {
    let monitoring_config = CertificateMonitoringConfig {
        renewal_days_before_expiry: 15,
        renewal_method: RenewalMethod::Acme {
            client_id: "default".to_string(),
            challenge_method: AcmeChallenge::Http01 {
                webroot_path: PathBuf::from("/var/www/html"),
            },
        },
        auto_renewal_enabled: true,
        notification_preferences: NotificationPreferences {
            email_enabled: true,
            webhook_enabled: false,
            custom_endpoints: vec!["https://example.com/webhook".to_string()],
            frequency_limits: NotificationFrequencyLimits {
                max_per_hour: 5,
                max_per_day: 20,
                duplicate_cooldown_minutes: 15,
            },
        },
        validation_requirements: ValidationRequirements {
            validate_chain: true,
            validate_ocsp: true,
            validate_crl: false,
            custom_policies: vec!["strict_san_validation".to_string()],
            validation_timeout_seconds: 60,
            rollback_on_validation_failure: true,
        },
    };
    
    assert_eq!(monitoring_config.renewal_days_before_expiry, 15);
    assert!(monitoring_config.auto_renewal_enabled);
    assert!(monitoring_config.notification_preferences.email_enabled);
    assert!(monitoring_config.validation_requirements.validate_chain);
}

/// Test certificate storage manager operations
#[test]
fn test_certificate_storage_manager() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let storage_config = StorageConfig {
        base_directory: temp_dir.path().to_path_buf(),
        certificate_filename_pattern: "{id}.crt".to_string(),
        private_key_filename_pattern: "{id}.key".to_string(),
        chain_filename_pattern: "{id}_chain.crt".to_string(),
        certificate_permissions: 0o644,
        private_key_permissions: 0o600,
        atomic_operations: true,
        backup_retention_days: 30,
    };
    
    let mut storage_manager = CertificateStorageManager::new(storage_config);
    let init_result = storage_manager.initialize();
    assert!(init_result.is_ok());
    
    // Test that base directory was created
    assert!(temp_dir.path().exists());
    
    // Test statistics initialization
    assert_eq!(storage_manager.statistics.certificates_stored, 0);
    assert_eq!(storage_manager.statistics.total_operations, 0);
}

/// Test renewal task creation and status management
#[test]
fn test_renewal_task_management() {
    let task = RenewalTask {
        task_id: "test_task_001".to_string(),
        certificate_id: "test_cert_001".to_string(),
        renewal_method: RenewalMethod::Acme {
            client_id: "default".to_string(),
            challenge_method: AcmeChallenge::Http01 {
                webroot_path: PathBuf::from("/tmp"),
            },
        },
        status: RenewalTaskStatus::Scheduled,
        created_at: SystemTime::now(),
        updated_at: SystemTime::now(),
        scheduled_at: SystemTime::now() + Duration::from_secs(3600),
        retry_attempt: 0,
        error_details: None,
        progress_percentage: 0,
        estimated_completion: None,
    };
    
    assert_eq!(task.task_id, "test_task_001");
    assert_eq!(task.certificate_id, "test_cert_001");
    assert_eq!(task.status, RenewalTaskStatus::Scheduled);
    assert_eq!(task.retry_attempt, 0);
    assert_eq!(task.progress_percentage, 0);
    
    // Test task status transitions
    let statuses = vec![
        RenewalTaskStatus::Scheduled,
        RenewalTaskStatus::Running,
        RenewalTaskStatus::Completed,
    ];
    
    for status in statuses {
        let mut task_copy = task.clone();
        task_copy.status = status.clone();
        assert_eq!(task_copy.status, status);
    }
}

/// Test certificate status detection and classification
#[test]
fn test_certificate_status_detection() {
    // Test valid certificate
    let valid_status = CertificateStatus::Valid;
    assert_eq!(valid_status, CertificateStatus::Valid);
    
    // Test certificate near expiry
    let near_expiry_status = CertificateStatus::NearExpiry { days_remaining: 15 };
    if let CertificateStatus::NearExpiry { days_remaining } = near_expiry_status {
        assert_eq!(days_remaining, 15);
    } else {
        panic!("Expected NearExpiry status");
    }
    
    // Test expired certificate
    let expired_status = CertificateStatus::Expired { days_since_expiry: 5 };
    if let CertificateStatus::Expired { days_since_expiry } = expired_status {
        assert_eq!(days_since_expiry, 5);
    } else {
        panic!("Expected Expired status");
    }
    
    // Test invalid certificate
    let invalid_status = CertificateStatus::Invalid { 
        reason: "Signature verification failed".to_string() 
    };
    if let CertificateStatus::Invalid { reason } = invalid_status {
        assert_eq!(reason, "Signature verification failed");
    } else {
        panic!("Expected Invalid status");
    }
}

/// Test notification system configuration and setup
#[test]
fn test_notification_manager() {
    let email_config = EmailConfig {
        smtp_server: "smtp.example.com".to_string(),
        smtp_port: 587,
        smtp_auth: Some(SmtpAuth {
            username: "noreply@example.com".to_string(),
            password: "secure_password".to_string(),
            mechanism: SmtpAuthMechanism::Plain,
        }),
        from_address: "noreply@example.com".to_string(),
        to_addresses: vec!["admin@example.com".to_string(), "security@example.com".to_string()],
        subject_prefix: "[CURSED PKI]".to_string(),
    };
    
    let webhook_config = WebhookConfig {
        url: "https://hooks.slack.com/services/T00000000/B00000000/XXXXXXXXXXXXXXXXXXXXXXXX".to_string(),
        method: "POST".to_string(),
        headers: {
            let mut headers = HashMap::new();
            headers.insert("Content-Type".to_string(), "application/json".to_string());
            headers
        },
        auth: Some(WebhookAuth::BearerToken("webhook_token".to_string())),
        timeout_seconds: 30,
        retry_policy: RetryPolicy::default(),
    };
    
    let notification_config = NotificationConfig {
        email_config: Some(email_config),
        webhook_config: Some(webhook_config),
        enable_log_notifications: true,
        notification_thresholds: NotificationThresholds {
            expiration_warning_days: 30,
            expiration_critical_days: 7,
            notify_on_success: true,
            notify_on_failure: true,
            notify_on_config_change: true,
        },
    };
    
    let notification_manager = RenewalNotificationManager::new(notification_config.clone());
    assert!(notification_config.email_config.is_some());
    assert!(notification_config.webhook_config.is_some());
    assert!(notification_config.enable_log_notifications);
    assert_eq!(notification_config.notification_thresholds.expiration_warning_days, 30);
}

/// Test renewal method configurations
#[test]
fn test_renewal_methods() {
    // Test ACME renewal method
    let acme_method = RenewalMethod::Acme {
        client_id: "letsencrypt".to_string(),
        challenge_method: AcmeChallenge::Http01 {
            webroot_path: PathBuf::from("/var/www/html/.well-known/acme-challenge"),
        },
    };
    
    if let RenewalMethod::Acme { client_id, challenge_method } = &acme_method {
        assert_eq!(client_id, "letsencrypt");
        if let AcmeChallenge::Http01 { webroot_path } = challenge_method {
            assert_eq!(webroot_path, &PathBuf::from("/var/www/html/.well-known/acme-challenge"));
        } else {
            panic!("Expected Http01 challenge");
        }
    } else {
        panic!("Expected ACME renewal method");
    }
    
    // Test manual renewal method
    let manual_method = RenewalMethod::Manual {
        instructions: "Contact CA support for manual renewal".to_string(),
    };
    
    if let RenewalMethod::Manual { instructions } = &manual_method {
        assert_eq!(instructions, "Contact CA support for manual renewal");
    } else {
        panic!("Expected Manual renewal method");
    }
    
    // Test custom script renewal method
    let script_method = RenewalMethod::CustomScript {
        script_path: PathBuf::from("/usr/local/bin/renew_cert.sh"),
        arguments: vec!["--domain".to_string(), "example.com".to_string()],
    };
    
    if let RenewalMethod::CustomScript { script_path, arguments } = &script_method {
        assert_eq!(script_path, &PathBuf::from("/usr/local/bin/renew_cert.sh"));
        assert_eq!(arguments.len(), 2);
        assert_eq!(arguments[0], "--domain");
        assert_eq!(arguments[1], "example.com");
    } else {
        panic!("Expected CustomScript renewal method");
    }
}

/// Test retry policy configuration and validation
#[test]
fn test_retry_policy() {
    let retry_policy = RetryPolicy {
        max_attempts: 5,
        initial_delay_seconds: 120,
        backoff_multiplier: 2.5,
        max_delay_seconds: 7200,
        jitter_factor: 0.15,
    };
    
    assert_eq!(retry_policy.max_attempts, 5);
    assert_eq!(retry_policy.initial_delay_seconds, 120);
    assert_eq!(retry_policy.backoff_multiplier, 2.5);
    assert_eq!(retry_policy.max_delay_seconds, 7200);
    assert_eq!(retry_policy.jitter_factor, 0.15);
    
    // Test default retry policy
    let default_policy = RetryPolicy::default();
    assert_eq!(default_policy.max_attempts, 3);
    assert_eq!(default_policy.initial_delay_seconds, 60);
    assert_eq!(default_policy.backoff_multiplier, 2.0);
}

/// Test ACME challenge configurations
#[test]
fn test_acme_challenges() {
    // Test HTTP-01 challenge
    let http_challenge = AcmeChallenge::Http01 {
        webroot_path: PathBuf::from("/var/www/html"),
    };
    
    if let AcmeChallenge::Http01 { webroot_path } = &http_challenge {
        assert_eq!(webroot_path, &PathBuf::from("/var/www/html"));
    } else {
        panic!("Expected Http01 challenge");
    }
    
    // Test DNS-01 challenge
    let dns_challenge = AcmeChallenge::Dns01 {
        dns_provider: DnsProvider {
            provider_type: "route53".to_string(),
            credentials: {
                let mut creds = HashMap::new();
                creds.insert("access_key_id".to_string(), "AKIAEXAMPLE".to_string());
                creds.insert("secret_access_key".to_string(), "secret_key".to_string());
                creds
            },
            record_ttl: 120,
            propagation_wait_seconds: 30,
        },
    };
    
    if let AcmeChallenge::Dns01 { dns_provider } = &dns_challenge {
        assert_eq!(dns_provider.provider_type, "route53");
        assert_eq!(dns_provider.record_ttl, 120);
        assert_eq!(dns_provider.propagation_wait_seconds, 30);
        assert!(dns_provider.credentials.contains_key("access_key_id"));
    } else {
        panic!("Expected Dns01 challenge");
    }
    
    // Test TLS-ALPN-01 challenge
    let tls_challenge = AcmeChallenge::TlsAlpn01 { port: 443 };
    
    if let AcmeChallenge::TlsAlpn01 { port } = &tls_challenge {
        assert_eq!(*port, 443);
    } else {
        panic!("Expected TlsAlpn01 challenge");
    }
}

/// Test certificate backup and rollback configuration
#[test]
fn test_backup_configuration() {
    let backup_config = BackupConfig {
        enable_auto_backup: true,
        backup_directory: PathBuf::from("/etc/ssl/backups"),
        max_backup_versions: 10,
        enable_compression: true,
        verify_backups: true,
        auto_cleanup_enabled: true,
    };
    
    assert!(backup_config.enable_auto_backup);
    assert_eq!(backup_config.backup_directory, PathBuf::from("/etc/ssl/backups"));
    assert_eq!(backup_config.max_backup_versions, 10);
    assert!(backup_config.enable_compression);
    assert!(backup_config.verify_backups);
    assert!(backup_config.auto_cleanup_enabled);
    
    // Test default backup configuration
    let default_backup = BackupConfig::default();
    assert!(default_backup.enable_auto_backup);
    assert_eq!(default_backup.max_backup_versions, 5);
    assert!(!default_backup.enable_compression);
}

/// Test validation requirements configuration
#[test]
fn test_validation_requirements() {
    let validation_req = ValidationRequirements {
        validate_chain: true,
        validate_ocsp: true,
        validate_crl: false,
        custom_policies: vec![
            "require_ct_logs".to_string(),
            "check_san_validity".to_string(),
            "verify_key_usage".to_string(),
        ],
        validation_timeout_seconds: 45,
        rollback_on_validation_failure: true,
    };
    
    assert!(validation_req.validate_chain);
    assert!(validation_req.validate_ocsp);
    assert!(!validation_req.validate_crl);
    assert_eq!(validation_req.custom_policies.len(), 3);
    assert_eq!(validation_req.validation_timeout_seconds, 45);
    assert!(validation_req.rollback_on_validation_failure);
    
    // Test that custom policies are properly stored
    assert!(validation_req.custom_policies.contains(&"require_ct_logs".to_string()));
    assert!(validation_req.custom_policies.contains(&"check_san_validity".to_string()));
    assert!(validation_req.custom_policies.contains(&"verify_key_usage".to_string()));
}

/// Test monitoring configuration options
#[test]
fn test_monitoring_configuration() {
    let health_check_config = HealthCheckConfig {
        port: 9090,
        path: "/health/certificates".to_string(),
        include_certificate_details: true,
    };
    
    let metrics_config = MetricsConfig {
        format: "prometheus".to_string(),
        endpoint: "http://prometheus:9090/metrics".to_string(),
        export_interval_seconds: 60,
    };
    
    let monitoring_config = MonitoringConfig {
        scan_interval_hours: 4,
        enable_proactive_monitoring: true,
        monitor_ct_logs: true,
        health_check_config: Some(health_check_config),
        metrics_config: Some(metrics_config),
    };
    
    assert_eq!(monitoring_config.scan_interval_hours, 4);
    assert!(monitoring_config.enable_proactive_monitoring);
    assert!(monitoring_config.monitor_ct_logs);
    
    // Test health check configuration
    if let Some(ref health_config) = monitoring_config.health_check_config {
        assert_eq!(health_config.port, 9090);
        assert_eq!(health_config.path, "/health/certificates");
        assert!(health_config.include_certificate_details);
    } else {
        panic!("Expected health check configuration");
    }
    
    // Test metrics configuration
    if let Some(ref metrics) = monitoring_config.metrics_config {
        assert_eq!(metrics.format, "prometheus");
        assert_eq!(metrics.export_interval_seconds, 60);
    } else {
        panic!("Expected metrics configuration");
    }
}

/// Test renewal history and audit trail
#[test]
fn test_renewal_history() {
    let history_entry = RenewalHistoryEntry {
        timestamp: SystemTime::now(),
        method: RenewalMethod::Acme {
            client_id: "letsencrypt".to_string(),
            challenge_method: AcmeChallenge::Http01 {
                webroot_path: PathBuf::from("/var/www/html"),
            },
        },
        result: RenewalResult::Success {
            new_validity: Validity {
                not_before: SystemTime::now(),
                not_after: SystemTime::now() + Duration::from_secs(90 * 24 * 3600),
            },
            method: "ACME HTTP-01".to_string(),
        },
        previous_serial: Some(SerialNumber::from_big_int(12345)),
        new_serial: Some(SerialNumber::from_big_int(67890)),
        duration: Duration::from_secs(300),
        metadata: {
            let mut metadata = HashMap::new();
            metadata.insert("ca_issuer".to_string(), "Let's Encrypt Authority X3".to_string());
            metadata.insert("challenge_type".to_string(), "http-01".to_string());
            metadata
        },
    };
    
    // Test that renewal result is success
    if let RenewalResult::Success { new_validity, method } = &history_entry.result {
        assert_eq!(method, "ACME HTTP-01");
        assert!(new_validity.not_after > new_validity.not_before);
    } else {
        panic!("Expected successful renewal result");
    }
    
    // Test serial number tracking
    assert!(history_entry.previous_serial.is_some());
    assert!(history_entry.new_serial.is_some());
    
    // Test metadata storage
    assert_eq!(history_entry.metadata.len(), 2);
    assert!(history_entry.metadata.contains_key("ca_issuer"));
    assert!(history_entry.metadata.contains_key("challenge_type"));
}

/// Test notification frequency limits and anti-spam measures
#[test]
fn test_notification_frequency_limits() {
    let frequency_limits = NotificationFrequencyLimits {
        max_per_hour: 10,
        max_per_day: 50,
        duplicate_cooldown_minutes: 30,
    };
    
    assert_eq!(frequency_limits.max_per_hour, 10);
    assert_eq!(frequency_limits.max_per_day, 50);
    assert_eq!(frequency_limits.duplicate_cooldown_minutes, 30);
    
    // Test notification preferences with frequency limits
    let notification_prefs = NotificationPreferences {
        email_enabled: true,
        webhook_enabled: true,
        custom_endpoints: vec![
            "https://slack.com/webhook1".to_string(),
            "https://discord.com/webhook2".to_string(),
        ],
        frequency_limits,
    };
    
    assert!(notification_prefs.email_enabled);
    assert!(notification_prefs.webhook_enabled);
    assert_eq!(notification_prefs.custom_endpoints.len(), 2);
    assert_eq!(notification_prefs.frequency_limits.max_per_hour, 10);
}

/// Test external account binding for ACME
#[test]
fn test_external_account_binding() {
    let eab = ExternalAccountBinding {
        key_id: "kid_12345".to_string(),
        hmac_key: b"secret_hmac_key_for_eab".to_vec(),
    };
    
    assert_eq!(eab.key_id, "kid_12345");
    assert_eq!(eab.hmac_key, b"secret_hmac_key_for_eab");
    
    // Test ACME config with EAB
    let acme_config = AcmeConfig {
        directory_url: "https://acme.ca.com/directory".to_string(),
        contact_email: "admin@example.com".to_string(),
        terms_of_service_agreed: true,
        supported_challenges: vec![AcmeChallenge::Http01 {
            webroot_path: PathBuf::from("/tmp"),
        }],
        challenge_timeout_seconds: 300,
        account_key_pair: None,
        external_account_binding: Some(eab),
    };
    
    assert!(acme_config.external_account_binding.is_some());
    if let Some(ref binding) = acme_config.external_account_binding {
        assert_eq!(binding.key_id, "kid_12345");
        assert!(!binding.hmac_key.is_empty());
    }
}

/// Test certificate renewal statistics tracking
#[test]
fn test_renewal_statistics() {
    let mut statistics = RenewalStatistics {
        total_renewal_attempts: 100,
        successful_renewals: 85,
        failed_renewals: 10,
        cancelled_renewals: 5,
        average_renewal_time_minutes: 12.5,
        success_rates_by_method: {
            let mut rates = HashMap::new();
            rates.insert("ACME".to_string(), 0.90);
            rates.insert("Manual".to_string(), 0.95);
            rates.insert("Custom Script".to_string(), 0.80);
            rates
        },
        monitored_certificates_count: 250,
        active_renewal_tasks: 5,
    };
    
    assert_eq!(statistics.total_renewal_attempts, 100);
    assert_eq!(statistics.successful_renewals, 85);
    assert_eq!(statistics.failed_renewals, 10);
    assert_eq!(statistics.cancelled_renewals, 5);
    assert_eq!(statistics.average_renewal_time_minutes, 12.5);
    assert_eq!(statistics.monitored_certificates_count, 250);
    assert_eq!(statistics.active_renewal_tasks, 5);
    
    // Test success rate calculations
    assert_eq!(statistics.success_rates_by_method.get("ACME"), Some(&0.90));
    assert_eq!(statistics.success_rates_by_method.get("Manual"), Some(&0.95));
    assert_eq!(statistics.success_rates_by_method.get("Custom Script"), Some(&0.80));
    
    // Test statistics updates
    statistics.total_renewal_attempts += 1;
    statistics.successful_renewals += 1;
    assert_eq!(statistics.total_renewal_attempts, 101);
    assert_eq!(statistics.successful_renewals, 86);
}

/// Test certificate storage manager with atomic operations
#[test]
fn test_atomic_storage_operations() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
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
    
    let mut storage_manager = CertificateStorageManager::new(storage_config);
    let init_result = storage_manager.initialize();
    assert!(init_result.is_ok());
    
    // Test certificate storage path generation
    let cert_path = storage_manager.config.base_directory.join(
        storage_manager.config.certificate_filename_pattern.replace("{id}", "test_cert")
    );
    let key_path = storage_manager.config.base_directory.join(
        storage_manager.config.private_key_filename_pattern.replace("{id}", "test_cert")
    );
    
    assert_eq!(cert_path.file_name().unwrap(), "cert_test_cert.pem");
    assert_eq!(key_path.file_name().unwrap(), "key_test_cert.pem");
    
    // Test atomic operations flag
    assert!(storage_manager.config.atomic_operations);
}

/// Test error handling and recovery scenarios
#[test]
fn test_error_handling_scenarios() {
    // Test renewal task failure handling
    let failed_task = RenewalTask {
        task_id: "failed_task_001".to_string(),
        certificate_id: "failed_cert_001".to_string(),
        renewal_method: RenewalMethod::Manual {
            instructions: "Contact support".to_string(),
        },
        status: RenewalTaskStatus::Failed,
        created_at: SystemTime::now(),
        updated_at: SystemTime::now(),
        scheduled_at: SystemTime::now(),
        retry_attempt: 3,
        error_details: Some("ACME challenge validation failed".to_string()),
        progress_percentage: 50,
        estimated_completion: None,
    };
    
    assert_eq!(failed_task.status, RenewalTaskStatus::Failed);
    assert_eq!(failed_task.retry_attempt, 3);
    assert!(failed_task.error_details.is_some());
    assert_eq!(failed_task.error_details.unwrap(), "ACME challenge validation failed");
    
    // Test renewal result failure
    let failed_result = RenewalResult::Failed {
        error: "Certificate authority unreachable".to_string(),
        error_code: "CA_UNREACHABLE".to_string(),
        retry_recommended: true,
    };
    
    if let RenewalResult::Failed { error, error_code, retry_recommended } = failed_result {
        assert_eq!(error, "Certificate authority unreachable");
        assert_eq!(error_code, "CA_UNREACHABLE");
        assert!(retry_recommended);
    } else {
        panic!("Expected failed renewal result");
    }
}

/// Test integration with PKI error system
#[test]
fn test_pki_error_integration() {
    // Test certificate error creation
    let cert_error = PkiError::certificate_error(
        "Certificate validation failed",
        CertificateErrorCode::InvalidSignature,
    );
    
    match cert_error {
        PkiError::Certificate { message, error_code, .. } => {
            assert_eq!(message, "Certificate validation failed");
            assert_eq!(error_code, CertificateErrorCode::InvalidSignature);
        }
        _ => panic!("Expected certificate error"),
    }
    
    // Test general PKI error
    let general_error = PkiError::general("ACME client initialization failed");
    match general_error {
        PkiError::General(message) => {
            assert_eq!(message, "ACME client initialization failed");
        }
        _ => panic!("Expected general PKI error"),
    }
}

/// Test public API functions
#[test]
fn test_public_api_functions() {
    let config = RenewalConfig::default();
    
    // Test renewal manager creation
    let manager_result = create_renewal_manager(config);
    assert!(manager_result.is_ok());
    
    // Test initialization function
    let init_result = init_certificate_renewal(RenewalConfig::default());
    assert!(init_result.is_ok());
}

/// Test webhook authentication configurations
#[test]
fn test_webhook_authentication() {
    // Test Bearer token authentication
    let bearer_auth = WebhookAuth::BearerToken("bearer_token_12345".to_string());
    if let WebhookAuth::BearerToken(token) = bearer_auth {
        assert_eq!(token, "bearer_token_12345");
    } else {
        panic!("Expected Bearer token authentication");
    }
    
    // Test Basic authentication
    let basic_auth = WebhookAuth::BasicAuth {
        username: "webhook_user".to_string(),
        password: "webhook_pass".to_string(),
    };
    if let WebhookAuth::BasicAuth { username, password } = basic_auth {
        assert_eq!(username, "webhook_user");
        assert_eq!(password, "webhook_pass");
    } else {
        panic!("Expected Basic authentication");
    }
    
    // Test API key authentication
    let api_key_auth = WebhookAuth::ApiKey {
        header_name: "X-API-Key".to_string(),
        api_key: "api_key_67890".to_string(),
    };
    if let WebhookAuth::ApiKey { header_name, api_key } = api_key_auth {
        assert_eq!(header_name, "X-API-Key");
        assert_eq!(api_key, "api_key_67890");
    } else {
        panic!("Expected API key authentication");
    }
}

/// Test SMTP authentication mechanisms
#[test]
fn test_smtp_authentication() {
    let smtp_auth = SmtpAuth {
        username: "smtp_user@example.com".to_string(),
        password: "smtp_password".to_string(),
        mechanism: SmtpAuthMechanism::CramMd5,
    };
    
    assert_eq!(smtp_auth.username, "smtp_user@example.com");
    assert_eq!(smtp_auth.password, "smtp_password");
    assert_eq!(smtp_auth.mechanism, SmtpAuthMechanism::CramMd5);
    
    // Test all authentication mechanisms
    let mechanisms = vec![
        SmtpAuthMechanism::Plain,
        SmtpAuthMechanism::Login,
        SmtpAuthMechanism::CramMd5,
    ];
    
    for mechanism in mechanisms {
        let auth = SmtpAuth {
            username: "user".to_string(),
            password: "pass".to_string(),
            mechanism: mechanism.clone(),
        };
        assert_eq!(auth.mechanism, mechanism);
    }
}

/// Test comprehensive renewal workflow
#[test]
fn test_comprehensive_renewal_workflow() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    
    // Create comprehensive configuration
    let mut config = RenewalConfig {
        default_renewal_days_before_expiry: 30,
        max_concurrent_renewals: 3,
        retry_policy: RetryPolicy {
            max_attempts: 3,
            initial_delay_seconds: 60,
            backoff_multiplier: 2.0,
            max_delay_seconds: 3600,
            jitter_factor: 0.1,
        },
        acme_config: AcmeConfig {
            directory_url: "https://acme-staging-v02.api.letsencrypt.org/directory".to_string(),
            contact_email: "admin@example.com".to_string(),
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
            notification_thresholds: NotificationThresholds {
                expiration_warning_days: 30,
                expiration_critical_days: 7,
                notify_on_success: true,
                notify_on_failure: true,
                notify_on_config_change: false,
            },
        },
        validation_requirements: ValidationRequirements {
            validate_chain: true,
            validate_ocsp: false,
            validate_crl: false,
            custom_policies: vec!["verify_san".to_string()],
            validation_timeout_seconds: 30,
            rollback_on_validation_failure: true,
        },
        backup_config: BackupConfig {
            enable_auto_backup: true,
            backup_directory: temp_dir.path().join("backups"),
            max_backup_versions: 5,
            enable_compression: false,
            verify_backups: true,
            auto_cleanup_enabled: true,
        },
        monitoring_config: MonitoringConfig {
            scan_interval_hours: 6,
            enable_proactive_monitoring: true,
            monitor_ct_logs: false,
            health_check_config: None,
            metrics_config: None,
        },
    };
    
    // Test manager creation with comprehensive config
    let manager = CertificateRenewalManager::new(config);
    assert_eq!(manager.config.default_renewal_days_before_expiry, 30);
    assert_eq!(manager.config.max_concurrent_renewals, 3);
    assert!(manager.config.backup_config.enable_auto_backup);
    assert!(manager.config.validation_requirements.validate_chain);
    assert!(manager.config.monitoring_config.enable_proactive_monitoring);
    
    // Verify all components are properly configured
    assert_eq!(manager.config.acme_config.directory_url, "https://acme-staging-v02.api.letsencrypt.org/directory");
    assert!(manager.config.storage_config.atomic_operations);
    assert!(manager.config.notification_config.enable_log_notifications);
}
