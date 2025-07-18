//! C language bridge implementation
//!
//! This module provides a concrete implementation of the C language bridge
//! for FFI operations, including dynamic library loading and function calling.

use crate::error::CursedError;
use super::multi_language::{LanguageBridge, CBridge};
use super::{FfiValue, ForeignValue, LibraryHandle};

// Re-export CBridge for convenience
pub use super::multi_language::CBridge;

/// C function signature for dynamic calls
pub type CFunctionSignature = extern "C" fn();

/// C library loader
pub struct CLibraryLoader {
    /// Loaded libraries
    libraries: std::collections::HashMap<String, LibraryHandle>,
}

impl CLibraryLoader {
    /// Create new C library loader
    pub fn new() -> Self {
        Self {
            libraries: std::collections::HashMap::new(),
        }
    }
    
    /// Load a C library
    pub fn load_library(&mut self, path: &str) -> Result<&LibraryHandle, CursedError> {
        if let Some(handle) = self.libraries.get(path) {
            return Ok(handle);
        }
        
        let bridge = CBridge::new()?;
        let handle = bridge.load_library(path)?;
        
        self.libraries.insert(path.to_string(), handle);
        Ok(self.libraries.get(path).unwrap())
    }
    
    /// Get function address from library
    pub fn get_function(&self, library_path: &str, function_name: &str) -> Result<*mut std::ffi::c_void, CursedError> {
        let handle = self.libraries.get(library_path)
            .ok_or_else(|| CursedError::General(format!("Library not loaded: {}", library_path)))?;
        
        self.get_function_address(handle, function_name)
    }
    
    fn get_function_address(&self, library: &LibraryHandle, function_name: &str) -> Result<*mut std::ffi::c_void, CursedError> {
        #[cfg(unix)]
        {
            use std::ffi::CString;
            
            let c_name = CString::new(function_name)
                .map_err(|_| CursedError::General("Invalid function name".to_string()))?;
            
            let func_ptr = unsafe { libc::dlsym(library.handle, c_name.as_ptr()) };
            
            if func_ptr.is_null() {
                return Err(CursedError::General(format!("Function not found: {}", function_name)));
            }
            
            Ok(func_ptr)
        }
        
        #[cfg(windows)]
        {
            use std::ffi::CString;
            use winapi::um::libloaderapi::GetProcAddress;
            
            let c_name = CString::new(function_name)
                .map_err(|_| CursedError::General("Invalid function name".to_string()))?;
            
            let func_ptr = unsafe { GetProcAddress(library.handle as _, c_name.as_ptr()) };
            
            if func_ptr.is_null() {
                return Err(CursedError::General(format!("Function not found: {}", function_name)));
            }
            
            Ok(func_ptr as *mut std::ffi::c_void)
        }
    }
}

/// C function caller for dynamic function invocation
pub struct CFunctionCaller {
    /// Function pointer
    function_ptr: *mut std::ffi::c_void,
    
    /// Function name
    function_name: String,
}

impl CFunctionCaller {
    /// Create new C function caller
    pub fn new(function_ptr: *mut std::ffi::c_void, function_name: String) -> Self {
        Self {
            function_ptr,
            function_name,
        }
    }
    
    /// Call function with no arguments
    pub fn call_void(&self) -> Result<(), CursedError> {
        if self.function_ptr.is_null() {
            return Err(CursedError::General("Null function pointer".to_string()));
        }
        
        unsafe {
            let func: extern "C" fn() = std::mem::transmute(self.function_ptr);
            func();
        }
        
        Ok(())
    }
    
    /// Call function with integer argument
    pub fn call_int(&self, arg: i32) -> Result<i32, CursedError> {
        if self.function_ptr.is_null() {
            return Err(CursedError::General("Null function pointer".to_string()));
        }
        
        unsafe {
            let func: extern "C" fn(i32) -> i32 = std::mem::transmute(self.function_ptr);
            Ok(func(arg))
        }
    }
    
    /// Call function with two integer arguments
    pub fn call_int_int(&self, arg1: i32, arg2: i32) -> Result<i32, CursedError> {
        if self.function_ptr.is_null() {
            return Err(CursedError::General("Null function pointer".to_string()));
        }
        
        unsafe {
            let func: extern "C" fn(i32, i32) -> i32 = std::mem::transmute(self.function_ptr);
            Ok(func(arg1, arg2))
        }
    }
    
    /// Call function with string argument
    pub fn call_string(&self, arg: &str) -> Result<String, CursedError> {
        if self.function_ptr.is_null() {
            return Err(CursedError::General("Null function pointer".to_string()));
        }
        
        let c_string = std::ffi::CString::new(arg)
            .map_err(|_| CursedError::General("Invalid string argument".to_string()))?;
        
        unsafe {
            let func: extern "C" fn(*const i8) -> *const i8 = std::mem::transmute(self.function_ptr);
            let result_ptr = func(c_string.as_ptr());
            
            if result_ptr.is_null() {
                return Err(CursedError::General("Function returned null string".to_string()));
            }
            
            let result_cstr = std::ffi::CStr::from_ptr(result_ptr);
            Ok(result_cstr.to_string_lossy().into_owned())
        }
    }
    
    /// Get function name
    pub fn name(&self) -> &str {
        &self.function_name
    }
}

/// C type converter utilities
pub struct CTypeConverter;

impl CTypeConverter {
    /// Convert CURSED value to C representation
    pub fn to_c_value(value: &FfiValue) -> Result<CValue, CursedError> {
        match value {
            FfiValue::SignedInteger(val) => Ok(CValue::Int(*val as i32)),
            FfiValue::UnsignedInteger(val) => Ok(CValue::UInt(*val as u32)),
            FfiValue::Float(val) => Ok(CValue::Float(*val as f32)),
            FfiValue::Boolean(val) => Ok(CValue::Bool(*val)),
            FfiValue::String(val) => {
                let c_string = std::ffi::CString::new(val.clone())
                    .map_err(|_| CursedError::General("Invalid string for C conversion".to_string()))?;
                Ok(CValue::String(c_string))
            }
            FfiValue::Pointer(ptr) => Ok(CValue::Pointer(*ptr)),
            _ => Err(CursedError::General("Unsupported type for C conversion".to_string())),
        }
    }
    
    /// Convert C value to CURSED representation
    pub fn from_c_value(value: &CValue) -> Result<FfiValue, CursedError> {
        match value {
            CValue::Int(val) => Ok(FfiValue::SignedInteger(*val as i64)),
            CValue::UInt(val) => Ok(FfiValue::UnsignedInteger(*val as u64)),
            CValue::Float(val) => Ok(FfiValue::Float(*val as f64)),
            CValue::Bool(val) => Ok(FfiValue::Boolean(*val)),
            CValue::String(val) => {
                let string = val.to_string_lossy().into_owned();
                Ok(FfiValue::String(string))
            }
            CValue::Pointer(ptr) => Ok(FfiValue::Pointer(*ptr)),
        }
    }
}

/// C value representation
#[derive(Debug, Clone)]
pub enum CValue {
    Int(i32),
    UInt(u32),
    Float(f32),
    Bool(bool),
    String(std::ffi::CString),
    Pointer(*mut std::ffi::c_void),
}

impl CValue {
    /// Get the type name
    pub fn type_name(&self) -> &str {
        match self {
            CValue::Int(_) => "int",
            CValue::UInt(_) => "unsigned int",
            CValue::Float(_) => "float",
            CValue::Bool(_) => "bool",
            CValue::String(_) => "char*",
            CValue::Pointer(_) => "void*",
        }
    }
    
    /// Get the size of the value
    pub fn size(&self) -> usize {
        match self {
            CValue::Int(_) => std::mem::size_of::<i32>(),
            CValue::UInt(_) => std::mem::size_of::<u32>(),
            CValue::Float(_) => std::mem::size_of::<f32>(),
            CValue::Bool(_) => std::mem::size_of::<bool>(),
            CValue::String(s) => s.as_bytes().len() + 1,
            CValue::Pointer(_) => std::mem::size_of::<*mut std::ffi::c_void>(),
        }
    }
}

impl Default for CLibraryLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_c_library_loader() {
        let mut loader = CLibraryLoader::new();
        
        // Test loading a library (would need actual library for real test)
        // let result = loader.load_library("libm.so");
        // assert!(result.is_ok());
    }
    
    #[test]
    fn test_c_function_caller() {
        // Create a dummy function pointer for testing
        let dummy_ptr = std::ptr::null_mut();
        let caller = CFunctionCaller::new(dummy_ptr, "test_function".to_string());
        
        assert_eq!(caller.name(), "test_function");
        
        // Can't actually call with null pointer, but we can test the structure
        let result = caller.call_void();
        assert!(result.is_err());
    }
    
    #[test]
    fn test_c_type_converter() {
        let value = FfiValue::SignedInteger(42);
        let c_value = CTypeConverter::to_c_value(&value).unwrap();
        
        if let CValue::Int(val) = c_value {
            assert_eq!(val, 42);
        } else {
            panic!("Expected int value");
        }
        
        let back_value = CTypeConverter::from_c_value(&c_value).unwrap();
        if let FfiValue::SignedInteger(val) = back_value {
            assert_eq!(val, 42);
        } else {
            panic!("Expected signed integer value");
        }
    }
    
    #[test]
    fn test_c_value_operations() {
        let value = CValue::Int(42);
        assert_eq!(value.type_name(), "int");
        assert_eq!(value.size(), 4);
        
        let string_value = CValue::String(std::ffi::CString::new("test").unwrap());
        assert_eq!(string_value.type_name(), "char*");
        assert_eq!(string_value.size(), 5); // "test" + null terminator
    }
}
