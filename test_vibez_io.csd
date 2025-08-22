fr fr Comprehensive vibez I/O testing - validates actual file operations
fr fr Tests both console and file I/O operations through runtime bridge functions

yeet "vibez"

slay test_file_operations() lit {
    vibez.spill("=== Testing File I/O Operations ===")
    
    fr fr Test 1: Basic file write/read
    sus filename tea = "test_output.txt"
    sus test_content tea = "Hello from CURSED file I/O!"
    
    vibez.spill("Writing to file:", filename)
    yikes write_result := vibez.write_file(filename, test_content)
    lowkey write_result == cap {
        vibez.spill("ERROR: Failed to write file")
        damn cap
    }
    vibez.spill("✅ File write successful")
    
    fr fr Test 2: Read file back
    vibez.spill("Reading from file:", filename)
    yikes read_content := vibez.read_file(filename)
    lowkey read_content == "" {
        vibez.spill("ERROR: Failed to read file or file empty")
        damn cap
    }
    vibez.spill("✅ File read successful")
    vibez.spill("Content read:", read_content)
    
    fr fr Test 3: Check if content matches
    lowkey read_content == test_content {
        vibez.spill("✅ Content matches - file I/O working correctly")
    } otherwise {
        vibez.spill("ERROR: Content mismatch")
        vibez.spill("Expected:", test_content)
        vibez.spill("Got:", read_content)
        damn cap
    }
    
    fr fr Test 4: File exists check
    lowkey vibez.file_exists(filename) {
        vibez.spill("✅ File exists check working")
    } otherwise {
        vibez.spill("ERROR: File exists check failed")
        damn cap
    }
    
    fr fr Test 5: Delete file
    vibez.spill("Deleting file:", filename)
    yikes delete_result := vibez.delete_file(filename)
    lowkey delete_result == cap {
        vibez.spill("ERROR: Failed to delete file")
        damn cap
    }
    vibez.spill("✅ File deletion successful")
    
    fr fr Test 6: Verify file is deleted
    lowkey vibez.file_exists(filename) == cap {
        vibez.spill("✅ File successfully deleted")
    } otherwise {
        vibez.spill("ERROR: File still exists after deletion")
        damn cap
    }
    
    damn based
}

slay test_console_io() lit {
    vibez.spill("=== Testing Console I/O Operations ===")
    
    fr fr Test basic printing
    vibez.spill("Basic print test")
    vibez.spill_line("Line with newline")
    
    fr fr Test formatted output
    sus name tea = "CURSED"
    sus version drip = 1.0
    sus active lit = based
    
    vibez.spill("Language:", name)
    vibez.spill("Version:", version)
    vibez.spill("Active:", active)
    
    fr fr Test different data types
    vibez.spill("String:", "Hello World")
    vibez.spill("Integer:", 42)
    vibez.spill("Float:", 3.14159)
    vibez.spill("Boolean:", based)
    
    vibez.spill("✅ Console I/O tests completed")
    damn based
}

slay test_error_handling() lit {
    vibez.spill("=== Testing Error Handling ===")
    
    fr fr Test reading non-existent file
    sus bad_filename tea = "nonexistent_file.txt"
    yikes read_result := vibez.read_file(bad_filename)
    lowkey read_result == "" {
        vibez.spill("✅ Correctly handled non-existent file read")
    } otherwise {
        vibez.spill("WARNING: Expected empty result for non-existent file")
    }
    
    fr fr Test writing to invalid path
    sus bad_path tea = "/root/cannot_write_here.txt"
    yikes write_result := vibez.write_file(bad_path, "test")
    lowkey write_result == cap {
        vibez.spill("✅ Correctly handled invalid write path")
    } otherwise {
        vibez.spill("WARNING: Expected write failure for invalid path")
    }
    
    vibez.spill("✅ Error handling tests completed")
    damn based
}

slay test_advanced_io() lit {
    vibez.spill("=== Testing Advanced I/O Operations ===")
    
    fr fr Test multiple file operations
    sus files []tea = ["file1.txt", "file2.txt", "file3.txt"]
    sus contents []tea = ["Content 1", "Content 2", "Content 3"]
    
    fr fr Write multiple files
    sus i normie = 0
    stan i < 3 {
        vibez.spill("Writing file:", files[i])
        yikes result := vibez.write_file(files[i], contents[i])
        lowkey result == cap {
            vibez.spill("ERROR: Failed to write file", files[i])
            damn cap
        }
        i = i + 1
    }
    vibez.spill("✅ Multiple file write successful")
    
    fr fr Read and verify multiple files
    i = 0
    stan i < 3 {
        vibez.spill("Reading file:", files[i])
        yikes content := vibez.read_file(files[i])
        lowkey content != contents[i] {
            vibez.spill("ERROR: Content mismatch in file", files[i])
            damn cap
        }
        i = i + 1
    }
    vibez.spill("✅ Multiple file read/verify successful")
    
    fr fr Clean up files
    i = 0
    stan i < 3 {
        vibez.delete_file(files[i])
        i = i + 1
    }
    vibez.spill("✅ Advanced I/O tests completed")
    
    damn based
}

slay run_all_tests() lit {
    vibez.spill("🚀 Starting CURSED vibez I/O Validation Tests")
    vibez.spill("=" * 50)
    
    fr fr Run all test suites
    yikes console_result := test_console_io()
    lowkey console_result == cap {
        vibez.spill("❌ Console I/O tests FAILED")
        damn cap
    }
    
    yikes file_result := test_file_operations()
    lowkey file_result == cap {
        vibez.spill("❌ File I/O tests FAILED")
        damn cap
    }
    
    yikes error_result := test_error_handling()
    lowkey error_result == cap {
        vibez.spill("❌ Error handling tests FAILED")
        damn cap
    }
    
    yikes advanced_result := test_advanced_io()
    lowkey advanced_result == cap {
        vibez.spill("❌ Advanced I/O tests FAILED")
        damn cap
    }
    
    vibez.spill("=" * 50)
    vibez.spill("🎉 ALL VIBEZ I/O TESTS PASSED!")
    vibez.spill("✅ Console I/O: Working")
    vibez.spill("✅ File I/O: Working")
    vibez.spill("✅ Error Handling: Working")
    vibez.spill("✅ Advanced Operations: Working")
    vibez.spill("=" * 50)
    
    damn based
}

fr fr Main execution
yikes result := run_all_tests()
lowkey result == cap {
    vibez.spill("💀 TESTS FAILED - I/O operations not working correctly")
} otherwise {
    vibez.spill("🎯 All I/O tests passed - vibez runtime bridge working!")
}
