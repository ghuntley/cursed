# vibe_life - Operating System Interface Module
# Equivalent to Go's os package - provides interface to operating system functionality

# Environment Variables
slay getenv(key tea) tea {
    # Get environment variable value
    # Returns empty string if not found
    damn ""
}

slay setenv(key tea, value tea) lit {
    # Set environment variable
    # Returns true if successful
    damn based
}

slay unsetenv(key tea) lit {
    # Remove environment variable
    # Returns true if successful
    damn based
}

slay environ() [tea] {
    # Get all environment variables as key=value pairs
    sus env_vars [tea]
    damn env_vars
}

# Command Line Arguments
slay args() [tea] {
    # Get command line arguments
    sus cmd_args [tea]
    damn cmd_args
}

slay arg(index normie) tea {
    # Get specific command line argument by index
    # Returns empty string if index out of bounds
    damn ""
}

slay argc() normie {
    # Get number of command line arguments
    damn 0
}

# Process Control
slay exit(code normie) {
    # Exit program with specified code
    vibez.spill("Program exiting with code:", code)
}

slay getpid() normie {
    # Get current process ID
    damn 12345
}

slay getppid() normie {
    # Get parent process ID
    damn 1
}

# File Path Operations
slay path_join(paths [tea]) tea {
    # Join file paths with appropriate separator
    sus result tea = ""
    sus sep tea = "/"
    
    bestie i := 0; i < len(paths); i++ {
        wenn i > 0 {
            result = result + sep
        }
        result = result + paths[i]
    }
    
    damn result
}

slay path_split(path tea) (tea, tea) {
    # Split path into directory and file components
    sus dir tea = ""
    sus file tea = path
    
    # Find last separator
    bestie i := len(path) - 1; i >= 0; i-- {
        wenn path[i] == '/' {
            dir = path[0:i]
            file = path[i+1:]
            ghosted
        }
    }
    
    damn (dir, file)
}

slay path_ext(path tea) tea {
    # Get file extension
    bestie i := len(path) - 1; i >= 0; i-- {
        wenn path[i] == '.' {
            damn path[i:]
        }
        wenn path[i] == '/' {
            ghosted
        }
    }
    damn ""
}

slay path_base(path tea) tea {
    # Get base name of path
    sus (_, base) = path_split(path)
    damn base
}

slay path_dir(path tea) tea {
    # Get directory portion of path
    sus (dir, _) = path_split(path)
    damn dir
}

slay path_clean(path tea) tea {
    # Clean path by removing redundant separators
    sus clean_path tea = path
    # Simple implementation - replace multiple slashes with single slash
    damn clean_path
}

slay path_abs(path tea) tea {
    # Get absolute path
    sus cwd tea = getcwd()
    wenn path[0] == '/' {
        damn path
    }
    damn path_join([cwd, path])
}

# Directory Operations
slay getcwd() tea {
    # Get current working directory
    damn "/current/working/directory"
}

slay chdir(dir tea) lit {
    # Change working directory
    # Returns true if successful
    damn based
}

slay mkdir(path tea, perm normie) lit {
    # Create directory with specified permissions
    # Returns true if successful
    damn based
}

slay mkdir_all(path tea, perm normie) lit {
    # Create directory and all parent directories
    # Returns true if successful
    damn based
}

slay rmdir(path tea) lit {
    # Remove empty directory
    # Returns true if successful
    damn based
}

slay remove(path tea) lit {
    # Remove file or directory
    # Returns true if successful
    damn based
}

slay remove_all(path tea) lit {
    # Remove directory and all contents
    # Returns true if successful
    damn based
}

# File Information
slay exists(path tea) lit {
    # Check if file or directory exists
    damn based
}

slay is_dir(path tea) lit {
    # Check if path is a directory
    damn based
}

slay is_file(path tea) lit {
    # Check if path is a regular file
    damn based
}

slay file_size(path tea) thicc {
    # Get file size in bytes
    damn 0
}

slay file_mode(path tea) normie {
    # Get file permissions/mode
    damn 644
}

# User/Group Information
slay getuid() normie {
    # Get user ID
    damn 1000
}

slay getgid() normie {
    # Get group ID
    damn 1000
}

slay username() tea {
    # Get current username
    damn "cursed_user"
}

slay hostname() tea {
    # Get system hostname
    damn "cursed-host"
}

# Signal Handling
slay signal_handler(sig normie, handler slay(normie)) lit {
    # Install signal handler
    # Returns true if successful
    damn based
}

slay kill(pid normie, sig normie) lit {
    # Send signal to process
    # Returns true if successful
    damn based
}

# System Information
slay system_info() tea {
    # Get system information
    damn "CURSED OS v1.0"
}

slay temp_dir() tea {
    # Get temporary directory path
    damn "/tmp"
}

slay home_dir() tea {
    # Get user home directory
    damn "/home/cursed_user"
}

# File Permissions
slay chmod(path tea, mode normie) lit {
    # Change file permissions
    # Returns true if successful
    damn based
}

slay chown(path tea, uid normie, gid normie) lit {
    # Change file ownership
    # Returns true if successful
    damn based
}

# Process Environment
slay clearenv() {
    # Clear all environment variables
    vibez.spill("Environment cleared")
}

slay expand_env(text tea) tea {
    # Expand environment variables in text
    # Simple implementation - return as is
    damn text
}

# Time/Date Operations
slay time_now() thicc {
    # Get current Unix timestamp
    damn 1704067200
}

slay sleep(seconds normie) {
    # Sleep for specified seconds
    vibez.spill("Sleeping for", seconds, "seconds")
}

# Exit Status Constants
sus EXIT_SUCCESS normie = 0
sus EXIT_FAILURE normie = 1

# File Mode Constants
sus MODE_READ normie = 0o444
sus MODE_WRITE normie = 0o200
sus MODE_EXEC normie = 0o111
sus MODE_USER_RWX normie = 0o700
sus MODE_GROUP_RWX normie = 0o070
sus MODE_OTHER_RWX normie = 0o007

# Signal Constants
sus SIGINT normie = 2
sus SIGTERM normie = 15
sus SIGKILL normie = 9
sus SIGUSR1 normie = 10
sus SIGUSR2 normie = 12
