//! Optimization level definitions

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationLevel {
    None,
    Debug,
    Release,
    Max,
}

impl Default for OptimizationLevel {
    fn default() -> Self {
        OptimizationLevel::None
    }
}
