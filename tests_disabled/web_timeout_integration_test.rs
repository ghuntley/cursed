/// Integration tests for the CURSED web framework timeout system
/// 
/// Tests comprehensive timeout mechanisms including request, connection,
/// session, and database timeouts with real async operations.

use cursed::stdlib::web_vibez::{
    TimeoutMiddleware, TimeoutConfig, TimeoutError, TimeoutResult,
    TimeoutMemorySessionStore, TimeoutFileSessionStore, TimeoutSessionManager,
    config::{ServerConfig, SessionConfig, SessionStoreType}
};
use std::time::{Duration, Instant};
use std::sync::Arc;
use tempfile::tempdir;
use tokio::time::timeout;
use uuid::Uuid;

#[tokio::test]
async fn test_timeout_middleware_creation() {
    let server_config = ServerConfig::default();
    let session_config = SessionConfig::default();
    
    let middleware = TimeoutMiddleware::new(server_config, session_config);
    
    assert_eq!(middleware.name(), "Timeout");
    assert_eq!(middleware.priority(), 25);
    
    let stats = middleware.get_timeout_statistics();
    assert_eq!(stats.active_requests, 0);
    assert_eq!(stats.active_connections, 0);
    assert_eq!(stats.active_sessions, 0);
    assert_eq!(stats.active_database_operations, 0);
    assert!(stats.request_timeout_enabled);
    assert!(stats.connection_timeout_enabled);
    assert!(stats.session_timeout_enabled);
    assert!(stats.database_timeout_enabled);
}

#[tokio::test]
async fn test_timeout_config_customization() {
    let server_config = ServerConfig::default();
    let session_config = SessionConfig::default();
    
    let timeout_config = TimeoutConfig {
        enable_request_timeout: false,
        enable_connection_timeout: true,
        enable_session_timeout: true,
        enable_database_timeout: false,
        graceful_shutdown_timeout: Duration::from_secs(5),
        cleanup_interval: Duration::from_secs(30),
        log_timeout_events: false,
    };
    
    let middleware = TimeoutMiddleware::new(server_config, session_config)
        .with_config(timeout_config);
    
    let stats = middleware.get_timeout_statistics();
    assert!(!stats.request_timeout_enabled);
    assert!(stats.connection_timeout_enabled);
    assert!(stats.session_timeout_enabled);
    assert!(!stats.database_timeout_enabled);
}

#[tokio::test]
async fn test_request_timeout_tracking() {
    let server_config = ServerConfig::default();
    let session_config = SessionConfig::default();
    let middleware = TimeoutMiddleware::new(server_config, session_config);
    
    // Simulate request context
    let request_id = Uuid::new_v4().to_string();
    let context = cursed::stdlib::web_vibez::context::RequestContext::new(
        "GET".to_string(),
        "/test".to_string()
    );
    
    // Start request timeout tracking
    middleware.start_request_timeout(&context);
    
    let stats = middleware.get_timeout_statistics();
    assert_eq!(stats.active_requests, 1);
    
    // Stop request timeout tracking
    middleware.stop_request_timeout(&context.request_id);
    
    let stats = middleware.get_timeout_statistics();
    assert_eq!(stats.active_requests, 0);
}

#[tokio::test]
async fn test_connection_timeout_tracking() {
    let server_config = ServerConfig::default();
    let session_config = SessionConfig::default();
    let middleware = TimeoutMiddleware::new(server_config, session_config);
    
    let connection_id = "conn_123".to_string();
    let client_ip = Some("192.168.1.1".to_string());
    
    // Start connection tracking
    middleware.start_connection_timeout(connection_id.clone(), client_ip);
    
    let stats = middleware.get_timeout_statistics();
    assert_eq!(stats.active_connections, 1);
    
    // Update activity
    middleware.update_connection_activity(&connection_id);
    
    // Stop tracking
    middleware.stop_connection_timeout(&connection_id);
    
    let stats = middleware.get_timeout_statistics();
    assert_eq!(stats.active_connections, 0);
}

#[tokio::test]
async fn test_session_timeout_tracking() {
    let server_config = ServerConfig::default();
    let session_config = SessionConfig::default();
    let middleware = TimeoutMiddleware::new(server_config, session_config);
    
    let session_id = "test_session_123".to_string();
    
    // Start session tracking
    middleware.start_session_timeout(session_id.clone());
    
    let stats = middleware.get_timeout_statistics();
    assert_eq!(stats.active_sessions, 1);
    
    // Should not be timed out immediately
    assert!(!middleware.is_session_timed_out(&session_id));
    
    // Update activity
    middleware.update_session_activity(&session_id);
    assert!(!middleware.is_session_timed_out(&session_id));
}

#[tokio::test]
async fn test_database_timeout_tracking() {
    let server_config = ServerConfig::default();
    let session_config = SessionConfig::default();
    let middleware = TimeoutMiddleware::new(server_config, session_config);
    
    let operation_id = "db_op_123".to_string();
    let operation_type = "SELECT".to_string();
    
    // Start database operation tracking
    middleware.start_database_timeout(operation_id.clone(), operation_type);
    
    let stats = middleware.get_timeout_statistics();
    assert_eq!(stats.active_database_operations, 1);
    
    // Stop tracking
    middleware.stop_database_timeout(&operation_id);
    
    let stats = middleware.get_timeout_statistics();
    assert_eq!(stats.active_database_operations, 0);
}

#[tokio::test]
async fn test_database_timeout_success() {
    let server_config = ServerConfig::default();
    let session_config = SessionConfig::default();
    let middleware = TimeoutMiddleware::new(server_config, session_config);
    
    let operation_id = "test_op_success".to_string();
    let operation_type = "SELECT".to_string();
    
    // Test successful operation within timeout
    let result = middleware.with_database_timeout(
        operation_id,
        operation_type,
        async {
            tokio::time::sleep(Duration::from_millis(100)).await;
            "success"
        }
    ).await;
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "success");
}

#[tokio::test]
async fn test_database_timeout_failure() {
    let mut server_config = ServerConfig::default();
    let mut session_config = SessionConfig::default();
    
    // Set very short timeout for testing
    session_config.database_timeout = Duration::from_millis(100);
    
    let middleware = TimeoutMiddleware::new(server_config, session_config);
    
    let operation_id = "test_op_timeout".to_string();
    let operation_type = "SLOW_SELECT".to_string();
    
    // Test operation that should timeout
    let result = middleware.with_database_timeout(
        operation_id,
        operation_type.clone(),
        async {
            tokio::time::sleep(Duration::from_millis(500)).await; // Longer than timeout
            "should_not_reach"
        }
    ).await;
    
    assert!(result.is_err());
    match result.unwrap_err() {
        TimeoutError::DatabaseTimeout { operation, .. } => {
            assert_eq!(operation, operation_type);
        }
        _ => panic!("Expected DatabaseTimeout error"),
    }
}

#[tokio::test]
async fn test_request_timeout_wrapper() {
    let server_config = ServerConfig::default();
    let session_config = SessionConfig::default();
    let middleware = TimeoutMiddleware::new(server_config, session_config);
    
    let request_id = "test_request_123".to_string();
    
    // Test successful request within timeout
    let result = middleware.with_request_timeout(
        request_id,
        async {
            tokio::time::sleep(Duration::from_millis(100)).await;
            "request_success"
        }
    ).await;
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "request_success");
}

#[tokio::test]
async fn test_cleanup_expired_timeouts() {
    let server_config = ServerConfig::default();
    let session_config = SessionConfig::default();
    let middleware = TimeoutMiddleware::new(server_config, session_config);
    
    // Add some timeout tracking
    let context = cursed::stdlib::web_vibez::context::RequestContext::new(
        "GET".to_string(),
        "/test".to_string()
    );
    middleware.start_request_timeout(&context);
    
    let session_id = "test_session".to_string();
    middleware.start_session_timeout(session_id);
    
    let connection_id = "test_connection".to_string();
    middleware.start_connection_timeout(connection_id, Some("127.0.0.1".to_string()));
    
    // Initial state
    let stats = middleware.get_timeout_statistics();
    assert_eq!(stats.active_requests, 1);
    assert_eq!(stats.active_sessions, 1);
    assert_eq!(stats.active_connections, 1);
    
    // Cleanup (should not remove anything yet since timeouts are long)
    middleware.cleanup_expired_timeouts();
    
    let stats = middleware.get_timeout_statistics();
    assert_eq!(stats.active_requests, 1);
    assert_eq!(stats.active_sessions, 1);
    assert_eq!(stats.active_connections, 1);
}

#[tokio::test]
async fn test_timeout_memory_session_store() {
    let config = SessionConfig::default();
    let store = cursed::stdlib::web_vibez::session_timeout::TimeoutMemorySessionStore::new(config.clone());
    let timeout_middleware = TimeoutMiddleware::new(
        ServerConfig::default(),
        config
    );

    let session_id = "test_memory_session";
    
    // Test session doesn't exist
    let result = store.exists_with_timeout(session_id, &timeout_middleware).await;
    assert!(result.is_ok());
    assert!(!result.unwrap());

    // Create and save session
    let session = cursed::stdlib::web_vibez::session::Session {
        id: session_id.to_string(),
        data: std::collections::HashMap::new(),
        created_at: 0,
        last_accessed: 0,
        expires_at: None,
        is_new: true,
        is_dirty: false,
    };

    let result = store.save_with_timeout(&session, &timeout_middleware).await;
    assert!(result.is_ok());

    // Test session exists
    let result = store.exists_with_timeout(session_id, &timeout_middleware).await;
    assert!(result.is_ok());
    assert!(result.unwrap());

    // Load session
    let result = store.load_with_timeout(session_id, &timeout_middleware).await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());

    // Delete session
    let result = store.delete_with_timeout(session_id, &timeout_middleware).await;
    assert!(result.is_ok());

    // Test session no longer exists
    let result = store.exists_with_timeout(session_id, &timeout_middleware).await;
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[tokio::test]
async fn test_timeout_file_session_store() {
    let temp_dir = tempdir().unwrap();
    let config = SessionConfig::default();
    let store = cursed::stdlib::web_vibez::session_timeout::TimeoutFileSessionStore::new(
        temp_dir.path().to_path_buf(),
        config.clone()
    ).unwrap();
    let timeout_middleware = TimeoutMiddleware::new(
        ServerConfig::default(),
        config
    );

    let session_id = "test_file_session";
    
    // Test session doesn't exist
    let result = store.exists_with_timeout(session_id, &timeout_middleware).await;
    assert!(result.is_ok());
    assert!(!result.unwrap());

    // Create and save session
    let session = cursed::stdlib::web_vibez::session::Session {
        id: session_id.to_string(),
        data: std::collections::HashMap::new(),
        created_at: 0,
        last_accessed: 0,
        expires_at: None,
        is_new: true,
        is_dirty: false,
    };

    let result = store.save_with_timeout(&session, &timeout_middleware).await;
    assert!(result.is_ok());

    // Test session exists
    let result = store.exists_with_timeout(session_id, &timeout_middleware).await;
    assert!(result.is_ok());
    assert!(result.unwrap());

    // Load session
    let result = store.load_with_timeout(session_id, &timeout_middleware).await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());

    // Delete session
    let result = store.delete_with_timeout(session_id, &timeout_middleware).await;
    assert!(result.is_ok());

    // Test session no longer exists
    let result = store.exists_with_timeout(session_id, &timeout_middleware).await;
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[tokio::test]
async fn test_timeout_session_manager() {
    let config = SessionConfig::default();
    let manager = cursed::stdlib::web_vibez::session_timeout::TimeoutSessionManager::new(config.clone()).unwrap();
    let timeout_middleware = TimeoutMiddleware::new(
        ServerConfig::default(),
        config
    );

    // Create new session
    let result = manager.create_session_with_timeout(&timeout_middleware).await;
    assert!(result.is_ok());
    let session = result.unwrap();

    // Load session
    let result = manager.load_session_with_timeout(&session.id, &timeout_middleware).await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_some());

    // Save session
    let result = manager.save_session_with_timeout(&session, &timeout_middleware).await;
    assert!(result.is_ok());

    // Check if session exists
    let result = manager.session_exists_with_timeout(&session.id, &timeout_middleware).await;
    assert!(result.is_ok());
    assert!(result.unwrap());

    // Delete session
    let result = manager.delete_session_with_timeout(&session.id, &timeout_middleware).await;
    assert!(result.is_ok());

    // Check if session no longer exists
    let result = manager.session_exists_with_timeout(&session.id, &timeout_middleware).await;
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[tokio::test]
async fn test_concurrent_timeout_operations() {
    let server_config = ServerConfig::default();
    let session_config = SessionConfig::default();
    let middleware = Arc::new(TimeoutMiddleware::new(server_config, session_config));

    // Spawn multiple concurrent operations
    let handles: Vec<_> = (0..10).map(|i| {
        let middleware = middleware.clone();
        tokio::spawn(async move {
            let operation_id = format!("concurrent_op_{}", i);
            let operation_type = "CONCURRENT_TEST".to_string();
            
            middleware.with_database_timeout(
                operation_id,
                operation_type,
                async move {
                    tokio::time::sleep(Duration::from_millis(50)).await;
                    format!("result_{}", i)
                }
            ).await
        })
    }).collect();

    // Wait for all operations to complete
    let results = futures::future::join_all(handles).await;
    
    // All operations should succeed
    for (i, result) in results.into_iter().enumerate() {
        let inner_result = result.unwrap();
        assert!(inner_result.is_ok());
        assert_eq!(inner_result.unwrap(), format!("result_{}", i));
    }

    // Check that tracking was cleaned up
    let stats = middleware.get_timeout_statistics();
    assert_eq!(stats.active_database_operations, 0);
}

#[test]
fn test_timeout_error_display() {
    let request_timeout = TimeoutError::RequestTimeout {
        elapsed: Duration::from_millis(5000),
        timeout: Duration::from_millis(3000),
    };
    assert!(request_timeout.to_string().contains("Request timeout"));
    assert!(request_timeout.to_string().contains("5000ms elapsed"));
    assert!(request_timeout.to_string().contains("3000ms timeout"));

    let db_timeout = TimeoutError::DatabaseTimeout {
        elapsed: Duration::from_millis(2000),
        timeout: Duration::from_millis(1000),
        operation: "SELECT".to_string(),
    };
    assert!(db_timeout.to_string().contains("Database operation 'SELECT' timeout"));
    assert!(db_timeout.to_string().contains("2000ms elapsed"));
}

#[tokio::test]
async fn test_timeout_middleware_integration() {
    let server_config = ServerConfig::default();
    let session_config = SessionConfig::default();
    let middleware = TimeoutMiddleware::new(server_config, session_config);

    // Test middleware trait methods
    assert_eq!(middleware.name(), "Timeout");
    assert_eq!(middleware.priority(), 25);

    // Create mock request context
    let mut context = cursed::stdlib::web_vibez::context::RequestContext::new(
        "GET".to_string(),
        "/test".to_string()
    );
    let mut response = cursed::stdlib::web_vibez::context::ResponseContext::new();

    // Test before_request
    let result = middleware.before_request(&mut context, &mut response);
    assert!(result.is_ok());

    // Test after_response
    let result = middleware.after_response(&context, &mut response);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_performance_and_memory() {
    let server_config = ServerConfig::default();
    let session_config = SessionConfig::default();
    let middleware = TimeoutMiddleware::new(server_config, session_config);

    let start_time = Instant::now();
    
    // Create many timeout tracking entries
    for i in 0..1000 {
        let session_id = format!("perf_session_{}", i);
        middleware.start_session_timeout(session_id);
        
        let connection_id = format!("perf_conn_{}", i);
        middleware.start_connection_timeout(connection_id, Some("127.0.0.1".to_string()));
    }

    let creation_time = start_time.elapsed();
    println!("Created 1000 timeout entries in: {:?}", creation_time);

    let stats = middleware.get_timeout_statistics();
    assert_eq!(stats.active_sessions, 1000);
    assert_eq!(stats.active_connections, 1000);

    // Test cleanup performance
    let cleanup_start = Instant::now();
    middleware.cleanup_expired_timeouts();
    let cleanup_time = cleanup_start.elapsed();
    println!("Cleanup completed in: {:?}", cleanup_time);

    // Should be fast (< 10ms for 1000 entries)
    assert!(creation_time < Duration::from_millis(100));
    assert!(cleanup_time < Duration::from_millis(50));
}
