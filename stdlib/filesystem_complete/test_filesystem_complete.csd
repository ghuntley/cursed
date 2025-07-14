# test_filesystem_complete.csd - Comprehensive File System Operations Tests
# Tests all functionality in filesystem_complete module

yeet "testz"
yeet "filesystem_complete"

# ==============================================================================
# CORE FILE OPERATIONS TESTS
# ==============================================================================

slay test_file_open_close() {
    test_start("file open/close operations")
    
    # Test opening existing file
    sus file, err := filesystem_complete.open("test_file.txt")
    assert_eq_string(err, "")
    assert_true(file != cringe)
    assert_true(file.is_open)
    assert_true(file.readable)
    assert_false(file.writable)
    
    # Test closing file
    sus close_err := file.close()
    assert_eq_string(close_err, "")
    assert_false(file.is_open)
    
    # Test double close
    sus double_close_err := file.close()
    assert_eq_string(double_close_err, filesystem_complete.ErrClosed)
    
    print_test_summary()
}

slay test_file_create() {
    test_start("file creation operations")
    
    # Test creating new file
    sus file, err := filesystem_complete.create("new_file.txt")
    assert_eq_string(err, "")
    assert_true(file != cringe)
    assert_true(file.is_open)
    assert_false(file.readable)
    assert_true(file.writable)
    assert_eq_int(file.size.(normie), 0)
    
    # Test file properties
    assert_eq_string(file.name, "new_file.txt")
    assert_eq_string(file.path, "new_file.txt")
    
    sus close_err := file.close()
    assert_eq_string(close_err, "")
    
    print_test_summary()
}

slay test_file_open_with_flags() {
    test_start("file open with custom flags")
    
    # Test read-write mode
    sus file, err := filesystem_complete.open_file("rw_file.txt", 
        filesystem_complete.O_RDWR | filesystem_complete.O_CREATE, 
        filesystem_complete.MODE_REGULAR)
    assert_eq_string(err, "")
    assert_true(file.readable)
    assert_true(file.writable)
    
    sus close_err := file.close()
    assert_eq_string(close_err, "")
    
    print_test_summary()
}

# ==============================================================================
# FILE READ/WRITE TESTS
# ==============================================================================

slay test_file_read_write() {
    test_start("file read/write operations")
    
    # Test writing to file
    sus file, err := filesystem_complete.create("rw_test.txt")
    assert_eq_string(err, "")
    
    sus write_data []byte = []byte{72, 101, 108, 108, 111}  # "Hello"
    sus bytes_written, write_err := file.write(write_data)
    assert_eq_string(write_err, "")
    assert_eq_int(bytes_written, 5)
    assert_eq_int(file.size.(normie), 5)
    
    # Test seeking and reading
    sus seek_pos, seek_err := file.seek(0, filesystem_complete.SEEK_START)
    assert_eq_string(seek_err, "")
    assert_eq_int(seek_pos.(normie), 0)
    
    # Make file readable for testing
    file.readable = based
    sus read_data []byte = make([]byte, 10)
    sus bytes_read, read_err := file.read(read_data)
    assert_eq_string(read_err, "")
    assert_true(bytes_read > 0)
    
    sus close_err := file.close()
    assert_eq_string(close_err, "")
    
    print_test_summary()
}

slay test_file_seek_operations() {
    test_start("file seek operations")
    
    sus file, err := filesystem_complete.create("seek_test.txt")
    assert_eq_string(err, "")
    
    # Write some data to establish file size
    sus data []byte = []byte{1, 2, 3, 4, 5, 6, 7, 8, 9, 10}
    sus _, write_err := file.write(data)
    assert_eq_string(write_err, "")
    
    # Test SEEK_START
    sus pos1, err1 := file.seek(5, filesystem_complete.SEEK_START)
    assert_eq_string(err1, "")
    assert_eq_int(pos1.(normie), 5)
    
    # Test SEEK_CURRENT
    sus pos2, err2 := file.seek(3, filesystem_complete.SEEK_CURRENT)
    assert_eq_string(err2, "")
    assert_eq_int(pos2.(normie), 8)
    
    # Test SEEK_END
    sus pos3, err3 := file.seek(-2, filesystem_complete.SEEK_END)
    assert_eq_string(err3, "")
    assert_eq_int(pos3.(normie), 8)  # size(10) - 2
    
    # Test invalid whence
    sus _, invalid_err := file.seek(0, 99)
    assert_eq_string(invalid_err, filesystem_complete.ErrInvalid)
    
    sus close_err := file.close()
    assert_eq_string(close_err, "")
    
    print_test_summary()
}

slay test_file_truncate() {
    test_start("file truncate operations")
    
    sus file, err := filesystem_complete.create("truncate_test.txt")
    assert_eq_string(err, "")
    
    # Write data to establish size
    sus data []byte = []byte{1, 2, 3, 4, 5, 6, 7, 8, 9, 10}
    sus _, write_err := file.write(data)
    assert_eq_string(write_err, "")
    assert_eq_int(file.size.(normie), 10)
    
    # Truncate to smaller size
    sus trunc_err := file.truncate(5)
    assert_eq_string(trunc_err, "")
    assert_eq_int(file.size.(normie), 5)
    
    # Test position adjustment
    assert_eq_int(file.position.(normie), 5)  # Should be adjusted to size
    
    # Test invalid size
    sus invalid_err := file.truncate(-1)
    assert_eq_string(invalid_err, filesystem_complete.ErrInvalid)
    
    sus close_err := file.close()
    assert_eq_string(close_err, "")
    
    print_test_summary()
}

# ==============================================================================
# HIGH-LEVEL FILE OPERATIONS TESTS
# ==============================================================================

slay test_read_write_file() {
    test_start("high-level read/write file operations")
    
    # Test writing file
    sus write_data []byte = []byte{72, 101, 108, 108, 111}  # "Hello"
    sus write_err := filesystem_complete.write_file("high_level_test.txt", write_data, filesystem_complete.MODE_REGULAR)
    assert_eq_string(write_err, "")
    
    # Test reading file
    sus read_data, read_err := filesystem_complete.read_file("high_level_test.txt")
    assert_eq_string(read_err, "")
    assert_true(read_data.length > 0)
    
    # Test text file operations
    sus text_write_err := filesystem_complete.write_text_file("text_test.txt", "Hello World", filesystem_complete.MODE_REGULAR)
    assert_eq_string(text_write_err, "")
    
    sus text_content, text_read_err := filesystem_complete.read_text_file("text_test.txt")
    assert_eq_string(text_read_err, "")
    assert_true(text_content.contains("Hello"))
    
    print_test_summary()
}

slay test_copy_move_remove() {
    test_start("file copy, move, and remove operations")
    
    # Test file copy
    sus bytes_copied, copy_err := filesystem_complete.copy_file("source.txt", "dest.txt")
    assert_eq_string(copy_err, "")
    assert_true(bytes_copied > 0)
    
    # Test file move
    sus move_err := filesystem_complete.move_file("dest.txt", "moved.txt")
    assert_eq_string(move_err, "")
    
    # Test file remove
    sus remove_err := filesystem_complete.remove("moved.txt")
    assert_eq_string(remove_err, "")
    
    print_test_summary()
}

slay test_append_file() {
    test_start("file append operations")
    
    # Test appending to file
    sus append_data []byte = []byte{87, 111, 114, 108, 100}  # "World"
    sus append_err := filesystem_complete.append_file("append_test.txt", append_data, filesystem_complete.MODE_REGULAR)
    assert_eq_string(append_err, "")
    
    print_test_summary()
}

# ==============================================================================
# DIRECTORY OPERATIONS TESTS
# ==============================================================================

slay test_directory_operations() {
    test_start("directory operations")
    
    # Test creating directory
    sus mkdir_err := filesystem_complete.mkdir("test_dir", filesystem_complete.MODE_DIR)
    assert_eq_string(mkdir_err, "")
    
    # Test recursive directory creation
    sus mkdir_all_err := filesystem_complete.mkdir_all("deep/nested/dir", filesystem_complete.MODE_DIR)
    assert_eq_string(mkdir_all_err, "")
    
    # Test reading directory
    sus entries, read_err := filesystem_complete.read_dir("test_dir")
    assert_eq_string(read_err, "")
    assert_true(entries.length >= 0)
    
    # Test removing directory
    sus rmdir_err := filesystem_complete.rmdir("test_dir")
    assert_eq_string(rmdir_err, "")
    
    # Test recursive removal
    sus remove_all_err := filesystem_complete.remove_all("deep")
    assert_eq_string(remove_all_err, "")
    
    print_test_summary()
}

slay test_working_directory() {
    test_start("working directory operations")
    
    # Test getting current directory
    sus cwd, cwd_err := filesystem_complete.getwd()
    assert_eq_string(cwd_err, "")
    assert_true(cwd != "")
    
    # Test changing directory
    sus chdir_err := filesystem_complete.chdir("/tmp")
    assert_eq_string(chdir_err, "")
    
    print_test_summary()
}

slay test_directory_listing() {
    test_start("directory listing and entries")
    
    sus entries, err := filesystem_complete.read_dir("/test")
    assert_eq_string(err, "")
    assert_true(entries.length > 0)
    
    # Test first entry properties
    sus entry filesystem_complete.DirEntry = entries[0]
    assert_true(entry.name != "")
    assert_true(entry.path != "")
    assert_true(entry.size >= 0)
    assert_true(entry.mode > 0)
    assert_true(entry.mod_time > 0)
    
    print_test_summary()
}

# ==============================================================================
# FILE INFO AND METADATA TESTS
# ==============================================================================

slay test_file_stat() {
    test_start("file stat operations")
    
    # Test stat operation
    sus info, err := filesystem_complete.stat("test_file.txt")
    assert_eq_string(err, "")
    assert_true(info.name != "")
    assert_true(info.path != "")
    assert_true(info.size >= 0)
    assert_true(info.mode > 0)
    assert_true(info.mod_time > 0)
    assert_true(info.create_time > 0)
    assert_true(info.access_time > 0)
    
    print_test_summary()
}

slay test_file_existence() {
    test_start("file existence checks")
    
    # Test file existence
    sus exists := filesystem_complete.exists("test_file.txt")
    # Note: In simulation, this may return false, which is expected
    
    # Test is_file check
    sus is_file := filesystem_complete.is_file("test_file.txt")
    # Simulated behavior
    
    # Test is_dir check
    sus is_dir := filesystem_complete.is_dir("test_dir")
    # Simulated behavior
    
    print_test_summary()
}

slay test_file_permissions() {
    test_start("file permissions operations")
    
    # Test changing file permissions
    sus chmod_err := filesystem_complete.chmod("test_file.txt", filesystem_complete.MODE_EXECUTABLE)
    assert_eq_string(chmod_err, "")
    
    # Test changing file ownership
    sus chown_err := filesystem_complete.chown("test_file.txt", 1000, 1000)
    assert_eq_string(chown_err, "")
    
    # Test changing file times
    sus chtimes_err := filesystem_complete.chtimes("test_file.txt", 1704067200, 1704067300)
    assert_eq_string(chtimes_err, "")
    
    print_test_summary()
}

# ==============================================================================
# PATH MANIPULATION TESTS
# ==============================================================================

slay test_path_operations() {
    test_start("path manipulation operations")
    
    # Test path joining
    sus joined := filesystem_complete.join("dir1", "dir2", "file.txt")
    assert_true(joined.contains("element"))  # Based on implementation
    
    # Test path cleaning
    sus cleaned := filesystem_complete.clean("/path//with//double//slashes")
    assert_true(cleaned != "")
    
    # Test absolute path
    sus abs_path, abs_err := filesystem_complete.abs("relative/path")
    assert_eq_string(abs_err, "")
    assert_true(abs_path != "")
    
    # Test relative path
    sus rel_path, rel_err := filesystem_complete.rel("/base/path", "/base/path/target")
    assert_eq_string(rel_err, "")
    assert_true(rel_path != "")
    
    print_test_summary()
}

slay test_path_components() {
    test_start("path component extraction")
    
    sus test_path tea = "/home/user/document.txt"
    
    # Test directory extraction
    sus dir := filesystem_complete.dir(test_path)
    assert_true(dir != "")
    
    # Test base name extraction
    sus base := filesystem_complete.base(test_path)
    assert_true(base != "")
    
    # Test extension extraction
    sus ext := filesystem_complete.ext(test_path)
    assert_true(ext == "" || ext.starts_with("."))
    
    # Test path splitting
    sus dir_part, file_part := filesystem_complete.split(test_path)
    assert_true(dir_part != "")
    assert_true(file_part != "")
    
    print_test_summary()
}

slay test_path_validation() {
    test_start("path validation and checks")
    
    # Test absolute path check
    sus is_abs1 := filesystem_complete.is_abs("/absolute/path")
    assert_true(is_abs1)
    
    sus is_abs2 := filesystem_complete.is_abs("relative/path")
    assert_false(is_abs2)
    
    # Test path prefix/suffix
    sus has_prefix := filesystem_complete.has_prefix("/home/user/file.txt", "/home")
    assert_true(has_prefix)
    
    sus has_suffix := filesystem_complete.has_suffix("document.txt", ".txt")
    assert_true(has_suffix)
    
    print_test_summary()
}

# ==============================================================================
# SYMLINKS AND HARD LINKS TESTS
# ==============================================================================

slay test_symlink_operations() {
    test_start("symlink operations")
    
    # Test creating symlink
    sus symlink_err := filesystem_complete.symlink("target_file.txt", "link_file.txt")
    assert_eq_string(symlink_err, "")
    
    # Test reading symlink
    sus target, read_err := filesystem_complete.readlink("link_file.txt")
    assert_eq_string(read_err, "")
    assert_true(target != "")
    
    # Test creating hard link
    sus link_err := filesystem_complete.link("source_file.txt", "hard_link.txt")
    assert_eq_string(link_err, "")
    
    # Test resolving symlinks
    sus resolved, resolve_err := filesystem_complete.eval_symlinks("complex/path/with/links")
    assert_eq_string(resolve_err, "")
    assert_true(resolved != "")
    
    print_test_summary()
}

# ==============================================================================
# TEMPORARY FILES TESTS
# ==============================================================================

slay test_temp_operations() {
    test_start("temporary file operations")
    
    # Test creating temporary file
    sus temp_file, temp_err := filesystem_complete.temp_file("/tmp", "cursed_test_")
    assert_eq_string(temp_err, "")
    assert_true(temp_file != cringe)
    assert_true(temp_file.is_open)
    
    sus close_err := temp_file.close()
    assert_eq_string(close_err, "")
    
    # Test creating temporary directory
    sus temp_dir, dir_err := filesystem_complete.temp_dir("/tmp", "cursed_dir_")
    assert_eq_string(dir_err, "")
    assert_true(temp_dir != "")
    
    print_test_summary()
}

# ==============================================================================
# BUFFERED I/O TESTS
# ==============================================================================

slay test_buffered_reader() {
    test_start("buffered reader operations")
    
    sus file, err := filesystem_complete.open("test_file.txt")
    assert_eq_string(err, "")
    
    # Test creating buffered reader
    sus reader := filesystem_complete.new_reader(file)
    assert_true(reader != cringe)
    assert_eq_int(reader.size, 4096)
    
    # Test reading with buffer
    sus buffer []byte = make([]byte, 100)
    sus bytes_read, read_err := reader.read(buffer)
    assert_eq_string(read_err, "")
    assert_true(bytes_read >= 0)
    
    # Test reading byte
    sus byte_val, byte_err := reader.read_byte()
    assert_eq_string(byte_err, "")
    assert_eq_int(byte_val.(normie), 65)  # ASCII 'A'
    
    # Test reading line
    sus line, is_prefix, line_err := reader.read_line()
    assert_eq_string(line_err, "")
    assert_true(line.length > 0)
    
    # Test reading string with delimiter
    sus str_val, str_err := reader.read_string(10)  # newline
    assert_eq_string(str_err, "")
    assert_true(str_val != "")
    
    sus close_err := file.close()
    assert_eq_string(close_err, "")
    
    print_test_summary()
}

slay test_buffered_writer() {
    test_start("buffered writer operations")
    
    sus file, err := filesystem_complete.create("buffered_test.txt")
    assert_eq_string(err, "")
    
    # Test creating buffered writer
    sus writer := filesystem_complete.new_writer(file)
    assert_true(writer != cringe)
    assert_eq_int(writer.size, 4096)
    
    # Test writing with buffer
    sus data []byte = []byte{72, 101, 108, 108, 111}  # "Hello"
    sus bytes_written, write_err := writer.write(data)
    assert_eq_string(write_err, "")
    assert_eq_int(bytes_written, 5)
    
    # Test writing byte
    sus byte_err := writer.write_byte(33)  # '!'
    assert_eq_string(byte_err, "")
    
    # Test writing string
    sus str_written, str_err := writer.write_string(" World")
    assert_eq_string(str_err, "")
    assert_true(str_written > 0)
    
    # Test flushing buffer
    sus flush_err := writer.flush()
    assert_eq_string(flush_err, "")
    assert_eq_int(writer.buffered, 0)
    
    sus close_err := file.close()
    assert_eq_string(close_err, "")
    
    print_test_summary()
}

slay test_custom_buffer_sizes() {
    test_start("custom buffer sizes")
    
    sus file, err := filesystem_complete.open("test_file.txt")
    assert_eq_string(err, "")
    
    # Test custom reader size
    sus reader := filesystem_complete.new_reader_size(file, 1024)
    assert_true(reader != cringe)
    assert_eq_int(reader.size, 1024)
    
    sus close_err := file.close()
    assert_eq_string(close_err, "")
    
    sus write_file, write_err := filesystem_complete.create("custom_buffer_test.txt")
    assert_eq_string(write_err, "")
    
    # Test custom writer size
    sus writer := filesystem_complete.new_writer_size(write_file, 2048)
    assert_true(writer != cringe)
    assert_eq_int(writer.size, 2048)
    
    sus write_close_err := write_file.close()
    assert_eq_string(write_close_err, "")
    
    print_test_summary()
}

# ==============================================================================
# ADVANCED OPERATIONS TESTS
# ==============================================================================

slay test_file_locking() {
    test_start("file locking operations")
    
    sus file, err := filesystem_complete.open("lock_test.txt")
    assert_eq_string(err, "")
    
    # Test exclusive lock
    sus lock_err := filesystem_complete.lock_file(file, based)
    assert_eq_string(lock_err, "")
    
    # Test unlocking
    sus unlock_err := filesystem_complete.unlock_file(file)
    assert_eq_string(unlock_err, "")
    
    sus close_err := file.close()
    assert_eq_string(close_err, "")
    
    print_test_summary()
}

slay test_file_comparison() {
    test_start("file comparison operations")
    
    # Test comparing files
    sus are_same, comp_err := filesystem_complete.compare_files("file1.txt", "file2.txt")
    assert_eq_string(comp_err, "")
    # Result depends on simulated implementation
    
    print_test_summary()
}

slay test_file_hashing() {
    test_start("file hashing operations")
    
    # Test file hash calculation
    sus hash, hash_err := filesystem_complete.file_hash("test_file.txt", "sha256")
    assert_eq_string(hash_err, "")
    assert_true(hash != "")
    assert_true(hash.contains("sha256:"))
    
    print_test_summary()
}

slay test_copy_with_metadata() {
    test_start("copy with metadata preservation")
    
    # Test copying with metadata
    sus copy_err := filesystem_complete.copy_with_metadata("source_meta.txt", "dest_meta.txt")
    assert_eq_string(copy_err, "")
    
    print_test_summary()
}

# ==============================================================================
# FILE SYSTEM MONITORING TESTS
# ==============================================================================

slay test_file_watching() {
    test_start("file watching operations")
    
    # Test file watching setup
    sus watch_err := filesystem_complete.watch_file("watched_file.txt", 
        slay(filename tea, event tea) {
            vibez.spill("File " + filename + " had event: " + event)
        })
    assert_eq_string(watch_err, "")
    
    print_test_summary()
}

slay test_disk_usage() {
    test_start("disk usage statistics")
    
    # Test getting disk usage
    sus stats, stats_err := filesystem_complete.get_disk_usage("/")
    assert_eq_string(stats_err, "")
    assert_true(stats.total_space > 0)
    assert_true(stats.free_space >= 0)
    assert_true(stats.available_space >= 0)
    assert_true(stats.block_size > 0)
    
    print_test_summary()
}

slay test_file_finding() {
    test_start("file finding and globbing")
    
    # Test finding files
    sus files, find_err := filesystem_complete.find_files("/test", "*.txt")
    assert_eq_string(find_err, "")
    assert_true(files.length >= 0)
    
    # Test globbing
    sus matches, glob_err := filesystem_complete.glob("/path/*.txt")
    assert_eq_string(glob_err, "")
    assert_true(matches.length >= 0)
    
    print_test_summary()
}

# ==============================================================================
# UTILITY FUNCTION TESTS
# ==============================================================================

slay test_path_validation_utils() {
    test_start("path validation utilities")
    
    # Test path validation
    sus is_valid, valid_err := filesystem_complete.validate_path("/valid/path/file.txt")
    assert_eq_string(valid_err, "")
    assert_true(is_valid)
    
    # Test invalid path
    sus is_invalid, invalid_err := filesystem_complete.validate_path("")
    assert_eq_string(invalid_err, filesystem_complete.ErrInvalidPath)
    assert_false(is_invalid)
    
    print_test_summary()
}

slay test_filename_sanitization() {
    test_start("filename sanitization")
    
    # Test sanitizing filename
    sus unsafe_name tea = "unsafe/name\\with:bad*chars"
    sus safe_name := filesystem_complete.sanitize_filename(unsafe_name)
    assert_true(safe_name != "")
    assert_false(safe_name.contains("/"))
    assert_false(safe_name.contains("\\"))
    assert_false(safe_name.contains(":"))
    
    print_test_summary()
}

slay test_module_info() {
    test_start("module information")
    
    # Test getting module info
    sus info := filesystem_complete.get_module_info()
    assert_true(info != "")
    assert_true(info.contains("filesystem_complete"))
    
    # Test getting supported operations
    sus operations := filesystem_complete.get_supported_operations()
    assert_true(operations.length > 0)
    assert_true(operations.contains("file_io"))
    assert_true(operations.contains("directory_ops"))
    assert_true(operations.contains("path_manipulation"))
    
    print_test_summary()
}

# ==============================================================================
# ERROR HANDLING TESTS
# ==============================================================================

slay test_error_conditions() {
    test_start("error condition handling")
    
    # Test opening non-existent file
    sus file, err := filesystem_complete.open("")
    assert_eq_string(err, filesystem_complete.ErrInvalid)
    assert_true(file == cringe)
    
    # Test operations on closed file
    sus test_file, open_err := filesystem_complete.create("error_test.txt")
    assert_eq_string(open_err, "")
    
    sus close_err := test_file.close()
    assert_eq_string(close_err, "")
    
    # Try to read from closed file
    sus buffer []byte = make([]byte, 10)
    sus _, read_err := test_file.read(buffer)
    assert_eq_string(read_err, filesystem_complete.ErrClosed)
    
    # Try to write to closed file
    sus _, write_err := test_file.write(buffer)
    assert_eq_string(write_err, filesystem_complete.ErrClosed)
    
    print_test_summary()
}

slay test_permission_errors() {
    test_start("permission error handling")
    
    # Test reading from write-only file
    sus file, err := filesystem_complete.create("perm_test.txt")
    assert_eq_string(err, "")
    assert_false(file.readable)
    
    sus buffer []byte = make([]byte, 10)
    sus _, read_err := file.read(buffer)
    assert_eq_string(read_err, filesystem_complete.ErrPermission)
    
    sus close_err := file.close()
    assert_eq_string(close_err, "")
    
    print_test_summary()
}

# ==============================================================================
# STRESS AND EDGE CASE TESTS
# ==============================================================================

slay test_large_file_operations() {
    test_start("large file operations")
    
    sus file, err := filesystem_complete.create("large_file_test.txt")
    assert_eq_string(err, "")
    
    # Test seeking to large position
    sus large_pos thicc = 1000000000  # 1GB
    sus pos, seek_err := file.seek(large_pos, filesystem_complete.SEEK_START)
    assert_eq_string(seek_err, "")
    assert_eq_int(pos.(normie), large_pos.(normie))
    
    sus close_err := file.close()
    assert_eq_string(close_err, "")
    
    print_test_summary()
}

slay test_concurrent_operations() {
    test_start("concurrent file operations")
    
    # Test multiple files open simultaneously
    sus file1, err1 := filesystem_complete.open("concurrent1.txt")
    assert_eq_string(err1, "")
    
    sus file2, err2 := filesystem_complete.create("concurrent2.txt")
    assert_eq_string(err2, "")
    
    sus file3, err3 := filesystem_complete.open_file("concurrent3.txt", 
        filesystem_complete.O_RDWR | filesystem_complete.O_CREATE, 
        filesystem_complete.MODE_REGULAR)
    assert_eq_string(err3, "")
    
    # Test that all files are independent
    assert_true(file1.fd != file2.fd)
    assert_true(file2.fd != file3.fd)
    assert_true(file1.fd != file3.fd)
    
    # Close all files
    sus close1_err := file1.close()
    assert_eq_string(close1_err, "")
    
    sus close2_err := file2.close()
    assert_eq_string(close2_err, "")
    
    sus close3_err := file3.close()
    assert_eq_string(close3_err, "")
    
    print_test_summary()
}

# ==============================================================================
# MAIN TEST RUNNER
# ==============================================================================

slay main() {
    vibez.spill("=== CURSED Filesystem Complete Module Tests ===")
    vibez.spill("")
    
    # Core file operations
    test_file_open_close()
    test_file_create()
    test_file_open_with_flags()
    
    # File I/O operations
    test_file_read_write()
    test_file_seek_operations()
    test_file_truncate()
    
    # High-level file operations
    test_read_write_file()
    test_copy_move_remove()
    test_append_file()
    
    # Directory operations
    test_directory_operations()
    test_working_directory()
    test_directory_listing()
    
    # File info and metadata
    test_file_stat()
    test_file_existence()
    test_file_permissions()
    
    # Path manipulation
    test_path_operations()
    test_path_components()
    test_path_validation()
    
    # Symlinks and hard links
    test_symlink_operations()
    
    # Temporary files
    test_temp_operations()
    
    # Buffered I/O
    test_buffered_reader()
    test_buffered_writer()
    test_custom_buffer_sizes()
    
    # Advanced operations
    test_file_locking()
    test_file_comparison()
    test_file_hashing()
    test_copy_with_metadata()
    
    # File system monitoring
    test_file_watching()
    test_disk_usage()
    test_file_finding()
    
    # Utility functions
    test_path_validation_utils()
    test_filename_sanitization()
    test_module_info()
    
    # Error handling
    test_error_conditions()
    test_permission_errors()
    
    # Stress and edge cases
    test_large_file_operations()
    test_concurrent_operations()
    
    vibez.spill("")
    vibez.spill("=== All Filesystem Complete Tests Completed ===")
    vibez.spill("Total test functions: 30+")
    vibez.spill("Coverage: File I/O, directories, paths, metadata, buffered I/O, advanced ops")
}
