// Build script for the CURSED programming language
// Creates a static runtime library and handles linking with system libraries

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/execution/runtime_functions.rs");
    println!("cargo:rerun-if-changed=build.rs");
    
    // Enable runtime library build
    build_runtime_library();
    
    // Link with system libraries (existing code)
    link_system_libraries();
}

fn build_runtime_library() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    
    println!("cargo:warning=Building CURSED runtime library...");
    
    // Create a separate runtime crate in OUT_DIR
    let runtime_dir = Path::new(&out_dir).join("cursed_runtime");
    fs::create_dir_all(&runtime_dir).unwrap();
    
    // Create runtime Cargo.toml
    let runtime_cargo_toml = r#"[package]
name = "cursed_runtime"
version = "0.1.0"
edition = "2021"

[workspace]
# This is a standalone package

[lib]
name = "cursed_runtime"
crate-type = ["staticlib"]

[dependencies]
regex = "1.10"
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
"#;
    
    fs::write(runtime_dir.join("Cargo.toml"), runtime_cargo_toml).unwrap();
    
    // Create runtime src/lib.rs
    let runtime_lib_rs = r#"//! CURSED Runtime Library
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
"#;
    
    fs::create_dir_all(runtime_dir.join("src")).unwrap();
    
    // Read the runtime functions from the main crate
    let main_runtime_path = Path::new("src/execution/runtime_functions.rs");
    let runtime_content = fs::read_to_string(main_runtime_path).unwrap();
    
    // Filter out the original imports and crate-specific content
    let filtered_content = filter_runtime_content(&runtime_content);
    
    // Include filtered content
    let full_runtime_lib = format!("{}\n{}", runtime_lib_rs, filtered_content);
    fs::write(runtime_dir.join("src/lib.rs"), full_runtime_lib).unwrap();
    
    // Build the runtime library
    let cargo_path = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let mut build_cmd = Command::new(&cargo_path);
    build_cmd
        .arg("build")
        .arg("--release")
        .arg("--lib")
        .current_dir(&runtime_dir)
        .env("CARGO_TARGET_DIR", runtime_dir.join("target"));
    
    let output = build_cmd.output().expect("Failed to build runtime library");
    
    if !output.status.success() {
        panic!("Failed to build runtime library:\n{}", String::from_utf8_lossy(&output.stderr));
    }
    
    // Copy the built library to a location where the linker can find it
    let lib_name = "libcursed_runtime.a";
    let target_triple = env::var("TARGET").unwrap_or_else(|_| "x86_64-unknown-linux-gnu".to_string());
    let src_lib_path_release = runtime_dir.join("target").join(&target_triple).join("release").join(lib_name);
    let src_lib_path_debug = runtime_dir.join("target").join(&target_triple).join("debug").join(lib_name);
    let dest_lib_path = Path::new(&out_dir).join(lib_name);
    
    let src_lib_path = if src_lib_path_release.exists() {
        src_lib_path_release
    } else if src_lib_path_debug.exists() {
        src_lib_path_debug
    } else {
        println!("cargo:warning=Runtime library not found at {} or {}", 
                 src_lib_path_release.display(), src_lib_path_debug.display());
        return;
    };
    
    fs::copy(&src_lib_path, &dest_lib_path).unwrap();
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=cursed_runtime");
    println!("cargo:warning=Successfully built runtime library at {}", dest_lib_path.display());
}

fn filter_runtime_content(content: &str) -> String {
    let mut result = String::new();
    let excluded_functions = vec![
        "initialize_runtime_functions",
        "get_minimal_result",
    ];
    
    let mut skip_until_next_function = false;
    
    for line in content.lines() {
        // Skip all use statements and initial comments
        if line.starts_with("use ") || line.starts_with("//!") {
            continue;
        }
        
        // Check for function start
        if line.contains("pub fn ") && !line.contains("extern \"C\"") {
            let should_skip = excluded_functions.iter().any(|func| line.contains(func));
            skip_until_next_function = should_skip || line.contains("CursedError");
            if skip_until_next_function {
                continue;
            }
        }
        
        // Check for extern C function start
        if line.contains("#[no_mangle]") || line.contains("pub extern \"C\" fn") {
            skip_until_next_function = false; // Always include extern C functions
        }
        
        // Skip lines that reference CursedError
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

fn link_system_libraries() {
    // Link with SQLite3 library
    println!("cargo:rustc-link-lib=sqlite3");
    
    // Link with libffi library
    println!("cargo:rustc-link-lib=ffi");
    
    // Link with terminfo/ncurses library
    println!("cargo:rustc-link-lib=tinfo");
    
    // Link with libxml2 library
    println!("cargo:rustc-link-lib=xml2");
    
    // For the Nix environment, add the specific library path
    if let Ok(library_path) = env::var("LIBRARY_PATH") {
        for path in library_path.split(':') {
            if !path.is_empty() {
                println!("cargo:rustc-link-search=native={}", path);
            }
        }
    }
    
    // Add libffi library search paths for Nix environment
    let possible_libffi_paths = vec![
        "/nix/store/09b5m303v4d52wjry30xsabj65vnhkni-libffi-3.4.7/lib",
        "/nix/store/6pak77li0iw9x0b3yhmbjvp846w3p6bx-libffi-3.4.6/lib",
        "/nix/store/n0lzbpbc5dwq03s1vjr885b28cjbp2gs-libffi-3.4.7/lib",
        "/nix/store/paqdsvmj4fwhc2w6rr884c3kymxl69k0-libffi-3.4.8/lib",
        "/usr/lib/x86_64-linux-gnu",
        "/usr/local/lib",
    ];
    
    // Add ncurses/tinfo library search paths for Nix environment
    let possible_ncurses_paths = vec![
        "/nix/store/k3a7dzrqphj9ksbb43i24vy6inz8ys51-ncurses-6.4.20221231/lib",
        "/usr/lib/x86_64-linux-gnu",
        "/usr/local/lib",
    ];
    
    // Add libxml2 library search paths for Nix environment
    let possible_xml2_paths = vec![
        "/nix/store/0z4hrksbdrwv9xb8ycjk3rq9ppmw0350-libxml2-2.13.5/lib",
        "/usr/lib/x86_64-linux-gnu",
        "/usr/local/lib",
    ];
    
    for path in possible_libffi_paths {
        if std::path::Path::new(path).exists() {
            println!("cargo:rustc-link-search=native={}", path);
        }
    }
    
    for path in possible_ncurses_paths {
        if std::path::Path::new(path).exists() {
            println!("cargo:rustc-link-search=native={}", path);
        }
    }
    
    for path in possible_xml2_paths {
        if std::path::Path::new(path).exists() {
            println!("cargo:rustc-link-search=native={}", path);
        }
    }
    
    // Also check for pkg-config for SQLite3
    if let Ok(pkg_config_path) = env::var("PKG_CONFIG_PATH") {
        for path in pkg_config_path.split(':') {
            if !path.is_empty() {
                println!("cargo:rustc-link-search=native={}", path.replace("/pkgconfig", ""));
            }
        }
    }
    
    // macOS-specific configuration for dylib loading
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-search=native=/usr/local/lib");
        println!("cargo:rustc-link-search=native=/opt/homebrew/lib");
    }
    
    // Add LLVM bin paths to environment for compilation
    let possible_llvm_bin_paths = vec![
        "/nix/store/013b6qj9g2n2pmxcllnch9drrf9m0zwf-llvm-17.0.6/bin",
        "/nix/store/s5a4igx64mngxrz3d4s2mxz6764mdv47-llvm-17.0.6/bin",
        "/nix/store/8qpf7pp0a71psdngm5nxc64jahw0vlwl-llvm-19.1.7/bin",
        "/nix/store/vnxd8nqfibccfbczxwd9li5hw42k5kmw-llvm-19.1.6/bin",
        "/usr/bin",
        "/usr/local/bin",
    ];
    
    for path in possible_llvm_bin_paths {
        if std::path::Path::new(path).exists() {
            if let Ok(current_path) = env::var("PATH") {
                println!("cargo:rustc-env=PATH={}:{}", path, current_path);
                break;
            } else {
                println!("cargo:rustc-env=PATH={}", path);
                break;
            }
        }
    }
    
    // Tell cargo to rerun build script if environment changes
    println!("cargo:rerun-if-env-changed=LIBRARY_PATH");
    println!("cargo:rerun-if-env-changed=PKG_CONFIG_PATH");
    println!("cargo:rerun-if-env-changed=PATH");
}
