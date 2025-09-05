facts exec_slay = import("stdlib::exec_slay");

slay main_character() {
    vibez.spill("ExecSlay Demo - Running external commands with style!");
    
    fr fr Basic command execution
    vibez.spill("\n1. Basic command execution:");
    sus cmd = exec_slay.new_slay_command("echo", ["Hello, World!"]);
    sus output = cmd.output()?;
    vibez.spill("Output:", tea(output));
    
    fr fr Using command builder
    vibez.spill("\n2. Using command builder:");
    sus builder_cmd = exec_slay.new_slay_command_builder("echo")
        .with_arg("Builder pattern works!")
        .with_timeout(5 * time.Second)
        .build()?;
    sus builder_output = builder_cmd.output()?;
    vibez.spill("Builder output:", tea(builder_output));
    
    fr fr Command pipeline
    vibez.spill("\n3. Command pipeline:");
    sus echo_cmd = exec_slay.new_slay_command("echo", ["apple\nbanana\napple\ncherry"]);
    sus grep_cmd = exec_slay.new_slay_command("grep", ["apple"]);
    sus wc_cmd = exec_slay.new_slay_command("wc", ["-l"]);
    
    sus pipeline = exec_slay.pipe([echo_cmd, grep_cmd, wc_cmd]);
    sus pipeline_result = pipeline.output()?;
    vibez.spill("Pipeline result (lines with 'apple'):", tea(pipeline_result));
    
    fr fr Background task
    vibez.spill("\n4. Background task:");
    sus sleep_cmd = exec_slay.new_slay_command("sleep", ["2"]);
    sus task = exec_slay.run_background(sleep_cmd);
    
    vibez.spill("Task started, waiting...");
    time.Sleep(1 * time.Second);
    
    lowkey task.is_running() {
        vibez.spill("Task is still running after 1 second");
    } else {
        vibez.spill("Task completed quickly");
    }
    
    task.wait()?;
    vibez.spill("Task completed with exit code:", task.exit_code());
    
    fr fr Shell commands
    vibez.spill("\n5. Shell commands:");
    exec_slay.run_shell("echo 'Direct shell execution'")?;
    
    sus shell_output = exec_slay.shell_output("echo 'Shell output capture'")?;
    vibez.spill("Shell output:", tea(shell_output));
    
    fr fr Command with timeout
    vibez.spill("\n6. Command with timeout:");
    sus timeout_cmd = exec_slay.new_slay_command("echo", ["Timeout test"]);
    sus timeout_output = exec_slay.output_with_timeout(timeout_cmd, 1 * time.Second)?;
    vibez.spill("Timeout output:", tea(timeout_output));
    
    fr fr Environment variables
    vibez.spill("\n7. Environment variables:");
    sus env_map = map[tea]tea{
        "MESSAGE": "Hello from environment!",
    };
    exec_slay.run_shell_with_env("echo $MESSAGE", env_map)?;
    
    vibez.spill("\nExecSlay demo completed successfully!");
}
