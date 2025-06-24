use crate::error::Error;
// Profiling infrastructure for optimization analysis
// 
// Provides comprehensive profiling capabilities to measure performance
// characteristics and guide optimization decisions.

use std::time::{Duration, Instant};
use std::collections::HashMap;
use crate::error::Result;

/// Comprehensive profiling system for compilation and optimization
#[derive(Debug, Clone)]
pub struct ProfilingSystem {
    profiles: HashMap<String, Profile>,
    active_sessions: HashMap<String, ProfilingSession>,
    config: ProfilingConfig,
}

/// Configuration for profiling
#[derive(Debug, Clone)]
pub struct ProfilingConfig {
    pub enable_detailed_timing: bool,
    pub enable_memory_tracking: bool,
    pub enable_cpu_profiling: bool,
    pub sample_rate: Duration,
    pub max_profile_duration: Duration,
}

/// Individual profiling session
#[derive(Debug, Clone)]
pub struct ProfilingSession {
    pub name: String,
    pub start_time: Instant,
    pub samples: Vec<ProfileSample>,
    pub metadata: HashMap<String, String>,
}

/// A single profiling sample
#[derive(Debug, Clone)]
pub struct ProfileSample {
    pub timestamp: Instant,
    pub cpu_usage: f64,
    pub memory_usage: usize,
    pub operation: String,
    pub duration: Option<Duration>,
}

/// Complete profile data
#[derive(Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub total_duration: Duration,
    pub samples: Vec<ProfileSample>,
    pub statistics: ProfileStatistics,
    pub hotspots: Vec<Hotspot>,
}

/// Statistical analysis of profile data
#[derive(Debug, Clone)]
pub struct ProfileStatistics {
    pub total_samples: usize,
    pub average_cpu_usage: f64,
    pub peak_memory_usage: usize,
    pub average_memory_usage: usize,
    pub operation_counts: HashMap<String, usize>,
    pub operation_durations: HashMap<String, Duration>,
}

/// Performance hotspot identification
#[derive(Debug, Clone)]
pub struct Hotspot {
    pub operation: String,
    pub total_time: Duration,
    pub percentage_of_total: f64,
    pub sample_count: usize,
    pub optimization_opportunity: OptimizationOpportunity,
}

/// Optimization opportunity classification
#[derive(Debug, Clone, PartialEq)]
pub enum OptimizationOpportunity {
    High,
    Medium,
    Low,
    AlreadyOptimized,
}

/// Compilation phase profiling
#[derive(Debug, Clone)]
pub struct CompilationPhaseProfile {
    pub phase_name: String,
    pub duration: Duration,
    pub cpu_usage: f64,
    pub memory_delta: i64,
    pub optimizations_applied: usize,
    pub bottlenecks: Vec<String>,
}

impl ProfilingSystem {
    /// Creates a new profiling system
    pub fn new(config: ProfilingConfig) -> Self {
        Self {
            profiles: HashMap::new(),
            active_sessions: HashMap::new(),
            config,
        }
    }

    /// Starts a new profiling session
    pub fn start_session(&mut self, name: String) -> Result<()> {
        let session = ProfilingSession {
            name: name.clone(),
            start_time: Instant::now(),
            samples: Vec::new(),
            metadata: HashMap::new(),
        };
        
        self.active_sessions.insert(name, session);
        Ok(())
    }

    /// Adds a sample to an active session
    pub fn sample(&mut self, session_name: &str, operation: String) -> Result<()> {
        if let Some(session) = self.active_sessions.get_mut(session_name) {
            let sample = ProfileSample {
                timestamp: Instant::now(),
                cpu_usage: self.get_current_cpu_usage(),
                memory_usage: self.get_current_memory_usage(),
                operation,
                duration: None,
            };
            session.samples.push(sample);
        }
        Ok(())
    }

    /// Records the duration of a specific operation
    pub fn record_operation_duration(&mut self, session_name: &str, operation: String, duration: Duration) -> Result<()> {
        if let Some(session) = self.active_sessions.get_mut(session_name) {
            let sample = ProfileSample {
                timestamp: Instant::now(),
                cpu_usage: self.get_current_cpu_usage(),
                memory_usage: self.get_current_memory_usage(),
                operation,
                duration: Some(duration),
            };
            session.samples.push(sample);
        }
        Ok(())
    }

    /// Ends a profiling session and generates profile
    pub fn end_session(&mut self, session_name: &str) -> Result<Profile> {
        if let Some(session) = self.active_sessions.remove(session_name) {
            let total_duration = session.start_time.elapsed();
            let statistics = self.calculate_statistics(&session.samples);
            let hotspots = self.identify_hotspots(&session.samples, total_duration);

            let profile = Profile {
                name: session.name.clone(),
                total_duration,
                samples: session.samples,
                statistics,
                hotspots,
            };

            self.profiles.insert(session.name, profile.clone());
            Ok(profile)
        } else {
            Err(crate::error::Error::General(format!("No active session named '{}'", session_name)))
        }
    }

    /// Profiles a compilation phase
    pub fn profile_compilation_phase<F>(&mut self, phase_name: &str, f: F) -> Result<CompilationPhaseProfile>
    where
        F: FnOnce() -> Result<usize>, // Returns number of optimizations applied
    {
        let start_time = Instant::now();
        let start_memory = self.get_current_memory_usage();
        let start_cpu = self.get_current_cpu_usage();

        let optimizations_applied = f()?;

        let duration = start_time.elapsed();
        let end_memory = self.get_current_memory_usage();
        let end_cpu = self.get_current_cpu_usage();

        let profile = CompilationPhaseProfile {
            phase_name: phase_name.to_string(),
            duration,
            cpu_usage: (start_cpu + end_cpu) / 2.0,
            memory_delta: end_memory as i64 - start_memory as i64,
            optimizations_applied,
            bottlenecks: self.detect_phase_bottlenecks(phase_name),
        };

        Ok(profile)
    }

    fn calculate_statistics(&self, samples: &[ProfileSample]) -> ProfileStatistics {
        let mut operation_counts = HashMap::new();
        let mut operation_durations = HashMap::new();
        let mut total_cpu = 0.0;
        let mut total_memory = 0;
        let mut peak_memory = 0;

        for sample in samples {
            *operation_counts.entry(sample.operation.clone()).or_insert(0) += 1;
            
            if let Some(duration) = sample.duration {
                let total_duration = operation_durations.entry(sample.operation.clone()).or_insert(Duration::new(0, 0));
                *total_duration += duration;
            }

            total_cpu += sample.cpu_usage;
            total_memory += sample.memory_usage;
            peak_memory = peak_memory.max(sample.memory_usage);
        }

        ProfileStatistics {
            total_samples: samples.len(),
            average_cpu_usage: if samples.is_empty() { 0.0 } else { total_cpu / samples.len() as f64 },
            peak_memory_usage: peak_memory,
            average_memory_usage: if samples.is_empty() { 0 } else { total_memory / samples.len() },
            operation_counts,
            operation_durations,
        }
    }

    fn identify_hotspots(&self, samples: &[ProfileSample], total_duration: Duration) -> Vec<Hotspot> {
        let mut hotspots = Vec::new();
        let mut operation_times = HashMap::new();
        let mut operation_counts = HashMap::new();

        // Aggregate operation times
        for sample in samples {
            if let Some(duration) = sample.duration {
                let total_time = operation_times.entry(sample.operation.clone()).or_insert(Duration::new(0, 0));
                *total_time += duration;
                *operation_counts.entry(sample.operation.clone()).or_insert(0) += 1;
            }
        }

        // Create hotspots for operations taking >5% of total time
        for (operation, time) in operation_times {
            let percentage = (time.as_nanos() as f64 / total_duration.as_nanos() as f64) * 100.0;
            if percentage > 5.0 {
                let opportunity = if percentage > 20.0 {
                    OptimizationOpportunity::High
                } else if percentage > 10.0 {
                    OptimizationOpportunity::Medium
                } else {
                    OptimizationOpportunity::Low
                };

                hotspots.push(Hotspot {
                    operation: operation.clone(),
                    total_time: time,
                    percentage_of_total: percentage,
                    sample_count: operation_counts[&operation],
                    optimization_opportunity: opportunity,
                });
            }
        }

        // Sort by percentage of total time
        hotspots.sort_by(|a, b| b.percentage_of_total.partial_cmp(&a.percentage_of_total).unwrap());
        hotspots
    }

    fn detect_phase_bottlenecks(&self, phase_name: &str) -> Vec<String> {
        // Simulate bottleneck detection based on phase
        match phase_name {
            "parsing" => vec!["Token allocation".to_string(), "AST construction".to_string()],
            "optimization" => vec!["LLVM pass execution".to_string(), "Data flow analysis".to_string()],
            "codegen" => vec!["Instruction selection".to_string(), "Register allocation".to_string()],
            _ => vec!["Unknown bottleneck".to_string()],
        }
    }

    fn get_current_cpu_usage(&self) -> f64 {
        // Simulate CPU usage (in real implementation, would use system APIs)
        25.0 + (rand::random::<f64>() * 50.0)
    }

    fn get_current_memory_usage(&self) -> usize {
        // Simulate memory usage (in real implementation, would use system APIs)
        1024 * 1024 + (rand::random::<usize>() % (512 * 1024))
    }

    /// Gets a completed profile by name
    pub fn get_profile(&self, name: &str) -> Option<&Profile> {
        self.profiles.get(name)
    }

    /// Gets all completed profiles
    pub fn get_all_profiles(&self) -> Vec<&Profile> {
        self.profiles.values().collect()
    }

    /// Generates a summary report of all profiles
    pub fn generate_summary_report(&self) -> ProfileSummaryReport {
        let mut total_compilation_time = Duration::new(0, 0);
        let mut total_samples = 0;
        let mut all_hotspots = Vec::new();

        for profile in self.profiles.values() {
            total_compilation_time += profile.total_duration;
            total_samples += profile.statistics.total_samples;
            all_hotspots.extend(profile.hotspots.clone());
        }

        // Find top global hotspots
        all_hotspots.sort_by(|a, b| b.percentage_of_total.partial_cmp(&a.percentage_of_total).unwrap());
        let top_hotspots = all_hotspots.into_iter().take(10).collect();

        ProfileSummaryReport {
            total_profiles: self.profiles.len(),
            total_compilation_time,
            total_samples,
            top_hotspots,
            average_profile_duration: if self.profiles.is_empty() { 
                Duration::new(0, 0) 
            } else { 
                total_compilation_time / self.profiles.len() as u32 
            },
        }
    }
}

/// Summary report of all profiling data
#[derive(Debug, Clone)]
pub struct ProfileSummaryReport {
    pub total_profiles: usize,
    pub total_compilation_time: Duration,
    pub total_samples: usize,
    pub top_hotspots: Vec<Hotspot>,
    pub average_profile_duration: Duration,
}

/// CPU profiler for detailed CPU usage analysis
#[derive(Debug, Clone)]
pub struct CpuProfiler {
    pub sample_rate: Duration,
    pub track_per_thread: bool,
    pub collect_stack_traces: bool,
}

/// Memory profiler for allocation tracking
#[derive(Debug, Clone)]
pub struct MemoryProfiler {
    pub track_allocations: bool,
    pub track_deallocations: bool,
    pub collect_stack_traces: bool,
    pub heap_profiling: bool,
}

impl Default for ProfilingConfig {
    fn default() -> Self {
        Self {
            enable_detailed_timing: true,
            enable_memory_tracking: true,
            enable_cpu_profiling: true,
            sample_rate: Duration::from_millis(10),
            max_profile_duration: Duration::from_secs(300),
        }
    }
}

/// Convenience macro for profiling a block of code
#[macro_export]
macro_rules! profile_block {
    ($profiler:expr, $session:expr, $operation:expr, $block:block) => {
        {
            let start = std::time::Instant::now();
            let result = $block;
            let duration = start.elapsed();
            $profiler.record_operation_duration($session, $operation.to_string(), duration)?;
            result
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profiling_session() {
        let config = ProfilingConfig::default();
        let mut profiler = ProfilingSystem::new(config);
        
        profiler.start_session("test_session".to_string()).unwrap();
        profiler.sample("test_session", "operation1".to_string()).unwrap();
        profiler.record_operation_duration("test_session", "operation2".to_string(), Duration::from_millis(100)).unwrap();
        
        let profile = profiler.end_session("test_session").unwrap();
        assert_eq!(profile.statistics.total_samples, 2);
        assert!(!profile.statistics.operation_counts.is_empty());
    }

    #[test]
    fn test_compilation_phase_profiling() {
        let config = ProfilingConfig::default();
        let mut profiler = ProfilingSystem::new(config);
        
        let phase_profile = profiler.profile_compilation_phase("test_phase", || {
            std::thread::sleep(Duration::from_millis(10));
            Ok(5) // 5 optimizations applied
        }).unwrap();
        
        assert_eq!(phase_profile.phase_name, "test_phase");
        assert_eq!(phase_profile.optimizations_applied, 5);
        assert!(phase_profile.duration > Duration::from_millis(9));
    }
}
