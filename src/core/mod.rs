/// Core language functionality
pub mod type_checker;
pub mod char;
pub mod performance_pipeline;

pub use type_checker::{Type, TypeChecker};
pub use char::{CharMethods, CharObject};
pub use performance_pipeline::{PerformancePipeline, ParallelConfig, IncrementalConfig, ProgressConfig};
