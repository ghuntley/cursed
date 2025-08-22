yeet "vibez"

fr fr Direct runtime function test
slay test_file_operations() {
    vibez.spill("Testing real file I/O operations...")
    
    fr fr Write file
    sus write_success lit = runtime_write_file("real_test.txt", "Hello from CURSED!")
    ready (write_success) {
        vibez.spill("✅ File write successful")
    } otherwise {
        vibez.spill("❌ File write failed")
        damn
    }
    
    fr fr Check file exists
    ready (runtime_file_exists("real_test.txt")) {
        vibez.spill("✅ File exists")
    } otherwise {
        vibez.spill("❌ File does not exist")
        damn
    }
    
    fr fr Read file
    sus content tea = runtime_read_file("real_test.txt")
    ready (content != "ERROR") {
        vibez.spill("✅ File read successful:", content)
    } otherwise {
        vibez.spill("❌ File read failed")
    }
    
    fr fr Get file size
    sus size drip = runtime_file_size("real_test.txt")
    ready (size > 0) {
        vibez.spill("✅ File size:", size)
    } otherwise {
        vibez.spill("❌ File size failed or zero")
    }
    
    fr fr Append to file
    sus append_success lit = runtime_append_file("real_test.txt", "\nAppended content!")
    ready (append_success) {
        vibez.spill("✅ File append successful")
    } otherwise {
        vibez.spill("❌ File append failed")
    }
    
    fr fr Read final content
    sus final_content tea = runtime_read_file("real_test.txt")
    vibez.spill("Final content:", final_content)
    
    fr fr Cleanup
    ready (runtime_delete_file("real_test.txt")) {
        vibez.spill("✅ File deleted successfully")
    } otherwise {
        vibez.spill("❌ File deletion failed")
    }
}

test_file_operations()
vibez.spill("Real file I/O test complete!")
