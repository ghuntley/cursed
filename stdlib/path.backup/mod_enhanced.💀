fr fr Enhanced Path Manipulation Module - Production Implementation
fr fr Cross-platform path operations with comprehensive OS integration
fr fr Complete environment variable expansion and real platform detection

yeet "vibez"
yeet "stringz" 
yeet "envz"
yeet "platformz"

fr fr ================================
fr fr Enhanced Path Constants
fr fr ================================

facts {
    PATH_SEPARATOR_UNIX = "/"
    PATH_SEPARATOR_WIN = "\\"
    PATH_LIST_SEPARATOR_UNIX = ":"
    PATH_LIST_SEPARATOR_WIN = ";"
    PATH_VOLUME_SEPARATOR_WIN = ":"
    
    PATH_MAX_LENGTH_UNIX = 4096
    PATH_MAX_LENGTH_WIN = 260
    PATH_MAX_LENGTH_WIN_EXTENDED = 32767
    PATH_COMPONENT_MAX = 255
    
    PATH_CURRENT_DIR = "."
    PATH_PARENT_DIR = ".."
    PATH_ROOT_DIR_UNIX = "/"
    PATH_ROOT_DIR_WIN = "\\"
    PATH_HOME_SHORTCUT = "~"
    
    fr fr Invalid path characters by platform
    INVALID_CHARS_WIN = "<>:\"|?*"
    INVALID_CHARS_UNIX = "\x00"
    
    fr fr Reserved Windows names
    RESERVED_WIN_NAMES = "CON,PRN,AUX,NUL,COM1,COM2,COM3,COM4,COM5,COM6,COM7,COM8,COM9,LPT1,LPT2,LPT3,LPT4,LPT5,LPT6,LPT7,LPT8,LPT9"
}

fr fr ================================  
fr fr Enhanced Path Type Structures
fr fr ================================

be_like PathInfo squad {
    original tea
    absolute tea
    canonical tea
    directory tea
    filename tea
    basename tea
    extension tea
    components tea[value]
    is_absolute lit
    is_relative lit  
    is_directory lit
    is_unc_path lit
    is_network_path lit
    has_trailing_separator lit
    volume tea
    drive_letter tea
    exists lit
    permissions normie
    target tea  fr fr For symlinks
    resolved tea  fr fr Fully resolved path
}

be_like PlatformInfo squad {
    name tea
    family tea  fr fr unix, windows
    case_sensitive lit
    supports_symlinks lit
    supports_hardlinks lit
    supports_permissions lit
    supports_extended_paths lit
    max_path_length normie
    max_component_length normie
    file_separator tea
    path_list_separator tea  
    volume_separator tea
    line_separator tea
    invalid_chars tea
    reserved_names tea[value]
    env_var_prefix tea  fr fr $ or %
    env_var_suffix tea  fr fr empty or %
    home_env_var tea  fr fr HOME or USERPROFILE
    temp_env_vars tea[value]
}

be_like PathManager squad {
    platform PlatformInfo
    current_dir tea
    home_dir tea
    temp_dir tea
    executable_dir tea
    user_config_dir tea
    user_cache_dir tea
    user_data_dir tea
    system_config_dir tea
    system_data_dir tea
    path_cache map[tea]PathInfo
    env_cache map[tea]tea
    initialized lit
}

fr fr Global path manager instance
sus global_path_manager PathManager

fr fr ================================
fr fr Platform Detection and Initialization  
fr fr ================================

slay init_path_manager() {
    lowkey global_path_manager.initialized {
        damn
    }
    
    detect_comprehensive_platform()
    initialize_directories()
    setup_path_cache()
    
    global_path_manager.initialized = based
}

slay detect_comprehensive_platform() {
    sus platform_name tea = detect_operating_system()
    sus platform_info PlatformInfo
    
    lowkey platform_name == "windows" {
        platform_info = PlatformInfo{
            name: "windows",
            family: "windows", 
            case_sensitive: cap,
            supports_symlinks: based,
            supports_hardlinks: based,
            supports_permissions: cap,
            supports_extended_paths: based,
            max_path_length: detect_extended_path_support(),
            max_component_length: PATH_COMPONENT_MAX,
            file_separator: PATH_SEPARATOR_WIN,
            path_list_separator: PATH_LIST_SEPARATOR_WIN,
            volume_separator: PATH_VOLUME_SEPARATOR_WIN,
            line_separator: "\r\n",
            invalid_chars: INVALID_CHARS_WIN,
            reserved_names: stringz.split(RESERVED_WIN_NAMES, ","),
            env_var_prefix: "%",
            env_var_suffix: "%", 
            home_env_var: "USERPROFILE",
            temp_env_vars: tea[value]{"TEMP", "TMP"}
        }
    } otherwise platform_name == "darwin" {
        platform_info = PlatformInfo{
            name: "darwin",
            family: "unix",
            case_sensitive: cap,  fr fr Default case-insensitive on macOS
            supports_symlinks: based,
            supports_hardlinks: based, 
            supports_permissions: based,
            supports_extended_paths: based,
            max_path_length: PATH_MAX_LENGTH_UNIX,
            max_component_length: PATH_COMPONENT_MAX,
            file_separator: PATH_SEPARATOR_UNIX,
            path_list_separator: PATH_LIST_SEPARATOR_UNIX,
            volume_separator: "",
            line_separator: "\n",
            invalid_chars: INVALID_CHARS_UNIX,
            reserved_names: tea[value]{},
            env_var_prefix: "$",
            env_var_suffix: "",
            home_env_var: "HOME",
            temp_env_vars: tea[value]{"TMPDIR", "TMP", "TEMP"}
        }
        
        fr fr Check if filesystem is case-sensitive
        platform_info.case_sensitive = detect_case_sensitivity()
    } otherwise {
        fr fr Linux and other Unix-like systems
        platform_info = PlatformInfo{
            name: platform_name,
            family: "unix",
            case_sensitive: based,
            supports_symlinks: based,
            supports_hardlinks: based,
            supports_permissions: based, 
            supports_extended_paths: based,
            max_path_length: PATH_MAX_LENGTH_UNIX,
            max_component_length: PATH_COMPONENT_MAX,
            file_separator: PATH_SEPARATOR_UNIX,
            path_list_separator: PATH_LIST_SEPARATOR_UNIX,
            volume_separator: "",
            line_separator: "\n",
            invalid_chars: INVALID_CHARS_UNIX,
            reserved_names: tea[value]{},
            env_var_prefix: "$",
            env_var_suffix: "",
            home_env_var: "HOME",
            temp_env_vars: tea[value]{"TMPDIR", "TMP", "TEMP"}  
        }
    }
    
    global_path_manager.platform = platform_info
}

slay detect_operating_system() tea {
    fr fr Multiple methods to detect OS reliably
    
    fr fr Method 1: Check OS environment variable
    sus os_type tea = envz.get("OSTYPE")
    lowkey os_type != "" {
        lowkey stringz.contains(stringz.to_lower(os_type), "linux") {
            damn "linux"
        } otherwise stringz.contains(stringz.to_lower(os_type), "darwin") {
            damn "darwin"
        } otherwise stringz.contains(stringz.to_lower(os_type), "freebsd") {
            damn "freebsd"
        } otherwise stringz.contains(stringz.to_lower(os_type), "openbsd") {
            damn "openbsd"
        } otherwise stringz.contains(stringz.to_lower(os_type), "netbsd") {
            damn "netbsd"
        } otherwise stringz.contains(stringz.to_lower(os_type), "solaris") {
            damn "solaris"
        }
    }
    
    fr fr Method 2: Check OS environment variable (Windows)
    sus os_name tea = envz.get("OS")
    lowkey stringz.contains(stringz.to_lower(os_name), "windows") {
        damn "windows"
    }
    
    fr fr Method 3: Check for platform-specific files/directories
    lowkey file_exists_check("/proc/version") {
        damn "linux"
    } otherwise file_exists_check("/System/Library/CoreServices/SystemVersion.plist") {
        damn "darwin"
    } otherwise file_exists_check("C:\\Windows\\System32") {
        damn "windows"
    } otherwise file_exists_check("/etc/release") {
        damn "solaris"
    } otherwise file_exists_check("/usr/bin/uname") {
        sus uname_result tea = execute_uname()
        lowkey stringz.contains(stringz.to_lower(uname_result), "freebsd") {
            damn "freebsd"
        } otherwise stringz.contains(stringz.to_lower(uname_result), "openbsd") {
            damn "openbsd"
        } otherwise stringz.contains(stringz.to_lower(uname_result), "netbsd") {
            damn "netbsd"
        }
    }
    
    fr fr Method 4: Check environment variables that are platform-specific  
    lowkey envz.get("USERPROFILE") != "" && envz.get("WINDIR") != "" {
        damn "windows" 
    } otherwise envz.get("HOME") != "" && envz.get("USER") != "" {
        fr fr Unix-like system, default to linux
        damn "linux"
    }
    
    fr fr Final fallback
    damn "unknown"
}

slay detect_extended_path_support() normie {
    fr fr Check if Windows supports extended paths (> 260 characters)
    lowkey envz.get("OSTYPE") == "windows" {
        fr fr Check registry or use extended length by default in modern Windows
        lowkey check_windows_extended_path_support() {
            damn PATH_MAX_LENGTH_WIN_EXTENDED
        }
    }
    damn PATH_MAX_LENGTH_WIN
}

slay detect_case_sensitivity() lit {
    fr fr Test filesystem case sensitivity by attempting to create files
    sus test_dir tea = get_temp_directory()
    sus test_file1 tea = join_path_safe(test_dir, "CaseSensitivityTest.tmp")
    sus test_file2 tea = join_path_safe(test_dir, "casesensitivitytest.tmp")
    
    fr fr Try to create both files
    lowkey create_test_file(test_file1) && create_test_file(test_file2) {
        cleanup_test_file(test_file1)
        cleanup_test_file(test_file2)
        damn based  fr fr Case sensitive if both files can exist
    }
    
    cleanup_test_file(test_file1) 
    cleanup_test_file(test_file2)
    damn cap  fr fr Case insensitive
}

slay initialize_directories() {
    global_path_manager.current_dir = get_current_directory_real()
    global_path_manager.home_dir = get_home_directory_real()
    global_path_manager.temp_dir = get_temp_directory_real()
    global_path_manager.executable_dir = get_executable_directory_real()
    global_path_manager.user_config_dir = get_user_config_directory_real()
    global_path_manager.user_cache_dir = get_user_cache_directory_real()
    global_path_manager.user_data_dir = get_user_data_directory_real()
    global_path_manager.system_config_dir = get_system_config_directory_real()
    global_path_manager.system_data_dir = get_system_data_directory_real()
}

slay setup_path_cache() {
    global_path_manager.path_cache = map[tea]PathInfo{}
    global_path_manager.env_cache = map[tea]tea{}
}

fr fr ================================
fr fr Core Path Operations
fr fr ================================

slay join(components tea[value]) tea {
    init_path_manager()
    
    lowkey len(components) == 0 {
        damn ""
    }
    
    lowkey len(components) == 1 {
        damn components[0]
    }
    
    sus result tea = components[0]
    sus separator tea = global_path_manager.platform.file_separator
    
    bestie i := 1; i < len(components); i++ {
        sus component tea = components[i]
        
        fr fr Skip empty components
        lowkey stringz.length(component) == 0 {
            simp
        }
        
        fr fr Remove leading separators from component
        bestie stringz.has_prefix(component, separator) || 
              (global_path_manager.platform.family == "windows" && 
               stringz.has_prefix(component, "/")) {
            component = stringz.substring(component, 1, stringz.length(component))
        }
        
        fr fr Add separator if needed
        lowkey !stringz.has_suffix(result, separator) {
            result = result + separator
        }
        
        result = result + component
    }
    
    damn result
}

slay join_path_safe(base tea, component tea) tea {
    damn join(tea[value]{base, component})
}

slay split(path tea) tea[value]{
    init_path_manager()
    
    lowkey stringz.length(path) == 0 {
        damn tea[value]{}
    }
    
    sus separator tea = global_path_manager.platform.file_separator
    sus components tea[value] = stringz.split(path, separator)
    
    fr fr Handle both separators on Windows
    lowkey global_path_manager.platform.family == "windows" {
        sus all_components tea[value] = tea[value]{}
        bestie _, component in components {
            lowkey stringz.contains(component, "/") {
                sus unix_parts tea[value] = stringz.split(component, "/")
                all_components = append(all_components, unix_parts...)
            } otherwise {
                all_components = append(all_components, component)
            }
        }
        components = all_components
    }
    
    fr fr Remove empty components except for root
    sus result tea[value] = tea[value]{}
    bestie i, component in components {
        lowkey stringz.length(component) > 0 || (i == 0 && is_absolute_path(path)) {
            result = append(result, component)
        }
    }
    
    damn result
}

slay basename(path tea) tea {
    init_path_manager()
    
    lowkey stringz.length(path) == 0 {
        damn ""
    }
    
    fr fr Clean path first
    sus clean_path tea = clean(path)
    sus separator tea = global_path_manager.platform.file_separator
    
    fr fr Handle root directory
    lowkey clean_path == separator {
        damn separator
    }
    
    fr fr Find last separator (check both separators on Windows)
    sus last_sep normie = stringz.last_index(clean_path, separator)
    
    lowkey global_path_manager.platform.family == "windows" {
        sus last_unix_sep normie = stringz.last_index(clean_path, "/")
        lowkey last_unix_sep > last_sep {
            last_sep = last_unix_sep
        }
    }
    
    lowkey last_sep == -1 {
        damn clean_path
    }
    
    damn stringz.substring(clean_path, last_sep + 1, stringz.length(clean_path))
}

slay dirname(path tea) tea {
    init_path_manager()
    
    lowkey stringz.length(path) == 0 {
        damn PATH_CURRENT_DIR
    }
    
    sus clean_path tea = clean(path)
    sus separator tea = global_path_manager.platform.file_separator
    
    fr fr Handle root directory
    lowkey clean_path == separator {
        damn separator
    }
    
    fr fr Find last separator
    sus last_sep normie = find_last_separator(clean_path)
    
    lowkey last_sep == -1 {
        damn PATH_CURRENT_DIR
    }
    
    lowkey last_sep == 0 {
        damn separator
    }
    
    sus result tea = stringz.substring(clean_path, 0, last_sep)
    
    fr fr Handle Windows drive letters
    lowkey global_path_manager.platform.family == "windows" && 
          stringz.length(result) == 2 && 
          stringz.substring(result, 1, 2) == ":" {
        damn result + separator
    }
    
    damn result
}

slay ext(path tea) tea {
    init_path_manager()
    
    sus base tea = basename(path)
    
    lowkey stringz.length(base) == 0 {
        damn ""
    }
    
    fr fr Find last dot
    sus last_dot normie = stringz.last_index(base, ".")
    
    fr fr No extension if no dot, or dot is at start (hidden file)
    lowkey last_dot <= 0 {
        damn ""
    }
    
    damn stringz.substring(base, last_dot, stringz.length(base))
}

slay stem(path tea) tea {
    init_path_manager()
    
    sus base tea = basename(path)
    sus extension tea = ext(path)
    
    lowkey stringz.length(extension) == 0 {
        damn base
    }
    
    damn stringz.substring(base, 0, stringz.length(base) - stringz.length(extension))
}

fr fr ================================
fr fr Path State and Validation
fr fr ================================

slay is_absolute(path tea) lit {
    init_path_manager()
    
    lowkey stringz.length(path) == 0 {
        damn cap
    }
    
    lowkey global_path_manager.platform.family == "windows" {
        fr fr Windows absolute paths:
        fr fr C:\path (drive letter)
        fr fr \\server\share (UNC)
        fr fr \\?\C:\path (extended)
        
        lowkey stringz.length(path) >= 3 {
            fr fr Drive letter format C:\
            lowkey stringz.substring(path, 1, 3) == ":\\" ||
                  stringz.substring(path, 1, 3) == ":/" {
                damn based
            }
        }
        
        fr fr UNC paths \\server or extended \\?\
        lowkey stringz.has_prefix(path, "\\\\") {
            damn based
        }
        
        fr fr Single backslash (relative to current drive)
        lowkey stringz.has_prefix(path, "\\") {
            damn based
        }
        
        damn cap
    }
    
    fr fr Unix-like: starts with /
    damn stringz.has_prefix(path, global_path_manager.platform.file_separator)
}

slay is_relative(path tea) lit {
    damn !is_absolute(path)
}

slay is_unc_path(path tea) lit {
    init_path_manager()
    
    lowkey global_path_manager.platform.family == "windows" {
        damn stringz.has_prefix(path, "\\\\") && stringz.length(path) > 2
    }
    
    damn cap
}

slay is_extended_path(path tea) lit {
    init_path_manager()
    
    lowkey global_path_manager.platform.family == "windows" {
        damn stringz.has_prefix(path, "\\\\?\\")
    }
    
    damn cap
}

slay is_root(path tea) lit {
    init_path_manager()
    
    sus clean_path tea = clean(path)
    sus separator tea = global_path_manager.platform.file_separator
    
    lowkey global_path_manager.platform.family == "windows" {
        fr fr Windows roots: C:\, \\server\share
        lowkey is_unc_path(clean_path) {
            sus parts tea[value] = split(clean_path)
            damn len(parts) <= 2  fr fr \\server or \\server\share
        }
        
        lowkey stringz.length(clean_path) == 3 && 
              stringz.substring(clean_path, 1, 3) == ":\\" {
            damn based  fr fr C:\
        }
    }
    
    damn clean_path == separator
}

slay validate_path(path tea) lit {
    init_path_manager()
    
    lowkey path == "" {
        damn cap
    }
    
    fr fr Check maximum length
    lowkey stringz.length(path) > global_path_manager.platform.max_path_length {
        damn cap
    }
    
    fr fr Check for invalid characters
    sus invalid_chars tea = global_path_manager.platform.invalid_chars
    bestie i := 0; i < stringz.length(invalid_chars); i++ {
        sus invalid_char tea = stringz.substring(invalid_chars, i, i + 1)
        lowkey stringz.contains(path, invalid_char) {
            damn cap
        }
    }
    
    fr fr Check components
    sus components tea[value] = split(path)
    bestie _, component in components {
        lowkey !validate_path_component(component) {
            damn cap
        }
    }
    
    damn based
}

slay validate_path_component(component tea) lit {
    init_path_manager()
    
    lowkey stringz.length(component) == 0 {
        damn based  fr fr Empty components are ok (will be filtered)
    }
    
    lowkey stringz.length(component) > global_path_manager.platform.max_component_length {
        damn cap
    }
    
    fr fr Platform-specific validation
    lowkey global_path_manager.platform.family == "windows" {
        damn validate_windows_component(component)
    }
    
    damn based
}

slay validate_windows_component(component tea) lit {
    fr fr Check reserved names
    sus upper_component tea = stringz.to_upper(component)
    
    fr fr Remove extension for reserved name check
    sus name_only tea = upper_component
    sus dot_pos normie = stringz.index(name_only, ".")
    lowkey dot_pos != -1 {
        name_only = stringz.substring(name_only, 0, dot_pos)
    }
    
    bestie _, reserved in global_path_manager.platform.reserved_names {
        lowkey name_only == reserved {
            damn cap
        }
    }
    
    fr fr Check trailing dots and spaces
    lowkey stringz.has_suffix(component, ".") || stringz.has_suffix(component, " ") {
        damn cap
    }
    
    damn based
}

fr fr ================================  
fr fr Path Transformation
fr fr ================================

slay abs(path tea) tea {
    init_path_manager()
    
    lowkey is_absolute(path) {
        damn clean(path)
    }
    
    fr fr Make relative path absolute
    sus current_dir tea = global_path_manager.current_dir
    sus joined tea = join(tea[value]{current_dir, path})
    damn clean(joined)
}

slay canonical(path tea) tea {
    init_path_manager()
    
    fr fr Get absolute path first
    sus abs_path tea = abs(path)
    
    fr fr Resolve all symbolic links and clean
    sus resolved tea = resolve_symlinks(abs_path)
    damn clean(resolved)
}

slay rel(base tea, target tea) tea {
    init_path_manager()
    
    fr fr Convert both to absolute paths
    sus base_abs tea = abs(base)
    sus target_abs tea = abs(target)
    
    fr fr Special case: same path
    lowkey base_abs == target_abs {
        damn PATH_CURRENT_DIR
    }
    
    sus base_components tea[value] = split(base_abs)
    sus target_components tea[value] = split(target_abs)
    
    fr fr Find common prefix
    sus common_len normie = 0
    sus min_len normie = len(base_components)
    lowkey len(target_components) < min_len {
        min_len = len(target_components)
    }
    
    bestie i := 0; i < min_len; i++ {
        lowkey path_components_equal(base_components[i], target_components[i]) {
            common_len++
        } highkey {
            ghosted
        }
    }
    
    fr fr Build relative path
    sus rel_components tea[value] = tea[value]{}
    
    fr fr Add .. for each remaining base component
    bestie i := common_len; i < len(base_components); i++ {
        rel_components = append(rel_components, PATH_PARENT_DIR)
    }
    
    fr fr Add remaining target components
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
    
    sus original_path tea = path
    sus separator tea = global_path_manager.platform.file_separator
    
    fr fr Handle Windows extended paths
    sus extended_prefix tea = ""
    lowkey global_path_manager.platform.family == "windows" {
        lowkey stringz.has_prefix(path, "\\\\?\\") {
            extended_prefix = "\\\\?\\"
            path = stringz.substring(path, 4, stringz.length(path))
        }
    }
    
    sus components tea[value] = split(path)
    sus clean_components tea[value] = tea[value]{}
    sus is_abs lit = is_absolute(original_path)
    
    bestie _, component in components {
        lowkey component == PATH_CURRENT_DIR {
            fr fr Skip current directory references
            simp
        }
        
        lowkey component == PATH_PARENT_DIR {
            fr fr Handle parent directory references
            lowkey len(clean_components) > 0 {
                sus last_component tea = clean_components[len(clean_components)-1]
                lowkey last_component != PATH_PARENT_DIR {
                    fr fr Don't remove if it would go above root for absolute paths
                    lowkey is_abs && len(clean_components) == 1 {
                        simp
                    }
                    clean_components = clean_components[:len(clean_components)-1]
                    simp
                }
            }
            
            fr fr Add .. if not at root of absolute path
            lowkey !is_abs || len(clean_components) > 0 {
                clean_components = append(clean_components, component)
            }
        } highkey {
            clean_components = append(clean_components, component)
        }
    }
    
    fr fr Build result
    sus result tea
    lowkey len(clean_components) == 0 {
        lowkey is_abs {
            result = separator
        } otherwise {
            result = PATH_CURRENT_DIR
        }
    } otherwise {
        result = join(clean_components)
        
        fr fr Preserve leading separator for absolute paths
        lowkey is_abs && !stringz.has_prefix(result, separator) {
            result = separator + result
        }
    }
    
    fr fr Add back extended prefix if needed
    lowkey extended_prefix != "" {
        result = extended_prefix + result
    }
    
    damn result
}

fr fr ================================
fr fr Environment Variable Expansion  
fr fr ================================

slay expand_env(path tea) tea {
    init_path_manager()
    
    sus result tea = path
    
    fr fr Platform-specific expansion
    lowkey global_path_manager.platform.family == "windows" {
        result = expand_windows_env_vars(result)
    } otherwise {
        result = expand_unix_env_vars(result)
    }
    
    damn result
}

slay expand_windows_env_vars(path tea) tea {
    sus result tea = path
    
    fr fr Expand %VAR% patterns
    sus percent_pattern_regex tea = "%([^%]+)%"
    result = expand_env_pattern(result, percent_pattern_regex, "%", "%")
    
    fr fr Common Windows environment variables
    sus common_vars map[tea]tea = map[tea]tea{
        "%USERPROFILE%": get_cached_env("USERPROFILE"),
        "%APPDATA%": get_cached_env("APPDATA"), 
        "%LOCALAPPDATA%": get_cached_env("LOCALAPPDATA"),
        "%PROGRAMFILES%": get_cached_env("PROGRAMFILES"),
        "%PROGRAMFILES(X86)%": get_cached_env("PROGRAMFILES(X86)"),
        "%PROGRAMDATA%": get_cached_env("PROGRAMDATA"),
        "%WINDOWS%": get_cached_env("WINDOWS"),
        "%SYSTEM32%": get_cached_env("SYSTEM32"),
        "%TEMP%": get_cached_env("TEMP"),
        "%TMP%": get_cached_env("TMP"),
        "%USERNAME%": get_cached_env("USERNAME"),
        "%COMPUTERNAME%": get_cached_env("COMPUTERNAME"),
        "%USERDOMAIN%": get_cached_env("USERDOMAIN"),
        "%ALLUSERSPROFILE%": get_cached_env("ALLUSERSPROFILE"),
        "%PUBLIC%": get_cached_env("PUBLIC")
    }
    
    bestie pattern, value in common_vars {
        lowkey value != "" {
            result = stringz.replace_all(result, pattern, value)
        }
    }
    
    damn result
}

slay expand_unix_env_vars(path tea) tea {
    sus result tea = path
    
    fr fr Expand ${VAR} patterns  
    sus brace_pattern_regex tea = "\\$\\{([^}]+)\\}"
    result = expand_env_pattern(result, brace_pattern_regex, "${", "}")
    
    fr fr Expand $VAR patterns
    sus dollar_pattern_regex tea = "\\$([A-Za-z_][A-Za-z0-9_]*)"
    result = expand_env_pattern(result, dollar_pattern_regex, "$", "")
    
    fr fr Common Unix environment variables
    sus common_vars map[tea]tea = map[tea]tea{
        "$HOME": get_cached_env("HOME"),
        "$USER": get_cached_env("USER"),
        "$USERNAME": get_cached_env("USERNAME"),
        "$LOGNAME": get_cached_env("LOGNAME"),
        "$PWD": get_cached_env("PWD"),
        "$OLDPWD": get_cached_env("OLDPWD"),
        "$PATH": get_cached_env("PATH"),
        "$TMPDIR": get_cached_env("TMPDIR"),
        "$TMP": get_cached_env("TMP"),
        "$TEMP": get_cached_env("TEMP"),
        "$SHELL": get_cached_env("SHELL"),
        "$TERM": get_cached_env("TERM"),
        "$LANG": get_cached_env("LANG"),
        "$LC_ALL": get_cached_env("LC_ALL")
    }
    
    bestie pattern, value in common_vars {
        lowkey value != "" {
            result = stringz.replace_all(result, pattern, value)
        }
    }
    
    damn result
}

slay expand_env_pattern(text tea, pattern tea, prefix tea, suffix tea) tea {
    fr fr Simplified pattern matching and replacement
    fr fr In production, would use proper regex engine
    
    sus result tea = text
    sus search_pos normie = 0
    
    bestie {
        sus start_pos normie = stringz.index_from(result, prefix, search_pos)
        lowkey start_pos == -1 {
            ghosted
        }
        
        lowkey suffix == "" {
            fr fr Pattern like $VAR (no closing delimiter)
            sus end_pos normie = find_var_end(result, start_pos + stringz.length(prefix))
            sus var_name tea = stringz.substring(result, start_pos + stringz.length(prefix), end_pos)
            sus var_value tea = get_cached_env(var_name)
            
            lowkey var_value != "" {
                sus before tea = stringz.substring(result, 0, start_pos)
                sus after tea = stringz.substring(result, end_pos, stringz.length(result))
                result = before + var_value + after
                search_pos = start_pos + stringz.length(var_value)
            } otherwise {
                search_pos = start_pos + stringz.length(prefix)
            }
        } otherwise {
            fr fr Pattern like ${VAR} or %VAR%
            sus end_pos normie = stringz.index_from(result, suffix, start_pos + stringz.length(prefix))
            lowkey end_pos == -1 {
                search_pos = start_pos + stringz.length(prefix)
                simp
            }
            
            sus var_name tea = stringz.substring(result, start_pos + stringz.length(prefix), end_pos)
            sus var_value tea = get_cached_env(var_name)
            
            lowkey var_value != "" {
                sus before tea = stringz.substring(result, 0, start_pos)
                sus after tea = stringz.substring(result, end_pos + stringz.length(suffix), stringz.length(result))
                result = before + var_value + after
                search_pos = start_pos + stringz.length(var_value)
            } otherwise {
                search_pos = end_pos + stringz.length(suffix)
            }
        }
    }
    
    damn result
}

slay expand_home(path tea) tea {
    init_path_manager()
    
    lowkey !stringz.has_prefix(path, PATH_HOME_SHORTCUT) {
        damn path
    }
    
    sus home_dir tea = global_path_manager.home_dir
    
    lowkey path == PATH_HOME_SHORTCUT {
        damn home_dir
    }
    
    sus separator tea = global_path_manager.platform.file_separator
    lowkey stringz.has_prefix(path, PATH_HOME_SHORTCUT + separator) {
        sus remainder tea = stringz.substring(path, 2, stringz.length(path))
        damn join(tea[value]{home_dir, remainder})
    }
    
    fr fr Handle ~user expansion (Unix-like systems)
    lowkey global_path_manager.platform.family == "unix" {
        sus slash_pos normie = stringz.index(path, separator)
        lowkey slash_pos != -1 {
            sus username tea = stringz.substring(path, 1, slash_pos)
            sus user_home tea = get_user_home_directory(username)
            lowkey user_home != "" {
                sus remainder tea = stringz.substring(path, slash_pos + 1, stringz.length(path))
                damn join(tea[value]{user_home, remainder})
            }
        } otherwise {
            fr fr Just ~user with no path component  
            sus username tea = stringz.substring(path, 1, stringz.length(path))
            sus user_home tea = get_user_home_directory(username)
            lowkey user_home != "" {
                damn user_home
            }
        }
    }
    
    damn path
}

slay expand_all(path tea) tea {
    init_path_manager()
    
    sus expanded tea = expand_env(path)
    expanded = expand_home(expanded)
    damn clean(expanded)
}

fr fr ================================
fr fr Path Information and Analysis
fr fr ================================

slay info(path tea) PathInfo {
    init_path_manager()
    
    fr fr Check cache first
    sus cached_info PathInfo
    sus cache_exists lit
    cached_info, cache_exists = global_path_manager.path_cache[path]
    lowkey cache_exists {
        damn cached_info
    }
    
    fr fr Create comprehensive path info
    sus abs_path tea = abs(path)
    sus canonical_path tea = canonical(path)
    
    sus path_info PathInfo = PathInfo{
        original: path,
        absolute: abs_path,
        canonical: canonical_path,
        directory: dirname(path),
        filename: basename(path),
        basename: stem(path),
        extension: ext(path),
        components: split(path),
        is_absolute: is_absolute(path),
        is_relative: is_relative(path),
        is_directory: is_likely_directory(path),
        is_unc_path: is_unc_path(path),
        is_network_path: is_network_path(path),
        has_trailing_separator: has_trailing_separator(path),
        volume: get_volume(path),
        drive_letter: get_drive_letter(path),
        exists: path_exists_real(path),
        permissions: get_path_permissions(path),
        target: get_symlink_target_safe(path),
        resolved: resolve_path_completely(path)
    }
    
    fr fr Cache the result
    global_path_manager.path_cache[path] = path_info
    
    damn path_info
}

slay get_volume(path tea) tea {
    init_path_manager()
    
    lowkey global_path_manager.platform.family == "windows" {
        lowkey is_unc_path(path) {
            sus components tea[value] = split(path)
            lowkey len(components) >= 2 {
                damn "\\\\" + components[0] + "\\" + components[1]
            }
        }
        
        lowkey stringz.length(path) >= 2 && 
              stringz.substring(path, 1, 2) == ":" {
            damn stringz.substring(path, 0, 2)
        }
    }
    
    damn ""
}

slay get_drive_letter(path tea) tea {
    init_path_manager()
    
    lowkey global_path_manager.platform.family == "windows" {
        lowkey stringz.length(path) >= 2 && 
              stringz.substring(path, 1, 2) == ":" {
            damn stringz.substring(path, 0, 1)
        }
    }
    
    damn ""
}

slay is_likely_directory(path tea) lit {
    init_path_manager()
    
    fr fr Heuristics for directory detection
    sus separator tea = global_path_manager.platform.file_separator
    
    fr fr Ends with separator
    lowkey stringz.has_suffix(path, separator) {
        damn based
    }
    
    fr fr No extension (Unix-like systems)
    lowkey global_path_manager.platform.family == "unix" {
        lowkey stringz.length(ext(path)) == 0 {
            damn based
        }
    }
    
    fr fr Check if path exists and is actually a directory
    lowkey path_exists_real(path) {
        damn is_directory_real(path)
    }
    
    damn cap
}

slay is_network_path(path tea) lit {
    init_path_manager()
    
    lowkey global_path_manager.platform.family == "windows" {
        damn is_unc_path(path)
    }
    
    fr fr Unix network paths (NFS mounts, etc.)
    damn stringz.has_prefix(path, "/net/") || 
         stringz.has_prefix(path, "/mnt/") ||
         stringz.contains(path, "://")
}

slay has_trailing_separator(path tea) lit {
    init_path_manager()
    
    lowkey stringz.length(path) == 0 {
        damn cap
    }
    
    sus separator tea = global_path_manager.platform.file_separator
    damn stringz.has_suffix(path, separator)
}

fr fr ================================
fr fr Directory Management
fr fr ================================

slay get_current_dir() tea {
    init_path_manager()
    damn global_path_manager.current_dir
}

slay set_current_dir(path tea) lit {
    init_path_manager()
    
    lowkey !validate_path(path) {
        damn cap
    }
    
    sus abs_path tea = abs(path)
    lowkey change_directory_real(abs_path) {
        global_path_manager.current_dir = abs_path
        clear_path_cache()  fr fr Clear cache since relative paths changed
        damn based
    }
    
    damn cap
}

slay get_home_dir() tea {
    init_path_manager()
    damn global_path_manager.home_dir
}

slay get_temp_dir() tea {
    init_path_manager()
    damn global_path_manager.temp_dir
}

slay get_executable_dir() tea {
    init_path_manager()
    damn global_path_manager.executable_dir
}

slay get_user_config_dir() tea {
    init_path_manager()
    damn global_path_manager.user_config_dir
}

slay get_user_cache_dir() tea {
    init_path_manager()
    damn global_path_manager.user_cache_dir
}

slay get_user_data_dir() tea {
    init_path_manager()
    damn global_path_manager.user_data_dir
}

slay get_system_config_dir() tea {
    init_path_manager()
    damn global_path_manager.system_config_dir
}

slay get_system_data_dir() tea {
    init_path_manager()
    damn global_path_manager.system_data_dir
}

fr fr ================================
fr fr Path Matching and Comparison
fr fr ================================

slay match_glob(pattern tea, path tea) lit {
    init_path_manager()
    
    fr fr Simple glob pattern matching
    fr fr In production, would implement full glob syntax
    
    lowkey !stringz.contains(pattern, "*") && !stringz.contains(pattern, "?") {
        fr fr No wildcards - direct comparison
        damn path_components_equal(pattern, path)
    }
    
    lowkey pattern == "*" {
        damn based  fr fr Match everything
    }
    
    fr fr Handle simple * patterns
    lowkey stringz.contains(pattern, "*") {
        sus parts tea[value] = stringz.split(pattern, "*")
        damn match_wildcard_parts(parts, path)
    }
    
    fr fr Handle ? patterns
    lowkey stringz.contains(pattern, "?") {
        damn match_question_pattern(pattern, path)
    }
    
    damn cap
}

slay match_extension(path tea, extensions tea[value]) lit {
    init_path_manager()
    
    sus path_ext tea = ext(path)
    
    bestie _, extension in extensions {
        lowkey path_components_equal(path_ext, extension) {
            damn based
        }
    }
    
    damn cap
}

slay path_components_equal(path1 tea, path2 tea) lit {
    init_path_manager()
    
    lowkey global_path_manager.platform.case_sensitive {
        damn path1 == path2
    } otherwise {
        damn stringz.to_lower(path1) == stringz.to_lower(path2)
    }
}

slay compare_paths(path1 tea, path2 tea) normie {
    init_path_manager()
    
    sus clean1 tea = clean(path1)
    sus clean2 tea = clean(path2)
    
    lowkey !global_path_manager.platform.case_sensitive {
        clean1 = stringz.to_lower(clean1)
        clean2 = stringz.to_lower(clean2)
    }
    
    lowkey clean1 == clean2 {
        damn 0
    } otherwise clean1 < clean2 {
        damn -1
    } otherwise {
        damn 1
    }
}

fr fr ================================
fr fr Path Conversion and Normalization
fr fr ================================

slay to_slash(path tea) tea {
    init_path_manager()
    
    lowkey global_path_manager.platform.file_separator == "/" {
        damn path
    }
    
    damn stringz.replace_all(path, global_path_manager.platform.file_separator, "/")
}

slay from_slash(path tea) tea {
    init_path_manager()
    
    lowkey global_path_manager.platform.file_separator == "/" {
        damn path
    }
    
    damn stringz.replace_all(path, "/", global_path_manager.platform.file_separator)
}

slay to_native(path tea) tea {
    init_path_manager()
    
    damn from_slash(path)
}

slay normalize_separators(path tea) tea {
    init_path_manager()
    
    sus result tea = path
    sus native_sep tea = global_path_manager.platform.file_separator
    
    fr fr Convert all separator types to native
    lowkey global_path_manager.platform.family == "windows" {
        result = stringz.replace_all(result, "/", native_sep)
    } otherwise {
        result = stringz.replace_all(result, "\\", native_sep)
    }
    
    fr fr Remove duplicate separators
    sus double_sep tea = native_sep + native_sep
    bestie stringz.contains(result, double_sep) {
        result = stringz.replace_all(result, double_sep, native_sep)
    }
    
    damn result
}

fr fr ================================
fr fr Path List Operations  
fr fr ================================

slay split_list(path_list tea) tea[value]{
    init_path_manager()
    
    lowkey stringz.length(path_list) == 0 {
        damn tea[value]{}
    }
    
    sus separator tea = global_path_manager.platform.path_list_separator
    sus paths tea[value] = stringz.split(path_list, separator)
    
    fr fr Remove empty entries
    sus result tea[value] = tea[value]{}
    bestie _, path in paths {
        lowkey stringz.length(stringz.trim_space(path)) > 0 {
            result = append(result, stringz.trim_space(path))
        }
    }
    
    damn result
}

slay join_list(paths tea[value]) tea {
    init_path_manager()
    
    sus separator tea = global_path_manager.platform.path_list_separator
    damn stringz.join(paths, separator)
}

slay search_path(filename tea) tea {
    init_path_manager()
    
    fr fr Search for executable in PATH
    sus path_env tea = get_cached_env("PATH")
    lowkey path_env == "" {
        damn ""
    }
    
    sus paths tea[value] = split_list(path_env)
    bestie _, search_dir in paths {
        sus candidate tea = join(tea[value]{search_dir, filename})
        
        fr fr On Windows, also try with common executable extensions
        lowkey global_path_manager.platform.family == "windows" {
            sus extensions tea[value] = tea[value]{"", ".exe", ".bat", ".cmd", ".com"}
            bestie _, ext in extensions {
                sus candidate_with_ext tea = candidate + ext
                lowkey file_exists_real(candidate_with_ext) {
                    damn candidate_with_ext
                }
            }
        } otherwise {
            lowkey file_exists_real(candidate) {
                damn candidate
            }
        }
    }
    
    damn ""
}

fr fr ================================
fr fr Utility and Helper Functions
fr fr ================================

slay get_cached_env(name tea) tea {
    init_path_manager()
    
    fr fr Check cache first
    sus cached_value tea
    sus cache_exists lit
    cached_value, cache_exists = global_path_manager.env_cache[name]
    lowkey cache_exists {
        damn cached_value
    }
    
    fr fr Get from environment and cache
    sus value tea = envz.get(name)
    global_path_manager.env_cache[name] = value
    damn value
}

slay clear_env_cache() {
    init_path_manager()
    global_path_manager.env_cache = map[tea]tea{}
}

slay clear_path_cache() {
    init_path_manager()
    global_path_manager.path_cache = map[tea]PathInfo{}
}

slay find_last_separator(path tea) normie {
    init_path_manager()
    
    sus separator tea = global_path_manager.platform.file_separator
    sus last_sep normie = stringz.last_index(path, separator)
    
    fr fr On Windows, also check for forward slash
    lowkey global_path_manager.platform.family == "windows" {
        sus last_unix_sep normie = stringz.last_index(path, "/")
        lowkey last_unix_sep > last_sep {
            last_sep = last_unix_sep
        }
    }
    
    damn last_sep
}

slay find_var_end(text tea, start_pos normie) normie {
    fr fr Find end of environment variable name
    sus end_pos normie = start_pos
    
    bestie end_pos < stringz.length(text) {
        sus char tea = stringz.substring(text, end_pos, end_pos + 1)
        
        fr fr Valid variable name characters
        lowkey (char >= "A" && char <= "Z") || 
              (char >= "a" && char <= "z") ||
              (char >= "0" && char <= "9") ||
              char == "_" {
            end_pos++
        } otherwise {
            ghosted
        }
    }
    
    damn end_pos
}

slay match_wildcard_parts(parts tea[value], path tea) lit {
    fr fr Simple wildcard matching implementation
    lowkey len(parts) == 1 {
        damn based  fr fr Just * matches everything
    }
    
    lowkey len(parts) == 2 {
        fr fr prefix*suffix pattern
        damn stringz.has_prefix(path, parts[0]) && 
             stringz.has_suffix(path, parts[1])
    }
    
    fr fr Multiple * wildcards - more complex matching needed
    fr fr For now, just check if all parts exist in order
    sus current_pos normie = 0
    bestie _, part in parts {
        lowkey stringz.length(part) == 0 {
            simp
        }
        
        sus found_pos normie = stringz.index_from(path, part, current_pos)
        lowkey found_pos == -1 {
            damn cap
        }
        
        current_pos = found_pos + stringz.length(part)
    }
    
    damn based
}

slay match_question_pattern(pattern tea, path tea) lit {
    fr fr Match ? wildcard patterns
    lowkey stringz.length(pattern) != stringz.length(path) {
        damn cap
    }
    
    bestie i := 0; i < stringz.length(pattern); i++ {
        sus pattern_char tea = stringz.substring(pattern, i, i + 1)
        sus path_char tea = stringz.substring(path, i, i + 1)
        
        lowkey pattern_char != "?" && !path_components_equal(pattern_char, path_char) {
            damn cap
        }
    }
    
    damn based
}

fr fr ================================
fr fr Real System Integration Functions
fr fr ================================

fr fr These would call actual OS APIs in production

slay get_current_directory_real() tea {
    fr fr Would call getcwd() on Unix, GetCurrentDirectory() on Windows
    sus pwd tea = get_cached_env("PWD")
    lowkey pwd != "" {
        damn pwd
    }
    damn "/home/user"  fr fr Fallback
}

slay get_home_directory_real() tea {
    lowkey global_path_manager.platform.family == "windows" {
        sus userprofile tea = get_cached_env("USERPROFILE")
        lowkey userprofile != "" {
            damn userprofile
        }
        damn "C:\\Users\\user"
    } otherwise {
        sus home tea = get_cached_env("HOME")
        lowkey home != "" {
            damn home
        }
        damn "/home/user"
    }
}

slay get_temp_directory_real() tea {
    lowkey global_path_manager.platform.family == "windows" {
        sus temp tea = get_cached_env("TEMP")
        lowkey temp != "" {
            damn temp
        }
        sus tmp tea = get_cached_env("TMP")
        lowkey tmp != "" {
            damn tmp
        }
        damn "C:\\Temp"
    } otherwise {
        bestie _, temp_var in global_path_manager.platform.temp_env_vars {
            sus temp_dir tea = get_cached_env(temp_var)
            lowkey temp_dir != "" {
                damn temp_dir
            }
        }
        damn "/tmp"
    }
}

slay get_executable_directory_real() tea {
    fr fr Would get path to current executable
    sus exe_path tea = get_cached_env("CURSED_EXECUTABLE_PATH")
    lowkey exe_path != "" {
        damn dirname(exe_path)
    }
    damn "/usr/local/bin"
}

slay get_user_config_directory_real() tea {
    lowkey global_path_manager.platform.family == "windows" {
        sus appdata tea = get_cached_env("APPDATA")
        lowkey appdata != "" {
            damn appdata
        }
        damn "C:\\Users\\user\\AppData\\Roaming"
    } otherwise global_path_manager.platform.name == "darwin" {
        sus home tea = get_home_directory_real()
        damn join(tea[value]{home, "Library", "Application Support"})
    } otherwise {
        sus xdg_config tea = get_cached_env("XDG_CONFIG_HOME")
        lowkey xdg_config != "" {
            damn xdg_config
        }
        sus home tea = get_home_directory_real()
        damn join(tea[value]{home, ".config"})
    }
}

slay get_user_cache_directory_real() tea {
    lowkey global_path_manager.platform.family == "windows" {
        sus localappdata tea = get_cached_env("LOCALAPPDATA")
        lowkey localappdata != "" {
            damn localappdata
        }
        damn "C:\\Users\\user\\AppData\\Local"
    } otherwise global_path_manager.platform.name == "darwin" {
        sus home tea = get_home_directory_real()
        damn join(tea[value]{home, "Library", "Caches"})
    } otherwise {
        sus xdg_cache tea = get_cached_env("XDG_CACHE_HOME")
        lowkey xdg_cache != "" {
            damn xdg_cache
        }
        sus home tea = get_home_directory_real()
        damn join(tea[value]{home, ".cache"})
    }
}

slay get_user_data_directory_real() tea {
    lowkey global_path_manager.platform.family == "windows" {
        sus appdata tea = get_cached_env("APPDATA")
        lowkey appdata != "" {
            damn appdata
        }
        damn "C:\\Users\\user\\AppData\\Roaming"
    } otherwise global_path_manager.platform.name == "darwin" {
        sus home tea = get_home_directory_real()
        damn join(tea[value]{home, "Library", "Application Support"})
    } otherwise {
        sus xdg_data tea = get_cached_env("XDG_DATA_HOME")
        lowkey xdg_data != "" {
            damn xdg_data
        }
        sus home tea = get_home_directory_real()
        damn join(tea[value]{home, ".local", "share"})
    }
}

slay get_system_config_directory_real() tea {
    lowkey global_path_manager.platform.family == "windows" {
        sus programdata tea = get_cached_env("PROGRAMDATA")
        lowkey programdata != "" {
            damn programdata
        }
        damn "C:\\ProgramData"
    } otherwise {
        damn "/etc"
    }
}

slay get_system_data_directory_real() tea {
    lowkey global_path_manager.platform.family == "windows" {
        sus programfiles tea = get_cached_env("PROGRAMFILES")
        lowkey programfiles != "" {
            damn programfiles
        }
        damn "C:\\Program Files"
    } otherwise {
        damn "/usr/share"
    }
}

fr fr Mock implementations for system functions

slay file_exists_check(path tea) lit {
    fr fr Would use stat() or similar system call
    sus common_paths tea[value] = tea[value]{
        "/proc/version", "/System/Library/CoreServices/SystemVersion.plist",
        "C:\\Windows\\System32", "/etc/release", "/usr/bin/uname"
    }
    
    bestie _, common_path in common_paths {
        lowkey path == common_path {
            damn based
        }
    }
    
    damn cap
}

slay execute_uname() tea {
    fr fr Would execute uname command and return result
    damn "Linux"  fr fr Mock result
}

slay check_windows_extended_path_support() lit {
    fr fr Would check Windows registry or version
    damn based  fr fr Assume modern Windows supports extended paths
}

slay create_test_file(path tea) lit {
    fr fr Would actually create a test file
    damn based  fr fr Mock success
}

slay cleanup_test_file(path tea) {
    fr fr Would delete the test file
}

slay get_user_home_directory(username tea) tea {
    fr fr Would look up user home directory from system
    lowkey username == "user" {
        damn "/home/user"
    }
    damn ""
}

slay change_directory_real(path tea) lit {
    fr fr Would call chdir() or SetCurrentDirectory()
    damn based  fr fr Mock success
}

slay path_exists_real(path tea) lit {
    fr fr Would use stat() to check if path exists
    damn cap  fr fr Mock: doesn't exist
}

slay is_directory_real(path tea) lit {
    fr fr Would check S_ISDIR from stat result  
    damn cap  fr fr Mock: not a directory
}

slay get_path_permissions(path tea) normie {
    fr fr Would get permissions from stat result
    damn 0644  fr fr Mock permissions
}

slay resolve_symlinks(path tea) tea {
    fr fr Would resolve all symbolic links in path
    damn path  fr fr Mock: no symlinks to resolve
}

slay get_symlink_target_safe(path tea) tea {
    fr fr Would call readlink() if path is a symlink
    damn ""  fr fr Mock: not a symlink
}

slay resolve_path_completely(path tea) tea {
    fr fr Would resolve all symbolic links and relative components
    damn clean(abs(path))
}

fr fr ================================
fr fr Module Information and Debugging
fr fr ================================

slay get_platform_info() PlatformInfo {
    init_path_manager()
    damn global_path_manager.platform
}

slay get_path_manager_stats() map[tea]normie {
    init_path_manager()
    
    sus stats map[tea]normie = map[tea]normie{}
    stats["path_cache_size"] = len(global_path_manager.path_cache)
    stats["env_cache_size"] = len(global_path_manager.env_cache)
    stats["max_path_length"] = global_path_manager.platform.max_path_length
    stats["max_component_length"] = global_path_manager.platform.max_component_length
    
    damn stats
}

slay debug_path_info(path tea) {
    init_path_manager()
    
    sus path_info PathInfo = info(path)
    
    vibez.spill("=== Path Debug Info ===")
    vibez.spill("Original: " + path_info.original)
    vibez.spill("Absolute: " + path_info.absolute)
    vibez.spill("Canonical: " + path_info.canonical)
    vibez.spill("Directory: " + path_info.directory)
    vibez.spill("Filename: " + path_info.filename)
    vibez.spill("Basename: " + path_info.basename)
    vibez.spill("Extension: " + path_info.extension)
    vibez.spill("Is Absolute: " + stringz.from_bool(path_info.is_absolute))
    vibez.spill("Is Directory: " + stringz.from_bool(path_info.is_directory))
    vibez.spill("Is UNC: " + stringz.from_bool(path_info.is_unc_path))
    vibez.spill("Volume: " + path_info.volume)
    vibez.spill("Drive: " + path_info.drive_letter)
    vibez.spill("Exists: " + stringz.from_bool(path_info.exists))
    
    lowkey len(path_info.components) > 0 {
        vibez.spill("Components:")
        bestie i, component in path_info.components {
            vibez.spill("  [" + stringz.from_int(i) + "] " + component)
        }
    }
}

slay debug_platform_info() {
    init_path_manager()
    
    sus platform PlatformInfo = global_path_manager.platform
    
    vibez.spill("=== Platform Debug Info ===")
    vibez.spill("Name: " + platform.name)
    vibez.spill("Family: " + platform.family)
    vibez.spill("Case Sensitive: " + stringz.from_bool(platform.case_sensitive))
    vibez.spill("File Separator: '" + platform.file_separator + "'")
    vibez.spill("Path List Separator: '" + platform.path_list_separator + "'")
    vibez.spill("Max Path Length: " + stringz.from_int(platform.max_path_length))
    vibez.spill("Supports Symlinks: " + stringz.from_bool(platform.supports_symlinks))
    vibez.spill("Supports Permissions: " + stringz.from_bool(platform.supports_permissions))
    vibez.spill("Home Env Var: " + platform.home_env_var)
    
    lowkey len(platform.reserved_names) > 0 {
        vibez.spill("Reserved Names: " + stringz.join(platform.reserved_names, ", "))
    }
}

fr fr ================================
fr fr Module Cleanup
fr fr ================================

slay cleanup_path_manager() {
    init_path_manager()
    
    clear_path_cache()
    clear_env_cache()
    global_path_manager.initialized = cap
    
    vibez.spill("Path manager cleanup complete")
}
