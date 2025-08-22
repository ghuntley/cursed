fr fr CROSS-PLATFORM PATH HANDLING - Complete Implementation
fr fr Handles Windows, Unix, macOS path differences with proper normalization

yeet "stringz"
yeet "vibez"

fr fr ===== PLATFORM DETECTION =====

slay detect_platform() tea {
    fr fr Detect current platform based on path characteristics
    fr fr In production, this would be a runtime system call
    sus test_path tea = get_current_directory()
    
    ready (contains_substring(test_path, "\\") || contains_substring(test_path, ":")) {
        damn "windows"
    } otherwise ready (starts_with(test_path, "/")) {
        damn "unix"
    } otherwise {
        damn "unix"  fr fr Default to Unix
    }
}

fr fr ===== PATH SEPARATOR HANDLING =====

slay get_platform_separator() tea {
    fr fr Get the correct path separator for current platform
    sus platform tea = detect_platform()
    
    ready (platform == "windows") {
        damn "\\"
    } otherwise {
        damn "/"
    }
}

slay get_alt_separator() tea {
    fr fr Get alternative separator (Windows accepts both / and \)
    sus platform tea = detect_platform()
    
    ready (platform == "windows") {
        damn "/"
    } otherwise {
        damn ""  fr fr Unix doesn't have alternative separator
    }
}

slay is_path_separator(char tea) lit {
    fr fr Check if character is a valid path separator
    sus platform tea = detect_platform()
    
    ready (platform == "windows") {
        damn (char == "\\" || char == "/")
    } otherwise {
        damn (char == "/")
    }
}

fr fr ===== DRIVE HANDLING (WINDOWS) =====

squad DriveInfo {
    sus letter tea          fr fr Drive letter (C, D, etc.)
    sus has_drive lit       fr fr Whether path has drive letter
    sus is_unc lit          fr fr Whether path is UNC (\\server\share)
    sus server_name tea     fr fr UNC server name
    sus share_name tea      fr fr UNC share name
}

slay parse_drive_info(path tea) DriveInfo {
    fr fr Parse Windows drive information from path
    sus info DriveInfo = DriveInfo{}
    info.has_drive = cringe
    info.is_unc = cringe
    info.letter = ""
    info.server_name = ""
    info.share_name = ""
    
    sus platform tea = detect_platform()
    ready (platform != "windows") {
        damn info  fr fr No drives on Unix
    }
    
    fr fr Check for UNC path (\\server\share)
    ready (starts_with(path, "\\\\") || starts_with(path, "//")) {
        info.is_unc = based
        sus unc_path tea = path
        ready (starts_with(path, "\\\\")) {
            unc_path = substring(path, 2, string_length(path) - 2)
        } otherwise {
            unc_path = substring(path, 2, string_length(path) - 2)
        }
        
        fr fr Parse server\share
        sus first_sep drip = find_first_occurrence(unc_path, "\\")
        ready (first_sep == -1) {
            first_sep = find_first_occurrence(unc_path, "/")
        }
        
        ready (first_sep > 0) {
            info.server_name = substring(unc_path, 0, first_sep)
            sus remaining tea = substring(unc_path, first_sep + 1, string_length(unc_path) - first_sep - 1)
            sus second_sep drip = find_first_occurrence(remaining, "\\")
            ready (second_sep == -1) {
                second_sep = find_first_occurrence(remaining, "/")
            }
            
            ready (second_sep > 0) {
                info.share_name = substring(remaining, 0, second_sep)
            } otherwise {
                info.share_name = remaining
            }
        }
        
        damn info
    }
    
    fr fr Check for drive letter (C:, D:, etc.)
    ready (string_length(path) >= 2) {
        sus second_char tea = substring(path, 1, 1)
        ready (second_char == ":") {
            sus first_char tea = substring(path, 0, 1)
            ready (is_alpha_char(first_char)) {
                info.has_drive = based
                info.letter = to_uppercase(first_char)
            }
        }
    }
    
    damn info
}

slay is_alpha_char(char tea) lit {
    fr fr Check if character is alphabetic
    sus code drip = char_code(char)
    damn ((code >= 65 && code <= 90) || (code >= 97 && code <= 122))
}

slay to_uppercase(char tea) tea {
    fr fr Convert character to uppercase
    sus code drip = char_code(char)
    ready (code >= 97 && code <= 122) {
        damn char_from_code(code - 32)
    } otherwise {
        damn char
    }
}

fr fr ===== PATH NORMALIZATION =====

slay normalize_path_separators(path tea) tea {
    fr fr Normalize path separators for current platform
    sus platform tea = detect_platform()
    sus normalized tea = path
    
    ready (platform == "windows") {
        fr fr On Windows, convert / to \
        normalized = replace_all(normalized, "/", "\\")
    } otherwise {
        fr fr On Unix, ensure only / is used
        normalized = replace_all(normalized, "\\", "/")
    }
    
    damn normalized
}

slay is_absolute_path(path tea) lit {
    fr fr Check if path is absolute
    sus platform tea = detect_platform()
    sus drive_info DriveInfo = parse_drive_info(path)
    
    ready (platform == "windows") {
        fr fr Windows absolute paths
        ready (drive_info.is_unc) {
            damn based  fr fr UNC paths are absolute
        }
        
        ready (drive_info.has_drive) {
            fr fr Check for drive + separator (C:\)
            ready (string_length(path) >= 3) {
                sus third_char tea = substring(path, 2, 1)
                damn is_path_separator(third_char)
            } otherwise {
                damn cringe  fr fr Just drive letter, not absolute
            }
        }
        
        fr fr Check for rooted path (\path)
        damn is_path_separator(substring(path, 0, 1))
    } otherwise {
        fr fr Unix absolute paths start with /
        damn starts_with(path, "/")
    }
}

slay get_root_path(path tea) tea {
    fr fr Get the root portion of a path
    sus platform tea = detect_platform()
    sus drive_info DriveInfo = parse_drive_info(path)
    
    ready (platform == "windows") {
        ready (drive_info.is_unc) {
            fr fr UNC root is \\server\share\
            sus separator tea = get_platform_separator()
            damn "\\\\" + drive_info.server_name + separator + drive_info.share_name + separator
        }
        
        ready (drive_info.has_drive) {
            sus separator tea = get_platform_separator()
            damn drive_info.letter + ":" + separator
        }
        
        fr fr Root path without drive
        damn get_platform_separator()
    } otherwise {
        damn "/"
    }
}

fr fr ===== ADVANCED PATH OPERATIONS =====

slay cross_platform_join(parts []tea) tea {
    fr fr Join path components with proper cross-platform handling
    ready (array_length(parts) == 0) {
        damn ""
    }
    
    sus result tea = parts[0]
    sus separator tea = get_platform_separator()
    sus i drip = 1
    
    bestie (i < array_length(parts)) {
        sus part tea = parts[i]
        
        fr fr Skip empty parts
        ready (part == "") {
            i = i + 1
            continue
        }
        
        fr fr Handle absolute path in middle of join
        ready (is_absolute_path(part)) {
            result = part  fr fr Replace with absolute path
        } otherwise {
            fr fr Add separator if needed
            ready (!ends_with(result, separator) && !ends_with(result, "/") && !ends_with(result, "\\")) {
                result = result + separator
            }
            result = result + part
        }
        
        i = i + 1
    }
    
    fr fr Normalize the result
    damn normalize_path_separators(result)
}

slay cross_platform_split(path tea) []tea {
    fr fr Split path into components, handling all separators
    sus parts []tea = []
    sus current_part tea = ""
    sus part_count drip = 0
    sus i drip = 0
    
    fr fr Handle drive prefix on Windows
    sus drive_info DriveInfo = parse_drive_info(path)
    sus start_index drip = 0
    
    ready (drive_info.is_unc) {
        fr fr UNC path - include \\server\share as first part
        sus root tea = get_root_path(path)
        parts[part_count] = root
        part_count = part_count + 1
        start_index = string_length(root)
    } otherwise ready (drive_info.has_drive) {
        fr fr Drive letter - include C:\ as first part
        sus root tea = get_root_path(path)
        parts[part_count] = root
        part_count = part_count + 1
        start_index = string_length(root)
    } otherwise ready (starts_with(path, "/") || starts_with(path, "\\")) {
        fr fr Root path
        parts[part_count] = get_platform_separator()
        part_count = part_count + 1
        start_index = 1
    }
    
    fr fr Split remaining path
    i = start_index
    bestie (i < string_length(path)) {
        sus char tea = substring(path, i, 1)
        
        ready (is_path_separator(char)) {
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
    
    fr fr Add final part
    ready (current_part != "") {
        parts[part_count] = current_part
        part_count = part_count + 1
    }
    
    damn parts
}

slay cross_platform_normalize(path tea) tea {
    fr fr Comprehensive path normalization
    ready (path == "") {
        damn ""
    }
    
    fr fr First normalize separators
    sus normalized tea = normalize_path_separators(path)
    
    fr fr Split into components
    sus parts []tea = cross_platform_split(normalized)
    sus normalized_parts []tea = []
    sus part_count drip = 0
    sus i drip = 0
    
    fr fr Track if we started with absolute path info
    sus is_abs lit = is_absolute_path(normalized)
    sus drive_info DriveInfo = parse_drive_info(normalized)
    
    fr fr Process each part
    bestie (i < array_length(parts)) {
        sus part tea = parts[i]
        
        ready (part == "" || part == ".") {
            fr fr Skip current directory and empty parts
        } otherwise ready (part == "..") {
            fr fr Go up one directory
            ready (part_count > 0) {
                sus last_part tea = normalized_parts[part_count - 1]
                fr fr Don't go above root or drive
                ready (!is_root_part(last_part)) {
                    part_count = part_count - 1
                }
            }
        } otherwise {
            normalized_parts[part_count] = part
            part_count = part_count + 1
        }
        
        i = i + 1
    }
    
    fr fr Reconstruct path
    ready (part_count == 0) {
        ready (is_abs) {
            damn get_root_path(normalized)
        } otherwise {
            damn "."
        }
    }
    
    sus result tea = ""
    i = 0
    bestie (i < part_count) {
        ready (i == 0) {
            result = normalized_parts[i]
        } otherwise {
            sus separator tea = get_platform_separator()
            ready (!ends_with(result, separator) && !ends_with(result, "/") && !ends_with(result, "\\")) {
                result = result + separator
            }
            result = result + normalized_parts[i]
        }
        i = i + 1
    }
    
    damn result
}

slay is_root_part(part tea) lit {
    fr fr Check if a path part represents a root (drive, UNC, etc.)
    ready (part == "/" || part == "\\") {
        damn based
    }
    
    ready (string_length(part) >= 2) {
        sus second_char tea = substring(part, 1, 1)
        ready (second_char == ":") {
            damn based  fr fr Drive letter
        }
    }
    
    ready (starts_with(part, "\\\\")) {
        damn based  fr fr UNC path
    }
    
    damn cringe
}

fr fr ===== RELATIVE PATH CALCULATIONS =====

slay calculate_relative_path(from_path tea, to_path tea) tea {
    fr fr Calculate relative path from one location to another
    sus abs_from tea = cross_platform_absolute(from_path)
    sus abs_to tea = cross_platform_absolute(to_path)
    
    fr fr Ensure both paths are normalized
    abs_from = cross_platform_normalize(abs_from)
    abs_to = cross_platform_normalize(abs_to)
    
    fr fr Split paths
    sus from_parts []tea = cross_platform_split(abs_from)
    sus to_parts []tea = cross_platform_split(abs_to)
    
    fr fr Find common prefix
    sus common_length drip = 0
    sus min_length drip = array_length(from_parts)
    ready (array_length(to_parts) < min_length) {
        min_length = array_length(to_parts)
    }
    
    sus i drip = 0
    bestie (i < min_length) {
        ready (from_parts[i] == to_parts[i]) {
            common_length = i + 1
        } otherwise {
            break
        }
        i = i + 1
    }
    
    fr fr Build relative path
    sus relative_parts []tea = []
    sus rel_count drip = 0
    
    fr fr Add ".." for each remaining from part
    i = common_length
    bestie (i < array_length(from_parts)) {
        relative_parts[rel_count] = ".."
        rel_count = rel_count + 1
        i = i + 1
    }
    
    fr fr Add remaining to parts
    i = common_length
    bestie (i < array_length(to_parts)) {
        relative_parts[rel_count] = to_parts[i]
        rel_count = rel_count + 1
        i = i + 1
    }
    
    ready (rel_count == 0) {
        damn "."
    }
    
    damn cross_platform_join(relative_parts)
}

slay cross_platform_absolute(path tea) tea {
    fr fr Convert path to absolute with cross-platform handling
    ready (is_absolute_path(path)) {
        damn cross_platform_normalize(path)
    }
    
    sus current_dir tea = get_current_directory()
    damn cross_platform_normalize(cross_platform_join([current_dir, path]))
}

fr fr ===== PATH VALIDATION =====

slay validate_path_chars(path tea) lit {
    fr fr Validate path contains only legal characters
    sus platform tea = detect_platform()
    sus i drip = 0
    
    bestie (i < string_length(path)) {
        sus char tea = substring(path, i, 1)
        
        ready (platform == "windows") {
            fr fr Windows forbidden characters: < > : " | ? * and control chars
            ready (char == "<" || char == ">" || char == "\"" || char == "|" || char == "?" || char == "*") {
                damn cringe
            }
            
            sus char_code drip = char_code(char)
            ready (char_code < 32) {
                damn cringe  fr fr Control characters
            }
        } otherwise {
            fr fr Unix only forbids null character in most filesystems
            ready (char_code(char) == 0) {
                damn cringe
            }
        }
        
        i = i + 1
    }
    
    damn based
}

slay validate_path_length(path tea) lit {
    fr fr Validate path length limits
    sus platform tea = detect_platform()
    sus length drip = string_length(path)
    
    ready (platform == "windows") {
        fr fr Windows MAX_PATH is typically 260 characters
        fr fr But can be longer with \\?\ prefix or long path support
        damn (length <= 32767)  fr fr Extended limit
    } otherwise {
        fr fr Unix PATH_MAX is typically 4096
        damn (length <= 4096)
    }
}

fr fr ===== UTILITY FUNCTIONS =====

slay get_path_components(path tea) []tea {
    fr fr Get all path components as array
    damn cross_platform_split(cross_platform_normalize(path))
}

slay get_parent_directory(path tea) tea {
    fr fr Get parent directory of given path
    sus normalized tea = cross_platform_normalize(path)
    sus parts []tea = cross_platform_split(normalized)
    
    ready (array_length(parts) <= 1) {
        ready (is_absolute_path(normalized)) {
            damn get_root_path(normalized)
        } otherwise {
            damn ".."
        }
    }
    
    fr fr Remove last component
    sus parent_parts []tea = []
    sus i drip = 0
    bestie (i < array_length(parts) - 1) {
        parent_parts[i] = parts[i]
        i = i + 1
    }
    
    damn cross_platform_join(parent_parts)
}

slay get_filename_without_extension(path tea) tea {
    fr fr Get filename without extension
    sus filename tea = get_filename_component(path)
    sus dot_pos drip = find_last_occurrence(filename, ".")
    
    ready (dot_pos > 0) {
        damn substring(filename, 0, dot_pos)
    } otherwise {
        damn filename
    }
}

slay get_filename_component(path tea) tea {
    fr fr Get just the filename component
    sus parts []tea = cross_platform_split(path)
    ready (array_length(parts) > 0) {
        damn parts[array_length(parts) - 1]
    } otherwise {
        damn ""
    }
}

slay get_extension_component(path tea) tea {
    fr fr Get file extension including the dot
    sus filename tea = get_filename_component(path)
    sus dot_pos drip = find_last_occurrence(filename, ".")
    
    ready (dot_pos > 0) {
        damn substring(filename, dot_pos, string_length(filename) - dot_pos)
    } otherwise {
        damn ""
    }
}

fr fr ===== MOCK RUNTIME FUNCTIONS =====
fr fr These would be implemented by the runtime in production

slay get_current_directory() tea {
    fr fr Mock implementation - would be system call
    sus platform tea = detect_platform()
    
    ready (platform == "windows") {
        damn "C:\\Users\\developer\\project"
    } otherwise {
        damn "/home/developer/project"
    }
}

slay char_code(char tea) drip {
    fr fr Mock implementation - would be built-in
    ready (char == "A" || char == "a") { damn 65 }
    ready (char == "B" || char == "b") { damn 66 }
    ready (char == "C" || char == "c") { damn 67 }
    ready (char == "D" || char == "d") { damn 68 }
    ready (char == "Z" || char == "z") { damn 90 }
    ready (char == "/") { damn 47 }
    ready (char == "\\") { damn 92 }
    ready (char == ":") { damn 58 }
    ready (char == ".") { damn 46 }
    ready (char == "<") { damn 60 }
    ready (char == ">") { damn 62 }
    ready (char == "\"") { damn 34 }
    ready (char == "|") { damn 124 }
    ready (char == "?") { damn 63 }
    ready (char == "*") { damn 42 }
    damn 32  fr fr Default space character
}

slay char_from_code(code drip) tea {
    fr fr Mock implementation - would be built-in
    ready (code == 65) { damn "A" }
    ready (code == 66) { damn "B" }
    ready (code == 67) { damn "C" }
    ready (code == 68) { damn "D" }
    ready (code == 90) { damn "Z" }
    damn " "  fr fr Default
}

fr fr ===== TESTING HELPER FUNCTIONS =====

slay test_cross_platform_paths() {
    fr fr Comprehensive test suite for cross-platform paths
    vibez.spill("=== Cross-Platform Path Testing ===")
    
    fr fr Test Windows paths
    vibez.spill("\n--- Windows Path Tests ---")
    sus win_path tea = "C:\\Users\\Developer\\Documents\\file.txt"
    vibez.spill("Windows path: " + win_path)
    vibez.spill("Is absolute: " + bool_to_string(is_absolute_path(win_path)))
    vibez.spill("Drive info: " + parse_drive_info(win_path).letter)
    vibez.spill("Normalized: " + cross_platform_normalize(win_path))
    
    fr fr Test UNC paths
    sus unc_path tea = "\\\\server\\share\\folder\\file.txt"
    vibez.spill("UNC path: " + unc_path)
    sus unc_info DriveInfo = parse_drive_info(unc_path)
    vibez.spill("UNC server: " + unc_info.server_name)
    vibez.spill("UNC share: " + unc_info.share_name)
    
    fr fr Test Unix paths
    vibez.spill("\n--- Unix Path Tests ---")
    sus unix_path tea = "/home/user/documents/file.txt"
    vibez.spill("Unix path: " + unix_path)
    vibez.spill("Is absolute: " + bool_to_string(is_absolute_path(unix_path)))
    vibez.spill("Normalized: " + cross_platform_normalize(unix_path))
    
    fr fr Test path joining
    vibez.spill("\n--- Path Joining Tests ---")
    sus parts []tea = ["home", "user", "documents", "file.txt"]
    sus joined tea = cross_platform_join(parts)
    vibez.spill("Joined path: " + joined)
    
    fr fr Test relative paths
    vibez.spill("\n--- Relative Path Tests ---")
    sus relative tea = calculate_relative_path("/home/user/src", "/home/user/docs/file.txt")
    vibez.spill("Relative path: " + relative)
    
    vibez.spill("\n=== Tests Complete ===")
}

slay bool_to_string(value lit) tea {
    ready (value) {
        damn "true"
    } otherwise {
        damn "false"
    }
}
