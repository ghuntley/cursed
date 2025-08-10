#[cfg(test)]
mod tests {
    use super::super::runtime::channels::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_channel_fixes_no_panic() {
        // Test that channel operations don't panic
        let (sender, receiver) = simple_channel::<i32>();
        
        // Test non-blocking send (should not panic)
        let send_result = sender.try_send(42);
        println!("Send result: {:?}", send_result);
        
        // Test non-blocking receive (should not panic)
        let recv_result = receiver.try_recv();
        println!("Receive result: {:?}", recv_result);
        
        // Test that operations complete successfully
        let sender_handle = thread::spawn(move || {
            sender.send(42)
        });
        
        let recv_result = receiver.recv();
        assert!(recv_result.is_ok());
        
        sender_handle.join().unwrap();
    }

    #[test]
    fn test_buffered_channel_fixes_no_panic() {
        // Test buffered channel operations don't panic
        let (sender, receiver) = simple_buffered_channel::<String>(3);
        
        // Fill the buffer
        assert!(sender.send("test1".to_string()).is_ok());
        assert!(sender.send("test2".to_string()).is_ok());
        assert!(sender.send("test3".to_string()).is_ok());
        
        // Try to send when full (should not panic)
        let send_result = sender.try_send("test4".to_string());
        println!("Buffer full send result: {:?}", send_result);
        
        // Receive values
        let result1 = receiver.recv();
        let result2 = receiver.recv();
        let result3 = receiver.recv();
        
        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert!(result3.is_ok());
        
        // Try to receive when empty (should not panic)
        let recv_result = receiver.try_recv();
        println!("Empty receive result: {:?}", recv_result);
    }

    #[test]
    fn test_channel_close_fixes_no_panic() {
        // Test channel close operations don't panic
        let (sender, receiver) = simple_buffered_channel::<i32>(1);
        
        // Send a value
        assert!(sender.send(42).is_ok());
        
        // Close the channel
        sender.close();
        
        // Should still receive buffered value (should not panic)
        let recv_result = receiver.recv();
        assert!(recv_result.is_ok());
        
        // Further receives should indicate closed (should not panic)
        let closed_result = receiver.try_recv();
        println!("Closed channel receive result: {:?}", closed_result);
    }

    #[test]
    fn test_result_unwrap_methods() {
        // Test the new unwrap methods don't panic
        let send_result: SendResult<i32> = SendResult::Sent;
        assert!(send_result.is_ok());
        
        let send_result_with_value = SendResult::WouldBlock(42);
        let value = send_result_with_value.unwrap_value();
        assert_eq!(value, Some(42));
        
        let recv_result = ReceiveResult::Received(100);
        assert!(recv_result.is_ok());
        
        let recv_result_closed = ReceiveResult::<i32>::Closed;
        assert!(!recv_result_closed.is_ok());
    }
}
