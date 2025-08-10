use crate::error::{Result, CursedError};
use crate::optimization::distributed::worker_node::{WorkerNode, WorkerNodeManager};
use crate::optimization::distributed::distributed_compiler::CompilationJob;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Load balancing strategies
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LoadBalancingStrategy {
    RoundRobin,
    LeastLoaded,
    WeightedRoundRobin,
    ConsistentHashing,
    Adaptive,
}

/// Node selection result
#[derive(Debug, Clone)]
pub struct NodeSelection {
    pub worker_id: String,
    pub estimated_completion_time: std::time::Duration,
    pub confidence: f64,
}

/// Load balancer for distributed compilation
#[derive(Debug)]
pub struct LoadBalancer {
    strategy: LoadBalancingStrategy,
    worker_manager: Arc<RwLock<WorkerNodeManager>>,
    round_robin_index: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    node_weights: HashMap<String, f64>,
    selection_history: Vec<NodeSelection>,
}

impl LoadBalancer {
    pub fn new(strategy: LoadBalancingStrategy) -> Result<Self> {
        Ok(Self {
            strategy,
            worker_manager: Arc::new(RwLock::new(WorkerNodeManager::new())),
            round_robin_index: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            node_weights: HashMap::new(),
            selection_history: Vec::new(),
        })
    }

    pub async fn start(&mut self) -> Result<()> {
        tracing::info!("Starting load balancer with strategy: {:?}", self.strategy);
        
        // Initialize node weights based on capabilities
        let worker_manager = self.worker_manager.read().await;
        for worker in worker_manager.get_available_workers() {
            let weight = self.calculate_node_weight(worker);
            self.node_weights.insert(worker.id.clone(), weight);
        }

        Ok(())
    }

    pub async fn stop(&mut self) -> Result<()> {
        tracing::info!("Stopping load balancer");
        Ok(())
    }

    pub async fn select_node(&mut self, job: &CompilationJob) -> Result<NodeSelection> {
        let worker_data: Vec<WorkerNode> = {
            let worker_manager = self.worker_manager.read().await;
            let available_workers = worker_manager.get_available_workers();

            if available_workers.is_empty() {
                return Err(CursedError::system_error("No available workers"));
            }

            // Filter workers that can handle this job
            available_workers
                .into_iter()
                .filter(|w| w.can_handle_job(&job.target_platform))
                .cloned()
                .collect()
        };

        if worker_data.is_empty() {
            return Err(CursedError::system_error(&format!(
                "No workers available for target: {}", job.target_platform
            )));
        }

        let worker_refs: Vec<&WorkerNode> = worker_data.iter().collect();
        
        let selected_worker = match self.strategy {
            LoadBalancingStrategy::RoundRobin => {
                self.select_round_robin(&worker_refs)
            }
            LoadBalancingStrategy::LeastLoaded => {
                self.select_least_loaded(&worker_refs)
            }
            LoadBalancingStrategy::WeightedRoundRobin => {
                self.select_weighted_round_robin(&worker_refs)
            }
            LoadBalancingStrategy::ConsistentHashing => {
                self.select_consistent_hash(&worker_refs, job)
            }
            LoadBalancingStrategy::Adaptive => {
                self.select_adaptive(&worker_refs, job)
            }
        };

        let selection = NodeSelection {
            worker_id: selected_worker.id.clone(),
            estimated_completion_time: self.estimate_completion_time(selected_worker, job),
            confidence: self.calculate_confidence(selected_worker, job),
        };

        // Record selection in history
        self.selection_history.push(selection.clone());
        
        // Keep only recent history
        if self.selection_history.len() > 100 {
            self.selection_history.remove(0);
        }

        tracing::debug!("Selected worker {} for job {}", selection.worker_id, job.id);
        Ok(selection)
    }

    pub async fn add_worker(&mut self, worker: WorkerNode) -> Result<()> {
        let weight = self.calculate_node_weight(&worker);
        self.node_weights.insert(worker.id.clone(), weight);
        
        let mut worker_manager = self.worker_manager.write().await;
        worker_manager.add_worker(worker)
    }

    pub async fn remove_worker(&mut self, worker_id: &str) -> Result<()> {
        self.node_weights.remove(worker_id);
        
        let mut worker_manager = self.worker_manager.write().await;
        worker_manager.remove_worker(worker_id)
    }

    pub async fn update_strategy(&mut self, new_strategy: LoadBalancingStrategy) -> Result<()> {
        tracing::info!("Updating load balancing strategy from {:?} to {:?}", 
                      self.strategy, new_strategy);
        self.strategy = new_strategy;
        Ok(())
    }

    pub async fn get_active_workers(&self) -> Result<Vec<WorkerNode>> {
        let worker_manager = self.worker_manager.read().await;
        Ok(worker_manager.get_available_workers().into_iter().cloned().collect())
    }

    pub async fn active_worker_count(&self) -> Result<usize> {
        let worker_manager = self.worker_manager.read().await;
        Ok(worker_manager.get_healthy_worker_count())
    }

    pub async fn get_efficiency(&self) -> Result<f64> {
        let worker_manager = self.worker_manager.read().await;
        let utilization = worker_manager.get_current_utilization();
        
        // Calculate efficiency based on utilization and selection history
        let recent_selections = self.selection_history.len().min(10);
        let avg_confidence = if recent_selections > 0 {
            self.selection_history.iter()
                .rev()
                .take(recent_selections)
                .map(|s| s.confidence)
                .sum::<f64>() / recent_selections as f64
        } else {
            0.5
        };

        Ok(utilization * avg_confidence)
    }

    // Private helper methods

    fn select_round_robin<'a>(&mut self, workers: &[&'a WorkerNode]) -> &'a WorkerNode {
        let index = self.round_robin_index.fetch_add(1, std::sync::atomic::Ordering::Relaxed) % workers.len();
        workers[index]
    }

    fn select_least_loaded<'a>(&self, workers: &[&'a WorkerNode]) -> &'a WorkerNode {
        workers.iter()
            .min_by(|a, b| a.get_load_factor().partial_cmp(&b.get_load_factor()).unwrap())
            .unwrap()
    }

    fn select_weighted_round_robin<'a>(&mut self, workers: &[&'a WorkerNode]) -> &'a WorkerNode {
        // Select based on weights, with higher weights having higher probability
        let total_weight: f64 = workers.iter()
            .map(|w| self.node_weights.get(&w.id).copied().unwrap_or(1.0))
            .sum();

        if total_weight == 0.0 {
            return self.select_round_robin(workers);
        }

        let mut cumulative_weight = 0.0;
        let target = fastrand::f64() * total_weight;

        for worker in workers {
            cumulative_weight += self.node_weights.get(&worker.id).copied().unwrap_or(1.0);
            if cumulative_weight >= target {
                return worker;
            }
        }

        // Fallback to first worker
        workers[0]
    }

    fn select_consistent_hash<'a>(&self, workers: &[&'a WorkerNode], job: &CompilationJob) -> &'a WorkerNode {
        // Use job ID for consistent hashing
        let hash = {
            use std::hash::{Hash, Hasher};
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            job.id.hash(&mut hasher);
            hasher.finish()
        };

        let index = (hash as usize) % workers.len();
        workers[index]
    }

    fn select_adaptive<'a>(&mut self, workers: &[&'a WorkerNode], job: &CompilationJob) -> &'a WorkerNode {
        // Adaptive selection based on historical performance and current load
        let mut best_worker = workers[0];
        let mut best_score = 0.0;

        for worker in workers {
            let load_factor = worker.get_load_factor();
            let success_rate = worker.get_success_rate();
            let weight = self.node_weights.get(&worker.id).copied().unwrap_or(1.0);
            
            // Score based on multiple factors
            let score = (1.0 - load_factor) * 0.4 + 
                       success_rate * 0.3 + 
                       weight * 0.2 + 
                       self.calculate_job_affinity(worker, job) * 0.1;

            if score > best_score {
                best_score = score;
                best_worker = worker;
            }
        }

        best_worker
    }

    fn calculate_node_weight(&self, worker: &WorkerNode) -> f64 {
        // Weight based on capabilities
        let cpu_weight = worker.capabilities.cpu_cores as f64 / 16.0; // Normalize to 16 cores
        let memory_weight = worker.capabilities.memory_gb as f64 / 32.0; // Normalize to 32GB
        let network_weight = worker.capabilities.network_bandwidth_mbps as f64 / 1000.0; // Normalize to 1Gbps

        (cpu_weight + memory_weight + network_weight) / 3.0
    }

    fn estimate_completion_time(&self, worker: &WorkerNode, job: &CompilationJob) -> std::time::Duration {
        // Base estimation on job's estimated duration and worker's average
        let base_time = job.estimated_duration;
        let worker_avg = worker.metrics.average_job_duration;
        
        // Adjust based on current load
        let load_multiplier = 1.0 + worker.get_load_factor() * 0.5;
        
        // Combine estimates
        let estimated_secs = if worker_avg.as_secs() > 0 {
            (base_time.as_secs_f64() + worker_avg.as_secs_f64()) / 2.0 * load_multiplier
        } else {
            base_time.as_secs_f64() * load_multiplier
        };

        std::time::Duration::from_secs_f64(estimated_secs)
    }

    fn calculate_confidence(&self, worker: &WorkerNode, _job: &CompilationJob) -> f64 {
        // Confidence based on worker reliability and load
        let success_rate = worker.get_success_rate();
        let load_factor = worker.get_load_factor();
        let health_factor = if worker.is_healthy() { 1.0 } else { 0.5 };
        
        success_rate * (1.0 - load_factor * 0.3) * health_factor
    }

    fn calculate_job_affinity(&self, worker: &WorkerNode, job: &CompilationJob) -> f64 {
        // Simple affinity based on target platform support
        if worker.capabilities.supported_targets.contains(&job.target_platform) {
            1.0
        } else {
            0.0
        }
    }
}
