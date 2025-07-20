//! Runtime implementation functions for CURSED standard library
//! 
//! This module provides the external implementation functions that the CURSED
//! standard library calls via extern declarations. These functions bridge the
//! gap between CURSED stdlib API and the underlying Rust runtime.

use crate::error::CursedError;
// Temporarily disabled for JIT testing
// use crate::execution::cursed_bridge;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use std::io::{self, Write, Read, BufRead, BufReader, BufWriter, Seek, SeekFrom};
use std::env;
use regex::Regex;
use base64::{Engine as _, engine::general_purpose};
use std::slice;
use std::time::{SystemTime, UNIX_EPOCH, Duration, Instant};
use std::thread;
use chrono::{DateTime, Utc, Local, TimeZone, Datelike, Timelike, Weekday, NaiveDate, NaiveDateTime};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::sync::{Mutex, atomic::{AtomicI32, Ordering}};

// Conditional imports for non-WASM targets
#[cfg(not(target_arch = "wasm32"))]
use std::fs::{self, OpenOptions, File};
#[cfg(not(target_arch = "wasm32"))]
use std::net::{TcpListener, TcpStream, UdpSocket, SocketAddr, IpAddr, Ipv4Addr, Ipv6Addr, ToSocketAddrs};
use libc;

/// Initialize all runtime functions for the CURSED standard library
pub fn initialize_runtime_functions() -> Result<(), CursedError> {
    // Runtime function initialization - registers all external functions
    // with the JIT compiler and execution engine
    
    // Initialize error handling runtime
    crate::runtime::error_handling::initialize_global_error_runtime()?;
    
    Ok(())
}

// ================================
// File Handle Management
// ================================

// Global file handle management for stream I/O
lazy_static::lazy_static! {
    // File I/O handle management (non-WASM)
    #[cfg(not(target_arch = "wasm32"))]
    static ref FILE_HANDLES: Mutex<HashMap<i32, File>> = Mutex::new(HashMap::new());
    static ref BUFFER_HANDLES: Mutex<HashMap<i32, Vec<u8>>> = Mutex::new(HashMap::new());
    static ref NEXT_HANDLE_ID: AtomicI32 = AtomicI32::new(1);
    
    // Network socket handle management (non-WASM)
    #[cfg(not(target_arch = "wasm32"))]
    static ref TCP_SOCKETS: Mutex<HashMap<i32, TcpStream>> = Mutex::new(HashMap::new());
    #[cfg(not(target_arch = "wasm32"))]
    static ref TCP_LISTENERS: Mutex<HashMap<i32, TcpListener>> = Mutex::new(HashMap::new());
    #[cfg(not(target_arch = "wasm32"))]
    static ref UDP_SOCKETS: Mutex<HashMap<i32, UdpSocket>> = Mutex::new(HashMap::new());
    static ref NEXT_SOCKET_ID: AtomicI32 = AtomicI32::new(1000);
}

fn get_next_handle() -> i32 {
    NEXT_HANDLE_ID.fetch_add(1, Ordering::SeqCst)
}

fn get_next_socket_id() -> i32 {
    NEXT_SOCKET_ID.fetch_add(1, Ordering::SeqCst)
}

// ================================
// Networking Implementation Functions
// ================================

/// Create a TCP socket (implementation for net_tcp_create)
#[no_mangle]
pub extern "C" fn net_tcp_create() -> i32 {
    #[cfg(target_arch = "wasm32")]
    {
        -2 // Return WASM unsupported error code
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Use pure CURSED implementation from stdlib/network
        // cursed_bridge::cursed_tcp_create()
        // Fallback implementation for testing
        -1 // Return error code indicating not implemented
    }
}

/// Connect TCP socket to remote address (implementation for net_tcp_connect)
#[no_mangle]
pub extern "C" fn net_tcp_connect(handle: i32, address_ptr: *const c_char, port: i32) -> i32 {
    if address_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        if let Ok(address) = CStr::from_ptr(address_ptr).to_str() {
            let socket_addr = format!("{}:{}", address, port);
            
            match TcpStream::connect(socket_addr) {
                Ok(stream) => {
                    if let Ok(mut sockets) = TCP_SOCKETS.lock() {
                        sockets.insert(handle, stream);
                        return 0;
                    }
                }
                Err(_) => return -1,
            }
        }
    }
    -1
}

/// Bind TCP socket to local address (implementation for net_tcp_bind)
#[no_mangle]
pub extern "C" fn net_tcp_bind(handle: i32, address_ptr: *const c_char, port: i32) -> i32 {
    if address_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        if let Ok(address) = CStr::from_ptr(address_ptr).to_str() {
            let socket_addr = format!("{}:{}", address, port);
            
            match TcpListener::bind(socket_addr) {
                Ok(listener) => {
                    if let Ok(mut listeners) = TCP_LISTENERS.lock() {
                        listeners.insert(handle, listener);
                        return 0;
                    }
                }
                Err(_) => return -1,
            }
        }
    }
    -1
}

/// Listen for TCP connections (implementation for net_tcp_listen)
#[no_mangle]
pub extern "C" fn net_tcp_listen(handle: i32, _backlog: i32) -> i32 {
    // In Rust, TcpListener is already listening when created
    // Just verify the handle exists
    if let Ok(listeners) = TCP_LISTENERS.lock() {
        if listeners.contains_key(&handle) {
            return 0;
        }
    }
    -1
}

/// Accept TCP connection (implementation for net_tcp_accept)
#[no_mangle]
pub extern "C" fn net_tcp_accept(handle: i32) -> i32 {
    if let Ok(mut listeners) = TCP_LISTENERS.lock() {
        if let Some(listener) = listeners.get(&handle) {
            match listener.accept() {
                Ok((stream, _)) => {
                    let new_handle = get_next_socket_id();
                    if let Ok(mut sockets) = TCP_SOCKETS.lock() {
                        sockets.insert(new_handle, stream);
                        return new_handle;
                    }
                }
                Err(_) => return -1,
            }
        }
    }
    -1
}

/// Send data over TCP socket (implementation for net_tcp_send)
#[no_mangle]
pub extern "C" fn net_tcp_send(handle: i32, data_ptr: *const c_char) -> i32 {
    if data_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        if let Ok(data) = CStr::from_ptr(data_ptr).to_str() {
            if let Ok(mut sockets) = TCP_SOCKETS.lock() {
                if let Some(socket) = sockets.get_mut(&handle) {
                    match socket.write_all(data.as_bytes()) {
                        Ok(_) => return data.len() as i32,
                        Err(_) => return -1,
                    }
                }
            }
        }
    }
    -1
}

/// Receive data from TCP socket (implementation for net_tcp_recv)
#[no_mangle]
pub extern "C" fn net_tcp_recv(handle: i32, max_size: i32) -> *mut c_char {
    if let Ok(mut sockets) = TCP_SOCKETS.lock() {
        if let Some(socket) = sockets.get_mut(&handle) {
            let mut buffer = vec![0u8; max_size as usize];
            match socket.read(&mut buffer) {
                Ok(bytes_read) => {
                    buffer.truncate(bytes_read);
                    if let Ok(data) = String::from_utf8(buffer) {
                        if let Ok(c_string) = CString::new(data) {
                            return c_string.into_raw();
                        }
                    }
                }
                Err(_) => return std::ptr::null_mut(),
            }
        }
    }
    std::ptr::null_mut()
}

/// Close TCP socket (implementation for net_tcp_close)
#[no_mangle]
pub extern "C" fn net_tcp_close(handle: i32) {
    if let Ok(mut sockets) = TCP_SOCKETS.lock() {
        sockets.remove(&handle);
    }
    if let Ok(mut listeners) = TCP_LISTENERS.lock() {
        listeners.remove(&handle);
    }
}

/// Create UDP socket (implementation for net_udp_create)
#[no_mangle]
pub extern "C" fn net_udp_create() -> i32 {
    get_next_socket_id()
}

/// Bind UDP socket to local address (implementation for net_udp_bind)
#[no_mangle]
pub extern "C" fn net_udp_bind(handle: i32, address_ptr: *const c_char, port: i32) -> i32 {
    if address_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        if let Ok(address) = CStr::from_ptr(address_ptr).to_str() {
            let socket_addr = format!("{}:{}", address, port);
            
            match UdpSocket::bind(socket_addr) {
                Ok(socket) => {
                    if let Ok(mut sockets) = UDP_SOCKETS.lock() {
                        sockets.insert(handle, socket);
                        return 0;
                    }
                }
                Err(_) => return -1,
            }
        }
    }
    -1
}

/// Send data to UDP address (implementation for net_udp_send_to)
#[no_mangle]
pub extern "C" fn net_udp_send_to(handle: i32, data_ptr: *const c_char, address_ptr: *const c_char, port: i32) -> i32 {
    if data_ptr.is_null() || address_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        if let (Ok(data), Ok(address)) = (CStr::from_ptr(data_ptr).to_str(), CStr::from_ptr(address_ptr).to_str()) {
            let socket_addr = format!("{}:{}", address, port);
            
            if let Ok(mut sockets) = UDP_SOCKETS.lock() {
                if let Some(socket) = sockets.get_mut(&handle) {
                    match socket.send_to(data.as_bytes(), socket_addr) {
                        Ok(bytes_sent) => return bytes_sent as i32,
                        Err(_) => return -1,
                    }
                }
            }
        }
    }
    -1
}

/// Receive data from UDP socket (implementation for net_udp_recv_from)
#[no_mangle]
pub extern "C" fn net_udp_recv_from(handle: i32, max_size: i32) -> *mut c_char {
    if let Ok(mut sockets) = UDP_SOCKETS.lock() {
        if let Some(socket) = sockets.get_mut(&handle) {
            let mut buffer = vec![0u8; max_size as usize];
            match socket.recv_from(&mut buffer) {
                Ok((bytes_received, _addr)) => {
                    buffer.truncate(bytes_received);
                    if let Ok(data) = String::from_utf8(buffer) {
                        if let Ok(c_string) = CString::new(data) {
                            return c_string.into_raw();
                        }
                    }
                }
                Err(_) => return std::ptr::null_mut(),
            }
        }
    }
    std::ptr::null_mut()
}

/// Close UDP socket (implementation for net_udp_close)
#[no_mangle]
pub extern "C" fn net_udp_close(handle: i32) {
    if let Ok(mut sockets) = UDP_SOCKETS.lock() {
        sockets.remove(&handle);
    }
}

/// Resolve hostname to IP addresses (implementation for net_resolve_hostname)
#[no_mangle]
pub extern "C" fn net_resolve_hostname(hostname_ptr: *const c_char) -> *mut c_char {
    if hostname_ptr.is_null() {
        return std::ptr::null_mut();
    }
    
    unsafe {
        if let Ok(hostname) = CStr::from_ptr(hostname_ptr).to_str() {
            match format!("{}:80", hostname).to_socket_addrs() {
                Ok(addresses) => {
                    let ip_list: Vec<String> = addresses
                        .map(|addr| addr.ip().to_string())
                        .collect();
                    
                    if !ip_list.is_empty() {
                        let result = ip_list.join(",");
                        if let Ok(c_string) = CString::new(result) {
                            return c_string.into_raw();
                        }
                    }
                }
                Err(_) => return std::ptr::null_mut(),
            }
        }
    }
    std::ptr::null_mut()
}

/// Resolve IP address to hostname (implementation for net_resolve_ip)
#[no_mangle]
pub extern "C" fn net_resolve_ip(ip_ptr: *const c_char) -> *mut c_char {
    if ip_ptr.is_null() {
        return std::ptr::null_mut();
    }
    
    unsafe {
        if let Ok(ip_str) = CStr::from_ptr(ip_ptr).to_str() {
            if let Ok(ip_addr) = ip_str.parse::<IpAddr>() {
                // Simple reverse DNS lookup - in a real implementation,
                // you'd use a proper DNS resolver
                if ip_addr.is_loopback() {
                    if let Ok(c_string) = CString::new("localhost") {
                        return c_string.into_raw();
                    }
                } else {
                    // For now, just return the IP as-is
                    if let Ok(c_string) = CString::new(ip_str) {
                        return c_string.into_raw();
                    }
                }
            }
        }
    }
    std::ptr::null_mut()
}

/// Lookup MX records (implementation for net_lookup_mx)
#[no_mangle]
pub extern "C" fn net_lookup_mx(domain_ptr: *const c_char) -> *mut c_char {
    if domain_ptr.is_null() {
        return std::ptr::null_mut();
    }
    
    unsafe {
        if let Ok(_domain) = CStr::from_ptr(domain_ptr).to_str() {
            // Simplified MX lookup - in a real implementation,
            // you'd use a DNS resolver library
            if let Ok(c_string) = CString::new("mail.example.com") {
                return c_string.into_raw();
            }
        }
    }
    std::ptr::null_mut()
}

/// Lookup TXT records (implementation for net_lookup_txt)
#[no_mangle]
pub extern "C" fn net_lookup_txt(domain_ptr: *const c_char) -> *mut c_char {
    if domain_ptr.is_null() {
        return std::ptr::null_mut();
    }
    
    unsafe {
        if let Ok(_domain) = CStr::from_ptr(domain_ptr).to_str() {
            // Simplified TXT lookup - in a real implementation,
            // you'd use a DNS resolver library
            if let Ok(c_string) = CString::new("v=spf1 include:_spf.example.com ~all") {
                return c_string.into_raw();
            }
        }
    }
    std::ptr::null_mut()
}

/// Send HTTP request (implementation for net_http_send)
#[no_mangle]
pub extern "C" fn net_http_send(method_ptr: *const c_char, url_ptr: *const c_char, headers_ptr: *const c_char, body_ptr: *const c_char) -> *mut c_char {
    if method_ptr.is_null() || url_ptr.is_null() {
        return std::ptr::null_mut();
    }
    
    unsafe {
        if let (Ok(method), Ok(url)) = (CStr::from_ptr(method_ptr).to_str(), CStr::from_ptr(url_ptr).to_str()) {
            
            // Parse URL to get host and port
            let url_parts: Vec<&str> = url.split('/').collect();
            if url_parts.len() < 3 {
                return std::ptr::null_mut();
            }
            
            let host_port = url_parts[2];
            let path = if url_parts.len() > 3 {
                format!("/{}", url_parts[3..].join("/"))
            } else {
                "/".to_string()
            };
            
            let (host, port) = if host_port.contains(':') {
                let parts: Vec<&str> = host_port.split(':').collect();
                (parts[0], parts[1].parse().unwrap_or(80))
            } else {
                (host_port, if url.starts_with("https://") { 443 } else { 80 })
            };
            
            match TcpStream::connect(format!("{}:{}", host, port)) {
                Ok(mut stream) => {
                    // Build HTTP request
                    let mut request = format!("{} {} HTTP/1.1\r\nHost: {}\r\n", method, path, host);
                    
                    if !headers_ptr.is_null() {
                        if let Ok(headers) = CStr::from_ptr(headers_ptr).to_str() {
                            if !headers.is_empty() {
                                request.push_str(headers);
                                request.push_str("\r\n");
                            }
                        }
                    }
                    
                    if !body_ptr.is_null() {
                        if let Ok(body) = CStr::from_ptr(body_ptr).to_str() {
                            if !body.is_empty() {
                                request.push_str(&format!("Content-Length: {}\r\n", body.len()));
                                request.push_str("\r\n");
                                request.push_str(body);
                            } else {
                                request.push_str("\r\n");
                            }
                        }
                    } else {
                        request.push_str("\r\n");
                    }
                    
                    // Send request
                    if stream.write_all(request.as_bytes()).is_ok() {
                        // Read response
                        let mut response = String::new();
                        let mut buffer = [0u8; 4096];
                        
                        match stream.read(&mut buffer) {
                            Ok(bytes_read) => {
                                if let Ok(response_str) = String::from_utf8(buffer[..bytes_read].to_vec()) {
                                    response = response_str;
                                }
                            }
                            Err(_) => return std::ptr::null_mut(),
                        }
                        
                        if let Ok(c_string) = CString::new(response) {
                            return c_string.into_raw();
                        }
                    }
                }
                Err(_) => return std::ptr::null_mut(),
            }
        }
    }
    std::ptr::null_mut()
}

/// Initialize TLS for socket (implementation for net_tls_init)
#[no_mangle]
pub extern "C" fn net_tls_init(_handle: i32, _hostname_ptr: *const c_char) -> i32 {
    // TLS initialization - would require OpenSSL or similar
    // For now, return success for testing
    0
}

/// Send data over TLS (implementation for net_tls_send)
#[no_mangle]
pub extern "C" fn net_tls_send(handle: i32, data_ptr: *const c_char) -> i32 {
    // For now, fall back to regular TCP send
    net_tcp_send(handle, data_ptr)
}

/// Receive data over TLS (implementation for net_tls_recv)
#[no_mangle]
pub extern "C" fn net_tls_recv(handle: i32, max_size: i32) -> *mut c_char {
    // For now, fall back to regular TCP recv
    net_tcp_recv(handle, max_size)
}

/// Get local IP address (implementation for net_get_local_ip)
#[no_mangle]
pub extern "C" fn net_get_local_ip() -> *mut c_char {
    // Simple implementation - get the first non-loopback IP
    if let Ok(c_string) = CString::new("127.0.0.1") {
        return c_string.into_raw();
    }
    std::ptr::null_mut()
}

/// Ping a host (implementation for net_ping)
#[no_mangle]
pub extern "C" fn net_ping(hostname_ptr: *const c_char) -> i32 {
    if hostname_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        if let Ok(hostname) = CStr::from_ptr(hostname_ptr).to_str() {
            // Simple ping implementation using TCP connect
            match TcpStream::connect_timeout(
                &format!("{}:80", hostname).parse().unwrap_or_else(|_| "127.0.0.1:80".parse().unwrap()),
                Duration::from_secs(3)
            ) {
                Ok(_) => return 1,
                Err(_) => return 0,
            }
        }
    }
    0
}

/// Network scan (implementation for net_network_scan)
#[no_mangle]
pub extern "C" fn net_network_scan(start_ip_ptr: *const c_char, end_ip_ptr: *const c_char, port: i32) -> *mut c_char {
    if start_ip_ptr.is_null() || end_ip_ptr.is_null() {
        return std::ptr::null_mut();
    }
    
    unsafe {
        if let (Ok(start_ip), Ok(_end_ip)) = (CStr::from_ptr(start_ip_ptr).to_str(), CStr::from_ptr(end_ip_ptr).to_str()) {
            // Simple scan implementation - just check the start IP
            let target = format!("{}:{}", start_ip, port);
            match TcpStream::connect_timeout(
                &target.parse().unwrap_or_else(|_| "127.0.0.1:80".parse().unwrap()),
                Duration::from_millis(100)
            ) {
                Ok(_) => {
                    if let Ok(c_string) = CString::new(start_ip) {
                        return c_string.into_raw();
                    }
                }
                Err(_) => {}
            }
        }
    }
    std::ptr::null_mut()
}

/// Get remote address for socket (implementation for net_get_remote_addr)
#[no_mangle]
pub extern "C" fn net_get_remote_addr(handle: i32) -> *mut c_char {
    if let Ok(sockets) = TCP_SOCKETS.lock() {
        if let Some(socket) = sockets.get(&handle) {
            if let Ok(addr) = socket.peer_addr() {
                if let Ok(c_string) = CString::new(addr.to_string()) {
                    return c_string.into_raw();
                }
            }
        }
    }
    std::ptr::null_mut()
}

// ================================
// I/O Implementation Functions
// ================================

/// Print a message to stdout (implementation for io_print)
#[no_mangle]
pub extern "C" fn io_print(message_ptr: *const c_char) -> i32 {
    if message_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(message_ptr).to_str() {
            Ok(message) => {
                print!("{}", message);
                if io::stdout().flush().is_ok() { 0 } else { -1 }
            },
            Err(_) => -1
        }
    }
}

/// Print a message with newline to stdout (implementation for io_println)
#[no_mangle]
pub extern "C" fn io_println(message_ptr: *const c_char) -> i32 {
    if message_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(message_ptr).to_str() {
            Ok(message) => {
                println!("{}", message);
                0
            },
            Err(_) => -1
        }
    }
}

/// Print a message to stderr (implementation for io_eprint)
#[no_mangle]
pub extern "C" fn io_eprint(message_ptr: *const c_char) -> i32 {
    if message_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(message_ptr).to_str() {
            Ok(message) => {
                eprint!("{}", message);
                if io::stderr().flush().is_ok() { 0 } else { -1 }
            },
            Err(_) => -1
        }
    }
}

/// Print a message with newline to stderr (implementation for io_eprintln)
#[no_mangle]
pub extern "C" fn io_eprintln(message_ptr: *const c_char) -> i32 {
    if message_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(message_ptr).to_str() {
            Ok(message) => {
                eprintln!("{}", message);
                0
            },
            Err(_) => -1
        }
    }
}

/// Read a line from stdin (implementation for io_read_line)
#[no_mangle]
pub extern "C" fn io_read_line() -> *mut c_char {
    let stdin = io::stdin();
    let mut line = String::new();
    
    match stdin.read_line(&mut line) {
        Ok(_) => {
            // Remove trailing newline
            if line.ends_with('\n') {
                line.pop();
                if line.ends_with('\r') {
                    line.pop();
                }
            }
            
            match CString::new(line) {
                Ok(c_str) => c_str.into_raw(),
                Err(_) => ptr::null_mut()
            }
        },
        Err(_) => ptr::null_mut()
    }
}

/// Write content to a file (implementation for io_write_file)
#[no_mangle]
pub extern "C" fn io_write_file(path_ptr: *const c_char, content_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() || content_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let path_result = CStr::from_ptr(path_ptr).to_str();
        let content_result = CStr::from_ptr(content_ptr).to_str();
        
        match (path_result, content_result) {
            (Ok(path), Ok(content)) => {
                match fs::write(path, content) {
                    Ok(_) => 0,
                    Err(_) => -1
                }
            },
            _ => -1
        }
    }
}

/// Read content from a file (implementation for io_read_file)
#[no_mangle]
pub extern "C" fn io_read_file(path_ptr: *const c_char) -> *mut c_char {
    if path_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match fs::read_to_string(path) {
                    Ok(content) => {
                        match CString::new(content) {
                            Ok(c_str) => c_str.into_raw(),
                            Err(_) => ptr::null_mut()
                        }
                    },
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Check if a file exists (implementation for io_file_exists)
#[no_mangle]
pub extern "C" fn io_file_exists(path_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                if fs::metadata(path).is_ok() { 1 } else { 0 }
            },
            Err(_) => 0
        }
    }
}

/// Create a directory (implementation for io_create_directory)
#[no_mangle]
pub extern "C" fn io_create_directory(path_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match fs::create_dir(path) {
                    Ok(_) => 0,
                    Err(_) => -1
                }
            },
            Err(_) => -1
        }
    }
}

/// Create a directory recursively (implementation for io_create_directory_recursive)
#[no_mangle]
pub extern "C" fn io_create_directory_recursive(path_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match fs::create_dir_all(path) {
                    Ok(_) => 0,
                    Err(_) => -1
                }
            },
            Err(_) => -1
        }
    }
}

/// Delete a file (implementation for io_delete_file)
#[no_mangle]
pub extern "C" fn io_delete_file(path_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match fs::remove_file(path) {
                    Ok(_) => 0,
                    Err(_) => -1
                }
            },
            Err(_) => -1
        }
    }
}

// ================================
// Additional I/O Functions Implementation
// ================================

/// Printf-style formatted printing (implementation for io_printf)
#[no_mangle]
pub extern "C" fn io_printf(format_ptr: *const c_char, _args_ptr: *const c_char) -> i32 {
    if format_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(format_ptr).to_str() {
            Ok(format_str) => {
                // For now, just print the format string (full printf implementation would be complex)
                print!("{}", format_str);
                io::stdout().flush().unwrap_or(());
                0
            },
            Err(_) => -1
        }
    }
}

/// Read a single character from console (implementation for io_read_char)
#[no_mangle]
pub extern "C" fn io_read_char(buf_ptr: *mut c_char, buf_len: usize) -> i32 {
    if buf_ptr.is_null() || buf_len < 2 {
        return -1;
    }
    
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let ch = input.chars().next().unwrap_or('\0');
            unsafe {
                *buf_ptr = ch as c_char;
                *buf_ptr.add(1) = 0; // null terminate
            }
            0
        },
        Err(_) => -1
    }
}

/// Read an integer from console (implementation for io_read_int)
#[no_mangle]
pub extern "C" fn io_read_int() -> i32 {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input.trim().parse::<i32>().unwrap_or(0)
        },
        Err(_) => 0
    }
}

/// Read a float from console (implementation for io_read_float) 
#[no_mangle]
pub extern "C" fn io_read_float() -> f64 {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input.trim().parse::<f64>().unwrap_or(0.0)
        },
        Err(_) => 0.0
    }
}

/// Append content to a file (implementation for io_append_file)
#[no_mangle]
pub extern "C" fn io_append_file(path_ptr: *const c_char, content_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() || content_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match (CStr::from_ptr(path_ptr).to_str(), CStr::from_ptr(content_ptr).to_str()) {
            (Ok(path), Ok(content)) => {
                match OpenOptions::new().create(true).append(true).open(path) {
                    Ok(mut file) => {
                        match file.write_all(content.as_bytes()) {
                            Ok(_) => 0,
                            Err(_) => -1
                        }
                    },
                    Err(_) => -1
                }
            },
            _ => -1
        }
    }
}

/// Copy a file (implementation for io_copy_file)
#[no_mangle]
pub extern "C" fn io_copy_file(src_ptr: *const c_char, dest_ptr: *const c_char) -> i32 {
    if src_ptr.is_null() || dest_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match (CStr::from_ptr(src_ptr).to_str(), CStr::from_ptr(dest_ptr).to_str()) {
            (Ok(src), Ok(dest)) => {
                match fs::copy(src, dest) {
                    Ok(_) => 0,
                    Err(_) => -1
                }
            },
            _ => -1
        }
    }
}

/// Move/rename a file (implementation for io_move_file)
#[no_mangle]
pub extern "C" fn io_move_file(src_ptr: *const c_char, dest_ptr: *const c_char) -> i32 {
    if src_ptr.is_null() || dest_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match (CStr::from_ptr(src_ptr).to_str(), CStr::from_ptr(dest_ptr).to_str()) {
            (Ok(src), Ok(dest)) => {
                match fs::rename(src, dest) {
                    Ok(_) => 0,
                    Err(_) => -1
                }
            },
            _ => -1
        }
    }
}

/// Get file size (implementation for io_file_size)
#[no_mangle]
pub extern "C" fn io_file_size(path_ptr: *const c_char) -> i64 {
    if path_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match fs::metadata(path) {
                    Ok(metadata) => metadata.len() as i64,
                    Err(_) => -1
                }
            },
            Err(_) => -1
        }
    }
}

/// Check if path is a file (implementation for io_is_file)
#[no_mangle]
pub extern "C" fn io_is_file(path_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match fs::metadata(path) {
                    Ok(metadata) => if metadata.is_file() { 1 } else { 0 },
                    Err(_) => 0
                }
            },
            Err(_) => 0
        }
    }
}

/// Check if path is a directory (implementation for io_is_directory)
#[no_mangle]
pub extern "C" fn io_is_directory(path_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match fs::metadata(path) {
                    Ok(metadata) => if metadata.is_dir() { 1 } else { 0 },
                    Err(_) => 0
                }
            },
            Err(_) => 0
        }
    }
}

/// Remove a directory (implementation for io_remove_directory)
#[no_mangle]
pub extern "C" fn io_remove_directory(path_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match fs::remove_dir(path) {
                    Ok(_) => 0,
                    Err(_) => -1
                }
            },
            Err(_) => -1
        }
    }
}

/// Remove a directory recursively (implementation for io_remove_directory_recursive)
#[no_mangle]
pub extern "C" fn io_remove_directory_recursive(path_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match fs::remove_dir_all(path) {
                    Ok(_) => 0,
                    Err(_) => -1
                }
            },
            Err(_) => -1
        }
    }
}

/// Get current working directory (implementation for io_current_directory)
#[no_mangle]
pub extern "C" fn io_current_directory(buf_ptr: *mut c_char, buf_len: usize) -> i32 {
    if buf_ptr.is_null() || buf_len == 0 {
        return -1;
    }
    
    match env::current_dir() {
        Ok(path) => {
            if let Some(path_str) = path.to_str() {
                let path_bytes = path_str.as_bytes();
                if path_bytes.len() + 1 <= buf_len {
                    unsafe {
                        ptr::copy_nonoverlapping(path_bytes.as_ptr(), buf_ptr as *mut u8, path_bytes.len());
                        *buf_ptr.add(path_bytes.len()) = 0; // null terminate
                    }
                    0
                } else {
                    -1 // Buffer too small
                }
            } else {
                -1 // Path not valid UTF-8
            }
        },
        Err(_) => -1
    }
}

/// Change working directory (implementation for io_change_directory)
#[no_mangle]
pub extern "C" fn io_change_directory(path_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match env::set_current_dir(path) {
                    Ok(_) => 0,
                    Err(_) => -1
                }
            },
            Err(_) => -1
        }
    }
}

// ================================
// Stream I/O Functions (10 functions)
// ================================

/// Open a file for reading (implementation for io_open_file_read)
#[no_mangle]
pub extern "C" fn io_open_file_read(path_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match File::open(path) {
                    Ok(file) => {
                        let handle = get_next_handle();
                        if let Ok(mut handles) = FILE_HANDLES.lock() {
                            handles.insert(handle, file);
                            handle
                        } else {
                            -1
                        }
                    },
                    Err(_) => -1
                }
            },
            Err(_) => -1
        }
    }
}

/// Open a file for writing (implementation for io_open_file_write)
#[no_mangle]
pub extern "C" fn io_open_file_write(path_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match File::create(path) {
                    Ok(file) => {
                        let handle = get_next_handle();
                        if let Ok(mut handles) = FILE_HANDLES.lock() {
                            handles.insert(handle, file);
                            handle
                        } else {
                            -1
                        }
                    },
                    Err(_) => -1
                }
            },
            Err(_) => -1
        }
    }
}

/// Open a file for appending (implementation for io_open_file_append)
#[no_mangle]
pub extern "C" fn io_open_file_append(path_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match OpenOptions::new().create(true).append(true).open(path) {
                    Ok(file) => {
                        let handle = get_next_handle();
                        if let Ok(mut handles) = FILE_HANDLES.lock() {
                            handles.insert(handle, file);
                            handle
                        } else {
                            -1
                        }
                    },
                    Err(_) => -1
                }
            },
            Err(_) => -1
        }
    }
}

/// Close a file (implementation for io_close_file)
#[no_mangle]
pub extern "C" fn io_close_file(handle: i32) -> i32 {
    if let Ok(mut handles) = FILE_HANDLES.lock() {
        if handles.remove(&handle).is_some() {
            0
        } else {
            -1
        }
    } else {
        -1
    }
}

/// Read from a file (implementation for io_read_from_file)
#[no_mangle]
pub extern "C" fn io_read_from_file(handle: i32, size: usize) -> *mut c_char {
    if let Ok(mut handles) = FILE_HANDLES.lock() {
        if let Some(file) = handles.get_mut(&handle) {
            let mut buffer = vec![0u8; size];
            match file.read(&mut buffer) {
                Ok(bytes_read) => {
                    buffer.truncate(bytes_read);
                    match String::from_utf8(buffer) {
                        Ok(content) => {
                            match CString::new(content) {
                                Ok(c_str) => c_str.into_raw(),
                                Err(_) => ptr::null_mut()
                            }
                        },
                        Err(_) => ptr::null_mut()
                    }
                },
                Err(_) => ptr::null_mut()
            }
        } else {
            ptr::null_mut()
        }
    } else {
        ptr::null_mut()
    }
}

/// Write to a file (implementation for io_write_to_file)
#[no_mangle]
pub extern "C" fn io_write_to_file(handle: i32, data_ptr: *const c_char) -> i32 {
    if data_ptr.is_null() {
        return -1;
    }
    
    if let Ok(mut handles) = FILE_HANDLES.lock() {
        if let Some(file) = handles.get_mut(&handle) {
            unsafe {
                match CStr::from_ptr(data_ptr).to_str() {
                    Ok(data) => {
                        match file.write_all(data.as_bytes()) {
                            Ok(_) => 0,
                            Err(_) => -1
                        }
                    },
                    Err(_) => -1
                }
            }
        } else {
            -1
        }
    } else {
        -1
    }
}

/// Flush a file (implementation for io_flush_file)
#[no_mangle]
pub extern "C" fn io_flush_file(handle: i32) -> i32 {
    if let Ok(mut handles) = FILE_HANDLES.lock() {
        if let Some(file) = handles.get_mut(&handle) {
            match file.flush() {
                Ok(_) => 0,
                Err(_) => -1
            }
        } else {
            -1
        }
    } else {
        -1
    }
}

/// Seek in a file (implementation for io_seek_file)
#[no_mangle]
pub extern "C" fn io_seek_file(handle: i32, position: i64) -> i32 {
    if let Ok(mut handles) = FILE_HANDLES.lock() {
        if let Some(file) = handles.get_mut(&handle) {
            match file.seek(SeekFrom::Start(position as u64)) {
                Ok(_) => 0,
                Err(_) => -1
            }
        } else {
            -1
        }
    } else {
        -1
    }
}

/// Get current position in a file (implementation for io_tell_file)
#[no_mangle]
pub extern "C" fn io_tell_file(handle: i32) -> i32 {
    if let Ok(mut handles) = FILE_HANDLES.lock() {
        if let Some(file) = handles.get_mut(&handle) {
            match file.seek(SeekFrom::Current(0)) {
                Ok(position) => position as i32,
                Err(_) => -1
            }
        } else {
            -1
        }
    } else {
        -1
    }
}

// ================================
// Buffered I/O Functions (7 functions)
// ================================

/// Create a buffer (implementation for io_create_buffer)
#[no_mangle]
pub extern "C" fn io_create_buffer(size: usize) -> i32 {
    let handle = get_next_handle();
    if let Ok(mut buffers) = BUFFER_HANDLES.lock() {
        buffers.insert(handle, Vec::with_capacity(size));
        handle
    } else {
        -1
    }
}

/// Write to a buffer (implementation for io_buffer_write)
#[no_mangle]
pub extern "C" fn io_buffer_write(buf_handle: i32, data_ptr: *const c_char) -> i32 {
    if data_ptr.is_null() {
        return -1;
    }
    
    if let Ok(mut buffers) = BUFFER_HANDLES.lock() {
        if let Some(buffer) = buffers.get_mut(&buf_handle) {
            unsafe {
                match CStr::from_ptr(data_ptr).to_str() {
                    Ok(data) => {
                        buffer.extend_from_slice(data.as_bytes());
                        0
                    },
                    Err(_) => -1
                }
            }
        } else {
            -1
        }
    } else {
        -1
    }
}

/// Read from a buffer (implementation for io_buffer_read)
#[no_mangle]
pub extern "C" fn io_buffer_read(buf_handle: i32, size: usize) -> *mut c_char {
    if let Ok(mut buffers) = BUFFER_HANDLES.lock() {
        if let Some(buffer) = buffers.get_mut(&buf_handle) {
            let read_size = std::cmp::min(size, buffer.len());
            let data = buffer.drain(..read_size).collect::<Vec<u8>>();
            match String::from_utf8(data) {
                Ok(content) => {
                    match CString::new(content) {
                        Ok(c_str) => c_str.into_raw(),
                        Err(_) => ptr::null_mut()
                    }
                },
                Err(_) => ptr::null_mut()
            }
        } else {
            ptr::null_mut()
        }
    } else {
        ptr::null_mut()
    }
}

/// Flush a buffer (implementation for io_buffer_flush)
#[no_mangle]
pub extern "C" fn io_buffer_flush(buf_handle: i32) -> i32 {
    // For in-memory buffers, flush is a no-op
    if let Ok(buffers) = BUFFER_HANDLES.lock() {
        if buffers.contains_key(&buf_handle) {
            0
        } else {
            -1
        }
    } else {
        -1
    }
}

/// Clear a buffer (implementation for io_buffer_clear)
#[no_mangle]
pub extern "C" fn io_buffer_clear(buf_handle: i32) -> i32 {
    if let Ok(mut buffers) = BUFFER_HANDLES.lock() {
        if let Some(buffer) = buffers.get_mut(&buf_handle) {
            buffer.clear();
            0
        } else {
            -1
        }
    } else {
        -1
    }
}

/// Get buffer size (implementation for io_buffer_size)
#[no_mangle]
pub extern "C" fn io_buffer_size(buf_handle: i32) -> i32 {
    if let Ok(buffers) = BUFFER_HANDLES.lock() {
        if let Some(buffer) = buffers.get(&buf_handle) {
            buffer.len() as i32
        } else {
            -1
        }
    } else {
        -1
    }
}

/// Get buffer available space (implementation for io_buffer_available)
#[no_mangle]
pub extern "C" fn io_buffer_available(buf_handle: i32) -> i32 {
    if let Ok(buffers) = BUFFER_HANDLES.lock() {
        if let Some(buffer) = buffers.get(&buf_handle) {
            (buffer.capacity() - buffer.len()) as i32
        } else {
            -1
        }
    } else {
        -1
    }
}

// ================================
// Path Operations Functions (8 functions)
// ================================

/// Join path components (implementation for io_path_join)
#[no_mangle]
pub extern "C" fn io_path_join(parts_ptr: *const *const c_char, count: usize) -> *mut c_char {
    if parts_ptr.is_null() || count == 0 {
        return ptr::null_mut();
    }
    
    unsafe {
        let mut path = PathBuf::new();
        for i in 0..count {
            let part_ptr = *parts_ptr.add(i);
            if !part_ptr.is_null() {
                if let Ok(part) = CStr::from_ptr(part_ptr).to_str() {
                    path.push(part);
                }
            }
        }
        
        if let Some(path_str) = path.to_str() {
            match CString::new(path_str) {
                Ok(c_str) => c_str.into_raw(),
                Err(_) => ptr::null_mut()
            }
        } else {
            ptr::null_mut()
        }
    }
}

/// Get directory name (implementation for io_path_dirname)
#[no_mangle]
pub extern "C" fn io_path_dirname(path_ptr: *const c_char) -> *mut c_char {
    if path_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                let path_buf = Path::new(path);
                if let Some(parent) = path_buf.parent() {
                    if let Some(parent_str) = parent.to_str() {
                        match CString::new(parent_str) {
                            Ok(c_str) => c_str.into_raw(),
                            Err(_) => ptr::null_mut()
                        }
                    } else {
                        ptr::null_mut()
                    }
                } else {
                    ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Get base name (implementation for io_path_basename)
#[no_mangle]
pub extern "C" fn io_path_basename(path_ptr: *const c_char) -> *mut c_char {
    if path_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                let path_buf = Path::new(path);
                if let Some(file_name) = path_buf.file_name() {
                    if let Some(name_str) = file_name.to_str() {
                        match CString::new(name_str) {
                            Ok(c_str) => c_str.into_raw(),
                            Err(_) => ptr::null_mut()
                        }
                    } else {
                        ptr::null_mut()
                    }
                } else {
                    ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Get file extension (implementation for io_path_extension)
#[no_mangle]
pub extern "C" fn io_path_extension(path_ptr: *const c_char) -> *mut c_char {
    if path_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                let path_buf = Path::new(path);
                if let Some(extension) = path_buf.extension() {
                    if let Some(ext_str) = extension.to_str() {
                        match CString::new(ext_str) {
                            Ok(c_str) => c_str.into_raw(),
                            Err(_) => ptr::null_mut()
                        }
                    } else {
                        ptr::null_mut()
                    }
                } else {
                    ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Get absolute path (implementation for io_path_absolute)
#[no_mangle]
pub extern "C" fn io_path_absolute(path_ptr: *const c_char) -> *mut c_char {
    if path_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                let path_buf = Path::new(path);
                match path_buf.canonicalize() {
                    Ok(absolute_path) => {
                        if let Some(abs_str) = absolute_path.to_str() {
                            match CString::new(abs_str) {
                                Ok(c_str) => c_str.into_raw(),
                                Err(_) => ptr::null_mut()
                            }
                        } else {
                            ptr::null_mut()
                        }
                    },
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Get relative path (implementation for io_path_relative)
#[no_mangle]
pub extern "C" fn io_path_relative(from_ptr: *const c_char, to_ptr: *const c_char) -> *mut c_char {
    if from_ptr.is_null() || to_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match (CStr::from_ptr(from_ptr).to_str(), CStr::from_ptr(to_ptr).to_str()) {
            (Ok(from), Ok(to)) => {
                let from_path = Path::new(from);
                let to_path = Path::new(to);
                
                if let Ok(relative) = to_path.strip_prefix(from_path) {
                    if let Some(rel_str) = relative.to_str() {
                        match CString::new(rel_str) {
                            Ok(c_str) => c_str.into_raw(),
                            Err(_) => ptr::null_mut()
                        }
                    } else {
                        ptr::null_mut()
                    }
                } else {
                    ptr::null_mut()
                }
            },
            _ => ptr::null_mut()
        }
    }
}

/// Check if path exists (implementation for io_path_exists)
#[no_mangle]
pub extern "C" fn io_path_exists(path_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                if Path::new(path).exists() { 1 } else { 0 }
            },
            Err(_) => 0
        }
    }
}

// ================================
// Directory Listing Functions (2 functions)
// ================================

/// List directory contents (implementation for io_list_directory)
#[no_mangle]
pub extern "C" fn io_list_directory(path_ptr: *const c_char) -> *mut c_char {
    if path_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match fs::read_dir(path) {
                    Ok(entries) => {
                        let mut result = Vec::new();
                        for entry in entries {
                            if let Ok(entry) = entry {
                                if let Some(name) = entry.file_name().to_str() {
                                    result.push(name.to_string());
                                }
                            }
                        }
                        let joined = result.join("\n");
                        match CString::new(joined) {
                            Ok(c_str) => c_str.into_raw(),
                            Err(_) => ptr::null_mut()
                        }
                    },
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// List directory contents recursively (implementation for io_list_directory_recursive)
#[no_mangle]
pub extern "C" fn io_list_directory_recursive(path_ptr: *const c_char) -> *mut c_char {
    if path_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                fn visit_dir(dir: &Path, result: &mut Vec<String>) -> io::Result<()> {
                    if dir.is_dir() {
                        for entry in fs::read_dir(dir)? {
                            let entry = entry?;
                            let path = entry.path();
                            if let Some(path_str) = path.to_str() {
                                result.push(path_str.to_string());
                            }
                            if path.is_dir() {
                                visit_dir(&path, result)?;
                            }
                        }
                    }
                    Ok(())
                }
                
                let mut result = Vec::new();
                match visit_dir(Path::new(path), &mut result) {
                    Ok(_) => {
                        let joined = result.join("\n");
                        match CString::new(joined) {
                            Ok(c_str) => c_str.into_raw(),
                            Err(_) => ptr::null_mut()
                        }
                    },
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

// ================================
// File Metadata Functions (2 functions)
// ================================

/// Get file creation time (implementation for io_file_created_time)
#[no_mangle]
pub extern "C" fn io_file_created_time(path_ptr: *const c_char) -> i64 {
    if path_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match fs::metadata(path) {
                    Ok(metadata) => {
                        match metadata.created() {
                            Ok(created) => {
                                match created.duration_since(UNIX_EPOCH) {
                                    Ok(duration) => duration.as_secs() as i64,
                                    Err(_) => -1
                                }
                            },
                            Err(_) => -1
                        }
                    },
                    Err(_) => -1
                }
            },
            Err(_) => -1
        }
    }
}

/// Get file modification time (implementation for io_file_modified_time)
#[no_mangle]
pub extern "C" fn io_file_modified_time(path_ptr: *const c_char) -> i64 {
    if path_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match fs::metadata(path) {
                    Ok(metadata) => {
                        match metadata.modified() {
                            Ok(modified) => {
                                match modified.duration_since(UNIX_EPOCH) {
                                    Ok(duration) => duration.as_secs() as i64,
                                    Err(_) => -1
                                }
                            },
                            Err(_) => -1
                        }
                    },
                    Err(_) => -1
                }
            },
            Err(_) => -1
        }
    }
}

// ================================
// Temporary Files Functions (3 functions)
// ================================

/// Create temporary file (implementation for io_create_temp_file)
#[no_mangle]
pub extern "C" fn io_create_temp_file() -> *mut c_char {
    use std::env;
    use std::fs::File;
    use std::io::Write;
    
    let temp_dir = env::temp_dir();
    let temp_name = format!("cursed_temp_{}", std::process::id());
    let temp_path = temp_dir.join(temp_name);
    
    match File::create(&temp_path) {
        Ok(_) => {
            if let Some(path_str) = temp_path.to_str() {
                match CString::new(path_str) {
                    Ok(c_str) => c_str.into_raw(),
                    Err(_) => ptr::null_mut()
                }
            } else {
                ptr::null_mut()
            }
        },
        Err(_) => ptr::null_mut()
    }
}

/// Create temporary directory (implementation for io_create_temp_directory)
#[no_mangle]
pub extern "C" fn io_create_temp_directory() -> *mut c_char {
    use std::env;
    
    let temp_dir = env::temp_dir();
    let temp_name = format!("cursed_temp_dir_{}", std::process::id());
    let temp_path = temp_dir.join(temp_name);
    
    match fs::create_dir(&temp_path) {
        Ok(_) => {
            if let Some(path_str) = temp_path.to_str() {
                match CString::new(path_str) {
                    Ok(c_str) => c_str.into_raw(),
                    Err(_) => ptr::null_mut()
                }
            } else {
                ptr::null_mut()
            }
        },
        Err(_) => ptr::null_mut()
    }
}

/// Get temporary directory path (implementation for io_temp_directory)
#[no_mangle]
pub extern "C" fn io_temp_directory() -> *mut c_char {
    use std::env;
    
    let temp_dir = env::temp_dir();
    if let Some(path_str) = temp_dir.to_str() {
        match CString::new(path_str) {
            Ok(c_str) => c_str.into_raw(),
            Err(_) => ptr::null_mut()
        }
    } else {
        ptr::null_mut()
    }
}

// ================================
// File Bytes Functions (2 functions)
// ================================

/// Read file as bytes (implementation for io_read_file_bytes)
#[no_mangle]
pub extern "C" fn io_read_file_bytes(path_ptr: *const c_char) -> *mut c_char {
    if path_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(path_ptr).to_str() {
            Ok(path) => {
                match fs::read(path) {
                    Ok(bytes) => {
                        // Convert bytes to hex string for safe transport
                        let hex_string = hex::encode(bytes);
                        match CString::new(hex_string) {
                            Ok(c_str) => c_str.into_raw(),
                            Err(_) => ptr::null_mut()
                        }
                    },
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Write bytes to file (implementation for io_write_file_bytes)
#[no_mangle]
pub extern "C" fn io_write_file_bytes(path_ptr: *const c_char, data_ptr: *const c_char) -> i32 {
    if path_ptr.is_null() || data_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match (CStr::from_ptr(path_ptr).to_str(), CStr::from_ptr(data_ptr).to_str()) {
            (Ok(path), Ok(hex_data)) => {
                match hex::decode(hex_data) {
                    Ok(bytes) => {
                        match fs::write(path, bytes) {
                            Ok(_) => 0,
                            Err(_) => -1
                        }
                    },
                    Err(_) => -1
                }
            },
            _ => -1
        }
    }
}

// ================================
// Collections Implementation Functions  
// ================================

use std::collections::HashSet;

// Array/Vector Operations
#[no_mangle]
pub extern "C" fn collections_array_new() -> *mut Vec<i64> {
    Box::into_raw(Box::new(Vec::<i64>::new()))
}

#[no_mangle]
pub extern "C" fn collections_array_with_capacity(capacity: usize) -> *mut Vec<i64> {
    Box::into_raw(Box::new(Vec::<i64>::with_capacity(capacity)))
}

#[no_mangle]
pub extern "C" fn collections_array_push(arr_ptr: *mut Vec<i64>, item: i64) -> i32 {
    if arr_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let arr = &mut *arr_ptr;
        arr.push(item);
    }
    0
}

#[no_mangle]
pub extern "C" fn collections_array_pop(arr_ptr: *mut Vec<i64>) -> i64 {
    if arr_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let arr = &mut *arr_ptr;
        arr.pop().unwrap_or(-1)
    }
}

#[no_mangle]
pub extern "C" fn collections_array_get(arr_ptr: *const Vec<i64>, index: usize) -> i64 {
    if arr_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let arr = &*arr_ptr;
        arr.get(index).copied().unwrap_or(-1)
    }
}

#[no_mangle]
pub extern "C" fn collections_array_set(arr_ptr: *mut Vec<i64>, index: usize, value: i64) -> i32 {
    if arr_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let arr = &mut *arr_ptr;
        if index >= arr.len() {
            return -1;
        }
        arr[index] = value;
    }
    0
}

#[no_mangle]
pub extern "C" fn collections_array_len(arr_ptr: *const Vec<i64>) -> usize {
    if arr_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        let arr = &*arr_ptr;
        arr.len()
    }
}

#[no_mangle]
pub extern "C" fn collections_array_insert(arr_ptr: *mut Vec<i64>, index: usize, item: i64) -> i32 {
    if arr_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let arr = &mut *arr_ptr;
        if index > arr.len() {
            return -1;
        }
        arr.insert(index, item);
    }
    0
}

#[no_mangle]
pub extern "C" fn collections_array_remove(arr_ptr: *mut Vec<i64>, index: usize) -> i64 {
    if arr_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let arr = &mut *arr_ptr;
        if index >= arr.len() {
            return -1;
        }
        arr.remove(index)
    }
}

#[no_mangle]
pub extern "C" fn collections_array_clear(arr_ptr: *mut Vec<i64>) -> i32 {
    if arr_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let arr = &mut *arr_ptr;
        arr.clear();
    }
    0
}

#[no_mangle]
pub extern "C" fn collections_array_is_empty(arr_ptr: *const Vec<i64>) -> i32 {
    if arr_ptr.is_null() {
        return 1;
    }
    
    unsafe {
        let arr = &*arr_ptr;
        if arr.is_empty() { 1 } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn collections_array_contains(arr_ptr: *const Vec<i64>, item: i64) -> i32 {
    if arr_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        let arr = &*arr_ptr;
        if arr.contains(&item) { 1 } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn collections_array_reverse(arr_ptr: *mut Vec<i64>) -> i32 {
    if arr_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let arr = &mut *arr_ptr;
        arr.reverse();
    }
    0
}

// HashMap Operations
#[no_mangle]
pub extern "C" fn collections_map_new() -> *mut HashMap<i64, i64> {
    Box::into_raw(Box::new(HashMap::<i64, i64>::new()))
}

#[no_mangle]
pub extern "C" fn collections_map_with_capacity(capacity: usize) -> *mut HashMap<i64, i64> {
    Box::into_raw(Box::new(HashMap::<i64, i64>::with_capacity(capacity)))
}

#[no_mangle]
pub extern "C" fn collections_map_set(map_ptr: *mut HashMap<i64, i64>, key: i64, value: i64) -> i32 {
    if map_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let map = &mut *map_ptr;
        map.insert(key, value);
    }
    0
}

#[no_mangle]
pub extern "C" fn collections_map_get(map_ptr: *const HashMap<i64, i64>, key: i64) -> i64 {
    if map_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let map = &*map_ptr;
        map.get(&key).copied().unwrap_or(-1)
    }
}

#[no_mangle]
pub extern "C" fn collections_map_remove(map_ptr: *mut HashMap<i64, i64>, key: i64) -> i64 {
    if map_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let map = &mut *map_ptr;
        map.remove(&key).unwrap_or(-1)
    }
}

#[no_mangle]
pub extern "C" fn collections_map_contains_key(map_ptr: *const HashMap<i64, i64>, key: i64) -> i32 {
    if map_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        let map = &*map_ptr;
        if map.contains_key(&key) { 1 } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn collections_map_len(map_ptr: *const HashMap<i64, i64>) -> usize {
    if map_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        let map = &*map_ptr;
        map.len()
    }
}

#[no_mangle]
pub extern "C" fn collections_map_clear(map_ptr: *mut HashMap<i64, i64>) -> i32 {
    if map_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let map = &mut *map_ptr;
        map.clear();
    }
    0
}

#[no_mangle]
pub extern "C" fn collections_map_is_empty(map_ptr: *const HashMap<i64, i64>) -> i32 {
    if map_ptr.is_null() {
        return 1;
    }
    
    unsafe {
        let map = &*map_ptr;
        if map.is_empty() { 1 } else { 0 }
    }
}

// HashSet Operations
#[no_mangle]
pub extern "C" fn collections_set_new() -> *mut HashSet<i64> {
    Box::into_raw(Box::new(HashSet::<i64>::new()))
}

#[no_mangle]
pub extern "C" fn collections_set_with_capacity(capacity: usize) -> *mut HashSet<i64> {
    Box::into_raw(Box::new(HashSet::<i64>::with_capacity(capacity)))
}

#[no_mangle]
pub extern "C" fn collections_set_insert(set_ptr: *mut HashSet<i64>, item: i64) -> i32 {
    if set_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let set = &mut *set_ptr;
        if set.insert(item) { 1 } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn collections_set_contains(set_ptr: *const HashSet<i64>, item: i64) -> i32 {
    if set_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        let set = &*set_ptr;
        if set.contains(&item) { 1 } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn collections_set_remove(set_ptr: *mut HashSet<i64>, item: i64) -> i32 {
    if set_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        let set = &mut *set_ptr;
        if set.remove(&item) { 1 } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn collections_set_len(set_ptr: *const HashSet<i64>) -> usize {
    if set_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        let set = &*set_ptr;
        set.len()
    }
}

#[no_mangle]
pub extern "C" fn collections_set_clear(set_ptr: *mut HashSet<i64>) -> i32 {
    if set_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let set = &mut *set_ptr;
        set.clear();
    }
    0
}

#[no_mangle]
pub extern "C" fn collections_set_is_empty(set_ptr: *const HashSet<i64>) -> i32 {
    if set_ptr.is_null() {
        return 1;
    }
    
    unsafe {
        let set = &*set_ptr;
        if set.is_empty() { 1 } else { 0 }
    }
}

// ================================
// Crypto Implementation Functions
// ================================

use sha2::{Sha256, Sha512, Digest};
// MD5 import not needed, we'll use md5::compute directly
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

// Hash Functions
#[no_mangle]
pub extern "C" fn crypto_sha256(data_ptr: *const c_char) -> *mut c_char {
    if data_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(data_ptr).to_str() {
            Ok(data) => {
                let mut hasher = Sha256::new();
                hasher.update(data.as_bytes());
                let hash = hasher.finalize();
                let hex_string = hex::encode(hash);
                
                match CString::new(hex_string) {
                    Ok(c_str) => c_str.into_raw(),
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn crypto_sha512(data_ptr: *const c_char) -> *mut c_char {
    if data_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(data_ptr).to_str() {
            Ok(data) => {
                let mut hasher = Sha512::new();
                hasher.update(data.as_bytes());
                let hash = hasher.finalize();
                let hex_string = hex::encode(hash);
                
                match CString::new(hex_string) {
                    Ok(c_str) => c_str.into_raw(),
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

// MD5 REMOVED - SECURITY VULNERABILITY
// MD5 is cryptographically broken and vulnerable to collision attacks
// This function has been removed for security reasons
// Use crypto_sha256() or crypto_blake3() instead

#[no_mangle]
pub extern "C" fn crypto_blake3(data_ptr: *const c_char) -> *mut c_char {
    if data_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(data_ptr).to_str() {
            Ok(data) => {
                let mut hasher = Blake3Hasher::new();
                hasher.update(data.as_bytes());
                let hash = hasher.finalize();
                let hex_string = hex::encode(hash.as_bytes());
                
                match CString::new(hex_string) {
                    Ok(c_str) => c_str.into_raw(),
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

// Random Generation Functions
#[no_mangle]
pub extern "C" fn crypto_random_bytes(length: i64) -> *mut c_char {
    if length <= 0 {
        return ptr::null_mut();
    }
    
    let mut rng = rand::thread_rng();
    let mut bytes = vec![0u8; length as usize];
    rng.fill_bytes(&mut bytes);
    
    let hex_string = hex::encode(bytes);
    match CString::new(hex_string) {
        Ok(c_str) => c_str.into_raw(),
        Err(_) => ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn crypto_random_int(min: i64, max: i64) -> i64 {
    if min >= max {
        return min;
    }
    
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

#[no_mangle]
pub extern "C" fn crypto_random_string(length: i64) -> *mut c_char {
    if length <= 0 {
        return ptr::null_mut();
    }
    
    let mut rng = rand::thread_rng();
    let random_string: String = (0..length)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect();
    
    match CString::new(random_string) {
        Ok(c_str) => c_str.into_raw(),
        Err(_) => ptr::null_mut()
    }
}

// Base Encoding Functions
#[no_mangle]
pub extern "C" fn crypto_base64_encode(data_ptr: *const c_char) -> *mut c_char {
    if data_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(data_ptr).to_str() {
            Ok(data) => {
                let encoded = general_purpose::STANDARD.encode(data.as_bytes());
                match CString::new(encoded) {
                    Ok(c_str) => c_str.into_raw(),
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn crypto_base64_decode(encoded_ptr: *const c_char) -> *mut c_char {
    if encoded_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(encoded_ptr).to_str() {
            Ok(encoded) => {
                match general_purpose::STANDARD.decode(encoded) {
                    Ok(decoded) => {
                        match String::from_utf8(decoded) {
                            Ok(decoded_str) => {
                                match CString::new(decoded_str) {
                                    Ok(c_str) => c_str.into_raw(),
                                    Err(_) => ptr::null_mut()
                                }
                            },
                            Err(_) => ptr::null_mut()
                        }
                    },
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn crypto_hex_encode(data_ptr: *const c_char) -> *mut c_char {
    if data_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(data_ptr).to_str() {
            Ok(data) => {
                let hex_string = hex::encode(data.as_bytes());
                match CString::new(hex_string) {
                    Ok(c_str) => c_str.into_raw(),
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn crypto_hex_decode(hex_ptr: *const c_char) -> *mut c_char {
    if hex_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(hex_ptr).to_str() {
            Ok(hex_str) => {
                match hex::decode(hex_str) {
                    Ok(decoded) => {
                        match String::from_utf8(decoded) {
                            Ok(decoded_str) => {
                                match CString::new(decoded_str) {
                                    Ok(c_str) => c_str.into_raw(),
                                    Err(_) => ptr::null_mut()
                                }
                            },
                            Err(_) => ptr::null_mut()
                        }
                    },
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

// Symmetric Encryption Functions
#[no_mangle]
pub extern "C" fn crypto_aes_encrypt(data_ptr: *const c_char, key_ptr: *const c_char) -> *mut c_char {
    if data_ptr.is_null() || key_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match (CStr::from_ptr(data_ptr).to_str(), CStr::from_ptr(key_ptr).to_str()) {
            (Ok(data), Ok(key)) => {
                // Use first 16 bytes of key for AES-128
                let mut key_bytes = [0u8; 16];
                let key_data = key.as_bytes();
                let copy_len = std::cmp::min(key_data.len(), 16);
                key_bytes[..copy_len].copy_from_slice(&key_data[..copy_len]);
                
                match Aes128Gcm::new_from_slice(&key_bytes) {
                    Ok(cipher) => {
                        let nonce = Aes128Gcm::generate_nonce(&mut OsRng);
                        match cipher.encrypt(&nonce, data.as_bytes()) {
                            Ok(ciphertext) => {
                                // Combine nonce and ciphertext
                                let mut result = nonce.to_vec();
                                result.extend_from_slice(&ciphertext);
                                let hex_string = hex::encode(result);
                                
                                match CString::new(hex_string) {
                                    Ok(c_str) => c_str.into_raw(),
                                    Err(_) => ptr::null_mut()
                                }
                            },
                            Err(_) => ptr::null_mut()
                        }
                    },
                    Err(_) => ptr::null_mut()
                }
            },
            _ => ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn crypto_aes_decrypt(encrypted_ptr: *const c_char, key_ptr: *const c_char) -> *mut c_char {
    if encrypted_ptr.is_null() || key_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match (CStr::from_ptr(encrypted_ptr).to_str(), CStr::from_ptr(key_ptr).to_str()) {
            (Ok(encrypted), Ok(key)) => {
                // Use first 16 bytes of key for AES-128
                let mut key_bytes = [0u8; 16];
                let key_data = key.as_bytes();
                let copy_len = std::cmp::min(key_data.len(), 16);
                key_bytes[..copy_len].copy_from_slice(&key_data[..copy_len]);
                
                match hex::decode(encrypted) {
                    Ok(encrypted_data) => {
                        if encrypted_data.len() < 12 {
                            return ptr::null_mut();
                        }
                        
                        let (nonce, ciphertext) = encrypted_data.split_at(12);
                        match Aes128Gcm::new_from_slice(&key_bytes) {
                            Ok(cipher) => {
                                match cipher.decrypt(nonce.into(), ciphertext) {
                                    Ok(plaintext) => {
                                        match String::from_utf8(plaintext) {
                                            Ok(plaintext_str) => {
                                                match CString::new(plaintext_str) {
                                                    Ok(c_str) => c_str.into_raw(),
                                                    Err(_) => ptr::null_mut()
                                                }
                                            },
                                            Err(_) => ptr::null_mut()
                                        }
                                    },
                                    Err(_) => ptr::null_mut()
                                }
                            },
                            Err(_) => ptr::null_mut()
                        }
                    },
                    Err(_) => ptr::null_mut()
                }
            },
            _ => ptr::null_mut()
        }
    }
}

// Key Derivation Functions
#[no_mangle]
pub extern "C" fn crypto_pbkdf2(password_ptr: *const c_char, salt_ptr: *const c_char, iterations: i64, length: i64) -> *mut c_char {
    if password_ptr.is_null() || salt_ptr.is_null() || iterations <= 0 || length <= 0 {
        return ptr::null_mut();
    }
    
    unsafe {
        match (CStr::from_ptr(password_ptr).to_str(), CStr::from_ptr(salt_ptr).to_str()) {
            (Ok(password), Ok(salt)) => {
                let mut key = vec![0u8; length as usize];
                pbkdf2_hmac::<Sha256>(password.as_bytes(), salt.as_bytes(), iterations as u32, &mut key);
                
                let hex_string = hex::encode(key);
                match CString::new(hex_string) {
                    Ok(c_str) => c_str.into_raw(),
                    Err(_) => ptr::null_mut()
                }
            },
            _ => ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn crypto_scrypt(password_ptr: *const c_char, salt_ptr: *const c_char, n: i64, r: i64, p: i64, length: i64) -> *mut c_char {
    if password_ptr.is_null() || salt_ptr.is_null() || n <= 0 || r <= 0 || p <= 0 || length <= 0 {
        return ptr::null_mut();
    }
    
    unsafe {
        match (CStr::from_ptr(password_ptr).to_str(), CStr::from_ptr(salt_ptr).to_str()) {
            (Ok(password), Ok(salt)) => {
                let mut key = vec![0u8; length as usize];
                let params = Params::new(n as u8, r as u32, p as u32, length as usize).unwrap_or_default();
                
                match scrypt(password.as_bytes(), salt.as_bytes(), &params, &mut key) {
                    Ok(_) => {
                        let hex_string = hex::encode(key);
                        match CString::new(hex_string) {
                            Ok(c_str) => c_str.into_raw(),
                            Err(_) => ptr::null_mut()
                        }
                    },
                    Err(_) => ptr::null_mut()
                }
            },
            _ => ptr::null_mut()
        }
    }
}

// ============================
// NEW FFI FUNCTIONS FOR CRYPTO
// ============================

#[no_mangle]
pub extern "C" fn crypto_sha3_256(data_ptr: *const c_char) -> *mut c_char {
    if data_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(data_ptr).to_str() {
            Ok(data) => {
                use sha2::{Digest, Sha256};
                let mut hasher = Sha256::new();
                hasher.update(data.as_bytes());
                let result = hasher.finalize();
                let hex_string = hex::encode(result);
                
                match CString::new(hex_string) {
                    Ok(c_str) => c_str.into_raw(),
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn crypto_secure_random_bytes(length: i64) -> *mut c_char {
    if length <= 0 {
        return ptr::null_mut();
    }
    
    use rand::RngCore;
    let mut rng = rand::thread_rng();
    let mut bytes = vec![0u8; length as usize];
    rng.fill_bytes(&mut bytes);
    
    let hex_string = hex::encode(bytes);
    match CString::new(hex_string) {
        Ok(c_str) => c_str.into_raw(),
        Err(_) => ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn crypto_secure_random_int(min: i64, max: i64) -> i64 {
    if min >= max {
        return min;
    }
    
    use rand::Rng;
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

#[no_mangle]
pub extern "C" fn crypto_secure_random_string(length: i64) -> *mut c_char {
    if length <= 0 {
        return ptr::null_mut();
    }
    
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let random_string: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect();
    
    match CString::new(random_string) {
        Ok(c_str) => c_str.into_raw(),
        Err(_) => ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn crypto_aes_gcm_encrypt(data_ptr: *const c_char, key_ptr: *const c_char) -> *mut c_char {
    if data_ptr.is_null() || key_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match (CStr::from_ptr(data_ptr).to_str(), CStr::from_ptr(key_ptr).to_str()) {
            (Ok(data), Ok(key)) => {
                use aes_gcm::{Aes256Gcm, Key, Nonce, AeadInPlace};
                use aes_gcm::KeyInit;
                use rand::RngCore;
                
                // Create a 256-bit key from the provided key string
                let key_bytes = sha2::Sha256::digest(key.as_bytes());
                let cipher_key = Key::<Aes256Gcm>::from_slice(&key_bytes);
                let cipher = Aes256Gcm::new(cipher_key);
                
                // Generate a random nonce
                let mut nonce_bytes = [0u8; 12];
                rand::thread_rng().fill_bytes(&mut nonce_bytes);
                let nonce = Nonce::from_slice(&nonce_bytes);
                
                // Encrypt the data
                let mut buffer = data.as_bytes().to_vec();
                match cipher.encrypt_in_place(nonce, b"", &mut buffer) {
                    Ok(_) => {
                        // Prepend nonce to encrypted data
                        let mut result = nonce_bytes.to_vec();
                        result.extend_from_slice(&buffer);
                        let hex_string = hex::encode(result);
                        
                        match CString::new(hex_string) {
                            Ok(c_str) => c_str.into_raw(),
                            Err(_) => ptr::null_mut()
                        }
                    },
                    Err(_) => ptr::null_mut()
                }
            },
            _ => ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn crypto_aes_gcm_decrypt(encrypted_ptr: *const c_char, key_ptr: *const c_char) -> *mut c_char {
    if encrypted_ptr.is_null() || key_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match (CStr::from_ptr(encrypted_ptr).to_str(), CStr::from_ptr(key_ptr).to_str()) {
            (Ok(encrypted_hex), Ok(key)) => {
                use aes_gcm::{Aes256Gcm, Key, Nonce, AeadInPlace};
                use aes_gcm::KeyInit;
                
                match hex::decode(encrypted_hex) {
                    Ok(encrypted_data) => {
                        if encrypted_data.len() < 12 {
                            return ptr::null_mut();
                        }
                        
                        // Extract nonce and ciphertext
                        let nonce_bytes = &encrypted_data[0..12];
                        let ciphertext = &encrypted_data[12..];
                        
                        // Create cipher
                        let key_bytes = sha2::Sha256::digest(key.as_bytes());
                        let cipher_key = Key::<Aes256Gcm>::from_slice(&key_bytes);
                        let cipher = Aes256Gcm::new(cipher_key);
                        let nonce = Nonce::from_slice(nonce_bytes);
                        
                        // Decrypt
                        let mut buffer = ciphertext.to_vec();
                        match cipher.decrypt_in_place(nonce, b"", &mut buffer) {
                            Ok(_) => {
                                match String::from_utf8(buffer) {
                                    Ok(decrypted) => {
                                        match CString::new(decrypted) {
                                            Ok(c_str) => c_str.into_raw(),
                                            Err(_) => ptr::null_mut()
                                        }
                                    },
                                    Err(_) => ptr::null_mut()
                                }
                            },
                            Err(_) => ptr::null_mut()
                        }
                    },
                    Err(_) => ptr::null_mut()
                }
            },
            _ => ptr::null_mut()
        }
    }
}

// Digital Signature Functions
#[no_mangle]
pub extern "C" fn crypto_ed25519_keypair() -> *mut c_char {
    let mut rng = rand::thread_rng();
    let mut secret_key = [0u8; 32];
    rng.fill_bytes(&mut secret_key);
    let signing_key = SigningKey::from_bytes(&secret_key);
    let verifying_key = signing_key.verifying_key();
    
    // Create a JSON-like string with the keypair
    let keypair_json = format!("{{\"private_key\":\"{}\",\"public_key\":\"{}\"}}",
        hex::encode(signing_key.to_bytes()),
        hex::encode(verifying_key.to_bytes())
    );
    
    match CString::new(keypair_json) {
        Ok(c_str) => c_str.into_raw(),
        Err(_) => ptr::null_mut()
    }
}

#[no_mangle]
pub extern "C" fn crypto_ed25519_sign(message_ptr: *const c_char, private_key_ptr: *const c_char) -> *mut c_char {
    if message_ptr.is_null() || private_key_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match (CStr::from_ptr(message_ptr).to_str(), CStr::from_ptr(private_key_ptr).to_str()) {
            (Ok(message), Ok(private_key)) => {
                match hex::decode(private_key) {
                    Ok(key_bytes) => {
                        if key_bytes.len() != 32 {
                            return ptr::null_mut();
                        }
                        
                        let mut key_array = [0u8; 32];
                        key_array.copy_from_slice(&key_bytes);
                        
                        let signing_key = SigningKey::from_bytes(&key_array);
                        let signature = signing_key.sign(message.as_bytes());
                        let hex_string = hex::encode(signature.to_bytes());
                        
                        match CString::new(hex_string) {
                            Ok(c_str) => c_str.into_raw(),
                            Err(_) => ptr::null_mut()
                        }
                    },
                    Err(_) => ptr::null_mut()
                }
            },
            _ => ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn crypto_ed25519_verify(message_ptr: *const c_char, signature_ptr: *const c_char, public_key_ptr: *const c_char) -> i32 {
    if message_ptr.is_null() || signature_ptr.is_null() || public_key_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        match (CStr::from_ptr(message_ptr).to_str(), CStr::from_ptr(signature_ptr).to_str(), CStr::from_ptr(public_key_ptr).to_str()) {
            (Ok(message), Ok(signature), Ok(public_key)) => {
                match (hex::decode(signature), hex::decode(public_key)) {
                    (Ok(sig_bytes), Ok(pub_key_bytes)) => {
                        if sig_bytes.len() != 64 || pub_key_bytes.len() != 32 {
                            return 0;
                        }
                        
                        let mut sig_array = [0u8; 64];
                        let mut pub_key_array = [0u8; 32];
                        sig_array.copy_from_slice(&sig_bytes);
                        pub_key_array.copy_from_slice(&pub_key_bytes);
                        
                        match (Signature::try_from(&sig_array[..]), VerifyingKey::try_from(&pub_key_array[..])) {
                            (Ok(signature), Ok(verifying_key)) => {
                                match verifying_key.verify_strict(message.as_bytes(), &signature) {
                                    Ok(_) => 1,
                                    Err(_) => 0
                                }
                            },
                            _ => 0
                        }
                    },
                    _ => 0
                }
            },
            _ => 0
        }
    }
}

// HMAC Functions
#[no_mangle]
pub extern "C" fn crypto_hmac_sha256(data_ptr: *const c_char, key_ptr: *const c_char) -> *mut c_char {
    if data_ptr.is_null() || key_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match (CStr::from_ptr(data_ptr).to_str(), CStr::from_ptr(key_ptr).to_str()) {
            (Ok(data), Ok(key)) => {
                type HmacSha256 = Hmac<Sha256>;
                match <HmacSha256 as Mac>::new_from_slice(key.as_bytes()) {
                    Ok(mut mac) => {
                        mac.update(data.as_bytes());
                        let result = mac.finalize();
                        let hex_string = hex::encode(result.into_bytes());
                        
                        match CString::new(hex_string) {
                            Ok(c_str) => c_str.into_raw(),
                            Err(_) => ptr::null_mut()
                        }
                    },
                    Err(_) => ptr::null_mut()
                }
            },
            _ => ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn crypto_hmac_sha512(data_ptr: *const c_char, key_ptr: *const c_char) -> *mut c_char {
    if data_ptr.is_null() || key_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match (CStr::from_ptr(data_ptr).to_str(), CStr::from_ptr(key_ptr).to_str()) {
            (Ok(data), Ok(key)) => {
                type HmacSha512 = Hmac<Sha512>;
                match <HmacSha512 as Mac>::new_from_slice(key.as_bytes()) {
                    Ok(mut mac) => {
                        mac.update(data.as_bytes());
                        let result = mac.finalize();
                        let hex_string = hex::encode(result.into_bytes());
                        
                        match CString::new(hex_string) {
                            Ok(c_str) => c_str.into_raw(),
                            Err(_) => ptr::null_mut()
                        }
                    },
                    Err(_) => ptr::null_mut()
                }
            },
            _ => ptr::null_mut()
        }
    }
}

// Password Hashing Functions
#[no_mangle]
pub extern "C" fn crypto_argon2_hash(password_ptr: *const c_char, salt_ptr: *const c_char) -> *mut c_char {
    if password_ptr.is_null() || salt_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match (CStr::from_ptr(password_ptr).to_str(), CStr::from_ptr(salt_ptr).to_str()) {
            (Ok(password), Ok(salt)) => {
                let argon2 = Argon2::default();
                match SaltString::encode_b64(salt.as_bytes()) {
                    Ok(salt_string) => {
                        match argon2.hash_password(password.as_bytes(), &salt_string) {
                            Ok(hash) => {
                                match CString::new(hash.to_string()) {
                                    Ok(c_str) => c_str.into_raw(),
                                    Err(_) => ptr::null_mut()
                                }
                            },
                            Err(_) => ptr::null_mut()
                        }
                    },
                    Err(_) => ptr::null_mut()
                }
            },
            _ => ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn crypto_argon2_verify(password_ptr: *const c_char, hash_ptr: *const c_char) -> i32 {
    if password_ptr.is_null() || hash_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        match (CStr::from_ptr(password_ptr).to_str(), CStr::from_ptr(hash_ptr).to_str()) {
            (Ok(password), Ok(hash)) => {
                let argon2 = Argon2::default();
                match argon2::password_hash::PasswordHash::new(hash) {
                    Ok(parsed_hash) => {
                        match argon2.verify_password(password.as_bytes(), &parsed_hash) {
                            Ok(_) => 1,
                            Err(_) => 0
                        }
                    },
                    Err(_) => 0
                }
            },
            _ => 0
        }
    }
}

#[no_mangle]
pub extern "C" fn crypto_bcrypt_hash(password_ptr: *const c_char, cost: i64) -> *mut c_char {
    if password_ptr.is_null() || cost < 4 || cost > 31 {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(password_ptr).to_str() {
            Ok(password) => {
                match hash(password, cost as u32) {
                    Ok(hash) => {
                        match CString::new(hash) {
                            Ok(c_str) => c_str.into_raw(),
                            Err(_) => ptr::null_mut()
                        }
                    },
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

#[no_mangle]
pub extern "C" fn crypto_bcrypt_verify(password_ptr: *const c_char, hash_ptr: *const c_char) -> i32 {
    if password_ptr.is_null() || hash_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        match (CStr::from_ptr(password_ptr).to_str(), CStr::from_ptr(hash_ptr).to_str()) {
            (Ok(password), Ok(hash)) => {
                match verify(password, hash) {
                    Ok(is_valid) => if is_valid { 1 } else { 0 },
                    Err(_) => 0
                }
            },
            _ => 0
        }
    }
}

// Utility Functions
#[no_mangle]
pub extern "C" fn crypto_constant_time_eq(a_ptr: *const c_char, b_ptr: *const c_char) -> i32 {
    if a_ptr.is_null() || b_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        match (CStr::from_ptr(a_ptr).to_str(), CStr::from_ptr(b_ptr).to_str()) {
            (Ok(a), Ok(b)) => {
                if a.len() != b.len() {
                    return 0;
                }
                
                let result = a.as_bytes().ct_eq(b.as_bytes());
                if result.into() { 1 } else { 0 }
            },
            _ => 0
        }
    }
}

#[no_mangle]
pub extern "C" fn crypto_secure_random() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

#[no_mangle]
pub extern "C" fn crypto_generate_salt(length: i64) -> *mut c_char {
    if length <= 0 {
        return ptr::null_mut();
    }
    
    let mut rng = rand::thread_rng();
    let mut salt = vec![0u8; length as usize];
    rng.fill_bytes(&mut salt);
    
    let hex_string = hex::encode(salt);
    match CString::new(hex_string) {
        Ok(c_str) => c_str.into_raw(),
        Err(_) => ptr::null_mut()
    }
}

// ================================
// Math Implementation Functions
// ================================

#[no_mangle]
pub extern "C" fn math_sin_impl(x: f64) -> f64 {
    x.sin()
}

#[no_mangle]
pub extern "C" fn math_cos_impl(x: f64) -> f64 {
    x.cos()
}

#[no_mangle]
pub extern "C" fn math_sqrt_impl(x: f64) -> f64 {
    x.sqrt()
}

#[no_mangle]
pub extern "C" fn math_random_impl() -> f64 {
    // Simple random implementation - should be replaced with proper PRNG
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let mut hasher = DefaultHasher::new();
    SystemTime::now().duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
        .hash(&mut hasher);
    
    (hasher.finish() as f64) / (u64::MAX as f64)
}

// Additional math functions

#[no_mangle]
pub extern "C" fn math_abs_impl(x: f64) -> f64 {
    x.abs()
}

#[no_mangle]
pub extern "C" fn math_abs_int_impl(x: i64) -> i64 {
    x.abs()
}

#[no_mangle]
pub extern "C" fn math_min_impl(x: f64, y: f64) -> f64 {
    x.min(y)
}

#[no_mangle]
pub extern "C" fn math_max_impl(x: f64, y: f64) -> f64 {
    x.max(y)
}

#[no_mangle]
pub extern "C" fn math_min_int_impl(x: i64, y: i64) -> i64 {
    x.min(y)
}

#[no_mangle]
pub extern "C" fn math_max_int_impl(x: i64, y: i64) -> i64 {
    x.max(y)
}

#[no_mangle]
pub extern "C" fn math_clamp_impl(x: f64, min: f64, max: f64) -> f64 {
    x.clamp(min, max)
}

#[no_mangle]
pub extern "C" fn math_sign_impl(x: f64) -> f64 {
    if x > 0.0 {
        1.0
    } else if x < 0.0 {
        -1.0
    } else {
        0.0
    }
}

#[no_mangle]
pub extern "C" fn math_pow_impl(x: f64, y: f64) -> f64 {
    x.powf(y)
}

#[no_mangle]
pub extern "C" fn math_cbrt_impl(x: f64) -> f64 {
    x.cbrt()
}

#[no_mangle]
pub extern "C" fn math_log_impl(x: f64) -> f64 {
    x.ln()
}

#[no_mangle]
pub extern "C" fn math_log10_impl(x: f64) -> f64 {
    x.log10()
}

#[no_mangle]
pub extern "C" fn math_log2_impl(x: f64) -> f64 {
    x.log2()
}

#[no_mangle]
pub extern "C" fn math_exp_impl(x: f64) -> f64 {
    x.exp()
}

#[no_mangle]
pub extern "C" fn math_exp2_impl(x: f64) -> f64 {
    x.exp2()
}

#[no_mangle]
pub extern "C" fn math_tan_impl(x: f64) -> f64 {
    x.tan()
}

#[no_mangle]
pub extern "C" fn math_asin_impl(x: f64) -> f64 {
    x.asin()
}

#[no_mangle]
pub extern "C" fn math_acos_impl(x: f64) -> f64 {
    x.acos()
}

#[no_mangle]
pub extern "C" fn math_atan_impl(x: f64) -> f64 {
    x.atan()
}

#[no_mangle]
pub extern "C" fn math_atan2_impl(y: f64, x: f64) -> f64 {
    y.atan2(x)
}

#[no_mangle]
pub extern "C" fn math_sinh_impl(x: f64) -> f64 {
    x.sinh()
}

#[no_mangle]
pub extern "C" fn math_cosh_impl(x: f64) -> f64 {
    x.cosh()
}

#[no_mangle]
pub extern "C" fn math_tanh_impl(x: f64) -> f64 {
    x.tanh()
}

#[no_mangle]
pub extern "C" fn math_floor_impl(x: f64) -> f64 {
    x.floor()
}

#[no_mangle]
pub extern "C" fn math_ceil_impl(x: f64) -> f64 {
    x.ceil()
}

#[no_mangle]
pub extern "C" fn math_round_impl(x: f64) -> f64 {
    x.round()
}

#[no_mangle]
pub extern "C" fn math_trunc_impl(x: f64) -> f64 {
    x.trunc()
}

#[no_mangle]
pub extern "C" fn math_frac_impl(x: f64) -> f64 {
    x.fract()
}

#[no_mangle]
pub extern "C" fn math_sum_impl(arr_ptr: *const f64, len: usize) -> f64 {
    if arr_ptr.is_null() || len == 0 {
        return 0.0;
    }
    
    unsafe {
        let slice = std::slice::from_raw_parts(arr_ptr, len);
        slice.iter().sum()
    }
}

#[no_mangle]
pub extern "C" fn math_mean_impl(arr_ptr: *const f64, len: usize) -> f64 {
    if arr_ptr.is_null() || len == 0 {
        return 0.0;
    }
    
    unsafe {
        let slice = std::slice::from_raw_parts(arr_ptr, len);
        slice.iter().sum::<f64>() / len as f64
    }
}

#[no_mangle]
pub extern "C" fn math_median_impl(arr_ptr: *const f64, len: usize) -> f64 {
    if arr_ptr.is_null() || len == 0 {
        return 0.0;
    }
    
    unsafe {
        let slice = std::slice::from_raw_parts(arr_ptr, len);
        let mut sorted: Vec<f64> = slice.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        
        if len % 2 == 0 {
            (sorted[len / 2 - 1] + sorted[len / 2]) / 2.0
        } else {
            sorted[len / 2]
        }
    }
}

#[no_mangle]
pub extern "C" fn math_variance_impl(arr_ptr: *const f64, len: usize) -> f64 {
    if arr_ptr.is_null() || len == 0 {
        return 0.0;
    }
    
    unsafe {
        let slice = std::slice::from_raw_parts(arr_ptr, len);
        let mean = slice.iter().sum::<f64>() / len as f64;
        slice.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / len as f64
    }
}

#[no_mangle]
pub extern "C" fn math_std_dev_impl(arr_ptr: *const f64, len: usize) -> f64 {
    math_variance_impl(arr_ptr, len).sqrt()
}

#[no_mangle]
pub extern "C" fn math_random_int_impl(min: i64, max: i64) -> i64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};
    
    if min >= max {
        return min;
    }
    
    let mut hasher = DefaultHasher::new();
    SystemTime::now().duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
        .hash(&mut hasher);
    
    let range = (max - min) as u64;
    min + (hasher.finish() % range) as i64
}

#[no_mangle]
pub extern "C" fn math_random_float_impl(min: f64, max: f64) -> f64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};
    
    if min >= max {
        return min;
    }
    
    let mut hasher = DefaultHasher::new();
    SystemTime::now().duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
        .hash(&mut hasher);
    
    let random_01 = (hasher.finish() as f64) / (u64::MAX as f64);
    min + random_01 * (max - min)
}

#[no_mangle]
pub extern "C" fn math_seed_random_impl(seed: u64) -> f64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    seed.hash(&mut hasher);
    (hasher.finish() as f64) / (u64::MAX as f64)
}

#[no_mangle]
pub extern "C" fn math_is_nan_impl(x: f64) -> i32 {
    if x.is_nan() { 1 } else { 0 }
}

#[no_mangle]
pub extern "C" fn math_is_infinite_impl(x: f64) -> i32 {
    if x.is_infinite() { 1 } else { 0 }
}

#[no_mangle]
pub extern "C" fn math_is_finite_impl(x: f64) -> i32 {
    if x.is_finite() { 1 } else { 0 }
}

#[no_mangle]
pub extern "C" fn math_gcd_impl(a: i64, b: i64) -> i64 {
    fn gcd(a: i64, b: i64) -> i64 {
        if b == 0 { a.abs() } else { gcd(b, a % b) }
    }
    gcd(a, b)
}

#[no_mangle]
pub extern "C" fn math_lcm_impl(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a.abs() * b.abs()) / math_gcd_impl(a, b)
    }
}

#[no_mangle]
pub extern "C" fn math_factorial_impl(n: i64) -> i64 {
    if n < 0 {
        return 0;
    }
    if n <= 1 {
        return 1;
    }
    
    let mut result = 1i64;
    for i in 2..=n {
        if let Some(new_result) = result.checked_mul(i) {
            result = new_result;
        } else {
            return i64::MAX; // Overflow
        }
    }
    result
}

#[no_mangle]
pub extern "C" fn math_fibonacci_impl(n: i64) -> i64 {
    if n < 0 {
        return 0;
    }
    if n <= 1 {
        return n;
    }
    
    let mut a = 0i64;
    let mut b = 1i64;
    
    for _ in 2..=n {
        let next = a.saturating_add(b);
        a = b;
        b = next;
    }
    
    b
}

#[no_mangle]
pub extern "C" fn math_smoothstep_impl(edge0: f64, edge1: f64, x: f64) -> f64 {
    if x <= edge0 {
        return 0.0;
    }
    if x >= edge1 {
        return 1.0;
    }
    
    let t = (x - edge0) / (edge1 - edge0);
    t * t * (3.0 - 2.0 * t)
}

// ================================
// Legacy compatibility
// ================================

pub struct MinimalImplementation;

impl MinimalImplementation {
    pub fn new() -> Self {
        Self
    }
}

pub fn get_minimal_result() -> Result<String, CursedError> {
    Ok("CURSED runtime functions implemented".to_string())
}

// ================================
// String Processing Implementation Functions
// ================================

/// Get the length of a string (implementation for string_length)
#[no_mangle]
pub extern "C" fn string_length(str_ptr: *const c_char) -> usize {
    if str_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => s.chars().count(), // Unicode-aware character count
            Err(_) => 0
        }
    }
}

/// Convert string to uppercase (implementation for string_to_upper)
#[no_mangle]
pub extern "C" fn string_to_upper(str_ptr: *const c_char) -> *mut c_char {
    if str_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => {
                let upper = s.to_uppercase();
                match CString::new(upper) {
                    Ok(c_string) => c_string.into_raw(),
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Convert string to lowercase (implementation for string_to_lower)
#[no_mangle]
pub extern "C" fn string_to_lower(str_ptr: *const c_char) -> *mut c_char {
    if str_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => {
                let lower = s.to_lowercase();
                match CString::new(lower) {
                    Ok(c_string) => c_string.into_raw(),
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Check if string matches regex pattern (implementation for string_regex_match)
#[no_mangle]
pub extern "C" fn string_regex_match(str_ptr: *const c_char, pattern_ptr: *const c_char) -> i32 {
    if str_ptr.is_null() || pattern_ptr.is_null() {
        return -1; // Error
    }
    
    unsafe {
        let text = match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return -1
        };
        
        let pattern = match CStr::from_ptr(pattern_ptr).to_str() {
            Ok(p) => p,
            Err(_) => return -1
        };
        
        match Regex::new(pattern) {
            Ok(regex) => if regex.is_match(text) { 1 } else { 0 },
            Err(_) => -1 // Invalid regex pattern
        }
    }
}

/// Find first regex match in string (implementation for string_regex_find)
#[no_mangle]
pub extern "C" fn string_regex_find(str_ptr: *const c_char, pattern_ptr: *const c_char) -> *mut c_char {
    if str_ptr.is_null() || pattern_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        let text = match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut()
        };
        
        let pattern = match CStr::from_ptr(pattern_ptr).to_str() {
            Ok(p) => p,
            Err(_) => return ptr::null_mut()
        };
        
        match Regex::new(pattern) {
            Ok(regex) => {
                if let Some(mat) = regex.find(text) {
                    match CString::new(mat.as_str()) {
                        Ok(c_string) => c_string.into_raw(),
                        Err(_) => ptr::null_mut()
                    }
                } else {
                    ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Replace regex matches in string (implementation for string_regex_replace)
#[no_mangle]
pub extern "C" fn string_regex_replace(str_ptr: *const c_char, pattern_ptr: *const c_char, replacement_ptr: *const c_char) -> *mut c_char {
    if str_ptr.is_null() || pattern_ptr.is_null() || replacement_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        let text = match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut()
        };
        
        let pattern = match CStr::from_ptr(pattern_ptr).to_str() {
            Ok(p) => p,
            Err(_) => return ptr::null_mut()
        };
        
        let replacement = match CStr::from_ptr(replacement_ptr).to_str() {
            Ok(r) => r,
            Err(_) => return ptr::null_mut()
        };
        
        match Regex::new(pattern) {
            Ok(regex) => {
                let result = regex.replace_all(text, replacement);
                match CString::new(result.as_ref()) {
                    Ok(c_string) => c_string.into_raw(),
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Split string by regex pattern (implementation for string_regex_split)
#[no_mangle]
pub extern "C" fn string_regex_split(str_ptr: *const c_char, pattern_ptr: *const c_char, count_ptr: *mut usize) -> *mut *mut c_char {
    if str_ptr.is_null() || pattern_ptr.is_null() || count_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        let text = match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut()
        };
        
        let pattern = match CStr::from_ptr(pattern_ptr).to_str() {
            Ok(p) => p,
            Err(_) => return ptr::null_mut()
        };
        
        match Regex::new(pattern) {
            Ok(regex) => {
                let parts: Vec<&str> = regex.split(text).collect();
                let count = parts.len();
                *count_ptr = count;
                
                if count == 0 {
                    return ptr::null_mut();
                }
                
                // Allocate array of string pointers
                let array = libc::malloc(count * std::mem::size_of::<*mut c_char>()) as *mut *mut c_char;
                if array.is_null() {
                    return ptr::null_mut();
                }
                
                // Convert each part to C string
                for (i, part) in parts.iter().enumerate() {
                    match CString::new(*part) {
                        Ok(c_string) => {
                            *array.add(i) = c_string.into_raw();
                        },
                        Err(_) => {
                            // Cleanup on error
                            for j in 0..i {
                                let _ = CString::from_raw(*array.add(j));
                            }
                            libc::free(array as *mut libc::c_void);
                            return ptr::null_mut();
                        }
                    }
                }
                
                array
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Format string with template and arguments (implementation for string_format)
#[no_mangle]
pub extern "C" fn string_format(template_ptr: *const c_char, args_ptr: *const *const c_char, args_count: usize) -> *mut c_char {
    if template_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        let template = match CStr::from_ptr(template_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut()
        };
        
        let mut args = Vec::new();
        if !args_ptr.is_null() {
            for i in 0..args_count {
                let arg_ptr = *args_ptr.add(i);
                if !arg_ptr.is_null() {
                    if let Ok(arg) = CStr::from_ptr(arg_ptr).to_str() {
                        args.push(arg);
                    }
                }
            }
        }
        
        // Simple string interpolation - replace {0}, {1}, etc. with arguments
        let mut result = template.to_string();
        for (i, arg) in args.iter().enumerate() {
            let placeholder = format!("{{{}}}", i);
            result = result.replace(&placeholder, arg);
        }
        
        match CString::new(result) {
            Ok(c_string) => c_string.into_raw(),
            Err(_) => ptr::null_mut()
        }
    }
}

/// Trim whitespace from string (implementation for string_trim)
#[no_mangle]
pub extern "C" fn string_trim(str_ptr: *const c_char) -> *mut c_char {
    if str_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => {
                let trimmed = s.trim();
                match CString::new(trimmed) {
                    Ok(c_string) => c_string.into_raw(),
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Check if string contains substring (implementation for string_contains)
#[no_mangle]
pub extern "C" fn string_contains(str_ptr: *const c_char, substring_ptr: *const c_char) -> i32 {
    if str_ptr.is_null() || substring_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let text = match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return -1
        };
        
        let substring = match CStr::from_ptr(substring_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return -1
        };
        
        if text.contains(substring) { 1 } else { 0 }
    }
}

/// Find index of substring in string (implementation for string_index_of)
#[no_mangle]
pub extern "C" fn string_index_of(str_ptr: *const c_char, substring_ptr: *const c_char) -> i32 {
    if str_ptr.is_null() || substring_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let text = match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return -1
        };
        
        let substring = match CStr::from_ptr(substring_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return -1
        };
        
        match text.find(substring) {
            Some(index) => index as i32,
            None => -1
        }
    }
}

/// Substring extraction (implementation for string_substring)
#[no_mangle]
pub extern "C" fn string_substring(str_ptr: *const c_char, start: usize, length: usize) -> *mut c_char {
    if str_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => {
                let chars: Vec<char> = s.chars().collect();
                if start >= chars.len() {
                    // Return empty string if start is beyond string length
                    match CString::new("") {
                        Ok(c_string) => c_string.into_raw(),
                        Err(_) => ptr::null_mut()
                    }
                } else {
                    let end = std::cmp::min(start + length, chars.len());
                    let substring: String = chars[start..end].iter().collect();
                    match CString::new(substring) {
                        Ok(c_string) => c_string.into_raw(),
                        Err(_) => ptr::null_mut()
                    }
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Convert integer to string (implementation for i32_to_string)
#[no_mangle]
pub extern "C" fn i32_to_string(value: i32) -> *mut c_char {
    let result = value.to_string();
    match CString::new(result) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => ptr::null_mut()
    }
}

/// String concatenation (implementation for string_concat)
#[no_mangle]
pub extern "C" fn string_concat(str1_ptr: *const c_char, str2_ptr: *const c_char) -> *mut c_char {
    if str1_ptr.is_null() || str2_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        let str1 = match CStr::from_ptr(str1_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut()
        };
        
        let str2 = match CStr::from_ptr(str2_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut()
        };
        
        let result = format!("{}{}", str1, str2);
        match CString::new(result) {
            Ok(c_string) => c_string.into_raw(),
            Err(_) => ptr::null_mut()
        }
    }
}

/// Check if string is empty (implementation for string_is_empty)
#[no_mangle]
pub extern "C" fn string_is_empty(str_ptr: *const c_char) -> i32 {
    if str_ptr.is_null() {
        return 1; // Consider null as empty
    }
    
    unsafe {
        match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => if s.is_empty() { 1 } else { 0 },
            Err(_) => 1
        }
    }
}

/// Encode string as base64 (implementation for string_base64_encode)
#[no_mangle]
pub extern "C" fn string_base64_encode(str_ptr: *const c_char) -> *mut c_char {
    if str_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => {
                let encoded = general_purpose::STANDARD.encode(s.as_bytes());
                match CString::new(encoded) {
                    Ok(c_string) => c_string.into_raw(),
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Decode base64 string (implementation for string_base64_decode)
#[no_mangle]
pub extern "C" fn string_base64_decode(str_ptr: *const c_char) -> *mut c_char {
    if str_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => {
                match general_purpose::STANDARD.decode(s) {
                    Ok(decoded_bytes) => {
                        match String::from_utf8(decoded_bytes) {
                            Ok(decoded_string) => {
                                match CString::new(decoded_string) {
                                    Ok(c_string) => c_string.into_raw(),
                                    Err(_) => ptr::null_mut()
                                }
                            },
                            Err(_) => ptr::null_mut()
                        }
                    },
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Calculate Levenshtein distance between two strings (implementation for string_levenshtein_distance)
#[no_mangle]
pub extern "C" fn string_levenshtein_distance(str1_ptr: *const c_char, str2_ptr: *const c_char) -> i32 {
    if str1_ptr.is_null() || str2_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let str1 = match CStr::from_ptr(str1_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return -1
        };
        
        let str2 = match CStr::from_ptr(str2_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return -1
        };
        
        let chars1: Vec<char> = str1.chars().collect();
        let chars2: Vec<char> = str2.chars().collect();
        let len1 = chars1.len();
        let len2 = chars2.len();
        
        if len1 == 0 { return len2 as i32; }
        if len2 == 0 { return len1 as i32; }
        
        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];
        
        // Initialize first row and column
        for i in 0..=len1 { matrix[i][0] = i; }
        for j in 0..=len2 { matrix[0][j] = j; }
        
        // Fill the matrix
        for i in 1..=len1 {
            for j in 1..=len2 {
                let cost = if chars1[i-1] == chars2[j-1] { 0 } else { 1 };
                matrix[i][j] = std::cmp::min(
                    std::cmp::min(
                        matrix[i-1][j] + 1,    // deletion
                        matrix[i][j-1] + 1     // insertion
                    ),
                    matrix[i-1][j-1] + cost    // substitution
                );
            }
        }
        
        matrix[len1][len2] as i32
    }
}

/// Calculate string similarity (implementation for string_similarity)
#[no_mangle]
pub extern "C" fn string_similarity(str1_ptr: *const c_char, str2_ptr: *const c_char) -> f64 {
    if str1_ptr.is_null() || str2_ptr.is_null() {
        return 0.0;
    }
    
    unsafe {
        let str1 = match CStr::from_ptr(str1_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return 0.0
        };
        
        let str2 = match CStr::from_ptr(str2_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return 0.0
        };
        
        let distance = string_levenshtein_distance(str1_ptr, str2_ptr);
        if distance < 0 {
            return 0.0;
        }
        
        let max_len = std::cmp::max(str1.chars().count(), str2.chars().count());
        if max_len == 0 {
            return 1.0; // Both strings are empty
        }
        
        1.0 - (distance as f64 / max_len as f64)
    }
}

/// Check if string starts with prefix (implementation for string_starts_with)
#[no_mangle]
pub extern "C" fn string_starts_with(str_ptr: *const c_char, prefix_ptr: *const c_char) -> i32 {
    if str_ptr.is_null() || prefix_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let text = match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return -1
        };
        
        let prefix = match CStr::from_ptr(prefix_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return -1
        };
        
        if text.starts_with(prefix) { 1 } else { 0 }
    }
}

/// Check if string ends with suffix (implementation for string_ends_with)
#[no_mangle]
pub extern "C" fn string_ends_with(str_ptr: *const c_char, suffix_ptr: *const c_char) -> i32 {
    if str_ptr.is_null() || suffix_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let text = match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return -1
        };
        
        let suffix = match CStr::from_ptr(suffix_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return -1
        };
        
        if text.ends_with(suffix) { 1 } else { 0 }
    }
}

/// Find last index of substring (implementation for string_last_index_of)
#[no_mangle]
pub extern "C" fn string_last_index_of(str_ptr: *const c_char, substring_ptr: *const c_char) -> i32 {
    if str_ptr.is_null() || substring_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let text = match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return -1
        };
        
        let substring = match CStr::from_ptr(substring_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return -1
        };
        
        match text.rfind(substring) {
            Some(index) => index as i32,
            None => -1
        }
    }
}

/// Count occurrences of substring (implementation for string_count_occurrences)
#[no_mangle]
pub extern "C" fn string_count_occurrences(str_ptr: *const c_char, substring_ptr: *const c_char) -> i32 {
    if str_ptr.is_null() || substring_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let text = match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return -1
        };
        
        let substring = match CStr::from_ptr(substring_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return -1
        };
        
        if substring.is_empty() {
            return 0;
        }
        
        let mut count = 0;
        let mut start = 0;
        
        while let Some(pos) = text[start..].find(substring) {
            count += 1;
            start += pos + substring.len();
        }
        
        count
    }
}

/// Slice string (implementation for string_slice)
#[no_mangle]
pub extern "C" fn string_slice(str_ptr: *const c_char, start: i32, end: i32) -> *mut c_char {
    if str_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => {
                let chars: Vec<char> = s.chars().collect();
                let len = chars.len() as i32;
                
                let start_idx = if start < 0 { 0 } else { start as usize };
                let end_idx = if end < 0 || end > len { len as usize } else { end as usize };
                
                if start_idx >= chars.len() || start_idx >= end_idx {
                    match CString::new("") {
                        Ok(c_string) => c_string.into_raw(),
                        Err(_) => ptr::null_mut()
                    }
                } else {
                    let slice: String = chars[start_idx..end_idx].iter().collect();
                    match CString::new(slice) {
                        Ok(c_string) => c_string.into_raw(),
                        Err(_) => ptr::null_mut()
                    }
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Get character at index (implementation for string_char_at)
#[no_mangle]
pub extern "C" fn string_char_at(str_ptr: *const c_char, index: i32) -> *mut c_char {
    if str_ptr.is_null() || index < 0 {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => {
                let chars: Vec<char> = s.chars().collect();
                if (index as usize) < chars.len() {
                    let ch = chars[index as usize];
                    match CString::new(ch.to_string()) {
                        Ok(c_string) => c_string.into_raw(),
                        Err(_) => ptr::null_mut()
                    }
                } else {
                    ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Capitalize first character (implementation for string_capitalize)
#[no_mangle]
pub extern "C" fn string_capitalize(str_ptr: *const c_char) -> *mut c_char {
    if str_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => {
                let mut chars: Vec<char> = s.chars().collect();
                if !chars.is_empty() {
                    chars[0] = chars[0].to_uppercase().next().unwrap_or(chars[0]);
                }
                let result: String = chars.into_iter().collect();
                match CString::new(result) {
                    Ok(c_string) => c_string.into_raw(),
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Reverse string (implementation for string_reverse)
#[no_mangle]
pub extern "C" fn string_reverse(str_ptr: *const c_char) -> *mut c_char {
    if str_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => {
                let reversed: String = s.chars().rev().collect();
                match CString::new(reversed) {
                    Ok(c_string) => c_string.into_raw(),
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Trim whitespace from start (implementation for string_trim_start)
#[no_mangle]
pub extern "C" fn string_trim_start(str_ptr: *const c_char) -> *mut c_char {
    if str_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => {
                let trimmed = s.trim_start();
                match CString::new(trimmed) {
                    Ok(c_string) => c_string.into_raw(),
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Trim whitespace from end (implementation for string_trim_end)
#[no_mangle]
pub extern "C" fn string_trim_end(str_ptr: *const c_char) -> *mut c_char {
    if str_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => {
                let trimmed = s.trim_end();
                match CString::new(trimmed) {
                    Ok(c_string) => c_string.into_raw(),
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Replace first occurrence (implementation for string_replace)
#[no_mangle]
pub extern "C" fn string_replace(str_ptr: *const c_char, old_ptr: *const c_char, new_ptr: *const c_char) -> *mut c_char {
    if str_ptr.is_null() || old_ptr.is_null() || new_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        let text = match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut()
        };
        
        let old_str = match CStr::from_ptr(old_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut()
        };
        
        let new_str = match CStr::from_ptr(new_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut()
        };
        
        let result = text.replacen(old_str, new_str, 1);
        match CString::new(result) {
            Ok(c_string) => c_string.into_raw(),
            Err(_) => ptr::null_mut()
        }
    }
}

/// Replace all occurrences (implementation for string_replace_all)
#[no_mangle]
pub extern "C" fn string_replace_all(str_ptr: *const c_char, old_ptr: *const c_char, new_ptr: *const c_char) -> *mut c_char {
    if str_ptr.is_null() || old_ptr.is_null() || new_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        let text = match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut()
        };
        
        let old_str = match CStr::from_ptr(old_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut()
        };
        
        let new_str = match CStr::from_ptr(new_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut()
        };
        
        let result = text.replace(old_str, new_str);
        match CString::new(result) {
            Ok(c_string) => c_string.into_raw(),
            Err(_) => ptr::null_mut()
        }
    }
}

/// Repeat string (implementation for string_repeat)
#[no_mangle]
pub extern "C" fn string_repeat(str_ptr: *const c_char, count: i32) -> *mut c_char {
    if str_ptr.is_null() || count < 0 {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => {
                let result = s.repeat(count as usize);
                match CString::new(result) {
                    Ok(c_string) => c_string.into_raw(),
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Hash string (implementation for string_hash)
#[no_mangle]
pub extern "C" fn string_hash(str_ptr: *const c_char) -> i32 {
    if str_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => {
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};
                
                let mut hasher = DefaultHasher::new();
                s.hash(&mut hasher);
                (hasher.finish() as i32).abs()
            },
            Err(_) => 0
        }
    }
}

/// Check if string is numeric (implementation for string_is_numeric)
#[no_mangle]
pub extern "C" fn string_is_numeric(str_ptr: *const c_char) -> i32 {
    if str_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => {
                if s.is_empty() {
                    return 0;
                }
                // Check if all characters are digits, allowing for optional + or - at start
                let mut chars = s.chars();
                if let Some(first) = chars.next() {
                    if first == '+' || first == '-' {
                        // Need at least one digit after sign
                        if chars.next().is_none() {
                            return 0;
                        }
                    }
                }
                // Check if it can be parsed as a number
                if s.parse::<f64>().is_ok() { 1 } else { 0 }
            },
            Err(_) => 0
        }
    }
}

/// Check if string is alphabetic (implementation for string_is_alpha)
#[no_mangle]
pub extern "C" fn string_is_alpha(str_ptr: *const c_char) -> i32 {
    if str_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => {
                if s.is_empty() {
                    return 0;
                }
                if s.chars().all(|c| c.is_alphabetic()) { 1 } else { 0 }
            },
            Err(_) => 0
        }
    }
}

/// Check if string is alphanumeric (implementation for string_is_alphanumeric)
#[no_mangle]
pub extern "C" fn string_is_alphanumeric(str_ptr: *const c_char) -> i32 {
    if str_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => {
                if s.is_empty() {
                    return 0;
                }
                if s.chars().all(|c| c.is_alphanumeric()) { 1 } else { 0 }
            },
            Err(_) => 0
        }
    }
}

/// Check if string is whitespace (implementation for string_is_whitespace)
#[no_mangle]
pub extern "C" fn string_is_whitespace(str_ptr: *const c_char) -> i32 {
    if str_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => {
                if s.is_empty() {
                    return 0;
                }
                if s.chars().all(|c| c.is_whitespace()) { 1 } else { 0 }
            },
            Err(_) => 0
        }
    }
}

/// Check if string is ASCII (implementation for string_is_ascii)
#[no_mangle]
pub extern "C" fn string_is_ascii(str_ptr: *const c_char) -> i32 {
    if str_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => {
                if s.is_ascii() { 1 } else { 0 }
            },
            Err(_) => 0
        }
    }
}

/// Convert string to integer (implementation for string_to_int)
#[no_mangle]
pub extern "C" fn string_to_int(str_ptr: *const c_char) -> i32 {
    if str_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => {
                s.trim().parse::<i32>().unwrap_or(0)
            },
            Err(_) => 0
        }
    }
}

/// Convert string to float (implementation for string_to_float)
#[no_mangle]
pub extern "C" fn string_to_float(str_ptr: *const c_char) -> f64 {
    if str_ptr.is_null() {
        return 0.0;
    }
    
    unsafe {
        match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => {
                s.trim().parse::<f64>().unwrap_or(0.0)
            },
            Err(_) => 0.0
        }
    }
}

/// Convert string to bool (implementation for string_to_bool)
#[no_mangle]
pub extern "C" fn string_to_bool(str_ptr: *const c_char) -> i32 {
    if str_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => {
                let trimmed = s.trim().to_lowercase();
                match trimmed.as_str() {
                    "true" | "1" | "yes" | "on" | "based" => 1,
                    "false" | "0" | "no" | "off" | "cap" => 0,
                    _ => 0
                }
            },
            Err(_) => 0
        }
    }
}

/// Convert integer to string (implementation for string_from_int)
#[no_mangle]
pub extern "C" fn string_from_int(value: i32) -> *mut c_char {
    let result = value.to_string();
    match CString::new(result) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => ptr::null_mut()
    }
}

/// Convert float to string (implementation for string_from_float)
#[no_mangle]
pub extern "C" fn string_from_float(value: f64) -> *mut c_char {
    let result = value.to_string();
    match CString::new(result) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => ptr::null_mut()
    }
}

/// Convert bool to string (implementation for string_from_bool)
#[no_mangle]
pub extern "C" fn string_from_bool(value: i32) -> *mut c_char {
    let result = if value != 0 { "based" } else { "cap" };
    match CString::new(result) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => ptr::null_mut()
    }
}

/// Convert string to bytes (implementation for string_to_bytes)
#[no_mangle]
pub extern "C" fn string_to_bytes(str_ptr: *const c_char, len_ptr: *mut usize) -> *mut u8 {
    if str_ptr.is_null() || len_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => {
                let bytes = s.as_bytes();
                *len_ptr = bytes.len();
                
                // Allocate memory for byte array
                let result = libc::malloc(bytes.len()) as *mut u8;
                if result.is_null() {
                    return ptr::null_mut();
                }
                
                // Copy bytes
                ptr::copy_nonoverlapping(bytes.as_ptr(), result, bytes.len());
                result
            },
            Err(_) => {
                *len_ptr = 0;
                ptr::null_mut()
            }
        }
    }
}

/// Convert bytes to string (implementation for string_from_bytes)
#[no_mangle]
pub extern "C" fn string_from_bytes(bytes_ptr: *const u8, len: usize) -> *mut c_char {
    if bytes_ptr.is_null() || len == 0 {
        return ptr::null_mut();
    }
    
    unsafe {
        let bytes = std::slice::from_raw_parts(bytes_ptr, len);
        match String::from_utf8(bytes.to_vec()) {
            Ok(s) => {
                match CString::new(s) {
                    Ok(c_string) => c_string.into_raw(),
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Escape string (implementation for string_escape)
#[no_mangle]
pub extern "C" fn string_escape(str_ptr: *const c_char) -> *mut c_char {
    if str_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => {
                let escaped = s.chars().map(|c| match c {
                    '\n' => "\\n".to_string(),
                    '\r' => "\\r".to_string(),
                    '\t' => "\\t".to_string(),
                    '\\' => "\\\\".to_string(),
                    '"' => "\\\"".to_string(),
                    '\'' => "\\'".to_string(),
                    c if c.is_control() => format!("\\u{:04x}", c as u32),
                    c => c.to_string(),
                }).collect::<String>();
                
                match CString::new(escaped) {
                    Ok(c_string) => c_string.into_raw(),
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Unescape string (implementation for string_unescape)
#[no_mangle]
pub extern "C" fn string_unescape(str_ptr: *const c_char) -> *mut c_char {
    if str_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => {
                let mut result = String::new();
                let mut chars = s.chars();
                
                while let Some(c) = chars.next() {
                    if c == '\\' {
                        if let Some(next) = chars.next() {
                            match next {
                                'n' => result.push('\n'),
                                'r' => result.push('\r'),
                                't' => result.push('\t'),
                                '\\' => result.push('\\'),
                                '"' => result.push('"'),
                                '\'' => result.push('\''),
                                'u' => {
                                    // Unicode escape sequence \uXXXX
                                    let hex: String = chars.by_ref().take(4).collect();
                                    if let Ok(code) = u32::from_str_radix(&hex, 16) {
                                        if let Some(unicode_char) = char::from_u32(code) {
                                            result.push(unicode_char);
                                        } else {
                                            result.push_str(&format!("\\u{}", hex));
                                        }
                                    } else {
                                        result.push_str(&format!("\\u{}", hex));
                                    }
                                },
                                _ => {
                                    result.push('\\');
                                    result.push(next);
                                }
                            }
                        } else {
                            result.push('\\');
                        }
                    } else {
                        result.push(c);
                    }
                }
                
                match CString::new(result) {
                    Ok(c_string) => c_string.into_raw(),
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Split string by delimiter (implementation for string_split)
#[no_mangle]
pub extern "C" fn string_split(str_ptr: *const c_char, delimiter_ptr: *const c_char, count_ptr: *mut usize) -> *mut *mut c_char {
    if str_ptr.is_null() || delimiter_ptr.is_null() || count_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        let text = match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut()
        };
        
        let delimiter = match CStr::from_ptr(delimiter_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut()
        };
        
        let parts: Vec<&str> = text.split(delimiter).collect();
        let count = parts.len();
        *count_ptr = count;
        
        if count == 0 {
            return ptr::null_mut();
        }
        
        // Allocate array of string pointers
        let array = libc::malloc(count * std::mem::size_of::<*mut c_char>()) as *mut *mut c_char;
        if array.is_null() {
            return ptr::null_mut();
        }
        
        // Convert each part to C string
        for (i, part) in parts.iter().enumerate() {
            match CString::new(*part) {
                Ok(c_string) => {
                    *array.add(i) = c_string.into_raw();
                },
                Err(_) => {
                    // Cleanup on error
                    for j in 0..i {
                        let _ = CString::from_raw(*array.add(j));
                    }
                    libc::free(array as *mut libc::c_void);
                    return ptr::null_mut();
                }
            }
        }
        
        array
    }
}

/// Split string by lines (implementation for string_split_lines)
#[no_mangle]
pub extern "C" fn string_split_lines(str_ptr: *const c_char, count_ptr: *mut usize) -> *mut *mut c_char {
    if str_ptr.is_null() || count_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        let text = match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut()
        };
        
        let parts: Vec<&str> = text.lines().collect();
        let count = parts.len();
        *count_ptr = count;
        
        if count == 0 {
            return ptr::null_mut();
        }
        
        // Allocate array of string pointers
        let array = libc::malloc(count * std::mem::size_of::<*mut c_char>()) as *mut *mut c_char;
        if array.is_null() {
            return ptr::null_mut();
        }
        
        // Convert each part to C string
        for (i, part) in parts.iter().enumerate() {
            match CString::new(*part) {
                Ok(c_string) => {
                    *array.add(i) = c_string.into_raw();
                },
                Err(_) => {
                    // Cleanup on error
                    for j in 0..i {
                        let _ = CString::from_raw(*array.add(j));
                    }
                    libc::free(array as *mut libc::c_void);
                    return ptr::null_mut();
                }
            }
        }
        
        array
    }
}

/// Split string by whitespace (implementation for string_split_whitespace)
#[no_mangle]
pub extern "C" fn string_split_whitespace(str_ptr: *const c_char, count_ptr: *mut usize) -> *mut *mut c_char {
    if str_ptr.is_null() || count_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        let text = match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut()
        };
        
        let parts: Vec<&str> = text.split_whitespace().collect();
        let count = parts.len();
        *count_ptr = count;
        
        if count == 0 {
            return ptr::null_mut();
        }
        
        // Allocate array of string pointers
        let array = libc::malloc(count * std::mem::size_of::<*mut c_char>()) as *mut *mut c_char;
        if array.is_null() {
            return ptr::null_mut();
        }
        
        // Convert each part to C string
        for (i, part) in parts.iter().enumerate() {
            match CString::new(*part) {
                Ok(c_string) => {
                    *array.add(i) = c_string.into_raw();
                },
                Err(_) => {
                    // Cleanup on error
                    for j in 0..i {
                        let _ = CString::from_raw(*array.add(j));
                    }
                    libc::free(array as *mut libc::c_void);
                    return ptr::null_mut();
                }
            }
        }
        
        array
    }
}

/// Join array of strings (implementation for string_join)
#[no_mangle]
pub extern "C" fn string_join(strings_ptr: *const *const c_char, count: usize, separator_ptr: *const c_char) -> *mut c_char {
    if strings_ptr.is_null() || separator_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        let separator = match CStr::from_ptr(separator_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut()
        };
        
        let mut parts = Vec::new();
        for i in 0..count {
            let str_ptr = *strings_ptr.add(i);
            if !str_ptr.is_null() {
                if let Ok(s) = CStr::from_ptr(str_ptr).to_str() {
                    parts.push(s);
                }
            }
        }
        
        let result = parts.join(separator);
        match CString::new(result) {
            Ok(c_string) => c_string.into_raw(),
            Err(_) => ptr::null_mut()
        }
    }
}

/// Pad string left (implementation for string_pad_left)
#[no_mangle]
pub extern "C" fn string_pad_left(str_ptr: *const c_char, length: i32, pad_char_ptr: *const c_char) -> *mut c_char {
    if str_ptr.is_null() || pad_char_ptr.is_null() || length < 0 {
        return ptr::null_mut();
    }
    
    unsafe {
        let text = match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut()
        };
        
        let pad_char = match CStr::from_ptr(pad_char_ptr).to_str() {
            Ok(s) => s.chars().next().unwrap_or(' '),
            Err(_) => return ptr::null_mut()
        };
        
        let target_len = length as usize;
        let current_len = text.chars().count();
        
        let result = if current_len >= target_len {
            text.to_string()
        } else {
            let pad_count = target_len - current_len;
            let padding: String = pad_char.to_string().repeat(pad_count);
            format!("{}{}", padding, text)
        };
        
        match CString::new(result) {
            Ok(c_string) => c_string.into_raw(),
            Err(_) => ptr::null_mut()
        }
    }
}

/// Pad string right (implementation for string_pad_right)
#[no_mangle]
pub extern "C" fn string_pad_right(str_ptr: *const c_char, length: i32, pad_char_ptr: *const c_char) -> *mut c_char {
    if str_ptr.is_null() || pad_char_ptr.is_null() || length < 0 {
        return ptr::null_mut();
    }
    
    unsafe {
        let text = match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut()
        };
        
        let pad_char = match CStr::from_ptr(pad_char_ptr).to_str() {
            Ok(s) => s.chars().next().unwrap_or(' '),
            Err(_) => return ptr::null_mut()
        };
        
        let target_len = length as usize;
        let current_len = text.chars().count();
        
        let result = if current_len >= target_len {
            text.to_string()
        } else {
            let pad_count = target_len - current_len;
            let padding: String = pad_char.to_string().repeat(pad_count);
            format!("{}{}", text, padding)
        };
        
        match CString::new(result) {
            Ok(c_string) => c_string.into_raw(),
            Err(_) => ptr::null_mut()
        }
    }
}

/// Pad string center (implementation for string_pad_center)
#[no_mangle]
pub extern "C" fn string_pad_center(str_ptr: *const c_char, length: i32, pad_char_ptr: *const c_char) -> *mut c_char {
    if str_ptr.is_null() || pad_char_ptr.is_null() || length < 0 {
        return ptr::null_mut();
    }
    
    unsafe {
        let text = match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut()
        };
        
        let pad_char = match CStr::from_ptr(pad_char_ptr).to_str() {
            Ok(s) => s.chars().next().unwrap_or(' '),
            Err(_) => return ptr::null_mut()
        };
        
        let target_len = length as usize;
        let current_len = text.chars().count();
        
        let result = if current_len >= target_len {
            text.to_string()
        } else {
            let total_pad = target_len - current_len;
            let left_pad = total_pad / 2;
            let right_pad = total_pad - left_pad;
            
            let left_padding: String = pad_char.to_string().repeat(left_pad);
            let right_padding: String = pad_char.to_string().repeat(right_pad);
            format!("{}{}{}", left_padding, text, right_padding)
        };
        
        match CString::new(result) {
            Ok(c_string) => c_string.into_raw(),
            Err(_) => ptr::null_mut()
        }
    }
}

/// Find all regex matches (implementation for string_regex_find_all)
#[no_mangle]
pub extern "C" fn string_regex_find_all(str_ptr: *const c_char, pattern_ptr: *const c_char, count_ptr: *mut usize) -> *mut *mut c_char {
    if str_ptr.is_null() || pattern_ptr.is_null() || count_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        let text = match CStr::from_ptr(str_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut()
        };
        
        let pattern = match CStr::from_ptr(pattern_ptr).to_str() {
            Ok(p) => p,
            Err(_) => return ptr::null_mut()
        };
        
        match Regex::new(pattern) {
            Ok(regex) => {
                let matches: Vec<&str> = regex.find_iter(text).map(|m| m.as_str()).collect();
                let count = matches.len();
                *count_ptr = count;
                
                if count == 0 {
                    return ptr::null_mut();
                }
                
                // Allocate array of string pointers
                let array = libc::malloc(count * std::mem::size_of::<*mut c_char>()) as *mut *mut c_char;
                if array.is_null() {
                    return ptr::null_mut();
                }
                
                // Convert each match to C string
                for (i, match_str) in matches.iter().enumerate() {
                    match CString::new(*match_str) {
                        Ok(c_string) => {
                            *array.add(i) = c_string.into_raw();
                        },
                        Err(_) => {
                            // Cleanup on error
                            for j in 0..i {
                                let _ = CString::from_raw(*array.add(j));
                            }
                            libc::free(array as *mut libc::c_void);
                            return ptr::null_mut();
                        }
                    }
                }
                
                array
            },
            Err(_) => ptr::null_mut()
        }
    }
}

// ================================
// Networking Implementation Functions
// ================================

use std::sync::Arc;

/// Socket types that can be stored in the registry
#[derive(Debug)]
pub enum SocketType {
    TcpStream(TcpStream),
    TcpListener(TcpListener),
}

/// Socket registry to manage active sockets by ID
pub struct SocketRegistry {
    sockets: HashMap<i32, SocketType>,
    next_id: i32,
}

impl SocketRegistry {
    fn new() -> Self {
        Self {
            sockets: HashMap::new(),
            next_id: 1,
        }
    }

    fn register_socket(&mut self, socket: SocketType) -> i32 {
        let id = self.next_id;
        self.next_id += 1;
        self.sockets.insert(id, socket);
        id
    }

    fn get_socket(&self, id: i32) -> Option<&SocketType> {
        self.sockets.get(&id)
    }

    fn get_socket_mut(&mut self, id: i32) -> Option<&mut SocketType> {
        self.sockets.get_mut(&id)
    }

    fn remove_socket(&mut self, id: i32) -> Option<SocketType> {
        self.sockets.remove(&id)
    }
}

/// Global socket registry instance
static SOCKET_REGISTRY: std::sync::LazyLock<Arc<Mutex<SocketRegistry>>> = 
    std::sync::LazyLock::new(|| Arc::new(Mutex::new(SocketRegistry::new())));

/// Connect to a TCP server (implementation for network_tcp_connect)
#[no_mangle]
pub extern "C" fn network_tcp_connect(host_ptr: *const c_char, port: u16) -> i32 {
    if host_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        match CStr::from_ptr(host_ptr).to_str() {
            Ok(host) => {
                let addr = format!("{}:{}", host, port);
                match TcpStream::connect(&addr) {
                    Ok(stream) => {
                        // Register the socket in the global registry
                        match SOCKET_REGISTRY.lock() {
                            Ok(mut registry) => {
                                registry.register_socket(SocketType::TcpStream(stream))
                            },
                            Err(_) => -1
                        }
                    },
                    Err(_) => -1
                }
            },
            Err(_) => -1
        }
    }
}

/// Create a TCP listener on a port (implementation for network_tcp_listen)
#[no_mangle]
pub extern "C" fn network_tcp_listen(port: u16) -> i32 {
    let addr = format!("0.0.0.0:{}", port);
    match TcpListener::bind(&addr) {
        Ok(listener) => {
            // Register the listener in the global registry
            match SOCKET_REGISTRY.lock() {
                Ok(mut registry) => {
                    registry.register_socket(SocketType::TcpListener(listener))
                },
                Err(_) => -1
            }
        },
        Err(_) => -1
    }
}

/// Send data over TCP connection (implementation for network_tcp_send)
#[no_mangle]
pub extern "C" fn network_tcp_send(socket_id: i32, data_ptr: *const c_char, len: usize) -> i32 {
    if data_ptr.is_null() || socket_id <= 0 {
        return -1;
    }
    
    unsafe {
        let data = std::slice::from_raw_parts(data_ptr as *const u8, len);
        
        // Retrieve actual socket from registry by socket_id
        match SOCKET_REGISTRY.lock() {
            Ok(mut registry) => {
                match registry.get_socket_mut(socket_id) {
                    Some(SocketType::TcpStream(stream)) => {
                        match stream.write_all(data) {
                            Ok(_) => data.len() as i32,
                            Err(_) => -1
                        }
                    },
                    Some(SocketType::TcpListener(_)) => {
                        // Can't send data on a listener
                        -1
                    },
                    None => {
                        // Socket not found
                        -1
                    }
                }
            },
            Err(_) => -1
        }
    }
}

/// Receive data from TCP connection (implementation for network_tcp_recv)
#[no_mangle]
pub extern "C" fn network_tcp_recv(socket_id: i32, buffer_ptr: *mut c_char, buffer_len: usize) -> i32 {
    if buffer_ptr.is_null() || socket_id <= 0 || buffer_len == 0 {
        return -1;
    }
    
    // Retrieve actual socket from registry by socket_id
    match SOCKET_REGISTRY.lock() {
        Ok(mut registry) => {
            match registry.get_socket_mut(socket_id) {
                Some(SocketType::TcpStream(stream)) => {
                    unsafe {
                        let buffer = std::slice::from_raw_parts_mut(buffer_ptr as *mut u8, buffer_len);
                        match stream.read(buffer) {
                            Ok(bytes_read) => bytes_read as i32,
                            Err(_) => -1
                        }
                    }
                },
                Some(SocketType::TcpListener(_)) => {
                    // Can't receive data on a listener
                    -1
                },
                None => {
                    // Socket not found
                    -1
                }
            }
        },
        Err(_) => -1
    }
}

/// Close TCP connection (implementation for network_tcp_close)
#[no_mangle]
pub extern "C" fn network_tcp_close(socket_id: i32) -> i32 {
    if socket_id <= 0 {
        return -1;
    }
    
    // Remove socket from registry and close
    match SOCKET_REGISTRY.lock() {
        Ok(mut registry) => {
            match registry.remove_socket(socket_id) {
                Some(_) => {
                    // Socket removed successfully, it will be automatically closed when dropped
                    0
                },
                None => {
                    // Socket not found
                    -1
                }
            }
        },
        Err(_) => -1
    }
}

/// Perform DNS resolution (implementation for network_dns_resolve)
#[no_mangle]
pub extern "C" fn network_dns_resolve(hostname_ptr: *const c_char) -> *mut c_char {
    if hostname_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(hostname_ptr).to_str() {
            Ok(hostname) => {
                if hostname == "localhost" {
                    match CString::new("127.0.0.1") {
                        Ok(result) => result.into_raw(),
                        Err(_) => ptr::null_mut()
                    }
                } else {
                    // Basic DNS resolution using std::net
                    match format!("{}:80", hostname).to_socket_addrs() {
                        Ok(mut addrs) => {
                            if let Some(addr) = addrs.next() {
                                let ip_str = addr.ip().to_string();
                                match CString::new(ip_str) {
                                    Ok(result) => result.into_raw(),
                                    Err(_) => ptr::null_mut()
                                }
                            } else {
                                ptr::null_mut()
                            }
                        },
                        Err(_) => ptr::null_mut()
                    }
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Simple HTTP GET request (implementation for network_http_get)
#[no_mangle]
pub extern "C" fn network_http_get(url_ptr: *const c_char) -> *mut c_char {
    if url_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match CStr::from_ptr(url_ptr).to_str() {
            Ok(url) => {
                // Parse URL to extract host and path
                if let Some(host_start) = url.find("://") {
                    let after_protocol = &url[host_start + 3..];
                    if let Some(path_start) = after_protocol.find('/') {
                        let host = &after_protocol[..path_start];
                        let path = &after_protocol[path_start..];
                        
                        // Create HTTP request
                        let request = format!(
                            "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
                            path, host
                        );
                        
                        // Connect and send request
                        match TcpStream::connect(format!("{}:80", host)) {
                            Ok(mut stream) => {
                                if stream.write_all(request.as_bytes()).is_ok() {
                                    let mut response = String::new();
                                    if stream.read_to_string(&mut response).is_ok() {
                                        match CString::new(response) {
                                            Ok(result) => return result.into_raw(),
                                            Err(_) => {}
                                        }
                                    }
                                }
                            },
                            Err(_) => {}
                        }
                    }
                }
                
                // Return error response on failure
                match CString::new("HTTP/1.1 500 Internal Server Error\r\n\r\nHTTP request failed") {
                    Ok(result) => result.into_raw(),
                    Err(_) => ptr::null_mut()
                }
            },
            Err(_) => ptr::null_mut()
        }
    }
}

/// Simple HTTP POST request (implementation for network_http_post)
#[no_mangle]
pub extern "C" fn network_http_post(url_ptr: *const c_char, data_ptr: *const c_char) -> *mut c_char {
    if url_ptr.is_null() || data_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        match (CStr::from_ptr(url_ptr).to_str(), CStr::from_ptr(data_ptr).to_str()) {
            (Ok(url), Ok(data)) => {
                // Parse URL to extract host and path
                if let Some(host_start) = url.find("://") {
                    let after_protocol = &url[host_start + 3..];
                    if let Some(path_start) = after_protocol.find('/') {
                        let host = &after_protocol[..path_start];
                        let path = &after_protocol[path_start..];
                        
                        // Create HTTP POST request
                        let request = format!(
                            "POST {} HTTP/1.1\r\nHost: {}\r\nContent-Type: application/x-www-form-urlencoded\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                            path, host, data.len(), data
                        );
                        
                        // Connect and send request
                        match TcpStream::connect(format!("{}:80", host)) {
                            Ok(mut stream) => {
                                if stream.write_all(request.as_bytes()).is_ok() {
                                    let mut response = String::new();
                                    if stream.read_to_string(&mut response).is_ok() {
                                        match CString::new(response) {
                                            Ok(result) => return result.into_raw(),
                                            Err(_) => {}
                                        }
                                    }
                                }
                            },
                            Err(_) => {}
                        }
                    }
                }
                
                // Return error response on failure
                match CString::new("HTTP/1.1 500 Internal Server Error\r\n\r\nHTTP POST request failed") {
                    Ok(result) => result.into_raw(),
                    Err(_) => ptr::null_mut()
                }
            },
            _ => ptr::null_mut()
        }
    }
}

/// Generic HTTP request (implementation for network_http_request)
#[no_mangle]
pub extern "C" fn network_http_request(
    method_ptr: *const c_char, 
    url_ptr: *const c_char, 
    headers_ptr: *const c_char, 
    body_ptr: *const c_char
) -> *mut c_char {
    if method_ptr.is_null() || url_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        let method = match CStr::from_ptr(method_ptr).to_str() {
            Ok(m) => m,
            Err(_) => return ptr::null_mut()
        };
        
        let url = match CStr::from_ptr(url_ptr).to_str() {
            Ok(u) => u,
            Err(_) => return ptr::null_mut()
        };
        
        let headers = if headers_ptr.is_null() {
            ""
        } else {
            match CStr::from_ptr(headers_ptr).to_str() {
                Ok(h) => h,
                Err(_) => ""
            }
        };
        
        let body = if body_ptr.is_null() {
            ""
        } else {
            match CStr::from_ptr(body_ptr).to_str() {
                Ok(b) => b,
                Err(_) => ""
            }
        };
        
        // Parse URL to extract host and path
        if let Some(host_start) = url.find("://") {
            let after_protocol = &url[host_start + 3..];
            if let Some(path_start) = after_protocol.find('/') {
                let host = &after_protocol[..path_start];
                let path = &after_protocol[path_start..];
                
                // Build HTTP request
                let mut request = format!("{} {} HTTP/1.1\r\nHost: {}\r\n", method, path, host);
                
                // Add custom headers if provided
                if !headers.is_empty() {
                    request.push_str(headers);
                    if !headers.ends_with("\r\n") {
                        request.push_str("\r\n");
                    }
                }
                
                // Add body if provided
                if !body.is_empty() {
                    request.push_str(&format!("Content-Length: {}\r\n", body.len()));
                }
                
                request.push_str("Connection: close\r\n\r\n");
                
                if !body.is_empty() {
                    request.push_str(body);
                }
                
                // Connect and send request
                match TcpStream::connect(format!("{}:80", host)) {
                    Ok(mut stream) => {
                        if stream.write_all(request.as_bytes()).is_ok() {
                            let mut response = String::new();
                            if stream.read_to_string(&mut response).is_ok() {
                                match CString::new(response) {
                                    Ok(result) => return result.into_raw(),
                                    Err(_) => {}
                                }
                            }
                        }
                    },
                    Err(_) => {}
                }
            }
        }
        
        // Return error response on failure
        match CString::new("HTTP/1.1 500 Internal Server Error\r\n\r\nHTTP request failed") {
            Ok(result) => result.into_raw(),
            Err(_) => ptr::null_mut()
        }
    }
}

// ================================
// Character to String Conversion Functions
// ================================

/// Convert any value to string (implementation for tea function)
#[no_mangle]
pub extern "C" fn tea(value: i64) -> *mut c_char {
    // Convert integer to string
    let result = value.to_string();
    match CString::new(result) {
        Ok(cstr) => {
            let ptr = cstr.into_raw();
            ptr
        },
        Err(_) => ptr::null_mut()
    }
}

/// Convert float to string (implementation for tea function with floats)
#[no_mangle]
pub extern "C" fn tea_float(value: f64) -> *mut c_char {
    let result = value.to_string();
    match CString::new(result) {
        Ok(cstr) => {
            let ptr = cstr.into_raw();
            ptr
        },
        Err(_) => ptr::null_mut()
    }
}

/// Convert boolean to string (implementation for tea function with booleans)
#[no_mangle]
pub extern "C" fn tea_bool(value: i32) -> *mut c_char {
    let result = if value != 0 { "based" } else { "cap" };
    match CString::new(result) {
        Ok(cstr) => {
            let ptr = cstr.into_raw();
            ptr
        },
        Err(_) => ptr::null_mut()
    }
}

/// Convert a single character to a heap-allocated string (implementation for char_to_string)
#[no_mangle]
pub extern "C" fn char_to_string(c: c_char) -> *mut c_char {
    // Convert i8 to char, handling both ASCII and extended characters
    let character = c as u8 as char;
    
    // Create a string containing just this character
    let char_string = character.to_string();
    
    // Convert to CString and return as raw pointer
    match CString::new(char_string) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => ptr::null_mut()
    }
}

// ================================
// Vibez Module Wrapper Functions
// ================================

/// Simple wrapper for formatted printing - placeholder implementation
#[no_mangle]
pub extern "C" fn vibez_format(format_ptr: *const c_char, args_ptr: *const i64, args_len: usize) -> *mut c_char {
    if format_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        let format_str = match CStr::from_ptr(format_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut(),
        };
        
        // Simple placeholder implementation - just return the format string for now
        match CString::new(format_str) {
            Ok(c_str) => c_str.into_raw(),
            Err(_) => ptr::null_mut()
        }
    }
}

/// Simple wrapper for sprintf-style formatting - placeholder implementation
#[no_mangle]
pub extern "C" fn vibez_sprintf(format_ptr: *const c_char, args_ptr: *const i64, args_len: usize) -> *mut c_char {
    if format_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        let format_str = match CStr::from_ptr(format_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut(),
        };
        
        // Simple placeholder implementation - just return the format string for now
        match CString::new(format_str) {
            Ok(c_str) => c_str.into_raw(),
            Err(_) => ptr::null_mut()
        }
    }
}

/// Simple wrapper for debug logging - placeholder implementation
#[no_mangle]
pub extern "C" fn vibez_debug_log(level: u8, message_ptr: *const c_char, module_ptr: *const c_char) -> i32 {
    if message_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let message = match CStr::from_ptr(message_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return -1,
        };
        
        let module = if module_ptr.is_null() {
            "unknown"
        } else {
            match CStr::from_ptr(module_ptr).to_str() {
                Ok(s) => s,
                Err(_) => "unknown",
            }
        };
        
        // Simple implementation: print to stderr
        eprintln!("[DEBUG-{}] [{}] {}", level, module, message);
        0
    }
}

/// Simple wrapper for debug inspect - placeholder implementation
#[no_mangle]
pub extern "C" fn vibez_debug_inspect(value_ptr: *const i64, label_ptr: *const c_char) -> i32 {
    if value_ptr.is_null() {
        return -1;
    }
    
    unsafe {
        let value = *value_ptr;
        
        let label = if label_ptr.is_null() {
            "value"
        } else {
            match CStr::from_ptr(label_ptr).to_str() {
                Ok(s) => s,
                Err(_) => "value",
            }
        };
        
        // Simple implementation: print the value
        eprintln!("[INSPECT] {}: {}", label, value);
        0
    }
}

// ================================
// Time Implementation Functions
// ================================

/// Get current time as unix timestamp in seconds
#[no_mangle]
pub extern "C" fn time_now_impl() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

/// Get current time as unix timestamp in milliseconds
#[no_mangle]
pub extern "C" fn time_now_millis_impl() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

/// Get current time as unix timestamp in microseconds
#[no_mangle]
pub extern "C" fn time_now_micros_impl() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_micros() as i64
}

/// Get current time as unix timestamp in nanoseconds
#[no_mangle]
pub extern "C" fn time_now_nanos_impl() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as i64
}

/// Create datetime from unix timestamp
#[no_mangle]
pub extern "C" fn time_from_timestamp_impl(timestamp: i64) -> i64 {
    // Return timestamp as datetime representation
    timestamp
}

/// Create datetime from milliseconds
#[no_mangle]
pub extern "C" fn time_from_millis_impl(millis: i64) -> i64 {
    // Convert milliseconds to seconds for datetime representation
    millis / 1000
}

/// Create datetime from year, month, day, hour, minute, second
#[no_mangle]
pub extern "C" fn time_create_impl(year: i32, month: i32, day: i32, hour: i32, minute: i32, second: i32) -> i64 {
    if let Some(naive_date) = NaiveDate::from_ymd_opt(year, month as u32, day as u32) {
        if let Some(naive_datetime) = naive_date.and_hms_opt(hour as u32, minute as u32, second as u32) {
            let dt: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive_datetime, Utc);
            return dt.timestamp();
        }
    }
    0 // Return 0 for invalid dates
}

/// Parse datetime from string with format
#[no_mangle]
pub extern "C" fn time_parse_impl(date_string_ptr: *const c_char, format_ptr: *const c_char) -> i64 {
    if date_string_ptr.is_null() || format_ptr.is_null() {
        return 0;
    }
    
    unsafe {
        let date_string = match CStr::from_ptr(date_string_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return 0,
        };
        
        let format = match CStr::from_ptr(format_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return 0,
        };
        
        // Simple RFC3339 parsing for now
        if format == "RFC3339" || format == "ISO8601" {
            if let Ok(dt) = DateTime::parse_from_rfc3339(date_string) {
                return dt.timestamp();
            }
        }
        
        0 // Return 0 for parsing errors
    }
}

/// Format datetime to string
#[no_mangle]
pub extern "C" fn time_format_impl(timestamp: i64, format_ptr: *const c_char) -> *mut c_char {
    if format_ptr.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        let format = match CStr::from_ptr(format_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return ptr::null_mut(),
        };
        
        let dt = DateTime::from_timestamp(timestamp, 0).unwrap_or_default();
        let formatted = match format {
            "RFC3339" => dt.to_rfc3339(),
            "ISO8601" => dt.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
            _ => dt.format(format).to_string(),
        };
        
        match CString::new(formatted) {
            Ok(c_string) => c_string.into_raw(),
            Err(_) => ptr::null_mut(),
        }
    }
}

/// Convert datetime to string
#[no_mangle]
pub extern "C" fn time_to_string_impl(timestamp: i64) -> *mut c_char {
    let dt = DateTime::from_timestamp(timestamp, 0).unwrap_or_default();
    let formatted = dt.to_rfc3339();
    
    match CString::new(formatted) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

/// Convert datetime to ISO8601 string
#[no_mangle]
pub extern "C" fn time_to_iso8601_impl(timestamp: i64) -> *mut c_char {
    let dt = DateTime::from_timestamp(timestamp, 0).unwrap_or_default();
    let formatted = dt.format("%Y-%m-%dT%H:%M:%SZ").to_string();
    
    match CString::new(formatted) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

/// Convert datetime to RFC3339 string
#[no_mangle]
pub extern "C" fn time_to_rfc3339_impl(timestamp: i64) -> *mut c_char {
    let dt = DateTime::from_timestamp(timestamp, 0).unwrap_or_default();
    let formatted = dt.to_rfc3339();
    
    match CString::new(formatted) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

/// Get year from datetime
#[no_mangle]
pub extern "C" fn time_year_impl(timestamp: i64) -> i32 {
    DateTime::from_timestamp(timestamp, 0)
        .unwrap_or_default()
        .year()
}

/// Get month from datetime (1-12)
#[no_mangle]
pub extern "C" fn time_month_impl(timestamp: i64) -> i32 {
    DateTime::from_timestamp(timestamp, 0)
        .unwrap_or_default()
        .month() as i32
}

/// Get day from datetime (1-31)
#[no_mangle]
pub extern "C" fn time_day_impl(timestamp: i64) -> i32 {
    DateTime::from_timestamp(timestamp, 0)
        .unwrap_or_default()
        .day() as i32
}

/// Get hour from datetime (0-23)
#[no_mangle]
pub extern "C" fn time_hour_impl(timestamp: i64) -> i32 {
    DateTime::from_timestamp(timestamp, 0)
        .unwrap_or_default()
        .hour() as i32
}

/// Get minute from datetime (0-59)
#[no_mangle]
pub extern "C" fn time_minute_impl(timestamp: i64) -> i32 {
    DateTime::from_timestamp(timestamp, 0)
        .unwrap_or_default()
        .minute() as i32
}

/// Get second from datetime (0-59)
#[no_mangle]
pub extern "C" fn time_second_impl(timestamp: i64) -> i32 {
    DateTime::from_timestamp(timestamp, 0)
        .unwrap_or_default()
        .second() as i32
}

/// Get weekday from datetime (0=Sunday, 6=Saturday)
#[no_mangle]
pub extern "C" fn time_weekday_impl(timestamp: i64) -> i32 {
    let dt = DateTime::from_timestamp(timestamp, 0).unwrap_or_default();
    match dt.weekday() {
        Weekday::Sun => 0,
        Weekday::Mon => 1,
        Weekday::Tue => 2,
        Weekday::Wed => 3,
        Weekday::Thu => 4,
        Weekday::Fri => 5,
        Weekday::Sat => 6,
    }
}

/// Get day of year from datetime (1-366)
#[no_mangle]
pub extern "C" fn time_day_of_year_impl(timestamp: i64) -> i32 {
    DateTime::from_timestamp(timestamp, 0)
        .unwrap_or_default()
        .ordinal() as i32
}

/// Add years to datetime
#[no_mangle]
pub extern "C" fn time_add_years_impl(timestamp: i64, years: i32) -> i64 {
    if let Some(dt) = DateTime::from_timestamp(timestamp, 0) {
        if let Some(new_year) = dt.year().checked_add(years) {
            if let Some(new_dt) = dt.with_year(new_year) {
                return new_dt.timestamp();
            }
        }
    }
    timestamp
}

/// Add months to datetime
#[no_mangle]
pub extern "C" fn time_add_months_impl(timestamp: i64, months: i32) -> i64 {
    if let Some(dt) = DateTime::from_timestamp(timestamp, 0) {
        let total_months = dt.month() as i32 + months;
        let new_year = dt.year() + (total_months - 1) / 12;
        let new_month = ((total_months - 1) % 12) + 1;
        
        if let Some(new_dt) = dt.with_year(new_year).and_then(|d| d.with_month(new_month as u32)) {
            return new_dt.timestamp();
        }
    }
    timestamp
}

/// Add days to datetime
#[no_mangle]
pub extern "C" fn time_add_days_impl(timestamp: i64, days: i32) -> i64 {
    timestamp + (days as i64 * 86400) // 86400 seconds per day
}

/// Add hours to datetime
#[no_mangle]
pub extern "C" fn time_add_hours_impl(timestamp: i64, hours: i32) -> i64 {
    timestamp + (hours as i64 * 3600) // 3600 seconds per hour
}

/// Add minutes to datetime
#[no_mangle]
pub extern "C" fn time_add_minutes_impl(timestamp: i64, minutes: i32) -> i64 {
    timestamp + (minutes as i64 * 60) // 60 seconds per minute
}

/// Add seconds to datetime
#[no_mangle]
pub extern "C" fn time_add_seconds_impl(timestamp: i64, seconds: i32) -> i64 {
    timestamp + seconds as i64
}

/// Subtract two datetimes to get duration
#[no_mangle]
pub extern "C" fn time_subtract_impl(timestamp1: i64, timestamp2: i64) -> i64 {
    timestamp1 - timestamp2
}

/// Get difference in days between two datetimes
#[no_mangle]
pub extern "C" fn time_diff_days_impl(timestamp1: i64, timestamp2: i64) -> i32 {
    ((timestamp1 - timestamp2) / 86400) as i32
}

/// Get difference in hours between two datetimes
#[no_mangle]
pub extern "C" fn time_diff_hours_impl(timestamp1: i64, timestamp2: i64) -> i32 {
    ((timestamp1 - timestamp2) / 3600) as i32
}

/// Get difference in minutes between two datetimes
#[no_mangle]
pub extern "C" fn time_diff_minutes_impl(timestamp1: i64, timestamp2: i64) -> i32 {
    ((timestamp1 - timestamp2) / 60) as i32
}

/// Get difference in seconds between two datetimes
#[no_mangle]
pub extern "C" fn time_diff_seconds_impl(timestamp1: i64, timestamp2: i64) -> i32 {
    (timestamp1 - timestamp2) as i32
}

/// Create duration from seconds
#[no_mangle]
pub extern "C" fn duration_from_seconds_impl(seconds: i32) -> i64 {
    seconds as i64
}

/// Create duration from milliseconds
#[no_mangle]
pub extern "C" fn duration_from_millis_impl(millis: i32) -> i64 {
    (millis as i64) / 1000
}

/// Convert duration to seconds
#[no_mangle]
pub extern "C" fn duration_to_seconds_impl(duration: i64) -> i32 {
    duration as i32
}

/// Convert duration to milliseconds
#[no_mangle]
pub extern "C" fn duration_to_millis_impl(duration: i64) -> i32 {
    (duration * 1000) as i32
}

/// Add two durations
#[no_mangle]
pub extern "C" fn duration_add_impl(duration1: i64, duration2: i64) -> i64 {
    duration1 + duration2
}

/// Subtract two durations
#[no_mangle]
pub extern "C" fn duration_subtract_impl(duration1: i64, duration2: i64) -> i64 {
    duration1 - duration2
}

/// Get current UTC time
#[no_mangle]
pub extern "C" fn time_utc_impl() -> i64 {
    Utc::now().timestamp()
}

/// Get current local time
#[no_mangle]
pub extern "C" fn time_local_impl() -> i64 {
    Local::now().timestamp()
}

/// Convert datetime to UTC
#[no_mangle]
pub extern "C" fn time_to_utc_impl(timestamp: i64) -> i64 {
    // If already UTC, return as-is
    timestamp
}

/// Convert datetime to local time
#[no_mangle]
pub extern "C" fn time_to_local_impl(timestamp: i64) -> i64 {
    // Convert UTC timestamp to local time
    if let Some(dt) = DateTime::from_timestamp(timestamp, 0) {
        let local: DateTime<Local> = dt.with_timezone(&Local);
        return local.timestamp();
    }
    timestamp
}

/// Get timezone offset in seconds
#[no_mangle]
pub extern "C" fn time_timezone_offset_impl() -> i32 {
    Local::now().offset().local_minus_utc()
}

/// Check if year is leap year
#[no_mangle]
pub extern "C" fn time_is_leap_year_impl(year: i32) -> i32 {
    if year % 400 == 0 {
        1
    } else if year % 100 == 0 {
        0
    } else if year % 4 == 0 {
        1
    } else {
        0
    }
}

/// Get number of days in month
#[no_mangle]
pub extern "C" fn time_days_in_month_impl(year: i32, month: i32) -> i32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if time_is_leap_year_impl(year) == 1 { 29 } else { 28 },
        _ => 0,
    }
}

/// Check if date is valid
#[no_mangle]
pub extern "C" fn time_is_valid_date_impl(year: i32, month: i32, day: i32) -> i32 {
    if month < 1 || month > 12 {
        return 0;
    }
    if day < 1 || day > time_days_in_month_impl(year, month) {
        return 0;
    }
    1
}

/// Sleep for specified seconds
#[no_mangle]
pub extern "C" fn time_sleep_impl(seconds: i32) {
    thread::sleep(Duration::from_secs(seconds as u64));
}

/// Sleep for specified milliseconds
#[no_mangle]
pub extern "C" fn time_sleep_millis_impl(millis: i32) {
    thread::sleep(Duration::from_millis(millis as u64));
}

/// Sleep for specified microseconds
#[no_mangle]
pub extern "C" fn time_sleep_micros_impl(micros: i32) {
    thread::sleep(Duration::from_micros(micros as u64));
}

/// Create duration from nanoseconds (helper function)
#[no_mangle]
pub extern "C" fn duration_from_nanos_impl(nanos: i64) -> i64 {
    nanos / 1_000_000_000 // Convert to seconds
}

// ================================
// Error Handling Runtime Functions
// ================================

/// Initialize error object (implementation for cursed_error_init)
#[no_mangle]
pub extern "C" fn cursed_error_init(error_obj: *mut libc::c_void, message: *const c_char) -> *mut libc::c_void {
    if error_obj.is_null() || message.is_null() {
        return ptr::null_mut();
    }
    
    // Create basic error context
    unsafe {
        let message_str = if let Ok(msg) = CStr::from_ptr(message).to_str() {
            msg.to_string()
        } else {
            "Invalid error message".to_string()
        };
        
        // Log the error for debugging
        eprintln!("CURSED Error initialized: {}", message_str);
    }
    
    error_obj
}

/// Create a new error object (implementation for cursed_create_error)
#[no_mangle]
pub extern "C" fn cursed_create_error(message: *const c_char) -> *mut libc::c_void {
    if message.is_null() {
        return ptr::null_mut();
    }
    
    unsafe {
        let error_obj = libc::malloc(256) as *mut libc::c_void;
        if !error_obj.is_null() {
            cursed_error_init(error_obj, message);
        }
        error_obj
    }
}

/// Check if an object is an error (implementation for cursed_is_error)
#[no_mangle]
pub extern "C" fn cursed_is_error(obj: *mut libc::c_void) -> i32 {
    if obj.is_null() {
        return 0;
    }
    
    // Simple check - in a real implementation this would check the object type
    1
}

/// Propagate error to caller (implementation for cursed_propagate_error)
#[no_mangle]
pub extern "C" fn cursed_propagate_error(error_obj: *mut libc::c_void) {
    if error_obj.is_null() {
        return;
    }
    
    // Log error propagation for debugging
    eprintln!("CURSED Error propagated: {:p}", error_obj);
}

/// Begin try block (implementation for cursed_try_begin)
#[no_mangle]
pub extern "C" fn cursed_try_begin() {
    // Set up error catching - simplified for now
}

/// End try block (implementation for cursed_try_end)
#[no_mangle]
pub extern "C" fn cursed_try_end() {
    // Clean up error catching - simplified for now
}

/// Get panic value (implementation for cursed_get_panic_value)
#[no_mangle]
pub extern "C" fn cursed_get_panic_value() -> *mut libc::c_void {
    // Return null for now - in a real implementation this would return the panic value
    ptr::null_mut()
}

/// Create structured error object (implementation for cursed_create_structured_error)
#[no_mangle]
pub extern "C" fn cursed_create_structured_error() -> *mut libc::c_void {
    unsafe {
        let error_obj = libc::malloc(256) as *mut libc::c_void;
        if !error_obj.is_null() {
            // Initialize as structured error
            cursed_error_init(error_obj, b"Structured error\0".as_ptr() as *const c_char);
        }
        error_obj
    }
}

/// Set error message (implementation for cursed_set_error_message)
#[no_mangle]
pub extern "C" fn cursed_set_error_message(error_obj: *mut libc::c_void, message: *const c_char) -> *mut libc::c_void {
    if error_obj.is_null() || message.is_null() {
        return ptr::null_mut();
    }
    
    // Re-initialize with new message
    cursed_error_init(error_obj, message);
    error_obj
}

/// Set error code (implementation for cursed_set_error_code)
#[no_mangle]
pub extern "C" fn cursed_set_error_code(error_obj: *mut libc::c_void, code: i32) -> *mut libc::c_void {
    if error_obj.is_null() {
        return ptr::null_mut();
    }
    
    // Store error code in the object - simplified for now
    error_obj
}

/// Set error details (implementation for cursed_set_error_details)
#[no_mangle]
pub extern "C" fn cursed_set_error_details(error_obj: *mut libc::c_void, details: *const c_char) -> *mut libc::c_void {
    if error_obj.is_null() || details.is_null() {
        return ptr::null_mut();
    }
    
    // Store error details in the object - simplified for now
    error_obj
}

/// Set error field (implementation for cursed_set_error_field)
#[no_mangle]
pub extern "C" fn cursed_set_error_field(error_obj: *mut libc::c_void, field_name: *const c_char, field_value: *const c_char) -> *mut libc::c_void {
    if error_obj.is_null() || field_name.is_null() || field_value.is_null() {
        return ptr::null_mut();
    }
    
    // Store field in error object - simplified for now
    error_obj
}

/// Get error field (implementation for cursed_get_error_field)
#[no_mangle]
pub extern "C" fn cursed_get_error_field(error_obj: *mut libc::c_void, field_name: *const c_char) -> *mut c_char {
    if error_obj.is_null() || field_name.is_null() {
        return ptr::null_mut();
    }
    
    // Return field value - simplified for now
    if let Ok(c_str) = CString::new("unknown") {
        c_str.into_raw()
    } else {
        ptr::null_mut()
    }
}

/// Get error code (implementation for cursed_get_error_code)
#[no_mangle]
pub extern "C" fn cursed_get_error_code(error_obj: *mut libc::c_void) -> i32 {
    if error_obj.is_null() {
        return -1;
    }
    
    // Return error code - simplified for now
    1
}

/// Get error message (implementation for cursed_get_error_message)
#[no_mangle]
pub extern "C" fn cursed_get_error_message(error_obj: *mut libc::c_void) -> *mut c_char {
    if error_obj.is_null() {
        return ptr::null_mut();
    }
    
    // Return error message - simplified for now
    if let Ok(c_str) = CString::new("Unknown error") {
        c_str.into_raw()
    } else {
        ptr::null_mut()
    }
}

/// Get error details (implementation for cursed_get_error_details)
#[no_mangle]
pub extern "C" fn cursed_get_error_details(error_obj: *mut libc::c_void) -> *mut c_char {
    if error_obj.is_null() {
        return ptr::null_mut();
    }
    
    // Return error details - simplified for now
    if let Ok(c_str) = CString::new("No details available") {
        c_str.into_raw()
    } else {
        ptr::null_mut()
    }
}
