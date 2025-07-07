//! Bridge between Rust FFI and Pure CURSED Implementations
//! 
//! This module provides a bridge layer that calls pure CURSED stdlib functions
//! instead of native Rust implementations, eliminating FFI dependencies.

use crate::error::CursedError;
use crate::runtime::value::Value;
use crate::execution::CursedExecutionEngine;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

/// Global executor instance for calling CURSED functions
static mut CURSED_EXECUTOR: Option<CursedExecutionEngine> = None;

/// Initialize the CURSED bridge with an executor
pub fn initialize_cursed_bridge(executor: CursedExecutionEngine) -> Result<(), CursedError> {
    unsafe {
        CURSED_EXECUTOR = Some(executor);
    }
    Ok(())
}

/// Call a CURSED function with arguments and return the result
fn call_cursed_function(function_name: &str, args: &[Value]) -> Result<Value, CursedError> {
    unsafe {
        if let Some(ref mut executor) = CURSED_EXECUTOR {
            executor.call_function(function_name, args)
        } else {
            Err(CursedError::runtime_error("CURSED bridge not initialized"))
        }
    }
}

/// Convert C string to CURSED Value
fn c_str_to_value(ptr: *const c_char) -> Result<Value, CursedError> {
    if ptr.is_null() {
        return Ok(Value::String("".to_string()));
    }
    
    unsafe {
        match CStr::from_ptr(ptr).to_str() {
            Ok(s) => Ok(Value::String(s.to_string())),
            Err(_) => Ok(Value::String("".to_string())),
        }
    }
}

/// Convert CURSED Value to C string
fn value_to_c_str(value: &Value) -> *mut c_char {
    match value {
        Value::String(s) => {
            match CString::new(s.clone()) {
                Ok(c_string) => c_string.into_raw(),
                Err(_) => ptr::null_mut(),
            }
        },
        Value::Integer(i) => {
            let s = i.to_string();
            match CString::new(s) {
                Ok(c_string) => c_string.into_raw(),
                Err(_) => ptr::null_mut(),
            }
        },
        Value::Boolean(b) => {
            let s = if *b { "1" } else { "0" };
            match CString::new(s) {
                Ok(c_string) => c_string.into_raw(),
                Err(_) => ptr::null_mut(),
            }
        },
        _ => ptr::null_mut(),
    }
}

// ================================
// Pure CURSED Network Functions
// ================================

/// Create TCP socket using pure CURSED implementation
pub fn cursed_tcp_create() -> i32 {
    match call_cursed_function("tcp_create", &[]) {
        Ok(Value::Integer(handle)) => handle as i32,
        _ => -1,
    }
}

/// Connect TCP socket using pure CURSED implementation
pub fn cursed_tcp_connect(handle: i32, address_ptr: *const c_char, port: i32) -> i32 {
    let address = match c_str_to_value(address_ptr) {
        Ok(addr) => addr,
        Err(_) => return -1,
    };
    
    let args = vec![
        Value::Integer(handle as i64),
        address,
        Value::Integer(port as i64),
    ];
    
    match call_cursed_function("tcp_connect", &args) {
        Ok(Value::Integer(result)) => result as i32,
        _ => -1,
    }
}

/// Bind TCP socket using pure CURSED implementation
pub fn cursed_tcp_bind(handle: i32, address_ptr: *const c_char, port: i32) -> i32 {
    let address = match c_str_to_value(address_ptr) {
        Ok(addr) => addr,
        Err(_) => return -1,
    };
    
    let args = vec![
        Value::Integer(handle as i64),
        address,
        Value::Integer(port as i64),
    ];
    
    match call_cursed_function("tcp_bind", &args) {
        Ok(Value::Integer(result)) => result as i32,
        _ => -1,
    }
}

/// Listen on TCP socket using pure CURSED implementation
pub fn cursed_tcp_listen(handle: i32, backlog: i32) -> i32 {
    let args = vec![
        Value::Integer(handle as i64),
        Value::Integer(backlog as i64),
    ];
    
    match call_cursed_function("tcp_listen", &args) {
        Ok(Value::Integer(result)) => result as i32,
        _ => -1,
    }
}

/// Accept TCP connection using pure CURSED implementation
pub fn cursed_tcp_accept(handle: i32) -> i32 {
    let args = vec![Value::Integer(handle as i64)];
    
    match call_cursed_function("tcp_accept", &args) {
        Ok(Value::Integer(result)) => result as i32,
        _ => -1,
    }
}

/// Send data over TCP socket using pure CURSED implementation
pub fn cursed_tcp_send(handle: i32, data_ptr: *const c_char) -> i32 {
    let data = match c_str_to_value(data_ptr) {
        Ok(d) => d,
        Err(_) => return -1,
    };
    
    let args = vec![
        Value::Integer(handle as i64),
        data,
    ];
    
    match call_cursed_function("tcp_send", &args) {
        Ok(Value::Integer(result)) => result as i32,
        _ => -1,
    }
}

/// Receive data from TCP socket using pure CURSED implementation
pub fn cursed_tcp_recv(handle: i32, max_size: i32) -> *mut c_char {
    let args = vec![
        Value::Integer(handle as i64),
        Value::Integer(max_size as i64),
    ];
    
    match call_cursed_function("tcp_recv", &args) {
        Ok(result) => value_to_c_str(&result),
        _ => ptr::null_mut(),
    }
}

/// Close TCP socket using pure CURSED implementation
pub fn cursed_tcp_close(handle: i32) {
    let args = vec![Value::Integer(handle as i64)];
    let _ = call_cursed_function("tcp_close", &args);
}

/// Create UDP socket using pure CURSED implementation
pub fn cursed_udp_create() -> i32 {
    match call_cursed_function("udp_create", &[]) {
        Ok(Value::Integer(handle)) => handle as i32,
        _ => -1,
    }
}

/// Bind UDP socket using pure CURSED implementation
pub fn cursed_udp_bind(handle: i32, address_ptr: *const c_char, port: i32) -> i32 {
    let address = match c_str_to_value(address_ptr) {
        Ok(addr) => addr,
        Err(_) => return -1,
    };
    
    let args = vec![
        Value::Integer(handle as i64),
        address,
        Value::Integer(port as i64),
    ];
    
    match call_cursed_function("udp_bind", &args) {
        Ok(Value::Integer(result)) => result as i32,
        _ => -1,
    }
}

/// Send UDP data using pure CURSED implementation
pub fn cursed_udp_send_to(handle: i32, data_ptr: *const c_char, address_ptr: *const c_char, port: i32) -> i32 {
    let data = match c_str_to_value(data_ptr) {
        Ok(d) => d,
        Err(_) => return -1,
    };
    
    let address = match c_str_to_value(address_ptr) {
        Ok(addr) => addr,
        Err(_) => return -1,
    };
    
    let args = vec![
        Value::Integer(handle as i64),
        data,
        address,
        Value::Integer(port as i64),
    ];
    
    match call_cursed_function("udp_send_to", &args) {
        Ok(Value::Integer(result)) => result as i32,
        _ => -1,
    }
}

/// Receive UDP data using pure CURSED implementation
pub fn cursed_udp_recv_from(handle: i32, max_size: i32) -> *mut c_char {
    let args = vec![
        Value::Integer(handle as i64),
        Value::Integer(max_size as i64),
    ];
    
    match call_cursed_function("udp_recv_from", &args) {
        Ok(result) => value_to_c_str(&result),
        _ => ptr::null_mut(),
    }
}

/// Close UDP socket using pure CURSED implementation
pub fn cursed_udp_close(handle: i32) {
    let args = vec![Value::Integer(handle as i64)];
    let _ = call_cursed_function("udp_close", &args);
}

/// Resolve hostname using pure CURSED implementation
pub fn cursed_resolve_hostname(hostname_ptr: *const c_char) -> *mut c_char {
    let hostname = match c_str_to_value(hostname_ptr) {
        Ok(h) => h,
        Err(_) => return ptr::null_mut(),
    };
    
    let args = vec![hostname];
    
    match call_cursed_function("resolve_hostname", &args) {
        Ok(result) => value_to_c_str(&result),
        _ => ptr::null_mut(),
    }
}

/// Resolve IP to hostname using pure CURSED implementation
pub fn cursed_resolve_ip(ip_ptr: *const c_char) -> *mut c_char {
    let ip = match c_str_to_value(ip_ptr) {
        Ok(i) => i,
        Err(_) => return ptr::null_mut(),
    };
    
    let args = vec![ip];
    
    match call_cursed_function("resolve_ip", &args) {
        Ok(result) => value_to_c_str(&result),
        _ => ptr::null_mut(),
    }
}

/// Lookup MX records using pure CURSED implementation
pub fn cursed_lookup_mx(domain_ptr: *const c_char) -> *mut c_char {
    let domain = match c_str_to_value(domain_ptr) {
        Ok(d) => d,
        Err(_) => return ptr::null_mut(),
    };
    
    let args = vec![domain];
    
    match call_cursed_function("lookup_mx", &args) {
        Ok(result) => value_to_c_str(&result),
        _ => ptr::null_mut(),
    }
}

/// Lookup TXT records using pure CURSED implementation
pub fn cursed_lookup_txt(domain_ptr: *const c_char) -> *mut c_char {
    let domain = match c_str_to_value(domain_ptr) {
        Ok(d) => d,
        Err(_) => return ptr::null_mut(),
    };
    
    let args = vec![domain];
    
    match call_cursed_function("lookup_txt", &args) {
        Ok(result) => value_to_c_str(&result),
        _ => ptr::null_mut(),
    }
}

/// Send HTTP request using pure CURSED implementation
pub fn cursed_http_send(method_ptr: *const c_char, url_ptr: *const c_char, headers_ptr: *const c_char, body_ptr: *const c_char) -> *mut c_char {
    let method = match c_str_to_value(method_ptr) {
        Ok(m) => m,
        Err(_) => return ptr::null_mut(),
    };
    
    let url = match c_str_to_value(url_ptr) {
        Ok(u) => u,
        Err(_) => return ptr::null_mut(),
    };
    
    let headers = match c_str_to_value(headers_ptr) {
        Ok(h) => h,
        Err(_) => Value::String("".to_string()),
    };
    
    let body = match c_str_to_value(body_ptr) {
        Ok(b) => b,
        Err(_) => Value::String("".to_string()),
    };
    
    let args = vec![method, url, headers, body];
    
    match call_cursed_function("http_send", &args) {
        Ok(result) => value_to_c_str(&result),
        _ => ptr::null_mut(),
    }
}

/// Get local IP using pure CURSED implementation
pub fn cursed_get_local_ip() -> *mut c_char {
    match call_cursed_function("get_local_ip", &[]) {
        Ok(result) => value_to_c_str(&result),
        _ => ptr::null_mut(),
    }
}

/// Ping host using pure CURSED implementation
pub fn cursed_ping(hostname_ptr: *const c_char) -> i32 {
    let hostname = match c_str_to_value(hostname_ptr) {
        Ok(h) => h,
        Err(_) => return 0,
    };
    
    let args = vec![hostname];
    
    match call_cursed_function("ping", &args) {
        Ok(Value::Boolean(result)) => if result { 1 } else { 0 },
        Ok(Value::Integer(result)) => result as i32,
        _ => 0,
    }
}

/// Network scan using pure CURSED implementation
pub fn cursed_network_scan(start_ip_ptr: *const c_char, end_ip_ptr: *const c_char, port: i32) -> *mut c_char {
    let start_ip = match c_str_to_value(start_ip_ptr) {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };
    
    let end_ip = match c_str_to_value(end_ip_ptr) {
        Ok(e) => e,
        Err(_) => return ptr::null_mut(),
    };
    
    let args = vec![
        start_ip,
        end_ip,
        Value::Integer(port as i64),
    ];
    
    match call_cursed_function("network_scan", &args) {
        Ok(result) => value_to_c_str(&result),
        _ => ptr::null_mut(),
    }
}

/// Get remote address using pure CURSED implementation
pub fn cursed_get_remote_addr(handle: i32) -> *mut c_char {
    let args = vec![Value::Integer(handle as i64)];
    
    match call_cursed_function("get_remote_addr", &args) {
        Ok(result) => value_to_c_str(&result),
        _ => ptr::null_mut(),
    }
}

/// TLS initialization using pure CURSED implementation
pub fn cursed_tls_init(handle: i32, hostname_ptr: *const c_char) -> i32 {
    let hostname = match c_str_to_value(hostname_ptr) {
        Ok(h) => h,
        Err(_) => return 0,
    };
    
    let args = vec![
        Value::Integer(handle as i64),
        hostname,
    ];
    
    match call_cursed_function("tls_init", &args) {
        Ok(Value::Boolean(result)) => if result { 1 } else { 0 },
        Ok(Value::Integer(result)) => result as i32,
        _ => 0,
    }
}

/// TLS send using pure CURSED implementation
pub fn cursed_tls_send(handle: i32, data_ptr: *const c_char) -> i32 {
    let data = match c_str_to_value(data_ptr) {
        Ok(d) => d,
        Err(_) => return -1,
    };
    
    let args = vec![
        Value::Integer(handle as i64),
        data,
    ];
    
    match call_cursed_function("tls_send", &args) {
        Ok(Value::Integer(result)) => result as i32,
        _ => -1,
    }
}

/// TLS receive using pure CURSED implementation
pub fn cursed_tls_recv(handle: i32, max_size: i32) -> *mut c_char {
    let args = vec![
        Value::Integer(handle as i64),
        Value::Integer(max_size as i64),
    ];
    
    match call_cursed_function("tls_recv", &args) {
        Ok(result) => value_to_c_str(&result),
        _ => ptr::null_mut(),
    }
}
