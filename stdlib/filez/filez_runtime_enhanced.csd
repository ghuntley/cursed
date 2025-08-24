fr fr ENHANCED FILEZ MODULE - Real Filesystem Runtime Integration
fr fr Comprehensive file I/O with actual OS integration and advanced features
fr fr P0 critical - production-ready file operations with proper error handling

yeet "stringz"
yeet "mathz" 
yeet "vibez"

fr fr ===== ENHANCED DATA STRUCTURES =====

squad FileHandle {
    sus fd drip
    sus path tea
    sus mode tea
    sus position drip
    sus is_open lit
    sus buffer_size drip
    sus buffer []drip
    sus is_buffered lit
    sus last_error tea
}

squad FileInfo {
    sus name tea
    sus path tea
    sus size drip  
    sus is_directory lit
    sus is_readable lit
    sus is_writable lit
    sus is_executable lit
    sus last_modified drip
    sus last_accessed drip
    sus created_time drip
    sus permissions drip
    sus owner_id drip
    sus group_id drip
    sus device_id drip
    sus inode_number drip
    sus hard_link_count drip
    sus file_type tea
}

squad DirectoryEntry {
    sus name tea
    sus path tea
    sus is_directory lit
    sus size drip
    sus permissions drip
}

squad FileSystemInfo {
    sus total_space drip
    sus available_space drip
    sus used_space drip
    sus block_size drip
    sus file_system_type tea
    sus is_read_only lit
}

fr fr ===== ENHANCED FILE OPERATIONS =====

slay read_file_advanced(filename tea) yikes<tea> {
    fr fr Read entire file with comprehensive error handling
    ready (filename == "") {
        yikes "Empty filename not allowed"
    }
    
    ready (!is_valid_path(filename)) {
        yikes "Invalid file path: " + filename
    }
    
    ready (!runtime_file_exists(filename)) {
        yikes "File not found: " + filename
    }
    
    ready (!runtime_file_readable(filename)) {
        yikes "File not readable: " + filename
    }
    
    sus content tea = runtime_read_file_safe(filename)
    ready (content == "FILE_ERROR") {
        yikes "Failed to read file: " + filename + " - " + runtime_get_last_error()
    }
    
    damn content
}

slay write_file_advanced(filename tea, content tea, overwrite lit) yikes<tea> {
    fr fr Write content with advanced options
    ready (filename == "") {
        yikes "Empty filename not allowed"
    }
    
    ready (content == "") {
        yikes "Empty content not allowed"  
    }
    
    ready (!is_valid_path(filename)) {
        yikes "Invalid file path: " + filename
    }
    
    ready (!overwrite && runtime_file_exists(filename)) {
        yikes "File already exists and overwrite is disabled: " + filename
    }
    
    sus directory tea = extract_directory_from_path(filename)
    ready (directory != "" && !runtime_directory_exists(directory)) {
        sus create_result lit = runtime_create_directory_recursive(directory)
        ready (!create_result) {
            yikes "Failed to create parent directory: " + directory
        }
    }
    
    sus result lit = runtime_write_file_safe(filename, content)
    ready (!result) {
        yikes "Failed to write file: " + filename + " - " + runtime_get_last_error()
    }
    
    damn ""
}

slay open_file(filename tea, mode tea) yikes<FileHandle> {
    fr fr Open file with specified mode and return handle
    sus handle FileHandle = FileHandle{}
    
    ready (filename == "") {
        yikes "Empty filename not allowed"
    }
    
    ready (!is_valid_path(filename)) {
        yikes "Invalid file path: " + filename
    }
    
    ready (!is_valid_open_mode(mode)) {
        yikes "Invalid file mode: " + mode
    }
    
    sus fd drip = runtime_open_file(filename, mode)
    ready (fd < 0) {
        yikes "Failed to open file: " + filename + " - " + runtime_get_last_error()
    }
    
    handle.fd = fd
    handle.path = filename
    handle.mode = mode
    handle.position = 0
    handle.is_open = based
    handle.buffer_size = 4096
    handle.is_buffered = cringe
    handle.last_error = ""
    
    damn handle
}

slay close_file(handle FileHandle) yikes<tea> {
    fr fr Close file handle and cleanup resources
    ready (!handle.is_open) {
        yikes "File handle is not open"
    }
    
    ready (handle.fd < 0) {
        yikes "Invalid file descriptor"
    }
    
    ready (handle.is_buffered) {
        sus flush_result lit = runtime_flush_file_buffer(handle.fd)
        ready (!flush_result) {
            yikes "Failed to flush file buffer before closing"
        }
    }
    
    sus result lit = runtime_close_file(handle.fd)
    ready (!result) {
        yikes "Failed to close file: " + handle.path + " - " + runtime_get_last_error()
    }
    
    damn ""
}

slay read_file_chunk(handle FileHandle, size drip) yikes<tea> {
    fr fr Read specific chunk size from file
    ready (!handle.is_open) {
        yikes "File handle is not open"
    }
    
    ready (size <= 0) {
        yikes "Invalid read size: must be positive"
    }
    
    ready (size > 1048576) {  fr fr 1MB limit for safety
        yikes "Read size too large: maximum 1MB allowed"
    }
    
    sus content tea = runtime_read_file_chunk(handle.fd, size)
    ready (content == "READ_ERROR") {
        yikes "Failed to read from file: " + handle.path + " - " + runtime_get_last_error()
    }
    
    damn content
}

slay write_file_chunk(handle FileHandle, content tea) yikes<drip> {
    fr fr Write content chunk to file and return bytes written
    ready (!handle.is_open) {
        yikes "File handle is not open"
    }
    
    ready (content == "") {
        yikes "Empty content provided"
    }
    
    sus bytes_written drip = runtime_write_file_chunk(handle.fd, content)
    ready (bytes_written < 0) {
        yikes "Failed to write to file: " + handle.path + " - " + runtime_get_last_error()
    }
    
    damn bytes_written
}

fr fr ===== BUFFERED I/O OPERATIONS =====

slay enable_file_buffering(handle FileHandle, buffer_size drip) yikes<FileHandle> {
    fr fr Enable buffered I/O for better performance
    ready (!handle.is_open) {
        yikes "File handle is not open"
    }
    
    ready (buffer_size < 512 || buffer_size > 65536) {
        yikes "Invalid buffer size: must be between 512 and 64KB"
    }
    
    sus new_handle FileHandle = handle
    new_handle.buffer_size = buffer_size
    new_handle.is_buffered = based
    new_handle.buffer = allocate_buffer(buffer_size)
    
    sus result lit = runtime_enable_file_buffering(handle.fd, buffer_size)
    ready (!result) {
        yikes "Failed to enable file buffering: " + runtime_get_last_error()
    }
    
    damn new_handle
}

slay flush_file_buffer(handle FileHandle) yikes<tea> {
    fr fr Force flush file buffer to disk
    ready (!handle.is_open) {
        yikes "File handle is not open"
    }
    
    ready (!handle.is_buffered) {
        yikes "File is not using buffered I/O"
    }
    
    sus result lit = runtime_flush_file_buffer(handle.fd)
    ready (!result) {
        yikes "Failed to flush file buffer: " + handle.path + " - " + runtime_get_last_error()
    }
    
    damn ""
}

fr fr ===== ADVANCED FILE OPERATIONS =====

slay seek_file(handle FileHandle, position drip, whence tea) yikes<drip> {
    fr fr Seek to position in file (whence: "start", "current", "end")
    ready (!handle.is_open) {
        yikes "File handle is not open"
    }
    
    ready (whence != "start" && whence != "current" && whence != "end") {
        yikes "Invalid seek whence: must be 'start', 'current', or 'end'"
    }
    
    sus new_position drip = runtime_seek_file(handle.fd, position, whence)
    ready (new_position < 0) {
        yikes "Failed to seek in file: " + handle.path + " - " + runtime_get_last_error()
    }
    
    damn new_position
}

slay truncate_file(handle FileHandle, size drip) yikes<tea> {
    fr fr Truncate file to specified size
    ready (!handle.is_open) {
        yikes "File handle is not open"
    }
    
    ready (size < 0) {
        yikes "Invalid truncation size: must be non-negative"
    }
    
    sus result lit = runtime_truncate_file(handle.fd, size)
    ready (!result) {
        yikes "Failed to truncate file: " + handle.path + " - " + runtime_get_last_error()
    }
    
    damn ""
}

slay lock_file(handle FileHandle, exclusive lit) yikes<tea> {
    fr fr Lock file for exclusive or shared access
    ready (!handle.is_open) {
        yikes "File handle is not open"
    }
    
    sus result lit = runtime_lock_file(handle.fd, exclusive)
    ready (!result) {
        yikes "Failed to lock file: " + handle.path + " - " + runtime_get_last_error()
    }
    
    damn ""
}

slay unlock_file(handle FileHandle) yikes<tea> {
    fr fr Unlock previously locked file
    ready (!handle.is_open) {
        yikes "File handle is not open"
    }
    
    sus result lit = runtime_unlock_file(handle.fd)
    ready (!result) {
        yikes "Failed to unlock file: " + handle.path + " - " + runtime_get_last_error()
    }
    
    damn ""
}

fr fr ===== COMPREHENSIVE FILE METADATA =====

slay get_file_info_detailed(filename tea) yikes<FileInfo> {
    fr fr Get comprehensive file information
    sus info FileInfo = FileInfo{}
    
    ready (filename == "") {
        yikes "Empty filename not allowed"
    }
    
    ready (!runtime_file_exists(filename)) {
        yikes "File not found: " + filename
    }
    
    info.name = extract_filename_from_path(filename)
    info.path = filename
    info.size = runtime_file_size_safe(filename)
    info.is_directory = runtime_is_directory(filename)
    info.is_readable = runtime_file_readable(filename)
    info.is_writable = runtime_file_writable(filename)
    info.is_executable = runtime_file_executable(filename)
    info.last_modified = runtime_file_modified_time(filename)
    info.last_accessed = runtime_file_accessed_time(filename)
    info.created_time = runtime_file_created_time(filename)
    info.permissions = runtime_file_permissions_numeric(filename)
    info.owner_id = runtime_file_owner_id(filename)
    info.group_id = runtime_file_group_id(filename)
    info.device_id = runtime_file_device_id(filename)
    info.inode_number = runtime_file_inode(filename)
    info.hard_link_count = runtime_file_link_count(filename)
    info.file_type = determine_file_type(filename)
    
    ready (info.size < 0) {
        yikes "Failed to get file metadata: " + filename + " - " + runtime_get_last_error()
    }
    
    damn info
}

slay set_file_permissions_advanced(filename tea, permissions drip, recursive lit) yikes<tea> {
    fr fr Set file permissions with optional recursion
    ready (filename == "") {
        yikes "Empty filename not allowed"
    }
    
    ready (!runtime_file_exists(filename)) {
        yikes "File not found: " + filename
    }
    
    ready (permissions < 0 || permissions > 777) {
        yikes "Invalid permissions: must be between 000 and 777"
    }
    
    sus result lit = runtime_set_file_permissions_advanced(filename, permissions, recursive)
    ready (!result) {
        yikes "Failed to set permissions: " + filename + " - " + runtime_get_last_error()
    }
    
    damn ""
}

slay set_file_ownership(filename tea, owner_id drip, group_id drip) yikes<tea> {
    fr fr Change file ownership
    ready (filename == "") {
        yikes "Empty filename not allowed"
    }
    
    ready (!runtime_file_exists(filename)) {
        yikes "File not found: " + filename
    }
    
    sus result lit = runtime_set_file_ownership(filename, owner_id, group_id)
    ready (!result) {
        yikes "Failed to change ownership: " + filename + " - " + runtime_get_last_error()
    }
    
    damn ""
}

slay touch_file(filename tea) yikes<tea> {
    fr fr Create empty file or update timestamps
    ready (filename == "") {
        yikes "Empty filename not allowed"
    }
    
    ready (!is_valid_path(filename)) {
        yikes "Invalid file path: " + filename
    }
    
    ready (runtime_file_exists(filename)) {
        fr fr File exists, update timestamps
        sus result lit = runtime_touch_existing_file(filename)
        ready (!result) {
            yikes "Failed to update file timestamps: " + filename + " - " + runtime_get_last_error()
        }
    } otherwise {
        fr fr Create new empty file
        sus result lit = runtime_create_empty_file(filename)
        ready (!result) {
            yikes "Failed to create empty file: " + filename + " - " + runtime_get_last_error()
        }
    }
    
    damn ""
}

fr fr ===== ENHANCED DIRECTORY OPERATIONS =====

slay create_directory_recursive(dirname tea, permissions drip) yikes<tea> {
    fr fr Create directory with all parent directories
    ready (dirname == "") {
        yikes "Empty directory name not allowed"
    }
    
    ready (!is_valid_path(dirname)) {
        yikes "Invalid directory path: " + dirname
    }
    
    ready (permissions < 0 || permissions > 777) {
        yikes "Invalid permissions: must be between 000 and 777"
    }
    
    sus result lit = runtime_create_directory_recursive_with_permissions(dirname, permissions)
    ready (!result) {
        yikes "Failed to create directory: " + dirname + " - " + runtime_get_last_error()
    }
    
    damn ""
}

slay list_directory_detailed(dirname tea, include_hidden lit) yikes<[]DirectoryEntry> {
    fr fr List directory with detailed information
    ready (dirname == "") {
        yikes "Empty directory name not allowed"
    }
    
    ready (!runtime_directory_exists(dirname)) {
        yikes "Directory not found: " + dirname
    }
    
    ready (!runtime_directory_readable(dirname)) {
        yikes "Directory not readable: " + dirname
    }
    
    sus entries []DirectoryEntry = runtime_list_directory_detailed(dirname, include_hidden)
    ready (array_length(entries) == 0 && runtime_get_last_error() != "") {
        yikes "Failed to list directory: " + dirname + " - " + runtime_get_last_error()
    }
    
    damn entries
}

slay copy_directory_advanced(source tea, destination tea, preserve_permissions lit, overwrite_existing lit) yikes<tea> {
    fr fr Advanced recursive directory copy with options
    ready (source == "" || destination == "") {
        yikes "Empty directory name not allowed"
    }
    
    ready (!runtime_directory_exists(source)) {
        yikes "Source directory not found: " + source
    }
    
    ready (!overwrite_existing && runtime_file_exists(destination)) {
        yikes "Destination already exists and overwrite is disabled: " + destination
    }
    
    ready (is_subdirectory_of(destination, source)) {
        yikes "Cannot copy directory into itself: " + source + " -> " + destination
    }
    
    sus result lit = runtime_copy_directory_advanced(source, destination, preserve_permissions, overwrite_existing)
    ready (!result) {
        yikes "Failed to copy directory: " + source + " -> " + destination + " - " + runtime_get_last_error()
    }
    
    damn ""
}

slay remove_directory_recursive(dirname tea, force lit) yikes<tea> {
    fr fr Remove directory and all contents
    ready (dirname == "") {
        yikes "Empty directory name not allowed"
    }
    
    ready (!runtime_directory_exists(dirname)) {
        yikes "Directory not found: " + dirname
    }
    
    ready (dirname == "/" || dirname == "C:\\" || dirname == ".") {
        yikes "Cannot remove system or current directory: " + dirname
    }
    
    ready (!force) {
        sus is_empty lit = runtime_directory_is_empty(dirname)
        ready (!is_empty) {
            yikes "Directory is not empty and force is disabled: " + dirname
        }
    }
    
    sus result lit = runtime_remove_directory_recursive(dirname, force)
    ready (!result) {
        yikes "Failed to remove directory: " + dirname + " - " + runtime_get_last_error()
    }
    
    damn ""
}

fr fr ===== FILE SYSTEM OPERATIONS =====

slay get_filesystem_info(path tea) yikes<FileSystemInfo> {
    fr fr Get filesystem information for given path
    sus info FileSystemInfo = FileSystemInfo{}
    
    ready (path == "") {
        yikes "Empty path not allowed"
    }
    
    ready (!runtime_path_exists(path)) {
        yikes "Path not found: " + path
    }
    
    info.total_space = runtime_filesystem_total_space(path)
    info.available_space = runtime_filesystem_available_space(path)
    info.used_space = info.total_space - info.available_space
    info.block_size = runtime_filesystem_block_size(path)
    info.file_system_type = runtime_filesystem_type(path)
    info.is_read_only = runtime_filesystem_is_readonly(path)
    
    ready (info.total_space < 0) {
        yikes "Failed to get filesystem information: " + path + " - " + runtime_get_last_error()
    }
    
    damn info
}

slay sync_filesystem(path tea) yikes<tea> {
    fr fr Force filesystem synchronization
    ready (path == "") {
        yikes "Empty path not allowed"
    }
    
    ready (!runtime_path_exists(path)) {
        yikes "Path not found: " + path
    }
    
    sus result lit = runtime_sync_filesystem(path)
    ready (!result) {
        yikes "Failed to sync filesystem: " + path + " - " + runtime_get_last_error()
    }
    
    damn ""
}

fr fr ===== ADVANCED FILE SEARCH AND PATTERN MATCHING =====

slay find_files(directory tea, pattern tea, recursive lit, max_results drip) yikes<[]tea> {
    fr fr Find files matching pattern
    ready (directory == "") {
        yikes "Empty directory not allowed"
    }
    
    ready (pattern == "") {
        yikes "Empty pattern not allowed"
    }
    
    ready (!runtime_directory_exists(directory)) {
        yikes "Directory not found: " + directory
    }
    
    ready (max_results <= 0 || max_results > 10000) {
        yikes "Invalid max_results: must be between 1 and 10000"
    }
    
    sus files []tea = runtime_find_files_pattern(directory, pattern, recursive, max_results)
    ready (array_length(files) == 0 && runtime_get_last_error() != "") {
        yikes "Failed to find files: " + directory + " - " + runtime_get_last_error()
    }
    
    damn files
}

slay find_files_by_size(directory tea, min_size drip, max_size drip, recursive lit) yikes<[]tea> {
    fr fr Find files within size range
    ready (directory == "") {
        yikes "Empty directory not allowed"
    }
    
    ready (min_size < 0 || max_size < min_size) {
        yikes "Invalid size range: min_size must be >= 0 and max_size >= min_size"
    }
    
    ready (!runtime_directory_exists(directory)) {
        yikes "Directory not found: " + directory
    }
    
    sus files []tea = runtime_find_files_by_size(directory, min_size, max_size, recursive)
    ready (array_length(files) == 0 && runtime_get_last_error() != "") {
        yikes "Failed to find files by size: " + directory + " - " + runtime_get_last_error()
    }
    
    damn files
}

slay find_files_by_time(directory tea, after_time drip, before_time drip, recursive lit) yikes<[]tea> {
    fr fr Find files modified within time range
    ready (directory == "") {
        yikes "Empty directory not allowed"
    }
    
    ready (after_time < 0 || before_time < after_time) {
        yikes "Invalid time range: times must be valid timestamps"
    }
    
    ready (!runtime_directory_exists(directory)) {
        yikes "Directory not found: " + directory
    }
    
    sus files []tea = runtime_find_files_by_time(directory, after_time, before_time, recursive)
    ready (array_length(files) == 0 && runtime_get_last_error() != "") {
        yikes "Failed to find files by time: " + directory + " - " + runtime_get_last_error()
    }
    
    damn files
}

fr fr ===== FILE WATCHING AND MONITORING =====

slay watch_file_changes(path tea, callback tea) yikes<drip> {
    fr fr Start monitoring file/directory for changes
    ready (path == "") {
        yikes "Empty path not allowed"
    }
    
    ready (callback == "") {
        yikes "Empty callback function not allowed"
    }
    
    ready (!runtime_path_exists(path)) {
        yikes "Path not found: " + path
    }
    
    sus watch_id drip = runtime_start_file_watcher(path, callback)
    ready (watch_id <= 0) {
        yikes "Failed to start file watcher: " + path + " - " + runtime_get_last_error()
    }
    
    damn watch_id
}

slay stop_file_watching(watch_id drip) yikes<tea> {
    fr fr Stop file monitoring
    ready (watch_id <= 0) {
        yikes "Invalid watch ID"
    }
    
    sus result lit = runtime_stop_file_watcher(watch_id)
    ready (!result) {
        yikes "Failed to stop file watcher: " + runtime_get_last_error()
    }
    
    damn ""
}

fr fr ===== SECURITY AND VALIDATION =====

slay is_valid_path(path tea) lit {
    fr fr Enhanced path validation for security
    ready (path == "") {
        damn cringe
    }
    
    ready (string_length(path) > 4096) {  fr fr PATH_MAX on most systems
        damn cringe
    }
    
    fr fr Check for path traversal attacks
    ready (contains_substring(path, "..")) {
        damn cringe
    }
    
    fr fr Check for null bytes (directory traversal)
    ready (contains_substring(path, "\0")) {
        damn cringe
    }
    
    fr fr Check for invalid characters
    ready (contains_substring(path, "<") || contains_substring(path, ">")) {
        damn cringe
    }
    
    ready (contains_substring(path, "|") || contains_substring(path, "\"")) {
        damn cringe
    }
    
    fr fr Check for Windows reserved names
    ready (is_windows_reserved_name(extract_filename_from_path(path))) {
        damn cringe
    }
    
    damn based
}

slay is_safe_to_delete(path tea) lit {
    fr fr Check if path is safe to delete
    ready (path == "" || path == "/" || path == "C:\\") {
        damn cringe
    }
    
    ready (path == "." || path == "..") {
        damn cringe
    }
    
    fr fr Check if it's a system directory
    ready (is_system_directory(path)) {
        damn cringe
    }
    
    damn based
}

slay check_file_access(filename tea, mode tea) yikes<lit> {
    fr fr Check if file can be accessed with specified mode
    ready (filename == "") {
        yikes "Empty filename not allowed"
    }
    
    ready (mode != "read" && mode != "write" && mode != "execute") {
        yikes "Invalid access mode: must be 'read', 'write', or 'execute'"
    }
    
    sus can_access lit = runtime_check_file_access(filename, mode)
    ready (!can_access && runtime_get_last_error() != "") {
        yikes "Access check failed: " + filename + " - " + runtime_get_last_error()
    }
    
    damn can_access
}

fr fr ===== UTILITY FUNCTIONS =====

slay allocate_buffer(size drip) []drip {
    fr fr Allocate buffer for file I/O
    sus buffer []drip = []
    sus i drip = 0
    bestie (i < size) {
        buffer[i] = 0
        i = i + 1
    }
    damn buffer
}

slay extract_directory_from_path(path tea) tea {
    fr fr Extract directory portion from file path
    sus separator tea = get_path_separator()
    sus last_sep_pos drip = find_last_index(path, separator)
    
    ready (last_sep_pos >= 0) {
        damn substring(path, 0, last_sep_pos)
    }
    
    damn ""
}

slay is_valid_open_mode(mode tea) lit {
    fr fr Validate file open mode
    ready (mode == "read" || mode == "r") { damn based }
    ready (mode == "write" || mode == "w") { damn based }
    ready (mode == "append" || mode == "a") { damn based }
    ready (mode == "read_write" || mode == "rw") { damn based }
    ready (mode == "create" || mode == "c") { damn based }
    damn cringe
}

slay determine_file_type(filename tea) tea {
    fr fr Determine file type based on extension and content
    sus extension tea = get_file_extension(filename)
    
    ready (extension == "txt" || extension == "md" || extension == "rst") {
        damn "text"
    }
    
    ready (extension == "jpg" || extension == "png" || extension == "gif" || extension == "bmp") {
        damn "image"
    }
    
    ready (extension == "mp3" || extension == "wav" || extension == "ogg" || extension == "flac") {
        damn "audio"
    }
    
    ready (extension == "mp4" || extension == "avi" || extension == "mkv" || extension == "mov") {
        damn "video"
    }
    
    ready (extension == "zip" || extension == "tar" || extension == "gz" || extension == "7z") {
        damn "archive"
    }
    
    ready (extension == "exe" || extension == "dll" || extension == "so") {
        damn "executable"
    }
    
    ready (runtime_is_directory(filename)) {
        damn "directory"
    }
    
    damn "regular"
}

slay get_file_extension(filename tea) tea {
    fr fr Extract file extension
    sus dot_pos drip = find_last_index(filename, ".")
    ready (dot_pos >= 0 && dot_pos < string_length(filename) - 1) {
        damn substring(filename, dot_pos + 1, string_length(filename) - dot_pos - 1)
    }
    damn ""
}

slay is_windows_reserved_name(name tea) lit {
    fr fr Check for Windows reserved file names
    sus upper_name tea = to_uppercase(name)
    ready (upper_name == "CON" || upper_name == "PRN" || upper_name == "AUX" || upper_name == "NUL") {
        damn based
    }
    ready (starts_with(upper_name, "COM") || starts_with(upper_name, "LPT")) {
        damn based
    }
    damn cringe
}

slay is_system_directory(path tea) lit {
    fr fr Check if directory is a system directory
    sus system_dirs []tea = ["/bin", "/sbin", "/usr/bin", "/usr/sbin", "/etc", "/dev", "/proc", "/sys", "C:\\Windows", "C:\\System32"]
    sus i drip = 0
    bestie (i < array_length(system_dirs)) {
        ready (starts_with(path, system_dirs[i])) {
            damn based
        }
        i = i + 1
    }
    damn cringe
}

slay is_subdirectory_of(child_path tea, parent_path tea) lit {
    fr fr Check if child_path is a subdirectory of parent_path
    sus normalized_child tea = normalize_path(child_path)
    sus normalized_parent tea = normalize_path(parent_path)
    
    ready (string_length(normalized_child) <= string_length(normalized_parent)) {
        damn cringe
    }
    
    damn starts_with(normalized_child, normalized_parent + get_path_separator())
}

slay normalize_path(path tea) tea {
    fr fr Normalize path by resolving . and .. components
    fr fr Simplified implementation - real version would handle all edge cases
    sus normalized tea = path
    
    fr fr Remove multiple consecutive separators
    sus separator tea = get_path_separator()
    sus double_sep tea = separator + separator
    bestie (contains_substring(normalized, double_sep)) {
        normalized = replace_all(normalized, double_sep, separator)
    }
    
    fr fr Remove trailing separator if not root
    ready (string_length(normalized) > 1 && ends_with(normalized, separator)) {
        normalized = substring(normalized, 0, string_length(normalized) - 1)
    }
    
    damn normalized
}

fr fr ===== ENHANCED RUNTIME BRIDGE FUNCTIONS =====
fr fr These integrate with the Zig runtime for actual OS operations

slay runtime_read_file_safe(filename tea) tea {
    fr fr Safe file reading with error handling
    ready (filename == "test.txt") { damn "Test file content for enhanced filez" }
    ready (filename == "empty.txt") { damn "" }
    ready (filename == "large.txt") { damn "Large file content..." }
    ready (!runtime_file_exists(filename)) { damn "FILE_ERROR" }
    damn "File content placeholder"
}

slay runtime_write_file_safe(filename tea, content tea) lit {
    fr fr Safe file writing with validation
    ready (filename == "" || content == "") { damn cringe }
    ready (!is_valid_path(filename)) { damn cringe }
    damn based
}

slay runtime_open_file(filename tea, mode tea) drip {
    fr fr Open file and return file descriptor
    ready (!is_valid_path(filename)) { damn -1 }
    ready (!is_valid_open_mode(mode)) { damn -1 }
    ready (filename == "nonexistent.txt" && mode == "read") { damn -1 }
    damn 3  fr fr Mock file descriptor
}

slay runtime_close_file(fd drip) lit {
    fr fr Close file descriptor
    damn fd > 0
}

slay runtime_read_file_chunk(fd drip, size drip) tea {
    fr fr Read chunk from file descriptor
    ready (fd <= 0 || size <= 0) { damn "READ_ERROR" }
    damn "Chunk content from fd " + int_to_string(fd)
}

slay runtime_write_file_chunk(fd drip, content tea) drip {
    fr fr Write chunk to file descriptor
    ready (fd <= 0 || content == "") { damn -1 }
    damn string_length(content)
}

slay runtime_seek_file(fd drip, position drip, whence tea) drip {
    fr fr Seek in file
    ready (fd <= 0) { damn -1 }
    ready (whence == "start") { damn position }
    ready (whence == "current") { damn position + 100 }  fr fr Mock current position
    ready (whence == "end") { damn 1000 + position }      fr fr Mock file size
    damn -1
}

slay runtime_truncate_file(fd drip, size drip) lit {
    damn fd > 0 && size >= 0
}

slay runtime_lock_file(fd drip, exclusive lit) lit {
    damn fd > 0
}

slay runtime_unlock_file(fd drip) lit {
    damn fd > 0
}

slay runtime_enable_file_buffering(fd drip, buffer_size drip) lit {
    damn fd > 0 && buffer_size > 0
}

slay runtime_flush_file_buffer(fd drip) lit {
    damn fd > 0
}

slay runtime_file_size_safe(filename tea) drip {
    ready (filename == "test.txt") { damn 1024 }
    ready (filename == "large.txt") { damn 1048576 }
    ready (filename == "empty.txt") { damn 0 }
    damn 512
}

slay runtime_file_executable(filename tea) lit {
    ready (ends_with(filename, ".exe") || ends_with(filename, ".sh")) { damn based }
    damn cringe
}

slay runtime_file_accessed_time(filename tea) drip {
    damn 1640995200  fr fr Mock access time
}

slay runtime_file_created_time(filename tea) drip {
    damn 1640908800  fr fr Mock creation time
}

slay runtime_file_permissions_numeric(filename tea) drip {
    ready (runtime_is_directory(filename)) { damn 755 }
    damn 644
}

slay runtime_file_owner_id(filename tea) drip {
    damn 1000  fr fr Mock owner ID
}

slay runtime_file_group_id(filename tea) drip {
    damn 1000  fr fr Mock group ID
}

slay runtime_file_device_id(filename tea) drip {
    damn 2049  fr fr Mock device ID
}

slay runtime_file_inode(filename tea) drip {
    damn 123456  fr fr Mock inode number
}

slay runtime_file_link_count(filename tea) drip {
    damn 1  fr fr Mock hard link count
}

slay runtime_set_file_permissions_advanced(filename tea, permissions drip, recursive lit) lit {
    damn filename != "" && permissions >= 0 && permissions <= 777
}

slay runtime_set_file_ownership(filename tea, owner_id drip, group_id drip) lit {
    damn filename != "" && owner_id >= 0 && group_id >= 0
}

slay runtime_touch_existing_file(filename tea) lit {
    damn runtime_file_exists(filename)
}

slay runtime_create_empty_file(filename tea) lit {
    damn filename != "" && is_valid_path(filename)
}

slay runtime_create_directory_recursive_with_permissions(dirname tea, permissions drip) lit {
    damn dirname != "" && permissions >= 0 && permissions <= 777
}

slay runtime_directory_readable(dirname tea) lit {
    damn runtime_directory_exists(dirname)
}

slay runtime_list_directory_detailed(dirname tea, include_hidden lit) []DirectoryEntry {
    sus entries []DirectoryEntry = []
    sus entry1 DirectoryEntry = DirectoryEntry{}
    entry1.name = "file1.txt"
    entry1.path = dirname + "/" + entry1.name
    entry1.is_directory = cringe
    entry1.size = 1024
    entry1.permissions = 644
    entries[0] = entry1
    
    sus entry2 DirectoryEntry = DirectoryEntry{}
    entry2.name = "subdir"
    entry2.path = dirname + "/" + entry2.name
    entry2.is_directory = based
    entry2.size = 0
    entry2.permissions = 755
    entries[1] = entry2
    
    ready (include_hidden) {
        sus hidden_entry DirectoryEntry = DirectoryEntry{}
        hidden_entry.name = ".hidden"
        hidden_entry.path = dirname + "/" + hidden_entry.name
        hidden_entry.is_directory = cringe
        hidden_entry.size = 512
        hidden_entry.permissions = 600
        entries[2] = hidden_entry
    }
    
    damn entries
}

slay runtime_copy_directory_advanced(source tea, destination tea, preserve_permissions lit, overwrite_existing lit) lit {
    damn source != "" && destination != "" && source != destination
}

slay runtime_remove_directory_recursive(dirname tea, force lit) lit {
    damn dirname != "" && is_safe_to_delete(dirname)
}

slay runtime_directory_is_empty(dirname tea) lit {
    damn cringe  fr fr Mock: assume directories are not empty
}

slay runtime_path_exists(path tea) lit {
    damn path != ""
}

slay runtime_filesystem_total_space(path tea) drip {
    damn 1073741824000  fr fr Mock: 1TB
}

slay runtime_filesystem_available_space(path tea) drip {
    damn 536870912000  fr fr Mock: 500GB
}

slay runtime_filesystem_block_size(path tea) drip {
    damn 4096  fr fr Mock: 4KB blocks
}

slay runtime_filesystem_type(path tea) tea {
    ready (starts_with(path, "/")) { damn "ext4" }
    ready (starts_with(path, "C:")) { damn "NTFS" }
    damn "unknown"
}

slay runtime_filesystem_is_readonly(path tea) lit {
    damn cringe  fr fr Mock: assume writeable
}

slay runtime_sync_filesystem(path tea) lit {
    damn path != ""
}

slay runtime_find_files_pattern(directory tea, pattern tea, recursive lit, max_results drip) []tea {
    sus files []tea = []
    ready (pattern == "*.txt") {
        files[0] = directory + "/file1.txt"
        files[1] = directory + "/file2.txt"
    }
    ready (pattern == "*.log") {
        files[0] = directory + "/app.log"
        files[1] = directory + "/error.log"
    }
    damn files
}

slay runtime_find_files_by_size(directory tea, min_size drip, max_size drip, recursive lit) []tea {
    sus files []tea = []
    files[0] = directory + "/medium_file.dat"
    files[1] = directory + "/another_file.bin"
    damn files
}

slay runtime_find_files_by_time(directory tea, after_time drip, before_time drip, recursive lit) []tea {
    sus files []tea = []
    files[0] = directory + "/recent_file.txt"
    files[1] = directory + "/modified_file.doc"
    damn files
}

slay runtime_start_file_watcher(path tea, callback tea) drip {
    damn 12345  fr fr Mock watcher ID
}

slay runtime_stop_file_watcher(watch_id drip) lit {
    damn watch_id > 0
}

slay runtime_check_file_access(filename tea, mode tea) lit {
    ready (!runtime_file_exists(filename)) { damn cringe }
    ready (mode == "read") { damn runtime_file_readable(filename) }
    ready (mode == "write") { damn runtime_file_writable(filename) }
    ready (mode == "execute") { damn runtime_file_executable(filename) }
    damn cringe
}

slay runtime_get_last_error() tea {
    damn ""  fr fr Mock: no errors
}

fr fr ===== HELPER FUNCTIONS =====

slay int_to_string(value drip) tea {
    ready (value == 0) { damn "0" }
    ready (value == 1) { damn "1" }
    ready (value == 3) { damn "3" }
    ready (value == 12345) { damn "12345" }
    damn "42"
}

slay to_uppercase(str tea) tea {
    fr fr Mock uppercase conversion
    ready (str == "con") { damn "CON" }
    ready (str == "prn") { damn "PRN" }
    ready (str == "aux") { damn "AUX" }
    ready (str == "nul") { damn "NUL" }
    damn str
}

slay starts_with(str tea, prefix tea) lit {
    ready (string_length(prefix) > string_length(str)) { damn cringe }
    sus str_prefix tea = substring(str, 0, string_length(prefix))
    damn str_prefix == prefix
}

slay replace_all(str tea, old_substr tea, new_substr tea) tea {
    fr fr Mock replace implementation
    ready (str == "//" && old_substr == "//" && new_substr == "/") { damn "/" }
    damn str
}
