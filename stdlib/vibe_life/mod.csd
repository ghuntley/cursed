# vibe_life - Essential OS Functionality for CURSED Self-Hosting
# Pure CURSED implementation without FFI dependencies
# Provides command line, environment, process, and filesystem operations

# Global state for command line arguments (simulated)
sus command_line_args [tea] = ["cursed", "program.csd"]
sus arg_count normie = 2
sus current_working_directory tea = "/home/user"
sus process_exit_code normie = 0

# Environment variables storage (simulated)
sus env_vars map = {}
sus env_count normie = 0

# ==============================================================================
# COMMAND LINE ARGUMENT FUNCTIONS
# ==============================================================================

# Get all command line arguments as array
slay get_args() [tea] {
    damn command_line_args
}

# Get number of command line arguments
slay get_arg_count() normie {
    damn arg_count
}

# Get specific command line argument by index
slay get_arg(index normie) tea {
    if index >= 0 && index < arg_count {
        damn command_line_args[index]
    }
    damn ""
}

# Set command line arguments (for testing)
slay set_args(args [tea]) {
    command_line_args = args
    arg_count = args.length()
}

# ==============================================================================
# ENVIRONMENT VARIABLE FUNCTIONS
# ==============================================================================

# Get environment variable by key
slay get_env(key tea) tea {
    if env_vars.has_key(key) {
        damn env_vars.get(key)
    }
    damn ""
}

# Set environment variable
slay set_env(key tea, value tea) lit {
    env_vars.set(key, value)
    env_count = env_count + 1
    damn based
}

# Check if environment variable exists
slay has_env(key tea) lit {
    damn env_vars.has_key(key)
}

# Get all environment variable keys
slay get_env_keys() [tea] {
    damn env_vars.keys()
}

# Remove environment variable
slay unset_env(key tea) lit {
    if env_vars.has_key(key) {
        env_vars.remove(key)
        env_count = env_count - 1
        damn based
    }
    damn cap
}

# ==============================================================================
# PROCESS CONTROL FUNCTIONS
# ==============================================================================

# Exit the process with given code
slay exit(code normie) {
    process_exit_code = code
    vibez.spill("Process exiting with code: " + core.tea(code))
    # In real implementation, this would terminate the process
}

# Get current process exit code
slay get_exit_code() normie {
    damn process_exit_code
}

# Get current process ID (simulated)
slay get_pid() normie {
    damn 1234  # Simulated PID
}

# Get parent process ID (simulated)
slay get_ppid() normie {
    damn 1000  # Simulated PPID
}

# ==============================================================================
# WORKING DIRECTORY FUNCTIONS
# ==============================================================================

# Get current working directory
slay get_cwd() tea {
    damn current_working_directory
}

# Change working directory
slay set_cwd(path tea) lit {
    # Basic path validation
    if path != "" {
        current_working_directory = path
        damn based
    }
    damn cap
}

# Join path components
slay join_path(base tea, component tea) tea {
    if base == "" {
        damn component
    }
    if component == "" {
        damn base
    }
    if base.ends_with("/") {
        damn base + component
    }
    damn base + "/" + component
}

# Get directory name from path
slay dirname(path tea) tea {
    sus last_slash normie = -1
    sus i normie = 0
    while i < path.length() {
        if path.char_at(i) == '/' {
            last_slash = i
        }
        i = i + 1
    }
    if last_slash >= 0 {
        damn path.substring(0, last_slash)
    }
    damn "."
}

# Get base name from path
slay basename(path tea) tea {
    sus last_slash normie = -1
    sus i normie = 0
    while i < path.length() {
        if path.char_at(i) == '/' {
            last_slash = i
        }
        i = i + 1
    }
    if last_slash >= 0 {
        damn path.substring(last_slash + 1)
    }
    damn path
}

# ==============================================================================
# FILE SYSTEM OPERATIONS
# ==============================================================================

# Simulated file system storage
sus file_system map = {}
sus file_count normie = 0

# Check if file exists
slay file_exists(path tea) lit {
    damn file_system.has_key(path)
}

# Create a file with content
slay create_file(path tea, content tea) lit {
    file_system.set(path, content)
    file_count = file_count + 1
    damn based
}

# Read file content
slay read_file(path tea) tea {
    if file_system.has_key(path) {
        damn file_system.get(path)
    }
    damn ""
}

# Write content to file
slay write_file(path tea, content tea) lit {
    damn create_file(path, content)
}

# Append content to file
slay append_file(path tea, content tea) lit {
    if file_system.has_key(path) {
        sus existing tea = file_system.get(path)
        file_system.set(path, existing + content)
        damn based
    } else {
        damn create_file(path, content)
    }
}

# Delete a file
slay delete_file(path tea) lit {
    if file_system.has_key(path) {
        file_system.remove(path)
        file_count = file_count - 1
        damn based
    }
    damn cap
}

# Get file size (character count)
slay get_file_size(path tea) normie {
    if file_system.has_key(path) {
        sus content tea = file_system.get(path)
        damn content.length()
    }
    damn -1
}

# List all files in simulated filesystem
slay list_files() [tea] {
    damn file_system.keys()
}

# Create directory (simulated)
slay create_dir(path tea) lit {
    # Mark directory in filesystem
    sus dir_marker tea = path + "/.directory"
    file_system.set(dir_marker, "directory")
    damn based
}

# Check if path is directory
slay is_dir(path tea) lit {
    sus dir_marker tea = path + "/.directory"
    damn file_system.has_key(dir_marker)
}

# ==============================================================================
# UTILITY FUNCTIONS
# ==============================================================================

# Get current timestamp (simulated)
slay get_timestamp() normie {
    damn 1704067200  # Simulated Unix timestamp
}

# Sleep for specified seconds (simulated)
slay sleep(seconds normie) {
    vibez.spill("Sleeping for " + core.tea(seconds) + " seconds")
    # In real implementation, this would actually sleep
}

# Initialize vibe_life module with default environment
slay init_vibe_life() {
    # Set up default environment variables
    set_env("HOME", "/home/user")
    set_env("PATH", "/usr/bin:/bin")
    set_env("CURSED_HOME", "/usr/local/cursed")
    set_env("USER", "cursed_user")
    
    # Create some default files
    create_file("/tmp/cursed.log", "CURSED log file\n")
    create_file("/home/user/.cursedrc", "# CURSED configuration\n")
    
    vibez.spill("vibe_life module initialized")
}

# Module information
slay get_module_info() tea {
    damn "vibe_life v1.0 - Essential OS functionality for CURSED self-hosting"
}
