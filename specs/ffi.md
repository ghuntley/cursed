# FFI (Foreign Function Interface) Specification

## Overview

The CURSED FFI system provides safe, efficient interoperability with C libraries and native code. It includes comprehensive type marshaling, memory management across language boundaries, and runtime bridge APIs for seamless integration.

## Architecture

### Core Components

1. **FFI Bridge** - Runtime bridge between CURSED and C code
2. **Type Marshaling** - Automatic conversion between CURSED and C types
3. **Memory Management** - Safe memory handling across boundaries
4. **C Runtime Bridge** - Standard library integration
5. **Safety Layer** - Memory safety and error handling

### System Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│     CURSED      │    │   FFI Bridge    │    │    C Runtime   │
│    Runtime      │◄──►│                 │◄──►│                 │
│                 │    │                 │    │                 │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│  Type Marshal   │    │ Memory Manager  │    │  Native Libs    │
│                 │    │                 │    │                 │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

## Type System Integration

### CURSED to C Type Mapping

```rust
pub enum CType {
    /// C integer types
    CInt,
    CLong,
    CUint,
    CUlong,
    CSize,
    
    /// C floating point types
    CFloat,
    CDouble,
    
    /// C string types
    CChar,
    CString,
    
    /// C pointer types
    CPointer(*mut c_void),
    
    /// C array types
    CArray { element_type: Box<CType>, length: usize },
    
    /// C struct types
    CStruct { name: String, fields: Vec<CField> },
    
    /// C function pointer types
    CFunctionPointer {
        return_type: Box<CType>,
        parameters: Vec<CType>,
    },
}
```

### Automatic Type Conversion

```rust
impl TypeMarshaler {
    pub fn cursed_to_c(&self, value: &CursedValue, c_type: &CType) -> Result<CValue, FfiError> {
        match (value, c_type) {
            (CursedValue::Integer(i), CType::CInt) => {
                Ok(CValue::CInt(*i as c_int))
            }
            (CursedValue::Float(f), CType::CDouble) => {
                Ok(CValue::CDouble(*f as c_double))
            }
            (CursedValue::String(s), CType::CString) => {
                let c_string = CString::new(s.clone())?;
                Ok(CValue::CString(c_string))
            }
            (CursedValue::Boolean(b), CType::CInt) => {
                Ok(CValue::CInt(if *b { 1 } else { 0 }))
            }
            (CursedValue::Array(arr), CType::CArray { element_type, length }) => {
                self.marshal_array(arr, element_type, *length)
            }
            (CursedValue::Struct(s), CType::CStruct { name, fields }) => {
                self.marshal_struct(s, name, fields)
            }
            _ => Err(FfiError::IncompatibleTypes {
                cursed_type: value.type_name(),
                c_type: c_type.name(),
            }),
        }
    }
    
    pub fn c_to_cursed(&self, value: &CValue, cursed_type: &CursedType) -> Result<CursedValue, FfiError> {
        match (value, cursed_type) {
            (CValue::CInt(i), CursedType::Integer) => {
                Ok(CursedValue::Integer(*i as i64))
            }
            (CValue::CDouble(d), CursedType::Float) => {
                Ok(CursedValue::Float(*d))
            }
            (CValue::CString(s), CursedType::String) => {
                let rust_string = s.to_string_lossy().into_owned();
                Ok(CursedValue::String(rust_string))
            }
            (CValue::CInt(i), CursedType::Boolean) => {
                Ok(CursedValue::Boolean(*i != 0))
            }
            _ => Err(FfiError::IncompatibleTypes {
                c_type: value.type_name(),
                cursed_type: cursed_type.name(),
            }),
        }
    }
}
```

## Memory Management

### Cross-Boundary Memory Safety

#### 1. Memory Ownership Model

```rust
pub enum MemoryOwnership {
    /// CURSED owns the memory
    CursedOwned,
    /// C code owns the memory
    COwned,
    /// Shared ownership with reference counting
    Shared(Arc<MemoryDescriptor>),
    /// Temporary borrow
    Borrowed { lifetime: Duration },
}

pub struct MemoryDescriptor {
    ptr: *mut c_void,
    size: usize,
    ownership: MemoryOwnership,
    cleanup: Option<Box<dyn FnOnce() + Send + Sync>>,
}
```

#### 2. Memory Allocation Strategies

```rust
impl FfiMemoryManager {
    /// Allocate memory for passing to C code
    pub fn allocate_for_c(&mut self, size: usize) -> Result<*mut c_void, FfiError> {
        let ptr = unsafe { libc::malloc(size) };
        if ptr.is_null() {
            return Err(FfiError::AllocationFailed { size });
        }
        
        // Register for cleanup
        self.register_c_allocation(ptr, size);
        Ok(ptr)
    }
    
    /// Allocate memory in CURSED heap for C interop
    pub fn allocate_cursed_for_c(&mut self, size: usize) -> Result<*mut c_void, FfiError> {
        let ptr = self.cursed_allocator.allocate(size)?;
        
        // Pin memory to prevent GC movement
        self.gc_integration.pin_memory(ptr, size)?;
        
        Ok(ptr as *mut c_void)
    }
    
    /// Free memory allocated for C interop
    pub fn deallocate_c_memory(&mut self, ptr: *mut c_void) -> Result<(), FfiError> {
        if let Some(descriptor) = self.memory_descriptors.remove(&ptr) {
            match descriptor.ownership {
                MemoryOwnership::CursedOwned => {
                    self.cursed_allocator.deallocate(ptr as *mut u8)?;
                }
                MemoryOwnership::COwned => {
                    unsafe { libc::free(ptr); }
                }
                MemoryOwnership::Shared(arc) => {
                    // Reference counting handles cleanup
                    drop(arc);
                }
                MemoryOwnership::Borrowed { .. } => {
                    // No cleanup needed for borrowed memory
                }
            }
            
            // Execute cleanup function if provided
            if let Some(cleanup) = descriptor.cleanup {
                cleanup();
            }
            
            Ok(())
        } else {
            Err(FfiError::InvalidPointer(ptr))
        }
    }
}
```

### String Handling

#### String Marshaling

```rust
impl StringMarshaler {
    /// Convert CURSED string to C string
    pub fn cursed_to_c_string(&self, s: &str) -> Result<CString, FfiError> {
        CString::new(s).map_err(|e| FfiError::StringConversion {
            reason: format!("Null byte in string: {}", e),
        })
    }
    
    /// Convert C string to CURSED string
    pub fn c_to_cursed_string(&self, c_str: *const c_char) -> Result<String, FfiError> {
        if c_str.is_null() {
            return Err(FfiError::NullPointer);
        }
        
        let c_string = unsafe { CStr::from_ptr(c_str) };
        c_string.to_str()
            .map(|s| s.to_owned())
            .map_err(|e| FfiError::StringConversion {
                reason: format!("Invalid UTF-8: {}", e),
            })
    }
    
    /// Convert C string with length to CURSED string
    pub fn c_to_cursed_string_with_len(&self, c_str: *const c_char, len: usize) -> Result<String, FfiError> {
        if c_str.is_null() {
            return Err(FfiError::NullPointer);
        }
        
        let bytes = unsafe { std::slice::from_raw_parts(c_str as *const u8, len) };
        std::str::from_utf8(bytes)
            .map(|s| s.to_owned())
            .map_err(|e| FfiError::StringConversion {
                reason: format!("Invalid UTF-8: {}", e),
            })
    }
}
```

### Array and Buffer Management

```rust
impl ArrayMarshaler {
    /// Marshal CURSED array to C array
    pub fn marshal_array<T>(&self, cursed_array: &[T], c_type: &CType) -> Result<(*mut c_void, usize), FfiError> 
    where 
        T: Clone + CursedToCConvertible
    {
        let len = cursed_array.len();
        let element_size = c_type.size();
        let total_size = len * element_size;
        
        // Allocate C array
        let c_array = self.memory_manager.allocate_for_c(total_size)?;
        
        // Convert and copy elements
        for (i, element) in cursed_array.iter().enumerate() {
            let c_element = element.to_c_value(c_type)?;
            unsafe {
                let dest = (c_array as *mut u8).add(i * element_size);
                c_element.copy_to_ptr(dest);
            }
        }
        
        Ok((c_array, len))
    }
    
    /// Marshal C array to CURSED array
    pub fn unmarshal_array<T>(&self, c_array: *const c_void, len: usize, cursed_type: &CursedType) -> Result<Vec<T>, FfiError>
    where
        T: CToCursedConvertible
    {
        if c_array.is_null() {
            return Err(FfiError::NullPointer);
        }
        
        let element_size = cursed_type.size();
        let mut result = Vec::with_capacity(len);
        
        for i in 0..len {
            unsafe {
                let src = (c_array as *const u8).add(i * element_size);
                let c_value = CValue::from_ptr(src, cursed_type)?;
                let cursed_value = T::from_c_value(c_value)?;
                result.push(cursed_value);
            }
        }
        
        Ok(result)
    }
}
```

## Runtime Bridge APIs

### C Runtime Integration

```rust
/// C runtime bridge for standard library functions
pub struct CRuntimeBridge {
    /// Function registry
    functions: HashMap<String, Box<dyn CFunction>>,
    /// Type registry
    types: HashMap<String, CType>,
    /// Memory manager
    memory_manager: Arc<Mutex<FfiMemoryManager>>,
}

impl CRuntimeBridge {
    pub fn new() -> Self {
        let mut bridge = Self {
            functions: HashMap::new(),
            types: HashMap::new(),
            memory_manager: Arc::new(Mutex::new(FfiMemoryManager::new())),
        };
        
        // Register standard C functions
        bridge.register_standard_functions();
        bridge.register_standard_types();
        
        bridge
    }
    
    fn register_standard_functions(&mut self) {
        // String functions
        self.register_function("strlen", Box::new(StrlenFunction));
        self.register_function("strcpy", Box::new(StrcpyFunction));
        self.register_function("strcat", Box::new(StrcatFunction));
        self.register_function("strcmp", Box::new(StrcmpFunction));
        
        // Memory functions
        self.register_function("malloc", Box::new(MallocFunction));
        self.register_function("free", Box::new(FreeFunction));
        self.register_function("memcpy", Box::new(MemcpyFunction));
        self.register_function("memset", Box::new(MemsetFunction));
        
        // I/O functions
        self.register_function("printf", Box::new(PrintfFunction));
        self.register_function("fprintf", Box::new(FprintfFunction));
        self.register_function("fopen", Box::new(FopenFunction));
        self.register_function("fclose", Box::new(FcloseFunction));
        
        // Math functions
        self.register_function("sin", Box::new(SinFunction));
        self.register_function("cos", Box::new(CosFunction));
        self.register_function("sqrt", Box::new(SqrtFunction));
        self.register_function("pow", Box::new(PowFunction));
    }
    
    fn register_standard_types(&mut self) {
        self.types.insert("int".to_string(), CType::CInt);
        self.types.insert("long".to_string(), CType::CLong);
        self.types.insert("float".to_string(), CType::CFloat);
        self.types.insert("double".to_string(), CType::CDouble);
        self.types.insert("char*".to_string(), CType::CString);
        self.types.insert("void*".to_string(), CType::CPointer(std::ptr::null_mut()));
    }
    
    pub fn call_function(&self, name: &str, args: &[CursedValue]) -> Result<CursedValue, FfiError> {
        if let Some(function) = self.functions.get(name) {
            function.call(args, &self.memory_manager)
        } else {
            Err(FfiError::FunctionNotFound(name.to_string()))
        }
    }
}
```

### Function Call Interface

```rust
pub trait CFunction: Send + Sync {
    fn name(&self) -> &str;
    fn signature(&self) -> &CFunctionSignature;
    fn call(&self, args: &[CursedValue], memory_manager: &Arc<Mutex<FfiMemoryManager>>) -> Result<CursedValue, FfiError>;
}

pub struct CFunctionSignature {
    pub return_type: CType,
    pub parameters: Vec<CParameter>,
    pub is_variadic: bool,
}

pub struct CParameter {
    pub name: String,
    pub param_type: CType,
    pub is_const: bool,
}

// Example implementation for strlen
pub struct StrlenFunction;

impl CFunction for StrlenFunction {
    fn name(&self) -> &str {
        "strlen"
    }
    
    fn signature(&self) -> &CFunctionSignature {
        static SIGNATURE: CFunctionSignature = CFunctionSignature {
            return_type: CType::CSize,
            parameters: vec![CParameter {
                name: "str".to_string(),
                param_type: CType::CString,
                is_const: true,
            }],
            is_variadic: false,
        };
        &SIGNATURE
    }
    
    fn call(&self, args: &[CursedValue], _memory_manager: &Arc<Mutex<FfiMemoryManager>>) -> Result<CursedValue, FfiError> {
        if args.len() != 1 {
            return Err(FfiError::InvalidArgumentCount {
                expected: 1,
                actual: args.len(),
            });
        }
        
        match &args[0] {
            CursedValue::String(s) => {
                let len = s.len();
                Ok(CursedValue::Integer(len as i64))
            }
            _ => Err(FfiError::InvalidArgumentType {
                expected: "string".to_string(),
                actual: args[0].type_name(),
            }),
        }
    }
}
```

## Safety Considerations

### Memory Safety

#### 1. Pointer Validation

```rust
impl PointerValidator {
    pub fn validate_pointer(&self, ptr: *const c_void, size: usize) -> Result<(), FfiError> {
        if ptr.is_null() {
            return Err(FfiError::NullPointer);
        }
        
        // Check if pointer is within valid memory range
        if !self.is_valid_memory_range(ptr, size) {
            return Err(FfiError::InvalidMemoryRange {
                ptr: ptr as usize,
                size,
            });
        }
        
        // Check if pointer is properly aligned
        if !self.is_properly_aligned(ptr, size) {
            return Err(FfiError::MisalignedPointer {
                ptr: ptr as usize,
                alignment: self.get_required_alignment(size),
            });
        }
        
        Ok(())
    }
    
    fn is_valid_memory_range(&self, ptr: *const c_void, size: usize) -> bool {
        // Platform-specific memory range validation
        #[cfg(target_os = "linux")]
        {
            // Check /proc/self/maps for valid memory ranges
            self.check_proc_maps(ptr, size)
        }
        
        #[cfg(target_os = "windows")]
        {
            // Use VirtualQuery to check memory validity
            self.check_virtual_query(ptr, size)
        }
        
        #[cfg(target_os = "macos")]
        {
            // Use mach_vm_region to check memory validity
            self.check_mach_vm_region(ptr, size)
        }
    }
}
```

#### 2. Buffer Overflow Protection

```rust
impl BufferGuard {
    pub fn create_guarded_buffer(&self, size: usize) -> Result<GuardedBuffer, FfiError> {
        let total_size = size + 2 * self.guard_size;
        let raw_ptr = self.memory_manager.allocate_for_c(total_size)?;
        
        // Set up guard pages
        let guard_before = raw_ptr;
        let buffer_ptr = unsafe { (raw_ptr as *mut u8).add(self.guard_size) };
        let guard_after = unsafe { (raw_ptr as *mut u8).add(self.guard_size + size) };
        
        // Mark guard pages as no-access
        self.protect_guard_page(guard_before, self.guard_size)?;
        self.protect_guard_page(guard_after, self.guard_size)?;
        
        Ok(GuardedBuffer {
            raw_ptr,
            buffer_ptr: buffer_ptr as *mut c_void,
            size,
            guard_size: self.guard_size,
        })
    }
    
    fn protect_guard_page(&self, ptr: *mut c_void, size: usize) -> Result<(), FfiError> {
        #[cfg(unix)]
        {
            let result = unsafe { libc::mprotect(ptr, size, libc::PROT_NONE) };
            if result != 0 {
                return Err(FfiError::ProtectionFailed {
                    ptr: ptr as usize,
                    size,
                });
            }
        }
        
        #[cfg(windows)]
        {
            use winapi::um::memoryapi::VirtualProtect;
            use winapi::um::winnt::PAGE_NOACCESS;
            
            let mut old_protect = 0;
            let result = unsafe {
                VirtualProtect(ptr, size, PAGE_NOACCESS, &mut old_protect)
            };
            if result == 0 {
                return Err(FfiError::ProtectionFailed {
                    ptr: ptr as usize,
                    size,
                });
            }
        }
        
        Ok(())
    }
}
```

### Thread Safety

#### 1. Synchronization

```rust
pub struct ThreadSafeFfiBridge {
    inner: Arc<RwLock<FfiBridge>>,
    call_count: AtomicUsize,
    active_calls: AtomicUsize,
}

impl ThreadSafeFfiBridge {
    pub fn call_function(&self, name: &str, args: &[CursedValue]) -> Result<CursedValue, FfiError> {
        let _guard = self.inner.read().map_err(|_| FfiError::LockPoisoned)?;
        
        // Increment active call count
        self.active_calls.fetch_add(1, Ordering::SeqCst);
        let _call_guard = CallGuard::new(&self.active_calls);
        
        // Track total calls
        self.call_count.fetch_add(1, Ordering::SeqCst);
        
        // Perform the actual function call
        self.inner.call_function(name, args)
    }
}

struct CallGuard<'a> {
    counter: &'a AtomicUsize,
}

impl<'a> CallGuard<'a> {
    fn new(counter: &'a AtomicUsize) -> Self {
        Self { counter }
    }
}

impl<'a> Drop for CallGuard<'a> {
    fn drop(&mut self) {
        self.counter.fetch_sub(1, Ordering::SeqCst);
    }
}
```

#### 2. Callback Safety

```rust
pub struct CallbackManager {
    callbacks: Arc<RwLock<HashMap<CallbackId, Box<dyn Callback>>>>,
    next_id: AtomicUsize,
}

impl CallbackManager {
    pub fn register_callback<F>(&self, callback: F) -> Result<CallbackId, FfiError>
    where
        F: Fn(&[CursedValue]) -> Result<CursedValue, FfiError> + Send + Sync + 'static,
    {
        let id = CallbackId(self.next_id.fetch_add(1, Ordering::SeqCst));
        let mut callbacks = self.callbacks.write().map_err(|_| FfiError::LockPoisoned)?;
        callbacks.insert(id, Box::new(callback));
        Ok(id)
    }
    
    pub fn call_callback(&self, id: CallbackId, args: &[CursedValue]) -> Result<CursedValue, FfiError> {
        let callbacks = self.callbacks.read().map_err(|_| FfiError::LockPoisoned)?;
        if let Some(callback) = callbacks.get(&id) {
            callback.call(args)
        } else {
            Err(FfiError::CallbackNotFound(id))
        }
    }
}

// C-callable wrapper for callbacks
extern "C" fn callback_wrapper(
    id: usize,
    args: *const CValue,
    arg_count: usize,
    result: *mut CValue,
) -> i32 {
    // Safety wrapper that handles panics and converts between C and CURSED values
    let callback_id = CallbackId(id);
    
    // Convert C arguments to CURSED values
    let cursed_args = unsafe {
        std::slice::from_raw_parts(args, arg_count)
            .iter()
            .map(|c_val| c_val.to_cursed_value())
            .collect::<Result<Vec<_>, _>>()
    };
    
    let cursed_args = match cursed_args {
        Ok(args) => args,
        Err(_) => return -1, // Error converting arguments
    };
    
    // Call the callback
    let callback_result = GLOBAL_CALLBACK_MANAGER.call_callback(callback_id, &cursed_args);
    
    match callback_result {
        Ok(cursed_result) => {
            // Convert result back to C value
            match cursed_result.to_c_value() {
                Ok(c_result) => {
                    unsafe { *result = c_result; }
                    0 // Success
                }
                Err(_) => -1, // Error converting result
            }
        }
        Err(_) => -1, // Error in callback
    }
}
```

## Error Handling

### FFI Error Types

```rust
#[derive(Debug, Clone)]
pub enum FfiError {
    /// Type conversion error
    IncompatibleTypes {
        cursed_type: String,
        c_type: String,
    },
    
    /// Memory allocation failed
    AllocationFailed {
        size: usize,
    },
    
    /// Invalid pointer
    InvalidPointer(*mut c_void),
    
    /// Null pointer dereference
    NullPointer,
    
    /// Invalid memory range
    InvalidMemoryRange {
        ptr: usize,
        size: usize,
    },
    
    /// Misaligned pointer
    MisalignedPointer {
        ptr: usize,
        alignment: usize,
    },
    
    /// Function not found
    FunctionNotFound(String),
    
    /// Invalid argument count
    InvalidArgumentCount {
        expected: usize,
        actual: usize,
    },
    
    /// Invalid argument type
    InvalidArgumentType {
        expected: String,
        actual: String,
    },
    
    /// String conversion error
    StringConversion {
        reason: String,
    },
    
    /// Callback not found
    CallbackNotFound(CallbackId),
    
    /// Lock poisoned
    LockPoisoned,
    
    /// Protection failed
    ProtectionFailed {
        ptr: usize,
        size: usize,
    },
    
    /// Library load error
    LibraryLoadError {
        path: String,
        error: String,
    },
    
    /// Symbol not found
    SymbolNotFound {
        library: String,
        symbol: String,
    },
}
```

### Error Recovery

```rust
impl FfiErrorRecovery {
    pub fn handle_error(&self, error: &FfiError) -> RecoveryAction {
        match error {
            FfiError::AllocationFailed { size } => {
                if *size > self.max_allocation_size {
                    RecoveryAction::Reject
                } else {
                    RecoveryAction::Retry
                }
            }
            
            FfiError::InvalidPointer(_) => {
                RecoveryAction::Cleanup
            }
            
            FfiError::NullPointer => {
                RecoveryAction::ReturnDefault
            }
            
            FfiError::FunctionNotFound(_) => {
                RecoveryAction::LoadLibrary
            }
            
            _ => RecoveryAction::Propagate,
        }
    }
}

pub enum RecoveryAction {
    Retry,
    Reject,
    Cleanup,
    ReturnDefault,
    LoadLibrary,
    Propagate,
}
```

## Performance Considerations

### Optimization Strategies

#### 1. Call Optimization

```rust
impl FfiOptimizer {
    pub fn optimize_call(&self, function: &str, args: &[CursedValue]) -> OptimizedCall {
        // Cache frequently called functions
        if let Some(cached) = self.call_cache.get(function) {
            return cached.clone();
        }
        
        // Batch small allocations
        let total_size = args.iter().map(|arg| arg.size()).sum::<usize>();
        if total_size < self.batch_threshold {
            return OptimizedCall::Batched(self.create_batch_call(function, args));
        }
        
        // Use stack allocation for small arguments
        if total_size < self.stack_threshold {
            return OptimizedCall::Stack(self.create_stack_call(function, args));
        }
        
        OptimizedCall::Heap(self.create_heap_call(function, args))
    }
}
```

#### 2. Memory Pool Management

```rust
pub struct FfiMemoryPool {
    small_pools: Vec<MemoryPool>,
    large_pools: Vec<MemoryPool>,
    recycled_pointers: HashMap<usize, Vec<*mut c_void>>,
}

impl FfiMemoryPool {
    pub fn allocate(&mut self, size: usize) -> Result<*mut c_void, FfiError> {
        // Try to reuse recycled memory
        if let Some(pool) = self.recycled_pointers.get_mut(&size) {
            if let Some(ptr) = pool.pop() {
                return Ok(ptr);
            }
        }
        
        // Allocate from appropriate pool
        if size <= self.small_pool_threshold {
            self.allocate_from_small_pool(size)
        } else {
            self.allocate_from_large_pool(size)
        }
    }
    
    pub fn deallocate(&mut self, ptr: *mut c_void, size: usize) -> Result<(), FfiError> {
        // Add to recycled pool if it's a common size
        if self.is_common_size(size) {
            self.recycled_pointers.entry(size).or_insert_with(Vec::new).push(ptr);
            Ok(())
        } else {
            // Free immediately for uncommon sizes
            unsafe { libc::free(ptr); }
            Ok(())
        }
    }
}
```

### Performance Monitoring

```rust
pub struct FfiPerformanceMonitor {
    call_counts: HashMap<String, u64>,
    call_times: HashMap<String, Duration>,
    memory_usage: AtomicUsize,
    allocation_count: AtomicUsize,
}

impl FfiPerformanceMonitor {
    pub fn record_call(&self, function: &str, duration: Duration) {
        let mut call_counts = self.call_counts.lock().unwrap();
        let mut call_times = self.call_times.lock().unwrap();
        
        *call_counts.entry(function.to_string()).or_insert(0) += 1;
        *call_times.entry(function.to_string()).or_insert(Duration::ZERO) += duration;
    }
    
    pub fn record_allocation(&self, size: usize) {
        self.memory_usage.fetch_add(size, Ordering::SeqCst);
        self.allocation_count.fetch_add(1, Ordering::SeqCst);
    }
    
    pub fn get_performance_report(&self) -> FfiPerformanceReport {
        let call_counts = self.call_counts.lock().unwrap();
        let call_times = self.call_times.lock().unwrap();
        
        FfiPerformanceReport {
            total_calls: call_counts.values().sum(),
            total_time: call_times.values().sum(),
            memory_usage: self.memory_usage.load(Ordering::SeqCst),
            allocation_count: self.allocation_count.load(Ordering::SeqCst),
            function_stats: call_counts.iter().map(|(name, count)| {
                let time = call_times.get(name).copied().unwrap_or(Duration::ZERO);
                FunctionStats {
                    name: name.clone(),
                    call_count: *count,
                    total_time: time,
                    average_time: time / *count as u32,
                }
            }).collect(),
        }
    }
}
```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_type_marshaling() {
        let marshaler = TypeMarshaler::new();
        
        // Test integer conversion
        let cursed_int = CursedValue::Integer(42);
        let c_int = marshaler.cursed_to_c(&cursed_int, &CType::CInt).unwrap();
        assert_eq!(c_int, CValue::CInt(42));
        
        // Test string conversion
        let cursed_string = CursedValue::String("hello".to_string());
        let c_string = marshaler.cursed_to_c(&cursed_string, &CType::CString).unwrap();
        assert_eq!(c_string.to_string(), "hello");
    }
    
    #[test]
    fn test_memory_management() {
        let mut memory_manager = FfiMemoryManager::new();
        
        // Test allocation
        let ptr = memory_manager.allocate_for_c(1024).unwrap();
        assert!(!ptr.is_null());
        
        // Test deallocation
        memory_manager.deallocate_c_memory(ptr).unwrap();
    }
    
    #[test]
    fn test_function_calls() {
        let bridge = CRuntimeBridge::new();
        
        // Test strlen function
        let args = vec![CursedValue::String("hello".to_string())];
        let result = bridge.call_function("strlen", &args).unwrap();
        assert_eq!(result, CursedValue::Integer(5));
    }
}
```

### Integration Tests

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[test]
    fn test_c_library_integration() {
        // Test integration with actual C library
        let bridge = CRuntimeBridge::new();
        
        // Test math functions
        let args = vec![CursedValue::Float(2.0)];
        let result = bridge.call_function("sqrt", &args).unwrap();
        assert!((result.as_float() - 1.414).abs() < 0.001);
    }
    
    #[test]
    fn test_callback_integration() {
        let callback_manager = CallbackManager::new();
        
        // Register callback
        let callback_id = callback_manager.register_callback(|args| {
            if let [CursedValue::Integer(a), CursedValue::Integer(b)] = args {
                Ok(CursedValue::Integer(a + b))
            } else {
                Err(FfiError::InvalidArgumentType {
                    expected: "integer".to_string(),
                    actual: "mixed".to_string(),
                })
            }
        }).unwrap();
        
        // Call callback
        let args = vec![CursedValue::Integer(3), CursedValue::Integer(4)];
        let result = callback_manager.call_callback(callback_id, &args).unwrap();
        assert_eq!(result, CursedValue::Integer(7));
    }
}
```

## Platform-Specific Considerations

### Windows

```rust
#[cfg(windows)]
mod windows_ffi {
    use super::*;
    use winapi::um::libloaderapi::{LoadLibraryW, GetProcAddress};
    
    impl FfiBridge {
        pub fn load_windows_library(&mut self, path: &str) -> Result<(), FfiError> {
            let wide_path: Vec<u16> = path.encode_utf16().chain(Some(0)).collect();
            let handle = unsafe { LoadLibraryW(wide_path.as_ptr()) };
            
            if handle.is_null() {
                return Err(FfiError::LibraryLoadError {
                    path: path.to_string(),
                    error: "LoadLibraryW failed".to_string(),
                });
            }
            
            self.loaded_libraries.insert(path.to_string(), handle);
            Ok(())
        }
    }
}
```

### Unix/Linux

```rust
#[cfg(unix)]
mod unix_ffi {
    use super::*;
    use std::ffi::CString;
    
    impl FfiBridge {
        pub fn load_unix_library(&mut self, path: &str) -> Result<(), FfiError> {
            let c_path = CString::new(path).map_err(|_| FfiError::InvalidPath)?;
            let handle = unsafe { libc::dlopen(c_path.as_ptr(), libc::RTLD_LAZY) };
            
            if handle.is_null() {
                let error = unsafe { CStr::from_ptr(libc::dlerror()) };
                return Err(FfiError::LibraryLoadError {
                    path: path.to_string(),
                    error: error.to_string_lossy().to_string(),
                });
            }
            
            self.loaded_libraries.insert(path.to_string(), handle);
            Ok(())
        }
    }
}
```

This comprehensive FFI specification provides the foundation for safe, efficient interoperability between CURSED and C code, with robust error handling, memory management, and performance optimization.
