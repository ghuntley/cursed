// Optimization analysis module
use crate::error_types::CursedError;
use crate::optimization::benchmarking::{BenchmarkResults, BenchmarkStatistics};
use std::collections::HashMap;
use std::time::Duration;

/// Performance report for optimization analysis
#[derive(Debug, Clone)]
pub struct PerformanceReport {
    pub test_name: String,
    pub benchmark_results: BenchmarkResults,
    pub statistics: BenchmarkStatistics,
    pub baseline_comparison: Option<BaselineComparison>,
    pub optimization_recommendations: Vec<OptimizationRecommendation>,
}

impl PerformanceReport {
    pub fn new(test_name: String, benchmark_results: BenchmarkResults) -> Self {
        let statistics = BenchmarkStatistics::from_results(&benchmark_results);
        
        Self {
            test_name,
            benchmark_results,
            statistics,
            baseline_comparison: None,
            optimization_recommendations: Vec::new(),
        }
    }

    pub fn with_baseline_comparison(mut self, comparison: BaselineComparison) -> Self {
        self.baseline_comparison = Some(comparison);
        self
    }

    pub fn add_recommendation(&mut self, recommendation: OptimizationRecommendation) {
        self.optimization_recommendations.push(recommendation);
    }

    pub fn generate_summary(&self) -> String {
        format!(
            "Performance Report: {}\n\
             Average Time: {:?}\n\
             Iterations: {}\n\
             Standard Deviation: {:?}\n\
             95th Percentile: {:?}\n\
             Recommendations: {}",
            self.test_name,
            self.statistics.mean,
            self.benchmark_results.iterations,
            self.statistics.standard_deviation,
            self.statistics.percentile_95,
            self.optimization_recommendations.len()
        )
    }
}

/// Baseline comparison result
#[derive(Debug, Clone)]
pub struct BaselineComparison {
    pub baseline_time: Duration,
    pub current_time: Duration,
    pub improvement_ratio: f64,
    pub is_regression: bool,
    pub significance_level: f64,
}

impl BaselineComparison {
    pub fn new(baseline_time: Duration, current_time: Duration) -> Self {
        let improvement_ratio = if current_time.as_nanos() > 0 {
            baseline_time.as_nanos() as f64 / current_time.as_nanos() as f64
        } else {
            1.0
        };

        let is_regression = improvement_ratio < 1.0;
        let significance_level = (improvement_ratio - 1.0).abs();

        Self {
            baseline_time,
            current_time,
            improvement_ratio,
            is_regression,
            significance_level,
        }
    }

    pub fn percentage_change(&self) -> f64 {
        (self.improvement_ratio - 1.0) * 100.0
    }

    pub fn is_significant(&self, threshold: f64) -> bool {
        self.significance_level > threshold
    }
}

/// Optimization recommendation
#[derive(Debug, Clone)]
pub struct OptimizationRecommendation {
    pub category: RecommendationCategory,
    pub description: String,
    pub expected_improvement: Option<f64>,
    pub complexity: RecommendationComplexity,
    pub priority: RecommendationPriority,
}

impl OptimizationRecommendation {
    pub fn new(
        category: RecommendationCategory,
        description: String,
    ) -> Self {
        Self {
            category,
            description,
            expected_improvement: None,
            complexity: RecommendationComplexity::Medium,
            priority: RecommendationPriority::Medium,
        }
    }

    pub fn with_expected_improvement(mut self, improvement: f64) -> Self {
        self.expected_improvement = Some(improvement);
        self
    }

    pub fn with_complexity(mut self, complexity: RecommendationComplexity) -> Self {
        self.complexity = complexity;
        self
    }

    pub fn with_priority(mut self, priority: RecommendationPriority) -> Self {
        self.priority = priority;
        self
    }
}

/// Recommendation categories
#[derive(Debug, Clone, PartialEq)]
pub enum RecommendationCategory {
    CompilerOptimization,
    AlgorithmImprovement,
    DataStructureOptimization,
    MemoryManagement,
    ParallelProcessing,
    CacheOptimization,
    IoOptimization,
    ProfileGuidedOptimization,
}

/// Recommendation complexity levels
#[derive(Debug, Clone, PartialEq)]
pub enum RecommendationComplexity {
    Low,
    Medium,
    High,
    VeryHigh,
}

/// Recommendation priority levels
#[derive(Debug, Clone, PartialEq)]
pub enum RecommendationPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Optimization analysis engine
#[derive(Debug)]
pub struct OptimizationAnalyzer {
    pub baseline_reports: HashMap<String, PerformanceReport>,
    pub regression_threshold: f64,
    pub improvement_threshold: f64,
}

impl OptimizationAnalyzer {
    pub fn new() -> Self {
        Self {
            baseline_reports: HashMap::new(),
            regression_threshold: 0.05, // 5% regression threshold
            improvement_threshold: 0.02, // 2% improvement threshold
        }
    }

    pub fn with_thresholds(mut self, regression: f64, improvement: f64) -> Self {
        self.regression_threshold = regression;
        self.improvement_threshold = improvement;
        self
    }

    pub fn set_baseline(&mut self, test_name: String, report: PerformanceReport) {
        self.baseline_reports.insert(test_name, report);
    }

    pub fn analyze_performance(&self, test_name: &str, current_results: BenchmarkResults) -> PerformanceReport {
        let mut report = PerformanceReport::new(test_name.to_string(), current_results);

        // Compare with baseline if available
        if let Some(baseline_report) = self.baseline_reports.get(test_name) {
            let comparison = BaselineComparison::new(
                baseline_report.statistics.mean,
                report.statistics.mean,
            );

            // Generate recommendations based on comparison
            if comparison.is_regression && comparison.is_significant(self.regression_threshold) {
                report.add_recommendation(OptimizationRecommendation::new(
                    RecommendationCategory::CompilerOptimization,
                    format!("Performance regression detected: {:.2}% slower than baseline", 
                           -comparison.percentage_change()),
                ).with_priority(RecommendationPriority::High));
            }

            report = report.with_baseline_comparison(comparison);
        }

        // Generate general recommendations
        self.generate_recommendations(&mut report);

        report
    }

    pub fn generate_recommendations(&self, report: &mut PerformanceReport) {
        let stats = &report.statistics;

        // High variance suggests inconsistent performance
        if stats.coefficient_of_variation > 0.1 {
            report.add_recommendation(OptimizationRecommendation::new(
                RecommendationCategory::CacheOptimization,
                "High performance variance detected. Consider cache-friendly data structures.".to_string(),
            ).with_complexity(RecommendationComplexity::Medium));
        }

        // Long execution times suggest optimization opportunities
        if stats.mean > Duration::from_millis(100) {
            report.add_recommendation(OptimizationRecommendation::new(
                RecommendationCategory::AlgorithmImprovement,
                "Long execution time. Consider algorithmic optimizations.".to_string(),
            ).with_priority(RecommendationPriority::High));
        }

        // High standard deviation suggests optimization potential
        if stats.standard_deviation > stats.mean / 4 {
            report.add_recommendation(OptimizationRecommendation::new(
                RecommendationCategory::ProfileGuidedOptimization,
                "High standard deviation. Profile-guided optimization may help.".to_string(),
            ).with_complexity(RecommendationComplexity::High));
        }
    }

    pub fn generate_overall_report(&self, reports: &[PerformanceReport]) -> OverallAnalysisReport {
        let total_tests = reports.len();
        let regressions = reports.iter()
            .filter(|r| r.baseline_comparison.as_ref().map_or(false, |c| c.is_regression))
            .count();
        let improvements = reports.iter()
            .filter(|r| r.baseline_comparison.as_ref().map_or(false, |c| !c.is_regression && c.improvement_ratio > 1.02))
            .count();

        let average_improvement = reports.iter()
            .filter_map(|r| r.baseline_comparison.as_ref())
            .map(|c| c.improvement_ratio - 1.0)
            .sum::<f64>() / reports.len().max(1) as f64;

        OverallAnalysisReport {
            total_tests,
            regressions,
            improvements,
            average_improvement_percentage: average_improvement * 100.0,
            recommendations: self.aggregate_recommendations(reports),
        }
    }

    fn aggregate_recommendations(&self, reports: &[PerformanceReport]) -> Vec<OptimizationRecommendation> {
        let mut recommendation_counts: HashMap<RecommendationCategory, usize> = HashMap::new();
        
        for report in reports {
            for rec in &report.optimization_recommendations {
                *recommendation_counts.entry(rec.category.clone()).or_insert(0) += 1;
            }
        }

        let mut aggregated = Vec::new();
        for (category, count) in recommendation_counts {
            if count > reports.len() / 4 {  // If more than 25% of tests suggest this optimization
                let description = format!("{:?} optimization suggested by {} out of {} tests", 
                                        category, count, reports.len());
                aggregated.push(OptimizationRecommendation::new(category, description)
                    .with_priority(RecommendationPriority::High));
            }
        }

        aggregated
    }
}

impl Default for OptimizationAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Overall analysis report
#[derive(Debug, Clone)]
pub struct OverallAnalysisReport {
    pub total_tests: usize,
    pub regressions: usize,
    pub improvements: usize,
    pub average_improvement_percentage: f64,
    pub recommendations: Vec<OptimizationRecommendation>,
}

impl OverallAnalysisReport {
    pub fn generate_summary(&self) -> String {
        format!(
            "Overall Performance Analysis:\n\
             Total Tests: {}\n\
             Regressions: {} ({:.1}%)\n\
             Improvements: {} ({:.1}%)\n\
             Average Change: {:.2}%\n\
             Top Recommendations: {}",
            self.total_tests,
            self.regressions,
            (self.regressions as f64 / self.total_tests as f64) * 100.0,
            self.improvements,
            (self.improvements as f64 / self.total_tests as f64) * 100.0,
            self.average_improvement_percentage,
            self.recommendations.len()
        )
    }
}
