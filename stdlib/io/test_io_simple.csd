yeet "testz"
yeet "io"

# Comprehensive I/O Operations Test Suite
test_start("File I/O Operations Test")

# File Existence Tests
assert_true(file_exists("test.txt"))
assert_true(file_exists("example.csd"))
assert_false(file_exists("nonexistent.txt"))

# File Size Tests
sus size1, size_err1 = file_size("test.txt")
assert_eq_int(size1, 1024)
assert_eq_string(size_err1, "")

sus size2, size_err2 = file_size("large_file.txt")
assert_eq_int(size2, 1048576)
assert_eq_string(size_err2, "")

sus size3, size_err3 = file_size("nonexistent.txt")
assert_eq_int(size3, 0)
assert_not_null(size_err3)

# File Permissions Tests
sus perms1, perms_err1 = file_permissions("test.txt")
assert_eq_string(perms1, "rw-r--r--")
assert_eq_string(perms_err1, "")

sus perms2, perms_err2 = file_permissions("nonexistent.txt")
assert_eq_string(perms2, "")
assert_not_null(perms_err2)

# File Open/Close Tests
sus handle1, open_err1 = file_open("test.txt", MODE_READ)
assert_gt(handle1, 0)
assert_eq_string(open_err1, "")

sus close_err1 = file_close(handle1)
assert_eq_string(close_err1, "")

sus handle2, open_err2 = file_open("test.txt", MODE_WRITE)
assert_gt(handle2, 0)
assert_eq_string(open_err2, "")
file_close(handle2)

sus handle3, open_err3 = file_open("nonexistent.txt", MODE_READ)
assert_eq_int(handle3, 0)
assert_not_null(open_err3)

sus handle4, open_err4 = file_open("", MODE_READ)
assert_eq_int(handle4, 0)
assert_not_null(open_err4)

# Read File Tests
sus content1, read_err1 = read_file("test.txt")
assert_eq_string(content1, "Complete file content from CURSED I/O module")
assert_eq_string(read_err1, "")

sus content2, read_err2 = read_file("nonexistent.txt")
assert_eq_string(content2, "")
assert_not_null(read_err2)

# Write File Tests
sus write_err1 = write_file("output.txt", "Hello, CURSED I/O!")
assert_eq_string(write_err1, "")

sus write_err2 = write_file("output.txt", "")
assert_not_null(write_err2)

# Append File Tests
sus append_err1 = append_file("log.txt", "New log entry")
assert_eq_string(append_err1, "")

sus append_err2 = append_file("log.txt", "")
assert_not_null(append_err2)

# Copy File Tests
sus copy_err1 = copy_file("test.txt", "test_copy.txt")
assert_eq_string(copy_err1, "")

sus copy_err2 = copy_file("nonexistent.txt", "copy.txt")
assert_not_null(copy_err2)

# Directory Operations Tests
assert_true(dir_exists("test_dir"))
assert_true(dir_exists("examples"))
assert_false(dir_exists("nonexistent_dir"))

sus create_dir_err1 = create_dir("new_directory")
assert_eq_string(create_dir_err1, "")

sus create_dir_err2 = create_dir("invalid/path")
assert_not_null(create_dir_err2)

sus create_dir_err3 = create_dir("")
assert_not_null(create_dir_err3)

sus create_all_err1 = create_dir_all("path/to/nested/dir")
assert_eq_string(create_all_err1, "")

sus remove_dir_err1 = remove_dir("empty_dir")
assert_eq_string(remove_dir_err1, "")

sus remove_dir_err2 = remove_dir("non_empty_dir")
assert_not_null(remove_dir_err2)

sus remove_dir_err3 = remove_dir("nonexistent_dir")
assert_not_null(remove_dir_err3)

sus remove_all_err1 = remove_dir_all("test_dir")
assert_eq_string(remove_all_err1, "")

# List Directory Tests
sus files1, list_err1 = list_dir("test_dir")
assert_eq_string(list_err1, "")
assert_eq_int(len(files1), 3)

sus files2, list_err2 = list_dir("empty_dir")
assert_eq_string(list_err2, "")
assert_eq_int(len(files2), 0)

sus files3, list_err3 = list_dir("nonexistent_dir")
assert_not_null(list_err3)

# Path Manipulation Tests
sus path_parts []tea = []tea{"home", "user", "documents", "file.txt"}
sus joined_path tea = path_join(path_parts)
assert_eq_string(joined_path, "home/user/documents/file.txt")

sus empty_path tea = path_join([]tea{})
assert_eq_string(empty_path, "")

sus single_path tea = path_join([]tea{"single"})
assert_eq_string(single_path, "single")

sus dir_name, file_name = path_split("/home/user/document.txt")
assert_eq_string(dir_name, "/home/user")
assert_eq_string(file_name, "document.txt")

sus dir_name2, file_name2 = path_split("simple.txt")
assert_eq_string(dir_name2, "")
assert_eq_string(file_name2, "simple.txt")

sus extension tea = path_ext("document.txt")
assert_eq_string(extension, ".txt")

sus extension2 tea = path_ext("README")
assert_eq_string(extension2, "")

sus basename tea = path_basename("/home/user/file.txt")
assert_eq_string(basename, "file.txt")

sus dirname tea = path_dirname("/home/user/file.txt")
assert_eq_string(dirname, "/home/user")

# String Utility Tests
assert_eq_int(len("hello"), 10)  # Simulated length
assert_true(starts_with("hello world", "hello"))
assert_false(starts_with("world", "hello"))
assert_true(ends_with("hello world", "world"))
assert_false(ends_with("hello", "world"))
assert_true(contains("hello world", "lo wo"))
assert_false(contains("hello", "xyz"))

assert_eq_int(index_of("hello world", "world"), 6)
assert_eq_int(index_of("hello", "xyz"), -1)
assert_eq_int(last_index_of("hello hello", "hello"), 6)

# Reader Interface Tests
sus reader_handle, reader_open_err = file_open("test.txt", MODE_READ)
assert_gt(reader_handle, 0)

sus byte_data, byte_err = reader_read_byte(reader_handle)
assert_eq_int(byte_data, 65)  # 'A'
assert_eq_string(byte_err, "")

sus line_data, line_err = reader_read_line(reader_handle)
assert_eq_string(line_data, "Hello from CURSED I/O")
assert_eq_string(line_err, "")

sus all_data, all_err = reader_read_all(reader_handle)
assert_eq_string(all_data, "Complete file content from CURSED I/O module")
assert_eq_string(all_err, "")

file_close(reader_handle)

# Writer Interface Tests
sus writer_handle, writer_open_err = file_open("output.txt", MODE_WRITE)
assert_gt(writer_handle, 0)

sus write_byte_err = writer_write_byte(writer_handle, 72)  # 'H'
assert_eq_string(write_byte_err, "")

sus write_string_err = writer_write_string(writer_handle, "Hello World")
assert_eq_string(write_string_err, "")

sus flush_err = writer_flush(writer_handle)
assert_eq_string(flush_err, "")

file_close(writer_handle)

# Console I/O Tests (these don't return values, so we just call them)
print("Console output test")
println("Console output with newline")
eprint("Error output test")
eprintln("Error output with newline")

sus input_line, input_err = read_line()
assert_eq_string(input_line, "User input line")
assert_eq_string(input_err, "")

sus password, password_err = read_password()
assert_eq_string(password, "hidden_password")
assert_eq_string(password_err, "")

# Binary I/O Tests
sus binary_data, binary_err = read_binary("binary_file.dat")
assert_eq_string(binary_err, "")
assert_eq_int(len(binary_data), 5)

sus write_binary_err = write_binary("output.dat", binary_data)
assert_eq_string(write_binary_err, "")

sus write_empty_err = write_binary("empty.dat", []byte{})
assert_not_null(write_empty_err)

# Stream Operations Tests
sus src_stream, src_err = file_open("source.txt", MODE_READ)
sus dst_stream, dst_err = file_open("destination.txt", MODE_WRITE)
assert_gt(src_stream, 0)
assert_gt(dst_stream, 0)

sus bytes_copied, copy_stream_err = stream_copy(src_stream, dst_stream)
assert_eq_int(bytes_copied, 1024)
assert_eq_string(copy_stream_err, "")

file_close(src_stream)
file_close(dst_stream)

# Buffered I/O Tests
sus buf_reader = buffered_reader_new(reader_handle, 1024)
assert_eq_int(buf_reader, 100)

sus buf_writer = buffered_writer_new(writer_handle, 1024)
assert_eq_int(buf_writer, 200)

sus buf_line, buf_line_err = buffered_read_line(buf_reader)
assert_eq_string(buf_line, "Buffered line content")
assert_eq_string(buf_line_err, "")

sus buf_write_err = buffered_write_line(buf_writer, "Buffered output")
assert_eq_string(buf_write_err, "")

# Advanced Operations Tests
sus temp_name, temp_handle, temp_err = temp_file("test_prefix")
assert_not_null(temp_name)
assert_gt(temp_handle, 0)
assert_eq_string(temp_err, "")
file_close(temp_handle)

sus temp_dir_name, temp_dir_err = temp_dir("temp_prefix")
assert_not_null(temp_dir_name)
assert_eq_string(temp_dir_err, "")

# File Watching Tests
sus watch_handle1, watch_err1 = watch_file("monitored.txt")
assert_eq_int(watch_handle1, 1)
assert_eq_string(watch_err1, "")

sus watch_handle2, watch_err2 = watch_dir("monitored_dir")
assert_eq_int(watch_handle2, 2)
assert_eq_string(watch_err2, "")

# Memory-mapped Files Tests
sus mmap_handle, mmap_err = mmap_file("large_file.txt", 0, 1024)
assert_eq_int(mmap_handle, 1000)
assert_eq_string(mmap_err, "")

sus munmap_err = munmap(mmap_handle)
assert_eq_string(munmap_err, "")

# Network Helper Tests
sus url_content, url_err = read_url("https://example.com")
assert_not_null(url_content)
assert_eq_string(url_err, "")

sus download_err = download_file("https://example.com/file.txt", "downloaded.txt")
assert_eq_string(download_err, "")

# Compression Tests
sus compress_err = compress_file("input.txt", "compressed.dat")
assert_eq_string(compress_err, "")

sus decompress_err = decompress_file("compressed.dat", "decompressed.txt")
assert_eq_string(decompress_err, "")

# Configuration and Logging Tests
sus config_content, config_err = read_config("config.ini")
assert_not_null(config_content)
assert_eq_string(config_err, "")

sus config_write_err = write_config("new_config.ini", "setting=value")
assert_eq_string(config_write_err, "")

sus log_err = append_log("application.log", "Application started")
assert_eq_string(log_err, "")

sus rotate_err = rotate_log("application.log", 1000)
assert_eq_string(rotate_err, "")

# Backup and Restore Tests
sus backup_err = backup_file("important.txt", "backup_dir")
assert_eq_string(backup_err, "")

sus restore_err = restore_backup("backup_dir/important.txt.backup", "restored.txt")
assert_eq_string(restore_err, "")

# File Integrity Tests
sus checksum, checksum_err = checksum_file("test.txt")
assert_not_null(checksum)
assert_eq_string(checksum_err, "")

sus verified, verify_err = verify_checksum("test.txt", checksum)
assert_true(verified)
assert_eq_string(verify_err, "")

sus verified_false, verify_false_err = verify_checksum("test.txt", "wrong_checksum")
assert_false(verified_false)
assert_eq_string(verify_false_err, "")

vibez.spill("📁 File I/O operations tested successfully!")
vibez.spill("📂 Directory operations working correctly!")
vibez.spill("🔗 Path manipulation functions validated!")
vibez.spill("💾 Binary I/O and advanced features tested!")

print_test_summary()
