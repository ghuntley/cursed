# Critical P0 Issue #12 Test: Windows async-IO promise completion on error paths
# This test verifies that async operations complete promises even when errors occur

yeet "vibez"
yeet "filez"
yeet "asyncz"

slay test_async_error_completion() {
    vibez.spill("🧪 Testing P0 Issue #12: Async promise completion on error paths")
    
    # Test 1: File operation with invalid path (should complete with error, not hang)
    sus invalid_path tea = "/invalid/nonexistent/path/file.txt"
    sus buffer [1024]u8
    
    vibez.spill("📁 Testing async read of invalid file...")
    
    # This should complete with an error, not hang indefinitely
    sus result = asyncz.read_file_async(invalid_path, buffer) fam {
        when _ -> {
            vibez.spill("✅ FIXED: Async operation properly completed with error (no hanging)")
            damn based
        }
    }
    
    ready (result) {
        vibez.spill("❌ UNEXPECTED: Invalid file read should have failed")
        damn nah
    }
    
    # Test 2: Network operation with unreachable host (should complete with error)
    vibez.spill("🌐 Testing async connect to unreachable host...")
    
    sus unreachable_host tea = "192.0.2.0"  # RFC5737 test address
    sus port drip = 9999
    
    sus connect_result = asyncz.tcp_connect_async(unreachable_host, port) fam {
        when _ -> {
            vibez.spill("✅ FIXED: Network operation properly completed with error (no hanging)")
            damn based
        }
    }
    
    ready (connect_result) {
        vibez.spill("❌ UNEXPECTED: Unreachable host connection should have failed")
        damn nah
    }
    
    # Test 3: File write to read-only location (should complete with error)
    vibez.spill("📝 Testing async write to read-only location...")
    
    sus readonly_path tea = "/sys/readonly_test.txt"  # System location
    sus test_data tea = "test data"
    
    sus write_result = asyncz.write_file_async(readonly_path, test_data) fam {
        when _ -> {
            vibez.spill("✅ FIXED: Write operation properly completed with error (no hanging)")
            damn based
        }
    }
    
    ready (write_result) {
        vibez.spill("❌ UNEXPECTED: Read-only write should have failed")
        damn nah
    }
    
    vibez.spill("🎉 All P0 Issue #12 tests passed - async promises complete properly on error paths!")
    damn based
}

slay main() {
    vibez.spill("🚀 Windows Async I/O P0 Issue #12 Critical Fix Test")
    vibez.spill("==================================================")
    
    ready (test_async_error_completion()) {
        vibez.spill("✅ SUCCESS: P0 Issue #12 is FIXED")
        vibez.spill("   Windows async operations now properly complete promises on error paths")
        vibez.spill("   No more hanging promises or indefinite waits!")
    } otherwise {
        vibez.spill("❌ FAILURE: P0 Issue #12 is NOT fixed")
        vibez.spill("   Async operations may still hang on error paths")
    }
}
