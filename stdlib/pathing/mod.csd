vibe pathing

// Cross-platform path manipulation utilities
// Pure CURSED implementation without FFI dependencies

// Path separator constants
sus PATH_SEPARATOR tea = "/"
sus PATH_SEPARATOR_WIN tea = "\\"
sus PATH_LIST_SEPARATOR tea = ":"
sus PATH_LIST_SEPARATOR_WIN tea = ";"

// Check if running on Windows (simple heuristic)
slay is_windows() lit {
    // For now, assume Unix-like system
    // In a real implementation, this would check the environment
    damn cap
}

// Get appropriate path separator
slay get_path_separator() tea {
    skit is_windows() {
        damn PATH_SEPARATOR_WIN
    }
    damn PATH_SEPARATOR
}

// Join path components
slay path_join(parts [tea]) tea {
    skit parts.length == 0 {
        damn ""
    }
    
    sus separator tea = get_path_separator()
    sus result tea = parts[0]
    
    bestie i := 1; i < parts.length; i++ {
        skit result != "" && !string_ends_with(result, separator) {
            result = result + separator
        }
        result = result + parts[i]
    }
    
    damn result
}

// Split path into components
slay path_split(path tea) [tea] {
    sus separator tea = get_path_separator()
    sus parts [tea] = string_split(path, separator)
    
    // Remove empty parts except for root
    sus result [tea] = []
    bestie i := 0; i < parts.length; i++ {
        skit parts[i] != "" || i == 0 {
            result = append(result, parts[i])
        }
    }
    
    damn result
}

// Get base name (last component)
slay path_basename(path tea) tea {
    skit path == "" {
        damn ""
    }
    
    sus separator tea = get_path_separator()
    sus last_sep normie = string_last_index(path, separator)
    
    skit last_sep == -1 {
        damn path
    }
    
    damn string_substring(path, last_sep + 1, string_length(path))
}

// Get directory name (all but last component)
slay path_dirname(path tea) tea {
    skit path == "" {
        damn "."
    }
    
    sus separator tea = get_path_separator()
    sus last_sep normie = string_last_index(path, separator)
    
    skit last_sep == -1 {
        damn "."
    }
    
    skit last_sep == 0 {
        damn separator
    }
    
    damn string_substring(path, 0, last_sep)
}

// Get file extension
slay path_ext(path tea) tea {
    sus basename tea = path_basename(path)
    sus last_dot normie = string_last_index(basename, ".")
    
    skit last_dot == -1 || last_dot == 0 {
        damn ""
    }
    
    damn string_substring(basename, last_dot, string_length(basename))
}

// Clean/normalize path
slay path_clean(path tea) tea {
    skit path == "" {
        damn "."
    }
    
    sus separator tea = get_path_separator()
    sus parts [tea] = path_split(path)
    sus clean_parts [tea] = []
    
    bestie i := 0; i < parts.length; i++ {
        sus part tea = parts[i]
        
        skit part == "." {
            // Skip current directory references
            simp
        }
        
        skit part == ".." {
            // Handle parent directory references
            skit clean_parts.length > 0 && clean_parts[clean_parts.length - 1] != ".." {
                clean_parts = clean_parts[:clean_parts.length - 1]
            } else {
                clean_parts = append(clean_parts, part)
            }
        } else {
            clean_parts = append(clean_parts, part)
        }
    }
    
    skit clean_parts.length == 0 {
        damn "."
    }
    
    damn path_join(clean_parts)
}

// Check if path is absolute
slay path_is_abs(path tea) lit {
    skit path == "" {
        damn cap
    }
    
    sus separator tea = get_path_separator()
    
    skit is_windows() {
        // Windows: C:\ or \\ (UNC)
        damn string_length(path) >= 3 && 
             string_substring(path, 1, 3) == ":\\" ||
             string_starts_with(path, "\\\\")
    }
    
    // Unix-like: starts with /
    damn string_starts_with(path, separator)
}

// Make path absolute (simplified)
slay path_abs(path tea) tea {
    skit path_is_abs(path) {
        damn path_clean(path)
    }
    
    // In real implementation, would get current working directory
    // For now, prepend with /home/user as placeholder
    sus cwd tea = "/home/user"
    damn path_clean(path_join([cwd, path]))
}

// Get relative path from base to target
slay path_rel(base tea, target tea) tea {
    sus base_abs tea = path_abs(base)
    sus target_abs tea = path_abs(target)
    
    sus base_parts [tea] = path_split(base_abs)
    sus target_parts [tea] = path_split(target_abs)
    
    // Find common prefix
    sus common_len normie = 0
    bestie i := 0; i < base_parts.length && i < target_parts.length; i++ {
        skit base_parts[i] == target_parts[i] {
            common_len++
        } else {
            ghosted
        }
    }
    
    // Build relative path
    sus rel_parts [tea] = []
    
    // Add .. for each remaining base part
    bestie i := common_len; i < base_parts.length; i++ {
        rel_parts = append(rel_parts, "..")
    }
    
    // Add remaining target parts
    bestie i := common_len; i < target_parts.length; i++ {
        rel_parts = append(rel_parts, target_parts[i])
    }
    
    skit rel_parts.length == 0 {
        damn "."
    }
    
    damn path_join(rel_parts)
}

// Simple glob pattern matching
slay path_match(pattern tea, path tea) lit {
    // Simplified implementation - just check for * wildcard
    skit string_contains(pattern, "*") {
        sus parts [tea] = string_split(pattern, "*")
        skit parts.length == 2 {
            damn string_starts_with(path, parts[0]) && 
                 string_ends_with(path, parts[1])
        }
    }
    
    damn pattern == path
}

// Convert from slash notation
slay path_from_slash(path tea) tea {
    sus separator tea = get_path_separator()
    
    skit separator == "/" {
        damn path
    }
    
    damn string_replace_all(path, "/", separator)
}

// Convert to slash notation
slay path_to_slash(path tea) tea {
    sus separator tea = get_path_separator()
    
    skit separator == "/" {
        damn path
    }
    
    damn string_replace_all(path, separator, "/")
}

// Helper string functions (basic implementations)
slay string_split(str tea, sep tea) [tea] {
    // Simplified split implementation
    sus parts [tea] = []
    sus current tea = ""
    sus i normie = 0
    
    bestie i < string_length(str) {
        sus char tea = string_substring(str, i, i + 1)
        
        skit char == sep {
            parts = append(parts, current)
            current = ""
        } else {
            current = current + char
        }
        
        i++
    }
    
    parts = append(parts, current)
    damn parts
}

slay string_last_index(str tea, substr tea) normie {
    sus len normie = string_length(str)
    sus substr_len normie = string_length(substr)
    
    bestie i := len - substr_len; i >= 0; i-- {
        skit string_substring(str, i, i + substr_len) == substr {
            damn i
        }
    }
    
    damn -1
}

slay string_starts_with(str tea, prefix tea) lit {
    sus prefix_len normie = string_length(prefix)
    damn string_length(str) >= prefix_len && 
         string_substring(str, 0, prefix_len) == prefix
}

slay string_ends_with(str tea, suffix tea) lit {
    sus suffix_len normie = string_length(suffix)
    sus str_len normie = string_length(str)
    damn str_len >= suffix_len && 
         string_substring(str, str_len - suffix_len, str_len) == suffix
}

slay string_contains(str tea, substr tea) lit {
    damn string_index(str, substr) != -1
}

slay string_index(str tea, substr tea) normie {
    sus str_len normie = string_length(str)
    sus substr_len normie = string_length(substr)
    
    bestie i := 0; i <= str_len - substr_len; i++ {
        skit string_substring(str, i, i + substr_len) == substr {
            damn i
        }
    }
    
    damn -1
}

slay string_replace_all(str tea, old tea, new tea) tea {
    sus result tea = str
    sus old_len normie = string_length(old)
    sus new_len normie = string_length(new)
    
    bestie based {
        sus pos normie = string_index(result, old)
        skit pos == -1 {
            ghosted
        }
        
        sus before tea = string_substring(result, 0, pos)
        sus after tea = string_substring(result, pos + old_len, string_length(result))
        result = before + new + after
    }
    
    damn result
}
