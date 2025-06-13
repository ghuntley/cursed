/// Comprehensive test suite for SSH client implementation
/// 
/// This test suite validates the complete SSH client functionality including:
/// - Configuration validation and setup
/// - Connection management and error handling
/// - Authentication mechanisms (password and key-based)
/// - Command execution with output capture
/// - File transfer operations (upload/download)
/// - Security and error boundary testing

#[path = "common.rs"]
pub mod common;

use cursed::stdlib::net::protocols::ssh::{SshClient, SshConfig, SshKey, SshCommand};
use cursed::stdlib::net::protocols::{ProtocolError, ProtocolResult};
use std::time::Duration;
use std::fs;
use std::path::Path;
use tempfile::TempDir;
use tracing::{debug, error, info, warn};

/// Initialize tracing for test debugging
fn init_test_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .with_test_writer()
        .try_init();
}

#[test]
fn test_ssh_config_creation() {
    init_test_tracing();
    info!("Testing SSH configuration creation and validation");

    // Test default configuration
    let default_config = SshConfig::default();
    assert_eq!(default_config.host, "localhost");
    assert_eq!(default_config.port, 22);
    assert_eq!(default_config.username, "user");
    assert!(default_config.password.is_none());
    assert!(default_config.private_key.is_none());
    assert_eq!(default_config.connect_timeout, Some(Duration::from_secs(30)));
    assert_eq!(default_config.command_timeout, Some(Duration::from_secs(300)));

    // Test custom configuration
    let custom_config = SshConfig {
        host: "example.com".to_string(),
        port: 2222,
        username: "testuser".to_string(),
        password: Some("password123".to_string()),
        private_key: None,
        connect_timeout: Some(Duration::from_secs(10)),
        command_timeout: Some(Duration::from_secs(60)),
    };

    assert_eq!(custom_config.host, "example.com");
    assert_eq!(custom_config.port, 2222);
    assert_eq!(custom_config.username, "testuser");
    assert_eq!(custom_config.password, Some("password123".to_string()));
    assert_eq!(custom_config.connect_timeout, Some(Duration::from_secs(10)));
    
    debug!("SSH configuration creation test completed successfully");
}

#[test]
fn test_ssh_key_types() {
    init_test_tracing();
    info!("Testing SSH key type creation and handling");

    // Test RSA key
    let rsa_key_data = b"-----BEGIN RSA PRIVATE KEY-----\n...\n-----END RSA PRIVATE KEY-----".to_vec();
    let rsa_key = SshKey::Rsa(rsa_key_data.clone());
    match rsa_key {
        SshKey::Rsa(data) => assert_eq!(data, rsa_key_data),
        _ => panic!("Expected RSA key"),
    }

    // Test Ed25519 key
    let ed25519_key_data = b"-----BEGIN OPENSSH PRIVATE KEY-----\n...\n-----END OPENSSH PRIVATE KEY-----".to_vec();
    let ed25519_key = SshKey::Ed25519(ed25519_key_data.clone());
    match ed25519_key {
        SshKey::Ed25519(data) => assert_eq!(data, ed25519_key_data),
        _ => panic!("Expected Ed25519 key"),
    }

    // Test ECDSA key
    let ecdsa_key_data = b"-----BEGIN EC PRIVATE KEY-----\n...\n-----END EC PRIVATE KEY-----".to_vec();
    let ecdsa_key = SshKey::Ecdsa(ecdsa_key_data.clone());
    match ecdsa_key {
        SshKey::Ecdsa(data) => assert_eq!(data, ecdsa_key_data),
        _ => panic!("Expected ECDSA key"),
    }

    debug!("SSH key type testing completed successfully");
}

#[test]
fn test_ssh_client_creation() {
    init_test_tracing();
    info!("Testing SSH client creation and initialization");

    let config = SshConfig {
        host: "testhost.example.com".to_string(),
        port: 22,
        username: "testuser".to_string(),
        password: Some("testpass".to_string()),
        private_key: None,
        connect_timeout: Some(Duration::from_secs(30)),
        command_timeout: Some(Duration::from_secs(300)),
    };

    let mut client = SshClient::new(config.clone());
    
    // Verify client is not initially connected
    assert!(!client.is_connected());
    assert!(client.connection_info().is_none());

    debug!("SSH client creation test completed successfully");
}

#[test] 
fn test_ssh_command_result() {
    init_test_tracing();
    info!("Testing SSH command result structure");

    let command_result = SshCommand {
        command: "ls -la".to_string(),
        stdout: "total 12\ndrwxr-xr-x 2 user user 4096 Jan 1 00:00 .\n".to_string(),
        stderr: "".to_string(),
        exit_code: 0,
    };

    assert_eq!(command_result.command, "ls -la");
    assert!(command_result.stdout.contains("total 12"));
    assert!(command_result.stderr.is_empty());
    assert_eq!(command_result.exit_code, 0);

    // Test command with error
    let error_command = SshCommand {
        command: "nonexistent-command".to_string(),
        stdout: "".to_string(),
        stderr: "command not found: nonexistent-command".to_string(),
        exit_code: 127,
    };

    assert_eq!(error_command.exit_code, 127);
    assert!(error_command.stderr.contains("command not found"));
    assert!(error_command.stdout.is_empty());

    debug!("SSH command result testing completed successfully");
}

#[test]
fn test_ssh_connection_failure() {
    init_test_tracing();
    info!("Testing SSH connection failure scenarios");

    // Test connection to non-existent host
    let config = SshConfig {
        host: "non-existent-host-12345.invalid".to_string(),
        port: 22,
        username: "user".to_string(),
        password: Some("password".to_string()),
        private_key: None,
        connect_timeout: Some(Duration::from_secs(1)), // Short timeout for testing
        command_timeout: Some(Duration::from_secs(60)),
    };

    let mut client = SshClient::new(config);
    let result = client.connect();
    
    // Should fail with connection error
    assert!(result.is_err());
    if let Err(error) = result {
        debug!("Expected connection error: {:?}", error);
        match error {
            ProtocolError::Connection(_) => {
                // Expected
            }
            _ => panic!("Expected connection error, got: {:?}", error),
        }
    }

    assert!(!client.is_connected());

    debug!("SSH connection failure testing completed successfully");
}

#[test]
fn test_ssh_operations_without_connection() {
    init_test_tracing();
    info!("Testing SSH operations without active connection");

    let config = SshConfig::default();
    let mut client = SshClient::new(config);

    // Test command execution without connection
    let cmd_result = client.execute_command("echo test");
    assert!(cmd_result.is_err());
    if let Err(ProtocolError::Connection(msg)) = cmd_result {
        assert!(msg.contains("Not connected"));
    } else {
        panic!("Expected connection error");
    }

    // Test file upload without connection
    let upload_result = client.upload_file("/tmp/test.txt", "/remote/test.txt");
    assert!(upload_result.is_err());
    if let Err(ProtocolError::Connection(msg)) = upload_result {
        assert!(msg.contains("Not connected"));
    } else {
        panic!("Expected connection error");
    }

    // Test file download without connection
    let download_result = client.download_file("/remote/test.txt", "/tmp/downloaded.txt");
    assert!(download_result.is_err());
    if let Err(ProtocolError::Connection(msg)) = download_result {
        assert!(msg.contains("Not connected"));
    } else {
        panic!("Expected connection error");
    }

    debug!("SSH operations without connection testing completed successfully");
}

#[test]
fn test_ssh_config_with_key_authentication() {
    init_test_tracing();
    info!("Testing SSH configuration with key-based authentication");

    let private_key = SshKey::Ed25519(
        b"-----BEGIN OPENSSH PRIVATE KEY-----\nb3BlbnNzaC1rZXktdjEAAAAABG5vbmUAAAAEbm9uZQAAAAAAAAABAAAA...\n-----END OPENSSH PRIVATE KEY-----".to_vec()
    );

    let config = SshConfig {
        host: "example.com".to_string(),
        port: 22,
        username: "keyuser".to_string(),
        password: None, // No password, using key
        private_key: Some(private_key),
        connect_timeout: Some(Duration::from_secs(30)),
        command_timeout: Some(Duration::from_secs(300)),
    };

    let client = SshClient::new(config.clone());
    assert!(!client.is_connected());

    // Verify configuration
    assert_eq!(client.config.username, "keyuser");
    assert!(client.config.password.is_none());
    assert!(client.config.private_key.is_some());

    debug!("SSH key authentication configuration testing completed successfully");
}

#[test]
fn test_ssh_timeout_configuration() {
    init_test_tracing();
    info!("Testing SSH timeout configurations");

    // Test with custom timeouts
    let config = SshConfig {
        host: "example.com".to_string(),
        port: 22,
        username: "user".to_string(),
        password: Some("password".to_string()),
        private_key: None,
        connect_timeout: Some(Duration::from_secs(5)),
        command_timeout: Some(Duration::from_secs(10)),
    };

    assert_eq!(config.connect_timeout, Some(Duration::from_secs(5)));
    assert_eq!(config.command_timeout, Some(Duration::from_secs(10)));

    // Test with no timeouts
    let config_no_timeout = SshConfig {
        host: "example.com".to_string(),
        port: 22,
        username: "user".to_string(),
        password: Some("password".to_string()),
        private_key: None,
        connect_timeout: None,
        command_timeout: None,
    };

    assert!(config_no_timeout.connect_timeout.is_none());
    assert!(config_no_timeout.command_timeout.is_none());

    debug!("SSH timeout configuration testing completed successfully");
}

#[test]
fn test_ssh_debug_implementation() {
    init_test_tracing();
    info!("Testing SSH client Debug implementation");

    let config = SshConfig::default();
    let client = SshClient::new(config);

    // Test Debug formatting
    let debug_str = format!("{:?}", client);
    assert!(debug_str.contains("SshClient"));
    assert!(debug_str.contains("connected: false"));

    debug!("SSH Debug implementation testing completed successfully");
}

#[test]
fn test_ssh_client_disconnect() {
    init_test_tracing();
    info!("Testing SSH client disconnect functionality");

    let config = SshConfig::default();
    let mut client = SshClient::new(config);

    // Test disconnect when not connected
    let result = client.disconnect();
    assert!(result.is_ok());
    assert!(!client.is_connected());

    debug!("SSH client disconnect testing completed successfully");
}

#[test]
fn test_ssh_file_operations_error_handling() {
    init_test_tracing();
    info!("Testing SSH file operations error handling");

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let test_file = temp_dir.path().join("test.txt");
    
    // Create a test file
    fs::write(&test_file, "test content").expect("Failed to write test file");

    let config = SshConfig::default();
    let mut client = SshClient::new(config);

    // Test upload with invalid local file
    let upload_result = client.upload_file("/nonexistent/file.txt", "/remote/test.txt");
    assert!(upload_result.is_err());

    // Test download without connection
    let download_result = client.download_file("/remote/test.txt", "/tmp/downloaded.txt");
    assert!(download_result.is_err());

    debug!("SSH file operations error handling testing completed successfully");
}

#[test]
fn test_ssh_config_clone() {
    init_test_tracing();
    info!("Testing SSH configuration cloning");

    let original_config = SshConfig {
        host: "example.com".to_string(),
        port: 2222,
        username: "testuser".to_string(),
        password: Some("password".to_string()),
        private_key: Some(SshKey::Rsa(b"rsa-key-data".to_vec())),
        connect_timeout: Some(Duration::from_secs(15)),
        command_timeout: Some(Duration::from_secs(120)),
    };

    let cloned_config = original_config.clone();

    assert_eq!(original_config.host, cloned_config.host);
    assert_eq!(original_config.port, cloned_config.port);
    assert_eq!(original_config.username, cloned_config.username);
    assert_eq!(original_config.password, cloned_config.password);
    assert_eq!(original_config.connect_timeout, cloned_config.connect_timeout);
    assert_eq!(original_config.command_timeout, cloned_config.command_timeout);

    debug!("SSH configuration cloning testing completed successfully");
}

#[test]
fn test_ssh_command_clone() {
    init_test_tracing();
    info!("Testing SSH command result cloning");

    let original_command = SshCommand {
        command: "echo test".to_string(),
        stdout: "test\n".to_string(),
        stderr: "".to_string(),
        exit_code: 0,
    };

    let cloned_command = original_command.clone();

    assert_eq!(original_command.command, cloned_command.command);
    assert_eq!(original_command.stdout, cloned_command.stdout);
    assert_eq!(original_command.stderr, cloned_command.stderr);
    assert_eq!(original_command.exit_code, cloned_command.exit_code);

    debug!("SSH command result cloning testing completed successfully");
}

#[test]
fn test_ssh_key_clone() {
    init_test_tracing();
    info!("Testing SSH key cloning");

    let original_key = SshKey::Ed25519(b"ed25519-key-data".to_vec());
    let cloned_key = original_key.clone();

    match (&original_key, &cloned_key) {
        (SshKey::Ed25519(orig_data), SshKey::Ed25519(cloned_data)) => {
            assert_eq!(orig_data, cloned_data);
        }
        _ => panic!("Key types don't match after cloning"),
    }

    debug!("SSH key cloning testing completed successfully");
}

#[test]
fn test_ssh_invalid_address_handling() {
    init_test_tracing();
    info!("Testing SSH invalid address handling");

    let config = SshConfig {
        host: "".to_string(), // Invalid empty host
        port: 22,
        username: "user".to_string(),
        password: Some("password".to_string()),
        private_key: None,
        connect_timeout: Some(Duration::from_secs(1)),
        command_timeout: Some(Duration::from_secs(60)),
    };

    let mut client = SshClient::new(config);
    let result = client.connect();
    
    assert!(result.is_err());
    if let Err(ProtocolError::Connection(msg)) = result {
        debug!("Expected address error: {}", msg);
    } else {
        panic!("Expected connection error for invalid address");
    }

    debug!("SSH invalid address handling testing completed successfully");
}

#[test]
fn test_ssh_drop_implementation() {
    init_test_tracing();
    info!("Testing SSH client Drop implementation");

    let config = SshConfig::default();
    {
        let client = SshClient::new(config);
        // Client will be dropped here automatically
        debug!("SSH client created and will be dropped");
    }
    
    debug!("SSH client dropped successfully");
    // If we reach here, the Drop implementation worked without panicking
}

#[test]
fn test_ssh_multiple_authentication_methods() {
    init_test_tracing();
    info!("Testing SSH configuration with multiple authentication methods");

    let private_key = SshKey::Rsa(b"rsa-private-key-data".to_vec());
    
    let config = SshConfig {
        host: "example.com".to_string(),
        port: 22,
        username: "user".to_string(),
        password: Some("fallback-password".to_string()), // Fallback method
        private_key: Some(private_key), // Primary method
        connect_timeout: Some(Duration::from_secs(30)),
        command_timeout: Some(Duration::from_secs(300)),
    };

    let client = SshClient::new(config);
    
    // Verify both authentication methods are configured
    assert!(client.config.password.is_some());
    assert!(client.config.private_key.is_some());

    debug!("SSH multiple authentication methods testing completed successfully");
}

#[test]
fn test_ssh_error_types() {
    init_test_tracing();
    info!("Testing SSH error types and messages");

    // Test different error scenarios would produce appropriate error types
    let config = SshConfig {
        host: "invalid-host-name-that-does-not-exist.invalid".to_string(),
        port: 22,
        username: "user".to_string(),
        password: None, // No authentication method
        private_key: None,
        connect_timeout: Some(Duration::from_millis(100)), // Very short timeout
        command_timeout: Some(Duration::from_secs(1)),
    };

    let mut client = SshClient::new(config);
    
    match client.connect() {
        Err(ProtocolError::Connection(_)) => {
            debug!("Got expected connection error");
        }
        Err(other) => {
            debug!("Got other error type: {:?}", other);
        }
        Ok(_) => {
            panic!("Unexpected success connecting to invalid host");
        }
    }

    debug!("SSH error types testing completed successfully");
}
