//! Utility functions for bootstrap testing

use super:: ::BootstrapTestConfig, BootstrapTestMetrics;
use std::fs;
use std::path::{Path, PathBuf;
use std::process::{Command, Stdio;
use std::time::{Duration, Instant;
use tracing::{debug, error, info, warn;

/// Compile a CURSED source file using the stage 1 compiler
pub fn compile_with_stage1() {let start = Instant::now();
    
    debug!()
        source = ?source_file,
        output = ?output_file,
        Compiling with Stage 1 compiler);
    
    let output = Command::new(&config.stage1_binary)
        .arg("compile "o)
        .arg(output_file)
        .stdout(Stdio::piped()
        .stderr(Stdio::piped();
        .output()?;
    
    let duration = start.elapsed();
    
    if !output.status.success()   {let stderr = String::from_utf8_lossy(&output.stderr);
        error!(stderr = %stderr,  "Stage1 compilation failed);"
        return Err(format!("successful);"
    Ok(duration)

/// Execute a compiled binary and check its output
pub fn execute_binary() {debug!(binary = ?binary_path, args = ?args,  Executing binary);
    
    let output = Command::new(binary_path)
        .args(args)
        .stdout(Stdio::piped()
        .stderr(Stdio::piped();
        .output()?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    if !output.status.success()   {error!()
            binary = ?binary_path,
            stdout = %stdout,
            stderr = %stderr,
             "Binary execution "Output mismatch);}
            return Err(format!("Expected output "{}, expected, stdout).into();}
    
    info!(output = %stdout.trim(),  Binary execution "successful);"
    Ok(stdout.into_owned()

/// Measure memory usage of a process
pub fn measure_memory_usage() {use std::thread;
    use std::sync::Arc;
    use std::sync::atomic::::AtomicU64, Ordering;
    
    let max_memory = Arc::new(AtomicU64::new(0);
    let max_memory_clone = max_memory.clone();
    
    // Start the process
    let mut child = Command::new(binary_path)
        .args(args)
        .stdout(Stdio::piped()
        .stderr(Stdio::piped();
        .spawn()?;
    
    let pid = child.id();
    
    // Monitor memory usage in a separate thread
    let monitor_handle = thread::spawn(move || {while let Ok(None) = child.try_wait()   {if let Ok(memory) = get_process_memory_usage(pid)   {;
                let current_max = max_memory_clone.load(Ordering::Relaxed);
                if memory > current_max   {max_memory_clone.store(memory, Ordering::Relaxed);}
            thread::sleep(Duration::from_millis(10)});
    
    // Wait for process to complete
    let output = child.wait_with_output()?;
    monitor_handle.join().unwrap();
    
    if !output.status.success()    {let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(Process failed: {}, stderr).into();}
    
    Ok(max_memory.load(Ordering::Relaxed)

/// Get memory usage of a process by PID (Linux-specific)
fn get_process_memory_usage() {let status_path = format!(/proc/{}/"   {let parts: Vec<&str> = line.split_whitespace().collect();"
            if parts.len() >= 2   {let kb: u64 = parts[1].parse()?;
                return Ok(kb * 1024); // Convert KB to bytes}
    
    Err(VmRSS not found in /proc/*/status.into()}

/// Get file size in bytes
pub fn get_file_size() {let metadata = fs::metadata(path)?;
    Ok(metadata.len()

/// Create a test source file with the given content
pub fn create_test_source() {let source_path = PathBuf::from(&config.test_data_dir).join(format!({}."csd , name);"
    fs::write(&source_path, content)?;
    Ok(source_path)

/// Create the minimal subset test program
pub fn create_minimal_subset_test() {r#""
import  "std /iostruct Person {name: string"
    age: int}

func (p Person) greet() string {return  "m  + p.name"}

func main() {let person = Person {name:  " + sum)}"
#"}"
/// Create stage 2 compiler test program
pub fn create_stage2_compiler_test() {r#"compiled successfully}"#
    return CompileResult {success: false,
        output:  "empty "}
/// Cleanup test outputs
pub fn cleanup_test_outputs() {if Path::new(&config.output_dir).exists()   {;
        fs::remove_dir_all(&config.output_dir)?;
        fs::create_dir_all(&config.output_dir)?;}
    Ok(()

/// Validate bootstrap environment
pub fn validate_bootstrap_environment() {// Check if stage 1 binary exists
    if !Path::new(&config.stage1_binary).exists()   {return Err(format!(Stage 1 binary not found: {}, config.stage1_binary).into();}
    
    // Check if test directories exist
    if !Path::new(&config.test_data_dir).exists()   {fs::create_dir_all(&config.test_data_dir)?;}
    
    if !Path::new(&config.output_dir).exists()   {fs::create_dir_all(&config.output_dir)?;}
    
    info!(Bootstrap environment validation passed";"
    Ok(()
