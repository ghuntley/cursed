yeet "testz"
yeet "io"

# Test comprehensive I/O operations for self-hosting

# === INITIALIZATION TESTS ===

test_start("I/O Module Initialization")
sus init_result IOResult = init_io()
assert_true(init_result.success)
assert_eq_string(init_result.data, "I/O module initialized for self-hosting")

# === FILE READING TESTS ===

test_start("File Reading Operations")

# Test reading existing file
sus read_result IOResult = read_file("test.csd")
assert_true(read_result.success)
assert_eq_string(read_result.data, "vibez.spill(\"Hello from file\")")

# Test reading main source file
sus main_result IOResult = read_file("main.csd")
assert_true(main_result.success)
assert_true(string_length(main_result.data) > 0)

# Test reading empty file
sus empty_result IOResult = read_file("empty.csd")
assert_true(empty_result.success)
assert_eq_string(empty_result.data, "")

# Test reading non-existent file
sus missing_result IOResult = read_file("nonexistent.csd")
assert_false(missing_result.success)
assert_eq_string(missing_result.error, "File not found: nonexistent.csd")

# Test text file reading
sus text_result IOResult = read_text_file("test.csd")
assert_true(text_result.success)
assert_eq_string(text_result.data, "vibez.spill(\"Hello from file\")")

# === FILE WRITING TESTS ===

test_start("File Writing Operations")

# Test writing file
sus write_result IOResult = write_file("output.csd", "vibez.spill(\"Written content\")")
assert_true(write_result.success)
assert_true(string_length(write_result.data) > 0)

# Test writing text file
sus text_write_result IOResult = write_text_file("output.txt", "Hello, World!")
assert_true(text_write_result.success)

# Test writing empty file
sus empty_write_result IOResult = write_file("empty_output.csd", "")
assert_true(empty_write_result.success)

# === DIRECTORY OPERATIONS TESTS ===

test_start("Directory Operations")

# Test creating directory
sus create_dir_result IOResult = create_dir("test_dir")
assert_true(create_dir_result.success)
assert_eq_string(create_dir_result.data, "Directory created: test_dir")

# Test listing current directory
sus list_current_result IOResult = list_dir(".")
assert_true(list_current_result.success)
assert_eq_string(list_current_result.data, "main.csd\ntest.csd\nlib.csd")

# Test listing src directory
sus list_src_result IOResult = list_dir("src")
assert_true(list_src_result.success)
assert_eq_string(list_src_result.data, "compiler.csd\nparser.csd\ncodegen.csd")

# Test listing non-existent directory
sus list_missing_result IOResult = list_dir("nonexistent")
assert_false(list_missing_result.success)
assert_eq_string(list_missing_result.error, "Directory not found: nonexistent")

# === EXISTENCE CHECKS ===

test_start("File/Directory Existence Checks")

# Test existing file
sus exists_file lit = exists("test.csd")
assert_true(exists_file)

# Test existing directory
sus exists_dir lit = exists("src")
assert_true(exists_dir)

# Test non-existent path
sus exists_missing lit = exists("nonexistent.csd")
assert_false(exists_missing)

# === FILE SYSTEM OPERATIONS ===

test_start("File System Operations")

# Test file removal (existing file)
sus remove_result IOResult = remove_file("test.csd")
assert_true(remove_result.success)
assert_eq_string(remove_result.data, "File removed: test.csd")

# Test file removal (non-existent file)
sus remove_missing_result IOResult = remove_file("nonexistent.csd")
assert_false(remove_missing_result.success)
assert_eq_string(remove_missing_result.error, "File not found: nonexistent.csd")

# Test file copying (existing source)
sus copy_result IOResult = copy_file("main.csd", "main_backup.csd")
assert_true(copy_result.success)
assert_eq_string(copy_result.data, "File copied: main.csd → main_backup.csd")

# Test file copying (non-existent source)
sus copy_missing_result IOResult = copy_file("nonexistent.csd", "backup.csd")
assert_false(copy_missing_result.success)
assert_eq_string(copy_missing_result.error, "Source file not found: nonexistent.csd")

# === STANDARD I/O TESTS ===

test_start("Standard I/O Operations")

# Test print
sus print_result IOResult = print("Hello, stdout!")
assert_true(print_result.success)
assert_eq_string(print_result.data, "Hello, stdout!")

# Test println
sus println_result IOResult = println("Hello, stdout with newline!")
assert_true(println_result.success)
assert_eq_string(println_result.data, "Hello, stdout with newline!")

# Test read_line
sus readline_result IOResult = read_line()
assert_true(readline_result.success)
assert_eq_string(readline_result.data, "user_input_line")

# === ADVANCED FILE OPERATIONS ===

test_start("Advanced File Operations")

# Test getting file size
sus size_result IOResult = get_file_size("test.csd")
assert_true(size_result.success)
assert_gt_int(size_result.data, 0)

# Test getting file size for non-existent file
sus size_missing_result IOResult = get_file_size("nonexistent.csd")
assert_false(size_missing_result.success)
assert_eq_string(size_missing_result.error, "File not found: nonexistent.csd")

# Test getting file extension
sus extension_csd tea = get_file_extension("test.csd")
assert_eq_string(extension_csd, "csd")

sus extension_txt tea = get_file_extension("readme.txt")
assert_eq_string(extension_txt, "txt")

sus extension_none tea = get_file_extension("noextension")
assert_eq_string(extension_none, "")

# Test getting file basename
sus basename_csd tea = get_file_basename("test.csd")
assert_eq_string(basename_csd, "test")

sus basename_txt tea = get_file_basename("readme.txt")
assert_eq_string(basename_txt, "readme")

sus basename_none tea = get_file_basename("noextension")
assert_eq_string(basename_none, "noextension")

# === BUFFERED I/O TESTS ===

test_start("Buffered I/O Operations")

# Test creating buffer
sus buffer IOBuffer = create_buffer(1024)
assert_eq_int(buffer.capacity, 1024)
assert_eq_int(buffer.size, 0)
assert_eq_int(buffer.position, 0)

# Test writing to buffer
sus buffer_write_result IOResult = buffer_write(buffer, "Hello Buffer")
assert_true(buffer_write_result.success)
assert_eq_string(buffer_write_result.data, "Written 12 bytes to buffer")

# Test reading from buffer
sus buffer_read_result IOResult = buffer_read(buffer, 5)
assert_true(buffer_read_result.success)
assert_eq_string(buffer_read_result.data, "Hello")

# Test buffer overflow
sus large_buffer IOBuffer = create_buffer(10)
sus overflow_result IOResult = buffer_write(large_buffer, "This is a very long string that exceeds capacity")
assert_false(overflow_result.success)
assert_eq_string(overflow_result.error, "Buffer overflow: capacity 10 exceeded")

# Test buffer underflow
sus small_buffer IOBuffer = create_buffer(100)
sus underflow_result IOResult = buffer_read(small_buffer, 50)
assert_false(underflow_result.success)
assert_eq_string(underflow_result.error, "Buffer underflow: not enough data to read")

# Test buffer flush
sus flush_result IOResult = buffer_flush(buffer)
assert_true(flush_result.success)
assert_eq_string(flush_result.data, "Buffer flushed")

# === SELF-HOSTING COMPILER HELPERS ===

test_start("Self-Hosting Compiler Helpers")

# Test reading source file
sus source_result IOResult = read_source_file("main.csd")
assert_true(source_result.success)
assert_true(string_length(source_result.data) > 0)

# Test invalid source file extension
sus invalid_source_result IOResult = read_source_file("main.txt")
assert_false(invalid_source_result.success)
assert_eq_string(invalid_source_result.error, "Invalid source file extension: txt")

# Test writing compiled output
sus compiled_result IOResult = write_compiled_output("main.ll", "LLVM IR code here")
assert_true(compiled_result.success)

# Test reading compiler configuration
sus config_result IOResult = read_compiler_config("cursed.config")
assert_true(config_result.success)
assert_eq_string(config_result.data, "optimization_level=2\ntarget=native\ndebug=false")

# Test writing compiler log
sus log_result IOResult = write_compiler_log("Compilation started")
assert_true(log_result.success)
assert_eq_string(log_result.data, "Log written: Compilation started")

# === SHUTDOWN TESTS ===

test_start("I/O Module Shutdown")
sus shutdown_result IOResult = shutdown_io()
assert_true(shutdown_result.success)
assert_eq_string(shutdown_result.data, "I/O module shutdown complete")

# === COMPREHENSIVE INTEGRATION TEST ===

test_start("Self-Hosting Integration Test")

# Simulate a complete self-hosting workflow
sus workflow_init IOResult = init_io()
assert_true(workflow_init.success)

# Read source file
sus workflow_read IOResult = read_source_file("main.csd")
assert_true(workflow_read.success)

# Process the source (simulated)
sus processed_content tea = "// Compiled from " + workflow_read.data

# Write compiled output
sus workflow_write IOResult = write_compiled_output("main.ll", processed_content)
assert_true(workflow_write.success)

# Log the compilation
sus workflow_log IOResult = write_compiler_log("Self-hosting compilation complete")
assert_true(workflow_log.success)

# Shutdown
sus workflow_shutdown IOResult = shutdown_io()
assert_true(workflow_shutdown.success)

print_test_summary()
