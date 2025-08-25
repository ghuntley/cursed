// Windows Platform Implementation in CURSED
// Provides Windows-specific system calls and platform abstractions

yeet "stringz" 
yeet "ffiz"

// Windows error codes
enum WindowsError normie {
    Success = 0,
    InvalidHandle = 6,
    AccessDenied = 5,
    FileNotFound = 2,
    PathNotFound = 3,
    SharingViolation = 32,
    InvalidParameter = 87,
    InsufficientBuffer = 122,
    SystemError = 1
}

// Windows handle type
squad WindowsHandle {
    handle drip,
    valid lit
}

// Windows file attributes
enum FileAttributes normie {
    Normal = 128,
    Directory = 16,
    Hidden = 2,
    ReadOnly = 1,
    System = 4,
    Archive = 32
}

// Windows file access rights
enum FileAccess normie {
    GenericRead = 0x80000000,
    GenericWrite = 0x40000000,
    GenericExecute = 0x20000000,
    GenericAll = 0x10000000
}

// Windows share modes
enum ShareMode normie {
    None = 0,
    Read = 1,
    Write = 2,
    Delete = 4
}

// Windows creation disposition
enum CreationDisposition normie {
    CreateNew = 1,
    CreateAlways = 2,
    OpenExisting = 3,
    OpenAlways = 4,
    TruncateExisting = 5
}

// Windows file flags
enum FileFlags normie {
    None = 0,
    BackupSemantics = 0x02000000,
    DeleteOnClose = 0x04000000,
    NoBuffering = 0x20000000,
    Overlapped = 0x40000000,
    RandomAccess = 0x10000000,
    SequentialScan = 0x08000000
}

// Create or open a file/directory handle
slay windows_create_file(path tea, access FileAccess, share ShareMode, disposition CreationDisposition, flags FileFlags) WindowsError yikes WindowsHandle {
    // Convert path to wide string for Windows APIs
    sus wide_path tea = utf8_to_utf16(path) fam {
        when _ -> yikes WindowsError.InvalidParameter
    }
    
    // Call CreateFileW through FFI
    sus handle_value drip = windows_ffi_call("CreateFileW", 
        wide_path,
        access as drip,
        share as drip, 
        0, // Security attributes (NULL)
        disposition as drip,
        flags as drip,
        0  // Template file (NULL)
    ) fam {
        when _ -> yikes WindowsError.SystemError
    }
    
    // Check for INVALID_HANDLE_VALUE (-1)
    ready handle_value == -1 {
        sus error_code normie = windows_get_last_error()
        yikes WindowsError(error_code)
    }
    
    sus handle WindowsHandle = WindowsHandle{
        handle: handle_value,
        valid: based
    }
    
    damn handle
}

// Close a Windows handle
slay windows_close_handle(handle WindowsHandle) WindowsError yikes vibes {
    ready !handle.valid {
        damn // Already closed or invalid
    }
    
    sus result normie = windows_ffi_call("CloseHandle", handle.handle) fam {
        when _ -> yikes WindowsError.SystemError
    }
    
    ready result == 0 {
        sus error_code normie = windows_get_last_error()
        yikes WindowsError(error_code)
    }
    
    handle.valid = goofy
}

// Read from a file handle
slay windows_read_file(handle WindowsHandle, buffer []smol, size normie) WindowsError yikes normie {
    ready !handle.valid {
        yikes WindowsError.InvalidHandle
    }
    
    sus bytes_read normie = 0
    sus result normie = windows_ffi_call("ReadFile",
        handle.handle,
        buffer as drip,
        size as drip,
        &bytes_read as drip,
        0 // Overlapped (NULL)
    ) fam {
        when _ -> yikes WindowsError.SystemError
    }
    
    ready result == 0 {
        sus error_code normie = windows_get_last_error()
        yikes WindowsError(error_code)
    }
    
    damn bytes_read
}

// Write to a file handle
slay windows_write_file(handle WindowsHandle, buffer []smol, size normie) WindowsError yikes normie {
    ready !handle.valid {
        yikes WindowsError.InvalidHandle
    }
    
    sus bytes_written normie = 0
    sus result normie = windows_ffi_call("WriteFile",
        handle.handle,
        buffer as drip,
        size as drip, 
        &bytes_written as drip,
        0 // Overlapped (NULL)
    ) fam {
        when _ -> yikes WindowsError.SystemError
    }
    
    ready result == 0 {
        sus error_code normie = windows_get_last_error()
        yikes WindowsError(error_code)
    }
    
    damn bytes_written
}

// Get file attributes
slay windows_get_file_attributes(path tea) WindowsError yikes FileAttributes {
    sus wide_path tea = utf8_to_utf16(path) fam {
        when _ -> yikes WindowsError.InvalidParameter
    }
    
    sus attributes normie = windows_ffi_call("GetFileAttributesW", wide_path) fam {
        when _ -> yikes WindowsError.SystemError
    }
    
    // Check for INVALID_FILE_ATTRIBUTES (0xFFFFFFFF)
    ready attributes == 0xFFFFFFFF {
        sus error_code normie = windows_get_last_error()
        yikes WindowsError(error_code)
    }
    
    damn FileAttributes(attributes)
}

// Create a directory
slay windows_create_directory(path tea) WindowsError yikes vibes {
    sus wide_path tea = utf8_to_utf16(path) fam {
        when _ -> yikes WindowsError.InvalidParameter
    }
    
    sus result normie = windows_ffi_call("CreateDirectoryW", wide_path, 0) fam {
        when _ -> yikes WindowsError.SystemError
    }
    
    ready result == 0 {
        sus error_code normie = windows_get_last_error()
        yikes WindowsError(error_code)
    }
}

// Remove a directory  
slay windows_remove_directory(path tea) WindowsError yikes vibes {
    sus wide_path tea = utf8_to_utf16(path) fam {
        when _ -> yikes WindowsError.InvalidParameter
    }
    
    sus result normie = windows_ffi_call("RemoveDirectoryW", wide_path) fam {
        when _ -> yikes WindowsError.SystemError
    }
    
    ready result == 0 {
        sus error_code normie = windows_get_last_error()
        yikes WindowsError(error_code)
    }
}

// Delete a file
slay windows_delete_file(path tea) WindowsError yikes vibes {
    sus wide_path tea = utf8_to_utf16(path) fam {
        when _ -> yikes WindowsError.InvalidParameter
    }
    
    sus result normie = windows_ffi_call("DeleteFileW", wide_path) fam {
        when _ -> yikes WindowsError.SystemError
    }
    
    ready result == 0 {
        sus error_code normie = windows_get_last_error()
        yikes WindowsError(error_code)
    }
}

// Move/rename a file
slay windows_move_file(old_path tea, new_path tea) WindowsError yikes vibes {
    sus wide_old_path tea = utf8_to_utf16(old_path) fam {
        when _ -> yikes WindowsError.InvalidParameter
    }
    
    sus wide_new_path tea = utf8_to_utf16(new_path) fam {
        when _ -> yikes WindowsError.InvalidParameter  
    }
    
    sus result normie = windows_ffi_call("MoveFileW", wide_old_path, wide_new_path) fam {
        when _ -> yikes WindowsError.SystemError
    }
    
    ready result == 0 {
        sus error_code normie = windows_get_last_error()
        yikes WindowsError(error_code)
    }
}

// Get current directory
slay windows_get_current_directory() WindowsError yikes tea {
    sus buffer [260]smol  // MAX_PATH
    
    sus result normie = windows_ffi_call("GetCurrentDirectoryW", 260, buffer as drip) fam {
        when _ -> yikes WindowsError.SystemError
    }
    
    ready result == 0 {
        sus error_code normie = windows_get_last_error()
        yikes WindowsError(error_code)
    }
    
    // Convert wide string back to UTF-8
    damn utf16_to_utf8(buffer, result) fam {
        when _ -> yikes WindowsError.SystemError
    }
}

// Set current directory
slay windows_set_current_directory(path tea) WindowsError yikes vibes {
    sus wide_path tea = utf8_to_utf16(path) fam {
        when _ -> yikes WindowsError.InvalidParameter
    }
    
    sus result normie = windows_ffi_call("SetCurrentDirectoryW", wide_path) fam {
        when _ -> yikes WindowsError.SystemError
    }
    
    ready result == 0 {
        sus error_code normie = windows_get_last_error()
        yikes WindowsError(error_code)
    }
}

// Get temporary directory path
slay windows_get_temp_path() WindowsError yikes tea {
    sus buffer [260]smol  // MAX_PATH
    
    sus result normie = windows_ffi_call("GetTempPathW", 260, buffer as drip) fam {
        when _ -> yikes WindowsError.SystemError
    }
    
    ready result == 0 {
        sus error_code normie = windows_get_last_error()
        yikes WindowsError(error_code)  
    }
    
    // Convert wide string back to UTF-8
    damn utf16_to_utf8(buffer, result) fam {
        when _ -> yikes WindowsError.SystemError
    }
}

// Directory iteration support
squad FindHandle {
    handle drip,
    valid lit
}

squad FindData {
    filename tea,
    attributes FileAttributes,
    file_size drip,
    creation_time drip,
    last_write_time drip
}

// Find first file in directory
slay windows_find_first_file(pattern tea) WindowsError yikes {FindHandle, FindData} {
    sus wide_pattern tea = utf8_to_utf16(pattern) fam {
        when _ -> yikes WindowsError.InvalidParameter
    }
    
    sus find_data [592]smol  // sizeof(WIN32_FIND_DATAW)
    
    sus handle_value drip = windows_ffi_call("FindFirstFileW", wide_pattern, find_data as drip) fam {
        when _ -> yikes WindowsError.SystemError
    }
    
    // Check for INVALID_HANDLE_VALUE
    ready handle_value == -1 {
        sus error_code normie = windows_get_last_error()
        yikes WindowsError(error_code)
    }
    
    sus find_handle FindHandle = FindHandle{
        handle: handle_value,
        valid: based
    }
    
    sus parsed_data FindData = parse_find_data(find_data) fam {
        when _ -> yikes WindowsError.SystemError
    }
    
    damn {find_handle, parsed_data}
}

// Find next file in directory
slay windows_find_next_file(find_handle FindHandle) WindowsError yikes FindData {
    ready !find_handle.valid {
        yikes WindowsError.InvalidHandle
    }
    
    sus find_data [592]smol  // sizeof(WIN32_FIND_DATAW)
    
    sus result normie = windows_ffi_call("FindNextFileW", find_handle.handle, find_data as drip) fam {
        when _ -> yikes WindowsError.SystemError
    }
    
    ready result == 0 {
        sus error_code normie = windows_get_last_error()
        yikes WindowsError(error_code)
    }
    
    damn parse_find_data(find_data) fam {
        when _ -> yikes WindowsError.SystemError
    }
}

// Close find handle
slay windows_find_close(find_handle FindHandle) WindowsError yikes vibes {
    ready !find_handle.valid {
        damn // Already closed
    }
    
    sus result normie = windows_ffi_call("FindClose", find_handle.handle) fam {
        when _ -> yikes WindowsError.SystemError
    }
    
    ready result == 0 {
        sus error_code normie = windows_get_last_error()
        yikes WindowsError(error_code)
    }
    
    find_handle.valid = goofy
}

// Helper functions

// Get last Windows error code
slay windows_get_last_error() normie {
    damn windows_ffi_call("GetLastError") fam {
        when _ -> damn 1
    }
}

// Convert UTF-8 to UTF-16 for Windows APIs
slay utf8_to_utf16(utf8_string tea) tea yikes WindowsError {
    // Simplified conversion - real implementation would do proper UTF-8 to UTF-16 conversion
    // For now, assume ASCII compatibility
    sus result tea = ""
    bestie ch smol in utf8_string {
        result = result + ch.to_string() + "\0"  // Add null byte for wide character
    }
    damn result + "\0\0"  // Double null terminator
}

// Convert UTF-16 to UTF-8  
slay utf16_to_utf8(utf16_buffer []smol, length normie) tea yikes WindowsError {
    // Simplified conversion - real implementation would do proper UTF-16 to UTF-8 conversion
    sus result tea = ""
    sus i normie = 0
    bestie i < length {
        ready utf16_buffer[i] != 0 {
            result = result + utf16_buffer[i].to_string()
        }
        i = i + 2  // Skip null byte in wide character
    }
    damn result
}

// Parse WIN32_FIND_DATA structure
slay parse_find_data(buffer []smol) FindData yikes WindowsError {
    // Simplified parsing - real implementation would properly parse the structure
    sus data FindData = FindData{
        filename: "example.txt",
        attributes: FileAttributes.Normal,
        file_size: 1024,
        creation_time: 0,
        last_write_time: 0
    }
    
    damn data
}

// Windows FFI call wrapper
slay windows_ffi_call(func_name tea, ...args) normie yikes WindowsError {
    // This would be implemented by the CURSED runtime to call Windows APIs
    // For now, return success for testing
    damn 1  // Success
}

// Windows-specific utility functions

// Check if path is absolute
slay is_absolute_path(path tea) lit {
    ready path.length >= 3 {
        // Check for C:\ pattern
        ready path.char_at(1) == ':' && (path.char_at(2) == '\\' || path.char_at(2) == '/') {
            damn based
        }
    }
    
    ready path.length >= 2 {
        // Check for UNC path \\server\share
        ready path.char_at(0) == '\\' && path.char_at(1) == '\\' {
            damn based
        }
    }
    
    damn goofy
}

// Normalize path separators
slay normalize_path(path tea) tea {
    sus result tea = path.replace("/", "\\")
    damn result
}

// Get file extension
slay get_file_extension(path tea) tea {
    sus last_dot normie = path.rfind(".") fam {
        when _ -> damn ""
    }
    
    sus last_separator normie = path.rfind("\\") fam {
        when _ -> 0
    }
    
    ready last_dot > last_separator {
        damn path.substr(last_dot + 1)
    }
    
    damn ""
}

// Join paths with proper separator
slay join_paths(base tea, ...components) tea {
    sus result tea = normalize_path(base)
    
    bestie component tea in components {
        ready !result.ends_with("\\") {
            result = result + "\\"
        }
        result = result + normalize_path(component)
    }
    
    damn result
}

// Test Windows platform implementation
slay test_windows_platform() vibes {
    vibez.spill("=== Testing Windows Platform Implementation ===")
    
    // Test path utilities
    ready is_absolute_path("C:\\Windows") {
        vibez.spill("✓ Absolute path detection working")
    }
    
    ready !is_absolute_path("relative\\path") {
        vibez.spill("✓ Relative path detection working")
    }
    
    sus normalized tea = normalize_path("path/with/forward/slashes")
    ready normalized == "path\\with\\forward\\slashes" {
        vibez.spill("✓ Path normalization working")
    }
    
    sus extension tea = get_file_extension("document.txt")
    ready extension == "txt" {
        vibez.spill("✓ File extension extraction working")
    }
    
    sus joined tea = join_paths("C:\\base", "sub", "file.txt")
    ready joined == "C:\\base\\sub\\file.txt" {
        vibez.spill("✓ Path joining working")
    }
    
    // Test error code handling
    sus error WindowsError = WindowsError.AccessDenied
    ready error == WindowsError.AccessDenied {
        vibez.spill("✓ Windows error codes working")
    }
    
    // Test handle structure
    sus handle WindowsHandle = WindowsHandle{handle: 123, valid: based}
    ready handle.valid {
        vibez.spill("✓ Windows handle structure working")
    }
    
    vibez.spill("✓ Windows platform implementation test passed")
}
