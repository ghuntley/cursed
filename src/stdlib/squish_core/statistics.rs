//! Statistics tracking for compression operations

use std::sync::{RwLock, Mutex};

/// Module statistics
#[derive(Debug, Clone, Default)]
pub struct ModuleStats {
    pub operations_count: u64,
    pub bytes_compressed: u64,
    pub bytes_decompressed: u64,
    pub compression_time: std::time::Duration,
    pub decompression_time: std::time::Duration,
}

static MODULE_STATS: RwLock<ModuleStats> = RwLock::new(ModuleStats {
    operations_count: 0,
    bytes_compressed: 0,
    bytes_decompressed: 0,
    compression_time: std::time::Duration::from_secs(0),
    decompression_time: std::time::Duration::from_secs(0),
});

pub fn get_module_stats() -> ModuleStats {
    MODULE_STATS.read().unwrap().clone()
}

pub fn cleanup() {
    if let Ok(mut stats) = MODULE_STATS.write() {
        *stats = ModuleStats::default();
    }
}
