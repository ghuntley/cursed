/// Optimization result types for the CURSED compiler
/// 
/// Provides unified result types for all optimization operations

use crate::error::{CursedError, Result};

use std::collections::HashMap;
use std::time::Duration;

/// General optimization result
#[derive(Debug, Clone)]
pub struct OptimizationResult {
impl Default for OptimizationResult {
    fn default() -> Self {
        Self {
        }
    }
impl OptimizationResult {
    pub fn new() -> Self {
        Self::default()
    pub fn with_improvement(mut self, improvement: f64) -> Self {
        self.performance_improvement = improvement;
        self
    pub fn with_time_saved(mut self, time_saved: Duration) -> Self {
        self.compilation_time_saved = time_saved;
        self
    pub fn add_optimization(mut self, optimization: String) -> Self {
        self.optimizations_applied.push(optimization);
        self
    pub fn add_metric(mut self, name: String, value: f64) -> Self {
        self.metrics.insert(name, value);
        self
    pub fn add_warning(mut self, warning: String) -> Self {
        self.warnings.push(warning);
        self
    pub fn add_error(mut self, error: String) -> Self {
        self.errors.push(error);
        self.success = false;
        self
    }
}

// Incremental optimization results
pub use OptimizationResult as IncrementalResult;

// Adaptive optimization results  
#[derive(Debug, Clone)]
pub struct AdaptiveResults {
#[derive(Debug, Clone)]
pub struct AdaptiveStrategy {
// Memory optimization results
#[derive(Debug, Clone)]
pub struct MemoryOptimizer {
#[derive(Debug, Clone)]  
pub struct MemoryOptimizationResults {
// Build optimization results
#[derive(Debug, Clone)]
pub struct BuildOptimizer {
#[derive(Debug, Clone)]
pub struct BuildOptimizationResults {
// Parallel compilation results
#[derive(Debug, Clone)]
pub struct ParallelCompilationResults {
// Profiler results
#[derive(Debug, Clone)]
pub struct OptimizationProfiler {
#[derive(Debug, Clone)]
pub struct ProfilerResults {
// Runtime optimization results
#[derive(Debug, Clone)]
pub struct RuntimeOptimizer {
#[derive(Debug, Clone)]
pub struct RuntimeOptimizationResults {
// Profiling config
#[derive(Debug, Clone)]
pub struct ProfilingConfig {
impl Default for ProfilingConfig {
    fn default() -> Self {
        Self {
            sample_rate: 1000.0, // samples per second
        }
    }
}
