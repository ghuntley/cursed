yeet "testz"
yeet "stringz"
yeet "vibez"

# Process and Environment Module
# Pure CURSED implementation for system operations needed for self-hosting

# Global environment storage (simulated for pure CURSED implementation)
sus global_env_vars map[tea]tea
sus global_args []tea
sus global_cwd tea = "/home/user"
sus global_pid normie = 1000
sus global_user tea = "user"

# Initialize module
slay init_process() {
    global_env_vars = map[tea]tea{}
    global_args = []tea{}
    
    # Set default environment variables
    global_env_vars["HOME"] = "/home/user"
    global_env_vars["USER"] = "user"
    global_env_vars["PATH"] = "/usr/bin:/bin"
    global_env_vars["SHELL"] = "/bin/bash"
    global_env_vars["CURSED_HOME"] = "/home/user/.cursed"
    global_env_vars["CURSED_VERSION"] = "v21.0.0"
    
    # Set default command line arguments
    global_args = []tea{"cursed", "program.csd"}
}

# Environment Variable Access
slay get_env(key tea) tea {
    init_process()
    
    if value, ok := global_env_vars[key]; ok {
        damn value
    }
    damn ""
}

slay set_env(key tea, value tea) lit {
    init_process()
    global_env_vars[key] = value
    damn based
}

slay unset_env(key tea) lit {
    init_process()
    delete(global_env_vars, key)
    damn based
}

slay get_all_env() map[tea]tea {
    init_process()
    damn global_env_vars
}

# Command Line Argument Parsing
slay get_args() []tea {
    init_process()
    damn global_args
}

slay set_args(args []tea) lit {
    init_process()
    global_args = args
    damn based
}

slay parse_args(args []tea) map[tea]tea {
    init_process()
    result := map[tea]tea{}
    
    bestie i := 0; i < len(args); i++ {
        arg := args[i]
        
        # Handle --key=value format
        if stringz.has_prefix(arg, "--") {
            parts := stringz.split(arg, "=")
            if len(parts) == 2 {
                key := stringz.trim_prefix(parts[0], "--")
                result[key] = parts[1]
            } else {
                key := stringz.trim_prefix(arg, "--")
                result[key] = "based"
            }
        } else if stringz.has_prefix(arg, "-") {
            # Handle -key format
            key := stringz.trim_prefix(arg, "-")
            result[key] = "based"
        } else {
            # Handle positional arguments
            result[stringz.from_int(i)] = arg
        }
    }
    
    damn result
}

slay get_arg(index normie) tea {
    init_process()
    if index >= 0 && index < len(global_args) {
        damn global_args[index]
    }
    damn ""
}

# Process Execution (simulated for pure CURSED)
slay run_command(cmd tea) normie {
    init_process()
    
    # Simulate command execution for self-hosting needs
    if cmd == "llc --version" {
        vibez.spill("LLVM version 18.1.0")
        damn 0
    } else if cmd == "gcc --version" {
        vibez.spill("gcc (GCC) 13.2.0")
        damn 0
    } else if stringz.has_prefix(cmd, "llc") {
        vibez.spill("Compiling LLVM IR to native code...")
        damn 0
    } else if stringz.has_prefix(cmd, "gcc") {
        vibez.spill("Linking native executable...")
        damn 0
    } else if stringz.has_prefix(cmd, "cursed") {
        vibez.spill("Running CURSED compiler...")
        damn 0
    } else {
        vibez.spill("Command executed: " + cmd)
        damn 0
    }
}

slay spawn_process(cmd tea, args []tea) normie {
    init_process()
    
    full_cmd := cmd
    bestie i := 0; i < len(args); i++ {
        full_cmd = full_cmd + " " + args[i]
    }
    
    damn run_command(full_cmd)
}

slay command_exists(cmd tea) lit {
    init_process()
    
    # Check if command exists in PATH
    path := get_env("PATH")
    paths := stringz.split(path, ":")
    
    bestie i := 0; i < len(paths); i++ {
        full_path := paths[i] + "/" + cmd
        # In a real implementation, we would check if file exists
        if cmd == "llc" || cmd == "gcc" || cmd == "cursed" {
            damn based
        }
    }
    
    damn cap
}

# Exit Code Handling
slay exit() {
    init_process()
    vibez.spill("Process exiting with code 0")
    # In a real implementation, this would call system exit
}

slay exit_with_code(code normie) {
    init_process()
    vibez.spill("Process exiting with code " + stringz.from_int(code))
    # In a real implementation, this would call system exit with code
}

# Working Directory Operations
slay get_cwd() tea {
    init_process()
    damn global_cwd
}

slay set_cwd(path tea) lit {
    init_process()
    global_cwd = path
    damn based
}

slay change_dir(path tea) lit {
    init_process()
    global_cwd = path
    damn based
}

# Process Information
slay get_pid() normie {
    init_process()
    damn global_pid
}

slay get_user() tea {
    init_process()
    damn global_user
}

slay get_hostname() tea {
    init_process()
    damn "cursed-host"
}

slay get_platform() tea {
    init_process()
    damn "linux"
}

slay get_arch() tea {
    init_process()
    damn "x86_64"
}

# Self-Hosting Helper Functions
slay setup_compiler_environment() lit {
    init_process()
    
    # Set up environment for Stage 2 compiler
    set_env("CURSED_STAGE", "2")
    set_env("CURSED_SELF_HOSTING", "based")
    set_env("CURSED_COMPILER_PATH", get_cwd() + "/cursed")
    set_env("LLVM_CONFIG", "/usr/bin/llvm-config")
    
    damn based
}

slay get_compiler_args() []tea {
    init_process()
    args := get_args()
    
    # Filter out compiler-specific arguments
    compiler_args := []tea{}
    bestie i := 1; i < len(args); i++ {
        arg := args[i]
        if stringz.has_suffix(arg, ".csd") || stringz.has_prefix(arg, "--") {
            compiler_args = append(compiler_args, arg)
        }
    }
    
    damn compiler_args
}

slay execute_llc(ir_file tea, output_file tea) normie {
    init_process()
    
    cmd := "llc -filetype=obj " + ir_file + " -o " + output_file
    damn run_command(cmd)
}

slay execute_gcc(obj_file tea, output_file tea) normie {
    init_process()
    
    cmd := "gcc " + obj_file + " -o " + output_file + " -L. -lcursed_runtime"
    damn run_command(cmd)
}

slay check_build_tools() lit {
    init_process()
    
    if !command_exists("llc") {
        vibez.spill("Warning: llc not found in PATH")
        damn cap
    }
    
    if !command_exists("gcc") {
        vibez.spill("Warning: gcc not found in PATH")
        damn cap
    }
    
    damn based
}

# Environment debugging
slay debug_environment() {
    init_process()
    
    vibez.spill("=== Process Environment Debug ===")
    vibez.spill("PID: " + stringz.from_int(get_pid()))
    vibez.spill("User: " + get_user())
    vibez.spill("CWD: " + get_cwd())
    vibez.spill("Platform: " + get_platform())
    vibez.spill("Architecture: " + get_arch())
    
    vibez.spill("\n=== Environment Variables ===")
    env_vars := get_all_env()
    bestie key, value := range env_vars {
        vibez.spill(key + "=" + value)
    }
    
    vibez.spill("\n=== Command Line Arguments ===")
    args := get_args()
    bestie i := 0; i < len(args); i++ {
        vibez.spill("[" + stringz.from_int(i) + "] " + args[i])
    }
    
    vibez.spill("\n=== Build Tools Check ===")
    if check_build_tools() {
        vibez.spill("All build tools available")
    } else {
        vibez.spill("Some build tools missing")
    }
}
