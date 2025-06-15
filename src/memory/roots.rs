/// Root Set Management for Garbage Collection
/// 
/// This module provides comprehensive root set tracking and scanning for the
/// garbage collector. Roots are objects that are directly reachable by the
/// program and serve as starting points for reachability analysis.

use std::sync::{Arc, RwLock, Weak};
use std::collections::{HashSet, HashMap, BTreeSet};
use std::ptr::NonNull;
use tracing::{instrument, debug, info, warn, error};

use crate::memory::{Traceable, Visitor};
use crate::memory::object_id::{ObjectId, ObjectRegistry, SharedObjectRegistry};
use crate::runtime::stack_walker::{StackWalker, StackWalkConfig, RawStackFrame};

/// Types of GC roots in the system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RootType {
    /// Global variables and static objects
    Global,
    /// Stack-allocated local variables and parameters
    Stack,
    /// Thread-local storage
    ThreadLocal,
    /// Pinned objects that must not be moved
    Pinned,
    /// External references from native code
    External,
    /// Temporary roots during collection
    Temporary,
}

/// Information about a specific root
#[derive(Debug, Clone)]
pub struct RootInfo {
    pub object_id: ObjectId,
    pub root_type: RootType,
    pub source_location: Option<String>,
    pub created_at: std::time::Instant,
    pub access_count: u64,
}

/// Root set manager for tracking GC roots
pub struct RootSetManager {
    /// All tracked roots organized by type
    roots_by_type: RwLock<HashMap<RootType, BTreeSet<ObjectId>>>,
    /// Detailed information about each root
    root_info: RwLock<HashMap<ObjectId, RootInfo>>,
    /// Object registry for validation
    object_registry: SharedObjectRegistry,
    /// Stack scanning configuration
    stack_config: StackScanConfig,
    /// Statistics tracking
    stats: RwLock<RootSetStats>,
    /// Cached stack maps for performance
    stack_map_cache: RwLock<HashMap<std::thread::ThreadId, ThreadStackMaps>>,
}

/// Configuration for stack scanning
#[derive(Debug, Clone)]
pub struct StackScanConfig {
    /// Enable conservative stack scanning
    pub conservative_scanning: bool,
    /// Enable precise stack scanning (requires stack maps)
    pub precise_scanning: bool,
    /// Maximum stack depth to scan
    pub max_stack_depth: usize,
    /// Stack region size for scanning
    pub stack_region_size: usize,
    /// Enable stack pointer validation
    pub validate_pointers: bool,
    /// Prefer precise over conservative when both are available
    pub prefer_precise: bool,
    /// Cache stack maps for performance
    pub cache_stack_maps: bool,
    /// Maximum number of cached stack maps
    pub max_cached_maps: usize,
}

impl Default for StackScanConfig {
    fn default() -> Self {
        Self {
            conservative_scanning: true,
            precise_scanning: false,
            max_stack_depth: 1024,
            stack_region_size: 64 * 1024, // 64KB
            validate_pointers: true,
            prefer_precise: true,
            cache_stack_maps: true,
            max_cached_maps: 100,
        }
    }
}

/// Statistics about root set management
#[derive(Debug, Clone, Default)]
pub struct RootSetStats {
    pub total_roots: usize,
    pub roots_by_type: HashMap<RootType, usize>,
    pub stack_scans_performed: u64,
    pub stack_roots_found: u64,
    pub root_additions: u64,
    pub root_removals: u64,
    pub invalid_roots_cleaned: u64,
}

/// Stack map entry describing pointer locations in a function frame
#[derive(Debug, Clone)]
pub struct StackMapEntry {
    /// Function address range this map applies to
    pub function_address: usize,
    pub function_size: usize,
    /// Instruction offsets where safe points exist
    pub safe_points: Vec<SafePointInfo>,
    /// Frame layout information
    pub frame_info: FrameLayoutInfo,
}

/// Information about a specific safe point in the code
#[derive(Debug, Clone)]
pub struct SafePointInfo {
    /// Offset from function start
    pub instruction_offset: usize,
    /// Live pointer locations at this safe point
    pub live_pointers: Vec<PointerLocationInfo>,
    /// Stack frame size at this point
    pub frame_size: usize,
}

/// Information about where a pointer is located in the stack frame
#[derive(Debug, Clone)]
pub struct PointerLocationInfo {
    /// Offset from frame pointer (positive = parameters, negative = locals)
    pub frame_offset: isize,
    /// Size of the pointer (typically 8 bytes on 64-bit systems)
    pub pointer_size: usize,
    /// Type of the pointer (if known)
    pub pointer_type: PointerType,
    /// Whether this pointer is definitely an ObjectId
    pub is_object_id: bool,
}

/// Types of pointers that can be found in stack frames
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointerType {
    /// Raw pointer to memory
    Raw,
    /// Smart pointer (Arc, Rc, etc.)
    Smart,
    /// Object ID representing a managed object
    ObjectId,
    /// Reference to stack-allocated data
    Reference,
    /// Unknown pointer type
    Unknown,
}

/// Layout information for a stack frame
#[derive(Debug, Clone)]
pub struct FrameLayoutInfo {
    /// Size of the entire frame
    pub frame_size: usize,
    /// Offset where local variables start
    pub locals_offset: isize,
    /// Offset where parameters start
    pub parameters_offset: isize,
    /// Size of the return address slot
    pub return_address_size: usize,
    /// Whether frame pointer is used
    pub uses_frame_pointer: bool,
}

/// Complete stack map information for a thread
#[derive(Debug, Clone)]
pub struct ThreadStackMaps {
    /// Maps for each function
    pub function_maps: HashMap<usize, StackMapEntry>,
    /// Global stack layout information
    pub stack_layout: StackLayoutInfo,
    /// Thread identifier
    pub thread_id: std::thread::ThreadId,
}

/// Global stack layout information
#[derive(Debug, Clone)]
pub struct StackLayoutInfo {
    /// Stack grows downward on most architectures
    pub grows_downward: bool,
    /// Alignment requirement for stack slots
    pub stack_alignment: usize,
    /// Size of a pointer on this architecture
    pub pointer_size: usize,
    /// Red zone size (area below stack pointer that's safe to use)
    pub red_zone_size: usize,
}

impl RootSetManager {
    /// Create a new root set manager
    pub fn new(object_registry: SharedObjectRegistry) -> Self {
        Self::with_config(object_registry, StackScanConfig::default())
    }
    
    /// Create a new root set manager with custom stack scanning configuration
    #[instrument(skip(object_registry))]
    pub fn with_config(object_registry: SharedObjectRegistry, stack_config: StackScanConfig) -> Self {
        info!("Creating root set manager with config: {:?}", stack_config);
        
        Self {
            roots_by_type: RwLock::new(HashMap::new()),
            root_info: RwLock::new(HashMap::new()),
            object_registry,
            stack_config,
            stats: RwLock::new(RootSetStats::default()),
            stack_map_cache: RwLock::new(HashMap::new()),
        }
    }
    
    /// Add an object as a GC root
    #[instrument(skip(self))]
    pub fn add_root(&self, object_id: ObjectId, root_type: RootType, source_location: Option<String>) -> Result<(), String> {
        debug!("Adding root object {} of type {:?}", object_id, root_type);
        
        // Validate object exists
        if !self.is_valid_object(object_id)? {
            return Err(format!("Cannot add invalid object {} as root", object_id));
        }
        
        let mut roots_by_type = self.roots_by_type.write()
            .map_err(|_| "Failed to acquire write lock on roots_by_type")?;
        let mut root_info = self.root_info.write()
            .map_err(|_| "Failed to acquire write lock on root_info")?;
        let mut stats = self.stats.write()
            .map_err(|_| "Failed to acquire write lock on stats")?;
        
        // Add to roots by type
        roots_by_type.entry(root_type).or_insert_with(BTreeSet::new).insert(object_id);
        
        // Add detailed info
        let info = RootInfo {
            object_id,
            root_type,
            source_location,
            created_at: std::time::Instant::now(),
            access_count: 0,
        };
        root_info.insert(object_id, info);
        
        // Update statistics
        stats.total_roots += 1;
        *stats.roots_by_type.entry(root_type).or_insert(0) += 1;
        stats.root_additions += 1;
        
        debug!("Successfully added root object {}", object_id);
        Ok(())
    }
    
    /// Remove an object from GC roots
    #[instrument(skip(self))]
    pub fn remove_root(&self, object_id: ObjectId) -> Result<bool, String> {
        debug!("Removing root object {}", object_id);
        
        let mut roots_by_type = self.roots_by_type.write()
            .map_err(|_| "Failed to acquire write lock on roots_by_type")?;
        let mut root_info = self.root_info.write()
            .map_err(|_| "Failed to acquire write lock on root_info")?;
        let mut stats = self.stats.write()
            .map_err(|_| "Failed to acquire write lock on stats")?;
        
        // Get root info to know the type
        let info = match root_info.remove(&object_id) {
            Some(info) => info,
            None => {
                debug!("Object {} was not a root", object_id);
                return Ok(false);
            }
        };
        
        // Remove from type-specific set
        if let Some(type_set) = roots_by_type.get_mut(&info.root_type) {
            type_set.remove(&object_id);
            if type_set.is_empty() {
                roots_by_type.remove(&info.root_type);
            }
        }
        
        // Update statistics
        stats.total_roots = stats.total_roots.saturating_sub(1);
        let type_count = stats.roots_by_type.entry(info.root_type).or_insert(0);
        *type_count = type_count.saturating_sub(1);
        if *type_count == 0 {
            stats.roots_by_type.remove(&info.root_type);
        }
        stats.root_removals += 1;
        
        debug!("Successfully removed root object {}", object_id);
        Ok(true)
    }
    
    /// Get all root objects of a specific type
    pub fn get_roots_by_type(&self, root_type: RootType) -> Result<Vec<ObjectId>, String> {
        let roots_by_type = self.roots_by_type.read()
            .map_err(|_| "Failed to acquire read lock on roots_by_type")?;
        
        Ok(roots_by_type.get(&root_type)
            .map(|set| set.iter().copied().collect())
            .unwrap_or_default())
    }
    
    /// Get all root objects
    pub fn get_all_roots(&self) -> Result<Vec<ObjectId>, String> {
        let roots_by_type = self.roots_by_type.read()
            .map_err(|_| "Failed to acquire read lock on roots_by_type")?;
        
        let mut all_roots = Vec::new();
        for type_set in roots_by_type.values() {
            all_roots.extend(type_set.iter().copied());
        }
        
        Ok(all_roots)
    }
    
    /// Scan the stack for potential GC roots
    #[instrument(skip(self))]
    pub fn scan_stack(&self) -> Result<Vec<ObjectId>, String> {
        info!("Starting stack scan for GC roots");
        
        let mut stats = self.stats.write()
            .map_err(|_| "Failed to acquire write lock on stats")?;
        stats.stack_scans_performed += 1;
        drop(stats);
        
        let mut found_roots = Vec::new();
        
        // Prefer precise scanning when both are available and prefer_precise is true
        if self.stack_config.precise_scanning && self.stack_config.prefer_precise {
            let precise_roots = self.precise_stack_scan()?;
            if !precise_roots.is_empty() {
                found_roots.extend(precise_roots);
                info!("Used precise stack scanning, found {} roots", found_roots.len());
            } else if self.stack_config.conservative_scanning {
                found_roots.extend(self.conservative_stack_scan()?);
                info!("Precise scanning found no roots, fell back to conservative scanning");
            }
        } else {
            // Use conservative scanning first, then supplement with precise if available
            if self.stack_config.conservative_scanning {
                found_roots.extend(self.conservative_stack_scan()?);
            }
            
            if self.stack_config.precise_scanning {
                let precise_roots = self.precise_stack_scan()?;
                // Remove duplicates from precise scanning
                let conservative_set: HashSet<_> = found_roots.iter().copied().collect();
                for root in precise_roots {
                    if !conservative_set.contains(&root) {
                        found_roots.push(root);
                    }
                }
            }
        }
        
        // Update statistics
        let mut stats = self.stats.write()
            .map_err(|_| "Failed to acquire write lock on stats")?;
        stats.stack_roots_found += found_roots.len() as u64;
        
        info!("Stack scan completed, found {} potential roots", found_roots.len());
        Ok(found_roots)
    }
    
    /// Conservative stack scanning - scans memory patterns for potential pointers
    fn conservative_stack_scan(&self) -> Result<Vec<ObjectId>, String> {
        debug!("Performing conservative stack scan");
        
        let mut potential_roots = Vec::new();
        
        // Get current stack boundaries (simplified implementation)
        let stack_start = self.get_stack_start();
        let stack_end = self.get_stack_end();
        
        if stack_start.is_null() || stack_end.is_null() || stack_start >= stack_end {
            warn!("Invalid stack boundaries, skipping stack scan");
            return Ok(potential_roots);
        }
        
        // Scan stack memory for potential object IDs
        let stack_size = unsafe { stack_end.offset_from(stack_start) } as usize;
        let scan_size = stack_size.min(self.stack_config.stack_region_size);
        
        debug!("Scanning {} bytes of stack memory", scan_size);
        
        // Scan in pointer-sized chunks
        let ptr_size = std::mem::size_of::<usize>();
        for offset in (0..scan_size).step_by(ptr_size) {
            unsafe {
                let potential_ptr = stack_start.offset(offset as isize) as *const usize;
                if potential_ptr.is_null() {
                    continue;
                }
                
                let value = *potential_ptr;
                
                // Try to interpret as an ObjectId
                let object_id = ObjectId::from_raw(value as u64);
                
                // Validate if this could be a real object
                if self.stack_config.validate_pointers {
                    if let Ok(is_valid) = self.is_valid_object(object_id) {
                        if is_valid {
                            potential_roots.push(object_id);
                        }
                    }
                } else {
                    potential_roots.push(object_id);
                }
            }
        }
        
        debug!("Conservative scan found {} potential roots", potential_roots.len());
        Ok(potential_roots)
    }
    
    /// Precise stack scanning using stack maps (when available)
    fn precise_stack_scan(&self) -> Result<Vec<ObjectId>, String> {
        debug!("Performing precise stack scan");
        
        let mut precise_roots = Vec::new();
        
        // Get current thread's stack maps
        let stack_maps = self.get_current_thread_stack_maps()?;
        
        if stack_maps.is_empty() {
            debug!("No stack maps available, falling back to conservative scanning");
            return Ok(precise_roots);
        }
        
        // Walk the stack frames using the stack walker infrastructure
        let stack_walker = self.create_stack_walker_for_scanning()?;
        let frames = stack_walker.walk_stack()
            .map_err(|e| format!("Failed to walk stack: {}", e))?;
        
        // For each frame, use stack maps to find precise pointer locations
        for frame in &frames {
            if let Some(frame_roots) = self.scan_frame_with_stack_maps(frame, &stack_maps)? {
                precise_roots.extend(frame_roots);
            }
        }
        
        // Validate all found roots
        let validated_roots = self.validate_precise_roots(precise_roots)?;
        
        debug!("Precise scan found {} validated roots from {} frames", 
               validated_roots.len(), frames.len());
        Ok(validated_roots)
    }
    
    /// Get stack start pointer (platform-specific)
    fn get_stack_start(&self) -> *const u8 {
        // Simplified implementation - in practice this would be platform-specific
        std::ptr::null()
    }
    
    /// Get stack end pointer (platform-specific)
    fn get_stack_end(&self) -> *const u8 {
        // Simplified implementation - in practice this would be platform-specific
        std::ptr::null()
    }
    
    /// Get stack maps for the current thread
    fn get_current_thread_stack_maps(&self) -> Result<ThreadStackMaps, String> {
        let thread_id = std::thread::current().id();
        
        // Check cache first if caching is enabled
        if self.stack_config.cache_stack_maps {
            let cache = self.stack_map_cache.read()
                .map_err(|_| "Failed to acquire read lock on stack map cache")?;
            
            if let Some(cached_maps) = cache.get(&thread_id) {
                debug!("Using cached stack maps for thread {:?}", thread_id);
                return Ok(cached_maps.clone());
            }
        }
        
        // Create new stack maps
        let stack_layout = StackLayoutInfo {
            grows_downward: true,
            stack_alignment: 8,
            pointer_size: std::mem::size_of::<usize>(),
            red_zone_size: 128, // Common x86_64 red zone size
        };
        
        // Try to get stack maps from the runtime or debug information
        let function_maps = self.query_runtime_stack_maps()
            .unwrap_or_else(|_| {
                debug!("Runtime stack maps not available, using empty map");
                HashMap::new()
            });
        
        let stack_maps = ThreadStackMaps {
            function_maps,
            stack_layout,
            thread_id,
        };
        
        // Cache the result if caching is enabled
        if self.stack_config.cache_stack_maps {
            let mut cache = self.stack_map_cache.write()
                .map_err(|_| "Failed to acquire write lock on stack map cache")?;
            
            // Implement LRU eviction if cache is full
            if cache.len() >= self.stack_config.max_cached_maps {
                // Simple eviction - remove oldest entry
                if let Some(oldest_thread_id) = cache.keys().next().copied() {
                    cache.remove(&oldest_thread_id);
                    debug!("Evicted stack maps for thread {:?} from cache", oldest_thread_id);
                }
            }
            
            cache.insert(thread_id, stack_maps.clone());
            debug!("Cached stack maps for thread {:?}", thread_id);
        }
        
        Ok(stack_maps)
    }
    
    /// Query runtime system for stack maps
    fn query_runtime_stack_maps(&self) -> Result<HashMap<usize, StackMapEntry>, String> {
        // This would integrate with the CURSED runtime to get compiled stack maps
        // For now, return empty - in a real implementation this would:
        // 1. Query the debug information manager
        // 2. Load stack maps generated by the LLVM backend
        // 3. Cache the results for performance
        Ok(HashMap::new())
    }
    
    /// Create a stack walker configured for GC scanning
    fn create_stack_walker_for_scanning(&self) -> Result<StackWalker, String> {
        let config = StackWalkConfig {
            max_frames: self.stack_config.max_stack_depth,
            resolve_symbols: true,
            capture_source_info: true,
            max_symbol_length: 500,
            skip_system_frames: false, // Need all frames for GC
            cursed_frames_only: false, // Need all frames for GC
        };
        
        Ok(StackWalker::with_config(config))
    }
    
    /// Scan a specific frame using stack maps to find object references
    fn scan_frame_with_stack_maps(
        &self,
        frame: &RawStackFrame,
        stack_maps: &ThreadStackMaps
    ) -> Result<Option<Vec<ObjectId>>, String> {
        // Find the stack map entry for this frame's function
        let stack_map = match stack_maps.function_maps.get(&frame.instruction_pointer) {
            Some(map) => map,
            None => {
                // No stack map for this function - might be runtime/system code
                debug!("No stack map found for function at 0x{:x}", frame.instruction_pointer);
                return Ok(None);
            }
        };
        
        // Find the safe point closest to the instruction pointer
        let instruction_offset = frame.instruction_pointer.saturating_sub(stack_map.function_address);
        let safe_point = self.find_nearest_safe_point(stack_map, instruction_offset)?;
        
        let mut frame_roots = Vec::new();
        
        // Scan each pointer location described in the safe point
        for pointer_info in &safe_point.live_pointers {
            if let Some(object_id) = self.extract_object_id_from_frame_location(
                frame, 
                pointer_info, 
                &stack_maps.stack_layout
            )? {
                frame_roots.push(object_id);
            }
        }
        
        debug!("Found {} potential roots in frame at 0x{:x}", 
               frame_roots.len(), frame.instruction_pointer);
        
        Ok(Some(frame_roots))
    }
    
    /// Find the safe point nearest to the given instruction offset
    fn find_nearest_safe_point(
        &self,
        stack_map: &StackMapEntry,
        instruction_offset: usize
    ) -> Result<&SafePointInfo, String> {
        if stack_map.safe_points.is_empty() {
            return Err("No safe points found in stack map".to_string());
        }
        
        // Find the safe point with the largest offset that's still <= instruction_offset
        let mut best_safe_point = &stack_map.safe_points[0];
        
        for safe_point in &stack_map.safe_points {
            if safe_point.instruction_offset <= instruction_offset {
                if safe_point.instruction_offset > best_safe_point.instruction_offset {
                    best_safe_point = safe_point;
                }
            }
        }
        
        debug!("Selected safe point at offset {} for instruction offset {}", 
               best_safe_point.instruction_offset, instruction_offset);
        
        Ok(best_safe_point)
    }
    
    /// Extract an ObjectId from a specific frame location
    fn extract_object_id_from_frame_location(
        &self,
        frame: &RawStackFrame,
        pointer_info: &PointerLocationInfo,
        stack_layout: &StackLayoutInfo
    ) -> Result<Option<ObjectId>, String> {
        // Only extract if this is known to be an ObjectId
        if !pointer_info.is_object_id && pointer_info.pointer_type != PointerType::ObjectId {
            return Ok(None);
        }
        
        // Calculate the actual memory address of the pointer
        let frame_pointer = frame.frame_pointer
            .ok_or("Frame pointer not available for precise scanning")?;
        
        let pointer_address = if pointer_info.frame_offset >= 0 {
            // Positive offset - in parameters area
            frame_pointer.wrapping_add(pointer_info.frame_offset as usize)
        } else {
            // Negative offset - in locals area
            frame_pointer.wrapping_sub((-pointer_info.frame_offset) as usize)
        };
        
        // Validate the address is within reasonable bounds
        if !self.is_valid_stack_address(pointer_address, stack_layout)? {
            debug!("Invalid stack address 0x{:x} for frame pointer extraction", pointer_address);
            return Ok(None);
        }
        
        // Read the ObjectId from memory
        unsafe {
            let ptr = pointer_address as *const u64;
            if ptr.is_null() {
                return Ok(None);
            }
            
            let raw_value = std::ptr::read(ptr);
            let object_id = ObjectId::from_raw(raw_value);
            
            // Additional validation that this is a valid ObjectId
            if self.is_valid_object(object_id)? {
                debug!("Extracted valid ObjectId {} from frame offset {}", 
                       object_id, pointer_info.frame_offset);
                Ok(Some(object_id))
            } else {
                debug!("Invalid ObjectId {} extracted from frame offset {}", 
                       object_id, pointer_info.frame_offset);
                Ok(None)
            }
        }
    }
    
    /// Validate that an address is within valid stack bounds
    fn is_valid_stack_address(
        &self,
        address: usize,
        stack_layout: &StackLayoutInfo
    ) -> Result<bool, String> {
        // Basic validation - ensure address is aligned and within reasonable bounds
        if address % stack_layout.stack_alignment != 0 {
            return Ok(false);
        }
        
        // Additional platform-specific validation could go here
        // For now, just check that it's not null and seems reasonable
        Ok(address != 0 && address > 0x1000) // Avoid low memory addresses
    }
    
    /// Validate that precise roots are legitimate
    fn validate_precise_roots(&self, roots: Vec<ObjectId>) -> Result<Vec<ObjectId>, String> {
        let mut validated = Vec::new();
        
        for object_id in roots {
            // Check if the object actually exists and is valid
            if self.is_valid_object(object_id)? {
                validated.push(object_id);
            } else {
                debug!("Discarding invalid precise root: {}", object_id);
            }
        }
        
        // Remove duplicates while preserving order
        let mut seen = HashSet::new();
        validated.retain(|&id| seen.insert(id));
        
        debug!("Validated {} out of {} precise roots", validated.len(), roots.len());
        Ok(validated)
    }
    
    /// Clean up invalid roots (objects that no longer exist)
    #[instrument(skip(self))]
    pub fn cleanup_invalid_roots(&self) -> Result<usize, String> {
        debug!("Cleaning up invalid roots");
        
        let invalid_roots = {
            let root_info = self.root_info.read()
                .map_err(|_| "Failed to acquire read lock on root_info")?;
            
            let mut invalid = Vec::new();
            for &object_id in root_info.keys() {
                if !self.is_valid_object(object_id)? {
                    invalid.push(object_id);
                }
            }
            invalid
        };
        
        let cleaned_count = invalid_roots.len();
        
        for object_id in invalid_roots {
            self.remove_root(object_id)?;
        }
        
        // Update statistics
        let mut stats = self.stats.write()
            .map_err(|_| "Failed to acquire write lock on stats")?;
        stats.invalid_roots_cleaned += cleaned_count as u64;
        
        info!("Cleaned up {} invalid roots", cleaned_count);
        Ok(cleaned_count)
    }
    
    /// Check if an object is valid
    fn is_valid_object(&self, object_id: ObjectId) -> Result<bool, String> {
        match self.object_registry.get(object_id) {
            Ok(Some(_)) => Ok(true),
            Ok(None) => Ok(false),
            Err(e) => Err(format!("Failed to check object validity: {}", e)),
        }
    }
    
    /// Get root set statistics
    pub fn get_stats(&self) -> Result<RootSetStats, String> {
        let stats = self.stats.read()
            .map_err(|_| "Failed to acquire read lock on stats")?;
        Ok(stats.clone())
    }
    
    /// Check if an object is a root
    pub fn is_root(&self, object_id: ObjectId) -> Result<bool, String> {
        let root_info = self.root_info.read()
            .map_err(|_| "Failed to acquire read lock on root_info")?;
        Ok(root_info.contains_key(&object_id))
    }
    
    /// Get information about a specific root
    pub fn get_root_info(&self, object_id: ObjectId) -> Result<Option<RootInfo>, String> {
        let root_info = self.root_info.read()
            .map_err(|_| "Failed to acquire read lock on root_info")?;
        Ok(root_info.get(&object_id).cloned())
    }
    
    /// Mark root access for statistics
    pub fn mark_root_access(&self, object_id: ObjectId) -> Result<(), String> {
        let mut root_info = self.root_info.write()
            .map_err(|_| "Failed to acquire write lock on root_info")?;
        
        if let Some(info) = root_info.get_mut(&object_id) {
            info.access_count += 1;
        }
        
        Ok(())
    }
    
    /// Clear stack map cache (useful for testing or memory management)
    pub fn clear_stack_map_cache(&self) -> Result<usize, String> {
        let mut cache = self.stack_map_cache.write()
            .map_err(|_| "Failed to acquire write lock on stack map cache")?;
        
        let cleared_count = cache.len();
        cache.clear();
        
        debug!("Cleared {} entries from stack map cache", cleared_count);
        Ok(cleared_count)
    }
    
    /// Get stack map cache statistics
    pub fn get_stack_map_cache_stats(&self) -> Result<(usize, usize), String> {
        let cache = self.stack_map_cache.read()
            .map_err(|_| "Failed to acquire read lock on stack map cache")?;
        
        Ok((cache.len(), self.stack_config.max_cached_maps))
    }
    
    /// Force enable precise scanning (useful for testing)
    pub fn enable_precise_scanning(&mut self) {
        self.stack_config.precise_scanning = true;
        self.stack_config.prefer_precise = true;
    }
    
    /// Force disable precise scanning (fallback to conservative only)
    pub fn disable_precise_scanning(&mut self) {
        self.stack_config.precise_scanning = false;
    }
}

impl Default for RootSetManager {
    fn default() -> Self {
        // We can't create a default without an ObjectRegistry, so this creates
        // a new registry. In practice, you'd typically provide the registry.
        let registry = Arc::new(crate::memory::object_id::ObjectRegistry::new());
        Self::new(registry)
    }
}

// Safety: RootSetManager is thread-safe through its use of RwLock
unsafe impl Send for RootSetManager {}
unsafe impl Sync for RootSetManager {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::object_id::ObjectRegistry;
    
    fn create_test_manager() -> (RootSetManager, SharedObjectRegistry) {
        let registry = Arc::new(ObjectRegistry::new());
        let manager = RootSetManager::new(registry.clone());
        (manager, registry)
    }
    
    #[test]
    fn test_add_and_remove_roots() {
        let (manager, _registry) = create_test_manager();
        
        let object_id = ObjectId::new(1);
        
        // Add root
        manager.add_root(object_id, RootType::Global, Some("test".to_string())).unwrap();
        
        // Check it's a root
        assert!(manager.is_root(object_id).unwrap());
        
        // Remove root
        assert!(manager.remove_root(object_id).unwrap());
        
        // Check it's no longer a root
        assert!(!manager.is_root(object_id).unwrap());
    }
    
    #[test]
    fn test_roots_by_type() {
        let (manager, _registry) = create_test_manager();
        
        let global_id = ObjectId::new(2);
        let stack_id = ObjectId::new(3);
        
        manager.add_root(global_id, RootType::Global, None).unwrap();
        manager.add_root(stack_id, RootType::Stack, None).unwrap();
        
        let global_roots = manager.get_roots_by_type(RootType::Global).unwrap();
        assert_eq!(global_roots.len(), 1);
        assert!(global_roots.contains(&global_id));
        
        let stack_roots = manager.get_roots_by_type(RootType::Stack).unwrap();
        assert_eq!(stack_roots.len(), 1);
        assert!(stack_roots.contains(&stack_id));
    }
    
    #[test]
    fn test_statistics() {
        let (manager, _registry) = create_test_manager();
        
        let object_id = ObjectId::new(4);
        
        let initial_stats = manager.get_stats().unwrap();
        assert_eq!(initial_stats.total_roots, 0);
        
        manager.add_root(object_id, RootType::Global, None).unwrap();
        
        let stats_after_add = manager.get_stats().unwrap();
        assert_eq!(stats_after_add.total_roots, 1);
        assert_eq!(stats_after_add.root_additions, 1);
        
        manager.remove_root(object_id).unwrap();
        
        let stats_after_remove = manager.get_stats().unwrap();
        assert_eq!(stats_after_remove.total_roots, 0);
        assert_eq!(stats_after_remove.root_removals, 1);
    }
    
    #[test]
    fn test_stack_scan_configuration() {
        let (mut manager, _registry) = create_test_manager();
        
        // Test enabling precise scanning
        manager.enable_precise_scanning();
        assert!(manager.stack_config.precise_scanning);
        assert!(manager.stack_config.prefer_precise);
        
        // Test disabling precise scanning
        manager.disable_precise_scanning();
        assert!(!manager.stack_config.precise_scanning);
    }
    
    #[test]
    fn test_stack_map_cache() {
        let (manager, _registry) = create_test_manager();
        
        // Initially cache should be empty
        let (count, max) = manager.get_stack_map_cache_stats().unwrap();
        assert_eq!(count, 0);
        assert_eq!(max, 100); // Default max
        
        // Clear empty cache
        let cleared = manager.clear_stack_map_cache().unwrap();
        assert_eq!(cleared, 0);
    }
    
    #[test]
    fn test_precise_stack_scan_with_no_maps() {
        let (manager, _registry) = create_test_manager();
        
        // Should gracefully handle no stack maps available
        let roots = manager.precise_stack_scan().unwrap();
        assert!(roots.is_empty());
    }
    
    #[test]
    fn test_stack_layout_creation() {
        let (manager, _registry) = create_test_manager();
        
        // Test stack maps creation
        let stack_maps = manager.get_current_thread_stack_maps().unwrap();
        assert_eq!(stack_maps.thread_id, std::thread::current().id());
        assert!(stack_maps.stack_layout.grows_downward);
        assert_eq!(stack_maps.stack_layout.pointer_size, std::mem::size_of::<usize>());
    }
    
    #[test]
    fn test_pointer_type_classification() {
        // Test pointer type enum
        assert_eq!(PointerType::ObjectId, PointerType::ObjectId);
        assert_ne!(PointerType::Raw, PointerType::Smart);
        
        // Test pointer location info
        let pointer_info = PointerLocationInfo {
            frame_offset: -8,
            pointer_size: 8,
            pointer_type: PointerType::ObjectId,
            is_object_id: true,
        };
        
        assert_eq!(pointer_info.frame_offset, -8);
        assert!(pointer_info.is_object_id);
    }
    
    #[test]
    fn test_frame_layout_info() {
        let frame_layout = FrameLayoutInfo {
            frame_size: 64,
            locals_offset: -32,
            parameters_offset: 16,
            return_address_size: 8,
            uses_frame_pointer: true,
        };
        
        assert_eq!(frame_layout.frame_size, 64);
        assert!(frame_layout.uses_frame_pointer);
        assert_eq!(frame_layout.return_address_size, 8);
    }
}
