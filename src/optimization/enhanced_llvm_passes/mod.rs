use crate::error::Error;
/// Enhanced LLVM Passes Module
/// 
/// This module contains specialized optimizers for CURSED-specific constructs
/// and advanced LLVM optimization passes.

pub mod function_specializer;
pub mod goroutine_optimizer;
pub mod real_goroutine_optimizer;
pub mod channel_optimizer;
pub mod memory_layout_optimizer;
pub mod gen_z_slang_optimizer;
pub mod error_propagation_optimizer;
pub mod interprocedural_analyzer;
pub mod vectorization_optimizer;
pub mod cache_optimizer;
pub mod branch_predictor;

pub use function_specializer::FunctionSpecializer;
pub use goroutine_optimizer::GoroutineOptimizer;
pub use real_goroutine_optimizer::RealGoroutineOptimizer;
pub use channel_optimizer::ChannelOptimizer;
pub use memory_layout_optimizer::MemoryLayoutOptimizer;
pub use gen_z_slang_optimizer::GenZSlangOptimizer;
pub use error_propagation_optimizer::ErrorPropagationOptimizer;
pub use interprocedural_analyzer::InterproceduralAnalyzer;
pub use vectorization_optimizer::VectorizationOptimizer;
pub use cache_optimizer::CacheOptimizer;
pub use branch_predictor::BranchPredictor;
