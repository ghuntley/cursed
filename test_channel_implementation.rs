#[cfg(test)]
mod channel_lifecycle_tests {
    use crate::runtime::channels::lifecycle::*;
    use crate::runtime::gc::GarbageCollector;
    use std::sync::Arc;
    
    #[test]
    fn test_enhanced_channel_creation() {
        let manager = ChannelLifecycleManager::new();
        
        // Test enhanced channel creation with buffer
        let buffer_size = 1024;
        let buffer_ptr = Box::into_raw(vec![0u8; buffer_size].into_boxed_slice()) as *mut u8;
        
        let result = manager.create_channel_with_buffer(
            "test_channel".to_string(),
            100,
            buffer_ptr,
            buffer_size
        );
        
        assert!(result.is_ok());
        let channel_id = result.unwrap();
        
        // Verify channel was created
        let stats = manager.get_lifecycle_stats();
        assert_eq!(stats.total_created, 1);
        assert_eq!(stats.active_channels, 1);
        
        // Test channel destruction
        let destroy_result = manager.destroy_channel(channel_id);
        assert!(destroy_result.is_ok());
        
        let stats_after = manager.get_lifecycle_stats();
        assert_eq!(stats_after.total_created, 1);
        assert_eq!(stats_after.total_closed, 1);
        assert_eq!(stats_after.active_channels, 0);
        
        // Clean up
        unsafe {
            let _ = Box::from_raw(std::slice::from_raw_parts_mut(buffer_ptr, buffer_size));
        }
    }
    
    #[test]
    fn test_buffer_resize() {
        let manager = ChannelLifecycleManager::new();
        
        let channel_id = manager.register_channel("test".to_string(), 100).unwrap();
        
        // Test buffer resize
        let resize_result = manager.resize_channel_buffer(channel_id, 200);
        assert!(resize_result.is_ok());
        
        // Verify the channel info was updated
        if let Some(stats) = manager.get_channel_stats(channel_id) {
            assert_eq!(stats.capacity, 200);
        } else {
            panic!("Channel not found after resize");
        }
        
        manager.unregister_channel(channel_id).unwrap();
    }
    
    #[test]
    fn test_sync_barriers() {
        let manager = ChannelLifecycleManager::new();
        
        // Create multiple channels
        let channel1 = manager.register_channel("chan1".to_string(), 10).unwrap();
        let channel2 = manager.register_channel("chan2".to_string(), 10).unwrap();
        let channel3 = manager.register_channel("chan3".to_string(), 10).unwrap();
        
        // Create sync barrier
        let barrier_result = manager.create_channel_sync_barrier(vec![channel1, channel2, channel3]);
        assert!(barrier_result.is_ok());
        let barrier_id = barrier_result.unwrap();
        
        // Test synchronization
        assert!(manager.sync_channels(barrier_id).is_ok());
        assert!(manager.sync_channels(barrier_id).is_ok());
        assert!(manager.sync_channels(barrier_id).is_ok()); // Should complete barrier
        
        // Clean up
        manager.unregister_channel(channel1).unwrap();
        manager.unregister_channel(channel2).unwrap();
        manager.unregister_channel(channel3).unwrap();
        manager.destroy_channel(barrier_id).unwrap();
    }
    
    #[test]
    fn test_error_handling() {
        let manager = ChannelLifecycleManager::new();
        
        let channel_id = manager.register_channel("test".to_string(), 100).unwrap();
        
        // Test error handling
        let error = crate::runtime::channels::ChannelError::BufferFull;
        let result = manager.handle_channel_error(channel_id, &error);
        assert!(result.is_ok());
        
        manager.unregister_channel(channel_id).unwrap();
    }
}
