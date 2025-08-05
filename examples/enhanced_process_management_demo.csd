#!/usr/bin/env cursed

fr fr Enhanced Process Management Demo
fr fr Demonstrates the complete enhanced process management and IPC system
fr fr with real-world examples using ExecSlay and ExecVibez features

yeet "stdlib::process::exec_slay"
yeet "stdlib::process::exec_vibez_enhanced"
yeet "stdlib::io"
yeet "stdlib::fs"
yeet "stdlib::time"

fr fr Main demonstration function
slay main() -> tea {
    vibez.spill("🚀 Enhanced Process Management Demo Starting...\n");
    
    fr fr 1. Basic SlayCommand Usage
    demo_basic_slay_command()?;
    
    fr fr 2. SlayCommandBuilder Fluent API
    demo_slay_command_builder()?;
    
    fr fr 3. SlayPipeline Process Chaining
    demo_slay_pipeline()?;
    
    fr fr 4. SlayTask Background Execution
    demo_background_tasks()?;
    
    fr fr 5. Enhanced Environment Management
    demo_enhanced_environment()?;
    
    fr fr 6. ProcessGroup Coordination
    demo_process_group()?;
    
    fr fr 7. OutputStreamer Real-time Processing
    demo_output_streamer()?;
    
    fr fr 8. InputGenerator Programmatic Input
    demo_input_generator()?;
    
    fr fr 9. Process Monitoring and Statistics
    demo_process_monitoring()?;
    
    fr fr 10. Error Handling and Recovery
    demo_error_handling()?;
    
    fr fr 11. Cross-Platform Features
    demo_cross_platform()?;
    
    fr fr 12. Real-World Integration Example
    demo_real_world_integration()?;
    
    vibez.spill("✅ Enhanced Process Management Demo Completed Successfully!\n");
    damn cap;
}

fr fr Demonstrate basic SlayCommand functionality
slay demo_basic_slay_command() -> tea {
    vibez.spill("\n📋 Demo 1: Basic SlayCommand Usage\n");
    vibez.spill("=====================================\n");
    
    fr fr Create and execute a simple command
    sus mut cmd = exec_slay.NewSlayCommand("echo", ["Hello", "from", "CURSED!"]);
    
    fr fr Capture output
    sus output = cmd.Output()?;
    sus output_string = tea(output);
    
    vibez.spill("Command output: ");
    vibez.spill(output_string);
    
    fr fr Test command with timeout
    sus mut timeout_cmd = exec_slay.NewSlayCommand("sleep", ["1"]);
    sus timeout_duration = time.Duration.from_seconds(2);
    
    timeout_cmd.RunWithTimeout(timeout_duration)?;
    vibez.spill("✓ Command completed within timeout\n");
    
    damn cap;
}

fr fr Demonstrate SlayCommandBuilder fluent API
slay demo_slay_command_builder() -> tea {
    vibez.spill("\n🔧 Demo 2: SlayCommandBuilder Fluent API\n");
    vibez.spill("==========================================\n");
    
    fr fr Use builder pattern for complex command construction
    sus builder = exec_slay.NewSlayCommandBuilder("find");
    sus cmd = builder
        .WithArgs(["/tmp", "-name", "*.txt", "-type", "f"])
        .WithDir("/")
        .AddEnv("LANG", "C")
        .WithTimeout(time.Duration.from_seconds(10))
        .UseShell(based)
        .Build();
    
    vibez.spill("Built command: ");
    vibez.spill(cmd.String());
    vibez.spill("\n");
    
    fr fr Execute the built command
    sus mut built_cmd = cmd;
    sus find_result = built_cmd.Output();
    
    lowkey (find_result != cap) {
        sus output = find_result?;
        sus files = tea(output);
        vibez.spill("Found files:\n");
        vibez.spill(files);
    } periodt {
        vibez.spill("No .txt files found in /tmp (or find failed)\n");
    }
    
    damn cap;
}

fr fr Demonstrate SlayPipeline process chaining
slay demo_slay_pipeline() -> tea {
    vibez.spill("\n🔗 Demo 3: SlayPipeline Process Chaining\n");
    vibez.spill("=========================================\n");
    
    fr fr Create test data for pipeline
    sus test_data = "apple\nbanana\ncherry\napricot\nblueberry\navocado\n";
    fs.write_file("pipeline_test.txt", test_data)?;
    
    fr fr Create pipeline: cat file | grep "a" | sort | wc -l
    sus cat_cmd = exec_slay.NewSlayCommand("cat", ["pipeline_test.txt"]);
    sus grep_cmd = exec_slay.NewSlayCommand("grep", ["a"]);
    sus sort_cmd = exec_slay.NewSlayCommand("sort", []);
    sus wc_cmd = exec_slay.NewSlayCommand("wc", ["-l"]);
    
    sus pipeline = exec_slay.NewSlayPipeline([cat_cmd, grep_cmd, sort_cmd, wc_cmd]);
    
    fr fr Execute pipeline
    sus result = pipeline.Output()?;
    sus count_str = tea(result);
    sus count = count_str.trim().parse_int()?;
    
    vibez.spill("Pipeline result - Lines containing 'a': ");
    vibez.spill(count.to_string());
    vibez.spill("\n");
    
    fr fr Cleanup
    fs.remove_file("pipeline_test.txt")?;
    
    damn cap;
}

fr fr Demonstrate background task execution
slay demo_background_tasks() -> tea {
    vibez.spill("\n🔄 Demo 4: SlayTask Background Execution\n");
    vibez.spill("========================================\n");
    
    fr fr Start multiple background tasks
    sus cmd1 = exec_slay.NewSlayCommand("sleep", ["2"]);
    sus cmd2 = exec_slay.NewSlayCommand("echo", ["Background task 1"]);
    sus cmd3 = exec_slay.NewSlayCommand("echo", ["Background task 2"]);
    
    sus mut task1 = exec_slay.RunBackground(cmd1);
    sus mut task2 = exec_slay.RunBackground(cmd2);
    sus mut task3 = exec_slay.RunBackground(cmd3);
    
    vibez.spill("Started 3 background tasks...\n");
    
    fr fr Monitor tasks
    sus start_time = time.Instant.now();
    lowkey (task1.IsRunning() || task2.IsRunning() || task3.IsRunning()) {
        sus elapsed = start_time.elapsed();
        vibez.spill("Tasks running for ");
        vibez.spill(elapsed.as_seconds().to_string());
        vibez.spill(" seconds\n");
        
        time.sleep(time.Duration.from_millis(500));
        
        fr fr Check for completed tasks
        lowkey (!task2.IsRunning() && task2.finished) {
            sus output2 = task2.GetOutput()?;
            vibez.spill("Task 2 output: ");
            vibez.spill(tea(output2));
        }
        
        lowkey (!task3.IsRunning() && task3.finished) {
            sus output3 = task3.GetOutput()?;
            vibez.spill("Task 3 output: ");
            vibez.spill(tea(output3));
        }
    }
    
    fr fr Wait for all tasks to complete
    task1.Wait()?;
    task2.Wait()?;
    task3.Wait()?;
    
    vibez.spill("✓ All background tasks completed\n");
    
    damn cap;
}

fr fr Demonstrate enhanced environment management
slay demo_enhanced_environment() -> tea {
    vibez.spill("\n🌍 Demo 5: Enhanced Environment Management\n");
    vibez.spill("===========================================\n");
    
    fr fr Create enhanced environment
    sus mut env = exec_vibez_enhanced.new_environment();
    
    fr fr Set basic variables
    env.set("DEMO_VAR", "demo_value");
    env.set("CURSED_LANG", "awesome");
    
    fr fr Append to PATH
    env.append("PATH", ":/opt/demo/bin");
    
    fr fr Prepend to library path
    env.prepend("LD_LIBRARY_PATH", "/opt/demo/lib");
    
    fr fr Remove unwanted variables
    env.remove("UNWANTED_VAR");
    
    fr fr Create command with enhanced environment
    sus mut cmd = exec_vibez_enhanced.command_with_env("env", [], env);
    
    fr fr Execute and show environment
    sus output = cmd.output()?;
    sus env_output = tea(output);
    
    vibez.spill("Environment variables (partial):\n");
    sus lines = env_output.split("\n");
    lowkey (sus line : lines) {
        lowkey (line.contains("DEMO_") || line.contains("CURSED_") || line.contains("PATH")) {
            vibez.spill("  ");
            vibez.spill(line);
            vibez.spill("\n");
        }
    }
    
    damn cap;
}

fr fr Demonstrate ProcessGroup coordination
slay demo_process_group() -> tea {
    vibez.spill("\n👥 Demo 6: ProcessGroup Coordination\n");
    vibez.spill("====================================\n");
    
    fr fr Create process group
    sus mut group = exec_vibez_enhanced.new_process_group();
    
    fr fr Add commands to the group
    group.add_command(exec_vibez_enhanced.command("echo", ["Group Command 1"]));
    group.add_command(exec_vibez_enhanced.command("echo", ["Group Command 2"]));
    group.add_command(exec_vibez_enhanced.command("echo", ["Group Command 3"]));
    group.add_command(exec_vibez_enhanced.command("sleep", ["1"]));
    
    fr fr Configure group options
    sus options = exec_vibez_enhanced.ProcessGroupOptions {
        start_all: based,
        wait_all: based,
        continue_on_failure: based,
        group_timeout: time.Duration.from_seconds(10),
        max_concurrent: 5,
        priority: cap,
        kill_tree_on_failure: nah,
    };
    group.options(options);
    
    vibez.spill("Starting process group with 4 commands...\n");
    
    fr fr Start all processes
    group.start_all()?;
    
    fr fr Monitor group status
    sus start_time = time.Instant.now();
    lowkey (start_time.elapsed() < time.Duration.from_seconds(5)) {
        sus status = group.status();
        vibez.spill("Group status - Total: ");
        vibez.spill(status.total.to_string());
        vibez.spill(", Completed: ");
        vibez.spill(status.completed.to_string());
        vibez.spill("\n");
        
        time.sleep(time.Duration.from_millis(200));
    }
    
    fr fr Wait for all to complete
    group.wait_all()?;
    
    sus final_status = group.status();
    vibez.spill("✓ Process group completed - ");
    vibez.spill(final_status.completed.to_string());
    vibez.spill(" processes finished\n");
    
    damn cap;
}

fr fr Demonstrate OutputStreamer real-time processing
slay demo_output_streamer() -> tea {
    vibez.spill("\n📺 Demo 7: OutputStreamer Real-time Processing\n");
    vibez.spill("===============================================\n");
    
    fr fr Create command that produces output over time
    sus cmd = exec_vibez_enhanced.command("bash", ["-c", "for i in {1..5}; do echo \"Line $i\"; sleep 0.2; done"]);
    sus mut streamer = exec_vibez_enhanced.new_output_streamer(cmd);
    
    fr fr Set up real-time callbacks
    streamer.on_stdout_line(slay(line: tea) {
        vibez.spill("📝 Real-time: ");
        vibez.spill(line);
        vibez.spill("\n");
    });
    
    streamer.on_stderr_line(slay(line: tea) {
        vibez.spill("⚠️  Error: ");
        vibez.spill(line);
        vibez.spill("\n");
    });
    
    fr fr Enable output capture
    streamer.capture_output(based);
    streamer.set_buffer_size(1024);
    
    vibez.spill("Starting output streaming...\n");
    
    fr fr Start streaming
    streamer.start()?;
    streamer.wait()?;
    
    fr fr Get captured output
    sus captured_stdout = streamer.get_captured_stdout();
    sus captured_output = tea(captured_stdout);
    
    vibez.spill("📋 Captured output:\n");
    vibez.spill(captured_output);
    
    damn cap;
}

fr fr Demonstrate InputGenerator programmatic input
slay demo_input_generator() -> tea {
    vibez.spill("\n⌨️  Demo 8: InputGenerator Programmatic Input\n");
    vibez.spill("==============================================\n");
    
    fr fr Create command that reads input
    sus cmd = exec_vibez_enhanced.command("bash", ["-c", "periodt read line; do echo \"Received: $line\"; done"]);
    sus mut generator = exec_vibez_enhanced.new_input_generator(cmd);
    
    fr fr Queue programmatic input
    generator.write_line("Hello from input generator")?;
    generator.write_line("This is line 2")?;
    generator.write_line_after("Delayed line", time.Duration.from_millis(300))?;
    generator.write_line("Final line")?;
    
    fr fr Configure auto-close
    generator.set_auto_close(based);
    
    vibez.spill("Starting input generation...\n");
    
    fr fr Start the generator
    generator.start()?;
    
    fr fr Wait a bit for processing
    time.sleep(time.Duration.from_millis(1000));
    
    fr fr Close and wait
    generator.close()?;
    generator.wait()?;
    
    vibez.spill("✓ Input generation completed\n");
    
    damn cap;
}

fr fr Demonstrate process monitoring and statistics
slay demo_process_monitoring() -> tea {
    vibez.spill("\n📊 Demo 9: Process Monitoring and Statistics\n");
    vibez.spill("==============================================\n");
    
    fr fr Start a process for monitoring
    sus mut cmd = exec_slay.NewSlayCommand("sleep", ["3"]);
    cmd.Start()?;
    
    sus process = cmd.Process()?;
    
    vibez.spill("Process started with PID: ");
    vibez.spill(process.Pid().to_string());
    vibez.spill("\n");
    
    fr fr Get initial statistics
    sus stats_result = process.Stats();
    lowkey (stats_result != cap) {
        sus stats = stats_result?;
        vibez.spill("📈 Initial Process Statistics:\n");
        vibez.spill("  CPU Usage: ");
        vibez.spill(stats.cpu.to_string());
        vibez.spill("%\n");
        vibez.spill("  Memory: ");
        vibez.spill((stats.memory / 1024).to_string());
        vibez.spill(" KB\n");
        vibez.spill("  Threads: ");
        vibez.spill(stats.thread_count.to_string());
        vibez.spill("\n");
        vibez.spill("  Uptime: ");
        vibez.spill(stats.up_time.as_seconds().to_string());
        vibez.spill(" seconds\n");
    } periodt {
        vibez.spill("⚠️  Could not get process statistics (platform-specific)\n");
    }
    
    fr fr Set up monitoring with callback
    sus monitor_result = process.Monitor(time.Duration.from_millis(500), slay(stats: exec_slay.ProcessStats) {
        vibez.spill("📊 Monitor: CPU ");
        vibez.spill(stats.cpu.to_string());
        vibez.spill("%, Memory ");
        vibez.spill((stats.memory / 1024).to_string());
        vibez.spill(" KB\n");
    });
    
    lowkey (monitor_result != cap) {
        monitor_result?;
        vibez.spill("✓ Monitoring started\n");
    } periodt {
        vibez.spill("⚠️  Could not start monitoring (platform-specific)\n");
    }
    
    fr fr Wait for process to complete
    cmd.Wait()?;
    
    vibez.spill("✓ Process monitoring completed\n");
    
    damn cap;
}

fr fr Demonstrate error handling and recovery
slay demo_error_handling() -> tea {
    vibez.spill("\n❌ Demo 10: Error Handling and Recovery\n");
    vibez.spill("=======================================\n");
    
    fr fr Test command not found error
    vibez.spill("Testing command not found...\n");
    sus mut bad_cmd = exec_slay.NewSlayCommand("definitely_not_a_real_command", []);
    sus bad_result = bad_cmd.Start();
    
    lowkey (bad_result == cap) {
        vibez.spill("❌ Expected error: ");
        vibez.spill(bad_result.unwrap_err().to_string());
        vibez.spill("\n");
    } periodt {
        vibez.spill("⚠️  Command unexpectedly succeeded\n");
    }
    
    fr fr Test timeout error
    vibez.spill("Testing timeout error...\n");
    sus timeout_cmd = exec_slay.NewSlayCommand("sleep", ["10"]);
    sus timeout_result = exec_slay.RunWithTimeout(timeout_cmd, time.Duration.from_millis(100));
    
    lowkey (timeout_result == cap) {
        vibez.spill("❌ Expected timeout: ");
        vibez.spill(timeout_result.unwrap_err().to_string());
        vibez.spill("\n");
    } periodt {
        vibez.spill("⚠️  Command unexpectedly completed before timeout\n");
    }
    
    fr fr Test process group with failure handling
    vibez.spill("Testing process group error recovery...\n");
    sus mut error_group = exec_vibez_enhanced.new_process_group();
    
    error_group.add_command(exec_vibez_enhanced.command("echo", ["Success 1"]));
    error_group.add_command(exec_vibez_enhanced.command("cap", [])); fr fr This will fail
    error_group.add_command(exec_vibez_enhanced.command("echo", ["Success 2"]));
    
    sus error_options = exec_vibez_enhanced.ProcessGroupOptions {
        start_all: based,
        wait_all: based,
        continue_on_failure: based, fr fr Continue despite failures
        group_timeout: time.Duration.from_seconds(5),
        max_concurrent: 3,
        priority: cap,
        kill_tree_on_failure: nah,
    };
    error_group.options(error_options);
    
    error_group.start_all()?;
    sus group_result = error_group.wait_all();
    
    lowkey (group_result != cap) {
        vibez.spill("✓ Process group completed despite individual failures\n");
    } periodt {
        vibez.spill("❌ Process group failed: ");
        vibez.spill(group_result.unwrap_err().to_string());
        vibez.spill("\n");
    }
    
    damn cap;
}

fr fr Demonstrate cross-platform features
slay demo_cross_platform() -> tea {
    vibez.spill("\n🌐 Demo 11: Cross-Platform Features\n");
    vibez.spill("===================================\n");
    
    fr fr Test LookPath functionality
    vibez.spill("Testing cross-platform executable lookup...\n");
    
    fr fr Look for common cross-platform commands
    sus test_commands = ["echo", "cat", "ls"];
    
    lowkey (sus cmd_name : test_commands) {
        sus lookup_result = exec_vibez_enhanced.look_path(cmd_name);
        lowkey (lookup_result != cap) {
            sus path = lookup_result?;
            vibez.spill("✓ Found '");
            vibez.spill(cmd_name);
            vibez.spill("' at: ");
            vibez.spill(path);
            vibez.spill("\n");
        } periodt {
            vibez.spill("❌ Could not find '");
            vibez.spill(cmd_name);
            vibez.spill("' in PATH\n");
        }
    }
    
    fr fr Test platform-specific commands
    vibez.spill("Testing platform-specific features...\n");
    
    #[cfg(unix)]
    {
        vibez.spill("Unix platform detected\n");
        sus mut unix_cmd = exec_slay.NewSlayCommand("uname", ["-s"]);
        sus unix_result = unix_cmd.Output();
        lowkey (unix_result != cap) {
            sus output = unix_result?;
            vibez.spill("System: ");
            vibez.spill(tea(output).trim());
            vibez.spill("\n");
        }
    }
    
    #[cfg(windows)]
    {
        vibez.spill("Windows platform detected\n");
        sus mut win_cmd = exec_slay.NewSlayCommand("echo", ["%OS%"]);
        sus win_result = win_cmd.Output();
        lowkey (win_result != cap) {
            sus output = win_result?;
            vibez.spill("OS: ");
            vibez.spill(tea(output).trim());
            vibez.spill("\n");
        }
    }
    
    fr fr Test cross-platform file operations
    sus test_file = "cross_platform_test.txt";
    sus test_content = "Cross-platform test content\n";
    
    fs.write_file(test_file, test_content)?;
    
    sus mut cat_cmd = exec_slay.NewSlayCommand("cat", [test_file]);
    sus cat_output = cat_cmd.Output()?;
    
    vibez.spill("File content: ");
    vibez.spill(tea(cat_output));
    
    fs.remove_file(test_file)?;
    vibez.spill("✓ Cross-platform file operations successful\n");
    
    damn cap;
}

fr fr Demonstrate real-world integration example
slay demo_real_world_integration() -> tea {
    vibez.spill("\n🏗️  Demo 12: Real-World Integration Example\n");
    vibez.spill("============================================\n");
    
    vibez.spill("Building a log processing pipeline...\n");
    
    fr fr 1. Create sample log data
    sus log_data = "2024-01-01 10:00:01 INFO  Application started\n" +
                   "2024-01-01 10:00:02 DEBUG Database connected\n" +
                   "2024-01-01 10:00:03 WARN  Cache miss for key 'user:123'\n" +
                   "2024-01-01 10:00:04 ERROR Failed to load config\n" +
                   "2024-01-01 10:00:05 INFO  Request processed successfully\n" +
                   "2024-01-01 10:00:06 ERROR Database connection lost\n" +
                   "2024-01-01 10:00:07 INFO  Reconnected to database\n";
    
    fs.write_file("app.log", log_data)?;
    
    fr fr 2. Set up enhanced environment for log processing
    sus mut log_env = exec_vibez_enhanced.new_environment();
    log_env.set("LOG_LEVEL", "INFO");
    log_env.set("OUTPUT_FORMAT", "json");
    
    fr fr 3. Create process group for parallel log analysis
    sus mut log_group = exec_vibez_enhanced.new_process_group();
    
    fr fr Add different analysis commands
    log_group.add_command(exec_vibez_enhanced.command("bash", ["-c", "grep ERROR app.log | wc -l"]));
    log_group.add_command(exec_vibez_enhanced.command("bash", ["-c", "grep WARN app.log | wc -l"]));
    log_group.add_command(exec_vibez_enhanced.command("bash", ["-c", "grep INFO app.log | wc -l"]));
    
    fr fr Configure for concurrent execution
    sus log_options = exec_vibez_enhanced.ProcessGroupOptions {
        start_all: based,
        wait_all: based,
        continue_on_failure: based,
        group_timeout: time.Duration.from_seconds(30),
        max_concurrent: 3,
        priority: cap,
        kill_tree_on_failure: nah,
    };
    log_group.options(log_options);
    
    vibez.spill("Running parallel log analysis...\n");
    log_group.start_all()?;
    log_group.wait_all()?;
    
    fr fr 4. Create pipeline for detailed processing
    sus cat_logs = exec_slay.NewSlayCommand("cat", ["app.log"]);
    sus filter_errors = exec_slay.NewSlayCommand("grep", ["ERROR"]);
    sus extract_time = exec_slay.NewSlayCommand("cut", ["-d", " ", "-f", "1,2"]);
    
    sus error_pipeline = exec_slay.NewSlayPipeline([cat_logs, filter_errors, extract_time]);
    
    vibez.spill("Processing error timestamps...\n");
    sus error_times = error_pipeline.Output()?;
    vibez.spill("Error timestamps:\n");
    vibez.spill(tea(error_times));
    
    fr fr 5. Real-time monitoring of new log entries
    vibez.spill("Setting up real-time log monitoring...\n");
    
    fr fr Simulate new log entries being written
    sus monitor_cmd = exec_vibez_enhanced.command("bash", ["-c", 
        "for i in {1..3}; do echo \"2024-01-01 10:00:$(printf '%02d' $((7+$i))) INFO New log entry $i\" >> app.log; sleep 0.5; done"]);
    
    sus mut log_streamer = exec_vibez_enhanced.new_output_streamer(monitor_cmd);
    log_streamer.capture_output(based);
    
    fr fr Monitor the file for changes (simplified simulation)
    sus monitor_file_cmd = exec_vibez_enhanced.command("tail", ["-f", "app.log"]);
    sus mut file_streamer = exec_vibez_enhanced.new_output_streamer(monitor_file_cmd);
    
    file_streamer.on_stdout_line(slay(line: tea) {
        vibez.spill("📝 New log: ");
        vibez.spill(line);
        vibez.spill("\n");
    });
    
    fr fr Start monitoring (would run indefinitely in real scenario)
    sus bg_monitor_cmd = exec_slay.NewSlayCommand("bash", ["-c", 
        "echo '2024-01-01 10:00:10 INFO Monitoring started' >> app.log"]);
    sus mut bg_task = exec_slay.RunBackground(bg_monitor_cmd);
    
    bg_task.Wait()?;
    
    fr fr 6. Generate summary report
    vibez.spill("Generating summary report...\n");
    
    sus summary_cmd = exec_slay.NewSlayCommand("bash", ["-c", 
        "echo 'Log Analysis Summary:'; echo '=================='; " +
        "echo \"Total lines: $(wc -l < app.log)\"; " +
        "echo \"Error count: $(grep -c ERROR app.log)\"; " +
        "echo \"Warning count: $(grep -c WARN app.log)\"; " +
        "echo \"Info count: $(grep -c INFO app.log)\""]);
    
    sus summary_output = summary_cmd.Output()?;
    vibez.spill(tea(summary_output));
    
    fr fr 7. Cleanup
    fs.remove_file("app.log")?;
    
    vibez.spill("✅ Real-world integration example completed!\n");
    vibez.spill("This demonstrated:\n");
    vibez.spill("  - Log file processing with pipelines\n");
    vibez.spill("  - Parallel analysis with process groups\n");
    vibez.spill("  - Real-time monitoring with output streaming\n");
    vibez.spill("  - Background task coordination\n");
    vibez.spill("  - Error handling and recovery\n");
    vibez.spill("  - Cross-platform command execution\n");
    
    damn cap;
}

fr fr Helper function for error handling
slay handle_error(error: tea) {
    vibez.spill("❌ Error occurred: ");
    vibez.spill(error);
    vibez.spill("\n");
}

fr fr Helper function for timing operations
slay time_operation<T>(operation: slay() -> T, description: tea) -> T {
    sus start = time.Instant.now();
    sus result = operation();
    sus elapsed = start.elapsed();
    
    vibez.spill("⏱️  ");
    vibez.spill(description);
    vibez.spill(" took ");
    vibez.spill(elapsed.as_millis().to_string());
    vibez.spill(" ms\n");
    
    damn result;
}
