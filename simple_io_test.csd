yeet "io_simple"

# Test console input functions directly
sus line tea = io_read_line()
vibez.spill("Read line: " + line)

sus char tea = io_read_char()
vibez.spill("Read char: " + char)

sus int_val normie = io_read_int()
vibez.spill("Read int: " + string_format_int(int_val))

sus float_val meal = io_read_float()
vibez.spill("Read float: " + string_format_float(float_val))

# Test parsing functions
sus parsed_int normie = io_parse_int("123")
vibez.spill("Parsed int 123: " + string_format_int(parsed_int))

sus parsed_float meal = io_parse_float("2.5")
vibez.spill("Parsed float 2.5: " + string_format_float(parsed_float))

sus parsed_bool lit = io_parse_bool("based")
vibez.spill("Parsed bool 'based': " + string_format_bool(parsed_bool))

# Test output functions
io_print("Hello ")
io_println("World!")
io_print_int(42)
io_println("")
io_print_float(3.14)
io_println("")
io_print_bool(based)
io_println("")
io_print_bool(cap)
io_println("")

vibez.spill("I/O Simple module test completed successfully!")
