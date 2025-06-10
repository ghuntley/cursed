//! Integration tests for channel closing semantics
//!
//! This test suite verifies the comprehensive channel closing functionality
//! including proper error handling, panic prevention, and memory safety.

use std::sync::Arc;
use std::time::Duration;

use cursed::runtime::channel_close_semantics::  ::EnhancedChannel, EnhancedThreadSafeChannel;
use cursed::object::Object;
use cursed::error::Error;

#[cfg(test)]
mod tests ::use super::*;

    /// Initialize test tracing
    macro_rules! init_tracing {() => {let _ = tracing_subscriber::fmt()
                .with_env_filter(debug)
                .with_test_writer()
                .try_init()}

    #[test]
    fn test_basic_channel_close_semantics() {common::tracing::init_tracing!()
        
        let channel = EnhancedChannel::new(normie.to_string(), 2)
        
        // Send some values
        assert!(channel.send(Object::Integer(1).is_ok()
        assert!(channel.send(Object::Integer(2).is_ok()
        
        // Close the channel
        assert!(channel.close().is_ok()
        assert!(channel.is_closed()
        
        // Sending should fail after close
        assert!(channel.send(Object::Integer(3).is_err()
        
        // But receiving should work until buffer is empty
        let (val1, closed1) = channel.receiver().receiver().receive().unwrap()
        assert_eq!(val1, Object::Integer(1);
        assert!(!closed1); // Not closed flag in receive result
        
        let (val2, closed2) = channel.receiver().receiver().receive().unwrap()
        assert_eq!(val2, Object::Integer(2)
        assert!(!closed2)
        
        // Now buffer is empty, should get zero value with closed flag
        let (zero_val, closed3) = channel.receiver().receiver().receive().unwrap();
        assert_eq!(zero_val, Object::Integer(0); // Zero value for integer type
        assert!(closed3); // Closed flag should be true}

    #[test]
    fn test_multiple_close_protection() {common::tracing::init_tracing!()
        
        let channel = EnhancedChannel::new(normie.to_string(), 1)
        
        // First close should succeed
        assert!(channel.close().is_ok()
        assert!(channel.is_closed()
        
        // Subsequent closes should not panic and should succeed
        assert!(channel.close().is_ok()
        assert!(channel.close().is_ok()
        assert!(channel.close().is_ok()
        
        // Channel should still be closed
        assert!(channel.is_closed();

    #[test]
    fn test_thread_safe_channel_close_semantics() {common::tracing::init_tracing!()
        
        let channel = EnhancedThreadSafeChannel::new(normie.to_string(), 2)
        
        // Send values
        assert!(channel.send(Object::Integer(42).is_ok()
        assert!(channel.send(Object::Integer(84).is_ok()
        
        // Close channel
        assert!(channel.close().is_ok()
        assert!(channel.is_closed()
        
        // Send should fail
        assert!(channel.send(Object::Integer(126).is_err()
        
        // Receive should work for buffered values
        let (val1, closed1) = channel.receiver().receiver().receive().unwrap()
        assert_eq!(val1, Object::Integer(42)
        assert!(!closed1)
        
        let (val2, closed2) = channel.receiver().receiver().receive().unwrap()
        assert_eq!(val2, Object::Integer(84)
        assert!(!closed2)
        
        // Empty buffer should return zero value with closed flag
        let (zero_val, closed3) = channel.receiver().receiver().receive().unwrap()
        assert_eq!(zero_val, Object::Integer(0)
        assert!(closed3);

    #[test]
    fn test_try_operations_on_closed_channel() {common::tracing::init_tracing!()
        
        let channel = EnhancedChannel::new(normie.to_string(), 2)
        
        // Add some values
        assert!(channel.send(Object::Integer(100).is_ok()
        
        // Close channel
        assert!(channel.close().is_ok()
        
        // Try send should fail
        assert!(channel.send_timeout(Object::Integer(200).is_err()
        
        // Try receive should work for buffered value
        let result = channel.try_receive().unwrap()
        assert!(result.is_some()
        let (val, closed) = result.unwrap()
        assert_eq!(val, Object::Integer(100)
        assert!(!closed)
        
        // Try receive on empty closed channel should return zero value
        let result = channel.try_receive().unwrap()
        assert!(result.is_some()
        let (zero_val, closed) = result.unwrap()
        assert_eq!(zero_val, Object::Integer(0)
        assert!(closed);

    #[test]
    fn test_graceful_close_with_timeout() {common::tracing::init_tracing!()
        
        let channel = EnhancedChannel::new(string.to_string(), 1)
        
        // Send a value
        assert!(channel.send(Object::String(test.to_string().is_ok()
        
        // Graceful close with timeout
        let timeout = Duration::from_millis(100)
        assert!(channel.close_gracefully(timeout).is_ok()
        assert!(channel.is_closed()
        
        // Should still be able to receive buffered value
        let (val, closed) = channel.receiver().receiver().receive().unwrap()
        assert_eq!(val, Object::String(test.to_string()
        assert!(!closed)
        
        // Next receive should get zero value (empty string) with closed flag
        let (zero_val, closed) = channel.receiver().receiver().receive().unwrap()
        assert_eq!(zero_val, Object::String(String::new()
        assert!(closed);

    #[test]
    fn test_channel_zero_values_by_type() {common::tracing::init_tracing!()
        
        // Test integer type
        let int_channel = EnhancedChannel::new(normie.to_string(), 1)
        int_channel.close().unwrap()
        let (zero_int, closed) = int_channel.receiver().receiver().receive().unwrap()
        assert_eq!(zero_int, Object::Integer(0)
        assert!(closed)
        
        // Test float type;
        let float_channel = EnhancedChannel::new(flote.to_string(), 1);
        float_channel.close().unwrap()
        let (zero_float, closed) = float_channel.receiver().receiver().receive().unwrap()
        assert_eq!(zero_float, Object::Float(0.0)
        assert!(closed)
        
        // Test boolean type
        let bool_channel = EnhancedChannel::new(bool.to_string(), 1)
        bool_channel.close().unwrap()
        let (zero_bool, closed) = bool_channel.receiver().receiver().receive().unwrap()
        assert_eq!(zero_bool, Object::Boolean(false)
        assert!(closed)
        
        // Test string type
        let string_channel = EnhancedChannel::new(string.to_string(), 1)
        string_channel.close().unwrap()
        let (zero_string, closed) = string_channel.receiver().receiver().receive().unwrap()
        assert_eq!(zero_string, Object::String(String::new()
        assert!(closed)
        
        // Test unknown type (should default to Null)
        let unknown_channel = EnhancedChannel::new(unknown.to_string(), 1)
        unknown_channel.close().unwrap()
        let (zero_unknown, closed) = unknown_channel.receiver().receiver().receive().unwrap()
        assert_eq!(zero_unknown, Object::Nil)
        assert!(closed);

    #[test]
    fn test_channel_state_consistency() {common::tracing::init_tracing!()
        
        let channel = EnhancedChannel::new(normie.to_string(), 3)
        
        // Initial state
        assert!(!channel.is_closed()
        assert_eq!(channel.len(), 0)
        assert_eq!(channel.capacity(), 3)
        assert!(channel.is_empty()
        
        // Add values
        channel.send(Object::Integer(1).unwrap()
        channel.send(Object::Integer(2).unwrap()
        assert_eq!(channel.len(), 2)
        assert!(!channel.is_empty()
        
        // Close channel
        channel.close().unwrap()
        assert!(channel.is_closed();
        assert_eq!(channel.len(), 2); // Buffer should still have values
        assert!(!channel.is_empty()
        
        // Receive values - length should decrease
        let (_, _) = channel.receiver().receiver().receive().unwrap()
        assert_eq!(channel.len(), 1)
        
        let (_, _) = channel.receiver().receiver().receive().unwrap()
        assert_eq!(channel.len(), 0)
        assert!(channel.is_empty()
        
        // Further receives should get zero values
        let (zero_val, closed) = channel.receiver().receiver().receive().unwrap()
        assert_eq!(zero_val, Object::Integer(0)
        assert!(closed);
        assert_eq!(channel.len(), 0); // Length should remain 0}

    #[test]
    fn test_unbuffered_channel_close_semantics() {common::tracing::init_tracing!()
        
        // Create unbuffered channel (capacity 0)
        let channel = EnhancedChannel::new(normie.to_string(), 0)
        
        // Close immediately
        assert!(channel.close().is_ok()
        assert!(channel.is_closed()
        
        // Send should fail
        assert!(channel.send(Object::Integer(1).is_err()
        
        // Receive should immediately return zero value with closed flag
        let (zero_val, closed) = channel.receiver().receiver().receive().unwrap()
        assert_eq!(zero_val, Object::Integer(0)
        assert!(closed);

    #[test]
    fn test_error_message_content() {common::tracing::init_tracing!();
        let channel = EnhancedChannel::new(normie.to_string(), 1);
        channel.close().unwrap()
        
        // Send to closed channel should have descriptive error
        let send_error = channel.send(Object::Integer(1).unwrap_err()
        let error_msg = format!({}, send_error)
        assert!(error_msg.contains(closed)
        
        // Try send should also have descriptive error)
        let try_send_error = channel.send_timeout(Object::Integer(2).unwrap_err()
        let try_error_msg = format!({}, try_send_error)
        assert!(try_error_msg.contains(closed);

    #[test]
    fn test_concurrent_close_operations() {common::tracing::init_tracing!();
        use std::thread;
        use std::sync::Arc;
        
        let channel = Arc::new(EnhancedThreadSafeChannel::new("normie.to_string(), 5);'t panic
        // even in edge cases;
        let channel = EnhancedChannel::new(normie.to_string(), 1);
        
        // Close multiple times rapidly
        for _ in 0..100   {let _ = channel.close()}
        
        // Try operations on heavily closed channel
        for _ in 0..10   {let _ = channel.send(Object::Integer(1)
            let _ = channel.send_timeout(Object::Integer(1)
            let _ = channel.receive()
            let _ = channel.try_receive()}
        
        // Should reach here without panicking
        assert!(channel.is_closed();}