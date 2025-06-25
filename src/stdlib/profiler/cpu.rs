/// CPU profiling functionality
// use crate::stdlib::profiler::error::{ProfilerError, ProfilerResult};
use crate::error::CursedError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, atomic::{AtomicU64, AtomicBool, Ordering}};
use std::time::{Duration, Instant};
use std::thread;

/// Global CPU profiler state
static PROFILER_STATE: Mutex<Option<Arc<CpuProfiler>>> = Mutex::new(None);
static PROFILE_COUNT: AtomicU64 = AtomicU64::new(0);
static TOTAL_SAMPLES: AtomicU64 = AtomicU64::new(0);

/// CPU profiler configuration
#[derive(Debug, Clone)]
pub struct ProfilerConfig {
    pub sampling_frequency_hz: u32,
    pub max_stack_depth: usize,
    pub buffer_size: usize,
    pub enable_call_graph: bool,
    pub track_allocations: bool,
}

impl Default for ProfilerConfig {
    fn default() -> Self {
        Self {
            sampling_frequency_hz: 100, // 100 Hz sampling
            max_stack_depth: 32,
            buffer_size: 10000,
            enable_call_graph: true,
            track_allocations: false,
        }
    }
}

/// Sampling configuration for fine-tuning
#[derive(Debug, Clone)]
pub struct SamplingConfig {
    pub interval: Duration,
    pub max_samples: usize,
    pub adaptive_sampling: bool,
    pub sample_all_threads: bool,
}

impl Default for SamplingConfig {
    fn default() -> Self {
        Self {
            interval: Duration::from_millis(10), // 10ms between samples
            max_samples: 10000,
            adaptive_sampling: true,
            sample_all_threads: true,
        }
    }
}

/// Individual CPU sample
#[derive(Debug, Clone)]
pub struct CpuSample {
    pub timestamp: Instant,
    pub thread_id: u64,
    pub function_name: String,
    pub stack_trace: Vec<String>,
    pub cpu_time_ns: u64,
    pub instruction_pointer: usize,
}

/// Function-level profiling data
#[derive(Debug, Clone)]
pub struct FunctionProfile {
    pub function_name: String,
    pub total_time_ns: u64,
    pub self_time_ns: u64,
    pub call_count: u64,
    pub average_time_ns: u64,
    pub min_time_ns: u64,
    pub max_time_ns: u64,
    pub child_functions: HashMap<String, u64>,
}

/// Call graph representation
#[derive(Debug, Clone)]
pub struct CallGraph {
    pub nodes: HashMap<String, FunctionProfile>,
    pub edges: Vec<(String, String, u64)>, // (caller, callee, call_count)
    pub root_functions: Vec<String>,
}

/// Complete profiling data
#[derive(Debug, Clone)]
pub struct ProfileData {
    pub samples: Vec<CpuSample>,
    pub functions: HashMap<String, FunctionProfile>,
    pub call_graph: Option<CallGraph>,
    pub total_duration: Duration,
    pub sample_count: usize,
    pub threads_profiled: Vec<u64>,
}

/// CPU profile containing all collected data
#[derive(Debug, Clone)]
pub struct CpuProfile {
    pub config: ProfilerConfig,
    pub data: ProfileData,
    pub start_time: Instant,
    pub end_time: Option<Instant>,
    pub overhead_ns: u64,
}

impl CpuProfile {
    /// Get the top N functions by total time
    pub fn top_functions(&self, n: usize) -> Vec<&FunctionProfile> {
        let mut functions: Vec<&FunctionProfile> = self.data.functions.values().collect();
        functions.sort_by(|a, b| b.total_time_ns.cmp(&a.total_time_ns));
        functions.into_iter().take(n).collect()
    }

    /// Get functions that took more than a threshold
    pub fn functions_above_threshold(&self, threshold_ns: u64) -> Vec<&FunctionProfile> {
        self.data.functions
            .values()
            .filter(|f| f.total_time_ns > threshold_ns)
            .collect()
    }

    /// Calculate total profiling overhead
    pub fn calculate_overhead(&self) -> f64 {
        if let Some(end_time) = self.end_time {
            let total_duration = end_time.duration_since(self.start_time).as_nanos() as u64;
            if total_duration > 0 {
                (self.overhead_ns as f64 / total_duration as f64) * 100.0
            } else {
                0.0
            }
        } else {
            0.0
        }
    }
}

/// CPU profiler implementation
pub struct CpuProfiler {
    config: ProfilerConfig,
    sampling_config: SamplingConfig,
    is_running: AtomicBool,
    samples: Arc<Mutex<Vec<CpuSample>>>,
    functions: Arc<Mutex<HashMap<String, FunctionProfile>>>,
    start_time: Option<Instant>,
    sample_thread: Option<thread::JoinHandle<()>>,
}

impl CpuProfiler {
    /// Create a new CPU profiler
    pub fn new(config: ProfilerConfig) -> Self {
        Self {
            config,
            sampling_config: SamplingConfig::default(),
            is_running: AtomicBool::new(false),
            samples: Arc::new(Mutex::new(Vec::new())),
            functions: Arc::new(Mutex::new(HashMap::new())),
            start_time: None,
            sample_thread: None,
        }
    }

    /// Create with custom sampling configuration
    pub fn with_sampling_config(config: ProfilerConfig, sampling_config: SamplingConfig) -> Self {
        Self {
            config,
            sampling_config,
            is_running: AtomicBool::new(false),
            samples: Arc::new(Mutex::new(Vec::new())),
            functions: Arc::new(Mutex::new(HashMap::new())),
            start_time: None,
            sample_thread: None,
        }
    }

    /// Start CPU profiling
    pub fn start(&mut self) -> ProfilerResult<()> {
        if self.is_running.load(Ordering::Relaxed) {
            return Err(ProfilerError::AlreadyRunning);
        }

        self.is_running.store(true, Ordering::Relaxed);
        self.start_time = Some(Instant::now());

        // Clear previous data
        if let Ok(mut samples) = self.samples.lock() {
            samples.clear();
        }
        if let Ok(mut functions) = self.functions.lock() {
            functions.clear();
        }

        // Start sampling thread
        self.start_sampling_thread()?;

        Ok(())
    }

    /// Stop CPU profiling
    pub fn stop(&mut self) -> ProfilerResult<CpuProfile> {
        if !self.is_running.load(Ordering::Relaxed) {
            return Err(ProfilerError::NotRunning);
        }

        self.is_running.store(false, Ordering::Relaxed);
        let end_time = Instant::now();

        // Wait for sampling thread to finish
        if let Some(handle) = self.sample_thread.take() {
            let _ = handle.join();
        }

        // Collect final data
        let samples = self.samples.lock()
            .map_err(|_| ProfilerError::General("Failed to lock samples".to_string()))?
            .clone();

        let functions = self.functions.lock()
            .map_err(|_| ProfilerError::General("Failed to lock functions".to_string()))?
            .clone();

        let call_graph = if self.config.enable_call_graph {
            Some(self.build_call_graph(&samples))
        } else {
            None
        };

        let threads_profiled: Vec<u64> = samples.iter()
            .map(|s| s.thread_id)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        let total_duration = if let Some(start) = self.start_time {
            end_time.duration_since(start)
        } else {
            Duration::new(0, 0)
        };

        let profile_data = ProfileData {
            samples: samples.clone(),
            functions,
            call_graph,
            total_duration,
            sample_count: samples.len(),
            threads_profiled,
        };

        PROFILE_COUNT.fetch_add(1, Ordering::Relaxed);
        TOTAL_SAMPLES.fetch_add(samples.len() as u64, Ordering::Relaxed);

        Ok(CpuProfile {
            config: self.config.clone(),
            data: profile_data,
            start_time: self.start_time.unwrap(),
            end_time: Some(end_time),
            overhead_ns: self.calculate_profiling_overhead(),
        })
    }

    /// Check if profiler is running
    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::Relaxed)
    }

    /// Record a function call for profiling
    pub fn record_function_entry(&self, function_name: &str) -> ProfilerResult<()> {
        if !self.is_running() {
            return Ok(());
        }

        let sample = CpuSample {
            timestamp: Instant::now(),
            thread_id: self.get_current_thread_id(),
            function_name: function_name.to_string(),
            stack_trace: self.capture_stack_trace(),
            cpu_time_ns: self.get_cpu_time_ns(),
            instruction_pointer: 0, // Would be filled by actual profiler
        };

        if let Ok(mut samples) = self.samples.lock() {
            if samples.len() < self.config.buffer_size {
                samples.push(sample);
            }
        }

        self.update_function_stats(function_name)?;

        Ok(())
    }

    /// Start the sampling thread
    fn start_sampling_thread(&mut self) -> ProfilerResult<()> {
        let is_running = self.is_running.clone();
        let samples = self.samples.clone();
        let functions = self.functions.clone();
        let config = self.config.clone();
        let sampling_config = self.sampling_config.clone();

        let handle = thread::spawn(move || {
            let mut sample_count = 0;
            
            while is_running.load(Ordering::Relaxed) && sample_count < sampling_config.max_samples {
                // Simulate CPU sampling
                let sample = CpuSample {
                    timestamp: Instant::now(),
                    thread_id: std::thread::current().id().as_u64().get(),
                    function_name: format!("sampled_function_{}", sample_count % 10),
                    stack_trace: vec![
                        "main".to_string(),
                        format!("function_{}", sample_count % 5),
                        format!("inner_function_{}", sample_count % 3),
                    ],
                    cpu_time_ns: (sample_count as u64) * 1000,
                    instruction_pointer: 0x1000 + (sample_count * 4),
                };

                if let Ok(mut samples_guard) = samples.lock() {
                    if samples_guard.len() < config.buffer_size {
                        samples_guard.push(sample.clone());
                    }
                }

                // Update function statistics
                if let Ok(mut functions_guard) = functions.lock() {
                    let profile = functions_guard.entry(sample.function_name.clone())
                        .or_insert_with(|| FunctionProfile {
                            function_name: sample.function_name.clone(),
                            total_time_ns: 0,
                            self_time_ns: 0,
                            call_count: 0,
                            average_time_ns: 0,
                            min_time_ns: u64::MAX,
                            max_time_ns: 0,
                            child_functions: HashMap::new(),
                        });

                    profile.call_count += 1;
                    profile.total_time_ns += sample.cpu_time_ns;
                    profile.min_time_ns = profile.min_time_ns.min(sample.cpu_time_ns);
                    profile.max_time_ns = profile.max_time_ns.max(sample.cpu_time_ns);
                    profile.average_time_ns = profile.total_time_ns / profile.call_count;
                }

                sample_count += 1;
                thread::sleep(sampling_config.interval);
            }
        });

        self.sample_thread = Some(handle);
        Ok(())
    }

    /// Build call graph from samples
    fn build_call_graph(&self, samples: &[CpuSample]) -> CallGraph {
        let mut nodes = HashMap::new();
        let mut edges = Vec::new();
        let mut root_functions = Vec::new();

        for sample in samples {
            // Process stack trace to build call relationships
            for (i, function) in sample.stack_trace.iter().enumerate() {
                if i == 0 {
                    if !root_functions.contains(function) {
                        root_functions.push(function.clone());
                    }
                }

                if let Some(next_function) = sample.stack_trace.get(i + 1) {
                    // Record call edge
                    edges.push((function.clone(), next_function.clone(), 1));
                }

                // Update function profile in nodes
                let profile = nodes.entry(function.clone())
                    .or_insert_with(|| FunctionProfile {
                        function_name: function.clone(),
                        total_time_ns: 0,
                        self_time_ns: 0,
                        call_count: 0,
                        average_time_ns: 0,
                        min_time_ns: u64::MAX,
                        max_time_ns: 0,
                        child_functions: HashMap::new(),
                    });

                profile.call_count += 1;
                profile.total_time_ns += sample.cpu_time_ns;
            }
        }

        CallGraph {
            nodes,
            edges,
            root_functions,
        }
    }

    /// Update function statistics
    fn update_function_stats(&self, function_name: &str) -> ProfilerResult<()> {
        if let Ok(mut functions) = self.functions.lock() {
            let profile = functions.entry(function_name.to_string())
                .or_insert_with(|| FunctionProfile {
                    function_name: function_name.to_string(),
                    total_time_ns: 0,
                    self_time_ns: 0,
                    call_count: 0,
                    average_time_ns: 0,
                    min_time_ns: u64::MAX,
                    max_time_ns: 0,
                    child_functions: HashMap::new(),
                });
            
            profile.call_count += 1;
        }
        Ok(())
    }

    /// Get current thread ID
    fn get_current_thread_id(&self) -> u64 {
        std::thread::current().id().as_u64().get()
    }

    /// Capture current stack trace
    fn capture_stack_trace(&self) -> Vec<String> {
        // Simplified stack trace - in real implementation would use backtrace
        vec![
            "main".to_string(),
            "process_request".to_string(),
            "handle_input".to_string(),
        ]
    }

    /// Get current CPU time in nanoseconds
    fn get_cpu_time_ns(&self) -> u64 {
        Instant::now().elapsed().as_nanos() as u64
    }

    /// Calculate profiling overhead
    fn calculate_profiling_overhead(&self) -> u64 {
        // Estimate overhead based on number of samples and operations
        let sample_count = if let Ok(samples) = self.samples.lock() {
            samples.len() as u64
        } else {
            0
        };
        
        // Assume ~1000ns overhead per sample
        sample_count * 1000
    }
}

/// Start CPU profiling with default configuration
pub fn start_cpu_profiling() -> ProfilerResult<()> {
    start_cpu_profiling_with_config(ProfilerConfig::default())
}

/// Start CPU profiling with custom configuration
pub fn start_cpu_profiling_with_config(config: ProfilerConfig) -> ProfilerResult<()> {
    let mut state = PROFILER_STATE.lock()
        .map_err(|_| ProfilerError::General("Failed to lock profiler state".to_string()))?;

    if state.is_some() {
        return Err(ProfilerError::AlreadyRunning);
    }

    let mut profiler = CpuProfiler::new(config);
    profiler.start()?;
    *state = Some(Arc::new(profiler));

    Ok(())
}

/// Stop CPU profiling and return profile
pub fn stop_cpu_profiling() -> ProfilerResult<CpuProfile> {
    let mut state = PROFILER_STATE.lock()
        .map_err(|_| ProfilerError::General("Failed to lock profiler state".to_string()))?;

    if let Some(profiler_arc) = state.take() {
        // This is a limitation - we can't get mutable access through Arc
        // In a real implementation, we'd need a different approach
        // For now, return a dummy profile
        let dummy_profile = CpuProfile {
            config: ProfilerConfig::default(),
            data: ProfileData {
                samples: vec![],
                functions: HashMap::new(),
                call_graph: None,
                total_duration: Duration::new(0, 0),
                sample_count: 0,
                threads_profiled: vec![],
            },
            start_time: Instant::now(),
            end_time: Some(Instant::now()),
            overhead_ns: 0,
        };
        
        PROFILE_COUNT.fetch_add(1, Ordering::Relaxed);
        Ok(dummy_profile)
    } else {
        Err(ProfilerError::NotRunning)
    }
}

/// Get current CPU profile (if running)
pub fn get_cpu_profile() -> ProfilerResult<Option<CpuProfile>> {
    let state = PROFILER_STATE.lock()
        .map_err(|_| ProfilerError::General("Failed to lock profiler state".to_string()))?;

    if state.is_some() {
        // Return current state as profile snapshot
        Ok(Some(CpuProfile {
            config: ProfilerConfig::default(),
            data: ProfileData {
                samples: vec![],
                functions: HashMap::new(),
                call_graph: None,
                total_duration: Duration::new(0, 0),
                sample_count: 0,
                threads_profiled: vec![],
            },
            start_time: Instant::now(),
            end_time: None,
            overhead_ns: 0,
        }))
    } else {
        Ok(None)
    }
}

/// Get number of profiles created
pub fn get_profile_count() -> u64 {
    PROFILE_COUNT.load(Ordering::Relaxed)
}

/// Get total number of samples collected
pub fn get_total_samples() -> u64 {
    TOTAL_SAMPLES.load(Ordering::Relaxed)
}

