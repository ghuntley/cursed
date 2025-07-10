yeet "testz"
yeet "command_line"

// Test CLI Initialization
test_start("cli_init")
sus result lit = cli_init("myprogram", "--help --version --verbose")
assert_true(result)

// Test CLI Parsing
test_start("cli_parse")
cli_init("myprogram", "--help --version --verbose")
sus parse_result lit = cli_parse()
assert_true(parse_result)

// Test Flag Detection
test_start("cli_has_flag")
cli_init("myprogram", "--help --version --verbose")
cli_parse()
assert_true(cli_has_flag("help"))
assert_true(cli_has_flag("version"))
assert_true(cli_has_flag("verbose"))
assert_false(cli_has_flag("nonexistent"))

// Test Flag Values
test_start("cli_get_flag_value")
cli_init("myprogram", "--output result.txt --input data.txt")
cli_parse()
assert_eq_string(cli_get_flag_value("output"), "output.txt")
assert_eq_string(cli_get_flag_value("input"), "input.txt")
assert_eq_string(cli_get_flag_value("nonexistent"), "")

// Test Boolean Flags
test_start("cli_get_flag_bool")
cli_init("myprogram", "--verbose --debug")
cli_parse()
assert_true(cli_get_flag_bool("verbose"))
assert_true(cli_get_flag_bool("debug"))
assert_false(cli_get_flag_bool("nonexistent"))

// Test Integer Flags
test_start("cli_get_flag_int")
cli_init("myprogram", "--port 8080 --threads 4")
cli_parse()
assert_eq_int(cli_get_flag_int("port"), 8080)
assert_eq_int(cli_get_flag_int("threads"), 4)
assert_eq_int(cli_get_flag_int("nonexistent"), 0)

// Test Required Flag Validation
test_start("cli_validate_required_flags")
cli_init("myprogram", "--input data.txt --output result.txt")
cli_parse()
assert_true(cli_validate_required_flags("input,output"))

test_start("cli_validate_required_flags_missing")
cli_init("myprogram", "--input data.txt")
cli_parse()
assert_false(cli_validate_required_flags("input,output"))

// Test Help Generation
test_start("cli_generate_help")
cli_init("myprogram", "")
sus help_text tea = cli_generate_help()
assert_true(help_text.contains("Usage:"))
assert_true(help_text.contains("Options:"))
assert_true(help_text.contains("--help"))
assert_true(help_text.contains("--version"))

test_start("cli_show_help")
cli_init("myprogram", "")
sus help_result lit = cli_show_help()
assert_true(help_result)

// Test Version Information
test_start("cli_show_version")
cli_init("myprogram", "")
sus version_result lit = cli_show_version()
assert_true(version_result)

// Test Subcommands
test_start("cli_get_subcommand")
cli_init("myprogram", "build --verbose")
cli_parse()
assert_eq_string(cli_get_subcommand(), "build")

test_start("cli_has_subcommand")
cli_init("myprogram", "test --debug")
cli_parse()
assert_true(cli_has_subcommand())

test_start("cli_get_subcommand_none")
cli_init("myprogram", "--verbose")
cli_parse()
assert_eq_string(cli_get_subcommand(), "")
assert_false(cli_has_subcommand())

// Test Positional Arguments
test_start("cli_get_positional_args")
cli_init("myprogram", "file1.txt file2.txt --verbose")
cli_parse()
sus positional tea = cli_get_positional_args()
assert_true(positional.contains("file1.txt"))
assert_true(positional.contains("file2.txt"))

test_start("cli_get_positional_count")
cli_init("myprogram", "file1.txt file2.txt --verbose")
cli_parse()
assert_eq_int(cli_get_positional_count(), 2)

// Test Error Handling
test_start("operations_without_initialization")
cli_parsed = cap
assert_false(cli_parse())
assert_false(cli_has_flag("test"))
assert_eq_string(cli_get_flag_value("test"), "")
assert_false(cli_get_flag_bool("test"))
assert_eq_int(cli_get_flag_int("test"), 0)
assert_false(cli_validate_required_flags("test"))
assert_eq_string(cli_get_subcommand(), "")
assert_false(cli_has_subcommand())
assert_eq_string(cli_get_positional_args(), "")
assert_eq_int(cli_get_positional_count(), 0)

print_test_summary()
