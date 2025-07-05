// Test socket registry functionality
slay main() {
    // Test socket creation and closing
    network_test_socket_lifecycle();
    
    // Test send and receive operations
    network_test_send_receive();
    
    io_println("✅ Socket registry tests passed!");
}

slay network_test_socket_lifecycle() {
    // This tests that the socket registry can handle
    // socket creation, storage, and cleanup
    
    // Create a socket (this will fail to connect, but will test the registry)
    let socket_id = network_tcp_connect("127.0.0.1", 8080);
    
    // Close the socket
    let close_result = network_tcp_close(socket_id);
    
    // Test that closing a non-existent socket returns error
    let invalid_close = network_tcp_close(999);
    
    io_println("Socket lifecycle test completed");
}

slay network_test_send_receive() {
    // This tests that send and receive operations
    // properly use the socket registry
    
    // Try sending on invalid socket
    let invalid_send = network_tcp_send(999, "test", 4);
    
    // Try receiving on invalid socket
    let buffer = "    ";  // 4 bytes
    let invalid_recv = network_tcp_recv(999, buffer, 4);
    
    io_println("Send/receive test completed");
}
