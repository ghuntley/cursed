// envz Module - Environment Variable Operations
// Provides comprehensive environment variable management with runtime bridge integration

yeet "testz"

// Environment variable operations with runtime bridge to system calls

slay get_env(name tea) (tea, tea) {
    // Get environment variable value
    lowkey len_str(name) == 0 {
        damn ("", "Empty environment variable name not allowed")
    }
    
    // Bridge to Zig runtime for actual environment variable retrieval
    (value, err) := runtime_get_env(name)
    lowkey err != "" {
        damn ("", "Failed to get environment variable: " + name + " - " + err)
    }
    
    damn (value, "")
}

slay set_env(name tea, value tea) tea {
    // Set environment variable
    lowkey len_str(name) == 0 {
        damn "Empty environment variable name not allowed"
    }
    
    // Bridge to Zig runtime for actual environment variable setting
    err := runtime_set_env(name, value)
    lowkey err != "" {
        damn "Failed to set environment variable: " + name + " - " + err
    }
    
    damn ""
}

slay unset_env(name tea) tea {
    // Remove environment variable
    lowkey len_str(name) == 0 {
        damn "Empty environment variable name not allowed"
    }
    
    // Bridge to Zig runtime for environment variable removal
    err := runtime_unset_env(name)
    lowkey err != "" {
        damn "Failed to unset environment variable: " + name + " - " + err
    }
    
    damn ""
}

slay env_exists(name tea) lit {
    // Check if environment variable exists
    lowkey len_str(name) == 0 {
        damn cringe
    }
    
    (_, err) := get_env(name)
    damn err == ""
}

slay list_env() ([]tea, tea) {
    // List all environment variables
    (env_list, err) := runtime_list_env()
    lowkey err != "" {
        damn ([], "Failed to list environment variables - " + err)
    }
    
    damn (env_list, "")
}

slay get_env_with_default(name tea, default_value tea) tea {
    // Get environment variable with default value
    (value, err) := get_env(name)
    lowkey err != "" {
        damn default_value
    }
    
    lowkey len_str(value) == 0 {
        damn default_value
    }
    
    damn value
}

slay get_env_as_int(name tea, default_value normie) normie {
    // Get environment variable as integer
    (value, err) := get_env(name)
    lowkey err != "" {
        damn default_value
    }
    
    (int_value, parse_err) := parse_int(value)
    lowkey parse_err != "" {
        damn default_value
    }
    
    damn int_value
}

slay get_env_as_bool(name tea, default_value lit) lit {
    // Get environment variable as boolean
    (value, err) := get_env(name)
    lowkey err != "" {
        damn default_value
    }
    
    sus lower_value tea = to_lowercase(value)
    lowkey lower_value == "true" || lower_value == "1" || lower_value == "yes" || lower_value == "on" {
        damn based
    }
    
    lowkey lower_value == "false" || lower_value == "0" || lower_value == "no" || lower_value == "off" {
        damn cringe
    }
    
    damn default_value
}

slay expand_env(text tea) tea {
    // Expand environment variables in text (${VAR} or $VAR format)
    expanded := runtime_expand_env(text)
    damn expanded
}

slay get_path_env() []tea {
    // Get PATH environment variable as array of directories
    (path_value, err) := get_env("PATH")
    lowkey err != "" {
        damn []
    }
    
    paths := split_path_string(path_value)
    damn paths
}

slay get_home_dir() (tea, tea) {
    // Get user home directory from environment
    (home, err) := get_env("HOME")
    lowkey err == "" && len_str(home) > 0 {
        damn (home, "")
    }
    
    // Try Windows-style home directory
    (userprofile, profile_err) := get_env("USERPROFILE")
    lowkey profile_err == "" && len_str(userprofile) > 0 {
        damn (userprofile, "")
    }
    
    damn ("", "Could not determine home directory")
}

slay get_temp_dir_env() (tea, tea) {
    // Get temporary directory from environment
    // Try multiple common environment variables
    sus temp_vars []tea = ["TMPDIR", "TMP", "TEMP", "TEMPDIR"]
    sus i normie = 0
    
    bestie (i < len(temp_vars)) {
        (temp_dir, err) := get_env(temp_vars[i])
        lowkey err == "" && len_str(temp_dir) > 0 {
            damn (temp_dir, "")
        }
        i = i + 1
    }
    
    damn ("", "Could not determine temp directory")
}

slay get_user_name() (tea, tea) {
    // Get current user name from environment
    (user, err) := get_env("USER")
    lowkey err == "" && len_str(user) > 0 {
        damn (user, "")
    }
    
    // Try Windows-style username
    (username, username_err) := get_env("USERNAME")
    lowkey username_err == "" && len_str(username) > 0 {
        damn (username, "")
    }
    
    damn ("", "Could not determine user name")
}

slay get_shell() (tea, tea) {
    // Get user shell from environment
    (shell, err) := get_env("SHELL")
    lowkey err == "" && len_str(shell) > 0 {
        damn (shell, "")
    }
    
    damn ("", "Could not determine shell")
}

slay get_editor() (tea, tea) {
    // Get preferred editor from environment
    (editor, err) := get_env("EDITOR")
    lowkey err == "" && len_str(editor) > 0 {
        damn (editor, "")
    }
    
    // Try visual editor
    (visual, visual_err) := get_env("VISUAL")
    lowkey visual_err == "" && len_str(visual) > 0 {
        damn (visual, "")
    }
    
    damn ("", "Could not determine editor")
}

slay is_development_env() lit {
    // Check if running in development environment
    (env, err) := get_env("NODE_ENV")
    lowkey err == "" && env == "development" {
        damn based
    }
    
    (rails_env, rails_err) := get_env("RAILS_ENV")
    lowkey rails_err == "" && rails_env == "development" {
        damn based
    }
    
    (env_var, env_err) := get_env("ENVIRONMENT")
    lowkey env_err == "" && (env_var == "development" || env_var == "dev") {
        damn based
    }
    
    damn cringe
}

slay is_production_env() lit {
    // Check if running in production environment
    (env, err) := get_env("NODE_ENV")
    lowkey err == "" && env == "production" {
        damn based
    }
    
    (rails_env, rails_err) := get_env("RAILS_ENV")
    lowkey rails_err == "" && rails_env == "production" {
        damn based
    }
    
    (env_var, env_err) := get_env("ENVIRONMENT")
    lowkey env_err == "" && (env_var == "production" || env_var == "prod") {
        damn based
    }
    
    damn cringe
}

slay clear_env() tea {
    // Clear all environment variables (dangerous operation)
    err := runtime_clear_env()
    lowkey err != "" {
        damn "Failed to clear environment variables - " + err
    }
    
    damn ""
}

slay copy_env_to_new_process() []tea {
    // Get environment as key=value strings for new process
    (env_strings, err) := runtime_env_for_process()
    lowkey err != "" {
        damn []
    }
    
    damn env_strings
}

// Helper functions for string operations
slay len_str(str tea) normie {
    // String length - runtime implemented
    length := runtime_string_length(str)
    damn length
}

slay to_lowercase(str tea) tea {
    // Convert string to lowercase - runtime implemented
    lower := runtime_to_lowercase(str)
    damn lower
}

slay split_path_string(path_str tea) []tea {
    // Split PATH string by separator - runtime implemented
    paths := runtime_split_path(path_str)
    damn paths
}

slay parse_int(str tea) (normie, tea) {
    // Parse string to integer - runtime implemented
    (value, err) := runtime_parse_int(str)
    damn (value, err)
}

// Runtime bridge functions - implemented in Zig runtime
slay runtime_get_env(name tea) (tea, tea) {
    // Get environment variable - implemented in Zig runtime
    damn ("", "Runtime binding required")
}

slay runtime_set_env(name tea, value tea) tea {
    // Set environment variable - implemented in Zig runtime
    damn "Runtime binding required"
}

slay runtime_unset_env(name tea) tea {
    // Unset environment variable - implemented in Zig runtime
    damn "Runtime binding required"
}

slay runtime_list_env() ([]tea, tea) {
    // List all environment variables - implemented in Zig runtime
    damn ([], "Runtime binding required")
}

slay runtime_expand_env(text tea) tea {
    // Expand environment variables in text - implemented in Zig runtime
    damn text // Fallback: return text unchanged
}

slay runtime_clear_env() tea {
    // Clear all environment variables - implemented in Zig runtime
    damn "Runtime binding required"
}

slay runtime_env_for_process() ([]tea, tea) {
    // Get environment for new process - implemented in Zig runtime
    damn ([], "Runtime binding required")
}

slay runtime_string_length(str tea) normie {
    // String length - implemented in Zig runtime
    damn 0
}

slay runtime_to_lowercase(str tea) tea {
    // Convert to lowercase - implemented in Zig runtime
    damn str
}

slay runtime_split_path(path_str tea) []tea {
    // Split PATH string - implemented in Zig runtime
    damn []
}

slay runtime_parse_int(str tea) (normie, tea) {
    // Parse string to integer - implemented in Zig runtime
    damn (0, "Runtime binding required")
}
