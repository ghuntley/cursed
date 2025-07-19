//! Performance Tracking System
//! 
//! Comprehensive performance monitoring for futures, context switching,
//! memory usage, and runtime metrics.

use std::sync::{Arc, Mutex, atomic::{AtomicU64, AtomicUsize, Ordering}};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use once_cell::sync::Lazy;

/// Global performance tracking instance
pub static PERFORMANCE_TRACKER: Lazy<Arc<PerformanceTracker>> = 
    Lazy::new(|| Arc::new(PerformanceTracker::new()));

/// Comprehensive performance tracking system
pub struct PerformanceTracker {
    /// Future execution metrics
    pub future_metrics: Arc<Mutex<FutureMetrics>>,
    /// Context switch tracking
    pub context_metrics: Arc<Mutex<ContextMetrics>>,
    /// Memory usage tracking
    pub memory_metrics: Arc<Mutex<MemoryMetrics>>,
    /// Thread pool metrics
    pub thread_metrics: Arc<Mutex<ThreadMetrics>>,
    /// Network operation metrics
    pub network_metrics: Arc<Mutex<NetworkMetrics>>,
    /// Performance timers
    pub timers: Arc<Mutex<HashMap<String, PerformanceTimer>>>,
}

#[derive(Debug, Default)]
pub struct FutureMetrics {
    pub total_created: AtomicU64,
    pub completed_futures: AtomicU64,
    pub failed_futures: AtomicU64,
    pub cancelled_futures: AtomicU64,
    pub avg_execution_time_nanos: AtomicU64,
    pub max_execution_time_nanos: AtomicU64,
    pub min_execution_time_nanos: AtomicU64,
    pub pending_futures: AtomicUsize,
}

#[derive(Debug, Default)]
pub struct ContextMetrics {
    pub total_context_switches: AtomicU64,
    pub avg_switch_time_nanos: AtomicU64,
    pub max_switch_time_nanos: AtomicU64,
    pub min_switch_time_nanos: AtomicU64,
    pub active_contexts: AtomicUsize,
    pub context_switch_errors: AtomicU64,
}

#[derive(Debug, Default)]
pub struct MemoryMetrics {
    pub heap_allocations: AtomicU64,
    pub heap_deallocations: AtomicU64,
    pub current_heap_usage: AtomicUsize,
    pub max_heap_usage: AtomicUsize,
    pub stack_allocations: AtomicU64,
    pub gc_cycles: AtomicU64,
    pub gc_total_time_nanos: AtomicU64,
}

#[derive(Debug, Default)]
pub struct ThreadMetrics {
    pub active_threads: AtomicUsize,
    pub max_threads: AtomicUsize,
    pub thread_creations: AtomicU64,
    pub thread_destructions: AtomicU64,
    pub blocked_threads: AtomicUsize,
    pub cpu_utilization_percent: AtomicU64,
}

#[derive(Debug, Default)]
pub struct NetworkMetrics {
    pub pending_operations: AtomicUsize,
    pub completed_operations: AtomicU64,
    pub failed_operations: AtomicU64,
    pub bytes_sent: AtomicU64,
    pub bytes_received: AtomicU64,
    pub connection_count: AtomicUsize,
}

#[derive(Debug, Clone)]
pub struct PerformanceTimer {
    pub start_time: Instant,
    pub end_time: Option<Instant>,
    pub description: String,
}

impl PerformanceTracker {
    pub fn new() -> Self {
        Self {
            future_metrics: Arc::new(Mutex::new(FutureMetrics::default())),
            context_metrics: Arc::new(Mutex::new(ContextMetrics::default())),
            memory_metrics: Arc::new(Mutex::new(MemoryMetrics::default())),
            thread_metrics: Arc::new(Mutex::new(ThreadMetrics::default())),
            network_metrics: Arc::new(Mutex::new(NetworkMetrics::default())),
            timers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // Future Tracking Methods
    pub fn track_future_created(&self) {
        if let Ok(metrics) = self.future_metrics.lock() {
            metrics.total_created.fetch_add(1, Ordering::Relaxed);
            metrics.pending_futures.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn track_future_completed(&self, execution_time: Duration) {
        if let Ok(metrics) = self.future_metrics.lock() {
            metrics.completed_futures.fetch_add(1, Ordering::Relaxed);
            metrics.pending_futures.fetch_sub(1, Ordering::Relaxed);
            
            let nanos = execution_time.as_nanos() as u64;
            self.update_timing_stats(&metrics.avg_execution_time_nanos, 
                                   &metrics.max_execution_time_nanos,
                                   &metrics.min_execution_time_nanos, 
                                   nanos);
        }
    }

    pub fn track_future_failed(&self, execution_time: Duration) {
        if let Ok(metrics) = self.future_metrics.lock() {
            metrics.failed_futures.fetch_add(1, Ordering::Relaxed);
            metrics.pending_futures.fetch_sub(1, Ordering::Relaxed);
            
            let nanos = execution_time.as_nanos() as u64;
            self.update_timing_stats(&metrics.avg_execution_time_nanos, 
                                   &metrics.max_execution_time_nanos,
                                   &metrics.min_execution_time_nanos, 
                                   nanos);
        }
    }

    pub fn track_future_cancelled(&self) {
        if let Ok(metrics) = self.future_metrics.lock() {
            metrics.cancelled_futures.fetch_add(1, Ordering::Relaxed);
            metrics.pending_futures.fetch_sub(1, Ordering::Relaxed);
        }
    }

    // Context Switch Tracking Methods
    pub fn track_context_switch(&self, switch_time: Duration) {
        if let Ok(metrics) = self.context_metrics.lock() {
            metrics.total_context_switches.fetch_add(1, Ordering::Relaxed);
            
            let nanos = switch_time.as_nanos() as u64;
            self.update_timing_stats(&metrics.avg_switch_time_nanos,
                                   &metrics.max_switch_time_nanos,
                                   &metrics.min_switch_time_nanos,
                                   nanos);
        }
    }

    pub fn track_context_switch_error(&self) {
        if let Ok(metrics) = self.context_metrics.lock() {
            metrics.context_switch_errors.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn update_active_contexts(&self, count: usize) {
        if let Ok(metrics) = self.context_metrics.lock() {
            metrics.active_contexts.store(count, Ordering::Relaxed);
        }
    }

    // Memory Tracking Methods
    pub fn track_heap_allocation(&self, size: usize) {
        if let Ok(metrics) = self.memory_metrics.lock() {
            metrics.heap_allocations.fetch_add(1, Ordering::Relaxed);
            let current = metrics.current_heap_usage.fetch_add(size, Ordering::Relaxed) + size;
            
            // Update max usage if needed
            let mut max = metrics.max_heap_usage.load(Ordering::Relaxed);
            while current > max {
                match metrics.max_heap_usage.compare_exchange_weak(max, current, Ordering::Relaxed, Ordering::Relaxed) {
                    Ok(_) => break,
                    Err(x) => max = x,
                }
            }
        }
    }

    pub fn track_heap_deallocation(&self, size: usize) {
        if let Ok(metrics) = self.memory_metrics.lock() {
            metrics.heap_deallocations.fetch_add(1, Ordering::Relaxed);
            metrics.current_heap_usage.fetch_sub(size, Ordering::Relaxed);
        }
    }

    pub fn track_gc_cycle(&self, gc_time: Duration) {
        if let Ok(metrics) = self.memory_metrics.lock() {
            metrics.gc_cycles.fetch_add(1, Ordering::Relaxed);
            metrics.gc_total_time_nanos.fetch_add(gc_time.as_nanos() as u64, Ordering::Relaxed);
        }
    }

    // Network Tracking Methods
    pub fn track_network_operation_start(&self) {
        if let Ok(metrics) = self.network_metrics.lock() {
            metrics.pending_operations.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn track_network_operation_completed(&self, bytes_sent: u64, bytes_received: u64) {
        if let Ok(metrics) = self.network_metrics.lock() {
            metrics.pending_operations.fetch_sub(1, Ordering::Relaxed);
            metrics.completed_operations.fetch_add(1, Ordering::Relaxed);
            metrics.bytes_sent.fetch_add(bytes_sent, Ordering::Relaxed);
            metrics.bytes_received.fetch_add(bytes_received, Ordering::Relaxed);
        }
    }

    pub fn track_network_operation_failed(&self) {
        if let Ok(metrics) = self.network_metrics.lock() {
            metrics.pending_operations.fetch_sub(1, Ordering::Relaxed);
            metrics.failed_operations.fetch_add(1, Ordering::Relaxed);
        }
    }

    // Thread Pool Tracking Methods
    pub fn update_thread_count(&self, active: usize, blocked: usize) {
        if let Ok(metrics) = self.thread_metrics.lock() {
            metrics.active_threads.store(active, Ordering::Relaxed);
            metrics.blocked_threads.store(blocked, Ordering::Relaxed);
            
            // Update max threads if needed
            let mut max = metrics.max_threads.load(Ordering::Relaxed);
            while active > max {
                match metrics.max_threads.compare_exchange_weak(max, active, Ordering::Relaxed, Ordering::Relaxed) {
                    Ok(_) => break,
                    Err(x) => max = x,
                }
            }
        }
    }

    pub fn track_thread_created(&self) {
        if let Ok(metrics) = self.thread_metrics.lock() {
            metrics.thread_creations.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn track_thread_destroyed(&self) {
        if let Ok(metrics) = self.thread_metrics.lock() {
            metrics.thread_destructions.fetch_add(1, Ordering::Relaxed);
        }
    }

    // Performance Timer Methods
    pub fn start_timer(&self, name: String, description: String) {
        if let Ok(mut timers) = self.timers.lock() {
            timers.insert(name, PerformanceTimer {
                start_time: Instant::now(),
                end_time: None,
                description,
            });
        }
    }

    pub fn end_timer(&self, name: &str) -> Option<Duration> {
        if let Ok(mut timers) = self.timers.lock() {
            if let Some(timer) = timers.get_mut(name) {
                let end_time = Instant::now();
                timer.end_time = Some(end_time);
                return Some(end_time.duration_since(timer.start_time));
            }
        }
        None
    }

    // Utility Methods
    fn update_timing_stats(&self, avg: &AtomicU64, max: &AtomicU64, min: &AtomicU64, new_value: u64) {
        // Update max
        let mut current_max = max.load(Ordering::Relaxed);
        while new_value > current_max {
            match max.compare_exchange_weak(current_max, new_value, Ordering::Relaxed, Ordering::Relaxed) {
                Ok(_) => break,
                Err(x) => current_max = x,
            }
        }

        // Update min (special case for initial value)
        let mut current_min = min.load(Ordering::Relaxed);
        if current_min == 0 || new_value < current_min {
            while current_min == 0 || new_value < current_min {
                match min.compare_exchange_weak(current_min, new_value, Ordering::Relaxed, Ordering::Relaxed) {
                    Ok(_) => break,
                    Err(x) => current_min = x,
                }
            }
        }

        // Update average (simple exponential moving average)
        let current_avg = avg.load(Ordering::Relaxed);
        let new_avg = if current_avg == 0 {
            new_value
        } else {
            (current_avg * 9 + new_value) / 10  // 90% old, 10% new
        };
        avg.store(new_avg, Ordering::Relaxed);
    }

    // Report Generation
    pub fn generate_performance_report(&self) -> PerformanceReport {
        let future_metrics = self.future_metrics.lock().unwrap();
        let context_metrics = self.context_metrics.lock().unwrap();
        let memory_metrics = self.memory_metrics.lock().unwrap();
        let thread_metrics = self.thread_metrics.lock().unwrap();
        let network_metrics = self.network_metrics.lock().unwrap();
        let timers = self.timers.lock().unwrap();

        PerformanceReport {
            future_stats: FutureStats {
                total_created: future_metrics.total_created.load(Ordering::Relaxed),
                completed: future_metrics.completed_futures.load(Ordering::Relaxed),
                failed: future_metrics.failed_futures.load(Ordering::Relaxed),
                cancelled: future_metrics.cancelled_futures.load(Ordering::Relaxed),
                pending: future_metrics.pending_futures.load(Ordering::Relaxed),
                avg_execution_time_ms: future_metrics.avg_execution_time_nanos.load(Ordering::Relaxed) as f64 / 1_000_000.0,
                max_execution_time_ms: future_metrics.max_execution_time_nanos.load(Ordering::Relaxed) as f64 / 1_000_000.0,
                min_execution_time_ms: future_metrics.min_execution_time_nanos.load(Ordering::Relaxed) as f64 / 1_000_000.0,
            },
            context_stats: ContextStats {
                total_switches: context_metrics.total_context_switches.load(Ordering::Relaxed),
                active_contexts: context_metrics.active_contexts.load(Ordering::Relaxed),
                avg_switch_time_us: context_metrics.avg_switch_time_nanos.load(Ordering::Relaxed) as f64 / 1_000.0,
                max_switch_time_us: context_metrics.max_switch_time_nanos.load(Ordering::Relaxed) as f64 / 1_000.0,
                min_switch_time_us: context_metrics.min_switch_time_nanos.load(Ordering::Relaxed) as f64 / 1_000.0,
                switch_errors: context_metrics.context_switch_errors.load(Ordering::Relaxed),
            },
            memory_stats: MemoryStats {
                heap_allocations: memory_metrics.heap_allocations.load(Ordering::Relaxed),
                heap_deallocations: memory_metrics.heap_deallocations.load(Ordering::Relaxed),
                current_heap_usage_kb: memory_metrics.current_heap_usage.load(Ordering::Relaxed) as f64 / 1024.0,
                max_heap_usage_kb: memory_metrics.max_heap_usage.load(Ordering::Relaxed) as f64 / 1024.0,
                gc_cycles: memory_metrics.gc_cycles.load(Ordering::Relaxed),
                gc_total_time_ms: memory_metrics.gc_total_time_nanos.load(Ordering::Relaxed) as f64 / 1_000_000.0,
            },
            thread_stats: ThreadStats {
                active_threads: thread_metrics.active_threads.load(Ordering::Relaxed),
                max_threads: thread_metrics.max_threads.load(Ordering::Relaxed),
                thread_creations: thread_metrics.thread_creations.load(Ordering::Relaxed),
                thread_destructions: thread_metrics.thread_destructions.load(Ordering::Relaxed),
                blocked_threads: thread_metrics.blocked_threads.load(Ordering::Relaxed),
            },
            network_stats: NetworkStats {
                pending_operations: network_metrics.pending_operations.load(Ordering::Relaxed),
                completed_operations: network_metrics.completed_operations.load(Ordering::Relaxed),
                failed_operations: network_metrics.failed_operations.load(Ordering::Relaxed),
                bytes_sent_mb: network_metrics.bytes_sent.load(Ordering::Relaxed) as f64 / 1_048_576.0,
                bytes_received_mb: network_metrics.bytes_received.load(Ordering::Relaxed) as f64 / 1_048_576.0,
            },
            active_timers: timers.len(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PerformanceReport {
    pub future_stats: FutureStats,
    pub context_stats: ContextStats,
    pub memory_stats: MemoryStats,
    pub thread_stats: ThreadStats,
    pub network_stats: NetworkStats,
    pub active_timers: usize,
}

#[derive(Debug, Clone)]
pub struct FutureStats {
    pub total_created: u64,
    pub completed: u64,
    pub failed: u64,
    pub cancelled: u64,
    pub pending: usize,
    pub avg_execution_time_ms: f64,
    pub max_execution_time_ms: f64,
    pub min_execution_time_ms: f64,
}

#[derive(Debug, Clone)]
pub struct ContextStats {
    pub total_switches: u64,
    pub active_contexts: usize,
    pub avg_switch_time_us: f64,
    pub max_switch_time_us: f64,
    pub min_switch_time_us: f64,
    pub switch_errors: u64,
}

#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub heap_allocations: u64,
    pub heap_deallocations: u64,
    pub current_heap_usage_kb: f64,
    pub max_heap_usage_kb: f64,
    pub gc_cycles: u64,
    pub gc_total_time_ms: f64,
}

#[derive(Debug, Clone)]
pub struct ThreadStats {
    pub active_threads: usize,
    pub max_threads: usize,
    pub thread_creations: u64,
    pub thread_destructions: u64,
    pub blocked_threads: usize,
}

#[derive(Debug, Clone)]
pub struct NetworkStats {
    pub pending_operations: usize,
    pub completed_operations: u64,
    pub failed_operations: u64,
    pub bytes_sent_mb: f64,
    pub bytes_received_mb: f64,
}
