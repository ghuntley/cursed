//! Performance hooks module

use crate::error::CursedError;
use crate::performance::PerformanceConfig;

pub struct PerformanceHooks {
    config: PerformanceConfig,
}

impl PerformanceHooks {
    pub fn new(config: PerformanceConfig) -> Result<Self, CursedError> {
        Ok(Self { config })
    }

    pub fn start(&self) -> Result<(), CursedError> {
        println!("Performance hooks started");
        Ok(())
    }

    pub fn stop(&self) -> Result<(), CursedError> {
        println!("Performance hooks stopped");
        Ok(())
    }
}
