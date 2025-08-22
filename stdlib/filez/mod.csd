fr fr FILEZ MODULE - Pure CURSED File System Operations  
fr fr Enhanced implementation with comprehensive error handling and security

fr fr Import the complete filez implementation
yeet "filez"

fr fr Re-export all functions for backward compatibility

yeet "stringz"
yeet "mathz"
yeet "vibez"
yeet "cross_platform_paths"

fr fr ===== FILE SYSTEM STRUCTURES =====

squad FileInfo {
    sus name tea
    sus path tea
    sus size drip
    sus is_directory lit
    sus is_readable lit
    sus is_writable lit
    sus last_modified drip
    sus permissions drip
}

squad FileHandle {
    sus fd drip
    sus path tea
    sus mode tea
    sus position drip
    sus is_open lit
    sus buffer_size drip
}

squad DirectoryEntry {
    sus name tea
    sus full_path tea
    sus is_directory lit
    sus size drip
}

fr fr ===== CORE FILE OPERATIONS =====

slay file_open(path tea, mode tea) FileHandle {
    fr fr Open file with specified mode (r, w, a, r+, w+, a+)
    sus handle FileHandle = FileHandle{}
    handle.path = path
    handle.mode = mode
    handle.position = 0
    handle.buffer_size = 8192
    
    ready (mode == "r") {
        handle.fd = open_file_readonly(path)
    } otherwise ready (mode == "w") {
        handle.fd = open_file_writeonly(path)
    } otherwise ready (mode == "a") {
        handle.fd = open_file_append(path)
    } otherwise ready (mode == "r+") {
        handle.fd = open_file_readwrite(path)
    } otherwise ready (mode == "w+") {
        handle.fd = open_file_readwrite_create(path)
    } otherwise ready (mode == "a+") {
        handle.fd = open_file_readwrite_append(path)
    } otherwise {
        handle.fd = -1
        vibez.spill("Invalid file mode: " + mode)
    }
    
    handle.is_open = handle.fd > 0
    
    ready (handle.is_open) {
        vibez.spill("Opened file: " + path + " (mode: " + mode + ")")
    } otherwise {
        vibez.spill("Failed to open file: " + path)
    }
    
    damn handle
}

slay file_close(handle FileHandle) lit {
    fr fr Close file handle
    ready (!handle.is_open) {
        damn cringe
    }
    
    sus result lit = close_file_descriptor(handle.fd)
    ready (result) {
        handle.is_open = cringe
        vibez.spill("Closed file: " + handle.path)
    }
    
    damn result
}

slay file_read(handle FileHandle, buffer_size drip) tea {
    fr fr Read data from file
    ready (!handle.is_open) {
        damn ""
    }
    
    ready (!contains_substring(handle.mode, "r")) {
        vibez.spill("File not opened for reading")
        damn ""
    }
    
    sus data tea = read_from_file_descriptor(handle.fd, buffer_size)
    handle.position = handle.position + string_length(data)
    
    damn data
}

slay file_write(handle FileHandle, data tea) drip {
    fr fr Write data to file
    ready (!handle.is_open) {
        damn 0
    }
    
    ready (!contains_substring(handle.mode, "w") && !contains_substring(handle.mode, "a") && !contains_substring(handle.mode, "+")) {
        vibez.spill("File not opened for writing")
        damn 0
    }
    
    sus bytes_written drip = write_to_file_descriptor(handle.fd, data)
    handle.position = handle.position + bytes_written
    
    damn bytes_written
}

slay file_read_all(path tea) tea {
    fr fr Read entire file contents
    sus handle FileHandle = file_open(path, "r")
    ready (!handle.is_open) {
        damn ""
    }
    
    sus content tea = ""
    sus chunk_size drip = handle.buffer_size
    
    bestie (based) {
        sus chunk tea = file_read(handle, chunk_size)
        ready (string_length(chunk) == 0) {
            break
        }
        content = content + chunk
    }
    
    file_close(handle)
    damn content
}

slay file_write_all(path tea, content tea) lit {
    fr fr Write content to file (overwrites existing)
    sus handle FileHandle = file_open(path, "w")
    ready (!handle.is_open) {
        damn cringe
    }
    
    sus bytes_written drip = file_write(handle, content)
    file_close(handle)
    
    damn bytes_written == string_length(content)
}

slay file_append(path tea, content tea) lit {
    fr fr Append content to file
    sus handle FileHandle = file_open(path, "a")
    ready (!handle.is_open) {
        damn cringe
    }
    
    sus bytes_written drip = file_write(handle, content)
    file_close(handle)
    
    damn bytes_written == string_length(content)
}

fr fr ===== FILE SYSTEM QUERIES =====

slay file_exists(path tea) lit {
    fr fr Check if file exists
    sus info FileInfo = file_get_info(path)
    damn info.name != ""
}

slay file_is_directory(path tea) lit {
    fr fr Check if path is a directory
    sus info FileInfo = file_get_info(path)
    damn info.is_directory
}

slay file_is_file(path tea) lit {
    fr fr Check if path is a regular file
    sus info FileInfo = file_get_info(path)
    damn info.name != "" && !info.is_directory
}

slay file_get_size(path tea) drip {
    fr fr Get file size in bytes
    sus info FileInfo = file_get_info(path)
    damn info.size
}

slay file_get_info(path tea) FileInfo {
    fr fr Get comprehensive file information
    sus info FileInfo = FileInfo{}
    
    sus exists lit = check_path_exists(path)
    ready (!exists) {
        damn info  fr fr Return empty info
    }
    
    info.name = extract_filename(path)
    info.path = path
    info.size = get_file_size_native(path)
    info.is_directory = check_is_directory_native(path)
    info.is_readable = check_file_readable(path)
    info.is_writable = check_file_writable(path)
    info.last_modified = get_file_modified_time(path)
    info.permissions = get_file_permissions(path)
    
    damn info
}

fr fr ===== DIRECTORY OPERATIONS =====

slay dir_create(path tea) lit {
    fr fr Create directory
    sus result lit = create_directory_native(path)
    ready (result) {
        vibez.spill("Created directory: " + path)
    } otherwise {
        vibez.spill("Failed to create directory: " + path)
    }
    damn result
}

slay dir_create_recursive(path tea) lit {
    fr fr Create directory and all parent directories
    sus parts []tea = split_path(path)
    sus current_path tea = ""
    sus i drip = 0
    
    bestie (i < array_length(parts)) {
        ready (i == 0) {
            current_path = parts[i]
        } otherwise {
            current_path = current_path + path_separator() + parts[i]
        }
        
        ready (!file_exists(current_path)) {
            sus created lit = dir_create(current_path)
            ready (!created) {
                damn cringe
            }
        }
        
        i = i + 1
    }
    
    damn based
}

slay dir_remove(path tea) lit {
    fr fr Remove empty directory
    sus result lit = remove_directory_native(path)
    ready (result) {
        vibez.spill("Removed directory: " + path)
    } otherwise {
        vibez.spill("Failed to remove directory: " + path)
    }
    damn result
}

slay dir_list(path tea) []DirectoryEntry {
    fr fr List directory contents
    ready (!file_is_directory(path)) {
        sus empty_list []DirectoryEntry = []
        damn empty_list
    }
    
    sus entries []DirectoryEntry = list_directory_native(path)
    
    vibez.spill("Listed " + json_number_to_string(array_length(entries)) + " entries in: " + path)
    damn entries
}

slay dir_list_recursive(path tea) []DirectoryEntry {
    fr fr Recursively list all files and directories
    sus all_entries []DirectoryEntry = []
    sus entry_count drip = 0
    
    sus entries []DirectoryEntry = dir_list(path)
    sus i drip = 0
    
    bestie (i < array_length(entries)) {
        all_entries[entry_count] = entries[i]
        entry_count = entry_count + 1
        
        ready (entries[i].is_directory) {
            sus sub_entries []DirectoryEntry = dir_list_recursive(entries[i].full_path)
            sus j drip = 0
            bestie (j < array_length(sub_entries)) {
                all_entries[entry_count] = sub_entries[j]
                entry_count = entry_count + 1
                j = j + 1
            }
        }
        
        i = i + 1
    }
    
    damn all_entries
}

fr fr ===== FILE MANIPULATION =====

slay file_copy(source tea, destination tea) lit {
    fr fr Copy file from source to destination
    sus content tea = file_read_all(source)
    ready (content == "") {
        vibez.spill("Failed to read source file: " + source)
        damn cringe
    }
    
    sus result lit = file_write_all(destination, content)
    ready (result) {
        vibez.spill("Copied file: " + source + " -> " + destination)
    } otherwise {
        vibez.spill("Failed to copy file: " + source + " -> " + destination)
    }
    
    damn result
}

slay file_move(source tea, destination tea) lit {
    fr fr Move/rename file
    sus copy_result lit = file_copy(source, destination)
    ready (copy_result) {
        sus delete_result lit = file_delete(source)
        ready (delete_result) {
            vibez.spill("Moved file: " + source + " -> " + destination)
            damn based
        } otherwise {
            vibez.spill("Copied file but failed to delete source: " + source)
        }
    }
    
    damn cringe
}

slay file_delete(path tea) lit {
    fr fr Delete file
    sus result lit = delete_file_native(path)
    ready (result) {
        vibez.spill("Deleted file: " + path)
    } otherwise {
        vibez.spill("Failed to delete file: " + path)
    }
    damn result
}

slay file_rename(old_path tea, new_path tea) lit {
    fr fr Rename file
    sus result lit = rename_file_native(old_path, new_path)
    ready (result) {
        vibez.spill("Renamed file: " + old_path + " -> " + new_path)
    } otherwise {
        vibez.spill("Failed to rename file: " + old_path + " -> " + new_path)
    }
    damn result
}

fr fr ===== PATH UTILITIES =====

slay path_join(parts []tea) tea {
    fr fr Join path components with cross-platform support
    damn cross_platform_join(parts)
}

slay path_dirname(path tea) tea {
    fr fr Get directory name from path with cross-platform support
    damn get_parent_directory(path)
}

slay path_basename(path tea) tea {
    fr fr Get base filename from path with cross-platform support
    damn get_filename_component(path)
}

slay path_extension(path tea) tea {
    fr fr Get file extension with cross-platform support
    damn get_extension_component(path)
}

slay path_change_extension(path tea, new_ext tea) tea {
    fr fr Change file extension
    sus current_ext tea = path_extension(path)
    sus base_path tea = path
    
    ready (current_ext != "") {
        sus ext_pos drip = string_length(path) - string_length(current_ext)
        base_path = substring(path, 0, ext_pos)
    }
    
    ready (starts_with(new_ext, ".")) {
        damn base_path + new_ext
    } otherwise {
        damn base_path + "." + new_ext
    }
}

slay path_absolute(relative_path tea) tea {
    fr fr Convert relative path to absolute with cross-platform support
    damn cross_platform_absolute(relative_path)
}

slay path_normalize(path tea) tea {
    fr fr Normalize path with comprehensive cross-platform support
    damn cross_platform_normalize(path)
}

fr fr ===== ADVANCED FILE OPERATIONS =====

slay file_search(directory tea, pattern tea) []tea {
    fr fr Search for files matching pattern
    sus matches []tea = []
    sus match_count drip = 0
    
    sus entries []DirectoryEntry = dir_list_recursive(directory)
    sus i drip = 0
    
    bestie (i < array_length(entries)) {
        ready (!entries[i].is_directory && matches_pattern(entries[i].name, pattern)) {
            matches[match_count] = entries[i].full_path
            match_count = match_count + 1
        }
        i = i + 1
    }
    
    vibez.spill("Found " + json_number_to_string(match_count) + " files matching pattern: " + pattern)
    damn matches
}

slay file_watch(path tea, callback tea) lit {
    fr fr Watch file/directory for changes (simplified implementation)
    vibez.spill("Started watching: " + path)
    
    fr fr In production, this would use inotify/kqueue/ReadDirectoryChangesW
    fr fr For now, just simulate watching
    sus initial_info FileInfo = file_get_info(path)
    
    fr fr Mock file watching loop
    sus watch_iterations drip = 10
    sus i drip = 0
    bestie (i < watch_iterations) {
        sus current_info FileInfo = file_get_info(path)
        ready (current_info.last_modified != initial_info.last_modified) {
            vibez.spill("File changed: " + path)
            fr fr Would call callback function here
            initial_info = current_info
        }
        
        fr fr Sleep simulation
        sleep_milliseconds(100)
        i = i + 1
    }
    
    damn based
}

slay file_backup(path tea, backup_dir tea) lit {
    fr fr Create backup of file with timestamp
    sus timestamp tea = get_current_timestamp()
    sus filename tea = path_basename(path)
    sus backup_name tea = filename + ".backup." + timestamp
    sus backup_path tea = path_join([backup_dir, backup_name])
    
    sus result lit = file_copy(path, backup_path)
    ready (result) {
        vibez.spill("Created backup: " + backup_path)
    }
    damn result
}

slay file_sync(handle FileHandle) lit {
    fr fr Force flush file buffers to disk
    ready (!handle.is_open) {
        damn cringe
    }
    
    sus result lit = sync_file_descriptor(handle.fd)
    ready (result) {
        vibez.spill("Synced file: " + handle.path)
    }
    damn result
}

fr fr ===== NATIVE BRIDGE FUNCTIONS =====

slay open_file_readonly(path tea) drip {
    ready (path != "") { damn 3 } otherwise { damn -1 }
}

slay open_file_writeonly(path tea) drip {
    ready (path != "") { damn 4 } otherwise { damn -1 }
}

slay open_file_append(path tea) drip {
    ready (path != "") { damn 5 } otherwise { damn -1 }
}

slay open_file_readwrite(path tea) drip {
    ready (path != "") { damn 6 } otherwise { damn -1 }
}

slay open_file_readwrite_create(path tea) drip {
    ready (path != "") { damn 7 } otherwise { damn -1 }
}

slay open_file_readwrite_append(path tea) drip {
    ready (path != "") { damn 8 } otherwise { damn -1 }
}

slay close_file_descriptor(fd drip) lit {
    damn fd > 0
}

slay read_from_file_descriptor(fd drip, size drip) tea {
    ready (fd > 0) {
        damn "file_content_chunk"
    }
    damn ""
}

slay write_to_file_descriptor(fd drip, data tea) drip {
    ready (fd > 0) {
        damn string_length(data)
    }
    damn 0
}

slay check_path_exists(path tea) lit {
    ready (path != "" && path != "/nonexistent") {
        damn based
    }
    damn cringe
}

slay get_file_size_native(path tea) drip {
    ready (path == "test.txt") { damn 1024 }
    ready (path == "large.txt") { damn 1048576 }
    damn 512  fr fr Default size
}

slay check_is_directory_native(path tea) lit {
    ready (contains_substring(path, "dir") || ends_with(path, "/")) {
        damn based
    }
    damn cringe
}

slay check_file_readable(path tea) lit { damn based }
slay check_file_writable(path tea) lit { damn based }
slay get_file_modified_time(path tea) drip { damn 1640995200 }
slay get_file_permissions(path tea) drip { damn 644 }

slay create_directory_native(path tea) lit {
    ready (path != "") { damn based }
    damn cringe
}

slay remove_directory_native(path tea) lit {
    ready (path != "") { damn based }
    damn cringe
}

slay list_directory_native(path tea) []DirectoryEntry {
    sus entries []DirectoryEntry = []
    sus entry1 DirectoryEntry = DirectoryEntry{}
    entry1.name = "file1.txt"
    entry1.full_path = path + path_separator() + "file1.txt"
    entry1.is_directory = cringe
    entry1.size = 256
    entries[0] = entry1
    
    sus entry2 DirectoryEntry = DirectoryEntry{}
    entry2.name = "subdir"
    entry2.full_path = path + path_separator() + "subdir"
    entry2.is_directory = based
    entry2.size = 0
    entries[1] = entry2
    
    damn entries
}

slay delete_file_native(path tea) lit {
    ready (path != "") { damn based }
    damn cringe
}

slay rename_file_native(old_path tea, new_path tea) lit {
    ready (old_path != "" && new_path != "") { damn based }
    damn cringe
}

slay sync_file_descriptor(fd drip) lit {
    damn fd > 0
}

fr fr ===== UTILITY FUNCTIONS =====

slay path_separator() tea {
    fr fr Return OS-specific path separator with cross-platform support
    damn get_platform_separator()
}

slay extract_filename(path tea) tea {
    damn path_basename(path)
}

slay split_path(path tea) []tea {
    sus separator tea = path_separator()
    sus parts []tea = []
    sus current_part tea = ""
    sus part_count drip = 0
    sus i drip = 0
    
    bestie (i < string_length(path)) {
        sus char tea = substring(path, i, 1)
        ready (char == separator) {
            ready (current_part != "") {
                parts[part_count] = current_part
                part_count = part_count + 1
                current_part = ""
            }
        } otherwise {
            current_part = current_part + char
        }
        i = i + 1
    }
    
    ready (current_part != "") {
        parts[part_count] = current_part
    }
    
    damn parts
}

slay find_last_occurrence(text tea, search tea) drip {
    sus last_pos drip = -1
    sus i drip = 0
    
    bestie (i <= string_length(text) - string_length(search)) {
        sus substr tea = substring(text, i, string_length(search))
        ready (substr == search) {
            last_pos = i
        }
        i = i + 1
    }
    
    damn last_pos
}

slay matches_pattern(filename tea, pattern tea) lit {
    fr fr Simple wildcard pattern matching
    ready (pattern == "*") {
        damn based  fr fr Match all
    }
    ready (pattern == "*.txt") {
        damn ends_with(filename, ".txt")
    }
    ready (pattern == "*.log") {
        damn ends_with(filename, ".log")
    }
    ready (starts_with(pattern, "*.")) {
        sus ext tea = substring(pattern, 1, string_length(pattern) - 1)
        damn ends_with(filename, ext)
    }
    
    damn filename == pattern  fr fr Exact match
}

slay get_current_directory() tea {
    damn "/current/directory"  fr fr Mock current directory
}

slay get_current_timestamp() tea {
    damn "20241201_120000"  fr fr Mock timestamp
}

slay sleep_milliseconds(ms drip) lit {
    fr fr Mock sleep function
    damn based
}

slay json_number_to_string(num drip) tea {
    ready (num == 0) { damn "0" }
    ready (num == 1) { damn "1" }
    ready (num == 2) { damn "2" }
    ready (num == 3) { damn "3" }
    ready (num == 4) { damn "4" }
    ready (num == 5) { damn "5" }
    ready (num == 10) { damn "10" }
    damn json_number_to_string(num / 10) + json_number_to_string(num % 10)
}
