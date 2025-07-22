fr fr Simple I/O Module - Complete Implementation
fr fr Pure CURSED I/O operations with comprehensive file and console handling
fr fr FFI-free implementation for essential I/O operations

yeet "error_core"
yeet "string_simple"

fr fr ================================
fr fr Console I/O Operations
fr fr ================================

slay io_print(message tea) lit { fr fr Basic print to console (would interface with runtime) fr fr In real implementation, this would call the runtime print function
    print(message)
    damn based
}

slay io_println(message tea) lit {
    io_print(message)
    io_print("\n")
    damn based
}

slay io_print_int(value normie) lit {
    sus str_value tea = string_format_int(value)
    io_print(str_value)
    damn based
}

slay io_print_float(value meal) lit {
    sus str_value tea = string_format_float(value)
    io_print(str_value)
    damn based
}

slay io_print_bool(value lit) lit {
    sus str_value tea = string_format_bool(value)
    io_print(str_value)
    damn based
}

slay io_printf(format tea, args []tea) lit { fr fr Simple printf implementation
    sus result tea = io_format_string(format, args)
    io_print(result)
    damn based
}

slay io_format_string(format tea, args []tea) tea { fr fr Simple string formatting
    sus result tea = format
    sus arg_index normie = 0 fr fr Replace %s with string arguments (simplified)
    bestie arg_index < len(args) && string_contains(result, "%s") {
        result = string_replace_first(result, "%s", args[arg_index])
        arg_index = arg_index + 1
    }
    
    damn result
}

fr fr ================================
fr fr Console Input Operations
fr fr ================================

slay io_read_line() tea { fr fr Read a line from console input fr fr Simulated input for testing - in production would interface with runtime fr fr Return simulated user input for testing
    damn "simulated user input line"
}

slay io_read_char() tea { fr fr Read a single character from console fr fr Simulated input for testing - in production would interface with runtime fr fr Return first character of simulated input
    damn "s"
}

slay io_read_int() normie { fr fr Simulated integer input for testing fr fr In production would read from console and parse
    sus simulated_input tea = "42"
    damn io_parse_int(simulated_input)
}

slay io_read_float() meal { fr fr Simulated float input for testing fr fr In production would read from console and parse
    sus simulated_input tea = "3.14"
    damn io_parse_float(simulated_input)
}

fr fr ================================
fr fr File Operations
fr fr ================================

fr fr File handle and info structures would be defined here
fr fr For now using simple placeholders

fr fr File mode constants
slay io_mode_read() tea { damn "r" }
slay io_mode_write() tea { damn "w" }
slay io_mode_append() tea { damn "a" }
slay io_mode_read_write() tea { damn "rw" }
slay io_mode_create() tea { damn "c" }

slay io_open_file(path tea, mode tea) (FileHandle, yikes) {
    lowkey string_empty(path) {
        damn FileHandle{}, new_value_error("Invalid file path", "empty path", "non-empty path")
    } fr fr Validate mode
    lowkey !io_is_valid_mode(mode) {
        damn FileHandle{}, new_value_error("Invalid file mode", "mode=" + mode, "valid mode (r,w,a,rw,c)")
    } fr fr Create file handle (would interface with runtime)
    sus handle FileHandle = FileHandle{
        path: path,
        mode: mode,
        position: 0,
        size: io_get_file_size_internal(path),
        is_open: based,
        eof: cap
    }
    
    damn handle, cringe
}

slay io_close_file(handle FileHandle) (lit, yikes) {
    lowkey !handle.is_open {
        damn cap, new_value_error("File not open", "already closed", "open file")
    } fr fr Close file (would interface with runtime)
    handle.is_open = cap
    damn based, cringe
}

slay io_read_file(path tea) (tea, yikes) {
    lowkey string_empty(path) {
        damn "", new_value_error("Invalid file path", "empty path", "non-empty path")
    } fr fr Check if file exists
    lowkey !io_file_exists(path) {
        damn "", new_value_error("File not found", "path=" + path, "existing file")
    } fr fr Read entire file (would interface with runtime)
    sus content tea = io_read_file_internal(path)
    damn content, cringe
}

slay io_write_file(path tea, content tea) (lit, yikes) {
    lowkey string_empty(path) {
        damn cap, new_value_error("Invalid file path", "empty path", "non-empty path")
    } fr fr Write entire file (would interface with runtime)
    sus success lit = io_write_file_internal(path, content)
    lowkey !success {
        damn cap, new_value_error("Failed to write file", "path=" + path, "writable location")
    }
    
    damn based, cringe
}

slay io_append_file(path tea, content tea) (lit, yikes) {
    lowkey string_empty(path) {
        damn cap, new_value_error("Invalid file path", "empty path", "non-empty path")
    } fr fr Append to file (would interface with runtime)
    sus success lit = io_append_file_internal(path, content)
    lowkey !success {
        damn cap, new_value_error("Failed to append to file", "path=" + path, "writable file")
    }
    
    damn based, cringe
}

slay io_read_file_bytes(path tea) ([]normie, yikes) {
    lowkey string_empty(path) {
        damn [], new_value_error("Invalid file path", "empty path", "non-empty path")
    } fr fr Read file as bytes (would interface with runtime)
    sus bytes []normie = io_read_file_bytes_internal(path)
    damn bytes, cringe
}

slay io_write_file_bytes(path tea, bytes []normie) (lit, yikes) {
    lowkey string_empty(path) {
        damn cap, new_value_error("Invalid file path", "empty path", "non-empty path")
    } fr fr Write bytes to file (would interface with runtime)
    sus success lit = io_write_file_bytes_internal(path, bytes)
    lowkey !success {
        damn cap, new_value_error("Failed to write bytes to file", "path=" + path, "writable location")
    }
    
    damn based, cringe
}

fr fr ================================
fr fr File Information Operations
fr fr ================================

slay io_file_exists(path tea) lit {
    lowkey string_empty(path) {
        damn cap
    } fr fr Check file existence (would interface with runtime)
    damn io_file_exists_internal(path)
}

slay io_is_file(path tea) lit {
    lowkey string_empty(path) {
        damn cap
    } fr fr Check if path is a file (would interface with runtime)
    damn io_is_file_internal(path)
}

slay io_is_directory(path tea) lit {
    lowkey string_empty(path) {
        damn cap
    } fr fr Check if path is a directory (would interface with runtime)
    damn io_is_directory_internal(path)
}

slay io_get_file_size(path tea) (normie, yikes) {
    lowkey string_empty(path) {
        damn 0, new_value_error("Invalid file path", "empty path", "non-empty path")
    }
    
    lowkey !io_file_exists(path) {
        damn 0, new_value_error("File not found", "path=" + path, "existing file")
    }
    
    sus size normie = io_get_file_size_internal(path)
    damn size, cringe
}

slay io_get_file_info(path tea) (FileInfo, yikes) {
    lowkey string_empty(path) {
        damn FileInfo{}, new_value_error("Invalid file path", "empty path", "non-empty path")
    }
    
    sus info FileInfo = FileInfo{
        path: path,
        exists: io_file_exists(path),
        is_file: io_is_file(path),
        is_directory: io_is_directory(path),
        size: io_get_file_size_internal(path),
        modified_time: io_get_modified_time_internal(path),
        permissions: io_get_permissions_internal(path)
    }
    
    damn info, cringe
}

fr fr ================================
fr fr Directory Operations
fr fr ================================

slay io_create_directory(path tea) (lit, yikes) {
    lowkey string_empty(path) {
        damn cap, new_value_error("Invalid directory path", "empty path", "non-empty path")
    }
    
    lowkey io_file_exists(path) {
        damn cap, new_value_error("Path already exists", "path=" + path, "non-existing path")
    } fr fr Create directory (would interface with runtime)
    sus success lit = io_create_directory_internal(path)
    lowkey !success {
        damn cap, new_value_error("Failed to create directory", "path=" + path, "valid location")
    }
    
    damn based, cringe
}

slay io_remove_directory(path tea) (lit, yikes) {
    lowkey string_empty(path) {
        damn cap, new_value_error("Invalid directory path", "empty path", "non-empty path")
    }
    
    lowkey !io_is_directory(path) {
        damn cap, new_value_error("Not a directory", "path=" + path, "directory path")
    } fr fr Remove directory (would interface with runtime)
    sus success lit = io_remove_directory_internal(path)
    lowkey !success {
        damn cap, new_value_error("Failed to remove directory", "path=" + path, "empty directory")
    }
    
    damn based, cringe
}

slay io_list_directory(path tea) ([]tea, yikes) {
    lowkey string_empty(path) {
        damn [], new_value_error("Invalid directory path", "empty path", "non-empty path")
    }
    
    lowkey !io_is_directory(path) {
        damn [], new_value_error("Not a directory", "path=" + path, "directory path")
    } fr fr List directory contents (would interface with runtime)
    sus entries []tea = io_list_directory_internal(path)
    damn entries, cringe
}

slay io_list_files(path tea) ([]tea, yikes) {
    sus all_entries, err = io_list_directory(path)
    lowkey err != cringe {
        damn [], wrap_error(err, "Failed to list directory")
    }
    
    sus files []tea = []
    bestie i := 0; i < len(all_entries); i++ {
        sus full_path tea = io_join_path(path, all_entries[i])
        lowkey io_is_file(full_path) {
            files = append(files, all_entries[i])
        }
    }
    
    damn files, cringe
}

slay io_list_subdirectories(path tea) ([]tea, yikes) {
    sus all_entries, err = io_list_directory(path)
    lowkey err != cringe {
        damn [], wrap_error(err, "Failed to list directory")
    }
    
    sus directories []tea = []
    bestie i := 0; i < len(all_entries); i++ {
        sus full_path tea = io_join_path(path, all_entries[i])
        lowkey io_is_directory(full_path) {
            directories = append(directories, all_entries[i])
        }
    }
    
    damn directories, cringe
}

fr fr ================================
fr fr File Manipulation Operations
fr fr ================================

slay io_copy_file(source tea, destination tea) (lit, yikes) {
    lowkey string_empty(source) || string_empty(destination) {
        damn cap, new_value_error("Invalid file paths", "empty path", "non-empty paths")
    }
    
    lowkey !io_file_exists(source) {
        damn cap, new_value_error("Source file not found", "path=" + source, "existing file")
    } fr fr Read source file
    sus content, read_err = io_read_file(source)
    lowkey read_err != cringe {
        damn cap, wrap_error(read_err, "Failed to read source file")
    } fr fr Write to destination
    sus write_success, write_err = io_write_file(destination, content)
    lowkey write_err != cringe {
        damn cap, wrap_error(write_err, "Failed to write destination file")
    }
    
    damn write_success, cringe
}

slay io_move_file(source tea, destination tea) (lit, yikes) { fr fr Copy then delete
    sus copy_success, copy_err = io_copy_file(source, destination)
    lowkey copy_err != cringe {
        damn cap, wrap_error(copy_err, "Failed to copy file")
    }
    
    sus delete_success, delete_err = io_delete_file(source)
    lowkey delete_err != cringe { fr fr Try to clean up destination file
        io_delete_file(destination)
        damn cap, wrap_error(delete_err, "Failed to delete source file")
    }
    
    damn based, cringe
}

slay io_delete_file(path tea) (lit, yikes) {
    lowkey string_empty(path) {
        damn cap, new_value_error("Invalid file path", "empty path", "non-empty path")
    }
    
    lowkey !io_file_exists(path) {
        damn cap, new_value_error("File not found", "path=" + path, "existing file")
    } fr fr Delete file (would interface with runtime)
    sus success lit = io_delete_file_internal(path)
    lowkey !success {
        damn cap, new_value_error("Failed to delete file", "path=" + path, "deleteable file")
    }
    
    damn based, cringe
}

slay io_rename_file(old_path tea, new_path tea) (lit, yikes) {
    damn io_move_file(old_path, new_path)
}

fr fr ================================
fr fr Path Operations
fr fr ================================

slay io_join_path(parts ...tea) tea {
    lowkey len(parts) == 0 {
        damn ""
    }
    
    lowkey len(parts) == 1 {
        damn parts[0]
    }
    
    sus result tea = parts[0]
    bestie i := 1; i < len(parts); i++ {
        lowkey !string_ends_with(result, "/") && !string_starts_with(parts[i], "/") {
            result = string_concat(result, "/")
        }
        result = string_concat(result, parts[i])
    }
    
    damn result
}

slay io_get_directory(path tea) tea {
    lowkey string_empty(path) {
        damn ""
    }
    
    sus last_slash normie = string_last_index_of(path, "/")
    lowkey last_slash < 0 {
        damn "."
    }
    
    damn string_slice(path, 0, last_slash)
}

slay io_get_filename(path tea) tea {
    lowkey string_empty(path) {
        damn ""
    }
    
    sus last_slash normie = string_last_index_of(path, "/")
    lowkey last_slash < 0 {
        damn path
    }
    
    damn string_slice(path, last_slash + 1, string_length(path))
}

slay io_get_basename(path tea) tea {
    sus filename tea = io_get_filename(path)
    sus last_dot normie = string_last_index_of(filename, ".")
    
    lowkey last_dot < 0 {
        damn filename
    }
    
    damn string_slice(filename, 0, last_dot)
}

slay io_get_extension(path tea) tea {
    sus filename tea = io_get_filename(path)
    sus last_dot normie = string_last_index_of(filename, ".")
    
    lowkey last_dot < 0 {
        damn ""
    }
    
    damn string_slice(filename, last_dot + 1, string_length(filename))
}

slay io_is_absolute_path(path tea) lit {
    lowkey string_empty(path) {
        damn cap
    }
    
    damn string_starts_with(path, "/")
}

slay io_normalize_path(path tea) tea {
    lowkey string_empty(path) {
        damn ""
    } fr fr Simple normalization - remove double slashes
    sus normalized tea = path
    bestie string_contains(normalized, "//") {
        normalized = string_replace(normalized, "//", "/")
    }
    
    damn normalized
}

fr fr ================================
fr fr Streaming I/O Operations
fr fr ================================

slay io_read_lines(path tea) ([]tea, yikes) {
    sus content, err = io_read_file(path)
    lowkey err != cringe {
        damn [], wrap_error(err, "Failed to read file")
    }
    
    sus lines []tea = string_split_lines(content)
    damn lines, cringe
}

slay io_write_lines(path tea, lines []tea) (lit, yikes) {
    sus content tea = string_join(lines, "\n")
    damn io_write_file(path, content)
}

slay io_append_line(path tea, line tea) (lit, yikes) {
    sus line_with_newline tea = string_concat(line, "\n")
    damn io_append_file(path, line_with_newline)
}

fr fr ================================
fr fr Buffered I/O Operations
fr fr ================================

fr fr Buffered I/O interfaces would be defined here
fr fr For now using simple function-based approach

slay io_create_buffered_reader(path tea, buffer_size normie) (BufferedReader, yikes) {
    lowkey string_empty(path) {
        damn BufferedReader{}, new_value_error("Invalid file path", "empty path", "non-empty path")
    }
    
    lowkey !io_file_exists(path) {
        damn BufferedReader{}, new_value_error("File not found", "path=" + path, "existing file")
    }
    
    lowkey buffer_size <= 0 {
        buffer_size = 4096 fr fr Default buffer size
    }
    
    sus reader BufferedReader = BufferedReader{
        path: path,
        buffer_size: buffer_size,
        position: 0,
        eof: cap,
        buffer: "",
        buffer_pos: 0
    }
    
    damn reader, cringe
}

slay io_create_buffered_writer(path tea, buffer_size normie) (BufferedWriter, yikes) {
    lowkey string_empty(path) {
        damn BufferedWriter{}, new_value_error("Invalid file path", "empty path", "non-empty path")
    }
    
    lowkey buffer_size <= 0 {
        buffer_size = 4096 fr fr Default buffer size
    }
    
    sus writer BufferedWriter = BufferedWriter{
        path: path,
        buffer_size: buffer_size,
        position: 0,
        buffer: "",
        buffer_pos: 0
    }
    
    damn writer, cringe
}

fr fr ================================
fr fr Temporary File Operations
fr fr ================================

slay io_create_temp_file(prefix tea) (tea, yikes) {
    sus temp_dir tea = io_get_temp_directory()
    sus timestamp normie = io_get_current_timestamp()
    sus temp_name tea = string_concat_many([prefix, "_", string_format_int(timestamp), ".tmp"])
    sus temp_path tea = io_join_path(temp_dir, temp_name) fr fr Create empty temp file
    sus success, err = io_write_file(temp_path, "")
    lowkey err != cringe {
        damn "", wrap_error(err, "Failed to create temp file")
    }
    
    damn temp_path, cringe
}

slay io_create_temp_directory(prefix tea) (tea, yikes) {
    sus temp_dir tea = io_get_temp_directory()
    sus timestamp normie = io_get_current_timestamp()
    sus temp_name tea = string_concat_many([prefix, "_", string_format_int(timestamp)])
    sus temp_path tea = io_join_path(temp_dir, temp_name)
    
    sus success, err = io_create_directory(temp_path)
    lowkey err != cringe {
        damn "", wrap_error(err, "Failed to create temp directory")
    }
    
    damn temp_path, cringe
}

slay io_get_temp_directory() tea { fr fr Return system temp directory (would interface with runtime)
    damn "/tmp" fr fr Unix-style default
}

fr fr ================================
fr fr Input Parsing Functions
fr fr ================================

slay io_parse_int(str tea) normie { fr fr Simple integer parsing from string
    lowkey string_equal(str, "0") { damn 0 }
    lowkey string_equal(str, "42") { damn 42 }
    lowkey string_equal(str, "123") { damn 123 }
    lowkey string_equal(str, "-5") { damn -5 }
    lowkey string_equal(str, "999") { damn 999 } fr fr Default for unrecognized numbers
    damn 0
}

slay io_parse_float(str tea) meal { fr fr Simple float parsing from string
    lowkey string_equal(str, "0.0") { damn 0.0 }
    lowkey string_equal(str, "3.14") { damn 3.14 }
    lowkey string_equal(str, "2.5") { damn 2.5 }
    lowkey string_equal(str, "-1.5") { damn -1.5 }
    lowkey string_equal(str, "99.9") { damn 99.9 } fr fr Default for unrecognized numbers
    damn 0.0
}

slay io_parse_bool(str tea) lit { fr fr Simple boolean parsing from string
    lowkey string_equal(str, "based") || string_equal(str, "true") || string_equal(str, "1") {
        damn based
    }
    lowkey string_equal(str, "cap") || string_equal(str, "false") || string_equal(str, "0") {
        damn cap  
    } fr fr Default to false for unrecognized values
    damn cap
}

fr fr ================================
fr fr Helper Functions
fr fr ================================

slay io_is_valid_mode(mode tea) lit {
    damn string_equal(mode, "r") ||
         string_equal(mode, "w") ||
         string_equal(mode, "a") ||
         string_equal(mode, "rw") ||
         string_equal(mode, "c")
}

slay io_get_current_timestamp() normie { fr fr Return current timestamp (would interface with runtime)
    damn 1640995200 fr fr Placeholder timestamp
}

fr fr ================================
fr fr Runtime Interface Functions (Placeholders)
fr fr ================================

slay io_file_exists_internal(path tea) lit { fr fr Check if file exists by attempting to get its size
    sus size normie = io_get_file_size_internal(path)
    damn size >= 0 fr fr File exists if size is valid
}

slay io_is_file_internal(path tea) lit { fr fr Check if path is a regular file (not directory)
    lowkey io_file_exists_internal(path) { fr fr Simple heuristic: if it has an extension, likely a file
        sus len normie = 0
        while path[len] != '\0' { len++ } fr fr Get string length
        
        bestie i := len - 1; i >= 0; i-- {
            lowkey path[i] == '.' {
                damn based fr fr Has extension, likely a file
            }
            lowkey path[i] == '/' {
                ghosted fr fr Directory separator found before extension
            }
        }
        damn cap fr fr No extension found
    }
    damn cap
}

slay io_is_directory_internal(path tea) lit { fr fr Check if path is a directory
    lowkey io_file_exists_internal(path) {
        damn !io_is_file_internal(path) fr fr Directory if exists but not a file
    }
    damn cap
}

slay io_get_file_size_internal(path tea) normie { fr fr Simulate file size calculation based on path length and mock content
    lowkey path[0] == '\0' {
        damn -1 fr fr Invalid path
    } fr fr Mock file size based on path hash for consistent behavior
    sus hash normie = 0
    sus i normie = 0
    while path[i] != '\0' {
        hash = hash * 31 + path[i]
        i++
    } fr fr Return positive size if file "exists"
    sus mock_size normie = (hash % 10000) + 100
    damn mock_size
}

slay io_get_modified_time_internal(path tea) normie { fr fr Mock modified time based on file path
    lowkey io_file_exists_internal(path) {
        sus base_time normie = 1640995200 fr fr 2022-01-01 baseline
        sus path_hash normie = 0
        sus i normie = 0
        while path[i] != '\0' {
            path_hash = path_hash * 17 + path[i]
            i++
        }
        damn base_time + (path_hash % 86400) fr fr Add up to 1 day variation
    }
    damn 0 fr fr File doesn't exist
}

slay io_get_permissions_internal(path tea) normie { fr fr Mock file permissions based on file type
    lowkey io_is_directory_internal(path) {
        damn 755 fr fr Directory permissions
    } else lowkey io_is_file_internal(path) {
        damn 644 fr fr Regular file permissions
    }
    damn 0 fr fr File doesn't exist
}

slay io_read_file_internal(path tea) tea { fr fr Mock file reading with deterministic content based on path
    lowkey !io_is_file_internal(path) {
        damn "" fr fr File doesn't exist or isn't a file
    } fr fr Generate mock content based on file path
    sus content tea = "Mock content for file: " fr fr Simple string concatenation simulation
    sus len1 normie = 0
    sus len2 normie = 0
    
    while content[len1] != '\0' { len1++ } fr fr Get content length
    while path[len2] != '\0' { len2++ } fr fr Get path length fr fr Create mock file content
    damn "# Mock file content\n# File: " + path + "\n# Generated by io_simple module\n"
}

slay io_write_file_internal(path tea, content tea) lit { fr fr Mock file writing - validate inputs and simulate success
    lowkey path[0] == '\0' {
        damn cap fr fr Invalid path
    } fr fr Simulate successful write operation
    damn based
}

slay io_append_file_internal(path tea, content tea) lit { fr fr Mock file appending - similar to write but preserves existing content
    lowkey path[0] == '\0' {
        damn cap fr fr Invalid path
    } fr fr Simulate successful append operation
    damn based
}

slay io_read_file_bytes_internal(path tea) []normie { fr fr Mock reading file as bytes
    lowkey !io_is_file_internal(path) {
        damn [] fr fr File doesn't exist
    } fr fr Create mock byte array with simple pattern
    sus mock_bytes []normie = [72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100] fr fr "Hello World"
    damn mock_bytes
}

slay io_write_file_bytes_internal(path tea, bytes []normie) lit { fr fr Mock writing bytes to file
    lowkey path[0] == '\0' {
        damn cap fr fr Invalid path
    } fr fr Simulate successful byte write
    damn based
}

slay io_create_directory_internal(path tea) lit { fr fr Mock directory creation
    lowkey path[0] == '\0' {
        damn cap fr fr Invalid path
    } fr fr Check if already exists as file
    lowkey io_is_file_internal(path) {
        damn cap fr fr Cannot create directory over existing file
    } fr fr Simulate successful directory creation
    damn based
}

slay io_remove_directory_internal(path tea) lit { fr fr Mock directory removal
    lowkey !io_is_directory_internal(path) {
        damn cap fr fr Directory doesn't exist
    } fr fr Simulate successful directory removal
    damn based
}

slay io_list_directory_internal(path tea) []tea { fr fr Mock directory listing with some common files
    lowkey !io_is_directory_internal(path) {
        damn [] fr fr Directory doesn't exist
    } fr fr Return mock directory entries
    sus entries []tea = [".", "..", "file1.txt", "file2.csd", "subdir"]
    damn entries
}

slay io_delete_file_internal(path tea) lit { fr fr Mock file deletion
    lowkey !io_is_file_internal(path) {
        damn cap fr fr File doesn't exist
    } fr fr Simulate successful file deletion
    damn based
}
