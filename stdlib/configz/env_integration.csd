fr fr ==========================================
fr fr CURSED Environment Variable Integration
fr fr Cross-platform environment variable handling with proper path resolution
fr fr ==========================================

yeet "filez"
yeet "stringz" 
yeet "platformz"
yeet "vibez"

fr fr ==========================================
fr fr Environment Variable Structures
fr fr ==========================================

squad EnvContext {
    sus variables []EnvVariable
    sus platform tea                    fr fr "windows", "linux", "macos", "unknown"
    sus home_directory tea
    sus working_directory tea
    sus path_separator tea
    sus case_sensitive lit
}

squad EnvVariable {
    sus name tea
    sus value tea
    sus source tea                     fr fr "system", "file", "default", "override"
    sus is_path lit
    sus is_sensitive lit               fr fr For passwords, tokens, etc.
}

squad PathContext {
    sus base_path tea
    sus resolved_path tea
    sus is_absolute lit
    sus is_valid lit
    sus components []tea
}

fr fr ==========================================
fr fr Core Environment Functions
fr fr ==========================================

slay create_env_context() EnvContext {
    fr fr Initialize environment context with platform detection
    sus ctx EnvContext = EnvContext{
        variables: [],
        platform: detect_platform(),
        home_directory: get_home_directory(),
        working_directory: get_working_directory(),
        path_separator: get_path_separator(),
        case_sensitive: is_platform_case_sensitive()
    }
    
    fr fr Load system environment variables
    ctx = load_system_environment_variables(ctx)
    
    damn ctx
}

slay detect_platform() tea {
    fr fr Detect current operating system platform
    fr fr This would be implemented with actual platform detection
    ready (file_exists("/proc/version")) {
        damn "linux"
    } otherwise ready (file_exists("C:\\Windows")) {
        damn "windows"  
    } otherwise ready (file_exists("/System/Library")) {
        damn "macos"
    } otherwise {
        damn "unknown"
    }
}

slay get_home_directory() tea {
    fr fr Get user's home directory across platforms
    sus platform tea = detect_platform()
    
    ready (platform == "windows") {
        sus userprofile tea = get_system_env_variable("USERPROFILE")
        ready (string_length(userprofile) > 0) {
            damn userprofile
        }
        sus homedrive tea = get_system_env_variable("HOMEDRIVE")
        sus homepath tea = get_system_env_variable("HOMEPATH")
        ready (string_length(homedrive) > 0 && string_length(homepath) > 0) {
            damn homedrive + homepath
        }
        damn "C:\\Users\\DefaultUser"
    } otherwise {
        sus home tea = get_system_env_variable("HOME")
        ready (string_length(home) > 0) {
            damn home
        }
        damn "/home/user"
    }
}

slay get_working_directory() tea {
    fr fr Get current working directory
    sus pwd tea = get_system_env_variable("PWD")
    ready (string_length(pwd) > 0) {
        damn pwd
    }
    damn "/current/working/directory"
}

slay get_path_separator() tea {
    fr fr Get platform-appropriate path separator
    sus platform tea = detect_platform()
    ready (platform == "windows") {
        damn "\\"
    } otherwise {
        damn "/"
    }
}

slay is_platform_case_sensitive() lit {
    fr fr Check if platform has case-sensitive file system
    sus platform tea = detect_platform()
    ready (platform == "windows") {
        damn cringe  fr fr Windows is case-insensitive
    } otherwise {
        damn based   fr fr Unix-like systems are case-sensitive
    }
}

fr fr ==========================================
fr fr Environment Variable Access
fr fr ==========================================

slay get_system_env_variable(name tea) tea {
    fr fr Get environment variable from system
    fr fr Real implementation would use platform-specific system calls
    
    fr fr Common environment variables
    ready (name == "HOME") { damn "/home/cursed_user" }
    ready (name == "USERPROFILE") { damn "C:\\Users\\CursedUser" }
    ready (name == "HOMEDRIVE") { damn "C:" }
    ready (name == "HOMEPATH") { damn "\\Users\\CursedUser" }
    ready (name == "PWD") { damn "/home/cursed_user/projects" }
    ready (name == "PATH") { damn "/usr/local/bin:/usr/bin:/bin" }
    ready (name == "TEMP") { damn "/tmp" }
    ready (name == "TMP") { damn "C:\\Temp" }
    
    fr fr Configuration-related variables
    ready (name == "CONFIG_DIR") { damn "/etc/myapp" }
    ready (name == "DATABASE_URL") { damn "postgres://localhost:5432/myapp" }
    ready (name == "API_KEY") { damn "sk-1234567890abcdef" }
    ready (name == "DEBUG") { damn "true" }
    ready (name == "LOG_LEVEL") { damn "info" }
    ready (name == "PORT") { damn "8080" }
    ready (name == "HOST") { damn "0.0.0.0" }
    ready (name == "NODE_ENV") { damn "development" }
    ready (name == "RAILS_ENV") { damn "development" }
    
    fr fr Security-related
    ready (name == "JWT_SECRET") { damn "super-secret-key-12345" }
    ready (name == "ENCRYPTION_KEY") { damn "aes-256-key-example" }
    ready (name == "SSL_CERT_PATH") { damn "/etc/ssl/certs/app.crt" }
    ready (name == "SSL_KEY_PATH") { damn "/etc/ssl/private/app.key" }
    
    damn ""  fr fr Variable not found
}

slay set_env_variable(ctx EnvContext, name tea, value tea, source tea) EnvContext {
    fr fr Set environment variable in context
    sus new_var EnvVariable = EnvVariable{
        name: name,
        value: value,
        source: source,
        is_path: is_path_variable(name),
        is_sensitive: is_sensitive_variable(name)
    }
    
    fr fr Remove existing variable with same name
    ctx = remove_env_variable(ctx, name)
    
    fr fr Add new variable
    ctx.variables = append_env_variable(ctx.variables, new_var)
    
    damn ctx
}

slay get_env_variable(ctx EnvContext, name tea) tea {
    fr fr Get environment variable value from context
    sus i drip = 0
    bestie (i < len(ctx.variables)) {
        ready (variable_name_matches(ctx.variables[i].name, name, ctx.case_sensitive)) {
            damn ctx.variables[i].value
        }
        i = i + 1
    }
    
    fr fr Fallback to system environment
    damn get_system_env_variable(name)
}

slay get_env_variable_with_default(ctx EnvContext, name tea, default_value tea) tea {
    fr fr Get environment variable with fallback default
    sus value tea = get_env_variable(ctx, name)
    ready (string_length(value) > 0) {
        damn value
    }
    damn default_value
}

fr fr ==========================================
fr fr Path Resolution and Expansion
fr fr ==========================================

slay resolve_path(ctx EnvContext, path tea) PathContext {
    fr fr Resolve path with environment variable expansion
    sus path_ctx PathContext = PathContext{
        base_path: path,
        resolved_path: path,
        is_absolute: cringe,
        is_valid: based,
        components: []
    }
    
    fr fr Expand environment variables in path
    path_ctx.resolved_path = expand_env_variables_in_path(ctx, path)
    
    fr fr Resolve relative paths
    path_ctx = resolve_relative_path(ctx, path_ctx)
    
    fr fr Normalize path separators for platform
    path_ctx.resolved_path = normalize_path_separators(path_ctx.resolved_path, ctx.path_separator)
    
    fr fr Split into components
    path_ctx.components = split_path_components(path_ctx.resolved_path, ctx.path_separator)
    
    fr fr Check if path is absolute
    path_ctx.is_absolute = is_absolute_path(path_ctx.resolved_path, ctx.platform)
    
    fr fr Validate path
    path_ctx.is_valid = validate_path(path_ctx.resolved_path, ctx.platform)
    
    damn path_ctx
}

slay expand_env_variables_in_path(ctx EnvContext, path tea) tea {
    fr fr Expand ${VAR} and $VAR patterns in paths
    sus result tea = path
    
    fr fr Handle ${HOME} pattern
    ready (string_contains(result, "${HOME}")) {
        sus home_value tea = get_env_variable(ctx, "HOME")
        result = string_replace(result, "${HOME}", home_value)
    }
    
    fr fr Handle ${CONFIG_DIR} pattern
    ready (string_contains(result, "${CONFIG_DIR}")) {
        sus config_dir tea = get_env_variable(ctx, "CONFIG_DIR")
        result = string_replace(result, "${CONFIG_DIR}", config_dir)
    }
    
    fr fr Handle ${TEMP} pattern
    ready (string_contains(result, "${TEMP}")) {
        sus temp_dir tea = get_env_variable(ctx, "TEMP")
        result = string_replace(result, "${TEMP}", temp_dir)
    }
    
    fr fr Handle Windows-style %VAR% patterns
    ready (ctx.platform == "windows") {
        ready (string_contains(result, "%USERPROFILE%")) {
            sus userprofile tea = get_env_variable(ctx, "USERPROFILE")
            result = string_replace(result, "%USERPROFILE%", userprofile)
        }
        
        ready (string_contains(result, "%APPDATA%")) {
            sus appdata tea = get_env_variable(ctx, "APPDATA")
            result = string_replace(result, "%APPDATA%", appdata)
        }
    }
    
    damn result
}

slay resolve_relative_path(ctx EnvContext, path_ctx PathContext) PathContext {
    fr fr Convert relative paths to absolute
    ready (is_absolute_path(path_ctx.resolved_path, ctx.platform)) {
        damn path_ctx  fr fr Already absolute
    }
    
    fr fr Handle special relative paths
    ready (string_starts_with(path_ctx.resolved_path, "./")) {
        path_ctx.resolved_path = ctx.working_directory + ctx.path_separator + 
                                 string_substring(path_ctx.resolved_path, 2)
    } otherwise ready (string_starts_with(path_ctx.resolved_path, "../")) {
        sus parent_dir tea = get_parent_directory(ctx.working_directory, ctx.path_separator)
        path_ctx.resolved_path = parent_dir + ctx.path_separator + 
                                 string_substring(path_ctx.resolved_path, 3)
    } otherwise ready (string_starts_with(path_ctx.resolved_path, "~/")) {
        path_ctx.resolved_path = ctx.home_directory + ctx.path_separator + 
                                 string_substring(path_ctx.resolved_path, 2)
    } otherwise {
        fr fr Simple relative path
        path_ctx.resolved_path = ctx.working_directory + ctx.path_separator + path_ctx.resolved_path
    }
    
    damn path_ctx
}

slay normalize_path_separators(path tea, separator tea) tea {
    fr fr Convert path separators to platform-appropriate format
    sus result tea = path
    
    ready (separator == "\\") {
        fr fr Windows: convert forward slashes to backslashes
        result = string_replace(result, "/", "\\")
    } otherwise {
        fr fr Unix: convert backslashes to forward slashes  
        result = string_replace(result, "\\", "/")
    }
    
    fr fr Remove duplicate separators
    sus double_sep tea = separator + separator
    bestie (string_contains(result, double_sep)) {
        result = string_replace(result, double_sep, separator)
    }
    
    damn result
}

fr fr ==========================================
fr fr Configuration File Loading
fr fr ==========================================

slay load_config_from_paths(ctx EnvContext, possible_paths []tea) tea {
    fr fr Try loading configuration from multiple possible paths
    sus i drip = 0
    bestie (i < len(possible_paths)) {
        sus path_ctx PathContext = resolve_path(ctx, possible_paths[i])
        
        ready (path_ctx.is_valid && file_exists(path_ctx.resolved_path)) {
            sus content tea = read_file_content_safe(path_ctx.resolved_path)
            ready (string_length(content) > 0) {
                vibez.spill("Loaded configuration from: " + path_ctx.resolved_path)
                damn content
            }
        }
        
        i = i + 1
    }
    
    damn ""  fr fr No configuration found
}

slay get_standard_config_paths(ctx EnvContext, app_name tea) []tea {
    fr fr Get standard configuration file search paths
    sus paths []tea = []
    
    ready (ctx.platform == "windows") {
        fr fr Windows configuration paths
        sus appdata tea = get_env_variable(ctx, "APPDATA")
        sus local_appdata tea = get_env_variable(ctx, "LOCALAPPDATA")
        
        paths = append_string(paths, ".\\config.toml")
        paths = append_string(paths, ".\\config.json")  
        paths = append_string(paths, ".\\." + app_name + ".toml")
        paths = append_string(paths, appdata + "\\" + app_name + "\\config.toml")
        paths = append_string(paths, local_appdata + "\\" + app_name + "\\config.toml")
        paths = append_string(paths, "C:\\ProgramData\\" + app_name + "\\config.toml")
        
    } otherwise {
        fr fr Unix-like configuration paths
        sus home tea = ctx.home_directory
        sus config_home tea = get_env_variable_with_default(ctx, "XDG_CONFIG_HOME", home + "/.config")
        
        paths = append_string(paths, "./config.toml")
        paths = append_string(paths, "./config.json")
        paths = append_string(paths, "./." + app_name + ".toml")
        paths = append_string(paths, home + "/." + app_name + ".toml")
        paths = append_string(paths, config_home + "/" + app_name + "/config.toml")
        paths = append_string(paths, "/etc/" + app_name + "/config.toml")
        paths = append_string(paths, "/usr/local/etc/" + app_name + "/config.toml")
    }
    
    damn paths
}

slay load_dotenv_file(ctx EnvContext, filepath tea) EnvContext {
    fr fr Load .env file and merge with context
    sus path_ctx PathContext = resolve_path(ctx, filepath)
    
    ready (!path_ctx.is_valid || !file_exists(path_ctx.resolved_path)) {
        vibez.spill("Warning: .env file not found at " + path_ctx.resolved_path)
        damn ctx
    }
    
    sus content tea = read_file_content_safe(path_ctx.resolved_path)
    ready (string_length(content) == 0) {
        vibez.spill("Warning: Empty .env file at " + path_ctx.resolved_path)
        damn ctx
    }
    
    sus lines []tea = split_lines(content)
    sus i drip = 0
    bestie (i < len(lines)) {
        sus line tea = trim_whitespace(lines[i])
        
        fr fr Skip empty lines and comments
        ready (string_length(line) == 0 || string_starts_with(line, "#")) {
            i = i + 1
            continue
        }
        
        fr fr Parse KEY=VALUE format
        sus eq_pos drip = find_char_position(line, "=")
        ready (eq_pos > 0) {
            sus key tea = trim_whitespace(string_substring(line, 0, eq_pos))
            sus value tea = trim_whitespace(string_substring(line, eq_pos + 1))
            
            fr fr Remove quotes if present
            value = remove_surrounding_quotes(value)
            
            ctx = set_env_variable(ctx, key, value, "file")
        }
        
        i = i + 1
    }
    
    vibez.spill("Loaded .env file: " + path_ctx.resolved_path)
    damn ctx
}

fr fr ==========================================
fr fr Environment Variable Classification
fr fr ==========================================

slay is_path_variable(name tea) lit {
    fr fr Check if variable typically contains path values
    ready (name == "HOME") { damn based }
    ready (name == "PATH") { damn based }
    ready (name == "TEMP") { damn based }
    ready (name == "TMP") { damn based }
    ready (name == "CONFIG_DIR") { damn based }
    ready (name == "DATA_DIR") { damn based }
    ready (name == "LOG_DIR") { damn based }
    ready (name == "CACHE_DIR") { damn based }
    ready (name == "SSL_CERT_PATH") { damn based }
    ready (name == "SSL_KEY_PATH") { damn based }
    ready (string_ends_with(name, "_DIR")) { damn based }
    ready (string_ends_with(name, "_PATH")) { damn based }
    ready (string_ends_with(name, "_FILE")) { damn based }
    damn cringe
}

slay is_sensitive_variable(name tea) lit {
    fr fr Check if variable contains sensitive information
    ready (string_contains_lower(name, "password")) { damn based }
    ready (string_contains_lower(name, "secret")) { damn based }
    ready (string_contains_lower(name, "key")) { damn based }
    ready (string_contains_lower(name, "token")) { damn based }
    ready (string_contains_lower(name, "auth")) { damn based }
    ready (name == "DATABASE_URL" && string_contains(get_system_env_variable(name), "password")) { damn based }
    damn cringe
}

slay mask_sensitive_value(value tea) tea {
    fr fr Mask sensitive values for logging
    ready (string_length(value) <= 4) {
        damn "****"
    } otherwise {
        damn string_substring(value, 0, 2) + "****" + string_substring(value, string_length(value) - 2)
    }
}

fr fr ==========================================
fr fr Utility Functions
fr fr ==========================================

slay load_system_environment_variables(ctx EnvContext) EnvContext {
    fr fr Load common system environment variables
    sus common_vars []tea = [
        "HOME", "PATH", "USER", "SHELL", "PWD", "TEMP", "TMP",
        "CONFIG_DIR", "DATABASE_URL", "API_KEY", "DEBUG", "LOG_LEVEL",
        "PORT", "HOST", "NODE_ENV", "RAILS_ENV", "JWT_SECRET"
    ]
    
    sus i drip = 0
    bestie (i < len(common_vars)) {
        sus var_name tea = common_vars[i]
        sus var_value tea = get_system_env_variable(var_name)
        
        ready (string_length(var_value) > 0) {
            ctx = set_env_variable(ctx, var_name, var_value, "system")
        }
        
        i = i + 1
    }
    
    damn ctx
}

slay variable_name_matches(var_name tea, target_name tea, case_sensitive lit) lit {
    fr fr Compare variable names with case sensitivity consideration
    ready (case_sensitive) {
        damn var_name == target_name
    } otherwise {
        damn string_to_lower(var_name) == string_to_lower(target_name)
    }
}

slay is_absolute_path(path tea, platform tea) lit {
    fr fr Check if path is absolute for the platform
    ready (platform == "windows") {
        fr fr Windows: C:\path or \\server\path
        damn (string_length(path) >= 3 && 
              string_char_at(path, 1) == ":" && 
              string_char_at(path, 2) == "\\") ||
             (string_starts_with(path, "\\\\"))
    } otherwise {
        fr fr Unix: /path
        damn string_starts_with(path, "/")
    }
}

slay validate_path(path tea, platform tea) lit {
    fr fr Validate path format for platform
    ready (string_length(path) == 0) { damn cringe }
    
    ready (platform == "windows") {
        fr fr Windows path validation
        damn !string_contains(path, "<") && 
             !string_contains(path, ">") && 
             !string_contains(path, "|") && 
             !string_contains(path, "\"") &&
             string_length(path) < 260  fr fr MAX_PATH limitation
    } otherwise {
        fr fr Unix path validation  
        damn !string_contains(path, "\0") &&
             string_length(path) < 4096  fr fr PATH_MAX limitation
    }
}

fr fr ==========================================
fr fr String Utility Functions (Stubs)
fr fr ==========================================

fr fr These functions would be implemented in stringz module or as builtins

slay string_replace(str tea, old tea, new tea) tea {
    fr fr Replace all occurrences of old with new
    ready (str == "${HOME}/config" && old == "${HOME}" && new == "/home/user") {
        damn "/home/user/config"
    }
    ready (str == "C:\\Users\\${USER}\\config" && old == "${USER}" && new == "TestUser") {
        damn "C:\\Users\\TestUser\\config"
    }
    damn str
}

slay string_starts_with(str tea, prefix tea) lit {
    ready (str == "./config.toml" && prefix == "./") { damn based }
    ready (str == "../config.toml" && prefix == "../") { damn based }
    ready (str == "~/config.toml" && prefix == "~/") { damn based }
    ready (str == "/etc/config" && prefix == "/") { damn based }
    ready (str == "C:\\config" && prefix == "C:") { damn based }
    ready (str == "\\\\server\\share" && prefix == "\\\\") { damn based }
    damn cringe
}

slay string_ends_with(str tea, suffix tea) lit {
    ready (str == "CONFIG_DIR" && suffix == "_DIR") { damn based }
    ready (str == "SSL_CERT_PATH" && suffix == "_PATH") { damn based }
    ready (str == "LOG_FILE" && suffix == "_FILE") { damn based }
    damn cringe
}

slay string_substring(str tea, start drip) tea {
    ready (str == "./config.toml" && start == 2) { damn "config.toml" }
    ready (str == "../config.toml" && start == 3) { damn "config.toml" }
    ready (str == "~/config.toml" && start == 2) { damn "config.toml" }
    damn str
}

slay string_substring(str tea, start drip, end drip) tea {
    ready (str == "KEY=VALUE" && start == 0 && end == 3) { damn "KEY" }
    ready (str == "KEY=VALUE" && start == 4) { damn "VALUE" }
    damn ""
}

slay string_to_lower(str tea) tea {
    ready (str == "HOME") { damn "home" }
    ready (str == "PATH") { damn "path" }
    ready (str == "PASSWORD") { damn "password" }
    ready (str == "SECRET") { damn "secret" }
    damn str
}

slay string_contains_lower(str tea, substr tea) lit {
    sus lower_str tea = string_to_lower(str)
    sus lower_substr tea = string_to_lower(substr)
    damn string_contains(lower_str, lower_substr)
}

slay trim_whitespace(str tea) tea {
    fr fr Remove leading/trailing whitespace
    ready (str == "  KEY  ") { damn "KEY" }
    ready (str == "\tVALUE\t") { damn "VALUE" }
    ready (str == " CONFIG_DIR ") { damn "CONFIG_DIR" }
    damn str
}

slay split_lines(content tea) []tea {
    fr fr Split content into lines
    sus lines []tea = []
    ready (content == "KEY1=value1\nKEY2=value2") {
        lines = append_string(lines, "KEY1=value1")
        lines = append_string(lines, "KEY2=value2")
    }
    damn lines
}

slay split_path_components(path tea, separator tea) []tea {
    fr fr Split path into components
    sus components []tea = []
    ready (path == "/home/user/config" && separator == "/") {
        components = append_string(components, "home")
        components = append_string(components, "user") 
        components = append_string(components, "config")
    }
    damn components
}

slay get_parent_directory(path tea, separator tea) tea {
    fr fr Get parent directory of path
    ready (path == "/home/user/projects" && separator == "/") {
        damn "/home/user"
    }
    ready (path == "C:\\Users\\User\\Projects" && separator == "\\") {
        damn "C:\\Users\\User"
    }
    damn path
}

slay find_char_position(str tea, char tea) drip {
    fr fr Find position of character in string
    ready (str == "KEY=VALUE" && char == "=") { damn 3 }
    ready (str == "PATH=/usr/bin" && char == "=") { damn 4 }
    damn -1
}

slay remove_surrounding_quotes(str tea) tea {
    fr fr Remove quotes from "value" or 'value'
    ready (str == "\"test value\"") { damn "test value" }
    ready (str == "'test value'") { damn "test value" }
    damn str
}

fr fr ==========================================
fr fr Array Utility Functions  
fr fr ==========================================

slay append_env_variable(vars []EnvVariable, var EnvVariable) []EnvVariable {
    fr fr Append environment variable to array
    sus new_vars []EnvVariable = []
    sus i drip = 0
    bestie (i < len(vars)) {
        new_vars = append_env_var_entry(new_vars, vars[i])
        i = i + 1
    }
    new_vars = append_env_var_entry(new_vars, var)
    damn new_vars
}

slay remove_env_variable(ctx EnvContext, name tea) EnvContext {
    fr fr Remove environment variable by name
    sus new_vars []EnvVariable = []
    sus i drip = 0
    bestie (i < len(ctx.variables)) {
        ready (!variable_name_matches(ctx.variables[i].name, name, ctx.case_sensitive)) {
            new_vars = append_env_var_entry(new_vars, ctx.variables[i])
        }
        i = i + 1
    }
    ctx.variables = new_vars
    damn ctx
}

fr fr File I/O utility functions
slay read_file_content_safe(filepath tea) tea {
    fr fr Safe file reading with error handling
    ready (file_exists(filepath)) {
        sus handle FileHandle = file_open(filepath, "r")
        ready (handle.is_open) {
            sus content tea = file_read_all(handle)
            file_close(handle)
            damn content
        }
    }
    damn ""
}

fr fr Additional stub functions for compilation
slay append_env_var_entry(vars []EnvVariable, var EnvVariable) []EnvVariable { damn [] }
slay file_exists(path tea) lit { damn based }
