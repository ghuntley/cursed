# exec_vibez - Pure CURSED Process Execution Module
# Enhanced external command execution and process management
# Zero FFI dependencies - completely self-contained

# Global process execution state
sus process_registry map = {}
sus running_processes map = {}
sus process_groups map = {}
sus command_history [map] = []
sus exec_statistics map = {}
sus exec_config map = {}
sus process_environments map = {}
sus output_streamers map = {}
sus timeout_manager map = {}

# Process state constants
sus PROCESS_STATE_CREATED tea = "created"
sus PROCESS_STATE_RUNNING tea = "running"  
sus PROCESS_STATE_FINISHED tea = "finished"
sus PROCESS_STATE_FAILED tea = "failed"
sus PROCESS_STATE_KILLED tea = "killed"
sus PROCESS_STATE_TIMEOUT tea = "timeout"

# Exit code constants
sus EXIT_SUCCESS normie = 0
sus EXIT_FAILURE normie = 1
sus EXIT_TIMEOUT normie = 124
sus EXIT_KILLED normie = 137

# Command execution modes
sus EXEC_MODE_SYNC tea = "synchronous"
sus EXEC_MODE_ASYNC tea = "asynchronous" 
sus EXEC_MODE_DETACHED tea = "detached"

# Stream types
sus STREAM_STDOUT tea = "stdout"
sus STREAM_STDERR tea = "stderr"
sus STREAM_STDIN tea = "stdin"

# Signal constants for process management
sus SIGTERM normie = 15
sus SIGINT normie = 2
sus SIGCHLD normie = 17

# ==============================================================================
# INITIALIZATION AND CONFIGURATION
# ==============================================================================

# Initialize exec_vibez module
slay init_exec_vibez() lit {
    exec_config = {
        "default_timeout_ms": 30000,        # 30 seconds
        "max_concurrent_processes": 100,
        "max_command_history": 1000,
        "default_working_dir": "/tmp",
        "capture_output": based,
        "shell_command": "/bin/sh",
        "shell_flag": "-c",
        "max_output_buffer": 1048576,       # 1MB
        "process_cleanup_interval": 5000    # 5 seconds
    }
    
    exec_statistics = {
        "commands_executed": 0,
        "processes_created": 0,
        "processes_running": 0,
        "processes_completed": 0,
        "processes_failed": 0,
        "processes_timeout": 0,
        "total_execution_time": 0,
        "avg_execution_time": 0
    }
    
    vibez.spill("exec_vibez module initialized")
    damn based
}

# Configure exec_vibez settings
slay configure_exec(config map) lit {
    sus keys [tea] = config.keys()
    sus i normie = 0
    
    while i < keys.length() {
        sus key tea = keys[i]
        if exec_config.has_key(key) {
            exec_config.set(key, config.get(key))
        }
        i = i + 1
    }
    
    damn based
}

# Get exec configuration
slay get_exec_config() map {
    damn exec_config
}

# ==============================================================================
# COMMAND CREATION AND EXECUTION
# ==============================================================================

# Create a command object
slay create_command(program tea, args [tea]) map {
    sus command_id tea = generate_command_id()
    sus command_data map = {
        "id": command_id,
        "program": program,
        "args": args,
        "working_dir": exec_config.get("default_working_dir"),
        "environment": {},
        "stdin_data": "",
        "capture_stdout": exec_config.get("capture_output"),
        "capture_stderr": exec_config.get("capture_output"),
        "timeout_ms": exec_config.get("default_timeout_ms"),
        "exec_mode": EXEC_MODE_SYNC,
        "created_at": get_current_timestamp(),
        "state": PROCESS_STATE_CREATED
    }
    
    damn command_data
}

# Set command working directory
slay set_working_dir(command map, dir tea) lit {
    command.set("working_dir", dir)
    damn based
}

# Set command environment variables
slay set_environment(command map, env map) lit {
    command.set("environment", env)
    damn based
}

# Add environment variable
slay add_env_var(command map, key tea, value tea) lit {
    sus env map = command.get("environment")
    env.set(key, value)
    command.set("environment", env)
    damn based
}

# Set command input data
slay set_stdin(command map, input tea) lit {
    command.set("stdin_data", input)
    damn based
}

# Set command timeout
slay set_timeout(command map, timeout_ms normie) lit {
    command.set("timeout_ms", timeout_ms)
    damn based
}

# Set execution mode
slay set_exec_mode(command map, mode tea) lit {
    if mode == EXEC_MODE_SYNC || mode == EXEC_MODE_ASYNC || mode == EXEC_MODE_DETACHED {
        command.set("exec_mode", mode)
        damn based
    }
    damn cap
}

# ==============================================================================
# COMMAND EXECUTION
# ==============================================================================

# Execute command synchronously
slay execute_command(command map) map {
    sus process_id tea = generate_process_id()
    sus execution_start normie = get_current_timestamp()
    
    # Update command state
    command.set("state", PROCESS_STATE_RUNNING)
    command.set("process_id", process_id)
    command.set("start_time", execution_start)
    
    # Register process
    register_process(process_id, command)
    
    # Simulate command execution
    sus result map = simulate_command_execution(command)
    
    # Update statistics
    update_execution_statistics(result)
    
    # Add to command history
    add_to_command_history(command, result)
    
    # Cleanup process
    unregister_process(process_id)
    
    damn result
}

# Execute command asynchronously
slay execute_async(command map) tea {
    sus process_id tea = generate_process_id()
    
    # Update command state
    command.set("state", PROCESS_STATE_RUNNING)
    command.set("process_id", process_id)
    command.set("start_time", get_current_timestamp())
    command.set("exec_mode", EXEC_MODE_ASYNC)
    
    # Register process
    register_process(process_id, command)
    
    # Store for async monitoring
    running_processes.set(process_id, command)
    
    # Simulate async start
    vibez.spill("Started async process: " + process_id)
    
    damn process_id
}

# Wait for async process completion
slay wait_for_process(process_id tea) map {
    if !running_processes.has_key(process_id) {
        damn { "error": "Process not found", "exit_code": EXIT_FAILURE }
    }
    
    sus command map = running_processes.get(process_id)
    
    # Simulate waiting for completion
    sus result map = simulate_command_execution(command)
    
    # Cleanup
    running_processes.remove(process_id)
    unregister_process(process_id)
    
    # Update statistics
    update_execution_statistics(result)
    
    damn result
}

# Kill a running process
slay kill_process(process_id tea) lit {
    if !running_processes.has_key(process_id) {
        damn cap
    }
    
    sus command map = running_processes.get(process_id)
    command.set("state", PROCESS_STATE_KILLED)
    command.set("end_time", get_current_timestamp())
    
    # Create kill result
    sus result map = {
        "exit_code": EXIT_KILLED,
        "stdout": "",
        "stderr": "Process killed",
        "execution_time_ms": get_current_timestamp() - command.get("start_time"),
        "state": PROCESS_STATE_KILLED
    }
    
    # Update statistics
    update_execution_statistics(result)
    
    # Cleanup
    running_processes.remove(process_id)
    unregister_process(process_id)
    
    vibez.spill("Killed process: " + process_id)
    damn based
}

# ==============================================================================
# COMMAND SIMULATION (Pure CURSED Implementation)
# ==============================================================================

# Simulate command execution without FFI
slay simulate_command_execution(command map) map {
    sus program tea = command.get("program")
    sus args [tea] = command.get("args")
    sus timeout_ms normie = command.get("timeout_ms")
    sus start_time normie = get_current_timestamp()
    
    # Simulate execution time
    sus execution_time normie = simulate_execution_time(program, args)
    
    # Check timeout
    if execution_time > timeout_ms {
        command.set("state", PROCESS_STATE_TIMEOUT)
        damn {
            "exit_code": EXIT_TIMEOUT,
            "stdout": "",
            "stderr": "Command timed out after " + core.tea(timeout_ms) + "ms",
            "execution_time_ms": timeout_ms,
            "state": PROCESS_STATE_TIMEOUT
        }
    }
    
    # Simulate command output based on program
    sus output_result map = simulate_command_output(program, args)
    sus exit_code normie = output_result.get("exit_code")
    sus stdout tea = output_result.get("stdout")
    sus stderr tea = output_result.get("stderr")
    
    # Determine final state
    sus final_state tea = PROCESS_STATE_FINISHED
    if exit_code != EXIT_SUCCESS {
        final_state = PROCESS_STATE_FAILED
    }
    
    command.set("state", final_state)
    command.set("end_time", get_current_timestamp())
    
    # Create execution result
    sus result map = {
        "exit_code": exit_code,
        "stdout": stdout,
        "stderr": stderr,
        "execution_time_ms": execution_time,
        "state": final_state,
        "command": command
    }
    
    damn result
}

# Simulate execution time based on command
slay simulate_execution_time(program tea, args [tea]) normie {
    # Simulate different execution times for different commands
    if program == "echo" {
        damn 50  # 50ms for echo
    } else if program == "ls" {
        damn 100  # 100ms for ls
    } else if program == "grep" {
        damn 200  # 200ms for grep
    } else if program == "find" {
        damn 500  # 500ms for find
    } else if program == "sleep" {
        if args.length() > 0 {
            sus sleep_time tea = args[0]
            damn core.normie(sleep_time) * 1000  # Convert seconds to ms
        }
        damn 1000  # Default 1 second
    } else {
        damn 300  # Default 300ms for unknown commands
    }
}

# Simulate command output
slay simulate_command_output(program tea, args [tea]) map {
    if program == "echo" {
        sus output tea = ""
        sus i normie = 0
        while i < args.length() {
            if i > 0 {
                output = output + " "
            }
            output = output + args[i]
            i = i + 1
        }
        damn { "exit_code": EXIT_SUCCESS, "stdout": output, "stderr": "" }
    }
    
    if program == "ls" {
        damn { 
            "exit_code": EXIT_SUCCESS, 
            "stdout": "file1.txt\nfile2.txt\ndirectory1\n", 
            "stderr": "" 
        }
    }
    
    if program == "pwd" {
        damn { 
            "exit_code": EXIT_SUCCESS, 
            "stdout": "/current/directory", 
            "stderr": "" 
        }
    }
    
    if program == "whoami" {
        damn { 
            "exit_code": EXIT_SUCCESS, 
            "stdout": "cursed_user", 
            "stderr": "" 
        }
    }
    
    if program == "date" {
        damn { 
            "exit_code": EXIT_SUCCESS, 
            "stdout": "Mon Jan  1 12:00:00 UTC 2024", 
            "stderr": "" 
        }
    }
    
    if program == "cat" {
        if args.length() > 0 {
            damn { 
                "exit_code": EXIT_SUCCESS, 
                "stdout": "Contents of " + args[0], 
                "stderr": "" 
            }
        } else {
            damn { 
                "exit_code": EXIT_FAILURE, 
                "stdout": "", 
                "stderr": "cat: missing filename" 
            }
        }
    }
    
    if program == "false" {
        damn { "exit_code": EXIT_FAILURE, "stdout": "", "stderr": "" }
    }
    
    if program == "true" {
        damn { "exit_code": EXIT_SUCCESS, "stdout": "", "stderr": "" }
    }
    
    # Default simulation for unknown commands
    damn { 
        "exit_code": EXIT_SUCCESS, 
        "stdout": "Simulated output for " + program, 
        "stderr": "" 
    }
}

# ==============================================================================
# PROCESS MANAGEMENT
# ==============================================================================

# Register process
slay register_process(process_id tea, command map) lit {
    sus process_data map = {
        "id": process_id,
        "command": command,
        "registered_at": get_current_timestamp(),
        "state": command.get("state"),
        "parent_group": "",
        "child_processes": []
    }
    
    process_registry.set(process_id, process_data)
    
    sus stats normie = exec_statistics.get("processes_created")
    exec_statistics.set("processes_created", stats + 1)
    
    sus running normie = exec_statistics.get("processes_running")
    exec_statistics.set("processes_running", running + 1)
    
    damn based
}

# Unregister process
slay unregister_process(process_id tea) lit {
    if process_registry.has_key(process_id) {
        process_registry.remove(process_id)
        
        sus running normie = exec_statistics.get("processes_running")
        if running > 0 {
            exec_statistics.set("processes_running", running - 1)
        }
        
        damn based
    }
    damn cap
}

# Get process info
slay get_process_info(process_id tea) map {
    if process_registry.has_key(process_id) {
        damn process_registry.get(process_id)
    }
    damn {}
}

# List running processes
slay list_running_processes() [tea] {
    damn running_processes.keys()
}

# List all registered processes
slay list_all_processes() [tea] {
    damn process_registry.keys()
}

# ==============================================================================
# PROCESS GROUPS
# ==============================================================================

# Create process group
slay create_process_group(group_name tea) lit {
    if process_groups.has_key(group_name) {
        damn cap  # Group already exists
    }
    
    sus group_data map = {
        "name": group_name,
        "created_at": get_current_timestamp(),
        "processes": [],
        "leader_process": "",
        "group_state": "active"
    }
    
    process_groups.set(group_name, group_data)
    vibez.spill("Created process group: " + group_name)
    damn based
}

# Add process to group
slay add_to_group(group_name tea, process_id tea) lit {
    if !process_groups.has_key(group_name) {
        damn cap  # Group doesn't exist
    }
    
    sus group_data map = process_groups.get(group_name)
    sus processes [tea] = group_data.get("processes")
    processes.push(process_id)
    group_data.set("processes", processes)
    
    # Set leader if first process
    if processes.length() == 1 {
        group_data.set("leader_process", process_id)
    }
    
    process_groups.set(group_name, group_data)
    damn based
}

# Kill process group
slay kill_process_group(group_name tea) lit {
    if !process_groups.has_key(group_name) {
        damn cap
    }
    
    sus group_data map = process_groups.get(group_name)
    sus processes [tea] = group_data.get("processes")
    sus i normie = 0
    
    while i < processes.length() {
        sus process_id tea = processes[i]
        kill_process(process_id)
        i = i + 1
    }
    
    group_data.set("group_state", "terminated")
    process_groups.set(group_name, group_data)
    
    vibez.spill("Killed process group: " + group_name)
    damn based
}

# ==============================================================================
# OUTPUT STREAMING AND CAPTURE
# ==============================================================================

# Create output streamer
slay create_output_streamer(process_id tea, stream_type tea) lit {
    sus streamer_data map = {
        "process_id": process_id,
        "stream_type": stream_type,
        "buffer": [],
        "max_buffer_size": exec_config.get("max_output_buffer"),
        "bytes_captured": 0,
        "is_active": based
    }
    
    sus streamer_key tea = process_id + "_" + stream_type
    output_streamers.set(streamer_key, streamer_data)
    damn based
}

# Capture stream output
slay capture_stream_output(process_id tea, stream_type tea, data tea) lit {
    sus streamer_key tea = process_id + "_" + stream_type
    
    if !output_streamers.has_key(streamer_key) {
        damn cap
    }
    
    sus streamer_data map = output_streamers.get(streamer_key)
    sus buffer [tea] = streamer_data.get("buffer")
    sus max_size normie = streamer_data.get("max_buffer_size")
    
    if streamer_data.get("bytes_captured") + data.length() > max_size {
        # Buffer overflow - truncate
        vibez.spill("Output buffer overflow for process " + process_id)
        damn cap
    }
    
    buffer.push(data)
    streamer_data.set("buffer", buffer)
    
    sus bytes normie = streamer_data.get("bytes_captured")
    streamer_data.set("bytes_captured", bytes + data.length())
    
    output_streamers.set(streamer_key, streamer_data)
    damn based
}

# Get captured output
slay get_captured_output(process_id tea, stream_type tea) tea {
    sus streamer_key tea = process_id + "_" + stream_type
    
    if !output_streamers.has_key(streamer_key) {
        damn ""
    }
    
    sus streamer_data map = output_streamers.get(streamer_key)
    sus buffer [tea] = streamer_data.get("buffer")
    sus output tea = ""
    sus i normie = 0
    
    while i < buffer.length() {
        output = output + buffer[i]
        i = i + 1
    }
    
    damn output
}

# ==============================================================================
# UTILITY FUNCTIONS
# ==============================================================================

# Generate unique command ID
slay generate_command_id() tea {
    sus timestamp normie = get_current_timestamp()
    sus random normie = timestamp % 10000
    damn "cmd_" + core.tea(timestamp) + "_" + core.tea(random)
}

# Generate unique process ID
slay generate_process_id() tea {
    sus timestamp normie = get_current_timestamp()
    sus random normie = (timestamp * 31) % 100000
    damn "proc_" + core.tea(timestamp) + "_" + core.tea(random)
}

# Get current timestamp (simulated)
slay get_current_timestamp() normie {
    damn 1704067200  # Simulated timestamp
}

# Add command to history
slay add_to_command_history(command map, result map) {
    sus history_entry map = {
        "command": command,
        "result": result,
        "executed_at": get_current_timestamp()
    }
    
    command_history.push(history_entry)
    
    # Limit history size
    sus max_history normie = exec_config.get("max_command_history")
    if command_history.length() > max_history {
        command_history.remove(0)
    }
}

# Update execution statistics
slay update_execution_statistics(result map) {
    sus executed normie = exec_statistics.get("commands_executed")
    exec_statistics.set("commands_executed", executed + 1)
    
    sus state tea = result.get("state")
    if state == PROCESS_STATE_FINISHED {
        sus completed normie = exec_statistics.get("processes_completed")
        exec_statistics.set("processes_completed", completed + 1)
    } else if state == PROCESS_STATE_FAILED {
        sus failed normie = exec_statistics.get("processes_failed")
        exec_statistics.set("processes_failed", failed + 1)
    } else if state == PROCESS_STATE_TIMEOUT {
        sus timeout normie = exec_statistics.get("processes_timeout")
        exec_statistics.set("processes_timeout", timeout + 1)
    }
    
    # Update timing statistics
    sus exec_time normie = result.get("execution_time_ms")
    sus total_time normie = exec_statistics.get("total_execution_time")
    exec_statistics.set("total_execution_time", total_time + exec_time)
    
    sus executed_count normie = exec_statistics.get("commands_executed")
    if executed_count > 0 {
        sus avg_time normie = total_time / executed_count
        exec_statistics.set("avg_execution_time", avg_time)
    }
}

# ==============================================================================
# HIGH-LEVEL CONVENIENCE FUNCTIONS
# ==============================================================================

# Simple command execution
slay run_command(program tea, args [tea]) map {
    sus command map = create_command(program, args)
    damn execute_command(command)
}

# Run command with output capture
slay run_with_output(program tea, args [tea]) map {
    sus command map = create_command(program, args)
    command.set("capture_stdout", based)
    command.set("capture_stderr", based)
    damn execute_command(command)
}

# Run command with timeout
slay run_with_timeout(program tea, args [tea], timeout_ms normie) map {
    sus command map = create_command(program, args)
    set_timeout(command, timeout_ms)
    damn execute_command(command)
}

# Run command in background
slay run_background(program tea, args [tea]) tea {
    sus command map = create_command(program, args)
    damn execute_async(command)
}

# Run shell command
slay run_shell(command_str tea) map {
    sus shell tea = exec_config.get("shell_command")
    sus flag tea = exec_config.get("shell_flag")
    sus args [tea] = [flag, command_str]
    damn run_command(shell, args)
}

# ==============================================================================
# STATISTICS AND MONITORING
# ==============================================================================

# Get execution statistics
slay get_exec_statistics() map {
    damn exec_statistics
}

# Get command history
slay get_command_history() [map] {
    damn command_history
}

# Get recent commands
slay get_recent_commands(count normie) [map] {
    sus recent [map] = []
    sus start normie = 0
    sus history_length normie = command_history.length()
    
    if history_length > count {
        start = history_length - count
    }
    
    sus i normie = start
    while i < history_length {
        recent.push(command_history[i])
        i = i + 1
    }
    
    damn recent
}

# Dump execution state
slay dump_exec_state() {
    vibez.spill("=== Exec Vibez State ===")
    vibez.spill("Running processes: " + core.tea(running_processes.size()))
    vibez.spill("Registered processes: " + core.tea(process_registry.size()))
    vibez.spill("Process groups: " + core.tea(process_groups.size()))
    vibez.spill("Commands executed: " + core.tea(exec_statistics.get("commands_executed")))
    vibez.spill("Average execution time: " + core.tea(exec_statistics.get("avg_execution_time")) + "ms")
    
    sus running [tea] = list_running_processes()
    if running.length() > 0 {
        vibez.spill("\nRunning processes:")
        sus i normie = 0
        while i < running.length() {
            sus process_id tea = running[i]
            sus process_info map = get_process_info(process_id)
            sus command map = process_info.get("command")
            vibez.spill("  " + process_id + ": " + command.get("program"))
            i = i + 1
        }
    }
}

# Reset exec_vibez state
slay reset_exec() {
    process_registry = {}
    running_processes = {}
    process_groups = {}
    command_history = []
    process_environments = {}
    output_streamers = {}
    timeout_manager = {}
    
    # Reset statistics
    exec_statistics = {
        "commands_executed": 0,
        "processes_created": 0,
        "processes_running": 0,
        "processes_completed": 0,
        "processes_failed": 0,
        "processes_timeout": 0,
        "total_execution_time": 0,
        "avg_execution_time": 0
    }
    
    vibez.spill("exec_vibez state reset")
}

# Get module information
slay get_module_info() tea {
    damn "exec_vibez v1.0 - Pure CURSED process execution and management system"
}
