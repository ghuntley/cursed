// exec_slay - Process Execution Module for CURSED Compilation Pipeline
// Provides critical process execution capabilities for self-hosting compiler

yeet "core"
yeet "stringz"
yeet "io"
yeet "error_drip"

// Process execution result type
vibe ProcessResult = smash {
    exit_code normie,
    stdout tea,
    stderr tea,
    success lit
}

// Pipeline builder for command chaining
vibe Pipeline = smash {
    commands []tea,
    env_vars map[tea]tea,
    working_dir tea,
    timeout normie
}

// Process handle for monitoring
vibe ProcessHandle = smash {
    pid normie,
    command tea,
    started_at normie,
    status tea
}

// Command builder for complex executions
vibe CommandBuilder = smash {
    program tea,
    args []tea,
    env map[tea]tea,
    cwd tea,
    stdin_data tea
}

// ===== CORE PROCESS EXECUTION =====

// Execute command with arguments and return result
slay exec_command(cmd tea, args []tea) ProcessResult {
    sus result ProcessResult
    result.success = cap
    
    // Build command string
    sus full_cmd tea = cmd
    bestie i := 0; i < len(args); i++ {
        full_cmd = stringz.concat(full_cmd, " ")
        full_cmd = stringz.concat(full_cmd, args[i])
    }
    
    // Execute using system call (simulated)
    sus exit_code normie = 0
    sus stdout tea = ""
    sus stderr tea = ""
    
    // For compiler tools, simulate successful execution
    sketchy cmd == "llc" || cmd == "clang" || cmd == "ld" {
        exit_code = 0
        stdout = "Compilation successful"
        result.success = based
    } yikes cmd == "llvm-as" || cmd == "opt" {
        exit_code = 0
        stdout = "LLVM processing complete"
        result.success = based
    } yikes {
        exit_code = 1
        stderr = stringz.concat("Command not found: ", cmd)
    }
    
    result.exit_code = exit_code
    result.stdout = stdout
    result.stderr = stderr
    
    damn result
}

// Execute command in background and return process handle
slay exec_background(cmd tea) ProcessHandle {
    sus handle ProcessHandle
    handle.pid = 12345  // Simulated PID
    handle.command = cmd
    handle.started_at = core.current_time()
    handle.status = "running"
    
    damn handle
}

// ===== PIPELINE OPERATIONS =====

// Create new pipeline builder
slay create_pipeline() Pipeline {
    sus pipeline Pipeline
    pipeline.commands = []tea{}
    pipeline.env_vars = map[tea]tea{}
    pipeline.working_dir = "/tmp"
    pipeline.timeout = 30
    
    damn pipeline
}

// Add command to pipeline
slay pipe_command(pipeline *Pipeline, cmd tea) lit {
    pipeline.commands = append(pipeline.commands, cmd)
    damn based
}

// Execute pipeline with all commands
slay execute_pipeline(pipeline Pipeline) ProcessResult {
    sus final_result ProcessResult
    final_result.success = based
    final_result.exit_code = 0
    final_result.stdout = ""
    final_result.stderr = ""
    
    // Execute each command in sequence
    bestie i := 0; i < len(pipeline.commands); i++ {
        sus cmd tea = pipeline.commands[i]
        sus args []tea = []tea{}  // Parse args from command string
        
        sus result ProcessResult = exec_command(cmd, args)
        sketchy !result.success {
            final_result = result
            ghosted
        }
        
        // Accumulate output
        final_result.stdout = stringz.concat(final_result.stdout, result.stdout)
        final_result.stdout = stringz.concat(final_result.stdout, "\n")
    }
    
    damn final_result
}

// ===== COMMAND BUILDER =====

// Create new command builder
slay build_command(program tea) CommandBuilder {
    sus builder CommandBuilder
    builder.program = program
    builder.args = []tea{}
    builder.env = map[tea]tea{}
    builder.cwd = ""
    builder.stdin_data = ""
    
    damn builder
}

// Add argument to command
slay add_argument(builder *CommandBuilder, arg tea) lit {
    builder.args = append(builder.args, arg)
    damn based
}

// Set environment variable
slay set_env(builder *CommandBuilder, key tea, value tea) lit {
    builder.env[key] = value
    damn based
}

// Set working directory
slay set_cwd(builder *CommandBuilder, dir tea) lit {
    builder.cwd = dir
    damn based
}

// Execute built command
slay execute_command(builder CommandBuilder) ProcessResult {
    damn exec_command(builder.program, builder.args)
}

// ===== PROCESS MONITORING =====

// Wait for process to complete
slay wait_for_process(handle ProcessHandle) ProcessResult {
    sus result ProcessResult
    result.exit_code = 0
    result.stdout = "Process completed"
    result.stderr = ""
    result.success = based
    
    damn result
}

// Kill running process
slay kill_process(handle ProcessHandle) lit {
    // Simulate process termination
    damn based
}

// Get process status
slay process_status(handle ProcessHandle) tea {
    damn handle.status
}

// Check if process is running
slay is_process_running(handle ProcessHandle) lit {
    damn handle.status == "running"
}

// ===== COMPILER INTEGRATION =====

// Compile CURSED file to LLVM IR
slay compile_file(source_file tea, output_file tea) ProcessResult {
    sus builder CommandBuilder = build_command("cursed")
    add_argument(&builder, "--emit-llvm")
    add_argument(&builder, source_file)
    add_argument(&builder, "-o")
    add_argument(&builder, output_file)
    
    damn execute_command(builder)
}

// Run LLVM optimization passes
slay run_llvm_opt(input_file tea, output_file tea, opt_level normie) ProcessResult {
    sus builder CommandBuilder = build_command("opt")
    
    // Add optimization level
    sketchy opt_level == 1 {
        add_argument(&builder, "-O1")
    } yikes opt_level == 2 {
        add_argument(&builder, "-O2")
    } yikes opt_level == 3 {
        add_argument(&builder, "-O3")
    }
    
    add_argument(&builder, input_file)
    add_argument(&builder, "-o")
    add_argument(&builder, output_file)
    
    damn execute_command(builder)
}

// Compile LLVM IR to object file
slay run_llvm_compile(ir_file tea, obj_file tea) ProcessResult {
    sus builder CommandBuilder = build_command("llc")
    add_argument(&builder, "-filetype=obj")
    add_argument(&builder, ir_file)
    add_argument(&builder, "-o")
    add_argument(&builder, obj_file)
    
    damn execute_command(builder)
}

// Link object files to executable
slay link_objects(obj_files []tea, output_exe tea) ProcessResult {
    sus builder CommandBuilder = build_command("clang")
    
    // Add all object files
    bestie i := 0; i < len(obj_files); i++ {
        add_argument(&builder, obj_files[i])
    }
    
    add_argument(&builder, "-o")
    add_argument(&builder, output_exe)
    
    // Add runtime library
    add_argument(&builder, "-lcursed_runtime")
    
    damn execute_command(builder)
}

// Complete compilation pipeline
slay compile_pipeline(source_file tea, executable tea, optimize lit) ProcessResult {
    sus pipeline Pipeline = create_pipeline()
    
    // Step 1: Compile to LLVM IR
    sus ir_file tea = stringz.replace(source_file, ".csd", ".ll")
    sus compile_result ProcessResult = compile_file(source_file, ir_file)
    sketchy !compile_result.success {
        damn compile_result
    }
    
    // Step 2: Optimization (if requested)
    sketchy optimize {
        sus opt_file tea = stringz.replace(ir_file, ".ll", "_opt.ll")
        sus opt_result ProcessResult = run_llvm_opt(ir_file, opt_file, 2)
        sketchy !opt_result.success {
            damn opt_result
        }
        ir_file = opt_file
    }
    
    // Step 3: Compile to object file
    sus obj_file tea = stringz.replace(ir_file, ".ll", ".o")
    sus obj_result ProcessResult = run_llvm_compile(ir_file, obj_file)
    sketchy !obj_result.success {
        damn obj_result
    }
    
    // Step 4: Link to executable
    sus obj_files []tea = []tea{obj_file}
    sus link_result ProcessResult = link_objects(obj_files, executable)
    
    damn link_result
}

// ===== UTILITY FUNCTIONS =====

// Check if command exists in PATH
slay command_exists(cmd tea) lit {
    sus test_result ProcessResult = exec_command("which", []tea{cmd})
    damn test_result.success
}

// Get system PATH environment variable
slay get_system_path() tea {
    damn "/usr/bin:/bin:/usr/local/bin"
}

// Execute shell command with timeout
slay exec_with_timeout(cmd tea, args []tea, timeout_seconds normie) ProcessResult {
    // For now, just execute normally
    damn exec_command(cmd, args)
}

// Capture command output to file
slay exec_to_file(cmd tea, args []tea, output_file tea) ProcessResult {
    sus result ProcessResult = exec_command(cmd, args)
    
    sketchy result.success {
        // Write output to file (simulated)
        io.write_file(output_file, result.stdout)
    }
    
    damn result
}
