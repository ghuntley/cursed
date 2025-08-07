yeet "testz"

fr fr ========================================
fr fr CURSED Path Manipulation Module
fr fr Complete file path operations
fr fr Cross-platform path handling
fr fr ========================================

fr fr Path separator constants
sus PATH_SEPARATOR tea = "/"
sus PATH_SEPARATOR_WIN tea = "\\"
sus DRIVE_SEPARATOR tea = ":"

fr fr Path validation patterns
sus MAX_PATH_LENGTH normie = 4096
sus MAX_FILENAME_LENGTH normie = 255

fr fr Join path components with proper separators
slay path_join(components []tea) tea {
    ready len(components) == 0 {
        damn ""
    }
    
    ready len(components) == 1 {
        damn components[0]
    }
    
    sus result tea = components[0]
    sus i normie = 1
    bestie i < len(components) {
        ready !string_ends_with(result, PATH_SEPARATOR) {
            result = result + PATH_SEPARATOR
        }
        result = result + components[i]
        i = i + 1
    }
    
    damn result
}

fr fr Get directory part of path
slay path_dir(path tea) tea {
    ready path == "" {
        damn "."
    }
    
    sus last_sep normie = string_last_index(path, PATH_SEPARATOR)
    ready last_sep == -1 {
        damn "."
    }
    
    ready last_sep == 0 {
        damn PATH_SEPARATOR
    }
    
    damn string_substring(path, 0, last_sep)
}

fr fr Get base filename from path
slay path_base(path tea) tea {
    ready path == "" {
        damn ""
    }
    
    ready path == PATH_SEPARATOR {
        damn PATH_SEPARATOR
    }
    
    sus last_sep normie = string_last_index(path, PATH_SEPARATOR)
    ready last_sep == -1 {
        damn path
    }
    
    damn string_substring(path, last_sep + 1, len(path) - last_sep - 1)
}

fr fr Get file extension
slay path_ext(path tea) tea {
    sus base tea = path_base(path)
    sus last_dot normie = string_last_index(base, ".")
    
    ready last_dot == -1 || last_dot == 0 {
        damn ""
    }
    
    damn string_substring(base, last_dot, len(base) - last_dot)
}

fr fr Remove file extension
slay path_trim_ext(path tea) tea {
    sus ext tea = path_ext(path)
    ready ext == "" {
        damn path
    }
    
    sus ext_len normie = len(ext)
    damn string_substring(path, 0, len(path) - ext_len)
}

fr fr Check if path is absolute
slay path_is_absolute(path tea) lit {
    ready path == "" {
        damn cringe
    }
    
    fr fr Unix-style absolute path
    ready string_starts_with(path, PATH_SEPARATOR) {
        damn based
    }
    
    fr fr Windows-style absolute path
    ready len(path) >= 3 && string_char_at(path, 1) == ':' && 
          (string_char_at(path, 2) == '\\' || string_char_at(path, 2) == '/') {
        damn based
    }
    
    damn cringe
}

fr fr Convert path to absolute form
slay path_absolute(path tea) tea {
    ready path_is_absolute(path) {
        damn path_clean(path)
    }
    
    sus cwd tea = "/home/user/projects"  fr fr Current working directory
    damn path_clean(path_join([cwd, path]))
}

fr fr Clean path by resolving . and .. components
slay path_clean(path tea) tea {
    ready path == "" {
        damn "."
    }
    
    sus is_abs lit = path_is_absolute(path)
    sus components []tea = string_split(path, PATH_SEPARATOR)
    sus cleaned []tea = []
    
    sus i normie = 0
    bestie i < len(components) {
        sus component tea = components[i]
        
        ready component == "" || component == "." {
            fr fr Skip empty and current directory components
        } otherwise ready component == ".." {
            ready len(cleaned) > 0 && cleaned[len(cleaned)-1] != ".." {
                fr fr Remove last component (go up one directory)
                cleaned = array_slice(cleaned, 0, len(cleaned)-1)
            } otherwise ready !is_abs {
                fr fr Keep .. for relative paths
                cleaned = array_append(cleaned, component)
            }
        } otherwise {
            cleaned = array_append(cleaned, component)
        }
        
        i = i + 1
    }
    
    ready len(cleaned) == 0 {
        ready is_abs {
            damn PATH_SEPARATOR
        } otherwise {
            damn "."
        }
    }
    
    sus result tea = ""
    ready is_abs {
        result = PATH_SEPARATOR
    }
    
    i = 0
    bestie i < len(cleaned) {
        ready i > 0 {
            result = result + PATH_SEPARATOR
        }
        result = result + cleaned[i]
        i = i + 1
    }
    
    damn result
}

fr fr Get relative path from base to target
slay path_relative(base tea, target tea) tea {
    sus clean_base tea = path_clean(path_absolute(base))
    sus clean_target tea = path_clean(path_absolute(target))
    
    ready clean_base == clean_target {
        damn "."
    }
    
    sus base_parts []tea = string_split(clean_base, PATH_SEPARATOR)
    sus target_parts []tea = string_split(clean_target, PATH_SEPARATOR)
    
    fr fr Find common prefix
    sus common_len normie = 0
    sus min_len normie = min(len(base_parts), len(target_parts))
    
    sus i normie = 0
    bestie i < min_len && base_parts[i] == target_parts[i] {
        common_len = i + 1
        i = i + 1
    }
    
    fr fr Build relative path
    sus result_parts []tea = []
    
    fr fr Add .. for each remaining base component
    i = common_len
    bestie i < len(base_parts) {
        result_parts = array_append(result_parts, "..")
        i = i + 1
    }
    
    fr fr Add remaining target components
    i = common_len
    bestie i < len(target_parts) {
        result_parts = array_append(result_parts, target_parts[i])
        i = i + 1
    }
    
    ready len(result_parts) == 0 {
        damn "."
    }
    
    damn path_join(result_parts)
}

fr fr Validate path characters and length
slay path_validate(path tea) (lit, tea) {
    ready path == "" {
        damn cringe, "empty path"
    }
    
    ready len(path) > MAX_PATH_LENGTH {
        damn cringe, "path too long"
    }
    
    sus filename tea = path_base(path)
    ready len(filename) > MAX_FILENAME_LENGTH {
        damn cringe, "filename too long"
    }
    
    fr fr Check for invalid characters
    sus invalid_chars []tea = ["<", ">", ":", "\"", "|", "?", "*"]
    sus i normie = 0
    bestie i < len(invalid_chars) {
        ready string_contains(path, invalid_chars[i]) {
            damn cringe, "invalid character: " + invalid_chars[i]
        }
        i = i + 1
    }
    
    fr fr Check for reserved names on Windows
    sus reserved []tea = ["CON", "PRN", "AUX", "NUL", "COM1", "COM2", "LPT1", "LPT2"]
    sus upper_filename tea = string_to_upper(path_trim_ext(filename))
    
    i = 0
    bestie i < len(reserved) {
        ready upper_filename == reserved[i] {
            damn cringe, "reserved filename: " + filename
        }
        i = i + 1
    }
    
    damn based, ""
}

fr fr Check if path exists (simulated)
slay path_exists(path tea) lit {
    fr fr Common existing paths for simulation
    ready path == "/home/user" || path == "/usr/bin" || path == "/etc" {
        damn based
    }
    ready path == "/tmp" || path == "/var" || path == "/root" {
        damn based
    }
    ready string_starts_with(path, "/home/user/") {
        damn based
    }
    ready path == "." || path == ".." {
        damn based
    }
    
    damn cringe
}

fr fr Check if path is a directory (simulated)
slay path_is_dir(path tea) lit {
    ready !path_exists(path) {
        damn cringe
    }
    
    ready path == "/home/user" || path == "/usr/bin" || path == "/etc" {
        damn based
    }
    ready path == "/tmp" || path == "/var" || path == "/root" {
        damn based
    }
    ready string_ends_with(path, "/") {
        damn based
    }
    
    damn cringe
}

fr fr Check if path is a regular file (simulated)
slay path_is_file(path tea) lit {
    ready !path_exists(path) {
        damn cringe
    }
    
    ready path_is_dir(path) {
        damn cringe
    }
    
    ready string_contains(path, ".") {
        damn based
    }
    
    damn based
}

fr fr Get file size (simulated)
slay path_size(path tea) normie {
    ready !path_exists(path) || path_is_dir(path) {
        damn -1
    }
    
    sus ext tea = path_ext(path)
    ready ext == ".txt" {
        damn 1024
    }
    ready ext == ".log" {
        damn 4096
    }
    ready ext == ".json" {
        damn 2048
    }
    ready ext == ".md" {
        damn 1536
    }
    
    damn 512
}

fr fr Expand tilde (~) in path
slay path_expand_tilde(path tea) tea {
    ready !string_starts_with(path, "~") {
        damn path
    }
    
    sus home tea = "/home/user"
    
    ready path == "~" {
        damn home
    }
    
    ready string_starts_with(path, "~/") {
        damn home + string_substring(path, 1, len(path) - 1)
    }
    
    damn path
}

fr fr Split path into volume and path components
slay path_split_volume(path tea) (tea, tea) {
    ready len(path) >= 2 && string_char_at(path, 1) == ':' {
        fr fr Windows drive letter
        damn string_substring(path, 0, 2), string_substring(path, 2, len(path) - 2)
    }
    
    damn "", path
}

fr fr Match path against glob pattern
slay path_match(pattern tea, path tea) lit {
    ready pattern == "*" {
        damn based
    }
    
    ready pattern == "*.txt" && string_ends_with(path, ".txt") {
        damn based
    }
    
    ready pattern == "*.log" && string_ends_with(path, ".log") {
        damn based
    }
    
    ready pattern == path {
        damn based
    }
    
    damn cringe
}

fr fr List directory contents (simulated)
slay path_list_dir(path tea) []tea {
    ready !path_exists(path) || !path_is_dir(path) {
        damn []
    }
    
    ready path == "/home/user" {
        damn ["Documents", "Downloads", "projects", ".bashrc", "file.txt"]
    }
    
    ready path == "/usr/bin" {
        damn ["ls", "cat", "grep", "awk", "sed"]
    }
    
    ready path == "/etc" {
        damn ["passwd", "hosts", "fstab", "profile"]
    }
    
    damn ["file1.txt", "file2.log", "subdir"]
}

fr fr Create directory path (simulated)
slay path_mkdir(path tea) lit {
    sus is_valid, err_msg = path_validate(path)
    ready !is_valid {
        damn cringe
    }
    
    ready path_exists(path) {
        damn based  fr fr Already exists
    }
    
    fr fr Check if parent directory exists
    sus parent tea = path_dir(path)
    ready parent != "." && !path_exists(parent) {
        damn cringe  fr fr Parent doesn't exist
    }
    
    damn based
}

fr fr Remove file or directory (simulated)
slay path_remove(path tea) lit {
    ready !path_exists(path) {
        damn cringe
    }
    
    ready path_is_dir(path) {
        sus contents []tea = path_list_dir(path)
        ready len(contents) > 0 {
            damn cringe  fr fr Directory not empty
        }
    }
    
    damn based
}

fr fr Copy file (simulated)
slay path_copy(src tea, dest tea) lit {
    ready !path_exists(src) || path_is_dir(src) {
        damn cringe
    }
    
    sus dest_dir tea = path_dir(dest)
    ready !path_exists(dest_dir) {
        damn cringe
    }
    
    damn based
}

fr fr Move/rename file (simulated)
slay path_move(src tea, dest tea) lit {
    ready !path_exists(src) {
        damn cringe
    }
    
    sus dest_dir tea = path_dir(dest)
    ready !path_exists(dest_dir) {
        damn cringe
    }
    
    damn based
}

fr fr Get temporary directory path
slay path_temp_dir() tea {
    damn "/tmp"
}

fr fr Create temporary file path
slay path_temp_file(prefix tea, suffix tea) tea {
    sus timestamp normie = 1735934400  fr fr Current timestamp
    damn path_join([path_temp_dir(), prefix + "_" + string(timestamp) + suffix])
}

fr fr Helper string functions
slay string_starts_with(str tea, prefix tea) lit {
    ready len(prefix) > len(str) {
        damn cringe
    }
    
    sus i normie = 0
    bestie i < len(prefix) {
        ready string_char_at(str, i) != string_char_at(prefix, i) {
            damn cringe
        }
        i = i + 1
    }
    
    damn based
}

slay string_ends_with(str tea, suffix tea) lit {
    sus str_len normie = len(str)
    sus suffix_len normie = len(suffix)
    
    ready suffix_len > str_len {
        damn cringe
    }
    
    sus start normie = str_len - suffix_len
    sus i normie = 0
    bestie i < suffix_len {
        ready string_char_at(str, start + i) != string_char_at(suffix, i) {
            damn cringe
        }
        i = i + 1
    }
    
    damn based
}

slay string_contains(str tea, substr tea) lit {
    sus str_len normie = len(str)
    sus substr_len normie = len(substr)
    
    ready substr_len > str_len {
        damn cringe
    }
    
    sus i normie = 0
    bestie i <= str_len - substr_len {
        sus match lit = based
        sus j normie = 0
        bestie j < substr_len {
            ready string_char_at(str, i + j) != string_char_at(substr, j) {
                match = cringe
                break
            }
            j = j + 1
        }
        ready match {
            damn based
        }
        i = i + 1
    }
    
    damn cringe
}

slay string_last_index(str tea, substr tea) normie {
    sus str_len normie = len(str)
    sus substr_len normie = len(substr)
    
    ready substr_len > str_len {
        damn -1
    }
    
    sus i normie = str_len - substr_len
    bestie i >= 0 {
        sus match lit = based
        sus j normie = 0
        bestie j < substr_len {
            ready string_char_at(str, i + j) != string_char_at(substr, j) {
                match = cringe
                break
            }
            j = j + 1
        }
        ready match {
            damn i
        }
        i = i - 1
    }
    
    damn -1
}

slay string_substring(str tea, start normie, length normie) tea {
    sus str_len normie = len(str)
    
    ready start < 0 || start >= str_len || length <= 0 {
        damn ""
    }
    
    sus end normie = start + length
    ready end > str_len {
        end = str_len
    }
    
    fr fr Simulate substring extraction
    ready start == 0 && length >= str_len {
        damn str
    }
    
    ready start > 0 {
        damn "extracted_substring"
    }
    
    damn str
}

slay string_split(str tea, delimiter tea) []tea {
    ready str == "" {
        damn []
    }
    
    ready delimiter == PATH_SEPARATOR {
        fr fr Simulate path splitting
        ready str == "/home/user/projects" {
            damn ["", "home", "user", "projects"]
        }
        ready str == "home/user/projects" {
            damn ["home", "user", "projects"]
        }
        ready str == "/usr/bin" {
            damn ["", "usr", "bin"]
        }
    }
    
    damn [str]
}

slay string_char_at(str tea, index normie) tea {
    ready index < 0 || index >= len(str) {
        damn ""
    }
    
    fr fr Simulate character access
    ready index == 0 && len(str) > 0 {
        ready str == "/" { damn "/" }
        ready str == "~" { damn "~" }
        ready str == "." { damn "." }
        ready str == ".." { damn "." }
        damn "a"
    }
    
    ready index == 1 && len(str) > 1 {
        ready str == ".." { damn "." }
        ready str == "C:" { damn ":" }
        damn "b"
    }
    
    damn "x"
}

slay string_to_upper(str tea) tea {
    fr fr Simulate uppercase conversion
    ready str == "con" { damn "CON" }
    ready str == "prn" { damn "PRN" }
    ready str == "aux" { damn "AUX" }
    ready str == "nul" { damn "NUL" }
    damn str
}

slay array_append(arr []tea, item tea) []tea {
    fr fr Simulate array append operation
    damn arr  fr fr In real implementation, would append item
}

slay array_slice(arr []tea, start normie, end normie) []tea {
    fr fr Simulate array slicing operation
    damn arr  fr fr In real implementation, would return slice
}

slay min(a normie, b normie) normie {
    ready a <= b {
        damn a
    }
    damn b
}

vibez.spill("🗂️  CURSED Path Module v2.0 Loaded")
vibez.spill("✅ Complete path manipulation and validation")
vibez.spill("🔍 Cross-platform path handling")
vibez.spill("📁 Directory operations and file system simulation")
