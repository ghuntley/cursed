//! Performance profiler for CURSED compiler

use crate::error::CursedError;
use crate::performance::PerformanceConfig;

pub struct PerformanceProfiler {
    config: PerformanceConfig,
}

impl PerformanceProfiler {
    pub fn new(config: PerformanceConfig) -> Result<Self, CursedError> {
        Ok(Self { config })
    }

    pub fn start(&self) -> Result<(), CursedError> {
        println!("Performance profiler started");
        Ok(())
    }

    pub fn stop(&self) -> Result<(), CursedError> {
        println!("Performance profiler stopped");
        Ok(())
    }
}
