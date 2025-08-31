fr fr Test CURSED Advanced I/O Module
yeet "io_advanced"

fr fr Test formatted output
io_advanced.printf_string("%s", "Hello World")
io_advanced.printf_int("Count: %d", 42)
io_advanced.printf_float("Value: %.2f", 3.14159)
io_advanced.printf_bool("Status: %t", based)

fr fr Test validated input
sus age drip = io_advanced.read_validated_int("Enter age (1-100): ", 1, 100)
io_advanced.printf_int("Age entered: %d", age)

sus name tea = io_advanced.read_validated_string("Enter name (3-20 chars): ", 3, 20)
io_advanced.printf_string("Name: %s", name)

sus email tea = io_advanced.read_email("Enter email: ")
io_advanced.printf_string("Email: %s", email)

sus phone tea = io_advanced.read_phone("Enter phone: ")
io_advanced.printf_string("Phone: %s", phone)

lowkey io_advanced.read_yes_no("Continue?") {
    io_advanced.printf_string("%s", "Continuing...")
} nah {
    io_advanced.printf_string("%s", "Stopping...")
}

fr fr Test stream operations
sus input_stream drip = io_advanced.create_input_stream("stdin")
sus output_stream drip = io_advanced.create_output_stream("stdout")

lowkey input_stream > 0 && output_stream > 0 {
    sus data tea = io_advanced.stream_read(input_stream, 100)
    sus written drip = io_advanced.stream_write(output_stream, data)
    io_advanced.printf_int("Bytes written: %d", written)
    
    io_advanced.stream_close(input_stream)
    io_advanced.stream_close(output_stream)
}

fr fr Test file information
sus size drip = io_advanced.get_file_size("large.txt")
io_advanced.printf_int("File size: %d bytes", size)

sus modified tea = io_advanced.get_file_modified_time("test.txt")
io_advanced.printf_string("Modified: %s", modified)

sus permissions tea = io_advanced.get_file_permissions("executable.bin")
io_advanced.printf_string("Permissions: %s", permissions)

lowkey io_advanced.is_file_readable("test.txt") {
    io_advanced.printf_string("%s", "File is readable")
}

lowkey io_advanced.is_file_writable("test.txt") {
    io_advanced.printf_string("%s", "File is writable")
}

fr fr Test CSV operations
sus field_count drip = io_advanced.read_csv_line("name,age,city")
io_advanced.printf_int("CSV fields: %d", field_count)

sus csv_line tea = io_advanced.format_csv_line(3)
io_advanced.printf_string("CSV line: %s", csv_line)

sus escaped tea = io_advanced.escape_csv_field("text with, comma")
io_advanced.printf_string("Escaped: %s", escaped)

fr fr Test JSON operations
lowkey io_advanced.validate_json("{\"name\": \"value\"}") {
    io_advanced.printf_string("%s", "JSON is valid")
}

sus formatted tea = io_advanced.format_json("{\"name\":\"value\"}")
io_advanced.printf_string("Formatted JSON:\n%s", formatted)

sus minified tea = io_advanced.minify_json("{\n  \"name\": \"value\"\n}")
io_advanced.printf_string("Minified: %s", minified)

fr fr Test configuration operations
sus port tea = io_advanced.read_config_value("app.conf", "port")
io_advanced.printf_string("Port: %s", port)

lowkey io_advanced.write_config_value("app.conf", "debug", "true") {
    io_advanced.printf_string("%s", "Config value written")
}

sus key_count drip = io_advanced.list_config_keys("app.conf")
io_advanced.printf_int("Config keys: %d", key_count)

fr fr Test logging
lowkey io_advanced.write_log_entry("app.log", "INFO", "Test message") {
    io_advanced.printf_string("%s", "Log entry written")
}

lowkey io_advanced.rotate_log_file("app.log", 1000000) {
    io_advanced.printf_string("%s", "Log file rotated")
}

sus log_parts drip = io_advanced.parse_log_entry("2025-01-15T10:30:00Z INFO Test message")
io_advanced.printf_int("Log parts: %d", log_parts)

fr fr Test temporary files
sus temp_file tea = io_advanced.create_temp_file("test")
io_advanced.printf_string("Temp file: %s", temp_file)

sus temp_dir tea = io_advanced.create_temp_dir("work")
io_advanced.printf_string("Temp dir: %s", temp_dir)

sus cleaned drip = io_advanced.cleanup_temp_files("*.tmp")
io_advanced.printf_int("Cleaned files: %d", cleaned)

fr fr Test progress display
io_advanced.show_progress_bar(50, 100, 40)
io_advanced.update_progress_spinner(1)
io_advanced.display_file_transfer_progress("data.txt", 1024, 2048)

io_advanced.printf_string("%s", "Advanced I/O test completed successfully!")
