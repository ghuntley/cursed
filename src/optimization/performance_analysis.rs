/// Performance Analysis and Reporting
/// 
/// Provides comprehensive performance analysis capabilities including:
/// - Optimization result analysis
/// - Performance trend tracking
/// - Benchmark comparison
/// - Detailed reporting and visualization

use crate::error::{Error, Result};
use crate::optimization::{OptimizationResult, OptimizationManagerStats};
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};
use serde::{Serialize, Deserialize};

/// Performance analyzer for optimization results
pub struct PerformanceAnalyzer {
    historical_data: VecDeque<PerformanceDataPoint>,
    benchmarks: HashMap<String, BenchmarkSuite>,
    analysis_cache: HashMap<String, CachedAnalysis>,
    stats: PerformanceAnalysisStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceDataPoint {
    pub timestamp: u64,
    pub optimization_level: String,
    pub compilation_time: Duration,
    pub execution_time: Option<Duration>,
    pub memory_usage: Option<u64>,
    pub code_size: Option<u64>,
    pub performance_improvement: f64,
    pub optimization_passes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct BenchmarkSuite {
    pub name: String,
    pub benchmarks: Vec<Benchmark>,
    pub baseline_results: Option<BenchmarkResults>,
    pub last_updated: Instant,
}

#[derive(Debug, Clone)]
pub struct Benchmark {
    pub name: String,
    pub category: BenchmarkCategory,
    pub expected_performance: Option<Duration>,
    pub weight: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BenchmarkCategory {
    Compilation,
    Runtime,
    Memory,
    CodeSize,
    Throughput,
}

#[derive(Debug, Clone)]
pub struct BenchmarkResults {
    pub suite_name: String,
    pub results: HashMap<String, BenchmarkResult>,
    pub overall_score: f64,
    pub timestamp: Instant,
}

#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub benchmark_name: String,
    pub value: f64,
    pub unit: String,
    pub improvement_percentage: f64,
    pub performance_regression: bool,
}

#[derive(Debug, Clone)]
pub struct CachedAnalysis {
    pub analysis_type: String,
    pub results: AnalysisResults,
    pub created_at: Instant,
    pub expires_at: Instant,
}

#[derive(Debug, Clone)]
pub struct AnalysisResults {
    pub trends: TrendAnalysis,
    pub performance_insights: Vec<PerformanceInsight>,
    pub optimization_recommendations: Vec<OptimizationRecommendation>,
    pub statistical_summary: StatisticalSummary,
}

#[derive(Debug, Clone)]
pub struct TrendAnalysis {
    pub compilation_time_trend: TrendDirection,
    pub performance_improvement_trend: TrendDirection,
    pub memory_usage_trend: TrendDirection,
    pub code_size_trend: TrendDirection,
    pub optimization_effectiveness: f64,
    pub trend_confidence: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrendDirection {
    Improving,
    Stable,
    Degrading,
    Volatile,
    InsufficientData,
}

#[derive(Debug, Clone)]
pub struct PerformanceInsight {
    pub insight_type: InsightType,
    pub severity: InsightSeverity,
    pub description: String,
    pub impact_score: f64,
    pub suggested_actions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InsightType {
    PerformanceRegression,
    OptimizationOpportunity,
    MemoryIssue,
    CompilationBottleneck,
    ConfigurationIssue,
    TrendAlert,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum InsightSeverity {
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone)]
pub struct OptimizationRecommendation {
    pub recommendation_type: RecommendationType,
    pub priority: RecommendationPriority,
    pub description: String,
    pub estimated_impact: f64,
    pub implementation_effort: ImplementationEffort,
    pub configuration_changes: Vec<ConfigurationChange>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecommendationType {
    IncreaseOptimizationLevel,
    DecreaseOptimizationLevel,
    EnableSpecificPass,
    DisableSpecificPass,
    AdjustTimeouts,
    ChangeStrategy,
    HardwareUpgrade,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RecommendationPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImplementationEffort {
    Trivial,
    Easy,
    Moderate,
    Difficult,
    Complex,
}

#[derive(Debug, Clone)]
pub struct ConfigurationChange {
    pub parameter: String,
    pub current_value: String,
    pub recommended_value: String,
    pub rationale: String,
}

#[derive(Debug, Clone)]
pub struct StatisticalSummary {
    pub mean_compilation_time: Duration,
    pub median_compilation_time: Duration,
    pub std_dev_compilation_time: Duration,
    pub mean_performance_improvement: f64,
    pub median_performance_improvement: f64,
    pub success_rate: f64,
    pub total_samples: usize,
    pub confidence_interval: (f64, f64),
}

#[derive(Debug, Clone, Default)]
pub struct PerformanceAnalysisStats {
    pub analyses_performed: u64,
    pub insights_generated: u64,
    pub recommendations_made: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub total_analysis_time: Duration,
}

/// Comprehensive optimization report
#[derive(Debug, Clone, Serialize)]
pub struct OptimizationReport {
    pub report_id: String,
    pub generated_at: u64,
    pub summary: OptimizationSummary,
    pub detailed_analysis: AnalysisResults,
    pub benchmark_comparison: Option<BenchmarkComparison>,
    pub historical_trends: TrendAnalysis,
    pub performance_insights: Vec<PerformanceInsight>,
    pub recommendations: Vec<OptimizationRecommendation>,
    pub statistical_data: StatisticalSummary,
}

#[derive(Debug, Clone, Serialize)]
pub struct OptimizationSummary {
    pub total_optimizations: u64,
    pub successful_optimizations: u64,
    pub average_improvement: f64,
    pub total_time_saved: Duration,
    pub most_effective_passes: Vec<String>,
    pub overall_grade: PerformanceGrade,
}

#[derive(Debug, Clone, Serialize)]
pub struct BenchmarkComparison {
    pub current_results: BenchmarkResults,
    pub baseline_results: BenchmarkResults,
    pub improvements: HashMap<String, f64>,
    pub regressions: HashMap<String, f64>,
    pub overall_change: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum PerformanceGrade {
    Excellent,
    Good,
    Average,
    Poor,
    Critical,
}

impl PerformanceAnalyzer {
    pub fn new() -> Self {
        Self {
            historical_data: VecDeque::with_capacity(1000),
            benchmarks: HashMap::new(),
            analysis_cache: HashMap::new(),
            stats: PerformanceAnalysisStats::default(),
        }
    }

    /// Add performance data point for analysis
    pub fn add_performance_data(&mut self, data_point: PerformanceDataPoint) {
        self.historical_data.push_back(data_point);
        
        // Keep only recent data (last 1000 points)
        while self.historical_data.len() > 1000 {
            self.historical_data.pop_front();
        }

        // Invalidate relevant cached analyses
        self.invalidate_cache("trend_analysis");
        self.invalidate_cache("statistical_summary");
    }

    /// Analyze optimization results and generate insights
    pub fn analyze_optimization_results(&mut self, results: &[OptimizationResult]) -> Result<AnalysisResults> {
        let start_time = Instant::now();
        
        tracing::info!(
            results_count = results.len(),
            "Starting optimization results analysis"
        );

        // Check cache first
        let cache_key = self.generate_cache_key("optimization_analysis", results);
        if let Some(cached) = self.get_cached_analysis(&cache_key) {
            self.stats.cache_hits += 1;
            return Ok(cached.results);
        }

        self.stats.cache_misses += 1;

        // Convert results to data points
        for result in results {
            let data_point = PerformanceDataPoint {
                timestamp: chrono::Utc::now().timestamp() as u64,
                optimization_level: "default".to_string(), // Would extract from result
                compilation_time: result.optimization_time,
                execution_time: None,
                memory_usage: None,
                code_size: Some(result.code_size_change.abs() as u64),
                performance_improvement: result.performance_improvement,
                optimization_passes: result.passes_applied.clone(),
            };
            self.add_performance_data(data_point);
        }

        // Perform analysis
        let trends = self.analyze_trends()?;
        let insights = self.generate_insights(&trends)?;
        let recommendations = self.generate_recommendations(&trends, &insights)?;
        let statistics = self.calculate_statistics()?;

        let analysis_results = AnalysisResults {
            trends,
            performance_insights: insights,
            optimization_recommendations: recommendations,
            statistical_summary: statistics,
        };

        // Cache the results
        self.cache_analysis(cache_key, analysis_results.clone());

        self.stats.analyses_performed += 1;
        self.stats.total_analysis_time += start_time.elapsed();

        tracing::info!(
            analysis_time_ms = start_time.elapsed().as_millis(),
            insights_generated = analysis_results.performance_insights.len(),
            recommendations_made = analysis_results.optimization_recommendations.len(),
            "Optimization analysis completed"
        );

        Ok(analysis_results)
    }

    /// Analyze performance trends
    fn analyze_trends(&self) -> Result<TrendAnalysis> {
        if self.historical_data.len() < 5 {
            return Ok(TrendAnalysis {
                compilation_time_trend: TrendDirection::InsufficientData,
                performance_improvement_trend: TrendDirection::InsufficientData,
                memory_usage_trend: TrendDirection::InsufficientData,
                code_size_trend: TrendDirection::InsufficientData,
                optimization_effectiveness: 0.0,
                trend_confidence: 0.0,
            });
        }

        let recent_data: Vec<_> = self.historical_data.iter().rev().take(20).collect();
        
        // Analyze compilation time trend
        let compilation_times: Vec<f64> = recent_data.iter()
            .map(|d| d.compilation_time.as_millis() as f64)
            .collect();
        let compilation_time_trend = self.calculate_trend_direction(&compilation_times);

        // Analyze performance improvement trend
        let performance_improvements: Vec<f64> = recent_data.iter()
            .map(|d| d.performance_improvement)
            .collect();
        let performance_improvement_trend = self.calculate_trend_direction(&performance_improvements);

        // Analyze memory usage trend (if available)
        let memory_usage_trend = if recent_data.iter().any(|d| d.memory_usage.is_some()) {
            let memory_usages: Vec<f64> = recent_data.iter()
                .filter_map(|d| d.memory_usage.map(|m| m as f64))
                .collect();
            self.calculate_trend_direction(&memory_usages)
        } else {
            TrendDirection::InsufficientData
        };

        // Analyze code size trend
        let code_size_trend = if recent_data.iter().any(|d| d.code_size.is_some()) {
            let code_sizes: Vec<f64> = recent_data.iter()
                .filter_map(|d| d.code_size.map(|s| s as f64))
                .collect();
            self.calculate_trend_direction(&code_sizes)
        } else {
            TrendDirection::InsufficientData
        };

        // Calculate optimization effectiveness
        let avg_improvement = performance_improvements.iter().sum::<f64>() / performance_improvements.len() as f64;
        let optimization_effectiveness = (avg_improvement / 100.0).min(1.0).max(0.0);

        // Calculate trend confidence based on data consistency
        let trend_confidence = self.calculate_trend_confidence(&recent_data);

        Ok(TrendAnalysis {
            compilation_time_trend,
            performance_improvement_trend,
            memory_usage_trend,
            code_size_trend,
            optimization_effectiveness,
            trend_confidence,
        })
    }

    /// Calculate trend direction from numeric data
    fn calculate_trend_direction(&self, data: &[f64]) -> TrendDirection {
        if data.len() < 3 {
            return TrendDirection::InsufficientData;
        }

        // Simple linear regression to determine trend
        let n = data.len() as f64;
        let x_mean = (data.len() - 1) as f64 / 2.0;
        let y_mean = data.iter().sum::<f64>() / n;

        let mut numerator = 0.0;
        let mut denominator = 0.0;

        for (i, &y) in data.iter().enumerate() {
            let x = i as f64;
            numerator += (x - x_mean) * (y - y_mean);
            denominator += (x - x_mean) * (x - x_mean);
        }

        if denominator == 0.0 {
            return TrendDirection::Stable;
        }

        let slope = numerator / denominator;
        let slope_threshold = y_mean * 0.05; // 5% threshold

        // Calculate variance to detect volatility
        let variance = data.iter()
            .map(|&y| (y - y_mean).powi(2))
            .sum::<f64>() / n;
        let coefficient_of_variation = if y_mean != 0.0 {
            variance.sqrt() / y_mean.abs()
        } else {
            0.0
        };

        if coefficient_of_variation > 0.3 {
            TrendDirection::Volatile
        } else if slope.abs() < slope_threshold {
            TrendDirection::Stable
        } else if slope > 0.0 {
            TrendDirection::Improving
        } else {
            TrendDirection::Degrading
        }
    }

    /// Calculate trend confidence score
    fn calculate_trend_confidence(&self, data: &[&PerformanceDataPoint]) -> f64 {
        if data.len() < 5 {
            return 0.0;
        }

        // Confidence based on data consistency and sample size
        let sample_size_factor = (data.len() as f64 / 20.0).min(1.0);
        
        // Check consistency of optimization passes
        let pass_consistency = self.calculate_pass_consistency(data);
        
        // Overall confidence
        (sample_size_factor + pass_consistency) / 2.0
    }

    /// Calculate optimization pass consistency
    fn calculate_pass_consistency(&self, data: &[&PerformanceDataPoint]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }

        let first_passes = &data[0].optimization_passes;
        let consistent_count = data.iter()
            .filter(|d| d.optimization_passes == *first_passes)
            .count();

        consistent_count as f64 / data.len() as f64
    }

    /// Generate performance insights
    fn generate_insights(&mut self, trends: &TrendAnalysis) -> Result<Vec<PerformanceInsight>> {
        let mut insights = Vec::new();

        // Compilation time insights
        match trends.compilation_time_trend {
            TrendDirection::Degrading => {
                insights.push(PerformanceInsight {
                    insight_type: InsightType::CompilationBottleneck,
                    severity: InsightSeverity::Warning,
                    description: "Compilation times are increasing over recent builds".to_string(),
                    impact_score: 0.7,
                    suggested_actions: vec![
                        "Review enabled optimization passes".to_string(),
                        "Consider parallel compilation".to_string(),
                        "Check for incremental compilation issues".to_string(),
                    ],
                });
            }
            TrendDirection::Volatile => {
                insights.push(PerformanceInsight {
                    insight_type: InsightType::TrendAlert,
                    severity: InsightSeverity::Info,
                    description: "Compilation times are highly variable".to_string(),
                    impact_score: 0.4,
                    suggested_actions: vec![
                        "Investigate build environment consistency".to_string(),
                        "Check for resource contention".to_string(),
                    ],
                });
            }
            _ => {}
        }

        // Performance improvement insights
        if trends.optimization_effectiveness < 0.1 {
            insights.push(PerformanceInsight {
                insight_type: InsightType::OptimizationOpportunity,
                severity: InsightSeverity::Warning,
                description: "Optimizations are providing minimal benefit".to_string(),
                impact_score: 0.8,
                suggested_actions: vec![
                    "Consider increasing optimization level".to_string(),
                    "Enable profile-guided optimization".to_string(),
                    "Review optimization pass selection".to_string(),
                ],
            });
        } else if trends.optimization_effectiveness > 0.8 {
            insights.push(PerformanceInsight {
                insight_type: InsightType::OptimizationOpportunity,
                severity: InsightSeverity::Info,
                description: "Optimizations are highly effective".to_string(),
                impact_score: 0.9,
                suggested_actions: vec![
                    "Current optimization strategy is working well".to_string(),
                    "Consider sharing configuration with other projects".to_string(),
                ],
            });
        }

        // Memory usage insights
        match trends.memory_usage_trend {
            TrendDirection::Degrading => {
                insights.push(PerformanceInsight {
                    insight_type: InsightType::MemoryIssue,
                    severity: InsightSeverity::Critical,
                    description: "Memory usage is increasing significantly".to_string(),
                    impact_score: 0.9,
                    suggested_actions: vec![
                        "Review memory optimization passes".to_string(),
                        "Check for memory leaks in optimization pipeline".to_string(),
                        "Consider memory-focused optimization level".to_string(),
                    ],
                });
            }
            _ => {}
        }

        // Trend confidence insights
        if trends.trend_confidence < 0.5 {
            insights.push(PerformanceInsight {
                insight_type: InsightType::ConfigurationIssue,
                severity: InsightSeverity::Info,
                description: "Optimization results are inconsistent, possibly due to varying configurations".to_string(),
                impact_score: 0.5,
                suggested_actions: vec![
                    "Standardize optimization configuration".to_string(),
                    "Ensure consistent build environment".to_string(),
                ],
            });
        }

        self.stats.insights_generated += insights.len() as u64;
        Ok(insights)
    }

    /// Generate optimization recommendations
    fn generate_recommendations(&mut self, trends: &TrendAnalysis, insights: &[PerformanceInsight]) -> Result<Vec<OptimizationRecommendation>> {
        let mut recommendations = Vec::new();

        // Recommendations based on trends
        if trends.compilation_time_trend == TrendDirection::Degrading {
            recommendations.push(OptimizationRecommendation {
                recommendation_type: RecommendationType::DecreaseOptimizationLevel,
                priority: RecommendationPriority::Medium,
                description: "Consider reducing optimization level to improve compilation times".to_string(),
                estimated_impact: 0.6,
                implementation_effort: ImplementationEffort::Easy,
                configuration_changes: vec![
                    ConfigurationChange {
                        parameter: "optimization_level".to_string(),
                        current_value: "O3".to_string(),
                        recommended_value: "O2".to_string(),
                        rationale: "Reduce compilation time while maintaining reasonable performance".to_string(),
                    }
                ],
            });
        }

        if trends.optimization_effectiveness < 0.2 {
            recommendations.push(OptimizationRecommendation {
                recommendation_type: RecommendationType::EnableSpecificPass,
                priority: RecommendationPriority::High,
                description: "Enable profile-guided optimization for better results".to_string(),
                estimated_impact: 0.8,
                implementation_effort: ImplementationEffort::Moderate,
                configuration_changes: vec![
                    ConfigurationChange {
                        parameter: "pgo.enabled".to_string(),
                        current_value: "false".to_string(),
                        recommended_value: "true".to_string(),
                        rationale: "PGO can significantly improve optimization effectiveness".to_string(),
                    }
                ],
            });
        }

        // Recommendations based on insights
        for insight in insights {
            match insight.insight_type {
                InsightType::CompilationBottleneck => {
                    recommendations.push(OptimizationRecommendation {
                        recommendation_type: RecommendationType::ChangeStrategy,
                        priority: RecommendationPriority::Medium,
                        description: "Enable parallel compilation to reduce build times".to_string(),
                        estimated_impact: 0.7,
                        implementation_effort: ImplementationEffort::Easy,
                        configuration_changes: vec![
                            ConfigurationChange {
                                parameter: "parallel_compilation.enabled".to_string(),
                                current_value: "false".to_string(),
                                recommended_value: "true".to_string(),
                                rationale: "Parallel compilation can significantly reduce total build time".to_string(),
                            }
                        ],
                    });
                }
                InsightType::MemoryIssue => {
                    recommendations.push(OptimizationRecommendation {
                        recommendation_type: RecommendationType::EnableSpecificPass,
                        priority: RecommendationPriority::High,
                        description: "Enable memory optimization passes".to_string(),
                        estimated_impact: 0.8,
                        implementation_effort: ImplementationEffort::Easy,
                        configuration_changes: vec![
                            ConfigurationChange {
                                parameter: "memory_optimization.enabled".to_string(),
                                current_value: "false".to_string(),
                                recommended_value: "true".to_string(),
                                rationale: "Memory optimization can reduce runtime memory usage".to_string(),
                            }
                        ],
                    });
                }
                _ => {}
            }
        }

        self.stats.recommendations_made += recommendations.len() as u64;
        Ok(recommendations)
    }

    /// Calculate statistical summary
    fn calculate_statistics(&self) -> Result<StatisticalSummary> {
        if self.historical_data.is_empty() {
            return Ok(StatisticalSummary {
                mean_compilation_time: Duration::ZERO,
                median_compilation_time: Duration::ZERO,
                std_dev_compilation_time: Duration::ZERO,
                mean_performance_improvement: 0.0,
                median_performance_improvement: 0.0,
                success_rate: 0.0,
                total_samples: 0,
                confidence_interval: (0.0, 0.0),
            });
        }

        let data: Vec<_> = self.historical_data.iter().collect();
        
        // Compilation time statistics
        let mut compilation_times: Vec<_> = data.iter()
            .map(|d| d.compilation_time.as_millis() as f64)
            .collect();
        compilation_times.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mean_compilation_ms = compilation_times.iter().sum::<f64>() / compilation_times.len() as f64;
        let median_compilation_ms = if compilation_times.len() % 2 == 0 {
            (compilation_times[compilation_times.len() / 2 - 1] + compilation_times[compilation_times.len() / 2]) / 2.0
        } else {
            compilation_times[compilation_times.len() / 2]
        };

        let variance = compilation_times.iter()
            .map(|&x| (x - mean_compilation_ms).powi(2))
            .sum::<f64>() / compilation_times.len() as f64;
        let std_dev_compilation_ms = variance.sqrt();

        // Performance improvement statistics
        let mut performance_improvements: Vec<_> = data.iter()
            .map(|d| d.performance_improvement)
            .collect();
        performance_improvements.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mean_performance_improvement = performance_improvements.iter().sum::<f64>() / performance_improvements.len() as f64;
        let median_performance_improvement = if performance_improvements.len() % 2 == 0 {
            (performance_improvements[performance_improvements.len() / 2 - 1] + performance_improvements[performance_improvements.len() / 2]) / 2.0
        } else {
            performance_improvements[performance_improvements.len() / 2]
        };

        // Success rate (assuming all current data points represent successful optimizations)
        let success_rate = 1.0; // Would be calculated from actual success/failure data

        // Confidence interval for performance improvement (95% CI)
        let n = performance_improvements.len() as f64;
        let std_error = (variance / n).sqrt();
        let t_value = 1.96; // Approximate for 95% CI with large sample
        let margin_of_error = t_value * std_error;
        let confidence_interval = (
            mean_performance_improvement - margin_of_error,
            mean_performance_improvement + margin_of_error,
        );

        Ok(StatisticalSummary {
            mean_compilation_time: Duration::from_millis(mean_compilation_ms as u64),
            median_compilation_time: Duration::from_millis(median_compilation_ms as u64),
            std_dev_compilation_time: Duration::from_millis(std_dev_compilation_ms as u64),
            mean_performance_improvement,
            median_performance_improvement,
            success_rate,
            total_samples: data.len(),
            confidence_interval,
        })
    }

    /// Generate comprehensive optimization report
    pub fn generate_comprehensive_report(
        &self,
        optimization_history: &[OptimizationResult],
        manager_stats: &OptimizationManagerStats,
    ) -> Result<OptimizationReport> {
        let report_id = format!("opt_report_{}", chrono::Utc::now().timestamp());
        
        tracing::info!(
            report_id = report_id,
            "Generating comprehensive optimization report"
        );

        // Create summary
        let summary = OptimizationSummary {
            total_optimizations: manager_stats.total_optimizations_run,
            successful_optimizations: manager_stats.successful_optimizations,
            average_improvement: optimization_history.iter()
                .map(|r| r.performance_improvement)
                .sum::<f64>() / optimization_history.len().max(1) as f64,
            total_time_saved: Duration::from_secs(0), // Would calculate from actual data
            most_effective_passes: self.find_most_effective_passes(optimization_history),
            overall_grade: self.calculate_overall_grade(manager_stats),
        };

        // Perform detailed analysis
        let mut analyzer = self.clone();
        let detailed_analysis = analyzer.analyze_optimization_results(optimization_history)?;

        // Generate benchmark comparison if available
        let benchmark_comparison = self.generate_benchmark_comparison()?;

        let report = OptimizationReport {
            report_id,
            generated_at: chrono::Utc::now().timestamp() as u64,
            summary,
            detailed_analysis: detailed_analysis.clone(),
            benchmark_comparison,
            historical_trends: detailed_analysis.trends,
            performance_insights: detailed_analysis.performance_insights,
            recommendations: detailed_analysis.optimization_recommendations,
            statistical_data: detailed_analysis.statistical_summary,
        };

        tracing::info!(
            report_id = report.report_id,
            insights_count = report.performance_insights.len(),
            recommendations_count = report.recommendations.len(),
            "Comprehensive optimization report generated"
        );

        Ok(report)
    }

    /// Find most effective optimization passes
    fn find_most_effective_passes(&self, optimization_history: &[OptimizationResult]) -> Vec<String> {
        let mut pass_effectiveness = HashMap::new();

        for result in optimization_history {
            if result.success && result.performance_improvement > 0.0 {
                for pass in &result.passes_applied {
                    let entry = pass_effectiveness.entry(pass.clone()).or_insert((0.0, 0));
                    entry.0 += result.performance_improvement;
                    entry.1 += 1;
                }
            }
        }

        let mut effective_passes: Vec<_> = pass_effectiveness.into_iter()
            .map(|(pass, (total_improvement, count))| (pass, total_improvement / count as f64))
            .collect();
        
        effective_passes.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        effective_passes.into_iter().take(5).map(|(pass, _)| pass).collect()
    }

    /// Calculate overall performance grade
    fn calculate_overall_grade(&self, stats: &OptimizationManagerStats) -> PerformanceGrade {
        let success_rate = if stats.total_optimizations_run > 0 {
            stats.successful_optimizations as f64 / stats.total_optimizations_run as f64
        } else {
            0.0
        };

        let avg_improvement = if !stats.performance_improvements.is_empty() {
            stats.performance_improvements.values().sum::<f64>() / stats.performance_improvements.len() as f64
        } else {
            0.0
        };

        let score = (success_rate * 0.4 + (avg_improvement / 100.0).min(1.0) * 0.6) * 100.0;

        match score as u32 {
            90..=100 => PerformanceGrade::Excellent,
            75..=89 => PerformanceGrade::Good,
            50..=74 => PerformanceGrade::Average,
            25..=49 => PerformanceGrade::Poor,
            _ => PerformanceGrade::Critical,
        }
    }

    /// Generate benchmark comparison
    fn generate_benchmark_comparison(&self) -> Result<Option<BenchmarkComparison>> {
        // Placeholder - would implement actual benchmark comparison
        Ok(None)
    }

    // Cache management methods
    fn generate_cache_key(&self, analysis_type: &str, data: &[OptimizationResult]) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        analysis_type.hash(&mut hasher);
        data.len().hash(&mut hasher);
        
        // Hash key properties of the data
        for result in data {
            result.optimization_time.as_nanos().hash(&mut hasher);
            result.performance_improvement.to_bits().hash(&mut hasher);
            result.passes_applied.len().hash(&mut hasher);
        }

        format!("{}_{:x}", analysis_type, hasher.finish())
    }

    fn get_cached_analysis(&self, key: &str) -> Option<&CachedAnalysis> {
        if let Some(cached) = self.analysis_cache.get(key) {
            if cached.expires_at > Instant::now() {
                return Some(cached);
            }
        }
        None
    }

    fn cache_analysis(&mut self, key: String, results: AnalysisResults) {
        let cached_analysis = CachedAnalysis {
            analysis_type: key.clone(),
            results,
            created_at: Instant::now(),
            expires_at: Instant::now() + Duration::from_secs(300), // 5 minute cache
        };
        
        self.analysis_cache.insert(key, cached_analysis);
    }

    fn invalidate_cache(&mut self, analysis_type: &str) {
        self.analysis_cache.retain(|key, _| !key.starts_with(analysis_type));
    }

    pub fn get_stats(&self) -> &PerformanceAnalysisStats {
        &self.stats
    }
}

impl Clone for PerformanceAnalyzer {
    fn clone(&self) -> Self {
        Self {
            historical_data: self.historical_data.clone(),
            benchmarks: self.benchmarks.clone(),
            analysis_cache: HashMap::new(), // Don't clone cache
            stats: self.stats.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_analyzer_creation() {
        let analyzer = PerformanceAnalyzer::new();
        assert_eq!(analyzer.historical_data.len(), 0);
        assert_eq!(analyzer.stats.analyses_performed, 0);
    }

    #[test]
    fn test_add_performance_data() {
        let mut analyzer = PerformanceAnalyzer::new();
        
        let data_point = PerformanceDataPoint {
            timestamp: chrono::Utc::now().timestamp() as u64,
            optimization_level: "O2".to_string(),
            compilation_time: Duration::from_millis(1000),
            execution_time: Some(Duration::from_millis(500)),
            memory_usage: Some(1024),
            code_size: Some(2048),
            performance_improvement: 10.0,
            optimization_passes: vec!["inlining".to_string(), "dce".to_string()],
        };

        analyzer.add_performance_data(data_point);
        assert_eq!(analyzer.historical_data.len(), 1);
    }

    #[test]
    fn test_trend_calculation() {
        let analyzer = PerformanceAnalyzer::new();
        
        // Test improving trend
        let improving_data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let trend = analyzer.calculate_trend_direction(&improving_data);
        assert_eq!(trend, TrendDirection::Improving);

        // Test degrading trend
        let degrading_data = vec![5.0, 4.0, 3.0, 2.0, 1.0];
        let trend = analyzer.calculate_trend_direction(&degrading_data);
        assert_eq!(trend, TrendDirection::Degrading);

        // Test stable trend
        let stable_data = vec![3.0, 3.1, 2.9, 3.0, 3.1];
        let trend = analyzer.calculate_trend_direction(&stable_data);
        assert_eq!(trend, TrendDirection::Stable);
    }

    #[test]
    fn test_statistical_summary() {
        let mut analyzer = PerformanceAnalyzer::new();
        
        // Add some test data
        for i in 1..=10 {
            let data_point = PerformanceDataPoint {
                timestamp: chrono::Utc::now().timestamp() as u64,
                optimization_level: "O2".to_string(),
                compilation_time: Duration::from_millis(i * 100),
                execution_time: None,
                memory_usage: None,
                code_size: None,
                performance_improvement: i as f64,
                optimization_passes: vec![],
            };
            analyzer.add_performance_data(data_point);
        }

        let stats = analyzer.calculate_statistics().unwrap();
        assert_eq!(stats.total_samples, 10);
        assert!(stats.mean_performance_improvement > 0.0);
    }

    #[test]
    fn test_performance_grade_calculation() {
        let analyzer = PerformanceAnalyzer::new();
        
        let stats = OptimizationManagerStats {
            total_optimizations_run: 100,
            successful_optimizations: 95,
            performance_improvements: vec![("test".to_string(), 50.0)].into_iter().collect(),
            ..Default::default()
        };

        let grade = analyzer.calculate_overall_grade(&stats);
        assert!(matches!(grade, PerformanceGrade::Excellent | PerformanceGrade::Good));
    }
}
