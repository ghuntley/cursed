use crate::error::CursedError;
// CPU profiling with call stack sampling and performance analysis

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, instrument, warn};

// use crate::profiling::core::{DataCollector, CollectorStats, ProfilerError};

/// CPU profiler for call stack sampling and execution analysis
#[derive(Debug)]
pub struct CpuProfiler {
    sampling_frequency: u64,
    max_stack_depth: usize,
    collecting: Arc<Mutex<bool>>,
    data: Arc<RwLock<CpuProfileData>>,
    stats: Arc<RwLock<CollectorStats>>,
    sampling_thread: Option<thread::JoinHandle<()>>,
}

impl CpuProfiler {
    pub fn new(sampling_frequency: u64, max_stack_depth: usize) -> Self {
        Self {
            sampling_frequency,
            max_stack_depth,
            collecting: Arc::new(Mutex::new(false)),
            data: Arc::new(RwLock::new(CpuProfileData::new())),
            stats: Arc::new(RwLock::new(CollectorStats::default())),
            sampling_thread: None,
        }
    }
    
    #[instrument(skip(self))]
    fn start_sampling(&mut self) -> crate::error::Result<()> {
        let collecting = Arc::clone(&self.collecting);
        let data = Arc::clone(&self.data);
        let stats = Arc::clone(&self.stats);
        let frequency = self.sampling_frequency;
        let max_depth = self.max_stack_depth;
        
        *self.collecting.lock().unwrap() = true;
        
        let handle = thread::spawn(move || {
            Self::sampling_loop(collecting, data, stats, frequency, max_depth);
        });
        
        self.sampling_thread = Some(handle);
        info!("Started CPU sampling at {} Hz", self.sampling_frequency);
        Ok(())
    }
    
    fn sampling_loop(
        collecting: Arc<Mutex<bool>>,
        data: Arc<RwLock<CpuProfileData>>,
        stats: Arc<RwLock<CollectorStats>>,
        frequency: u64,
        max_depth: usize,
    ) {
        let interval = Duration::from_nanos(1_000_000_000 / frequency);
        let start_time = Instant::now();
        
        while *collecting.lock().unwrap() {
            let sample_start = Instant::now();
            
            // Capture stack trace
            if let Ok(stack_trace) = Self::capture_stack_trace(max_depth) {
                // Record sample
                if let Ok(mut profile_data) = data.write() {
                    profile_data.add_sample(stack_trace);
                }
                
                // Update stats
                if let Ok(mut collector_stats) = stats.write() {
                    collector_stats.data_points += 1;
                    collector_stats.collection_time = start_time.elapsed();
                }
            } else {
                if let Ok(mut collector_stats) = stats.write() {
                    collector_stats.errors += 1;
                }
            }
            
            // Sleep until next sample
            let sample_duration = sample_start.elapsed();
            if sample_duration < interval {
                thread::sleep(interval - sample_duration);
            }
        }
    }
    
    fn capture_stack_trace(max_depth: usize) -> crate::error::Result<()> {
        // In a real implementation, this would use platform-specific APIs
        // like libunwind, Windows StackWalk, or signal-based sampling
        
        let mut frames = Vec::new();
        
        // Simulate stack trace capture
        // In practice, this would use:
        // - backtrace crate for Rust stack traces
        // - DWARF debugging information for CURSED code
        // - Platform-specific APIs for native code
        
        for i in 0..std::cmp::min(max_depth, 10) {
            frames.push(StackFrame {
                function_name: format!("function_{}", i),
                file_name: Some(format!("file_{}.csd", i)),
                line_number: Some(i as u32 * 10 + 5),
                instruction_pointer: 0x1000 + (i as u64 * 0x100),
                module_name: Some("cursed_module".to_string()),
            });
        }
        
        Ok(StackTrace {
            frames,
            timestamp: Instant::now(),
            thread_id: Self::get_current_thread_id(),
        })
    }
    
    fn get_current_thread_id() -> u64 {
        // Use a simple hash of the thread id since as_u64() is unstable
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        std::thread::current().id().hash(&mut hasher);
        hasher.finish()
    }
    
    pub fn get_profile_data(&self) -> CpuProfileData {
        self.data.read().unwrap().clone()
    }
    
    pub fn generate_flame_graph(&self) -> crate::error::Result<()> {
        let profile_data = self.get_profile_data();
        FlameGraph::from_cpu_profile(&profile_data)
    }
}

impl DataCollector for CpuProfiler {
    #[instrument(skip(self))]
    fn start_collection(&mut self) -> crate::error::Result<()> {
        if self.is_collecting() {
            return Err(ProfilerError::ConfigError("CPU profiler already collecting".to_string()));
        }
        
        self.start_sampling()
    }
    
    #[instrument(skip(self))]
    fn stop_collection(&mut self) -> crate::error::Result<()> {
        if !self.is_collecting() {
            return Err(ProfilerError::ConfigError("CPU profiler not collecting".to_string()));
        }
        
        // Stop sampling
        *self.collecting.lock().unwrap() = false;
        
        // Wait for sampling thread to finish
        if let Some(handle) = self.sampling_thread.take() {
            if let Err(e) = handle.join() {
                error!("Failed to join sampling thread: {:?}", e);
            }
        }
        
        // Serialize profile data
        let profile_data = self.get_profile_data();
        match bincode::serialize(&profile_data) {
            Ok(data) => {
                if let Ok(mut stats) = self.stats.write() {
                    stats.bytes_collected = data.len() as u64;
                }
                info!("CPU profiling stopped, collected {} samples", profile_data.samples.len());
                Ok(data)
            }
            Err(e) => Err(ProfilerError::SerializationError(e.to_string())),
        }
    }
    
    fn is_collecting(&self) -> bool {
        *self.collecting.lock().unwrap()
    }
    
    fn get_stats(&self) -> CollectorStats {
        self.stats.read().unwrap().clone()
    }
}

/// CPU profiling data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuProfileData {
    pub samples: Vec<StackTrace>,
    pub sample_count: u64,
    pub total_duration: Duration,
    pub function_stats: HashMap<String, FunctionStats>,
}

impl CpuProfileData {
    pub fn new() -> Self {
        Self {
            samples: Vec::new(),
            sample_count: 0,
            total_duration: Duration::default(),
            function_stats: HashMap::new(),
        }
    }
    
    pub fn add_sample(&mut self, stack_trace: StackTrace) {
        // Update function statistics
        for frame in &stack_trace.frames {
            let stats = self.function_stats
                .entry(frame.function_name.clone())
                .or_insert_with(FunctionStats::default);
            
            stats.sample_count += 1;
            stats.exclusive_time += Duration::from_nanos(1); // Simplified
        }
        
        self.samples.push(stack_trace);
        self.sample_count += 1;
    }
    
    pub fn get_hot_functions(&self, limit: usize) -> Vec<(&String, &FunctionStats)> {
        let mut functions: Vec<_> = self.function_stats.iter().collect();
        functions.sort_by(|a, b| b.1.sample_count.cmp(&a.1.sample_count));
        functions.into_iter().take(limit).collect()
    }
    
    pub fn get_call_graph(&self) -> CallGraph {
        let mut call_graph = CallGraph::new();
        
        for sample in &self.samples {
            // Build call edges from stack trace
            for window in sample.frames.windows(2) {
                if let [caller, callee] = window {
                    call_graph.add_edge(
                        caller.function_name.clone(),
                        callee.function_name.clone(),
                    );
                }
            }
        }
        
        call_graph
    }
}

/// Individual stack frame information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackFrame {
    pub function_name: String,
    pub file_name: Option<String>,
    pub line_number: Option<u32>,
    pub instruction_pointer: u64,
    pub module_name: Option<String>,
}

/// Complete stack trace sample
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackTrace {
    pub frames: Vec<StackFrame>,
    #[serde(skip, default = "Instant::now")]
    pub timestamp: Instant,
    pub thread_id: u64,
}

/// Function execution statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FunctionStats {
    pub sample_count: u64,
    pub exclusive_time: Duration,
    pub inclusive_time: Duration,
    pub call_count: u64,
}

impl FunctionStats {
    pub fn percentage(&self, total_samples: u64) -> f64 {
        if total_samples == 0 {
            0.0
        } else {
            (self.sample_count as f64 / total_samples as f64) * 100.0
        }
    }
}

/// Call graph representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallGraph {
    pub edges: HashMap<String, HashMap<String, u64>>, // caller -> callee -> count
    pub nodes: HashMap<String, FunctionStats>,
}

impl CallGraph {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
            nodes: HashMap::new(),
        }
    }
    
    pub fn add_edge(&mut self, caller: String, callee: String) {
        let caller_edges = self.edges.entry(caller).or_default();
        *caller_edges.entry(callee).or_default() += 1;
    }
    
    pub fn get_call_frequency(&self, caller: &str, callee: &str) -> u64 {
        self.edges
            .get(caller)
            .and_then(|edges| edges.get(callee))
            .copied()
            .unwrap_or(0)
    }
}

/// Flame graph generation for CPU profiling visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlameGraph {
    pub nodes: Vec<FlameGraphNode>,
    pub total_samples: u64,
    pub max_depth: usize,
}

impl FlameGraph {
    pub fn from_cpu_profile(profile: &CpuProfileData) -> crate::error::Result<()> {
        let mut nodes = Vec::new();
        let mut stack_counts: HashMap<Vec<String>, u64> = HashMap::new();
        
        // Count stack combinations
        for sample in &profile.samples {
            let stack: Vec<String> = sample.frames
                .iter()
                .map(|f| f.function_name.clone())
                .collect();
            
            // Count all prefixes of the stack
            for i in 1..=stack.len() {
                let prefix = stack[..i].to_vec();
                *stack_counts.entry(prefix).or_default() += 1;
            }
        }
        
        // Convert to flame graph nodes
        for (stack, count) in stack_counts {
            if let Some(function_name) = stack.last() {
                nodes.push(FlameGraphNode {
                    name: function_name.clone(),
                    value: count,
                    depth: stack.len() - 1,
                    stack_trace: stack,
                });
            }
        }
        
        nodes.sort_by(|a, b| a.depth.cmp(&b.depth).then(b.value.cmp(&a.value)));
        
        Ok(FlameGraph {
            total_samples: profile.sample_count,
            max_depth: nodes.iter().map(|n| n.depth).max().unwrap_or(0),
            nodes,
        })
    }
    
    pub fn to_svg(&self) -> String {
        // Generate SVG representation of flame graph
        let mut svg = String::new();
        svg.push_str(r#"<svg viewBox="0 0 1200 600" xmlns="http://www.w3.org/2000/svg">"#);
        svg.push_str(r#"<style>rect { stroke: white; } text { font-family: Arial; font-size: 12px; }</style>"#);
        
        let width_per_sample = 1200.0 / self.total_samples as f64;
        let height_per_level = 20.0;
        
        for node in &self.nodes {
            let x = 0.0; // Simplified positioning
            let y = node.depth as f64 * height_per_level;
            let width = node.value as f64 * width_per_sample;
            let height = height_per_level;
            
            svg.push_str(&format!(
                r#"<rect x="{}" y="{}" width="{}" height="{}" fill="hsl({}, 70%, 50%)" />"#,
                x, y, width, height,
                (node.name.len() * 137) % 360 // Color based on function name
            ));
            
            if width > 20.0 { // Only show text if wide enough
                svg.push_str(&format!(
                    r#"<text x="{}" y="{}" fill="black">{}</text>"#,
                    x + 2.0, y + height_per_level - 4.0, node.name
                ));
            }
        }
        
        svg.push_str("</svg>");
        svg
    }
}

/// Individual node in flame graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlameGraphNode {
    pub name: String,
    pub value: u64,
    pub depth: usize,
    pub stack_trace: Vec<String>,
}

