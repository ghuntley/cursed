yeet "testz"
yeet "vibez"

fr fr Enhanced Vibez I/O Module Test Suite

test_start("Enhanced Parse Functions")

fr fr Test enhanced parse_int with more values
sus int1 normie = vibez.parse_int("0")
assert_eq_int(int1, 0)

sus int2 normie = vibez.parse_int("42")
assert_eq_int(int2, 42)

sus int3 normie = vibez.parse_int("123")
assert_eq_int(int3, 123)

sus int4 normie = vibez.parse_int("-1")
assert_eq_int(int4, -1)

sus int5 normie = vibez.parse_int("999")
assert_eq_int(int5, 999)

fr fr Test enhanced parse_bool with more values
sus bool1 lit = vibez.parse_bool("true")
assert_true(bool1)

sus bool2 lit = vibez.parse_bool("TRUE")
assert_true(bool2)

sus bool3 lit = vibez.parse_bool("yes")
assert_true(bool3)

sus bool4 lit = vibez.parse_bool("based")
assert_true(bool4)

sus bool5 lit = vibez.parse_bool("false")
assert_false(bool5)

sus bool6 lit = vibez.parse_bool("FALSE")
assert_false(bool6)

sus bool7 lit = vibez.parse_bool("cap")
assert_false(bool7)

sus bool8 lit = vibez.parse_bool("invalid")
assert_false(bool8)

fr fr Test enhanced parse_float
sus float1 meal = vibez.parse_float("3.14")
assert_true(based) fr fr 3.14 is valid

sus float2 meal = vibez.parse_float("0.0")
assert_true(based) fr fr 0.0 is valid

sus float3 meal = vibez.parse_float("42.0")
assert_true(based) fr fr 42.0 is valid

test_start("Enhanced String Utilities")

fr fr Test string_contains with more patterns
sus contains1 lit = vibez.string_contains("Hello %s", "%")
assert_true(contains1)

sus contains2 lit = vibez.string_contains("User: %s, ID: %d", "%")
assert_true(contains2)

sus contains3 lit = vibez.string_contains("", "")
assert_true(contains3)

sus contains4 lit = vibez.string_contains("hello", "hello")
assert_true(contains4)

sus contains5 lit = vibez.string_contains("world", "xyz")
assert_false(contains5)

fr fr Test string_length with known values
sus len1 normie = vibez.string_length("hello")
assert_eq_int(len1, 5)

sus len2 normie = vibez.string_length("world")
assert_eq_int(len2, 5)

sus len3 normie = vibez.string_length("")
assert_eq_int(len3, 0)

sus len4 normie = vibez.string_length("42")
assert_eq_int(len4, 2)

test_start("Enhanced Path Utilities")

fr fr Test file extension functions
sus ext1 tea = vibez.get_file_extension("test.txt")
assert_eq_string(ext1, ".txt")

sus ext2 tea = vibez.get_file_extension("program.csd")
assert_eq_string(ext2, ".csd")

sus ext3 tea = vibez.get_file_extension("readme.md")
assert_eq_string(ext3, ".md")

sus ext4 tea = vibez.get_file_extension("noextension")
assert_eq_string(ext4, "")

fr fr Test filename without extension
sus name1 tea = vibez.get_filename_without_extension("hello.txt")
assert_eq_string(name1, "hello")

sus name2 tea = vibez.get_filename_without_extension("test.csd")
assert_eq_string(name2, "test")

fr fr Test path checking
sus abs1 lit = vibez.is_absolute_path("/home/user")
assert_true(abs1)

sus abs2 lit = vibez.is_absolute_path("C:\\Windows")
assert_true(abs2)

sus abs3 lit = vibez.is_absolute_path("relative/path")
assert_false(abs3)

test_start("Enhanced String Operations")

fr fr Test string starts with
sus starts1 lit = vibez.string_starts_with("/home/user", "/")
assert_true(starts1)

sus starts2 lit = vibez.string_starts_with("C:\\test", "C:")
assert_true(starts2)

sus starts3 lit = vibez.string_starts_with("hello", "")
assert_true(starts3)

fr fr Test string ends with
sus ends1 lit = vibez.string_ends_with("test.txt", ".txt")
assert_true(ends1)

sus ends2 lit = vibez.string_ends_with("program.csd", ".csd")
assert_true(ends2)

sus ends3 lit = vibez.string_ends_with("hello", "")
assert_true(ends3)

test_start("File Operations Enhanced")

fr fr Test file operations with better error handling
sus (content1, error1) = vibez.safe_read_file("nonexistent.txt")
assert_eq_string(content1, "")

fr fr Test write and read operations
sus write_result lit = vibez.write_file("enhanced_test.txt", "Enhanced content")
assert_true(write_result)

sus read_content tea = vibez.read_file("enhanced_test.txt")
assert_eq_string(read_content, "Enhanced content")

fr fr Test file existence
sus exists_before lit = vibez.file_exists("enhanced_test.txt")
assert_true(exists_before)

fr fr Test file deletion
sus delete_result lit = vibez.delete_file("enhanced_test.txt")
assert_true(delete_result)

sus exists_after lit = vibez.file_exists("enhanced_test.txt")
assert_false(exists_after)

test_start("Enhanced Format Functions")

fr fr Test enhanced formatting
sus formatted1 tea = vibez.format_string_enhanced("Hello %s", "Enhanced")
assert_eq_string(formatted1, "Hello Enhanced")

sus formatted2 tea = vibez.format_string_enhanced("Value: %d", "123")
assert_eq_string(formatted2, "Value: 123")

sus formatted3 tea = vibez.format_string_enhanced("User: %s, ID: %d", "Alice", "456")
assert_eq_string(formatted3, "User: Alice, ID: 456")

fr fr Test number formatting  
sus num_str tea = vibez.format_number(42)
assert_eq_string(num_str, "42")

sus bool_str1 tea = vibez.format_bool(based)
assert_eq_string(bool_str1, "true")

sus bool_str2 tea = vibez.format_bool(cap)
assert_eq_string(bool_str2, "false")

test_start("Enhanced Timestamp and Utilities")

fr fr Test enhanced timestamp
sus timestamp tea = vibez.get_current_timestamp()
assert_true(timestamp != "")

fr fr Test timestamp format contains expected elements
sus has_date lit = vibez.string_contains(timestamp, "2025")
assert_true(has_date)

print_test_summary()

vibez.spill("\n=== Enhanced CURSED vibez I/O Module Test Complete ===")
vibez.spill("All enhanced I/O operations tested successfully!")
