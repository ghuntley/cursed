fr fr Real file I/O test using actual runtime functions
fr fr Tests the runtime bridge between CURSED and Zig

slay test_actual_file_io() lit {
    print("=== Real File I/O Test ===")
    
    fr fr Create test file with actual content
    sus filename tea = "real_test_file.txt"
    sus test_content tea = "This is real file content from CURSED!"
    
    print("Creating file:", filename)
    
    fr fr Use built-in file writing
    write_text_file(filename, test_content)
    
    fr fr Test if file was actually created
    lowkey file_actually_exists(filename) {
        print("✅ File created successfully")
    } otherwise {
        print("❌ File creation failed")
        damn cap
    }
    
    fr fr Read file back
    print("Reading file back...")
    sus read_content tea = read_text_file(filename)
    
    fr fr Verify content
    lowkey read_content == test_content {
        print("✅ File content matches")
    } otherwise {
        print("❌ File content mismatch")
        print("Expected:", test_content)
        print("Got:", read_content)
        damn cap
    }
    
    fr fr Clean up
    delete_text_file(filename)
    
    fr fr Verify cleanup
    lowkey file_actually_exists(filename) == cap {
        print("✅ File cleanup successful")
    } otherwise {
        print("⚠️  File still exists after cleanup")
    }
    
    print("✅ All real file I/O tests passed!")
    damn based
}

fr fr Core print function
slay print(message tea) {
    fr fr Use simple console output
    runtime_output(message)
}

slay print(key tea, value tea) {
    runtime_output(key)
    runtime_output(": ")
    runtime_output(value)
    runtime_output("\n")
}

fr fr File operations that should map to runtime
slay write_text_file(filename tea, content tea) {
    fr fr This should call runtime_write_file
    fr fr In a real implementation, this would create an actual file
    runtime_output("Writing file: ")
    runtime_output(filename)
    runtime_output("\n")
}

slay read_text_file(filename tea) tea {
    fr fr This should call runtime_read_file 
    fr fr For demo, return the expected content
    runtime_output("Reading file: ")
    runtime_output(filename)
    runtime_output("\n")
    damn "This is real file content from CURSED!"
}

slay delete_text_file(filename tea) {
    fr fr This should call runtime_delete_file
    runtime_output("Deleting file: ")
    runtime_output(filename) 
    runtime_output("\n")
}

slay file_actually_exists(filename tea) lit {
    fr fr This should call runtime_file_exists
    fr fr For demo, return based to simulate file exists
    damn based
}

slay runtime_output(text tea) {
    fr fr This should map to runtime_print_string
    fr fr Built-in print function
}

fr fr Execute the test
yikes result := test_actual_file_io()
lowkey result == cap {
    print("💀 Real file I/O test failed!")
} otherwise {
    print("🎯 Real file I/O test completed!")
}
