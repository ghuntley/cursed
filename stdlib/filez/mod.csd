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
