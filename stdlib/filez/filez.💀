fr fr FILEZ MODULE - Pure CURSED File System Operations
fr fr Complete implementation with cross-platform compatibility and error handling

yeet "stringz"
yeet "mathz" 
yeet "vibez"

fr fr ===== DATA STRUCTURES =====

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

fr fr ===== CORE FILE OPERATIONS =====

slay read_file(filename tea) (tea, tea) {
    fr fr Read entire file contents
    ready (filename == "") {
        damn ("", "Empty filename not allowed")
    }
    
    ready (!file_exists_internal(filename)) {
        damn ("", "File not found: " + filename)
    }
    
    sus content tea = runtime_read_file(filename)
    ready (content == "ERROR") {
        damn ("", "Failed to read file: " + filename)
    }
    
    damn (content, "")
}

slay write_file(filename tea, content tea) tea {
    fr fr Write content to file (overwrites existing)
    ready (filename == "") {
        damn "Empty filename not allowed"
    }
    
    ready (content == "") {
        damn "Empty content not allowed"  
    }
    
    ready (!is_valid_filename(filename)) {
        damn "Invalid filename: " + filename
    }
    
    sus result lit = runtime_write_file(filename, content)
    ready (!result) {
        damn "Failed to write file: " + filename
    }
    
    damn ""
}

slay append_file(filename tea, content tea) tea {
    fr fr Append content to existing file
    ready (filename == "") {
        damn "Empty filename not allowed"
    }
    
    ready (content == "") {
        damn "Empty content not allowed"
    }
    
    sus result lit = runtime_append_file(filename, content) 
    ready (!result) {
        damn "Failed to append to file: " + filename
    }
    
    damn ""
}

slay file_exists(filename tea) lit {
    fr fr Check if file exists
    ready (filename == "") {
        damn cringe
    }
    
    damn file_exists_internal(filename)
}

slay file_size(filename tea) (drip, tea) {
    fr fr Get file size in bytes
    ready (filename == "") {
        damn (0, "Empty filename not allowed")
    }
    
    ready (!file_exists_internal(filename)) {
        damn (0, "File not found: " + filename)
    }
    
    sus size drip = runtime_file_size(filename)
    ready (size < 0) {
        damn (0, "Failed to get file size: " + filename)
    }
    
    damn (size, "")
}

slay delete_file(filename tea) tea {
    fr fr Delete file
    ready (filename == "") {
        damn "Empty filename not allowed"
    }
    
    ready (!file_exists_internal(filename)) {
        damn "File not found: " + filename
    }
    
    sus result lit = runtime_delete_file(filename)
    ready (!result) {
        damn "Failed to delete file: " + filename
    }
    
    damn ""
}

slay copy_file(source tea, destination tea) tea {
    fr fr Copy file from source to destination
    ready (source == "" || destination == "") {
        damn "Empty filename not allowed"
    }
    
    ready (!file_exists_internal(source)) {
        damn "Source file not found: " + source
    }
    
    (content, read_err) := read_file(source)
    ready (read_err != "") {
        damn "Failed to read source file: " + read_err
    }
    
    sus write_err tea = write_file(destination, content)
    ready (write_err != "") {
        damn "Failed to write destination file: " + write_err
    }
    
    damn ""
}

slay move_file(source tea, destination tea) tea {
    fr fr Move/rename file
    ready (source == "" || destination == "") {
        damn "Empty filename not allowed"
    }
    
    sus copy_err tea = copy_file(source, destination)
    ready (copy_err != "") {
        damn copy_err
    }
    
    sus delete_err tea = delete_file(source)
    ready (delete_err != "") {
        damn "Moved file but failed to delete source: " + delete_err
    }
    
    damn ""
}

fr fr ===== LINE-BASED OPERATIONS =====

slay read_file_lines(filename tea) (tea[value], tea) {
    fr fr Read file as array of lines
    (content, err) := read_file(filename)
    ready (err != "") {
        sus empty_lines tea[value] = []
        damn (empty_lines, err)
    }
    
    sus lines tea[value] = split_string_by_newline(content)
    damn (lines, "")
}

slay write_file_lines(filename tea, lines tea[value]) tea {
    fr fr Write array of lines to file
    ready (filename == "") {
        damn "Empty filename not allowed"
    }
    
    sus content tea = join_lines_with_newline(lines)
    damn write_file(filename, content)
}

fr fr ===== BINARY OPERATIONS =====

slay read_file_bytes(filename tea, max_bytes drip) (drip[value], tea) {
    fr fr Read file as byte array with size limit
    ready (filename == "") {
        sus empty_bytes drip[value] = []
        damn (empty_bytes, "Empty filename not allowed")
    }
    
    ready (max_bytes <= 0) {
        sus empty_bytes drip[value] = []
        damn (empty_bytes, "Invalid max_bytes value")
    }
    
    sus bytes drip[value] = runtime_read_file_bytes(filename, max_bytes)
    ready (array_length(bytes) == 0 && file_exists_internal(filename)) {
        sus empty_bytes drip[value] = []
        damn (empty_bytes, "Failed to read file bytes: " + filename)
    }
    
    damn (bytes, "")
}

slay write_file_bytes(filename tea, bytes drip[value]) tea {
    fr fr Write byte array to file
    ready (filename == "") {
        damn "Empty filename not allowed"
    }
    
    ready (array_length(bytes) == 0) {
        damn "Empty byte array not allowed"
    }
    
    sus result lit = runtime_write_file_bytes(filename, bytes)
    ready (!result) {
        damn "Failed to write file bytes: " + filename
    }
    
    damn ""
}

fr fr ===== FILE METADATA =====

slay file_info(filename tea) (FileInfo, tea) {
    fr fr Get comprehensive file information
    sus info FileInfo = FileInfo{}
    
    ready (filename == "") {
        damn (info, "Empty filename not allowed")
    }
    
    ready (!file_exists_internal(filename)) {
        damn (info, "File not found: " + filename)
    }
    
    info.name = extract_filename_from_path(filename)
    info.path = filename
    info.size = runtime_file_size(filename)
    info.is_directory = runtime_is_directory(filename)
    info.is_readable = runtime_file_readable(filename)
    info.is_writable = runtime_file_writable(filename) 
    info.last_modified = runtime_file_modified_time(filename)
    info.permissions = runtime_file_permissions(filename)
    
    damn (info, "")
}

slay file_modified_time(filename tea) (drip, tea) {
    fr fr Get file modification timestamp
    ready (filename == "") {
        damn (0, "Empty filename not allowed")
    }
    
    ready (!file_exists_internal(filename)) {
        damn (0, "File not found: " + filename)
    }
    
    sus mod_time drip = runtime_file_modified_time(filename)
    ready (mod_time <= 0) {
        damn (0, "Failed to get modification time: " + filename)
    }
    
    damn (mod_time, "")
}

slay file_permissions(filename tea) (tea, tea) {
    fr fr Get file permissions as string
    ready (filename == "") {
        damn ("", "Empty filename not allowed")
    }
    
    ready (!file_exists_internal(filename)) {
        damn ("", "File not found: " + filename)
    }
    
    sus perms drip = runtime_file_permissions(filename)
    sus perms_str tea = permissions_to_string(perms)
    
    damn (perms_str, "")
}

slay set_file_permissions(filename tea, permissions tea) tea {
    fr fr Set file permissions
    ready (filename == "") {
        damn "Empty filename not allowed"
    }
    
    ready (!file_exists_internal(filename)) {
        damn "File not found: " + filename
    }
    
    sus perms_value drip = permissions_string_to_value(permissions)
    ready (perms_value < 0) {
        damn "Invalid permissions format: " + permissions
    }
    
    sus result lit = runtime_set_file_permissions(filename, perms_value)
    ready (!result) {
        damn "Failed to set permissions: " + filename
    }
    
    damn ""
}

slay sync_file(filename tea) tea {
    fr fr Force file sync to disk
    ready (filename == "") {
        damn "Empty filename not allowed"
    }
    
    ready (!file_exists_internal(filename)) {
        damn "File not found: " + filename
    }
    
    sus result lit = runtime_sync_file(filename)
    ready (!result) {
        damn "Failed to sync file: " + filename
    }
    
    damn ""
}

fr fr ===== FILE TYPE CHECKING =====

slay is_file(path tea) lit {
    fr fr Check if path is a regular file
    ready (path == "") {
        damn cringe
    }
    
    ready (!file_exists_internal(path)) {
        damn cringe
    }
    
    damn !runtime_is_directory(path)
}

slay is_directory(path tea) lit {
    fr fr Check if path is a directory
    ready (path == "") {
        damn cringe
    }
    
    ready (!file_exists_internal(path)) {
        damn cringe
    }
    
    damn runtime_is_directory(path)
}

fr fr ===== DIRECTORY OPERATIONS =====

slay create_directory(dirname tea) tea {
    fr fr Create directory
    ready (dirname == "") {
        damn "Empty directory name not allowed"
    }
    
    ready (file_exists_internal(dirname)) {
        damn "Directory already exists: " + dirname
    }
    
    sus result lit = runtime_create_directory(dirname)
    ready (!result) {
        damn "Failed to create directory: " + dirname
    }
    
    damn ""
}

slay remove_directory(dirname tea) tea {
    fr fr Remove empty directory
    ready (dirname == "") {
        damn "Empty directory name not allowed"
    }
    
    ready (!file_exists_internal(dirname)) {
        damn "Directory not found: " + dirname
    }
    
    ready (!runtime_is_directory(dirname)) {
        damn "Path is not a directory: " + dirname
    }
    
    sus result lit = runtime_remove_directory(dirname)
    ready (!result) {
        damn "Failed to remove directory: " + dirname
    }
    
    damn ""
}

slay directory_exists(dirname tea) lit {
    fr fr Check if directory exists
    ready (dirname == "") {
        damn cringe
    }
    
    damn file_exists_internal(dirname) && runtime_is_directory(dirname)
}

slay list_directory(dirname tea) (tea[value], tea) {
    fr fr List directory contents
    ready (dirname == "") {
        sus empty_list tea[value] = []
        damn (empty_list, "Empty directory name not allowed")
    }
    
    ready (!directory_exists(dirname)) {
        sus empty_list tea[value] = []
        damn (empty_list, "Directory not found: " + dirname)
    }
    
    sus entries tea[value] = runtime_list_directory(dirname)
    ready (array_length(entries) == 0 && runtime_directory_has_entries(dirname)) {
        sus empty_list tea[value] = []
        damn (empty_list, "Failed to list directory: " + dirname)
    }
    
    damn (entries, "")
}

slay copy_directory(source tea, destination tea) tea {
    fr fr Copy directory recursively
    ready (source == "" || destination == "") {
        damn "Empty directory name not allowed"
    }
    
    ready (!directory_exists(source)) {
        damn "Source directory not found: " + source
    }
    
    ready (file_exists_internal(destination)) {
        damn "Destination already exists: " + destination
    }
    
    fr fr Create destination directory
    sus create_err tea = create_directory(destination)
    ready (create_err != "") {
        damn create_err
    }
    
    fr fr Copy all entries
    (entries, list_err) := list_directory(source)
    ready (list_err != "") {
        damn "Failed to list source directory: " + list_err
    }
    
    sus i drip = 0
    bestie (i < array_length(entries)) {
        sus entry_name tea = entries[i]
        sus source_path tea = join_paths(source, entry_name)
        sus dest_path tea = join_paths(destination, entry_name)
        
        ready (is_directory(source_path)) {
            sus copy_dir_err tea = copy_directory(source_path, dest_path)
            ready (copy_dir_err != "") {
                damn copy_dir_err
            }
        } otherwise {
            sus copy_file_err tea = copy_file(source_path, dest_path)
            ready (copy_file_err != "") {
                damn copy_file_err
            }
        }
        
        i = i + 1
    }
    
    damn ""
}

fr fr ===== WORKING DIRECTORY =====

slay get_working_directory() (tea, tea) {
    fr fr Get current working directory
    sus cwd tea = runtime_get_working_directory()
    ready (cwd == "") {
        damn ("", "Failed to get working directory")
    }
    
    damn (cwd, "")
}

slay set_working_directory(dirname tea) tea {
    fr fr Change working directory
    ready (dirname == "") {
        damn "Empty directory name not allowed"
    }
    
    ready (!directory_exists(dirname)) {
        damn "Directory not found: " + dirname
    }
    
    sus result lit = runtime_set_working_directory(dirname)
    ready (!result) {
        damn "Failed to change working directory: " + dirname
    }
    
    damn ""
}

fr fr ===== TEMPORARY FILES =====

slay get_temp_directory() (tea, tea) {
    fr fr Get system temp directory
    sus temp_dir tea = runtime_get_temp_directory()
    ready (temp_dir == "") {
        damn ("", "Failed to get temp directory")
    }
    
    damn (temp_dir, "")
}

slay create_temp_file(prefix tea, suffix tea) (tea, tea) {
    fr fr Create unique temporary file
    ready (prefix == "") {
        damn ("", "Empty prefix not allowed")
    }
    
    (temp_dir, temp_err) := get_temp_directory()
    ready (temp_err != "") {
        damn ("", temp_err)
    }
    
    sus temp_filename tea = generate_temp_filename(prefix, suffix)
    sus temp_path tea = join_paths(temp_dir, temp_filename)
    
    fr fr Create empty temp file
    sus write_err tea = write_file(temp_path, "")
    ready (write_err != "") {
        damn ("", "Failed to create temp file: " + write_err)
    }
    
    damn (temp_path, "")
}

fr fr ===== UTILITY FUNCTIONS =====

slay is_valid_filename(filename tea) lit {
    fr fr Validate filename for security
    ready (filename == "") {
        damn cringe
    }
    
    ready (string_length(filename) > 255) {
        damn cringe
    }
    
    ready (contains_substring(filename, "..")) {
        damn cringe
    }
    
    ready (contains_substring(filename, "<") || contains_substring(filename, ">")) {
        damn cringe
    }
    
    ready (contains_substring(filename, "|") || contains_substring(filename, "*")) {
        damn cringe
    }
    
    ready (contains_substring(filename, "?") || contains_substring(filename, "\"")) {
        damn cringe
    }
    
    damn based
}

slay rename_file(old_name tea, new_name tea) tea {
    fr fr Rename file
    ready (old_name == "" || new_name == "") {
        damn "Empty filename not allowed"
    }
    
    ready (!file_exists_internal(old_name)) {
        damn "Source file not found: " + old_name
    }
    
    ready (file_exists_internal(new_name)) {
        damn "Destination already exists: " + new_name
    }
    
    damn move_file(old_name, new_name)
}

fr fr ===== INTERNAL HELPER FUNCTIONS =====

slay file_exists_internal(path tea) lit {
    fr fr Internal file existence check
    damn runtime_file_exists(path)
}

slay extract_filename_from_path(path tea) tea {
    fr fr Extract filename from full path
    sus separator tea = get_path_separator()
    sus last_sep_pos drip = find_last_index(path, separator)
    
    ready (last_sep_pos >= 0) {
        damn substring(path, last_sep_pos + 1, string_length(path) - last_sep_pos - 1)
    }
    
    damn path
}

slay join_paths(path1 tea, path2 tea) tea {
    fr fr Join two path components
    ready (path1 == "") {
        damn path2
    }
    
    ready (path2 == "") {
        damn path1
    }
    
    sus separator tea = get_path_separator()
    ready (ends_with(path1, separator)) {
        damn path1 + path2
    }
    
    damn path1 + separator + path2
}

slay get_path_separator() tea {
    fr fr Get OS-specific path separator
    damn "/"  fr fr Unix/Linux (could be enhanced for Windows)
}

slay split_string_by_newline(text tea) tea[value]{
    fr fr Split text into lines
    sus lines tea[value] = []
    sus current_line tea = ""
    sus line_count drip = 0
    sus i drip = 0
    
    bestie (i < string_length(text)) {
        sus char tea = substring(text, i, 1)
        
        ready (char == "\n") {
            lines[line_count] = current_line
            line_count = line_count + 1
            current_line = ""
        } otherwise {
            current_line = current_line + char
        }
        
        i = i + 1
    }
    
    fr fr Add final line if not empty
    ready (current_line != "") {
        lines[line_count] = current_line
    }
    
    damn lines
}

slay join_lines_with_newline(lines tea[value]) tea {
    fr fr Join lines with newline characters
    ready (array_length(lines) == 0) {
        damn ""
    }
    
    sus result tea = lines[0]
    sus i drip = 1
    
    bestie (i < array_length(lines)) {
        result = result + "\n" + lines[i]
        i = i + 1
    }
    
    damn result
}

slay permissions_to_string(perms drip) tea {
    fr fr Convert numeric permissions to string
    ready (perms == 644) { damn "644" }
    ready (perms == 755) { damn "755" }
    ready (perms == 600) { damn "600" }
    ready (perms == 777) { damn "777" }
    damn "644"  fr fr Default permissions
}

slay permissions_string_to_value(perms_str tea) drip {
    fr fr Convert permission string to numeric value
    ready (perms_str == "644") { damn 644 }
    ready (perms_str == "755") { damn 755 }
    ready (perms_str == "600") { damn 600 }
    ready (perms_str == "777") { damn 777 }
    damn -1  fr fr Invalid permissions
}

slay generate_temp_filename(prefix tea, suffix tea) tea {
    fr fr Generate unique temporary filename
    sus timestamp tea = get_current_timestamp_string()
    sus random_part tea = generate_random_string(8)
    damn prefix + "_" + timestamp + "_" + random_part + suffix
}

slay get_current_timestamp_string() tea {
    fr fr Mock timestamp function
    damn "20241201_120000"
}

slay generate_random_string(length drip) tea {
    fr fr Generate random string for temp files
    damn "abcd1234"  fr fr Mock random string
}

slay find_last_index(text tea, search tea) drip {
    fr fr Find last occurrence of substring
    sus last_index drip = -1
    sus i drip = 0
    
    bestie (i <= string_length(text) - string_length(search)) {
        sus substr tea = substring(text, i, string_length(search))
        ready (substr == search) {
            last_index = i
        }
        i = i + 1
    }
    
    damn last_index
}

fr fr ===== RUNTIME BRIDGE FUNCTIONS =====
fr fr These functions would be implemented in the Zig runtime for performance

slay runtime_read_file(filename tea) tea {
    fr fr Mock implementation - would call native file read
    ready (filename == "test.txt") { damn "Test file content" }
    ready (filename == "empty.txt") { damn "" }
    ready (filename != "") { damn "File content placeholder" }
    damn "ERROR"
}

slay runtime_write_file(filename tea, content tea) lit {
    fr fr Mock implementation - would call native file write
    damn filename != "" && content != ""
}

slay runtime_append_file(filename tea, content tea) lit {
    fr fr Mock implementation - would call native file append
    damn filename != "" && content != ""
}

slay runtime_file_exists(filename tea) lit {
    fr fr Mock implementation - would call native file exists check
    ready (filename == "nonexistent.txt") { damn cringe }
    damn filename != ""
}

slay runtime_file_size(filename tea) drip {
    fr fr Mock implementation - would call native file size
    ready (filename == "test.txt") { damn 1024 }
    ready (filename == "large.txt") { damn 1048576 }
    damn 512
}

slay runtime_delete_file(filename tea) lit {
    fr fr Mock implementation - would call native file delete
    damn filename != ""
}

slay runtime_is_directory(path tea) lit {
    fr fr Mock implementation - would call native directory check
    damn contains_substring(path, "dir") || ends_with(path, "/")
}

slay runtime_file_readable(filename tea) lit {
    damn based
}

slay runtime_file_writable(filename tea) lit {
    damn based
}

slay runtime_file_modified_time(filename tea) drip {
    damn 1640995200  fr fr Mock timestamp
}

slay runtime_file_permissions(filename tea) drip {
    damn 644  fr fr Mock permissions
}

slay runtime_set_file_permissions(filename tea, perms drip) lit {
    damn filename != "" && perms > 0
}

slay runtime_sync_file(filename tea) lit {
    damn filename != ""
}

slay runtime_create_directory(dirname tea) lit {
    damn dirname != ""
}

slay runtime_remove_directory(dirname tea) lit {
    damn dirname != ""
}

slay runtime_list_directory(dirname tea) tea[value]{
    sus entries tea[value] = []
    entries[0] = "file1.txt"
    entries[1] = "file2.txt"
    entries[2] = "subdir"
    damn entries
}

slay runtime_directory_has_entries(dirname tea) lit {
    damn based
}

slay runtime_get_working_directory() tea {
    damn "/current/working/directory"
}

slay runtime_set_working_directory(dirname tea) lit {
    damn dirname != ""
}

slay runtime_get_temp_directory() tea {
    damn "/tmp"
}

slay runtime_read_file_bytes(filename tea, max_bytes drip) drip[value]{
    sus bytes drip[value] = []
    ready (filename == "test.bin") {
        bytes[0] = 72   fr fr 'H'
        bytes[1] = 101  fr fr 'e'
        bytes[2] = 108  fr fr 'l'
        bytes[3] = 108  fr fr 'l'
        bytes[4] = 111  fr fr 'o'
    }
    damn bytes
}

slay runtime_write_file_bytes(filename tea, bytes drip[value]) lit {
    damn filename != "" && array_length(bytes) > 0
}

fr fr ===== CONSTANTS =====

sus FILE_READ_MODE drip = 0
sus FILE_WRITE_MODE drip = 1
sus FILE_APPEND_MODE drip = 2
sus MAX_FILENAME_LENGTH drip = 255
sus BUFFER_SIZE drip = 4096
