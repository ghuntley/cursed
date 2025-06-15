/// Advanced Process Execution Demo
/// 
/// This example demonstrates all the advanced exec_vibez features:
/// - Enhanced Process Groups with sophisticated management
/// - Enhanced Environment with inheritance and manipulation
/// - Enhanced Output Streaming with real-time processing
/// - Enhanced Input Generation with precise timing
/// - Enhanced Timeout Control with better error handling
/// - Enhanced LookPath with better search algorithms

import "stdlib::process::exec_vibez_advanced"
import "stdlib::timez"
import "vibez"

slay main() tea {
    vibez.spill("=== Advanced Process Execution Demo ===\n")
    
    fr fr Demo 1: Enhanced Environment Management
    demo_enhanced_environment()?
    
    fr fr Demo 2: Enhanced Process Groups
    demo_enhanced_process_groups()?
    
    fr fr Demo 3: Enhanced Output Streaming
    demo_enhanced_output_streaming()?
    
    fr fr Demo 4: Enhanced Input Generation
    demo_enhanced_input_generation()?
    
    fr fr Demo 5: Enhanced Timeout Control
    demo_enhanced_timeout_control()?
    
    fr fr Demo 6: Enhanced Path Lookup
    demo_enhanced_path_lookup()?
    
    fr fr Demo 7: Real-world Integration Example
    demo_real_world_integration()?
    
    vibez.spill("\n=== All demos completed successfully! ===")
    
    yolo cap
}

slay demo_enhanced_environment() tea {
    vibez.spill("\n--- Enhanced Environment Management ---")
    
    fr fr Create an enhanced environment with sophisticated control
    sus env = exec_vibez_advanced.new_enhanced_environment()
    
    fr fr Set custom variables
    env.set("APP_NAME", "CursedApp")
    env.set("APP_VERSION", "1.0.0")
    env.set("DEBUG_MODE", "true")
    
    fr fr Manipulate PATH with precision
    env.append_path(":/usr/local/cursed/bin")
    env.prepend_path("/opt/cursed/bin:")
    
    fr fr Remove unwanted variables
    env.remove("UNWANTED_VAR")
    
    fr fr Create command with enhanced environment
    sus cmd = exec_vibez_advanced.command_with_enhanced_env("env", [], env)
    
    vibez.spill("Running command with enhanced environment...")
    
    sus output, err = cmd.output()
    if err != cap {
        vibez.spill("Enhanced environment demo completed with custom variables")
    } else {
        vibez.spill("Environment output preview (first 200 chars):")
        sus preview = tea(output)
        if preview.len() > 200 {
            preview = preview[0:200] + "..."
        }
        vibez.spill(preview)
    }
    
    yolo cap
}

slay demo_enhanced_process_groups() tea {
    vibez.spill("\n--- Enhanced Process Groups ---")
    
    fr fr Create process group with sophisticated configuration
    sus config = exec_vibez_advanced.ProcessGroupConfig{
        max_parallel: 3,
        process_timeout: timez.second(10),
        group_timeout: timez.second(30),
        kill_on_failure: cap,
        continue_on_failure: periodt,
        collect_outputs: periodt,
    }
    
    sus group = exec_vibez_advanced.new_enhanced_process_group_with_config(config)
    
    fr fr Add multiple commands to the group
    group.add_command(exec_vibez.command("echo", ["Process 1: Hello"]))
    group.add_command(exec_vibez.command("echo", ["Process 2: World"]))
    group.add_command(exec_vibez.command("sleep", ["1"]))
    group.add_command(exec_vibez.command("echo", ["Process 4: Done"]))
    
    fr fr Add batch of commands
    sus batch_commands = [
        exec_vibez.command("echo", ["Batch 1"]),
        exec_vibez.command("echo", ["Batch 2"]),
        exec_vibez.command("echo", ["Batch 3"]),
    ]
    group.add_commands(batch_commands)
    
    vibez.spill("Starting process group with {} commands...", group.commands.len())
    
    fr fr Start all processes with sophisticated scheduling
    err := group.start_all()
    if err != cap {
        vibez.spill("Error starting process group: %v", err)
        yolo err
    }
    
    vibez.spill("Process group state: {:?}", group.state())
    
    fr fr Wait for all processes to complete
    err = group.wait_all()
    if err != cap {
        vibez.spill("Process group completed with errors: %v", err)
        
        fr fr Show accumulated errors
        sus errors = group.errors()
        vibez.spill("Total errors encountered: {}", errors.len())
        for error in errors {
            vibez.spill("  - {}", error)
        }
    } else {
        vibez.spill("Process group completed successfully!")
    }
    
    vibez.spill("Final group state: {:?}", group.state())
    
    yolo cap
}

slay demo_enhanced_output_streaming() tea {
    vibez.spill("\n--- Enhanced Output Streaming ---")
    
    fr fr Create command that produces output over time
    sus cmd = exec_vibez.command("sh", ["-c", "for i in 1 2 3 4 5; do echo 'Line $i'; sleep 0.3; done"])
    
    fr fr Create enhanced output streamer
    sus streamer = exec_vibez_advanced.new_enhanced_output_streamer(cmd)
    
    fr fr Configure streaming options
    streamer.set_buffer_size(4096)
    streamer.set_stream_stderr(periodt)
    streamer.set_timestamp_lines(periodt)
    
    fr fr Set up line processing callback
    streamer.on_line(slay(line tea) {
        vibez.spill("📥 Received: {}", line)
    })
    
    fr fr Set up chunk processing callback
    streamer.on_chunk(slay(chunk []byte) {
        vibez.spill("📦 Chunk received: {} bytes", chunk.len())
    })
    
    vibez.spill("Starting enhanced output streaming...")
    vibez.spill("Streamer state: {:?}", streamer.state())
    
    fr fr Start streaming
    err := streamer.start()
    if err != cap {
        vibez.spill("Error starting streaming: %v", err)
        yolo err
    }
    
    vibez.spill("Streaming state after start: {:?}", streamer.state())
    
    fr fr Wait for streaming to complete
    err = streamer.wait()
    if err != cap {
        vibez.spill("Streaming completed with error: %v", err)
    } else {
        vibez.spill("Enhanced output streaming completed successfully!")
    }
    
    vibez.spill("Final streaming state: {:?}", streamer.state())
    
    yolo cap
}

slay demo_enhanced_input_generation() tea {
    vibez.spill("\n--- Enhanced Input Generation ---")
    
    fr fr Create command that processes input
    sus cmd = exec_vibez.command("grep", ["hello"])
    
    fr fr Create enhanced input generator
    sus generator = exec_vibez_advanced.new_enhanced_input_generator(cmd)
    
    vibez.spill("Setting up input generation sequence...")
    vibez.spill("Generator state: {:?}", generator.state())
    
    fr fr Schedule various types of input
    generator.write("hello world\n")?
    generator.write_line("This contains hello")?
    generator.write_after("goodbye world\n", timez.millisecond(100))?
    generator.write_line_after("Another hello line", timez.millisecond(200))?
    
    fr fr Schedule periodic input
    generator.write_periodic("hello periodic\n", timez.millisecond(150), 3)?
    
    vibez.spill("Starting enhanced input generation...")
    
    fr fr Start input generation
    err := generator.start()
    if err != cap {
        vibez.spill("Error starting input generation: %v", err)
        yolo err
    }
    
    vibez.spill("Input generation state after start: {:?}", generator.state())
    
    fr fr Wait a bit for input to be processed
    timez.sleep(timez.second(1))
    
    fr fr Close input stream
    err = generator.close()
    if err != cap {
        vibez.spill("Error closing input generator: %v", err)
    } else {
        vibez.spill("Enhanced input generation completed successfully!")
    }
    
    vibez.spill("Final generator state: {:?}", generator.state())
    
    yolo cap
}

slay demo_enhanced_timeout_control() tea {
    vibez.spill("\n--- Enhanced Timeout Control ---")
    
    fr fr Demo 1: Successful execution within timeout
    vibez.spill("Testing successful execution within timeout...")
    
    sus result, err = exec_vibez_advanced.run_with_enhanced_timeout(
        "echo", 
        ["Quick execution"], 
        timez.second(5),
        cap  fr fr No timeout callback
    )
    
    if err != cap {
        vibez.spill("Quick command failed: %v", err)
    } else {
        vibez.spill("Quick command output: {}", tea(result))
    }
    
    fr fr Demo 2: Timeout with callback
    vibez.spill("Testing timeout with callback...")
    
    sus timeout_occurred = cap
    sus timeout_callback = slay() {
        timeout_occurred = periodt
        vibez.spill("⏰ Timeout callback triggered!")
    }
    
    sus _result, err = exec_vibez_advanced.run_with_enhanced_timeout(
        "sleep", 
        ["5"],  fr fr Sleep for 5 seconds
        timez.millisecond(500),  fr fr But timeout after 500ms
        timeout_callback
    )
    
    if err != cap {
        vibez.spill("Long command timed out as expected: %v", err)
        if timeout_occurred {
            vibez.spill("✅ Timeout callback was triggered correctly")
        } else {
            vibez.spill("⚠️  Timeout callback was not triggered")
        }
    } else {
        vibez.spill("⚠️  Long command completed unexpectedly")
    }
    
    fr fr Demo 3: Timeout manager usage
    vibez.spill("Testing timeout manager...")
    
    sus timeout_mgr = exec_vibez_advanced.TimeoutManager.new(timez.millisecond(200))
    
    vibez.spill("Timeout manager created, expired: {}", timeout_mgr.is_expired())
    vibez.spill("Remaining time: {:?}", timeout_mgr.remaining())
    
    fr fr Wait for timeout
    timez.sleep(timez.millisecond(100))
    vibez.spill("After 100ms - expired: {}, remaining: {:?}", 
                timeout_mgr.is_expired(), timeout_mgr.remaining())
    
    timez.sleep(timez.millisecond(150))
    vibez.spill("After 250ms total - expired: {}, remaining: {:?}", 
                timeout_mgr.is_expired(), timeout_mgr.remaining())
    
    yolo cap
}

slay demo_enhanced_path_lookup() tea {
    vibez.spill("\n--- Enhanced Path Lookup ---")
    
    fr fr Test with common commands that should exist
    sus test_commands = ["sh", "echo", "ls", "cat", "grep"]
    
    for command in test_commands {
        sus path, err = exec_vibez_advanced.enhanced_look_path(command)
        if err != cap {
            vibez.spill("❌ Command '{}' not found: {}", command, err)
        } else {
            vibez.spill("✅ Found '{}' at: {}", command, path)
        }
    }
    
    fr fr Test with non-existent command
    sus _path, err = exec_vibez_advanced.enhanced_look_path("definitely_does_not_exist_12345")
    if err != cap {
        vibez.spill("✅ Correctly failed to find non-existent command: {}", err)
    } else {
        vibez.spill("⚠️  Unexpectedly found non-existent command")
    }
    
    fr fr Test with absolute path
    #[cfg(unix)]
    {
        sus path, err = exec_vibez_advanced.enhanced_look_path("/bin/sh")
        if err != cap {
            vibez.spill("Absolute path lookup for /bin/sh failed: {}", err)
        } else {
            vibez.spill("✅ Absolute path lookup succeeded: {}", path)
        }
    }
    
    #[cfg(windows)]
    {
        sus path, err = exec_vibez_advanced.enhanced_look_path("C:\\Windows\\System32\\cmd.exe")
        if err != cap {
            vibez.spill("Absolute path lookup for cmd.exe failed: {}", err)
        } else {
            vibez.spill("✅ Absolute path lookup succeeded: {}", path)
        }
    }
    
    yolo cap
}

slay demo_real_world_integration() tea {
    vibez.spill("\n--- Real-world Integration Example ---")
    vibez.spill("Building a development environment setup automation...")
    
    fr fr Create enhanced environment for development
    sus dev_env = exec_vibez_advanced.new_enhanced_environment()
    dev_env.set("NODE_ENV", "development")
    dev_env.set("APP_DEBUG", "true")
    dev_env.set("LOG_LEVEL", "debug")
    dev_env.append_path(":/usr/local/node/bin")
    dev_env.append_path(":/opt/cursed/bin")
    
    fr fr Create process group for development setup
    sus setup_config = exec_vibez_advanced.ProcessGroupConfig{
        max_parallel: 2,  fr fr Don't overwhelm the system
        process_timeout: timez.second(30),
        group_timeout: timez.minute(5),
        kill_on_failure: cap,  fr fr Continue despite individual failures
        continue_on_failure: periodt,
        collect_outputs: periodt,
    }
    
    sus setup_group = exec_vibez_advanced.new_enhanced_process_group_with_config(setup_config)
    
    fr fr Add development setup commands
    setup_group.add_command(
        exec_vibez_advanced.command_with_enhanced_env("echo", ["Setting up development environment..."], dev_env.clone())
    )
    
    setup_group.add_command(exec_vibez.command("echo", ["Checking system dependencies..."]))
    setup_group.add_command(exec_vibez.command("echo", ["Initializing project structure..."]))
    setup_group.add_command(exec_vibez.command("echo", ["Setting up configuration files..."]))
    
    fr fr Create a monitoring command with output streaming
    sus monitor_cmd = exec_vibez.command("sh", ["-c", 
        "echo 'Starting monitoring...'; for i in 1 2 3 4 5; do echo 'Status check $i'; sleep 0.2; done; echo 'Monitoring complete'"
    ])
    
    sus monitor_streamer = exec_vibez_advanced.new_enhanced_output_streamer(monitor_cmd)
    monitor_streamer.set_timestamp_lines(periodt)
    
    monitor_streamer.on_line(slay(line tea) {
        vibez.spill("🔍 Monitor: {}", line)
    })
    
    fr fr Create interactive setup command with input generation
    sus interactive_cmd = exec_vibez.command("cat")  fr fr Simple echo for demo
    sus input_gen = exec_vibez_advanced.new_enhanced_input_generator(interactive_cmd)
    
    input_gen.write_line("configuration=development")?
    input_gen.write_line_after("debug=true", timez.millisecond(100))?
    input_gen.write_line_after("setup_complete=true", timez.millisecond(200))?
    
    vibez.spill("🚀 Starting development environment setup...")
    
    fr fr Start the setup process group
    err := setup_group.start_all()
    if err != cap {
        vibez.spill("❌ Failed to start setup group: %v", err)
    } else {
        vibez.spill("✅ Setup group started successfully")
    }
    
    fr fr Start monitoring in parallel
    err = monitor_streamer.start()
    if err != cap {
        vibez.spill("❌ Failed to start monitoring: %v", err)
    } else {
        vibez.spill("✅ Monitoring started successfully")
    }
    
    fr fr Start interactive configuration
    err = input_gen.start()
    if err != cap {
        vibez.spill("❌ Failed to start configuration input: %v", err)
    } else {
        vibez.spill("✅ Configuration input started successfully")
    }
    
    fr fr Wait for setup to complete
    vibez.spill("⏳ Waiting for setup processes to complete...")
    
    err = setup_group.wait_all()
    if err != cap {
        vibez.spill("⚠️  Setup completed with some errors: %v", err)
        
        sus errors = setup_group.errors()
        if errors.len() > 0 {
            vibez.spill("Setup errors:")
            for error in errors {
                vibez.spill("  - {}", error)
            }
        }
    } else {
        vibez.spill("✅ Setup completed successfully!")
    }
    
    fr fr Wait for monitoring to complete
    err = monitor_streamer.wait()
    if err != cap {
        vibez.spill("❌ Monitoring failed: %v", err)
    } else {
        vibez.spill("✅ Monitoring completed successfully!")
    }
    
    fr fr Close configuration input
    err = input_gen.close()
    if err != cap {
        vibez.spill("❌ Failed to close configuration input: %v", err)
    } else {
        vibez.spill("✅ Configuration input completed successfully!")
    }
    
    vibez.spill("🎉 Development environment setup automation completed!")
    vibez.spill("Final states:")
    vibez.spill("  - Setup group: {:?}", setup_group.state())
    vibez.spill("  - Monitor streamer: {:?}", monitor_streamer.state())
    vibez.spill("  - Input generator: {:?}", input_gen.state())
    
    yolo cap
}

fr fr Helper function to demonstrate timeout callback
slay create_timeout_callback() slay() {
    sus start_time = timez.now()
    
    yolo slay() {
        sus elapsed = timez.since(start_time)
        vibez.spill("⏰ Timeout occurred after {:?}", elapsed)
    }
}

fr fr Helper function to create development environment
slay create_dev_environment() exec_vibez_advanced.EnhancedEnvironment {
    sus env = exec_vibez_advanced.new_enhanced_environment()
    
    fr fr Set development-specific variables
    env.set("ENVIRONMENT", "development")
    env.set("DEBUG", "true")
    env.set("LOG_LEVEL", "debug")
    env.set("CURSED_HOME", "/opt/cursed")
    
    fr fr Configure development PATH
    env.prepend_path("/opt/cursed/bin:")
    env.append_path(":/usr/local/development/bin")
    env.append_path(":./node_modules/.bin")
    
    fr fr Remove production-only variables
    env.remove("PRODUCTION_API_KEY")
    env.remove("PROD_DATABASE_URL")
    
    yolo env
}

fr fr Utility function to format duration nicely
slay format_duration(d timez.Duration) tea {
    if d >= timez.second(1) {
        yolo format!("{:.2}s", d.as_secs_f64())
    } bestie if d >= timez.millisecond(1) {
        yolo format!("{}ms", d.as_millis())
    } else {
        yolo format!("{}μs", d.as_micros())
    }
}
