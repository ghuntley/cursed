/// Memory Bridge for CURSED Runtime
/// 
/// This module provides the FFI bridge between C runtime functions
/// and the Rust memory management system with GC integration.

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::ptr::NonNull;
use std::sync::{Arc, Mutex, LazyLock};
use std::collections::HashSet;

use crate::error::CursedError;
use crate::memory::Tag;
use crate::runtime::gc::{GarbageCollector, get_global_gc};
use crate::runtime::heap_optimizer::{HeapOptimizer, HeapOptimizerConfig};
use crate::runtime::memory::{MemoryManager, MemoryConfig};

/// Track allocation source to prevent double-free
#[derive(Debug, PartialEq, Eq)]
enum AllocationSource {
    GarbageCollector,
    SystemMalloc,
}

/// Global allocation tracker to prevent double-free
static ALLOCATION_TRACKER: LazyLock<Mutex<HashSet<usize>>> = LazyLock::new(|| Mutex::new(HashSet::new()));
static GC_ALLOCATIONS: LazyLock<Mutex<HashSet<usize>>> = LazyLock::new(|| Mutex::new(HashSet::new()));

/// FFI functions called from C runtime bridge

/// Allocate memory through heap optimizer
#[no_mangle]
pub extern "C" fn rust_heap_allocate(size: usize, tag: i32) -> *mut c_void {
    let memory_tag = match tag {
        1 => Tag::Object,
        2 => Tag::Array,
        3 => Tag::String,
        4 => Tag::Function,
        5 => Tag::Channel,
        6 => Tag::Object, // Use Object as fallback for Goroutine
        _ => Tag::Object,
    };

    // Try to get global GC and allocate through it
    if let Some(gc) = get_global_gc() {
        match gc.allocate(size, memory_tag) {
            Ok(ptr) => {
                let ptr_addr = ptr.as_ptr() as usize;
                // Track this as a GC allocation
                if let Ok(mut gc_allocs) = GC_ALLOCATIONS.lock() {
                    gc_allocs.insert(ptr_addr);
                }
                if let Ok(mut tracker) = ALLOCATION_TRACKER.lock() {
                    tracker.insert(ptr_addr);
                }
                ptr.as_ptr() as *mut c_void
            }
            Err(_) => std::ptr::null_mut(),
        }
    } else {
        // Fallback to system allocation
        unsafe {
            let ptr = libc::malloc(size);
            if !ptr.is_null() {
                let ptr_addr = ptr as usize;
                // Track this as a system allocation (NOT in GC_ALLOCATIONS)
                if let Ok(mut tracker) = ALLOCATION_TRACKER.lock() {
                    tracker.insert(ptr_addr);
                }
            }
            ptr
        }
    }
}

/// Deallocate memory through GC system
#[no_mangle]
pub extern "C" fn rust_heap_deallocate(ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }

    let ptr_addr = ptr as usize;
    
    // Check if this pointer was allocated by us to prevent double-free
    let is_tracked = if let Ok(tracker) = ALLOCATION_TRACKER.lock() {
        tracker.contains(&ptr_addr)
    } else {
        false
    };
    
    if !is_tracked {
        // Not our allocation, don't free it
        return;
    }
    
    // Check if this was a GC allocation
    let is_gc_allocation = if let Ok(gc_allocs) = GC_ALLOCATIONS.lock() {
        gc_allocs.contains(&ptr_addr)
    } else {
        false
    };
    
    // Remove from tracking sets first to prevent double-free
    if let Ok(mut tracker) = ALLOCATION_TRACKER.lock() {
        tracker.remove(&ptr_addr);
    }
    
    if is_gc_allocation {
        if let Ok(mut gc_allocs) = GC_ALLOCATIONS.lock() {
            gc_allocs.remove(&ptr_addr);
        }
        // Use GC deallocate for GC allocations
        if let Some(gc) = get_global_gc() {
            let _ = gc.deallocate(ptr as *mut u8);
        }
    } else {
        // Use system free for system allocations
        unsafe {
            libc::free(ptr);
        }
    }
}

/// Reallocate memory through GC system
#[no_mangle]
pub extern "C" fn rust_heap_reallocate(ptr: *mut c_void, new_size: usize) -> *mut c_void {
    if ptr.is_null() {
        return rust_heap_allocate(new_size, 1); // Default to object tag
    }

    if new_size == 0 {
        rust_heap_deallocate(ptr);
        return std::ptr::null_mut();
    }

    let ptr_addr = ptr as usize;
    
    // Check if this is our allocation
    let is_tracked = if let Ok(tracker) = ALLOCATION_TRACKER.lock() {
        tracker.contains(&ptr_addr)
    } else {
        false
    };
    
    if !is_tracked {
        // Not our allocation, can't safely realloc
        return std::ptr::null_mut();
    }

    // Allocate new memory
    let new_ptr = rust_heap_allocate(new_size, 1);
    if !new_ptr.is_null() {
        // Copy existing data (conservative size estimation)
        unsafe {
            std::ptr::copy_nonoverlapping(ptr as *const u8, new_ptr as *mut u8, new_size.min(1024));
        }
        // Free old memory using our tracked deallocate
        rust_heap_deallocate(ptr);
    }

    new_ptr
}

/// Trigger garbage collection
#[no_mangle]
pub extern "C" fn rust_gc_collect() -> i32 {
    if let Some(gc) = get_global_gc() {
        match gc.force_collection() {
            Ok(_) => 1024, // Return placeholder bytes freed
            Err(_) => 0,
        }
    } else {
        0
    }
}

/// Get GC statistics as C string
#[no_mangle]
pub extern "C" fn rust_gc_stats() -> *mut c_char {
    if let Some(gc) = get_global_gc() {
        match gc.get_stats() {
            Ok(stats) => {
                let stats_str = format!(
                    "GC Collections: {}, Objects Marked: {}, Objects Swept: {}, Heap Size: {}",
                    stats.total_collections,
                    stats.objects_marked,
                    stats.objects_swept,
                    stats.current_heap_size
                );
                
                match CString::new(stats_str) {
                    Ok(c_str) => c_str.into_raw(),
                    Err(_) => std::ptr::null_mut(),
                }
            }
            Err(_) => std::ptr::null_mut(),
        }
    } else {
        match CString::new("GC not initialized") {
            Ok(c_str) => c_str.into_raw(),
            Err(_) => std::ptr::null_mut(),
        }
    }
}

/// Get memory statistics as C string
#[no_mangle]
pub extern "C" fn rust_memory_stats() -> *mut c_char {
    // Placeholder implementation - would gather real stats
    let stats_str = "Memory Stats: allocations tracked, GC active";
    
    match CString::new(stats_str) {
        Ok(c_str) => c_str.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Track allocation for debugging
#[no_mangle]
pub extern "C" fn rust_track_allocation(ptr: *mut c_void, size: usize, tag: *const c_char) -> bool {
    if ptr.is_null() || tag.is_null() {
        return false;
    }

    // In real implementation, this would add to allocation tracking database
    // For now, just return true as tracking is handled elsewhere
    true
}

/// Get memory pressure (0.0-1.0)
#[no_mangle]
pub extern "C" fn rust_memory_pressure() -> f64 {
    if let Some(gc) = get_global_gc() {
        let heap_size = gc.get_heap_size();
        let threshold = 64 * 1024 * 1024; // 64MB threshold
        (heap_size as f64) / (threshold as f64)
    } else {
        0.0
    }
}

/// Get current stack size
#[no_mangle]
pub extern "C" fn rust_stack_size() -> i32 {
    // Placeholder - would get from stack manager
    8192 // 8KB default
}

/// Check for stack overflow
#[no_mangle]
pub extern "C" fn rust_check_stack_overflow() -> bool {
    // Try goroutine-based stack overflow detection first
    if let Some(scheduler) = crate::runtime::goroutine::get_global_scheduler() {
        if let Some(goroutine_id) = scheduler.get_current_goroutine_id() {
            // Try to get stack overflow status from scheduler if it tracks this
            // For now, we'll use the platform-specific fallback
        }
    }
    
    // Use platform-specific stack detection
    detect_platform_stack_overflow()
}

/// Platform-specific stack overflow detection as fallback
fn detect_platform_stack_overflow() -> bool {
    // Use platform-specific methods to detect stack overflow
    #[cfg(unix)]
    {
        detect_unix_stack_overflow()
    }
    
    #[cfg(windows)]
    {
        detect_windows_stack_overflow()
    }
    
    #[cfg(target_arch = "wasm32")]
    {
        detect_wasm_stack_overflow()
    }
    
    #[cfg(not(any(unix, windows, target_arch = "wasm32")))]
    {
        // Generic fallback - check current stack pointer
        detect_generic_stack_overflow()
    }
}

#[cfg(unix)]
fn detect_unix_stack_overflow() -> bool {
    unsafe {
        let mut stack_ptr: *mut u8 = std::ptr::null_mut();
        
        // Get current stack pointer for supported architectures
        #[cfg(target_arch = "x86_64")]
        std::arch::asm!("mov {}, rsp", out(reg) stack_ptr);
        
        #[cfg(target_arch = "aarch64")]
        std::arch::asm!("mov {}, sp", out(reg) stack_ptr);
        
        // If we couldn't get the stack pointer on an unsupported architecture, return false
        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        {
            return false;
        }
        
        // Get stack limits using getrlimit
        let mut rlimit = libc::rlimit { rlim_cur: 0, rlim_max: 0 };
        if libc::getrlimit(libc::RLIMIT_STACK, &mut rlimit) == 0 {
            // Get the stack base (approximate)
            let mut stack_base = std::ptr::null_mut();
            let mut stack_size = 0;
            
            // Use pthread_attr_getstack to get actual stack info
            let mut attr: libc::pthread_attr_t = std::mem::zeroed();
            if libc::pthread_getattr_np(libc::pthread_self(), &mut attr) == 0 {
                if libc::pthread_attr_getstack(&attr, &mut stack_base as *mut *mut _ as *mut _, &mut stack_size) == 0 {
                    // Check if current stack pointer is near the limit
                    let stack_end = (stack_base as usize).wrapping_sub(stack_size);
                    let current_pos = stack_ptr as usize;
                    let threshold = 64 * 1024; // 64KB threshold
                    
                    libc::pthread_attr_destroy(&mut attr);
                    return current_pos <= stack_end + threshold;
                }
                libc::pthread_attr_destroy(&mut attr);
            }
        }
        
        // Fallback: simple heuristic
        false
    }
}

#[cfg(windows)]
fn detect_windows_stack_overflow() -> bool {
    // Windows-specific stack overflow detection
    // This would use Windows APIs like GetCurrentThreadStackLimits
    // For now, return false as a placeholder
    false
}

#[cfg(target_arch = "wasm32")]
fn detect_wasm_stack_overflow() -> bool {
    // WASM-specific stack overflow detection
    // Check if we're near the end of the linear memory stack
    false
}

#[cfg(not(any(unix, windows, target_arch = "wasm32")))]
fn detect_generic_stack_overflow() -> bool {
    // Generic fallback that doesn't rely on platform-specific APIs
    false
}

/// Create memory pool
#[no_mangle]
pub extern "C" fn rust_create_memory_pool(block_size: i32, block_count: i32) -> *mut c_void {
    if block_size <= 0 || block_count <= 0 {
        return std::ptr::null_mut();
    }

    // Create a simple pool ID (would be more sophisticated in real implementation)
    let pool_id = Box::new((block_size, block_count));
    Box::into_raw(pool_id) as *mut c_void
}

/// Allocate from memory pool
#[no_mangle]
pub extern "C" fn rust_pool_alloc(pool_id: *mut c_void, size: i32) -> *mut c_void {
    if pool_id.is_null() || size <= 0 {
        return std::ptr::null_mut();
    }

    // For now, fallback to regular allocation
    rust_heap_allocate(size as usize, 1)
}

/// Free to memory pool
#[no_mangle]
pub extern "C" fn rust_pool_free(pool_id: *mut c_void, ptr: *mut c_void) -> bool {
    if pool_id.is_null() || ptr.is_null() {
        return false;
    }

    // For now, fallback to regular deallocation
    rust_heap_deallocate(ptr);
    true
}

/// Zero memory
#[no_mangle]
pub extern "C" fn rust_zero_memory(ptr: *mut c_void, size: i32) -> bool {
    if ptr.is_null() || size <= 0 {
        return false;
    }

    unsafe {
        std::ptr::write_bytes(ptr as *mut u8, 0, size as usize);
    }
    true
}

/// Copy memory
#[no_mangle]
pub extern "C" fn rust_copy_memory(dest: *mut c_void, src: *mut c_void, size: i32) -> bool {
    if dest.is_null() || src.is_null() || size <= 0 {
        return false;
    }

    unsafe {
        std::ptr::copy_nonoverlapping(src as *const u8, dest as *mut u8, size as usize);
    }
    true
}

/// Compare memory
#[no_mangle]
pub extern "C" fn rust_compare_memory(ptr1: *mut c_void, ptr2: *mut c_void, size: i32) -> i32 {
    if ptr1.is_null() || ptr2.is_null() || size <= 0 {
        return -1;
    }

    unsafe {
        let slice1 = std::slice::from_raw_parts(ptr1 as *const u8, size as usize);
        let slice2 = std::slice::from_raw_parts(ptr2 as *const u8, size as usize);
        
        match slice1.cmp(slice2) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        }
    }
}

/// Align size to boundary
#[no_mangle]
pub extern "C" fn rust_align_size(size: i32, alignment: i32) -> i32 {
    if size <= 0 || alignment <= 0 {
        return size;
    }

    let size = size as usize;
    let alignment = alignment as usize;
    
    ((size + alignment - 1) & !(alignment - 1)) as i32
}

/// Check if pointer is aligned
#[no_mangle]
pub extern "C" fn rust_is_aligned(ptr: *mut c_void, alignment: i32) -> bool {
    if ptr.is_null() || alignment <= 0 {
        return false;
    }

    (ptr as usize) % (alignment as usize) == 0
}

/// Set memory limit
#[no_mangle]
pub extern "C" fn rust_set_memory_limit(limit: usize) -> bool {
    // Placeholder - would set in memory manager
    true
}

/// Get current memory usage
#[no_mangle]
pub extern "C" fn rust_get_memory_usage() -> usize {
    if let Some(gc) = get_global_gc() {
        gc.get_heap_size()
    } else {
        0
    }
}

/// Compact memory
#[no_mangle]
pub extern "C" fn rust_memory_compact() -> i32 {
    if let Some(gc) = get_global_gc() {
        match gc.force_collection() {
            Ok(_) => 512, // Return placeholder bytes compacted
            Err(_) => 0,
        }
    } else {
        0
    }
}

/// Reset memory statistics
#[no_mangle]
pub extern "C" fn rust_reset_memory_stats() -> bool {
    // Placeholder - would reset stats in memory manager
    true
}

/// Initialize the memory system
pub fn initialize_memory_system() -> Result<(), CursedError> {
    // Initialize global GC if not already done
    if get_global_gc().is_none() {
        use crate::runtime::gc::{initialize_gc, GcConfig};
        use crate::runtime::stack::{RuntimeStack};
        
        let config = GcConfig::default();
        let stack_manager = Arc::new(RuntimeStack::new());
        initialize_gc(config, stack_manager)?;
    }
    
    Ok(())
}

/// Shutdown the memory system
pub fn shutdown_memory_system() -> Result<(), CursedError> {
    use crate::runtime::gc::shutdown_gc;
    shutdown_gc()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_allocation() {
        let _ = initialize_memory_system();
        
        let ptr = rust_heap_allocate(1024, 1);
        assert!(!ptr.is_null());
        
        rust_heap_deallocate(ptr);
    }

    #[test]
    fn test_memory_operations() {
        let ptr = rust_heap_allocate(1024, 1);
        assert!(!ptr.is_null());
        
        assert!(rust_zero_memory(ptr, 1024));
        
        let ptr2 = rust_heap_allocate(1024, 1);
        assert!(!ptr2.is_null());
        
        assert!(rust_copy_memory(ptr2, ptr, 1024));
        assert_eq!(rust_compare_memory(ptr, ptr2, 1024), 0);
        
        rust_heap_deallocate(ptr);
        rust_heap_deallocate(ptr2);
    }

    #[test]
    fn test_alignment() {
        assert_eq!(rust_align_size(100, 8), 104);
        assert_eq!(rust_align_size(128, 8), 128);
        
        let ptr = rust_heap_allocate(1024, 1);
        assert!(!ptr.is_null());
        
        // Most allocators provide at least 8-byte alignment
        assert!(rust_is_aligned(ptr, 8));
        
        rust_heap_deallocate(ptr);
    }
}
