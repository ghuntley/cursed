fr fr Test file for dropz file manipulation functions

vibe main

yeet "dropz"
yeet "vibez"
yeet "vibe_life"

fr fr Test file paths for the tests
sus TEST_FILE_PATH tea = "test_dropz_file.txt"
sus NON_EXISTENT_FILE tea = "does_not_exist.txt"

fr fr Test if a file exists
slay test_file_exists() lit {
    fr fr Write something to the file first to ensure it exists
    dropz.write_file(TEST_FILE_PATH, "Test content for file exists check")
    
    sus exists lit = dropz.file_exists(TEST_FILE_PATH)
    sus non_exists lit = dropz.file_exists(NON_EXISTENT_FILE)
    
    yolo exists
}

fr fr Test if a file is readable
slay test_is_readable() lit {
    fr fr Create the file first
    dropz.write_file(TEST_FILE_PATH, "Test content for readability check")
    
    sus readable lit = dropz.is_readable(TEST_FILE_PATH)
    sus non_readable lit = dropz.is_readable(NON_EXISTENT_FILE)
    
    yolo readable && !non_readable
}

fr fr Test if a file is writable
slay test_is_writable() lit {
    fr fr Create the file first
    dropz.write_file(TEST_FILE_PATH, "Test content for writability check")
    
    sus writable lit = dropz.is_writable(TEST_FILE_PATH)
    sus non_writable lit = dropz.is_writable(NON_EXISTENT_FILE)
    
    yolo writable && !non_writable
}

fr fr Test getting file information
slay test_file_info() lit {
    fr fr Create file with known content
    sus content tea = "Test content for file info check"
    dropz.write_file(TEST_FILE_PATH, content)
    
    sus info tea = dropz.file_info(TEST_FILE_PATH)
    sus has_info lit = info.len() > 0
    
    fr fr Try to get info for non-existent file
    sus non_info tea = dropz.file_info(NON_EXISTENT_FILE)
    sus has_error lit = non_info.len() == 0
    
    yolo has_info && has_error
}

fr fr Test removing a file
slay test_remove_file() lit {
    fr fr Create file first
    dropz.write_file(TEST_FILE_PATH, "Test content for remove check")
    
    fr fr Verify file exists before removal
    sus exists_before lit = dropz.file_exists(TEST_FILE_PATH)
    
    fr fr Remove the file
    dropz.remove_file(TEST_FILE_PATH)
    
    fr fr Verify file no longer exists
    sus exists_after lit = dropz.file_exists(TEST_FILE_PATH)
    
    yolo exists_before && !exists_after
}

fr fr Test appending to a file
slay test_append_file() lit {
    fr fr Create initial file
    sus initial_content tea = "Initial content\n"
    dropz.write_file(TEST_FILE_PATH, initial_content)
    
    fr fr Append to the file
    sus append_content tea = "Appended content"
    dropz.append_file(TEST_FILE_PATH, append_content)
    
    fr fr Read file and check if it contains both contents
    sus content tea = dropz.read_file_string(TEST_FILE_PATH)
    sus has_initial lit = content.contains(initial_content)
    sus has_append lit = content.contains(append_content)
    
    yolo has_initial && has_append
}

fr fr Run all tests
slay main() {
    vibez.spill("Running dropz file manipulation tests...")
    
    sus tests = 0
    sus passed = 0
    
    tests++
    lowkey test_file_exists() {
        vibez.spill("u2705 file_exists test passed")
        passed++
    } highkey {
        vibez.spill("u274c file_exists test failed")
    }
    
    tests++
    lowkey test_is_readable() {
        vibez.spill("u2705 is_readable test passed")
        passed++
    } highkey {
        vibez.spill("u274c is_readable test failed")
    }
    
    tests++
    lowkey test_is_writable() {
        vibez.spill("u2705 is_writable test passed")
        passed++
    } highkey {
        vibez.spill("u274c is_writable test failed")
    }
    
    tests++
    lowkey test_file_info() {
        vibez.spill("u2705 file_info test passed")
        passed++
    } highkey {
        vibez.spill("u274c file_info test failed")
    }
    
    tests++
    lowkey test_remove_file() {
        vibez.spill("u2705 remove_file test passed")
        passed++
    } highkey {
        vibez.spill("u274c remove_file test failed")
    }
    
    tests++
    lowkey test_append_file() {
        vibez.spill("u2705 append_file test passed")
        passed++
    } highkey {
        vibez.spill("u274c append_file test failed")
    }
    
    fr fr Clean up test file if it exists
    lowkey dropz.file_exists(TEST_FILE_PATH) {
        dropz.remove_file(TEST_FILE_PATH)
    }
    
    vibez.spill("Tests complete!")
    vibez.spillf("Passed %d of %d tests", passed, tests)
}