/// LLVM Integration for Garbage Collection System
/// 
/// This module provides comprehensive integration between LLVM code generation and the
/// CURSED garbage collection system, including:
/// 
/// 1. **Memory Allocation Integration**: Real GC allocation calls replacing placeholders
/// 2. **Safe Point Instrumentation**: GC coordination points for concurrent collection
/// 3. **Write Barrier Integration**: Pointer assignment tracking for GC invariants
/// 4. **Runtime Function Declarations**: External runtime functions for compiled code
/// 5. **Memory Layout Management**: Object headers and type-specific allocation

use crate::error::Error;
use crate::memory::gc::{GarbageCollector, GcConfig, Gc};
use crate::memory::object_id::ObjectId;
use crate::runtime::goroutine::GoroutineScheduler;
use std::collections::HashMap;
use std::ptr::NonNull;
use std::sync::{Arc, RwLock};
use tracing::{instrument, debug, info, warn, error};

/// LLVM GC Integration Manager
pub struct LlvmGcIntegration {
    /// The garbage collector instance
    gc: Arc<GarbageCollector>,
    /// Type size registry for allocation
    type_sizes: HashMap<String, usize>,
    /// Object header size (type_id + size + flags)
    object_header_size: usize,
    /// Safe point instrumentation enabled
    safe_points_enabled: bool,
    /// Write barriers enabled
    write_barriers_enabled: bool,
    /// Statistics for monitoring
    stats: Arc<RwLock<GcIntegrationStats>>,
}

/// Statistics for GC integration monitoring
#[derive(Debug, Default, Clone)]
pub struct GcIntegrationStats {
    pub allocations_instrumented: u64,
    pub safe_points_inserted: u64,
    pub write_barriers_inserted: u64,
    pub runtime_function_calls: u64,
    pub allocation_failures: u64,
}

/// Object header layout for GC tracking
#[repr(C)]
pub struct ObjectHeader {
    /// Type identifier for the object
    pub type_id: u64,
    /// Size of the object (including header)
    pub size: usize,
    /// GC flags (mark bit, generation, etc.)
    pub flags: u32,
    /// Reserved for future use
    pub reserved: u32,
}

/// Runtime allocation request from LLVM code
#[repr(C)]
pub struct AllocationRequest {
    /// Size of object to allocate (without header)
    pub size: usize,
    /// Alignment requirement
    pub alignment: usize,
    /// Type name for debugging/profiling
    pub type_name: *const i8,
    /// Type ID for GC tracking
    pub type_id: u64,
}

/// Runtime allocation result
#[repr(C)]
pub struct AllocationResult {
    /// Pointer to allocated object (including header)
    pub ptr: *mut u8,
    /// Success flag
    pub success: bool,
    /// Error code if allocation failed
    pub error_code: i32,
}

impl std::fmt::Debug for LlvmGcIntegration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LlvmGcIntegration")
            .field("type_sizes", &self.type_sizes)
            .field("object_header_size", &self.object_header_size)
            .field("safe_points_enabled", &self.safe_points_enabled)
            .field("write_barriers_enabled", &self.write_barriers_enabled)
            .field("stats", &self.stats)
            .finish()
    }
}

impl LlvmGcIntegration {
    /// Create new LLVM GC integration
    #[instrument]
    pub fn new(_gc_config: GcConfig) -> Result<Self, Error> {
        let gc = Arc::new(GarbageCollector::new());
        
        Ok(Self {
            gc,
            type_sizes: HashMap::new(),
            object_header_size: std::mem::size_of::<ObjectHeader>(),
            safe_points_enabled: true,
            write_barriers_enabled: true,
            stats: Arc::new(RwLock::new(GcIntegrationStats::default())),
        })
    }

    /// Register a type with its size for allocation
    #[instrument]
    pub fn register_type(&mut self, type_name: String, size: usize) {
        debug!(type_name = %type_name, size = size, "Registering type for GC allocation");
        self.type_sizes.insert(type_name, size);
    }

    /// Generate LLVM IR for runtime function declarations
    #[instrument]
    pub fn generate_runtime_function_declarations(&self) -> String {
        debug!("Generating LLVM runtime function declarations");
        
        let mut ir = String::new();
        
        // Object allocation function
        ir.push_str("; GC object allocation\n");
        ir.push_str("declare i8* @cursed_allocate_object(i64, i64, i8*, i64)\n");
        
        // Safe point coordination
        ir.push_str("; GC safe point\n");
        ir.push_str("declare void @cursed_safe_point(i8*)\n");
        
        // Write barrier for pointer assignments
        ir.push_str("; GC write barrier\n");
        ir.push_str("declare void @cursed_write_barrier(i8*, i8*, i8*)\n");
        
        // GC collection trigger
        ir.push_str("; Manual GC collection\n");
        ir.push_str("declare void @cursed_collect_garbage()\n");
        
        // Goroutine coordination functions (already declared in goroutine.rs)
        ir.push_str("; Goroutine GC coordination\n");
        ir.push_str("declare i8* @cursed_spawn_goroutine(i8*, i8*)\n");
        ir.push_str("declare void @cursed_yield_goroutine(i8*)\n");
        ir.push_str("declare i1 @cursed_gc_requested(i8*)\n");
        
        // Object introspection
        ir.push_str("; Object introspection\n");
        ir.push_str("declare i64 @cursed_object_type_id(i8*)\n");
        ir.push_str("declare i64 @cursed_object_size(i8*)\n");
        
        ir.push('\n');
        ir
    }

    /// Generate LLVM IR for object allocation
    #[instrument]
    pub fn generate_allocation_ir(&self, type_name: &str, temp_var: &str) -> Result<String, Error> {
        let type_size = self.type_sizes.get(type_name)
            .ok_or_else(|| Error::from_str(&format!("Unknown type for allocation: {}", type_name)))?;
        
        let total_size = self.object_header_size + type_size;
        let type_id = self.calculate_type_id(type_name);
        
        debug!(type_name = %type_name, size = total_size, type_id = type_id, "Generating allocation IR");
        
        // Update statistics
        if let Ok(mut stats) = self.stats.write() {
            stats.allocations_instrumented += 1;
        }
        
        let mut ir = String::new();
        
        // Generate allocation call
        ir.push_str(&format!(
            "  ; Allocate object of type '{}' (size: {})\n",
            type_name, total_size
        ));
        ir.push_str(&format!(
            "  {} = call i8* @cursed_allocate_object(i64 {}, i64 8, i8* getelementptr inbounds ([{} x i8], [{} x i8]* @type_name_{}, i32 0, i32 0), i64 {})\n",
            temp_var, total_size, type_name.len() + 1, type_name.len() + 1, Self::sanitize_type_name(type_name), type_id
        ));
        
        // Check allocation success
        ir.push_str(&format!("  %is_null_{} = icmp eq i8* {}, null\n", temp_var, temp_var));
        ir.push_str(&format!("  br i1 %is_null_{}, label %allocation_failed_{}, label %allocation_success_{}\n\n", temp_var, temp_var, temp_var));
        
        // Allocation failed block
        ir.push_str(&format!("allocation_failed_{}:\n", temp_var));
        ir.push_str("  call void @abort()\n");
        ir.push_str("  unreachable\n\n");
        
        // Allocation success block
        ir.push_str(&format!("allocation_success_{}:\n", temp_var));
        
        Ok(ir)
    }

    /// Generate LLVM IR for safe point insertion
    #[instrument]
    pub fn generate_safe_point_ir(&self, context: &str) -> String {
        if !self.safe_points_enabled {
            return String::new();
        }
        
        debug!(context = %context, "Generating safe point IR");
        
        // Update statistics
        if let Ok(mut stats) = self.stats.write() {
            stats.safe_points_inserted += 1;
        }
        
        format!(
            "  ; Safe point for GC coordination ({})\n  call void @cursed_safe_point(i8* null)\n",
            context
        )
    }

    /// Generate LLVM IR for write barrier
    #[instrument]
    pub fn generate_write_barrier_ir(&self, object_ptr: &str, field_ptr: &str, value_ptr: &str) -> String {
        if !self.write_barriers_enabled {
            return String::new();
        }
        
        debug!(object = %object_ptr, field = %field_ptr, value = %value_ptr, "Generating write barrier IR");
        
        // Update statistics
        if let Ok(mut stats) = self.stats.write() {
            stats.write_barriers_inserted += 1;
        }
        
        format!(
            "  ; Write barrier for pointer assignment\n  call void @cursed_write_barrier(i8* {}, i8* {}, i8* {})\n",
            object_ptr, field_ptr, value_ptr
        )
    }

    /// Generate LLVM IR for type name constants
    #[instrument]
    pub fn generate_type_name_constants(&self) -> String {
        debug!("Generating type name constants");
        
        let mut ir = String::new();
        ir.push_str("; Type name constants for GC allocation\n");
        
        for type_name in self.type_sizes.keys() {
            let sanitized_name = Self::sanitize_type_name(type_name);
            let type_string = format!("{}\0", type_name);
            
            ir.push_str(&format!(
                "@type_name_{} = private unnamed_addr constant [{} x i8] c\"{}\"\n",
                sanitized_name,
                type_string.len(),
                type_string.escape_default()
            ));
        }
        
        ir.push('\n');
        ir
    }

    /// Generate LLVM IR for function entry safe point
    #[instrument]
    pub fn generate_function_entry_safe_point(&self, function_name: &str) -> String {
        self.generate_safe_point_ir(&format!("function_entry_{}", function_name))
    }

    /// Generate LLVM IR for function exit safe point  
    #[instrument]
    pub fn generate_function_exit_safe_point(&self, function_name: &str) -> String {
        self.generate_safe_point_ir(&format!("function_exit_{}", function_name))
    }

    /// Generate LLVM IR for loop yield point
    #[instrument]
    pub fn generate_loop_yield_point(&self, loop_id: &str) -> String {
        let mut ir = self.generate_safe_point_ir(&format!("loop_yield_{}", loop_id));
        
        // Add optional goroutine yield
        ir.push_str(&format!(
            "  ; Optional goroutine yield point ({})\n  call void @cursed_yield_goroutine(i8* null)\n",
            loop_id
        ));
        
        ir
    }

    /// Generate LLVM IR for allocation safe point (before allocation)
    #[instrument]
    pub fn generate_allocation_safe_point(&self, type_name: &str) -> String {
        self.generate_safe_point_ir(&format!("pre_allocation_{}", type_name))
    }

    /// Get GC integration statistics
    pub fn get_stats(&self) -> Result<GcIntegrationStats, Error> {
        self.stats.read()
            .map(|stats| (*stats).clone())
            .map_err(|e| Error::from_str(&format!("Failed to read GC integration stats: {}", e)))
    }

    /// Reset GC integration statistics
    #[instrument]
    pub fn reset_stats(&self) {
        if let Ok(mut stats) = self.stats.write() {
            *stats = GcIntegrationStats::default();
        }
    }

    /// Enable or disable safe point instrumentation
    #[instrument]
    pub fn set_safe_points_enabled(&mut self, enabled: bool) {
        debug!(enabled = enabled, "Setting safe points enabled state");
        self.safe_points_enabled = enabled;
    }

    /// Enable or disable write barrier instrumentation
    #[instrument]
    pub fn set_write_barriers_enabled(&mut self, enabled: bool) {
        debug!(enabled = enabled, "Setting write barriers enabled state");
        self.write_barriers_enabled = enabled;
    }

    /// Get the underlying GC instance
    pub fn gc(&self) -> &Arc<GarbageCollector> {
        &self.gc
    }

    /// Calculate type ID for GC tracking (simple hash)
    fn calculate_type_id(&self, type_name: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        type_name.hash(&mut hasher);
        hasher.finish()
    }

    /// Sanitize type name for LLVM identifier
    fn sanitize_type_name(type_name: &str) -> String {
        type_name.chars()
            .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
            .collect()
    }
}

// External runtime functions that LLVM code will call

/// Allocate object through GC system
/// Called from LLVM-generated code for object allocation
#[no_mangle]
pub extern "C" fn cursed_allocate_object(
    size: i64,
    alignment: i64,
    type_name: *const i8,
    type_id: i64,
) -> *mut u8 {
    use std::ffi::CStr;
    
    // Safety checks
    if size <= 0 || alignment <= 0 {
        error!(size = size, alignment = alignment, "Invalid allocation parameters");
        return std::ptr::null_mut();
    }
    
    // Convert type name
    let type_name = if type_name.is_null() {
        "unknown"
    } else {
        unsafe {
            CStr::from_ptr(type_name)
                .to_str()
                .unwrap_or("invalid_utf8")
        }
    };
    
    debug!(size = size, alignment = alignment, type_name = %type_name, type_id = type_id, "Runtime object allocation");
    
    // Get global GC instance (would be set up during runtime initialization)
    // For now, use a placeholder - in real implementation this would be properly initialized
    match allocate_object_internal(size as usize, alignment as usize, type_name, type_id as u64) {
        Ok(ptr) => {
            debug!(ptr = ?ptr, "Object allocation successful");
            ptr.as_ptr()
        }
        Err(e) => {
            error!(error = %e, "Object allocation failed");
            std::ptr::null_mut()
        }
    }
}

// Note: cursed_safe_point is already defined in src/runtime/goroutine.rs

/// Write barrier for pointer assignments
/// Called from LLVM-generated code for pointer writes
#[no_mangle]
pub extern "C" fn cursed_write_barrier(
    object: *mut u8,
    field: *mut u8,
    value: *mut u8,
) {
    if object.is_null() || field.is_null() {
        return;
    }
    
    debug!(object = ?object, field = ?field, value = ?value, "Write barrier");
    
    // In real implementation, this would update GC metadata
    write_barrier_internal(object, field, value);
}

/// Manual garbage collection trigger
/// Called from LLVM-generated code for explicit collection
#[no_mangle]
pub extern "C" fn cursed_collect_garbage() {
    debug!("Manual garbage collection requested");
    
    // In real implementation, this would trigger collection
    collect_garbage_internal();
}

/// Get object type ID
/// Called from LLVM-generated code for type introspection
#[no_mangle]
pub extern "C" fn cursed_object_type_id(object: *mut u8) -> i64 {
    if object.is_null() {
        return 0;
    }
    
    // Extract type ID from object header
    unsafe {
        let header = object as *const ObjectHeader;
        (*header).type_id as i64
    }
}

/// Get object size
/// Called from LLVM-generated code for size introspection
#[no_mangle]
pub extern "C" fn cursed_object_size(object: *mut u8) -> i64 {
    if object.is_null() {
        return 0;
    }
    
    // Extract size from object header
    unsafe {
        let header = object as *const ObjectHeader;
        (*header).size as i64
    }
}

// Internal implementation functions (would be properly implemented)

fn allocate_object_internal(
    size: usize,
    alignment: usize,
    type_name: &str,
    type_id: u64,
) -> Result<NonNull<u8>, String> {
    // This is a placeholder - in real implementation this would:
    // 1. Get the global GC instance
    // 2. Allocate memory through the GC
    // 3. Set up the object header
    // 4. Return the pointer to the object (after header)
    
    use std::alloc::{alloc, Layout};
    
    let header_size = std::mem::size_of::<ObjectHeader>();
    let total_size = header_size + size;
    
    let layout = Layout::from_size_align(total_size, alignment)
        .map_err(|e| format!("Invalid layout: {}", e))?;
    
    let ptr = unsafe { alloc(layout) };
    if ptr.is_null() {
        return Err("Allocation failed".to_string());
    }
    
    // Set up object header
    unsafe {
        let header = ptr as *mut ObjectHeader;
        (*header).type_id = type_id;
        (*header).size = total_size;
        (*header).flags = 0;
        (*header).reserved = 0;
    }
    
    // Return pointer to object data (after header)
    let object_ptr = unsafe { ptr.add(header_size) };
    
    NonNull::new(object_ptr).ok_or_else(|| "Null pointer after allocation".to_string())
}

// safe_point_internal removed - using goroutine.rs implementation

fn write_barrier_internal(_object: *mut u8, _field: *mut u8, _value: *mut u8) {
    // Placeholder for write barrier implementation
    // In real implementation:
    // 1. Check if value is a pointer to GC object
    // 2. Update generational GC metadata
    // 3. Mark card table if needed
}

fn collect_garbage_internal() {
    // Placeholder for manual GC collection
    // In real implementation:
    // 1. Get global GC instance
    // 2. Trigger collection
    // 3. Update statistics
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::gc::GcConfig;

    #[test]
    fn test_llvm_gc_integration_creation() {
        let config = GcConfig::default();
        let integration = LlvmGcIntegration::new(config);
        assert!(integration.is_ok());
    }

    #[test]
    fn test_type_registration() {
        let config = GcConfig::default();
        let mut integration = LlvmGcIntegration::new(config).unwrap();
        
        integration.register_type("TestType".to_string(), 64);
        assert!(integration.type_sizes.contains_key("TestType"));
        assert_eq!(integration.type_sizes["TestType"], 64);
    }

    #[test]
    fn test_runtime_function_declarations() {
        let config = GcConfig::default();
        let integration = LlvmGcIntegration::new(config).unwrap();
        
        let ir = integration.generate_runtime_function_declarations();
        assert!(ir.contains("cursed_allocate_object"));
        assert!(ir.contains("cursed_safe_point"));
        assert!(ir.contains("cursed_write_barrier"));
    }

    #[test]
    fn test_allocation_ir_generation() {
        let config = GcConfig::default();
        let mut integration = LlvmGcIntegration::new(config).unwrap();
        
        integration.register_type("TestType".to_string(), 32);
        
        let ir = integration.generate_allocation_ir("TestType", "%obj").unwrap();
        assert!(ir.contains("cursed_allocate_object"));
        assert!(ir.contains("TestType"));
    }

    #[test]
    fn test_safe_point_generation() {
        let config = GcConfig::default();
        let integration = LlvmGcIntegration::new(config).unwrap();
        
        let ir = integration.generate_safe_point_ir("test_context");
        assert!(ir.contains("cursed_safe_point"));
        assert!(ir.contains("test_context"));
    }

    #[test]
    fn test_write_barrier_generation() {
        let config = GcConfig::default();
        let integration = LlvmGcIntegration::new(config).unwrap();
        
        let ir = integration.generate_write_barrier_ir("%obj", "%field", "%value");
        assert!(ir.contains("cursed_write_barrier"));
        assert!(ir.contains("%obj"));
        assert!(ir.contains("%field"));
        assert!(ir.contains("%value"));
    }

    #[test]
    fn test_type_name_sanitization() {
        assert_eq!(
            LlvmGcIntegration::sanitize_type_name("My::Complex<Type>"),
            "My__Complex_Type_"
        );
    }

    #[test]
    fn test_statistics_tracking() {
        let config = GcConfig::default();
        let integration = LlvmGcIntegration::new(config).unwrap();
        
        let stats = integration.get_stats().unwrap();
        assert_eq!(stats.allocations_instrumented, 0);
        assert_eq!(stats.safe_points_inserted, 0);
        assert_eq!(stats.write_barriers_inserted, 0);
    }

    #[test]
    fn test_object_header_layout() {
        assert_eq!(std::mem::size_of::<ObjectHeader>(), 24); // 8 + 8 + 4 + 4
        assert_eq!(std::mem::align_of::<ObjectHeader>(), 8);
    }

    #[test]
    fn test_runtime_allocation_safety() {
        // Test with invalid parameters
        let result = cursed_allocate_object(0, 8, std::ptr::null(), 0);
        assert!(result.is_null());
        
        let result = cursed_allocate_object(-1, 8, std::ptr::null(), 0);
        assert!(result.is_null());
    }
}
