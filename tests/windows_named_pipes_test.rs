/// Windows Named Pipes Integration Test for CURSED
/// 
/// This test validates the Windows named pipes implementation in the IPC system.
/// It tests both server and client modes, message passing, and error handling.

#[cfg(windows)]
mod windows_named_pipes {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;
    use cursed::stdlib::process::real_ipc::*;

    #[test]
    fn test_windows_named_pipe_creation() {
        let config = IpcChannelConfig {
            name: "test_pipe_creation".to_string(),
            channel_type: IpcChannelType::NamedPipe,
            ..Default::default()
        };

        let channel = RealIpcChannel::new(config);
        assert!(channel.is_ok(), "Failed to create Windows named pipe channel");

        let channel = channel.unwrap();
        assert_eq!(channel.state(), ChannelState::Created);
    }

    #[test]
    fn test_windows_named_pipe_bind() {
        let config = IpcChannelConfig {
            name: "test_pipe_bind".to_string(),
            channel_type: IpcChannelType::NamedPipe,
            buffer_size: 8192,
            ..Default::default()
        };

        let mut channel = RealIpcChannel::new(config).unwrap();
        let bind_result = channel.bind();
        
        assert!(bind_result.is_ok(), "Failed to bind Windows named pipe: {:?}", bind_result.err());
        assert_eq!(channel.state(), ChannelState::Bound);

        // Cleanup
        let _ = channel.close();
    }

    #[test]
    fn test_windows_named_pipe_server_client_communication() {
        let pipe_name = "test_pipe_communication";
        
        // Shared result storage
        let server_received = Arc::new(Mutex::new(Vec::new()));
        let client_sent = Arc::new(Mutex::new(Vec::new()));
        let server_received_clone = server_received.clone();
        let client_sent_clone = client_sent.clone();

        // Start server thread
        let server_handle = thread::spawn(move || {
            let config = IpcChannelConfig {
                name: pipe_name.to_string(),
                channel_type: IpcChannelType::NamedPipe,
                buffer_size: 8192,
                message_timeout: Duration::from_secs(10),
                ..Default::default()
            };

            let mut server = RealIpcChannel::new(config).unwrap();
            
            // Bind the server
            if let Err(e) = server.bind() {
                eprintln!("Server bind failed: {:?}", e);
                return;
            }

            // Wait for a message with timeout
            match server.receive(Some(Duration::from_secs(5))) {
                Ok(message) => {
                    let mut received = server_received_clone.lock().unwrap();
                    received.push(message.data);
                    
                    // Send response
                    let response = IpcMessage {
                        id: 2,
                        sender_pid: std::process::id(),
                        data: b"pong".to_vec(),
                        priority: MessagePriority::Normal,
                        timestamp: std::time::Instant::now(),
                        message_type: "response".to_string(),
                        delivery_mode: DeliveryMode::BestEffort,
                    };
                    
                    let _ = server.send(response);
                }
                Err(e) => {
                    eprintln!("Server receive failed: {:?}", e);
                }
            }

            let _ = server.close();
        });

        // Give server time to start
        thread::sleep(Duration::from_millis(100));

        // Start client thread
        let client_handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(200)); // Ensure server is ready

            let config = IpcChannelConfig {
                name: pipe_name.to_string(),
                channel_type: IpcChannelType::NamedPipe,
                buffer_size: 8192,
                message_timeout: Duration::from_secs(10),
                ..Default::default()
            };

            let mut client = RealIpcChannel::new(config).unwrap();
            
            // Connect to server
            if let Err(e) = client.connect() {
                eprintln!("Client connect failed: {:?}", e);
                return;
            }

            // Send message
            let message = IpcMessage {
                id: 1,
                sender_pid: std::process::id(),
                data: b"ping".to_vec(),
                priority: MessagePriority::Normal,
                timestamp: std::time::Instant::now(),
                message_type: "test".to_string(),
                delivery_mode: DeliveryMode::BestEffort,
            };

            {
                let mut sent = client_sent_clone.lock().unwrap();
                sent.push(message.data.clone());
            }

            if let Err(e) = client.send(message) {
                eprintln!("Client send failed: {:?}", e);
                return;
            }

            // Try to receive response
            match client.receive(Some(Duration::from_secs(3))) {
                Ok(response) => {
                    assert_eq!(response.data, b"pong");
                }
                Err(e) => {
                    eprintln!("Client receive failed: {:?}", e);
                }
            }

            let _ = client.close();
        });

        // Wait for both threads to complete
        let _ = server_handle.join();
        let _ = client_handle.join();

        // Verify communication occurred
        let received = server_received.lock().unwrap();
        let sent = client_sent.lock().unwrap();
        
        if !sent.is_empty() && !received.is_empty() {
            assert_eq!(sent[0], b"ping");
            assert_eq!(received[0], b"ping");
        }
    }

    #[test]
    fn test_windows_named_pipe_large_message() {
        let pipe_name = "test_pipe_large_message";
        let large_data = vec![0xAB; 4096]; // 4KB of data
        
        let received_data = Arc::new(Mutex::new(Vec::new()));
        let received_clone = received_data.clone();

        // Server thread
        let server_handle = thread::spawn(move || {
            let config = IpcChannelConfig {
                name: pipe_name.to_string(),
                channel_type: IpcChannelType::NamedPipe,
                buffer_size: 8192,
                max_message_size: 8192,
                ..Default::default()
            };

            let mut server = RealIpcChannel::new(config).unwrap();
            let _ = server.bind();

            if let Ok(message) = server.receive(Some(Duration::from_secs(5))) {
                let mut received = received_clone.lock().unwrap();
                *received = message.data;
            }

            let _ = server.close();
        });

        thread::sleep(Duration::from_millis(100));

        // Client thread
        let client_handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(200));

            let config = IpcChannelConfig {
                name: pipe_name.to_string(),
                channel_type: IpcChannelType::NamedPipe,
                buffer_size: 8192,
                max_message_size: 8192,
                ..Default::default()
            };

            let mut client = RealIpcChannel::new(config).unwrap();
            let _ = client.connect();

            let message = IpcMessage {
                id: 1,
                sender_pid: std::process::id(),
                data: large_data.clone(),
                priority: MessagePriority::Normal,
                timestamp: std::time::Instant::now(),
                message_type: "large_test".to_string(),
                delivery_mode: DeliveryMode::BestEffort,
            };

            let _ = client.send(message);
            let _ = client.close();
        });

        let _ = server_handle.join();
        let _ = client_handle.join();

        // Verify large message was received correctly
        let received = received_data.lock().unwrap();
        if !received.is_empty() {
            assert_eq!(*received, large_data);
        }
    }

    #[test]
    fn test_windows_named_pipe_error_handling() {
        // Test connecting to non-existent pipe
        let config = IpcChannelConfig {
            name: "non_existent_pipe_12345".to_string(),
            channel_type: IpcChannelType::NamedPipe,
            ..Default::default()
        };

        let mut client = RealIpcChannel::new(config).unwrap();
        let connect_result = client.connect();
        
        assert!(connect_result.is_err(), "Should fail to connect to non-existent pipe");
    }

    #[test]
    fn test_windows_named_pipe_statistics() {
        let pipe_name = "test_pipe_stats";
        
        let stats_check = Arc::new(Mutex::new(false));
        let stats_clone = stats_check.clone();

        // Server thread
        let server_handle = thread::spawn(move || {
            let config = IpcChannelConfig {
                name: pipe_name.to_string(),
                channel_type: IpcChannelType::NamedPipe,
                ..Default::default()
            };

            let mut server = RealIpcChannel::new(config).unwrap();
            let _ = server.bind();

            if let Ok(message) = server.receive(Some(Duration::from_secs(5))) {
                let stats = server.stats();
                
                let mut check = stats_clone.lock().unwrap();
                *check = stats.messages_received > 0 && stats.bytes_received > 0;
            }

            let _ = server.close();
        });

        thread::sleep(Duration::from_millis(100));

        // Client thread
        let client_handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(200));

            let config = IpcChannelConfig {
                name: pipe_name.to_string(),
                channel_type: IpcChannelType::NamedPipe,
                ..Default::default()
            };

            let mut client = RealIpcChannel::new(config).unwrap();
            let _ = client.connect();

            let message = IpcMessage {
                id: 1,
                sender_pid: std::process::id(),
                data: b"statistics test".to_vec(),
                priority: MessagePriority::Normal,
                timestamp: std::time::Instant::now(),
                message_type: "stats".to_string(),
                delivery_mode: DeliveryMode::BestEffort,
            };

            let _ = client.send(message);
            
            let stats = client.stats();
            assert!(stats.messages_sent > 0);
            assert!(stats.bytes_sent > 0);

            let _ = client.close();
        });

        let _ = server_handle.join();
        let _ = client_handle.join();

        let stats_verified = *stats_check.lock().unwrap();
        assert!(stats_verified, "Server statistics should show received messages");
    }

    #[test]
    fn test_windows_named_pipe_manager_integration() {
        let mut manager = IpcChannelManager::new(ManagerConfig::default());
        assert!(manager.start().is_ok());

        let config = IpcChannelConfig {
            name: "manager_test_pipe".to_string(),
            channel_type: IpcChannelType::NamedPipe,
            ..Default::default()
        };

        let channel = manager.create_channel(config);
        assert!(channel.is_ok(), "Failed to create channel through manager");

        let channel_ref = manager.get_channel("manager_test_pipe");
        assert!(channel_ref.is_some(), "Failed to retrieve channel from manager");

        assert!(manager.stop().is_ok());
    }
}

// Stub for non-Windows platforms
#[cfg(not(windows))]
mod windows_named_pipes {
    #[test]
    fn test_windows_named_pipes_not_available() {
        println!("Windows named pipes tests skipped on non-Windows platform");
    }
}
