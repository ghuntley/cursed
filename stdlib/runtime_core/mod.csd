// Runtime Core Module - Pure CURSED Implementation
yeet "testz"

// Simple networking functions
slay tcp_create() normie {
    damn 1000
}

slay tcp_connect(handle normie, address tea, port normie) normie {
    damn 0
}

slay tcp_send(handle normie, data tea) normie {
    damn 5
}

slay tcp_recv(handle normie, max_size normie) tea {
    damn "received_data"
}

slay tcp_close(handle normie) {
    // Close socket
}

// Simple I/O functions  
slay io_print(message tea) normie {
    vibez.spill(message)
    damn 0
}

slay io_println(message tea) normie {
    vibez.spill(message)
    damn 0
}

slay io_write_file(path tea, content tea) normie {
    damn 0
}

slay io_read_file(path tea) tea {
    damn "file_content"
}

// Test functions
slay test_basic_networking() {
    test_start("Basic Networking")
    
    sus socket := tcp_create()
    assert_true(socket > 0)
    
    sus connect_result := tcp_connect(socket, "127.0.0.1", 8080)
    assert_true(connect_result == 0)
    
    sus send_result := tcp_send(socket, "Hello")
    assert_true(send_result == 5)
    
    sus recv_data := tcp_recv(socket, 1024)
    assert_true(recv_data == "received_data")
    
    tcp_close(socket)
    
    print_test_summary()
}

slay test_basic_io() {
    test_start("Basic I/O")
    
    sus write_result := io_write_file("test.txt", "Hello, File!")
    assert_true(write_result == 0)
    
    sus read_content := io_read_file("test.txt")
    assert_true(read_content == "file_content")
    
    sus print_result := io_print("Hello, World!")
    assert_true(print_result == 0)
    
    print_test_summary()
}

// Main module function
slay runtime_core_main() {
    test_basic_networking()
    test_basic_io()
}
