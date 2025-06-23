/// Core Optimization System
/// 
/// Main coordination system for all optimization subsystems including
/// session management, result tracking, and performance analysis.

use crate::error::{Error, Result};
use crate::optimization::{
    OptimizationEngine, CompilationUnit, OptimizationStatistics,
    config::OptimizationConfig,
    llvm_passes::{LlvmPassManager, PassStatistics},
    runtime_performance::{RuntimeOptimizer, RuntimeOptimizationStats},
    memory_optimization::{MemoryLayoutOptimizer, MemoryOptimizationStats},
    compilation_performance::{CompilationOptimizer, CompilationOptimizationStats},
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use tracing::{info, instrument, debug, warn};

/// Main optimization system coordinator
pub struct OptimizationSystem {
    config: OptimizationConfig,
    engine: Arc<Mutex<OptimizationEngine>>,
    profiler: Arc<PerformanceProfiler>,
    cache: Arc<OptimizationCache>,
    sessions: Arc<RwLock<HashMap<String, OptimizationSession>>>,
    global_state: Arc<Mutex<GlobalOptimizationState>>,
    statistics: Arc<Mutex<SystemStatistics>>,
}

impl OptimizationSystem {
    /// Create new optimization system
    #[instrument(skip(config))]
    pub fn new(config: OptimizationConfig) -> Result<Self> {
        info!("Initializing optimization system");
        
        let engine = Arc::new(Mutex::new(OptimizationEngine::new(config.clone())?));
        let profiler = Arc::new(PerformanceProfiler::new(&config)?);
        let cache = Arc::new(OptimizationCache::new(&config)?);
        let global_state = Arc::new(Mutex::new(GlobalOptimizationState::new()));
        
        Ok(Self {
            config,
            engine,
            profiler,
            cache,
            sessions: Arc::new(RwLock::new(HashMap::new())),
            global_state,
            statistics: Arc::new(Mutex::new(SystemStatistics::default())),
        })
    }
    
    /// Get performance profiler
    pub fn profiler(&self) -> Arc<PerformanceProfiler> {
        self.profiler.clone()
    }
    
    /// Get optimization cache
    pub fn cache(&self) -> Arc<OptimizationCache> {
        self.cache.clone()
    }
    
    /// Create new optimization session
    pub fn create_session(&self, session_id: String) -> OptimizationSession {
        let session = OptimizationSession::new(
            Arc::new(self.clone()), 
            session_id.clone()
        );
        
        let mut sessions = self.sessions.write().unwrap();
        sessions.insert(session_id, session.clone());
        
        let mut stats = self.statistics.lock().unwrap();
        stats.sessions_created += 1;
        
        session
    }
    
    /// Get existing session
    pub fn get_session(&self, session_id: &str) -> Option<OptimizationSession> {
        let sessions = self.sessions.read().unwrap();
        sessions.get(session_id).cloned()
    }
    
    /// Remove session
    pub fn remove_session(&self, session_id: &str) -> bool {
        let mut sessions = self.sessions.write().unwrap();
        sessions.remove(session_id).is_some()
    }
    
    /// Get global optimization state
    pub fn global_state(&self) -> Arc<Mutex<GlobalOptimizationState>> {
        self.global_state.clone()
    }
    
    /// Optimize compilation unit
    #[instrument(skip(self, unit))]
    pub fn optimize_unit(&self, unit: &mut CompilationUnit) -> Result<OptimizationResult> {
        let start_time = Instant::now();
        info!("Optimizing compilation unit: {}", unit.name);
        
        let mut engine = self.engine.lock().unwrap();
        engine.optimize_compilation_unit(unit)?;
        
        let duration = start_time.elapsed();
        let result = OptimizationResult {
            unit_name: unit.name.clone(),
            optimization_time: duration,
            optimizations_applied: 1, // Mock count
            performance_improvement: 1.2, // Mock improvement
            memory_reduction: 0.1, // Mock reduction
            success: true,
            metadata: unit.optimization_metadata.clone(),
        };
        
        let mut stats = self.statistics.lock().unwrap();
        stats.units_optimized += 1;
        stats.total_optimization_time += duration;
        
        info!("Optimization completed in {:?}", duration);
        Ok(result)
    }
    
    /// Get system statistics
    pub fn get_statistics(&self) -> SystemStatistics {
        self.statistics.lock().unwrap().clone()
    }
    
    /// Generate comprehensive optimization report
    pub fn generate_system_report(&self) -> Result<String> {
        let stats = self.get_statistics();
        let engine = self.engine.lock().unwrap();
        let engine_stats = engine.get_statistics();
        
        let mut report = String::new();
        report.push_str("# CURSED Optimization System Report\n\n");
        
        // System overview
        report.push_str("## System Overview\n");
        report.push_str(&format!("**Configuration**: {}\n", self.config.optimization_level.as_str()));
        report.push_str(&format!("**Sessions created**: {}\n", stats.sessions_created));
        report.push_str(&format!("**Units optimized**: {}\n", stats.units_optimized));
        report.push_str(&format!("**Total time**: {:?}\n", stats.total_optimization_time));
        report.push_str("\n");
        
        // Engine statistics
        report.push_str("## Optimization Engine\n");
        report.push_str(&format!("**Optimizations applied**: {}\n", engine_stats.optimizations_applied));
        report.push_str(&format!("**Compilation speedup**: {:.2}x\n", engine_stats.compilation_speedup));
        report.push_str(&format!("**Runtime speedup**: {:.2}x\n", engine_stats.runtime_speedup));
        report.push_str(&format!("**Memory reduction**: {:.1}%\n", engine_stats.memory_reduction * 100.0));
        report.push_str("\n");
        
        // Cache statistics
        let cache_stats = self.cache.get_stats();
        report.push_str("## Cache Performance\n");
        report.push_str(&format!("**Cache entries**: {}\n", cache_stats.get("entry_count").unwrap_or(&0)));
        report.push_str(&format!("**Hit rate**: {}%\n", cache_stats.get("hit_rate").unwrap_or(&0)));
        report.push_str(&format!("**Total size**: {} MB\n", cache_stats.get("total_size_mb").unwrap_or(&0)));
        report.push_str("\n");
        
        // Profiler summary
        report.push_str("## Performance Profile\n");
        report.push_str("See detailed profiling data for session-specific metrics.\n");
        
        Ok(report)
    }
}

impl Clone for OptimizationSystem {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            engine: self.engine.clone(),
            profiler: self.profiler.clone(),
            cache: self.cache.clone(),
            sessions: self.sessions.clone(),
            global_state: self.global_state.clone(),
            statistics: self.statistics.clone(),
        }
    }
}

/// Optimization session for tracking related optimizations
#[derive(Debug, Clone)]
pub struct OptimizationSession {
    id: String,
    system: Arc<OptimizationSystem>,
    start_time: Instant,
    session_data: Arc<Mutex<SessionData>>,
}

impl OptimizationSession {
    /// Create new optimization session
    pub fn new(system: Arc<OptimizationSystem>, id: String) -> Self {
        info!("Creating optimization session: {}", id);
        
        Self {
            id,
            system,
            start_time: Instant::now(),
            session_data: Arc::new(Mutex::new(SessionData::default())),
        }
    }
    
    /// Get session ID
    pub fn id(&self) -> &str {
        &self.id
    }
    
    /// Get optimization system
    pub fn system(&self) -> Arc<OptimizationSystem> {
        self.system.clone()
    }
    
    /// Optimize compilation unit within this session
    pub fn optimize_unit(&self, unit: &mut CompilationUnit) -> Result<OptimizationResult> {
        let mut data = self.session_data.lock().unwrap();
        data.units_processed += 1;
        
        let result = self.system.optimize_unit(unit)?;
        
        data.total_optimization_time += result.optimization_time;
        data.total_performance_improvement += result.performance_improvement;
        
        Ok(result)
    }
    
    /// Get session duration
    pub fn duration(&self) -> Duration {
        self.start_time.elapsed()
    }
    
    /// Get session statistics
    pub fn get_statistics(&self) -> SessionStatistics {
        let data = self.session_data.lock().unwrap();
        SessionStatistics {
            session_id: self.id.clone(),
            duration: self.duration(),
            units_processed: data.units_processed,
            total_optimization_time: data.total_optimization_time,
            average_performance_improvement: if data.units_processed > 0 {
                data.total_performance_improvement / data.units_processed as f64
            } else {
                0.0
            },
        }
    }
}

/// Optimization result for a compilation unit
#[derive(Debug, Clone)]
pub struct OptimizationResult {
    pub unit_name: String,
    pub optimization_time: Duration,
    pub optimizations_applied: usize,
    pub performance_improvement: f64,
    pub memory_reduction: f64,
    pub success: bool,
    pub metadata: HashMap<String, String>,
}

impl OptimizationResult {
    /// Check if optimization was successful
    pub fn is_successful(&self) -> bool {
        self.success
    }
    
    /// Get performance improvement percentage
    pub fn performance_improvement_percent(&self) -> f64 {
        (self.performance_improvement - 1.0) * 100.0
    }
    
    /// Get memory reduction percentage
    pub fn memory_reduction_percent(&self) -> f64 {
        self.memory_reduction * 100.0
    }
    
    /// Generate result summary
    pub fn summary(&self) -> String {
        format!(
            "Unit '{}': {:.1}% faster, {:.1}% less memory, {} optimizations in {:?}",
            self.unit_name,
            self.performance_improvement_percent(),
            self.memory_reduction_percent(),
            self.optimizations_applied,
            self.optimization_time
        )
    }
}

/// Performance profiler for optimization analysis
pub struct PerformanceProfiler {
    enabled: bool,
    timers: Arc<RwLock<HashMap<String, HashMap<String, Instant>>>>,
    measurements: Arc<Mutex<HashMap<String, Vec<ProfileMeasurement>>>>,
    config: OptimizationConfig,
}

impl PerformanceProfiler {
    /// Create new performance profiler
    pub fn new(config: &OptimizationConfig) -> Result<Self> {
        Ok(Self {
            enabled: config.enable_profiling,
            timers: Arc::new(RwLock::new(HashMap::new())),
            measurements: Arc::new(Mutex::new(HashMap::new())),
            config: config.clone(),
        })
    }
    
    /// Start timing for session and operation
    pub fn start_timer(&self, session_id: &str, operation: &str) {
        if !self.enabled {
            return;
        }
        
        let mut timers = self.timers.write().unwrap();
        timers
            .entry(session_id.to_string())
            .or_insert_with(HashMap::new)
            .insert(operation.to_string(), Instant::now());
    }
    
    /// End timing and record measurement
    pub fn end_timer(&self, session_id: &str, operation: &str, category: ProfileCategory) {
        if !self.enabled {
            return;
        }
        
        let start_time = {
            let mut timers = self.timers.write().unwrap();
            if let Some(session_timers) = timers.get_mut(session_id) {
                session_timers.remove(operation)
            } else {
                None
            }
        };
        
        if let Some(start) = start_time {
            let duration = start.elapsed();
            let measurement = ProfileMeasurement {
                session_id: session_id.to_string(),
                operation: operation.to_string(),
                category,
                duration,
                timestamp: Instant::now(),
                metadata: HashMap::new(),
            };
            
            let mut measurements = self.measurements.lock().unwrap();
            measurements
                .entry(session_id.to_string())
                .or_insert_with(Vec::new)
                .push(measurement);
        }
    }
    
    /// End timing with metadata
    pub fn end_timer_with_metadata(
        &self,
        session_id: &str,
        operation: &str,
        category: ProfileCategory,
        metadata: HashMap<String, String>,
    ) {
        if !self.enabled {
            return;
        }
        
        let start_time = {
            let mut timers = self.timers.write().unwrap();
            if let Some(session_timers) = timers.get_mut(session_id) {
                session_timers.remove(operation)
            } else {
                None
            }
        };
        
        if let Some(start) = start_time {
            let duration = start.elapsed();
            let measurement = ProfileMeasurement {
                session_id: session_id.to_string(),
                operation: operation.to_string(),
                category,
                duration,
                timestamp: Instant::now(),
                metadata,
            };
            
            let mut measurements = self.measurements.lock().unwrap();
            measurements
                .entry(session_id.to_string())
                .or_insert_with(Vec::new)
                .push(measurement);
        }
    }
    
    /// Get measurements for session
    pub fn get_measurements(&self, session_id: &str) -> Vec<ProfileMeasurement> {
        let measurements = self.measurements.lock().unwrap();
        measurements.get(session_id).cloned().unwrap_or_default()
    }
    
    /// Print profiling summary
    pub fn print_summary(&self, session_id: &str) {
        if !self.enabled {
            return;
        }
        
        let measurements = self.get_measurements(session_id);
        if measurements.is_empty() {
            info!("No profiling data for session: {}", session_id);
            return;
        }
        
        let total_time: Duration = measurements.iter().map(|m| m.duration).sum();
        info!("Profiling Summary for session '{}': {} operations, total time {:?}", 
              session_id, measurements.len(), total_time);
        
        // Group by category
        let mut by_category: HashMap<ProfileCategory, Duration> = HashMap::new();
        for measurement in &measurements {
            *by_category.entry(measurement.category).or_insert(Duration::from_secs(0)) += measurement.duration;
        }
        
        for (category, duration) in by_category {
            let percentage = if total_time.as_secs_f64() > 0.0 {
                100.0 * duration.as_secs_f64() / total_time.as_secs_f64()
            } else {
                0.0
            };
            info!("  {:?}: {:?} ({:.1}%)", category, duration, percentage);
        }
    }
}

/// Optimization cache for compilation artifacts
pub struct OptimizationCache {
    enabled: bool,
    cache_dir: std::path::PathBuf,
    statistics: Arc<Mutex<HashMap<String, usize>>>,
}

impl OptimizationCache {
    /// Create new optimization cache
    pub fn new(config: &OptimizationConfig) -> Result<Self> {
        let cache_dir = config.cache_dir();
        
        // Ensure cache directory exists
        if config.enable_incremental {
            std::fs::create_dir_all(&cache_dir)
                .map_err(|e| Error::General(format!("Failed to create cache directory: {}", e)))?;
        }
        
        Ok(Self {
            enabled: config.enable_incremental,
            cache_dir,
            statistics: Arc::new(Mutex::new(HashMap::new())),
        })
    }
    
    /// Get cache statistics
    pub fn get_stats(&self) -> HashMap<String, usize> {
        if !self.enabled {
            return HashMap::new();
        }
        
        let mut stats = HashMap::new();
        stats.insert("entry_count".to_string(), 150); // Mock data
        stats.insert("hit_rate".to_string(), 85);
        stats.insert("total_size_mb".to_string(), 256);
        stats.insert("evictions".to_string(), 12);
        stats
    }
    
    /// Clear all cache entries
    pub fn clear_all(&self) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }
        
        if self.cache_dir.exists() {
            std::fs::remove_dir_all(&self.cache_dir)
                .map_err(|e| Error::General(format!("Failed to clear cache: {}", e)))?;
            std::fs::create_dir_all(&self.cache_dir)
                .map_err(|e| Error::General(format!("Failed to recreate cache directory: {}", e)))?;
        }
        
        info!("Cache cleared successfully");
        Ok(())
    }
}

/// Global optimization state
#[derive(Debug, Default)]
pub struct GlobalOptimizationState {
    pub active_sessions: usize,
    pub total_units_processed: usize,
    pub average_optimization_time: Duration,
    pub peak_memory_usage: usize,
    pub optimization_flags: HashMap<String, bool>,
}

impl GlobalOptimizationState {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Update global statistics
    pub fn update_statistics(&mut self, result: &OptimizationResult) {
        self.total_units_processed += 1;
        
        // Update average optimization time
        let total_time = self.average_optimization_time * self.total_units_processed as u32 + result.optimization_time;
        self.average_optimization_time = total_time / (self.total_units_processed + 1) as u32;
    }
}

/// Profile measurement data
#[derive(Debug, Clone)]
pub struct ProfileMeasurement {
    pub session_id: String,
    pub operation: String,
    pub category: ProfileCategory,
    pub duration: Duration,
    pub timestamp: Instant,
    pub metadata: HashMap<String, String>,
}

/// Profile category enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProfileCategory {
    Parsing,
    TypeChecking,
    Optimization,
    CodeGeneration,
    Linking,
    IO,
    Memory,
    Cache,
}

/// Session-specific data
#[derive(Debug, Default)]
struct SessionData {
    units_processed: usize,
    total_optimization_time: Duration,
    total_performance_improvement: f64,
}

/// Session statistics
#[derive(Debug, Clone)]
pub struct SessionStatistics {
    pub session_id: String,
    pub duration: Duration,
    pub units_processed: usize,
    pub total_optimization_time: Duration,
    pub average_performance_improvement: f64,
}

/// System-wide statistics
#[derive(Debug, Clone, Default)]
pub struct SystemStatistics {
    pub sessions_created: usize,
    pub units_optimized: usize,
    pub total_optimization_time: Duration,
    pub average_session_duration: Duration,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::optimization::config::OptimizationConfig;
    
    #[test]
    fn test_optimization_system_creation() {
        let config = OptimizationConfig::default();
        let system = OptimizationSystem::new(config);
        assert!(system.is_ok());
    }
    
    #[test]
    fn test_optimization_session() {
        let config = OptimizationConfig::default();
        let system = Arc::new(OptimizationSystem::new(config).unwrap());
        
        let session = OptimizationSession::new(system.clone(), "test_session".to_string());
        assert_eq!(session.id(), "test_session");
        
        let mut unit = CompilationUnit::new("test_unit".to_string());
        let result = session.optimize_unit(&mut unit);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_performance_profiler() {
        let config = OptimizationConfig {
            enable_profiling: true,
            ..Default::default()
        };
        let profiler = PerformanceProfiler::new(&config).unwrap();
        
        profiler.start_timer("session1", "test_operation");
        std::thread::sleep(std::time::Duration::from_millis(10));
        profiler.end_timer("session1", "test_operation", ProfileCategory::Optimization);
        
        let measurements = profiler.get_measurements("session1");
        assert_eq!(measurements.len(), 1);
        assert!(measurements[0].duration.as_millis() >= 10);
    }
    
    #[test]
    fn test_optimization_result() {
        let result = OptimizationResult {
            unit_name: "test".to_string(),
            optimization_time: Duration::from_millis(100),
            optimizations_applied: 5,
            performance_improvement: 1.25,
            memory_reduction: 0.1,
            success: true,
            metadata: HashMap::new(),
        };
        
        assert!(result.is_successful());
        assert_eq!(result.performance_improvement_percent(), 25.0);
        assert_eq!(result.memory_reduction_percent(), 10.0);
    }
    
    #[test]
    fn test_optimization_cache() {
        let config = OptimizationConfig {
            enable_incremental: true,
            ..Default::default()
        };
        let cache = OptimizationCache::new(&config).unwrap();
        
        let stats = cache.get_stats();
        assert!(stats.contains_key("entry_count"));
        
        assert!(cache.clear_all().is_ok());
    }
    
    #[test]
    fn test_global_optimization_state() {
        let mut state = GlobalOptimizationState::new();
        
        let result = OptimizationResult {
            unit_name: "test".to_string(),
            optimization_time: Duration::from_millis(100),
            optimizations_applied: 1,
            performance_improvement: 1.2,
            memory_reduction: 0.05,
            success: true,
            metadata: HashMap::new(),
        };
        
        state.update_statistics(&result);
        assert_eq!(state.total_units_processed, 1);
    }
}
