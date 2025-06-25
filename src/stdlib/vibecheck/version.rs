/// Version and Runtime Information for vibecheck
/// 
/// Provides version information, compiler details, and platform identification

use crate::error::CursedError;
use std::env;

/// CURSED version information
const CURSED_VERSION: &str = env!("CARGO_PKG_VERSION");
const CURSED_COMPILER: &str = "cursed-llvm";

/// Get CURSED version information
pub fn version() -> String {
    format!("cursed version {}", CURSED_VERSION)
}

/// Get compiler information
pub fn compiler() -> String {
    CURSED_COMPILER.to_string()
}

/// Get architecture equivalent to GOARCH
pub fn goarch() -> String {
    match env::consts::ARCH {
        "x86_64" => "amd64".to_string(),
        "x86" => "386".to_string(),
        "aarch64" => "arm64".to_string(),
        "arm" => "arm".to_string(),
        "powerpc64" => "ppc64".to_string(),
        "powerpc64le" => "ppc64le".to_string(),
        "mips" => "mips".to_string(),
        "mips64" => "mips64".to_string(),
        "s390x" => "s390x".to_string(),
        "riscv64" => "riscv64".to_string(),
        other => other.to_string(),
    }
}

/// Get operating system equivalent to GOOS
pub fn goos() -> String {
    match env::consts::OS {
        "linux" => "linux".to_string(),
        "macos" => "darwin".to_string(),
        "windows" => "windows".to_string(),
        "freebsd" => "freebsd".to_string(),
        "netbsd" => "netbsd".to_string(),
        "openbsd" => "openbsd".to_string(),
        "dragonfly" => "dragonfly".to_string(),
        "android" => "android".to_string(),
        "ios" => "ios".to_string(),
        "solaris" => "solaris".to_string(),
        other => other.to_string(),
    }
}

/// Stack frame information
#[derive(Debug, Clone)]
pub struct StackFrame {
    /// Program counter
    pub pc: usize,
    /// Function name
    pub function: String,
    /// Source file name
    pub file: String,
    /// Line number
    pub line: u32,
}

/// Get caller frame information
/// Skip n frames in the call stack
pub fn caller(skip: i32) -> crate::error::Result<()> {
    // In a full implementation, this would use backtrace or similar
    // to walk the call stack and get symbol information
    
    // For now, provide a simplified implementation
    if skip < 0 {
        return Err(CursedError::Runtime("Skip count cannot be negative".to_string()));
    }
    
    // Simulate stack walking
    let frames = get_call_stack()?;
    
    let frame_index = (skip as usize) + 1; // +1 to skip caller() itself
    if frame_index < frames.len() {
        let frame = &frames[frame_index];
        Ok((frame.pc, frame.file.clone(), frame.line as i32, true))
    } else {
        // Not enough frames in stack
        Ok((0, String::new(), 0, false))
    }
}

/// Get function information for a program counter
pub fn func_for_pc(pc: usize) -> crate::error::Result<()> {
    // In a full implementation, this would resolve symbols
    // For now, provide a placeholder
    Ok(FuncInfo {
        name: format!("func_at_0x{:x}", pc),
        entry: pc,
        file: "unknown.csd".to_string(),
        line: 0,
    })
}

/// Function information structure
#[derive(Debug, Clone)]
pub struct FuncInfo {
    pub name: String,
    pub entry: usize,
    pub file: String,
    pub line: u32,
}

impl FuncInfo {
    /// Get function name
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Get function entry point
    pub fn entry(&self) -> usize {
        self.entry
    }
    
    /// Get file and line for a program counter within this function
    pub fn file_line(&self, _pc: usize) -> (String, u32) {
        // In a full implementation, this would map PC to source location
        (self.file.clone(), self.line)
    }
}

/// Get simplified call stack
fn get_call_stack() -> crate::error::Result<()> {
    // This is a simplified implementation
    // A full implementation would use backtrace crate or platform-specific APIs
    
    let mut frames = Vec::new();
    
    // Add some example frames
    frames.push(StackFrame {
        pc: 0x1000,
        function: "main".to_string(),
        file: "main.csd".to_string(),
        line: 10,
    });
    
    frames.push(StackFrame {
        pc: 0x2000,
        function: "cursed_function".to_string(),
        file: "module.csd".to_string(),
        line: 25,
    });
    
    frames.push(StackFrame {
        pc: 0x3000,
        function: "runtime.call".to_string(),
        file: "runtime.csd".to_string(),
        line: 100,
    });
    
    Ok(frames)
}

/// Build information
#[derive(Debug, Clone)]
pub struct BuildInfo {
    pub version: String,
    pub compiler: String,
    pub architecture: String,
    pub os: String,
    pub build_time: String,
    pub commit_hash: String,
    pub rust_version: String,
    pub llvm_version: String,
}

/// Get comprehensive build information
pub fn build_info() -> BuildInfo {
    BuildInfo {
        version: version(),
        compiler: compiler(),
        architecture: goarch(),
        os: goos(),
        build_time: option_env!("VERGEN_BUILD_TIMESTAMP").unwrap_or("unknown").to_string(),
        commit_hash: option_env!("VERGEN_GIT_SHA").unwrap_or("unknown").to_string(),
        rust_version: option_env!("VERGEN_RUSTC_SEMVER").unwrap_or("unknown").to_string(),
        llvm_version: get_llvm_version(),
    }
}

/// Get LLVM version if available
fn get_llvm_version() -> String {
    // This would query the LLVM version from the linked LLVM library
    // For now, return a placeholder
    "17.0.0".to_string()
}

/// Runtime feature flags
#[derive(Debug, Clone)]
pub struct RuntimeFeatures {
    pub gc_enabled: bool,
    pub jit_enabled: bool,
    pub goroutines_enabled: bool,
    pub channels_enabled: bool,
    pub async_enabled: bool,
    pub optimization_enabled: bool,
    pub debug_info_enabled: bool,
}

/// Get runtime feature flags
pub fn runtime_features() -> RuntimeFeatures {
    RuntimeFeatures {
        gc_enabled: cfg!(feature = "gc"),
        jit_enabled: cfg!(feature = "jit"),
        goroutines_enabled: cfg!(feature = "goroutines"),
        channels_enabled: cfg!(feature = "channels"),
        async_enabled: cfg!(feature = "async"),
        optimization_enabled: !cfg!(debug_assertions),
        debug_info_enabled: cfg!(debug_assertions),
    }
}

/// Memory layout information
#[derive(Debug, Clone)]
pub struct MemoryLayout {
    pub pointer_size: usize,
    pub page_size: usize,
    pub endianness: String,
    pub alignment: usize,
}

/// Get memory layout information
pub fn memory_layout() -> MemoryLayout {
    MemoryLayout {
        pointer_size: std::mem::size_of::<*const u8>(),
        page_size: get_page_size(),
        endianness: if cfg!(target_endian = "little") {
            "little".to_string()
        } else {
            "big".to_string()
        },
        alignment: std::mem::align_of::<*const u8>(),
    }
}

/// Get system page size
fn get_page_size() -> usize {
    #[cfg(unix)]
    {
        unsafe { libc::sysconf(libc::_SC_PAGESIZE) as usize }
    }
    
    #[cfg(windows)]
    {
        use std::mem;
        use winapi::um::sysinfoapi::{GetSystemInfo, SYSTEM_INFO};
        
        let mut info: SYSTEM_INFO = unsafe { mem::zeroed() };
        unsafe { GetSystemInfo(&mut info) };
        info.dwPageSize as usize
    }
    
    #[cfg(not(any(unix, windows)))]
    {
        4096 // Default page size
    }
}

