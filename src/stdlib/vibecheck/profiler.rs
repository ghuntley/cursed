/// Main Profiler Interface for CURSED vibecheck
/// 
/// Provides a unified interface for memory and CPU profiling with compiler integration,
/// runtime profiling capabilities, and export functionality.

use crate::error::CursedError;
// Placeholder imports disabled
// };

use std::sync::{Arc, Mutex, RwLock, atomic::{AtomicBool, Ordering}};
use std::time::{Duration, Instant};
use std::thread;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Comprehensive profiler configuration
#[derive(Debug, Clone)]
pub struct ProfilerConfig {
    /// Memory profiler configuration
    /// CPU profiler configuration
    /// Enable automatic profiling
    /// Profile session name
    /// Target application name
    /// Export directory
    /// Export formats
    /// Enable real-time profiling
    /// Real-time update interval
impl Default for ProfilerConfig {
    fn default() -> Self {
        Self {
            export_dir: "./profiles".to_string(),
        }
    }
/// Profiling session state
#[derive(Debug, Clone, PartialEq)]
pub enum ProfilerState {
    /// Profiler is stopped
    /// Profiler is starting up
    /// Profiler is running
    /// Profiler is stopping
    /// Profiler encountered an error
/// Profiling statistics
#[derive(Debug, Clone)]
pub struct ProfilingStats {
    /// Memory profiling enabled
    /// CPU profiling enabled
    /// Profiling start time
    /// Current profiling duration
    /// Number of memory samples collected
    /// Number of CPU samples collected
    /// Current memory usage
    /// Peak memory usage
    /// Estimated overhead percentage
/// Main profiler interface
pub struct Profiler {
/// Profiler event hooks
#[derive(Debug, Default)]
struct ProfilerHooks {
    /// Called when profiling starts
    /// Called when profiling stops
    /// Called on real-time updates
    /// Called on memory allocation
    /// Called on function entry/exit
impl Profiler {
    /// Create a new profiler with default configuration
    pub fn new() -> Self {
        Self::with_config(ProfilerConfig::default())
    /// Create a new profiler with custom configuration
    pub fn with_config(config: ProfilerConfig) -> Self {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs());

        let profile_data = ProfileData::new(session_id, config.target_name.clone());

        Self {
        }
    }

    /// Start profiling
    pub fn start(&self) -> crate::error::Result<()> {
        let mut state = self.state.write()
            .map_err(|_| CursedError::Runtime("Failed to lock profiler state".to_string()))?;

        if *state != ProfilerState::Stopped {
            return Err(CursedError::Runtime("Profiler is already running or starting".to_string()));
        *state = ProfilerState::Starting;
        drop(state);

        // Configure and start memory profiler
        if self.config.memory.sample_rate > 0 {
            memory_profiler::configure_memory_profiler(self.config.memory.clone())?;
            self.is_memory_enabled.store(true, Ordering::SeqCst);
        // Configure and start CPU profiler
        if self.config.cpu.sample_rate > 0 {
            cpu_profiler::configure_cpu_profiler(self.config.cpu.clone())?;
            cpu_profiler::start_cpu_profiling()?;
            self.is_cpu_enabled.store(true, Ordering::SeqCst);
        // Initialize profiling statistics
        {
            let mut stats = self.stats.lock()
                .map_err(|_| CursedError::Runtime("Failed to lock profiler stats".to_string()))?;
            stats.start_time = Some(Instant::now());
            stats.memory_enabled = self.is_memory_enabled.load(Ordering::SeqCst);
            stats.cpu_enabled = self.is_cpu_enabled.load(Ordering::SeqCst);
        // Start real-time monitoring if enabled
        if self.config.real_time {
            self.start_real_time_monitoring()?;
        // Call start hook
        {
            let hooks = self.hooks.lock()
                .map_err(|_| CursedError::Runtime("Failed to lock profiler hooks".to_string()))?;
            if let Some(ref on_start) = hooks.on_start {
                on_start();
            }
        }

        // Update state to running
        {
            let mut state = self.state.write()
                .map_err(|_| CursedError::Runtime("Failed to lock profiler state".to_string()))?;
            *state = ProfilerState::Running;
        Ok(())
    /// Stop profiling and return results
    pub fn stop(&self) -> crate::error::Result<()> {
        let mut state = self.state.write()
            .map_err(|_| CursedError::Runtime("Failed to lock profiler state".to_string()))?;

        if *state != ProfilerState::Running {
            return Err(CursedError::Runtime("Profiler is not running".to_string()));
        *state = ProfilerState::Stopping;
        drop(state);

        // Stop real-time monitoring
        self.stop_real_time_monitoring()?;

        // Collect final profiling data
        let mut final_profile_data = {
            let profile_data = self.profile_data.lock()
                .map_err(|_| CursedError::Runtime("Failed to lock profile data".to_string()))?;
            profile_data.clone()

        // Collect memory statistics
        if self.is_memory_enabled.load(Ordering::SeqCst) {
            match memory_profiler::memory_profile() {
                Ok(memory_stats) => {
                    final_profile_data.set_memory_data(&memory_stats);
                }
                Err(e) => {
                    eprintln!("Warning: Failed to collect memory statistics: {}", e);
                }
            }
        // Stop CPU profiler and collect statistics
        if self.is_cpu_enabled.load(Ordering::SeqCst) {
            match cpu_profiler::stop_cpu_profiling() {
                Ok(cpu_profile) => {
                    final_profile_data.set_cpu_data(&cpu_profile);
                }
                Err(e) => {
                    eprintln!("Warning: Failed to collect CPU statistics: {}", e);
                }
            }
            self.is_cpu_enabled.store(false, Ordering::SeqCst);
        // Finalize profile data
        final_profile_data.finalize();

        // Export profiles if configured
        if !self.config.export_formats.is_empty() {
            if let Err(e) = self.export_profiles(&final_profile_data) {
                eprintln!("Warning: Failed to export profiles: {}", e);
            }
        }

        // Call stop hook
        {
            let hooks = self.hooks.lock()
                .map_err(|_| CursedError::Runtime("Failed to lock profiler hooks".to_string()))?;
            if let Some(ref on_stop) = hooks.on_stop {
                on_stop(&final_profile_data);
            }
        }

        // Reset state
        self.is_memory_enabled.store(false, Ordering::SeqCst);
        
        {
            let mut state = self.state.write()
                .map_err(|_| CursedError::Runtime("Failed to lock profiler state".to_string()))?;
            *state = ProfilerState::Stopped;
        Ok(final_profile_data)
    /// Get current profiling state
    pub fn get_state(&self) -> crate::error::Result<()> {
        let state = self.state.read()
            .map_err(|_| CursedError::Runtime("Failed to lock profiler state".to_string()))?;
        Ok(state.clone())
    /// Get current profiling statistics
    pub fn get_stats(&self) -> crate::error::Result<()> {
        let mut stats = self.stats.lock()
            .map_err(|_| CursedError::Runtime("Failed to lock profiler stats".to_string()))?;

        // Update duration
        if let Some(start_time) = stats.start_time {
            stats.duration = start_time.elapsed();
        // Update memory statistics
        if self.is_memory_enabled.load(Ordering::SeqCst) {
            if let Ok(memory_stats) = memory_profiler::memory_profile() {
                stats.current_memory_kb = memory_stats.heap_analysis.current_allocated / 1024;
                stats.peak_memory_kb = memory_stats.heap_analysis.peak_allocated / 1024;
                stats.memory_samples = memory_stats.heap_analysis.active_allocations as u64;
            }
        }

        // Estimate overhead (simplified)
        stats.overhead_percentage = self.estimate_overhead();

        Ok(stats.clone())
    /// Record memory allocation (called by allocator hooks)
    pub fn record_allocation(&self, address: usize, size: usize, object_type: Option<String>) -> crate::error::Result<()> {
        if !self.is_memory_enabled.load(Ordering::SeqCst) {
            return Ok(());
        memory_profiler::profile_allocation(address, size, object_type.clone())?;

        // Call memory event hook
        {
            let hooks = self.hooks.lock()
                .map_err(|_| CursedError::Runtime("Failed to lock profiler hooks".to_string()))?;
            if let Some(ref on_memory_event) = hooks.on_memory_event {
                on_memory_event(size, true);
            }
        }

        Ok(())
    /// Record memory deallocation (called by allocator hooks)
    pub fn record_deallocation(&self, address: usize) -> crate::error::Result<()> {
        if !self.is_memory_enabled.load(Ordering::SeqCst) {
            return Ok(());
        memory_profiler::profile_deallocation(address)?;

        // Call memory event hook
        {
            let hooks = self.hooks.lock()
                .map_err(|_| CursedError::Runtime("Failed to lock profiler hooks".to_string()))?;
            if let Some(ref on_memory_event) = hooks.on_memory_event {
                on_memory_event(0, false); // Size unknown for deallocation
            }
        }

        Ok(())
    /// Record function entry (called by instrumentation)
    pub fn record_function_entry(&self, name: String, module: String) -> crate::error::Result<()> {
        if !self.is_cpu_enabled.load(Ordering::SeqCst) {
            return Ok(());
        cpu_profiler::profile_function_enter(name.clone(), module)?;

        // Call CPU event hook
        {
            let hooks = self.hooks.lock()
                .map_err(|_| CursedError::Runtime("Failed to lock profiler hooks".to_string()))?;
            if let Some(ref on_cpu_event) = hooks.on_cpu_event {
                on_cpu_event(&name, true);
            }
        }

        Ok(())
    /// Record function exit (called by instrumentation)
    pub fn record_function_exit(&self) -> crate::error::Result<()> {
        if !self.is_cpu_enabled.load(Ordering::SeqCst) {
            return Ok(());
        cpu_profiler::profile_function_exit()?;

        // Call CPU event hook
        {
            let hooks = self.hooks.lock()
                .map_err(|_| CursedError::Runtime("Failed to lock profiler hooks".to_string()))?;
            if let Some(ref on_cpu_event) = hooks.on_cpu_event {
                on_cpu_event("", false);
            }
        }

        Ok(())
    /// Add custom metric
    pub fn add_custom_metric(&self, name: String, value: MetricValue) -> crate::error::Result<()> {
        let mut profile_data = self.profile_data.lock()
            .map_err(|_| CursedError::Runtime("Failed to lock profile data".to_string()))?;
        profile_data.add_custom_metric(name, value);
        Ok(())
    /// Generate profile report
    pub fn generate_report(&self, config: ProfileReportConfig) -> crate::error::Result<()> {
        let profile_data = self.profile_data.lock()
            .map_err(|_| CursedError::Runtime("Failed to lock profile data".to_string()))?;
        
        let report = profile_data.create_report(config);
        report.generate()
    /// Set profiler hooks
    ) -> crate::error::Result<()> {
        let mut hooks = self.hooks.lock()
            .map_err(|_| CursedError::Runtime("Failed to lock profiler hooks".to_string()))?;
        
        hooks.on_start = on_start;
        hooks.on_stop = on_stop;
        hooks.on_update = on_update;
        
        Ok(())
    /// Start real-time monitoring thread
    fn start_real_time_monitoring(&self) -> crate::error::Result<()> {
        let stats_clone = self.stats.clone();
        let hooks_clone = self.hooks.clone();
        let update_interval = self.config.update_interval;
        let state_clone = self.state.clone();

        let monitoring_thread = thread::spawn(move || {
            while let Ok(state) = state_clone.read() {
                if *state != ProfilerState::Running {
                    break;
                }
                drop(state);

                if let Ok(stats) = stats_clone.lock() {
                    if let Ok(hooks) = hooks_clone.lock() {
                        if let Some(ref on_update) = hooks.on_update {
                            on_update(&stats);
                        }
                    }
                thread::sleep(update_interval);
            }
        });

        let mut real_time_thread = self.real_time_thread.lock()
            .map_err(|_| CursedError::Runtime("Failed to lock real-time thread".to_string()))?;
        *real_time_thread = Some(monitoring_thread);

        Ok(())
    /// Stop real-time monitoring thread
    fn stop_real_time_monitoring(&self) -> crate::error::Result<()> {
        let mut real_time_thread = self.real_time_thread.lock()
            .map_err(|_| CursedError::Runtime("Failed to lock real-time thread".to_string()))?;

        if let Some(thread_handle) = real_time_thread.take() {
            let _ = thread_handle.join();
        Ok(())
    /// Export profiles to configured formats
    fn export_profiles(&self, profile_data: &ProfileData) -> crate::error::Result<()> {
        // Create export directory if it doesn't exist
        fs::create_dir_all(&self.config.export_dir)
            .map_err(|e| CursedError::Runtime(format!("Failed to create export directory: {}", e)))?;

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        for format in &self.config.export_formats {
            let filename = match format {

            let filepath = Path::new(&self.config.export_dir).join(filename);
            
            let mut config = ProfileReportConfig::default();
            config.format = format.clone();
            
            let report = profile_data.create_report(config);
            let content = report.generate()?;

            fs::write(&filepath, content)
                .map_err(|e| CursedError::Runtime(format!("Failed to write profile report: {}", e)))?;
        Ok(())
    /// Estimate profiling overhead
    fn estimate_overhead(&self) -> f64 {
        let mut overhead = 0.0;

        if self.is_memory_enabled.load(Ordering::SeqCst) {
            overhead += 2.0; // ~2% for memory profiling
        if self.is_cpu_enabled.load(Ordering::SeqCst) {
            overhead += 5.0; // ~5% for CPU profiling
        if self.config.real_time {
            overhead += 1.0; // ~1% for real-time monitoring
        overhead
    }
}

impl Default for ProfilingStats {
    fn default() -> Self {
        Self {
        }
    }
/// Global profiler instance
static GLOBAL_PROFILER: std::sync::OnceLock<Arc<Profiler>> = std::sync::OnceLock::new();

/// Get or create the global profiler
pub fn get_profiler() -> Arc<Profiler> {
    GLOBAL_PROFILER.get_or_init(|| {
        Arc::new(Profiler::new())
    }).clone()
/// Configure the global profiler
pub fn configure_profiler(config: ProfilerConfig) -> crate::error::Result<()> {
    let profiler = Arc::new(Profiler::with_config(config));
    GLOBAL_PROFILER.set(profiler)
        .map_err(|_| CursedError::Runtime("Profiler already configured".to_string()))?;
    Ok(())
/// Start global profiling
pub fn start_profiling() -> crate::error::Result<()> {
    let profiler = get_profiler();
    profiler.start()
/// Stop global profiling
pub fn stop_profiling() -> crate::error::Result<()> {
    let profiler = get_profiler();
    profiler.stop()
/// Get current profiling statistics
pub fn profiling_stats() -> crate::error::Result<()> {
    let profiler = get_profiler();
    profiler.get_stats()
/// Generate a profiling report
pub fn generate_profiling_report(config: ProfileReportConfig) -> crate::error::Result<()> {
    let profiler = get_profiler();
    profiler.generate_report(config)
/// Quick profiling function for RAII-style profiling
pub struct ProfileScope {
impl ProfileScope {
    /// Create a new profile scope that starts profiling
    pub fn new() -> crate::error::Result<()> {
        let profiler = get_profiler();
        profiler.start()?;
        Ok(Self { profiler })
    /// Create a new profile scope with custom configuration
    pub fn with_config(config: ProfilerConfig) -> crate::error::Result<()> {
        configure_profiler(config)?;
        Self::new()
    }
}

impl Drop for ProfileScope {
    fn drop(&mut self) {
        if let Err(e) = self.profiler.stop() {
            eprintln!("Warning: Failed to stop profiling: {}", e);
        }
    }
/// Macro for easy function profiling
#[macro_export]
macro_rules! profile_scope {
    () => {
//         let _profile_scope = $crate::stdlib::vibecheck::profiler::ProfileScope::new()?;
    ($config:expr) => {
//         let _profile_scope = $crate::stdlib::vibecheck::profiler::ProfileScope::with_config($config)?;
