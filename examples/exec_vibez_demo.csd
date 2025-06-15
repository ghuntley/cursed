fr fr CURSED exec_vibez Demo - External Command Execution
fr fr This demonstrates the exec_vibez module for running external commands

import "stdlib::exec_vibez"
import "stdlib::io"
import "stdlib::time"

slay main() tea {
    fr fr Basic command execution
    io.println("=== Basic Command Execution ===")?
    
    let cmd = exec_vibez.command("echo", ["Hello", "from", "CURSED!"])
    let output = cmd.output()?
    io.printf("Echo output: {}\n", [tea(output)])?
    
    fr fr Command with combined output (stdout + stderr)
    io.println("\n=== Combined Output ===")?
    
    #if unix
    let cmd2 = exec_vibez.command("sh", ["-c", "echo stdout; echo stderr >&2"])
    #else
    let cmd2 = exec_vibez.command("cmd", ["/c", "echo stdout & echo stderr 1>&2"])
    #endif
    
    let combined = cmd2.combined_output()?
    io.printf("Combined output: {}\n", [tea(combined)])?
    
    fr fr Process lifecycle management
    io.println("\n=== Process Lifecycle ===")?
    
    let cmd3 = exec_vibez.command("sleep", ["1"])
    io.println("Starting sleep command...")?
    cmd3.start()?
    
    let process = cmd3.process()?
    io.printf("Process started with PID: {}\n", [process.pid])?
    
    io.println("Waiting for process to complete...")?
    cmd3.wait()?
    io.println("Process completed!")?
    
    fr fr Environment variables
    io.println("\n=== Custom Environment ===")?
    
    let env = exec_vibez.new_environment()
    env.set("CURSED_VAR", "cursed_value")
    env.set("DEMO_MODE", "true")
    
    let cmd4 = exec_vibez.command_with_env("env", [], env)
    let env_output = cmd4.output()?
    io.printf("Environment contains our variables:\n{}\n", [tea(env_output)])?
    
    fr fr Process groups
    io.println("\n=== Process Groups ===")?
    
    let group = exec_vibez.new_process_group()
    
    let cmd5 = exec_vibez.command("echo", ["Group command 1"])
    let cmd6 = exec_vibez.command("echo", ["Group command 2"])
    let cmd7 = exec_vibez.command("echo", ["Group command 3"])
    
    group.add_command(cmd5)
    group.add_command(cmd6)
    group.add_command(cmd7)
    
    io.println("Starting process group...")?
    group.start_all()?
    
    io.println("Waiting for all processes to complete...")?
    group.wait_all()?
    io.println("All processes in group completed!")?
    
    fr fr Timeout example
    io.println("\n=== Command with Timeout ===")?
    
    let timeout_result = exec_vibez.run_with_timeout(
        "echo", 
        ["timeout test"], 
        time.seconds(5)
    )
    
    if timeout_result is ok {
        let output = timeout_result.unwrap()
        io.printf("Timeout command output: {}\n", [tea(output)])?
    } else {
        io.println("Command timed out or failed")?
    }
    
    fr fr Real-time output streaming
    io.println("\n=== Output Streaming ===")?
    
    #if unix
    let stream_cmd = exec_vibez.command("sh", ["-c", "for i in 1 2 3; do echo Line $i; sleep 0.5; done"])
    #else
    let stream_cmd = exec_vibez.command("cmd", ["/c", "for /l %i in (1,1,3) do (echo Line %i & timeout /t 1 > nul)"])
    #endif
    
    let streamer = exec_vibez.new_output_streamer(stream_cmd)
    
    streamer.on_line(|line| {
        io.printf("Real-time: {}\n", [line]) catch {}
    })
    
    io.println("Starting real-time streaming...")?
    streamer.start()?
    streamer.wait()?
    io.println("Streaming completed!")?
    
    fr fr Interactive input
    io.println("\n=== Interactive Input ===")?
    
    let input_cmd = exec_vibez.command("cat", [])
    let input_gen = exec_vibez.new_input_generator(input_cmd)
    
    fr fr Set up output capture
    input_cmd.stdout = io.new_buffer()
    
    io.println("Starting cat with programmatic input...")?
    input_cmd.start()?
    
    fr fr Send input with delays
    input_gen.write("Line 1 from CURSED\n")?
    input_gen.write_after("Line 2 after delay\n", time.milliseconds(200))?
    input_gen.write_after("Line 3 final\n", time.milliseconds(200))?
    input_gen.close()?
    
    input_cmd.wait()?
    
    let cat_output = input_cmd.stdout.to_string()
    io.printf("Cat received:\n{}\n", [cat_output])?
    
    fr fr Signal handling (Unix only)
    #if unix
    io.println("\n=== Signal Handling ===")?
    
    let signal_cmd = exec_vibez.command("sleep", ["5"])
    signal_cmd.start()?
    
    let signal_process = signal_cmd.process()?
    io.printf("Started sleep process with PID: {}\n", [signal_process.pid])?
    
    io.println("Sending SIGTERM to process...")?
    signal_process.signal(15)?
    
    let wait_result = signal_cmd.wait()
    if wait_result is err {
        io.println("Process terminated by signal (as expected)")?
    } else {
        io.println("Process completed normally")?
    }
    #endif
    
    fr fr Working directory
    io.println("\n=== Working Directory ===")?
    
    let pwd_cmd = exec_vibez.command("pwd", [])
    pwd_cmd.dir = "/tmp"
    
    let pwd_output = pwd_cmd.output()?
    io.printf("Working directory: {}\n", [tea(pwd_output)])?
    
    fr fr Finding executables
    io.println("\n=== Executable Lookup ===")?
    
    #if unix
    let shell_path = exec_vibez.look_path("sh")?
    io.printf("Shell found at: {}\n", [shell_path])?
    #else
    let cmd_path = exec_vibez.look_path("cmd")?
    io.printf("Command prompt found at: {}\n", [cmd_path])?
    #endif
    
    fr fr Try to find non-existent command
    let fake_result = exec_vibez.look_path("definitely_not_real_command_xyz")
    if fake_result is err {
        io.println("Non-existent command correctly not found")?
    }
    
    fr fr Error handling demonstration
    io.println("\n=== Error Handling ===")?
    
    let failing_cmd = exec_vibez.command("false", [])
    let fail_result = failing_cmd.run()
    
    if fail_result is err {
        io.println("Command failure correctly detected")?
        let error = fail_result.unwrap_err()
        io.printf("Error: {}\n", [error.message])?
    }
    
    io.println("\n=== Demo Completed Successfully! ===")?
    
    tea cap  fr fr Success!
}

fr fr Helper function to demonstrate process state inspection
slay inspect_process_state(state: exec_vibez.ProcessState) tea {
    io.printf("Process State Inspection:\n")?
    io.printf("  PID: {}\n", [state.pid])?
    io.printf("  Exited: {}\n", [state.exited()])?
    io.printf("  Success: {}\n", [state.success()])?
    io.printf("  Exit Code: {}\n", [state.exit_code()])?
    io.printf("  User Time: {:?}\n", [state.user_time()])?
    io.printf("  System Time: {:?}\n", [state.system_time()])?
    io.printf("  String Repr: {}\n", [state.string()])?
    
    tea cap
}

fr fr Helper function for advanced process management
slay advanced_process_demo() tea {
    io.println("=== Advanced Process Management ===")?
    
    fr fr Create a command with timeout context
    let ctx = exec_vibez.ProcessContext.with_timeout(time.seconds(3))
    let timeout_cmd = exec_vibez.command_context(ctx, "sleep", ["2"])
    
    io.println("Running command with timeout context...")?
    let start_time = time.now()
    
    let result = timeout_cmd.run()
    let elapsed = time.since(start_time)
    
    io.printf("Command completed in: {:?}\n", [elapsed])?
    
    if result is ok {
        io.println("Command completed successfully within timeout")?
    } else {
        io.println("Command failed or timed out")?
    }
    
    fr fr Demonstrate process state inspection
    let inspect_cmd = exec_vibez.command("echo", ["state_test"])
    inspect_cmd.run()?
    
    let state = inspect_cmd.process_state()?
    inspect_process_state(state)?
    
    tea cap
}
