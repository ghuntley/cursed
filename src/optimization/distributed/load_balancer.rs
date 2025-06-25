// Load Balancer for Distributed Compilation
//
// Intelligent distribution of compilation jobs across worker nodes based on
// various strategies and real-time performance metrics.

use crate::error::{CursedError, Result};

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant, SystemTime};
use tracing::{debug, info, instrument, warn};

use super::worker_node::{WorkerNode, WorkerStatus};
use super::distributed_compiler::CompilationJob;

/// Load balancing strategies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LoadBalancingStrategy {
    /// Simple round-robin assignment
    /// Assign to worker with lowest current load
    /// Weighted round-robin based on worker capabilities
    /// Performance-based assignment considering historical data
    /// Work-stealing with dynamic load balancing
    /// Adaptive strategy that changes based on workload
    /// Geographic or network-aware assignment
    /// Assign based on job type and worker specialization
/// Node selection result
#[derive(Debug, Clone)]
pub struct NodeSelection {
/// Load balancer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerConfig {
    /// Primary load balancing strategy
    /// Fallback strategy if primary fails
    /// Enable adaptive strategy switching
    /// Metrics window for performance calculations
    /// Load threshold for triggering rebalancing
    /// Enable work stealing
    /// Work stealing threshold
impl Default for LoadBalancerConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Performance metrics for load balancing decisions
#[derive(Debug, Clone)]
struct WorkerPerformanceMetrics {
    throughput: f64, // jobs per minute
    reliability: f64, // success rate
/// Single performance data point
#[derive(Debug, Clone)]
struct PerformanceDataPoint {
/// Load balancer state
#[derive(Debug)]
struct LoadBalancerState {
    job_assignments: HashMap<String, String>, // job_id -> worker_id
/// Load balancer for distributed compilation
pub struct LoadBalancer {
/// Tracks efficiency of different strategies
#[derive(Debug)]
struct EfficiencyTracker {
/// Score tracking for a strategy
#[derive(Debug, Clone)]
struct StrategyScore {
impl LoadBalancer {
    /// Create a new load balancer
    #[instrument]
    pub fn new(strategy: LoadBalancingStrategy, max_workers: usize) -> Result<Self> {
        let config = LoadBalancerConfig {
            ..LoadBalancerConfig::default()

        let state = LoadBalancerState {

        let efficiency_tracker = EfficiencyTracker {

        Ok(Self {
        })
    /// Start the load balancer
    #[instrument(skip(self))]
    pub async fn start(&mut self) -> Result<()> {
        // Start monitoring and rebalancing tasks
        self.start_performance_monitor().await?;
        self.start_rebalancer().await?;

        info!("Load balancer started with strategy: {:?}", self.config.strategy);
        Ok(())
    /// Stop the load balancer
    #[instrument(skip(self))]
    pub async fn stop(&mut self) -> Result<()> {
        info!("Load balancer stopped");
        Ok(())
    /// Add a worker to the load balancer
    #[instrument(skip(self, worker))]
    pub async fn add_worker(&mut self, worker: WorkerNode) -> Result<()> {
        let worker_id = worker.id.clone();
        
        {
            let mut state = self.state.write()
                .map_err(|_| CursedError::system_error("Failed to lock state"))?;
            
            state.workers.insert(worker_id.clone(), worker.clone());
            
            // Initialize performance metrics
            let metrics = WorkerPerformanceMetrics {
                average_job_time: Duration::from_secs(30), // Default estimate
                throughput: 2.0, // 2 jobs per minute default
                reliability: 1.0, // Start with 100% reliability
            
            state.performance_metrics.insert(worker_id.clone(), metrics);
            state.weighted_assignments.insert(worker_id.clone(), 0);
        info!(worker_id, "Worker added to load balancer");
        Ok(())
    /// Remove a worker from the load balancer
    #[instrument(skip(self))]
    pub async fn remove_worker(&mut self, worker_id: &str) -> Result<()> {
        {
            let mut state = self.state.write()
                .map_err(|_| CursedError::system_error("Failed to lock state"))?;
            
            state.workers.remove(worker_id);
            state.performance_metrics.remove(worker_id);
            state.weighted_assignments.remove(worker_id);
            
            // Remove job assignments for this worker
            state.job_assignments.retain(|_, w_id| w_id != worker_id);
        info!(worker_id, "Worker removed from load balancer");
        Ok(())
    /// Select the best worker for a job
    #[instrument(skip(self, job))]
    pub async fn select_worker(&self, job: &CompilationJob) -> Result<Option<NodeSelection>> {
        let state = self.state.read()
            .map_err(|_| CursedError::system_error("Failed to lock state"))?;

        let available_workers: Vec<&WorkerNode> = state.workers
            .values()
            .filter(|w| w.is_available() && w.supports_target(&job.target_triple))
            .collect();

        if available_workers.is_empty() {
            return Ok(None);
        let selection = match &state.current_strategy {
            LoadBalancingStrategy::RoundRobin => {
                self.select_round_robin(&available_workers, &state)
            }
            LoadBalancingStrategy::LeastLoaded => {
                self.select_least_loaded(&available_workers, &state)
            }
            LoadBalancingStrategy::WeightedRoundRobin => {
                self.select_weighted_round_robin(&available_workers, &state)
            }
            LoadBalancingStrategy::PerformanceBased => {
                self.select_performance_based(&available_workers, &state, job)
            }
            LoadBalancingStrategy::WorkStealing => {
                self.select_work_stealing(&available_workers, &state)
            }
            LoadBalancingStrategy::AdaptiveWeighted => {
                self.select_adaptive_weighted(&available_workers, &state, job)
            }
            LoadBalancingStrategy::LocalityAware => {
                self.select_locality_aware(&available_workers, &state, job)
            }
            LoadBalancingStrategy::SpecializationBased => {
                self.select_specialization_based(&available_workers, &state, job)
            }

        if let Some(ref selection) = selection {
            debug!(
                "Worker selected for job"
            );
        Ok(selection)
    /// Get active workers
    pub async fn get_active_workers(&self) -> Result<Vec<WorkerNode>> {
        let state = self.state.read()
            .map_err(|_| CursedError::system_error("Failed to lock state"))?;
        
        Ok(state.workers
            .values()
            .filter(|w| matches!(w.status, WorkerStatus::Online | WorkerStatus::Busy))
            .cloned()
            .collect())
    /// Get current active worker count
    pub async fn active_worker_count(&self) -> Result<usize> {
        let workers = self.get_active_workers().await?;
        Ok(workers.len())
    /// Update strategy dynamically
    #[instrument(skip(self))]
    pub async fn update_strategy(&mut self, new_strategy: LoadBalancingStrategy) -> Result<()> {
        {
            let mut state = self.state.write()
                .map_err(|_| CursedError::system_error("Failed to lock state"))?;
            state.current_strategy = new_strategy.clone();
        {
            let mut tracker = self.efficiency_tracker.lock()
                .map_err(|_| CursedError::system_error("Failed to lock efficiency tracker"))?;
            tracker.switch_count += 1;
        info!(new_strategy = ?new_strategy, "Load balancing strategy updated");
        Ok(())
    /// Record job completion for performance tracking
    #[instrument(skip(self))]
    pub async fn record_job_completion(
    ) -> Result<()> {
        {
            let mut state = self.state.write()
                .map_err(|_| CursedError::system_error("Failed to lock state"))?;
            
            if let Some(metrics) = state.performance_metrics.get_mut(worker_id) {
                let data_point = PerformanceDataPoint {
                
                metrics.historical_performance.push_back(data_point);
                
                // Keep only recent data points
                let cutoff = SystemTime::now() - self.config.metrics_window;
                while let Some(front) = metrics.historical_performance.front() {
                    if front.timestamp < cutoff {
                        metrics.historical_performance.pop_front();
                    } else {
                        break;
                    }
                }
                
                // Update aggregated metrics
                self.update_aggregated_metrics(metrics);
            // Remove job assignment
            state.job_assignments.remove(job_id);
        // Update strategy performance tracking
        self.update_strategy_performance(worker_id, duration, success).await?;

        debug!(job_id, worker_id, duration = ?duration, success, "Job completion recorded");
        Ok(())
    /// Get current efficiency score
    pub async fn get_efficiency(&self) -> Result<f64> {
        let tracker = self.efficiency_tracker.lock()
            .map_err(|_| CursedError::system_error("Failed to lock efficiency tracker"))?;
        
        let state = self.state.read()
            .map_err(|_| CursedError::system_error("Failed to lock state"))?;

        if let Some(score) = tracker.strategy_scores.get(&state.current_strategy) {
            Ok(score.load_balance_quality)
        } else {
            Ok(0.0)
        }
    }

    /// Round-robin selection
    fn select_round_robin(
    ) -> Option<NodeSelection> {
        if workers.is_empty() {
            return None;
        let index = state.round_robin_index % workers.len();
        let worker = workers[index];
        
        Some(NodeSelection {
        })
    /// Least loaded selection
    fn select_least_loaded(
    ) -> Option<NodeSelection> {
        let best_worker = workers
            .iter()
            .min_by(|a, b| {
                a.load_factor().partial_cmp(&b.load_factor())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })?;

        let load = best_worker.load_factor();
        
        Some(NodeSelection {
            confidence: 1.0 - load, // Higher confidence for lower load
            load_after_assignment: load + (1.0 / best_worker.capabilities.max_concurrent_jobs as f64),
        })
    /// Weighted round-robin selection
    fn select_weighted_round_robin(
    ) -> Option<NodeSelection> {
        // Calculate weights based on performance scores
        let mut weighted_workers = Vec::new();
        for worker in workers {
            let weight = (worker.capabilities.performance_score * 10.0) as usize;
            for _ in 0..weight.max(1) {
                weighted_workers.push(worker);
            }
        }

        if weighted_workers.is_empty() {
            return None;
        let index = state.round_robin_index % weighted_workers.len();
        let worker = weighted_workers[index];
        
        Some(NodeSelection {
        })
    /// Performance-based selection
    fn select_performance_based(
    ) -> Option<NodeSelection> {
        let mut best_worker = None;
        let mut best_score = 0.0;

        for worker in workers {
            let mut score = worker.efficiency_score();
            
            // Adjust score based on historical performance
            if let Some(metrics) = state.performance_metrics.get(&worker.id) {
                score *= metrics.reliability;
                score *= 1.0 / (metrics.average_job_time.as_secs_f64() / 30.0).max(0.1);
            // Penalty for high load
            score *= 1.0 - worker.load_factor() * 0.5;
            
            if score > best_score {
                best_score = score;
                best_worker = Some(worker);
            }
        }

        best_worker.map(|worker| NodeSelection {
            confidence: (best_score / 10.0).min(1.0),
        })
    /// Work-stealing selection
    fn select_work_stealing(
    ) -> Option<NodeSelection> {
        // Find workers with very low load that can steal work
        let idle_workers: Vec<_> = workers
            .iter()
            .filter(|w| w.load_factor() < self.config.work_steal_threshold)
            .collect();

        if !idle_workers.is_empty() {
            return self.select_least_loaded(&idle_workers, state);
        // Fallback to least loaded
        self.select_least_loaded(workers, state)
    /// Adaptive weighted selection
    fn select_adaptive_weighted(
    ) -> Option<NodeSelection> {
        // Combine multiple factors with adaptive weights
        let mut best_worker = None;
        let mut best_score = 0.0;

        for worker in workers {
            let mut score = 0.0;
            
            // Performance factor (40% weight)
            score += worker.efficiency_score() * 0.4;
            
            // Load factor (30% weight)
            score += (1.0 - worker.load_factor()) * 0.3;
            
            // Reliability factor (20% weight)
            if let Some(metrics) = state.performance_metrics.get(&worker.id) {
                score += metrics.reliability * 0.2;
            } else {
                score += 0.2; // Default reliability
            // Specialization factor (10% weight)
            if worker.supports_target(&job.target_triple) {
                score += 0.1;
            if score > best_score {
                best_score = score;
                best_worker = Some(worker);
            }
        }

        best_worker.map(|worker| NodeSelection {
        })
    /// Locality-aware selection
    fn select_locality_aware(
    ) -> Option<NodeSelection> {
        // For now, prefer local workers (127.0.0.1)
        let local_workers: Vec<_> = workers
            .iter()
            .filter(|w| w.address.ip().is_loopback())
            .collect();

        if !local_workers.is_empty() {
            return self.select_least_loaded(&local_workers, state);
        // Fallback to performance-based selection
        self.select_least_loaded(workers, state)
    /// Specialization-based selection
    fn select_specialization_based(
    ) -> Option<NodeSelection> {
        // Find workers with specific capabilities for this job
        let specialized_workers: Vec<_> = workers
            .iter()
            .filter(|w| {
                // Check for optimization level support
                if job.optimization_level == "O3" && w.capabilities.features.contains(&"fast-compilation".to_string()) {
                    return true;
                // Check for target architecture specialization
                w.supports_target(&job.target_triple)
            })
            .collect();

        if !specialized_workers.is_empty() {
            return self.select_performance_based(&specialized_workers, state, job);
        // Fallback to performance-based selection
        self.select_performance_based(workers, state, job)
    /// Estimate job completion time on a worker
    fn estimate_completion_time(&self, worker: &WorkerNode, state: &LoadBalancerState) -> Duration {
        if let Some(metrics) = state.performance_metrics.get(&worker.id) {
            // Adjust based on current load
            let load_factor = worker.load_factor();
            let base_time = metrics.average_job_time;
            let adjusted_time = base_time.mul_f64(1.0 + load_factor);
            adjusted_time
        } else {
            // Default estimate
            Duration::from_secs(30)
        }
    }

    /// Update aggregated performance metrics
    fn update_aggregated_metrics(&self, metrics: &mut WorkerPerformanceMetrics) {
        if metrics.historical_performance.is_empty() {
            return;
        let total_jobs = metrics.historical_performance.len();
        let successful_jobs = metrics.historical_performance
            .iter()
            .filter(|p| p.job_success)
            .count();

        metrics.reliability = successful_jobs as f64 / total_jobs as f64;

        let total_duration: Duration = metrics.historical_performance
            .iter()
            .map(|p| p.job_duration)
            .sum();
        
        metrics.average_job_time = total_duration / total_jobs as u32;

        // Calculate throughput (jobs per minute)
        if let (Some(first), Some(last)) = (
        ) {
            let time_span = last.timestamp
                .duration_since(first.timestamp)
                .unwrap_or(Duration::from_secs(1));
            
            metrics.throughput = total_jobs as f64 / time_span.as_secs_f64() * 60.0;
        metrics.last_updated = SystemTime::now();
    /// Update strategy performance tracking
    async fn update_strategy_performance(
    ) -> Result<()> {
        let mut tracker = self.efficiency_tracker.lock()
            .map_err(|_| CursedError::system_error("Failed to lock efficiency tracker"))?;
        
        let state = self.state.read()
            .map_err(|_| CursedError::system_error("Failed to lock state"))?;

        let strategy = &state.current_strategy;
        let score = tracker.strategy_scores
            .entry(strategy.clone())
            .or_insert_with(|| StrategyScore {
            });

        score.total_jobs += 1;
        if success {
            score.successful_jobs += 1;
        // Update average completion time
        let alpha = 0.1; // Exponential moving average factor
        let new_avg = score.average_completion_time.as_secs_f64() * (1.0 - alpha)
                     + duration.as_secs_f64() * alpha;
        score.average_completion_time = Duration::from_secs_f64(new_avg);

        score.last_used = SystemTime::now();
        tracker.total_jobs_processed += 1;

        Ok(())
    /// Start performance monitoring task
    async fn start_performance_monitor(&self) -> Result<()> {
        // In a real implementation, this would spawn a background task
        // to continuously monitor and update worker performance metrics
        Ok(())
    /// Start rebalancing task
    async fn start_rebalancer(&self) -> Result<()> {
        // In a real implementation, this would spawn a background task
        // to periodically evaluate and potentially switch strategies
        Ok(())
    }
}

