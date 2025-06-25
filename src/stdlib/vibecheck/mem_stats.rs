/// Memory Statistics Implementation for vibecheck
/// 
/// Provides detailed memory usage statistics and allocation tracking

use crate::error::CursedError;
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicU64, Ordering};

/// Detailed memory statistics matching the vibecheck specification
#[derive(Debug, Clone, Copy)]
pub struct MemStats {
    /// Bytes allocated and not yet freed
    pub alloc: u64,
    /// Total bytes allocated (even if freed)
    pub total_alloc: u64,
    /// Total memory obtained from system
    pub sys: u64,
    /// Total number of allocations
    pub mallocs: u64,
    /// Total number of frees
    pub frees: u64,
    /// Bytes allocated and not yet freed (same as Alloc)
    pub heap_alloc: u64,
    /// Bytes obtained from system for heap
    pub heap_sys: u64,
    /// Bytes in idle heap spans
    pub heap_idle: u64,
    /// Bytes in non-idle heap spans  
    pub heap_inuse: u64,
    /// Bytes used by stack allocator
    pub stack_inuse: u64,
    /// Bytes obtained from system for stack allocator
    pub stack_sys: u64,
    /// Bytes used for GC metadata
    pub gc_sys: u64,
    /// Target heap size for next GC
    pub next_gc: u64,
    /// Time of last GC in nanoseconds since epoch
    pub last_gc: u64,
    /// Total GC pause time in nanoseconds
    pub pause_total_ns: u64,
    /// Number of completed GC cycles
    pub num_gc: u32,
    /// Fraction of CPU time used by GC
    pub gc_cpu_fraction: f64,
}

impl Default for MemStats {
    fn default() -> Self {
        Self {
            alloc: 0,
            total_alloc: 0,
            sys: 0,
            mallocs: 0,
            frees: 0,
            heap_alloc: 0,
            heap_sys: 0,
            heap_idle: 0,
            heap_inuse: 0,
            stack_inuse: 0,
            stack_sys: 0,
            gc_sys: 0,
            next_gc: 0,
            last_gc: 0,
            pause_total_ns: 0,
            num_gc: 0,
            gc_cpu_fraction: 0.0,
        }
    }
}

impl MemStats {
    /// Create a new MemStats with all fields initialized to zero
    pub fn new() -> Self {
        Self::default()
    }
}

/// Global memory tracking statistics
static GLOBAL_ALLOC_BYTES: AtomicU64 = AtomicU64::new(0);
static GLOBAL_TOTAL_ALLOC: AtomicU64 = AtomicU64::new(0);
static GLOBAL_MALLOCS: AtomicU64 = AtomicU64::new(0);
static GLOBAL_FREES: AtomicU64 = AtomicU64::new(0);
static GLOBAL_SYSTEM_BYTES: AtomicU64 = AtomicU64::new(0);

/// Read current memory statistics into provided MemStats struct
pub fn read_mem_stats(stats: &mut MemStats) -> crate::error::Result<()> {
    // Get basic allocation statistics
    let alloc_stats = super::get_alloc_stats()?;
    let gc_state = super::get_gc_state()?;
    
    // Read atomic statistics
    let current_alloc = GLOBAL_ALLOC_BYTES.load(Ordering::SeqCst);
    let total_alloc = GLOBAL_TOTAL_ALLOC.load(Ordering::SeqCst);
    let mallocs = GLOBAL_MALLOCS.load(Ordering::SeqCst);
    let frees = GLOBAL_FREES.load(Ordering::SeqCst);
    let sys_bytes = GLOBAL_SYSTEM_BYTES.load(Ordering::SeqCst);
    
    // Get system memory information
    let (heap_sys, stack_sys, gc_sys) = get_system_memory_info()?;
    
    // Calculate derived values
    let heap_inuse = current_alloc;
    let heap_idle = heap_sys.saturating_sub(heap_inuse);
    let stack_inuse = get_stack_usage()?;
    
    // Estimate next GC threshold (simplified heuristic)
    let next_gc = if gc_state.target_percent > 0 {
        current_alloc + (current_alloc * gc_state.target_percent as u64) / 100
    } else {
        current_alloc * 2 // Default to 2x current allocation
    };
    
    // Populate the stats structure
    stats.alloc = current_alloc;
    stats.total_alloc = total_alloc;
    stats.sys = sys_bytes;
    stats.mallocs = mallocs;
    stats.frees = frees;
    stats.heap_alloc = current_alloc; // Same as alloc
    stats.heap_sys = heap_sys;
    stats.heap_idle = heap_idle;
    stats.heap_inuse = heap_inuse;
    stats.stack_inuse = stack_inuse;
    stats.stack_sys = stack_sys;
    stats.gc_sys = gc_sys;
    stats.next_gc = next_gc;
    stats.last_gc = gc_state.last_gc_time;
    stats.pause_total_ns = gc_state.total_pause_time;
    stats.num_gc = gc_state.gc_count;
    stats.gc_cpu_fraction = gc_state.cpu_fraction;
    
    Ok(())
}

/// Update allocation statistics (called by memory allocator)
pub fn update_allocation_stats(size: usize, is_alloc: bool) {
    if is_alloc {
        GLOBAL_ALLOC_BYTES.fetch_add(size as u64, Ordering::SeqCst);
        GLOBAL_TOTAL_ALLOC.fetch_add(size as u64, Ordering::SeqCst);
        GLOBAL_MALLOCS.fetch_add(1, Ordering::SeqCst);
        
        // Update system bytes estimate
        let aligned_size = (size + 15) & !15; // Assume 16-byte alignment overhead
        GLOBAL_SYSTEM_BYTES.fetch_add(aligned_size as u64, Ordering::SeqCst);
    } else {
        GLOBAL_ALLOC_BYTES.fetch_sub(size as u64, Ordering::SeqCst);
        GLOBAL_FREES.fetch_add(1, Ordering::SeqCst);
    }
}

/// Get system memory information (platform-specific)
fn get_system_memory_info() -> crate::error::Result<()> {
    // Get heap system memory
    let heap_sys = GLOBAL_SYSTEM_BYTES.load(Ordering::SeqCst);
    
    // Estimate stack system memory
    let stack_sys = estimate_stack_system_memory()?;
    
    // Estimate GC system memory (rough heuristic: 5% of heap)
    let gc_sys = heap_sys / 20;
    
    Ok((heap_sys, stack_sys, gc_sys))
}

/// Estimate stack memory usage
fn get_stack_usage() -> crate::error::Result<()> {
    // Try to get goroutine stack information if available
    if let Ok(goroutine_count) = super::goroutine::num_goroutine() {
        // Estimate: each goroutine has approximately 64KB stack
        Ok(goroutine_count as u64 * 64 * 1024)
    } else {
        // Fallback: estimate based on current thread
        Ok(1024 * 1024) // 1MB estimate for main thread
    }
}

/// Estimate total stack system memory
fn estimate_stack_system_memory() -> crate::error::Result<()> {
    // Stack system memory is typically larger than in-use due to guard pages
    let stack_inuse = get_stack_usage()?;
    Ok(stack_inuse + (stack_inuse / 4)) // Add 25% overhead for guard pages
}

/// Memory profiling functionality
#[derive(Debug)]
pub struct MemoryProfile {
    /// Total heap allocations by size class
    pub heap_allocations: Vec<(usize, u64)>,
    /// Stack usage by goroutine
    pub stack_usage: Vec<(u64, usize)>,
    /// GC overhead breakdown
    pub gc_overhead: GcOverhead,
}

#[derive(Debug)]
pub struct GcOverhead {
    /// Time spent in GC
    pub total_gc_time: u64,
    /// Memory used for GC metadata  
    pub metadata_bytes: u64,
    /// Write barrier overhead
    pub write_barrier_cost: f64,
}

/// Create a memory profile snapshot
pub fn memory_profile() -> crate::error::Result<()> {
    let mut stats = MemStats::new();
    read_mem_stats(&mut stats)?;
    
    // Get goroutine stack information
    let stack_usage = if let Ok(goroutines) = super::goroutine::get_all_goroutine_info() {
        goroutines.into_iter()
            .map(|(id, info)| (id, info.stack_size))
            .collect()
    } else {
        vec![(1, 1024 * 1024)] // Main thread estimate
    };
    
    // Create simplified heap allocation breakdown
    let heap_allocations = vec![
        (64, stats.mallocs / 4),     // Small objects
        (512, stats.mallocs / 4),   // Medium objects  
        (4096, stats.mallocs / 4),  // Large objects
        (65536, stats.mallocs / 4), // Extra large objects
    ];
    
    let gc_overhead = GcOverhead {
        total_gc_time: stats.pause_total_ns,
        metadata_bytes: stats.gc_sys,
        write_barrier_cost: stats.gc_cpu_fraction,
    };
    
    Ok(MemoryProfile {
        heap_allocations,
        stack_usage,
        gc_overhead,
    })
}

/// Write memory profile to a string format
pub fn write_profile(profile: &MemoryProfile) -> String {
    let mut output = String::new();
    
    output.push_str("=== CURSED Memory Profile ===\n\n");
    
    output.push_str("Heap Allocations by Size:\n");
    for (size, count) in &profile.heap_allocations {
        output.push_str(&format!("  {:<8} bytes: {} allocations\n", size, count));
    }
    
    output.push_str("\nStack Usage by Goroutine:\n");
    for (id, size) in &profile.stack_usage {
        output.push_str(&format!("  Goroutine {}: {} bytes\n", id, size));
    }
    
    output.push_str("\nGC Overhead:\n");
    output.push_str(&format!("  Total GC time: {} ns\n", profile.gc_overhead.total_gc_time));
    output.push_str(&format!("  Metadata bytes: {}\n", profile.gc_overhead.metadata_bytes));
    output.push_str(&format!("  Write barrier cost: {:.4}\n", profile.gc_overhead.write_barrier_cost));
    
    output
}

/// Force memory to be returned to the operating system
pub fn free_os_memory() -> crate::error::Result<()> {
    // In a real implementation, this would call into the allocator
    // to release unused memory back to the OS
    
    // For now, we'll just trigger a GC cycle to clean up
    super::gc::run_gc()?;
    
    // On Unix systems, we could call malloc_trim(0)
    #[cfg(unix)]
    {
        unsafe {
            libc::malloc_trim(0);
        }
    }
    
    Ok(())
}

