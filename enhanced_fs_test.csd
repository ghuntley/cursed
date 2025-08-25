fr fr Comprehensive Test Suite for Enhanced Filesystem Module
fr fr Tests all enhanced functionality including real OS integration

yeet "fs/mod_enhanced"
yeet "path/mod_enhanced" 
yeet "vibez"
yeet "testz"

fr fr ================================
fr fr Test Enhanced Platform Detection
fr fr ================================

slay test_platform_detection() {
    vibez.spill("=== Testing Platform Detection ===")
    
    fr fr Test platform detection
    sus platform_info PlatformInfo = get_platform_info()
    vibez.spill("Detected Platform: " + platform_info.name)
    vibez.spill("Platform Family: " + platform_info.family)
    vibez.spill("Case Sensitive: " + stringz.from_bool(platform_info.case_sensitive))
    vibez.spill("File Separator: '" + platform_info.file_separator + "'")
    vibez.spill("Max Path Length: " + stringz.from_int(platform_info.max_path_length))
    
    fr fr Test filesystem capabilities
    sus capabilities map[tea]lit = get_platform_capabilities()
    vibez.spill("\nPlatform Capabilities:")
    bestie capability, supported in capabilities {
        vibez.spill("  " + capability + ": " + stringz.from_bool(supported))
    }
    
    testz.assert_true(platform_info.name != "", "Platform name should be detected")
    testz.assert_true(platform_info.family == "unix" || platform_info.family == "windows", "Platform family should be unix or windows")
    testz.assert_true(platform_info.max_path_length > 0, "Max path length should be positive")
}

fr fr ================================
fr fr Test Enhanced File Operations
fr fr ================================

slay test_enhanced_file_operations() {
    vibez.spill("\n=== Testing Enhanced File Operations ===")
    
    fr fr Test file existence with real OS calls
    sus test_file tea = "/tmp/cursed_test_file.txt"
    sus test_content tea = "Hello, enhanced CURSED filesystem!"
    
    vibez.spill("Testing file operations with: " + test_file)
    
    fr fr Test writing file
    sus write_success lit = write_file(test_file, test_content)
    testz.assert_true(write_success, "File write should succeed")
    vibez.spill("Write operation: " + stringz.from_bool(write_success))
    
    fr fr Test file existence
    sus exists lit = file_exists(test_file)
    testz.assert_true(exists, "File should exist after writing")
    vibez.spill("File exists: " + stringz.from_bool(exists))
    
    fr fr Test reading file
    sus read_content tea = read_file(test_file)
    vibez.spill("Read content length: " + stringz.from_int(stringz.length(read_content)))
    
    fr fr Test file metadata
    sus metadata FileMetadata = syscall_stat(test_file)
    vibez.spill("File metadata:")
    vibez.spill("  Name: " + metadata.name)
    vibez.spill("  Size: " + stringz.from_int(metadata.size))
    vibez.spill("  Is File: " + stringz.from_bool(metadata.is_file))
    vibez.spill("  Is Directory: " + stringz.from_bool(metadata.is_dir))
    vibez.spill("  Permissions: " + stringz.from_int(metadata.permissions))
    vibez.spill("  Modified Time: " + stringz.from_int(metadata.modified_time))
    
    fr fr Test file permissions
    vibez.spill("Permission checks:")
    vibez.spill("  Readable: " + stringz.from_bool(check_read_permission(test_file)))
    vibez.spill("  Writable: " + stringz.from_bool(check_write_permission(test_file)))
    vibez.spill("  Executable: " + stringz.from_bool(check_execute_permission(test_file)))
    
    fr fr Test file copy
    sus copy_dest tea = "/tmp/cursed_test_file_copy.txt"
    sus copy_success lit = copy_file(test_file, copy_dest)
    testz.assert_true(copy_success, "File copy should succeed")
    vibez.spill("Copy operation: " + stringz.from_bool(copy_success))
    
    fr fr Test file move
    sus move_dest tea = "/tmp/cursed_test_file_moved.txt"
    sus move_success lit = move_file(copy_dest, move_dest)
    testz.assert_true(move_success, "File move should succeed")
    vibez.spill("Move operation: " + stringz.from_bool(move_success))
    
    fr fr Cleanup
    delete_file(test_file)
    delete_file(move_dest)
}

fr fr ================================
fr fr Test Enhanced File Locking
fr fr ================================

slay test_enhanced_file_locking() {
    vibez.spill("\n=== Testing Enhanced File Locking ===")
    
    sus test_file tea = "/tmp/cursed_lock_test.txt"
    sus content tea = "File locking test content"
    
    fr fr Create test file
    write_file(test_file, content)
    
    fr fr Test exclusive lock
    vibez.spill("Testing exclusive file lock...")
    sus lock_success lit = lock_file(test_file)
    testz.assert_true(lock_success, "Exclusive lock should succeed")
    vibez.spill("Exclusive lock acquired: " + stringz.from_bool(lock_success))
    
    fr fr Test lock detection
    sus is_locked lit = is_file_locked(test_file)
    testz.assert_true(is_locked, "File should be detected as locked")
    vibez.spill("Lock detected: " + stringz.from_bool(is_locked))
    
    fr fr Test unlock
    sus unlock_success lit = unlock_file(test_file)
    testz.assert_true(unlock_success, "Unlock should succeed")
    vibez.spill("Unlock successful: " + stringz.from_bool(unlock_success))
    
    fr fr Test shared lock
    vibez.spill("Testing shared file lock...")
    sus shared_lock_success lit = lock_file_shared(test_file)
    testz.assert_true(shared_lock_success, "Shared lock should succeed")
    vibez.spill("Shared lock acquired: " + stringz.from_bool(shared_lock_success))
    
    unlock_file(test_file)
    
    fr fr Test blocking lock
    vibez.spill("Testing blocking lock (non-blocking for test)...")
    sus blocking_lock_success lit = lock_file_blocking(test_file)
    testz.assert_true(blocking_lock_success, "Blocking lock should succeed")
    vibez.spill("Blocking lock acquired: " + stringz.from_bool(blocking_lock_success))
    
    unlock_file(test_file)
    delete_file(test_file)
}

fr fr ================================
fr fr Test Enhanced Directory Operations
fr fr ================================

slay test_enhanced_directory_operations() {
    vibez.spill("\n=== Testing Enhanced Directory Operations ===")
    
    sus test_dir tea = "/tmp/cursed_test_dir"
    sus sub_dir tea = "/tmp/cursed_test_dir/subdir"
    
    fr fr Test directory creation
    vibez.spill("Testing directory creation...")
    sus create_success lit = create_dir(test_dir)
    testz.assert_true(create_success, "Directory creation should succeed")
    vibez.spill("Directory created: " + stringz.from_bool(create_success))
    
    fr fr Test recursive directory creation
    vibez.spill("Testing recursive directory creation...")
    sus recursive_success lit = create_dir_recursive(sub_dir)
    testz.assert_true(recursive_success, "Recursive directory creation should succeed")
    vibez.spill("Recursive directory created: " + stringz.from_bool(recursive_success))
    
    fr fr Test directory listing
    vibez.spill("Testing directory listing...")
    sus entries []DirEntry = list_dir(test_dir)
    vibez.spill("Directory entries found: " + stringz.from_int(len(entries)))
    
    bestie i, entry in entries {
        vibez.spill("  [" + stringz.from_int(i) + "] " + entry.name + 
                   " (dir: " + stringz.from_bool(entry.is_dir) + 
                   ", size: " + stringz.from_int(entry.size) + ")")
    }
    
    fr fr Create test file in directory
    sus test_file tea = join_path_safe(test_dir, "test_file.txt")
    write_file(test_file, "Test content in directory")
    
    fr fr List again with file
    entries = list_dir(test_dir)
    vibez.spill("Directory entries after adding file: " + stringz.from_int(len(entries)))
    
    fr fr Test directory removal (should fail - not empty)
    vibez.spill("Testing directory removal (should fail - not empty)...")
    sus remove_fail lit = remove_dir(test_dir)
    testz.assert_false(remove_fail, "Directory removal should fail when not empty")
    vibez.spill("Directory removal (should fail): " + stringz.from_bool(remove_fail))
    
    fr fr Test recursive directory removal
    vibez.spill("Testing recursive directory removal...")
    sus recursive_remove_success lit = remove_dir_recursive(test_dir)
    testz.assert_true(recursive_remove_success, "Recursive directory removal should succeed")
    vibez.spill("Recursive directory removed: " + stringz.from_bool(recursive_remove_success))
}

fr fr ================================
fr fr Test Enhanced Environment Variable Expansion
fr fr ================================

slay test_enhanced_env_expansion() {
    vibez.spill("\n=== Testing Enhanced Environment Variable Expansion ===")
    
    fr fr Test basic environment expansion
    sus home_path tea = expand_env("$HOME/documents")
    vibez.spill("$HOME expansion: " + home_path)
    testz.assert_true(stringz.length(home_path) > 11, "HOME expansion should work")
    
    fr fr Test brace expansion
    sus brace_path tea = expand_env("${HOME}/config")
    vibez.spill("${HOME} expansion: " + brace_path)
    testz.assert_true(stringz.length(brace_path) > 13, "Brace expansion should work")
    
    fr fr Test tilde expansion
    sus tilde_path tea = expand_home("~/documents")
    vibez.spill("~ expansion: " + tilde_path)
    testz.assert_true(stringz.length(tilde_path) > 11, "Tilde expansion should work")
    
    fr fr Test combined expansion
    sus combined_path tea = expand_all("~/config/${USER}/settings")
    vibez.spill("Combined expansion: " + combined_path)
    testz.assert_true(stringz.length(combined_path) > 20, "Combined expansion should work")
    
    fr fr Test platform-specific expansions
    sus platform_info PlatformInfo = get_platform_info()
    lowkey platform_info.family == "windows" {
        vibez.spill("Testing Windows-specific expansions...")
        sus userprofile_path tea = expand_env("%USERPROFILE%\\Documents")
        vibez.spill("%USERPROFILE% expansion: " + userprofile_path)
        
        sus temp_path tea = expand_env("%TEMP%\\cursed_test")
        vibez.spill("%TEMP% expansion: " + temp_path)
    } otherwise {
        vibez.spill("Testing Unix-specific expansions...")
        sus tmpdir_path tea = expand_env("${TMPDIR:-/tmp}/cursed_test")
        vibez.spill("TMPDIR expansion: " + tmpdir_path)
        
        sus user_path tea = expand_env("/home/$USER/documents")
        vibez.spill("$USER expansion: " + user_path)
    }
}

fr fr ================================
fr fr Test Enhanced Path Operations
fr fr ================================

slay test_enhanced_path_operations() {
    vibez.spill("\n=== Testing Enhanced Path Operations ===")
    
    fr fr Test path joining with proper separators
    sus joined_path tea = join([]tea{"home", "user", "documents", "file.txt"})
    vibez.spill("Path joining: " + joined_path)
    testz.assert_true(stringz.contains(joined_path, "documents"), "Joined path should contain components")
    
    fr fr Test path splitting
    sus components []tea = split(joined_path)
    vibez.spill("Path components: " + stringz.from_int(len(components)))
    bestie i, component in components {
        vibez.spill("  [" + stringz.from_int(i) + "] '" + component + "'")
    }
    testz.assert_true(len(components) >= 3, "Should have multiple path components")
    
    fr fr Test path information
    sus path_info PathInfo = info(joined_path)
    vibez.spill("Path information:")
    vibez.spill("  Original: " + path_info.original)
    vibez.spill("  Absolute: " + path_info.absolute)
    vibez.spill("  Directory: " + path_info.directory)
    vibez.spill("  Filename: " + path_info.filename)
    vibez.spill("  Basename: " + path_info.basename)
    vibez.spill("  Extension: " + path_info.extension)
    vibez.spill("  Is Absolute: " + stringz.from_bool(path_info.is_absolute))
    
    fr fr Test path validation
    sus valid_path lit = validate_path(joined_path)
    vibez.spill("Path validation: " + stringz.from_bool(valid_path))
    testz.assert_true(valid_path, "Valid path should pass validation")
    
    fr fr Test invalid path
    sus invalid_path tea = "invalid\x00path"
    sus invalid_validation lit = validate_path(invalid_path)
    vibez.spill("Invalid path validation: " + stringz.from_bool(invalid_validation))
    testz.assert_false(invalid_validation, "Invalid path should fail validation")
    
    fr fr Test path cleaning
    sus messy_path tea = "home//user/../user/./documents/../documents/file.txt"
    sus clean_path tea = clean(messy_path)
    vibez.spill("Path cleaning:")
    vibez.spill("  Original: " + messy_path)
    vibez.spill("  Cleaned: " + clean_path)
    testz.assert_true(!stringz.contains(clean_path, "//"), "Cleaned path should not have double separators")
    
    fr fr Test relative path calculation
    sus base_path tea = "/home/user/documents"
    sus target_path tea = "/home/user/projects/myproject"
    sus relative_path tea = rel(base_path, target_path)
    vibez.spill("Relative path from '" + base_path + "' to '" + target_path + "': " + relative_path)
    testz.assert_true(stringz.contains(relative_path, ".."), "Relative path should contain parent references")
}

fr fr ================================
fr fr Test Enhanced Path Matching and Comparison
fr fr ================================

slay test_enhanced_path_matching() {
    vibez.spill("\n=== Testing Enhanced Path Matching ===")
    
    fr fr Test glob pattern matching
    sus pattern tea = "*.txt"
    sus test_file tea = "document.txt"
    sus match_result lit = match_glob(pattern, test_file)
    vibez.spill("Glob match '" + pattern + "' vs '" + test_file + "': " + stringz.from_bool(match_result))
    testz.assert_true(match_result, "Glob pattern should match")
    
    fr fr Test extension matching
    sus extensions []tea = []tea{".txt", ".doc", ".pdf"}
    sus ext_match_result lit = match_extension("report.pdf", extensions)
    vibez.spill("Extension match result: " + stringz.from_bool(ext_match_result))
    testz.assert_true(ext_match_result, "Extension should match")
    
    fr fr Test path comparison
    sus path1 tea = "/home/user/Documents"
    sus path2 tea = "/home/user/documents"
    sus comparison normie = compare_paths(path1, path2)
    vibez.spill("Path comparison result: " + stringz.from_int(comparison))
    
    sus platform_info PlatformInfo = get_platform_info()
    lowkey platform_info.case_sensitive {
        testz.assert_true(comparison != 0, "Paths should not be equal on case-sensitive filesystem")
    } otherwise {
        testz.assert_true(comparison == 0, "Paths should be equal on case-insensitive filesystem")
    }
    
    fr fr Test path list operations
    sus path_list tea = "/usr/bin:/usr/local/bin:/home/user/bin"
    sus path_components []tea = split_list(path_list)
    vibez.spill("Path list components: " + stringz.from_int(len(path_components)))
    bestie i, path_component in path_components {
        vibez.spill("  [" + stringz.from_int(i) + "] " + path_component)
    }
    testz.assert_true(len(path_components) == 3, "Should have 3 path components")
    
    fr fr Test joining path list
    sus rejoined_list tea = join_list(path_components)
    vibez.spill("Rejoined path list: " + rejoined_list)
    testz.assert_true(stringz.length(rejoined_list) > 0, "Rejoined list should not be empty")
}

fr fr ================================
fr fr Test Directory Management
fr fr ================================

slay test_directory_management() {
    vibez.spill("\n=== Testing Directory Management ===")
    
    fr fr Test getting standard directories
    vibez.spill("Standard directories:")
    vibez.spill("  Current: " + get_current_dir())
    vibez.spill("  Home: " + get_home_dir())
    vibez.spill("  Temp: " + get_temp_dir())
    vibez.spill("  Executable: " + get_executable_dir())
    vibez.spill("  User Config: " + get_user_config_dir())
    vibez.spill("  User Cache: " + get_user_cache_dir())
    vibez.spill("  User Data: " + get_user_data_dir())
    vibez.spill("  System Config: " + get_system_config_dir())
    vibez.spill("  System Data: " + get_system_data_dir())
    
    fr fr Test all directories exist and are not empty
    testz.assert_true(stringz.length(get_current_dir()) > 0, "Current directory should not be empty")
    testz.assert_true(stringz.length(get_home_dir()) > 0, "Home directory should not be empty")
    testz.assert_true(stringz.length(get_temp_dir()) > 0, "Temp directory should not be empty")
    
    fr fr Test path search
    sus found_path tea = search_path("sh")  fr fr Look for shell
    vibez.spill("Shell executable path: " + found_path)
    fr fr Note: May be empty on systems without sh in PATH
}

fr fr ================================
fr fr Test Filesystem Information
fr fr ================================

slay test_filesystem_information() {
    vibez.spill("\n=== Testing Filesystem Information ===")
    
    fr fr Test filesystem info
    sus fs_info FileSystem = get_filesystem_info()
    vibez.spill("Filesystem information:")
    vibez.spill("  Platform: " + fs_info.platform)
    vibez.spill("  Case Sensitive: " + stringz.from_bool(fs_info.case_sensitive))
    vibez.spill("  Max Path Length: " + stringz.from_int(fs_info.max_path_length))
    vibez.spill("  Max Filename Length: " + stringz.from_int(fs_info.max_filename_length))
    vibez.spill("  File Separator: '" + fs_info.file_separator + "'")
    vibez.spill("  Path List Separator: '" + fs_info.path_list_separator + "'")
    vibez.spill("  Supports Symlinks: " + stringz.from_bool(fs_info.supports_symlinks))
    vibez.spill("  Supports Hardlinks: " + stringz.from_bool(fs_info.supports_hardlinks))
    vibez.spill("  Supports Permissions: " + stringz.from_bool(fs_info.supports_permissions))
    
    fr fr Test filesystem statistics
    sus open_files_count normie = get_open_files_count()
    sus active_locks_count normie = get_active_locks_count()
    vibez.spill("Filesystem statistics:")
    vibez.spill("  Open Files: " + stringz.from_int(open_files_count))
    vibez.spill("  Active Locks: " + stringz.from_int(active_locks_count))
    
    fr fr Test path manager stats  
    sus path_stats map[tea]normie = get_path_manager_stats()
    vibez.spill("Path manager statistics:")
    bestie stat_name, stat_value in path_stats {
        vibez.spill("  " + stat_name + ": " + stringz.from_int(stat_value))
    }
}

fr fr ================================
fr fr Main Test Runner
fr fr ================================

slay main() {
    vibez.spill("Enhanced Filesystem Module Comprehensive Test Suite")
    vibez.spill("=" * 60)
    
    testz.start_test_suite("Enhanced Filesystem Tests")
    
    fr fr Run all test suites
    test_platform_detection()
    test_enhanced_file_operations()
    test_enhanced_file_locking()
    test_enhanced_directory_operations() 
    test_enhanced_env_expansion()
    test_enhanced_path_operations()
    test_enhanced_path_matching()
    test_directory_management()
    test_filesystem_information()
    
    fr fr Debug information
    vibez.spill("\n=== Debug Information ===")
    debug_platform_info()
    debug_path_info("/home/user/documents/test.txt")
    
    fr fr Cleanup
    cleanup_filesystem()
    cleanup_path_manager()
    
    fr fr Test results
    vibez.spill("\n" + "=" * 60)
    testz.print_test_summary()
    vibez.spill("Enhanced filesystem module tests completed!")
}

main()
