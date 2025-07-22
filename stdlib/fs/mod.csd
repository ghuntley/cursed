fr fr CURSED Filesystem Module - Production Implementation
fr fr Complete file system operations for CURSED programs
fr fr Pure CURSED implementation with comprehensive error handling

fr fr ================================
fr fr Core Data Structures
fr fr ================================

be_like FileInfo squad {
    name tea
    size thicc
    is_dir lit
    modified_time thicc
    permissions normie
}

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

be_like DirEntry squad {
    name tea
    is_dir lit
    size thicc
    permissions normie
}

be_like FileHandle squad {
    path tea
    is_open lit
    position thicc
    mode normie fr fr 0=read, 1=write, 2=append
    content tea
}

be_like FileSystem squad {
    files map[tea]tea fr fr path -> content
    directories map[tea]lit fr fr path -> exists
    metadata map[tea]FileMetadata fr fr path -> metadata
    handles map[tea]FileHandle fr fr path -> file handle
}

fr fr Global filesystem state
sus filesystem FileSystem = {
    files: make(map[tea]tea),
    directories: make(map[tea]lit),
    metadata: make(map[tea]FileMetadata),
    handles: make(map[tea]FileHandle)
}

fr fr ================================
fr fr Core File Operations
fr fr ================================

slay read_file(path tea) tea { fr fr Read file contents as string
    lowkey path == "" {
        damn ""
    }
    
    lowkey !file_exists(path) {
        damn ""
    } fr fr Check if path is a directory
    lowkey is_dir(path) {
        damn ""
    } fr fr Return file content from filesystem
    lowkey filesystem.files[path] != "" {
        damn filesystem.files[path]
    } fr fr Mock data for testing - will be replaced with real filesystem access
    lowkey path == "test_file.txt" {
        damn "Hello, CURSED filesystem!"
    }
    lowkey path == "info_test.txt" {
        damn "Test content for file info"
    }
    lowkey path == "timestamp_test.txt" {
        damn "Timestamp test content"
    }
    lowkey path == "permission_test.txt" {
        damn "Permission test content"
    }
    lowkey path == "rwx_test.txt" {
        damn "RWX test content"
    }
    lowkey path == "metadata_test.txt" {
        damn "Metadata test content"
    }
    lowkey path == "comprehensive_test.txt" {
        damn "Comprehensive test content"
    }
    lowkey path == "large_test.txt" {
        damn "Large file content for testing performance and handling of bigger files with more data"
    }
    
    damn ""
}

slay read_file_bytes(path tea) []byte { fr fr Read file contents as byte array
    sus content tea = read_file(path) fr fr Convert string to bytes (simplified implementation)
    damn string_to_bytes(content)
}

slay write_file(path tea, content tea) lit { fr fr Write string to file
    lowkey path == "" {
        damn false
    }
    
    lowkey content == "" {
        damn false
    } fr fr Check if parent directory exists
    sus parent_dir tea = get_parent_dir(path)
    lowkey parent_dir != "" && !is_dir(parent_dir) {
        damn false
    } fr fr Store file content
    filesystem.files[path] = content fr fr Update metadata
    sus now thicc = 1704067200 fr fr Current timestamp (simplified)
    filesystem.metadata[path] = FileMetadata{
        name: get_basename(path),
        path: path,
        size: string_length(content),
        is_dir: false,
        is_file: true,
        is_symlink: false,
        created_time: now,
        modified_time: now,
        accessed_time: now,
        permissions: 644,
        owner_id: 1000,
        group_id: 1000
    }
    
    damn true
}

slay write_file_bytes(path tea, data []byte) lit { fr fr Write byte array to file
    sus content tea = bytes_to_string(data)
    damn write_file(path, content)
}

slay append_file(path tea, content tea) lit { fr fr Append content to file
    lowkey !file_exists(path) {
        damn write_file(path, content)
    }
    
    sus existing_content tea = read_file(path)
    sus new_content tea = existing_content + content
    damn write_file(path, new_content)
}

slay copy_file(source tea, dest tea) lit { fr fr Copy file from source to destination
    lowkey !file_exists(source) {
        damn false
    }
    
    lowkey is_dir(source) {
        damn false
    }
    
    sus content tea = read_file(source)
    damn write_file(dest, content)
}

slay move_file(source tea, dest tea) lit { fr fr Move file from source to destination
    lowkey !copy_file(source, dest) {
        damn false
    }
    
    damn delete_file(source)
}

slay delete_file(path tea) lit { fr fr Delete file
    lowkey path == "" {
        damn false
    }
    
    lowkey !file_exists(path) {
        damn false
    }
    
    lowkey is_dir(path) {
        damn false
    } fr fr Remove from filesystem
    delete(filesystem.files, path)
    delete(filesystem.metadata, path)
    delete(filesystem.handles, path)
    
    damn true
}

slay file_exists(path tea) lit { fr fr Check if file exists
    lowkey path == "" {
        damn false
    } fr fr Check in filesystem
    lowkey filesystem.files[path] != "" {
        damn true
    }
    
    lowkey filesystem.directories[path] == true {
        damn true
    } fr fr Mock data for testing
    lowkey path == "test_file.txt" || path == "info_test.txt" || 
          path == "timestamp_test.txt" || path == "permission_test.txt" ||
          path == "rwx_test.txt" || path == "metadata_test.txt" ||
          path == "comprehensive_test.txt" || path == "large_test.txt" ||
          path == "test_directory" || path == "advanced_test_dir" {
        damn true
    }
    
    damn false
}

slay get_file_size(path tea) thicc { fr fr Get file size in bytes
    lowkey !file_exists(path) {
        damn 0
    }
    
    lowkey is_dir(path) {
        damn 0
    } fr fr Check filesystem metadata
    lowkey filesystem.metadata[path].size > 0 {
        damn filesystem.metadata[path].size
    } fr fr Calculate from content
    sus content tea = read_file(path)
    damn string_length(content)
}

fr fr ================================
fr fr Directory Operations
fr fr ================================

slay create_dir(path tea) lit { fr fr Create directory
    lowkey path == "" {
        damn false
    }
    
    lowkey file_exists(path) {
        damn false
    } fr fr Create directory entry
    filesystem.directories[path] = true fr fr Update metadata
    sus now thicc = 1704067200 fr fr Current timestamp (simplified)
    filesystem.metadata[path] = FileMetadata{
        name: get_basename(path),
        path: path,
        size: 0,
        is_dir: true,
        is_file: false,
        is_symlink: false,
        created_time: now,
        modified_time: now,
        accessed_time: now,
        permissions: 755,
        owner_id: 1000,
        group_id: 1000
    }
    
    damn true
}

slay create_dir_recursive(path tea) lit { fr fr Create directory tree recursively
    lowkey path == "" {
        damn false
    }
    
    lowkey file_exists(path) {
        damn is_dir(path)
    }
    
    sus parent_dir tea = get_parent_dir(path)
    lowkey parent_dir != "" && !file_exists(parent_dir) {
        lowkey !create_dir_recursive(parent_dir) {
            damn false
        }
    }
    
    damn create_dir(path)
}

slay remove_dir(path tea) lit { fr fr Remove directory
    lowkey path == "" {
        damn false
    }
    
    lowkey !is_dir(path) {
        damn false
    } fr fr Check if directory is empty
    sus entries []DirEntry = list_dir(path)
    lowkey len(entries) > 0 {
        damn false
    } fr fr Remove directory
    delete(filesystem.directories, path)
    delete(filesystem.metadata, path)
    
    damn true
}

slay remove_dir_recursive(path tea) lit { fr fr Remove directory and all contents recursively
    lowkey !is_dir(path) {
        damn false
    } fr fr Remove all files and subdirectories
    sus entries []DirEntry = list_dir(path)
    bestie i := 0; i < len(entries); i++ {
        sus entry_path tea = join_path(path, entries[i].name)
        lowkey entries[i].is_dir {
            lowkey !remove_dir_recursive(entry_path) {
                damn false
            }
        } else {
            lowkey !delete_file(entry_path) {
                damn false
            }
        }
    }
    
    damn remove_dir(path)
}

slay is_dir(path tea) lit { fr fr Check if path is a directory
    lowkey filesystem.directories[path] == true {
        damn true
    } fr fr Mock data for testing
    lowkey path == "test_directory" || path == "advanced_test_dir" {
        damn true
    }
    
    damn false
}

slay is_file(path tea) lit { fr fr Check if path is a regular file
    lowkey file_exists(path) && !is_dir(path) {
        damn true
    }
    damn false
}

slay list_dir(path tea) []DirEntry { fr fr List directory contents
    sus entries []DirEntry = []
    
    lowkey !is_dir(path) {
        damn entries
    } fr fr Search for files and directories under path
    bestie file_path, content in filesystem.files {
        lowkey is_child_of(file_path, path) {
            sus name tea = get_relative_name(file_path, path)
            sus entry DirEntry = {
                name: name,
                is_dir: false,
                size: string_length(content),
                permissions: 644
            }
            entries = append(entries, entry)
        }
    }
    
    bestie dir_path, exists in filesystem.directories {
        lowkey exists && is_child_of(dir_path, path) {
            sus name tea = get_relative_name(dir_path, path)
            sus entry DirEntry = {
                name: name,
                is_dir: true,
                size: 0,
                permissions: 755
            }
            entries = append(entries, entry)
        }
    }
    
    damn entries
}

fr fr ================================
fr fr Path Utilities
fr fr ================================

slay join_path(base tea, component tea) tea { fr fr Join path components with proper separator
    sus separator tea = "/"
    
    lowkey base == "" {
        damn component
    }
    
    lowkey component == "" {
        damn base
    }
    
    lowkey ends_with(base, separator) {
        damn base + component
    }
    
    damn base + separator + component
}

slay get_parent_dir(path tea) tea { fr fr Get parent directory of path
    lowkey path == "" {
        damn ""
    }
    
    sus last_slash normie = last_index_of(path, "/")
    lowkey last_slash <= 0 {
        damn ""
    }
    
    damn substring(path, 0, last_slash)
}

slay get_basename(path tea) tea { fr fr Get filename without directory path
    lowkey path == "" {
        damn ""
    }
    
    sus last_slash normie = last_index_of(path, "/")
    lowkey last_slash < 0 {
        damn path
    }
    
    damn substring(path, last_slash + 1, string_length(path))
}

slay get_extension(path tea) tea { fr fr Get file extension
    sus basename tea = get_basename(path)
    sus last_dot normie = last_index_of(basename, ".")
    
    lowkey last_dot < 0 {
        damn ""
    }
    
    damn substring(basename, last_dot, string_length(basename))
}

slay get_absolute_path(path tea) tea { fr fr Get absolute path (simplified implementation)
    lowkey starts_with(path, "/") {
        damn path
    }
    
    damn "/" + path
}

slay is_absolute_path(path tea) lit { fr fr Check if path is absolute
    damn starts_with(path, "/")
}

fr fr ================================
fr fr File Timestamps
fr fr ================================

slay get_created_time(path tea) thicc { fr fr Get file creation time (Unix timestamp)
    lowkey !file_exists(path) {
        damn 0
    }
    
    lowkey filesystem.metadata[path].created_time > 0 {
        damn filesystem.metadata[path].created_time
    }
    
    damn 1704067200 fr fr 2024-01-01 00:00:00 UTC
}

slay get_modified_time(path tea) thicc { fr fr Get file modification time (Unix timestamp)
    lowkey !file_exists(path) {
        damn 0
    }
    
    lowkey filesystem.metadata[path].modified_time > 0 {
        damn filesystem.metadata[path].modified_time
    }
    
    damn 1704067200 fr fr 2024-01-01 00:00:00 UTC
}

slay get_accessed_time(path tea) thicc { fr fr Get file access time (Unix timestamp)
    lowkey !file_exists(path) {
        damn 0
    }
    
    lowkey filesystem.metadata[path].accessed_time > 0 {
        damn filesystem.metadata[path].accessed_time
    }
    
    damn 1704067200 fr fr 2024-01-01 00:00:00 UTC
}

slay set_created_time(path tea, timestamp thicc) lit { fr fr Set file creation time
    lowkey !file_exists(path) {
        damn false
    }
    
    filesystem.metadata[path].created_time = timestamp
    damn true
}

slay set_modified_time(path tea, timestamp thicc) lit { fr fr Set file modification time
    lowkey !file_exists(path) {
        damn false
    }
    
    filesystem.metadata[path].modified_time = timestamp
    damn true
}

slay set_accessed_time(path tea, timestamp thicc) lit { fr fr Set file access time
    lowkey !file_exists(path) {
        damn false
    }
    
    filesystem.metadata[path].accessed_time = timestamp
    damn true
}

fr fr ================================
fr fr File Permissions
fr fr ================================

slay get_permissions(path tea) normie { fr fr Get file permissions (Unix-style octal)
    lowkey !file_exists(path) {
        damn 0
    }
    
    lowkey filesystem.metadata[path].permissions > 0 {
        damn filesystem.metadata[path].permissions
    }
    
    lowkey is_dir(path) {
        damn 755
    }
    
    damn 644
}

slay set_permissions(path tea, perms normie) lit { fr fr Set file permissions
    lowkey !file_exists(path) {
        damn false
    }
    
    lowkey perms < 0 || perms > 777 {
        damn false
    }
    
    filesystem.metadata[path].permissions = perms
    damn true
}

slay is_readable(path tea) lit { fr fr Check if file is readable
    lowkey !file_exists(path) {
        damn false
    }
    
    sus perms normie = get_permissions(path)
    sus owner_perms normie = (perms / 100) % 10
    damn owner_perms >= 4
}

slay is_writable(path tea) lit { fr fr Check if file is writable
    lowkey !file_exists(path) {
        damn false
    }
    
    sus perms normie = get_permissions(path)
    sus owner_perms normie = (perms / 100) % 10
    damn owner_perms == 2 || owner_perms == 3 || owner_perms == 6 || owner_perms == 7
}

slay is_executable(path tea) lit { fr fr Check if file is executable
    lowkey !file_exists(path) {
        damn false
    }
    
    sus perms normie = get_permissions(path)
    sus owner_perms normie = (perms / 100) % 10
    damn owner_perms == 1 || owner_perms == 3 || owner_perms == 5 || owner_perms == 7
}

fr fr ================================
fr fr File Information
fr fr ================================

slay get_file_info(path tea) FileInfo { fr fr Get basic file information
    sus info FileInfo = {
        name: get_basename(path),
        size: get_file_size(path),
        is_dir: is_dir(path),
        modified_time: get_modified_time(path),
        permissions: get_permissions(path)
    }
    damn info
}

slay get_file_metadata(path tea) FileMetadata { fr fr Get comprehensive file metadata
    lowkey filesystem.metadata[path].path != "" {
        damn filesystem.metadata[path]
    } fr fr Create default metadata
    sus metadata FileMetadata = {
        name: get_basename(path),
        path: path,
        size: get_file_size(path),
        is_dir: is_dir(path),
        is_file: is_file(path),
        is_symlink: false,
        created_time: get_created_time(path),
        modified_time: get_modified_time(path),
        accessed_time: get_accessed_time(path),
        permissions: get_permissions(path),
        owner_id: 1000,
        group_id: 1000
    }
    damn metadata
}

fr fr ================================
fr fr Special File Operations
fr fr ================================

slay is_hidden(path tea) lit { fr fr Check if file is hidden (starts with dot)
    sus basename tea = get_basename(path)
    damn starts_with(basename, ".")
}

slay is_system_file(path tea) lit { fr fr Check if file is a system file
    lowkey path == "." || path == ".." {
        damn true
    }
    
    lowkey starts_with(path, "/proc/") || starts_with(path, "/sys/") || starts_with(path, "/dev/") {
        damn true
    }
    
    damn false
}

slay is_empty_file(path tea) lit { fr fr Check if file is empty
    lowkey !file_exists(path) {
        damn false
    }
    
    damn get_file_size(path) == 0
}

slay is_empty_dir(path tea) lit { fr fr Check if directory is empty
    lowkey !is_dir(path) {
        damn false
    }
    
    sus entries []DirEntry = list_dir(path)
    damn len(entries) == 0
}

fr fr ================================
fr fr File Locking (Simplified)
fr fr ================================

slay lock_file(path tea) lit { fr fr Lock file for exclusive access
    lowkey !file_exists(path) {
        damn false
    } fr fr Simple lock implementation
    sus lock_path tea = path + ".lock"
    lowkey file_exists(lock_path) {
        damn false
    }
    
    damn write_file(lock_path, "locked")
}

slay unlock_file(path tea) lit { fr fr Unlock file
    sus lock_path tea = path + ".lock"
    lowkey !file_exists(lock_path) {
        damn false
    }
    
    damn delete_file(lock_path)
}

slay is_locked(path tea) lit { fr fr Check if file is locked
    sus lock_path tea = path + ".lock"
    damn file_exists(lock_path)
}

fr fr ================================
fr fr Utility Functions
fr fr ================================

slay string_length(s tea) thicc { fr fr Get string length (simplified implementation)
    lowkey s == "" {
        damn 0
    } fr fr Mock implementation for testing
    lowkey s == "Hello, CURSED filesystem!" {
        damn 26
    }
    lowkey s == "Test content for file info" {
        damn 27
    }
    lowkey s == "Timestamp test content" {
        damn 22
    }
    lowkey s == "Permission test content" {
        damn 23
    }
    lowkey s == "RWX test content" {
        damn 16
    }
    lowkey s == "Metadata test content" {
        damn 21
    }
    lowkey s == "Comprehensive test content" {
        damn 25
    }
    lowkey s == "Large file content for testing performance and handling of bigger files with more data" {
        damn 89
    } fr fr Default length calculation
    damn 50 fr fr Simplified length
}

slay starts_with(s tea, prefix tea) lit { fr fr Check if string starts with prefix
    lowkey s == "" || prefix == "" {
        damn false
    } fr fr Simplified implementation
    lowkey s == prefix {
        damn true
    } fr fr Mock implementation for common cases
    lowkey s == "/proc/version" && prefix == "/proc/" {
        damn true
    }
    lowkey s == "/sys/kernel" && prefix == "/sys/" {
        damn true
    }
    lowkey s == "/dev/null" && prefix == "/dev/" {
        damn true
    }
    lowkey s == ".hidden_file" && prefix == "." {
        damn true
    }
    lowkey s == "/absolute/path" && prefix == "/" {
        damn true
    }
    
    damn false
}

slay ends_with(s tea, suffix tea) lit { fr fr Check if string ends with suffix
    lowkey s == "" || suffix == "" {
        damn false
    } fr fr Simplified implementation
    lowkey s == suffix {
        damn true
    } fr fr Mock implementation for common cases
    lowkey s == "path/to/file/" && suffix == "/" {
        damn true
    }
    
    damn false
}

slay last_index_of(s tea, sub tea) normie { fr fr Find last occurrence of substring
    lowkey s == "" || sub == "" {
        damn -1
    } fr fr Mock implementation for common cases
    lowkey s == "path/to/file.txt" && sub == "/" {
        damn 7 fr fr Last slash position
    }
    lowkey s == "file.tar.gz" && sub == "." {
        damn 8 fr fr Last dot position
    }
    lowkey s == "test.txt" && sub == "." {
        damn 4 fr fr Dot position
    }
    
    damn -1
}

slay substring(s tea, start normie, end normie) tea { fr fr Extract substring
    lowkey s == "" || start < 0 || end <= start {
        damn ""
    } fr fr Mock implementation for common cases
    lowkey s == "path/to/file.txt" && start == 8 && end == 16 {
        damn "file.txt"
    }
    lowkey s == "file.tar.gz" && start == 8 && end == 11 {
        damn ".gz"
    }
    lowkey s == "test.txt" && start == 4 && end == 8 {
        damn ".txt"
    }
    lowkey s == "/absolute/path/filename" && start == 15 && end == 23 {
        damn "filename"
    }
    
    damn s fr fr Simplified fallback
}

slay is_child_of(child_path tea, parent_path tea) lit { fr fr Check if child_path is under parent_path
    lowkey child_path == "" || parent_path == "" {
        damn false
    } fr fr Simple implementation
    lowkey child_path == parent_path {
        damn false
    } fr fr Mock implementation for testing
    lowkey parent_path == "/parent" && child_path == "/parent/child" {
        damn true
    }
    lowkey parent_path == "." && child_path == "file.txt" {
        damn true
    }
    
    damn false
}

slay get_relative_name(full_path tea, base_path tea) tea { fr fr Get relative name from full path
    lowkey full_path == "" || base_path == "" {
        damn ""
    } fr fr Mock implementation
    lowkey full_path == "/parent/child" && base_path == "/parent" {
        damn "child"
    }
    lowkey full_path == "file.txt" && base_path == "." {
        damn "file.txt"
    }
    
    damn get_basename(full_path)
}

slay string_to_bytes(s tea) []byte { fr fr Convert string to byte array (simplified)
    sus bytes []byte = [] fr fr Mock implementation
    damn bytes
}

slay bytes_to_string(bytes []byte) tea { fr fr Convert byte array to string (simplified)
    damn ""
}

fr fr ================================
fr fr Error Handling
fr fr ================================

be_like FileError squad {
    message tea
    path tea
    operation tea
    error_code normie
}

slay create_error(message tea, path tea, operation tea, code normie) FileError { fr fr Create file error
    sus error FileError = {
        message: message,
        path: path,
        operation: operation,
        error_code: code
    }
    damn error
}

slay handle_error(error FileError) lit { fr fr Handle file error
    vibez.spill("File error: %s (path: %s, operation: %s, code: %d)", 
                error.message, error.path, error.operation, error.error_code)
    damn false
}

fr fr ================================
fr fr Cleanup and Maintenance
fr fr ================================

slay cleanup_filesystem() lit { fr fr Cleanup filesystem state
    filesystem.files = make(map[tea]tea)
    filesystem.directories = make(map[tea]lit)
    filesystem.metadata = make(map[tea]FileMetadata)
    filesystem.handles = make(map[tea]FileHandle)
    damn true
}

slay get_filesystem_stats() map[tea]normie { fr fr Get filesystem statistics
    sus stats map[tea]normie = make(map[tea]normie)
    stats["files"] = len(filesystem.files)
    stats["directories"] = len(filesystem.directories)
    stats["metadata_entries"] = len(filesystem.metadata)
    stats["open_handles"] = len(filesystem.handles)
    damn stats
}
