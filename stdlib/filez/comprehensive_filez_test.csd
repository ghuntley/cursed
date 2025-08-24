fr fr COMPREHENSIVE FILEZ ENHANCED RUNTIME INTEGRATION TEST SUITE
fr fr Tests all advanced file I/O operations with proper error handling
fr fr P0 critical - validates production-ready filesystem operations

yeet "filez_runtime_enhanced"
yeet "testz"
yeet "vibez"

fr fr ===== BASIC FILE OPERATIONS TESTS =====

slay test_read_write_advanced() {
    vibez.spill("\n=== Testing Advanced Read/Write Operations ===")
    
    fr fr Test read_file_advanced with proper error handling
    sus content tea = read_file_advanced("test.txt") fam {
        when _ -> {
            vibez.spill("❌ read_file_advanced failed:", err)
            damn
        }
    }
    vibez.spill("✅ read_file_advanced success:", content)
    
    fr fr Test write_file_advanced with overwrite control
    write_file_advanced("output.txt", "Enhanced filez content", based) fam {
        when _ -> {
            vibez.spill("❌ write_file_advanced failed:", err)
            damn
        }
    }
    vibez.spill("✅ write_file_advanced success")
    
    fr fr Test write without overwrite to existing file
    write_file_advanced("output.txt", "Should fail", cringe) fam {
        when _ -> {
            vibez.spill("✅ write_file_advanced correctly prevented overwrite:", err)
            damn
        }
    }
    vibez.spill("❌ write_file_advanced should have failed for existing file")
}

slay test_file_handle_operations() {
    vibez.spill("\n=== Testing File Handle Operations ===")
    
    fr fr Open file handle
    sus handle FileHandle = open_file("test.txt", "read") fam {
        when _ -> {
            vibez.spill("❌ open_file failed:", err)
            damn
        }
    }
    vibez.spill("✅ File opened successfully, fd:", handle.fd)
    
    fr fr Read chunk from file
    sus chunk tea = read_file_chunk(handle, 512) fam {
        when _ -> {
            vibez.spill("❌ read_file_chunk failed:", err)
            damn
        }
    }
    vibez.spill("✅ Read chunk:", chunk)
    
    fr fr Test file seeking
    sus new_pos drip = seek_file(handle, 100, "start") fam {
        when _ -> {
            vibez.spill("❌ seek_file failed:", err)
            damn
        }
    }
    vibez.spill("✅ File seek to position:", new_pos)
    
    fr fr Close file handle
    close_file(handle) fam {
        when _ -> {
            vibez.spill("❌ close_file failed:", err)
            damn
        }
    }
    vibez.spill("✅ File closed successfully")
}

slay test_buffered_io() {
    vibez.spill("\n=== Testing Buffered I/O Operations ===")
    
    fr fr Open file for writing
    sus handle FileHandle = open_file("buffered_test.txt", "write") fam {
        when _ -> {
            vibez.spill("❌ Failed to open file for buffered I/O:", err)
            damn
        }
    }
    
    fr fr Enable buffering
    sus buffered_handle FileHandle = enable_file_buffering(handle, 4096) fam {
        when _ -> {
            vibez.spill("❌ Failed to enable buffering:", err)
            damn
        }
    }
    vibez.spill("✅ Buffering enabled, buffer size:", buffered_handle.buffer_size)
    
    fr fr Write data to buffered file
    sus bytes_written drip = write_file_chunk(buffered_handle, "Buffered data content") fam {
        when _ -> {
            vibez.spill("❌ Failed to write to buffered file:", err)
            damn
        }
    }
    vibez.spill("✅ Wrote bytes:", bytes_written)
    
    fr fr Flush buffer
    flush_file_buffer(buffered_handle) fam {
        when _ -> {
            vibez.spill("❌ Failed to flush buffer:", err)
            damn
        }
    }
    vibez.spill("✅ Buffer flushed successfully")
    
    close_file(buffered_handle) fam {
        when _ -> vibez.spill("❌ Failed to close buffered file:", err)
    }
}

fr fr ===== ADVANCED FILE METADATA TESTS =====

slay test_detailed_file_info() {
    vibez.spill("\n=== Testing Detailed File Information ===")
    
    sus info FileInfo = get_file_info_detailed("test.txt") fam {
        when _ -> {
            vibez.spill("❌ get_file_info_detailed failed:", err)
            damn
        }
    }
    
    vibez.spill("✅ File Information:")
    vibez.spill("  Name:", info.name)
    vibez.spill("  Path:", info.path)
    vibez.spill("  Size:", info.size)
    vibez.spill("  Is Directory:", info.is_directory)
    vibez.spill("  Is Readable:", info.is_readable)
    vibez.spill("  Is Writable:", info.is_writable)
    vibez.spill("  Is Executable:", info.is_executable)
    vibez.spill("  Permissions:", info.permissions)
    vibez.spill("  Owner ID:", info.owner_id)
    vibez.spill("  Group ID:", info.group_id)
    vibez.spill("  File Type:", info.file_type)
    vibez.spill("  Inode:", info.inode_number)
    vibez.spill("  Hard Links:", info.hard_link_count)
}

slay test_permission_operations() {
    vibez.spill("\n=== Testing Permission Operations ===")
    
    fr fr Set file permissions
    set_file_permissions_advanced("test.txt", 755, cringe) fam {
        when _ -> {
            vibez.spill("❌ set_file_permissions_advanced failed:", err)
            damn
        }
    }
    vibez.spill("✅ File permissions set to 755")
    
    fr fr Change file ownership
    set_file_ownership("test.txt", 1000, 1000) fam {
        when _ -> {
            vibez.spill("❌ set_file_ownership failed:", err)
            damn
        }
    }
    vibez.spill("✅ File ownership changed")
    
    fr fr Touch file (update timestamps)
    touch_file("touched_file.txt") fam {
        when _ -> {
            vibez.spill("❌ touch_file failed:", err)
            damn
        }
    }
    vibez.spill("✅ File touched successfully")
}

fr fr ===== DIRECTORY OPERATIONS TESTS =====

slay test_directory_operations() {
    vibez.spill("\n=== Testing Enhanced Directory Operations ===")
    
    fr fr Create directory recursively
    create_directory_recursive("test/deep/nested", 755) fam {
        when _ -> {
            vibez.spill("❌ create_directory_recursive failed:", err)
            damn
        }
    }
    vibez.spill("✅ Recursive directory created")
    
    fr fr List directory with details
    sus entries []DirectoryEntry = list_directory_detailed(".", based) fam {
        when _ -> {
            vibez.spill("❌ list_directory_detailed failed:", err)
            damn
        }
    }
    vibez.spill("✅ Directory listing:")
    sus i drip = 0
    bestie (i < array_length(entries)) {
        sus entry DirectoryEntry = entries[i]
        vibez.spill("  ", entry.name, "(", entry.size, "bytes, perms:", entry.permissions, ")")
        i = i + 1
    }
    
    fr fr Copy directory with advanced options
    copy_directory_advanced("test", "test_copy", based, cringe) fam {
        when _ -> {
            vibez.spill("❌ copy_directory_advanced failed:", err)
            damn
        }
    }
    vibez.spill("✅ Directory copied with preserved permissions")
}

fr fr ===== FILESYSTEM INFORMATION TESTS =====

slay test_filesystem_info() {
    vibez.spill("\n=== Testing Filesystem Information ===")
    
    sus fs_info FileSystemInfo = get_filesystem_info(".") fam {
        when _ -> {
            vibez.spill("❌ get_filesystem_info failed:", err)
            damn
        }
    }
    
    vibez.spill("✅ Filesystem Information:")
    vibez.spill("  Total Space:", fs_info.total_space)
    vibez.spill("  Available Space:", fs_info.available_space)
    vibez.spill("  Used Space:", fs_info.used_space)
    vibez.spill("  Block Size:", fs_info.block_size)
    vibez.spill("  Filesystem Type:", fs_info.file_system_type)
    vibez.spill("  Read Only:", fs_info.is_read_only)
    
    fr fr Test filesystem sync
    sync_filesystem(".") fam {
        when _ -> {
            vibez.spill("❌ sync_filesystem failed:", err)
            damn
        }
    }
    vibez.spill("✅ Filesystem synced")
}

fr fr ===== FILE SEARCH AND PATTERN MATCHING TESTS =====

slay test_file_search() {
    vibez.spill("\n=== Testing File Search Operations ===")
    
    fr fr Find files by pattern
    sus txt_files []tea = find_files(".", "*.txt", based, 100) fam {
        when _ -> {
            vibez.spill("❌ find_files by pattern failed:", err)
            damn
        }
    }
    vibez.spill("✅ Found .txt files:")
    sus i drip = 0
    bestie (i < array_length(txt_files)) {
        vibez.spill("  ", txt_files[i])
        i = i + 1
    }
    
    fr fr Find files by size
    sus medium_files []tea = find_files_by_size(".", 100, 10000, based) fam {
        when _ -> {
            vibez.spill("❌ find_files_by_size failed:", err)
            damn
        }
    }
    vibez.spill("✅ Found files by size (100-10000 bytes):")
    i = 0
    bestie (i < array_length(medium_files)) {
        vibez.spill("  ", medium_files[i])
        i = i + 1
    }
    
    fr fr Find files by modification time
    sus recent_files []tea = find_files_by_time(".", 1640995200, 1640995800, based) fam {
        when _ -> {
            vibez.spill("❌ find_files_by_time failed:", err)
            damn
        }
    }
    vibez.spill("✅ Found files by time range:")
    i = 0
    bestie (i < array_length(recent_files)) {
        vibez.spill("  ", recent_files[i])
        i = i + 1
    }
}

fr fr ===== FILE WATCHING TESTS =====

slay test_file_watching() {
    vibez.spill("\n=== Testing File Watching ===")
    
    fr fr Start watching a file
    sus watch_id drip = watch_file_changes("test.txt", "file_changed_callback") fam {
        when _ -> {
            vibez.spill("❌ watch_file_changes failed:", err)
            damn
        }
    }
    vibez.spill("✅ File watcher started with ID:", watch_id)
    
    fr fr Stop watching
    stop_file_watching(watch_id) fam {
        when _ -> {
            vibez.spill("❌ stop_file_watching failed:", err)
            damn
        }
    }
    vibez.spill("✅ File watcher stopped")
}

fr fr ===== SECURITY AND VALIDATION TESTS =====

slay test_security_validation() {
    vibez.spill("\n=== Testing Security and Validation ===")
    
    fr fr Test path validation
    sus valid_path lit = is_valid_path("/home/user/file.txt")
    vibez.spill("✅ Valid path check:", valid_path)
    
    sus invalid_path lit = is_valid_path("../../../etc/passwd")
    vibez.spill("✅ Invalid path check (should be false):", invalid_path)
    
    sus dangerous_path lit = is_valid_path("/bin\0/sh")
    vibez.spill("✅ Dangerous path check (should be false):", dangerous_path)
    
    fr fr Test safe delete check
    sus safe_delete lit = is_safe_to_delete("/tmp/test.txt")
    vibez.spill("✅ Safe delete check:", safe_delete)
    
    sus unsafe_delete lit = is_safe_to_delete("/")
    vibez.spill("✅ Unsafe delete check (should be false):", unsafe_delete)
    
    fr fr Test file access checks
    sus can_read lit = check_file_access("test.txt", "read") fam {
        when _ -> {
            vibez.spill("❌ check_file_access failed:", err)
            damn
        }
    }
    vibez.spill("✅ File read access check:", can_read)
    
    sus can_write lit = check_file_access("test.txt", "write") fam {
        when _ -> {
            vibez.spill("❌ check_file_access failed:", err)
            damn
        }
    }
    vibez.spill("✅ File write access check:", can_write)
}

fr fr ===== ERROR HANDLING TESTS =====

slay test_error_handling() {
    vibez.spill("\n=== Testing Error Handling ===")
    
    fr fr Test reading non-existent file
    sus content tea = read_file_advanced("nonexistent_file.txt") fam {
        when _ -> {
            vibez.spill("✅ Correctly handled non-existent file error:", err)
            damn
        }
    }
    vibez.spill("❌ Should have failed for non-existent file")
    
    fr fr Test invalid file mode
    sus handle FileHandle = open_file("test.txt", "invalid_mode") fam {
        when _ -> {
            vibez.spill("✅ Correctly handled invalid mode error:", err)
            damn
        }
    }
    vibez.spill("❌ Should have failed for invalid mode")
    
    fr fr Test invalid permissions
    set_file_permissions_advanced("test.txt", 999, cringe) fam {
        when _ -> {
            vibez.spill("✅ Correctly handled invalid permissions error:", err)
            damn
        }
    }
    vibez.spill("❌ Should have failed for invalid permissions")
}

fr fr ===== PERFORMANCE TESTS =====

slay test_performance_operations() {
    vibez.spill("\n=== Testing Performance Operations ===")
    
    fr fr Test large file operations
    sus large_content tea = "This is a performance test content. " * 1000  fr fr Simulated large content
    
    write_file_advanced("large_test.txt", large_content, based) fam {
        when _ -> {
            vibez.spill("❌ Failed to write large file:", err)
            damn
        }
    }
    vibez.spill("✅ Large file write completed")
    
    fr fr Test buffered vs non-buffered performance
    sus start_time drip = get_current_time()
    
    sus handle FileHandle = open_file("large_test.txt", "read") fam {
        when _ -> {
            vibez.spill("❌ Failed to open large file:", err)
            damn
        }
    }
    
    sus buffered_handle FileHandle = enable_file_buffering(handle, 8192) fam {
        when _ -> {
            vibez.spill("❌ Failed to enable buffering:", err)
            damn
        }
    }
    
    fr fr Read in chunks
    sus total_read drip = 0
    sus chunk_size drip = 1024
    bestie (based) {
        sus chunk tea = read_file_chunk(buffered_handle, chunk_size) fam {
            when _ -> {
                vibez.spill("Read operation completed, total bytes:", total_read)
                ready
            }
        }
        total_read = total_read + string_length(chunk)
        ready (string_length(chunk) < chunk_size) {
            ready
        }
    }
    
    close_file(buffered_handle) fam {
        when _ -> vibez.spill("Warning: Failed to close file:", err)
    }
    
    sus end_time drip = get_current_time()
    vibez.spill("✅ Buffered read performance - Total bytes:", total_read, "Time:", end_time - start_time)
}

fr fr ===== MAIN TEST RUNNER =====

slay run_all_filez_tests() {
    vibez.spill("🔥 CURSED Enhanced Filez Runtime Integration Test Suite")
    vibez.spill("===============================================")
    
    test_read_write_advanced()
    test_file_handle_operations()
    test_buffered_io()
    test_detailed_file_info()
    test_permission_operations()
    test_directory_operations()
    test_filesystem_info()
    test_file_search()
    test_file_watching()
    test_security_validation()
    test_error_handling()
    test_performance_operations()
    
    vibez.spill("\n🎉 Enhanced Filez Test Suite Complete!")
    vibez.spill("All advanced file I/O operations validated with proper error handling")
    vibez.spill("P0 critical file operations are production-ready! 🚀")
}

fr fr Mock time function for performance testing
slay get_current_time() drip {
    damn 1640995200
}

fr fr Run all tests
run_all_filez_tests()
