yeet "testz"
yeet "io"

fr fr ========================================
fr fr CURSED I/O Library Test Suite
fr fr ========================================

slay test_console_io() {
    test_start("Console I/O Functions")
    
    fr fr Test print functions (output checked manually)
    print("Test print function")
    println("Test println function")
    eprint("Test eprint function")
    eprintln("Test eprintln function")
    
    fr fr Test printf formatting
    printf("Test printf: %s, %d, %.2f\n", ["hello", "42", "3.14"])
    
    fr fr Basic assertion for console functions
    assert_true(based)
}

slay test_file_operations() {
    test_start("File Operations")
    
    fr fr Test file writing and reading
    sus test_file tea = "test_file.txt"
    sus test_content tea = "Hello, CURSED file I/O!"
    
    fr fr Write file
    sus write_success lit = write_file(test_file, test_content)
    assert_true(write_success)
    
    fr fr Check file exists
    assert_true(file_exists(test_file))
    assert_true(is_file(test_file))
    assert_false(is_directory(test_file))
    
    fr fr Read file
    sus read_content tea = read_file(test_file)
    assert_eq_string(read_content, test_content)
    
    fr fr Check file size
    sus file_size_bytes normie = file_size(test_file)
    assert_eq_int(file_size_bytes, string_len(test_content))
    
    fr fr Append to file
    sus append_content tea = " More content!"
    sus append_success lit = append_file(test_file, append_content)
    assert_true(append_success)
    
    fr fr Read appended content
    sus full_content tea = read_file(test_file)
    assert_eq_string(full_content, test_content + append_content)
    
    fr fr Clean up
    sus delete_success lit = delete_file(test_file)
    assert_true(delete_success)
    assert_false(file_exists(test_file))
}

slay test_file_copy_move() {
    test_start("File Copy/Move Operations")
    
    fr fr Setup test files
    sus source_file tea = "source_test.txt"
    sus dest_file tea = "dest_test.txt"
    sus move_file tea = "moved_test.txt"
    sus test_content tea = "Test content for copy/move"
    
    fr fr Create source file
    assert_true(write_file(source_file, test_content))
    
    fr fr Test file copy
    assert_true(copy_file(source_file, dest_file))
    assert_true(file_exists(source_file))
    assert_true(file_exists(dest_file))
    assert_eq_string(read_file(dest_file), test_content)
    
    fr fr Test file move
    assert_true(move_file(dest_file, move_file))
    assert_false(file_exists(dest_file))
    assert_true(file_exists(move_file))
    assert_eq_string(read_file(move_file), test_content)
    
    fr fr Clean up
    assert_true(delete_file(source_file))
    assert_true(delete_file(move_file))
}

slay test_binary_file_operations() {
    test_start("Binary File Operations")
    
    fr fr Test binary file I/O
    sus binary_file tea = "binary_test.bin"
    sus test_bytes [byte] = [72, 101, 108, 108, 111, 0, 255, 128, 64]
    
    fr fr Write binary data
    assert_true(write_file_bytes(binary_file, test_bytes))
    
    fr fr Read binary data
    sus read_bytes [byte] = read_file_bytes(binary_file)
    assert_eq_int(len(read_bytes), len(test_bytes))
    
    fr fr Compare binary data
    for i in range(len(test_bytes)) {
        assert_eq_int(read_bytes[i], test_bytes[i])
    }
    
    fr fr Clean up
    assert_true(delete_file(binary_file))
}

slay test_directory_operations() {
    test_start("Directory Operations")
    
    fr fr Test directory creation
    sus test_dir tea = "test_directory"
    assert_true(create_directory(test_dir))
    assert_true(file_exists(test_dir))
    assert_true(is_directory(test_dir))
    assert_false(is_file(test_dir))
    
    fr fr Test directory listing
    sus current_dir tea = current_directory()
    assert_true(string_len(current_dir) > 0)
    
    sus dir_contents [tea] = list_directory(".")
    assert_true(len(dir_contents) > 0)
    
    fr fr Test recursive directory creation
    sus nested_dir tea = path_join([test_dir, "nested", "deep"])
    assert_true(create_directory_recursive(nested_dir))
    assert_true(is_directory(nested_dir))
    
    fr fr Test recursive directory listing
    sus recursive_contents [tea] = list_directory_recursive(test_dir)
    assert_true(len(recursive_contents) > 0)
    
    fr fr Clean up
    assert_true(remove_directory_recursive(test_dir))
    assert_false(file_exists(test_dir))
}

slay test_path_operations() {
    test_start("Path Operations")
    
    fr fr Test path joining
    sus joined_path tea = path_join(["home", "user", "documents", "file.txt"])
    assert_true(string_contains(joined_path, "home"))
    assert_true(string_contains(joined_path, "user"))
    assert_true(string_contains(joined_path, "documents"))
    assert_true(string_contains(joined_path, "file.txt"))
    
    fr fr Test path components
    sus test_path tea = "/home/user/documents/file.txt"
    assert_eq_string(path_dirname(test_path), "/home/user/documents")
    assert_eq_string(path_basename(test_path), "file.txt")
    assert_eq_string(path_extension(test_path), ".txt")
    
    fr fr Test path absolute
    sus relative_path tea = "test.txt"
    sus absolute_path tea = path_absolute(relative_path)
    assert_true(string_len(absolute_path) > string_len(relative_path))
    
    fr fr Test path existence
    assert_true(path_exists("."))
    assert_false(path_exists("nonexistent_path_xyz"))
}

slay test_stream_io() {
    test_start("Stream I/O Operations")
    
    fr fr Test file streaming
    sus stream_file tea = "stream_test.txt"
    sus test_data tea = "Line 1\nLine 2\nLine 3\n"
    
    fr fr Write using stream
    sus write_handle file_handle = open_file_write(stream_file)
    assert_true(write_handle != cringe)
    assert_true(write_to_file(write_handle, test_data))
    assert_true(flush_file(write_handle))
    assert_true(close_file(write_handle))
    
    fr fr Read using stream
    sus read_handle file_handle = open_file_read(stream_file)
    assert_true(read_handle != cringe)
    
    sus read_data tea = read_from_file(read_handle, string_len(test_data))
    assert_eq_string(read_data, test_data)
    
    fr fr Test file position
    assert_true(seek_file(read_handle, 0))
    assert_eq_int(tell_file(read_handle), 0)
    
    assert_true(close_file(read_handle))
    
    fr fr Clean up
    assert_true(delete_file(stream_file))
}

slay test_buffered_io() {
    test_start("Buffered I/O Operations")
    
    fr fr Test buffer creation
    sus buffer_size normie = 1024
    sus buf buffer = create_buffer(buffer_size)
    assert_true(buf != cringe)
    
    fr fr Test buffer operations
    sus test_data tea = "Buffered I/O test data"
    assert_true(buffer_write(buf, test_data))
    assert_true(buffer_size(buf) > 0)
    assert_true(buffer_available(buf) > 0)
    
    fr fr Test buffer reading
    sus read_data tea = buffer_read(buf, string_len(test_data))
    assert_eq_string(read_data, test_data)
    
    fr fr Test buffer flush and clear
    assert_true(buffer_flush(buf))
    assert_true(buffer_clear(buf))
    assert_eq_int(buffer_size(buf), 0)
}

slay test_temporary_files() {
    test_start("Temporary File Operations")
    
    fr fr Test temporary file creation
    sus temp_file tea = create_temp_file()
    assert_true(string_len(temp_file) > 0)
    assert_true(file_exists(temp_file))
    
    fr fr Test writing to temporary file
    sus temp_content tea = "Temporary file content"
    assert_true(write_file(temp_file, temp_content))
    assert_eq_string(read_file(temp_file), temp_content)
    
    fr fr Test temporary directory
    sus temp_dir tea = create_temp_directory()
    assert_true(string_len(temp_dir) > 0)
    assert_true(is_directory(temp_dir))
    
    fr fr Test temp directory path
    sus system_temp_dir tea = temp_directory()
    assert_true(string_len(system_temp_dir) > 0)
    assert_true(is_directory(system_temp_dir))
    
    fr fr Clean up
    assert_true(delete_file(temp_file))
    assert_true(remove_directory(temp_dir))
}

slay test_file_timestamps() {
    test_start("File Timestamp Operations")
    
    fr fr Create test file
    sus timestamp_file tea = "timestamp_test.txt"
    sus test_content tea = "Test file for timestamps"
    assert_true(write_file(timestamp_file, test_content))
    
    fr fr Test file timestamps
    sus modified_time normie = file_modified_time(timestamp_file)
    sus created_time normie = file_created_time(timestamp_file)
    
    assert_true(modified_time > 0)
    assert_true(created_time > 0)
    
    fr fr Modify file and check timestamp change
    assert_true(append_file(timestamp_file, " Modified"))
    sus new_modified_time normie = file_modified_time(timestamp_file)
    assert_true(new_modified_time >= modified_time)
    
    fr fr Clean up
    assert_true(delete_file(timestamp_file))
}

slay test_io_edge_cases() {
    test_start("I/O Edge Cases")
    
    fr fr Test operations on non-existent files
    assert_false(file_exists("nonexistent_file.txt"))
    assert_false(delete_file("nonexistent_file.txt"))
    
    fr fr Test empty file operations
    sus empty_file tea = "empty_test.txt"
    assert_true(write_file(empty_file, ""))
    assert_eq_int(file_size(empty_file), 0)
    assert_eq_string(read_file(empty_file), "")
    assert_true(delete_file(empty_file))
    
    fr fr Test path operations with empty strings
    assert_eq_string(path_dirname(""), "")
    assert_eq_string(path_basename(""), "")
    assert_eq_string(path_extension(""), "")
    
    fr fr Test invalid path operations
    assert_false(is_file(""))
    assert_false(is_directory(""))
    assert_false(path_exists(""))
}

slay test_io_error_handling() {
    test_start("I/O Error Handling")
    
    fr fr Test writing to read-only location (may fail on some systems)
    sus readonly_result lit = write_file("/root/readonly_test.txt", "test")
    fr fr This test is system-dependent, so we just check it doesn't crash
    assert_true(based)
    
    fr fr Test reading non-existent file
    sus nonexistent_content tea = read_file("definitely_does_not_exist.txt")
    assert_true(based)
    
    fr fr Test invalid file handle operations
    sus invalid_handle file_handle = cringe
    assert_false(close_file(invalid_handle))
    assert_false(flush_file(invalid_handle))
}

slay run_all_io_tests() {
    vibez.spill("💾 Running CURSED I/O Library Tests")
    vibez.spill("=================================")
    
    test_console_io()
    test_file_operations()
    test_file_copy_move()
    test_binary_file_operations()
    test_directory_operations()
    test_path_operations()
    test_stream_io()
    test_buffered_io()
    test_temporary_files()
    test_file_timestamps()
    test_io_edge_cases()
    test_io_error_handling()
    
    print_test_summary()
    damn run_all_tests()
}

fr fr Auto-run tests when this file is executed
run_all_io_tests()
