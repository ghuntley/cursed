// CURSED Profiling System
// Comprehensive performance analysis and optimization tools

pub mod core;
pub mod cpu;
pub mod memory;
pub mod concurrency;
pub mod io;
pub mod benchmarking;
pub mod analysis;
pub mod visualization;
pub mod reporting;
pub mod cli;
pub mod integration;

pub use core::*;
pub use cpu::*;
pub use memory::*;
pub use concurrency::*;
pub use io::*;
pub use benchmarking::*;
pub use analysis::*;
pub use visualization::*;
pub use reporting::*;
pub use cli::*;
pub use integration::*;

/// Re-export commonly used profiling types and functions
pub use crate::profiling::{
    ProfilerBuilder, ProfilerConfig, ProfilerMode,
    ProfilingSession, ProfileData, PerformanceReport,
    BenchmarkSuite, BenchmarkResult, FlameGraph,
    MemoryProfileData, ConcurrencyProfileData, IoProfileData,
};
