yeet "testz"
yeet "io"

# Test the migrated I/O module functionality

test_start("I/O Module Migration Test")

# Test initialization
sus init_result IOResult = init_io()
assert_true(init_result.success)
vibez.spill("✅ Initialization successful")

# Test basic file operations
sus read_result IOResult = read_file("test.csd")
assert_true(read_result.success)
assert_eq_string(read_result.data, "vibez.spill(\"Hello from file\")")
vibez.spill("✅ File reading works")

# Test file writing
sus write_result IOResult = write_file("output.txt", "test content")
assert_true(write_result.success)
vibez.spill("✅ File writing works")

# Test directory operations
sus dir_result IOResult = list_dir(".")
assert_true(dir_result.success)
vibez.spill("✅ Directory listing works")

# Test existence check
sus exists_result lit = exists("test.csd")
assert_true(exists_result)
vibez.spill("✅ File existence check works")

# Test buffered I/O
sus buffer IOBuffer = create_buffer(1024)
assert_eq_int(buffer.capacity, 1024)
assert_eq_int(buffer.size, 0)
vibez.spill("✅ Buffer creation works")

# Test async I/O
sus async_result AsyncResult = async_read_file("test.csd")
assert_true(async_result.completed)
assert_true(async_result.result.success)
vibez.spill("✅ Async I/O works")

# Test self-hosting operations
sus source_result IOResult = read_source_file("main.csd")
assert_true(source_result.success)
vibez.spill("✅ Source file reading works")

# Test compilation output
sus compiled_result IOResult = write_compiled_output("main.ll", "LLVM IR code")
assert_true(compiled_result.success)
vibez.spill("✅ Compiled output writing works")

# Test compiler configuration
sus config_result IOResult = read_compiler_config("cursed.config")
assert_true(config_result.success)
vibez.spill("✅ Compiler configuration works")

# Test logging
sus log_result IOResult = write_compiler_log("Test message")
assert_true(log_result.success)
vibez.spill("✅ Compiler logging works")

# Test shutdown
sus shutdown_result IOResult = shutdown_io()
assert_true(shutdown_result.success)
vibez.spill("✅ Shutdown successful")

vibez.spill("🎉 I/O module migration completed successfully!")
vibez.spill("📊 All core I/O operations working in pure CURSED")
vibez.spill("🚀 Ready for self-hosting compilation")

print_test_summary()
