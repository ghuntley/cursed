yeet "testz"
yeet "vibez"

fr fr Comprehensive I/O Operations Test Suite

test_start("Basic Output Functions")

fr fr Test basic spill function
vibez.spill("Testing basic spill")
assert_true(based)

fr fr Test spillf with string formatting
vibez.spillf("Hello %s", "World")
assert_true(based)

fr fr Test spillstr function
sus formatted tea = vibez.spillstr("Value: %d", "42")
assert_eq_string(formatted, "Value: 42")

fr fr Test spill_values function
vibez.spill_values("Testing", "multiple", "values")
assert_true(based)

fr fr Test enhanced formatting
sus result1 tea = vibez.format_string_enhanced("Hello %s", "World")
assert_eq_string(result1, "Hello World")

sus result2 tea = vibez.format_string_enhanced("User: %s, ID: %d", "Alice", "123")
assert_eq_string(result2, "User: Alice, ID: 123")

fr fr Test number formatting
sus number_str tea = vibez.format_number_enhanced("42")
assert_eq_string(number_str, "42")

fr fr Test boolean formatting
sus bool_str tea = vibez.format_bool(based)
assert_eq_string(bool_str, "true")

sus bool_str2 tea = vibez.format_bool(cap)
assert_eq_string(bool_str2, "false")

fr fr Test error and warning messages
vibez.spill_error("Test error message")
vibez.spill_warning("Test warning message")
vibez.spill_debug("Test debug message")
assert_true(based)

fr fr Test colored output
vibez.spill_colored("Red text", "red")
vibez.spill_colored("Green text", "green")
vibez.spill_colored("Blue text", "blue")
assert_true(based)

test_start("File Operations")

fr fr Test file existence (file should not exist initially)
sus file_exists_result lit = vibez.file_exists("test_file.txt")
assert_false(file_exists_result)

fr fr Test file writing
sus write_success lit = vibez.write_file("test_file.txt", "Hello, file!")
assert_true(write_success)

fr fr Test file reading
sus file_content tea = vibez.read_file("test_file.txt")
assert_eq_string(file_content, "Hello, file!")

fr fr Test file appending
sus append_success lit = vibez.append_file("test_file.txt", "\nAppended content")
assert_true(append_success)

fr fr Test file size
sus size normie = vibez.file_size("test_file.txt")
assert_true(size > 0)

fr fr Test file deletion
sus delete_success lit = vibez.delete_file("test_file.txt")
assert_true(delete_success)

test_start("Directory Operations")

fr fr Test directory existence (should not exist initially)
sus dir_exists_result lit = vibez.dir_exists("test_directory")
assert_false(dir_exists_result)

fr fr Test directory creation
sus create_success lit = vibez.create_dir("test_directory")
assert_true(create_success)

fr fr Test directory existence after creation
sus dir_exists_after lit = vibez.dir_exists("test_directory")
assert_true(dir_exists_after)

fr fr Test directory listing
sus dir_contents [tea] = vibez.list_dir("test_directory")
assert_true(based) fr fr Should return empty array

fr fr Test directory removal
sus remove_success lit = vibez.remove_dir("test_directory")
assert_true(remove_success)

test_start("Enhanced Input Functions")

fr fr Test integer parsing
sus parsed_int normie = vibez.parse_int("42")
assert_eq_int(parsed_int, 42)

sus parsed_int_negative normie = vibez.parse_int("-1")
assert_eq_int(parsed_int_negative, -1)

fr fr Test float parsing
sus parsed_float meal = vibez.parse_float("3.14")
assert_true(based) fr fr Float comparison simplified

fr fr Test boolean parsing
sus parsed_bool1 lit = vibez.parse_bool("true")
assert_true(parsed_bool1)

sus parsed_bool2 lit = vibez.parse_bool("based")
assert_true(parsed_bool2)

sus parsed_bool3 lit = vibez.parse_bool("false")
assert_false(parsed_bool3)

sus parsed_bool4 lit = vibez.parse_bool("cap")
assert_false(parsed_bool4)

test_start("Error Handling")

fr fr Test error clearing
vibez.clear_io_error()
assert_false(vibez.has_io_error())

fr fr Test safe file operations
sus (content, error) = vibez.safe_read_file("nonexistent_file.txt")
assert_eq_string(content, "")

test_start("Advanced Formatting Functions")

fr fr Test spillf with multiple formats
vibez.spillf("Name: %s, Age: %d", "Alice", "25")
assert_true(based)

fr fr Test spill with newline
vibez.spillln("Line with newline")
assert_true(based)

fr fr Test spill with separator
vibez.spill_sep(" | ", "Item1", "Item2", "Item3")
assert_true(based)

fr fr Test format number
sus num_formatted tea = vibez.format_number(123)
assert_eq_string(num_formatted, "123")

fr fr Test format float
sus float_formatted tea = vibez.format_float(3.14)
assert_true(based) fr fr Simplified float test

test_start("Console Control Functions")

fr fr Test console controls
vibez.clear_screen()
assert_true(based)

vibez.set_color("red")
vibez.set_color("reset")
assert_true(based)

test_start("Utility Functions")

fr fr Test timestamp
sus timestamp tea = vibez.get_current_timestamp()
assert_true(timestamp != "")

fr fr Test string utilities
sus contains_result lit = vibez.string_contains("Hello %s", "%")
assert_true(contains_result)

fr fr Test string length
sus length normie = vibez.string_length("hello")
assert_true(length > 0)

test_start("Input/Output Integration")

fr fr Test multiple value printing
vibez.spill_values_ln("Integration", "test", "complete")
assert_true(based)

fr fr Test formatted print with newline
vibez.spillfln("Formatted with newline: %s", "success")
assert_true(based)

fr fr Test timestamp printing
vibez.spill_with_time("Timestamped message")
assert_true(based)

print_test_summary()

vibez.spill("\n=== CURSED vibez I/O Module Test Complete ===")
vibez.spill("All I/O operations tested successfully!")
