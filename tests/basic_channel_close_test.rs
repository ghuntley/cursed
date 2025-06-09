//! Basic test to verify channel close functionality compiles

use cursed::runtime::channel_close_semantics::EnhancedChannel;
use cursed::object::Object;

#[test]
fn test_channel_close_basic() {
    let channel = EnhancedChannel::new("normie".to_string(), 1);
    
    // Should be able to close channel
    assert!(channel.close().is_ok());
    assert!(channel.is_closed());
    
    // Multiple closes should be fine
    assert!(channel.close().is_ok());
    
    // Send should fail on closed channel
    assert!(channel.send(Object::Integer(42)).is_err());
}

#[test]
fn test_channel_close_receive_semantics() {
    let channel = EnhancedChannel::new("normie".to_string(), 2);
    
    // Send some values
    assert!(channel.send(Object::Integer(123)).is_ok());
    assert!(channel.send(Object::Integer(456)).is_ok());
    
    // Close channel
    assert!(channel.close().is_ok());
    
    // Should still be able to receive buffered values
    let (val1, closed1) = channel.receive().unwrap();
    assert_eq!(val1, Object::Integer(123));
    assert!(!closed1); // Not closed flag yet
    
    let (val2, closed2) = channel.receive().unwrap();
    assert_eq!(val2, Object::Integer(456));
    assert!(!closed2); // Not closed flag yet
    
    // Now should get zero value with closed flag
    let (zero_val, closed3) = channel.receive().unwrap();
    assert_eq!(zero_val, Object::Integer(0));
    assert!(closed3); // Closed flag set
}
