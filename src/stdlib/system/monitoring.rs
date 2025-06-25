/// System monitoring and performance metrics
/// 
/// This module provides system monitoring capabilities including:
/// - Resource usage tracking (CPU, memory, disk, network)
/// - Performance metrics collection
/// - Real-time system monitoring
/// - Historical performance data
/// - Cross-platform support (Linux, macOS, Windows)
/// - Efficient caching for expensive system calls

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use sysinfo::{System, Pid};
// use crate::stdlib::system::info::{SystemResult, SystemError};

#[cfg(target_os = "linux")]
use std::fs;

#[cfg(target_os = "windows")]
use std::process::Command;
use crate::error::CursedError;

/// System monitoring controller with caching
#[derive(Debug)]
pub struct SystemMonitor {
    start_time: Instant,
    enabled: bool,
    system: Arc<Mutex<System>>,
    cache: Arc<Mutex<MonitoringCache>>,
}

/// Cached monitoring data
#[derive(Debug, Clone)]
struct MonitoringCache {
    cpu_usage: Option<(f64, Instant)>,
    memory_usage: Option<(u64, Instant)>,
    disk_usage: Option<(u64, Instant)>,
    network_stats: Option<(NetworkStats, Instant)>,
    performance_metrics: Option<(PerformanceMetrics, Instant)>,
    cache_duration: Duration,
}

/// Network statistics
#[derive(Debug, Clone)]
struct NetworkStats {
    rx_bytes: u64,
    tx_bytes: u64,
    rx_packets: u64,
    tx_packets: u64,
}

impl Clone for SystemMonitor {
    fn clone(&self) -> Self {
        Self {
            start_time: self.start_time,
            enabled: self.enabled,
            system: Arc::clone(&self.system),
            cache: Arc::clone(&self.cache),
        }
    }
}

/// Resource usage information
#[derive(Debug, Clone)]
pub struct ResourceUsage {
    pub cpu_usage: f64,      // CPU usage percentage (0.0-100.0)
    pub memory_usage: u64,   // Memory usage in bytes
    pub disk_usage: u64,     // Disk usage in bytes
    pub network_rx: u64,     // Network bytes received
    pub network_tx: u64,     // Network bytes transmitted
}

/// Performance metrics
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub uptime: Duration,
    pub load_average: Vec<f64>,
    pub process_count: usize,
    pub thread_count: usize,
    pub handles: usize,
    pub metrics: HashMap<String, f64>,
}

impl Default for SystemMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for MonitoringCache {
    fn default() -> Self {
        Self {
            cpu_usage: None,
            memory_usage: None,
            disk_usage: None,
            network_stats: None,
            performance_metrics: None,
            cache_duration: Duration::from_secs(1), // 1 second cache by default
        }
    }
}

impl SystemMonitor {
    /// Create a new system monitor
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        
        Self {
            start_time: Instant::now(),
            enabled: false,
            system: Arc::new(Mutex::new(system)),
            cache: Arc::new(Mutex::new(MonitoringCache::default())),
        }
    }

    /// Create a new system monitor with custom cache duration
    pub fn with_cache_duration(cache_duration: Duration) -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        
        let mut cache = MonitoringCache::default();
        cache.cache_duration = cache_duration;
        
        Self {
            start_time: Instant::now(),
            enabled: false,
            system: Arc::new(Mutex::new(system)),
            cache: Arc::new(Mutex::new(cache)),
        }
    }

    /// Start monitoring
    pub fn start(&mut self) -> SystemResult<()> {
        self.enabled = true;
        self.start_time = Instant::now();
        Ok(())
    }

    /// Stop monitoring
    pub fn stop(&mut self) -> SystemResult<()> {
        self.enabled = false;
        Ok(())
    }

    /// Check if monitoring is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Get current resource usage
    pub fn get_resource_usage(&self) -> SystemResult<ResourceUsage> {
        let cpu_usage = self.get_cpu_usage()?;
        let memory_usage = self.get_memory_usage()?;
        let disk_usage = self.get_disk_usage()?;
        let network_stats = self.get_network_stats()?;
        
        Ok(ResourceUsage {
            cpu_usage,
            memory_usage,
            disk_usage,
            network_rx: network_stats.rx_bytes,
            network_tx: network_stats.tx_bytes,
        })
    }

    /// Get CPU usage percentage
    pub fn get_cpu_usage(&self) -> SystemResult<f64> {
        let now = Instant::now();
        
        // Check cache first
        if let Ok(cache) = self.cache.lock() {
            if let Some((cached_value, cached_time)) = cache.cpu_usage {
                if now.duration_since(cached_time) < cache.cache_duration {
                    return Ok(cached_value);
                }
            }
        }
        
        // Refresh system information
        if let Ok(mut system) = self.system.lock() {
            system.refresh_cpu();
            
            // Give the system time to calculate CPU usage
            std::thread::sleep(Duration::from_millis(100));
            system.refresh_cpu();
            
            let cpu_usage = system.global_cpu_info().cpu_usage() as f64;
            
            // Update cache
            if let Ok(mut cache) = self.cache.lock() {
                cache.cpu_usage = Some((cpu_usage, now));
            }
            
            Ok(cpu_usage)
        } else {
            Err(SystemError::MonitoringError("Failed to access system information".to_string()))
        }
    }

    /// Get memory usage in bytes
    pub fn get_memory_usage(&self) -> SystemResult<u64> {
        let now = Instant::now();
        
        // Check cache first
        if let Ok(cache) = self.cache.lock() {
            if let Some((cached_value, cached_time)) = cache.memory_usage {
                if now.duration_since(cached_time) < cache.cache_duration {
                    return Ok(cached_value);
                }
            }
        }
        
        if let Ok(mut system) = self.system.lock() {
            system.refresh_memory();
            let used_memory = system.used_memory() * 1024; // Convert from KB to bytes
            
            // Update cache
            if let Ok(mut cache) = self.cache.lock() {
                cache.memory_usage = Some((used_memory, now));
            }
            
            Ok(used_memory)
        } else {
            Err(SystemError::MonitoringError("Failed to access system information".to_string()))
        }
    }

    /// Get disk usage in bytes
    pub fn get_disk_usage(&self) -> SystemResult<u64> {
        let now = Instant::now();
        
        // Check cache first
        if let Ok(cache) = self.cache.lock() {
            if let Some((cached_value, cached_time)) = cache.disk_usage {
                if now.duration_since(cached_time) < cache.cache_duration {
                    return Ok(cached_value);
                }
            }
        }
        
        if let Ok(mut system) = self.system.lock() {
            system.refresh_disks();
            let total_used = system.disks().iter()
                .map(|disk| disk.total_space() - disk.available_space())
                .sum();
            
            // Update cache
            if let Ok(mut cache) = self.cache.lock() {
                cache.disk_usage = Some((total_used, now));
            }
            
            Ok(total_used)
        } else {
            Err(SystemError::MonitoringError("Failed to access system information".to_string()))
        }
    }

    /// Get network statistics
    pub fn get_network_stats(&self) -> SystemResult<NetworkStats> {
        let now = Instant::now();
        
        // Check cache first
        if let Ok(cache) = self.cache.lock() {
            if let Some((cached_value, cached_time)) = &cache.network_stats {
                if now.duration_since(*cached_time) < cache.cache_duration {
                    return Ok(cached_value.clone());
                }
            }
        }
        
        if let Ok(mut system) = self.system.lock() {
            system.refresh_networks();
            
            let (total_rx, total_tx) = system.networks().iter()
                .fold((0, 0), |(rx, tx), (_, network)| {
                    (rx + network.received(), tx + network.transmitted())
                });
            
            let stats = NetworkStats {
                rx_bytes: total_rx,
                tx_bytes: total_tx,
                rx_packets: 0, // sysinfo doesn't provide packet counts
                tx_packets: 0,
            };
            
            // Update cache
            if let Ok(mut cache) = self.cache.lock() {
                cache.network_stats = Some((stats.clone(), now));
            }
            
            Ok(stats)
        } else {
            Err(SystemError::MonitoringError("Failed to access system information".to_string()))
        }
    }

    /// Get performance metrics
    pub fn get_performance_metrics(&self) -> SystemResult<PerformanceMetrics> {
        let now = Instant::now();
        
        // Check cache first
        if let Ok(cache) = self.cache.lock() {
            if let Some((cached_value, cached_time)) = &cache.performance_metrics {
                if now.duration_since(*cached_time) < cache.cache_duration {
                    return Ok(cached_value.clone());
                }
            }
        }
        
        if let Ok(mut system) = self.system.lock() {
            system.refresh_all();
            
            let mut metrics = HashMap::new();
            let cpu_usage = system.global_cpu_info().cpu_usage() as f64;
            let memory_usage = system.used_memory() as f64;
            let total_memory = system.total_memory() as f64;
            
            metrics.insert("cpu_usage".to_string(), cpu_usage);
            metrics.insert("memory_usage".to_string(), memory_usage);
            metrics.insert("memory_usage_percent".to_string(), (memory_usage / total_memory) * 100.0);
            metrics.insert("total_memory".to_string(), total_memory);
            metrics.insert("free_memory".to_string(), system.free_memory() as f64);
            
            // Add per-CPU metrics
            for (i, cpu) in system.cpus().iter().enumerate() {
                metrics.insert(format!("cpu_{}_usage", i), cpu.cpu_usage() as f64);
            }
            
            let load_average = self.get_load_average()?;
            let process_count = system.processes().len();
            let thread_count = self.get_thread_count()?;
            
            let performance_metrics = PerformanceMetrics {
                uptime: self.get_system_uptime()?,
                load_average,
                process_count,
                thread_count,
                handles: self.get_handles_count()?,
                metrics,
            };
            
            // Update cache
            if let Ok(mut cache) = self.cache.lock() {
                cache.performance_metrics = Some((performance_metrics.clone(), now));
            }
            
            Ok(performance_metrics)
        } else {
            Err(SystemError::MonitoringError("Failed to access system information".to_string()))
        }
    }

    /// Get system load average
    pub fn get_load_average(&self) -> SystemResult<Vec<f64>> {
        #[cfg(unix)]
        {
            if let Ok(system) = self.system.lock() {
                let load_avg = system.load_average();
                Ok(vec![load_avg.one, load_avg.five, load_avg.fifteen])
            } else {
                Err(SystemError::MonitoringError("Failed to access system information".to_string()))
            }
        }
        
        #[cfg(windows)]
        {
            // Windows doesn't have load average, use CPU usage as approximation
            let cpu_usage = self.get_cpu_usage()?;
            Ok(vec![cpu_usage / 100.0, cpu_usage / 100.0, cpu_usage / 100.0])
        }
    }

    /// Get system uptime
    pub fn get_system_uptime(&self) -> SystemResult<Duration> {
        if let Ok(system) = self.system.lock() {
            Ok(Duration::from_secs(system.uptime()))
        } else {
            Err(SystemError::MonitoringError("Failed to get system uptime".to_string()))
        }
    }

    /// Get total thread count across all processes
    pub fn get_thread_count(&self) -> SystemResult<usize> {
        if let Ok(system) = self.system.lock() {
            let thread_count = system.processes().values()
                .map(|process| process.threads().unwrap_or(&[]).len())
                .sum();
            Ok(thread_count)
        } else {
            Err(SystemError::MonitoringError("Failed to get thread count".to_string()))
        }
    }

    /// Get handle count (platform-specific)
    pub fn get_handles_count(&self) -> SystemResult<usize> {
        #[cfg(target_os = "windows")]
        {
            // On Windows, we can get handle count via WMI or performance counters
            // For now, approximate based on process count
            if let Ok(system) = self.system.lock() {
                Ok(system.processes().len() * 50) // Rough estimate
            } else {
                Err(SystemError::MonitoringError("Failed to get handle count".to_string()))
            }
        }
        
        #[cfg(unix)]
        {
            // On Unix systems, count file descriptors
            self.get_fd_count()
        }
    }

    /// Get file descriptor count (Unix-specific)
    #[cfg(unix)]
    pub fn get_fd_count(&self) -> SystemResult<usize> {
        #[cfg(target_os = "linux")]
        {
            match fs::read_dir("/proc/self/fd") {
                Ok(entries) => Ok(entries.count()),
                Err(_) => {
                    // Fallback: estimate based on process count
                    if let Ok(system) = self.system.lock() {
                        Ok(system.processes().len() * 10)
                    } else {
                        Ok(100) // Conservative estimate
                    }
                }
            }
        }
        
        #[cfg(not(target_os = "linux"))]
        {
            // For macOS and other Unix systems, use a conservative estimate
            if let Ok(system) = self.system.lock() {
                Ok(system.processes().len() * 10)
            } else {
                Ok(100)
            }
        }
    }

    /// Clear monitoring cache
    pub fn clear_cache(&self) -> SystemResult<()> {
        if let Ok(mut cache) = self.cache.lock() {
            *cache = MonitoringCache::default();
            Ok(())
        } else {
            Err(SystemError::MonitoringError("Failed to clear cache".to_string()))
        }
    }

    /// Get cache statistics
    pub fn get_cache_stats(&self) -> SystemResult<HashMap<String, bool>> {
        if let Ok(cache) = self.cache.lock() {
            let mut stats = HashMap::new();
            stats.insert("cpu_usage_cached".to_string(), cache.cpu_usage.is_some());
            stats.insert("memory_usage_cached".to_string(), cache.memory_usage.is_some());
            stats.insert("disk_usage_cached".to_string(), cache.disk_usage.is_some());
            stats.insert("network_stats_cached".to_string(), cache.network_stats.is_some());
            stats.insert("performance_metrics_cached".to_string(), cache.performance_metrics.is_some());
            Ok(stats)
        } else {
            Err(SystemError::MonitoringError("Failed to get cache stats".to_string()))
        }
    }
}

/// Monitor system resources
pub fn monitor_system() -> SystemResult<SystemMonitor> {
    let mut monitor = SystemMonitor::new();
    monitor.start()?;
    Ok(monitor)
}

/// Monitor system resources with custom cache duration
pub fn monitor_system_with_cache(cache_duration: Duration) -> SystemResult<SystemMonitor> {
    let mut monitor = SystemMonitor::with_cache_duration(cache_duration);
    monitor.start()?;
    Ok(monitor)
}

/// Get current resource usage
pub fn get_resource_usage() -> SystemResult<ResourceUsage> {
    let monitor = SystemMonitor::new();
    monitor.get_resource_usage()
}

/// Get performance metrics
pub fn get_performance_metrics() -> SystemResult<PerformanceMetrics> {
    let monitor = SystemMonitor::new();
    monitor.get_performance_metrics()
}

/// Get current CPU usage
pub fn get_cpu_usage() -> SystemResult<f64> {
    let monitor = SystemMonitor::new();
    monitor.get_cpu_usage()
}

/// Get current memory usage
pub fn get_memory_usage() -> SystemResult<u64> {
    let monitor = SystemMonitor::new();
    monitor.get_memory_usage()
}

/// Get disk usage
pub fn get_disk_usage() -> SystemResult<u64> {
    let monitor = SystemMonitor::new();
    monitor.get_disk_usage()
}

/// Get network statistics
pub fn get_network_statistics() -> SystemResult<(u64, u64)> {
    let monitor = SystemMonitor::new();
    let stats = monitor.get_network_stats()?;
    Ok((stats.rx_bytes, stats.tx_bytes))
}

/// Initialize monitoring subsystem
pub fn init_monitoring() -> SystemResult<()> {
    // Initialize platform-specific monitoring
    #[cfg(target_os = "windows")]
    init_windows_monitoring()?;
    
    #[cfg(unix)]
    init_unix_monitoring()?;
    
    Ok(())
}

/// Cleanup monitoring subsystem
pub fn cleanup_monitoring() -> SystemResult<()> {
    // Cleanup platform-specific monitoring
    #[cfg(target_os = "windows")]
    cleanup_windows_monitoring()?;
    
    #[cfg(unix)]
    cleanup_unix_monitoring()?;
    
    Ok(())
}

// Platform-specific implementations

#[cfg(target_os = "windows")]
fn init_windows_monitoring() -> SystemResult<()> {
    // Windows-specific monitoring initialization
    Ok(())
}

#[cfg(target_os = "windows")]
fn cleanup_windows_monitoring() -> SystemResult<()> {
    // Windows-specific monitoring cleanup
    Ok(())
}

#[cfg(unix)]
fn init_unix_monitoring() -> SystemResult<()> {
    // Unix-specific monitoring initialization
    Ok(())
}

#[cfg(unix)]
fn cleanup_unix_monitoring() -> SystemResult<()> {
    // Unix-specific monitoring cleanup
    Ok(())
}

// Additional utility functions

/// Get system information summary
pub fn get_system_info_summary() -> SystemResult<HashMap<String, String>> {
    let mut info = HashMap::new();
    let system = System::new_all();
    
    info.insert("os_name".to_string(), system.name().unwrap_or_else(|| "Unknown".to_string()));
    info.insert("kernel_version".to_string(), system.kernel_version().unwrap_or_else(|| "Unknown".to_string()));
    info.insert("os_version".to_string(), system.os_version().unwrap_or_else(|| "Unknown".to_string()));
    info.insert("host_name".to_string(), system.host_name().unwrap_or_else(|| "Unknown".to_string()));
    info.insert("cpu_count".to_string(), system.cpus().len().to_string());
    info.insert("total_memory".to_string(), (system.total_memory() * 1024).to_string());
    info.insert("uptime".to_string(), system.uptime().to_string());
    
    Ok(info)
}

/// Monitor system continuously with callback
pub fn monitor_continuous<F>(
    interval: Duration,
    mut callback: F,
) -> SystemResult<()>
where
    F: FnMut(ResourceUsage) -> bool, // Return false to stop monitoring
{
    let monitor = SystemMonitor::with_cache_duration(interval);
    
    loop {
        match monitor.get_resource_usage() {
            Ok(usage) => {
                if !callback(usage) {
                    break;
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
        
        std::thread::sleep(interval);
    }
    
    Ok(())
}

/// Get process information by PID
pub fn get_process_info(pid: u32) -> SystemResult<HashMap<String, String>> {
    let mut system = System::new();
    system.refresh_processes();
    
    if let Some(process) = system.process(Pid::from(pid as usize)) {
        let mut info = HashMap::new();
        info.insert("name".to_string(), process.name().to_string());
        info.insert("cpu_usage".to_string(), process.cpu_usage().to_string());
        info.insert("memory".to_string(), (process.memory() * 1024).to_string());
        info.insert("virtual_memory".to_string(), (process.virtual_memory() * 1024).to_string());
        info.insert("status".to_string(), format!("{:?}", process.status()));
        info.insert("start_time".to_string(), process.start_time().to_string());
        info.insert("run_time".to_string(), process.run_time().to_string());
        
        if let Some(exe) = process.exe() {
            info.insert("executable".to_string(), exe.to_string_lossy().to_string());
        }
        
        Ok(info)
    } else {
        Err(SystemError::MonitoringError(format!("Process with PID {} not found", pid)))
    }
}

/// Get top processes by CPU usage
pub fn get_top_processes_by_cpu(limit: usize) -> SystemResult<Vec<(u32, String, f32)>> {
    let mut system = System::new_all();
    system.refresh_processes();
    
    let mut processes: Vec<(u32, String, f32)> = system.processes()
        .iter()
        .map(|(pid, process)| {
            (pid.as_u32(), process.name().to_string(), process.cpu_usage())
        })
        .collect();
    
    processes.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
    processes.truncate(limit);
    
    Ok(processes)
}

/// Get top processes by memory usage
pub fn get_top_processes_by_memory(limit: usize) -> SystemResult<Vec<(u32, String, u64)>> {
    let mut system = System::new_all();
    system.refresh_processes();
    
    let mut processes: Vec<(u32, String, u64)> = system.processes()
        .iter()
        .map(|(pid, process)| {
            (pid.as_u32(), process.name().to_string(), process.memory() * 1024)
        })
        .collect();
    
    processes.sort_by(|a, b| b.2.cmp(&a.2));
    processes.truncate(limit);
    
    Ok(processes)
}

