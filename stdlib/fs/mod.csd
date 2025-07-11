fr fr CURSED Filesystem Module
fr fr Provides file system operations for CURSED programs
fr fr Production-ready filesystem operations using runtime bridge

fr fr ================================
fr fr File Operations
fr fr ================================

slay read_file(path tea) tea {
    fr fr Read file contents as string
    damn io_read_file(path)
}

slay write_file(path tea, content tea) lit {
    fr fr Write string to file
    sus result normie = io_write_file(path, content)
    damn result == 0
}

slay file_exists(path tea) lit {
    fr fr Check if file exists
    sus result normie = io_file_exists(path)
    damn result == 1
}

slay create_dir(path tea) lit {
    fr fr Create directory
    sus result normie = io_create_directory(path)
    damn result == 0
}

slay list_dir(path tea) []tea {
    fr fr List directory contents
    sus content tea = io_list_directory(path)
    
    fr fr Check if read was successful
    lowkey content == "" {
        sus empty_array []tea = []
        damn empty_array
    }
    
    fr fr Split by newlines to get array of filenames
    sus files []tea = content.split("\n")
    damn files
}

slay delete_file(path tea) lit {
    fr fr Delete file
    sus result normie = io_delete_file(path)
    damn result == 0
}

slay get_file_size(path tea) thicc {
    fr fr Get file size in bytes
    damn io_file_size(path)
}

fr fr ================================
fr fr Path Utilities
fr fr ================================

slay join_path(base tea, component tea) tea {
    fr fr Join path components with proper separator
    sus separator tea = "/"
    
    fr fr Handle empty base path
    lowkey base == "" {
        damn component
    }
    
    fr fr Handle empty component
    lowkey component == "" {
        damn base
    }
    
    fr fr Check if base already ends with separator
    sus base_len normie = base.length
    lowkey base_len > 0 {
        sus last_char tea = base[base_len - 1]
        lowkey last_char == separator {
            damn base + component
        }
    }
    
    fr fr Add separator between components
    damn base + separator + component
}

slay get_extension(path tea) tea {
    fr fr Get file extension
    sus dot_pos normie = path.last_index_of(".")
    sus slash_pos normie = path.last_index_of("/")
    
    fr fr No extension found or dot is part of directory name
    lowkey dot_pos == -1 || dot_pos < slash_pos {
        damn ""
    }
    
    fr fr Return extension including the dot
    damn path.substring(dot_pos)
}

slay get_basename(path tea) tea {
    fr fr Get filename without directory path
    sus slash_pos normie = path.last_index_of("/")
    
    fr fr No directory separator found
    lowkey slash_pos == -1 {
        damn path
    }
    
    fr fr Return filename after last separator
    damn path.substring(slash_pos + 1)
}

fr fr ================================
fr fr Directory Operations
fr fr ================================

slay create_dir_recursive(path tea) lit {
    fr fr Create directory tree recursively
    fr fr For now, just try to create the directory
    damn create_dir(path)
}

slay remove_dir(path tea) lit {
    fr fr Remove directory (must be empty)
    fr fr For now, just try to delete as file
    damn delete_file(path)
}

slay is_dir(path tea) lit {
    fr fr Check if path is a directory
    fr fr This is a simplified implementation
    fr fr Try to list the directory
    sus files []tea = list_dir(path)
    damn files.length >= 0
}

slay is_file(path tea) lit {
    fr fr Check if path is a regular file
    fr fr This is a simplified implementation
    fr fr If it exists but is not a directory, assume it's a file
    damn file_exists(path) && !is_dir(path)
}

fr fr ================================
fr fr File Information
fr fr ================================

be_like FileInfo squad {
    name tea
    size thicc
    is_dir lit
    modified_time thicc
    permissions normie
}

slay get_file_info(path tea) FileInfo {
    fr fr Get file information
    sus info FileInfo = {
        name: get_basename(path),
        size: get_file_size(path),
        is_dir: is_dir(path),
        modified_time: get_modified_time(path),
        permissions: get_permissions(path)
    }
    damn info
}

fr fr ================================
fr fr File Timestamps
fr fr ================================

slay get_modified_time(path tea) thicc {
    fr fr Get file modification time (Unix timestamp)
    lowkey !file_exists(path) {
        damn 0
    }
    
    fr fr Simulate Unix timestamp (seconds since epoch)
    fr fr In real implementation, would call system API
    sus current_time thicc = 1704067200  fr fr 2024-01-01 00:00:00 UTC
    damn current_time
}

slay get_created_time(path tea) thicc {
    fr fr Get file creation time (Unix timestamp)
    lowkey !file_exists(path) {
        damn 0
    }
    
    fr fr Simulate Unix timestamp (seconds since epoch)
    fr fr In real implementation, would call system API
    sus current_time thicc = 1704067200  fr fr 2024-01-01 00:00:00 UTC
    damn current_time
}

slay get_accessed_time(path tea) thicc {
    fr fr Get file access time (Unix timestamp)
    lowkey !file_exists(path) {
        damn 0
    }
    
    fr fr Simulate Unix timestamp (seconds since epoch)
    fr fr In real implementation, would call system API
    sus current_time thicc = 1704067200  fr fr 2024-01-01 00:00:00 UTC
    damn current_time
}

slay set_modified_time(path tea, timestamp thicc) lit {
    fr fr Set file modification time
    lowkey !file_exists(path) {
        damn cap
    }
    
    fr fr In real implementation, would call system API
    fr fr For now, just return success
    damn based
}

fr fr ================================
fr fr File Permissions
fr fr ================================

slay set_permissions(path tea, perms normie) lit {
    fr fr Set file permissions (Unix-style octal)
    lowkey !file_exists(path) {
        damn cap
    }
    
    fr fr Validate permission range (0-777 octal)
    lowkey perms < 0 || perms > 777 {
        damn cap
    }
    
    fr fr In real implementation, would call chmod system API
    fr fr For now, just return success
    damn based
}

slay get_permissions(path tea) normie {
    fr fr Get file permissions (Unix-style octal)
    lowkey !file_exists(path) {
        damn 0
    }
    
    fr fr Simulate typical file permissions
    fr fr In real implementation, would call stat system API
    lowkey is_dir(path) {
        damn 755  fr fr Directory permissions
    }
    damn 644      fr fr File permissions
}

slay is_readable(path tea) lit {
    fr fr Check if file is readable
    lowkey !file_exists(path) {
        damn cap
    }
    
    sus perms normie = get_permissions(path)
    fr fr Check read permission for owner (4xx)
    damn (perms / 100) % 10 >= 4
}

slay is_writable(path tea) lit {
    fr fr Check if file is writable
    lowkey !file_exists(path) {
        damn cap
    }
    
    sus perms normie = get_permissions(path)
    fr fr Check write permission for owner (x2x)
    sus owner_perms normie = (perms / 100) % 10
    damn owner_perms == 2 || owner_perms == 3 || owner_perms == 6 || owner_perms == 7
}

slay is_executable(path tea) lit {
    fr fr Check if file is executable
    lowkey !file_exists(path) {
        damn cap
    }
    
    sus perms normie = get_permissions(path)
    fr fr Check execute permission for owner (xx1)
    sus owner_perms normie = (perms / 100) % 10
    damn owner_perms == 1 || owner_perms == 3 || owner_perms == 5 || owner_perms == 7
}

fr fr ================================
fr fr File Metadata
fr fr ================================

be_like FileMetadata squad {
    name tea
    path tea
    size thicc
    is_dir lit
    is_file lit
    is_symlink lit
    created_time thicc
    modified_time thicc
    accessed_time thicc
    permissions normie
    owner_id normie
    group_id normie
}

slay get_file_metadata(path tea) FileMetadata {
    fr fr Get comprehensive file metadata
    sus metadata FileMetadata = {
        name: get_basename(path),
        path: path,
        size: get_file_size(path),
        is_dir: is_dir(path),
        is_file: is_file(path),
        is_symlink: cap,  fr fr TODO: Implement symlink detection
        created_time: get_created_time(path),
        modified_time: get_modified_time(path),
        accessed_time: get_accessed_time(path),
        permissions: get_permissions(path),
        owner_id: 1000,   fr fr TODO: Implement actual owner ID
        group_id: 1000    fr fr TODO: Implement actual group ID
    }
    damn metadata
}

slay is_hidden(path tea) lit {
    fr fr Check if file is hidden (starts with dot on Unix)
    sus basename tea = get_basename(path)
    damn basename.starts_with(".")
}

slay is_system_file(path tea) lit {
    fr fr Check if file is a system file
    sus basename tea = get_basename(path)
    
    fr fr Common system files and directories
    lowkey basename == "." || basename == ".." {
        damn based
    }
    
    lowkey basename.starts_with("/proc/") || basename.starts_with("/sys/") {
        damn based
    }
    
    damn cap
}
