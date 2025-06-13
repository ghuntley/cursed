/// CURSED Compiler Optimization Infrastructure
/// 
/// This module provides comprehensive optimization infrastructure for the CURSED compiler,
/// including advanced optimization passes, compilation speed improvements, and performance
/// monitoring capabilities.

// Core optimization modules (implemented)
pub mod profiling;
pub mod jit_optimization;
pub mod memory_optimization;
pub mod compilation_speed;

// Advanced optimization modules (implemented)
pub mod optimization_levels;
pub mod llvm_passes;
pub mod llvm_advanced;
pub mod parallel_compilation;
pub mod cache;

// Additional optimization modules (stubs for future implementation)
pub mod incremental_compilation;
pub mod benchmarking;
pub mod adaptive;

// Re-export main optimization types and functions from implemented modules
pub use profiling::{
    PerformanceProfiler,
    CompilationProfiler,
    RuntimeProfiler,
    PerformanceMetrics,
    OptimizationRecommendation,
    CompilationMetrics,
    RuntimeMetrics,
};

pub use jit_optimization::{
    AdaptiveJitOptimizer,
    HotPathProfiler,
    ProfileGuidedOptimizer,
    JitOptimizationConfig,
    ProfileData,
    OptimizationBenefit,
};

pub use memory_optimization::{
    MemoryLayoutOptimizer,
    AllocationOptimizer,
    CacheOptimizer,
    MemoryOptimizationConfig,
    ObjectLayout,
    MemoryAccessPattern,
    MemoryPool,
};

pub use compilation_speed::{
    CompilationSpeedOptimizer,
    ParallelAstProcessor,
    TypeCheckingOptimizer,
    CompilationSpeedConfig,
    CompilationUnit,
    DependencyGraph,
    CompilationStatus,
};

// Advanced optimization modules (implemented)
pub use optimization_levels::{
    OptimizationLevel,
    LevelConfig,
    LevelConfigBuilder,
    OptimizationLevelManager,
};

pub use llvm_passes::{
    LlvmPassManager,
    PassStatistics,
    LtoManager,
    PgoManager,
    ProfileData,
};

pub use llvm_advanced::{
    AdvancedOptimizationManager,
    AdvancedOptimizationConfig,
    OptimizationStatistics,
    OptimizationPipeline,
    FunctionInliner,
    LoopOptimizer,
    DeadCodeEliminator,
    ConstantPropagator,
    CommonSubexpressionEliminator,
    TailCallOptimizer,
    MemoryOptimizer,
    OptimizationPass,
};

pub use parallel_compilation::{
    ParallelCompiler,
    ParallelCompilationConfig,
    WorkItem,
    WorkerState,
    CompilationResult,
    ParallelCompilationStatistics,
    WorkScheduler,
    LoadBalancingStrategy,
    ModuleCompiler,
};

pub use cache::{
    CacheManager,
    CacheConfig,
    CacheEntry,
    CacheEntryType,
    CacheStatistics,
    CacheValidationStrategy,
    DependencyTracker,
};

// Stub modules (for future implementation)
// pub use parallel_compilation::*;
// pub use incremental_compilation::*;
// pub use benchmarking::*;
// pub use cache::*;
// pub use adaptive::*;

/// Optimization configuration for different use cases
#[derive(Debug, Clone)]
pub struct OptimizationConfig {
    /// Enable LLVM advanced optimization passes
    pub enable_advanced_llvm: bool,
    /// Enable parallel compilation
    pub enable_parallel_compilation: bool,
    /// Enable incremental compilation
    pub enable_incremental_compilation: bool,
    /// Enable JIT optimization
    pub enable_jit_optimization: bool,
    /// Enable memory layout optimization
    pub enable_memory_optimization: bool,
    /// Enable performance profiling
    pub enable_profiling: bool,
    /// Enable optimization caching
    pub enable_caching: bool,
    /// Enable adaptive optimization
    pub enable_adaptive_optimization: bool,
    /// Maximum number of parallel threads
    pub max_parallel_threads: usize,
    /// Optimization level (0-3)
    pub optimization_level: u32,
    /// Target architecture
    pub target_arch: String,
    /// Enable debugging optimizations
    pub debug_optimizations: bool,
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            enable_advanced_llvm: true,
            enable_parallel_compilation: true,
            enable_incremental_compilation: true,
            enable_jit_optimization: true,
            enable_memory_optimization: true,
            enable_profiling: false, // Off by default for performance
            enable_caching: true,
            enable_adaptive_optimization: false, // Experimental
            max_parallel_threads: num_cpus::get().max(1),
            optimization_level: 2,
            target_arch: "native".to_string(),
            debug_optimizations: false,
        }
    }
}

/// Main optimization manager that coordinates all optimization features
pub struct OptimizationManager {
    config: OptimizationConfig,
    optimization_level_manager: OptimizationLevelManager,
    llvm_pass_manager: Option<LlvmPassManager<'static>>,
    llvm_optimizer: Option<AdvancedOptimizationManager>,
    speed_optimizer: Option<CompilationSpeedOptimizer>,
    jit_optimizer: Option<AdaptiveJitOptimizer>,
    memory_optimizer: Option<MemoryLayoutOptimizer>,
    parallel_compiler: Option<ParallelCompiler>,
    incremental_compiler: Option<incremental_compilation::IncrementalCompiler>,
    profiler: Option<PerformanceProfiler>,
    cache_manager: Option<CacheManager>,
    adaptive_optimizer: Option<adaptive::AdaptiveOptimizer>,
    lto_manager: Option<LtoManager>,
    pgo_manager: Option<PgoManager>,
}

impl OptimizationManager {
    /// Create a new optimization manager with the given configuration
    pub fn new(config: OptimizationConfig) -> crate::error::Result<Self> {
        let optimization_level = match config.optimization_level {
            0 => OptimizationLevel::None,
            1 => OptimizationLevel::Basic,
            2 => OptimizationLevel::Standard,
            3 => OptimizationLevel::Aggressive,
            _ => OptimizationLevel::Standard,
        };

        let optimization_level_manager = OptimizationLevelManager::new(optimization_level);

        let mut manager = Self {
            config: config.clone(),
            optimization_level_manager,
            llvm_pass_manager: None,
            llvm_optimizer: None,
            speed_optimizer: None,
            jit_optimizer: None,
            memory_optimizer: None,
            parallel_compiler: None,
            incremental_compiler: None,
            profiler: None,
            cache_manager: None,
            adaptive_optimizer: None,
            lto_manager: None,
            pgo_manager: None,
        };

        manager.initialize()?;
        Ok(manager)
    }

    /// Initialize optimization components based on configuration
    fn initialize(&mut self) -> crate::error::Result<()> {
        // Initialize LTO and PGO managers
        let level_config = self.optimization_level_manager.current_config().clone();
        
        if level_config.enable_lto {
            self.lto_manager = Some(LtoManager::new(level_config.clone()));
        }

        if level_config.enable_pgo {
            self.pgo_manager = Some(PgoManager::new(level_config.clone()));
        }

        if self.config.enable_advanced_llvm {
            self.llvm_optimizer = Some(AdvancedOptimizationManager::new(&self.config)?);
        }

        if self.config.enable_parallel_compilation {
            self.speed_optimizer = Some(CompilationSpeedOptimizer::new(&self.config)?);
            self.parallel_compiler = Some(ParallelCompiler::new(&self.config)?);
        }

        if self.config.enable_jit_optimization {
            self.jit_optimizer = Some(AdaptiveJitOptimizer::new(&self.config)?);
        }

        if self.config.enable_memory_optimization {
            self.memory_optimizer = Some(MemoryLayoutOptimizer::new(&self.config)?);
        }

        if self.config.enable_incremental_compilation {
            self.incremental_compiler = Some(incremental_compilation::IncrementalCompiler::new(&self.config)?);
        }

        if self.config.enable_profiling {
            self.profiler = Some(PerformanceProfiler::new(&self.config)?);
        }

        if self.config.enable_caching {
            self.cache_manager = Some(CacheManager::new(&self.config)?);
        }

        if self.config.enable_adaptive_optimization {
            self.adaptive_optimizer = Some(adaptive::AdaptiveOptimizer::new(&self.config)?);
        }

        Ok(())
    }

    /// Get configuration
    pub fn config(&self) -> &OptimizationConfig {
        &self.config
    }

    /// Update configuration and reinitialize if needed
    pub fn update_config(&mut self, config: OptimizationConfig) -> crate::error::Result<()> {
        self.config = config;
        self.initialize()
    }

    /// Get LLVM optimizer if available
    pub fn llvm_optimizer(&self) -> Option<&AdvancedOptimizationManager> {
        self.llvm_optimizer.as_ref()
    }

    /// Get speed optimizer if available
    pub fn speed_optimizer(&self) -> Option<&CompilationSpeedOptimizer> {
        self.speed_optimizer.as_ref()
    }

    /// Get JIT optimizer if available
    pub fn jit_optimizer(&self) -> Option<&AdaptiveJitOptimizer> {
        self.jit_optimizer.as_ref()
    }

    /// Get memory optimizer if available
    pub fn memory_optimizer(&self) -> Option<&MemoryLayoutOptimizer> {
        self.memory_optimizer.as_ref()
    }

    /// Get parallel compiler if available
    pub fn parallel_compiler(&self) -> Option<&parallel_compilation::ParallelCompiler> {
        self.parallel_compiler.as_ref()
    }

    /// Get incremental compiler if available
    pub fn incremental_compiler(&self) -> Option<&incremental_compilation::IncrementalCompiler> {
        self.incremental_compiler.as_ref()
    }

    /// Get profiler if available
    pub fn profiler(&self) -> Option<&PerformanceProfiler> {
        self.profiler.as_ref()
    }

    /// Get cache manager if available
    pub fn cache_manager(&self) -> Option<&cache::CacheManager> {
        self.cache_manager.as_ref()
    }

    /// Get adaptive optimizer if available
    pub fn adaptive_optimizer(&self) -> Option<&adaptive::AdaptiveOptimizer> {
        self.adaptive_optimizer.as_ref()
    }

    /// Get optimization level manager
    pub fn optimization_level_manager(&self) -> &OptimizationLevelManager {
        &self.optimization_level_manager
    }

    /// Get optimization level manager (mutable)
    pub fn optimization_level_manager_mut(&mut self) -> &mut OptimizationLevelManager {
        &mut self.optimization_level_manager
    }

    /// Get LTO manager if available
    pub fn lto_manager(&self) -> Option<&LtoManager> {
        self.lto_manager.as_ref()
    }

    /// Get PGO manager if available
    pub fn pgo_manager(&self) -> Option<&PgoManager> {
        self.pgo_manager.as_ref()
    }

    /// Get PGO manager (mutable) if available
    pub fn pgo_manager_mut(&mut self) -> Option<&mut PgoManager> {
        self.pgo_manager.as_mut()
    }

    /// Get LLVM pass manager if available
    pub fn llvm_pass_manager(&self) -> Option<&LlvmPassManager> {
        self.llvm_pass_manager.as_ref()
    }

    /// Set optimization level and reinitialize
    pub fn set_optimization_level(&mut self, level: OptimizationLevel) -> crate::error::Result<()> {
        self.optimization_level_manager.set_level(level);
        self.initialize()
    }

    /// Print comprehensive optimization summary
    pub fn print_comprehensive_summary(&self) {
        println!("🚀 CURSED Compiler Optimization System Summary");
        println!("=".repeat(50));
        
        // Optimization level summary
        self.optimization_level_manager.print_summary();
        
        // Component summaries
        if let Some(ref llvm_optimizer) = self.llvm_optimizer {
            println!();
            llvm_optimizer.print_summary();
        }
        
        if let Some(ref parallel_compiler) = self.parallel_compiler {
            println!();
            parallel_compiler.print_summary();
        }
        
        if let Some(ref cache_manager) = self.cache_manager {
            println!();
            cache_manager.print_summary();
        }
        
        if let Some(ref speed_optimizer) = self.speed_optimizer {
            println!();
            speed_optimizer.print_summary();
        }
        
        if let Some(ref jit_optimizer) = self.jit_optimizer {
            println!();
            jit_optimizer.print_summary();
        }
        
        if let Some(ref memory_optimizer) = self.memory_optimizer {
            println!();
            memory_optimizer.print_summary();
        }
        
        if let Some(ref profiler) = self.profiler {
            println!();
            profiler.print_summary();
        }
        
        println!("\n{}", "=".repeat(50));
    }
}
