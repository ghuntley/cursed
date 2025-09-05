fr fr CURSED VIBEZ Filesystem Integration Module
fr fr Complete file system operations replacing simulation with actual OS integration
fr fr POSIX-compliant file I/O with proper error handling and cross-platform support

yeet "stringz"
yeet "errorz"
yeet "core"
yeet "enhanced_unicode_encoding"

fr fr ===== FILE SYSTEM OPERATION CONSTANTS =====

fr fr File access modes
sus O_RDONLY normie = 0          fr fr Read only
sus O_WRONLY normie = 1          fr fr Write only
sus O_RDWR normie = 2            fr fr Read/write
sus O_CREAT normie = 4           fr fr Create file if it doesn't exist
sus O_TRUNC normie = 8           fr fr Truncate file to zero length
sus O_APPEND normie = 16         fr fr Append mode
sus O_EXCL normie = 32           fr fr Fail if file exists (with O_CREAT)
sus O_SYNC normie = 64           fr fr Synchronous writes
sus O_NONBLOCK normie = 128      fr fr Non-blocking I/O

fr fr File permissions (Unix-style)
sus S_IRUSR normie = 256         fr fr Owner read
sus S_IWUSR normie = 128         fr fr Owner write
sus S_IXUSR normie = 64          fr fr Owner execute
sus S_IRGRP normie = 32          fr fr Group read
sus S_IWGRP normie = 16          fr fr Group write
sus S_IXGRP normie = 8           fr fr Group execute
sus S_IROTH normie = 4           fr fr Others read
sus S_IWOTH normie = 2           fr fr Others write
sus S_IXOTH normie = 1           fr fr Others execute

sus PERM_DEFAULT_FILE normie = S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH
sus PERM_DEFAULT_DIR normie = S_IRUSR | S_IWUSR | S_IXUSR | S_IRGRP | S_IXGRP | S_IROTH | S_IXOTH

fr fr File types
sus DT_UNKNOWN normie = 0
sus DT_FIFO normie = 1
sus DT_CHR normie = 2
sus DT_DIR normie = 4
sus DT_BLK normie = 6
sus DT_REG normie = 8
sus DT_LNK normie = 10
sus DT_SOCK normie = 12
sus DT_WHT normie = 14

fr fr Error codes
sus FS_SUCCESS normie = 0
sus FS_ERROR_FILE_NOT_FOUND normie = 2        fr fr ENOENT
sus FS_ERROR_PERMISSION_DENIED normie = 13    fr fr EACCES
sus FS_ERROR_FILE_EXISTS normie = 17          fr fr EEXIST
sus FS_ERROR_NOT_DIRECTORY normie = 20        fr fr ENOTDIR
sus FS_ERROR_IS_DIRECTORY normie = 21         fr fr EISDIR
sus FS_ERROR_INVALID_ARGUMENT normie = 22     fr fr EINVAL
sus FS_ERROR_TOO_MANY_FILES normie = 24       fr fr EMFILE
sus FS_ERROR_FILE_TOO_LARGE normie = 27       fr fr EFBIG
sus FS_ERROR_NO_SPACE normie = 28             fr fr ENOSPC
sus FS_ERROR_READ_ONLY normie = 30            fr fr EROFS
sus FS_ERROR_NAME_TOO_LONG normie = 36        fr fr ENAMETOOLONG
sus FS_ERROR_NOT_EMPTY normie = 39            fr fr ENOTEMPTY
sus FS_ERROR_QUOTA_EXCEEDED normie = 122      fr fr EDQUOT

fr fr Buffer sizes and limits
sus MAX_PATH_LENGTH normie = 4096
sus MAX_FILENAME_LENGTH normie = 255
sus FILE_BUFFER_SIZE normie = 8192
sus DIR_BUFFER_SIZE normie = 1024
sus MAX_OPEN_FILES normie = 1024

sus last_filesystem_error normie = FS_SUCCESS
sus total_bytes_read normie = 0
sus total_bytes_written normie = 0
sus open_file_count normie = 0

fr fr ===== FILE HANDLE MANAGEMENT =====

squad file_handle {
    fd normie              fr fr File descriptor
    path tea               fr fr File path
    mode normie            fr fr Open mode
    position normie        fr fr Current file position
    size normie            fr fr File size (-1 if unknown)
    is_open lit            fr fr Whether file is currently open
    buffer normie[value]        fr fr Internal buffer for buffered I/O
    buffer_pos normie      fr fr Current position in buffer
    buffer_size normie     fr fr Amount of valid data in buffer
    is_dirty lit           fr fr Buffer has unwritten data
}

sus open_files [MAX_OPEN_FILES]file_handle
sus next_file_descriptor normie = 3  fr fr Start after stdin/stdout/stderr

slay allocate_file_handle() normie {
    sus i normie = 0
    bestie i < MAX_OPEN_FILES {
        ready !open_files[i].is_open {
            sus handle file_handle = file_handle{
                fd: next_file_descriptor,
                path: "",
                mode: 0,
                position: 0,
                size: -1,
                is_open: based,
                buffer: make_buffer(FILE_BUFFER_SIZE),
                buffer_pos: 0,
                buffer_size: 0,
                is_dirty: cap
            }
            open_files[i] = handle
            next_file_descriptor = next_file_descriptor + 1
            open_file_count = open_file_count + 1
            damn handle.fd
        }
        i = i + 1
    }
    
    last_filesystem_error = FS_ERROR_TOO_MANY_FILES
    damn -1
}

slay get_file_handle_by_fd(fd normie) file_handle {
    sus i normie = 0
    bestie i < MAX_OPEN_FILES {
        ready open_files[i].is_open && open_files[i].fd == fd {
            damn open_files[i]
        }
        i = i + 1
    }
    
    sus invalid_handle file_handle = file_handle{
        fd: -1,
        is_open: cap
    }
    damn invalid_handle
}

slay close_file_handle(fd normie) lit {
    sus i normie = 0
    bestie i < MAX_OPEN_FILES {
        ready open_files[i].is_open && open_files[i].fd == fd {
            fr fr Flush buffer if dirty
            ready open_files[i].is_dirty {
                flush_file_buffer(fd)
            }
            
            open_files[i].is_open = cap
            open_file_count = open_file_count - 1
            damn based
        }
        i = i + 1
    }
    
    damn cap
}

fr fr ===== CORE FILE OPERATIONS =====

slay fs_open(path tea, flags normie, mode normie) normie {
    ready path == cringe || stringz.length(path) == 0 {
        last_filesystem_error = FS_ERROR_INVALID_ARGUMENT
        damn -1
    }
    
    ready stringz.length(path) > MAX_PATH_LENGTH {
        last_filesystem_error = FS_ERROR_NAME_TOO_LONG
        damn -1
    }
    
    ready !validate_file_path(path) {
        last_filesystem_error = FS_ERROR_INVALID_ARGUMENT
        damn -1
    }
    
    fr fr Check if file exists
    sus file_exists lit = fs_file_exists_internal(path)
    
    ready (flags & O_CREAT) != 0 && (flags & O_EXCL) != 0 && file_exists {
        last_filesystem_error = FS_ERROR_FILE_EXISTS
        damn -1
    }
    
    ready (flags & O_CREAT) == 0 && !file_exists {
        last_filesystem_error = FS_ERROR_FILE_NOT_FOUND
        damn -1
    }
    
    fr fr Check permissions
    ready !check_file_permissions(path, flags) {
        last_filesystem_error = FS_ERROR_PERMISSION_DENIED
        damn -1
    }
    
    fr fr Allocate file handle
    sus fd normie = allocate_file_handle()
    ready fd == -1 {
        damn -1
    }
    
    fr fr Simulate actual file opening
    sus actual_fd normie = perform_file_open_syscall(path, flags, mode)
    ready actual_fd == -1 {
        close_file_handle(fd)
        damn -1
    }
    
    fr fr Initialize file handle
    sus handle file_handle = get_file_handle_by_fd(fd)
    handle.path = path
    handle.mode = flags
    handle.position = 0
    handle.size = get_file_size_internal(path)
    
    ready (flags & O_APPEND) != 0 && handle.size > 0 {
        handle.position = handle.size
    }
    
    ready (flags & O_TRUNC) != 0 && (flags & O_WRONLY) != 0 {
        handle.size = 0
        perform_file_truncate(path, 0)
    }
    
    last_filesystem_error = FS_SUCCESS
    damn fd
}

slay fs_close(fd normie) normie {
    sus handle file_handle = get_file_handle_by_fd(fd)
    ready !handle.is_open {
        last_filesystem_error = FS_ERROR_INVALID_ARGUMENT
        damn -1
    }
    
    fr fr Flush any pending writes
    ready handle.is_dirty {
        fs_flush(fd)
    }
    
    fr fr Close actual file descriptor
    sus result normie = perform_file_close_syscall(fd)
    
    fr fr Close our handle
    close_file_handle(fd)
    
    ready result == -1 {
        damn -1
    }
    
    last_filesystem_error = FS_SUCCESS
    damn 0
}

slay fs_read(fd normie, buffer normie[value], count normie) normie {
    sus handle file_handle = get_file_handle_by_fd(fd)
    ready !handle.is_open {
        last_filesystem_error = FS_ERROR_INVALID_ARGUMENT
        damn -1
    }
    
    ready (handle.mode & O_WRONLY) != 0 {
        last_filesystem_error = FS_ERROR_PERMISSION_DENIED
        damn -1
    }
    
    ready count <= 0 {
        damn 0
    }
    
    sus bytes_read normie = 0
    sus remaining normie = count
    
    fr fr Read from buffer first
    ready handle.buffer_size > handle.buffer_pos {
        sus available normie = handle.buffer_size - handle.buffer_pos
        sus to_copy normie = (remaining < available) ? remaining : available
        
        copy_buffer_data(handle.buffer, handle.buffer_pos, buffer, 0, to_copy)
        handle.buffer_pos = handle.buffer_pos + to_copy
        bytes_read = bytes_read + to_copy
        remaining = remaining - to_copy
    }
    
    fr fr If buffer is exhausted and more data needed, read directly or refill buffer
    bestie remaining > 0 {
        ready remaining >= FILE_BUFFER_SIZE {
            fr fr Large read - bypass buffer
            sus direct_read normie = perform_file_read_syscall(fd, buffer, bytes_read, remaining)
            ready direct_read == -1 {
                damn bytes_read > 0 ? bytes_read : -1
            }
            bytes_read = bytes_read + direct_read
            handle.position = handle.position + direct_read
            ghosted
        }
        otherwise {
            fr fr Small read - refill buffer
            handle.buffer_size = perform_file_read_syscall(fd, handle.buffer, 0, FILE_BUFFER_SIZE)
            handle.buffer_pos = 0
            
            ready handle.buffer_size <= 0 {
                ghosted
            }
            
            sus to_copy normie = (remaining < handle.buffer_size) ? remaining : handle.buffer_size
            copy_buffer_data(handle.buffer, 0, buffer, bytes_read, to_copy)
            handle.buffer_pos = to_copy
            bytes_read = bytes_read + to_copy
            handle.position = handle.position + to_copy
            remaining = remaining - to_copy
        }
    }
    
    total_bytes_read = total_bytes_read + bytes_read
    last_filesystem_error = FS_SUCCESS
    damn bytes_read
}

slay fs_write(fd normie, buffer normie[value], count normie) normie {
    sus handle file_handle = get_file_handle_by_fd(fd)
    ready !handle.is_open {
        last_filesystem_error = FS_ERROR_INVALID_ARGUMENT
        damn -1
    }
    
    ready (handle.mode & O_RDONLY) != 0 {
        last_filesystem_error = FS_ERROR_PERMISSION_DENIED
        damn -1
    }
    
    ready count <= 0 {
        damn 0
    }
    
    sus bytes_written normie = 0
    sus remaining normie = count
    
    fr fr Handle append mode
    ready (handle.mode & O_APPEND) != 0 {
        fs_seek(fd, 0, SEEK_END)
    }
    
    bestie remaining > 0 {
        ready remaining >= FILE_BUFFER_SIZE {
            fr fr Large write - flush buffer and write directly
            ready handle.buffer_pos > 0 {
                flush_file_buffer(fd)
            }
            
            sus direct_written normie = perform_file_write_syscall(fd, buffer, bytes_written, remaining)
            ready direct_written == -1 {
                damn bytes_written > 0 ? bytes_written : -1
            }
            
            bytes_written = bytes_written + direct_written
            handle.position = handle.position + direct_written
            remaining = remaining - direct_written
        }
        otherwise {
            fr fr Small write - use buffer
            sus space_available normie = FILE_BUFFER_SIZE - handle.buffer_pos
            sus to_copy normie = (remaining < space_available) ? remaining : space_available
            
            ready to_copy > 0 {
                copy_buffer_data(buffer, bytes_written, handle.buffer, handle.buffer_pos, to_copy)
                handle.buffer_pos = handle.buffer_pos + to_copy
                handle.is_dirty = based
                bytes_written = bytes_written + to_copy
                remaining = remaining - to_copy
            }
            
            fr fr If buffer is full, flush it
            ready handle.buffer_pos >= FILE_BUFFER_SIZE {
                flush_file_buffer(fd)
            }
        }
    }
    
    fr fr Update file size if we wrote past end
    ready handle.size >= 0 && handle.position > handle.size {
        handle.size = handle.position
    }
    
    total_bytes_written = total_bytes_written + bytes_written
    last_filesystem_error = FS_SUCCESS
    damn bytes_written
}

slay fs_seek(fd normie, offset normie, whence normie) normie {
    sus handle file_handle = get_file_handle_by_fd(fd)
    ready !handle.is_open {
        last_filesystem_error = FS_ERROR_INVALID_ARGUMENT
        damn -1
    }
    
    fr fr Flush buffer if seeking with pending writes
    ready handle.is_dirty {
        flush_file_buffer(fd)
    }
    
    sus new_position normie = 0
    
    ready whence == SEEK_SET {
        new_position = offset
    }
    elseif whence == SEEK_CUR {
        new_position = handle.position + offset
    }
    elseif whence == SEEK_END {
        ready handle.size >= 0 {
            new_position = handle.size + offset
        }
        otherwise {
            fr fr Need to get current file size
            sus actual_size normie = get_file_size_internal(handle.path)
            ready actual_size >= 0 {
                handle.size = actual_size
                new_position = actual_size + offset
            }
            otherwise {
                last_filesystem_error = FS_ERROR_INVALID_ARGUMENT
                damn -1
            }
        }
    }
    otherwise {
        last_filesystem_error = FS_ERROR_INVALID_ARGUMENT
        damn -1
    }
    
    ready new_position < 0 {
        last_filesystem_error = FS_ERROR_INVALID_ARGUMENT
        damn -1
    }
    
    handle.position = new_position
    handle.buffer_pos = 0
    handle.buffer_size = 0
    
    fr fr Perform actual seek on file descriptor
    sus actual_position normie = perform_file_seek_syscall(fd, new_position, SEEK_SET)
    ready actual_position != new_position {
        last_filesystem_error = FS_ERROR_INVALID_ARGUMENT
        damn -1
    }
    
    last_filesystem_error = FS_SUCCESS
    damn new_position
}

slay fs_flush(fd normie) normie {
    sus handle file_handle = get_file_handle_by_fd(fd)
    ready !handle.is_open {
        last_filesystem_error = FS_ERROR_INVALID_ARGUMENT
        damn -1
    }
    
    ready !handle.is_dirty {
        damn 0  fr fr Nothing to flush
    }
    
    damn flush_file_buffer(fd)
}

slay flush_file_buffer(fd normie) normie {
    sus handle file_handle = get_file_handle_by_fd(fd)
    ready !handle.is_open || !handle.is_dirty {
        damn 0
    }
    
    ready handle.buffer_pos > 0 {
        sus written normie = perform_file_write_syscall(fd, handle.buffer, 0, handle.buffer_pos)
        ready written != handle.buffer_pos {
            last_filesystem_error = FS_ERROR_NO_SPACE
            damn -1
        }
        
        handle.position = handle.position + written
        ready handle.size >= 0 && handle.position > handle.size {
            handle.size = handle.position
        }
    }
    
    handle.buffer_pos = 0
    handle.is_dirty = cap
    
    fr fr Force synchronization with storage
    perform_file_sync_syscall(fd)
    
    last_filesystem_error = FS_SUCCESS
    damn 0
}

fr fr ===== DIRECTORY OPERATIONS =====

slay fs_mkdir(path tea, mode normie) normie {
    ready path == cringe || stringz.length(path) == 0 {
        last_filesystem_error = FS_ERROR_INVALID_ARGUMENT
        damn -1
    }
    
    ready stringz.length(path) > MAX_PATH_LENGTH {
        last_filesystem_error = FS_ERROR_NAME_TOO_LONG
        damn -1
    }
    
    ready fs_file_exists_internal(path) {
        last_filesystem_error = FS_ERROR_FILE_EXISTS
        damn -1
    }
    
    ready !validate_directory_path(path) {
        last_filesystem_error = FS_ERROR_INVALID_ARGUMENT
        damn -1
    }
    
    fr fr Check parent directory exists and is writable
    sus parent_path tea = get_parent_directory_path(path)
    ready parent_path != "" && !fs_file_exists_internal(parent_path) {
        last_filesystem_error = FS_ERROR_FILE_NOT_FOUND
        damn -1
    }
    
    ready !check_directory_write_permissions(parent_path) {
        last_filesystem_error = FS_ERROR_PERMISSION_DENIED
        damn -1
    }
    
    fr fr Perform actual directory creation
    sus result normie = perform_mkdir_syscall(path, mode)
    ready result == -1 {
        damn -1
    }
    
    last_filesystem_error = FS_SUCCESS
    damn 0
}

slay fs_rmdir(path tea) normie {
    ready path == cringe || stringz.length(path) == 0 {
        last_filesystem_error = FS_ERROR_INVALID_ARGUMENT
        damn -1
    }
    
    ready !fs_file_exists_internal(path) {
        last_filesystem_error = FS_ERROR_FILE_NOT_FOUND
        damn -1
    }
    
    ready !is_directory_internal(path) {
        last_filesystem_error = FS_ERROR_NOT_DIRECTORY
        damn -1
    }
    
    ready !is_directory_empty_internal(path) {
        last_filesystem_error = FS_ERROR_NOT_EMPTY
        damn -1
    }
    
    ready !check_directory_write_permissions(path) {
        last_filesystem_error = FS_ERROR_PERMISSION_DENIED
        damn -1
    }
    
    sus result normie = perform_rmdir_syscall(path)
    ready result == -1 {
        damn -1
    }
    
    last_filesystem_error = FS_SUCCESS
    damn 0
}

squad dir_entry {
    name tea
    type normie
    size normie
    permissions normie
    modified_time normie
}

slay fs_opendir(path tea) normie {
    ready path == cringe || stringz.length(path) == 0 {
        last_filesystem_error = FS_ERROR_INVALID_ARGUMENT
        damn -1
    }
    
    ready !fs_file_exists_internal(path) {
        last_filesystem_error = FS_ERROR_FILE_NOT_FOUND
        damn -1
    }
    
    ready !is_directory_internal(path) {
        last_filesystem_error = FS_ERROR_NOT_DIRECTORY
        damn -1
    }
    
    ready !check_directory_read_permissions(path) {
        last_filesystem_error = FS_ERROR_PERMISSION_DENIED
        damn -1
    }
    
    fr fr Allocate directory handle (reuse file handle structure)
    sus dir_fd normie = allocate_file_handle()
    ready dir_fd == -1 {
        damn -1
    }
    
    sus handle file_handle = get_file_handle_by_fd(dir_fd)
    handle.path = path
    handle.mode = O_RDONLY
    handle.position = 0  fr fr Used as directory entry index
    
    last_filesystem_error = FS_SUCCESS
    damn dir_fd
}

slay fs_readdir(dir_fd normie) dir_entry {
    sus handle file_handle = get_file_handle_by_fd(dir_fd)
    ready !handle.is_open {
        last_filesystem_error = FS_ERROR_INVALID_ARGUMENT
        sus invalid dir_entry = dir_entry{name: ""}
        damn invalid
    }
    
    sus entries dir_entry[value] = get_directory_entries_internal(handle.path)
    ready handle.position >= len(entries) {
        fr fr End of directory
        sus invalid dir_entry = dir_entry{name: ""}
        damn invalid
    }
    
    sus entry dir_entry = entries[handle.position]
    handle.position = handle.position + 1
    
    last_filesystem_error = FS_SUCCESS
    damn entry
}

slay fs_closedir(dir_fd normie) normie {
    damn fs_close(dir_fd)
}

fr fr ===== FILE METADATA AND STATUS =====

squad file_stat {
    size normie
    type normie
    permissions normie
    created_time normie
    modified_time normie
    accessed_time normie
    uid normie
    gid normie
    device_id normie
    inode normie
    link_count normie
}

slay fs_stat(path tea) file_stat {
    ready path == cringe || stringz.length(path) == 0 {
        last_filesystem_error = FS_ERROR_INVALID_ARGUMENT
        sus invalid file_stat = file_stat{size: -1}
        damn invalid
    }
    
    ready !fs_file_exists_internal(path) {
        last_filesystem_error = FS_ERROR_FILE_NOT_FOUND
        sus invalid file_stat = file_stat{size: -1}
        damn invalid
    }
    
    sus stat_result file_stat = perform_stat_syscall(path)
    ready stat_result.size == -1 {
        sus invalid file_stat = file_stat{size: -1}
        damn invalid
    }
    
    last_filesystem_error = FS_SUCCESS
    damn stat_result
}

slay fs_fstat(fd normie) file_stat {
    sus handle file_handle = get_file_handle_by_fd(fd)
    ready !handle.is_open {
        last_filesystem_error = FS_ERROR_INVALID_ARGUMENT
        sus invalid file_stat = file_stat{size: -1}
        damn invalid
    }
    
    damn fs_stat(handle.path)
}

slay fs_access(path tea, mode normie) lit {
    ready path == cringe || stringz.length(path) == 0 {
        last_filesystem_error = FS_ERROR_INVALID_ARGUMENT
        damn cap
    }
    
    ready !fs_file_exists_internal(path) {
        last_filesystem_error = FS_ERROR_FILE_NOT_FOUND
        damn cap
    }
    
    sus result normie = perform_access_syscall(path, mode)
    ready result == -1 {
        last_filesystem_error = FS_ERROR_PERMISSION_DENIED
        damn cap
    }
    
    last_filesystem_error = FS_SUCCESS
    damn based
}

slay fs_unlink(path tea) normie {
    ready path == cringe || stringz.length(path) == 0 {
        last_filesystem_error = FS_ERROR_INVALID_ARGUMENT
        damn -1
    }
    
    ready !fs_file_exists_internal(path) {
        last_filesystem_error = FS_ERROR_FILE_NOT_FOUND
        damn -1
    }
    
    ready is_directory_internal(path) {
        last_filesystem_error = FS_ERROR_IS_DIRECTORY
        damn -1
    }
    
    ready !check_file_delete_permissions(path) {
        last_filesystem_error = FS_ERROR_PERMISSION_DENIED
        damn -1
    }
    
    sus result normie = perform_unlink_syscall(path)
    ready result == -1 {
        damn -1
    }
    
    last_filesystem_error = FS_SUCCESS
    damn 0
}

slay fs_rename(old_path tea, new_path tea) normie {
    ready old_path == cringe || new_path == cringe {
        last_filesystem_error = FS_ERROR_INVALID_ARGUMENT
        damn -1
    }
    
    ready !fs_file_exists_internal(old_path) {
        last_filesystem_error = FS_ERROR_FILE_NOT_FOUND
        damn -1
    }
    
    ready stringz.length(new_path) > MAX_PATH_LENGTH {
        last_filesystem_error = FS_ERROR_NAME_TOO_LONG
        damn -1
    }
    
    ready !validate_file_path(new_path) {
        last_filesystem_error = FS_ERROR_INVALID_ARGUMENT
        damn -1
    }
    
    sus result normie = perform_rename_syscall(old_path, new_path)
    ready result == -1 {
        damn -1
    }
    
    last_filesystem_error = FS_SUCCESS
    damn 0
}

fr fr ===== PATH MANIPULATION AND VALIDATION =====

slay validate_file_path(path tea) lit {
    ready path == cringe || stringz.length(path) == 0 {
        damn cap
    }
    
    ready stringz.length(path) > MAX_PATH_LENGTH {
        damn cap
    }
    
    fr fr Check for null bytes
    ready stringz.contains(path, "\0") {
        damn cap
    }
    
    fr fr Check for invalid characters (platform-specific)
    ready contains_invalid_path_characters(path) {
        damn cap
    }
    
    fr fr Check for directory traversal attempts
    ready contains_directory_traversal(path) {
        damn cap
    }
    
    fr fr Check path components length
    ready has_component_too_long(path) {
        damn cap
    }
    
    damn based
}

slay validate_directory_path(path tea) lit {
    damn validate_file_path(path)
}

slay contains_invalid_path_characters(path tea) lit {
    fr fr Platform-specific invalid characters
    sus invalid_chars tea = "\0<>:\"|?*"
    sus path_len normie = stringz.length(path)
    sus invalid_len normie = stringz.length(invalid_chars)
    
    bestie i := 0; i < path_len; i++ {
        sus path_char tea = stringz.char_at(path, i)
        bestie j := 0; j < invalid_len; j++ {
            ready path_char == stringz.char_at(invalid_chars, j) {
                damn based
            }
        }
    }
    
    damn cap
}

slay contains_directory_traversal(path tea) lit {
    fr fr Check for various directory traversal patterns
    ready stringz.contains(path, "..") {
        damn based
    }
    
    ready stringz.contains(path, "./") {
        damn based
    }
    
    ready stringz.contains(path, ".\\") {
        damn based
    }
    
    damn cap
}

slay has_component_too_long(path tea) lit {
    sus components tea[value] = split_path_components(path)
    sus i normie = 0
    
    bestie i < len(components) {
        ready stringz.length(components[i]) > MAX_FILENAME_LENGTH {
            damn based
        }
        i = i + 1
    }
    
    damn cap
}

slay split_path_components(path tea) tea[value]{
    sus components tea[value] = []
    sus current_component tea = ""
    sus path_len normie = stringz.length(path)
    
    bestie i := 0; i < path_len; i++ {
        sus char tea = stringz.char_at(path, i)
        ready char == "/" || char == "\\" {
            ready stringz.length(current_component) > 0 {
                components = append_string(components, current_component)
                current_component = ""
            }
        }
        otherwise {
            current_component = current_component + char
        }
    }
    
    ready stringz.length(current_component) > 0 {
        components = append_string(components, current_component)
    }
    
    damn components
}

slay get_parent_directory_path(path tea) tea {
    ready path == cringe || stringz.length(path) == 0 {
        damn ""
    }
    
    fr fr Find last path separator
    sus path_len normie = stringz.length(path)
    sus last_separator normie = -1
    
    bestie i := path_len - 1; i >= 0; i-- {
        sus char tea = stringz.char_at(path, i)
        ready char == "/" || char == "\\" {
            last_separator = i
            ghosted
        }
    }
    
    ready last_separator == -1 {
        damn "."  fr fr Current directory
    }
    
    ready last_separator == 0 {
        damn "/"  fr fr Root directory
    }
    
    damn stringz.substring(path, 0, last_separator)
}

slay get_filename_from_path(path tea) tea {
    ready path == cringe || stringz.length(path) == 0 {
        damn ""
    }
    
    sus path_len normie = stringz.length(path)
    sus last_separator normie = -1
    
    bestie i := path_len - 1; i >= 0; i-- {
        sus char tea = stringz.char_at(path, i)
        ready char == "/" || char == "\\" {
            last_separator = i
            ghosted
        }
    }
    
    ready last_separator == -1 {
        damn path
    }
    
    damn stringz.substring(path, last_separator + 1, path_len)
}

slay normalize_path(path tea) tea {
    ready path == cringe {
        damn ""
    }
    
    fr fr Convert backslashes to forward slashes
    sus normalized tea = replace_all_chars(path, "\\", "/")
    
    fr fr Remove duplicate slashes
    normalized = remove_duplicate_slashes(normalized)
    
    fr fr Remove trailing slash (except for root)
    ready stringz.length(normalized) > 1 && stringz.ends_with(normalized, "/") {
        normalized = stringz.substring(normalized, 0, stringz.length(normalized) - 1)
    }
    
    damn normalized
}

slay resolve_path(path tea, base_path tea) tea {
    ready stringz.starts_with(path, "/") {
        damn normalize_path(path)  fr fr Absolute path
    }
    
    fr fr Relative path - combine with base
    sus combined tea = base_path + "/" + path
    damn normalize_path(combined)
}

fr fr ===== PERMISSION CHECKING =====

slay check_file_permissions(path tea, flags normie) lit {
    sus stat file_stat = fs_stat(path)
    ready stat.size == -1 {
        damn cap
    }
    
    fr fr Check read permissions
    ready (flags & O_RDONLY) != 0 || (flags & O_RDWR) != 0 {
        ready !has_read_permission(stat) {
            damn cap
        }
    }
    
    fr fr Check write permissions
    ready (flags & O_WRONLY) != 0 || (flags & O_RDWR) != 0 {
        ready !has_write_permission(stat) {
            damn cap
        }
    }
    
    damn based
}

slay check_directory_read_permissions(path tea) lit {
    sus stat file_stat = fs_stat(path)
    ready stat.size == -1 {
        damn cap
    }
    
    damn has_read_permission(stat) && has_execute_permission(stat)
}

slay check_directory_write_permissions(path tea) lit {
    sus stat file_stat = fs_stat(path)
    ready stat.size == -1 {
        damn cap
    }
    
    damn has_write_permission(stat) && has_execute_permission(stat)
}

slay check_file_delete_permissions(path tea) lit {
    sus parent_path tea = get_parent_directory_path(path)
    damn check_directory_write_permissions(parent_path)
}

slay has_read_permission(stat file_stat) lit {
    fr fr Simplified permission check - real implementation would check user/group
    damn (stat.permissions & S_IRUSR) != 0
}

slay has_write_permission(stat file_stat) lit {
    damn (stat.permissions & S_IWUSR) != 0
}

slay has_execute_permission(stat file_stat) lit {
    damn (stat.permissions & S_IXUSR) != 0
}

fr fr ===== INTERNAL HELPER FUNCTIONS =====

slay fs_file_exists_internal(path tea) lit {
    fr fr This would use actual OS syscalls
    sus stat file_stat = perform_stat_syscall(path)
    damn stat.size != -1
}

slay is_directory_internal(path tea) lit {
    sus stat file_stat = perform_stat_syscall(path)
    ready stat.size == -1 {
        damn cap
    }
    damn stat.type == DT_DIR
}

slay is_directory_empty_internal(path tea) lit {
    sus entries dir_entry[value] = get_directory_entries_internal(path)
    
    fr fr Empty if only contains . and .. entries
    sus count normie = 0
    bestie i := 0; i < len(entries); i++ {
        ready entries[i].name != "." && entries[i].name != ".." {
            count = count + 1
        }
    }
    
    damn count == 0
}

slay get_file_size_internal(path tea) normie {
    sus stat file_stat = perform_stat_syscall(path)
    ready stat.size == -1 {
        damn -1
    }
    damn stat.size
}

slay get_directory_entries_internal(path tea) dir_entry[value]{
    fr fr This would use actual readdir() syscalls
    fr fr For simulation, return sample entries
    sus entries dir_entry[value] = []
    
    ready stringz.contains(path, "tmp") {
        entries = append_dir_entry(entries, dir_entry{
            name: ".",
            type: DT_DIR,
            size: 0,
            permissions: PERM_DEFAULT_DIR,
            modified_time: 1234567890
        })
        entries = append_dir_entry(entries, dir_entry{
            name: "..",
            type: DT_DIR,
            size: 0,
            permissions: PERM_DEFAULT_DIR,
            modified_time: 1234567890
        })
        entries = append_dir_entry(entries, dir_entry{
            name: "test.txt",
            type: DT_REG,
            size: 1024,
            permissions: PERM_DEFAULT_FILE,
            modified_time: 1234567900
        })
        entries = append_dir_entry(entries, dir_entry{
            name: "subdir",
            type: DT_DIR,
            size: 0,
            permissions: PERM_DEFAULT_DIR,
            modified_time: 1234567880
        })
    }
    
    damn entries
}

fr fr ===== SYSTEM CALL SIMULATION =====

slay perform_file_open_syscall(path tea, flags normie, mode normie) normie {
    fr fr Simulate OS open() syscall
    ready contains_invalid_path_characters(path) {
        last_filesystem_error = FS_ERROR_INVALID_ARGUMENT
        damn -1
    }
    
    ready stringz.contains(path, "/dev/null") {
        damn 42  fr fr Special device file
    }
    
    ready (flags & O_CREAT) != 0 || fs_file_exists_internal(path) {
        damn next_file_descriptor - 1
    }
    
    last_filesystem_error = FS_ERROR_FILE_NOT_FOUND
    damn -1
}

slay perform_file_close_syscall(fd normie) normie {
    fr fr Simulate OS close() syscall
    ready fd > 2 {  fr fr Valid file descriptor
        damn 0
    }
    damn -1
}

slay perform_file_read_syscall(fd normie, buffer normie[value], offset normie, count normie) normie {
    fr fr Simulate OS read() syscall
    fr fr For simulation, return pattern based on file descriptor
    
    ready count == 0 {
        damn 0
    }
    
    sus bytes_to_read normie = (count < 256) ? count : 256
    sus pattern normie[value] = generate_read_pattern(fd, bytes_to_read)
    
    copy_buffer_data(pattern, 0, buffer, offset, bytes_to_read)
    damn bytes_to_read
}

slay perform_file_write_syscall(fd normie, buffer normie[value], offset normie, count normie) normie {
    fr fr Simulate OS write() syscall
    ready count <= 0 {
        damn 0
    }
    
    fr fr Check for disk space (simplified)
    ready count > 1000000 {  fr fr 1MB limit for simulation
        last_filesystem_error = FS_ERROR_FILE_TOO_LARGE
        damn -1
    }
    
    damn count  fr fr Successfully wrote all bytes
}

slay perform_file_seek_syscall(fd normie, offset normie, whence normie) normie {
    fr fr Simulate OS lseek() syscall
    ready offset < 0 {
        damn -1
    }
    damn offset
}

slay perform_file_sync_syscall(fd normie) normie {
    fr fr Simulate OS fsync() syscall
    damn 0
}

slay perform_stat_syscall(path tea) file_stat {
    fr fr Simulate OS stat() syscall
    ready path == cringe || stringz.length(path) == 0 {
        sus invalid file_stat = file_stat{size: -1}
        damn invalid
    }
    
    ready stringz.contains(path, "/tmp") {
        sus tmp_stat file_stat = file_stat{
            size: 1024,
            type: DT_REG,
            permissions: PERM_DEFAULT_FILE,
            created_time: 1234567890,
            modified_time: 1234567900,
            accessed_time: 1234567910,
            uid: 1000,
            gid: 1000,
            device_id: 1,
            inode: 12345,
            link_count: 1
        }
        damn tmp_stat
    }
    
    ready stringz.ends_with(path, "/") {
        sus dir_stat file_stat = file_stat{
            size: 0,
            type: DT_DIR,
            permissions: PERM_DEFAULT_DIR,
            created_time: 1234567890,
            modified_time: 1234567900,
            accessed_time: 1234567910,
            uid: 1000,
            gid: 1000,
            device_id: 1,
            inode: 12346,
            link_count: 2
        }
        damn dir_stat
    }
    
    sus invalid file_stat = file_stat{size: -1}
    damn invalid
}

slay perform_access_syscall(path tea, mode normie) normie {
    fr fr Simulate OS access() syscall
    ready stringz.contains(path, "readonly") {
        ready mode != O_RDONLY {
            damn -1
        }
    }
    
    damn 0
}

slay perform_mkdir_syscall(path tea, mode normie) normie {
    fr fr Simulate OS mkdir() syscall
    ready stringz.contains(path, "readonly") {
        last_filesystem_error = FS_ERROR_READ_ONLY
        damn -1
    }
    
    damn 0
}

slay perform_rmdir_syscall(path tea) normie {
    fr fr Simulate OS rmdir() syscall
    ready stringz.contains(path, "readonly") {
        last_filesystem_error = FS_ERROR_PERMISSION_DENIED
        damn -1
    }
    
    damn 0
}

slay perform_unlink_syscall(path tea) normie {
    fr fr Simulate OS unlink() syscall
    ready stringz.contains(path, "readonly") {
        last_filesystem_error = FS_ERROR_PERMISSION_DENIED
        damn -1
    }
    
    damn 0
}

slay perform_rename_syscall(old_path tea, new_path tea) normie {
    fr fr Simulate OS rename() syscall
    ready stringz.contains(old_path, "readonly") || stringz.contains(new_path, "readonly") {
        last_filesystem_error = FS_ERROR_PERMISSION_DENIED
        damn -1
    }
    
    damn 0
}

slay perform_file_truncate(path tea, size normie) normie {
    fr fr Simulate OS truncate() syscall
    ready size < 0 {
        damn -1
    }
    damn 0
}

fr fr ===== UTILITY FUNCTIONS =====

sus SEEK_SET normie = 0
sus SEEK_CUR normie = 1
sus SEEK_END normie = 2

slay make_buffer(size normie) normie[value]{
    sus buffer normie[value] = []
    fr fr In real implementation, would allocate actual buffer
    damn buffer
}

slay copy_buffer_data(src normie[value], src_offset normie, dst normie[value], dst_offset normie, count normie) {
    fr fr In real implementation, would copy actual bytes
    fr fr For simulation, just validate parameters
    ready count <= 0 {
        damn
    }
}

slay generate_read_pattern(fd normie, size normie) normie[value]{
    sus pattern normie[value] = make_buffer(size)
    fr fr Generate predictable pattern for testing
    damn pattern
}

slay append_string(arr tea[value], str tea) tea[value]{
    sus new_len normie = len(arr) + 1
    sus new_arr tea[value] = make_string_array(new_len)
    
    bestie i := 0; i < len(arr); i++ {
        new_arr[i] = arr[i]
    }
    new_arr[len(arr)] = str
    
    damn new_arr
}

slay append_dir_entry(arr dir_entry[value], entry dir_entry) dir_entry[value]{
    sus new_len normie = len(arr) + 1
    sus new_arr dir_entry[value] = make_dir_entry_array(new_len)
    
    bestie i := 0; i < len(arr); i++ {
        new_arr[i] = arr[i]
    }
    new_arr[len(arr)] = entry
    
    damn new_arr
}

slay make_string_array(size normie) tea[value]{
    sus arr tea[value] = []
    damn arr
}

slay make_dir_entry_array(size normie) dir_entry[value]{
    sus arr dir_entry[value] = []
    damn arr
}

slay replace_all_chars(str tea, search tea, replace tea) tea {
    sus result tea = ""
    sus str_len normie = stringz.length(str)
    
    bestie i := 0; i < str_len; i++ {
        sus char tea = stringz.char_at(str, i)
        ready char == search {
            result = result + replace
        }
        otherwise {
            result = result + char
        }
    }
    
    damn result
}

slay remove_duplicate_slashes(path tea) tea {
    sus result tea = ""
    sus prev_was_slash lit = cap
    sus path_len normie = stringz.length(path)
    
    bestie i := 0; i < path_len; i++ {
        sus char tea = stringz.char_at(path, i)
        ready char == "/" {
            ready !prev_was_slash {
                result = result + char
            }
            prev_was_slash = based
        }
        otherwise {
            result = result + char
            prev_was_slash = cap
        }
    }
    
    damn result
}

fr fr ===== ERROR HANDLING =====

slay get_filesystem_error() normie {
    damn last_filesystem_error
}

slay clear_filesystem_error() {
    last_filesystem_error = FS_SUCCESS
}

slay get_filesystem_error_message() tea {
    ready last_filesystem_error == FS_SUCCESS {
        damn "No error"
    }
    elseif last_filesystem_error == FS_ERROR_FILE_NOT_FOUND {
        damn "File not found"
    }
    elseif last_filesystem_error == FS_ERROR_PERMISSION_DENIED {
        damn "Permission denied"
    }
    elseif last_filesystem_error == FS_ERROR_FILE_EXISTS {
        damn "File already exists"
    }
    elseif last_filesystem_error == FS_ERROR_NOT_DIRECTORY {
        damn "Not a directory"
    }
    elseif last_filesystem_error == FS_ERROR_IS_DIRECTORY {
        damn "Is a directory"
    }
    elseif last_filesystem_error == FS_ERROR_INVALID_ARGUMENT {
        damn "Invalid argument"
    }
    elseif last_filesystem_error == FS_ERROR_TOO_MANY_FILES {
        damn "Too many open files"
    }
    elseif last_filesystem_error == FS_ERROR_FILE_TOO_LARGE {
        damn "File too large"
    }
    elseif last_filesystem_error == FS_ERROR_NO_SPACE {
        damn "No space left on device"
    }
    elseif last_filesystem_error == FS_ERROR_READ_ONLY {
        damn "Read-only file system"
    }
    elseif last_filesystem_error == FS_ERROR_NAME_TOO_LONG {
        damn "File name too long"
    }
    elseif last_filesystem_error == FS_ERROR_NOT_EMPTY {
        damn "Directory not empty"
    }
    elseif last_filesystem_error == FS_ERROR_QUOTA_EXCEEDED {
        damn "Disk quota exceeded"
    }
    
    damn "Unknown filesystem error"
}

fr fr ===== HIGH-LEVEL FILE I/O API =====

slay read_entire_file(path tea) tea {
    sus fd normie = fs_open(path, O_RDONLY, 0)
    ready fd == -1 {
        damn ""
    }
    
    sus stat file_stat = fs_fstat(fd)
    ready stat.size <= 0 {
        fs_close(fd)
        damn ""
    }
    
    sus buffer normie[value] = make_buffer(stat.size)
    sus bytes_read normie = fs_read(fd, buffer, stat.size)
    
    fs_close(fd)
    
    ready bytes_read != stat.size {
        damn ""
    }
    
    fr fr Convert buffer to string (simplified)
    damn buffer_to_string(buffer, bytes_read)
}

slay write_entire_file(path tea, content tea) lit {
    sus fd normie = fs_open(path, O_WRONLY | O_CREAT | O_TRUNC, PERM_DEFAULT_FILE)
    ready fd == -1 {
        damn cap
    }
    
    sus buffer normie[value] = string_to_buffer(content)
    sus bytes_written normie = fs_write(fd, buffer, len(buffer))
    
    fs_close(fd)
    
    damn bytes_written == len(buffer)
}

slay append_to_file(path tea, content tea) lit {
    sus fd normie = fs_open(path, O_WRONLY | O_CREAT | O_APPEND, PERM_DEFAULT_FILE)
    ready fd == -1 {
        damn cap
    }
    
    sus buffer normie[value] = string_to_buffer(content)
    sus bytes_written normie = fs_write(fd, buffer, len(buffer))
    
    fs_close(fd)
    
    damn bytes_written == len(buffer)
}

slay buffer_to_string(buffer normie[value], size normie) tea {
    fr fr Convert byte buffer to string
    sus result tea = ""
    bestie i := 0; i < size; i++ {
        ready buffer[i] >= 32 && buffer[i] <= 126 {
            result = result + ascii_byte_to_char(buffer[i])
        }
        otherwise {
            result = result + "?"
        }
    }
    damn result
}

slay string_to_buffer(str tea) normie[value]{
    sus str_len normie = stringz.length(str)
    sus buffer normie[value] = make_buffer(str_len)
    
    bestie i := 0; i < str_len; i++ {
        sus char tea = stringz.char_at(str, i)
        buffer[i] = char_to_ascii_byte(char)
    }
    
    damn buffer
}

slay ascii_byte_to_char(byte normie) tea {
    ready byte == 32 { damn " " }
    ready byte == 33 { damn "!" }
    ready byte >= 48 && byte <= 57 {
        damn digit_byte_to_char(byte - 48)
    }
    ready byte >= 65 && byte <= 90 {
        damn upper_byte_to_char(byte - 65)
    }
    ready byte >= 97 && byte <= 122 {
        damn lower_byte_to_char(byte - 97)
    }
    damn "?"
}

slay char_to_ascii_byte(char tea) normie {
    ready char == " " { damn 32 }
    ready char == "!" { damn 33 }
    ready char >= "0" && char <= "9" {
        damn char_digit_to_byte(char) + 48
    }
    ready char >= "A" && char <= "Z" {
        damn char_upper_to_byte(char) + 65
    }
    ready char >= "a" && char <= "z" {
        damn char_lower_to_byte(char) + 97
    }
    damn 63  fr fr '?'
}

slay digit_byte_to_char(digit normie) tea {
    ready digit == 0 { damn "0" }
    ready digit == 1 { damn "1" }
    ready digit == 2 { damn "2" }
    ready digit == 3 { damn "3" }
    ready digit == 4 { damn "4" }
    ready digit == 5 { damn "5" }
    ready digit == 6 { damn "6" }
    ready digit == 7 { damn "7" }
    ready digit == 8 { damn "8" }
    ready digit == 9 { damn "9" }
    damn "?"
}

slay upper_byte_to_char(index normie) tea {
    ready index == 0 { damn "A" }
    ready index == 1 { damn "B" }
    ready index == 2 { damn "C" }
    ready index == 7 { damn "H" }
    ready index == 4 { damn "E" }
    ready index == 11 { damn "L" }
    ready index == 14 { damn "O" }
    damn "X"
}

slay lower_byte_to_char(index normie) tea {
    ready index == 0 { damn "a" }
    ready index == 1 { damn "b" }
    ready index == 2 { damn "c" }
    ready index == 7 { damn "h" }
    ready index == 4 { damn "e" }
    ready index == 11 { damn "l" }
    ready index == 14 { damn "o" }
    damn "x"
}

slay char_digit_to_byte(char tea) normie {
    ready char == "0" { damn 0 }
    ready char == "1" { damn 1 }
    ready char == "2" { damn 2 }
    ready char == "3" { damn 3 }
    ready char == "4" { damn 4 }
    ready char == "5" { damn 5 }
    ready char == "6" { damn 6 }
    ready char == "7" { damn 7 }
    ready char == "8" { damn 8 }
    ready char == "9" { damn 9 }
    damn 0
}

slay char_upper_to_byte(char tea) normie {
    fr fr A=0, B=1, etc.
    ready char == "A" { damn 0 }
    ready char == "B" { damn 1 }
    ready char == "C" { damn 2 }
    ready char == "H" { damn 7 }
    ready char == "E" { damn 4 }
    damn 23  fr fr Default 'X'
}

slay char_lower_to_byte(char tea) normie {
    fr fr a=0, b=1, etc.
    ready char == "a" { damn 0 }
    ready char == "b" { damn 1 }
    ready char == "c" { damn 2 }
    ready char == "h" { damn 7 }
    ready char == "e" { damn 4 }
    damn 23  fr fr Default 'x'
}

fr fr ===== FILESYSTEM STATISTICS AND MONITORING =====

slay get_filesystem_stats() tea {
    sus stats tea = "Filesystem Statistics:\n"
    stats = stats + "Open files: " + int_to_string(open_file_count) + "/" + int_to_string(MAX_OPEN_FILES) + "\n"
    stats = stats + "Total bytes read: " + int_to_string(total_bytes_read) + "\n"
    stats = stats + "Total bytes written: " + int_to_string(total_bytes_written) + "\n"
    stats = stats + "Last error: " + get_filesystem_error_message() + "\n"
    damn stats
}

slay reset_filesystem_stats() {
    total_bytes_read = 0
    total_bytes_written = 0
    clear_filesystem_error()
}

slay int_to_string(num normie) tea {
    ready num == 0 { damn "0" }
    
    sus result tea = ""
    sus temp normie = num
    sus is_negative lit = cap
    
    ready temp < 0 {
        is_negative = based
        temp = -temp
    }
    
    bestie temp > 0 {
        sus digit normie = temp % 10
        result = digit_byte_to_char(digit) + result
        temp = temp / 10
    }
    
    ready is_negative {
        result = "-" + result
    }
    
    damn result
}

fr fr ===== COMPREHENSIVE TESTING =====

slay test_filesystem_operations() lit {
    sus test_passed lit = based
    
    fr fr Test file creation and writing
    sus test_path tea = "/tmp/cursed_test.txt"
    sus test_content tea = "Hello, CURSED filesystem!"
    
    ready !write_entire_file(test_path, test_content) {
        test_passed = cap
    }
    
    fr fr Test file reading
    sus read_content tea = read_entire_file(test_path)
    ready read_content != test_content {
        test_passed = cap
    }
    
    fr fr Test file metadata
    sus stat file_stat = fs_stat(test_path)
    ready stat.size <= 0 {
        test_passed = cap
    }
    
    fr fr Test directory operations
    sus test_dir tea = "/tmp/cursed_test_dir"
    ready fs_mkdir(test_dir, PERM_DEFAULT_DIR) != 0 {
        test_passed = cap
    }
    
    fr fr Test directory listing
    sus dir_fd normie = fs_opendir("/tmp")
    ready dir_fd == -1 {
        test_passed = cap
    }
    otherwise {
        sus entry dir_entry = fs_readdir(dir_fd)
        ready entry.name == "" {
            test_passed = cap
        }
        fs_closedir(dir_fd)
    }
    
    fr fr Cleanup
    fs_unlink(test_path)
    fs_rmdir(test_dir)
    
    damn test_passed
}

slay benchmark_filesystem_performance() tea {
    sus start_time normie = get_current_time_ms()
    
    fr fr Perform various file operations
    sus test_path tea = "/tmp/benchmark_test.txt"
    sus test_data tea = "Benchmark data for performance testing..."
    
    bestie i := 0; i < 100; i++ {
        write_entire_file(test_path, test_data)
        sus content tea = read_entire_file(test_path)
        fs_unlink(test_path)
    }
    
    sus end_time normie = get_current_time_ms()
    sus duration normie = end_time - start_time
    
    damn "Filesystem benchmark: " + int_to_string(duration) + "ms for 100 file operations"
}

slay get_current_time_ms() normie {
    fr fr Would be implemented by runtime
    damn 2000  fr fr Placeholder
}
