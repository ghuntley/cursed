fr fr envz - Environment Variable Management Module  
fr fr Pure CURSED environment variable handling for cross-platform compatibility
fr fr Essential for build systems, configuration, and deployment

yeet "core"
yeet "stringz" 
yeet "arrayz"

fr fr Environment variable storage and state
sus env_variables map<tea, tea> = {}
sus env_count normie = 0
sus env_modified lit = cap

fr fr Platform detection constants
fact PLATFORM_LINUX tea = "linux"
fact PLATFORM_WINDOWS tea = "windows" 
fact PLATFORM_MACOS tea = "macos"
fact PLATFORM_UNKNOWN tea = "unknown"

fr fr Environment variable constraints
fact MAX_VAR_NAME_LENGTH normie = 255
fact MAX_VAR_VALUE_LENGTH normie = 32767
fact MAX_ENV_VARS normie = 1000

fr fr ===== CORE ENVIRONMENT FUNCTIONS =====

slay get(key tea) tea {
    check key == "" {
        damn ""
    }
    
    check key.length() > MAX_VAR_NAME_LENGTH {
        damn ""
    }
    
    check env_variables.has_key(key) {
        damn env_variables.get(key)
    }
    
    # Try system environment (simulated)
    sus system_value tea = get_system_env(key)
    check system_value != "" {
        # Cache system value
        env_variables.set(key, system_value)
        damn system_value
    }
    
    damn ""
}

slay set(key tea, value tea) lit {
    check key == "" {
        damn cap
    }
    
    check key.length() > MAX_VAR_NAME_LENGTH {
        damn cap
    }
    
    check value.length() > MAX_VAR_VALUE_LENGTH {
        damn cap
    }
    
    check env_count >= MAX_ENV_VARS && !env_variables.has_key(key) {
        damn cap  # Too many environment variables
    }
    
    check !env_variables.has_key(key) {
        env_count = env_count + 1
    }
    
    env_variables.set(key, value)
    env_modified = based
    damn based
}

slay unset(key tea) lit {
    check key == "" {
        damn cap
    }
    
    check env_variables.has_key(key) {
        env_variables.remove(key)
        env_count = env_count - 1
        env_modified = based
        damn based
    }
    
    damn cap
}

slay exists(key tea) lit {
    check key == "" {
        damn cap
    }
    
    damn env_variables.has_key(key) || get_system_env(key) != ""
}

slay get_all() map<tea, tea> {
    # Merge system environment with local overrides
    sus all_vars map<tea, tea> = get_system_env_all()
    
    # Apply local overrides
    sus keys [tea] = env_variables.keys()
    sus i normie = 0
    bestie i < arrayz.len(keys) {
        sus key tea = keys[i]
        sus value tea = env_variables.get(key)
        all_vars.set(key, value)
        i = i + 1
    }
    
    damn all_vars
}

slay get_keys() [tea] {
    sus all_vars map<tea, tea> = get_all()
    damn all_vars.keys()
}

fr fr ===== ENVIRONMENT EXPANSION =====

slay expand(template tea) tea {
    check template == "" {
        damn ""
    }
    
    sus result tea = template
    sus i normie = 0
    
    bestie i < template.length() {
        check template.char_at(i) == '$' && i + 1 < template.length() {
            check template.char_at(i + 1) == '{' {
                # ${VAR_NAME} format
                sus end_pos normie = find_closing_brace(template, i + 2)
                check end_pos > i + 2 {
                    sus var_name tea = template.substring(i + 2, end_pos)
                    sus var_value tea = get(var_name)
                    sus placeholder tea = template.substring(i, end_pos + 1)
                    result = stringz.replace(result, placeholder, var_value)
                }
                i = end_pos + 1
            } else {
                # $VAR_NAME format (simple)
                sus var_end normie = find_var_end(template, i + 1)
                check var_end > i + 1 {
                    sus var_name tea = template.substring(i + 1, var_end)
                    sus var_value tea = get(var_name)
                    sus placeholder tea = template.substring(i, var_end)
                    result = stringz.replace(result, placeholder, var_value)
                }
                i = var_end
            }
        } else {
            i = i + 1
        }
    }
    
    damn result
}

slay find_closing_brace(text tea, start normie) normie {
    sus i normie = start
    bestie i < text.length() {
        check text.char_at(i) == '}' {
            damn i
        }
        i = i + 1
    }
    damn -1
}

slay find_var_end(text tea, start normie) normie {
    sus i normie = start
    bestie i < text.length() {
        sus c tea = text.char_at(i)
        check !is_var_char(c) {
            damn i
        }
        i = i + 1
    }
    damn text.length()
}

slay is_var_char(c tea) lit {
    # Valid environment variable characters: A-Z, a-z, 0-9, _
    check c >= "A" && c <= "Z" {
        damn based
    }
    check c >= "a" && c <= "z" {
        damn based
    }
    check c >= "0" && c <= "9" {
        damn based
    }
    check c == "_" {
        damn based
    }
    damn cap
}

fr fr ===== PLATFORM-SPECIFIC FUNCTIONS =====

slay get_platform() tea {
    # Detect platform from environment
    sus os_type tea = get("OS")
    sus ostype tea = get("OSTYPE")
    
    check stringz.contains(os_type, "Windows") {
        damn PLATFORM_WINDOWS
    }
    
    check stringz.contains(ostype, "linux") {
        damn PLATFORM_LINUX
    }
    
    check stringz.contains(ostype, "darwin") {
        damn PLATFORM_MACOS
    }
    
    # Default detection
    sus uname tea = get("UNAME") 
    check uname == "Linux" {
        damn PLATFORM_LINUX
    }
    check uname == "Darwin" {
        damn PLATFORM_MACOS
    }
    
    damn PLATFORM_UNKNOWN
}

slay get_path_separator() tea {
    sus platform tea = get_platform()
    check platform == PLATFORM_WINDOWS {
        damn ";"
    }
    damn ":"
}

slay get_path_var() tea {
    sus platform tea = get_platform()
    check platform == PLATFORM_WINDOWS {
        damn "PATH"
    }
    damn "PATH"
}

slay get_home_var() tea {
    sus platform tea = get_platform()
    check platform == PLATFORM_WINDOWS {
        damn "USERPROFILE"
    }
    damn "HOME"
}

fr fr ===== PATH MANIPULATION =====

slay get_path() [tea] {
    sus path_var tea = get_path_var()
    sus path_value tea = get(path_var)
    check path_value == "" {
        damn []tea{}
    }
    
    sus separator tea = get_path_separator()
    damn stringz.split(path_value, separator)
}

slay set_path(paths [tea]) lit {
    sus path_var tea = get_path_var()
    sus separator tea = get_path_separator()
    sus path_value tea = stringz.join(paths, separator)
    damn set(path_var, path_value)
}

slay add_to_path(new_path tea) lit {
    sus current_paths [tea] = get_path()
    
    # Check if already in path
    sus i normie = 0
    bestie i < arrayz.len(current_paths) {
        check current_paths[i] == new_path {
            damn based  # Already in path
        }
        i = i + 1
    }
    
    # Add to front of path
    sus new_paths [tea] = arrayz.prepend(current_paths, new_path)
    damn set_path(new_paths)
}

slay remove_from_path(remove_path tea) lit {
    sus current_paths [tea] = get_path()
    sus filtered_paths [tea] = arrayz.filter(current_paths, slay(path tea) lit {
        damn path != remove_path
    })
    damn set_path(filtered_paths)
}

fr fr ===== COMMON ENVIRONMENT VARIABLES =====

slay get_home() tea {
    sus home_var tea = get_home_var()
    damn get(home_var)
}

slay get_user() tea {
    sus platform tea = get_platform()
    check platform == PLATFORM_WINDOWS {
        damn get("USERNAME")
    }
    damn get("USER")
}

slay get_shell() tea {
    sus platform tea = get_platform()
    check platform == PLATFORM_WINDOWS {
        damn get("COMSPEC")
    }
    sus shell tea = get("SHELL")
    check shell == "" {
        damn "/bin/sh"  # Default
    }
    damn shell
}

slay get_editor() tea {
    sus editor tea = get("EDITOR")
    check editor != "" {
        damn editor
    }
    
    sus visual tea = get("VISUAL")
    check visual != "" {
        damn visual
    }
    
    # Platform defaults
    sus platform tea = get_platform()
    check platform == PLATFORM_WINDOWS {
        damn "notepad"
    }
    damn "vi"
}

slay get_temp_dir() tea {
    sus platform tea = get_platform()
    check platform == PLATFORM_WINDOWS {
        sus temp tea = get("TEMP")
        check temp != "" {
            damn temp
        }
        damn get("TMP")
    }
    
    sus tmpdir tea = get("TMPDIR")
    check tmpdir != "" {
        damn tmpdir
    }
    damn "/tmp"
}

fr fr ===== SYSTEM INTEGRATION (SIMULATED) =====

slay get_system_env(key tea) tea {
    # Simulate common system environment variables
    check key == "HOME" {
        damn "/home/user"
    }
    check key == "USER" {
        damn "cursed_user"  
    }
    check key == "SHELL" {
        damn "/bin/bash"
    }
    check key == "PATH" {
        damn "/usr/local/bin:/usr/bin:/bin"
    }
    check key == "EDITOR" {
        damn "vim"
    }
    check key == "TMPDIR" {
        damn "/tmp"
    }
    check key == "OS" {
        damn "Linux"
    }
    check key == "OSTYPE" {
        damn "linux-gnu"
    }
    check key == "LANG" {
        damn "en_US.UTF-8"
    }
    check key == "PWD" {
        damn "/home/user/cursed"
    }
    damn ""
}

slay get_system_env_all() map<tea, tea> {
    sus system_vars map<tea, tea> = {}
    
    # Common system environment variables
    system_vars.set("HOME", "/home/user")
    system_vars.set("USER", "cursed_user")
    system_vars.set("SHELL", "/bin/bash") 
    system_vars.set("PATH", "/usr/local/bin:/usr/bin:/bin")
    system_vars.set("EDITOR", "vim")
    system_vars.set("TMPDIR", "/tmp")
    system_vars.set("OS", "Linux")
    system_vars.set("OSTYPE", "linux-gnu")
    system_vars.set("LANG", "en_US.UTF-8")
    system_vars.set("PWD", "/home/user/cursed")
    system_vars.set("TERM", "xterm-256color")
    system_vars.set("COLORTERM", "truecolor")
    
    damn system_vars
}

fr fr ===== MODULE UTILITIES =====

slay print_env() {
    sus all_vars map<tea, tea> = get_all()
    sus keys [tea] = all_vars.keys()
    
    vibez.spill("Environment Variables:")
    sus i normie = 0
    bestie i < arrayz.len(keys) {
        sus key tea = keys[i]
        sus value tea = all_vars.get(key)
        vibez.spill(key + "=" + value)
        i = i + 1
    }
}

slay clear_local() {
    env_variables = {}
    env_count = 0
    env_modified = based
}

slay get_env_count() normie {
    damn get_all().size()
}

slay is_modified() lit {
    damn env_modified
}

slay reset_modified_flag() {
    env_modified = cap
}

fr fr ===== MODULE INITIALIZATION =====

slay init_envz() {
    env_variables = {}
    env_count = 0
    env_modified = cap
    vibez.spill("envz module initialized")
}

slay get_envz_info() tea {
    damn "envz v1.0 - Environment Variable Management Module"
}
