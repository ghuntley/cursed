//! Enhanced Performance Analysis for CURSED Compiler
//! 
//! Comprehensive analysis tools for identifying performance bottlenecks,
//! optimization opportunities, and providing actionable recommendations.

use crate::error::{Error, Result};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use tracing::{debug, info, instrument, warn};

/// Enhanced performance analyzer with machine learning-based recommendations
#[derive(Debug)]
pub struct EnhancedPerformanceAnalyzer {
    /// Analysis configuration
    config: AnalysisConfig,
    /// Compilation metrics collector
    metrics_collector: MetricsCollector,
    /// Pattern recognition engine
    pattern_engine: PatternRecognitionEngine,
    /// Recommendation generator
    recommendation_engine: RecommendationEngine,
    /// Historical data for trend analysis
    historical_data: Vec<AnalysisSnapshot>,
}

/// Configuration for performance analysis
#[derive(Debug, Clone)]
pub struct AnalysisConfig {
    /// Enable detailed phase timing
    pub detailed_timing: bool,
    /// Enable memory usage tracking
    pub memory_tracking: bool,
    /// Enable bottleneck detection
    pub bottleneck_detection: bool,
    /// Enable pattern recognition
    pub pattern_recognition: bool,
    /// Enable predictive analysis
    pub predictive_analysis: bool,
    /// Minimum improvement threshold for recommendations
    pub min_improvement_threshold: f64,
    /// Maximum analysis time
    pub max_analysis_time: Duration,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            detailed_timing: true,
            memory_tracking: true,
            bottleneck_detection: true,
            pattern_recognition: true,
            predictive_analysis: false,
            min_improvement_threshold: 0.05, // 5% minimum improvement
            max_analysis_time: Duration::from_secs(30),
        }
    }
}

/// Comprehensive analysis result with actionable insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedAnalysisResult {
    /// Overall analysis summary
    pub summary: AnalysisSummary,
    /// Detailed phase analysis
    pub phase_analysis: HashMap<CompilationPhase, PhaseMetrics>,
    /// Identified bottlenecks
    pub bottlenecks: Vec<PerformanceBottleneck>,
    /// Optimization recommendations
    pub recommendations: Vec<OptimizationRecommendation>,
    /// Predicted improvements
    pub predicted_improvements: HashMap<String, PredictedImprovement>,
    /// Resource usage analysis
    pub resource_usage: ResourceUsageAnalysis,
    /// Code complexity metrics
    pub complexity_metrics: ComplexityMetrics,
    /// Historical comparison
    pub historical_comparison: Option<HistoricalComparison>,
}

/// High-level analysis summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisSummary {
    /// Total compilation time
    pub total_time: Duration,
    /// Overall performance score (0-100)
    pub performance_score: f64,
    /// Efficiency rating
    pub efficiency_rating: EfficiencyRating,
    /// Primary bottleneck
    pub primary_bottleneck: Option<String>,
    /// Top recommendation
    pub top_recommendation: Option<String>,
    /// Estimated improvement potential
    pub improvement_potential: f64,
}

/// Compilation phase for detailed analysis
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompilationPhase {
    Lexing,
    Parsing,
    SemanticAnalysis,
    TypeChecking,
    IRGeneration,
    LLVMOptimization,
    CodeGeneration,
    Linking,
    Total,
}

/// Metrics for a specific compilation phase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseMetrics {
    /// Phase execution time
    pub execution_time: Duration,
    /// Memory usage during phase
    pub memory_usage: usize,
    /// CPU utilization percentage
    pub cpu_utilization: f64,
    /// Number of operations performed
    pub operation_count: usize,
    /// Efficiency score for this phase
    pub efficiency_score: f64,
    /// Detected issues
    pub issues: Vec<PhaseIssue>,
}

/// Performance bottleneck identification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBottleneck {
    /// Bottleneck identifier
    pub id: String,
    /// Human-readable description
    pub description: String,
    /// Affected compilation phase
    pub phase: CompilationPhase,
    /// Severity level (1-10)
    pub severity: u8,
    /// Performance impact percentage
    pub impact_percentage: f64,
    /// Suggested solutions
    pub solutions: Vec<String>,
    /// Estimated fix complexity
    pub fix_complexity: ComplexityLevel,
}

/// Optimization recommendation with priority and impact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationRecommendation {
    /// Recommendation identifier
    pub id: String,
    /// Title/summary
    pub title: String,
    /// Detailed description
    pub description: String,
    /// Priority level (1-10)
    pub priority: u8,
    /// Expected performance improvement
    pub expected_improvement: f64,
    /// Implementation effort required
    pub effort_level: EffortLevel,
    /// Specific actions to take
    pub actions: Vec<RecommendationAction>,
    /// Related optimization passes
    pub related_passes: Vec<String>,
    /// Confidence level (0-1)
    pub confidence: f64,
}

/// Predicted improvement from optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictedImprovement {
    /// Optimization name
    pub optimization: String,
    /// Predicted speedup factor
    pub speedup_factor: f64,
    /// Predicted memory reduction
    pub memory_reduction: f64,
    /// Confidence in prediction (0-1)
    pub confidence: f64,
    /// Basis for prediction
    pub basis: String,
}

/// Resource usage analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsageAnalysis {
    /// Peak memory usage
    pub peak_memory: usize,
    /// Average memory usage
    pub average_memory: usize,
    /// CPU time utilization
    pub cpu_time: Duration,
    /// I/O operations count
    pub io_operations: usize,
    /// Cache hit/miss ratios
    pub cache_metrics: CacheMetrics,
    /// Memory allocation patterns
    pub allocation_patterns: AllocationPatterns,
}

/// Code complexity metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityMetrics {
    /// Cyclomatic complexity
    pub cyclomatic_complexity: f64,
    /// Lines of code
    pub lines_of_code: usize,
    /// Function count
    pub function_count: usize,
    /// Type complexity
    pub type_complexity: f64,
    /// Dependency complexity
    pub dependency_complexity: f64,
    /// Template instantiation complexity
    pub template_complexity: f64,
}

/// Historical comparison data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalComparison {
    /// Previous analysis timestamp
    pub previous_timestamp: chrono::DateTime<chrono::Utc>,
    /// Performance trend
    pub performance_trend: PerformanceTrend,
    /// Compilation time change
    pub time_change_percentage: f64,
    /// Memory usage change
    pub memory_change_percentage: f64,
    /// New issues detected
    pub new_issues: Vec<String>,
    /// Resolved issues
    pub resolved_issues: Vec<String>,
}

// Supporting types and enums

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EfficiencyRating {
    Excellent,
    Good,
    Average,
    BelowAverage,
    Poor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplexityLevel {
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffortLevel {
    Minimal,
    Low,
    Medium,
    High,
    Significant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceTrend {
    Improving,
    Stable,
    Degrading,
    Fluctuating,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseIssue {
    pub issue_type: String,
    pub description: String,
    pub severity: u8,
    pub suggested_fix: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationAction {
    pub action_type: ActionType,
    pub description: String,
    pub parameters: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    EnableOptimization,
    DisableOptimization,
    ConfigureParameter,
    ChangeStrategy,
    RefactorCode,
    AddPragma,
    UpdateDependency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMetrics {
    pub instruction_cache_hits: usize,
    pub instruction_cache_misses: usize,
    pub data_cache_hits: usize,
    pub data_cache_misses: usize,
    pub tlb_hits: usize,
    pub tlb_misses: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationPatterns {
    pub small_allocations: usize,
    pub large_allocations: usize,
    pub frequent_allocations: usize,
    pub long_lived_allocations: usize,
    pub fragmentation_ratio: f64,
}

#[derive(Debug, Clone)]
pub struct AnalysisSnapshot {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub result: EnhancedAnalysisResult,
}

/// Metrics collector for gathering compilation data
#[derive(Debug)]
pub struct MetricsCollector {
    /// Phase timing data
    phase_timings: HashMap<CompilationPhase, Vec<Duration>>,
    /// Memory usage samples
    memory_samples: Vec<MemorySample>,
    /// CPU utilization data
    cpu_samples: Vec<CpuSample>,
    /// I/O operation tracking
    io_operations: Vec<IoOperation>,
}

#[derive(Debug, Clone)]
pub struct MemorySample {
    pub timestamp: Instant,
    pub phase: CompilationPhase,
    pub bytes_used: usize,
    pub bytes_allocated: usize,
}

#[derive(Debug, Clone)]
pub struct CpuSample {
    pub timestamp: Instant,
    pub phase: CompilationPhase,
    pub utilization: f64,
}

#[derive(Debug, Clone)]
pub struct IoOperation {
    pub timestamp: Instant,
    pub operation_type: IoOperationType,
    pub size: usize,
    pub duration: Duration,
}

#[derive(Debug, Clone)]
pub enum IoOperationType {
    FileRead,
    FileWrite,
    NetworkRequest,
    DiskSeek,
}

/// Pattern recognition engine for identifying optimization opportunities
#[derive(Debug)]
pub struct PatternRecognitionEngine {
    /// Known performance patterns
    patterns: Vec<PerformancePattern>,
    /// Machine learning model for advanced pattern recognition
    ml_model: Option<MLOptimizationModel>,
    /// Historical pattern matches for learning
    pattern_history: Vec<PatternMatch>,
}

#[derive(Debug, Clone)]
pub struct PerformancePattern {
    pub pattern_id: String,
    pub name: String,
    pub description: String,
    pub indicators: Vec<PatternIndicator>,
    pub recommendations: Vec<String>,
    pub confidence_threshold: f64,
    pub success_rate: f64,
    pub impact_score: f64,
}

#[derive(Debug, Clone)]
pub struct PatternIndicator {
    pub metric: String,
    pub operator: ComparisonOperator,
    pub threshold: f64,
    pub weight: f64,
}

#[derive(Debug, Clone)]
pub enum ComparisonOperator {
    Greater,
    Less,
    Equal,
    GreaterEqual,
    LessEqual,
    NotEqual,
    Between(f64, f64),
    OutsideRange(f64, f64),
}

/// Advanced machine learning model for optimization pattern recognition
#[derive(Debug)]
pub struct MLOptimizationModel {
    /// Model is trained and ready
    pub trained: bool,
    /// Current model accuracy
    pub accuracy: f64,
    /// Training features and their weights
    feature_weights: HashMap<String, f64>,
    /// Decision tree nodes for pattern classification
    decision_tree: Vec<DecisionNode>,
    /// Training data samples
    training_samples: Vec<TrainingSample>,
    /// Model configuration
    config: MLModelConfig,
}

#[derive(Debug, Clone)]
pub struct DecisionNode {
    pub feature: String,
    pub threshold: f64,
    pub left_child: Option<usize>,
    pub right_child: Option<usize>,
    pub prediction: Option<String>,
    pub confidence: f64,
}

#[derive(Debug, Clone)]
pub struct TrainingSample {
    pub features: HashMap<String, f64>,
    pub target: String,
    pub outcome_score: f64,
}

#[derive(Debug, Clone)]
pub struct MLModelConfig {
    pub max_tree_depth: usize,
    pub min_samples_split: usize,
    pub learning_rate: f64,
    pub feature_importance_threshold: f64,
}

#[derive(Debug, Clone)]
pub struct PatternMatch {
    pub pattern_id: String,
    pub confidence: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub metrics: HashMap<String, f64>,
    pub applied_optimizations: Vec<String>,
    pub performance_improvement: Option<f64>,
}

/// Recommendation engine for generating optimization suggestions
#[derive(Debug)]
pub struct RecommendationEngine {
    /// Rule-based recommendations
    rules: Vec<RecommendationRule>,
    /// Priority weights
    priority_weights: HashMap<String, f64>,
}

#[derive(Debug, Clone)]
pub struct RecommendationRule {
    pub rule_id: String,
    pub condition: RuleCondition,
    pub recommendation: OptimizationRecommendation,
}

#[derive(Debug, Clone)]
pub enum RuleCondition {
    PhaseTimeExceeds(CompilationPhase, Duration),
    MemoryUsageExceeds(usize),
    BottleneckDetected(String),
    PatternMatched(String),
    ComplexityExceeds(f64),
}

impl EnhancedPerformanceAnalyzer {
    /// Create a new enhanced performance analyzer
    pub fn new() -> Self {
        Self::with_config(AnalysisConfig::default())
    }

    /// Create analyzer with custom configuration
    pub fn with_config(config: AnalysisConfig) -> Self {
        Self {
            config,
            metrics_collector: MetricsCollector::new(),
            pattern_engine: PatternRecognitionEngine::new(),
            recommendation_engine: RecommendationEngine::new(),
            historical_data: Vec::new(),
        }
    }

    /// Perform comprehensive analysis of compilation performance
    #[instrument(skip(self, source))]
    pub async fn analyze_compilation(
        &mut self,
        source: &str,
        file_path: &str,
        optimization_level: crate::optimization::OptimizationLevel,
    ) -> Result<EnhancedAnalysisResult> {
        info!("Starting enhanced performance analysis for {}", file_path);
        
        let analysis_start = Instant::now();

        // Start metrics collection
        self.metrics_collector.start_collection();

        // Simulate compilation phases and collect metrics
        let phase_results = self.analyze_compilation_phases(source, file_path, optimization_level).await?;

        // Stop metrics collection
        let metrics = self.metrics_collector.stop_collection();

        // Detect bottlenecks
        let bottlenecks = self.detect_bottlenecks(&phase_results, &metrics)?;

        // Recognize patterns
        let patterns = self.pattern_engine.recognize_patterns(&metrics)?;

        // Generate recommendations
        let recommendations = self.recommendation_engine.generate_recommendations(
            &phase_results,
            &bottlenecks,
            &patterns,
        )?;

        // Predict improvements
        let predicted_improvements = self.predict_improvements(&recommendations)?;

        // Analyze resource usage
        let resource_usage = self.analyze_resource_usage(&metrics)?;

        // Calculate complexity metrics
        let complexity_metrics = self.calculate_complexity_metrics(source)?;

        // Create summary
        let summary = self.create_analysis_summary(
            &phase_results,
            &bottlenecks,
            &recommendations,
            analysis_start.elapsed(),
        )?;

        // Historical comparison
        let historical_comparison = self.compare_with_history(&summary)?;

        let result = EnhancedAnalysisResult {
            summary,
            phase_analysis: phase_results,
            bottlenecks,
            recommendations,
            predicted_improvements,
            resource_usage,
            complexity_metrics,
            historical_comparison,
        };

        // Store in historical data
        self.historical_data.push(AnalysisSnapshot {
            timestamp: chrono::Utc::now(),
            result: result.clone(),
        });

        // Keep only recent history (last 10 analyses)
        if self.historical_data.len() > 10 {
            self.historical_data.remove(0);
        }

        info!("Enhanced performance analysis completed in {:?}", analysis_start.elapsed());
        Ok(result)
    }

    /// Analyze individual compilation phases
    async fn analyze_compilation_phases(
        &mut self,
        source: &str,
        _file_path: &str,
        _optimization_level: crate::optimization::OptimizationLevel,
    ) -> Result<HashMap<CompilationPhase, PhaseMetrics>> {
        let mut results = HashMap::new();

        // Simulate analysis of each compilation phase
        let phases = vec![
            CompilationPhase::Lexing,
            CompilationPhase::Parsing,
            CompilationPhase::SemanticAnalysis,
            CompilationPhase::TypeChecking,
            CompilationPhase::IRGeneration,
            CompilationPhase::LLVMOptimization,
            CompilationPhase::CodeGeneration,
            CompilationPhase::Linking,
        ];

        for phase in phases {
            let phase_start = Instant::now();
            
            // Simulate phase execution with realistic timing
            let base_time = match phase {
                CompilationPhase::Lexing => Duration::from_millis(10),
                CompilationPhase::Parsing => Duration::from_millis(50),
                CompilationPhase::SemanticAnalysis => Duration::from_millis(100),
                CompilationPhase::TypeChecking => Duration::from_millis(80),
                CompilationPhase::IRGeneration => Duration::from_millis(60),
                CompilationPhase::LLVMOptimization => Duration::from_millis(200),
                CompilationPhase::CodeGeneration => Duration::from_millis(40),
                CompilationPhase::Linking => Duration::from_millis(30),
                CompilationPhase::Total => Duration::from_millis(0),
            };

            // Scale timing based on source complexity
            let complexity_factor = (source.len() as f64 / 1000.0).max(1.0);
            let execution_time = Duration::from_nanos(
                (base_time.as_nanos() as f64 * complexity_factor) as u64
            );

            // Simulate memory usage
            let memory_usage = match phase {
                CompilationPhase::Parsing => source.len() * 3,
                CompilationPhase::SemanticAnalysis => source.len() * 2,
                CompilationPhase::TypeChecking => source.len() * 2,
                CompilationPhase::IRGeneration => source.len() * 4,
                CompilationPhase::LLVMOptimization => source.len() * 5,
                _ => source.len(),
            };

            let metrics = PhaseMetrics {
                execution_time,
                memory_usage,
                cpu_utilization: 70.0 + (phase as u8 as f64 * 5.0), // Simulate varying CPU usage
                operation_count: source.len() / 10,
                efficiency_score: self.calculate_phase_efficiency(&phase, execution_time, memory_usage),
                issues: self.detect_phase_issues(&phase, execution_time, memory_usage),
            };

            results.insert(phase, metrics);

            // Record metrics
            self.metrics_collector.record_phase_timing(phase.clone(), execution_time);
            self.metrics_collector.record_memory_usage(phase.clone(), memory_usage);
        }

        Ok(results)
    }

    /// Detect performance bottlenecks
    fn detect_bottlenecks(
        &self,
        phase_results: &HashMap<CompilationPhase, PhaseMetrics>,
        _metrics: &CompilationMetrics,
    ) -> Result<Vec<PerformanceBottleneck>> {
        let mut bottlenecks = Vec::new();

        // Find the slowest phase
        if let Some((slowest_phase, slowest_metrics)) = phase_results
            .iter()
            .max_by_key(|(_, metrics)| metrics.execution_time)
        {
            if slowest_metrics.execution_time > Duration::from_millis(100) {
                bottlenecks.push(PerformanceBottleneck {
                    id: format!("slow_phase_{:?}", slowest_phase),
                    description: format!("{:?} phase is taking too long", slowest_phase),
                    phase: slowest_phase.clone(),
                    severity: 7,
                    impact_percentage: 40.0,
                    solutions: vec![
                        "Enable aggressive optimization".to_string(),
                        "Consider parallel processing".to_string(),
                        "Optimize source code structure".to_string(),
                    ],
                    fix_complexity: ComplexityLevel::Medium,
                });
            }
        }

        // Check for memory usage issues
        for (phase, metrics) in phase_results {
            if metrics.memory_usage > 100_000 {
                bottlenecks.push(PerformanceBottleneck {
                    id: format!("high_memory_{:?}", phase),
                    description: format!("{:?} phase uses too much memory", phase),
                    phase: phase.clone(),
                    severity: 5,
                    impact_percentage: 20.0,
                    solutions: vec![
                        "Enable memory optimization passes".to_string(),
                        "Reduce compilation unit size".to_string(),
                    ],
                    fix_complexity: ComplexityLevel::Low,
                });
            }
        }

        Ok(bottlenecks)
    }

    /// Predict improvements from optimizations
    fn predict_improvements(
        &self,
        recommendations: &[OptimizationRecommendation],
    ) -> Result<HashMap<String, PredictedImprovement>> {
        let mut improvements = HashMap::new();

        for rec in recommendations {
            let improvement = PredictedImprovement {
                optimization: rec.title.clone(),
                speedup_factor: 1.0 + rec.expected_improvement,
                memory_reduction: rec.expected_improvement * 0.5, // Assume 50% of speedup translates to memory reduction
                confidence: rec.confidence * 0.8, // Slightly lower confidence for predictions
                basis: format!("Based on recommendation: {}", rec.description),
            };

            improvements.insert(rec.id.clone(), improvement);
        }

        Ok(improvements)
    }

    /// Analyze resource usage patterns
    fn analyze_resource_usage(&self, metrics: &CompilationMetrics) -> Result<ResourceUsageAnalysis> {
        Ok(ResourceUsageAnalysis {
            peak_memory: metrics.peak_memory_usage,
            average_memory: metrics.average_memory_usage,
            cpu_time: metrics.total_cpu_time,
            io_operations: metrics.io_operations.len(),
            cache_metrics: CacheMetrics {
                instruction_cache_hits: 1000,
                instruction_cache_misses: 100,
                data_cache_hits: 2000,
                data_cache_misses: 200,
                tlb_hits: 500,
                tlb_misses: 50,
            },
            allocation_patterns: AllocationPatterns {
                small_allocations: 100,
                large_allocations: 10,
                frequent_allocations: 50,
                long_lived_allocations: 20,
                fragmentation_ratio: 0.15,
            },
        })
    }

    /// Calculate code complexity metrics
    fn calculate_complexity_metrics(&self, source: &str) -> Result<ComplexityMetrics> {
        let lines = source.lines().count();
        let functions = source.matches("slay ").count(); // CURSED function keyword
        
        Ok(ComplexityMetrics {
            cyclomatic_complexity: (functions as f64 * 2.5).max(1.0),
            lines_of_code: lines,
            function_count: functions,
            type_complexity: (source.matches("squad ").count() as f64 * 1.5).max(1.0),
            dependency_complexity: (source.matches("import").count() as f64 * 0.8).max(1.0),
            template_complexity: (source.matches("<").count() as f64 * 0.3).max(1.0),
        })
    }

    /// Create analysis summary
    fn create_analysis_summary(
        &self,
        phase_results: &HashMap<CompilationPhase, PhaseMetrics>,
        bottlenecks: &[PerformanceBottleneck],
        recommendations: &[OptimizationRecommendation],
        total_time: Duration,
    ) -> Result<AnalysisSummary> {
        // Calculate overall performance score
        let avg_efficiency: f64 = phase_results
            .values()
            .map(|m| m.efficiency_score)
            .sum::<f64>() / phase_results.len() as f64;

        let performance_score = (avg_efficiency * 100.0).min(100.0).max(0.0);

        // Determine efficiency rating
        let efficiency_rating = match performance_score {
            90.0..=100.0 => EfficiencyRating::Excellent,
            75.0..=89.9 => EfficiencyRating::Good,
            60.0..=74.9 => EfficiencyRating::Average,
            40.0..=59.9 => EfficiencyRating::BelowAverage,
            _ => EfficiencyRating::Poor,
        };

        // Get primary bottleneck
        let primary_bottleneck = bottlenecks
            .iter()
            .max_by_key(|b| b.severity)
            .map(|b| b.description.clone());

        // Get top recommendation
        let top_recommendation = recommendations
            .iter()
            .max_by_key(|r| r.priority)
            .map(|r| r.title.clone());

        // Calculate improvement potential
        let improvement_potential = recommendations
            .iter()
            .map(|r| r.expected_improvement)
            .fold(0.0, |acc, x| acc + x)
            .min(1.0); // Cap at 100% improvement

        Ok(AnalysisSummary {
            total_time,
            performance_score,
            efficiency_rating,
            primary_bottleneck,
            top_recommendation,
            improvement_potential,
        })
    }

    /// Compare with historical data
    fn compare_with_history(&self, summary: &AnalysisSummary) -> Result<Option<HistoricalComparison>> {
        if let Some(previous) = self.historical_data.last() {
            let time_change = ((summary.total_time.as_secs_f64() - previous.result.summary.total_time.as_secs_f64()) 
                / previous.result.summary.total_time.as_secs_f64()) * 100.0;

            let trend = if time_change < -5.0 {
                PerformanceTrend::Improving
            } else if time_change > 5.0 {
                PerformanceTrend::Degrading
            } else {
                PerformanceTrend::Stable
            };

            Ok(Some(HistoricalComparison {
                previous_timestamp: previous.timestamp,
                performance_trend: trend,
                time_change_percentage: time_change,
                memory_change_percentage: 0.0, // Placeholder
                new_issues: Vec::new(),
                resolved_issues: Vec::new(),
            }))
        } else {
            Ok(None)
        }
    }

    /// Calculate efficiency score for a phase
    fn calculate_phase_efficiency(&self, phase: &CompilationPhase, time: Duration, memory: usize) -> f64 {
        // Normalize metrics and calculate efficiency
        let time_score = match phase {
            CompilationPhase::LLVMOptimization => {
                // LLVM optimization expected to take longer
                if time < Duration::from_millis(300) { 0.9 } else { 0.6 }
            }
            _ => {
                if time < Duration::from_millis(100) { 0.9 } else { 0.7 }
            }
        };

        let memory_score = if memory < 50_000 { 0.9 } else { 0.7 };

        (time_score + memory_score) / 2.0
    }

    /// Detect issues in a specific phase
    fn detect_phase_issues(&self, phase: &CompilationPhase, time: Duration, memory: usize) -> Vec<PhaseIssue> {
        let mut issues = Vec::new();

        if time > Duration::from_millis(200) {
            issues.push(PhaseIssue {
                issue_type: "slow_execution".to_string(),
                description: format!("{:?} phase is slower than expected", phase),
                severity: 6,
                suggested_fix: "Consider enabling optimization passes".to_string(),
            });
        }

        if memory > 100_000 {
            issues.push(PhaseIssue {
                issue_type: "high_memory".to_string(),
                description: format!("{:?} phase uses excessive memory", phase),
                severity: 5,
                suggested_fix: "Enable memory optimization".to_string(),
            });
        }

        issues
    }
}

/// Compilation metrics collected during analysis
#[derive(Debug)]
pub struct CompilationMetrics {
    pub peak_memory_usage: usize,
    pub average_memory_usage: usize,
    pub total_cpu_time: Duration,
    pub io_operations: Vec<IoOperation>,
    pub phase_timings: HashMap<CompilationPhase, Duration>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            phase_timings: HashMap::new(),
            memory_samples: Vec::new(),
            cpu_samples: Vec::new(),
            io_operations: Vec::new(),
        }
    }

    pub fn start_collection(&mut self) {
        // Initialize collection
        self.phase_timings.clear();
        self.memory_samples.clear();
        self.cpu_samples.clear();
        self.io_operations.clear();
    }

    pub fn stop_collection(&self) -> CompilationMetrics {
        let peak_memory = self.memory_samples
            .iter()
            .map(|s| s.bytes_used)
            .max()
            .unwrap_or(0);

        let average_memory = if !self.memory_samples.is_empty() {
            self.memory_samples.iter().map(|s| s.bytes_used).sum::<usize>() / self.memory_samples.len()
        } else {
            0
        };

        let total_cpu_time = self.phase_timings.values().sum();

        CompilationMetrics {
            peak_memory_usage: peak_memory,
            average_memory_usage: average_memory,
            total_cpu_time,
            io_operations: self.io_operations.clone(),
            phase_timings: self.phase_timings.clone(),
        }
    }

    pub fn record_phase_timing(&mut self, phase: CompilationPhase, duration: Duration) {
        self.phase_timings.insert(phase, duration);
    }

    pub fn record_memory_usage(&mut self, phase: CompilationPhase, bytes: usize) {
        self.memory_samples.push(MemorySample {
            timestamp: Instant::now(),
            phase,
            bytes_used: bytes,
            bytes_allocated: bytes,
        });
    }
}

impl PatternRecognitionEngine {
    pub fn new() -> Self {
        Self {
            patterns: Self::create_default_patterns(),
            ml_model: Some(MLOptimizationModel::new()),
            pattern_history: Vec::new(),
        }
    }

    pub fn recognize_patterns(&mut self, metrics: &CompilationMetrics) -> Result<Vec<String>> {
        let mut detected_patterns = Vec::new();
        
        // Extract features from metrics
        let features = self.extract_features(metrics);
        
        // Rule-based pattern recognition
        for pattern in &self.patterns {
            if self.evaluate_pattern(pattern, &features)? {
                detected_patterns.push(pattern.pattern_id.clone());
                
                // Record pattern match for learning
                self.pattern_history.push(PatternMatch {
                    pattern_id: pattern.pattern_id.clone(),
                    confidence: self.calculate_pattern_confidence(pattern, &features),
                    timestamp: chrono::Utc::now(),
                    metrics: features.clone(),
                    applied_optimizations: Vec::new(),
                    performance_improvement: None,
                });
            }
        }
        
        // ML-based pattern recognition
        if let Some(ref mut ml_model) = self.ml_model {
            let ml_patterns = ml_model.predict_patterns(&features)?;
            for ml_pattern in ml_patterns {
                if !detected_patterns.contains(&ml_pattern) {
                    detected_patterns.push(ml_pattern);
                }
            }
        }
        
        tracing::debug!("Detected {} patterns: {:?}", detected_patterns.len(), detected_patterns);
        Ok(detected_patterns)
    }

    fn extract_features(&self, metrics: &CompilationMetrics) -> HashMap<String, f64> {
        let mut features = HashMap::new();
        
        // Basic timing features
        features.insert("total_time_ms".to_string(), metrics.total_cpu_time.as_millis() as f64);
        features.insert("peak_memory_mb".to_string(), metrics.peak_memory_usage as f64 / 1024.0 / 1024.0);
        features.insert("avg_memory_mb".to_string(), metrics.average_memory_usage as f64 / 1024.0 / 1024.0);
        features.insert("io_operations_count".to_string(), metrics.io_operations.len() as f64);
        
        // Phase-specific features
        for (phase, duration) in &metrics.phase_timings {
            let phase_name = format!("{:?}_time_ms", phase).to_lowercase();
            features.insert(phase_name, duration.as_millis() as f64);
        }
        
        // Derived features
        if let Some(parsing_time) = metrics.phase_timings.get(&CompilationPhase::Parsing) {
            if let Some(total_time) = features.get("total_time_ms") {
                let parsing_ratio = parsing_time.as_millis() as f64 / total_time;
                features.insert("parsing_time_ratio".to_string(), parsing_ratio);
            }
        }
        
        if let Some(llvm_time) = metrics.phase_timings.get(&CompilationPhase::LLVMOptimization) {
            if let Some(total_time) = features.get("total_time_ms") {
                let llvm_ratio = llvm_time.as_millis() as f64 / total_time;
                features.insert("llvm_optimization_ratio".to_string(), llvm_ratio);
            }
        }
        
        // Memory efficiency features
        if metrics.peak_memory_usage > 0 {
            let memory_efficiency = metrics.average_memory_usage as f64 / metrics.peak_memory_usage as f64;
            features.insert("memory_efficiency".to_string(), memory_efficiency);
        }
        
        features
    }

    fn evaluate_pattern(&self, pattern: &PerformancePattern, features: &HashMap<String, f64>) -> Result<bool> {
        let mut total_score = 0.0;
        let mut total_weight = 0.0;
        
        for indicator in &pattern.indicators {
            let metric_value = features.get(&indicator.metric).unwrap_or(&0.0);
            let indicator_score = self.evaluate_indicator(indicator, *metric_value)?;
            
            total_score += indicator_score * indicator.weight;
            total_weight += indicator.weight;
        }
        
        let average_score = if total_weight > 0.0 { total_score / total_weight } else { 0.0 };
        Ok(average_score >= pattern.confidence_threshold)
    }

    fn evaluate_indicator(&self, indicator: &PatternIndicator, value: f64) -> Result<f64> {
        let matches = match &indicator.operator {
            ComparisonOperator::Greater => value > indicator.threshold,
            ComparisonOperator::GreaterEqual => value >= indicator.threshold,
            ComparisonOperator::Less => value < indicator.threshold,
            ComparisonOperator::LessEqual => value <= indicator.threshold,
            ComparisonOperator::Equal => (value - indicator.threshold).abs() < 1e-6,
            ComparisonOperator::NotEqual => (value - indicator.threshold).abs() >= 1e-6,
            ComparisonOperator::Between(min, max) => value >= *min && value <= *max,
            ComparisonOperator::OutsideRange(min, max) => value < *min || value > *max,
        };
        
        Ok(if matches { 1.0 } else { 0.0 })
    }

    fn calculate_pattern_confidence(&self, pattern: &PerformancePattern, features: &HashMap<String, f64>) -> f64 {
        let mut confidence = 0.0;
        let mut total_weight = 0.0;
        
        for indicator in &pattern.indicators {
            if let Some(&value) = features.get(&indicator.metric) {
                let indicator_confidence = match self.evaluate_indicator(indicator, value) {
                    Ok(score) => score,
                    Err(_) => 0.0,
                };
                confidence += indicator_confidence * indicator.weight;
                total_weight += indicator.weight;
            }
        }
        
        if total_weight > 0.0 {
            confidence / total_weight
        } else {
            0.0
        }
    }

    pub fn update_pattern_feedback(&mut self, pattern_id: &str, applied_optimizations: Vec<String>, improvement: f64) {
        // Update pattern history with feedback
        if let Some(pattern_match) = self.pattern_history.iter_mut()
            .find(|pm| pm.pattern_id == pattern_id) {
            pattern_match.applied_optimizations = applied_optimizations;
            pattern_match.performance_improvement = Some(improvement);
        }
        
        // Update pattern success rates
        if let Some(pattern) = self.patterns.iter_mut()
            .find(|p| p.pattern_id == pattern_id) {
            // Simple update rule: exponential moving average
            let alpha = 0.1;
            let success_score = if improvement > 0.0 { 1.0 } else { 0.0 };
            pattern.success_rate = alpha * success_score + (1.0 - alpha) * pattern.success_rate;
            pattern.impact_score = alpha * improvement + (1.0 - alpha) * pattern.impact_score;
        }
        
        // Train ML model with new data
        if let Some(ref mut ml_model) = self.ml_model {
            if let Some(pattern_match) = self.pattern_history.last() {
                ml_model.add_training_sample(
                    pattern_match.metrics.clone(),
                    pattern_id.to_string(),
                    improvement,
                );
            }
        }
    }

    fn create_default_patterns() -> Vec<PerformancePattern> {
        vec![
            PerformancePattern {
                pattern_id: "slow_parsing".to_string(),
                name: "Slow Parsing Pattern".to_string(),
                description: "Parsing phase takes disproportionately long".to_string(),
                indicators: vec![
                    PatternIndicator {
                        metric: "parsing_time_ratio".to_string(),
                        operator: ComparisonOperator::Greater,
                        threshold: 0.3, // More than 30% of total time
                        weight: 1.0,
                    },
                    PatternIndicator {
                        metric: "parsing_time_ms".to_string(),
                        operator: ComparisonOperator::Greater,
                        threshold: 100.0, // More than 100ms
                        weight: 0.5,
                    },
                ],
                recommendations: vec![
                    "Consider simplifying syntax".to_string(),
                    "Enable parser optimization".to_string(),
                    "Use incremental parsing".to_string(),
                ],
                confidence_threshold: 0.7,
                success_rate: 0.8,
                impact_score: 0.25,
            },
            PerformancePattern {
                pattern_id: "high_memory_usage".to_string(),
                name: "High Memory Usage Pattern".to_string(),
                description: "Compilation uses excessive memory".to_string(),
                indicators: vec![
                    PatternIndicator {
                        metric: "peak_memory_mb".to_string(),
                        operator: ComparisonOperator::Greater,
                        threshold: 1024.0, // More than 1GB
                        weight: 1.0,
                    },
                    PatternIndicator {
                        metric: "memory_efficiency".to_string(),
                        operator: ComparisonOperator::Less,
                        threshold: 0.6, // Less than 60% efficiency
                        weight: 0.8,
                    },
                ],
                recommendations: vec![
                    "Enable memory optimization passes".to_string(),
                    "Reduce compilation unit size".to_string(),
                    "Use streaming compilation".to_string(),
                ],
                confidence_threshold: 0.75,
                success_rate: 0.9,
                impact_score: 0.4,
            },
            PerformancePattern {
                pattern_id: "llvm_optimization_bottleneck".to_string(),
                name: "LLVM Optimization Bottleneck".to_string(),
                description: "LLVM optimization takes too much time".to_string(),
                indicators: vec![
                    PatternIndicator {
                        metric: "llvm_optimization_ratio".to_string(),
                        operator: ComparisonOperator::Greater,
                        threshold: 0.5, // More than 50% of total time
                        weight: 1.0,
                    },
                ],
                recommendations: vec![
                    "Reduce LLVM optimization level".to_string(),
                    "Use selective optimization".to_string(),
                    "Enable parallel LLVM passes".to_string(),
                ],
                confidence_threshold: 0.8,
                success_rate: 0.7,
                impact_score: 0.3,
            },
            PerformancePattern {
                pattern_id: "io_intensive_compilation".to_string(),
                name: "I/O Intensive Compilation".to_string(),
                description: "Compilation is bottlenecked by I/O operations".to_string(),
                indicators: vec![
                    PatternIndicator {
                        metric: "io_operations_count".to_string(),
                        operator: ComparisonOperator::Greater,
                        threshold: 1000.0, // More than 1000 I/O operations
                        weight: 1.0,
                    },
                ],
                recommendations: vec![
                    "Enable file caching".to_string(),
                    "Use memory-mapped files".to_string(),
                    "Batch I/O operations".to_string(),
                ],
                confidence_threshold: 0.6,
                success_rate: 0.85,
                impact_score: 0.35,
            },
        ]
    }
}

impl RecommendationEngine {
    pub fn new() -> Self {
        Self {
            rules: Self::create_default_rules(),
            priority_weights: Self::create_default_weights(),
        }
    }

    pub fn generate_recommendations(
        &self,
        phase_results: &HashMap<CompilationPhase, PhaseMetrics>,
        bottlenecks: &[PerformanceBottleneck],
        _patterns: &[String],
    ) -> Result<Vec<OptimizationRecommendation>> {
        let mut recommendations = Vec::new();

        // Generate recommendations based on bottlenecks
        for bottleneck in bottlenecks {
            let rec = OptimizationRecommendation {
                id: format!("rec_{}", bottleneck.id),
                title: format!("Address {}", bottleneck.description),
                description: format!("Optimize {:?} phase to reduce performance impact", bottleneck.phase),
                priority: bottleneck.severity,
                expected_improvement: bottleneck.impact_percentage / 100.0,
                effort_level: match bottleneck.fix_complexity {
                    ComplexityLevel::Low => EffortLevel::Low,
                    ComplexityLevel::Medium => EffortLevel::Medium,
                    ComplexityLevel::High => EffortLevel::High,
                    ComplexityLevel::VeryHigh => EffortLevel::Significant,
                },
                actions: vec![
                    RecommendationAction {
                        action_type: ActionType::EnableOptimization,
                        description: "Enable appropriate optimization passes".to_string(),
                        parameters: HashMap::new(),
                    },
                ],
                related_passes: bottleneck.solutions.clone(),
                confidence: 0.8,
            };
            recommendations.push(rec);
        }

        // Generate recommendations based on phase performance
        for (phase, metrics) in phase_results {
            if metrics.efficiency_score < 0.7 {
                recommendations.push(OptimizationRecommendation {
                    id: format!("phase_opt_{:?}", phase),
                    title: format!("Optimize {:?} Phase", phase),
                    description: format!("Improve efficiency of {:?} compilation phase", phase),
                    priority: 6,
                    expected_improvement: (0.8 - metrics.efficiency_score) * 0.5,
                    effort_level: EffortLevel::Medium,
                    actions: vec![
                        RecommendationAction {
                            action_type: ActionType::ConfigureParameter,
                            description: format!("Tune {:?} phase parameters", phase),
                            parameters: HashMap::new(),
                        },
                    ],
                    related_passes: vec![format!("{:?}_optimization", phase)],
                    confidence: 0.7,
                });
            }
        }

        // Sort by priority
        recommendations.sort_by_key(|r| std::cmp::Reverse(r.priority));

        Ok(recommendations)
    }

    fn create_default_rules() -> Vec<RecommendationRule> {
        Vec::new() // Placeholder
    }

    fn create_default_weights() -> HashMap<String, f64> {
        HashMap::new() // Placeholder
    }
}

impl MLOptimizationModel {
    pub fn new() -> Self {
        Self {
            trained: false,
            accuracy: 0.0,
            feature_weights: HashMap::new(),
            decision_tree: Vec::new(),
            training_samples: Vec::new(),
            config: MLModelConfig {
                max_tree_depth: 10,
                min_samples_split: 5,
                learning_rate: 0.1,
                feature_importance_threshold: 0.01,
            },
        }
    }

    pub fn predict_patterns(&self, features: &HashMap<String, f64>) -> Result<Vec<String>> {
        if !self.trained {
            return Ok(Vec::new());
        }

        let mut predictions = Vec::new();
        
        // Use decision tree for prediction
        if let Some(prediction) = self.predict_with_tree(features, 0) {
            predictions.push(prediction);
        }
        
        // Use weighted feature analysis
        let weighted_score = self.calculate_weighted_score(features);
        if weighted_score > 0.7 {
            predictions.push("ml_detected_optimization_opportunity".to_string());
        }
        
        Ok(predictions)
    }

    fn predict_with_tree(&self, features: &HashMap<String, f64>, node_index: usize) -> Option<String> {
        if node_index >= self.decision_tree.len() {
            return None;
        }
        
        let node = &self.decision_tree[node_index];
        
        // If leaf node, return prediction
        if let Some(ref prediction) = node.prediction {
            return Some(prediction.clone());
        }
        
        // Get feature value
        let feature_value = features.get(&node.feature).unwrap_or(&0.0);
        
        // Navigate tree based on threshold
        let next_node = if *feature_value <= node.threshold {
            node.left_child
        } else {
            node.right_child
        };
        
        if let Some(next_index) = next_node {
            self.predict_with_tree(features, next_index)
        } else {
            None
        }
    }

    fn calculate_weighted_score(&self, features: &HashMap<String, f64>) -> f64 {
        let mut score = 0.0;
        let mut total_weight = 0.0;
        
        for (feature, &weight) in &self.feature_weights {
            if let Some(&value) = features.get(feature) {
                // Normalize feature value (simple approach)
                let normalized_value = value / (value + 1.0);
                score += normalized_value * weight;
                total_weight += weight.abs();
            }
        }
        
        if total_weight > 0.0 {
            score / total_weight
        } else {
            0.0
        }
    }

    pub fn add_training_sample(&mut self, features: HashMap<String, f64>, target: String, outcome_score: f64) {
        self.training_samples.push(TrainingSample {
            features,
            target,
            outcome_score,
        });
        
        // Retrain if we have enough samples
        if self.training_samples.len() >= 50 && self.training_samples.len() % 10 == 0 {
            if let Err(e) = self.train() {
                tracing::warn!("Failed to retrain ML model: {}", e);
            }
        }
    }

    pub fn train(&mut self) -> Result<()> {
        if self.training_samples.len() < self.config.min_samples_split {
            return Ok(());
        }
        
        tracing::info!("Training ML optimization model with {} samples", self.training_samples.len());
        
        // Calculate feature importance
        self.calculate_feature_importance()?;
        
        // Build decision tree
        self.build_decision_tree()?;
        
        // Calculate model accuracy
        self.accuracy = self.calculate_accuracy()?;
        
        self.trained = true;
        
        tracing::info!("ML model training completed with accuracy: {:.2}%", self.accuracy * 100.0);
        Ok(())
    }

    fn calculate_feature_importance(&mut self) -> Result<()> {
        self.feature_weights.clear();
        
        if self.training_samples.is_empty() {
            return Ok(());
        }
        
        // Collect all feature names
        let mut all_features = HashSet::new();
        for sample in &self.training_samples {
            for feature in sample.features.keys() {
                all_features.insert(feature.clone());
            }
        }
        
        // Calculate correlation between each feature and outcome
        for feature in all_features {
            let correlation = self.calculate_feature_correlation(&feature)?;
            if correlation.abs() > self.config.feature_importance_threshold {
                self.feature_weights.insert(feature, correlation);
            }
        }
        
        tracing::debug!("Calculated importance for {} features", self.feature_weights.len());
        Ok(())
    }

    fn calculate_feature_correlation(&self, feature: &str) -> Result<f64> {
        let mut feature_values = Vec::new();
        let mut outcome_values = Vec::new();
        
        for sample in &self.training_samples {
            if let Some(&value) = sample.features.get(feature) {
                feature_values.push(value);
                outcome_values.push(sample.outcome_score);
            }
        }
        
        if feature_values.len() < 2 {
            return Ok(0.0);
        }
        
        // Simple correlation calculation (Pearson correlation coefficient)
        let n = feature_values.len() as f64;
        let sum_x: f64 = feature_values.iter().sum();
        let sum_y: f64 = outcome_values.iter().sum();
        let sum_xy: f64 = feature_values.iter().zip(&outcome_values).map(|(x, y)| x * y).sum();
        let sum_x2: f64 = feature_values.iter().map(|x| x * x).sum();
        let sum_y2: f64 = outcome_values.iter().map(|y| y * y).sum();
        
        let numerator = n * sum_xy - sum_x * sum_y;
        let denominator = ((n * sum_x2 - sum_x * sum_x) * (n * sum_y2 - sum_y * sum_y)).sqrt();
        
        if denominator.abs() < 1e-10 {
            Ok(0.0)
        } else {
            Ok(numerator / denominator)
        }
    }

    fn build_decision_tree(&mut self) -> Result<()> {
        self.decision_tree.clear();
        
        if self.training_samples.is_empty() {
            return Ok(());
        }
        
        // Create root node
        let indices: Vec<usize> = (0..self.training_samples.len()).collect();
        self.build_tree_recursive(&indices, 0)?;
        
        tracing::debug!("Built decision tree with {} nodes", self.decision_tree.len());
        Ok(())
    }

    fn build_tree_recursive(&mut self, sample_indices: &[usize], depth: usize) -> Result<usize> {
        // Create new node
        let node_index = self.decision_tree.len();
        self.decision_tree.push(DecisionNode {
            feature: String::new(),
            threshold: 0.0,
            left_child: None,
            right_child: None,
            prediction: None,
            confidence: 0.0,
        });
        
        // Check stopping criteria
        if depth >= self.config.max_tree_depth || sample_indices.len() < self.config.min_samples_split {
            // Create leaf node
            let prediction = self.get_majority_class(sample_indices);
            let confidence = self.calculate_node_confidence(sample_indices);
            
            self.decision_tree[node_index].prediction = Some(prediction);
            self.decision_tree[node_index].confidence = confidence;
            return Ok(node_index);
        }
        
        // Find best split
        if let Some((best_feature, best_threshold)) = self.find_best_split(sample_indices)? {
            self.decision_tree[node_index].feature = best_feature.clone();
            self.decision_tree[node_index].threshold = best_threshold;
            
            // Split samples
            let (left_indices, right_indices) = self.split_samples(sample_indices, &best_feature, best_threshold);
            
            // Create child nodes
            if !left_indices.is_empty() {
                let left_child = self.build_tree_recursive(&left_indices, depth + 1)?;
                self.decision_tree[node_index].left_child = Some(left_child);
            }
            
            if !right_indices.is_empty() {
                let right_child = self.build_tree_recursive(&right_indices, depth + 1)?;
                self.decision_tree[node_index].right_child = Some(right_child);
            }
        } else {
            // No good split found, make leaf
            let prediction = self.get_majority_class(sample_indices);
            let confidence = self.calculate_node_confidence(sample_indices);
            
            self.decision_tree[node_index].prediction = Some(prediction);
            self.decision_tree[node_index].confidence = confidence;
        }
        
        Ok(node_index)
    }

    fn find_best_split(&self, sample_indices: &[usize]) -> Result<Option<(String, f64)>> {
        let mut best_feature = None;
        let mut best_threshold = 0.0;
        let mut best_score = f64::NEG_INFINITY;
        
        // Try each feature
        for feature in self.feature_weights.keys() {
            // Get feature values for these samples
            let mut values: Vec<f64> = sample_indices
                .iter()
                .filter_map(|&i| self.training_samples[i].features.get(feature))
                .copied()
                .collect();
            
            if values.len() < 2 {
                continue;
            }
            
            values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            
            // Try different thresholds
            for i in 1..values.len() {
                let threshold = (values[i - 1] + values[i]) / 2.0;
                let score = self.calculate_split_score(sample_indices, feature, threshold);
                
                if score > best_score {
                    best_score = score;
                    best_feature = Some(feature.clone());
                    best_threshold = threshold;
                }
            }
        }
        
        if let Some(feature) = best_feature {
            Ok(Some((feature, best_threshold)))
        } else {
            Ok(None)
        }
    }

    fn calculate_split_score(&self, sample_indices: &[usize], feature: &str, threshold: f64) -> f64 {
        let (left_indices, right_indices) = self.split_samples(sample_indices, feature, threshold);
        
        if left_indices.is_empty() || right_indices.is_empty() {
            return f64::NEG_INFINITY;
        }
        
        // Calculate information gain (simplified)
        let total_entropy = self.calculate_entropy(sample_indices);
        let left_weight = left_indices.len() as f64 / sample_indices.len() as f64;
        let right_weight = right_indices.len() as f64 / sample_indices.len() as f64;
        
        let left_entropy = self.calculate_entropy(&left_indices);
        let right_entropy = self.calculate_entropy(&right_indices);
        
        total_entropy - (left_weight * left_entropy + right_weight * right_entropy)
    }

    fn split_samples(&self, sample_indices: &[usize], feature: &str, threshold: f64) -> (Vec<usize>, Vec<usize>) {
        let mut left = Vec::new();
        let mut right = Vec::new();
        
        for &index in sample_indices {
            if let Some(&value) = self.training_samples[index].features.get(feature) {
                if value <= threshold {
                    left.push(index);
                } else {
                    right.push(index);
                }
            }
        }
        
        (left, right)
    }

    fn calculate_entropy(&self, sample_indices: &[usize]) -> f64 {
        if sample_indices.is_empty() {
            return 0.0;
        }
        
        let mut class_counts = HashMap::new();
        for &index in sample_indices {
            let target = &self.training_samples[index].target;
            *class_counts.entry(target.clone()).or_insert(0) += 1;
        }
        
        let total = sample_indices.len() as f64;
        let mut entropy = 0.0;
        
        for count in class_counts.values() {
            let probability = *count as f64 / total;
            if probability > 0.0 {
                entropy -= probability * probability.log2();
            }
        }
        
        entropy
    }

    fn get_majority_class(&self, sample_indices: &[usize]) -> String {
        let mut class_counts = HashMap::new();
        for &index in sample_indices {
            let target = &self.training_samples[index].target;
            *class_counts.entry(target.clone()).or_insert(0) += 1;
        }
        
        class_counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(class, _)| class)
            .unwrap_or_else(|| "unknown".to_string())
    }

    fn calculate_node_confidence(&self, sample_indices: &[usize]) -> f64 {
        if sample_indices.is_empty() {
            return 0.0;
        }
        
        let mut class_counts = HashMap::new();
        for &index in sample_indices {
            let target = &self.training_samples[index].target;
            *class_counts.entry(target.clone()).or_insert(0) += 1;
        }
        
        let max_count = class_counts.values().max().unwrap_or(&0);
        *max_count as f64 / sample_indices.len() as f64
    }

    fn calculate_accuracy(&self) -> Result<f64> {
        if !self.trained || self.training_samples.is_empty() {
            return Ok(0.0);
        }
        
        let mut correct = 0;
        let mut total = 0;
        
        for sample in &self.training_samples {
            if let Ok(predictions) = self.predict_patterns(&sample.features) {
                if predictions.contains(&sample.target) {
                    correct += 1;
                }
            }
            total += 1;
        }
        
        Ok(if total > 0 { correct as f64 / total as f64 } else { 0.0 })
    }
}

impl Default for EnhancedPerformanceAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_enhanced_analysis() {
        let mut analyzer = EnhancedPerformanceAnalyzer::new();
        let source = r#"
            slay main() {
                facts x = 42;
                println("Hello, world!");
            }
        "#;

        let result = analyzer.analyze_compilation(
            source,
            "test.csd",
            crate::optimization::OptimizationLevel::O2,
        ).await;

        assert!(result.is_ok());
        let analysis = result.unwrap();
        
        assert!(analysis.summary.performance_score >= 0.0);
        assert!(analysis.summary.performance_score <= 100.0);
        assert!(!analysis.phase_analysis.is_empty());
    }

    #[test]
    fn test_metrics_collector() {
        let mut collector = MetricsCollector::new();
        collector.start_collection();
        
        collector.record_phase_timing(CompilationPhase::Parsing, Duration::from_millis(50));
        collector.record_memory_usage(CompilationPhase::Parsing, 1024);
        
        let metrics = collector.stop_collection();
        assert_eq!(metrics.peak_memory_usage, 1024);
        assert!(metrics.phase_timings.contains_key(&CompilationPhase::Parsing));
    }

    #[test]
    fn test_pattern_engine() {
        let engine = PatternRecognitionEngine::new();
        assert!(!engine.patterns.is_empty());
    }

    #[test]
    fn test_recommendation_engine() {
        let engine = RecommendationEngine::new();
        let phase_results = HashMap::new();
        let bottlenecks = Vec::new();
        let patterns = Vec::new();

        let result = engine.generate_recommendations(&phase_results, &bottlenecks, &patterns);
        assert!(result.is_ok());
    }
}
