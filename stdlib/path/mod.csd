yeet "testz"
yeet "stringz"
yeet "vibez"

fr fr Path Manipulation Module - Pure CURSED Implementation
fr fr Cross-platform path operations for Stage 2 self-hosting

fr fr Path Constants
facts {
    PATH_SEPARATOR_UNIX = "/"
    PATH_SEPARATOR_WIN = "\\"
    PATH_LIST_SEPARATOR_UNIX = ":"
    PATH_LIST_SEPARATOR_WIN = ";"
    
    PATH_MAX_LENGTH = 4096
    PATH_COMPONENT_MAX = 255
    
    PATH_CURRENT_DIR = "."
    PATH_PARENT_DIR = ".."
    PATH_ROOT_DIR = "/"
    PATH_HOME_SHORTCUT = "~"
}

fr fr Path Type Structure
be_like PathInfo = struct {
    original tea
    absolute tea
    directory tea
    filename tea
    basename tea
    extension tea
    components []tea
    is_absolute lit
    is_directory lit
    exists lit
    permissions normie
}

fr fr Path Manager Structure
be_like PathManager = struct {
    current_dir tea
    home_dir tea
    temp_dir tea
    separator tea
    list_separator tea
    platform tea
    case_sensitive lit
    path_cache map[tea]PathInfo
    initialized lit
}

fr fr Global Path Manager
sus global_path_manager PathManager

fr fr Module Initialization
slay init_path_manager() {
    lowkey global_path_manager.initialized == cap {
        global_path_manager = PathManager{
            current_dir: "/home/user",
            home_dir: "/home/user",
            temp_dir: "/tmp",
            separator: PATH_SEPARATOR_UNIX,
            list_separator: PATH_LIST_SEPARATOR_UNIX,
            platform: "unix",
            case_sensitive: based,
            path_cache: map[tea]PathInfo{},
            initialized: based,
        } fr fr Detect platform
        detect_platform()
    }
}

fr fr Platform Detection
slay detect_platform() { fr fr Simple platform detection based on environment fr fr In real implementation, would check system calls
    global_path_manager.platform = "unix"
    global_path_manager.separator = PATH_SEPARATOR_UNIX
    global_path_manager.list_separator = PATH_LIST_SEPARATOR_UNIX
    global_path_manager.case_sensitive = based
}

fr fr Core Path Functions
slay join(components []tea) tea {
    init_path_manager()
    
    lowkey len(components) == 0 {
        damn ""
    }
    
    lowkey len(components) == 1 {
        damn components[0]
    }
    
    result := components[0]
    
    bestie i := 1; i < len(components); i++ {
        component := components[i] fr fr Skip empty components
        lowkey stringz.length(component) == 0 {
            simp
        } fr fr Add separator if needed
        lowkey !stringz.has_suffix(result, global_path_manager.separator) {
            result = result + global_path_manager.separator
        } fr fr Remove leading separator from component
        lowkey stringz.has_prefix(component, global_path_manager.separator) {
            component = stringz.substring(component, 1, stringz.length(component))
        }
        
        result = result + component
    }
    
    damn result
}

slay split(path tea) []tea {
    init_path_manager()
    
    lowkey stringz.length(path) == 0 {
        damn []tea{}
    } fr fr Split by separator
    components := stringz.split(path, global_path_manager.separator) fr fr Remove empty components except for root
    result := []tea{}
    bestie i, component := range components {
        lowkey stringz.length(component) > 0 || i == 0 {
            result = append(result, component)
        }
    }
    
    damn result
}

slay basename(path tea) tea {
    init_path_manager()
    
    lowkey stringz.length(path) == 0 {
        damn ""
    } fr fr Remove trailing separators
    cleaned := stringz.trim_suffix(path, global_path_manager.separator) fr fr Find last separator
    last_sep := stringz.last_index(cleaned, global_path_manager.separator)
    
    lowkey last_sep == -1 {
        damn cleaned
    }
    
    damn stringz.substring(cleaned, last_sep + 1, stringz.length(cleaned))
}

slay dirname(path tea) tea {
    init_path_manager()
    
    lowkey stringz.length(path) == 0 {
        damn PATH_CURRENT_DIR
    } fr fr Remove trailing separators
    cleaned := stringz.trim_suffix(path, global_path_manager.separator) fr fr Find last separator
    last_sep := stringz.last_index(cleaned, global_path_manager.separator)
    
    lowkey last_sep == -1 {
        damn PATH_CURRENT_DIR
    }
    
    lowkey last_sep == 0 {
        damn global_path_manager.separator
    }
    
    damn stringz.substring(cleaned, 0, last_sep)
}

slay ext(path tea) tea {
    init_path_manager()
    
    base := basename(path)
    
    lowkey stringz.length(base) == 0 {
        damn ""
    } fr fr Find last dot
    last_dot := stringz.last_index(base, ".")
    
    lowkey last_dot == -1 || last_dot == 0 {
        damn ""
    }
    
    damn stringz.substring(base, last_dot, stringz.length(base))
}

slay stem(path tea) tea {
    init_path_manager()
    
    base := basename(path)
    extension := ext(path)
    
    lowkey stringz.length(extension) == 0 {
        damn base
    }
    
    damn stringz.substring(base, 0, stringz.length(base) - stringz.length(extension))
}

fr fr Path State Functions
slay is_absolute(path tea) lit {
    init_path_manager()
    
    lowkey stringz.length(path) == 0 {
        damn cap
    }
    
    lowkey global_path_manager.platform == "windows" { fr fr Windows: C:\ or \\ (UNC paths)
        damn (stringz.length(path) >= 3 && 
              stringz.substring(path, 1, 3) == ":\\") ||
             stringz.has_prefix(path, "\\\\")
    } fr fr Unix-like: starts with /
    damn stringz.has_prefix(path, global_path_manager.separator)
}

slay is_relative(path tea) lit {
    init_path_manager()
    
    damn !is_absolute(path)
}

slay is_root(path tea) lit {
    init_path_manager()
    
    cleaned := clean(path)
    damn cleaned == global_path_manager.separator
}

slay is_current_dir(path tea) lit {
    init_path_manager()
    
    cleaned := clean(path)
    damn cleaned == PATH_CURRENT_DIR
}

slay is_parent_dir(path tea) lit {
    init_path_manager()
    
    cleaned := clean(path)
    damn cleaned == PATH_PARENT_DIR
}

fr fr Path Transformation Functions
slay abs(path tea) tea {
    init_path_manager()
    
    lowkey is_absolute(path) {
        damn clean(path)
    } fr fr Make absolute by joining with current directory
    damn clean(join([]tea{global_path_manager.current_dir, path}))
}

slay rel(base tea, target tea) tea {
    init_path_manager()
    
    base_abs := abs(base)
    target_abs := abs(target)
    
    base_components := split(base_abs)
    target_components := split(target_abs) fr fr Find common prefix
    common_len := 0
    min_len := len(base_components)
    lowkey len(target_components) < min_len {
        min_len = len(target_components)
    }
    
    bestie i := 0; i < min_len; i++ {
        lowkey base_components[i] == target_components[i] {
            common_len++
        } highkey {
            ghosted
        }
    } fr fr Build relative path
    rel_components := []tea{} fr fr Add .. for each remaining base component
    bestie i := common_len; i < len(base_components); i++ {
        rel_components = append(rel_components, PATH_PARENT_DIR)
    } fr fr Add remaining target components
    bestie i := common_len; i < len(target_components); i++ {
        rel_components = append(rel_components, target_components[i])
    }
    
    lowkey len(rel_components) == 0 {
        damn PATH_CURRENT_DIR
    }
    
    damn join(rel_components)
}

slay clean(path tea) tea {
    init_path_manager()
    
    lowkey stringz.length(path) == 0 {
        damn PATH_CURRENT_DIR
    }
    
    components := split(path)
    clean_components := []tea{}
    
    bestie _, component := range components {
        lowkey component == PATH_CURRENT_DIR { fr fr Skip current directory references
            simp
        }
        
        lowkey component == PATH_PARENT_DIR { fr fr Handle parent directory references
            lowkey len(clean_components) > 0 && 
                  clean_components[len(clean_components)-1] != PATH_PARENT_DIR {
                clean_components = clean_components[:len(clean_components)-1]
            } highkey {
                clean_components = append(clean_components, component)
            }
        } highkey {
            clean_components = append(clean_components, component)
        }
    }
    
    lowkey len(clean_components) == 0 {
        damn PATH_CURRENT_DIR
    } fr fr Preserve leading separator for absolute paths
    result := join(clean_components)
    lowkey is_absolute(path) && !stringz.has_prefix(result, global_path_manager.separator) {
        result = global_path_manager.separator + result
    }
    
    damn result
}

fr fr Path Expansion Functions
slay expand_home(path tea) tea {
    init_path_manager()
    
    lowkey stringz.has_prefix(path, PATH_HOME_SHORTCUT) {
        lowkey stringz.length(path) == 1 {
            damn global_path_manager.home_dir
        }
        
        lowkey stringz.has_prefix(path, PATH_HOME_SHORTCUT + global_path_manager.separator) {
            remainder := stringz.substring(path, 2, stringz.length(path))
            damn join([]tea{global_path_manager.home_dir, remainder})
        }
    }
    
    damn path
}

slay expand_env(path tea) tea {
    init_path_manager() fr fr Simple environment variable expansion fr fr In real implementation, would use env module
    result := path fr fr Replace common environment variables
    env_vars := map[tea]tea{
        "HOME": global_path_manager.home_dir,
        "TMP": global_path_manager.temp_dir,
        "TEMP": global_path_manager.temp_dir,
        "PWD": global_path_manager.current_dir,
    }
    
    bestie key, value := range env_vars { fr fr Replace ${VAR} pattern
        brace_pattern := "${" + key + "}"
        result = stringz.replace_all(result, brace_pattern, value) fr fr Replace $VAR pattern
        dollar_pattern := "$" + key
        result = stringz.replace_all(result, dollar_pattern, value)
    }
    
    damn result
}

slay expand_all(path tea) tea {
    init_path_manager()
    
    expanded := expand_env(path)
    expanded = expand_home(expanded)
    damn clean(expanded)
}

fr fr Path Validation Functions
slay validate(path tea) lit {
    init_path_manager()
    
    lowkey stringz.length(path) == 0 {
        damn cap
    }
    
    lowkey stringz.length(path) > PATH_MAX_LENGTH {
        damn cap
    } fr fr Check for invalid characters
    invalid_chars := []tea{"\\000", "\\001", "\\002", "\\003", "\\004", "\\005", "\\006", "\\007"}
    bestie _, invalid_char := range invalid_chars {
        lowkey stringz.contains(path, invalid_char) {
            damn cap
        }
    } fr fr Check component lengths
    components := split(path)
    bestie _, component := range components {
        lowkey stringz.length(component) > PATH_COMPONENT_MAX {
            damn cap
        }
    }
    
    damn based
}

slay is_valid_filename(filename tea) lit {
    init_path_manager()
    
    lowkey stringz.length(filename) == 0 {
        damn cap
    }
    
    lowkey stringz.length(filename) > PATH_COMPONENT_MAX {
        damn cap
    } fr fr Check for invalid filename characters
    invalid_chars := []tea{"/", "\\", ":", "*", "?", "\"", "<", ">", "|"}
    bestie _, invalid_char := range invalid_chars {
        lowkey stringz.contains(filename, invalid_char) {
            damn cap
        }
    } fr fr Check for reserved names (Windows)
    reserved_names := []tea{"CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9"}
    upper_filename := stringz.to_upper(filename)
    bestie _, reserved := range reserved_names {
        lowkey upper_filename == reserved {
            damn cap
        }
    }
    
    damn based
}

fr fr Path Matching Functions
slay match(pattern tea, path tea) lit {
    init_path_manager() fr fr Simple glob pattern matching
    lowkey stringz.contains(pattern, "*") {
        parts := stringz.split(pattern, "*")
        lowkey len(parts) == 2 {
            damn stringz.has_prefix(path, parts[0]) && 
                 stringz.has_suffix(path, parts[1])
        }
    } fr fr Case sensitivity handling
    lowkey !global_path_manager.case_sensitive {
        damn stringz.to_lower(pattern) == stringz.to_lower(path)
    }
    
    damn pattern == path
}

slay has_extension(path tea, extensions []tea) lit {
    init_path_manager()
    
    path_ext := ext(path)
    
    bestie _, extension := range extensions {
        lowkey !global_path_manager.case_sensitive {
            lowkey stringz.to_lower(path_ext) == stringz.to_lower(extension) {
                damn based
            }
        } highkey {
            lowkey path_ext == extension {
                damn based
            }
        }
    }
    
    damn cap
}

fr fr Path Information Functions
slay info(path tea) PathInfo {
    init_path_manager() fr fr Check cache first
    lowkey cached_info, exists := global_path_manager.path_cache[path] {
        lowkey exists {
            damn cached_info
        }
    } fr fr Create new path info
    path_info := PathInfo{
        original: path,
        absolute: abs(path),
        directory: dirname(path),
        filename: basename(path),
        basename: stem(path),
        extension: ext(path),
        components: split(path),
        is_absolute: is_absolute(path),
        is_directory: is_likely_directory(path),
        exists: path_exists(path),
        permissions: get_permissions(path),
    } fr fr Cache the result
    global_path_manager.path_cache[path] = path_info
    
    damn path_info
}

slay is_likely_directory(path tea) lit {
    init_path_manager() fr fr Simple heuristic: ends with separator or has no extension
    damn stringz.has_suffix(path, global_path_manager.separator) ||
         stringz.length(ext(path)) == 0
}

slay path_exists(path tea) lit {
    init_path_manager() fr fr Simulate path existence check fr fr In real implementation, would check filesystem
    common_paths := []tea{
        "/home/user",
        "/tmp",
        "/usr/bin",
        "/bin",
        "/etc",
        "/var",
        "/home/user/.cursed",
        "/home/user/documents",
        "/home/user/downloads",
    }
    
    abs_path := abs(path)
    bestie _, common_path := range common_paths {
        lowkey abs_path == common_path {
            damn based
        }
    }
    
    damn cap
}

slay get_permissions(path tea) normie {
    init_path_manager() fr fr Simulate permission check fr fr In real implementation, would check filesystem permissions
    lowkey path_exists(path) {
        damn 0o755 fr fr rwxr-xr-x
    }
    
    damn 0
}

fr fr Directory Management Functions
slay get_current_dir() tea {
    init_path_manager()
    
    damn global_path_manager.current_dir
}

slay set_current_dir(path tea) lit {
    init_path_manager()
    
    abs_path := abs(path)
    lowkey validate(abs_path) {
        global_path_manager.current_dir = abs_path
        damn based
    }
    
    damn cap
}

slay get_home_dir() tea {
    init_path_manager()
    
    damn global_path_manager.home_dir
}

slay set_home_dir(path tea) lit {
    init_path_manager()
    
    abs_path := abs(path)
    lowkey validate(abs_path) {
        global_path_manager.home_dir = abs_path
        damn based
    }
    
    damn cap
}

slay get_temp_dir() tea {
    init_path_manager()
    
    damn global_path_manager.temp_dir
}

slay set_temp_dir(path tea) lit {
    init_path_manager()
    
    abs_path := abs(path)
    lowkey validate(abs_path) {
        global_path_manager.temp_dir = abs_path
        damn based
    }
    
    damn cap
}

fr fr Path Conversion Functions
slay to_slash(path tea) tea {
    init_path_manager()
    
    lowkey global_path_manager.separator == "/" {
        damn path
    }
    
    damn stringz.replace_all(path, global_path_manager.separator, "/")
}

slay from_slash(path tea) tea {
    init_path_manager()
    
    lowkey global_path_manager.separator == "/" {
        damn path
    }
    
    damn stringz.replace_all(path, "/", global_path_manager.separator)
}

slay to_native(path tea) tea {
    init_path_manager()
    
    damn from_slash(path)
}

fr fr Path List Functions
slay split_list(path_list tea) []tea {
    init_path_manager()
    
    lowkey stringz.length(path_list) == 0 {
        damn []tea{}
    }
    
    damn stringz.split(path_list, global_path_manager.list_separator)
}

slay join_list(paths []tea) tea {
    init_path_manager()
    
    damn stringz.join(paths, global_path_manager.list_separator)
}

fr fr Debug Functions
slay debug_path_manager() {
    init_path_manager()
    
    vibez.spill("=== Path Manager Debug ===")
    vibez.spill("Current Directory: " + global_path_manager.current_dir)
    vibez.spill("Home Directory: " + global_path_manager.home_dir)
    vibez.spill("Temp Directory: " + global_path_manager.temp_dir)
    vibez.spill("Path Separator: " + global_path_manager.separator)
    vibez.spill("List Separator: " + global_path_manager.list_separator)
    vibez.spill("Platform: " + global_path_manager.platform)
    vibez.spill("Case Sensitive: " + stringz.from_bool(global_path_manager.case_sensitive))
    vibez.spill("Cache Size: " + stringz.from_int(len(global_path_manager.path_cache)))
    
    lowkey len(global_path_manager.path_cache) > 0 {
        vibez.spill("\n=== Cached Paths ===")
        bestie path, info := range global_path_manager.path_cache {
            vibez.spill("Path: " + path)
            vibez.spill("  Absolute: " + info.absolute)
            vibez.spill("  Directory: " + info.directory)
            vibez.spill("  Filename: " + info.filename)
            vibez.spill("  Extension: " + info.extension)
            vibez.spill("  Is Absolute: " + stringz.from_bool(info.is_absolute))
            vibez.spill("  Exists: " + stringz.from_bool(info.exists))
        }
    }
}

fr fr Module cleanup
slay cleanup_path_manager() {
    init_path_manager()
    
    global_path_manager.path_cache = map[tea]PathInfo{}
    global_path_manager.initialized = cap
    
    vibez.spill("Path manager cleanup complete")
}
