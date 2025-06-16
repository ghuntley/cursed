//! Build Analytics and Performance Monitoring
//! 
//! Tracks build performance metrics, identifies bottlenecks, and provides
//! actionable insights for build optimization.

use std::collections::{HashMap, VecDeque, BTreeMap};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use tracing::{info, debug, warn, instrument};

use crate::error::{CursedError, Result};

/// Individual build event for tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildEvent {
    pub id: String,
    pub event_type: BuildEventType,
    pub timestamp: u64,
    pub duration: Duration,
    pub file_path: Option<PathBuf>,
    pub module_name: Option<String>,
    pub memory_usage_mb: f64,
    pub cpu_usage_percent: f64,
    pub success: bool,
    pub error_message: Option<String>,
    pub metadata: HashMap<String, String>,
}

/// Types of build events to track
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum BuildEventType {
    CompilationStart,
    CompilationEnd,
    DependencyResolution,
    CacheHit,
    CacheMiss,
    OptimizationPass,
    Linking,
    CodeGeneration,
    TypeChecking,
    Parsing,
    GarbageCollection,
    FileIO,
    NetworkRequest,
    DistributedTask,
}

/// Build performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildMetrics {
    pub total_build_time: Duration,
    pub compilation_time: Duration,
    pub linking_time: Duration,
    pub dependency_resolution_time: Duration,
    pub cache_time: Duration,
    pub optimization_time: Duration,
    pub files_compiled: usize,
    pub cache_hit_rate: f64,
    pub memory_peak_mb: f64,
    pub memory_average_mb: f64,
    pub cpu_average_percent: f64,
    pub parallelism_efficiency: f64,
    pub network_time: Duration,
    pub disk_io_time: Duration,
}

/// Build bottleneck analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BottleneckAnalysis {
    pub slowest_files: Vec<(PathBuf, Duration)>,
    pub longest_dependencies: Vec<(String, Duration)>,
    pub memory_intensive_operations: Vec<(BuildEventType, f64)>,
    pub cpu_intensive_operations: Vec<(BuildEventType, f64)>,
    pub critical_path_duration: Duration,
    pub critical_path_files: Vec<PathBuf>,
    pub optimization_opportunities: Vec<OptimizationOpportunity>,
}

/// Optimization opportunity recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationOpportunity {
    pub category: OptimizationCategory,
    pub description: String,
    pub estimated_time_savings: Duration,
    pub effort_level: EffortLevel,
    pub specific_files: Vec<PathBuf>,
    pub recommended_actions: Vec<String>,
}

/// Categories of optimization opportunities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationCategory {
    Caching,
    Parallelization,
    DependencyOptimization,
    MemoryOptimization,
    IOOptimization,
    NetworkOptimization,
    AlgorithmOptimization,
    ConfigurationOptimization,
}

/// Effort level for implementing optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffortLevel {
    Low,     // < 1 hour
    Medium,  // 1-8 hours
    High,    // 1-3 days
    VeryHigh, // > 3 days
}

/// Trend analysis data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendAnalysis {
    pub build_time_trend: Vec<(u64, Duration)>, // timestamp, duration
    pub cache_hit_trend: Vec<(u64, f64)>,       // timestamp, hit_rate
    pub memory_usage_trend: Vec<(u64, f64)>,    // timestamp, memory_mb
    pub file_count_trend: Vec<(u64, usize)>,    // timestamp, file_count
    pub performance_regression_alerts: Vec<PerformanceAlert>,
    pub improvement_suggestions: Vec<String>,
}

/// Performance regression alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlert {
    pub alert_type: AlertType,
    pub severity: AlertSeverity,
    pub description: String,
    pub affected_files: Vec<PathBuf>,
    pub performance_degradation: f64, // percentage
    pub detected_at: u64,
    pub suggested_actions: Vec<String>,
}

/// Types of performance alerts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertType {
    BuildTimeIncrease,
    MemoryUsageIncrease,
    CacheHitRateDecrease,
    CompilationFailureIncrease,
    DependencyResolutionSlow,
    NetworkLatencyIncrease,
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

/// Configuration for build analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildAnalyticsConfig {
    pub enable_detailed_tracking: bool,
    pub enable_memory_profiling: bool,
    pub enable_cpu_profiling: bool,
    pub enable_network_monitoring: bool,
    pub enable_trend_analysis: bool,
    pub enable_regression_detection: bool,
    pub analytics_data_path: PathBuf,
    pub max_history_days: u32,
    pub sampling_interval_ms: u64,
    pub regression_threshold_percent: f64,
    pub report_generation_enabled: bool,
}

impl Default for BuildAnalyticsConfig {
    fn default() -> Self {
        Self {
            enable_detailed_tracking: true,
            enable_memory_profiling: true,
            enable_cpu_profiling: true,
            enable_network_monitoring: false,
            enable_trend_analysis: true,
            enable_regression_detection: true,
            analytics_data_path: PathBuf::from(".cursed_analytics"),
            max_history_days: 30,
            sampling_interval_ms: 100,
            regression_threshold_percent: 20.0,
            report_generation_enabled: true,
        }
    }
}

/// Build analytics engine
pub struct BuildAnalytics {
    config: BuildAnalyticsConfig,
    events: Arc<Mutex<VecDeque<BuildEvent>>>,
    metrics_history: Arc<RwLock<Vec<BuildMetrics>>>,
    trend_data: Arc<RwLock<TrendAnalysis>>,
    current_build_events: Arc<Mutex<Vec<BuildEvent>>>,
    performance_baselines: Arc<RwLock<HashMap<String, f64>>>,
    active_monitoring: Arc<Mutex<bool>>,
    system_monitor: Arc<Mutex<SystemMonitor>>,
}

/// System resource monitor
#[derive(Debug)]
pub struct SystemMonitor {
    memory_samples: VecDeque<f64>,
    cpu_samples: VecDeque<f64>,
    last_sample_time: Instant,
    sampling_interval: Duration,
}

impl BuildAnalytics {
    /// Create a new build analytics engine
    #[instrument]
    pub fn new(config: BuildAnalyticsConfig) -> Result<Self> {
        // Ensure analytics directory exists
        if !config.analytics_data_path.exists() {
            std::fs::create_dir_all(&config.analytics_data_path)?;
            info!("Created analytics directory: {:?}", config.analytics_data_path);
        }

        let analytics = Self {
            events: Arc::new(Mutex::new(VecDeque::new())),
            metrics_history: Arc::new(RwLock::new(Vec::new())),
            trend_data: Arc::new(RwLock::new(TrendAnalysis {
                build_time_trend: Vec::new(),
                cache_hit_trend: Vec::new(),
                memory_usage_trend: Vec::new(),
                file_count_trend: Vec::new(),
                performance_regression_alerts: Vec::new(),
                improvement_suggestions: Vec::new(),
            })),
            current_build_events: Arc::new(Mutex::new(Vec::new())),
            performance_baselines: Arc::new(RwLock::new(HashMap::new())),
            active_monitoring: Arc::new(Mutex::new(false)),
            system_monitor: Arc::new(Mutex::new(SystemMonitor {
                memory_samples: VecDeque::new(),
                cpu_samples: VecDeque::new(),
                last_sample_time: Instant::now(),
                sampling_interval: Duration::from_millis(config.sampling_interval_ms),
            })),
            config,
        };

        // Load historical data
        analytics.load_historical_data()?;

        info!("Build analytics engine initialized");
        Ok(analytics)
    }

    /// Start monitoring a build
    #[instrument(skip(self))]
    pub fn start_build_monitoring(&self) -> Result<()> {
        {
            let mut active = self.active_monitoring.lock().map_err(|_| CursedError::system_error("Failed to lock monitoring state"))?;
            if *active {
                return Err(CursedError::system_error("Build monitoring already active"));
            }
            *active = true;
        }

        // Clear current build events
        {
            let mut events = self.current_build_events.lock().map_err(|_| CursedError::system_error("Failed to lock current events"))?;
            events.clear();
        }

        // Start system monitoring if enabled
        if self.config.enable_memory_profiling || self.config.enable_cpu_profiling {
            self.start_system_monitoring()?;
        }

        debug!("Started build monitoring");
        Ok(())
    }

    /// Stop monitoring and analyze the build
    #[instrument(skip(self))]
    pub fn stop_build_monitoring(&self) -> Result<BuildMetrics> {
        {
            let mut active = self.active_monitoring.lock().map_err(|_| CursedError::system_error("Failed to lock monitoring state"))?;
            if !*active {
                return Err(CursedError::system_error("Build monitoring not active"));
            }
            *active = false;
        }

        // Stop system monitoring
        self.stop_system_monitoring()?;

        // Analyze the completed build
        let metrics = self.analyze_build()?;

        // Store metrics in history
        {
            let mut history = self.metrics_history.write().map_err(|_| CursedError::system_error("Failed to lock metrics history"))?;
            history.push(metrics.clone());

            // Limit history size
            let max_entries = (self.config.max_history_days * 24) as usize; // Assuming one build per hour
            if history.len() > max_entries {
                history.drain(0..history.len() - max_entries);
            }
        }

        // Update trend analysis
        self.update_trend_analysis(&metrics)?;

        // Check for performance regressions
        if self.config.enable_regression_detection {
            self.detect_performance_regressions(&metrics)?;
        }

        debug!(
            total_time = ?metrics.total_build_time,
            files_compiled = metrics.files_compiled,
            cache_hit_rate = metrics.cache_hit_rate,
            "Build monitoring completed"
        );

        Ok(metrics)
    }

    /// Record a build event
    #[instrument(skip(self, event))]
    pub fn record_event(&self, event: BuildEvent) -> Result<()> {
        // Add to current build events
        {
            let mut current_events = self.current_build_events.lock().map_err(|_| CursedError::system_error("Failed to lock current events"))?;
            current_events.push(event.clone());
        }

        // Add to global event history
        {
            let mut events = self.events.lock().map_err(|_| CursedError::system_error("Failed to lock events"))?;
            events.push_back(event.clone());

            // Limit event history size
            let max_events = 10000;
            if events.len() > max_events {
                events.pop_front();
            }
        }

        debug!(
            event_type = ?event.event_type,
            duration = ?event.duration,
            success = event.success,
            "Recorded build event"
        );

        Ok(())
    }

    /// Generate comprehensive build report
    #[instrument(skip(self))]
    pub fn generate_build_report(&self) -> Result<BuildReport> {
        let metrics = {
            let history = self.metrics_history.read().map_err(|_| CursedError::system_error("Failed to lock metrics history"))?;
            history.last().cloned().unwrap_or_else(|| BuildMetrics {
                total_build_time: Duration::ZERO,
                compilation_time: Duration::ZERO,
                linking_time: Duration::ZERO,
                dependency_resolution_time: Duration::ZERO,
                cache_time: Duration::ZERO,
                optimization_time: Duration::ZERO,
                files_compiled: 0,
                cache_hit_rate: 0.0,
                memory_peak_mb: 0.0,
                memory_average_mb: 0.0,
                cpu_average_percent: 0.0,
                parallelism_efficiency: 0.0,
                network_time: Duration::ZERO,
                disk_io_time: Duration::ZERO,
            })
        };

        let bottlenecks = self.analyze_bottlenecks()?;
        let trends = {
            let trend_data = self.trend_data.read().map_err(|_| CursedError::system_error("Failed to lock trend data"))?;
            trend_data.clone()
        };

        let report = BuildReport {
            generated_at: current_timestamp(),
            build_metrics: metrics,
            bottleneck_analysis: bottlenecks,
            trend_analysis: trends,
            recommendations: self.generate_recommendations()?,
            performance_comparison: self.generate_performance_comparison()?,
        };

        if self.config.report_generation_enabled {
            self.save_report_to_file(&report)?;
        }

        info!("Generated comprehensive build report");
        Ok(report)
    }

    /// Analyze bottlenecks in the build process
    #[instrument(skip(self))]
    pub fn analyze_bottlenecks(&self) -> Result<BottleneckAnalysis> {
        let events = self.current_build_events.lock().map_err(|_| CursedError::system_error("Failed to lock current events"))?;
        
        // Analyze slowest files
        let mut file_durations: HashMap<PathBuf, Duration> = HashMap::new();
        for event in events.iter() {
            if let Some(path) = &event.file_path {
                if matches!(event.event_type, BuildEventType::CompilationEnd) {
                    file_durations.entry(path.clone())
                        .and_modify(|d| *d += event.duration)
                        .or_insert(event.duration);
                }
            }
        }

        let mut slowest_files: Vec<(PathBuf, Duration)> = file_durations.into_iter().collect();
        slowest_files.sort_by(|a, b| b.1.cmp(&a.1));
        slowest_files.truncate(10);

        // Analyze longest dependencies
        let longest_dependencies = self.analyze_dependency_chain(&events)?;

        // Analyze memory intensive operations
        let mut memory_operations: HashMap<BuildEventType, f64> = HashMap::new();
        for event in events.iter() {
            memory_operations.entry(event.event_type.clone())
                .and_modify(|m| *m = m.max(event.memory_usage_mb))
                .or_insert(event.memory_usage_mb);
        }

        let mut memory_intensive: Vec<(BuildEventType, f64)> = memory_operations.into_iter().collect();
        memory_intensive.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        memory_intensive.truncate(5);

        // Analyze CPU intensive operations
        let mut cpu_operations: HashMap<BuildEventType, f64> = HashMap::new();
        for event in events.iter() {
            cpu_operations.entry(event.event_type.clone())
                .and_modify(|c| *c = c.max(event.cpu_usage_percent))
                .or_insert(event.cpu_usage_percent);
        }

        let mut cpu_intensive: Vec<(BuildEventType, f64)> = cpu_operations.into_iter().collect();
        cpu_intensive.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        cpu_intensive.truncate(5);

        // Calculate critical path
        let critical_path_duration = slowest_files.first()
            .map(|(_, duration)| *duration)
            .unwrap_or(Duration::ZERO);

        let critical_path_files = slowest_files.iter()
            .take(3)
            .map(|(path, _)| path.clone())
            .collect();

        // Generate optimization opportunities
        let optimization_opportunities = self.generate_optimization_opportunities(&slowest_files, &memory_intensive, &cpu_intensive)?;

        Ok(BottleneckAnalysis {
            slowest_files,
            longest_dependencies,
            memory_intensive_operations: memory_intensive,
            cpu_intensive_operations: cpu_intensive,
            critical_path_duration,
            critical_path_files,
            optimization_opportunities,
        })
    }

    /// Analyze dependency chain to find longest dependencies
    #[instrument(skip(self, events))]
    fn analyze_dependency_chain(&self, events: &[BuildEvent]) -> Result<Vec<(String, Duration)>> {
        let mut dependency_durations: HashMap<String, Duration> = HashMap::new();
        let mut dependency_graph: HashMap<String, Vec<String>> = HashMap::new();
        
        // Build dependency graph from events
        for event in events {
            if matches!(event.event_type, BuildEventType::DependencyResolution) {
                if let Some(module) = &event.module_name {
                    dependency_durations.entry(module.clone())
                        .and_modify(|d| *d += event.duration)
                        .or_insert(event.duration);
                    
                    // Extract dependencies from metadata
                    if let Some(deps) = event.metadata.get("dependencies") {
                        let deps: Vec<String> = deps.split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                        dependency_graph.insert(module.clone(), deps);
                    }
                }
            }
        }
        
        // Calculate transitive dependency costs
        let mut dependency_costs: HashMap<String, Duration> = HashMap::new();
        for (module, _) in &dependency_graph {
            let cost = self.calculate_dependency_cost(module, &dependency_graph, &dependency_durations, &mut HashMap::new())?;
            dependency_costs.insert(module.clone(), cost);
        }
        
        // Sort by total dependency cost
        let mut longest_dependencies: Vec<(String, Duration)> = dependency_costs.into_iter().collect();
        longest_dependencies.sort_by(|a, b| b.1.cmp(&a.1));
        longest_dependencies.truncate(10);
        
        Ok(longest_dependencies)
    }
    
    /// Calculate total dependency cost including transitive dependencies
    fn calculate_dependency_cost(
        &self,
        module: &str,
        dependency_graph: &HashMap<String, Vec<String>>,
        dependency_durations: &HashMap<String, Duration>,
        visited: &mut HashMap<String, Duration>,
    ) -> Result<Duration> {
        // Check for circular dependencies
        if visited.contains_key(module) {
            return Ok(visited[module]);
        }
        
        let mut total_cost = dependency_durations.get(module).copied().unwrap_or(Duration::ZERO);
        
        // Add transitive dependency costs
        if let Some(deps) = dependency_graph.get(module) {
            for dep in deps {
                let dep_cost = self.calculate_dependency_cost(dep, dependency_graph, dependency_durations, visited)?;
                total_cost += dep_cost;
            }
        }
        
        visited.insert(module.to_string(), total_cost);
        Ok(total_cost)
    }

    /// Generate optimization opportunities
    #[instrument(skip(self, slowest_files, memory_intensive, cpu_intensive))]
    fn generate_optimization_opportunities(
        &self,
        slowest_files: &[(PathBuf, Duration)],
        memory_intensive: &[(BuildEventType, f64)],
        cpu_intensive: &[(BuildEventType, f64)],
    ) -> Result<Vec<OptimizationOpportunity>> {
        let mut opportunities = Vec::new();

        // Caching opportunities
        if slowest_files.len() > 3 {
            opportunities.push(OptimizationOpportunity {
                category: OptimizationCategory::Caching,
                description: "Implement incremental compilation for slow files".to_string(),
                estimated_time_savings: slowest_files.iter().take(3).map(|(_, d)| *d).sum::<Duration>() / 2,
                effort_level: EffortLevel::Medium,
                specific_files: slowest_files.iter().take(3).map(|(p, _)| p.clone()).collect(),
                recommended_actions: vec![
                    "Enable AST caching for these files".to_string(),
                    "Implement more aggressive change detection".to_string(),
                ],
            });
        }

        // Memory optimization opportunities
        if let Some((_, memory_usage)) = memory_intensive.first() {
            if *memory_usage > 1000.0 { // > 1GB
                opportunities.push(OptimizationOpportunity {
                    category: OptimizationCategory::MemoryOptimization,
                    description: "Optimize memory usage for high-memory operations".to_string(),
                    estimated_time_savings: Duration::from_secs(30),
                    effort_level: EffortLevel::High,
                    specific_files: Vec::new(),
                    recommended_actions: vec![
                        "Implement streaming compilation".to_string(),
                        "Reduce memory allocations during compilation".to_string(),
                        "Enable garbage collection during compilation".to_string(),
                    ],
                });
            }
        }

        // Parallelization opportunities
        if slowest_files.len() > 1 {
            opportunities.push(OptimizationOpportunity {
                category: OptimizationCategory::Parallelization,
                description: "Increase parallelization of compilation tasks".to_string(),
                estimated_time_savings: slowest_files.iter().map(|(_, d)| *d).sum::<Duration>() / 4,
                effort_level: EffortLevel::Low,
                specific_files: Vec::new(),
                recommended_actions: vec![
                    "Increase number of parallel compilation jobs".to_string(),
                    "Optimize dependency graph for better parallelization".to_string(),
                ],
            });
        }

        Ok(opportunities)
    }

    /// Update trend analysis with new metrics
    #[instrument(skip(self, metrics))]
    fn update_trend_analysis(&self, metrics: &BuildMetrics) -> Result<()> {
        let mut trend_data = self.trend_data.write().map_err(|_| CursedError::system_error("Failed to lock trend data"))?;
        let timestamp = current_timestamp();

        // Update trends
        trend_data.build_time_trend.push((timestamp, metrics.total_build_time));
        trend_data.cache_hit_trend.push((timestamp, metrics.cache_hit_rate));
        trend_data.memory_usage_trend.push((timestamp, metrics.memory_peak_mb));
        trend_data.file_count_trend.push((timestamp, metrics.files_compiled));

        // Limit trend data size
        let max_points = 1000;
        if trend_data.build_time_trend.len() > max_points {
            trend_data.build_time_trend.drain(0..trend_data.build_time_trend.len() - max_points);
            trend_data.cache_hit_trend.drain(0..trend_data.cache_hit_trend.len() - max_points);
            trend_data.memory_usage_trend.drain(0..trend_data.memory_usage_trend.len() - max_points);
            trend_data.file_count_trend.drain(0..trend_data.file_count_trend.len() - max_points);
        }

        Ok(())
    }

    /// Detect performance regressions
    #[instrument(skip(self, metrics))]
    fn detect_performance_regressions(&self, metrics: &BuildMetrics) -> Result<()> {
        let history = self.metrics_history.read().map_err(|_| CursedError::system_error("Failed to lock metrics history"))?;
        
        if history.len() < 5 {
            return Ok(()); // Need some history for comparison
        }

        // Calculate baseline performance
        let recent_builds = &history[history.len().saturating_sub(5)..history.len().saturating_sub(1)];
        let avg_build_time = recent_builds.iter()
            .map(|m| m.total_build_time.as_secs_f64())
            .sum::<f64>() / recent_builds.len() as f64;

        let current_build_time = metrics.total_build_time.as_secs_f64();
        let degradation = ((current_build_time - avg_build_time) / avg_build_time) * 100.0;

        if degradation > self.config.regression_threshold_percent {
            let alert = PerformanceAlert {
                alert_type: AlertType::BuildTimeIncrease,
                severity: if degradation > 50.0 { AlertSeverity::Critical } else { AlertSeverity::Warning },
                description: format!("Build time increased by {:.1}% compared to recent average", degradation),
                affected_files: Vec::new(),
                performance_degradation: degradation,
                detected_at: current_timestamp(),
                suggested_actions: vec![
                    "Check for new dependencies that might be slowing compilation".to_string(),
                    "Review recent code changes for performance issues".to_string(),
                    "Consider enabling more aggressive caching".to_string(),
                ],
            };

            let mut trend_data = self.trend_data.write().map_err(|_| CursedError::system_error("Failed to lock trend data"))?;
            trend_data.performance_regression_alerts.push(alert);

            warn!(degradation, "Performance regression detected");
        }

        Ok(())
    }

    /// Start system resource monitoring
    fn start_system_monitoring(&self) -> Result<()> {
        if !self.config.enable_memory_profiling && !self.config.enable_cpu_profiling {
            return Ok(());
        }

        let system_monitor = Arc::clone(&self.system_monitor);
        let sampling_interval = Duration::from_millis(self.config.sampling_interval_ms);
        let enable_memory = self.config.enable_memory_profiling;
        let enable_cpu = self.config.enable_cpu_profiling;

        thread::spawn(move || {
            use sysinfo::{System, SystemExt, ProcessExt, Pid};
            let mut sys = System::new_all();
            let current_pid = Pid::from(std::process::id() as usize);

            loop {
                sys.refresh_all();
                let now = Instant::now();
                
                let mut monitor = match system_monitor.lock() {
                    Ok(monitor) => monitor,
                    Err(_) => break,
                };

                // Check if we should continue monitoring
                if now.duration_since(monitor.last_sample_time) < sampling_interval {
                    drop(monitor);
                    thread::sleep(Duration::from_millis(10));
                    continue;
                }

                // Sample memory usage
                if enable_memory {
                    if let Some(process) = sys.process(current_pid) {
                        let memory_mb = process.memory() as f64 / (1024.0 * 1024.0);
                        monitor.memory_samples.push_back(memory_mb);
                        
                        // Limit sample history
                        if monitor.memory_samples.len() > 1000 {
                            monitor.memory_samples.pop_front();
                        }
                    }
                }

                // Sample CPU usage
                if enable_cpu {
                    if let Some(process) = sys.process(current_pid) {
                        let cpu_percent = process.cpu_usage() as f64;
                        monitor.cpu_samples.push_back(cpu_percent);
                        
                        // Limit sample history
                        if monitor.cpu_samples.len() > 1000 {
                            monitor.cpu_samples.pop_front();
                        }
                    }
                }

                monitor.last_sample_time = now;
                drop(monitor);
                
                thread::sleep(sampling_interval);
            }
        });

        debug!("Started system monitoring with {}ms interval", self.config.sampling_interval_ms);
        Ok(())
    }

    /// Stop system resource monitoring
    fn stop_system_monitoring(&self) -> Result<()> {
        // The monitoring thread will stop when the system_monitor mutex is dropped
        // or when the process exits. For graceful shutdown, we could use a shutdown signal.
        debug!("Stopped system monitoring");
        Ok(())
    }

    /// Analyze the current build
    fn analyze_build(&self) -> Result<BuildMetrics> {
        let events = self.current_build_events.lock().map_err(|_| CursedError::system_error("Failed to lock current events"))?;
        
        let mut metrics = BuildMetrics {
            total_build_time: Duration::ZERO,
            compilation_time: Duration::ZERO,
            linking_time: Duration::ZERO,
            dependency_resolution_time: Duration::ZERO,
            cache_time: Duration::ZERO,
            optimization_time: Duration::ZERO,
            files_compiled: 0,
            cache_hit_rate: 0.0,
            memory_peak_mb: 0.0,
            memory_average_mb: 0.0,
            cpu_average_percent: 0.0,
            parallelism_efficiency: 0.0,
            network_time: Duration::ZERO,
            disk_io_time: Duration::ZERO,
        };

        let mut cache_hits = 0;
        let mut cache_misses = 0;
        let mut memory_samples = Vec::new();
        let mut cpu_samples = Vec::new();

        let start_time = events.first().map(|e| e.timestamp).unwrap_or(0);
        let end_time = events.last().map(|e| e.timestamp).unwrap_or(0);
        metrics.total_build_time = Duration::from_secs(end_time - start_time);

        for event in events.iter() {
            match event.event_type {
                BuildEventType::CompilationEnd if event.success => {
                    metrics.compilation_time += event.duration;
                    metrics.files_compiled += 1;
                }
                BuildEventType::Linking => metrics.linking_time += event.duration,
                BuildEventType::DependencyResolution => metrics.dependency_resolution_time += event.duration,
                BuildEventType::CacheHit => {
                    cache_hits += 1;
                    metrics.cache_time += event.duration;
                }
                BuildEventType::CacheMiss => cache_misses += 1,
                BuildEventType::OptimizationPass => metrics.optimization_time += event.duration,
                BuildEventType::NetworkRequest => metrics.network_time += event.duration,
                BuildEventType::FileIO => metrics.disk_io_time += event.duration,
                _ => {}
            }

            memory_samples.push(event.memory_usage_mb);
            cpu_samples.push(event.cpu_usage_percent);
        }

        // Calculate derived metrics
        if cache_hits + cache_misses > 0 {
            metrics.cache_hit_rate = cache_hits as f64 / (cache_hits + cache_misses) as f64;
        }

        if !memory_samples.is_empty() {
            metrics.memory_peak_mb = memory_samples.iter().copied().fold(0.0, f64::max);
            metrics.memory_average_mb = memory_samples.iter().sum::<f64>() / memory_samples.len() as f64;
        }

        if !cpu_samples.is_empty() {
            metrics.cpu_average_percent = cpu_samples.iter().sum::<f64>() / cpu_samples.len() as f64;
        }

        // Calculate parallelism efficiency (simplified)
        if metrics.files_compiled > 0 && metrics.total_build_time.as_secs() > 0 {
            let theoretical_sequential_time = metrics.compilation_time.as_secs_f64();
            let actual_time = metrics.total_build_time.as_secs_f64();
            metrics.parallelism_efficiency = theoretical_sequential_time / actual_time;
        }

        Ok(metrics)
    }

    /// Load historical analytics data
    fn load_historical_data(&self) -> Result<()> {
        let data_path = &self.config.analytics_data_path;
        
        // Load metrics history
        let metrics_file = data_path.join("metrics_history.json");
        if metrics_file.exists() {
            match std::fs::read_to_string(&metrics_file) {
                Ok(content) => {
                    match serde_json::from_str::<Vec<BuildMetrics>>(&content) {
                        Ok(metrics) => {
                            let mut history = self.metrics_history.write()
                                .map_err(|_| CursedError::system_error("Failed to lock metrics history"))?;
                            *history = metrics;
                            info!("Loaded {} historical metrics entries", history.len());
                        }
                        Err(e) => warn!("Failed to parse metrics history: {}", e),
                    }
                }
                Err(e) => warn!("Failed to read metrics history file: {}", e),
            }
        }

        // Load trend data
        let trends_file = data_path.join("trend_analysis.json");
        if trends_file.exists() {
            match std::fs::read_to_string(&trends_file) {
                Ok(content) => {
                    match serde_json::from_str::<TrendAnalysis>(&content) {
                        Ok(trends) => {
                            let mut trend_data = self.trend_data.write()
                                .map_err(|_| CursedError::system_error("Failed to lock trend data"))?;
                            *trend_data = trends;
                            info!("Loaded trend analysis data");
                        }
                        Err(e) => warn!("Failed to parse trend data: {}", e),
                    }
                }
                Err(e) => warn!("Failed to read trend data file: {}", e),
            }
        }

        // Load performance baselines
        let baselines_file = data_path.join("baselines.json");
        if baselines_file.exists() {
            match std::fs::read_to_string(&baselines_file) {
                Ok(content) => {
                    match serde_json::from_str::<HashMap<String, f64>>(&content) {
                        Ok(baselines) => {
                            let mut performance_baselines = self.performance_baselines.write()
                                .map_err(|_| CursedError::system_error("Failed to lock baselines"))?;
                            *performance_baselines = baselines;
                            info!("Loaded {} performance baselines", performance_baselines.len());
                        }
                        Err(e) => warn!("Failed to parse baselines: {}", e),
                    }
                }
                Err(e) => warn!("Failed to read baselines file: {}", e),
            }
        }

        debug!("Historical analytics data loading completed");
        Ok(())
    }

    /// Generate optimization recommendations
    fn generate_recommendations(&self) -> Result<Vec<String>> {
        let mut recommendations = Vec::new();
        
        // Get recent metrics for analysis
        let history = self.metrics_history.read()
            .map_err(|_| CursedError::system_error("Failed to lock metrics history"))?;
        
        if history.is_empty() {
            return Ok(vec!["Collect more build data to generate recommendations".to_string()]);
        }
        
        let latest = &history[history.len() - 1];
        
        // Analyze cache hit rate
        if latest.cache_hit_rate < 0.5 {
            recommendations.push(format!(
                "Cache hit rate is low ({:.1}%). Consider enabling more aggressive caching or checking cache invalidation logic.",
                latest.cache_hit_rate * 100.0
            ));
        }
        
        // Analyze build time
        if latest.total_build_time.as_secs() > 60 {
            recommendations.push(format!(
                "Build time is {}s. Consider increasing parallelization or enabling incremental compilation.",
                latest.total_build_time.as_secs()
            ));
        }
        
        // Analyze memory usage
        if latest.memory_peak_mb > 2048.0 {
            recommendations.push(format!(
                "Peak memory usage is {:.1}MB. Consider enabling streaming compilation or reducing concurrent jobs.",
                latest.memory_peak_mb
            ));
        }
        
        // Analyze parallelism efficiency
        if latest.parallelism_efficiency < 0.7 {
            recommendations.push(format!(
                "Parallelism efficiency is {:.1}%. Review dependency graph for better parallelization opportunities.",
                latest.parallelism_efficiency * 100.0
            ));
        }
        
        // Compare with historical data if available
        if history.len() >= 5 {
            let recent_avg_time = history.iter()
                .rev()
                .take(5)
                .map(|m| m.total_build_time.as_secs_f64())
                .sum::<f64>() / 5.0;
            
            let older_avg_time = history.iter()
                .rev()
                .skip(5)
                .take(5)
                .map(|m| m.total_build_time.as_secs_f64())
                .sum::<f64>() / 5.0;
            
            if recent_avg_time > older_avg_time * 1.2 {
                recommendations.push(format!(
                    "Build times have increased by {:.1}% recently. Review recent changes for performance impacts.",
                    ((recent_avg_time - older_avg_time) / older_avg_time) * 100.0
                ));
            }
        }
        
        // Analyze compilation vs linking time ratio
        let compile_ratio = latest.compilation_time.as_secs_f64() / latest.total_build_time.as_secs_f64();
        if compile_ratio < 0.3 {
            recommendations.push(
                "Linking takes a large portion of build time. Consider splitting large binaries or using dynamic linking.".to_string()
            );
        }
        
        // Network time analysis
        if latest.network_time.as_secs() > 5 {
            recommendations.push(format!(
                "Network operations take {}s. Consider using local package cache or faster registry.",
                latest.network_time.as_secs()
            ));
        }
        
        // Add general recommendations if no specific issues found
        if recommendations.is_empty() {
            recommendations.push("Build performance looks good! Consider enabling advanced optimizations for even better performance.".to_string());
        }
        
        Ok(recommendations)
    }

    /// Generate performance comparison with historical data
    fn generate_performance_comparison(&self) -> Result<PerformanceComparison> {
        let history = self.metrics_history.read()
            .map_err(|_| CursedError::system_error("Failed to lock metrics history"))?;
        
        if history.len() < 2 {
            return Ok(PerformanceComparison {
                compared_to_last_build: 0.0,
                compared_to_average: 0.0,
                compared_to_best: 0.0,
                trend_direction: TrendDirection::Stable,
            });
        }
        
        let latest = &history[history.len() - 1];
        let current_time = latest.total_build_time.as_secs_f64();
        
        // Compare to last build
        let last_build = &history[history.len() - 2];
        let last_time = last_build.total_build_time.as_secs_f64();
        let compared_to_last = if last_time > 0.0 {
            ((current_time - last_time) / last_time) * 100.0
        } else {
            0.0
        };
        
        // Compare to average of recent builds
        let recent_count = history.len().min(10);
        let recent_avg = history.iter()
            .rev()
            .take(recent_count)
            .map(|m| m.total_build_time.as_secs_f64())
            .sum::<f64>() / recent_count as f64;
        
        let compared_to_average = if recent_avg > 0.0 {
            ((current_time - recent_avg) / recent_avg) * 100.0
        } else {
            0.0
        };
        
        // Compare to best build time
        let best_time = history.iter()
            .map(|m| m.total_build_time.as_secs_f64())
            .fold(f64::INFINITY, f64::min);
        
        let compared_to_best = if best_time > 0.0 && best_time < f64::INFINITY {
            ((current_time - best_time) / best_time) * 100.0
        } else {
            0.0
        };
        
        // Determine trend direction
        let trend_direction = if history.len() >= 5 {
            let recent_trend = history.iter()
                .rev()
                .take(5)
                .map(|m| m.total_build_time.as_secs_f64())
                .collect::<Vec<_>>();
            
            // Simple linear regression to determine trend
            let n = recent_trend.len() as f64;
            let sum_x: f64 = (0..recent_trend.len()).map(|i| i as f64).sum();
            let sum_y: f64 = recent_trend.iter().sum();
            let sum_xy: f64 = recent_trend.iter().enumerate()
                .map(|(i, &y)| i as f64 * y)
                .sum();
            let sum_x2: f64 = (0..recent_trend.len()).map(|i| (i as f64).powi(2)).sum();
            
            let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x.powi(2));
            
            if slope > 0.1 {
                TrendDirection::Degrading
            } else if slope < -0.1 {
                TrendDirection::Improving
            } else {
                TrendDirection::Stable
            }
        } else {
            TrendDirection::Stable
        };
        
        Ok(PerformanceComparison {
            compared_to_last_build: compared_to_last,
            compared_to_average: compared_to_average,
            compared_to_best: compared_to_best,
            trend_direction,
        })
    }

    /// Save report to file
    fn save_report_to_file(&self, report: &BuildReport) -> Result<()> {
        let report_path = self.config.analytics_data_path
            .join(format!("build_report_{}.json", report.generated_at));
        
        let json = serde_json::to_string_pretty(report)?;
        std::fs::write(report_path, json)?;
        
        Ok(())
    }
}

/// Comprehensive build report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildReport {
    pub generated_at: u64,
    pub build_metrics: BuildMetrics,
    pub bottleneck_analysis: BottleneckAnalysis,
    pub trend_analysis: TrendAnalysis,
    pub recommendations: Vec<String>,
    pub performance_comparison: PerformanceComparison,
}

/// Performance comparison data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceComparison {
    pub compared_to_last_build: f64,    // percentage change
    pub compared_to_average: f64,       // percentage change
    pub compared_to_best: f64,          // percentage change
    pub trend_direction: TrendDirection,
}

/// Performance trend direction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Improving,
    Stable,
    Degrading,
}

/// Helper function to create a build event
pub fn create_build_event(event_type: BuildEventType, duration: Duration) -> BuildEvent {
    BuildEvent {
        id: uuid::Uuid::new_v4().to_string(),
        event_type,
        timestamp: current_timestamp(),
        duration,
        file_path: None,
        module_name: None,
        memory_usage_mb: 0.0,
        cpu_usage_percent: 0.0,
        success: true,
        error_message: None,
        metadata: HashMap::new(),
    }
}

/// Get current timestamp in seconds since epoch
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

// Export public API
pub use self::{

    BuildAnalyticsConfig,
    BuildEvent,
    BuildEventType,
    BuildMetrics,
    BottleneckAnalysis,
    BuildReport,
    OptimizationOpportunity,
    OptimizationCategory,
    PerformanceAlert,
    AlertType,
    AlertSeverity,

};
