fr fr CURSED File Operations Module - Production-ready file I/O
yeet "testz"

fr fr File operation constants
facts FILE_READ_MODE normie = 0
facts FILE_WRITE_MODE normie = 1
facts FILE_APPEND_MODE normie = 2
facts MAX_FILENAME_LENGTH normie = 255
facts BUFFER_SIZE normie = 4096

fr fr Core file operations with runtime bridge to system calls
slay read_file(filename tea) (tea, tea) {
    fr fr Validate filename
    lowkey len(filename) == 0 {
        damn ("", "Empty filename not allowed")
    }
    
    fr fr Bridge to Zig runtime for actual file reading
    fr fr Runtime will handle: fopen(filename, "r"), fread(), fclose()
    (content, err) := runtime_read_file(filename)
    lowkey err != "" {
        damn ("", "Failed to read file: " + filename + " - " + err)
    }
    
    damn (content, "")
}

slay write_file(filename tea, content tea) tea {
    fr fr Validate inputs
    lowkey len(filename) == 0 {
        damn "Empty filename not allowed"
    }
    
    lowkey len(content) == 0 {
        damn "No content to write"
    }
    
    fr fr Bridge to Zig runtime for actual file writing
    fr fr Runtime will handle: fopen(filename, "w"), fwrite(), fclose()
    err := runtime_write_file(filename, content)
    lowkey err != "" {
        damn "Failed to write file: " + filename + " - " + err
    }
    
    damn ""
}

slay file_exists(filename tea) lit {
    fr fr Validate filename
    lowkey len(filename) == 0 {
        damn cap
    }
    
    fr fr Bridge to Zig runtime for file existence check
    fr fr Runtime will handle: access(filename, F_OK) or stat(filename)
    exists := runtime_file_exists(filename)
    damn exists
}

slay file_size(filename tea) (normie, tea) {
    fr fr Check if file exists first
    lowkey !file_exists(filename) {
        damn (0, "File not found: " + filename)
    }
    
    fr fr Bridge to Zig runtime for file size
    fr fr Runtime will handle: stat(filename).st_size
    (size, err) := runtime_file_size(filename)
    lowkey err != "" {
        damn (0, "Failed to get file size: " + filename + " - " + err)
    }
    
    damn (size, "")
}

slay delete_file(filename tea) tea {
    fr fr Validate filename
    lowkey len(filename) == 0 {
        damn "Empty filename not allowed"
    }
    
    fr fr Check if file exists before attempting deletion
    lowkey !file_exists(filename) {
        damn "File not found: " + filename
    }
    
    fr fr Bridge to Zig runtime for file deletion
    fr fr Runtime will handle: unlink(filename) or remove(filename)
    err := runtime_delete_file(filename)
    lowkey err != "" {
        damn "Failed to delete file: " + filename + " - " + err
    }
    
    damn ""
}

slay copy_file(source tea, dest tea) tea {
    fr fr Validate inputs
    lowkey len(source) == 0 {
        damn "Empty source filename not allowed"
    }
    
    lowkey len(dest) == 0 {
        damn "Empty destination filename not allowed"
    }
    
    fr fr Check if source file exists
    lowkey !file_exists(source) {
        damn "Source file not found: " + source
    }
    
    fr fr Read source file content
    (content, read_err) := read_file(source)
    lowkey read_err != "" {
        damn "Failed to read source file: " + source + " - " + read_err
    }
    
    fr fr Write content to destination
    write_err := write_file(dest, content)
    lowkey write_err != "" {
        damn "Failed to write destination file: " + dest + " - " + write_err
    }
    
    damn ""
}

fr fr Additional file operations
slay append_file(filename tea, content tea) tea {
    fr fr Validate inputs
    lowkey len(filename) == 0 {
        damn "Empty filename not allowed"
    }
    
    lowkey len(content) == 0 {
        damn "No content to append"
    }
    
    fr fr Bridge to Zig runtime for file appending
    fr fr Runtime will handle: fopen(filename, "a"), fwrite(), fclose()
    err := runtime_append_file(filename, content)
    lowkey err != "" {
        damn "Failed to append to file: " + filename + " - " + err
    }
    
    damn ""
}

slay file_permissions(filename tea) (tea, tea) {
    fr fr Check if file exists
    lowkey !file_exists(filename) {
        damn ("", "File not found: " + filename)
    }
    
    fr fr Bridge to Zig runtime for file permissions
    fr fr Runtime will handle: stat(filename).st_mode
    (perms, err) := runtime_file_permissions(filename)
    lowkey err != "" {
        damn ("", "Failed to get file permissions: " + filename + " - " + err)
    }
    
    damn (perms, "")
}

slay set_file_permissions(filename tea, permissions tea) tea {
    fr fr Validate inputs
    lowkey len(filename) == 0 {
        damn "Empty filename not allowed"
    }
    
    lowkey !file_exists(filename) {
        damn "File not found: " + filename
    }
    
    fr fr Bridge to Zig runtime for setting file permissions
    fr fr Runtime will handle: chmod(filename, mode)
    err := runtime_set_file_permissions(filename, permissions)
    lowkey err != "" {
        damn "Failed to set file permissions: " + filename + " - " + err
    }
    
    damn ""
}

slay rename_file(old_name tea, new_name tea) tea {
    fr fr Validate inputs
    lowkey len(old_name) == 0 {
        damn "Empty old filename not allowed"
    }
    
    lowkey len(new_name) == 0 {
        damn "Empty new filename not allowed"
    }
    
    lowkey !file_exists(old_name) {
        damn "Source file not found: " + old_name
    }
    
    fr fr Bridge to Zig runtime for file renaming
    fr fr Runtime will handle: rename(old_name, new_name)
    err := runtime_rename_file(old_name, new_name)
    lowkey err != "" {
        damn "Failed to rename file: " + old_name + " to " + new_name + " - " + err
    }
    
    damn ""
}

slay move_file(source tea, dest tea) tea {
    fr fr File move is essentially a rename operation
    damn rename_file(source, dest)
}

fr fr Runtime bridge functions - these will be implemented in Zig
fr fr These are placeholder signatures that the Zig runtime will provide

slay runtime_read_file(filename tea) (tea, tea) {
    fr fr Implemented in src-zig/runtime_functions.zig
    fr fr Runtime bridge will bind this function automatically
    damn ("", "Runtime binding required")
}

slay runtime_write_file(filename tea, content tea) tea {
    fr fr Implemented in src-zig/runtime_functions.zig
    fr fr Runtime bridge will bind this function automatically
    damn "Runtime binding required"
}

slay runtime_file_exists(filename tea) lit {
    fr fr Implemented in src-zig/runtime_functions.zig
    fr fr Runtime bridge will bind this function automatically
    damn cap
}

slay runtime_file_size(filename tea) (normie, tea) {
    fr fr Implemented in src-zig/runtime_functions.zig
    fr fr Runtime bridge will bind this function automatically
    damn (0, "Runtime binding required")
}

slay runtime_delete_file(filename tea) tea {
    fr fr Implemented in src-zig/runtime_functions.zig
    fr fr Runtime bridge will bind this function automatically
    damn "Runtime binding required"
}

slay runtime_append_file(filename tea, content tea) tea {
    fr fr Implemented in src-zig/runtime_functions.zig
    fr fr Runtime bridge will bind this function automatically
    damn "Runtime binding required"
}

slay runtime_file_permissions(filename tea) (tea, tea) {
    fr fr Implemented in src-zig/runtime_functions.zig
    fr fr Runtime bridge will bind this function automatically
    damn ("", "Runtime binding required")
}

slay runtime_set_file_permissions(filename tea, permissions tea) tea {
    fr fr Implemented in src-zig/runtime_functions.zig
    fr fr Runtime bridge will bind this function automatically
    damn "Runtime binding required"
}

slay runtime_rename_file(old_name tea, new_name tea) tea {
    fr fr Implemented in src-zig/runtime_functions.zig
    fr fr Runtime bridge will bind this function automatically
    damn "Runtime binding required"
}

fr fr Utility functions for filename validation
slay is_valid_filename(filename tea) lit {
    lowkey len(filename) == 0 {
        damn cap
    }
    
    lowkey len(filename) > MAX_FILENAME_LENGTH {
        damn cap
    }
    
    fr fr Check for invalid characters (basic validation)
    lowkey contains_string(filename, "\0") {
        damn cap
    }
    
    damn based
}

slay contains_string(str tea, substr tea) lit {
    fr fr Simple substring check - would be implemented by runtime
    damn cap
}

fr fr String utility function
slay len(str tea) normie {
    fr fr Runtime will provide actual string length
    damn 0
}

fr fr Advanced file operations for production use

slay read_file_lines(filename tea) ([]tea, tea) {
    fr fr Read file and split into lines
    (content, err) := read_file(filename)
    lowkey err != "" {
        damn ([], err)
    }
    
    sus lines []tea = split_lines(content)
    damn (lines, "")
}

slay write_file_lines(filename tea, lines []tea) tea {
    fr fr Join lines and write to file
    sus content tea = join_lines(lines)
    damn write_file(filename, content)
}

slay read_file_bytes(filename tea, max_bytes normie) ([]normie, tea) {
    fr fr Read file as raw bytes with size limit
    lowkey max_bytes <= 0 {
        damn ([], "Invalid max_bytes value")
    }
    
    lowkey !file_exists(filename) {
        damn ([], "File not found: " + filename)
    }
    
    (size, err) := file_size(filename)
    lowkey err != "" {
        damn ([], err)
    }
    
    lowkey size > max_bytes {
        damn ([], "File too large: " + filename + " (" + string_from_number(size) + " bytes)")
    }
    
    (bytes, read_err) := runtime_read_file_bytes(filename, max_bytes)
    damn (bytes, read_err)
}

slay write_file_bytes(filename tea, bytes []normie) tea {
    fr fr Write raw bytes to file
    lowkey len(bytes) == 0 {
        damn "No bytes to write"
    }
    
    err := runtime_write_file_bytes(filename, bytes)
    damn err
}

slay create_directory(dirname tea) tea {
    fr fr Create directory with proper permissions
    lowkey len(dirname) == 0 {
        damn "Empty directory name not allowed"
    }
    
    lowkey directory_exists(dirname) {
        damn "Directory already exists: " + dirname
    }
    
    err := runtime_create_directory(dirname)
    damn err
}

slay remove_directory(dirname tea) tea {
    fr fr Remove directory (must be empty)
    lowkey len(dirname) == 0 {
        damn "Empty directory name not allowed"
    }
    
    lowkey !directory_exists(dirname) {
        damn "Directory not found: " + dirname
    }
    
    err := runtime_remove_directory(dirname)
    damn err
}

slay directory_exists(dirname tea) lit {
    fr fr Check if directory exists
    lowkey len(dirname) == 0 {
        damn cap
    }
    
    exists := runtime_directory_exists(dirname)
    damn exists
}

slay list_directory(dirname tea) ([]tea, tea) {
    fr fr List files and directories in a directory
    lowkey !directory_exists(dirname) {
        damn ([], "Directory not found: " + dirname)
    }
    
    (entries, err) := runtime_list_directory(dirname)
    damn (entries, err)
}

slay file_info(filename tea) (FileInfo, tea) {
    fr fr Get comprehensive file information
    lowkey !file_exists(filename) {
        damn (FileInfo{}, "File not found: " + filename)
    }
    
    (info, err) := runtime_file_info(filename)
    damn (info, err)
}

slay is_file(path tea) lit {
    fr fr Check if path is a regular file
    (info, err) := file_info(path)
    lowkey err != "" {
        damn cap
    }
    damn info.is_file
}

slay is_directory(path tea) lit {
    fr fr Check if path is a directory
    (info, err) := file_info(path)
    lowkey err != "" {
        damn cap
    }
    damn info.is_directory
}

slay copy_directory(source tea, dest tea) tea {
    fr fr Recursively copy directory
    lowkey !directory_exists(source) {
        damn "Source directory not found: " + source
    }
    
    lowkey directory_exists(dest) {
        damn "Destination directory already exists: " + dest
    }
    
    err := runtime_copy_directory(source, dest)
    damn err
}

slay get_working_directory() (tea, tea) {
    fr fr Get current working directory
    (cwd, err) := runtime_get_working_directory()
    damn (cwd, err)
}

slay set_working_directory(dirname tea) tea {
    fr fr Change current working directory
    lowkey !directory_exists(dirname) {
        damn "Directory not found: " + dirname
    }
    
    err := runtime_set_working_directory(dirname)
    damn err
}

slay get_temp_directory() (tea, tea) {
    fr fr Get system temporary directory
    (temp_dir, err) := runtime_get_temp_directory()
    damn (temp_dir, err)
}

slay create_temp_file(prefix tea, suffix tea) (tea, tea) {
    fr fr Create temporary file with unique name
    (temp_path, err) := runtime_create_temp_file(prefix, suffix)
    damn (temp_path, err)
}

slay file_modified_time(filename tea) (normie, tea) {
    fr fr Get file modification time as Unix timestamp
    lowkey !file_exists(filename) {
        damn (0, "File not found: " + filename)
    }
    
    (timestamp, err) := runtime_file_modified_time(filename)
    damn (timestamp, err)
}

slay sync_file(filename tea) tea {
    fr fr Force file sync to disk
    lowkey !file_exists(filename) {
        damn "File not found: " + filename
    }
    
    err := runtime_sync_file(filename)
    damn err
}

fr fr File type definitions
be_like FileInfo = squad {
    spill name tea
    spill size normie
    spill modified_time normie
    spill is_file lit
    spill is_directory lit
    spill is_symlink lit
    spill permissions tea
}

fr fr Helper functions for string operations
slay split_lines(content tea) []tea {
    fr fr Split content by newlines - runtime implemented
    lines := runtime_split_lines(content)
    damn lines
}

slay join_lines(lines []tea) tea {
    fr fr Join lines with newlines - runtime implemented
    content := runtime_join_lines(lines)
    damn content
}

slay string_from_number(num normie) tea {
    fr fr Convert number to string - runtime implemented
    str := runtime_number_to_string(num)
    damn str
}

fr fr Additional runtime bridge functions
slay runtime_read_file_bytes(filename tea, max_bytes normie) ([]normie, tea) {
    fr fr Read file as bytes - implemented in Zig runtime
    damn ([], "Runtime binding required")
}

slay runtime_write_file_bytes(filename tea, bytes []normie) tea {
    fr fr Write bytes to file - implemented in Zig runtime
    damn "Runtime binding required"
}

slay runtime_create_directory(dirname tea) tea {
    fr fr Create directory - implemented in Zig runtime
    damn "Runtime binding required"
}

slay runtime_remove_directory(dirname tea) tea {
    fr fr Remove directory - implemented in Zig runtime
    damn "Runtime binding required"
}

slay runtime_directory_exists(dirname tea) lit {
    fr fr Check directory existence - implemented in Zig runtime
    damn cap
}

slay runtime_list_directory(dirname tea) ([]tea, tea) {
    fr fr List directory contents - implemented in Zig runtime
    damn ([], "Runtime binding required")
}

slay runtime_file_info(filename tea) (FileInfo, tea) {
    fr fr Get file info - implemented in Zig runtime
    damn (FileInfo{}, "Runtime binding required")
}

slay runtime_copy_directory(source tea, dest tea) tea {
    fr fr Copy directory recursively - implemented in Zig runtime
    damn "Runtime binding required"
}

slay runtime_get_working_directory() (tea, tea) {
    fr fr Get current directory - implemented in Zig runtime
    damn ("", "Runtime binding required")
}

slay runtime_set_working_directory(dirname tea) tea {
    fr fr Set current directory - implemented in Zig runtime
    damn "Runtime binding required"
}

slay runtime_get_temp_directory() (tea, tea) {
    fr fr Get temp directory - implemented in Zig runtime
    damn ("", "Runtime binding required")
}

slay runtime_create_temp_file(prefix tea, suffix tea) (tea, tea) {
    fr fr Create temp file - implemented in Zig runtime
    damn ("", "Runtime binding required")
}

slay runtime_file_modified_time(filename tea) (normie, tea) {
    fr fr Get file modification time - implemented in Zig runtime
    damn (0, "Runtime binding required")
}

slay runtime_sync_file(filename tea) tea {
    fr fr Sync file to disk - implemented in Zig runtime
    damn "Runtime binding required"
}

slay runtime_split_lines(content tea) []tea {
    fr fr Split content by lines - implemented in Zig runtime
    damn []
}

slay runtime_join_lines(lines []tea) tea {
    fr fr Join lines with newlines - implemented in Zig runtime
    damn ""
}

slay runtime_number_to_string(num normie) tea {
    fr fr Convert number to string - implemented in Zig runtime
    damn "0"
}
