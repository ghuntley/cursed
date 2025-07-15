# CURSED Filesystem Module Tests
# Comprehensive testing of all filesystem functions
# Tests both interpretation and compilation modes

yeet "testz"
yeet "fs"

# ================================
# Basic File Operations Tests
# ================================

test_start("File Operations - Basic")

# Test file creation and deletion
sus test_file tea = "test_file.txt"
sus test_content tea = "Hello, CURSED filesystem!"

# Write and read file
assert_true(write_file(test_file, test_content))
assert_true(file_exists(test_file))
assert_eq_string(read_file(test_file), test_content)

# Clean up
assert_true(delete_file(test_file))
assert_false(file_exists(test_file))

test_start("Directory Operations - Basic")

# Test directory creation and removal
sus test_dir tea = "test_directory"
assert_true(create_dir(test_dir))
assert_true(file_exists(test_dir))
assert_true(is_dir(test_dir))
assert_false(is_file(test_dir))

# Clean up
assert_true(remove_dir(test_dir))

# ================================
# File Information Tests
# ================================

test_start("File Information - Size and Type")

# Create test file for info testing
sus info_file tea = "info_test.txt"
sus info_content tea = "Test content for file info"
assert_true(write_file(info_file, info_content))

# Test file size
sus expected_size thicc = 27  # Length of test content
assert_eq_int(get_file_size(info_file), expected_size)

# Test file type detection
assert_true(is_file(info_file))
assert_false(is_dir(info_file))

# Clean up
assert_true(delete_file(info_file))

# ================================
# Path Utilities Tests
# ================================

test_start("Path Utilities - Extension and Basename")

# Test file extension extraction
assert_eq_string(get_extension("test.txt"), ".txt")
assert_eq_string(get_extension("archive.tar.gz"), ".gz")
assert_eq_string(get_extension("no_extension"), "")
assert_eq_string(get_extension("path/to/file.cpp"), ".cpp")

# Test basename extraction
assert_eq_string(get_basename("test.txt"), "test.txt")
assert_eq_string(get_basename("path/to/file.cpp"), "file.cpp")
assert_eq_string(get_basename("/absolute/path/filename"), "filename")
assert_eq_string(get_basename("just_filename"), "just_filename")

# ================================
# Timestamp Functions Tests
# ================================

test_start("Timestamp Functions - Get Times")

# Create test file for timestamp testing
sus timestamp_file tea = "timestamp_test.txt"
assert_true(write_file(timestamp_file, "Timestamp test content"))

# Test timestamp retrieval
sus created_time thicc = get_created_time(timestamp_file)
sus modified_time thicc = get_modified_time(timestamp_file)
sus accessed_time thicc = get_accessed_time(timestamp_file)

# Timestamps should be non-zero for existing files
assert_true(created_time > 0)
assert_true(modified_time > 0)
assert_true(accessed_time > 0)

# Test timestamp for non-existent file
assert_eq_int(get_created_time("nonexistent.txt"), 0)
assert_eq_int(get_modified_time("nonexistent.txt"), 0)
assert_eq_int(get_accessed_time("nonexistent.txt"), 0)

# Test setting modification time
assert_true(set_modified_time(timestamp_file, 1704067200))
assert_false(set_modified_time("nonexistent.txt", 1704067200))

# Clean up
assert_true(delete_file(timestamp_file))

# ================================
# Permission Functions Tests
# ================================

test_start("Permission Functions - Basic Permissions")

# Create test file for permission testing
sus perm_file tea = "permission_test.txt"
assert_true(write_file(perm_file, "Permission test content"))

# Test getting permissions
sus perms normie = get_permissions(perm_file)
assert_true(perms > 0)
assert_true(perms <= 777)

# Test setting permissions
assert_true(set_permissions(perm_file, 755))
assert_false(set_permissions("nonexistent.txt", 755))

# Test invalid permissions
assert_false(set_permissions(perm_file, 999))
assert_false(set_permissions(perm_file, -1))

# Clean up
assert_true(delete_file(perm_file))

test_start("Permission Functions - Read/Write/Execute")

# Create test file for permission checking
sus rwx_file tea = "rwx_test.txt"
assert_true(write_file(rwx_file, "RWX test content"))

# Test permission checking functions
assert_true(is_readable(rwx_file))
assert_true(is_writable(rwx_file))

# Test permission checking for non-existent file
assert_false(is_readable("nonexistent.txt"))
assert_false(is_writable("nonexistent.txt"))
assert_false(is_executable("nonexistent.txt"))

# Clean up
assert_true(delete_file(rwx_file))

# ================================
# File Metadata Tests
# ================================

test_start("File Metadata - Comprehensive Info")

# Create test file for metadata testing
sus metadata_file tea = "metadata_test.txt"
sus metadata_content tea = "Metadata test content"
assert_true(write_file(metadata_file, metadata_content))

# Test FileInfo structure
sus info FileInfo = get_file_info(metadata_file)
assert_eq_string(info.name, "metadata_test.txt")
assert_eq_int(info.size, 21)  # Length of metadata_content
assert_false(info.is_dir)
assert_true(info.modified_time > 0)
assert_true(info.permissions > 0)

# Test FileMetadata structure
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

# Clean up
assert_true(delete_file(metadata_file))

# ================================
# Special File Tests
# ================================

test_start("Special Files - Hidden and System")

# Test hidden file detection
assert_true(is_hidden(".hidden_file"))
assert_true(is_hidden("path/to/.hidden"))
assert_false(is_hidden("normal_file.txt"))
assert_false(is_hidden("not.hidden"))

# Test system file detection
assert_true(is_system_file("."))
assert_true(is_system_file(".."))
assert_true(is_system_file("/proc/version"))
assert_true(is_system_file("/sys/kernel"))
assert_false(is_system_file("normal_file.txt"))

# ================================
# Directory Operations Tests
# ================================

test_start("Directory Operations - Advanced")

# Create test directory
sus test_dir tea = "advanced_test_dir"
assert_true(create_dir(test_dir))

# Test directory properties
assert_true(is_dir(test_dir))
assert_false(is_file(test_dir))

# Test directory permissions
sus dir_perms normie = get_permissions(test_dir)
assert_eq_int(dir_perms, 755)  # Directory permissions

# Test directory metadata
sus dir_metadata FileMetadata = get_file_metadata(test_dir)
assert_true(dir_metadata.is_dir)
assert_false(dir_metadata.is_file)

# Clean up
assert_true(remove_dir(test_dir))

# ================================
# Error Handling Tests
# ================================

test_start("Error Handling - Non-existent Files")

# Test operations on non-existent files
assert_false(file_exists("definitely_does_not_exist.txt"))
assert_eq_string(read_file("nonexistent.txt"), "")
assert_eq_int(get_file_size("nonexistent.txt"), 0)
assert_false(delete_file("nonexistent.txt"))

# Test permission operations on non-existent files
assert_eq_int(get_permissions("nonexistent.txt"), 0)
assert_false(set_permissions("nonexistent.txt", 644))

# ================================
# Cross-Platform Compatibility Tests
# ================================

test_start("Cross-Platform - Path Handling")

# Test path utilities with different separators
assert_eq_string(get_basename("unix/path/file.txt"), "file.txt")
assert_eq_string(get_extension("unix/path/file.txt"), ".txt")

# Test edge cases
assert_eq_string(get_basename(""), "")
assert_eq_string(get_extension(""), "")
assert_eq_string(get_basename("file"), "file")
assert_eq_string(get_extension("file"), "")

# ================================
# Performance Tests
# ================================

test_start("Performance - Large Operations")

# Test with larger file content
sus large_content tea = "Large file content for testing performance and handling of bigger files with more data"
sus large_file tea = "large_test.txt"

assert_true(write_file(large_file, large_content))
assert_true(file_exists(large_file))
assert_eq_string(read_file(large_file), large_content)

# Test file operations on larger file
sus large_info FileInfo = get_file_info(large_file)
assert_true(large_info.size > 50)
assert_eq_string(large_info.name, "large_test.txt")

# Clean up
assert_true(delete_file(large_file))

# ================================
# Test Summary
# ================================

print_test_summary()

# ================================
# Additional Test Functions
# ================================

# Test function to verify both modes work identically
slay test_filesystem_comprehensive() lit {
    # This function can be called to run comprehensive tests
    # in both interpretation and compilation modes
    
    # Test basic file operations
    sus test_file tea = "comprehensive_test.txt"
    sus test_content tea = "Comprehensive test content"
    
    lowkey !write_file(test_file, test_content) {
        damn false
    }
    
    lowkey !file_exists(test_file) {
        damn false
    }
    
    lowkey read_file(test_file) != test_content {
        damn false
    }
    
    # Test file information
    sus info FileInfo = get_file_info(test_file)
    lowkey info.name != "comprehensive_test.txt" {
        damn false
    }
    
    # Test permissions
    sus perms normie = get_permissions(test_file)
    lowkey perms <= 0 {
        damn false
    }
    
    # Test timestamps
    sus mod_time thicc = get_modified_time(test_file)
    lowkey mod_time <= 0 {
        damn false
    }
    
    # Clean up
    lowkey !delete_file(test_file) {
        damn false
    }
    
    damn true
}

# Run comprehensive test
test_start("Comprehensive Filesystem Test")
assert_true(test_filesystem_comprehensive())
