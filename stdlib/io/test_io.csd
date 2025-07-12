yeet "testz"
yeet "io"

# Test comprehensive I/O operations
test_start("Pure CURSED I/O comprehensive tests")

# === CONSOLE I/O TESTS ===

test_start("Console I/O operations")
sus console_result IOResult = console_print("Hello Console")
assert_true(console_result.success)
assert_eq_string(console_result.data, "Hello Console")

sus println_result IOResult = console_println("Hello Console Line")
assert_true(println_result.success)

sus int_result IOResult = console_print_int(42)
assert_true(int_result.success)

sus float_result IOResult = console_print_float(3.14)
assert_true(float_result.success)

sus bool_result IOResult = console_print_bool(based)
assert_true(bool_result.success)

# === BUFFERED I/O TESTS ===

test_start("Buffered I/O operations")
sus buffer IOBuffer = create_buffer(1024)
assert_eq_int(buffer.capacity, 1024)
assert_eq_int(buffer.size, 0)
assert_eq_int(buffer.position, 0)

sus write_result IOResult = buffer_write(buffer, "Hello Buffer")
assert_true(write_result.success)

sus read_result IOResult = buffer_read(buffer, 5)
assert_true(read_result.success)

sus flush_result IOResult = buffer_flush(buffer)
assert_true(flush_result.success)

# === STREAM I/O TESTS ===

test_start("Stream I/O operations")
sus stream IOStream = create_stream(1, based, based)
assert_eq_int(stream.id, 1)
assert_true(stream.readable)
assert_true(stream.writable)

sus stream_write_result IOResult = stream_write(stream, "Hello Stream")
assert_true(stream_write_result.success)

sus stream_read_result IOResult = stream_read(stream, 5)
assert_true(stream_read_result.success)

sus seek_result IOResult = stream_seek(stream, 0)
assert_true(seek_result.success)

# === FILE I/O TESTS ===

test_start("File I/O operations")
sus file_write_result IOResult = file_write("test.txt", "Hello File")
assert_true(file_write_result.success)

sus file_read_result IOResult = file_read("test.txt")
assert_true(file_read_result.success)

sus file_exists_result lit = file_exists("test.txt")
assert_true(file_exists_result)

sus file_delete_result IOResult = file_delete("test.txt")
assert_true(file_delete_result.success)

# === INTERACTIVE I/O TESTS ===

test_start("Interactive I/O operations")
sus prompt_result IOResult = prompt_user("Enter your name")
assert_true(prompt_result.success)

sus confirm_result IOResult = confirm_user("Are you sure?")
assert_true(confirm_result.success)

sus options []tea = ["Option 1", "Option 2", "Option 3"]
sus select_result IOResult = select_option("Choose an option", options)
assert_true(select_result.success)

# === INITIALIZATION TESTS ===

test_start("I/O initialization")
sus init_result IOResult = init_io()
assert_true(init_result.success)

sus shutdown_result IOResult = shutdown_io()
assert_true(shutdown_result.success)

# === LEGACY COMPATIBILITY TESTS ===

test_start("Legacy compatibility")
print("Legacy print test")
println("Legacy println test")
print_int(123)
print_float(2.71)
print_bool(cap)

sus legacy_line tea = read_line()
assert_eq_string(legacy_line, "user_input_placeholder")

sus legacy_int normie = read_int()
assert_eq_int(legacy_int, 42)

sus legacy_write_result lit = write_file("legacy.txt", "Legacy content")
assert_true(legacy_write_result)

sus legacy_read_result tea = read_file("legacy.txt")
assert_eq_string(legacy_read_result, "file_content_placeholder")

print_test_summary()
