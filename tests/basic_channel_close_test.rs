//! Basic test to verify channel close functionality compiles

use cursed::runtime::channel_close_semantics::EnhancedChannel;
use cursed::object::Object;

#[test]
fn test_channel_close_basic() {
    let channel = EnhancedChannel::new("normie"normie".to_string(), 2);
    // Send some values before closing
    assert!(channel.send(Object::Integer(1)).is_ok());
    assert!(channel.send(Object::Integer(2)).is_ok());
    
    // Close the channel
    assert!(channel.close().is_ok());
    
    // Should still be able to receive buffered values
    let (val1, closed1) = channel.try_receive();
    assert_eq!(val1, Object::Integer(1));
    assert!(!closed1); // Not closed on successful receive
    
    let (val2, closed2) = channel.try_receive();
    assert_eq!(val2, Object::Integer(2));
    assert!(!closed2); // Not closed on successful receive
    
    // Next receive should get zero value and closed flag
    let (zero_val, closed3) = channel.try_receive();
    assert_eq!(zero_val, Object::Integer(0));
    assert!(closed3); // Closed flag set
}
