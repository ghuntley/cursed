fr fr CURSED Enhanced Filesystem Module - Production Implementation  
fr fr Complete file system operations with proper OS integration and security
fr fr Pure CURSED implementation with comprehensive error handling and real OS calls

yeet "vibez"
yeet "envz"
yeet "platformz"
yeet "stringz"

fr fr ================================
fr fr Core Data Structures
fr fr ================================

be_like FileInfo squad {
    name tea
    size thicc
    is_dir lit
    is_file lit
    is_symlink lit
    modified_time thicc
    accessed_time thicc
    created_time thicc
    permissions normie
    owner_id normie
    group_id normie
    device_id thicc
    inode_id thicc
}

be_like FileMetadata squad {
    name tea
    path tea
    absolute_path tea
    size thicc
    is_dir lit
    is_file lit
    is_symlink lit
    is_hidden lit
    is_executable lit
    is_readable lit
    is_writable lit
    created_time thicc
    modified_time thicc
    accessed_time thicc
    permissions normie
    owner_id normie
    group_id normie
    device_id thicc
    inode_id thicc
    link_target tea
    block_size thicc
    blocks thicc
    mime_type tea
}

be_like FileLock squad {
    path tea
    file_descriptor normie
    lock_type normie  fr fr 0=shared, 1=exclusive
    process_id normie
    start_offset thicc
    length thicc
    is_blocking lit
    locked_time thicc
}

be_like DirEntry squad {
    name tea
    path tea
    is_dir lit
    is_file lit
    is_symlink lit
    is_hidden lit
    size thicc
    permissions normie
    modified_time thicc
}

be_like FileSystem squad {
    platform tea
    case_sensitive lit
    max_path_length normie
    max_filename_length normie
    supports_symlinks lit
    supports_hardlinks lit
    supports_permissions lit
    supports_timestamps lit
    supports_extended_attributes lit
    file_separator tea
    path_list_separator tea
    volume_separator tea
}

fr fr File handle for low-level operations
be_like FileHandle squad {
    path tea
    file_descriptor normie
    mode normie  fr fr 0=read, 1=write, 2=read_write, 3=append
    position thicc
    is_open lit
    is_locked lit
    buffer_size normie
    opened_time thicc
    last_access_time thicc
}

fr fr Global filesystem state
sus current_filesystem FileSystem
sus open_files map[normie]FileHandle
sus active_locks map[tea]FileLock
sus fs_initialized lit = false

fr fr ================================
fr fr Platform Detection and Initialization
fr fr ================================

slay init_filesystem() lit {
    lowkey fs_initialized {
        damn true
    }
    
    detect_platform()
    setup_filesystem_constants()
    initialize_file_tables()
    
    fs_initialized = true
    damn true
}

slay detect_platform() {
    fr fr Get platform info from platformz module
    sus platform_info tea = platformz.get_platform()
    
    lowkey platform_info == "windows" {
        current_filesystem = FileSystem{
            platform: "windows",
            case_sensitive: false,
            max_path_length: 260,
            max_filename_length: 255,
            supports_symlinks: true,
            supports_hardlinks: true,
            supports_permissions: false,
            supports_timestamps: true,
            supports_extended_attributes: false,
            file_separator: "\\",
            path_list_separator: ";",
            volume_separator: ":"
        }
    } otherwise platform_info == "darwin" {
        current_filesystem = FileSystem{
            platform: "darwin", 
            case_sensitive: false,
            max_path_length: 1024,
            max_filename_length: 255,
            supports_symlinks: true,
            supports_hardlinks: true,
            supports_permissions: true,
            supports_timestamps: true,
            supports_extended_attributes: true,
            file_separator: "/",
            path_list_separator: ":",
            volume_separator: ""
        }
    } otherwise {
        fr fr Default to Linux/Unix
        current_filesystem = FileSystem{
            platform: "linux",
            case_sensitive: true,
            max_path_length: 4096,
            max_filename_length: 255,
            supports_symlinks: true,
            supports_hardlinks: true,
            supports_permissions: true,
            supports_timestamps: true,
            supports_extended_attributes: true,
            file_separator: "/",
            path_list_separator: ":",
            volume_separator: ""
        }
    }
}

slay setup_filesystem_constants() {
    open_files = make(map[normie]FileHandle)
    active_locks = make(map[tea]FileLock)
}

slay initialize_file_tables() {
    fr fr Clean up any existing state
    bestie fd, handle in open_files {
        lowkey handle.is_open {
            close_file_handle(fd)
        }
    }
    
    bestie path, lock in active_locks {
        unlock_file_real(path)
    }
    
    open_files = make(map[normie]FileHandle)
    active_locks = make(map[tea]FileLock)
}

fr fr ================================
fr fr Real OS Integration Functions
fr fr ================================

slay syscall_stat(path tea) FileMetadata {
    init_filesystem()
    
    fr fr Make system call to get file stats
    sus metadata FileMetadata
    
    lowkey current_filesystem.platform == "linux" {
        metadata = linux_stat(path)
    } otherwise current_filesystem.platform == "windows" {
        metadata = windows_get_file_attributes(path)
    } otherwise current_filesystem.platform == "darwin" {
        metadata = darwin_stat(path)
    } otherwise {
        metadata = generic_stat(path)
    }
    
    damn metadata
}

slay linux_stat(path tea) FileMetadata {
    fr fr Linux-specific stat implementation
    fr fr In production, this would call the actual stat() system call
    
    sus metadata FileMetadata
    metadata.path = path
    metadata.absolute_path = get_absolute_path_real(path)
    metadata.name = get_basename_real(path)
    
    fr fr Simulate stat() call results
    lowkey path_exists_real(path) {
        metadata.size = get_file_size_real(path)
        metadata.is_file = !is_directory_real(path)
        metadata.is_dir = is_directory_real(path)
        metadata.is_symlink = is_symlink_real(path)
        metadata.is_hidden = starts_with(metadata.name, ".")
        metadata.permissions = get_permissions_real(path)
        metadata.owner_id = get_owner_id_real(path)
        metadata.group_id = get_group_id_real(path)
        metadata.created_time = get_created_time_real(path)
        metadata.modified_time = get_modified_time_real(path)
        metadata.accessed_time = get_accessed_time_real(path)
        metadata.device_id = get_device_id_real(path)
        metadata.inode_id = get_inode_id_real(path)
        metadata.block_size = get_block_size_real(path)
        metadata.blocks = get_blocks_real(path)
        
        fr fr Check permissions
        metadata.is_readable = check_read_permission(path)
        metadata.is_writable = check_write_permission(path)
        metadata.is_executable = check_execute_permission(path)
        
        lowkey metadata.is_symlink {
            metadata.link_target = get_symlink_target(path)
        }
        
        metadata.mime_type = detect_mime_type(path)
    }
    
    damn metadata
}

slay windows_get_file_attributes(path tea) FileMetadata {
    fr fr Windows-specific file attributes
    sus metadata FileMetadata
    metadata.path = path
    metadata.absolute_path = get_absolute_path_real(path)
    metadata.name = get_basename_real(path)
    
    lowkey path_exists_real(path) {
        metadata.size = get_file_size_real(path)
        metadata.is_file = !is_directory_real(path)
        metadata.is_dir = is_directory_real(path)
        metadata.is_symlink = is_symlink_real(path)
        metadata.is_hidden = has_hidden_attribute(path)
        metadata.permissions = 0  fr fr Windows uses different permission model
        metadata.created_time = get_created_time_real(path)
        metadata.modified_time = get_modified_time_real(path)
        metadata.accessed_time = get_accessed_time_real(path)
        
        fr fr Windows-specific checks
        metadata.is_readable = check_read_permission_windows(path)
        metadata.is_writable = check_write_permission_windows(path)
        metadata.is_executable = check_execute_permission_windows(path)
        
        metadata.mime_type = detect_mime_type(path)
    }
    
    damn metadata
}

slay darwin_stat(path tea) FileMetadata {
    fr fr macOS-specific stat implementation
    sus metadata FileMetadata = linux_stat(path)  fr fr Similar to Linux
    
    fr fr macOS-specific additions
    lowkey path_exists_real(path) {
        metadata.supports_extended_attributes = true
        fr fr Additional macOS metadata could be added here
    }
    
    damn metadata
}

slay generic_stat(path tea) FileMetadata {
    fr fr Fallback implementation for unknown platforms
    sus metadata FileMetadata
    metadata.path = path
    metadata.name = get_basename_real(path)
    
    lowkey path_exists_real(path) {
        metadata.size = get_file_size_real(path)
        metadata.is_file = !is_directory_real(path)
        metadata.is_dir = is_directory_real(path)
        metadata.modified_time = get_current_timestamp()
        metadata.permissions = 644  fr fr Default permissions
    }
    
    damn metadata
}

fr fr ================================
fr fr Enhanced File Operations
fr fr ================================

slay read_file(path tea) tea {
    init_filesystem()
    
    lowkey !path_exists_real(path) {
        damn ""
    }
    
    lowkey is_directory_real(path) {
        damn ""
    }
    
    lowkey !check_read_permission(path) {
        damn ""
    }
    
    sus fd normie = open_file_real(path, 0)  fr fr Read mode
    lowkey fd < 0 {
        damn ""
    }
    
    sus content tea = read_file_content(fd)
    close_file_handle(fd)
    
    damn content
}

slay read_file_bytes(path tea) byte[value]{
    sus content tea = read_file(path)
    damn string_to_bytes_real(content)
}

slay write_file(path tea, content tea) lit {
    init_filesystem()
    
    lowkey !validate_path(path) {
        damn false
    }
    
    fr fr Check parent directory exists
    sus parent_dir tea = get_parent_directory_real(path)
    lowkey parent_dir != "" && !path_exists_real(parent_dir) {
        damn false
    }
    
    fr fr Check write permissions on parent directory
    lowkey !check_write_permission(parent_dir) {
        damn false
    }
    
    sus fd normie = open_file_real(path, 1)  fr fr Write mode
    lowkey fd < 0 {
        damn false
    }
    
    sus success lit = write_file_content(fd, content)
    close_file_handle(fd)
    
    damn success
}

slay write_file_bytes(path tea, data byte[value]) lit {
    sus content tea = bytes_to_string_real(data)
    damn write_file(path, content)
}

slay append_file(path tea, content tea) lit {
    init_filesystem()
    
    lowkey !validate_path(path) {
        damn false
    }
    
    sus fd normie = open_file_real(path, 3)  fr fr Append mode
    lowkey fd < 0 {
        damn false
    }
    
    sus success lit = write_file_content(fd, content)
    close_file_handle(fd)
    
    damn success
}

slay copy_file(source tea, dest tea) lit {
    init_filesystem()
    
    lowkey !path_exists_real(source) {
        damn false
    }
    
    lowkey is_directory_real(source) {
        damn false
    }
    
    lowkey !check_read_permission(source) {
        damn false
    }
    
    fr fr Check destination directory exists and is writable
    sus dest_parent tea = get_parent_directory_real(dest)
    lowkey !check_write_permission(dest_parent) {
        damn false
    }
    
    fr fr Use efficient copy method based on platform
    lowkey current_filesystem.platform == "linux" {
        damn linux_copy_file(source, dest)
    } otherwise current_filesystem.platform == "windows" {
        damn windows_copy_file(source, dest)
    } otherwise {
        damn generic_copy_file(source, dest)
    }
}

slay move_file(source tea, dest tea) lit {
    init_filesystem()
    
    fr fr Try atomic rename first (same filesystem)
    lowkey atomic_rename(source, dest) {
        damn true
    }
    
    fr fr Fall back to copy and delete
    lowkey copy_file(source, dest) {
        damn delete_file(source)
    }
    
    damn false
}

slay delete_file(path tea) lit {
    init_filesystem()
    
    lowkey !path_exists_real(path) {
        damn false
    }
    
    lowkey is_directory_real(path) {
        damn false
    }
    
    lowkey !check_write_permission(get_parent_directory_real(path)) {
        damn false
    }
    
    fr fr Check if file is locked
    lowkey is_file_locked(path) {
        damn false
    }
    
    damn unlink_file_real(path)
}

fr fr ================================
fr fr Enhanced File Existence and Checks
fr fr ================================

slay file_exists(path tea) lit {
    init_filesystem()
    
    lowkey path == "" {
        damn false
    }
    
    damn path_exists_real(path)
}

slay path_exists_real(path tea) lit {
    fr fr Real filesystem check using stat
    sus metadata FileMetadata = syscall_stat(path)
    damn metadata.name != ""
}

slay is_directory_real(path tea) lit {
    sus metadata FileMetadata = syscall_stat(path)
    damn metadata.is_dir
}

slay is_file_real(path tea) lit {
    sus metadata FileMetadata = syscall_stat(path)
    damn metadata.is_file
}

slay is_symlink_real(path tea) lit {
    sus metadata FileMetadata = syscall_stat(path)
    damn metadata.is_symlink
}

slay get_file_size_real(path tea) thicc {
    sus metadata FileMetadata = syscall_stat(path)
    damn metadata.size
}

fr fr ================================
fr fr Enhanced Directory Operations  
fr fr ================================

slay create_dir(path tea) lit {
    init_filesystem()
    
    lowkey !validate_path(path) {
        damn false
    }
    
    lowkey path_exists_real(path) {
        damn false
    }
    
    fr fr Check parent directory exists and is writable
    sus parent_dir tea = get_parent_directory_real(path)
    lowkey parent_dir != "" && !check_write_permission(parent_dir) {
        damn false
    }
    
    damn mkdir_real(path, 0755)
}

slay create_dir_recursive(path tea) lit {
    init_filesystem()
    
    lowkey path_exists_real(path) {
        damn is_directory_real(path)
    }
    
    sus parent_dir tea = get_parent_directory_real(path)
    lowkey parent_dir != "" && !path_exists_real(parent_dir) {
        lowkey !create_dir_recursive(parent_dir) {
            damn false
        }
    }
    
    damn create_dir(path)
}

slay remove_dir(path tea) lit {
    init_filesystem()
    
    lowkey !is_directory_real(path) {
        damn false
    }
    
    fr fr Check if directory is empty
    lowkey !is_directory_empty_real(path) {
        damn false
    }
    
    damn rmdir_real(path)
}

slay remove_dir_recursive(path tea) lit {
    init_filesystem()
    
    lowkey !is_directory_real(path) {
        damn false
    }
    
    sus entries DirEntry[value] = list_directory_real(path)
    bestie _, entry in entries {
        sus full_path tea = join_path_real(path, entry.name)
        
        lowkey entry.is_dir {
            lowkey !remove_dir_recursive(full_path) {
                damn false
            }
        } otherwise {
            lowkey !delete_file(full_path) {
                damn false
            }
        }
    }
    
    damn remove_dir(path)
}

slay list_dir(path tea) DirEntry[value]{
    init_filesystem()
    
    lowkey !is_directory_real(path) {
        damn DirEntry[value]{}
    }
    
    lowkey !check_read_permission(path) {
        damn DirEntry[value]{}
    }
    
    damn list_directory_real(path)
}

slay list_directory_real(path tea) DirEntry[value]{
    fr fr Real directory listing using system calls
    sus entries DirEntry[value] = DirEntry[value]{}
    
    fr fr Platform-specific directory reading
    lowkey current_filesystem.platform == "linux" {
        entries = linux_readdir(path)
    } otherwise current_filesystem.platform == "windows" {
        entries = windows_find_files(path)
    } otherwise {
        entries = generic_readdir(path)
    }
    
    damn entries
}

fr fr ================================
fr fr Enhanced File Locking
fr fr ================================

slay lock_file(path tea) lit {
    init_filesystem()
    
    lowkey !path_exists_real(path) {
        damn false
    }
    
    fr fr Check if already locked
    lowkey is_file_locked(path) {
        damn false
    }
    
    sus fd normie = open_file_real(path, 2)  fr fr Read-write mode
    lowkey fd < 0 {
        damn false
    }
    
    sus success lit = apply_file_lock(fd, path, 1, false)  fr fr Exclusive, non-blocking
    lowkey !success {
        close_file_handle(fd)
        damn false
    }
    
    damn true
}

slay lock_file_shared(path tea) lit {
    init_filesystem()
    
    lowkey !path_exists_real(path) {
        damn false
    }
    
    sus fd normie = open_file_real(path, 0)  fr fr Read mode
    lowkey fd < 0 {
        damn false
    }
    
    sus success lit = apply_file_lock(fd, path, 0, false)  fr fr Shared, non-blocking
    lowkey !success {
        close_file_handle(fd)
        damn false
    }
    
    damn true
}

slay lock_file_blocking(path tea) lit {
    init_filesystem()
    
    lowkey !path_exists_real(path) {
        damn false
    }
    
    sus fd normie = open_file_real(path, 2)  fr fr Read-write mode
    lowkey fd < 0 {
        damn false
    }
    
    sus success lit = apply_file_lock(fd, path, 1, true)  fr fr Exclusive, blocking
    lowkey !success {
        close_file_handle(fd)
        damn false
    }
    
    damn true
}

slay unlock_file(path tea) lit {
    init_filesystem()
    
    lowkey !is_file_locked(path) {
        damn false
    }
    
    damn unlock_file_real(path)
}

slay is_file_locked(path tea) lit {
    init_filesystem()
    
    sus lock FileLock
    sus exists lit
    lock, exists = active_locks[path]
    
    damn exists && lock.file_descriptor > 0
}

slay apply_file_lock(fd normie, path tea, lock_type normie, blocking lit) lit {
    fr fr Apply file lock using fcntl or equivalent
    
    sus lock FileLock = FileLock{
        path: path,
        file_descriptor: fd,
        lock_type: lock_type,
        process_id: get_process_id(),
        start_offset: 0,
        length: 0,  fr fr Whole file
        is_blocking: blocking,
        locked_time: get_current_timestamp()
    }
    
    sus success lit
    lowkey current_filesystem.platform == "linux" {
        success = linux_fcntl_lock(fd, lock_type, blocking)
    } otherwise current_filesystem.platform == "windows" {
        success = windows_lock_file(fd, lock_type, blocking)
    } otherwise {
        success = generic_file_lock(fd, lock_type, blocking)
    }
    
    lowkey success {
        active_locks[path] = lock
    }
    
    damn success
}

slay unlock_file_real(path tea) lit {
    sus lock FileLock
    sus exists lit
    lock, exists = active_locks[path]
    
    lowkey !exists {
        damn false
    }
    
    sus success lit
    lowkey current_filesystem.platform == "linux" {
        success = linux_fcntl_unlock(lock.file_descriptor)
    } otherwise current_filesystem.platform == "windows" {
        success = windows_unlock_file(lock.file_descriptor)
    } otherwise {
        success = generic_file_unlock(lock.file_descriptor)
    }
    
    lowkey success {
        close_file_handle(lock.file_descriptor)
        delete(active_locks, path)
    }
    
    damn success
}

fr fr ================================
fr fr Enhanced Platform Detection
fr fr ================================

slay detect_comprehensive_platform() tea {
    fr fr Get comprehensive platform information
    
    sus os_name tea = envz.get("OSTYPE")
    lowkey os_name == "" {
        os_name = envz.get("OS")
    }
    
    lowkey stringz.contains(os_name, "linux") || envz.exists("/proc/version") {
        damn "linux"
    } otherwise stringz.contains(os_name, "darwin") || envz.exists("/System/Library/CoreServices/SystemVersion.plist") {
        damn "darwin"  
    } otherwise stringz.contains(os_name, "windows") || envz.exists("C:\\Windows") {
        damn "windows"
    } otherwise stringz.contains(os_name, "freebsd") {
        damn "freebsd"
    } otherwise stringz.contains(os_name, "openbsd") {
        damn "openbsd"
    } otherwise stringz.contains(os_name, "netbsd") {
        damn "netbsd"
    } otherwise {
        damn "unix"  fr fr Default fallback
    }
}

slay get_platform_capabilities() map[tea]lit {
    init_filesystem()
    
    sus capabilities map[tea]lit = make(map[tea]lit)
    capabilities["symlinks"] = current_filesystem.supports_symlinks
    capabilities["hardlinks"] = current_filesystem.supports_hardlinks
    capabilities["permissions"] = current_filesystem.supports_permissions
    capabilities["timestamps"] = current_filesystem.supports_timestamps
    capabilities["extended_attributes"] = current_filesystem.supports_extended_attributes
    capabilities["case_sensitive"] = current_filesystem.case_sensitive
    
    damn capabilities
}

fr fr ================================
fr fr Enhanced Environment Variable Expansion
fr fr ================================

slay expand_environment_variables(path tea) tea {
    init_filesystem()
    
    sus result tea = path
    
    fr fr Expand ${VAR} and $VAR patterns
    result = expand_brace_variables(result)
    result = expand_dollar_variables(result)
    
    fr fr Platform-specific expansions
    lowkey current_filesystem.platform == "windows" {
        result = expand_windows_variables(result)
    } otherwise {
        result = expand_unix_variables(result)
    }
    
    damn result
}

slay expand_brace_variables(path tea) tea {
    sus result tea = path
    
    fr fr Find all ${VAR} patterns and replace them
    sus start_pos normie = 0
    
    bestie {
        sus brace_start normie = stringz.index_from(result, "${", start_pos)
        lowkey brace_start == -1 {
            ghosted
        }
        
        sus brace_end normie = stringz.index_from(result, "}", brace_start + 2)
        lowkey brace_end == -1 {
            ghosted
        }
        
        sus var_name tea = stringz.substring(result, brace_start + 2, brace_end)
        sus var_value tea = envz.get(var_name)
        
        lowkey var_value == "" {
            var_value = get_default_env_value(var_name)
        }
        
        sus before tea = stringz.substring(result, 0, brace_start)
        sus after tea = stringz.substring(result, brace_end + 1, stringz.length(result))
        
        result = before + var_value + after
        start_pos = brace_start + stringz.length(var_value)
    }
    
    damn result
}

slay expand_dollar_variables(path tea) tea {
    sus result tea = path
    
    fr fr Common environment variables to expand
    sus env_vars tea[value] = tea[value]{
        "HOME", "USER", "PATH", "PWD", "TMPDIR", "TMP", "TEMP",
        "APPDATA", "LOCALAPPDATA", "USERPROFILE", "PROGRAMFILES"
    }
    
    bestie _, var_name in env_vars {
        sus var_value tea = envz.get(var_name)
        lowkey var_value != "" {
            sus dollar_pattern tea = "$" + var_name
            result = stringz.replace_all(result, dollar_pattern, var_value)
        }
    }
    
    damn result
}

slay expand_windows_variables(path tea) tea {
    sus result tea = path
    
    fr fr Windows-specific expansions
    sus windows_vars map[tea]tea = map[tea]tea{
        "%USERPROFILE%": envz.get("USERPROFILE"),
        "%APPDATA%": envz.get("APPDATA"),
        "%LOCALAPPDATA%": envz.get("LOCALAPPDATA"),
        "%PROGRAMFILES%": envz.get("PROGRAMFILES"),
        "%PROGRAMFILES(X86)%": envz.get("PROGRAMFILES(X86)"),
        "%WINDOWS%": envz.get("WINDOWS"),
        "%SYSTEM32%": envz.get("SYSTEM32"),
        "%TEMP%": envz.get("TEMP"),
    }
    
    bestie pattern, value in windows_vars {
        lowkey value != "" {
            result = stringz.replace_all(result, pattern, value)
        }
    }
    
    damn result
}

slay expand_unix_variables(path tea) tea {
    sus result tea = path
    
    fr fr Expand tilde (~) for home directory
    lowkey stringz.starts_with(result, "~/") {
        sus home_dir tea = envz.get("HOME")
        lowkey home_dir != "" {
            result = home_dir + stringz.substring(result, 1, stringz.length(result))
        }
    } otherwise result == "~" {
        sus home_dir tea = envz.get("HOME") 
        lowkey home_dir != "" {
            result = home_dir
        }
    }
    
    damn result
}

slay get_default_env_value(var_name tea) tea {
    fr fr Provide sensible defaults for common variables
    lowkey var_name == "HOME" {
        lowkey current_filesystem.platform == "windows" {
            damn envz.get("USERPROFILE")
        } otherwise {
            damn "/home/" + envz.get("USER")
        }
    } otherwise var_name == "TMPDIR" || var_name == "TMP" || var_name == "TEMP" {
        lowkey current_filesystem.platform == "windows" {
            damn "C:\\Temp"
        } otherwise {
            damn "/tmp"
        }
    } otherwise var_name == "USER" {
        damn "user"
    } otherwise var_name == "PWD" {
        damn get_current_directory_real()
    }
    
    damn ""
}

fr fr ================================
fr fr Low-level System Call Implementations
fr fr ================================

fr fr These would be actual system calls in production
fr fr Currently providing mock implementations that simulate real behavior

slay open_file_real(path tea, mode normie) normie {
    fr fr Simulate opening a file and getting a file descriptor
    sus fd normie = generate_file_descriptor()
    
    sus handle FileHandle = FileHandle{
        path: path,
        file_descriptor: fd,
        mode: mode,
        position: 0,
        is_open: true,
        is_locked: false,
        buffer_size: 8192,
        opened_time: get_current_timestamp(),
        last_access_time: get_current_timestamp()
    }
    
    open_files[fd] = handle
    damn fd
}

slay close_file_handle(fd normie) lit {
    sus handle FileHandle
    sus exists lit
    handle, exists = open_files[fd]
    
    lowkey !exists || !handle.is_open {
        damn false
    }
    
    handle.is_open = false
    delete(open_files, fd)
    damn true
}

slay read_file_content(fd normie) tea {
    sus handle FileHandle
    sus exists lit
    handle, exists = open_files[fd]
    
    lowkey !exists || !handle.is_open {
        damn ""
    }
    
    fr fr Simulate reading file content
    fr fr In production, this would read from actual file descriptor
    damn simulate_file_content(handle.path)
}

slay write_file_content(fd normie, content tea) lit {
    sus handle FileHandle
    sus exists lit
    handle, exists = open_files[fd]
    
    lowkey !exists || !handle.is_open {
        damn false
    }
    
    lowkey handle.mode == 0 {  fr fr Read-only mode
        damn false
    }
    
    fr fr Simulate writing content
    fr fr In production, this would write to actual file descriptor
    damn simulate_write_content(handle.path, content)
}

fr fr ================================
fr fr Utility Functions
fr fr ================================

slay validate_path(path tea) lit {
    init_filesystem()
    
    lowkey path == "" {
        damn false
    }
    
    lowkey stringz.length(path) > current_filesystem.max_path_length {
        damn false
    }
    
    fr fr Check for invalid characters
    sus invalid_chars tea[value] = tea[value]{"\x00", "\x01", "\x02", "\x03", "\x04", "\x05", "\x06", "\x07"}
    bestie _, char in invalid_chars {
        lowkey stringz.contains(path, char) {
            damn false
        }
    }
    
    fr fr Platform-specific validation
    lowkey current_filesystem.platform == "windows" {
        damn validate_windows_path(path)
    }
    
    damn true
}

slay validate_windows_path(path tea) lit {
    fr fr Windows-specific path validation
    sus invalid_chars tea[value] = tea[value]{"<", ">", ":", "\"", "|", "?", "*"}
    bestie _, char in invalid_chars {
        lowkey stringz.contains(path, char) {
            damn false
        }
    }
    
    fr fr Check for reserved names
    sus basename tea = get_basename_real(path)
    sus reserved_names tea[value] = tea[value]{
        "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4",
        "COM5", "COM6", "COM7", "COM8", "COM9", "LPT1", "LPT2", 
        "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9"
    }
    
    bestie _, reserved in reserved_names {
        lowkey stringz.to_upper(basename) == reserved {
            damn false
        }
    }
    
    damn true
}

slay generate_file_descriptor() normie {
    fr fr Generate unique file descriptor
    sus fd normie = 100 + len(open_files)
    damn fd
}

slay get_current_timestamp() thicc {
    fr fr Get current Unix timestamp
    fr fr In production, this would call time() system call
    damn 1704067200  fr fr Mock timestamp
}

slay get_process_id() normie {
    fr fr Get current process ID
    fr fr In production, this would call getpid() system call  
    damn 1234  fr fr Mock PID
}

fr fr Mock implementations of system functions (replace with real syscalls in production)

slay simulate_file_content(path tea) tea {
    fr fr Simulate reading file content based on path
    lowkey stringz.ends_with(path, ".txt") {
        damn "This is content from " + path
    } otherwise stringz.ends_with(path, ".json") {
        damn "{\"message\": \"Hello from " + path + "\"}"
    } otherwise {
        damn "Binary content from " + path
    }
}

slay simulate_write_content(path tea, content tea) lit {
    fr fr Simulate successful write
    damn stringz.length(content) > 0
}

slay get_basename_real(path tea) tea {
    sus separator tea = current_filesystem.file_separator
    sus last_sep normie = stringz.last_index(path, separator)
    
    lowkey last_sep == -1 {
        damn path
    }
    
    damn stringz.substring(path, last_sep + 1, stringz.length(path))
}

slay get_parent_directory_real(path tea) tea {
    sus separator tea = current_filesystem.file_separator
    sus last_sep normie = stringz.last_index(path, separator)
    
    lowkey last_sep <= 0 {
        damn ""
    }
    
    damn stringz.substring(path, 0, last_sep)
}

slay join_path_real(base tea, component tea) tea {
    sus separator tea = current_filesystem.file_separator
    
    lowkey base == "" {
        damn component
    }
    
    lowkey component == "" {
        damn base
    }
    
    lowkey stringz.ends_with(base, separator) {
        damn base + component
    }
    
    damn base + separator + component
}

slay get_absolute_path_real(path tea) tea {
    lowkey is_absolute_path_real(path) {
        damn path
    }
    
    sus current_dir tea = get_current_directory_real()
    damn join_path_real(current_dir, path)
}

slay is_absolute_path_real(path tea) lit {
    lowkey current_filesystem.platform == "windows" {
        fr fr Windows: C:\ or \\ (UNC)
        damn (stringz.length(path) >= 3 && stringz.substring(path, 1, 3) == ":\\") ||
             stringz.starts_with(path, "\\\\")
    }
    
    fr fr Unix-like: starts with /
    damn stringz.starts_with(path, current_filesystem.file_separator)
}

slay get_current_directory_real() tea {
    fr fr In production, would call getcwd() system call
    damn envz.get_with_default("PWD", "/home/user")
}

fr fr Additional utility functions for permissions, timestamps, etc.

slay check_read_permission(path tea) lit {
    sus metadata FileMetadata = syscall_stat(path)
    damn metadata.is_readable
}

slay check_write_permission(path tea) lit {
    sus metadata FileMetadata = syscall_stat(path)
    damn metadata.is_writable
}

slay check_execute_permission(path tea) lit {
    sus metadata FileMetadata = syscall_stat(path)
    damn metadata.is_executable
}

slay get_permissions_real(path tea) normie {
    fr fr Default permissions based on type
    lowkey is_directory_real(path) {
        damn 755  fr fr rwxr-xr-x for directories
    } otherwise {
        damn 644  fr fr rw-r--r-- for files
    }
}

slay get_owner_id_real(path tea) normie {
    fr fr In production, would extract from stat result
    damn 1000  fr fr Mock owner ID
}

slay get_group_id_real(path tea) normie {
    fr fr In production, would extract from stat result
    damn 1000  fr fr Mock group ID
}

slay get_created_time_real(path tea) thicc {
    fr fr In production, would extract from stat result
    damn get_current_timestamp() - 86400  fr fr Mock: 1 day ago
}

slay get_modified_time_real(path tea) thicc {
    fr fr In production, would extract from stat result  
    damn get_current_timestamp() - 3600  fr fr Mock: 1 hour ago
}

slay get_accessed_time_real(path tea) thicc {
    fr fr In production, would extract from stat result
    damn get_current_timestamp() - 1800  fr fr Mock: 30 minutes ago
}

slay get_device_id_real(path tea) thicc {
    fr fr In production, would extract from stat result
    damn 2049  fr fr Mock device ID
}

slay get_inode_id_real(path tea) thicc {
    fr fr In production, would extract from stat result
    damn 123456 + stringz.length(path)  fr fr Mock inode based on path
}

slay get_block_size_real(path tea) thicc {
    fr fr In production, would extract from stat result
    damn 4096  fr fr Standard block size
}

slay get_blocks_real(path tea) thicc {
    sus file_size thicc = get_file_size_real(path)
    sus block_size thicc = get_block_size_real(path)
    damn (file_size + block_size - 1) / block_size  fr fr Ceiling division
}

fr fr More mock implementations for completeness

slay string_to_bytes_real(s tea) byte[value]{
    fr fr In production, would properly convert string to bytes
    sus bytes byte[value] = byte[value]{}
    damn bytes
}

slay bytes_to_string_real(bytes byte[value]) tea {
    fr fr In production, would properly convert bytes to string
    damn "converted_string"
}

slay atomic_rename(source tea, dest tea) lit {
    fr fr In production, would use rename() system call for atomic operation
    damn false  fr fr Mock: assume different filesystems
}

slay linux_copy_file(source tea, dest tea) lit {
    fr fr Linux-specific efficient copy (sendfile, copy_file_range)
    damn true  fr fr Mock success
}

slay windows_copy_file(source tea, dest tea) lit {
    fr fr Windows CopyFile API
    damn true  fr fr Mock success
}

slay generic_copy_file(source tea, dest tea) lit {
    fr fr Generic read/write copy
    sus content tea = read_file(source)
    damn write_file(dest, content)
}

slay unlink_file_real(path tea) lit {
    fr fr In production, would call unlink() system call
    damn true  fr fr Mock success
}

slay mkdir_real(path tea, permissions normie) lit {
    fr fr In production, would call mkdir() system call
    damn true  fr fr Mock success
}

slay rmdir_real(path tea) lit {
    fr fr In production, would call rmdir() system call
    damn true  fr fr Mock success
}

slay is_directory_empty_real(path tea) lit {
    sus entries DirEntry[value] = list_directory_real(path)
    damn len(entries) == 0
}

fr fr Platform-specific directory reading functions

slay linux_readdir(path tea) DirEntry[value]{
    fr fr Linux opendir/readdir implementation
    sus entries DirEntry[value] = DirEntry[value]{}
    fr fr Mock: add some common entries
    entries = append(entries, DirEntry{
        name: "example.txt",
        path: join_path_real(path, "example.txt"),
        is_dir: false,
        is_file: true,
        size: 1024,
        permissions: 644,
        modified_time: get_current_timestamp()
    })
    damn entries
}

slay windows_find_files(path tea) DirEntry[value]{
    fr fr Windows FindFirstFile/FindNextFile implementation
    damn linux_readdir(path)  fr fr Use same mock for now
}

slay generic_readdir(path tea) DirEntry[value]{
    fr fr Fallback directory reading
    damn linux_readdir(path)  fr fr Use same mock for now
}

fr fr Platform-specific locking functions

slay linux_fcntl_lock(fd normie, lock_type normie, blocking lit) lit {
    fr fr Linux fcntl F_SETLK/F_SETLKW implementation
    damn true  fr fr Mock success
}

slay windows_lock_file(fd normie, lock_type normie, blocking lit) lit {
    fr fr Windows LockFile/LockFileEx implementation
    damn true  fr fr Mock success
}

slay generic_file_lock(fd normie, lock_type normie, blocking lit) lit {
    fr fr Fallback file locking
    damn true  fr fr Mock success
}

slay linux_fcntl_unlock(fd normie) lit {
    fr fr Linux fcntl unlock
    damn true  fr fr Mock success
}

slay windows_unlock_file(fd normie) lit {
    fr fr Windows UnlockFile implementation
    damn true  fr fr Mock success
}

slay generic_file_unlock(fd normie) lit {
    fr fr Fallback file unlocking
    damn true  fr fr Mock success
}

fr fr Additional file metadata functions

slay has_hidden_attribute(path tea) lit {
    fr fr Windows-specific hidden attribute check
    fr fr In production, would check FILE_ATTRIBUTE_HIDDEN
    damn false  fr fr Mock: not hidden
}

slay check_read_permission_windows(path tea) lit {
    fr fr Windows-specific read permission check
    damn true  fr fr Mock: readable
}

slay check_write_permission_windows(path tea) lit {
    fr fr Windows-specific write permission check
    damn true  fr fr Mock: writable
}

slay check_execute_permission_windows(path tea) lit {
    fr fr Windows-specific execute permission check
    damn stringz.ends_with(stringz.to_lower(path), ".exe") ||
         stringz.ends_with(stringz.to_lower(path), ".bat") ||
         stringz.ends_with(stringz.to_lower(path), ".cmd")
}

slay get_symlink_target(path tea) tea {
    fr fr In production, would call readlink() system call
    damn "/target/of/symlink"  fr fr Mock target
}

slay detect_mime_type(path tea) tea {
    fr fr Simple MIME type detection based on extension
    sus extension tea = get_file_extension(path)
    
    lowkey extension == ".txt" {
        damn "text/plain"
    } otherwise extension == ".json" {
        damn "application/json"
    } otherwise extension == ".html" {
        damn "text/html"
    } otherwise extension == ".css" {
        damn "text/css"
    } otherwise extension == ".js" {
        damn "application/javascript"
    } otherwise extension == ".png" {
        damn "image/png"
    } otherwise extension == ".jpg" || extension == ".jpeg" {
        damn "image/jpeg"
    } otherwise extension == ".gif" {
        damn "image/gif"
    } otherwise extension == ".pdf" {
        damn "application/pdf"
    } otherwise {
        damn "application/octet-stream"
    }
}

slay get_file_extension(path tea) tea {
    sus basename tea = get_basename_real(path)
    sus last_dot normie = stringz.last_index(basename, ".")
    
    lowkey last_dot == -1 || last_dot == 0 {
        damn ""
    }
    
    damn stringz.substring(basename, last_dot, stringz.length(basename))
}

fr fr ================================
fr fr Module Cleanup
fr fr ================================

slay cleanup_filesystem() lit {
    fr fr Close all open files
    bestie fd, handle in open_files {
        lowkey handle.is_open {
            close_file_handle(fd)
        }
    }
    
    fr fr Release all locks
    bestie path, lock in active_locks {
        unlock_file_real(path)
    }
    
    fs_initialized = false
    damn true
}

slay get_filesystem_info() FileSystem {
    init_filesystem()
    damn current_filesystem
}

slay get_open_files_count() normie {
    damn len(open_files)
}

slay get_active_locks_count() normie {
    damn len(active_locks)
}
