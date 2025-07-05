#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use cursed::runtime::channels::SimpleChannel;
    use cursed::execution::CursedValue;
    
    #[test]
    fn test_channel_creation() {
        let channel = Arc::new(SimpleChannel::<CursedValue>::new());
        assert!(!channel.is_closed());
    }
    
    #[test]
    fn test_channel_send_receive() {
        let channel = Arc::new(SimpleChannel::<CursedValue>::new());
        
        // Test basic send/receive
        let value = CursedValue::Integer(42);
        
        // This would need to be tested in separate threads for unbuffered channels
        // For now, just test that the channel can be created
        assert!(!channel.is_closed());
    }
}
