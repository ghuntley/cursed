fr fr CURSED Filesystem Module Tests
fr fr Comprehensive testing of all filesystem functions
fr fr Tests both interpretation and compilation modes

yeet "testz"
yeet "fs"

fr fr ================================
fr fr Basic File Operations Tests
fr fr ================================

test_start("File Operations - Basic")

fr fr Test file creation and deletion
sus test_file tea = "test_file.txt"
sus test_content tea = "Hello, CURSED filesystem!"

fr fr Write and read file
assert_true(write_file(test_file, test_content))
assert_true(file_exists(test_file))
assert_eq_string(read_file(test_file), test_content)

fr fr Clean up
assert_true(delete_file(test_file))
assert_false(file_exists(test_file))

test_start("Directory Operations - Basic")

fr fr Test directory creation and removal
sus test_dir tea = "test_directory"
assert_true(create_dir(test_dir))
assert_true(file_exists(test_dir))
assert_true(is_dir(test_dir))
assert_false(is_file(test_dir))

fr fr Clean up
assert_true(remove_dir(test_dir))

fr fr ================================
fr fr File Information Tests
fr fr ================================

test_start("File Information - Size and Type")

fr fr Create test file for info testing
sus info_file tea = "info_test.txt"
sus info_content tea = "Test content for file info"
assert_true(write_file(info_file, info_content))

fr fr Test file size
sus expected_size thicc = 27  fr fr Length of test content
assert_eq_int(get_file_size(info_file), expected_size)

fr fr Test file type detection
assert_true(is_file(info_file))
assert_false(is_dir(info_file))

fr fr Clean up
assert_true(delete_file(info_file))

fr fr ================================
fr fr Path Utilities Tests
fr fr ================================

test_start("Path Utilities - Extension and Basename")

fr fr Test file extension extraction
assert_eq_string(get_extension("test.txt"), ".txt")
assert_eq_string(get_extension("archive.tar.gz"), ".gz")
assert_eq_string(get_extension("no_extension"), "")
assert_eq_string(get_extension("path/to/file.cpp"), ".cpp")

fr fr Test basename extraction
assert_eq_string(get_basename("test.txt"), "test.txt")
assert_eq_string(get_basename("path/to/file.cpp"), "file.cpp")
assert_eq_string(get_basename("/absolute/path/filename"), "filename")
assert_eq_string(get_basename("just_filename"), "just_filename")

fr fr ================================
fr fr Timestamp Functions Tests
fr fr ================================

test_start("Timestamp Functions - Get Times")

fr fr Create test file for timestamp testing
sus timestamp_file tea = "timestamp_test.txt"
assert_true(write_file(timestamp_file, "Timestamp test content"))

fr fr Test timestamp retrieval
sus created_time thicc = get_created_time(timestamp_file)
sus modified_time thicc = get_modified_time(timestamp_file)
sus accessed_time thicc = get_accessed_time(timestamp_file)

fr fr Timestamps should be non-zero for existing files
assert_true(created_time > 0)
assert_true(modified_time > 0)
assert_true(accessed_time > 0)

fr fr Test timestamp for non-existent file
assert_eq_int(get_created_time("nonexistent.txt"), 0)
assert_eq_int(get_modified_time("nonexistent.txt"), 0)
assert_eq_int(get_accessed_time("nonexistent.txt"), 0)

fr fr Test setting modification time
assert_true(set_modified_time(timestamp_file, 1704067200))
assert_false(set_modified_time("nonexistent.txt", 1704067200))

fr fr Clean up
assert_true(delete_file(timestamp_file))

fr fr ================================
fr fr Permission Functions Tests
fr fr ================================

test_start("Permission Functions - Basic Permissions")

fr fr Create test file for permission testing
sus perm_file tea = "permission_test.txt"
assert_true(write_file(perm_file, "Permission test content"))

fr fr Test getting permissions
sus perms normie = get_permissions(perm_file)
assert_true(perms > 0)
assert_true(perms <= 777)

fr fr Test setting permissions
assert_true(set_permissions(perm_file, 755))
assert_false(set_permissions("nonexistent.txt", 755))

fr fr Test invalid permissions
assert_false(set_permissions(perm_file, 999))
assert_false(set_permissions(perm_file, -1))

fr fr Clean up
assert_true(delete_file(perm_file))

test_start("Permission Functions - Read/Write/Execute")

fr fr Create test file for permission checking
sus rwx_file tea = "rwx_test.txt"
assert_true(write_file(rwx_file, "RWX test content"))

fr fr Test permission checking functions
assert_true(is_readable(rwx_file))
assert_true(is_writable(rwx_file))

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
assert_eq_int(info.size, 21)  fr fr Length of metadata_content
assert_false(info.is_dir)
assert_true(info.modified_time > 0)
assert_true(info.permissions > 0)

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
assert_true(metadata.permissions > 0)
assert_true(metadata.owner_id > 0)
assert_true(metadata.group_id > 0)

fr fr Clean up
assert_true(delete_file(metadata_file))

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
assert_false(is_system_file("normal_file.txt"))

fr fr ================================
fr fr Directory Operations Tests
fr fr ================================

test_start("Directory Operations - Advanced")

fr fr Create test directory
sus test_dir tea = "advanced_test_dir"
assert_true(create_dir(test_dir))

fr fr Test directory properties
assert_true(is_dir(test_dir))
assert_false(is_file(test_dir))

fr fr Test directory permissions
sus dir_perms normie = get_permissions(test_dir)
assert_eq_int(dir_perms, 755)  fr fr Directory permissions

fr fr Test directory metadata
sus dir_metadata FileMetadata = get_file_metadata(test_dir)
assert_true(dir_metadata.is_dir)
assert_false(dir_metadata.is_file)

fr fr Clean up
assert_true(remove_dir(test_dir))

fr fr ================================
fr fr Error Handling Tests
fr fr ================================

test_start("Error Handling - Non-existent Files")

fr fr Test operations on non-existent files
assert_false(file_exists("definitely_does_not_exist.txt"))
assert_eq_string(read_file("nonexistent.txt"), "")
assert_eq_int(get_file_size("nonexistent.txt"), 0)
assert_false(delete_file("nonexistent.txt"))

fr fr Test permission operations on non-existent files
assert_eq_int(get_permissions("nonexistent.txt"), 0)
assert_false(set_permissions("nonexistent.txt", 644))

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

fr fr ================================
fr fr Performance Tests
fr fr ================================

test_start("Performance - Large Operations")

fr fr Test with larger file content
sus large_content tea = "Large file content for testing performance and handling of bigger files with more data"
sus large_file tea = "large_test.txt"

assert_true(write_file(large_file, large_content))
assert_true(file_exists(large_file))
assert_eq_string(read_file(large_file), large_content)

fr fr Test file operations on larger file
sus large_info FileInfo = get_file_info(large_file)
assert_true(large_info.size > 50)
assert_eq_string(large_info.name, "large_test.txt")

fr fr Clean up
assert_true(delete_file(large_file))

fr fr ================================
fr fr Test Summary
fr fr ================================

print_test_summary()

fr fr ================================
fr fr Additional Test Functions
fr fr ================================

fr fr Test function to verify both modes work identically
slay test_filesystem_comprehensive() lit {
    fr fr This function can be called to run comprehensive tests
    fr fr in both interpretation and compilation modes
    
    fr fr Test basic file operations
    sus test_file tea = "comprehensive_test.txt"
    sus test_content tea = "Comprehensive test content"
    
    lowkey !write_file(test_file, test_content) {
        damn cap
    }
    
    lowkey !file_exists(test_file) {
        damn cap
    }
    
    lowkey read_file(test_file) != test_content {
        damn cap
    }
    
    fr fr Test file information
    sus info FileInfo = get_file_info(test_file)
    lowkey info.name != "comprehensive_test.txt" {
        damn cap
    }
    
    fr fr Test permissions
    sus perms normie = get_permissions(test_file)
    lowkey perms <= 0 {
        damn cap
    }
    
    fr fr Test timestamps
    sus mod_time thicc = get_modified_time(test_file)
    lowkey mod_time <= 0 {
        damn cap
    }
    
    fr fr Clean up
    lowkey !delete_file(test_file) {
        damn cap
    }
    
    damn based
}

fr fr Run comprehensive test
test_start("Comprehensive Filesystem Test")
assert_true(test_filesystem_comprehensive())
