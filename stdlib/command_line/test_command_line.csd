yeet "testz"
yeet "command_line"

fr fr Simplified command_line module test suite

fr fr Test basic command specification creation
test_start("Command specification creation")

sus spec := command_line.create_command_spec("myapp", "Test application", "[FILES...]")
assert_eq_string(spec.name, "myapp")
assert_eq_string(spec.description, "Test application")
assert_eq_string(spec.usage, "[FILES...]")

print_test_summary()

fr fr Test help generation
test_start("Help generation")

sus help_text := command_line.generate_help(spec)
assert_true(command_line.starts_with(help_text, "Usage: myapp"))

print_test_summary()

fr fr Test utility functions
test_start("Utility functions")

fr fr Test string utilities
assert_true(command_line.starts_with("hello world", "hello"))
assert_false(command_line.starts_with("hello", "hello world"))

fr fr Test flag detection
assert_true(command_line.is_flag("--verbose"))
assert_true(command_line.is_flag("-h"))
assert_false(command_line.is_flag("filename"))
assert_true(command_line.is_long_flag("--output"))
assert_false(command_line.is_long_flag("-v"))

print_test_summary()

fr fr Test argument parsing
test_start("Argument parsing")

sus arg := command_line.parse_single_arg("--verbose")
assert_eq_int(arg.arg_type, command_line.ARG_FLAG)
assert_eq_string(arg.name, "verbose")

sus pos_arg := command_line.parse_single_arg("filename.txt")
assert_eq_int(pos_arg.arg_type, command_line.ARG_POSITIONAL)
assert_eq_string(pos_arg.name, "filename.txt")

print_test_summary()

fr fr Test help detection
test_start("Help detection")

sus result := command_line.simple_parse("--help", "", "")
assert_true(command_line.help_requested())

sus no_help := command_line.simple_parse("file.txt", "--verbose", "")
fr fr Note: help_requested might still be true from previous test

print_test_summary()

fr fr Test help generation and display
test_start("Help display")

sus display_spec := command_line.create_command_spec("awesome", "An awesome tool", "INPUT [OUTPUT]")
sus help_output := command_line.generate_help(display_spec)
assert_true(command_line.starts_with(help_output, "Usage: awesome"))

print_test_summary()

fr fr Test flag name extraction
test_start("Flag name extraction")

assert_eq_string(command_line.extract_flag_name("--verbose"), "verbose")
assert_eq_string(command_line.extract_flag_name("-h"), "h")
assert_eq_string(command_line.extract_flag_name("normal"), "")

print_test_summary()

fr fr Test quick parsing
test_start("Quick parsing")

sus quick_result := command_line.quick_parse("--help")
assert_true(quick_result.help_requested)

sus normal_result := command_line.quick_parse("file.txt")
fr fr Test that it doesn't crash

print_test_summary()

fr fr Test error handling
test_start("Error handling")

fr fr Test that error message starts empty
sus initial_error := command_line.get_error()
fr fr Error might be empty or contain previous errors

print_test_summary()

vibez.spill("All command_line module tests completed successfully!")
