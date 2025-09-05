fr fr/ CURSED Process Management - Basic Usage Examples
fr fr/ 
fr fr/ This example shows practical, everyday usage of the process management APIs
fr fr/ in the CURSED programming language.

yeet "stdlib::process"
yeet "stdlib::io"
yeet "stdlib::time"

fr fr/ Simple command execution
slay run_basic_commands() tea {
    vibez.spill("=== Basic Command Execution ===");
    
    fr fr Execute a simple command
    sus result = process.exec("echo 'Hello, CURSED world!'");
    lowkey result != cap {
        vibez.spill("Output: %s", result.stdout_lossy().trim());
    } else {
        vibez.spill("Error: %v", result.err());
        damn;
    }
    
    fr fr Execute command with arguments
    sus ls_result = process.exec_with_args("ls", ["-la", "."]);
    lowkey ls_result != cap {
        vibez.spill("Directory listing completed (%d bytes)", ls_result.stdout.len());
    }
}

fr fr/ Working with files and directories
slay file_operations() tea {
    vibez.spill("\n=== File Operations ===");
    
    fr fr Check current directory
    sus pwd_result = process.exec("pwd");
    lowkey pwd_result != cap {
        vibez.spill("Current directory: %s", pwd_result.stdout_lossy().trim());
    }
    
    fr fr Create a temporary file
    sus touch_result = process.exec("touch /tmp/cursed_test.txt");
    lowkey touch_result != cap {
        vibez.spill("Created temporary file");
        
        fr fr Write content to file
        sus echo_result = process.run_shell("echo 'CURSED test content' > /tmp/cursed_test.txt");
        lowkey echo_result != cap {
            fr fr Read the file back
            sus cat_result = process.exec_with_args("cat", ["/tmp/cursed_test.txt"]);
            lowkey cat_result != cap {
                vibez.spill("File content: %s", cat_result.stdout_lossy().trim());
            }
        }
        
        fr fr Clean up
        sus rm_result = process.exec_with_args("rm", ["/tmp/cursed_test.txt"]);
        lowkey rm_result != cap {
            vibez.spill("Cleaned up temporary file");
        }
    }
}

fr fr/ Process monitoring and management
slay process_management() tea {
    vibez.spill("\n=== Process Management ===");
    
    fr fr Start a background process
    sus sleep_cmd = process.new_slay_command("sleep", ["5"]);
    sus task = process.run_background(sleep_cmd);
    
    vibez.spill("Started background process...");
    
    fr fr Monitor the process
    lowkey i := 0; i < 6; i++ {
        lowkey task.is_running() {
            vibez.spill("Process running for %v seconds", task.elapsed_time().as_secs());
            timez.sleep(timez.Duration.from_secs(1));
        } else {
            vibez.spill("Process completed!");
            break;
        }
    }
    
    fr fr Wait for completion
    sus wait_result = task.wait();
    lowkey wait_result != cap {
        vibez.spill("Background process finished successfully");
    }
}

fr fr/ Building command pipelines
slay pipeline_example() tea {
    vibez.spill("\n=== Command Pipelines ===");
    
    fr fr Create a data processing pipeline
    fr fr Generate test data -> Sort it -> Count unique lines
    sus echo_cmd = process.new_slay_command("echo", ["banana\napple\ncherry\napple\nbanana\ndate"]);
    sus sort_cmd = process.new_slay_command("sort", []);
    sus uniq_cmd = process.new_slay_command("uniq", ["-c"]);
    
    sus pipeline = process.pipe([echo_cmd, sort_cmd, uniq_cmd]);
    
    vibez.spill("Pipeline: %s", pipeline.string());
    
    fr fr Execute the pipeline
    sus result = pipeline.output();
    lowkey result != cap {
        vibez.spill("Pipeline results:");
        vibez.spill("%s", tea(result));
    } else {
        vibez.spill("Pipeline error: %v", result.err());
    }
}

fr fr/ Working with environment variables
slay environment_example() tea {
    vibez.spill("\n=== Environment Variables ===");
    
    fr fr Create custom environment
    sus env = process.new_environment();
    env.set("CURSED_APP", "demo");
    env.set("LOG_LEVEL", "debug");
    env.set("USER_NAME", "cursed_user");
    
    fr fr Run command with custom environment
    sus env_cmd = process.command_with_env("printenv", ["CURSED_APP"], env);
    sus result = env_cmd.output();
    lowkey result != cap {
        vibez.spill("Environment variable CURSED_APP: %s", tea(result).trim());
    }
    
    fr fr Shell command with environment
    sus env_vars = map[tea]tea{
        "GREETING": "Hello from CURSED!",
        "VERSION": "1.0.0",
    };
    
    process.run_shell_with_env("echo $GREETING v$VERSION", env_vars);
}

fr fr/ Error handling patterns
slay error_handling_patterns() tea {
    vibez.spill("\n=== Error Handling ===");
    
    fr fr Handle command not found
    sus bad_result = process.exec("nonexistent_command");
    lowkey bad_result.is_err() {
        vibez.spill("✓ Handled missing command gracefully");
    } else {
        vibez.spill("✗ Expected error for missing command");
    }
    
    fr fr Handle command with error exit code
    sus fail_cmd = process.exec_with_args("ls", ["/nonexistent/path"]);
    lowkey fail_cmd.is_err() {
        vibez.spill("✓ Handled command failure gracefully");
    } else {
        vibez.spill("✗ Expected error for invalid path");
    }
    
    fr fr Timeout handling
    sus timeout_cmd = process.new_slay_command("sleep", ["10"]);
    sus timeout_result = process.slay_run_with_timeout(timeout_cmd, timez.Duration.from_secs(1));
    lowkey timeout_result.is_err() {
        vibez.spill("✓ Handled timeout gracefully");
    } else {
        vibez.spill("✗ Expected timeout error");
    }
}

fr fr/ File processing example
slay file_processing_example() tea {
    vibez.spill("\n=== File Processing Example ===");
    
    fr fr Create a test file with data
    sus create_result = process.run_shell("echo -e 'line1\nline2\nline3\nline1\nline4' > /tmp/test_data.txt");
    lowkey create_result != cap {
        vibez.spill("Created test data file");
        
        fr fr Process the file: sort and remove duplicates
        sus sort_cmd = process.new_slay_command("sort", ["/tmp/test_data.txt"]);
        sus uniq_cmd = process.new_slay_command("uniq", []);
        sus pipeline = process.pipe([sort_cmd, uniq_cmd]);
        
        sus result = pipeline.output();
        lowkey result != cap {
            vibez.spill("Processed file content:");
            vibez.spill("%s", tea(result));
        }
        
        fr fr Count lines in original vs processed
        sus wc_original = process.exec_with_args("wc", ["-l", "/tmp/test_data.txt"]);
        lowkey wc_original != cap {
            vibez.spill("Original file lines: %s", wc_original.stdout_lossy().trim());
        }
        
        fr fr Clean up
        process.exec_with_args("rm", ["/tmp/test_data.txt"]);
    }
}

fr fr/ System information gathering
slay system_info_example() tea {
    vibez.spill("\n=== System Information ===");
    
    fr fr Get system information
    sus uname_result = process.exec_with_args("uname", ["-a"]);
    lowkey uname_result != cap {
        vibez.spill("System: %s", uname_result.stdout_lossy().trim());
    }
    
    fr fr Get current user
    sus whoami_result = process.exec("whoami");
    lowkey whoami_result != cap {
        vibez.spill("User: %s", whoami_result.stdout_lossy().trim());
    }
    
    fr fr Get disk usage
    sus df_result = process.exec_with_args("df", ["-h", "."]);
    lowkey df_result != cap {
        vibez.spill("Disk usage for current directory:");
        vibez.spill("%s", df_result.stdout_lossy());
    }
}

fr fr/ Main function demonstrating all examples
slay main_character() tea {
    vibez.spill("CURSED Process Management - Basic Usage Examples");
    vibez.spill("=================================================");
    
    fr fr Check if we have required commands
    lowkey !process.command_exists("echo") {
        vibez.spill("Error: echo command not available");
        damn;
    }
    
    fr fr Run all examples
    run_basic_commands();
    file_operations();
    process_management();
    pipeline_example();
    environment_example();
    error_handling_patterns();
    file_processing_example();
    system_info_example();
    
    vibez.spill("\n=== All Examples Complete ===");
    vibez.spill("Process management examples completed successfully!");
    vibez.spill("Try modifying the examples to explore different features.");
}
