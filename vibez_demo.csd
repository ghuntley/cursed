yeet "vibez"

vibez.spill("=== CURSED Enhanced I/O Functions Demo ===\n")

fr fr Demonstrate enhanced parsing functions
vibez.spill("1. Enhanced Parsing Functions:")
sus user_age normie = vibez.parse_int("25")
sus user_score meal = vibez.parse_float("98.5")
sus user_active lit = vibez.parse_bool("true")

vibez.spillf("User age: %d", vibez.int_to_string(user_age))
vibez.spillf("User score: %s", vibez.float_to_string(user_score))
vibez.spillf("User active: %s", vibez.format_bool(user_active))
vibez.spill("")

fr fr Demonstrate file operations
vibez.spill("2. File Operations:")
sus demo_filename tea = "demo_output.txt"
sus demo_content tea = "Hello from enhanced CURSED I/O!\nThis file was created using vibez module."

fr fr Write file
sus write_success lit = vibez.write_file(demo_filename, demo_content)
vibez.spillf("File written: %s", vibez.format_bool(write_success))

fr fr Check file exists
sus exists lit = vibez.file_exists(demo_filename)
vibez.spillf("File exists: %s", vibez.format_bool(exists))

fr fr Read file back
sus read_content tea = vibez.read_file(demo_filename)
vibez.spillf("File content length: %d", vibez.string_length(read_content))

fr fr Get file size
sus size normie = vibez.file_size(demo_filename)
vibez.spillf("File size: %d bytes", size)

fr fr Append to file
sus append_success lit = vibez.append_file(demo_filename, "\nAppended line!")
vibez.spillf("Content appended: %s", vibez.format_bool(append_success))

vibez.spill("")

fr fr Demonstrate path utilities
vibez.spill("3. Path and String Utilities:")
sus filename tea = "document.txt"
sus extension tea = vibez.get_file_extension(filename)
sus basename tea = vibez.get_filename_without_extension(filename)

vibez.spillf("Filename: %s", filename)
vibez.spillf("Extension: %s", extension)
vibez.spillf("Basename: %s", basename)

sus abs_path tea = "/home/user/documents"
sus is_abs lit = vibez.is_absolute_path(abs_path)
vibez.spillf("Path '%s' is absolute: %s", abs_path, vibez.format_bool(is_abs))

vibez.spill("")

fr fr Demonstrate directory operations
vibez.spill("4. Directory Operations:")
sus test_dir tea = "test_directory"

fr fr Create directory
sus dir_created lit = vibez.create_dir(test_dir)
vibez.spillf("Directory created: %s", vibez.format_bool(dir_created))

fr fr Check if directory exists
sus dir_exists lit = vibez.dir_exists(test_dir)
vibez.spillf("Directory exists: %s", vibez.format_bool(dir_exists))

fr fr List directory (will be empty)
sus dir_contents [tea] = vibez.list_dir(test_dir)
vibez.spill("Directory contents: (empty)")

fr fr Remove directory
sus dir_removed lit = vibez.remove_dir(test_dir)
vibez.spillf("Directory removed: %s", vibez.format_bool(dir_removed))

vibez.spill("")

fr fr Demonstrate string operations
vibez.spill("5. Enhanced String Operations:")
sus test_string tea = "Hello, enhanced CURSED!"
sus string_len normie = vibez.string_length(test_string)
vibez.spillf("String length: %d", string_len)

sus contains_cursed lit = vibez.string_contains(test_string, "CURSED")
vibez.spillf("Contains 'CURSED': %s", vibez.format_bool(contains_cursed))

sus starts_hello lit = vibez.string_starts_with(test_string, "Hello")
vibez.spillf("Starts with 'Hello': %s", vibez.format_bool(starts_hello))

sus ends_exclaim lit = vibez.string_ends_with(test_string, "!")
vibez.spillf("Ends with '!': %s", vibez.format_bool(ends_exclaim))

vibez.spill("")

fr fr Demonstrate enhanced formatting
vibez.spill("6. Enhanced Formatting:")
vibez.spillf("Simple format: Hello %s", "World")
vibez.spillf("User format: User: %s, ID: %d", "Alice", "12345")
vibez.spillf("Multi-value: %s %s %s", "One", "Two", "Three")
vibez.spillf("Error format: Error: %s", "Something went wrong")

vibez.spill("")

fr fr Demonstrate console control
vibez.spill("7. Console Control:")
vibez.spill_colored("This is red text", "red")
vibez.spill_colored("This is green text", "green")
vibez.spill_colored("This is blue text", "blue")

vibez.spill("")

fr fr Demonstrate timestamps and utilities
vibez.spill("8. Timestamps and Utilities:")
sus timestamp tea = vibez.get_current_timestamp()
vibez.spillf("Current timestamp: %s", timestamp)

vibez.spill_with_time("Timestamped message")

vibez.spill("")

fr fr Clean up demo file
sus cleanup_success lit = vibez.delete_file(demo_filename)
vibez.spillf("Demo file cleaned up: %s", vibez.format_bool(cleanup_success))

vibez.spill("\n=== Enhanced I/O Functions Demo Complete ===")
vibez.spill("All enhanced vibez I/O functions demonstrated successfully!")
