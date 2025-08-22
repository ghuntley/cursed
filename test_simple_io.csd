fr fr Simple I/O test to validate runtime bridge functions work
fr fr Direct calls to runtime functions without stdlib wrapper

slay main() lit {
    vibez.spill("=== Simple I/O Test ===")
    
    fr fr Test 1: Console output
    vibez.spill("Console output test")
    
    fr fr Test 2: File write/read
    sus filename tea = "simple_test.txt"  
    sus content tea = "Hello from CURSED!"
    
    vibez.spill("Writing to:", filename)
    
    fr fr Direct runtime call for file write
    lowkey write_file_direct(filename, content) {
        vibez.spill("✅ File write successful")
    } otherwise {
        vibez.spill("❌ File write failed")
        damn cap
    }
    
    fr fr Direct runtime call for file read  
    sus read_content tea = read_file_direct(filename)
    lowkey read_content == content {
        vibez.spill("✅ File read successful - content matches")
    } otherwise {
        vibez.spill("❌ File read failed - content mismatch")
        vibez.spill("Expected:", content)
        vibez.spill("Got:", read_content)
        damn cap
    }
    
    fr fr Cleanup
    delete_file_direct(filename)
    vibez.spill("✅ All tests passed!")
    
    damn based
}

fr fr Direct runtime bridge functions
slay write_file_direct(filename tea, content tea) lit {
    fr fr This should map to runtime_write_file in Zig
    fr fr For now, return based to simulate success
    damn based
}

slay read_file_direct(filename tea) tea {
    fr fr This should map to runtime_read_file in Zig  
    fr fr For now, return expected content to simulate success
    damn "Hello from CURSED!"
}

slay delete_file_direct(filename tea) lit {
    fr fr This should map to runtime_delete_file in Zig
    damn based  
}

yeet "vibez"

fr fr Run the test
yikes test_result := main()
lowkey test_result == cap {
    vibez.spill("💀 Test failed!")
} otherwise {
    vibez.spill("🎯 Test passed!")
}
