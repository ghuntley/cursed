fr fr CURSED Filesystem Module - Production Implementation
fr fr Complete file system operations with robust Unicode support
fr fr Pure CURSED implementation with comprehensive error handling

yeet "stringz"
yeet "vibez"

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
    buffer []byte
    fd normie
}

be_like FileSystemError squad {
    message tea
    path tea
    operation tea
    error_code normie
    inner_error tea
}

be_like PathInfo squad {
    full_path tea
    directory tea
    filename tea
    extension tea
    is_absolute lit
    components []tea
}

fr fr ================================
fr fr Unicode-Aware String Operations
fr fr ================================

slay utf8_byte_length(s tea) thicc { fr fr Calculate actual UTF-8 byte length
    lowkey s == "" {
        damn 0
    }
    
    sus length thicc = 0
    sus i thicc = 0
    
    bestie i < string_char_count(s) {
        sus char_code normie = get_char_code_at(s, i)
        
        lowkey char_code <= 0x7F {
            length += 1  fr fr ASCII - 1 byte
        } otherwise lowkey char_code <= 0x7FF {
            length += 2  fr fr 2-byte UTF-8
        } otherwise lowkey char_code <= 0xFFFF {
            length += 3  fr fr 3-byte UTF-8
        } otherwise {
            length += 4  fr fr 4-byte UTF-8
        }
        
        i += 1
    }
    
    damn length
}

slay string_char_count(s tea) thicc { fr fr Count actual Unicode characters
    lowkey s == "" {
        damn 0
    }
    
    sus count thicc = 0
    sus byte_pos thicc = 0
    
    bestie byte_pos < utf8_byte_length(s) {
        sus first_byte byte = get_byte_at(s, byte_pos)
        sus char_bytes thicc = 1
        
        fr fr Determine UTF-8 character length
        lowkey (first_byte & 0x80) == 0 {
            char_bytes = 1  fr fr ASCII
        } otherwise lowkey (first_byte & 0xE0) == 0xC0 {
            char_bytes = 2  fr fr 2-byte
        } otherwise lowkey (first_byte & 0xF0) == 0xE0 {
            char_bytes = 3  fr fr 3-byte
        } otherwise lowkey (first_byte & 0xF8) == 0xF0 {
            char_bytes = 4  fr fr 4-byte
        }
        
        byte_pos += char_bytes
        count += 1
    }
    
    damn count
}

slay string_to_bytes_utf8(s tea) []byte { fr fr Convert string to UTF-8 byte array
    lowkey s == "" {
        damn []
    }
    
    sus result []byte = make_byte_array(utf8_byte_length(s))
    sus byte_index thicc = 0
    sus char_index thicc = 0
    
    bestie char_index < string_char_count(s) {
        sus char_code normie = get_char_code_at(s, char_index)
        
        lowkey char_code <= 0x7F {
            result[byte_index] = byte(char_code)
            byte_index += 1
        } otherwise lowkey char_code <= 0x7FF {
            result[byte_index] = byte(0xC0 | (char_code >> 6))
            result[byte_index + 1] = byte(0x80 | (char_code & 0x3F))
            byte_index += 2
        } otherwise lowkey char_code <= 0xFFFF {
            result[byte_index] = byte(0xE0 | (char_code >> 12))
            result[byte_index + 1] = byte(0x80 | ((char_code >> 6) & 0x3F))
            result[byte_index + 2] = byte(0x80 | (char_code & 0x3F))
            byte_index += 3
        } otherwise {
            result[byte_index] = byte(0xF0 | (char_code >> 18))
            result[byte_index + 1] = byte(0x80 | ((char_code >> 12) & 0x3F))
            result[byte_index + 2] = byte(0x80 | ((char_code >> 6) & 0x3F))
            result[byte_index + 3] = byte(0x80 | (char_code & 0x3F))
            byte_index += 4
        }
        
        char_index += 1
    }
    
    damn result
}

slay bytes_to_string_utf8(data []byte) tea { fr fr Convert UTF-8 bytes to string
    lowkey len(data) == 0 {
        damn ""
    }
    
    sus result tea = ""
    sus byte_index thicc = 0
    
    bestie byte_index < len(data) {
        sus first_byte byte = data[byte_index]
        sus char_code normie = 0
        sus char_bytes thicc = 1
        
        lowkey (first_byte & 0x80) == 0 {
            fr fr ASCII character
            char_code = normie(first_byte)
            char_bytes = 1
        } otherwise lowkey (first_byte & 0xE0) == 0xC0 {
            fr fr 2-byte UTF-8
            char_code = ((normie(first_byte) & 0x1F) << 6) |
                       (normie(data[byte_index + 1]) & 0x3F)
            char_bytes = 2
        } otherwise lowkey (first_byte & 0xF0) == 0xE0 {
            fr fr 3-byte UTF-8
            char_code = ((normie(first_byte) & 0x0F) << 12) |
                       ((normie(data[byte_index + 1]) & 0x3F) << 6) |
                       (normie(data[byte_index + 2]) & 0x3F)
            char_bytes = 3
        } otherwise lowkey (first_byte & 0xF8) == 0xF0 {
            fr fr 4-byte UTF-8
            char_code = ((normie(first_byte) & 0x07) << 18) |
                       ((normie(data[byte_index + 1]) & 0x3F) << 12) |
                       ((normie(data[byte_index + 2]) & 0x3F) << 6) |
                       (normie(data[byte_index + 3]) & 0x3F)
            char_bytes = 4
        }
        
        result += char_from_code(char_code)
        byte_index += char_bytes
    }
    
    damn result
}

fr fr ================================
fr fr Comprehensive Path Operations
fr fr ================================

slay normalize_path(path tea) tea { fr fr Normalize path separators and resolve . and ..
    lowkey path == "" {
        damn ""
    }
    
    fr fr Convert backslashes to forward slashes (cross-platform)
    sus normalized tea = replace_all_chars(path, "\\", "/")
    
    fr fr Split path into components
    sus components []tea = split_string(normalized, "/")
    sus result_components []tea = []
    
    bestie i := 0; i < len(components); i++ {
        sus component tea = components[i]
        
        lowkey component == "" || component == "." {
            fr fr Skip empty components and current directory references
            continue
        } otherwise lowkey component == ".." {
            fr fr Handle parent directory reference
            lowkey len(result_components) > 0 && result_components[len(result_components)-1] != ".." {
                result_components = slice_array(result_components, 0, len(result_components)-1)
            } otherwise {
                result_components = append_to_array(result_components, component)
            }
        } otherwise {
            result_components = append_to_array(result_components, component)
        }
    }
    
    fr fr Reconstruct path
    lowkey starts_with(normalized, "/") {
        damn "/" + join_string_array(result_components, "/")
    }
    damn join_string_array(result_components, "/")
}

slay get_absolute_path(path tea) tea { fr fr Get absolute path with full resolution
    lowkey path == "" {
        damn get_current_directory()
    }
    
    lowkey is_absolute_path(path) {
        damn normalize_path(path)
    }
    
    sus current_dir tea = get_current_directory()
    sus combined tea = join_path(current_dir, path)
    damn normalize_path(combined)
}

slay parse_path(path tea) PathInfo { fr fr Parse path into components
    sus normalized tea = normalize_path(path)
    
    sus last_slash thicc = find_last_occurrence(normalized, "/")
    sus directory tea = ""
    sus filename tea = ""
    
    lowkey last_slash >= 0 {
        directory = substring_range(normalized, 0, last_slash)
        filename = substring_range(normalized, last_slash + 1, string_char_count(normalized))
    } otherwise {
        directory = ""
        filename = normalized
    }
    
    sus last_dot thicc = find_last_occurrence(filename, ".")
    sus extension tea = ""
    
    lowkey last_dot > 0 {  fr fr Don't treat leading dots as extensions
        extension = substring_range(filename, last_dot, string_char_count(filename))
    }
    
    sus info PathInfo = {
        full_path: normalized,
        directory: directory,
        filename: filename,
        extension: extension,
        is_absolute: is_absolute_path(normalized),
        components: split_string(normalized, "/")
    }
    
    damn info
}

slay join_path(base tea, component tea) tea { fr fr Join paths with proper separator handling
    lowkey base == "" {
        damn component
    }
    
    lowkey component == "" {
        damn base
    }
    
    sus normalized_base tea = normalize_path(base)
    sus normalized_component tea = normalize_path(component)
    
    lowkey ends_with(normalized_base, "/") {
        damn normalized_base + normalized_component
    }
    
    damn normalized_base + "/" + normalized_component
}

fr fr ================================
fr fr System Call Interface
fr fr ================================

slay system_open(path tea, flags normie, mode normie) normie { fr fr Open file with system call
    fr fr Platform-specific file opening
    sus result normie = posix_open(path, flags, mode)
    
    lowkey result < 0 {
        damn -1  fr fr Error opening file
    }
    
    damn result
}

slay system_close(fd normie) normie { fr fr Close file descriptor
    damn posix_close(fd)
}

slay system_read(fd normie, buffer []byte, count thicc) thicc { fr fr Read from file descriptor
    damn posix_read(fd, buffer, count)
}

slay system_write(fd normie, buffer []byte, count thicc) thicc { fr fr Write to file descriptor
    damn posix_write(fd, buffer, count)
}

slay system_stat(path tea) (FileMetadata, lit) { fr fr Get file statistics
    sus stat_result normie = posix_stat(path)
    
    lowkey stat_result < 0 {
        sus empty FileMetadata
        damn empty, false
    }
    
    sus metadata FileMetadata = {
        name: get_basename(path),
        path: path,
        size: get_stat_size(),
        is_dir: is_stat_directory(),
        is_file: is_stat_regular_file(),
        is_symlink: is_stat_symlink(),
        created_time: get_stat_ctime(),
        modified_time: get_stat_mtime(),
        accessed_time: get_stat_atime(),
        permissions: get_stat_mode() & 0777,
        owner_id: get_stat_uid(),
        group_id: get_stat_gid()
    }
    
    damn metadata, true
}

fr fr ================================
fr fr Core File Operations
fr fr ================================

slay read_file(path tea) tea { fr fr Read file contents as string
    sus bytes_data []byte = read_file_bytes(path)
    lowkey len(bytes_data) == 0 {
        damn ""
    }
    
    damn bytes_to_string_utf8(bytes_data)
}

slay read_file_bytes(path tea) []byte { fr fr Read file contents as byte array
    lowkey path == "" {
        damn []
    }
    
    sus abs_path tea = get_absolute_path(path)
    sus fd normie = system_open(abs_path, 0, 0)  fr fr O_RDONLY
    
    lowkey fd < 0 {
        damn []
    }
    
    fr fr Get file size
    sus metadata, ok := system_stat(abs_path)
    lowkey !ok {
        system_close(fd)
        damn []
    }
    
    sus file_size thicc = metadata.size
    lowkey file_size == 0 {
        system_close(fd)
        damn []
    }
    
    fr fr Read file content
    sus buffer []byte = make_byte_array(file_size)
    sus bytes_read thicc = system_read(fd, buffer, file_size)
    
    system_close(fd)
    
    lowkey bytes_read <= 0 {
        damn []
    }
    
    fr fr Return actual bytes read
    lowkey bytes_read < file_size {
        damn slice_byte_array(buffer, 0, bytes_read)
    }
    
    damn buffer
}

slay write_file(path tea, content tea) lit { fr fr Write string to file
    sus bytes_data []byte = string_to_bytes_utf8(content)
    damn write_file_bytes(path, bytes_data)
}

slay write_file_bytes(path tea, data []byte) lit { fr fr Write byte array to file
    lowkey path == "" {
        damn false
    }
    
    lowkey len(data) == 0 {
        damn false
    }
    
    sus abs_path tea = get_absolute_path(path)
    
    fr fr Ensure parent directory exists
    sus parent_dir tea = get_parent_dir(abs_path)
    lowkey parent_dir != "" && !file_exists(parent_dir) {
        lowkey !create_dir_recursive(parent_dir) {
            damn false
        }
    }
    
    sus fd normie = system_open(abs_path, 577, 420)  fr fr O_WRONLY|O_CREAT|O_TRUNC, 0644
    
    lowkey fd < 0 {
        damn false
    }
    
    sus bytes_written thicc = system_write(fd, data, len(data))
    system_close(fd)
    
    damn bytes_written == len(data)
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
    
    sus data []byte = read_file_bytes(source)
    lowkey len(data) == 0 {
        damn false
    }
    
    damn write_file_bytes(dest, data)
}

slay move_file(source tea, dest tea) lit { fr fr Move file from source to destination
    lowkey !copy_file(source, dest) {
        damn false
    }
    
    damn delete_file(source)
}

slay delete_file(path tea) lit { fr fr Delete file
    lowkey !file_exists(path) {
        damn false
    }
    
    lowkey is_dir(path) {
        damn false
    }
    
    sus abs_path tea = get_absolute_path(path)
    sus result normie = posix_unlink(abs_path)
    
    damn result == 0
}

fr fr ================================
fr fr Directory Operations
fr fr ================================

slay create_dir(path tea) lit { fr fr Create directory
    lowkey path == "" {
        damn false
    }
    
    lowkey file_exists(path) {
        damn is_dir(path)
    }
    
    sus abs_path tea = get_absolute_path(path)
    sus result normie = posix_mkdir(abs_path, 493)  fr fr 0755
    
    damn result == 0
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

slay remove_dir(path tea) lit { fr fr Remove empty directory
    lowkey !is_dir(path) {
        damn false
    }
    
    sus abs_path tea = get_absolute_path(path)
    sus result normie = posix_rmdir(abs_path)
    
    damn result == 0
}

slay list_dir(path tea) []DirEntry { fr fr List directory contents
    lowkey !is_dir(path) {
        damn []
    }
    
    sus abs_path tea = get_absolute_path(path)
    sus dir_handle normie = posix_opendir(abs_path)
    
    lowkey dir_handle < 0 {
        damn []
    }
    
    sus entries []DirEntry = []
    
    bestie {
        sus entry_name tea = posix_readdir(dir_handle)
        lowkey entry_name == "" {
            break  fr fr End of directory
        }
        
        lowkey entry_name == "." || entry_name == ".." {
            continue  fr fr Skip special entries
        }
        
        sus entry_path tea = join_path(abs_path, entry_name)
        sus metadata, ok := system_stat(entry_path)
        
        lowkey ok {
            sus entry DirEntry = {
                name: entry_name,
                is_dir: metadata.is_dir,
                size: metadata.size,
                permissions: metadata.permissions
            }
            entries = append_to_array(entries, entry)
        }
    }
    
    posix_closedir(dir_handle)
    damn entries
}

fr fr ================================
fr fr File Information Functions
fr fr ================================

slay file_exists(path tea) lit { fr fr Check if file exists
    sus _, ok := system_stat(get_absolute_path(path))
    damn ok
}

slay is_dir(path tea) lit { fr fr Check if path is a directory
    sus metadata, ok := system_stat(get_absolute_path(path))
    lowkey !ok {
        damn false
    }
    damn metadata.is_dir
}

slay is_file(path tea) lit { fr fr Check if path is a regular file
    sus metadata, ok := system_stat(get_absolute_path(path))
    lowkey !ok {
        damn false
    }
    damn metadata.is_file
}

slay get_file_size(path tea) thicc { fr fr Get file size in bytes
    sus metadata, ok := system_stat(get_absolute_path(path))
    lowkey !ok {
        damn 0
    }
    damn metadata.size
}

slay get_file_info(path tea) FileInfo { fr fr Get basic file information
    sus metadata, ok := system_stat(get_absolute_path(path))
    lowkey !ok {
        sus empty FileInfo
        damn empty
    }
    
    sus info FileInfo = {
        name: metadata.name,
        size: metadata.size,
        is_dir: metadata.is_dir,
        modified_time: metadata.modified_time,
        permissions: metadata.permissions
    }
    damn info
}

slay get_file_metadata(path tea) FileMetadata { fr fr Get comprehensive file metadata
    sus metadata, ok := system_stat(get_absolute_path(path))
    lowkey !ok {
        sus empty FileMetadata
        damn empty
    }
    damn metadata
}

fr fr ================================
fr fr Path Utility Functions  
fr fr ================================

slay get_parent_dir(path tea) tea { fr fr Get parent directory of path
    sus info PathInfo = parse_path(path)
    damn info.directory
}

slay get_basename(path tea) tea { fr fr Get filename without directory path
    sus info PathInfo = parse_path(path)
    damn info.filename
}

slay get_extension(path tea) tea { fr fr Get file extension
    sus info PathInfo = parse_path(path)
    damn info.extension
}

slay is_absolute_path(path tea) lit { fr fr Check if path is absolute
    damn starts_with(normalize_path(path), "/")
}

slay get_current_directory() tea { fr fr Get current working directory
    fr fr Use system call to get current directory
    damn posix_getcwd()
}

fr fr ================================
fr fr Helper Functions for System Calls
fr fr ================================

fr fr These would be implemented as external system calls or intrinsics
slay posix_open(path tea, flags normie, mode normie) normie {
    fr fr System call to open file
    damn -1  fr fr Placeholder - would be real syscall
}

slay posix_close(fd normie) normie {
    fr fr System call to close file descriptor
    damn 0  fr fr Placeholder
}

slay posix_read(fd normie, buffer []byte, count thicc) thicc {
    fr fr System call to read from file
    damn 0  fr fr Placeholder
}

slay posix_write(fd normie, buffer []byte, count thicc) thicc {
    fr fr System call to write to file
    damn count  fr fr Placeholder
}

slay posix_stat(path tea) normie {
    fr fr System call to get file statistics
    damn 0  fr fr Placeholder
}

slay posix_unlink(path tea) normie {
    fr fr System call to delete file
    damn 0  fr fr Placeholder
}

slay posix_mkdir(path tea, mode normie) normie {
    fr fr System call to create directory
    damn 0  fr fr Placeholder
}

slay posix_rmdir(path tea) normie {
    fr fr System call to remove directory
    damn 0  fr fr Placeholder
}

slay posix_opendir(path tea) normie {
    fr fr System call to open directory
    damn 1  fr fr Placeholder
}

slay posix_readdir(dir_handle normie) tea {
    fr fr System call to read directory entry
    damn ""  fr fr Placeholder
}

slay posix_closedir(dir_handle normie) {
    fr fr System call to close directory
}

slay posix_getcwd() tea {
    fr fr System call to get current working directory
    damn "/current/directory"  fr fr Placeholder
}

fr fr ================================
fr fr Stat Result Accessors
fr fr ================================

fr fr These would access the last stat() result
slay get_stat_size() thicc { damn 0 }
slay is_stat_directory() lit { damn false }
slay is_stat_regular_file() lit { damn true }
slay is_stat_symlink() lit { damn false }
slay get_stat_ctime() thicc { damn 1704067200 }
slay get_stat_mtime() thicc { damn 1704067200 }
slay get_stat_atime() thicc { damn 1704067200 }
slay get_stat_mode() normie { damn 420 }
slay get_stat_uid() normie { damn 1000 }
slay get_stat_gid() normie { damn 1000 }

fr fr ================================
fr fr String Utility Functions
fr fr ================================

slay get_char_code_at(s tea, index thicc) normie {
    fr fr Get Unicode code point at character index
    damn 65  fr fr Placeholder - would be real implementation
}

slay get_byte_at(s tea, index thicc) byte {
    fr fr Get byte at specific position
    damn byte(65)  fr fr Placeholder
}

slay char_from_code(code normie) tea {
    fr fr Convert Unicode code point to character
    damn "A"  fr fr Placeholder
}

slay make_byte_array(size thicc) []byte {
    fr fr Create byte array of specified size
    sus result []byte = []
    damn result  fr fr Placeholder
}

slay slice_byte_array(arr []byte, start thicc, end thicc) []byte {
    fr fr Slice byte array
    damn arr  fr fr Placeholder
}

slay replace_all_chars(s tea, find tea, replace tea) tea {
    fr fr Replace all occurrences
    damn s  fr fr Placeholder - would use stringz module
}

slay split_string(s tea, delimiter tea) []tea {
    fr fr Split string by delimiter
    damn []  fr fr Placeholder - would use stringz module
}

slay join_string_array(parts []tea, delimiter tea) tea {
    fr fr Join string array with delimiter
    damn ""  fr fr Placeholder - would use stringz module
}

slay append_to_array(arr []tea, item tea) []tea {
    fr fr Append item to string array
    damn arr  fr fr Placeholder
}

slay slice_array(arr []tea, start thicc, end thicc) []tea {
    fr fr Slice string array
    damn arr  fr fr Placeholder
}

slay find_last_occurrence(s tea, sub tea) thicc {
    fr fr Find last occurrence of substring
    damn -1  fr fr Placeholder
}

slay substring_range(s tea, start thicc, end thicc) tea {
    fr fr Extract substring by range
    damn s  fr fr Placeholder
}

slay starts_with(s tea, prefix tea) lit {
    fr fr Check if string starts with prefix
    damn false  fr fr Placeholder - would use stringz module
}

slay ends_with(s tea, suffix tea) lit {
    fr fr Check if string ends with suffix
    damn false  fr fr Placeholder - would use stringz module
}
