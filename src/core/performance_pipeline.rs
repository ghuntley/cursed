// Performance pipeline and configuration management
use crate::error::CursedError;
use std::time::{Duration, Instant};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PerformancePipeline {
    stages: Vec<PipelineStage>,
    config: ParallelConfig,
    incremental_config: IncrementalConfig,
    progress_config: ProgressConfig,
}

#[derive(Debug, Clone)]
pub struct PipelineStage {
    pub name: String,
    pub duration: Duration,
    pub start_time: Option<Instant>,
}

#[derive(Debug, Clone)]
pub struct ParallelConfig {
    pub max_threads: usize,
    pub enable_parallel: bool,
    pub chunk_size: usize,
}

#[derive(Debug, Clone)]
pub struct IncrementalConfig {
    pub enable_incremental: bool,
    pub cache_size: usize,
    pub cache_dir: String,
}

#[derive(Debug, Clone)]
pub struct ProgressConfig {
    pub show_progress: bool,
    pub update_interval: Duration,
    pub detailed_output: bool,
}

impl PerformancePipeline {
    pub fn new() -> Self {
        Self {
            stages: Vec::new(),
            config: ParallelConfig::default(),
            incremental_config: IncrementalConfig::default(),
            progress_config: ProgressConfig::default(),
        }
    }

    pub fn with_config(
        parallel: ParallelConfig,
        incremental: IncrementalConfig,
        progress: ProgressConfig,
    ) -> Self {
        Self {
            stages: Vec::new(),
            config: parallel,
            incremental_config: incremental,
            progress_config: progress,
        }
    }

    pub fn add_stage(&mut self, name: String) {
        self.stages.push(PipelineStage {
            name,
            duration: Duration::new(0, 0),
            start_time: None,
        });
    }

    pub fn start_stage(&mut self, name: &str) -> Result<(), CursedError> {
        if let Some(stage) = self.stages.iter_mut().find(|s| s.name == name) {
            stage.start_time = Some(Instant::now());
            Ok(())
        } else {
            Err(CursedError::runtime_error(&format!("Stage '{}' not found", name)))
        }
    }

    pub fn end_stage(&mut self, name: &str) -> Result<Duration, CursedError> {
        if let Some(stage) = self.stages.iter_mut().find(|s| s.name == name) {
            if let Some(start) = stage.start_time.take() {
                stage.duration = start.elapsed();
                Ok(stage.duration)
            } else {
                Err(CursedError::runtime_error(&format!("Stage '{}' was not started", name)))
            }
        } else {
            Err(CursedError::runtime_error(&format!("Stage '{}' not found", name)))
        }
    }

    pub fn get_total_duration(&self) -> Duration {
        self.stages.iter().map(|s| s.duration).sum()
    }

    pub fn get_stage_report(&self) -> HashMap<String, Duration> {
        self.stages.iter().map(|s| (s.name.clone(), s.duration)).collect()
    }

    pub fn is_parallel_enabled(&self) -> bool {
        self.config.enable_parallel
    }

    pub fn is_incremental_enabled(&self) -> bool {
        self.incremental_config.enable_incremental
    }

    pub fn should_show_progress(&self) -> bool {
        self.progress_config.show_progress
    }
}

impl Default for ParallelConfig {
    fn default() -> Self {
        Self {
            max_threads: num_cpus::get(),
            enable_parallel: true,
            chunk_size: 1000,
        }
    }
}

impl Default for IncrementalConfig {
    fn default() -> Self {
        Self {
            enable_incremental: true,
            cache_size: 1024 * 1024 * 100, // 100MB
            cache_dir: ".cursed_cache".to_string(),
        }
    }
}

impl Default for ProgressConfig {
    fn default() -> Self {
        Self {
            show_progress: true,
            update_interval: Duration::from_millis(100),
            detailed_output: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_pipeline_creation() {
        let pipeline = PerformancePipeline::new();
        assert_eq!(pipeline.stages.len(), 0);
        assert!(pipeline.is_parallel_enabled());
        assert!(pipeline.is_incremental_enabled());
        assert!(pipeline.should_show_progress());
    }

    #[test]
    fn test_stage_management() {
        let mut pipeline = PerformancePipeline::new();
        pipeline.add_stage("parse".to_string());
        pipeline.add_stage("codegen".to_string());
        
        assert_eq!(pipeline.stages.len(), 2);
        assert!(pipeline.start_stage("parse").is_ok());
        
        // Simulate some work
        std::thread::sleep(Duration::from_millis(1));
        
        assert!(pipeline.end_stage("parse").is_ok());
        let report = pipeline.get_stage_report();
        assert!(report.contains_key("parse"));
        assert!(report["parse"] > Duration::new(0, 0));
    }
}
