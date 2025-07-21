yeet "testz"
yeet "io_simple"
yeet "io"

slay test_io_simple_console_input() {
    test_start("I/O Simple Console Input")
    
    # Test console input functions
    sus line tea = io_read_line()
    assert_eq_string(line, "simulated user input line")
    
    sus char tea = io_read_char()
    assert_eq_string(char, "s")
    
    sus int_val normie = io_read_int()
    assert_eq_int(int_val, 42)
    
    sus float_val meal = io_read_float()
    assert_eq_int(int(float_val * 100), 314)  # 3.14 * 100 = 314
    
    print_test_summary()
}

slay test_io_console_input() {
    test_start("I/O Console Input")
    
    # Test basic I/O module console input
    sus line, err = read_line()
    assert_eq_string(line, "User input line")
    assert_eq_string(err, "")
    
    sus char, char_err = read_char()
    assert_eq_string(char, "U")
    assert_eq_string(char_err, "")
    
    sus int_val, int_err = read_int()
    assert_eq_int(int_val, 42)
    assert_eq_string(int_err, "")
    
    sus float_val, float_err = read_float()
    assert_eq_int(int(float_val * 100), 314)  # 3.14 * 100 = 314
    assert_eq_string(float_err, "")
    
    print_test_summary()
}

slay test_io_simple_parsing() {
    test_start("I/O Simple Parsing Functions")
    
    # Test integer parsing
    assert_eq_int(io_parse_int("42"), 42)
    assert_eq_int(io_parse_int("0"), 0)
    assert_eq_int(io_parse_int("-5"), -5)
    assert_eq_int(io_parse_int("invalid"), 0)
    
    # Test float parsing
    assert_eq_int(int(io_parse_float("3.14") * 100), 314)
    assert_eq_int(int(io_parse_float("0.0") * 100), 0)
    assert_eq_int(int(io_parse_float("-1.5") * 100), -150)
    assert_eq_int(int(io_parse_float("invalid") * 100), 0)
    
    # Test boolean parsing
    assert_true(io_parse_bool("based"))
    assert_true(io_parse_bool("true"))
    assert_true(io_parse_bool("1"))
    assert_false(io_parse_bool("cap"))
    assert_false(io_parse_bool("false"))
    assert_false(io_parse_bool("0"))
    assert_false(io_parse_bool("invalid"))
    
    print_test_summary()
}

slay test_console_output() {
    test_start("Console Output Functions")
    
    # Test output functions
    io_print("Hello ")
    io_println("World!")
    io_print_int(42)
    io_print_float(3.14)
    io_print_bool(based)
    io_print_bool(cap)
    
    # Test formatted output
    sus args []tea = []tea{"World", "42"}
    io_printf("Hello %s! The answer is %s.", args)
    
    print_test_summary()
}

# Run all tests
test_io_simple_console_input()
test_io_console_input()
test_io_simple_parsing()
test_console_output()

vibez.spill("\nAll I/O console tests completed!")
