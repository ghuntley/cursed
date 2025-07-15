//! Comprehensive tests for channel lifecycle management

use super::lifecycle::*;
use crate::runtime::channels::ChannelError;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_lifecycle_basic() {
        let manager = ChannelLifecycleManager::new();
        
        // Register a channel
        let id = manager.register_channel("test_channel".to_string(), 100).unwrap();
        assert!(id > 0);
        
        // Check initial stats
        let stats = manager.get_lifecycle_stats();
        assert_eq!(stats.total_created, 1);
        assert_eq!(stats.active_channels, 1);
        
        // Record some activity
        manager.record_message_sent(id, 10).unwrap();
        manager.record_message_received(id, 10).unwrap();
        
        // Check updated stats
        let stats = manager.get_lifecycle_stats();
        assert_eq!(stats.total_messages_sent, 1);
        assert_eq!(stats.total_messages_received, 1);
        
        // Unregister channel
        manager.unregister_channel(id).unwrap();
        
        // Check final stats
        let stats = manager.get_lifecycle_stats();
        assert_eq!(stats.total_closed, 1);
        assert_eq!(stats.active_channels, 0);
    }

    #[test]
    fn test_resource_limits_enforcement() {
        let limits = ChannelResourceLimits {
            max_concurrent_channels: 2,
            max_buffer_size: 50,
            max_total_memory: 1000,
            max_messages_per_channel: 100,
            strict_enforcement: true,
        };
        
        let manager = ChannelLifecycleManager::with_limits(limits);
        
        // Register channels up to limit
        let id1 = manager.register_channel("test1".to_string(), 40).unwrap();
        let id2 = manager.register_channel("test2".to_string(), 40).unwrap();
        
        // Try to exceed channel limit
        let result = manager.register_channel("test3".to_string(), 40);
        assert!(result.is_err());
        match result.unwrap_err() {
            ChannelError::AllocationError(msg) => {
                assert!(msg.contains("Maximum concurrent channels reached"));
            }
            _ => panic!("Expected AllocationError"),
        }
        
        // Try to exceed buffer size limit
        manager.unregister_channel(id1).unwrap();
        let result = manager.register_channel("test4".to_string(), 100);
        assert!(result.is_err());
        match result.unwrap_err() {
            ChannelError::InvalidBufferSize(size) => {
                assert_eq!(size, 100);
            }
            _ => panic!("Expected InvalidBufferSize"),
        }
    }

    #[test]
    fn test_channel_integrity_verification() {
        let manager = ChannelLifecycleManager::new();
        
        // Register valid channel
        let id = manager.register_channel("test".to_string(), 100).unwrap();
        assert!(manager.verify_channel_integrity(id).unwrap());
        
        // Test with non-existent channel
        assert!(!manager.verify_channel_integrity(999).unwrap());
        
        // Test with normal activity
        manager.record_message_sent(id, 10).unwrap();
        manager.record_message_received(id, 10).unwrap();
        assert!(manager.verify_channel_integrity(id).unwrap());
    }

    #[test]
    fn test_event_system() {
        let manager = ChannelLifecycleManager::new();
        
        // Set up event collection
        let events = Arc::new(Mutex::new(Vec::new()));
        let events_clone = Arc::clone(&events);
        
        manager.add_event_listener(move |event| {
            events_clone.lock().unwrap().push(event.clone());
        }).unwrap();
        
        // Perform operations that should generate events
        let id = manager.register_channel("test".to_string(), 100).unwrap();
        manager.record_message_sent(id, 10).unwrap();
        manager.record_message_received(id, 10).unwrap();
        manager.unregister_channel(id).unwrap();
        
        // Check captured events
        let captured_events = events.lock().unwrap();
        assert_eq!(captured_events.len(), 4);
        
        // Verify event types
        match &captured_events[0] {
            ChannelEvent::Created { id: event_id, capacity } => {
                assert_eq!(*event_id, id);
                assert_eq!(*capacity, 100);
            }
            _ => panic!("Expected Created event"),
        }
        
        match &captured_events[1] {
            ChannelEvent::MessageSent { id: event_id, size } => {
                assert_eq!(*event_id, id);
                assert_eq!(*size, 10);
            }
            _ => panic!("Expected MessageSent event"),
        }
        
        match &captured_events[2] {
            ChannelEvent::MessageReceived { id: event_id, size } => {
                assert_eq!(*event_id, id);
                assert_eq!(*size, 10);
            }
            _ => panic!("Expected MessageReceived event"),
        }
        
        match &captured_events[3] {
            ChannelEvent::Closed { id: event_id } => {
                assert_eq!(*event_id, id);
            }
            _ => panic!("Expected Closed event"),
        }
    }

    #[test]
    fn test_buffer_address_tracking() {
        let manager = ChannelLifecycleManager::new();
        
        let id = manager.register_channel("test".to_string(), 100).unwrap();
        
        // Add some buffer addresses
        let addr1 = 0x1000;
        let addr2 = 0x2000;
        manager.add_buffer_address(id, addr1).unwrap();
        manager.add_buffer_address(id, addr2).unwrap();
        
        // Retrieve buffer addresses
        let addresses = manager.get_buffer_addresses(id);
        assert_eq!(addresses.len(), 2);
        assert!(addresses.contains(&addr1));
        assert!(addresses.contains(&addr2));
        
        // Test non-existent channel
        let empty_addresses = manager.get_buffer_addresses(999);
        assert!(empty_addresses.is_empty());
    }

    #[test]
    fn test_channel_statistics() {
        let manager = ChannelLifecycleManager::new();
        
        let id = manager.register_channel("test".to_string(), 100).unwrap();
        
        // Get initial stats
        let stats = manager.get_channel_stats(id).unwrap();
        assert_eq!(stats.capacity, 100);
        assert_eq!(stats.current_length, 0);
        assert_eq!(stats.total_sent, 0);
        assert_eq!(stats.total_received, 0);
        
        // Record some activity
        manager.record_message_sent(id, 10).unwrap();
        manager.record_message_sent(id, 20).unwrap();
        manager.record_message_received(id, 15).unwrap();
        
        // Check updated stats
        let stats = manager.get_channel_stats(id).unwrap();
        assert_eq!(stats.total_sent, 2);
        assert_eq!(stats.total_received, 1);
        assert_eq!(stats.current_length, 15); // 10 + 20 - 15
        
        // Test non-existent channel
        let no_stats = manager.get_channel_stats(999);
        assert!(no_stats.is_none());
    }

    #[test]
    fn test_monitoring_thread() {
        let mut manager = ChannelLifecycleManager::new();
        
        // Start monitoring
        manager.start_monitoring().unwrap();
        
        // Create some channels
        let id1 = manager.register_channel("test1".to_string(), 100).unwrap();
        let id2 = manager.register_channel("test2".to_string(), 200).unwrap();
        
        // Wait a bit for monitoring to run
        thread::sleep(Duration::from_millis(100));
        
        // Force cleanup
        manager.force_cleanup().unwrap();
        
        // Stop monitoring
        manager.stop_monitoring().unwrap();
        
        // Should not be able to start monitoring again while already running
        manager.start_monitoring().unwrap(); // This should work after stopping
        manager.stop_monitoring().unwrap();
    }

    #[test]
    fn test_concurrent_operations() {
        let manager = Arc::new(ChannelLifecycleManager::new());
        let num_threads = 10;
        let operations_per_thread = 100;
        
        let mut handles = Vec::new();
        
        // Spawn multiple threads doing concurrent operations
        for thread_id in 0..num_threads {
            let manager_clone = Arc::clone(&manager);
            let handle = thread::spawn(move || {
                for i in 0..operations_per_thread {
                    let channel_name = format!("thread{}_channel{}", thread_id, i);
                    let id = manager_clone.register_channel(channel_name, 100).unwrap();
                    
                    // Simulate some activity
                    manager_clone.record_message_sent(id, 10).unwrap();
                    manager_clone.record_message_received(id, 10).unwrap();
                    
                    // Verify integrity
                    assert!(manager_clone.verify_channel_integrity(id).unwrap());
                    
                    // Unregister
                    manager_clone.unregister_channel(id).unwrap();
                }
            });
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Check final statistics
        let stats = manager.get_lifecycle_stats();
        assert_eq!(stats.total_created, (num_threads * operations_per_thread) as u64);
        assert_eq!(stats.total_closed, (num_threads * operations_per_thread) as u64);
        assert_eq!(stats.active_channels, 0);
        assert_eq!(stats.total_messages_sent, (num_threads * operations_per_thread) as u64);
        assert_eq!(stats.total_messages_received, (num_threads * operations_per_thread) as u64);
    }

    #[test]
    fn test_global_manager() {
        let manager = get_global_channel_manager();
        
        // Test basic functionality
        let id = manager.register_channel("global_test".to_string(), 100).unwrap();
        assert!(id > 0);
        
        // Test that we get the same instance
        let manager2 = get_global_channel_manager();
        assert!(manager2.verify_channel_integrity(id).unwrap());
        
        manager.unregister_channel(id).unwrap();
    }

    #[test]
    fn test_limits_update() {
        let manager = ChannelLifecycleManager::new();
        
        // Create initial channels
        let id1 = manager.register_channel("test1".to_string(), 100).unwrap();
        let id2 = manager.register_channel("test2".to_string(), 100).unwrap();
        
        // Update limits to be more restrictive
        let new_limits = ChannelResourceLimits {
            max_concurrent_channels: 1,
            max_buffer_size: 50,
            max_total_memory: 500,
            max_messages_per_channel: 10,
            strict_enforcement: true,
        };
        
        manager.update_limits(new_limits).unwrap();
        
        // New channels should be rejected based on new limits
        let result = manager.register_channel("test3".to_string(), 100);
        assert!(result.is_err());
        
        // Clean up
        manager.unregister_channel(id1).unwrap();
        manager.unregister_channel(id2).unwrap();
    }

    #[test]
    fn test_debug_mode() {
        let mut manager = ChannelLifecycleManager::new();
        manager.enable_debug();
        
        // Operations should work the same but with debug output
        let id = manager.register_channel("debug_test".to_string(), 100).unwrap();
        manager.record_message_sent(id, 10).unwrap();
        manager.unregister_channel(id).unwrap();
        
        // Test integrity check with debug mode
        assert!(!manager.verify_channel_integrity(999).unwrap());
    }

    #[test]
    fn test_active_channels_listing() {
        let manager = ChannelLifecycleManager::new();
        
        // Initially no active channels
        let active = manager.get_active_channels();
        assert!(active.is_empty());
        
        // Create some channels
        let id1 = manager.register_channel("test1".to_string(), 100).unwrap();
        let id2 = manager.register_channel("test2".to_string(), 100).unwrap();
        let id3 = manager.register_channel("test3".to_string(), 100).unwrap();
        
        // Check active channels
        let active = manager.get_active_channels();
        assert_eq!(active.len(), 3);
        assert!(active.contains(&id1));
        assert!(active.contains(&id2));
        assert!(active.contains(&id3));
        
        // Close one channel
        manager.unregister_channel(id2).unwrap();
        
        // Check updated active channels
        let active = manager.get_active_channels();
        assert_eq!(active.len(), 2);
        assert!(active.contains(&id1));
        assert!(!active.contains(&id2));
        assert!(active.contains(&id3));
        
        // Clean up
        manager.unregister_channel(id1).unwrap();
        manager.unregister_channel(id3).unwrap();
        
        // Should be empty again
        let active = manager.get_active_channels();
        assert!(active.is_empty());
    }

    #[test]
    fn test_memory_tracking() {
        let manager = ChannelLifecycleManager::new();
        
        // Create channels of different sizes
        let id1 = manager.register_channel("small".to_string(), 10).unwrap();
        let id2 = manager.register_channel("medium".to_string(), 100).unwrap();
        let id3 = manager.register_channel("large".to_string(), 1000).unwrap();
        
        // Check memory allocation tracking
        let stats = manager.get_lifecycle_stats();
        assert!(stats.total_memory_allocated > 0);
        
        // Close channels and check memory freed
        manager.unregister_channel(id1).unwrap();
        manager.unregister_channel(id2).unwrap();
        manager.unregister_channel(id3).unwrap();
        
        let stats = manager.get_lifecycle_stats();
        assert!(stats.total_memory_freed > 0);
        assert_eq!(stats.total_memory_allocated, stats.total_memory_freed);
    }

    #[test]
    fn test_lifetime_tracking() {
        let manager = ChannelLifecycleManager::new();
        
        // Create and immediately close a channel
        let id = manager.register_channel("short_lived".to_string(), 100).unwrap();
        thread::sleep(Duration::from_millis(10));
        manager.unregister_channel(id).unwrap();
        
        // Check that lifetime was tracked
        let stats = manager.get_lifecycle_stats();
        assert!(stats.average_lifetime > 0.0);
        assert!(stats.average_lifetime < 1.0); // Should be less than 1 second
    }
}
