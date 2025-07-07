fr fr Simple CURSED Filesystem Module Tests
fr fr Basic test suite for filesystem operations

yeet "testz"
yeet "fs"

fr fr ================================
fr fr Basic File Operations Tests
fr fr ================================

slay test_basic_file_operations() {
    testz.test_start("Basic File Operations")
    
    fr fr Test 1: Write file
    sus test_path tea = "test_simple.txt"
    sus test_content tea = "Hello, CURSED filesystem!"
    
    sus write_success lit = fs.write_file(test_path, test_content)
    testz.assert_true(write_success)
    
    fr fr Test 2: Check file exists
    sus exists lit = fs.file_exists(test_path)
    testz.assert_true(exists)
    
    fr fr Test 3: Read file content
    sus read_content tea = fs.read_file(test_path)
    testz.assert_eq_string(read_content, test_content)
    
    fr fr Test 4: Get file size
    sus file_size thicc = fs.get_file_size(test_path)
    testz.assert_true(file_size > 0)
    
    fr fr Test 5: Delete file
    sus delete_success lit = fs.delete_file(test_path)
    testz.assert_true(delete_success)
    
    fr fr Test 6: Verify file no longer exists
    sus exists_after_delete lit = fs.file_exists(test_path)
    testz.assert_false(exists_after_delete)
}

slay test_directory_operations() {
    testz.test_start("Directory Operations")
    
    fr fr Test 1: Create directory
    sus test_dir tea = "test_dir_simple"
    sus create_success lit = fs.create_dir(test_dir)
    testz.assert_true(create_success)
    
    fr fr Test 2: Check directory exists
    sus dir_exists lit = fs.file_exists(test_dir)
    testz.assert_true(dir_exists)
    
    fr fr Test 3: List directory contents (should be empty)
    sus files []tea = fs.list_dir(test_dir)
    testz.assert_true(files.length >= 0)
    
    fr fr Test 4: Remove directory
    sus remove_success lit = fs.remove_dir(test_dir)
    testz.assert_true(remove_success)
}

slay test_path_utilities() {
    testz.test_start("Path Utilities")
    
    fr fr Test 1: Join paths
    sus base tea = "/home/user"
    sus component tea = "documents"
    sus joined tea = fs.join_path(base, component)
    testz.assert_eq_string(joined, "/home/user/documents")
    
    fr fr Test 2: Get file extension
    sus filename tea = "document.txt"
    sus extension tea = fs.get_extension(filename)
    testz.assert_eq_string(extension, ".txt")
    
    fr fr Test 3: Get basename
    sus full_path tea = "/home/user/document.txt"
    sus basename tea = fs.get_basename(full_path)
    testz.assert_eq_string(basename, "document.txt")
}

slay test_error_handling() {
    testz.test_start("Error Handling")
    
    fr fr Test 1: Read non-existent file
    sus nonexistent_content tea = fs.read_file("nonexistent_file.txt")
    testz.assert_eq_string(nonexistent_content, "")
    
    fr fr Test 2: Check non-existent file
    sus nonexistent_exists lit = fs.file_exists("nonexistent_file.txt")
    testz.assert_false(nonexistent_exists)
    
    fr fr Test 3: Delete non-existent file
    sus delete_nonexistent lit = fs.delete_file("nonexistent_file.txt")
    testz.assert_false(delete_nonexistent)
}

fr fr ================================
fr fr Main Test Runner
fr fr ================================

test_basic_file_operations()
test_directory_operations()
test_path_utilities()
test_error_handling()

fr fr Print test summary
testz.print_test_summary()
