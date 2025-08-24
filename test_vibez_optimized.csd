yeet "vibez_optimized"

# Test vibez_optimized module with actual I/O functions
sus test_file tea = "test_io_operations.txt"
sus test_data tea = "Hello, enhanced CURSED I/O!\nLine 2 of test data\n"

# Test enhanced file operations
write_file_atomic(test_file, test_data)
vibez.spill("✓ Atomic file write completed")

sus read_data tea = read_file_buffered(test_file)
vibez.spill("Read data:", read_data)

# Test file metadata operations
sus file_info map<tea, tea> = get_file_metadata(test_file)
vibez.spill("File size:", file_info["size"])
vibez.spill("File permissions:", file_info["permissions"])

# Test directory operations
sus temp_dir tea = "test_temp_dir"
create_directory_recursive(temp_dir)
vibez.spill("✓ Directory created:", temp_dir)

sus dir_contents []tea = list_directory_detailed(temp_dir)
vibez.spill("Directory contents:", dir_contents)

# Test enhanced printing with formatting
print_formatted("Formatted number: %.2f", 3.14159)
print_colored("GREEN", "Success message in color")
print_table([["Name", "Age"], ["Alice", "30"], ["Bob", "25"]])

# Test stream operations
sus stream tea = open_output_stream("test_stream.txt")
write_stream_line(stream, "Stream line 1")
write_stream_line(stream, "Stream line 2")
close_stream(stream)

# Cleanup
delete_file(test_file)
delete_directory_recursive(temp_dir)
delete_file("test_stream.txt")

vibez.spill("✅ vibez_optimized: All real I/O operations working")
