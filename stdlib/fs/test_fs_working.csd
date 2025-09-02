fr fr Working CURSED Filesystem Module Tests
fr fr Tests filesystem operations using main function

yeet "fs"

slay test_file_operations() {
    vibez.spill("Testing file operations...")
    
    fr fr Test 1: Write file
    sus test_path tea = "test_fs_working.txt"
    sus test_content tea = "Hello, CURSED filesystem!"
    
    vibez.spill("Writing file: " + test_path)
    sus write_success lit = fs.write_file(test_path, test_content)
    vibez.spill("Write success: " + tea(write_success))
    
    fr fr Test 2: Check file exists
    vibez.spill("Checking if file exists...")
    sus exists lit = fs.file_exists(test_path)
    vibez.spill("File exists: " + tea(exists))
    
    fr fr Test 3: Read file content
    vibez.spill("Reading file content...")
    sus read_content tea = fs.read_file(test_path)
    vibez.spill("Read content: " + read_content)
    
    fr fr Test 4: Get file size
    vibez.spill("Getting file size...")
    sus file_size thicc = fs.get_file_size(test_path)
    vibez.spill("File size: " + tea(file_size))
    
    fr fr Test 5: Delete file
    vibez.spill("Deleting file...")
    sus delete_success lit = fs.delete_file(test_path)
    vibez.spill("Delete success: " + tea(delete_success))
    
    fr fr Test 6: Verify file no longer exists
    vibez.spill("Checking if file exists after deletion...")
    sus exists_after_delete lit = fs.file_exists(test_path)
    vibez.spill("File exists after delete: " + tea(exists_after_delete))
}

slay test_directory_operations() {
    vibez.spill("Testing directory operations...")
    
    fr fr Test 1: Create directory
    sus test_dir tea = "test_fs_dir"
    vibez.spill("Creating directory: " + test_dir)
    sus create_success lit = fs.create_dir(test_dir)
    vibez.spill("Create directory success: " + tea(create_success))
    
    fr fr Test 2: Check directory exists
    vibez.spill("Checking if directory exists...")
    sus dir_exists lit = fs.file_exists(test_dir)
    vibez.spill("Directory exists: " + tea(dir_exists))
    
    fr fr Test 3: List directory contents
    vibez.spill("Listing directory contents...")
    sus files tea[value] = fs.list_dir(test_dir)
    vibez.spill("Directory file count: " + tea(files.length))
    
    fr fr Test 4: Remove directory
    vibez.spill("Removing directory...")
    sus remove_success lit = fs.remove_dir(test_dir)
    vibez.spill("Remove directory success: " + tea(remove_success))
}

slay test_path_utilities() {
    vibez.spill("Testing path utilities...")
    
    fr fr Test 1: Join paths
    sus base tea = "/home/user"
    sus component tea = "documents"
    sus joined tea = fs.join_path(base, component)
    vibez.spill("Joined path: " + joined)
    
    fr fr Test 2: Get file extension
    sus filename tea = "document.txt"
    sus extension tea = fs.get_extension(filename)
    vibez.spill("Extension: " + extension)
    
    fr fr Test 3: Get basename
    sus full_path tea = "/home/user/document.txt"
    sus basename tea = fs.get_basename(full_path)
    vibez.spill("Basename: " + basename)
}

slay test_error_handling() {
    vibez.spill("Testing error handling...")
    
    fr fr Test 1: Read non-existent file
    sus nonexistent_content tea = fs.read_file("nonexistent_file.txt")
    vibez.spill("Non-existent file content length: " + tea(nonexistent_content.length))
    
    fr fr Test 2: Check non-existent file
    sus nonexistent_exists lit = fs.file_exists("nonexistent_file.txt")
    vibez.spill("Non-existent file exists: " + tea(nonexistent_exists))
    
    fr fr Test 3: Delete non-existent file
    sus delete_nonexistent lit = fs.delete_file("nonexistent_file.txt")
    vibez.spill("Delete non-existent file result: " + tea(delete_nonexistent))
}

slay main() {
    vibez.spill("Starting CURSED Filesystem Module Tests...")
    
    test_file_operations()
    test_directory_operations()
    test_path_utilities()
    test_error_handling()
    
    vibez.spill("All filesystem tests completed!")
}

main()
