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
impl CpuProfiler {
    pub fn new(sampling_frequency: u64, max_stack_depth: usize) -> Self {
        Self {
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
    fn sampling_loop(
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
            });
        Ok(StackTrace {
        })
    fn get_current_thread_id() -> u64 {
        // Use a simple hash of the thread id since as_u64() is unstable
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        std::thread::current().id().hash(&mut hasher);
        hasher.finish()
    pub fn get_profile_data(&self) -> CpuProfileData {
        self.data.read().unwrap().clone()
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
        self.start_sampling()
    #[instrument(skip(self))]
    fn stop_collection(&mut self) -> crate::error::Result<()> {
        if !self.is_collecting() {
            return Err(ProfilerError::ConfigError("CPU profiler not collecting".to_string()));
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
        }
    }
    
    fn is_collecting(&self) -> bool {
        *self.collecting.lock().unwrap()
    fn get_stats(&self) -> CollectorStats {
        self.stats.read().unwrap().clone()
    }
}

/// CPU profiling data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuProfileData {
impl CpuProfileData {
    pub fn new() -> Self {
        Self {
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
        self.samples.push(stack_trace);
        self.sample_count += 1;
    pub fn get_hot_functions(&self, limit: usize) -> Vec<(&String, &FunctionStats)> {
        let mut functions: Vec<_> = self.function_stats.iter().collect();
        functions.sort_by(|a, b| b.1.sample_count.cmp(&a.1.sample_count));
        functions.into_iter().take(limit).collect()
    pub fn get_call_graph(&self) -> CallGraph {
        let mut call_graph = CallGraph::new();
        
        for sample in &self.samples {
            // Build call edges from stack trace
            for window in sample.frames.windows(2) {
                if let [caller, callee] = window {
                    call_graph.add_edge(
                    );
                }
            }
        call_graph
    }
}

/// Individual stack frame information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackFrame {
/// Complete stack trace sample
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackTrace {
    #[serde(skip, default = "Instant::now")]
/// Function execution statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FunctionStats {
impl FunctionStats {
    pub fn percentage(&self, total_samples: u64) -> f64 {
        if total_samples == 0 {
            0.0
        } else {
            (self.sample_count as f64 / total_samples as f64) * 100.0
        }
    }
/// Call graph representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallGraph {
    pub edges: HashMap<String, HashMap<String, u64>>, // caller -> callee -> count
impl CallGraph {
    pub fn new() -> Self {
        Self {
        }
    }
    
    pub fn add_edge(&mut self, caller: String, callee: String) {
        let caller_edges = self.edges.entry(caller).or_default();
        *caller_edges.entry(callee).or_default() += 1;
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
                });
            }
        }
        
        nodes.sort_by(|a, b| a.depth.cmp(&b.depth).then(b.value.cmp(&a.value)));
        
        Ok(FlameGraph {
        })
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
