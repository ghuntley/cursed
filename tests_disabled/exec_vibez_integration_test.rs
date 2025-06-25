use cursed::stdlib::process::exec_vibez::{
    command, command_context, look_path, run_with_timeout,
    new_process_group, new_environment, new_output_streamer, new_input_generator,
    Cmd, Process, ProcessState, Error, ProcessContext, ProcessGroup, Environment,
    OutputStreamer, InputGenerator
};
use cursed::stdlib::process::error::ProcessResult;
use std::time::Duration;
use std::sync::{Arc, Mutex};

#[test]
fn test_basic_command_execution() {
    // Test basic echo command
    let mut cmd = command("echo", &["hello", "world"]);
    let output = cmd.output().expect("Failed to execute echo");
    let output_str = String::from_utf8(output).expect("Invalid UTF-8");
    assert!(output_str.trim().contains("hello world"));
}

#[test]
fn test_command_with_args() {
    // Test command with multiple arguments
    let mut cmd = command("echo", &["-n", "test"]);
    let output = cmd.output().expect("Failed to execute echo -n");
    let output_str = String::from_utf8(output).expect("Invalid UTF-8");
    assert_eq!(output_str, "test");
}

#[test]
fn test_command_combined_output() {
    // Test combined stdout and stderr
    #[cfg(unix)]
    let mut cmd = command("sh", &["-c", "echo stdout; echo stderr >&2"]);
    #[cfg(windows)]
    let mut cmd = command("cmd", &["/c", "echo stdout & echo stderr 1>&2"]);
    
    let combined = cmd.combined_output().expect("Failed to get combined output");
    let combined_str = String::from_utf8(combined).expect("Invalid UTF-8");
    assert!(combined_str.contains("stdout"));
    assert!(combined_str.contains("stderr"));
}

#[test]
fn test_command_start_and_wait() {
    // Test start/wait workflow
    let mut cmd = command("echo", &["start_wait_test"]);
    cmd.start().expect("Failed to start command");
    cmd.wait().expect("Failed to wait for command");
}

#[test]
fn test_command_pipes() {
    use std::io::{Read, Write};
    
    // Test stdin/stdout pipes
    let mut cmd = command("cat", &[]);
    
    cmd.start().expect("Failed to start cat");
    
    let mut stdin = cmd.stdin_pipe().expect("Failed to get stdin pipe");
    let mut stdout = cmd.stdout_pipe().expect("Failed to get stdout pipe");
    
    // Write to stdin
    stdin.write_all(b"test input\n").expect("Failed to write to stdin");
    drop(stdin); // Close stdin
    
    // Read from stdout
    let mut output = String::new();
    stdout.read_to_string(&mut output).expect("Failed to read from stdout");
    
    cmd.wait().expect("Failed to wait for command");
    
    assert!(output.contains("test input"));
}

#[test]
fn test_process_context_timeout() {
    let ctx = ProcessContext::with_timeout(Duration::from_millis(100));
    assert!(!ctx.is_cancelled());
    assert_eq!(ctx.timeout, Some(Duration::from_millis(100)));
    
    ctx.cancel();
    assert!(ctx.is_cancelled());
}

#[test]
fn test_environment_management() {
    let mut env = Environment::new();
    
    // Test basic set/get
    env.set("TEST_VAR", "test_value");
    assert_eq!(env.get("TEST_VAR"), Some(&"test_value".to_string()));
    
    // Test append
    env.set("PATH", "/usr/bin");
    env.append("PATH", ":/usr/local/bin");
    assert_eq!(env.get("PATH"), Some(&"/usr/bin:/usr/local/bin".to_string()));
    
    // Test remove
    env.remove("TEST_VAR");
    assert_eq!(env.get("TEST_VAR"), None);
    
    // Test to_env_vec
    env.set("VAR1", "value1");
    env.set("VAR2", "value2");
    let env_vec = env.to_env_vec();
    assert!(env_vec.iter().any(|s| s.starts_with("VAR1=")));
    assert!(env_vec.iter().any(|s| s.starts_with("VAR2=")));
}

#[test]
fn test_process_group() {
    let mut group = new_process_group();
    
    // Add commands to group
    let cmd1 = command("echo", &["group1"]);
    let cmd2 = command("echo", &["group2"]);
    
    group.add_command(cmd1);
    group.add_command(cmd2);
    
    // Start all commands
    group.start_all().expect("Failed to start process group");
    
    // Wait for all to complete
    group.wait_all().expect("Failed to wait for process group");
}

#[test]
fn test_run_with_timeout() {
    // Test command that should complete within timeout
    let result = run_with_timeout("echo", &["timeout_test"], Duration::from_secs(5));
    assert!(result.is_ok());
    
    let output = result.unwrap();
    let output_str = String::from_utf8(output).expect("Invalid UTF-8");
    assert!(output_str.contains("timeout_test"));
}

#[test]
fn test_look_path() {
    // Test finding a common executable
    #[cfg(unix)]
    let path_result = look_path("sh");
    #[cfg(windows)]
    let path_result = look_path("cmd");
    
    assert!(path_result.is_ok());
    let path = path_result.unwrap();
    assert!(!path.is_empty());
    
    // Test looking for non-existent executable
    let bad_result = look_path("definitely_not_a_real_command_12345");
    assert!(bad_result.is_err());
}

#[test]
fn test_output_streamer() {
    let cmd = command("echo", &["streamer_test"]);
    let mut streamer = new_output_streamer(cmd);
    
    let lines = Arc::new(Mutex::new(Vec::new()));
    let lines_clone = lines.clone();
    
    streamer.on_line(move |line| {
        let mut l = lines_clone.lock().unwrap();
        l.push(line.to_string());
    });
    
    streamer.start().expect("Failed to start streamer");
    streamer.wait().expect("Failed to wait for streamer");
    
    let captured_lines = lines.lock().unwrap();
    assert!(!captured_lines.is_empty());
    assert!(captured_lines[0].contains("streamer_test"));
}

#[test]
fn test_input_generator() {
    let cmd = command("cat", &[]);
    let mut generator = new_input_generator(cmd);
    
    // Queue input
    generator.write("line1\n").expect("Failed to write input");
    generator.write_after("line2\n", Duration::from_millis(100)).expect("Failed to write delayed input");
    
    generator.start().expect("Failed to start input generator");
    generator.close().expect("Failed to close input generator");
}

#[test]
fn test_process_state_methods() {
    #[cfg(unix)]
    let exit_status = std::os::unix::process::ExitStatusExt::from_raw(0);
    #[cfg(not(unix))]
    let exit_status = std::process::Command::new("true").status().unwrap();
    
    let state = ProcessState {
        exit_status,
        pid: 1234,
        user_time: Duration::from_millis(100),
        system_time: Duration::from_millis(50),
        sys_info: Vec::new(),
    };
    
    assert!(state.exited());
    assert!(state.success());
    assert_eq!(state.exit_code(), 0);
    assert_eq!(state.user_time(), Duration::from_millis(100));
    assert_eq!(state.system_time(), Duration::from_millis(50));
    assert!(state.string().contains("1234"));
}

#[test]
fn test_error_creation_and_methods() {
    let err = Error::new("Test error message");
    assert_eq!(err.error(), "Test error message");
    assert_eq!(err.exit_code(), -1);
    assert_eq!(err.unwrap(), None);
}

#[test]
fn test_command_with_working_directory() {
    let mut cmd = command("pwd", &[]);
    cmd.dir = Some(std::path::PathBuf::from("/tmp"));
    
    let output = cmd.output().expect("Failed to execute pwd");
    let output_str = String::from_utf8(output).expect("Invalid UTF-8");
    assert!(output_str.trim().contains("tmp"));
}

#[test]
fn test_process_handle() {
    let mut cmd = command("sleep", &["1"]);
    cmd.start().expect("Failed to start sleep");
    
    let process = cmd.process().expect("Failed to get process handle");
    assert!(process.pid > 0);
    
    cmd.wait().expect("Failed to wait for sleep");
}

#[test]
#[cfg(unix)]
fn test_process_signal() {
    let mut cmd = command("sleep", &["10"]);
    cmd.start().expect("Failed to start sleep");
    
    let process = cmd.process().expect("Failed to get process handle");
    
    // Send SIGTERM (15)
    let signal_result = process.signal(15);
    assert!(signal_result.is_ok());
    
    // Process should terminate
    let wait_result = cmd.wait();
    // The process was killed by signal, so wait() should return an error
    assert!(wait_result.is_err());
}

#[test]
fn test_process_kill() {
    let mut cmd = command("sleep", &["10"]);
    cmd.start().expect("Failed to start sleep");
    
    let process = cmd.process().expect("Failed to get process handle");
    
    // Kill the process
    process.kill().expect("Failed to kill process");
    
    // Wait should now return an error since process was killed
    let wait_result = cmd.wait();
    assert!(wait_result.is_err());
}

#[test]
fn test_command_with_custom_environment() {
    let mut env = new_environment();
    env.set("CUSTOM_VAR", "custom_value");
    
    let mut cmd = command("env", &[]);
    cmd.env = env.to_env_vec();
    
    let output = cmd.output().expect("Failed to execute env");
    let output_str = String::from_utf8(output).expect("Invalid UTF-8");
    
    // Should contain our custom variable
    assert!(output_str.contains("CUSTOM_VAR=custom_value"));
}

#[test]
fn test_failed_command_execution() {
    // Test command that should fail
    let mut cmd = command("false", &[]);
    let result = cmd.run();
    
    // Should return an error since 'false' exits with code 1
    assert!(result.is_err());
}

#[test]
fn test_nonexistent_command() {
    // Test trying to run a command that doesn't exist
    let mut cmd = command("definitely_not_a_real_command_54321", &[]);
    let result = cmd.start();
    
    // Should fail to start
    assert!(result.is_err());
}
