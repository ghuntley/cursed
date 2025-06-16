/// Runtime information functionality for ChaosMode
/// 
/// Provides system information, version details, and runtime configuration

use crate::stdlib::chaos_mode::error::{ChaosResult, runtime_error};
use crate::stdlib::vibecheck;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::env;

/// Returns the CURSED version string
pub fn version() -> ChaosResult<String> {
    Ok(vibecheck::version())
}

/// Returns the Go architecture target (simulated for CURSED)
pub fn goarch() -> ChaosResult<String> {
    Ok(vibecheck::goarch())
}

/// Returns the Go operating system target (simulated for CURSED)
pub fn goos() -> ChaosResult<String> {
    Ok(vibecheck::goos())
}

/// Returns the compiler that built the binary
pub fn compiler() -> ChaosResult<String> {
    Ok(vibecheck::compiler())
}

/// Gets runtime statistics
pub fn runtime_stats() -> ChaosResult<HashMap<String, Value>> {
    let mut stats = HashMap::new();
    
    // System information
    stats.insert("version".to_string(), json!(version()?));
    stats.insert("compiler".to_string(), json!(compiler()?));
    stats.insert("goos".to_string(), json!(goos()?));
    stats.insert("goarch".to_string(), json!(goarch()?));
    stats.insert("goroot".to_string(), json!(goroot()?));
    
    // Runtime statistics from vibecheck
    let runtime_metrics = vibecheck::get_metrics();
    stats.insert("goroutines".to_string(), json!(runtime_metrics.goroutines));
    stats.insert("threads".to_string(), json!(runtime_metrics.threads));
    stats.insert("memory_mb".to_string(), json!(runtime_metrics.memory_mb));
    stats.insert("gc_cycles".to_string(), json!(runtime_metrics.gc_cycles));
    stats.insert("cpu_usage".to_string(), json!(runtime_metrics.cpu_usage));
    
    // System capabilities
    stats.insert("num_cpu".to_string(), json!(vibecheck::num_cpu()));
    stats.insert("gomaxprocs".to_string(), json!(vibecheck::gomaxprocs(0)));
    
    // Memory statistics
    let mem_stats = vibecheck::read_mem_stats();
    stats.insert("heap_alloc".to_string(), json!(mem_stats.heap_alloc));
    stats.insert("heap_sys".to_string(), json!(mem_stats.heap_sys));
    stats.insert("heap_objects".to_string(), json!(mem_stats.heap_objects));
    
    // GC statistics
    stats.insert("gc_percent".to_string(), json!(vibecheck::get_gc_percent()));
    stats.insert("gc_enabled".to_string(), json!(vibecheck::is_gc_enabled()));
    
    // JIT statistics if available
    let jit_stats = vibecheck::jit_stats();
    stats.insert("jit_compilations".to_string(), json!(jit_stats.compilations));
    stats.insert("jit_execution_time_ms".to_string(), json!(jit_stats.execution_time_ms));
    stats.insert("jit_optimization_level".to_string(), json!(jit_stats.optimization_level));
    
    // Build information
    let build_info = vibecheck::build_info();
    stats.insert("build_version".to_string(), json!(build_info.version));
    stats.insert("build_time".to_string(), json!(build_info.build_time));
    stats.insert("build_commit".to_string(), json!(build_info.commit_hash));
    stats.insert("build_mode".to_string(), json!(build_info.build_mode));
    
    // Runtime features
    let features = vibecheck::runtime_features();
    stats.insert("features".to_string(), json!({
        "gc_enabled": features.gc_enabled,
        "jit_enabled": features.jit_enabled,
        "goroutines_enabled": features.goroutines_enabled,
        "profiling_enabled": features.profiling_enabled,
        "debugging_enabled": features.debugging_enabled,
    }));
    
    // Environment information
    stats.insert("environment".to_string(), json!({
        "pid": std::process::id(),
        "parent_pid": get_parent_pid(),
        "working_directory": env::current_dir().ok().map(|p| p.to_string_lossy().to_string()),
        "executable": env::current_exe().ok().map(|p| p.to_string_lossy().to_string()),
        "command_line": env::args().collect::<Vec<String>>(),
    }));
    
    // Timing information
    let start_time = vibecheck::start_time();
    let uptime = std::time::SystemTime::now()
        .duration_since(start_time)
        .unwrap_or_default();
    
    stats.insert("timing".to_string(), json!({
        "start_time": start_time.duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs(),
        "uptime_seconds": uptime.as_secs(),
        "uptime_nanoseconds": uptime.as_nanos(),
    }));
    
    Ok(stats)
}

/// Gets the current Go root directory (simulated for CURSED)
pub fn goroot() -> ChaosResult<String> {
    // Try to get from environment variable first
    if let Ok(goroot) = env::var("CURSED_ROOT") {
        return Ok(goroot);
    }
    
    if let Ok(goroot) = env::var("GOROOT") {
        return Ok(goroot);
    }
    
    // Default CURSED installation path
    if cfg!(target_os = "windows") {
        Ok("C:\\Program Files\\CURSED".to_string())
    } else {
        Ok("/usr/local/cursed".to_string())
    }
}

/// Get detailed system information
pub fn system_info() -> ChaosResult<HashMap<String, Value>> {
    let mut info = HashMap::new();
    
    // Operating system details
    info.insert("os".to_string(), json!({
        "name": std::env::consts::OS,
        "family": std::env::consts::FAMILY,
        "arch": std::env::consts::ARCH,
        "dll_prefix": std::env::consts::DLL_PREFIX,
        "dll_suffix": std::env::consts::DLL_SUFFIX,
        "exe_suffix": std::env::consts::EXE_SUFFIX,
    }));
    
    // CPU information
    info.insert("cpu".to_string(), json!({
        "logical_cores": vibecheck::num_cpu(),
        "max_procs": vibecheck::gomaxprocs(0),
        "endianness": if cfg!(target_endian = "little") { "little" } else { "big" },
        "pointer_width": std::mem::size_of::<usize>() * 8,
    }));
    
    // Memory layout
    let memory_layout = vibecheck::memory_layout();
    info.insert("memory_layout".to_string(), json!({
        "page_size": memory_layout.page_size,
        "stack_size": memory_layout.stack_size,
        "heap_base": memory_layout.heap_base,
        "heap_size": memory_layout.heap_size,
        "address_space": memory_layout.address_space,
    }));
    
    // Process information
    info.insert("process".to_string(), json!({
        "pid": std::process::id(),
        "parent_pid": get_parent_pid(),
        "thread_id": get_current_thread_id(),
        "user_id": get_user_id(),
        "group_id": get_group_id(),
    }));
    
    Ok(info)
}

/// Get performance characteristics
pub fn performance_info() -> ChaosResult<HashMap<String, Value>> {
    let mut info = HashMap::new();
    
    let runtime_metrics = vibecheck::get_metrics();
    
    info.insert("runtime_performance".to_string(), json!({
        "goroutines": runtime_metrics.goroutines,
        "threads": runtime_metrics.threads,
        "memory_mb": runtime_metrics.memory_mb,
        "gc_cycles": runtime_metrics.gc_cycles,
        "cpu_usage": runtime_metrics.cpu_usage,
    }));
    
    let jit_stats = vibecheck::jit_stats();
    info.insert("jit_performance".to_string(), json!({
        "compilations": jit_stats.compilations,
        "execution_time_ms": jit_stats.execution_time_ms,
        "optimization_level": jit_stats.optimization_level,
        "cache_hits": jit_stats.cache_hits,
        "cache_misses": jit_stats.cache_misses,
    }));
    
    let mem_stats = vibecheck::read_mem_stats();
    info.insert("memory_performance".to_string(), json!({
        "allocations": mem_stats.mallocs,
        "deallocations": mem_stats.frees,
        "heap_alloc": mem_stats.heap_alloc,
        "heap_sys": mem_stats.heap_sys,
        "gc_pause_total_ns": mem_stats.pause_total_ns,
        "gc_count": mem_stats.num_gc,
        "gc_cpu_fraction": mem_stats.gc_cpu_fraction,
    }));
    
    Ok(info)
}

// Helper functions for system information

fn get_parent_pid() -> Option<u32> {
    // Platform-specific implementation would go here
    // For now, return None as it's not easily portable
    None
}

fn get_current_thread_id() -> u64 {
    // Use vibecheck goroutine ID as thread ID
    vibecheck::go_id()
}

fn get_user_id() -> Option<u32> {
    #[cfg(unix)]
    {
        Some(unsafe { libc::getuid() })
    }
    #[cfg(not(unix))]
    {
        None
    }
}

fn get_group_id() -> Option<u32> {
    #[cfg(unix)]
    {
        Some(unsafe { libc::getgid() })
    }
    #[cfg(not(unix))]
    {
        None
    }
}

/// Get environment variables related to CURSED runtime
pub fn cursed_env_vars() -> ChaosResult<HashMap<String, String>> {
    let mut vars = HashMap::new();
    
    let cursed_vars = [
        "CURSED_ROOT",
        "CURSED_DEBUG",
        "CURSED_GC_PERCENT", 
        "CURSED_GOMAXPROCS",
        "CURSED_JIT_LEVEL",
        "CURSED_PROFILE",
        "CURSED_TRACE",
        "GOROOT",
        "GOOS",
        "GOARCH",
        "GOMAXPROCS",
    ];
    
    for var in &cursed_vars {
        if let Ok(value) = env::var(var) {
            vars.insert(var.to_string(), value);
        }
    }
    
    Ok(vars)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let result = version();
        assert!(result.is_ok());
        let version_str = result.unwrap();
        assert!(!version_str.is_empty());
    }

    #[test]
    fn test_goarch() {
        let result = goarch();
        assert!(result.is_ok());
        let arch = result.unwrap();
        assert!(!arch.is_empty());
        // Should be a valid architecture
        assert!(["amd64", "386", "arm", "arm64", "x86_64", "aarch64"].iter().any(|&a| arch.contains(a)));
    }

    #[test]
    fn test_goos() {
        let result = goos();
        assert!(result.is_ok());
        let os = result.unwrap();
        assert!(!os.is_empty());
        // Should be a valid OS
        assert!(["linux", "windows", "darwin", "macos"].iter().any(|&o| os.contains(o)));
    }

    #[test]
    fn test_compiler() {
        let result = compiler();
        assert!(result.is_ok());
        let compiler_str = result.unwrap();
        assert!(!compiler_str.is_empty());
    }

    #[test]
    fn test_runtime_stats() {
        let result = runtime_stats();
        assert!(result.is_ok());
        
        let stats = result.unwrap();
        
        // Check that basic stats are present
        assert!(stats.contains_key("version"));
        assert!(stats.contains_key("compiler"));
        assert!(stats.contains_key("goos"));
        assert!(stats.contains_key("goarch"));
        assert!(stats.contains_key("num_cpu"));
        assert!(stats.contains_key("goroutines"));
        assert!(stats.contains_key("heap_alloc"));
        assert!(stats.contains_key("gc_enabled"));
        assert!(stats.contains_key("features"));
        assert!(stats.contains_key("environment"));
        assert!(stats.contains_key("timing"));
        
        // Verify data types
        assert!(stats["version"].is_string());
        assert!(stats["num_cpu"].is_number());
        assert!(stats["gc_enabled"].is_boolean());
        assert!(stats["features"].is_object());
    }

    #[test]
    fn test_goroot() {
        let result = goroot();
        assert!(result.is_ok());
        let root = result.unwrap();
        assert!(!root.is_empty());
        
        // Should be a valid path
        assert!(root.contains("cursed") || root.contains("CURSED") || root.contains("go"));
    }

    #[test]
    fn test_system_info() {
        let result = system_info();
        assert!(result.is_ok());
        
        let info = result.unwrap();
        
        // Check required sections
        assert!(info.contains_key("os"));
        assert!(info.contains_key("cpu"));
        assert!(info.contains_key("memory_layout"));
        assert!(info.contains_key("process"));
        
        // Check OS info
        let os_info = &info["os"];
        assert!(os_info["name"].is_string());
        assert!(os_info["arch"].is_string());
        
        // Check CPU info
        let cpu_info = &info["cpu"];
        assert!(cpu_info["logical_cores"].is_number());
        assert!(cpu_info["pointer_width"].is_number());
        
        // Check process info
        let process_info = &info["process"];
        assert!(process_info["pid"].is_number());
        assert!(process_info["thread_id"].is_number());
    }

    #[test]
    fn test_performance_info() {
        let result = performance_info();
        assert!(result.is_ok());
        
        let info = result.unwrap();
        
        // Check required sections
        assert!(info.contains_key("runtime_performance"));
        assert!(info.contains_key("jit_performance"));
        assert!(info.contains_key("memory_performance"));
        
        // Check runtime performance
        let runtime_perf = &info["runtime_performance"];
        assert!(runtime_perf["goroutines"].is_number());
        assert!(runtime_perf["threads"].is_number());
        assert!(runtime_perf["memory_mb"].is_number());
        
        // Check JIT performance
        let jit_perf = &info["jit_performance"];
        assert!(jit_perf["compilations"].is_number());
        assert!(jit_perf["optimization_level"].is_number());
        
        // Check memory performance
        let mem_perf = &info["memory_performance"];
        assert!(mem_perf["allocations"].is_number());
        assert!(mem_perf["heap_alloc"].is_number());
        assert!(mem_perf["gc_count"].is_number());
    }

    #[test]
    fn test_cursed_env_vars() {
        let result = cursed_env_vars();
        assert!(result.is_ok());
        
        let vars = result.unwrap();
        // Should return a HashMap (might be empty if no env vars are set)
        assert!(vars.len() >= 0);
        
        // If GOROOT is set, it should be included
        if std::env::var("GOROOT").is_ok() {
            assert!(vars.contains_key("GOROOT"));
        }
    }

    #[test]
    fn test_get_current_thread_id() {
        let thread_id = get_current_thread_id();
        assert!(thread_id > 0);
    }
}
