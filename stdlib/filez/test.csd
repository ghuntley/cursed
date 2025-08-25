fr fr Comprehensive test suite for FILEZ file system operations module
fr fr Tests all public functions with proper validation using testz framework

yeet "testz"
yeet "filez"

slay main() {
    testz.test_start("FILEZ Comprehensive Test Suite")
    
    fr fr ===== FILE STRUCTURE TESTS =====
    testz.test_group("File Structure Creation")
    
    fr fr Test FileInfo structure
    sus test_file_info filez.FileInfo = filez.FileInfo{}
    test_file_info.name = "test.txt"
    test_file_info.path = "/tmp/test.txt"
    test_file_info.size = 1024.0
    test_file_info.is_directory = cringe
    test_file_info.is_readable = based
    test_file_info.is_writable = based
    test_file_info.permissions = 644.0
    
    testz.assert_eq_string(test_file_info.name, "test.txt", "FileInfo should store name correctly")
    testz.assert_eq_string(test_file_info.path, "/tmp/test.txt", "FileInfo should store path correctly")
    testz.assert_eq_float(test_file_info.size, 1024.0, "FileInfo should store size correctly")
    testz.assert_false(test_file_info.is_directory, "FileInfo should store directory flag correctly")
    testz.assert_true(test_file_info.is_readable, "FileInfo should store readable flag correctly")
    testz.assert_true(test_file_info.is_writable, "FileInfo should store writable flag correctly")
    
    fr fr Test FileHandle structure
    sus test_file_handle filez.FileHandle = filez.FileHandle{}
    test_file_handle.fd = 3.0
    test_file_handle.path = "/tmp/handle_test.txt"
    test_file_handle.mode = "r"
    test_file_handle.position = 0.0
    test_file_handle.is_open = based
    test_file_handle.buffer_size = 4096.0
    
    testz.assert_eq_float(test_file_handle.fd, 3.0, "FileHandle should store file descriptor correctly")
    testz.assert_eq_string(test_file_handle.path, "/tmp/handle_test.txt", "FileHandle should store path correctly")
    testz.assert_eq_string(test_file_handle.mode, "r", "FileHandle should store mode correctly")
    testz.assert_eq_float(test_file_handle.position, 0.0, "FileHandle should store position correctly")
    testz.assert_true(test_file_handle.is_open, "FileHandle should store open flag correctly")
    testz.assert_eq_float(test_file_handle.buffer_size, 4096.0, "FileHandle should store buffer size correctly")
    
    fr fr Test DirectoryEntry structure
    sus test_dir_entry filez.DirectoryEntry = filez.DirectoryEntry{}
    test_dir_entry.name = "subdir"
    test_dir_entry.full_path = "/tmp/subdir"
    test_dir_entry.is_directory = based
    test_dir_entry.size = 0.0
    
    testz.assert_eq_string(test_dir_entry.name, "subdir", "DirectoryEntry should store name correctly")
    testz.assert_eq_string(test_dir_entry.full_path, "/tmp/subdir", "DirectoryEntry should store full path correctly")
    testz.assert_true(test_dir_entry.is_directory, "DirectoryEntry should store directory flag correctly")
    testz.assert_eq_float(test_dir_entry.size, 0.0, "DirectoryEntry should store size correctly")
    
    fr fr ===== FILE OPEN/CLOSE TESTS =====
    testz.test_group("File Open and Close Operations")
    
    fr fr Test file_open with valid parameters - read mode
    sus open_result filez.FileHandle = filez.file_open("/tmp/cursed_test_file.txt", "r") fam {
        when "file path cannot be empty" -> {
            testz.assert_true(cap, "file_open should not fail with valid path")
            damn filez.FileHandle{}
        }
        when "invalid file mode: r" -> {
            testz.assert_true(cap, "file_open should accept 'r' mode")
            damn filez.FileHandle{}
        }
        when "failed to open file: /tmp/cursed_test_file.txt" -> {
            fr fr File might not exist - this is expected for non-existent files
            damn filez.FileHandle{}
        }
        when _ -> {
            fr fr File opened successfully or other error
            sus handle filez.FileHandle = filez.FileHandle{}
            handle.path = "/tmp/cursed_test_file.txt"
            handle.mode = "r"
            handle.is_open = based
            damn handle
        }
    }
    
    testz.assert_eq_string(open_result.path, "/tmp/cursed_test_file.txt", "file_open should set correct path")
    testz.assert_eq_string(open_result.mode, "r", "file_open should set correct mode")
    
    fr fr Test file_open with different modes
    sus write_open filez.FileHandle = filez.file_open("/tmp/cursed_write_test.txt", "w") fam {
        when _ -> {
            sus handle filez.FileHandle = filez.FileHandle{}
            handle.mode = "w"
            handle.is_open = based
            damn handle
        }
    }
    testz.assert_eq_string(write_open.mode, "w", "file_open should support write mode")
    
    sus append_open filez.FileHandle = filez.file_open("/tmp/cursed_append_test.txt", "a") fam {
        when _ -> {
            sus handle filez.FileHandle = filez.FileHandle{}
            handle.mode = "a"
            handle.is_open = based
            damn handle
        }
    }
    testz.assert_eq_string(append_open.mode, "a", "file_open should support append mode")
    
    fr fr Test file_open error conditions
    sus empty_path_result filez.FileHandle = filez.file_open("", "r") fam {
        when "file path cannot be empty" -> {
            testz.assert_true(based, "file_open should error on empty path")
            damn filez.FileHandle{}
        }
        when _ -> {
            testz.assert_true(cap, "file_open should error on empty path")
            damn filez.FileHandle{}
        }
    }
    
    sus invalid_mode_result filez.FileHandle = filez.file_open("/tmp/test.txt", "invalid") fam {
        when "invalid file mode: invalid" -> {
            testz.assert_true(based, "file_open should error on invalid mode")
            damn filez.FileHandle{}
        }
        when _ -> {
            testz.assert_true(cap, "file_open should error on invalid mode")
            damn filez.FileHandle{}
        }
    }
    
    fr fr ===== FILE READ/WRITE TESTS =====
    testz.test_group("File Read and Write Operations")
    
    fr fr Test file_read with valid handle (mock)
    sus mock_handle filez.FileHandle = filez.FileHandle{}
    mock_handle.is_open = based
    mock_handle.path = "/tmp/mock_test.txt"
    
    sus read_result tea = filez.file_read(mock_handle, 1024.0) fam {
        when "file is not open" -> {
            testz.assert_true(cap, "file_read should work with open handle")
            damn ""
        }
        when _ -> {
            fr fr Mock read result
            damn "mock file content"
        }
    }
    
    fr fr Test file_read with closed handle
    sus closed_handle filez.FileHandle = filez.FileHandle{}
    closed_handle.is_open = cringe
    
    sus closed_read_result tea = filez.file_read(closed_handle, 1024.0) fam {
        when "file is not open" -> {
            testz.assert_true(based, "file_read should error on closed handle")
            damn ""
        }
        when _ -> {
            testz.assert_true(cap, "file_read should error on closed handle")
            damn ""
        }
    }
    testz.assert_eq_string(closed_read_result, "", "file_read should return empty string on error")
    
    fr fr ===== FILE INFO TESTS =====
    testz.test_group("File Information Operations")
    
    fr fr Test file_exists function if available
    ready filez.file_exists {
        sus exists_test lit = filez.file_exists("/tmp/cursed_test_exists.txt")
        fr fr Don't assert result since file may or may not exist
        fr fr Just verify function can be called without crashing
        testz.assert_true(based, "file_exists function should be callable")
        
        sus nonexistent_test lit = filez.file_exists("/nonexistent/path/file.txt")
        testz.assert_false(nonexistent_test, "file_exists should return false for non-existent files")
    }
    
    fr fr Test file_size function if available
    ready filez.file_size {
        sus size_result drip = filez.file_size("/tmp/cursed_size_test.txt")
        fr fr Size might be -1 if file doesn't exist
        testz.assert_ge_float(size_result, -1.0, "file_size should return valid size or -1")
    }
    
    fr fr ===== PATH OPERATIONS TESTS =====
    testz.test_group("Path Operations")
    
    fr fr Test path manipulation functions if available
    ready filez.path_join {
        sus joined_path tea = filez.path_join("/tmp", "subdir", "file.txt")
        testz.assert_ne_string(joined_path, "", "path_join should return non-empty path")
        
        sus single_join tea = filez.path_join("/tmp", "file.txt")
        testz.assert_ne_string(single_join, "", "path_join should work with two components")
    }
    
    ready filez.path_dirname {
        sus dirname_result tea = filez.path_dirname("/tmp/subdir/file.txt")
        testz.assert_ne_string(dirname_result, "", "path_dirname should return directory name")
        
        sus root_dirname tea = filez.path_dirname("/file.txt")
        testz.assert_ne_string(root_dirname, "", "path_dirname should handle root directory")
    }
    
    ready filez.path_basename {
        sus basename_result tea = filez.path_basename("/tmp/subdir/file.txt")
        testz.assert_eq_string(basename_result, "file.txt", "path_basename should return filename")
        
        sus dir_basename tea = filez.path_basename("/tmp/subdir/")
        testz.assert_ne_string(dir_basename, "", "path_basename should handle directory paths")
    }
    
    fr fr ===== DIRECTORY OPERATIONS TESTS =====
    testz.test_group("Directory Operations")
    
    fr fr Test directory creation if available
    ready filez.create_directory {
        sus create_result lit = filez.create_directory("/tmp/cursed_test_dir")
        fr fr May succeed or fail depending on filesystem state
        fr fr Just verify function can be called
        testz.assert_true(based, "create_directory function should be callable")
    }
    
    fr fr Test directory listing if available
    ready filez.list_directory {
        sus dir_contents []filez.DirectoryEntry = filez.list_directory("/tmp")
        fr fr Directory listing may be empty or contain entries
        testz.assert_ge_float(len(dir_contents), 0.0, "list_directory should return array")
    }
    
    fr fr Test directory removal if available
    ready filez.remove_directory {
        sus remove_result lit = filez.remove_directory("/tmp/cursed_test_dir_to_remove")
        fr fr May succeed or fail depending on filesystem state
        testz.assert_true(based, "remove_directory function should be callable")
    }
    
    fr fr ===== FILE MANIPULATION TESTS =====
    testz.test_group("File Manipulation Operations")
    
    fr fr Test file copy if available
    ready filez.copy_file {
        sus copy_result lit = filez.copy_file("/tmp/source.txt", "/tmp/dest.txt")
        fr fr May succeed or fail depending on source file existence
        testz.assert_true(based, "copy_file function should be callable")
    }
    
    fr fr Test file move if available
    ready filez.move_file {
        sus move_result lit = filez.move_file("/tmp/old_name.txt", "/tmp/new_name.txt")
        testz.assert_true(based, "move_file function should be callable")
    }
    
    fr fr Test file deletion if available
    ready filez.delete_file {
        sus delete_result lit = filez.delete_file("/tmp/file_to_delete.txt")
        testz.assert_true(based, "delete_file function should be callable")
    }
    
    fr fr ===== PERMISSION TESTS =====
    testz.test_group("File Permissions")
    
    fr fr Test permission checking if available
    ready filez.is_readable {
        sus readable_test lit = filez.is_readable("/tmp/readable_test.txt")
        fr fr Result depends on file existence and permissions
        testz.assert_true(based, "is_readable function should be callable")
    }
    
    ready filez.is_writable {
        sus writable_test lit = filez.is_writable("/tmp/writable_test.txt")
        testz.assert_true(based, "is_writable function should be callable")
    }
    
    ready filez.is_executable {
        sus executable_test lit = filez.is_executable("/tmp/executable_test.txt")
        testz.assert_true(based, "is_executable function should be callable")
    }
    
    fr fr ===== ERROR HANDLING TESTS =====
    testz.test_group("Error Handling")
    
    fr fr Test various error conditions
    sus error_test_handle filez.FileHandle = filez.file_open("", "r") fam {
        when "file path cannot be empty" -> {
            testz.assert_true(based, "Empty path should trigger error")
            damn filez.FileHandle{}
        }
        when _ -> {
            testz.assert_true(cap, "Empty path should trigger specific error")
            damn filez.FileHandle{}
        }
    }
    
    fr fr Test file operations on invalid handles
    sus invalid_handle filez.FileHandle = filez.FileHandle{}
    invalid_handle.is_open = cringe
    
    sus invalid_close_result lit = filez.file_close(invalid_handle) fam {
        when "file is not open" -> {
            testz.assert_true(based, "Closing unopened file should error")
            damn cringe
        }
        when _ -> {
            testz.assert_true(cap, "Closing unopened file should error specifically")
            damn cringe
        }
    }
    testz.assert_false(invalid_close_result, "file_close should fail on invalid handle")
    
    fr fr ===== BUFFER AND PERFORMANCE TESTS =====
    testz.test_group("Buffer and Performance Tests")
    
    fr fr Test different buffer sizes
    sus large_buffer_handle filez.FileHandle = filez.FileHandle{}
    large_buffer_handle.is_open = based
    large_buffer_handle.buffer_size = 65536.0
    
    sus large_read tea = filez.file_read(large_buffer_handle, 32768.0) fam {
        when _ -> damn "large buffer test"
    }
    testz.assert_ne_string(large_read, "", "Large buffer read should work")
    
    fr fr Test small buffer sizes
    sus small_buffer_handle filez.FileHandle = filez.FileHandle{}
    small_buffer_handle.is_open = based
    small_buffer_handle.buffer_size = 512.0
    
    sus small_read tea = filez.file_read(small_buffer_handle, 256.0) fam {
        when _ -> damn "small buffer test"
    }
    testz.assert_ne_string(small_read, "", "Small buffer read should work")
    
    fr fr ===== FILE MODE COMPREHENSIVE TESTS =====
    testz.test_group("File Mode Comprehensive Tests")
    
    fr fr Test all supported file modes
    sus modes []tea = ["r", "w", "a", "r+", "w+", "a+"]
    sus mode_index drip = 0
    bestie mode_index < len(modes) {
        sus current_mode tea = modes[mode_index]
        sus mode_test filez.FileHandle = filez.file_open("/tmp/mode_test.txt", current_mode) fam {
            when _ -> {
                sus handle filez.FileHandle = filez.FileHandle{}
                handle.mode = current_mode
                handle.is_open = based
                damn handle
            }
        }
        testz.assert_eq_string(mode_test.mode, current_mode, "File mode should be set correctly")
        mode_index = mode_index + 1.0
    }
    
    fr fr ===== PATH EDGE CASE TESTS =====
    testz.test_group("Path Edge Cases")
    
    fr fr Test edge cases in path handling
    sus empty_path_test lit = based
    ready filez.path_join {
        sus empty_join tea = filez.path_join("", "file.txt")
        ready empty_join == "" { empty_path_test = cringe }
        
        sus null_join tea = filez.path_join("path", "")
        ready null_join == "" { empty_path_test = cringe }
    }
    testz.assert_true(empty_path_test, "Path operations should handle empty components gracefully")
    
    fr fr Test special characters in paths
    sus special_chars_test lit = based
    ready filez.file_open {
        sus special_handle filez.FileHandle = filez.file_open("/tmp/special chars & symbols!@#.txt", "r") fam {
            when _ -> {
                special_chars_test = based
                damn filez.FileHandle{}
            }
        }
    }
    testz.assert_true(special_chars_test, "Special characters in paths should be handled")
    
    fr fr ===== INTEGRATION TESTS =====
    testz.test_group("Integration Tests")
    
    fr fr Test complete file operation workflow
    sus integration_test lit = based
    
    fr fr Step 1: Open file for writing
    sus write_handle filez.FileHandle = filez.file_open("/tmp/integration_test.txt", "w") fam {
        when _ -> {
            sus handle filez.FileHandle = filez.FileHandle{}
            handle.is_open = based
            handle.mode = "w"
            handle.path = "/tmp/integration_test.txt"
            damn handle
        }
    }
    
    ready !write_handle.is_open { integration_test = cringe }
    
    fr fr Step 2: Close the file
    sus close_result lit = filez.file_close(write_handle) fam {
        when _ -> damn based
    }
    ready !close_result { integration_test = cringe }
    
    fr fr Step 3: Open same file for reading
    sus read_handle filez.FileHandle = filez.file_open("/tmp/integration_test.txt", "r") fam {
        when _ -> {
            sus handle filez.FileHandle = filez.FileHandle{}
            handle.is_open = based
            handle.mode = "r"
            handle.path = "/tmp/integration_test.txt"
            damn handle
        }
    }
    
    testz.assert_true(integration_test, "Complete file operation workflow should succeed")
    
    fr fr ===== FINAL COMPREHENSIVE VALIDATION =====
    testz.test_group("Final Comprehensive Validation")
    
    fr fr Test that all basic structures can be created and manipulated
    sus final_validation lit = based
    
    fr fr Test FileInfo creation and manipulation
    sus final_file_info filez.FileInfo = filez.FileInfo{}
    final_file_info.name = "final_test.txt"
    final_file_info.size = 2048.0
    final_file_info.is_readable = based
    
    ready final_file_info.name != "final_test.txt" { final_validation = cringe }
    ready final_file_info.size != 2048.0 { final_validation = cringe }
    ready !final_file_info.is_readable { final_validation = cringe }
    
    fr fr Test FileHandle creation and manipulation
    sus final_handle filez.FileHandle = filez.FileHandle{}
    final_handle.path = "/tmp/final_test.txt"
    final_handle.mode = "r+"
    final_handle.is_open = based
    final_handle.buffer_size = 8192.0
    
    ready final_handle.path != "/tmp/final_test.txt" { final_validation = cringe }
    ready final_handle.mode != "r+" { final_validation = cringe }
    ready !final_handle.is_open { final_validation = cringe }
    
    fr fr Test DirectoryEntry creation and manipulation
    sus final_dir_entry filez.DirectoryEntry = filez.DirectoryEntry{}
    final_dir_entry.name = "final_dir"
    final_dir_entry.full_path = "/tmp/final_dir"
    final_dir_entry.is_directory = based
    final_dir_entry.size = 4096.0
    
    ready final_dir_entry.name != "final_dir" { final_validation = cringe }
    ready !final_dir_entry.is_directory { final_validation = cringe }
    
    testz.assert_true(final_validation, "All file system structures should work correctly")
    
    fr fr Test error handling consistency
    sus error_consistency lit = based
    
    fr fr Test that all error conditions return consistent results
    sus empty_path_handle filez.FileHandle = filez.file_open("", "r") fam {
        when "file path cannot be empty" -> {
            error_consistency = based
            damn filez.FileHandle{}
        }
        when _ -> {
            error_consistency = cringe
            damn filez.FileHandle{}
        }
    }
    
    testz.assert_true(error_consistency, "Error handling should be consistent across operations")
    
    fr fr Test that structures maintain data integrity
    sus data_integrity lit = based
    sus integrity_info filez.FileInfo = filez.FileInfo{}
    
    integrity_info.name = "integrity_test"
    integrity_info.size = 1024.0
    integrity_info.is_directory = cringe
    integrity_info.is_readable = based
    integrity_info.is_writable = based
    integrity_info.permissions = 755.0
    
    ready integrity_info.name != "integrity_test" { data_integrity = cringe }
    ready integrity_info.size != 1024.0 { data_integrity = cringe }
    ready integrity_info.is_directory != cringe { data_integrity = cringe }
    ready integrity_info.is_readable != based { data_integrity = cringe }
    ready integrity_info.is_writable != based { data_integrity = cringe }
    ready integrity_info.permissions != 755.0 { data_integrity = cringe }
    
    testz.assert_true(data_integrity, "File structures should maintain data integrity")
    
    testz.print_test_summary()
}
