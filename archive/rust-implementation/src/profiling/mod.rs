// Profiling module for CURSED
pub mod core;
pub mod performance;
pub mod memory;
pub mod cpu;
pub mod analysis;
pub mod benchmarking;
pub mod reporting;
pub mod concurrency;
pub mod integration;
pub mod visualization;
pub mod cli;
pub mod io;

// Re-export key types
pub use core::{CursedProfiler, ProfilerConfig, ProfilerMode, ProfileData, ProfilerError};
pub use performance::{PerformanceMonitor, CompilationPhase, ReportConfig, ReportFormat};
pub use memory::{MemoryProfiler, GcEvent, AllocationEvent, GcType, MemoryLeak};
pub use reporting::ReportGenerator;
pub use benchmarking::{BenchmarkSuite, BenchmarkConfig, BenchmarkResults};
