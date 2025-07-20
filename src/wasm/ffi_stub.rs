//! FFI stub for WASM
//! 
//! Provides stub implementations for FFI operations that are not
//! available in WebAssembly environments.

use std::io::{Error, ErrorKind, Result};

/// FFI stub for WASM that provides no-op implementations
/// for FFI operations that are not available in WASM
pub struct WasmFFIStub;

impl WasmFFIStub {
    pub fn new() -> Self {
        Self
    }

    /// Stub for dynamic library loading
    pub fn load_library(&self, _path: &str) -> Result<()> {
        Err(Error::new(
            ErrorKind::Unsupported,
            "Dynamic library loading not supported in WASM"
        ))
    }

    /// Stub for getting function from library
    pub fn get_function(&self, _lib: &str, _name: &str) -> Result<()> {
        Err(Error::new(
            ErrorKind::Unsupported,
            "Function loading from libraries not supported in WASM"
        ))
    }

    /// Stub for calling C functions
    pub fn call_c_function(&self, _func: &str, _args: &[u8]) -> Result<Vec<u8>> {
        Err(Error::new(
            ErrorKind::Unsupported,
            "C function calls not supported in WASM"
        ))
    }

    /// Stub for malloc/free operations
    pub fn allocate_memory(&self, _size: usize) -> Result<*mut u8> {
        Err(Error::new(
            ErrorKind::Unsupported,
            "Direct memory allocation not supported in WASM - use Vec/Box instead"
        ))
    }

    pub fn free_memory(&self, _ptr: *mut u8) -> Result<()> {
        Err(Error::new(
            ErrorKind::Unsupported,
            "Direct memory deallocation not supported in WASM - use Vec/Box instead"
        ))
    }

    /// Stub for signal handling
    pub fn install_signal_handler(&self, _signal: i32) -> Result<()> {
        Err(Error::new(
            ErrorKind::Unsupported,
            "Signal handling not supported in WASM"
        ))
    }

    /// Stub for process operations
    pub fn spawn_process(&self, _command: &str, _args: &[&str]) -> Result<()> {
        Err(Error::new(
            ErrorKind::Unsupported,
            "Process spawning not supported in WASM"
        ))
    }
}

/// Conditional compilation wrappers for common FFI operations

#[cfg(target_arch = "wasm32")]
pub fn malloc(_size: usize) -> *mut u8 {
    std::ptr::null_mut()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn malloc(size: usize) -> *mut u8 {
    unsafe { libc::malloc(size) as *mut u8 }
}

#[cfg(target_arch = "wasm32")]
pub fn free(_ptr: *mut u8) {
    // No-op in WASM
}

#[cfg(not(target_arch = "wasm32"))]
pub fn free(ptr: *mut u8) {
    unsafe { libc::free(ptr as *mut libc::c_void) }
}

#[cfg(target_arch = "wasm32")]
pub fn dlopen(_filename: &str) -> Result<()> {
    Err(Error::new(
        ErrorKind::Unsupported,
        "Dynamic library loading not supported in WASM"
    ))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn dlopen(filename: &str) -> Result<*mut libc::c_void> {
    use std::ffi::CString;
    let c_filename = CString::new(filename)
        .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
    
    let handle = unsafe { libc::dlopen(c_filename.as_ptr(), libc::RTLD_LAZY) };
    if handle.is_null() {
        Err(Error::new(ErrorKind::NotFound, "Failed to load library"))
    } else {
        Ok(handle)
    }
}

#[cfg(target_arch = "wasm32")]
pub fn dlsym(_handle: *mut u8, _symbol: &str) -> Result<()> {
    Err(Error::new(
        ErrorKind::Unsupported,
        "Symbol lookup not supported in WASM"
    ))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn dlsym(handle: *mut libc::c_void, symbol: &str) -> Result<*mut libc::c_void> {
    use std::ffi::CString;
    let c_symbol = CString::new(symbol)
        .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
    
    let sym = unsafe { libc::dlsym(handle, c_symbol.as_ptr()) };
    if sym.is_null() {
        Err(Error::new(ErrorKind::NotFound, "Symbol not found"))
    } else {
        Ok(sym)
    }
}

#[cfg(target_arch = "wasm32")]
pub fn dlclose(_handle: *mut u8) -> Result<()> {
    Ok(()) // No-op in WASM
}

#[cfg(not(target_arch = "wasm32"))]
pub fn dlclose(handle: *mut libc::c_void) -> Result<()> {
    let result = unsafe { libc::dlclose(handle) };
    if result != 0 {
        Err(Error::new(ErrorKind::Other, "Failed to close library"))
    } else {
        Ok(())
    }
}
