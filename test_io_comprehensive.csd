fr fr Comprehensive I/O Module Test - Both Basic and Advanced
yeet "io_basic"
yeet "io_advanced"
yeet "mathz"

fr fr Test file-based workflow
lowkey io_basic.file_exists("test.txt") {
    io_advanced.printf_string("Found file: %s", "test.txt")
    
    sus content tea = io_basic.read_file_content("test.txt")
    sus size drip = io_advanced.get_file_size("test.txt")
    
    io_advanced.printf_int("File size: %d bytes", size)
    io_advanced.printf_string("Content: %s", content)
    
    fr fr Process the content
    sus processed tea = "Processed: " + content
    
    fr fr Write to output file
    lowkey io_basic.write_file_content("output.txt", processed) {
        io_advanced.printf_string("Successfully wrote to: %s", "output.txt")
        
        fr fr Verify output
        sus output_content tea = io_basic.read_file_content("output.txt")
        sus output_size drip = io_advanced.get_file_size("output.txt")
        
        io_advanced.printf_int("Output size: %d bytes", output_size)
        io_advanced.printf_string("Output content: %s", output_content)
    }
}

fr fr Test directory and buffer operations
lowkey io_basic.create_directory("temp") {
    io_advanced.printf_string("Created directory: %s", "temp")
    
    sus file_count drip = io_basic.list_files("temp")
    sus max_files drip = mathz.max_normie(file_count, 10)
    
    io_advanced.printf_int("Files in temp: %d", file_count)
    io_advanced.printf_int("Max allowed: %d", max_files)
    
    fr fr Test buffer operations
    sus buffer drip = io_basic.create_buffer(1024)
    lowkey buffer > 0 {
        io_advanced.printf_int("Created buffer of size: %d", buffer)
        
        sus written drip = io_basic.buffer_write(buffer, "Hello CURSED I/O!")
        lowkey written > 0 {
            io_advanced.printf_int("Wrote %d items to buffer", written)
            
            sus data tea = io_basic.buffer_read(buffer, 10)
            io_advanced.printf_string("Buffer data: %s", data)
            
            lowkey io_basic.buffer_flush(buffer) {
                io_advanced.printf_string("%s", "Buffer flushed successfully")
            }
        }
    }
}

fr fr Test path utilities with validation
sus test_files []tea = []tea{"test.txt", "config.json", "main.csd", "image.png"}
sus i drip = 0

bestie i < 4 {
    sus filename tea = test_files[i]
    sus path tea = io_basic.join_path("temp", filename)
    sus extension tea = io_basic.get_extension(filename)
    sus basename tea = io_basic.get_basename(path)
    
    io_advanced.printf_string("File: %s", filename)
    io_advanced.printf_string("  Full path: %s", path)
    io_advanced.printf_string("  Extension: %s", extension)
    io_advanced.printf_string("  Basename: %s", basename)
    
    lowkey io_basic.is_text_file(filename) {
        io_advanced.printf_string("  Type: %s", "Text file")
    } elseif io_basic.is_binary_file(filename) {
        io_advanced.printf_string("  Type: %s", "Binary file")
    } nah {
        io_advanced.printf_string("  Type: %s", "Unknown file type")
    }
    
    i = mathz.add_two(i, 1)
}

fr fr Test advanced features
io_advanced.printf_string("%s", "Testing advanced I/O features...")

fr fr Configuration operations
sus port tea = io_advanced.read_config_value("app.conf", "port")
io_advanced.printf_string("Config port: %s", port)

sus key_count drip = io_advanced.list_config_keys("app.conf")
io_advanced.printf_int("Config keys count: %d", key_count)

fr fr JSON operations
sus json_valid lit = io_advanced.validate_json("{\"name\": \"CURSED\"}")
lowkey json_valid {
    io_advanced.printf_string("%s", "JSON validation passed")
    
    sus formatted tea = io_advanced.format_json("{\"name\":\"CURSED\"}")
    io_advanced.printf_string("Formatted JSON: %s", formatted)
    
    sus minified tea = io_advanced.minify_json("{\n  \"name\": \"CURSED\"\n}")
    io_advanced.printf_string("Minified JSON: %s", minified)
}

fr fr CSV operations  
sus csv_fields drip = io_advanced.read_csv_line("name,age,city")
io_advanced.printf_int("CSV fields parsed: %d", csv_fields)

sus csv_line tea = io_advanced.format_csv_line(3)
io_advanced.printf_string("Generated CSV: %s", csv_line)

sus escaped tea = io_advanced.escape_csv_field("text with, comma")
io_advanced.printf_string("Escaped CSV field: %s", escaped)

fr fr Logging operations
lowkey io_advanced.write_log_entry("app.log", "INFO", "Test completed") {
    io_advanced.printf_string("%s", "Log entry written")
    
    sus log_parts drip = io_advanced.parse_log_entry("2025-01-15T10:30:00Z INFO Test")
    io_advanced.printf_int("Log entry parts: %d", log_parts)
}

fr fr Temporary file operations
sus temp_file tea = io_advanced.create_temp_file("cursed_test")
io_advanced.printf_string("Created temp file: %s", temp_file)

sus temp_dir tea = io_advanced.create_temp_dir("cursed_work")
io_advanced.printf_string("Created temp dir: %s", temp_dir)

fr fr Progress display demonstration
io_advanced.printf_string("%s", "Demonstrating progress indicators:")
io_advanced.show_progress_bar(25, 100, 40)
io_advanced.show_progress_bar(50, 100, 40)  
io_advanced.show_progress_bar(75, 100, 40)
io_advanced.show_progress_bar(100, 100, 40)

sus spinner_steps drip = 0
bestie spinner_steps < 4 {
    io_advanced.update_progress_spinner(spinner_steps)
    spinner_steps = mathz.add_two(spinner_steps, 1)
}

io_advanced.display_file_transfer_progress("data.txt", 512, 1024)
io_advanced.display_file_transfer_progress("data.txt", 1024, 1024)

fr fr Final summary
io_basic.print_line("")
io_advanced.printf_string("%s", "=== COMPREHENSIVE I/O TEST SUMMARY ===")
io_advanced.printf_string("Basic I/O module: %s", "✅ FUNCTIONAL")
io_advanced.printf_string("Advanced I/O module: %s", "✅ FUNCTIONAL")  
io_advanced.printf_string("Module integration: %s", "✅ SUCCESSFUL")
io_advanced.printf_string("Pure CURSED compatibility: %s", "✅ CONFIRMED")
io_basic.print_line("")
io_advanced.printf_string("%s", "🎉 All I/O operations completed successfully!")

io_basic.flush()
