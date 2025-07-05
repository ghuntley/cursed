slay main() {
    io_println("Testing socket registry");
    
    // Test invalid socket operations
    let invalid_socket = -1;
    
    // Test sending on invalid socket (should return -1)
    let send_result = network_tcp_send(invalid_socket, "test", 4);
    
    // Test receiving on invalid socket (should return -1)
    let buffer = "    ";
    let recv_result = network_tcp_recv(invalid_socket, buffer, 4);
    
    // Test closing invalid socket (should return -1)
    let close_result = network_tcp_close(invalid_socket);
    
    io_println("Socket registry test completed");
}
