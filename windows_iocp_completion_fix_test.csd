// CRITICAL: Windows IOCP Async Promise Completion Fix Validation Test
// This test validates that async I/O operations properly complete on Windows
// and identifies specific completion port integration issues

yeet "vibez"
yeet "testz"
yeet "filez"
yeet "networkz"
yeet "timez"

// Test async file operations completion
slay test_async_file_completion() {
    vibez.spill("🧪 Testing Windows async file I/O completion...")
    
    // Test 1: Async file read completion
    sus test_file_path tea = "test_async_completion.txt"
    sus test_data tea = "Testing async file completion on Windows IOCP"
    sus read_buffer []drip = make([]drip, 1024)
    
    // Write test data first (sync)
    ready (write_file_sync(test_file_path, test_data)) {
        otherwise {
            vibez.spill("❌ Failed to create test file")
            damn false
        }
    }
    
    // Test async read - this should complete via IOCP
    sus start_time drip = get_current_time_ms()
    sus read_result tea = ""
    
    go {
        // Async read operation
        sus async_result tea = read_file_async(test_file_path) fam {
            when _ -> {
                vibez.spill("❌ Async file read failed - IOCP completion issue detected")
                damn ""
            }
        }
        read_result = async_result
        vibez.spill("✅ Async file read completed via IOCP")
    }
    
    // Wait for completion with timeout
    sus timeout_ms drip = 5000  // 5 second timeout
    bestie (read_result == "" && (get_current_time_ms() - start_time) < timeout_ms) {
        sleep(10)  // 10ms sleep
    }
    
    ready (read_result == "") {
        vibez.spill("❌ CRITICAL: Async file read timed out - IOCP promise never completed")
        damn false
    } otherwise {
        vibez.spill("✅ Async file read completed in {}ms", get_current_time_ms() - start_time)
    }
    
    // Cleanup
    delete_file(test_file_path)
    damn true
}

// Test async network operations completion
slay test_async_network_completion() {
    vibez.spill("🧪 Testing Windows async network I/O completion...")
    
    // Test 1: TCP server async accept completion
    sus server_port drip = 19901
    sus client_connected lit = false
    sus server_accepted lit = false
    
    // Start async TCP server
    go {
        sus server tea = start_tcp_server("127.0.0.1", server_port) fam {
            when _ -> {
                vibez.spill("❌ Failed to start TCP server")
                damn
            }
        }
        
        vibez.spill("TCP server started on port {}", server_port)
        
        // Async accept - this should complete via IOCP
        sus client_conn tea = accept_connection_async(server) fam {
            when _ -> {
                vibez.spill("❌ Async accept failed - IOCP completion issue")
                damn
            }
        }
        
        server_accepted = true
        vibez.spill("✅ Async accept completed via IOCP")
        close_connection(client_conn)
    }
    
    // Give server time to start
    sleep(100)
    
    // Start async TCP client
    go {
        sus client tea = create_tcp_client() fam {
            when _ -> {
                vibez.spill("❌ Failed to create TCP client")
                damn
            }
        }
        
        // Async connect - this should complete via IOCP
        connect_async(client, "127.0.0.1", server_port) fam {
            when _ -> {
                vibez.spill("❌ Async connect failed - IOCP completion issue")
                damn
            }
        }
        
        client_connected = true
        vibez.spill("✅ Async connect completed via IOCP")
        close_connection(client)
    }
    
    // Wait for both operations to complete with timeout
    sus start_time drip = get_current_time_ms()
    sus timeout_ms drip = 10000  // 10 second timeout
    
    bestie (!client_connected || !server_accepted) {
        ready ((get_current_time_ms() - start_time) > timeout_ms) {
            vibez.spill("❌ CRITICAL: Network async operations timed out - IOCP promises never completed")
            vibez.spill("   Client connected: {}, Server accepted: {}", client_connected, server_accepted)
            damn false
        }
        sleep(10)
    }
    
    vibez.spill("✅ All network async operations completed in {}ms", get_current_time_ms() - start_time)
    damn true
}

// Test async timer operations completion
slay test_async_timer_completion() {
    vibez.spill("🧪 Testing Windows async timer operations completion...")
    
    // Test 1: Async timer completion
    sus timer_completed lit = false
    sus timer_start_time drip = get_current_time_ms()
    sus timer_delay_ms drip = 500  // 500ms delay
    
    go {
        // Async timer operation
        sleep_async(timer_delay_ms) fam {
            when _ -> {
                vibez.spill("❌ Async timer failed - IOCP completion issue")
                damn
            }
        }
        
        timer_completed = true
        vibez.spill("✅ Async timer completed via IOCP")
    }
    
    // Wait for timer completion with timeout
    sus timeout_ms drip = 2000  // 2 second timeout
    
    bestie (!timer_completed) {
        sus elapsed drip = get_current_time_ms() - timer_start_time
        ready (elapsed > timeout_ms) {
            vibez.spill("❌ CRITICAL: Async timer timed out - IOCP promise never completed")
            damn false
        }
        sleep(10)
    }
    
    sus actual_delay drip = get_current_time_ms() - timer_start_time
    vibez.spill("✅ Async timer completed in {}ms (expected ~{}ms)", actual_delay, timer_delay_ms)
    
    // Verify timing is approximately correct
    ready (actual_delay < (timer_delay_ms - 50) || actual_delay > (timer_delay_ms + 200)) {
        vibez.spill("❌ WARNING: Timer completion timing is off - IOCP timing issues")
        damn false
    }
    
    damn true
}

// Test concurrent async operations to stress test IOCP
slay test_concurrent_async_operations() {
    vibez.spill("🧪 Testing concurrent Windows async operations...")
    
    sus num_operations drip = 20
    sus completed_operations drip = 0
    sus failed_operations drip = 0
    
    // Start multiple concurrent async operations
    bestie (drip i = 0; i < num_operations; i++) {
        go {
            sus op_id drip = i
            
            // Mix of different async operations
            ready (i % 3 == 0) {
                // File I/O operation
                sus test_file tea = format("test_concurrent_{}.txt", op_id)
                sus test_data tea = format("Concurrent test data {}", op_id)
                
                write_file_async(test_file, test_data) fam {
                    when _ -> {
                        failed_operations = failed_operations + 1
                        damn
                    }
                }
                
                read_file_async(test_file) fam {
                    when _ -> {
                        failed_operations = failed_operations + 1
                        damn
                    }
                }
                
                delete_file(test_file)
            } ready (i % 3 == 1) {
                // Timer operation
                sleep_async(100 + (i * 10)) fam {
                    when _ -> {
                        failed_operations = failed_operations + 1
                        damn
                    }
                }
            } otherwise {
                // Network operation (just create/close socket)
                sus client tea = create_tcp_client() fam {
                    when _ -> {
                        failed_operations = failed_operations + 1
                        damn
                    }
                }
                close_connection(client)
            }
            
            completed_operations = completed_operations + 1
            vibez.spill("Operation {} completed", op_id)
        }
    }
    
    // Wait for all operations to complete with timeout
    sus start_time drip = get_current_time_ms()
    sus timeout_ms drip = 30000  // 30 second timeout
    
    bestie (completed_operations + failed_operations < num_operations) {
        ready ((get_current_time_ms() - start_time) > timeout_ms) {
            vibez.spill("❌ CRITICAL: Concurrent operations timed out")
            vibez.spill("   Completed: {}/{}, Failed: {}", completed_operations, num_operations, failed_operations)
            damn false
        }
        sleep(50)
    }
    
    sus total_time drip = get_current_time_ms() - start_time
    vibez.spill("✅ Concurrent operations completed in {}ms", total_time)
    vibez.spill("   Completed: {}, Failed: {}", completed_operations, failed_operations)
    
    // Allow some failures but not too many
    ready (failed_operations > (num_operations / 4)) {
        vibez.spill("❌ Too many concurrent operations failed - IOCP issues detected")
        damn false
    }
    
    damn true
}

// Test IOCP error handling and recovery
slay test_iocp_error_recovery() {
    vibez.spill("🧪 Testing Windows IOCP error handling and recovery...")
    
    // Test 1: Invalid file operation should complete with error
    sus error_handled lit = false
    
    go {
        // Try to read non-existent file - should complete with error, not hang
        sus result tea = read_file_async("/nonexistent/path/file.txt") fam {
            when _ -> {
                error_handled = true
                vibez.spill("✅ File error properly handled via IOCP")
                damn
            }
        }
        
        vibez.spill("❌ CRITICAL: Invalid file read succeeded when it should have failed")
    }
    
    // Wait for error handling with timeout
    sus start_time drip = get_current_time_ms()
    sus timeout_ms drip = 5000
    
    bestie (!error_handled) {
        ready ((get_current_time_ms() - start_time) > timeout_ms) {
            vibez.spill("❌ CRITICAL: Error operation timed out - IOCP error promises never completed")
            damn false
        }
        sleep(10)
    }
    
    // Test 2: Invalid network operation should complete with error
    sus network_error_handled lit = false
    
    go {
        // Try to connect to invalid address - should complete with error
        sus client tea = create_tcp_client() fam {
            when _ -> {
                vibez.spill("❌ Failed to create client for error test")
                damn
            }
        }
        
        connect_async(client, "192.0.2.1", 12345) fam {  // RFC5737 test address
            when _ -> {
                network_error_handled = true
                vibez.spill("✅ Network error properly handled via IOCP")
                close_connection(client)
                damn
            }
        }
        
        vibez.spill("❌ CRITICAL: Invalid network connect succeeded when it should have failed")
        close_connection(client)
    }
    
    // Wait for network error handling
    start_time = get_current_time_ms()
    
    bestie (!network_error_handled) {
        ready ((get_current_time_ms() - start_time) > timeout_ms) {
            vibez.spill("❌ CRITICAL: Network error operation timed out - IOCP error promises never completed")
            damn false
        }
        sleep(10)
    }
    
    vibez.spill("✅ All error operations completed properly via IOCP")
    damn true
}

// Main test runner
slay main() {
    test_start("Windows IOCP Async Promise Completion Fix Validation")
    
    vibez.spill("🔍 Validating Windows IOCP (I/O Completion Ports) integration...")
    vibez.spill("This test identifies why async promises are not completing and validates fixes")
    vibez.spill("")
    
    sus all_tests_passed lit = true
    
    // Run individual test suites
    ready (!test_async_file_completion()) {
        all_tests_passed = false
        vibez.spill("❌ File async completion tests FAILED")
    }
    
    ready (!test_async_network_completion()) {
        all_tests_passed = false
        vibez.spill("❌ Network async completion tests FAILED")
    }
    
    ready (!test_async_timer_completion()) {
        all_tests_passed = false
        vibez.spill("❌ Timer async completion tests FAILED")
    }
    
    ready (!test_concurrent_async_operations()) {
        all_tests_passed = false
        vibez.spill("❌ Concurrent async operations tests FAILED")
    }
    
    ready (!test_iocp_error_recovery()) {
        all_tests_passed = false
        vibez.spill("❌ IOCP error recovery tests FAILED")
    }
    
    vibez.spill("")
    ready (all_tests_passed) {
        vibez.spill("🎉 ALL WINDOWS IOCP TESTS PASSED!")
        vibez.spill("✅ Async promises complete properly")
        vibez.spill("✅ File I/O operations work correctly")
        vibez.spill("✅ Network operations work correctly") 
        vibez.spill("✅ Timer operations work correctly")
        vibez.spill("✅ Error handling works correctly")
        vibez.spill("✅ Concurrent operations work correctly")
    } otherwise {
        vibez.spill("❌ WINDOWS IOCP INTEGRATION HAS ISSUES!")
        vibez.spill("⚠️  Async promises are not completing properly")
        vibez.spill("⚠️  IOCP completion port integration needs fixes")
    }
    
    print_test_summary()
    damn 0
}
