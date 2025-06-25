// Common module for CURSED
use crate::error::CursedError;

// Core modules
pub mod optimization_level;

// Re-export optimization level from dedicated module
pub use optimization_level::OptimizationLevel;

// Basic error type for minimal build
#[derive(CursedError, Debug, Clone)]
pub enum MinimalError {
    #[error("Parse error: {0}")]
    #[error("Codegen error: {0}")]  
    #[error("Runtime error: {0}")]
    #[error("Not implemented: {0}")]
    #[error("IO error: {0}")]
// Use minimal error as CursedError for now

pub type CursedResult<T> = std::result::Result<T, crate::error::CursedError>;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
impl Default for Value {
    fn default() -> Self {
        Value::Nil
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
        }
    }
// Additional type definitions
pub type SourceLocation = (usize, usize); // (line, column)
pub type ReturnType = Value;
pub type ParameterType = Value;
pub type Literal = Value;
pub type CompilationPhase = String;
pub type Module = String;
pub type MemoryProfiler = ();
pub type PerformanceMonitor = ();
pub type DebugInfoManager = ();
pub type PackageMetadata = ();
pub type PackageManager = ();
pub type ConstraintResolver = ();
pub type ChannelError = String;
pub type BinaryOperator = String;
pub type UnaryOperator = String;
pub type Function = String;
pub type Program = String;
pub type InstructionValue = ();
pub type DatabaseFunction = ();
pub type DebugInfo = ();
pub type ReportFormat = String;
pub type ReportConfig = ();
pub type GcType = String;
pub type ReadlineError = String;
pub type ImportError = String;
