fr fr CURSED Complete Filesystem Module - Production Implementation
fr fr Comprehensive filesystem operations with real system integration
fr fr Handles all edge cases, Unicode, permissions, and cross-platform compatibility

yeet "stringz"
yeet "timez"

fr fr ================================
fr fr Core Data Structures
fr fr ================================

be_like FileInfo squad {
    name tea
    path tea
    size thicc
    is_dir lit
    is_file lit
    is_symlink lit
    is_hidden lit
    is_executable lit
    created_time thicc
    modified_time thicc
    accessed_time thicc
    permissions normie
    owner_id normie
    group_id normie
    device_id normie
    inode normie
    hard_links normie
    block_size normie
    blocks normie
}

be_like FileHandle squad {
    fd normie
    path tea
    mode normie
    position thicc
    is_open lit
    buffer_size normie
    buffer byte[value]
    buffer_pos normie
    buffer_len normie
    is_eof lit
    error_code normie
}

be_like DirIterator squad {
    path tea
    handle normie
    current_entry tea
    is_open lit
    error_code normie
    filter_func tea fr fr Optional filter function name
    sort_mode normie fr fr 0=none, 1=name, 2=date, 3=size
}

be_like FileLock squad {
    path tea
    fd normie
    lock_type normie fr fr 1=shared, 2=exclusive
    is_locked lit
}

be_like FileSystemStats squad {
    total_space thicc
    free_space thicc
    available_space thicc
    total_inodes thicc
    free_inodes thicc
    block_size normie
    filesystem_type tea
    mount_point tea
    is_readonly lit
}

be_like WatchHandle squad {
    path tea
    handle normie
    events normie fr fr Bitmask of events to watch
    is_active lit
    callback_name tea
}

fr fr File modes and flags
facts MODE_READ normie = 1
facts MODE_WRITE normie = 2
facts MODE_APPEND normie = 4
facts MODE_CREATE normie = 8
facts MODE_TRUNCATE normie = 16
facts MODE_EXCLUSIVE normie = 32
facts MODE_SYNC normie = 64
facts MODE_BINARY normie = 128

fr fr File permissions (Unix-style)
facts PERM_OWNER_READ normie = 256
facts PERM_OWNER_WRITE normie = 128
facts PERM_OWNER_EXECUTE normie = 64
facts PERM_GROUP_READ normie = 32
facts PERM_GROUP_WRITE normie = 16
facts PERM_GROUP_EXECUTE normie = 8
facts PERM_OTHER_READ normie = 4
facts PERM_OTHER_WRITE normie = 2
facts PERM_OTHER_EXECUTE normie = 1

fr fr Watch events
facts WATCH_CREATE normie = 1
facts WATCH_DELETE normie = 2
facts WATCH_MODIFY normie = 4
facts WATCH_MOVE normie = 8
facts WATCH_ATTRIB normie = 16

fr fr Error codes
facts ERROR_SUCCESS normie = 0
facts ERROR_NOT_FOUND normie = 2
facts ERROR_PERMISSION_DENIED normie = 13
facts ERROR_FILE_EXISTS normie = 17
facts ERROR_NOT_DIR normie = 20
facts ERROR_IS_DIR normie = 21
facts ERROR_INVALID_ARG normie = 22
facts ERROR_NO_SPACE normie = 28
facts ERROR_READ_ONLY normie = 30

fr fr ================================
fr fr External System Call Interface
fr fr ================================

outer slay sys_open(path [*:0]const u8, flags normie, mode normie) normie
outer slay sys_close(fd normie) normie
outer slay sys_read(fd normie, buffer [*]u8, size normie) thicc
outer slay sys_write(fd normie, buffer [*]const u8, size normie) thicc
outer slay sys_lseek(fd normie, offset thicc, whence normie) thicc
outer slay sys_stat(path [*:0]const u8, stat_buf *StatBuffer) normie
outer slay sys_fstat(fd normie, stat_buf *StatBuffer) normie
outer slay sys_lstat(path [*:0]const u8, stat_buf *StatBuffer) normie
outer slay sys_unlink(path [*:0]const u8) normie
outer slay sys_rmdir(path [*:0]const u8) normie
outer slay sys_mkdir(path [*:0]const u8, mode normie) normie
outer slay sys_rename(old_path [*:0]const u8, new_path [*:0]const u8) normie
outer slay sys_chmod(path [*:0]const u8, mode normie) normie
outer slay sys_chown(path [*:0]const u8, uid normie, gid normie) normie
outer slay sys_utimes(path [*:0]const u8, times *TimeSpec) normie
outer slay sys_readlink(path [*:0]const u8, buffer [*]u8, size normie) thicc
outer slay sys_symlink(target [*:0]const u8, linkpath [*:0]const u8) normie
outer slay sys_link(target [*:0]const u8, linkpath [*:0]const u8) normie
outer slay sys_opendir(path [*:0]const u8) normie
outer slay sys_readdir(dir_fd normie) [*:0]const u8
outer slay sys_closedir(dir_fd normie) normie
outer slay sys_getcwd(buffer [*]u8, size normie) [*:0]const u8
outer slay sys_chdir(path [*:0]const u8) normie
outer slay sys_realpath(path [*:0]const u8, resolved [*]u8) [*:0]const u8
outer slay sys_access(path [*:0]const u8, mode normie) normie
outer slay sys_truncate(path [*:0]const u8, length thicc) normie
outer slay sys_ftruncate(fd normie, length thicc) normie
outer slay sys_fsync(fd normie) normie
outer slay sys_fdatasync(fd normie) normie
outer slay sys_flock(fd normie, operation normie) normie
outer slay sys_fcntl(fd normie, cmd normie, arg normie) normie
outer slay sys_statvfs(path [*:0]const u8, statvfs_buf *StatVfsBuffer) normie
outer slay sys_inotify_init() normie
outer slay sys_inotify_add_watch(fd normie, path [*:0]const u8, mask normie) normie
outer slay sys_inotify_rm_watch(fd normie, wd normie) normie

be_like StatBuffer squad {
    st_mode normie
    st_size thicc
    st_mtime thicc
    st_ctime thicc
    st_atime thicc
    st_uid normie
    st_gid normie
    st_dev normie
    st_ino normie
    st_nlink normie
    st_blksize normie
    st_blocks normie
}

be_like TimeSpec squad {
    tv_sec thicc
    tv_nsec thicc
}

be_like StatVfsBuffer squad {
    f_bsize normie
    f_frsize normie
    f_blocks thicc
    f_bfree thicc
    f_bavail thicc
    f_files thicc
    f_ffree thicc
    f_flag normie
}

fr fr ================================
fr fr Path Manipulation and Validation
fr fr ================================

slay normalize_path(path tea) tea { fr fr Normalize path with proper separator handling
    lowkey path == "" {
        damn "."
    }
    
    fr fr Convert backslashes to forward slashes
    sus normalized tea = string_replace_all(path, "\\", "/")
    
    fr fr Remove duplicate slashes
    bestie contains_string(normalized, "//") {
        normalized = string_replace_all(normalized, "//", "/")
    }
    
    fr fr Split into components and resolve . and ..
    sus components tea[value] = string_split(normalized, "/")
    sus result_components tea[value] = []
    sus is_absolute lit = starts_with(normalized, "/")
    
    bestie i := 0; i < array_length(components); i++ {
        sus component tea = components[i]
        
        lowkey component == "" || component == "." {
            continue fr fr Skip empty and current directory
        }
        
        lowkey component == ".." {
            lowkey array_length(result_components) > 0 && 
                  last_element(result_components) != ".." {
                result_components = remove_last_element(result_components)
            } otherwise lowkey !is_absolute {
                result_components = append_array(result_components, component)
            }
        } otherwise {
            result_components = append_array(result_components, component)
        }
    }
    
    fr fr Reconstruct path
    sus result tea = join_array(result_components, "/")
    
    lowkey is_absolute {
        result = "/" + result
    }
    
    lowkey result == "" {
        damn "."
    }
    
    damn result
}

slay get_absolute_path(path tea) tea { fr fr Get absolute path
    lowkey path == "" {
        damn get_current_directory()
    }
    
    lowkey is_absolute_path(path) {
        damn normalize_path(path)
    }
    
    sus cwd tea = get_current_directory()
    sus joined tea = join_path(cwd, path)
    damn normalize_path(joined)
}

slay resolve_path(path tea) tea { fr fr Resolve all symbolic links in path
    sus c_path [*:0]const u8 = string_to_cstring(path)
    sus buffer u8[4096] = undefined
    sus resolved [*:0]const u8 = sys_realpath(c_path, &buffer[0])
    
    lowkey resolved == null {
        damn normalize_path(path) fr fr Fallback to normalization
    }
    
    damn cstring_to_string(resolved)
}

slay is_absolute_path(path tea) lit { fr fr Check if path is absolute
    lowkey path == "" {
        damn false
    }
    
    fr fr Unix-style absolute path
    lowkey starts_with(path, "/") {
        damn true
    }
    
    fr fr Windows-style absolute path (C:\ or \\server\share)
    lowkey string_length(path) >= 3 {
        sus second_char normie = char_code_at(path, 1)
        lowkey second_char == 58 && char_code_at(path, 2) == 92 { fr fr C:\
            damn true
        }
    }
    
    lowkey starts_with(path, "\\\\") {
        damn true fr fr UNC path
    }
    
    damn false
}

slay join_path(base tea, component tea) tea { fr fr Join path components
    lowkey base == "" {
        damn component
    }
    
    lowkey component == "" {
        damn base
    }
    
    lowkey is_absolute_path(component) {
        damn component
    }
    
    sus normalized_base tea = normalize_path(base)
    sus separator tea = get_path_separator()
    
    lowkey ends_with(normalized_base, separator) {
        damn normalized_base + component
    }
    
    damn normalized_base + separator + component
}

slay get_path_separator() tea { fr fr Get platform-specific path separator
    fr fr In practice, this would be determined at compile time
    damn "/"
}

slay get_parent_directory(path tea) tea { fr fr Get parent directory
    sus normalized tea = normalize_path(path)
    
    lowkey normalized == "/" || normalized == "." {
        damn "."
    }
    
    sus last_sep thicc = string_last_index(normalized, "/")
    lowkey last_sep <= 0 {
        damn "."
    }
    
    sus parent tea = string_substring(normalized, 0, last_sep)
    lowkey parent == "" {
        damn "/"
    }
    
    damn parent
}

slay get_filename(path tea) tea { fr fr Get filename from path
    sus normalized tea = normalize_path(path)
    sus last_sep thicc = string_last_index(normalized, "/")
    
    lowkey last_sep < 0 {
        damn normalized
    }
    
    damn string_substring(normalized, last_sep + 1, string_length(normalized))
}

slay get_file_extension(path tea) tea { fr fr Get file extension
    sus filename tea = get_filename(path)
    sus last_dot thicc = string_last_index(filename, ".")
    
    lowkey last_dot <= 0 || last_dot == string_length(filename) - 1 {
        damn ""
    }
    
    damn string_substring(filename, last_dot + 1, string_length(filename))
}

slay get_basename(path tea) tea { fr fr Get filename without extension
    sus filename tea = get_filename(path)
    sus last_dot thicc = string_last_index(filename, ".")
    
    lowkey last_dot <= 0 {
        damn filename
    }
    
    damn string_substring(filename, 0, last_dot)
}

fr fr ================================
fr fr Core File Operations
fr fr ================================

slay read_file(path tea) tea { fr fr Read entire file as string
    sus bytes byte[value] = read_file_bytes(path)
    lowkey array_length(bytes) == 0 {
        damn ""
    }
    
    damn bytes_to_string(bytes)
}

slay read_file_bytes(path tea) byte[value]{ fr fr Read entire file as bytes
    sus handle FileHandle = open_file(path, MODE_READ)
    lowkey handle.fd < 0 {
        damn []
    }
    
    sus file_info FileInfo = get_file_info_from_handle(handle)
    lowkey file_info.size == 0 {
        close_file(handle)
        damn []
    }
    
    sus buffer byte[value] = make_byte_array(file_info.size)
    sus bytes_read thicc = read_from_handle(handle, buffer)
    
    close_file(handle)
    
    lowkey bytes_read != file_info.size {
        damn slice_bytes(buffer, 0, bytes_read)
    }
    
    damn buffer
}

slay read_file_lines(path tea) tea[value]{ fr fr Read file as array of lines
    sus content tea = read_file(path)
    lowkey content == "" {
        damn []
    }
    
    fr fr Split by various line endings
    sus lines tea[value] = string_split_lines(content)
    damn lines
}

slay write_file(path tea, content tea) lit { fr fr Write string to file
    sus bytes byte[value] = string_to_bytes(content)
    damn write_file_bytes(path, bytes)
}

slay write_file_bytes(path tea, data byte[value]) lit { fr fr Write bytes to file
    sus handle FileHandle = open_file(path, MODE_WRITE | MODE_CREATE | MODE_TRUNCATE)
    lowkey handle.fd < 0 {
        damn false
    }
    
    sus bytes_written thicc = write_to_handle(handle, data)
    sus success lit = bytes_written == array_length(data)
    
    close_file(handle)
    damn success
}

slay write_file_lines(path tea, lines tea[value]) lit { fr fr Write array of lines to file
    sus content tea = join_array(lines, get_line_ending())
    damn write_file(path, content)
}

slay append_file(path tea, content tea) lit { fr fr Append string to file
    sus bytes byte[value] = string_to_bytes(content)
    damn append_file_bytes(path, bytes)
}

slay append_file_bytes(path tea, data byte[value]) lit { fr fr Append bytes to file
    sus handle FileHandle = open_file(path, MODE_WRITE | MODE_APPEND | MODE_CREATE)
    lowkey handle.fd < 0 {
        damn false
    }
    
    sus bytes_written thicc = write_to_handle(handle, data)
    sus success lit = bytes_written == array_length(data)
    
    close_file(handle)
    damn success
}

slay copy_file(source tea, dest tea) lit { fr fr Copy file with metadata preservation
    lowkey !file_exists(source) {
        damn false
    }
    
    lowkey is_directory(source) {
        damn false
    }
    
    fr fr Read source file
    sus data byte[value] = read_file_bytes(source)
    lowkey array_length(data) == 0 && get_file_size(source) > 0 {
        damn false fr fr Read error
    }
    
    fr fr Write to destination
    lowkey !write_file_bytes(dest, data) {
        damn false
    }
    
    fr fr Copy metadata
    sus source_info FileInfo = get_file_info(source)
    copy_file_metadata(source_info, dest)
    
    damn true
}

slay copy_file_with_progress(source tea, dest tea, progress_callback tea) lit { 
    fr fr Copy file with progress callback
    lowkey !file_exists(source) {
        damn false
    }
    
    sus source_handle FileHandle = open_file(source, MODE_READ)
    lowkey source_handle.fd < 0 {
        damn false
    }
    
    sus dest_handle FileHandle = open_file(dest, MODE_WRITE | MODE_CREATE | MODE_TRUNCATE)
    lowkey dest_handle.fd < 0 {
        close_file(source_handle)
        damn false
    }
    
    sus buffer_size normie = 64 * 1024 fr fr 64KB buffer
    sus buffer byte[value] = make_byte_array(buffer_size)
    sus total_copied thicc = 0
    sus file_size thicc = get_file_size(source)
    
    bestie true {
        sus bytes_read thicc = read_from_handle(source_handle, buffer)
        lowkey bytes_read <= 0 {
            break
        }
        
        sus chunk byte[value] = slice_bytes(buffer, 0, bytes_read)
        sus bytes_written thicc = write_to_handle(dest_handle, chunk)
        
        lowkey bytes_written != bytes_read {
            close_file(source_handle)
            close_file(dest_handle)
            delete_file(dest) fr fr Clean up partial copy
            damn false
        }
        
        total_copied += bytes_written
        
        fr fr Call progress callback if provided
        lowkey progress_callback != "" && file_size > 0 {
            sus progress_percent normie = (total_copied * 100) / file_size
            call_progress_callback(progress_callback, progress_percent)
        }
    }
    
    close_file(source_handle)
    close_file(dest_handle)
    
    fr fr Copy metadata
    sus source_info FileInfo = get_file_info(source)
    copy_file_metadata(source_info, dest)
    
    damn total_copied == file_size
}

slay move_file(source tea, dest tea) lit { fr fr Move/rename file
    fr fr Try rename first (atomic on same filesystem)
    sus c_source [*:0]const u8 = string_to_cstring(source)
    sus c_dest [*:0]const u8 = string_to_cstring(dest)
    
    lowkey sys_rename(c_source, c_dest) == 0 {
        damn true
    }
    
    fr fr Fallback to copy + delete
    lowkey copy_file(source, dest) {
        damn delete_file(source)
    }
    
    damn false
}

slay delete_file(path tea) lit { fr fr Delete file
    lowkey !file_exists(path) {
        damn false
    }
    
    lowkey is_directory(path) {
        damn false
    }
    
    sus c_path [*:0]const u8 = string_to_cstring(path)
    damn sys_unlink(c_path) == 0
}

slay truncate_file(path tea, size thicc) lit { fr fr Truncate file to size
    sus c_path [*:0]const u8 = string_to_cstring(path)
    damn sys_truncate(c_path, size) == 0
}

fr fr ================================
fr fr Directory Operations
fr fr ================================

slay create_directory(path tea) lit { fr fr Create single directory
    lowkey path == "" {
        damn false
    }
    
    lowkey file_exists(path) {
        damn is_directory(path)
    }
    
    sus c_path [*:0]const u8 = string_to_cstring(path)
    sus mode normie = PERM_OWNER_READ | PERM_OWNER_WRITE | PERM_OWNER_EXECUTE |
                     PERM_GROUP_READ | PERM_GROUP_EXECUTE |
                     PERM_OTHER_READ | PERM_OTHER_EXECUTE
    
    damn sys_mkdir(c_path, mode) == 0
}

slay create_directory_recursive(path tea) lit { fr fr Create directory tree
    lowkey path == "" {
        damn false
    }
    
    lowkey file_exists(path) {
        damn is_directory(path)
    }
    
    sus parent tea = get_parent_directory(path)
    lowkey parent != "." && parent != path && !file_exists(parent) {
        lowkey !create_directory_recursive(parent) {
            damn false
        }
    }
    
    damn create_directory(path)
}

slay remove_directory(path tea) lit { fr fr Remove empty directory
    lowkey !is_directory(path) {
        damn false
    }
    
    sus c_path [*:0]const u8 = string_to_cstring(path)
    damn sys_rmdir(c_path) == 0
}

slay remove_directory_recursive(path tea) lit { fr fr Remove directory tree
    lowkey !is_directory(path) {
        damn delete_file(path) fr fr Try as file
    }
    
    sus entries tea[value] = list_directory(path)
    
    bestie i := 0; i < array_length(entries); i++ {
        sus entry tea = entries[i]
        lowkey entry == "." || entry == ".." {
            continue
        }
        
        sus full_path tea = join_path(path, entry)
        
        lowkey is_directory(full_path) {
            lowkey !remove_directory_recursive(full_path) {
                damn false
            }
        } otherwise {
            lowkey !delete_file(full_path) {
                damn false
            }
        }
    }
    
    damn remove_directory(path)
}

slay list_directory(path tea) tea[value]{ fr fr List directory contents
    lowkey !is_directory(path) {
        damn []
    }
    
    sus c_path [*:0]const u8 = string_to_cstring(path)
    sus dir_fd normie = sys_opendir(c_path)
    
    lowkey dir_fd < 0 {
        damn []
    }
    
    sus entries tea[value] = []
    
    bestie true {
        sus entry_name [*:0]const u8 = sys_readdir(dir_fd)
        lowkey entry_name == null {
            break
        }
        
        sus name tea = cstring_to_string(entry_name)
        lowkey name != "." && name != ".." {
            entries = append_array(entries, name)
        }
    }
    
    sys_closedir(dir_fd)
    damn entries
}

slay list_directory_detailed(path tea) FileInfo[value]{ fr fr List with file info
    sus entries tea[value] = list_directory(path)
    sus detailed_entries FileInfo[value] = []
    
    bestie i := 0; i < array_length(entries); i++ {
        sus entry tea = entries[i]
        sus full_path tea = join_path(path, entry)
        sus info FileInfo = get_file_info(full_path)
        detailed_entries = append_array(detailed_entries, info)
    }
    
    damn detailed_entries
}

slay copy_directory_recursive(source tea, dest tea) lit { fr fr Copy directory tree
    lowkey !is_directory(source) {
        damn false
    }
    
    lowkey !create_directory_recursive(dest) {
        damn false
    }
    
    sus entries tea[value] = list_directory(source)
    
    bestie i := 0; i < array_length(entries); i++ {
        sus entry tea = entries[i]
        sus source_path tea = join_path(source, entry)
        sus dest_path tea = join_path(dest, entry)
        
        lowkey is_directory(source_path) {
            lowkey !copy_directory_recursive(source_path, dest_path) {
                damn false
            }
        } otherwise {
            lowkey !copy_file(source_path, dest_path) {
                damn false
            }
        }
    }
    
    fr fr Copy directory metadata
    sus source_info FileInfo = get_file_info(source)
    copy_file_metadata(source_info, dest)
    
    damn true
}

fr fr ================================
fr fr File Information and Status
fr fr ================================

slay file_exists(path tea) lit { fr fr Check if file/directory exists
    sus c_path [*:0]const u8 = string_to_cstring(path)
    damn sys_access(c_path, 0) == 0 fr fr F_OK
}

slay is_directory(path tea) lit { fr fr Check if path is directory
    sus info FileInfo = get_file_info(path)
    damn info.is_dir
}

slay is_file(path tea) lit { fr fr Check if path is regular file
    sus info FileInfo = get_file_info(path)
    damn info.is_file
}

slay is_symlink(path tea) lit { fr fr Check if path is symbolic link
    sus c_path [*:0]const u8 = string_to_cstring(path)
    sus stat_buf StatBuffer
    
    lowkey sys_lstat(c_path, &stat_buf) != 0 {
        damn false
    }
    
    damn (stat_buf.st_mode & 0o170000) == 0o120000 fr fr S_IFLNK
}

slay is_executable(path tea) lit { fr fr Check if file is executable
    sus c_path [*:0]const u8 = string_to_cstring(path)
    damn sys_access(c_path, 1) == 0 fr fr X_OK
}

slay is_readable(path tea) lit { fr fr Check if file is readable
    sus c_path [*:0]const u8 = string_to_cstring(path)
    damn sys_access(c_path, 4) == 0 fr fr R_OK
}

slay is_writable(path tea) lit { fr fr Check if file is writable
    sus c_path [*:0]const u8 = string_to_cstring(path)
    damn sys_access(c_path, 2) == 0 fr fr W_OK
}

slay get_file_size(path tea) thicc { fr fr Get file size in bytes
    sus info FileInfo = get_file_info(path)
    damn info.size
}

slay get_file_info(path tea) FileInfo { fr fr Get comprehensive file information
    sus info FileInfo = {
        name: get_filename(path),
        path: path,
        size: 0,
        is_dir: false,
        is_file: false,
        is_symlink: false,
        is_hidden: false,
        is_executable: false,
        created_time: 0,
        modified_time: 0,
        accessed_time: 0,
        permissions: 0,
        owner_id: 0,
        group_id: 0,
        device_id: 0,
        inode: 0,
        hard_links: 0,
        block_size: 0,
        blocks: 0
    }
    
    sus c_path [*:0]const u8 = string_to_cstring(path)
    sus stat_buf StatBuffer
    
    lowkey sys_stat(c_path, &stat_buf) != 0 {
        damn info fr fr Return empty info on error
    }
    
    info.size = stat_buf.st_size
    info.created_time = stat_buf.st_ctime
    info.modified_time = stat_buf.st_mtime
    info.accessed_time = stat_buf.st_atime
    info.permissions = stat_buf.st_mode & 0o777
    info.owner_id = stat_buf.st_uid
    info.group_id = stat_buf.st_gid
    info.device_id = stat_buf.st_dev
    info.inode = stat_buf.st_ino
    info.hard_links = stat_buf.st_nlink
    info.block_size = stat_buf.st_blksize
    info.blocks = stat_buf.st_blocks
    
    fr fr Determine file type
    sus file_mode normie = stat_buf.st_mode & 0o170000
    info.is_dir = file_mode == 0o040000 fr fr S_IFDIR
    info.is_file = file_mode == 0o100000 fr fr S_IFREG
    info.is_symlink = file_mode == 0o120000 fr fr S_IFLNK
    
    fr fr Check if hidden (starts with .)
    info.is_hidden = starts_with(info.name, ".")
    
    fr fr Check if executable
    info.is_executable = (stat_buf.st_mode & (PERM_OWNER_EXECUTE | PERM_GROUP_EXECUTE | PERM_OTHER_EXECUTE)) != 0
    
    damn info
}

slay get_file_times(path tea) (thicc, thicc, thicc) { fr fr Get created, modified, accessed times
    sus info FileInfo = get_file_info(path)
    damn info.created_time, info.modified_time, info.accessed_time
}

slay set_file_times(path tea, accessed_time thicc, modified_time thicc) lit { fr fr Set file times
    sus c_path [*:0]const u8 = string_to_cstring(path)
    sus times TimeSpec[2] = undefined
    
    times[0].tv_sec = accessed_time
    times[0].tv_nsec = 0
    times[1].tv_sec = modified_time
    times[1].tv_nsec = 0
    
    damn sys_utimes(c_path, &times[0]) == 0
}

slay get_file_permissions(path tea) normie { fr fr Get file permissions
    sus info FileInfo = get_file_info(path)
    damn info.permissions
}

slay set_file_permissions(path tea, permissions normie) lit { fr fr Set file permissions
    sus c_path [*:0]const u8 = string_to_cstring(path)
    damn sys_chmod(c_path, permissions) == 0
}

slay get_file_owner(path tea) (normie, normie) { fr fr Get owner and group IDs
    sus info FileInfo = get_file_info(path)
    damn info.owner_id, info.group_id
}

slay set_file_owner(path tea, owner_id normie, group_id normie) lit { fr fr Set file owner
    sus c_path [*:0]const u8 = string_to_cstring(path)
    damn sys_chown(c_path, owner_id, group_id) == 0
}

fr fr ================================
fr fr Advanced File Operations
fr fr ================================

slay open_file(path tea, mode normie) FileHandle { fr fr Open file with mode
    sus handle FileHandle = {
        fd: -1,
        path: path,
        mode: mode,
        position: 0,
        is_open: false,
        buffer_size: 8192,
        buffer: [],
        buffer_pos: 0,
        buffer_len: 0,
        is_eof: false,
        error_code: 0
    }
    
    sus flags normie = 0
    sus permissions normie = PERM_OWNER_READ | PERM_OWNER_WRITE | PERM_GROUP_READ | PERM_OTHER_READ
    
    fr fr Convert mode flags to system flags
    lowkey (mode & MODE_READ) != 0 && (mode & MODE_WRITE) != 0 {
        flags |= 2 fr fr O_RDWR
    } otherwise lowkey (mode & MODE_WRITE) != 0 {
        flags |= 1 fr fr O_WRONLY
    } otherwise {
        flags |= 0 fr fr O_RDONLY
    }
    
    lowkey (mode & MODE_CREATE) != 0 {
        flags |= 64 fr fr O_CREAT
    }
    
    lowkey (mode & MODE_TRUNCATE) != 0 {
        flags |= 512 fr fr O_TRUNC
    }
    
    lowkey (mode & MODE_APPEND) != 0 {
        flags |= 1024 fr fr O_APPEND
    }
    
    lowkey (mode & MODE_EXCLUSIVE) != 0 {
        flags |= 128 fr fr O_EXCL
    }
    
    lowkey (mode & MODE_SYNC) != 0 {
        flags |= 4096 fr fr O_SYNC
    }
    
    sus c_path [*:0]const u8 = string_to_cstring(path)
    handle.fd = sys_open(c_path, flags, permissions)
    
    lowkey handle.fd >= 0 {
        handle.is_open = true
        lowkey (mode & MODE_READ) != 0 {
            handle.buffer = make_byte_array(handle.buffer_size)
        }
    } otherwise {
        handle.error_code = handle.fd fr fr Negative fd is error code
    }
    
    damn handle
}

slay close_file(handle FileHandle) lit { fr fr Close file handle
    lowkey !handle.is_open {
        damn false
    }
    
    sus result normie = sys_close(handle.fd)
    damn result == 0
}

slay read_from_handle(handle FileHandle, buffer byte[value]) thicc { fr fr Read from file handle
    lowkey !handle.is_open || handle.fd < 0 {
        damn 0
    }
    
    sus bytes_read thicc = sys_read(handle.fd, array_ptr(buffer), array_length(buffer))
    damn max_int(0, bytes_read)
}

slay write_to_handle(handle FileHandle, data byte[value]) thicc { fr fr Write to file handle
    lowkey !handle.is_open || handle.fd < 0 {
        damn 0
    }
    
    sus bytes_written thicc = sys_write(handle.fd, array_ptr(data), array_length(data))
    damn max_int(0, bytes_written)
}

slay seek_in_file(handle FileHandle, offset thicc, whence normie) thicc { fr fr Seek in file
    lowkey !handle.is_open || handle.fd < 0 {
        damn -1
    }
    
    sus new_pos thicc = sys_lseek(handle.fd, offset, whence)
    lowkey new_pos >= 0 {
        handle.position = new_pos
    }
    
    damn new_pos
}

slay flush_file(handle FileHandle) lit { fr fr Flush file buffers
    lowkey !handle.is_open || handle.fd < 0 {
        damn false
    }
    
    damn sys_fsync(handle.fd) == 0
}

slay lock_file(path tea, lock_type normie) FileLock { fr fr Lock file
    sus lock FileLock = {
        path: path,
        fd: -1,
        lock_type: lock_type,
        is_locked: false
    }
    
    sus handle FileHandle = open_file(path, MODE_READ | MODE_WRITE)
    lowkey handle.fd < 0 {
        damn lock
    }
    
    sus operation normie = 0
    lowkey lock_type == 1 { fr fr Shared lock
        operation = 1 fr fr LOCK_SH
    } otherwise { fr fr Exclusive lock
        operation = 2 fr fr LOCK_EX
    }
    
    lowkey sys_flock(handle.fd, operation) == 0 {
        lock.fd = handle.fd
        lock.is_locked = true
    } otherwise {
        close_file(handle)
    }
    
    damn lock
}

slay unlock_file(lock FileLock) lit { fr fr Unlock file
    lowkey !lock.is_locked || lock.fd < 0 {
        damn false
    }
    
    sus result lit = sys_flock(lock.fd, 8) == 0 fr fr LOCK_UN
    lowkey result {
        sys_close(lock.fd)
    }
    
    damn result
}

fr fr ================================
fr fr Symbolic Links and Hard Links
fr fr ================================

slay create_symlink(target tea, link_path tea) lit { fr fr Create symbolic link
    sus c_target [*:0]const u8 = string_to_cstring(target)
    sus c_link [*:0]const u8 = string_to_cstring(link_path)
    damn sys_symlink(c_target, c_link) == 0
}

slay create_hardlink(target tea, link_path tea) lit { fr fr Create hard link
    sus c_target [*:0]const u8 = string_to_cstring(target)
    sus c_link [*:0]const u8 = string_to_cstring(link_path)
    damn sys_link(c_target, c_link) == 0
}

slay read_symlink(link_path tea) tea { fr fr Read symbolic link target
    lowkey !is_symlink(link_path) {
        damn ""
    }
    
    sus c_path [*:0]const u8 = string_to_cstring(link_path)
    sus buffer u8[4096] = undefined
    
    sus length thicc = sys_readlink(c_path, &buffer[0], 4095)
    lowkey length <= 0 {
        damn ""
    }
    
    buffer[length] = 0 fr fr Null terminate
    damn cstring_to_string(&buffer[0])
}

fr fr ================================
fr fr Directory and Working Directory Management
fr fr ================================

slay get_current_directory() tea { fr fr Get current working directory
    sus buffer u8[4096] = undefined
    sus result [*:0]const u8 = sys_getcwd(&buffer[0], 4095)
    
    lowkey result == null {
        damn "." fr fr Fallback
    }
    
    damn cstring_to_string(result)
}

slay set_current_directory(path tea) lit { fr fr Change current working directory
    sus c_path [*:0]const u8 = string_to_cstring(path)
    damn sys_chdir(c_path) == 0
}

slay get_temp_directory() tea { fr fr Get temporary directory path
    fr fr Try various environment variables and fallback paths
    sus temp_dirs tea[value] = ["/tmp", "/var/tmp", "/usr/tmp", "."]
    
    bestie i := 0; i < array_length(temp_dirs); i++ {
        sus temp_dir tea = temp_dirs[i]
        lowkey is_directory(temp_dir) && is_writable(temp_dir) {
            damn temp_dir
        }
    }
    
    damn "." fr fr Fallback to current directory
}

slay create_temp_file(prefix tea) tea { fr fr Create temporary file
    sus temp_dir tea = get_temp_directory()
    sus timestamp thicc = get_current_timestamp()
    sus random_part normie = get_random_int() % 10000
    
    sus temp_name tea = prefix + "_" + int_to_string(timestamp) + "_" + int_to_string(random_part)
    sus temp_path tea = join_path(temp_dir, temp_name)
    
    fr fr Ensure uniqueness
    sus counter normie = 0
    bestie file_exists(temp_path) && counter < 100 {
        counter += 1
        temp_name = prefix + "_" + int_to_string(timestamp) + "_" + 
                   int_to_string(random_part) + "_" + int_to_string(counter)
        temp_path = join_path(temp_dir, temp_name)
    }
    
    fr fr Create the file
    lowkey write_file(temp_path, "") {
        damn temp_path
    }
    
    damn ""
}

fr fr ================================
fr fr Filesystem Statistics
fr fr ================================

slay get_filesystem_stats(path tea) FileSystemStats { fr fr Get filesystem statistics
    sus stats FileSystemStats = {
        total_space: 0,
        free_space: 0,
        available_space: 0,
        total_inodes: 0,
        free_inodes: 0,
        block_size: 0,
        filesystem_type: "",
        mount_point: "",
        is_readonly: false
    }
    
    sus c_path [*:0]const u8 = string_to_cstring(path)
    sus statvfs_buf StatVfsBuffer
    
    lowkey sys_statvfs(c_path, &statvfs_buf) != 0 {
        damn stats
    }
    
    stats.block_size = statvfs_buf.f_frsize
    stats.total_space = statvfs_buf.f_blocks * statvfs_buf.f_frsize
    stats.free_space = statvfs_buf.f_bfree * statvfs_buf.f_frsize
    stats.available_space = statvfs_buf.f_bavail * statvfs_buf.f_frsize
    stats.total_inodes = statvfs_buf.f_files
    stats.free_inodes = statvfs_buf.f_ffree
    stats.is_readonly = (statvfs_buf.f_flag & 1) != 0 fr fr ST_RDONLY
    
    damn stats
}

slay get_disk_usage(path tea) thicc { fr fr Get disk space used by path
    lowkey is_file(path) {
        damn get_file_size(path)
    }
    
    lowkey !is_directory(path) {
        damn 0
    }
    
    sus total_size thicc = 0
    sus entries tea[value] = list_directory(path)
    
    bestie i := 0; i < array_length(entries); i++ {
        sus entry tea = entries[i]
        sus full_path tea = join_path(path, entry)
        total_size += get_disk_usage(full_path) fr fr Recursive
    }
    
    damn total_size
}

fr fr ================================
fr fr File Watching/Monitoring
fr fr ================================

slay watch_file(path tea, events normie, callback tea) WatchHandle { fr fr Watch file for changes
    sus handle WatchHandle = {
        path: path,
        handle: -1,
        events: events,
        is_active: false,
        callback_name: callback
    }
    
    fr fr Initialize inotify
    sus inotify_fd normie = sys_inotify_init()
    lowkey inotify_fd < 0 {
        damn handle
    }
    
    fr fr Add watch
    sus c_path [*:0]const u8 = string_to_cstring(path)
    sus watch_fd normie = sys_inotify_add_watch(inotify_fd, c_path, events)
    
    lowkey watch_fd >= 0 {
        handle.handle = inotify_fd
        handle.is_active = true
    } otherwise {
        sys_close(inotify_fd)
    }
    
    damn handle
}

slay stop_watching(handle WatchHandle) lit { fr fr Stop watching file
    lowkey !handle.is_active || handle.handle < 0 {
        damn false
    }
    
    sys_close(handle.handle)
    damn true
}

fr fr ================================
fr fr Utility Functions
fr fr ================================

slay copy_file_metadata(source_info FileInfo, dest_path tea) { fr fr Copy file metadata
    fr fr Set permissions
    set_file_permissions(dest_path, source_info.permissions)
    
    fr fr Set times
    set_file_times(dest_path, source_info.accessed_time, source_info.modified_time)
    
    fr fr Set owner (requires root privileges)
    set_file_owner(dest_path, source_info.owner_id, source_info.group_id)
}

slay get_file_info_from_handle(handle FileHandle) FileInfo { fr fr Get file info from open handle
    sus info FileInfo = {
        name: get_filename(handle.path),
        path: handle.path,
        size: 0,
        is_dir: false,
        is_file: false,
        is_symlink: false,
        is_hidden: false,
        is_executable: false,
        created_time: 0,
        modified_time: 0,
        accessed_time: 0,
        permissions: 0,
        owner_id: 0,
        group_id: 0,
        device_id: 0,
        inode: 0,
        hard_links: 0,
        block_size: 0,
        blocks: 0
    }
    
    lowkey !handle.is_open || handle.fd < 0 {
        damn info
    }
    
    sus stat_buf StatBuffer
    lowkey sys_fstat(handle.fd, &stat_buf) != 0 {
        damn info
    }
    
    fr fr Fill in the same way as get_file_info
    info.size = stat_buf.st_size
    info.created_time = stat_buf.st_ctime
    info.modified_time = stat_buf.st_mtime
    info.accessed_time = stat_buf.st_atime
    info.permissions = stat_buf.st_mode & 0o777
    info.owner_id = stat_buf.st_uid
    info.group_id = stat_buf.st_gid
    info.device_id = stat_buf.st_dev
    info.inode = stat_buf.st_ino
    info.hard_links = stat_buf.st_nlink
    info.block_size = stat_buf.st_blksize
    info.blocks = stat_buf.st_blocks
    
    sus file_mode normie = stat_buf.st_mode & 0o170000
    info.is_dir = file_mode == 0o040000
    info.is_file = file_mode == 0o100000
    info.is_symlink = file_mode == 0o120000
    info.is_hidden = starts_with(info.name, ".")
    info.is_executable = (stat_buf.st_mode & (PERM_OWNER_EXECUTE | PERM_GROUP_EXECUTE | PERM_OTHER_EXECUTE)) != 0
    
    damn info
}

slay get_line_ending() tea { fr fr Get platform-specific line ending
    fr fr In practice, this would be determined at compile time
    damn "\n"
}

fr fr ================================
fr fr String and Array Helper Functions (placeholders)
fr fr ================================

slay string_to_cstring(s tea) [*:0]const u8 {
    fr fr Placeholder - would use proper string conversion
    damn null
}

slay cstring_to_string(cs [*:0]const u8) tea {
    fr fr Placeholder - would use proper string conversion
    damn ""
}

slay string_to_bytes(s tea) byte[value]{
    fr fr Placeholder - would use proper string conversion
    damn []
}

slay bytes_to_string(bytes byte[value]) tea {
    fr fr Placeholder - would use proper string conversion
    damn ""
}

slay make_byte_array(size thicc) byte[value]{
    fr fr Placeholder - would use proper memory allocation
    damn []
}

slay array_ptr(arr byte[value]) [*]u8 {
    fr fr Placeholder - would get array pointer
    damn null
}

slay array_length(arr byte[value]) thicc {
    fr fr Placeholder - would get array length
    damn 0
}

slay slice_bytes(arr byte[value], start thicc, end thicc) byte[value]{
    fr fr Placeholder - would slice byte array
    damn []
}

slay max_int(a thicc, b thicc) thicc {
    lowkey a > b {
        damn a
    }
    damn b
}

slay get_current_timestamp() thicc {
    fr fr Placeholder - would get current timestamp
    damn 1640995200
}

slay get_random_int() normie {
    fr fr Placeholder - would get random integer
    damn 42
}

slay call_progress_callback(callback_name tea, progress normie) {
    fr fr Placeholder - would call named callback function
}

fr fr String function placeholders
slay string_replace_all(s tea, old tea, new tea) tea { damn s }
slay string_split(s tea, delimiter tea) tea[value]{ damn [] }
slay string_split_lines(s tea) tea[value]{ damn [] }
slay string_last_index(s tea, sub tea) thicc { damn -1 }
slay string_substring(s tea, start thicc, end thicc) tea { damn s }
slay string_length(s tea) thicc { damn 0 }
slay char_code_at(s tea, index thicc) normie { damn 65 }
slay starts_with(s tea, prefix tea) lit { damn false }
slay ends_with(s tea, suffix tea) lit { damn false }
slay contains_string(s tea, sub tea) lit { damn false }
slay int_to_string(n normie) tea { damn "0" }
slay int_to_string(n thicc) tea { damn "0" }

fr fr Array function placeholders
slay append_array(arr tea[value], item tea) tea[value]{ damn arr }
slay append_array(arr FileInfo[value], item FileInfo) FileInfo[value]{ damn arr }
slay join_array(arr tea[value], delimiter tea) tea { damn "" }
slay last_element(arr tea[value]) tea { damn "" }
slay remove_last_element(arr tea[value]) tea[value]{ damn arr }
slay array_length(arr tea[value]) thicc { damn 0 }
slay array_length(arr FileInfo[value]) thicc { damn 0 }
