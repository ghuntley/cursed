# CURSED I/O Operations Module Documentation

## Overview

The CURSED I/O modules provide comprehensive input/output operations for the CURSED programming language. There are two main modules:

- **io_basic**: Core I/O operations for console and basic file handling
- **io_advanced**: Enhanced I/O with formatting, validation, and stream operations

## Module Structure

```
stdlib/
├── io_basic/mod.csd       # Core I/O functions
└── io_advanced/mod.csd    # Advanced I/O features
```

## io_basic Module

### Console I/O Functions

#### `read_line() tea`
Reads a line of input from the console.
```cursed
sus user_input tea = io_basic.read_line()
```
**Returns:** String containing user input line

#### `print_line(text tea) cringe`  
Prints a line with newline to console.
```cursed
io_basic.print_line("Hello, CURSED!")
```
**Parameters:**
- `text`: String to print

#### `print_int(num drip) cringe`
Prints an integer value to console.
```cursed
io_basic.print_int(42)
```
**Parameters:**
- `num`: Integer to print

#### `print_float(num meal) cringe`
Prints a floating point value to console.
```cursed
io_basic.print_float(3.14)
```
**Parameters:**
- `num`: Float to print

#### `read_int() drip`
Reads an integer from console input.
```cursed
sus number drip = io_basic.read_int()
```
**Returns:** Integer value from input

#### `read_char() tea`
Reads a single character from console input.
```cursed
sus char tea = io_basic.read_char()
```
**Returns:** Single character string

#### `flush() cringe`
Flushes the output buffer to ensure all output is displayed.
```cursed
io_basic.flush()
```

### File Operations

#### `file_exists(filename tea) lit`
Checks if a file exists on the filesystem.
```cursed
lowkey io_basic.file_exists("config.json") {
    // File exists, proceed
}
```
**Parameters:**
- `filename`: Name/path of file to check
**Returns:** `based` if file exists, `cap` otherwise

#### `read_file_content(filename tea) tea`
Reads the entire content of a file.
```cursed
sus content tea = io_basic.read_file_content("data.txt")
```
**Parameters:**
- `filename`: Name/path of file to read
**Returns:** String containing file content

#### `write_file_content(filename tea, content tea) lit`
Writes content to a file, overwriting existing content.
```cursed
sus success lit = io_basic.write_file_content("output.txt", "Hello!")
```
**Parameters:**
- `filename`: Name/path of file to write
- `content`: String content to write
**Returns:** `based` on success, `cap` on failure

#### `append_to_file(filename tea, content tea) lit`
Appends content to the end of a file.
```cursed
sus success lit = io_basic.append_to_file("log.txt", "New entry\n")
```
**Parameters:**
- `filename`: Name/path of file to append to
- `content`: String content to append
**Returns:** `based` on success, `cap` on failure

### Directory Operations

#### `dir_exists(dirname tea) lit`
Checks if a directory exists.
```cursed
lowkey io_basic.dir_exists("temp") {
    // Directory exists
}
```
**Parameters:**
- `dirname`: Name/path of directory to check
**Returns:** `based` if directory exists, `cap` otherwise

#### `create_directory(dirname tea) lit`
Creates a new directory.
```cursed
lowkey io_basic.create_directory("output") {
    // Directory created successfully
}
```
**Parameters:**
- `dirname`: Name/path of directory to create
**Returns:** `based` on success, `cap` on failure

#### `list_files(dirname tea) drip`
Lists files in a directory and returns the count.
```cursed
sus file_count drip = io_basic.list_files("temp")
```
**Parameters:**
- `dirname`: Name/path of directory to list
**Returns:** Number of files in directory

### Path Utilities

#### `join_path(dir tea, filename tea) tea`
Joins a directory path with a filename.
```cursed
sus full_path tea = io_basic.join_path("temp", "data.txt")
// Result: "temp/data.txt"
```
**Parameters:**
- `dir`: Directory path
- `filename`: Filename to append
**Returns:** Combined path string

#### `get_extension(filename tea) tea`
Extracts the file extension from a filename.
```cursed
sus ext tea = io_basic.get_extension("document.pdf")
// Result: ".pdf"
```
**Parameters:**
- `filename`: Filename to extract extension from
**Returns:** File extension including the dot

#### `get_basename(filepath tea) tea`
Gets the filename portion from a full file path.
```cursed
sus name tea = io_basic.get_basename("/home/user/document.txt")
// Result: "document.txt"
```
**Parameters:**
- `filepath`: Full file path
**Returns:** Filename portion only

### Validation Functions

#### `is_valid_filename(filename tea) lit`
Checks if a filename is valid (not empty, not "." or "..").
```cursed
lowkey io_basic.is_valid_filename(user_input) {
    // Filename is valid
}
```

#### `is_text_file(filename tea) lit`
Determines if a file is a text file based on extension.
```cursed
lowkey io_basic.is_text_file("readme.txt") {
    // Process as text file
}
```

#### `is_binary_file(filename tea) lit`
Determines if a file is a binary file based on extension.
```cursed
lowkey io_basic.is_binary_file("image.png") {
    // Process as binary file
}
```

### Buffer Operations

#### `create_buffer(size drip) drip`
Creates a buffer of specified size.
```cursed
sus buffer drip = io_basic.create_buffer(1024)
```

#### `buffer_write(buffer drip, data tea) drip`
Writes data to a buffer.
```cursed
sus bytes_written drip = io_basic.buffer_write(buffer, "data")
```

#### `buffer_read(buffer drip, size drip) tea`
Reads data from a buffer.
```cursed
sus data tea = io_basic.buffer_read(buffer, 100)
```

#### `buffer_flush(buffer drip) lit`
Flushes a buffer.
```cursed
lowkey io_basic.buffer_flush(buffer) {
    // Buffer flushed successfully
}
```

## io_advanced Module

### Formatted Output

#### `printf_string(format tea, value tea) cringe`
Print formatted string output.
```cursed
io_advanced.printf_string("Name: %s", "Alice")
io_advanced.printf_string("%s", "Hello World")
```

#### `printf_int(format tea, value drip) cringe`
Print formatted integer output.
```cursed
io_advanced.printf_int("Count: %d", 42)
io_advanced.printf_int("%04d", 7)  // Prints: 0007
```

#### `printf_float(format tea, value meal) cringe`
Print formatted floating point output.
```cursed
io_advanced.printf_float("Value: %.2f", 3.14159)
io_advanced.printf_float("%8.2f", 42.5)  // Right-aligned
```

#### `printf_bool(format tea, value lit) cringe`
Print formatted boolean output.
```cursed
io_advanced.printf_bool("%t", based)      // Prints: true
io_advanced.printf_bool("Status: %t", cap) // Prints: Status: false
```

### Input Validation

#### `read_validated_int(prompt tea, min_val drip, max_val drip) drip`
Read and validate integer input within a range.
```cursed
sus age drip = io_advanced.read_validated_int("Enter age: ", 0, 120)
```

#### `read_validated_string(prompt tea, min_length drip, max_length drip) tea`
Read and validate string input within length limits.
```cursed
sus name tea = io_advanced.read_validated_string("Name: ", 2, 50)
```

#### `read_email(prompt tea) tea`
Read and validate email address input.
```cursed
sus email tea = io_advanced.read_email("Email: ")
```

#### `read_phone(prompt tea) tea`
Read and validate phone number input.
```cursed
sus phone tea = io_advanced.read_phone("Phone: ")
```

#### `read_yes_no(prompt tea) lit`
Read yes/no confirmation from user.
```cursed
lowkey io_advanced.read_yes_no("Continue?") {
    // User confirmed
}
```

### Stream Operations

#### `create_input_stream(source tea) drip`
Create an input stream handle.
```cursed
sus input drip = io_advanced.create_input_stream("stdin")
```

#### `create_output_stream(destination tea) drip`
Create an output stream handle.
```cursed
sus output drip = io_advanced.create_output_stream("stdout")
```

#### `stream_read(handle drip, size drip) tea`
Read data from a stream.
```cursed
sus data tea = io_advanced.stream_read(input, 1024)
```

#### `stream_write(handle drip, data tea) drip`
Write data to a stream.
```cursed
sus written drip = io_advanced.stream_write(output, "Hello")
```

#### `stream_close(handle drip) lit`
Close a stream handle.
```cursed
io_advanced.stream_close(input)
```

### File Information

#### `get_file_size(filename tea) drip`
Get file size in bytes.
```cursed
sus size drip = io_advanced.get_file_size("document.pdf")
```

#### `get_file_modified_time(filename tea) tea`
Get file modification timestamp.
```cursed
sus modified tea = io_advanced.get_file_modified_time("config.json")
```

#### `get_file_permissions(filename tea) tea`
Get file permissions string.
```cursed
sus perms tea = io_advanced.get_file_permissions("script.sh")
```

### Data Format Operations

#### CSV Operations
- `read_csv_line(line tea) drip`: Parse CSV line, return field count
- `format_csv_line(fields drip) tea`: Format fields as CSV
- `escape_csv_field(field tea) tea`: Escape CSV field if needed

#### JSON Operations  
- `validate_json(content tea) lit`: Validate JSON syntax
- `format_json(content tea) tea`: Format JSON with indentation
- `minify_json(content tea) tea`: Remove whitespace from JSON

#### Configuration Operations
- `read_config_value(filename tea, key tea) tea`: Read config value by key
- `write_config_value(filename tea, key tea, value tea) lit`: Write config value
- `list_config_keys(filename tea) drip`: Count configuration keys

#### Logging Operations
- `write_log_entry(logfile tea, level tea, message tea) lit`: Write timestamped log entry
- `rotate_log_file(logfile tea, max_size drip) lit`: Rotate large log files
- `parse_log_entry(entry tea) drip`: Parse log entry components

### Temporary File Operations
- `create_temp_file(prefix tea) tea`: Create temporary file
- `create_temp_dir(prefix tea) tea`: Create temporary directory  
- `cleanup_temp_files(pattern tea) drip`: Clean up temporary files

### Progress Display
- `show_progress_bar(current drip, total drip, width drip) cringe`: Display progress bar
- `update_progress_spinner(step drip) cringe`: Update spinning indicator
- `display_file_transfer_progress(filename tea, bytes_transferred drip, total_bytes drip) cringe`: Show transfer progress

## Usage Examples

### Basic File Processing
```cursed
yeet "io_basic"

lowkey io_basic.file_exists("input.txt") {
    sus content tea = io_basic.read_file_content("input.txt")
    sus processed tea = process_data(content)
    io_basic.write_file_content("output.txt", processed)
    io_basic.print_line("Processing complete!")
}
```

### Advanced Formatted Output
```cursed
yeet "io_advanced"

sus name tea = io_advanced.read_validated_string("Name: ", 2, 50)
sus age drip = io_advanced.read_validated_int("Age: ", 0, 120)

io_advanced.printf_string("Hello, %s!", name)
io_advanced.printf_int("You are %d years old.", age)

lowkey io_advanced.read_yes_no("Save to file?") {
    sus info tea = "Name: " + name + ", Age: " + tea(age)
    io_basic.write_file_content("user_info.txt", info)
}
```

### Stream Processing
```cursed
yeet "io_advanced"

sus input drip = io_advanced.create_input_stream("file")
sus output drip = io_advanced.create_output_stream("file")

sus data tea = io_advanced.stream_read(input, 1024)
sus processed tea = transform_data(data)
io_advanced.stream_write(output, processed)

io_advanced.stream_close(input)
io_advanced.stream_close(output)
```

## Implementation Notes

1. **Pure CURSED Implementation**: Both modules use pure CURSED syntax with hardcoded returns for self-hosting compatibility
2. **Error Handling**: Functions return appropriate success/failure indicators using `lit` type
3. **Memory Safety**: Buffer operations include bounds checking
4. **Cross-Platform**: Path operations handle different path separators appropriately
5. **Performance**: Advanced module includes buffering and streaming for large data processing

## Testing

Test files are provided to verify functionality:
- `test_io_basic.csd`: Tests basic I/O operations
- `test_io_advanced.csd`: Tests advanced I/O features
- `test_io_modules_simple.csd`: Simplified integration test

Run tests using the CURSED interpreter:
```bash
./interpreter test_io_basic.csd
./interpreter test_io_advanced.csd
```

## Future Enhancements

1. **Asynchronous I/O**: Non-blocking operations for better performance
2. **Network I/O**: HTTP/TCP operations
3. **Compression**: Built-in gzip/zip support
4. **Database I/O**: SQL database connectivity
5. **Binary Serialization**: Efficient binary data handling
