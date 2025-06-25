//! Baseline storage for optimization benchmarks

pub struct BaselineStorage;
pub struct BaselineStorageConfig;

#[derive(Debug, Clone)]
pub enum BaselineType {
    Performance,
    Memory,
    Size,
}

impl BaselineStorage {
    pub fn new(_config: BaselineStorageConfig) -> Self { BaselineStorage }
}
