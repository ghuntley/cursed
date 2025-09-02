yeet "testz"
yeet "fmt"

test_start("Format module comprehensive tests")

fr fr Test basic formatting
assert_eq_string(fmt.format_int(42), "42")
assert_eq_string(fmt.format_int(-5), "-5")
assert_eq_string(fmt.format_int(0), "0")

assert_eq_string(fmt.format_bool(based), "based")
assert_eq_string(fmt.format_bool(cap), "cap")

assert_eq_string(fmt.format_float(3.14), "3.14")
assert_eq_string(fmt.format_float(0.0), "0.0")
assert_eq_string(fmt.format_float(1.0), "1.0")

assert_eq_string(fmt.format_char('a'), "'a'")
assert_eq_string(fmt.format_char('X'), "'X'")

fr fr Test padding functions
assert_eq_string(fmt.pad_left("hello", 8, ' '), "   hello")
assert_eq_string(fmt.pad_right("hello", 8, ' '), "hello   ")
assert_eq_string(fmt.pad_center("hi", 6, '-'), "--hi--")

fr fr Test padding with already long strings
assert_eq_string(fmt.pad_left("verylongstring", 5, ' '), "verylongstring")
assert_eq_string(fmt.pad_right("verylongstring", 5, ' '), "verylongstring")

fr fr Test repeat_char function
assert_eq_string(fmt.repeat_char('*', 0), "")
assert_eq_string(fmt.repeat_char('*', 1), "*")
assert_eq_string(fmt.repeat_char('*', 3), "***")
assert_eq_string(fmt.repeat_char('*', 5), "*****")

fr fr Test number base formatting
assert_eq_string(fmt.format_binary(0), "0")
assert_eq_string(fmt.format_binary(1), "1")
assert_eq_string(fmt.format_binary(2), "10")
assert_eq_string(fmt.format_binary(4), "100")

assert_eq_string(fmt.format_hex(0), "0")
assert_eq_string(fmt.format_hex(1), "1")
assert_eq_string(fmt.format_hex(10), "a")
assert_eq_string(fmt.format_hex(15), "f")
assert_eq_string(fmt.format_hex(16), "10")

assert_eq_string(fmt.format_octal(0), "0")
assert_eq_string(fmt.format_octal(1), "1")
assert_eq_string(fmt.format_octal(7), "7")
assert_eq_string(fmt.format_octal(8), "10")

fr fr Test precision formatting
assert_eq_string(fmt.format_float_precision(3.14159, 2), "3.14")
assert_eq_string(fmt.format_float_precision(2.0, 0), "2")

fr fr Test currency formatting
assert_eq_string(fmt.format_currency(10.50, "$"), "$10.5")
assert_eq_string(fmt.format_currency(42.99, "€"), "€42.99")

fr fr Test scientific notation
assert_eq_string(fmt.format_scientific(1000.0), "1.0e+3")
assert_eq_string(fmt.format_scientific(0.001), "1.0e-3")

fr fr Test percentage formatting
assert_eq_string(fmt.format_percentage(0.5), "50.0%")
assert_eq_string(fmt.format_percentage(1.0), "100.0%")

fr fr Test string formatting with args
sus args := tea[value]{"world", "CURSED"}
assert_eq_string(fmt.format_string("Hello", args), "Hello world CURSED")

sus single_arg := tea[value]{"42"}
assert_eq_string(fmt.format_string("The answer is", single_arg), "The answer is 42")

sus no_args := tea[value]{}
assert_eq_string(fmt.format_string("No args", no_args), "No args")

fr fr Test table formatting
sus columns := tea[value]{"Name", "Age"}
sus widths := normie[value]{10, 5}
sus header := fmt.format_table_header(columns, widths)
assert_true(stringz.contains(header, "Name"))
assert_true(stringz.contains(header, "Age"))

sus row := fmt.format_table_row(columns, widths, " | ")
assert_true(stringz.contains(row, "Name"))
assert_true(stringz.contains(row, "Age"))
assert_true(stringz.contains(row, " | "))

fr fr Test color formatting
sus red_text := fmt.format_with_color("error", "red")
assert_true(stringz.contains(red_text, "error"))
assert_true(stringz.contains(red_text, "\033[31m"))

sus green_text := fmt.format_with_color("success", "green")
assert_true(stringz.contains(green_text, "success"))
assert_true(stringz.contains(green_text, "\033[32m"))

fr fr Test style formatting
sus bold_text := fmt.format_bold("important")
assert_true(stringz.contains(bold_text, "important"))
assert_true(stringz.contains(bold_text, "\033[1m"))

sus italic_text := fmt.format_italic("emphasis")
assert_true(stringz.contains(italic_text, "emphasis"))
assert_true(stringz.contains(italic_text, "\033[3m"))

fr fr Test message formatting
sus error_msg := fmt.format_error("Something went wrong")
assert_true(stringz.contains(error_msg, "ERROR:"))
assert_true(stringz.contains(error_msg, "Something went wrong"))

sus warning_msg := fmt.format_warning("Be careful")
assert_true(stringz.contains(warning_msg, "WARNING:"))
assert_true(stringz.contains(warning_msg, "Be careful"))

sus success_msg := fmt.format_success("Operation completed")
assert_true(stringz.contains(success_msg, "SUCCESS:"))
assert_true(stringz.contains(success_msg, "Operation completed"))

sus info_msg := fmt.format_info("For your information")
assert_true(stringz.contains(info_msg, "INFO:"))
assert_true(stringz.contains(info_msg, "For your information"))

fr fr Test string escaping
sus escaped := fmt.escape_string("Hello \"world\"\nNew line")
assert_true(stringz.contains(escaped, "\\\""))
assert_true(stringz.contains(escaped, "\\n"))

sus unescaped := fmt.unescape_string("Hello \\\"world\\\"\\nNew line")
assert_true(stringz.contains(unescaped, "\""))
assert_true(stringz.contains(unescaped, "\n"))

fr fr Test printable character check
assert_true(fmt.is_printable('a'))
assert_true(fmt.is_printable('Z'))
assert_true(fmt.is_printable('5'))
assert_false(fmt.is_printable('\n'))
assert_false(fmt.is_printable('\t'))

print_test_summary()
