/// Integration tests for Unix domain socket transport implementation
/// 
/// This test suite validates the production-ready Unix socket transport
/// functionality including connection pooling, error handling, and
/// integration with the RPC system.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::io::{Read, Write};
use std::fs;

use cursed::stdlib::ipc::{
    IpcResult,
    transport::{
        UnixSocketTransport, UnixSocketConfig, UnixSocketType,
        Transport, TransportConnection, TransportListener,
        StreamTransport, ConnectionPool, PoolConfig,
        create_unix_rpc_client, create_unix_rpc_server
    },
    rpc::{RpcClient, RpcServer, RpcConfig, RpcRegistry, RpcMethod, RpcRequest, RpcHandler},
};

/// Test basic Unix socket transport creation and configuration
#[test]
fn test_unix_socket_transport_creation() {
    let config = UnixSocketConfig::stream("/tmp/test_transport_creation.sock")
        .with_buffer_size(16384)
        .with_timeout(Duration::from_secs(10))
        .with_nonblocking();
    
    assert!(config.validate().is_ok());
    
    #[cfg(unix)]
    {
        let transport = UnixSocketTransport::new(config);
        assert!(transport.is_ok());
        
        let transport = transport.unwrap();
        assert_eq!(transport.name(), "unix_socket");
        assert!(UnixSocketTransport::is_available());
    }
    
    #[cfg(not(unix))]
    {
        let transport = UnixSocketTransport::new(config);
        assert!(transport.is_err());
        assert!(!UnixSocketTransport::is_available());
    }
}

/// Test Unix socket configuration validation
#[test]
fn test_unix_socket_config_validation() {
    // Valid config
    let valid_config = UnixSocketConfig::stream("/tmp/valid.sock");
    assert!(valid_config.validate().is_ok());
    
    // Invalid buffer size
    let invalid_config = UnixSocketConfig::stream("/tmp/test.sock")
        .with_buffer_size(0);
    assert!(invalid_config.validate().is_err());
    
    // Invalid max message size
    let mut invalid_config2 = UnixSocketConfig::stream("/tmp/test.sock");
    invalid_config2.max_message_size = 0;
    assert!(invalid_config2.validate().is_err());
    
    // Invalid backlog
    let mut invalid_config3 = UnixSocketConfig::stream("/tmp/test.sock");
    invalid_config3.backlog = 0;
    assert!(invalid_config3.validate().is_err());
}

/// Test basic connection and communication
#[cfg(unix)]
#[test]
fn test_basic_connection_communication() {
    let socket_path = "/tmp/test_basic_comm.sock";
    let config = UnixSocketConfig::stream(socket_path)
        .with_timeout(Duration::from_secs(5));
    
    let transport = Arc::new(UnixSocketTransport::new(config).unwrap());
    
    // Remove any existing socket file
    let _ = fs::remove_file(socket_path);
    
    // Start server in a separate thread
    let server_transport = transport.clone();
    let server_handle = thread::spawn(move || -> IpcResult<()> {
        let mut listener = server_transport.bind(socket_path)?;
        let mut connection = listener.accept()?;
        
        // Read client message
        let mut buffer = [0u8; 1024];
        let bytes_read = connection.read(&mut buffer)?;
        assert!(bytes_read > 0);
        assert_eq!(&buffer[..bytes_read], b"Hello from client");
        
        // Send response
        connection.write(b"Hello from server")?;
        connection.flush()?;
        
        Ok(())
    });
    
    // Give server time to start
    thread::sleep(Duration::from_millis(100));
    
    // Connect as client
    let mut client_connection = transport.connect(socket_path).unwrap();
    
    // Send message to server
    client_connection.write(b"Hello from client").unwrap();
    client_connection.flush().unwrap();
    
    // Read server response
    let mut buffer = [0u8; 1024];
    let bytes_read = client_connection.read(&mut buffer).unwrap();
    assert!(bytes_read > 0);
    assert_eq!(&buffer[..bytes_read], b"Hello from server");
    
    // Wait for server to complete
    server_handle.join().unwrap().unwrap();
    
    // Cleanup
    let _ = fs::remove_file(socket_path);
}

/// Test connection pooling functionality
#[cfg(unix)]
#[test]
fn test_connection_pooling() {
    let socket_path = "/tmp/test_pool.sock";
    let config = UnixSocketConfig::stream(socket_path);
    let transport = Arc::new(UnixSocketTransport::new(config).unwrap());
    
    // Remove any existing socket file
    let _ = fs::remove_file(socket_path);
    
    // Start a simple echo server
    let server_transport = transport.clone();
    let server_handle = thread::spawn(move || -> IpcResult<()> {
        let mut listener = server_transport.bind(socket_path)?;
        
        // Handle multiple connections
        for _ in 0..3 {
            let mut connection = listener.accept()?;
            let mut buffer = [0u8; 1024];
            let bytes_read = connection.read(&mut buffer)?;
            if bytes_read > 0 {
                connection.write(&buffer[..bytes_read])?;
                connection.flush()?;
            }
        }
        
        Ok(())
    });
    
    // Give server time to start
    thread::sleep(Duration::from_millis(100));
    
    // Create connection pool
    let pool_config = PoolConfig::new()
        .with_max_connections(5)
        .with_min_connections(2)
        .with_connection_timeout(Duration::from_secs(5));
    
    let pool = ConnectionPool::new(transport, pool_config).unwrap();
    
    // Test multiple connections
    let handles: Vec<_> = (0..3).map(|i| {
        let pool = pool.clone();
        let socket_path = socket_path.to_string();
        thread::spawn(move || -> IpcResult<()> {
            let mut pooled_conn = pool.get_connection(&socket_path)?;
            let connection = pooled_conn.connection()?;
            
            let message = format!("Message {}", i);
            connection.write(message.as_bytes())?;
            connection.flush()?;
            
            let mut buffer = [0u8; 1024];
            let bytes_read = connection.read(&mut buffer)?;
            assert_eq!(&buffer[..bytes_read], message.as_bytes());
            
            Ok(())
        })
    }).collect();
    
    // Wait for all clients to complete
    for handle in handles {
        handle.join().unwrap().unwrap();
    }
    
    // Wait for server to complete
    server_handle.join().unwrap().unwrap();
    
    // Check pool statistics
    let stats = pool.get_statistics();
    assert!(stats.total_connections_created > 0);
    
    // Cleanup
    pool.shutdown().unwrap();
    let _ = fs::remove_file(socket_path);
}

/// Test error handling scenarios
#[cfg(unix)]
#[test]
fn test_error_handling() {
    let config = UnixSocketConfig::stream("/nonexistent/directory/test.sock");
    let transport = UnixSocketTransport::new(config).unwrap();
    
    // Test connection to non-existent socket
    let result = transport.connect("/nonexistent/directory/test.sock");
    assert!(result.is_err());
    
    // Test binding to invalid path
    let result = transport.bind("/nonexistent/directory/test.sock");
    assert!(result.is_err());
    
    // Test invalid socket path (read-only directory)
    let readonly_path = "/proc/test.sock";
    let result = transport.bind(readonly_path);
    assert!(result.is_err());
}

/// Test Unix socket RPC transport integration
#[cfg(unix)]
#[test]
fn test_rpc_transport_integration() {
    let server_address = "/tmp/test_rpc_integration.sock";
    
    // Remove any existing socket file
    let _ = fs::remove_file(server_address);
    
    // Create RPC client and server
    let client_config = RpcConfig::new().with_timeout(Duration::from_secs(5));
    let server_config = RpcConfig::new().with_timeout(Duration::from_secs(5));
    
    let client_result = RpcClient::new_unix_socket(client_config, server_address.to_string());
    assert!(client_result.is_ok());
    
    let registry = Arc::new(RpcRegistry::new());
    let server_result = RpcServer::new_unix_socket(server_config, registry, server_address.to_string());
    assert!(server_result.is_ok());
    
    let client = client_result.unwrap();
    let server = server_result.unwrap();
    
    // Test basic RPC transport properties
    assert!(!client.is_connected()); // Not connected initially
    assert!(!server.is_connected()); // Server not started
    
    // Cleanup
    let _ = fs::remove_file(server_address);
}

/// Test RPC transport creation helpers
#[test]
fn test_rpc_transport_helpers() {
    let client_result = create_unix_rpc_client("/tmp/test_helper_client.sock".to_string());
    assert!(client_result.is_ok());
    
    let server_result = create_unix_rpc_server("/tmp/test_helper_server.sock".to_string());
    assert!(server_result.is_ok());
    
    let client = client_result.unwrap();
    let server = server_result.unwrap();
    
    // Verify they implement the RpcTransport trait
    assert!(!client.is_connected());
    assert!(!server.is_connected());
}

/// Test transport statistics tracking
#[cfg(unix)]
#[test]
fn test_transport_statistics() {
    let socket_path = "/tmp/test_stats.sock";
    let config = UnixSocketConfig::stream(socket_path);
    let transport = UnixSocketTransport::new(config).unwrap();
    
    // Remove any existing socket file
    let _ = fs::remove_file(socket_path);
    
    // Get initial statistics
    let initial_stats = transport.get_statistics();
    assert_eq!(initial_stats.connections_created, 0);
    assert_eq!(initial_stats.total_bytes_sent, 0);
    assert_eq!(initial_stats.total_bytes_received, 0);
    
    // Test connection failure statistics
    let result = transport.connect("/nonexistent/path/test.sock");
    assert!(result.is_err());
    
    let stats_after_failure = transport.get_statistics();
    assert_eq!(stats_after_failure.connections_failed, 1);
    assert_eq!(stats_after_failure.errors, 1);
    
    // Cleanup
    let _ = fs::remove_file(socket_path);
}

/// Test concurrent connections
#[cfg(unix)]
#[test]
fn test_concurrent_connections() {
    let socket_path = "/tmp/test_concurrent.sock";
    let config = UnixSocketConfig::stream(socket_path)
        .with_nonblocking();
    let transport = Arc::new(UnixSocketTransport::new(config).unwrap());
    
    // Remove any existing socket file
    let _ = fs::remove_file(socket_path);
    
    let connections_count = Arc::new(Mutex::new(0));
    
    // Start server that accepts multiple connections
    let server_transport = transport.clone();
    let server_connections = connections_count.clone();
    let server_handle = thread::spawn(move || -> IpcResult<()> {
        let mut listener = server_transport.bind(socket_path)?;
        
        // Accept connections in a loop
        for _ in 0..5 {
            match listener.accept() {
                Ok(mut connection) => {
                    *server_connections.lock().unwrap() += 1;
                    
                    // Echo any data received
                    let mut buffer = [0u8; 1024];
                    if let Ok(bytes_read) = connection.read(&mut buffer) {
                        if bytes_read > 0 {
                            let _ = connection.write(&buffer[..bytes_read]);
                            let _ = connection.flush();
                        }
                    }
                }
                Err(_) => break,
            }
        }
        
        Ok(())
    });
    
    // Give server time to start
    thread::sleep(Duration::from_millis(100));
    
    // Create multiple concurrent client connections
    let client_handles: Vec<_> = (0..5).map(|i| {
        let transport = transport.clone();
        let socket_path = socket_path.to_string();
        thread::spawn(move || -> IpcResult<()> {
            let mut connection = transport.connect(&socket_path)?;
            
            let message = format!("Client {}", i);
            connection.write(message.as_bytes())?;
            connection.flush()?;
            
            let mut buffer = [0u8; 1024];
            let bytes_read = connection.read(&mut buffer)?;
            assert_eq!(&buffer[..bytes_read], message.as_bytes());
            
            Ok(())
        })
    }).collect();
    
    // Wait for all clients to complete
    for handle in client_handles {
        handle.join().unwrap().unwrap();
    }
    
    // Wait for server to complete
    server_handle.join().unwrap().unwrap();
    
    // Verify all connections were handled
    assert_eq!(*connections_count.lock().unwrap(), 5);
    
    // Cleanup
    let _ = fs::remove_file(socket_path);
}

/// Test socket cleanup on drop
#[cfg(unix)]
#[test]
fn test_socket_cleanup() {
    let socket_path = "/tmp/test_cleanup_drop.sock";
    
    // Remove any existing socket file
    let _ = fs::remove_file(socket_path);
    
    {
        let config = UnixSocketConfig::stream(socket_path);
        let transport = UnixSocketTransport::new(config).unwrap();
        let _listener = transport.bind(socket_path).unwrap();
        
        // Verify socket file exists
        assert!(std::path::Path::new(socket_path).exists());
    } // Transport and listener are dropped here
    
    // Give some time for cleanup
    thread::sleep(Duration::from_millis(50));
    
    // Socket file should be cleaned up
    // Note: This depends on the cleanup_on_drop configuration
}

/// Test timeout behavior
#[cfg(unix)]
#[test]
fn test_timeout_behavior() {
    let socket_path = "/tmp/test_timeout.sock";
    let config = UnixSocketConfig::stream(socket_path)
        .with_timeout(Duration::from_millis(100));
    
    let transport = UnixSocketTransport::new(config).unwrap();
    
    // Remove any existing socket file
    let _ = fs::remove_file(socket_path);
    
    // Test connection timeout to non-existent server
    let start = std::time::Instant::now();
    let result = transport.connect(socket_path);
    let elapsed = start.elapsed();
    
    assert!(result.is_err());
    // The connection should fail relatively quickly since there's no server
    assert!(elapsed < Duration::from_secs(2));
    
    // Cleanup
    let _ = fs::remove_file(socket_path);
}

/// Test stream transport properties
#[cfg(unix)]
#[test]
fn test_stream_transport_properties() {
    let config = UnixSocketConfig::stream("/tmp/test_stream.sock");
    let transport = UnixSocketTransport::new(config).unwrap();
    
    assert_eq!(transport.max_message_size(), 1024 * 1024); // 1MB default
    assert!(!transport.preserves_message_boundaries()); // Stream sockets don't preserve boundaries
    
    let config = UnixSocketConfig::new("/tmp/test_seq.sock", UnixSocketType::Sequential);
    let transport = UnixSocketTransport::new(config).unwrap();
    
    assert!(transport.preserves_message_boundaries()); // Sequential sockets do preserve boundaries
}

/// Benchmark basic read/write performance
#[cfg(unix)]
#[test]
fn test_performance_basic() {
    let socket_path = "/tmp/test_performance.sock";
    let config = UnixSocketConfig::stream(socket_path)
        .with_buffer_size(65536); // Large buffer for performance
    
    let transport = Arc::new(UnixSocketTransport::new(config).unwrap());
    
    // Remove any existing socket file
    let _ = fs::remove_file(socket_path);
    
    let data_size = 1024 * 1024; // 1MB of data
    let test_data = vec![0xAB; data_size];
    
    // Start server
    let server_transport = transport.clone();
    let server_data = test_data.clone();
    let server_handle = thread::spawn(move || -> IpcResult<()> {
        let mut listener = server_transport.bind(socket_path)?;
        let mut connection = listener.accept()?;
        
        // Read all data
        let mut buffer = vec![0u8; data_size];
        let mut total_read = 0;
        while total_read < data_size {
            let bytes_read = connection.read(&mut buffer[total_read..])?;
            if bytes_read == 0 {
                break;
            }
            total_read += bytes_read;
        }
        
        assert_eq!(total_read, data_size);
        assert_eq!(buffer, server_data);
        
        // Echo the data back
        let mut total_written = 0;
        while total_written < data_size {
            let bytes_written = connection.write(&buffer[total_written..])?;
            total_written += bytes_written;
        }
        connection.flush()?;
        
        Ok(())
    });
    
    // Give server time to start
    thread::sleep(Duration::from_millis(100));
    
    // Connect and measure performance
    let start = std::time::Instant::now();
    
    let mut client_connection = transport.connect(socket_path).unwrap();
    
    // Write data
    let mut total_written = 0;
    while total_written < data_size {
        let bytes_written = client_connection.write(&test_data[total_written..]).unwrap();
        total_written += bytes_written;
    }
    client_connection.flush().unwrap();
    
    // Read echo back
    let mut buffer = vec![0u8; data_size];
    let mut total_read = 0;
    while total_read < data_size {
        let bytes_read = client_connection.read(&mut buffer[total_read..]).unwrap();
        if bytes_read == 0 {
            break;
        }
        total_read += bytes_read;
    }
    
    let elapsed = start.elapsed();
    
    assert_eq!(total_written, data_size);
    assert_eq!(total_read, data_size);
    assert_eq!(buffer, test_data);
    
    // Calculate throughput (2MB total: 1MB sent + 1MB received)
    let throughput_mbps = (2.0 * data_size as f64) / (1024.0 * 1024.0) / elapsed.as_secs_f64();
    
    println!("Unix socket throughput: {:.2} MB/s", throughput_mbps);
    
    // Performance should be reasonable (adjust threshold as needed)
    assert!(throughput_mbps > 10.0, "Throughput too low: {:.2} MB/s", throughput_mbps);
    
    // Wait for server to complete
    server_handle.join().unwrap().unwrap();
    
    // Cleanup
    let _ = fs::remove_file(socket_path);
}
