yeet "vibez"

fr fr CURSED vibez I/O Module Demonstration

vibez.spill("=== CURSED vibez I/O Module Demo ===\n")

fr fr Basic Output Functions
vibez.spill("1. Basic Output:")
vibez.spill("   Hello from vibez.spill!")
vibez.spillln("   Line with automatic newline")

fr fr Formatted Output
vibez.spill("\n2. Formatted Output:")
vibez.spillf("   User: %s, ID: %d", "Alice", "12345")
vibez.spillfln("   Formatted with newline: %s", "Success!")

sus formatted tea = vibez.spillstr("Value: %d", "42")
vibez.spill("   Formatted string result: " + formatted)

fr fr Multiple Values and Separators
vibez.spill("\n3. Multiple Values:")
vibez.spill_values("Multiple", "values", "printed", "together")
vibez.spill("\n   ")
vibez.spill_sep(" | ", "Item1", "Item2", "Item3")

fr fr Colored Output
vibez.spill("\n\n4. Colored Output:")
vibez.spill_colored("   This text is red!", "red")
vibez.spill_colored("   This text is green!", "green")
vibez.spill_colored("   This text is blue!", "blue")

fr fr Error and Warning Messages
vibez.spill("\n5. Message Types:")
vibez.spill_error("Sample error message")
vibez.spill_warning("Sample warning message")
vibez.spill_debug("Sample debug message")

fr fr File Operations Demo
vibez.spill("\n6. File Operations:")
sus write_success lit = vibez.write_file("demo_file.txt", "Hello from CURSED!")
vibez.spillf("   File write success: %s", vibez.format_bool(write_success))

sus file_content tea = vibez.read_file("demo_file.txt")
vibez.spill("   File content: " + file_content)

sus append_success lit = vibez.append_file("demo_file.txt", "\nAppended line!")
vibez.spillf("   File append success: %s", vibez.format_bool(append_success))

fr fr Directory Operations Demo
vibez.spill("\n7. Directory Operations:")
sus create_dir_success lit = vibez.create_dir("demo_directory")
vibez.spillf("   Directory create success: %s", vibez.format_bool(create_dir_success))

sus dir_exists lit = vibez.dir_exists("demo_directory")
vibez.spillf("   Directory exists: %s", vibez.format_bool(dir_exists))

fr fr Number and Type Formatting
vibez.spill("\n8. Type Formatting:")
sus number_str tea = vibez.format_number(12345)
vibez.spill("   Number formatted: " + number_str)

sus bool_str tea = vibez.format_bool(based)
vibez.spill("   Boolean formatted: " + bool_str)

sus float_str tea = vibez.format_float(3.14159)
vibez.spill("   Float formatted: " + float_str)

fr fr Input Parsing Demo
vibez.spill("\n9. Input Parsing:")
sus parsed_int normie = vibez.parse_int("42")
vibez.spillf("   Parsed integer: %d", vibez.format_number(parsed_int))

sus parsed_bool lit = vibez.parse_bool("based")
vibez.spillf("   Parsed boolean: %s", vibez.format_bool(parsed_bool))

fr fr Error Handling Demo
vibez.spill("\n10. Error Handling:")
vibez.clear_io_error()
sus has_error lit = vibez.has_io_error()
vibez.spillf("    Has I/O error: %s", vibez.format_bool(has_error))

sus (content, error) = vibez.safe_read_file("nonexistent.txt")
vibez.spill("    Safe read result: " + content)

fr fr Console Control
vibez.spill("\n11. Console Control:")
vibez.spill_with_time("Timestamped message")

vibez.spill("\n=== vibez I/O Module Demo Complete ===")
vibez.spill("All I/O capabilities demonstrated successfully!")
