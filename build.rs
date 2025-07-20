// Enhanced Build script for the CURSED programming language
// Supports robust cross-compilation across platforms and architectures

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::collections::HashMap;

fn main() {
    println!("cargo:rerun-if-changed=src/execution/runtime_functions.rs");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=runtime/");
    println!("cargo:rerun-if-env-changed=TARGET");
    println!("cargo:rerun-if-env-changed=HOST");
    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_TRIPLE");
    
    // Cross-compilation detection and setup
    let cross_compilation_config = detect_cross_compilation();
    setup_cross_compilation_environment(&cross_compilation_config);
    
    // Build runtime library with proper cross-compilation support
    build_runtime_library(&cross_compilation_config);
    
    // Link with system libraries using target-aware resolution
    link_system_libraries(&cross_compilation_config);
}

#[derive(Debug, Clone)]
struct CrossCompilationConfig {
    target_triple: String,
    host_triple: String,
    target_arch: String,
    target_os: String,
    target_env: String,
    is_cross_compiling: bool,
    cargo_metadata: CargoMetadata,
}

#[derive(Debug, Clone)]
struct CargoMetadata {
    target_dir: PathBuf,
    out_dir: PathBuf,
    profile: String,
    manifest_dir: PathBuf,
}

fn detect_cross_compilation() -> CrossCompilationConfig {
    let target_triple = env::var("TARGET")
        .or_else(|_| env::var("CARGO_CFG_TARGET_TRIPLE"))
        .unwrap_or_else(|_| get_host_triple());
    
    let host_triple = env::var("HOST")
        .or_else(|_| env::var("CARGO_CFG_HOST_TRIPLE"))
        .unwrap_or_else(|_| get_host_triple());
    
    let is_cross_compiling = target_triple != host_triple;
    
    // Parse target triple components
    let target_parts: Vec<&str> = target_triple.split('-').collect();
    let target_arch = target_parts.get(0).unwrap_or(&"unknown").to_string();
    let target_os = if target_triple.contains("darwin") {
        "macos".to_string()
    } else if target_triple.contains("linux") {
        "linux".to_string()
    } else if target_triple.contains("windows") {
        "windows".to_string()
    } else if target_triple.contains("wasm") {
        "wasm".to_string()
    } else {
        target_parts.get(2).unwrap_or(&"unknown").to_string()
    };
    
    let target_env = if target_triple.contains("musl") {
        "musl".to_string()
    } else if target_triple.contains("gnu") {
        "gnu".to_string()
    } else if target_triple.contains("msvc") {
        "msvc".to_string()
    } else {
        "".to_string()
    };
    
    let cargo_metadata = CargoMetadata {
        target_dir: PathBuf::from(env::var("CARGO_TARGET_DIR")
            .unwrap_or_else(|_| "target".to_string())),
        out_dir: PathBuf::from(env::var("OUT_DIR").unwrap()),
        profile: env::var("PROFILE").unwrap_or_else(|_| "debug".to_string()),
        manifest_dir: PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()),
    };
    
    println!("cargo:warning=Cross-compilation config: target={}, host={}, cross={}", 
             target_triple, host_triple, is_cross_compiling);
    
    CrossCompilationConfig {
        target_triple,
        host_triple,
        target_arch,
        target_os,
        target_env,
        is_cross_compiling,
        cargo_metadata,
    }
}

fn get_host_triple() -> String {
    // Fallback host triple detection
    format!("{}-{}-{}",
        env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_else(|_| std::env::consts::ARCH.to_string()),
        env::var("CARGO_CFG_TARGET_VENDOR").unwrap_or_else(|_| "unknown".to_string()),
        env::var("CARGO_CFG_TARGET_OS").unwrap_or_else(|_| std::env::consts::OS.to_string())
    )
}

fn setup_cross_compilation_environment(config: &CrossCompilationConfig) {
    if !config.is_cross_compiling {
        return;
    }
    
    println!("cargo:warning=Setting up cross-compilation environment for {}", config.target_triple);
    
    // Propagate essential environment variables for cross-compilation
    let env_vars_to_propagate = [
        "RUSTFLAGS",
        "CARGO_TARGET_DIR",
        "CC", "CXX", "AR", "RANLIB", "STRIP",
        "CFLAGS", "CXXFLAGS", "LDFLAGS",
        "PKG_CONFIG_PATH", "PKG_CONFIG_LIBDIR",
        "LIBRARY_PATH", "LD_LIBRARY_PATH",
        "MACOSX_DEPLOYMENT_TARGET",
        "SDKROOT",
    ];
    
    for var in &env_vars_to_propagate {
        if let Ok(value) = env::var(var) {
            println!("cargo:rustc-env={}={}", var, value);
        }
    }
    
    // Set up target-specific configurations
    setup_target_specific_config(config);
}

fn setup_target_specific_config(config: &CrossCompilationConfig) {
    match (config.target_arch.as_str(), config.target_os.as_str()) {
        ("aarch64", "macos") => {
            // Apple Silicon macOS
            println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=11.0");
            if env::var("SDKROOT").is_err() {
                // Try to find macOS SDK
                let sdk_paths = [
                    "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk",
                    "/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk",
                ];
                for sdk_path in &sdk_paths {
                    if Path::new(sdk_path).exists() {
                        println!("cargo:rustc-env=SDKROOT={}", sdk_path);
                        break;
                    }
                }
            }
        },
        ("x86_64", "macos") => {
            // Intel macOS
            println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.9");
        },
        ("aarch64", "linux") => {
            // ARM64 Linux
            setup_linux_cross_compilation("aarch64-linux-gnu", config);
        },
        ("x86_64", "linux") => {
            // x86_64 Linux
            setup_linux_cross_compilation("x86_64-linux-gnu", config);
        },
        ("x86_64", "windows") => {
            // Windows x86_64
            setup_windows_cross_compilation(config);
        },
        ("wasm32", _) => {
            // WebAssembly
            setup_wasm_cross_compilation(config);
        },
        _ => {
            println!("cargo:warning=Unknown target architecture: {}-{}", 
                     config.target_arch, config.target_os);
        }
    }
}

fn setup_linux_cross_compilation(gnu_triple: &str, _config: &CrossCompilationConfig) {
    // Set up cross-compilation toolchain if not already configured
    if env::var("CC").is_err() {
        println!("cargo:rustc-env=CC={}-gcc", gnu_triple);
    }
    if env::var("CXX").is_err() {
        println!("cargo:rustc-env=CXX={}-g++", gnu_triple);
    }
    if env::var("AR").is_err() {
        println!("cargo:rustc-env=AR={}-ar", gnu_triple);
    }
    if env::var("RANLIB").is_err() {
        println!("cargo:rustc-env=RANLIB={}-ranlib", gnu_triple);
    }
    
    // Set up sysroot for cross-compilation
    let possible_sysroots = [
        format!("/usr/{}", gnu_triple),
        format!("/usr/lib/{}", gnu_triple),
        format!("/opt/cross/{}", gnu_triple),
    ];
    
    for sysroot in &possible_sysroots {
        if Path::new(sysroot).exists() {
            println!("cargo:rustc-link-search=native={}/lib", sysroot);
            break;
        }
    }
}

fn setup_windows_cross_compilation(_config: &CrossCompilationConfig) {
    // Set up MinGW cross-compilation
    if env::var("CC").is_err() {
        println!("cargo:rustc-env=CC=x86_64-w64-mingw32-gcc");
    }
    if env::var("CXX").is_err() {
        println!("cargo:rustc-env=CXX=x86_64-w64-mingw32-g++");
    }
    if env::var("AR").is_err() {
        println!("cargo:rustc-env=AR=x86_64-w64-mingw32-ar");
    }
    
    // Windows-specific linking
    println!("cargo:rustc-link-lib=ws2_32");
    println!("cargo:rustc-link-lib=advapi32");
    println!("cargo:rustc-link-lib=userenv");
}

fn setup_wasm_cross_compilation(_config: &CrossCompilationConfig) {
    // WebAssembly-specific configuration
    println!("cargo:rustc-link-arg=--no-entry");
    println!("cargo:rustc-link-arg=--export-dynamic");
    
    // Disable features not supported in WebAssembly
    println!("cargo:rustc-cfg=feature=\"wasm\"");
}

fn build_runtime_library(config: &CrossCompilationConfig) {
    println!("cargo:warning=Building CURSED runtime library for {}", config.target_triple);
    
    // Create runtime build directory using cargo metadata
    let runtime_dir = config.cargo_metadata.out_dir.join("cursed_runtime");
    fs::create_dir_all(&runtime_dir).unwrap();
    
    // Create target-aware runtime Cargo.toml
    let runtime_cargo_toml = create_runtime_cargo_toml(config);
    fs::write(runtime_dir.join("Cargo.toml"), runtime_cargo_toml).unwrap();
    
    // Create runtime library source
    create_runtime_library_source(&runtime_dir, config);
    
    // Build runtime with proper cross-compilation support
    build_runtime_with_cargo(&runtime_dir, config);
    
    // Copy and link the built library
    link_runtime_library(&runtime_dir, config);
}

fn create_runtime_cargo_toml(config: &CrossCompilationConfig) -> String {
    let mut cargo_toml = String::from(r#"[package]
name = "cursed_runtime"
version = "0.1.0"
edition = "2021"

[workspace]
# This is a standalone package

[lib]
name = "cursed_runtime"
crate-type = ["staticlib"]

[dependencies]
"#);
    
    // Add dependencies based on target platform
    if config.target_os != "wasm" {
        cargo_toml.push_str(r#"regex = "1.10"
base64 = "0.22"
libc = "0.2"
hex = "0.4"
rand = "0.8"
sha2 = "0.10"
blake3 = "1.5"
sha3 = "0.10"
hmac = "0.12"
pbkdf2 = "0.12"
scrypt = "0.11"
argon2 = "0.5"
bcrypt = "0.15"
ed25519-dalek = "2.0"
aes = "0.8"
aes-gcm = "0.10"
subtle = "2.5.0"
base64ct = "=1.6.0"
chrono = { version = "0.4", features = ["serde"] }
lazy_static = "1.4"
"#);
    } else {
        // WebAssembly-compatible subset
        cargo_toml.push_str(r#"base64 = "0.22"
hex = "0.4"
sha2 = "0.10"
blake3 = "1.5"
subtle = "2.5.0"
"#);
    }
    
    cargo_toml
}

fn create_runtime_library_source(runtime_dir: &Path, config: &CrossCompilationConfig) {
    fs::create_dir_all(runtime_dir.join("src")).unwrap();
    
    let runtime_lib_rs = if config.target_os == "wasm" {
        create_wasm_runtime_source()
    } else {
        create_native_runtime_source()
    };
    
    // Read and filter the original runtime functions
    let main_runtime_path = Path::new("src/execution/runtime_functions.rs");
    if main_runtime_path.exists() {
        let runtime_content = fs::read_to_string(main_runtime_path).unwrap();
        let filtered_content = filter_runtime_content(&runtime_content, config);
        let full_runtime_lib = format!("{}\n{}", runtime_lib_rs, filtered_content);
        fs::write(runtime_dir.join("src/lib.rs"), full_runtime_lib).unwrap();
    } else {
        fs::write(runtime_dir.join("src/lib.rs"), runtime_lib_rs).unwrap();
    }
}

fn create_native_runtime_source() -> String {
    r#"//! CURSED Runtime Library
//! 
//! This library provides external C functions for the CURSED standard library.
//! These functions are compiled into a static library and linked with CURSED executables.

use std::ffi::{CStr, CString, c_void};
use std::os::raw::c_char;
use std::ptr;
use std::fs::{self, OpenOptions, File};
use std::io::{self, Write, Read, BufRead, BufReader, BufWriter, Seek, SeekFrom};
use std::env;
use std::collections::{HashMap, HashSet};
use std::slice;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, atomic::{AtomicI32, Ordering}};
use std::time::{SystemTime, UNIX_EPOCH, Duration, Instant};
use std::thread;
use chrono::{DateTime, Utc, Local, TimeZone, Datelike, Timelike, Weekday, NaiveDate, NaiveDateTime};
use regex::Regex;
use base64::{Engine as _, engine::general_purpose};
use std::net::{TcpStream, TcpListener, UdpSocket, ToSocketAddrs, SocketAddr, IpAddr, Ipv4Addr, Ipv6Addr};
use std::io::{Read as IoRead, Write as IoWrite};
use std::sync::Arc;
use sha2::{Sha256, Sha512, Digest};
use sha3::{Sha3_256, Sha3_512};
use blake3::Hasher as Blake3Hasher;
use rand::{Rng, RngCore};
use rand::distributions::Alphanumeric;
use aes_gcm::{Aes128Gcm, KeyInit, aead::{Aead, AeadCore, OsRng}};
use hmac::{Hmac, Mac};
use pbkdf2::pbkdf2_hmac;
use scrypt::{scrypt, Params};
use argon2::{Argon2, password_hash::{PasswordHasher, PasswordVerifier, SaltString}};
use bcrypt::{hash, verify};
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer};
use subtle::ConstantTimeEq;

// Global file handle management for stream I/O
lazy_static::lazy_static! {
    static ref FILE_HANDLES: Mutex<HashMap<i32, File>> = Mutex::new(HashMap::new());
    static ref BUFFER_HANDLES: Mutex<HashMap<i32, Vec<u8>>> = Mutex::new(HashMap::new());
    static ref NEXT_HANDLE_ID: AtomicI32 = AtomicI32::new(1);
    
    // Network socket handle management
    static ref TCP_SOCKETS: Mutex<HashMap<i32, TcpStream>> = Mutex::new(HashMap::new());
    static ref TCP_LISTENERS: Mutex<HashMap<i32, TcpListener>> = Mutex::new(HashMap::new());
    static ref UDP_SOCKETS: Mutex<HashMap<i32, UdpSocket>> = Mutex::new(HashMap::new());
    static ref NEXT_SOCKET_ID: AtomicI32 = AtomicI32::new(1000);
}

fn get_next_handle() -> i32 {
    NEXT_HANDLE_ID.fetch_add(1, Ordering::SeqCst)
}

fn get_next_socket_id() -> i32 {
    NEXT_SOCKET_ID.fetch_add(1, Ordering::SeqCst)
}

// Export all runtime functions with C linkage
"#.to_string()
}

fn create_wasm_runtime_source() -> String {
    r#"//! CURSED Runtime Library - WebAssembly
//! 
//! This library provides WebAssembly-compatible runtime functions.

use std::ffi::{CStr, CString, c_void};
use std::os::raw::c_char;
use std::ptr;
use std::collections::HashMap;
use std::sync::{Mutex, atomic::{AtomicI32, Ordering}};
use base64::{Engine as _, engine::general_purpose};
use sha2::{Sha256, Digest};
use blake3::Hasher as Blake3Hasher;
use hex;
use subtle::ConstantTimeEq;

// Simplified globals for WebAssembly
static NEXT_HANDLE_ID: AtomicI32 = AtomicI32::new(1);

fn get_next_handle() -> i32 {
    NEXT_HANDLE_ID.fetch_add(1, Ordering::SeqCst)
}

// Export WebAssembly-compatible runtime functions
"#.to_string()
}

fn build_runtime_with_cargo(runtime_dir: &Path, config: &CrossCompilationConfig) {
    let cargo_path = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let mut build_cmd = Command::new(&cargo_path);
    
    build_cmd
        .arg("build")
        .arg("--release")
        .arg("--lib")
        .current_dir(&runtime_dir);
    
    // Set target directory to avoid conflicts
    let target_dir = runtime_dir.join("target");
    build_cmd.env("CARGO_TARGET_DIR", &target_dir);
    
    // Cross-compilation setup
    if config.is_cross_compiling {
        build_cmd.arg("--target").arg(&config.target_triple);
        
        // Pass through cross-compilation environment variables
        propagate_cross_compilation_env(&mut build_cmd, config);
    } else {
        // For same-platform builds, clean environment to avoid confusion
        clean_build_environment(&mut build_cmd);
    }
    
    // Execute build
    let output = build_cmd.output().expect("Failed to build runtime library");
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        panic!("Failed to build runtime library:\nSTDOUT:\n{}\nSTDERR:\n{}", stdout, stderr);
    }
    
    println!("cargo:warning=Successfully built runtime library for {}", config.target_triple);
}

fn propagate_cross_compilation_env(build_cmd: &mut Command, config: &CrossCompilationConfig) {
    // Propagate essential cross-compilation environment variables
    let env_vars = [
        "CC", "CXX", "AR", "RANLIB", "STRIP",
        "CFLAGS", "CXXFLAGS", "LDFLAGS",
        "PKG_CONFIG_PATH", "PKG_CONFIG_LIBDIR",
        "LIBRARY_PATH", "LD_LIBRARY_PATH",
        "MACOSX_DEPLOYMENT_TARGET", "SDKROOT",
        "RUSTFLAGS",
    ];
    
    for var in &env_vars {
        if let Ok(value) = env::var(var) {
            build_cmd.env(var, value);
        }
    }
    
    // Set target-specific environment variables
    match config.target_os.as_str() {
        "macos" => {
            if let Ok(sdk) = env::var("SDKROOT") {
                build_cmd.env("SDKROOT", sdk);
            }
            if let Ok(target) = env::var("MACOSX_DEPLOYMENT_TARGET") {
                build_cmd.env("MACOSX_DEPLOYMENT_TARGET", target);
            }
        },
        "linux" => {
            // Linux-specific cross-compilation setup
            if config.target_env == "musl" {
                build_cmd.env("RUSTFLAGS", "--cfg target_env=\"musl\"");
            }
        },
        _ => {}
    }
}

fn clean_build_environment(build_cmd: &mut Command) {
    // Remove environment variables that could interfere with same-platform builds
    let vars_to_remove = [
        "CC", "CXX", "AR", "RANLIB",
        "MACOSX_DEPLOYMENT_TARGET",
    ];
    
    for var in &vars_to_remove {
        build_cmd.env_remove(var);
    }
}

fn link_runtime_library(runtime_dir: &Path, config: &CrossCompilationConfig) {
    let lib_name = "libcursed_runtime.a";
    
    // Use cargo metadata to find the correct artifact location
    let target_dir = runtime_dir.join("target");
    let artifact_path = if config.is_cross_compiling {
        target_dir.join(&config.target_triple).join("release").join(lib_name)
    } else {
        target_dir.join("release").join(lib_name)
    };
    
    if !artifact_path.exists() {
        // Try debug build as fallback
        let debug_artifact_path = if config.is_cross_compiling {
            target_dir.join(&config.target_triple).join("debug").join(lib_name)
        } else {
            target_dir.join("debug").join(lib_name)
        };
        
        if debug_artifact_path.exists() {
            link_library_artifact(&debug_artifact_path, config);
        } else {
            println!("cargo:warning=Runtime library not found at {} or {}", 
                     artifact_path.display(), debug_artifact_path.display());
        }
    } else {
        link_library_artifact(&artifact_path, config);
    }
}

fn link_library_artifact(lib_path: &Path, config: &CrossCompilationConfig) {
    let dest_lib_path = config.cargo_metadata.out_dir.join("libcursed_runtime.a");
    fs::copy(lib_path, &dest_lib_path).unwrap();
    
    println!("cargo:rustc-link-search=native={}", config.cargo_metadata.out_dir.display());
    println!("cargo:rustc-link-lib=static=cursed_runtime");
    println!("cargo:warning=Successfully linked runtime library from {}", lib_path.display());
}

fn filter_runtime_content(content: &str, config: &CrossCompilationConfig) -> String {
    let mut result = String::new();
    let excluded_functions = vec![
        "initialize_runtime_functions",
        "get_minimal_result",
    ];
    
    // Additional exclusions for WebAssembly
    let wasm_excluded_functions = if config.target_os == "wasm" {
        vec!["file_", "network_", "thread_", "process_"]
    } else {
        vec![]
    };
    
    let mut skip_until_next_function = false;
    
    for line in content.lines() {
        // Skip all use statements and initial comments
        if line.starts_with("use ") || line.starts_with("//!") {
            continue;
        }
        
        // Check for function start
        if line.contains("pub fn ") && !line.contains("extern \"C\"") {
            let should_skip = excluded_functions.iter().any(|func| line.contains(func))
                || (config.target_os == "wasm" && wasm_excluded_functions.iter().any(|func| line.contains(func)));
            skip_until_next_function = should_skip || line.contains("CursedError");
            if skip_until_next_function {
                continue;
            }
        }
        
        // Check for extern C function start
        if line.contains("#[no_mangle]") || line.contains("pub extern \"C\" fn") {
            // For WebAssembly, skip functions that use unsupported features
            if config.target_os == "wasm" {
                skip_until_next_function = wasm_excluded_functions.iter().any(|func| line.contains(func));
            } else {
                skip_until_next_function = false; // Always include extern C functions for native targets
            }
        }
        
        // Skip lines that reference CursedError or unsupported features
        if line.contains("CursedError") || line.contains("crate::error::") {
            skip_until_next_function = true;
            continue;
        }
        
        if !skip_until_next_function {
            result.push_str(&fix_line(line));
            result.push('\n');
        }
    }
    
    result
}

fn fix_line(line: &str) -> String {
    let mut fixed_line = line.to_string();
    if fixed_line.contains("libc::c_void") {
        fixed_line = fixed_line.replace("libc::c_void", "c_void");
    }
    fixed_line
}

fn link_system_libraries(config: &CrossCompilationConfig) {
    println!("cargo:warning=Linking system libraries for {} on {}", 
             config.target_arch, config.target_os);
    
    // Target-specific library linking
    match config.target_os.as_str() {
        "macos" => link_macos_libraries(config),
        "linux" => link_linux_libraries(config),
        "windows" => link_windows_libraries(config),
        "wasm" => link_wasm_libraries(config),
        _ => link_fallback_libraries(config),
    }
    
    // Build and link additional runtime libraries
    build_and_link_runtime_libraries(config);
}

fn link_macos_libraries(config: &CrossCompilationConfig) {
    // Core libraries
    println!("cargo:rustc-link-lib=sqlite3");
    println!("cargo:rustc-link-lib=ffi");
    println!("cargo:rustc-link-lib=xml2");
    
    // macOS-specific paths
    let library_paths = match config.target_arch.as_str() {
        "aarch64" => vec![
            "/opt/homebrew/lib",
            "/usr/local/lib",
            "/usr/lib",
            "/System/Library/Frameworks",
        ],
        "x86_64" => vec![
            "/usr/local/lib",
            "/opt/homebrew/lib", // Fallback for Rosetta
            "/usr/lib",
            "/System/Library/Frameworks",
        ],
        _ => vec!["/usr/lib"],
    };
    
    for path in library_paths {
        if Path::new(path).exists() {
            println!("cargo:rustc-link-search=native={}", path);
        }
    }
    
    // Framework search paths
    println!("cargo:rustc-link-search=framework=/System/Library/Frameworks");
    println!("cargo:rustc-link-search=framework=/Library/Frameworks");
    
    // Set deployment target based on architecture
    if config.target_arch == "aarch64" {
        println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=11.0");
    } else {
        println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.9");
    }
}

fn link_linux_libraries(config: &CrossCompilationConfig) {
    // Core libraries
    println!("cargo:rustc-link-lib=sqlite3");
    println!("cargo:rustc-link-lib=ffi");
    println!("cargo:rustc-link-lib=xml2");
    
    // Linux-specific libraries
    if config.target_env != "musl" {
        println!("cargo:rustc-link-lib=dl");
        println!("cargo:rustc-link-lib=pthread");
    }
    
    // Architecture-specific library paths
    let lib_paths = match config.target_arch.as_str() {
        "aarch64" => vec![
            "/usr/lib/aarch64-linux-gnu",
            "/lib/aarch64-linux-gnu",
            "/usr/local/lib",
        ],
        "x86_64" => vec![
            "/usr/lib/x86_64-linux-gnu",
            "/lib/x86_64-linux-gnu",
            "/usr/local/lib",
        ],
        _ => vec!["/usr/lib", "/lib"],
    };
    
    for path in lib_paths {
        if Path::new(path).exists() {
            println!("cargo:rustc-link-search=native={}", path);
        }
    }
    
    // Add pkg-config paths if available
    add_pkg_config_paths();
}

fn link_windows_libraries(config: &CrossCompilationConfig) {
    // Windows-specific libraries
    println!("cargo:rustc-link-lib=ws2_32");
    println!("cargo:rustc-link-lib=advapi32");
    println!("cargo:rustc-link-lib=userenv");
    println!("cargo:rustc-link-lib=shell32");
    println!("cargo:rustc-link-lib=ole32");
    
    // Try to link optional libraries
    println!("cargo:rustc-link-lib=sqlite3");
    println!("cargo:rustc-link-lib=libxml2");
    
    // MinGW library paths
    if config.is_cross_compiling {
        let mingw_paths = [
            "/usr/x86_64-w64-mingw32/lib",
            "/usr/i686-w64-mingw32/lib",
            "/opt/mingw64/lib",
        ];
        
        for path in &mingw_paths {
            if Path::new(path).exists() {
                println!("cargo:rustc-link-search=native={}", path);
            }
        }
    }
}

fn link_wasm_libraries(_config: &CrossCompilationConfig) {
    // WebAssembly doesn't link with traditional system libraries
    println!("cargo:warning=WebAssembly target: skipping system library linking");
}

fn link_fallback_libraries(config: &CrossCompilationConfig) {
    println!("cargo:warning=Unknown target OS {}, using fallback library linking", config.target_os);
    
    // Try common library names
    println!("cargo:rustc-link-lib=sqlite3");
    println!("cargo:rustc-link-lib=ffi");
    
    // Common library paths
    let common_paths = ["/usr/lib", "/usr/local/lib", "/lib"];
    for path in &common_paths {
        if Path::new(path).exists() {
            println!("cargo:rustc-link-search=native={}", path);
        }
    }
}

fn add_pkg_config_paths() {
    if let Ok(pkg_config_path) = env::var("PKG_CONFIG_PATH") {
        for path in pkg_config_path.split(':') {
            if !path.is_empty() {
                let lib_path = path.replace("/pkgconfig", "");
                if Path::new(&lib_path).exists() {
                    println!("cargo:rustc-link-search=native={}", lib_path);
                }
            }
        }
    }
}

fn add_nix_store_paths() {
    // Dynamic Nix store library discovery
    if let Ok(entries) = fs::read_dir("/nix/store") {
        let mut found_paths = HashMap::new();
        
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                // Look for relevant packages
                let packages = ["libffi", "sqlite", "ncurses", "libxml2", "llvm"];
                for package in &packages {
                    if name.contains(package) {
                        let lib_path = path.join("lib");
                        if lib_path.exists() {
                            if let Some(path_str) = lib_path.to_str() {
                                found_paths.insert(package.to_string(), path_str.to_string());
                            }
                        }
                    }
                }
            }
        }
        
        // Add found paths
        for (package, path) in found_paths {
            println!("cargo:rustc-link-search=native={}", path);
            println!("cargo:warning=Found {} in Nix store: {}", package, path);
        }
    }
}

fn build_and_link_runtime_libraries(config: &CrossCompilationConfig) {
    let runtime_dir = Path::new("runtime");
    if !runtime_dir.exists() {
        println!("cargo:warning=Runtime directory not found, skipping additional runtime libraries");
        return;
    }
    
    // Check for build script
    let build_script = runtime_dir.join("build_runtime.sh");
    if !build_script.exists() {
        println!("cargo:warning=Runtime build script not found");
        return;
    }
    
    // Build runtime libraries for target platform
    println!("cargo:warning=Building additional runtime libraries for {}", config.target_triple);
    
    let mut build_cmd = Command::new("bash");
    build_cmd
        .arg(&build_script)
        .current_dir(runtime_dir);
    
    // Set cross-compilation environment for runtime build
    if config.is_cross_compiling {
        propagate_cross_compilation_env(&mut build_cmd, config);
    }
    
    match build_cmd.output() {
        Ok(output) => {
            if output.status.success() {
                println!("cargo:warning=Additional runtime libraries built successfully");
                link_additional_runtime_libraries();
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                println!("cargo:warning=Runtime library build failed: {}", stderr);
            }
        }
        Err(e) => {
            println!("cargo:warning=Failed to run runtime build script: {}", e);
        }
    }
}

fn link_additional_runtime_libraries() {
    // Link additional static runtime libraries
    let runtime_libs = [
        "cursed_minimal_shims",
        "cursed_interface_runtime",
        "cursed_type_assertion_runtime",
        "cursed_type_checking",
        "cursed_memory_runtime",
    ];
    
    println!("cargo:rustc-link-search=native=runtime");
    
    for lib in &runtime_libs {
        let lib_path = format!("runtime/lib{}.a", lib);
        if Path::new(&lib_path).exists() {
            println!("cargo:rustc-link-lib=static={}", lib);
            
            // Ensure proper indexing with ranlib
            if let Ok(_) = Command::new("ranlib").arg(&lib_path).output() {
                // ranlib succeeded
            }
        }
    }
}
