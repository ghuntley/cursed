fr fr CURSED Filesystem Module Tests - Production Version
fr fr Comprehensive testing of all filesystem functions
fr fr Tests both interpretation and compilation modes

yeet "testz"
yeet "fs"

fr fr ================================
fr fr Test Setup and Initialization
fr fr ================================

test_start("Filesystem Module - Initialization")

fr fr Test filesystem initialization
sus stats map[tea]normie = get_filesystem_stats()
assert_true(stats["files"] >= 0)
assert_true(stats["directories"] >= 0)
assert_true(stats["metadata_entries"] >= 0)
assert_true(stats["open_handles"] >= 0)

fr fr ================================
fr fr Basic File Operations Tests
fr fr ================================

test_start("File Operations - Create, Read, Write")

fr fr Test file creation and writing
sus test_file tea = "test_create_write.txt"
sus test_content tea = "Hello, CURSED filesystem!"

assert_true(write_file(test_file, test_content))
assert_true(file_exists(test_file))
assert_eq_string(read_file(test_file), test_content)

fr fr Test file size
assert_eq_int(get_file_size(test_file), 26)

fr fr Test file type detection
assert_true(is_file(test_file))
assert_false(is_dir(test_file))

fr fr Clean up
assert_true(delete_file(test_file))
assert_false(file_exists(test_file))

test_start("File Operations - Copy and Move")

fr fr Test file copy
sus source_file tea = "source.txt"
sus dest_file tea = "destination.txt"
sus copy_content tea = "Content to copy"

assert_true(write_file(source_file, copy_content))
assert_true(copy_file(source_file, dest_file))
assert_true(file_exists(dest_file))
assert_eq_string(read_file(dest_file), copy_content)

fr fr Test file move
sus moved_file tea = "moved.txt"
assert_true(move_file(dest_file, moved_file))
assert_false(file_exists(dest_file))
assert_true(file_exists(moved_file))
assert_eq_string(read_file(moved_file), copy_content)

fr fr Clean up
assert_true(delete_file(source_file))
assert_true(delete_file(moved_file))

test_start("File Operations - Append")

fr fr Test file append
sus append_file tea = "append_test.txt"
sus initial_content tea = "Initial content"
sus appended_content tea = " - Appended content"

assert_true(write_file(append_file, initial_content))
assert_true(append_file(append_file, appended_content))
assert_eq_string(read_file(append_file), initial_content + appended_content)

fr fr Clean up
assert_true(delete_file(append_file))

fr fr ================================
fr fr Directory Operations Tests
fr fr ================================

test_start("Directory Operations - Create and Remove")

fr fr Test directory creation
sus test_dir tea = "test_directory"
assert_true(create_dir(test_dir))
assert_true(file_exists(test_dir))
assert_true(is_dir(test_dir))
assert_false(is_file(test_dir))

fr fr Test directory metadata
sus dir_info FileInfo = get_file_info(test_dir)
assert_eq_string(dir_info.name, "test_directory")
assert_true(dir_info.is_dir)
assert_eq_int(dir_info.permissions, 755)

fr fr Test directory removal
assert_true(remove_dir(test_dir))
assert_false(file_exists(test_dir))

test_start("Directory Operations - Recursive")

fr fr Test recursive directory creation
sus nested_dir tea = "parent/child/grandchild"
assert_true(create_dir_recursive(nested_dir))
assert_true(is_dir(nested_dir))
assert_true(is_dir("parent"))
assert_true(is_dir("parent/child"))

fr fr Test recursive directory removal
assert_true(remove_dir_recursive("parent"))
assert_false(file_exists("parent"))

test_start("Directory Operations - Listing")

fr fr Create test directory structure
sus list_dir tea = "list_test"
assert_true(create_dir(list_dir))

fr fr Create files in directory
sus file1 tea = join_path(list_dir, "file1.txt")
sus file2 tea = join_path(list_dir, "file2.txt")
sus subdir tea = join_path(list_dir, "subdir")

assert_true(write_file(file1, "Content 1"))
assert_true(write_file(file2, "Content 2"))
assert_true(create_dir(subdir))

fr fr Test directory listing
sus entries []DirEntry = list_dir(list_dir)
assert_true(len(entries) >= 3)

fr fr Clean up
assert_true(remove_dir_recursive(list_dir))

fr fr ================================
fr fr Path Utilities Tests
fr fr ================================

test_start("Path Utilities - Basename and Extension")

fr fr Test basename extraction
assert_eq_string(get_basename("test.txt"), "test.txt")
assert_eq_string(get_basename("path/to/file.cpp"), "file.cpp")
assert_eq_string(get_basename("/absolute/path/filename"), "filename")
assert_eq_string(get_basename("just_filename"), "just_filename")
assert_eq_string(get_basename(""), "")

fr fr Test extension extraction
assert_eq_string(get_extension("test.txt"), ".txt")
assert_eq_string(get_extension("archive.tar.gz"), ".gz")
assert_eq_string(get_extension("no_extension"), "")
assert_eq_string(get_extension("path/to/file.cpp"), ".cpp")

test_start("Path Utilities - Path Operations")

fr fr Test path joining
assert_eq_string(join_path("base", "file.txt"), "base/file.txt")
assert_eq_string(join_path("", "file.txt"), "file.txt")
assert_eq_string(join_path("base", ""), "base")
assert_eq_string(join_path("base/", "file.txt"), "base/file.txt")

fr fr Test parent directory
assert_eq_string(get_parent_dir("path/to/file.txt"), "path/to")
assert_eq_string(get_parent_dir("file.txt"), "")
assert_eq_string(get_parent_dir(""), "")

fr fr Test absolute path operations
assert_true(is_absolute_path("/absolute/path"))
assert_false(is_absolute_path("relative/path"))
assert_eq_string(get_absolute_path("relative"), "/relative")
assert_eq_string(get_absolute_path("/already/absolute"), "/already/absolute")

fr fr ================================
fr fr Timestamp Functions Tests
fr fr ================================

test_start("Timestamp Functions - Get and Set Times")

fr fr Create test file for timestamp testing
sus timestamp_file tea = "timestamp_test.txt"
sus timestamp_content tea = "Timestamp test content"
assert_true(write_file(timestamp_file, timestamp_content))

fr fr Test timestamp retrieval
sus created_time thicc = get_created_time(timestamp_file)
sus modified_time thicc = get_modified_time(timestamp_file)
sus accessed_time thicc = get_accessed_time(timestamp_file)

fr fr Timestamps should be non-zero for existing files
assert_true(created_time > 0)
assert_true(modified_time > 0)
assert_true(accessed_time > 0)

fr fr Test timestamp setting
sus new_timestamp thicc = 1704067200 fr fr 2024-01-01 00:00:00 UTC
assert_true(set_created_time(timestamp_file, new_timestamp))
assert_true(set_modified_time(timestamp_file, new_timestamp))
assert_true(set_accessed_time(timestamp_file, new_timestamp))

fr fr Verify timestamp changes
assert_eq_int(get_created_time(timestamp_file), new_timestamp)
assert_eq_int(get_modified_time(timestamp_file), new_timestamp)
assert_eq_int(get_accessed_time(timestamp_file), new_timestamp)

fr fr Test timestamp for non-existent file
assert_eq_int(get_created_time("nonexistent.txt"), 0)
assert_eq_int(get_modified_time("nonexistent.txt"), 0)
assert_eq_int(get_accessed_time("nonexistent.txt"), 0)

fr fr Test setting timestamp for non-existent file
assert_false(set_created_time("nonexistent.txt", new_timestamp))
assert_false(set_modified_time("nonexistent.txt", new_timestamp))
assert_false(set_accessed_time("nonexistent.txt", new_timestamp))

fr fr Clean up
assert_true(delete_file(timestamp_file))

fr fr ================================
fr fr Permission Functions Tests
fr fr ================================

test_start("Permission Functions - Basic Permissions")

fr fr Create test file for permission testing
sus perm_file tea = "permission_test.txt"
sus perm_content tea = "Permission test content"
assert_true(write_file(perm_file, perm_content))

fr fr Test getting permissions
sus perms normie = get_permissions(perm_file)
assert_true(perms > 0)
assert_true(perms <= 777)
assert_eq_int(perms, 644) fr fr Default file permissions

fr fr Test setting permissions
assert_true(set_permissions(perm_file, 755))
assert_eq_int(get_permissions(perm_file), 755)

fr fr Test invalid permissions
assert_false(set_permissions(perm_file, 999))
assert_false(set_permissions(perm_file, -1))
assert_false(set_permissions("nonexistent.txt", 755))

fr fr Clean up
assert_true(delete_file(perm_file))

test_start("Permission Functions - Read/Write/Execute")

fr fr Create test file for permission checking
sus rwx_file tea = "rwx_test.txt"
sus rwx_content tea = "RWX test content"
assert_true(write_file(rwx_file, rwx_content))

fr fr Test default permissions (644)
assert_true(is_readable(rwx_file))
assert_true(is_writable(rwx_file))
assert_false(is_executable(rwx_file))

fr fr Test read-only permissions (444)
assert_true(set_permissions(rwx_file, 444))
assert_true(is_readable(rwx_file))
assert_false(is_writable(rwx_file))
assert_false(is_executable(rwx_file))

fr fr Test executable permissions (755)
assert_true(set_permissions(rwx_file, 755))
assert_true(is_readable(rwx_file))
assert_true(is_writable(rwx_file))
assert_true(is_executable(rwx_file))

fr fr Test permission checking for non-existent file
assert_false(is_readable("nonexistent.txt"))
assert_false(is_writable("nonexistent.txt"))
assert_false(is_executable("nonexistent.txt"))

fr fr Clean up
assert_true(delete_file(rwx_file))

fr fr ================================
fr fr File Metadata Tests
fr fr ================================

test_start("File Metadata - Comprehensive Info")

fr fr Create test file for metadata testing
sus metadata_file tea = "metadata_test.txt"
sus metadata_content tea = "Metadata test content"
assert_true(write_file(metadata_file, metadata_content))

fr fr Test FileInfo structure
sus info FileInfo = get_file_info(metadata_file)
assert_eq_string(info.name, "metadata_test.txt")
assert_eq_int(info.size, 21)
assert_false(info.is_dir)
assert_true(info.modified_time > 0)
assert_eq_int(info.permissions, 644)

fr fr Test FileMetadata structure
sus metadata FileMetadata = get_file_metadata(metadata_file)
assert_eq_string(metadata.name, "metadata_test.txt")
assert_eq_string(metadata.path, metadata_file)
assert_eq_int(metadata.size, 21)
assert_false(metadata.is_dir)
assert_true(metadata.is_file)
assert_false(metadata.is_symlink)
assert_true(metadata.created_time > 0)
assert_true(metadata.modified_time > 0)
assert_true(metadata.accessed_time > 0)
assert_eq_int(metadata.permissions, 644)
assert_eq_int(metadata.owner_id, 1000)
assert_eq_int(metadata.group_id, 1000)

fr fr Test directory metadata
sus metadata_dir tea = "metadata_dir"
assert_true(create_dir(metadata_dir))

sus dir_metadata FileMetadata = get_file_metadata(metadata_dir)
assert_true(dir_metadata.is_dir)
assert_false(dir_metadata.is_file)
assert_eq_int(dir_metadata.permissions, 755)

fr fr Clean up
assert_true(delete_file(metadata_file))
assert_true(remove_dir(metadata_dir))

fr fr ================================
fr fr Special File Tests
fr fr ================================

test_start("Special Files - Hidden and System")

fr fr Test hidden file detection
assert_true(is_hidden(".hidden_file"))
assert_true(is_hidden("path/to/.hidden"))
assert_false(is_hidden("normal_file.txt"))
assert_false(is_hidden("not.hidden"))

fr fr Test system file detection
assert_true(is_system_file("."))
assert_true(is_system_file(".."))
assert_true(is_system_file("/proc/version"))
assert_true(is_system_file("/sys/kernel"))
assert_true(is_system_file("/dev/null"))
assert_false(is_system_file("normal_file.txt"))

test_start("Special Files - Empty Files and Directories")

fr fr Test empty file detection
sus empty_file tea = "empty.txt"
assert_true(write_file(empty_file, ""))
assert_true(is_empty_file(empty_file))
assert_false(is_empty_file("nonexistent.txt"))

fr fr Test non-empty file
sus non_empty_file tea = "non_empty.txt"
assert_true(write_file(non_empty_file, "content"))
assert_false(is_empty_file(non_empty_file))

fr fr Test empty directory
sus empty_dir tea = "empty_dir"
assert_true(create_dir(empty_dir))
assert_true(is_empty_dir(empty_dir))

fr fr Test non-empty directory
sus non_empty_dir tea = "non_empty_dir"
assert_true(create_dir(non_empty_dir))
sus dir_file tea = join_path(non_empty_dir, "file.txt")
assert_true(write_file(dir_file, "content"))
assert_false(is_empty_dir(non_empty_dir))

fr fr Clean up
assert_true(delete_file(empty_file))
assert_true(delete_file(non_empty_file))
assert_true(remove_dir_recursive(non_empty_dir))
assert_true(remove_dir(empty_dir))

fr fr ================================
fr fr File Locking Tests
fr fr ================================

test_start("File Locking - Lock and Unlock")

fr fr Create test file for locking
sus lock_file tea = "lock_test.txt"
sus lock_content tea = "Lock test content"
assert_true(write_file(lock_file, lock_content))

fr fr Test file locking
assert_false(is_locked(lock_file))
assert_true(lock_file(lock_file))
assert_true(is_locked(lock_file))

fr fr Test double locking (should fail)
assert_false(lock_file(lock_file))

fr fr Test unlocking
assert_true(unlock_file(lock_file))
assert_false(is_locked(lock_file))

fr fr Test unlocking non-locked file
assert_false(unlock_file(lock_file))

fr fr Test locking non-existent file
assert_false(lock_file("nonexistent.txt"))

fr fr Clean up
assert_true(delete_file(lock_file))

fr fr ================================
fr fr Error Handling Tests
fr fr ================================

test_start("Error Handling - Non-existent Files")

fr fr Test operations on non-existent files
assert_false(file_exists("definitely_does_not_exist.txt"))
assert_eq_string(read_file("nonexistent.txt"), "")
assert_eq_int(get_file_size("nonexistent.txt"), 0)
assert_false(delete_file("nonexistent.txt"))

fr fr Test copy and move operations on non-existent files
assert_false(copy_file("nonexistent.txt", "dest.txt"))
assert_false(move_file("nonexistent.txt", "dest.txt"))

fr fr Test permission operations on non-existent files
assert_eq_int(get_permissions("nonexistent.txt"), 0)
assert_false(set_permissions("nonexistent.txt", 644))

fr fr Test directory operations on non-existent directories
assert_false(remove_dir("nonexistent_dir"))
assert_false(is_dir("nonexistent_dir"))

test_start("Error Handling - Invalid Operations")

fr fr Test invalid file operations
assert_false(write_file("", "content"))
assert_false(write_file("valid.txt", ""))
assert_false(create_dir(""))

fr fr Test operations on files vs directories
sus test_file tea = "test_file.txt"
sus test_dir tea = "test_dir"
assert_true(write_file(test_file, "content"))
assert_true(create_dir(test_dir))

fr fr Try to read directory as file
assert_eq_string(read_file(test_dir), "")

fr fr Try to create directory over existing file
assert_false(create_dir(test_file))

fr fr Try to remove directory as file
assert_false(delete_file(test_dir))

fr fr Clean up
assert_true(delete_file(test_file))
assert_true(remove_dir(test_dir))

fr fr ================================
fr fr Performance Tests
fr fr ================================

test_start("Performance - Large Operations")

fr fr Test with larger file content
sus large_content tea = "Large file content for testing performance and handling of bigger files with more data that spans multiple lines and contains various characters and symbols to test the robustness of the filesystem implementation"
sus large_file tea = "large_test.txt"

assert_true(write_file(large_file, large_content))
assert_true(file_exists(large_file))
assert_eq_string(read_file(large_file), large_content)

fr fr Test file operations on larger file
sus large_info FileInfo = get_file_info(large_file)
assert_true(large_info.size > 100)
assert_eq_string(large_info.name, "large_test.txt")

fr fr Test copy and move on large file
sus large_copy tea = "large_copy.txt"
sus large_moved tea = "large_moved.txt"
assert_true(copy_file(large_file, large_copy))
assert_true(move_file(large_copy, large_moved))

fr fr Clean up
assert_true(delete_file(large_file))
assert_true(delete_file(large_moved))

test_start("Performance - Multiple Files")

fr fr Test creating multiple files
sus file_count normie = 10
bestie i := 0; i < file_count; i++ {
    sus file_name tea = "multi_file_" + string(i) + ".txt"
    sus content tea = "Content for file " + string(i)
    assert_true(write_file(file_name, content))
    assert_true(file_exists(file_name))
}

fr fr Test filesystem statistics
sus stats map[tea]normie = get_filesystem_stats()
assert_true(stats["files"] >= file_count)

fr fr Clean up multiple files
bestie i := 0; i < file_count; i++ {
    sus file_name tea = "multi_file_" + string(i) + ".txt"
    assert_true(delete_file(file_name))
}

fr fr ================================
fr fr Cross-Platform Compatibility Tests
fr fr ================================

test_start("Cross-Platform - Path Handling")

fr fr Test path utilities with different separators
assert_eq_string(get_basename("unix/path/file.txt"), "file.txt")
assert_eq_string(get_extension("unix/path/file.txt"), ".txt")

fr fr Test edge cases
assert_eq_string(get_basename(""), "")
assert_eq_string(get_extension(""), "")
assert_eq_string(get_basename("file"), "file")
assert_eq_string(get_extension("file"), "")

fr fr Test complex paths
assert_eq_string(get_basename("very/long/path/with/many/components/final.txt"), "final.txt")
assert_eq_string(get_extension("archive.tar.gz"), ".gz")

fr fr ================================
fr fr Filesystem Utilities Tests
fr fr ================================

test_start("Filesystem Utilities - Statistics and Cleanup")

fr fr Test filesystem statistics
sus initial_stats map[tea]normie = get_filesystem_stats()
assert_true(initial_stats["files"] >= 0)
assert_true(initial_stats["directories"] >= 0)
assert_true(initial_stats["metadata_entries"] >= 0)

fr fr Create some test files and directories
sus util_file tea = "util_test.txt"
sus util_dir tea = "util_dir"
assert_true(write_file(util_file, "content"))
assert_true(create_dir(util_dir))

fr fr Check updated statistics
sus updated_stats map[tea]normie = get_filesystem_stats()
assert_true(updated_stats["files"] >= initial_stats["files"])
assert_true(updated_stats["directories"] >= initial_stats["directories"])

fr fr Test cleanup
assert_true(delete_file(util_file))
assert_true(remove_dir(util_dir))

fr fr ================================
fr fr Comprehensive Integration Tests
fr fr ================================

test_start("Integration - Complete Filesystem Workflow")

fr fr Test complete workflow: create, modify, copy, move, delete
sus workflow_file tea = "workflow_test.txt"
sus workflow_content tea = "Initial workflow content"

fr fr Create and verify
assert_true(write_file(workflow_file, workflow_content))
assert_true(file_exists(workflow_file))
assert_eq_string(read_file(workflow_file), workflow_content)

fr fr Modify permissions
assert_true(set_permissions(workflow_file, 755))
assert_eq_int(get_permissions(workflow_file), 755)

fr fr Append content
sus additional_content tea = " - Additional content"
assert_true(append_file(workflow_file, additional_content))
assert_eq_string(read_file(workflow_file), workflow_content + additional_content)

fr fr Copy file
sus workflow_copy tea = "workflow_copy.txt"
assert_true(copy_file(workflow_file, workflow_copy))
assert_true(file_exists(workflow_copy))
assert_eq_string(read_file(workflow_copy), workflow_content + additional_content)

fr fr Move file
sus workflow_moved tea = "workflow_moved.txt"
assert_true(move_file(workflow_copy, workflow_moved))
assert_false(file_exists(workflow_copy))
assert_true(file_exists(workflow_moved))

fr fr Test metadata consistency
sus metadata FileMetadata = get_file_metadata(workflow_moved)
assert_eq_string(metadata.name, "workflow_moved.txt")
assert_true(metadata.is_file)
assert_false(metadata.is_dir)
assert_eq_int(metadata.permissions, 755)

fr fr Clean up
assert_true(delete_file(workflow_file))
assert_true(delete_file(workflow_moved))

fr fr ================================
fr fr Test Summary and Validation
fr fr ================================

test_start("Filesystem Module - Final Validation")

fr fr Validate all core functions are working
assert_true(file_exists("test_file.txt")) fr fr Mock file should exist
assert_false(file_exists("definitely_nonexistent.txt"))
assert_true(get_file_size("test_file.txt") > 0)
assert_true(get_permissions("test_file.txt") > 0)

fr fr Validate path utilities
assert_eq_string(get_basename("path/file.txt"), "file.txt")
assert_eq_string(get_extension("file.txt"), ".txt")
assert_eq_string(join_path("path", "file.txt"), "path/file.txt")

fr fr Validate special file detection
assert_true(is_hidden(".hidden"))
assert_true(is_system_file("/proc/version"))

fr fr Final filesystem statistics
sus final_stats map[tea]normie = get_filesystem_stats()
assert_true(final_stats["files"] >= 0)
assert_true(final_stats["directories"] >= 0)
assert_true(final_stats["metadata_entries"] >= 0)
assert_true(final_stats["open_handles"] >= 0)

fr fr ================================
fr fr Test Summary
fr fr ================================

print_test_summary()

fr fr ================================
fr fr Utility Functions for Testing
fr fr ================================

fr fr Helper function to test both interpretation and compilation modes
slay test_filesystem_comprehensive() lit { fr fr This function provides a comprehensive test that can be used fr fr to verify the filesystem module works identically in both modes fr fr Test basic file operations
    sus comprehensive_file tea = "comprehensive_test.txt"
    sus comprehensive_content tea = "Comprehensive test content" fr fr Write and read
    lowkey !write_file(comprehensive_file, comprehensive_content) {
        damn false
    }
    
    lowkey !file_exists(comprehensive_file) {
        damn false
    }
    
    lowkey read_file(comprehensive_file) != comprehensive_content {
        damn false
    } fr fr Test file information
    sus info FileInfo = get_file_info(comprehensive_file)
    lowkey info.name != "comprehensive_test.txt" {
        damn false
    }
    
    lowkey info.size != 25 {
        damn false
    } fr fr Test permissions
    sus perms normie = get_permissions(comprehensive_file)
    lowkey perms <= 0 {
        damn false
    } fr fr Test timestamps
    sus mod_time thicc = get_modified_time(comprehensive_file)
    lowkey mod_time <= 0 {
        damn false
    } fr fr Test metadata
    sus metadata FileMetadata = get_file_metadata(comprehensive_file)
    lowkey metadata.path != comprehensive_file {
        damn false
    }
    
    lowkey !metadata.is_file {
        damn false
    }
    
    lowkey metadata.is_dir {
        damn false
    } fr fr Test directory operations
    sus test_dir tea = "comprehensive_dir"
    lowkey !create_dir(test_dir) {
        damn false
    }
    
    lowkey !is_dir(test_dir) {
        damn false
    } fr fr Test file copy
    sus copy_file tea = "comprehensive_copy.txt"
    lowkey !copy_file(comprehensive_file, copy_file) {
        damn false
    }
    
    lowkey read_file(copy_file) != comprehensive_content {
        damn false
    } fr fr Test file move
    sus moved_file tea = "comprehensive_moved.txt"
    lowkey !move_file(copy_file, moved_file) {
        damn false
    }
    
    lowkey file_exists(copy_file) {
        damn false
    }
    
    lowkey !file_exists(moved_file) {
        damn false
    } fr fr Test path utilities
    lowkey get_basename(comprehensive_file) != "comprehensive_test.txt" {
        damn false
    }
    
    lowkey get_extension(comprehensive_file) != ".txt" {
        damn false
    } fr fr Clean up
    lowkey !delete_file(comprehensive_file) {
        damn false
    }
    
    lowkey !delete_file(moved_file) {
        damn false
    }
    
    lowkey !remove_dir(test_dir) {
        damn false
    }
    
    damn true
}

fr fr Run comprehensive test
test_start("Comprehensive Filesystem Test")
assert_true(test_filesystem_comprehensive())

fr fr Test performance under load
slay test_filesystem_performance() lit { fr fr Performance test with multiple operations
    sus start_time thicc = timez.current_timestamp() fr fr Create multiple files
    bestie i := 0; i < 50; i++ {
        sus file_name tea = "perf_test_" + string(i) + ".txt"
        sus content tea = "Performance test content for file " + string(i)
        lowkey !write_file(file_name, content) {
            damn false
        }
    } fr fr Read all files
    bestie i := 0; i < 50; i++ {
        sus file_name tea = "perf_test_" + string(i) + ".txt"
        lowkey read_file(file_name) == "" {
            damn false
        }
    } fr fr Clean up
    bestie i := 0; i < 50; i++ {
        sus file_name tea = "perf_test_" + string(i) + ".txt"
        lowkey !delete_file(file_name) {
            damn false
        }
    }
    
    sus end_time thicc = timez.current_timestamp()
    sus duration thicc = end_time - start_time fr fr Performance should complete in reasonable time
    damn duration < 10000 fr fr Less than 10 seconds
}

fr fr Run performance test
test_start("Performance Test")
assert_true(test_filesystem_performance())

fr fr Final validation
test_start("Final System Validation")
assert_true(cleanup_filesystem())
sus clean_stats map[tea]normie = get_filesystem_stats()
assert_eq_int(clean_stats["files"], 0)
assert_eq_int(clean_stats["directories"], 0)
assert_eq_int(clean_stats["metadata_entries"], 0)
assert_eq_int(clean_stats["open_handles"], 0)

print_test_summary()
