yeet "testz"
yeet "filesystem_complete"

test_start("filesystem_complete Tests")

// Test File Information
test_case("File Information Retrieval") {
    sus test_file tea = "/tmp/cursed_fs_test.txt"
    sus content tea = "Hello, CURSED filesystem!"
    
    // Create test file
    sus success lit = fs_write_file(test_file, content)
    assert(success)
    
    // Get file info
    sus info FileInfo = fs_get_file_info(test_file)
    
    assert_eq_string(info.name, "cursed_fs_test.txt")
    assert(info.is_file)
    assert(!info.is_dir)
    assert_eq_int(info.size, len(content))
    assert(info.modified_time > 0)
    assert(info.created_time > 0)
    
    // Cleanup
    fs_delete_file(test_file)
}

// Test Directory Operations
test_case("Directory Creation and Listing") {
    sus test_dir tea = "/tmp/cursed_test_dir"
    sus test_file tea = test_dir + "/test_file.txt"
    
    // Create directory
    sus success lit = fs_create_dir(test_dir)
    assert(success)
    
    // Verify directory exists
    assert(fs_exists(test_dir))
    sus info FileInfo = fs_get_file_info(test_dir)
    assert(info.is_dir)
    
    // Create file in directory
    fs_write_file(test_file, "test content")
    
    // List directory contents
    sus entries []tea = fs_list_dir(test_dir)
    assert_eq_int(len(entries), 1)
    assert_eq_string(entries[0], "test_file.txt")
    
    // Cleanup
    fs_delete_file(test_file)
    fs_delete_dir(test_dir)
}

// Test File Reading and Writing
test_case("File Reading and Writing") {
    sus test_file tea = "/tmp/cursed_rw_test.txt"
    sus content tea = "CURSED filesystem read/write test\nSecond line\nThird line"
    
    // Write file
    sus write_success lit = fs_write_file(test_file, content)
    assert(write_success)
    
    // Read file back
    sus read_content tea = fs_read_file(test_file)
    assert_eq_string(content, read_content)
    
    // Append to file
    sus append_text tea = "\nAppended line"
    sus append_success lit = fs_append_file(test_file, append_text)
    assert(append_success)
    
    // Read again to verify append
    sus final_content tea = fs_read_file(test_file)
    assert_eq_string(final_content, content + append_text)
    
    // Cleanup
    fs_delete_file(test_file)
}

// Test File Handle Operations
test_case("File Handle Operations") {
    sus test_file tea = "/tmp/cursed_handle_test.txt"
    sus content tea = "File handle testing content"
    
    // Open file for writing
    sus write_handle FileHandle = fs_open_file(test_file, FS_MODE_WRITE | FS_MODE_CREATE)
    assert(write_handle.is_open)
    assert_eq_string(write_handle.path, test_file)
    
    // Write using handle
    sus bytes_written normie = fs_write_handle(write_handle, content)
    assert_eq_int(bytes_written, len(content))
    
    // Close write handle
    fs_close_handle(write_handle)
    assert(!write_handle.is_open)
    
    // Open file for reading
    sus read_handle FileHandle = fs_open_file(test_file, FS_MODE_READ)
    assert(read_handle.is_open)
    
    // Read using handle
    sus buffer []byte = fs_allocate_buffer(256)
    sus bytes_read normie = fs_read_handle(read_handle, buffer)
    assert_eq_int(bytes_read, len(content))
    
    sus read_content tea = string_from_bytes(buffer, bytes_read)
    assert_eq_string(read_content, content)
    
    // Close read handle
    fs_close_handle(read_handle)
    
    // Cleanup
    fs_delete_file(test_file)
}

// Test Path Operations
test_case("Path Operations") {
    sus path tea = "/home/user/documents/file.txt"
    
    sus dirname tea = fs_path_dirname(path)
    assert_eq_string(dirname, "/home/user/documents")
    
    sus basename tea = fs_path_basename(path)
    assert_eq_string(basename, "file.txt")
    
    sus extension tea = fs_path_extension(path)
    assert_eq_string(extension, ".txt")
    
    sus stem tea = fs_path_stem(path)
    assert_eq_string(stem, "file")
    
    sus joined tea = fs_path_join("/tmp", "subdir", "file.dat")
    assert_eq_string(joined, "/tmp/subdir/file.dat")
    
    sus absolute tea = fs_path_absolute("../relative/path")
    assert(string_starts_with(absolute, "/"))
    
    sus canonical tea = fs_path_canonical("/tmp/../tmp/./file")
    assert_eq_string(canonical, "/tmp/file")
}

// Test Permissions and Metadata
test_case("File Permissions and Metadata") {
    sus test_file tea = "/tmp/cursed_perms_test.txt"
    
    // Create test file
    fs_write_file(test_file, "permissions test")
    
    // Get current permissions
    sus info FileInfo = fs_get_file_info(test_file)
    sus original_perms normie = info.permissions
    
    // Set new permissions (read-only)
    sus perm_success lit = fs_set_permissions(test_file, FS_PERM_READ | FS_PERM_OWNER_READ)
    assert(perm_success)
    
    // Verify permissions changed
    info = fs_get_file_info(test_file)
    assert(info.permissions != original_perms)
    
    // Test readable/writable checks
    assert(fs_is_readable(test_file))
    assert(!fs_is_writable(test_file))
    
    // Restore original permissions
    fs_set_permissions(test_file, original_perms)
    
    // Cleanup
    fs_delete_file(test_file)
}

// Test Directory Iteration
test_case("Directory Iteration") {
    sus test_dir tea = "/tmp/cursed_iter_test"
    
    // Create test directory structure
    fs_create_dir(test_dir)
    fs_write_file(test_dir + "/file1.txt", "content1")
    fs_write_file(test_dir + "/file2.dat", "content2")
    fs_create_dir(test_dir + "/subdir")
    fs_write_file(test_dir + "/subdir/file3.log", "content3")
    
    // Iterate directory (non-recursive)
    sus iterator DirIterator = fs_iter_dir(test_dir, false)
    sus entries []tea = []
    
    bestie (fs_iter_has_next(iterator)) {
        sus entry tea = fs_iter_next(iterator)
        entries = array_append(entries, entry)
    }
    
    assert_eq_int(len(entries), 3)  // file1.txt, file2.dat, subdir
    
    fs_iter_close(iterator)
    
    // Recursive iteration
    sus recursive_iterator DirIterator = fs_iter_dir(test_dir, true)
    sus recursive_entries []tea = []
    
    bestie (fs_iter_has_next(recursive_iterator)) {
        sus entry tea = fs_iter_next(recursive_iterator)
        recursive_entries = array_append(recursive_entries, entry)
    }
    
    assert(len(recursive_entries) >= 4)  // Should include file3.log
    
    fs_iter_close(recursive_iterator)
    
    // Cleanup
    fs_delete_file(test_dir + "/subdir/file3.log")
    fs_delete_dir(test_dir + "/subdir")
    fs_delete_file(test_dir + "/file1.txt")
    fs_delete_file(test_dir + "/file2.dat")
    fs_delete_dir(test_dir)
}

// Test File Watching
test_case("File System Watching") {
    sus watch_dir tea = "/tmp/cursed_watch_test"
    sus test_file tea = watch_dir + "/watched_file.txt"
    
    // Create watch directory
    fs_create_dir(watch_dir)
    
    // Start watching directory
    sus watcher FileWatcher = fs_watch_dir(watch_dir, FS_WATCH_ALL)
    assert(watcher.is_active)
    
    // Create a file to trigger watch event
    fs_write_file(test_file, "watched content")
    
    // Check for events (with timeout)
    sus events []FileEvent = fs_watch_get_events(watcher, 1000)
    assert(len(events) > 0)
    
    sus found_create lit = false
    bestie (sus i normie = 0; i < len(events); i += 1) {
        ready (events[i].event_type == FS_EVENT_CREATE) {
            found_create = based
            assert(string_contains(events[i].path, "watched_file.txt"))
        }
    }
    assert(found_create)
    
    // Stop watching
    fs_watch_stop(watcher)
    
    // Cleanup
    fs_delete_file(test_file)
    fs_delete_dir(watch_dir)
}

// Test Error Handling
test_case("Error Handling") {
    sus nonexistent_file tea = "/tmp/does_not_exist.txt"
    
    // Try to read non-existent file
    sus content tea = fs_read_file(nonexistent_file)
    assert_eq_string(content, "")
    
    sus last_error normie = fs_get_last_error()
    assert(last_error != 0)
    
    sus error_msg tea = fs_get_error_message(last_error)
    assert(string_length(error_msg) > 0)
    
    // Try to delete non-existent file
    sus delete_success lit = fs_delete_file(nonexistent_file)
    assert(!delete_success)
    
    // Try to open read-only directory for writing
    sus readonly_handle FileHandle = fs_open_file("/etc", FS_MODE_WRITE)
    assert(!readonly_handle.is_open)
}

print_test_summary()
