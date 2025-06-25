//! Optimization configuration

#[derive(Debug, Clone)]
pub struct OptimizationConfig {
    pub level: u8,
    pub debug: bool,
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        OptimizationConfig {
            level: 2,
            debug: false,
        }
    }
}
