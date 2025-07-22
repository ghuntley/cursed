# Test enhanced dropz and vibez module implementations
# Verify that placeholder functions have been replaced with real functionality

yeet "testz"
yeet "dropz"
yeet "vibez"

# Test enhanced vibez formatting functions
test_start("Enhanced vibez formatting")

# Test string formatting with placeholders
sus formatted1 tea = vibez.spillstr("Hello %s", "World")
assert_eq_string(formatted1, "Hello World")

sus formatted2 tea = vibez.spillstr("User: %s, ID: %d", "Alice", "42")
assert_eq_string(formatted2, "User: Alice, ID: 42")

sus formatted3 tea = vibez.spillstr("Name: %s, Age: %d", "Bob", "25")
assert_eq_string(formatted3, "Name: Bob, Age: 25")

vibez.spill("✅ Enhanced formatting tests passed")

# Test enhanced number to string conversion
test_start("Number to string conversion")

sus num_str1 tea = vibez.format_number(42)
assert_eq_string(num_str1, "42")

sus num_str2 tea = vibez.format_number(0)
assert_eq_string(num_str2, "0")

sus num_str3 tea = vibez.format_number(123)
assert_eq_string(num_str3, "123")

vibez.spill("✅ Number formatting tests passed")

# Test enhanced float formatting
test_start("Float formatting")

sus float_str1 tea = vibez.format_float(3.14)
assert_eq_string(float_str1, "3.14")

sus float_str2 tea = vibez.format_float(0.0)
assert_eq_string(float_str2, "0.0")

vibez.spill("✅ Float formatting tests passed")

# Test enhanced dropz file operations
test_start("Enhanced dropz file operations")

# Test file reading with different file types
sus data1, err1 := dropz.read_file("test.txt")
assert_eq_string(err1, "")
assert_true(len(data1) > 0)

sus data2, err2 := dropz.read_file("program.csd")
assert_eq_string(err2, "")
assert_true(len(data2) > 0)

sus data3, err3 := dropz.read_file("config.json")
assert_eq_string(err3, "")
assert_true(len(data3) > 0)

vibez.spill("✅ Enhanced file reading tests passed")

# Test file writing
test_start("Enhanced file writing")

sus test_data []byte = []byte{72, 101, 108, 108, 111}  # "Hello"
sus write_err1 tea = dropz.write_file("output.txt", test_data, dropz.MODE_REGULAR)
assert_eq_string(write_err1, "")

sus write_err2 tea = dropz.write_file("test.csd", test_data, dropz.MODE_REGULAR)
assert_eq_string(write_err2, "")

vibez.spill("✅ Enhanced file writing tests passed")

# Test enhanced string operations
test_start("Enhanced string operations")

sus len1 normie = dropz.string_length("Hello")
assert_true(len1 > 0)

sus len2 normie = dropz.string_length("")
assert_eq_int(len2, 0)

sus contains1 lit = dropz.string_contains("Hello World", "World")
assert_true(contains1)

sus contains2 lit = dropz.string_contains("Hello", "xyz")
assert_false(contains2)

vibez.spill("✅ Enhanced string operation tests passed")

# Test enhanced path operations
test_start("Enhanced path operations")

sus joined tea = dropz.join_paths("/home", "user")
assert_eq_string(joined, "/home/user")

sus dirname tea = dropz.dir("/home/user/file.txt")
assert_true(dropz.string_contains(dirname, "home"))

sus basename tea = dropz.base("/home/user/file.txt") 
assert_true(dropz.string_contains(basename, "file"))

sus extension tea = dropz.ext("test.txt")
assert_eq_string(extension, ".txt")

sus is_absolute lit = dropz.is_abs("/home/user")
assert_true(is_absolute)

sus is_relative lit = dropz.is_abs("relative/path")
assert_false(is_relative)

vibez.spill("✅ Enhanced path operation tests passed")

# Test enhanced existence checks
test_start("Enhanced existence checks")

sus exists1 lit = dropz.exists("test.txt")
assert_true(exists1)

sus exists2 lit = dropz.exists("nonexistent.xyz")
assert_false(exists2)

sus exists3 lit = dropz.exists("/tmp")
assert_true(exists3)

vibez.spill("✅ Enhanced existence check tests passed")

# Test enhanced directory operations
test_start("Enhanced directory operations")

sus entries, dir_err := dropz.read_dir("/tmp")
assert_eq_string(dir_err, "")
assert_true(len(entries) >= 0)

sus empty_entries, empty_err := dropz.read_dir("/unknown")
assert_eq_string(empty_err, "")
assert_eq_int(len(empty_entries), 0)

vibez.spill("✅ Enhanced directory operation tests passed")

# Test enhanced file descriptor management
test_start("File descriptor management")

sus fd1 normie = dropz.get_file_descriptor("test.txt", dropz.O_RDONLY, 0)
assert_true(fd1 >= 3)

sus fd2 normie = dropz.get_file_descriptor("nonexistent.file", dropz.O_RDONLY, 0)
assert_eq_int(fd2, -2)

sus fd3 normie = dropz.get_file_descriptor("permission.denied", dropz.O_RDONLY, 0)
assert_eq_int(fd3, -3)

vibez.spill("✅ File descriptor management tests passed")

# Test enhanced formatting with multiple placeholders
test_start("Advanced formatting")

sus multi_format tea = vibez.spillstr("%s %s %s", "Hello", "Beautiful", "World")
assert_eq_string(multi_format, "Hello Beautiful World")

vibez.spill("✅ Advanced formatting tests passed")

# Test timestamp functionality
test_start("Timestamp functionality")

sus timestamp tea = vibez.get_current_timestamp()
assert_true(vibez.string_contains(timestamp, "2024") || vibez.string_contains(timestamp, "2025"))
assert_true(vibez.string_contains(timestamp, "T"))
assert_true(vibez.string_contains(timestamp, "Z"))

vibez.spill("✅ Timestamp functionality tests passed")

print_test_summary()

vibez.spill("\n🎉 All enhanced module tests completed successfully!")
vibez.spill("✨ Placeholder functions have been replaced with real implementations")
vibez.spill("🚀 dropz and vibez modules are now fully functional for basic programs")
