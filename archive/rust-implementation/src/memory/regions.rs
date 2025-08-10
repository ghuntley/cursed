//! Memory regions for CURSED runtime
//! 
//! Implements region-based memory management for better locality
//! and more efficient allocation patterns.

use crate::error::CursedError;
use crate::memory::heap::{ObjectId, get_global_heap};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::alloc::Layout;

/// Memory region manager
pub struct RegionManager {
    /// Active regions
    regions: Mutex<HashMap<RegionId, Region>>,
    /// Next region ID
    next_region_id: std::sync::atomic::AtomicU64,
    /// Global statistics
    stats: Mutex<RegionStats>,
}

/// Unique region identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RegionId(u64);

/// A memory region
#[derive(Debug, Clone)]
pub struct Region {
    /// Region ID
    pub id: RegionId,
    /// Objects allocated in this region
    pub objects: HashSet<ObjectId>,
    /// Total size of objects in this region
    pub total_size: usize,
    /// Region type
    pub region_type: RegionType,
    /// Whether the region is currently active
    pub active: bool,
    /// Allocation count
    pub allocation_count: usize,
}

/// Types of memory regions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionType {
    /// Short-lived objects (stack-like)
    Ephemeral,
    /// Medium-lived objects
    Persistent,
    /// Long-lived objects (globals, etc.)
    Permanent,
    /// Large objects
    Large,
    /// Objects with specific allocation patterns
    Custom(u32),
}

/// Region statistics
#[derive(Debug, Clone, Default)]
pub struct RegionStats {
    pub total_regions: usize,
    pub active_regions: usize,
    pub total_size: usize,
    pub ephemeral_regions: usize,
    pub persistent_regions: usize,
    pub permanent_regions: usize,
    pub large_regions: usize,
}

impl Region {
    /// Create a new region
    pub fn new(id: RegionId, region_type: RegionType) -> Self {
        Self {
            id,
            objects: HashSet::new(),
            total_size: 0,
            region_type,
            active: true,
            allocation_count: 0,
        }
    }

    /// Allocate an object in this region
    pub fn allocate(&mut self, object_id: ObjectId, size: usize) -> Result<(), CursedError> {
        if !self.active {
            return Err(CursedError::RuntimeError(
                "Cannot allocate in inactive region".to_string()
            ));
        }

        self.objects.insert(object_id);
        self.total_size += size;
        self.allocation_count += 1;
        Ok(())
    }

    /// Deallocate an object from this region
    pub fn deallocate(&mut self, object_id: ObjectId, size: usize) -> bool {
        if self.objects.remove(&object_id) {
            self.total_size = self.total_size.saturating_sub(size);
            true
        } else {
            false
        }
    }

    /// Check if the region contains an object
    pub fn contains(&self, object_id: ObjectId) -> bool {
        self.objects.contains(&object_id)
    }

    /// Get object count in this region
    pub fn object_count(&self) -> usize {
        self.objects.len()
    }

    /// Deactivate the region (no new allocations)
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Clear all objects from the region
    pub fn clear(&mut self) {
        self.objects.clear();
        self.total_size = 0;
    }

    /// Get fragmentation ratio (0.0 = no fragmentation, 1.0 = highly fragmented)
    pub fn fragmentation_ratio(&self) -> f64 {
        if self.allocation_count == 0 {
            0.0
        } else {
            1.0 - (self.objects.len() as f64 / self.allocation_count as f64)
        }
    }
}

impl RegionManager {
    /// Create a new region manager
    pub fn new() -> Self {
        Self {
            regions: Mutex::new(HashMap::new()),
            next_region_id: std::sync::atomic::AtomicU64::new(1),
            stats: Mutex::new(RegionStats::default()),
        }
    }

    /// Create a new region
    pub fn create_region(&self, region_type: RegionType) -> Result<RegionId, CursedError> {
        let region_id = RegionId(
            self.next_region_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        );
        
        let region = Region::new(region_id, region_type);
        
        let mut regions = self.regions.lock().unwrap();
        regions.insert(region_id, region);
        
        // Update statistics
        let mut stats = self.stats.lock().unwrap();
        stats.total_regions += 1;
        stats.active_regions += 1;
        match region_type {
            RegionType::Ephemeral => stats.ephemeral_regions += 1,
            RegionType::Persistent => stats.persistent_regions += 1,
            RegionType::Permanent => stats.permanent_regions += 1,
            RegionType::Large => stats.large_regions += 1,
            RegionType::Custom(_) => {} // Don't track custom types in global stats
        }
        
        Ok(region_id)
    }

    /// Allocate an object in a specific region
    pub fn allocate_in_region(
        &self, 
        region_id: RegionId, 
        object_id: ObjectId, 
        size: usize
    ) -> Result<(), CursedError> {
        let mut regions = self.regions.lock().unwrap();
        
        if let Some(region) = regions.get_mut(&region_id) {
            region.allocate(object_id, size)?;
            
            // Update global statistics
            let mut stats = self.stats.lock().unwrap();
            stats.total_size += size;
            
            Ok(())
        } else {
            Err(CursedError::RuntimeError(
                format!("Region {:?} not found", region_id)
            ))
        }
    }

    /// Find the best region for an allocation
    pub fn find_best_region(&self, size: usize, preferred_type: RegionType) -> Option<RegionId> {
        let regions = self.regions.lock().unwrap();
        
        // First, try to find an active region of the preferred type
        let mut best_region = None;
        let mut best_fragmentation = f64::INFINITY;
        
        for (region_id, region) in regions.iter() {
            if region.active && region.region_type == preferred_type {
                let fragmentation = region.fragmentation_ratio();
                if fragmentation < best_fragmentation {
                    best_fragmentation = fragmentation;
                    best_region = Some(*region_id);
                }
            }
        }
        
        best_region
    }

    /// Allocate object with automatic region selection
    pub fn allocate_object(
        &self, 
        object_id: ObjectId, 
        size: usize, 
        preferred_type: RegionType
    ) -> Result<RegionId, CursedError> {
        // Try to find existing region
        if let Some(region_id) = self.find_best_region(size, preferred_type) {
            self.allocate_in_region(region_id, object_id, size)?;
            Ok(region_id)
        } else {
            // Create new region
            let region_id = self.create_region(preferred_type)?;
            self.allocate_in_region(region_id, object_id, size)?;
            Ok(region_id)
        }
    }

    /// Deallocate an object from its region
    pub fn deallocate_object(&self, object_id: ObjectId, size: usize) -> Result<Option<RegionId>, CursedError> {
        let mut regions = self.regions.lock().unwrap();
        
        // Find the region containing this object
        for (region_id, region) in regions.iter_mut() {
            if region.deallocate(object_id, size) {
                // Update global statistics
                let mut stats = self.stats.lock().unwrap();
                stats.total_size = stats.total_size.saturating_sub(size);
                
                return Ok(Some(*region_id));
            }
        }
        
        Ok(None)
    }

    /// Get region information
    pub fn get_region(&self, region_id: RegionId) -> Option<Region> {
        let regions = self.regions.lock().unwrap();
        regions.get(&region_id).cloned()
    }

    /// Deactivate a region (no new allocations)
    pub fn deactivate_region(&self, region_id: RegionId) -> Result<(), CursedError> {
        let mut regions = self.regions.lock().unwrap();
        
        if let Some(region) = regions.get_mut(&region_id) {
            region.deactivate();
            
            let mut stats = self.stats.lock().unwrap();
            stats.active_regions = stats.active_regions.saturating_sub(1);
            
            Ok(())
        } else {
            Err(CursedError::RuntimeError(
                format!("Region {:?} not found", region_id)
            ))
        }
    }

    /// Destroy a region and all its objects
    pub fn destroy_region(&self, region_id: RegionId) -> Result<usize, CursedError> {
        let mut regions = self.regions.lock().unwrap();
        
        if let Some(region) = regions.remove(&region_id) {
            let object_count = region.object_count();
            let region_size = region.total_size;
            let region_type = region.region_type;
            
            // Update statistics
            let mut stats = self.stats.lock().unwrap();
            stats.total_regions = stats.total_regions.saturating_sub(1);
            if region.active {
                stats.active_regions = stats.active_regions.saturating_sub(1);
            }
            stats.total_size = stats.total_size.saturating_sub(region_size);
            
            match region_type {
                RegionType::Ephemeral => {
                    stats.ephemeral_regions = stats.ephemeral_regions.saturating_sub(1);
                }
                RegionType::Persistent => {
                    stats.persistent_regions = stats.persistent_regions.saturating_sub(1);
                }
                RegionType::Permanent => {
                    stats.permanent_regions = stats.permanent_regions.saturating_sub(1);
                }
                RegionType::Large => {
                    stats.large_regions = stats.large_regions.saturating_sub(1);
                }
                RegionType::Custom(_) => {}
            }
            
            Ok(object_count)
        } else {
            Err(CursedError::RuntimeError(
                format!("Region {:?} not found", region_id)
            ))
        }
    }

    /// Collect garbage in ephemeral regions
    pub fn collect_ephemeral_regions(&self) -> Result<RegionCollectionResult, CursedError> {
        let mut regions = self.regions.lock().unwrap();
        let mut collected_objects = 0;
        let mut collected_bytes = 0;
        let mut destroyed_regions = 0;
        
        let ephemeral_regions: Vec<RegionId> = regions
            .iter()
            .filter(|(_, region)| region.region_type == RegionType::Ephemeral && !region.active)
            .map(|(id, _)| *id)
            .collect();
        
        for region_id in ephemeral_regions {
            if let Some(region) = regions.remove(&region_id) {
                collected_objects += region.object_count();
                collected_bytes += region.total_size;
                destroyed_regions += 1;
            }
        }
        
        // Update statistics
        let mut stats = self.stats.lock().unwrap();
        stats.total_regions = stats.total_regions.saturating_sub(destroyed_regions);
        stats.ephemeral_regions = stats.ephemeral_regions.saturating_sub(destroyed_regions);
        stats.total_size = stats.total_size.saturating_sub(collected_bytes);
        
        Ok(RegionCollectionResult {
            collected_objects,
            collected_bytes,
            destroyed_regions,
        })
    }

    /// Get all regions of a specific type
    pub fn get_regions_by_type(&self, region_type: RegionType) -> Vec<RegionId> {
        let regions = self.regions.lock().unwrap();
        regions
            .iter()
            .filter(|(_, region)| region.region_type == region_type)
            .map(|(id, _)| *id)
            .collect()
    }

    /// Get region statistics
    pub fn stats(&self) -> RegionStats {
        let stats = self.stats.lock().unwrap();
        stats.clone()
    }

    /// Get detailed region information
    pub fn get_region_details(&self) -> Vec<RegionDetails> {
        let regions = self.regions.lock().unwrap();
        regions
            .values()
            .map(|region| RegionDetails {
                id: region.id,
                region_type: region.region_type,
                object_count: region.object_count(),
                total_size: region.total_size,
                active: region.active,
                fragmentation_ratio: region.fragmentation_ratio(),
            })
            .collect()
    }
}

impl Default for RegionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of region collection
#[derive(Debug, Clone)]
pub struct RegionCollectionResult {
    pub collected_objects: usize,
    pub collected_bytes: usize,
    pub destroyed_regions: usize,
}

impl std::fmt::Display for RegionCollectionResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "Region Collection: {} objects ({} bytes) from {} regions",
            self.collected_objects,
            self.collected_bytes,
            self.destroyed_regions
        )
    }
}

/// Detailed region information
#[derive(Debug, Clone)]
pub struct RegionDetails {
    pub id: RegionId,
    pub region_type: RegionType,
    pub object_count: usize,
    pub total_size: usize,
    pub active: bool,
    pub fragmentation_ratio: f64,
}

/// Global region manager
static GLOBAL_REGION_MANAGER: std::sync::LazyLock<Arc<RegionManager>> = 
    std::sync::LazyLock::new(|| Arc::new(RegionManager::new()));

/// Get the global region manager
pub fn get_global_region_manager() -> Arc<RegionManager> {
    Arc::clone(&GLOBAL_REGION_MANAGER)
}

/// Convenience function to create a region
pub fn create_region(region_type: RegionType) -> Result<RegionId, CursedError> {
    get_global_region_manager().create_region(region_type)
}

/// Convenience function to allocate in a region
pub fn allocate_in_region(region_id: RegionId, object_id: ObjectId, size: usize) -> Result<(), CursedError> {
    get_global_region_manager().allocate_in_region(region_id, object_id, size)
}

/// Compatibility exports
pub use RegionManager as MinimalImplementation;

/// Convenience function for compatibility
pub fn get_minimal_result() -> Result<String, CursedError> {
    let manager = get_global_region_manager();
    let stats = manager.stats();
    Ok(format!("Region manager ready - {} regions, {} bytes", 
               stats.total_regions, stats.total_size))
}
