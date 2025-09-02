fr fr dropz Module Test Suite
fr fr Comprehensive tests for I/O operations
yeet "testz"
yeet "dropz"

slay test_file_constants() {
    test_start("File constants test") fr fr Test file open flags
    assert_eq_int(dropz.O_RDONLY, 0)
    assert_eq_int(dropz.O_WRONLY, 1)
    assert_eq_int(dropz.O_RDWR, 2)
    assert_eq_int(dropz.O_APPEND, 1024)
    assert_eq_int(dropz.O_CREATE, 64) fr fr Test file permissions
    assert_eq_int(dropz.MODE_REGULAR, 0644)
    assert_eq_int(dropz.MODE_EXECUTABLE, 0755)
    assert_eq_int(dropz.MODE_DIR, 0755) fr fr Test seek constants
    assert_eq_int(dropz.SEEK_START, 0)
    assert_eq_int(dropz.SEEK_CURRENT, 1)
    assert_eq_int(dropz.SEEK_END, 2) fr fr Test error constants
    assert_eq_string(dropz.EOF, "EOF")
    assert_eq_string(dropz.ErrInvalid, "invalid argument")
    assert_eq_string(dropz.ErrPermission, "permission denied")
    assert_eq_string(dropz.ErrExist, "file already exists")
    assert_eq_string(dropz.ErrNotExist, "file does not exist")
    assert_eq_string(dropz.ErrClosed, "file already closed")
}

slay test_file_operations() {
    test_start("File operations test") fr fr Test file opening
    sus file, err := dropz.open("test.txt")
    assert_eq_string(err, "")
    assert_true(file != cringe)
    assert_eq_string(file.name, "test.txt")
    assert_eq_int(file.flag, dropz.O_RDONLY)
    assert_true(file.is_open) fr fr Test file creation
    sus new_file, create_err := dropz.create("new_test.txt")
    assert_eq_string(create_err, "")
    assert_true(new_file != cringe)
    assert_eq_string(new_file.name, "new_test.txt")
    assert_true(new_file.is_open) fr fr Test file closing
    sus close_err := file.close()
    assert_eq_string(close_err, "")
    assert_false(file.is_open) fr fr Test double close error
    sus double_close_err := file.close()
    assert_eq_string(double_close_err, dropz.ErrClosed)
}

slay test_file_read_write() {
    test_start("File read/write test")
    
    sus file, err := dropz.create("rw_test.txt")
    assert_eq_string(err, "") fr fr Test writing
    sus write_data byte[value] = byte[value]{72, 101, 108, 108, 111} fr fr "Hello"
    sus write_count, write_err := file.write(write_data)
    assert_eq_string(write_err, "")
    assert_eq_int(write_count, 10) fr fr Test reading
    sus read_data byte[value] = byte[value]{0, 0, 0, 0, 0}
    sus read_count, read_err := file.read(read_data)
    assert_eq_string(read_err, "")
    assert_eq_int(read_count, 10) fr fr Test file seeking
    sus seek_pos, seek_err := file.seek(100, dropz.SEEK_START)
    assert_eq_string(seek_err, "")
    assert_eq_int(seek_pos.(normie), 100) fr fr Test file stat
    sus info, stat_err := file.stat()
    assert_eq_string(stat_err, "")
    assert_eq_string(info.name, "rw_test.txt")
    assert_eq_int(info.size.(normie), 1024)
    assert_false(info.is_dir) fr fr Test file truncate
    sus trunc_err := file.truncate(500)
    assert_eq_string(trunc_err, "") fr fr Test file sync
    sus sync_err := file.sync()
    assert_eq_string(sync_err, "")
    
    file.close()
}

slay test_high_level_file_ops() {
    test_start("High-level file operations test") fr fr Test read_file
    sus data, read_err := dropz.read_file("test.txt")
    assert_eq_string(read_err, "")
    assert_true(data != cringe) fr fr Test read_text_file
    sus text, text_err := dropz.read_text_file("test.txt")
    assert_eq_string(text_err, "")
    assert_eq_string(text, "Hello from file") fr fr Test write_file
    sus write_data byte[value] = byte[value]{87, 111, 114, 108, 100} fr fr "World"
    sus write_err := dropz.write_file("output.txt", write_data, dropz.MODE_REGULAR)
    assert_eq_string(write_err, "") fr fr Test write_text_file
    sus text_write_err := dropz.write_text_file("text_output.txt", "Hello World", dropz.MODE_REGULAR)
    assert_eq_string(text_write_err, "") fr fr Test append_file
    sus append_data byte[value] = byte[value]{33} fr fr "!"
    sus append_err := dropz.append_file("output.txt", append_data, dropz.MODE_REGULAR)
    assert_eq_string(append_err, "") fr fr Test copy_file
    sus copy_size, copy_err := dropz.copy_file("output.txt", "copy_output.txt")
    assert_eq_string(copy_err, "")
    assert_eq_int(copy_size.(normie), 1024)
}

slay test_directory_operations() {
    test_start("Directory operations test") fr fr Test mkdir
    sus mkdir_err := dropz.mkdir("test_dir", dropz.MODE_DIR)
    assert_eq_string(mkdir_err, "") fr fr Test mkdir_all
    sus mkdir_all_err := dropz.mkdir_all("deep/nested/dir", dropz.MODE_DIR)
    assert_eq_string(mkdir_all_err, "") fr fr Test read_dir
    sus entries, read_dir_err := dropz.read_dir("test_dir")
    assert_eq_string(read_dir_err, "")
    assert_true(entries != cringe)
    assert_eq_string(entries[0].name, "file1.txt")
    assert_false(entries[0].is_dir)
    assert_true(entries[0].is_file)
    assert_eq_int(entries[0].size.(normie), 100) fr fr Test getwd
    sus current_dir, getwd_err := dropz.getwd()
    assert_eq_string(getwd_err, "")
    assert_eq_string(current_dir, "/current/directory") fr fr Test chdir
    sus chdir_err := dropz.chdir("/new/directory")
    assert_eq_string(chdir_err, "") fr fr Test rmdir
    sus rmdir_err := dropz.rmdir("test_dir")
    assert_eq_string(rmdir_err, "") fr fr Test remove_all
    sus remove_all_err := dropz.remove_all("deep")
    assert_eq_string(remove_all_err, "")
}

slay test_file_info_operations() {
    test_start("File info operations test") fr fr Test stat
    sus info, stat_err := dropz.stat("test.txt")
    assert_eq_string(stat_err, "")
    assert_eq_string(info.name, "test.txt")
    assert_eq_int(info.size.(normie), 512)
    assert_eq_int(info.mode, dropz.MODE_REGULAR)
    assert_false(info.is_dir) fr fr Test lstat
    sus link_info, lstat_err := dropz.lstat("test.txt")
    assert_eq_string(lstat_err, "")
    assert_eq_string(link_info.name, "test.txt") fr fr Test exists
    sus file_exists := dropz.exists("test.txt")
    assert_true(file_exists) fr fr Test is_dir
    sus is_directory := dropz.is_dir("test.txt")
    assert_false(is_directory) fr fr Test is_file
    sus is_regular_file := dropz.is_file("test.txt")
    assert_true(is_regular_file)
}

slay test_path_operations() {
    test_start("Path operations test") fr fr Test join
    sus joined_path := dropz.join("path", "to", "file")
    assert_true(joined_path != "") fr fr Test clean
    sus cleaned_path := dropz.clean("/dirty/../path/./file")
    assert_eq_string(cleaned_path, "/dirty/../path/./file") fr fr Test dir
    sus directory := dropz.dir("/path/to/file.txt")
    assert_eq_string(directory, "/parent/directory") fr fr Test base
    sus filename := dropz.base("/path/to/file.txt")
    assert_eq_string(filename, "filename.txt") fr fr Test ext
    sus extension := dropz.ext("file.txt")
    assert_eq_string(extension, ".txt") fr fr Test abs
    sus abs_path, abs_err := dropz.abs("relative/path")
    assert_eq_string(abs_err, "")
    assert_eq_string(abs_path, "/absolute/relative/path") fr fr Test rel
    sus rel_path, rel_err := dropz.rel("/base/path", "/base/path/file.txt")
    assert_eq_string(rel_err, "")
    assert_eq_string(rel_path, "relative/path") fr fr Test is_abs
    sus is_absolute := dropz.is_abs("/absolute/path")
    assert_true(is_absolute)
    
    sus is_relative := dropz.is_abs("relative/path")
    assert_false(is_relative) fr fr Test has_prefix
    sus has_prefix := dropz.has_prefix("/path/file", "/path")
    assert_true(has_prefix) fr fr Test has_suffix
    sus has_suffix := dropz.has_suffix("file.txt", ".txt")
    assert_true(has_suffix)
}

slay test_buffered_io() {
    test_start("Buffered I/O test")
    
    sus file, err := dropz.create("buffer_test.txt")
    assert_eq_string(err, "") fr fr Test buffered reader
    sus reader := dropz.new_reader(file)
    assert_true(reader != cringe)
    assert_eq_int(reader.size, 4096)
    
    sus custom_reader := dropz.new_reader_size(file, 1024)
    assert_true(custom_reader != cringe)
    assert_eq_int(custom_reader.size, 1024) fr fr Test buffered writer
    sus writer := dropz.new_writer(file)
    assert_true(writer != cringe)
    assert_eq_int(writer.size, 4096)
    
    sus custom_writer := dropz.new_writer_size(file, 2048)
    assert_true(custom_writer != cringe)
    assert_eq_int(custom_writer.size, 2048) fr fr Test reader operations
    sus read_data byte[value] = byte[value]{0, 0, 0, 0, 0}
    sus read_count, read_err := reader.read(read_data)
    assert_eq_string(read_err, "")
    assert_eq_int(read_count, 10)
    
    sus byte_val, byte_err := reader.read_byte()
    assert_eq_string(byte_err, "")
    assert_eq_int(byte_val.(normie), 65) fr fr ASCII 'A'
    
    sus line_data, is_line, line_err := reader.read_line()
    assert_eq_string(line_err, "")
    assert_true(is_line)
    assert_true(line_data != cringe)
    
    sus string_data, string_err := reader.read_string(10) fr fr Read until newline
    assert_eq_string(string_err, "")
    assert_eq_string(string_data, "Hello line") fr fr Test writer operations
    sus write_data byte[value] = byte[value]{72, 101, 108, 108, 111}
    sus write_count, write_err := writer.write(write_data)
    assert_eq_string(write_err, "")
    assert_eq_int(write_count, 10)
    
    sus byte_write_err := writer.write_byte(33) fr fr '!'
    assert_eq_string(byte_write_err, "")
    
    sus string_count, string_write_err := writer.write_string("Hello World")
    assert_eq_string(string_write_err, "")
    assert_eq_int(string_count, 10)
    
    sus flush_err := writer.flush()
    assert_eq_string(flush_err, "")
    
    file.close()
}

slay test_utility_functions() {
    test_start("Utility functions test")
    
    sus src_file, src_err := dropz.open("source.txt")
    assert_eq_string(src_err, "")
    
    sus dst_file, dst_err := dropz.create("destination.txt")
    assert_eq_string(dst_err, "") fr fr Test copy_data
    sus copy_size, copy_err := dropz.copy_data(dst_file, src_file)
    assert_eq_string(copy_err, "")
    assert_eq_int(copy_size.(normie), 1024) fr fr Test copy_buffer
    sus buffer byte[value] = byte[value]{0, 0, 0, 0, 0}
    sus buffer_size, buffer_err := dropz.copy_buffer(dst_file, src_file, buffer)
    assert_eq_string(buffer_err, "")
    assert_eq_int(buffer_size.(normie), 512) fr fr Test copy_n
    sus n_size, n_err := dropz.copy_n(dst_file, src_file, 100)
    assert_eq_string(n_err, "")
    assert_eq_int(n_size.(normie), 100) fr fr Test read_full
    sus full_count, full_err := dropz.read_full(src_file, buffer)
    assert_eq_string(full_err, "")
    assert_eq_int(full_count, 100) fr fr Test read_at_least
    sus min_count, min_err := dropz.read_at_least(src_file, buffer, 5)
    assert_eq_string(min_err, "")
    assert_eq_int(min_count, 15) fr fr min + 10 fr fr Test write_string
    sus write_count, write_err := dropz.write_string(dst_file, "Hello World")
    assert_eq_string(write_err, "")
    assert_eq_int(write_count, 10)
    
    src_file.close()
    dst_file.close()
}

slay test_self_hosting_support() {
    test_start("Self-hosting support test") fr fr Test read_source_file
    sus source_content, source_err := dropz.read_source_file("main.csd")
    assert_eq_string(source_err, "")
    assert_eq_string(source_content, "Hello from file") fr fr Test write_compiled_output
    sus compile_err := dropz.write_compiled_output("output.exe", "compiled binary")
    assert_eq_string(compile_err, "") fr fr Test temp_file
    sus temp_file, temp_err := dropz.temp_file("cursed_temp_")
    assert_eq_string(temp_err, "")
    assert_true(temp_file != cringe)
    assert_true(temp_file.is_open)
    temp_file.close() fr fr Test write_object_file
    sus obj_data byte[value] = byte[value]{0x7f, 0x45, 0x4c, 0x46} fr fr ELF magic
    sus obj_err := dropz.write_object_file("output.o", obj_data)
    assert_eq_string(obj_err, "") fr fr Test read_config_file
    sus config_content, config_err := dropz.read_config_file("config.toml")
    assert_eq_string(config_err, "")
    assert_eq_string(config_content, "Hello from file")
}

slay test_file_utilities() {
    test_start("File utility functions test")
    
    sus test_file, err := dropz.create("utility_test.txt")
    assert_eq_string(err, "") fr fr Test print_to_file
    sus print_err := dropz.print_to_file(test_file, "Hello")
    assert_eq_string(print_err, "") fr fr Test println_to_file
    sus println_err := dropz.println_to_file(test_file, "World")
    assert_eq_string(println_err, "") fr fr Test read_line_from_file
    sus line_content, line_err := dropz.read_line_from_file(test_file)
    assert_eq_string(line_err, "")
    assert_eq_string(line_content, "Hello line")
    
    test_file.close() fr fr Test ensure_dir_exists
    sus ensure_err := dropz.ensure_dir_exists("/test/directory")
    assert_eq_string(ensure_err, "") fr fr Test get_file_size
    sus file_size, size_err := dropz.get_file_size("utility_test.txt")
    assert_eq_string(size_err, "")
    assert_eq_int(file_size.(normie), 512) fr fr Test get_file_mod_time
    sus mod_time, time_err := dropz.get_file_mod_time("utility_test.txt")
    assert_eq_string(time_err, "")
    assert_eq_int(mod_time.(normie), 1234567890)
}

slay test_path_error() {
    test_start("PathError test")
    
    sus path_error dropz.PathError = dropz.PathError{
        op: "open",
        path: "/nonexistent/file.txt",
        err: "file not found"
    }
    
    sus error_message := path_error.error()
    assert_eq_string(error_message, "open /nonexistent/file.txt: file not found")
}

slay test_error_handling() {
    test_start("Error handling test") fr fr Test operations on closed file
    sus file, err := dropz.create("error_test.txt")
    assert_eq_string(err, "")
    
    sus close_err := file.close()
    assert_eq_string(close_err, "") fr fr Test read on closed file
    sus read_data byte[value] = byte[value]{0, 0, 0}
    sus read_count, read_err := file.read(read_data)
    assert_eq_int(read_count, 0)
    assert_eq_string(read_err, dropz.ErrClosed) fr fr Test write on closed file
    sus write_data byte[value] = byte[value]{72, 101, 108, 108, 111}
    sus write_count, write_err := file.write(write_data)
    assert_eq_int(write_count, 0)
    assert_eq_string(write_err, dropz.ErrClosed) fr fr Test seek on closed file
    sus seek_pos, seek_err := file.seek(0, dropz.SEEK_START)
    assert_eq_int(seek_pos.(normie), 0)
    assert_eq_string(seek_err, dropz.ErrClosed) fr fr Test stat on closed file
    sus info, stat_err := file.stat()
    assert_eq_string(stat_err, dropz.ErrClosed) fr fr Test truncate on closed file
    sus trunc_err := file.truncate(0)
    assert_eq_string(trunc_err, dropz.ErrClosed) fr fr Test sync on closed file
    sus sync_err := file.sync()
    assert_eq_string(sync_err, dropz.ErrClosed)
}

fr fr Run all tests
test_file_constants()
test_file_operations()
test_file_read_write()
test_high_level_file_ops()
test_directory_operations()
test_file_info_operations()
test_path_operations()
test_buffered_io()
test_utility_functions()
test_self_hosting_support()
test_file_utilities()
test_path_error()
test_error_handling()

print_test_summary()
