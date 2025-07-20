/// Documentation Coverage Analyzer
/// 
/// Analyzes and reports on documentation coverage across CURSED modules,
/// providing detailed metrics and identifying gaps in documentation.

use crate::error::CursedError;
use crate::documentation::{Documentation, DocumentedModule, DocumentedFunction, DocumentedVariable, DocumentedConstant, DocumentedType};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};

/// Comprehensive documentation coverage analyzer
#[derive(Debug, Clone)]
pub struct CoverageAnalyzer {
    pub coverage_stats: HashMap<String, CoverageMetrics>,
    pub uncovered_items: Vec<UncoveredItem>,
    pub coverage_thresholds: CoverageThresholds,
    pub module_coverage: HashMap<String, ModuleCoverage>,
    pub global_coverage: GlobalCoverage,
}

/// Coverage metrics for a specific component
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageMetrics {
    pub total_items: usize,
    pub documented_items: usize,
    pub coverage_percentage: f64,
    pub missing_descriptions: usize,
    pub missing_examples: usize,
    pub missing_parameters: usize,
    pub quality_score: f64,
}

/// Items that lack documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UncoveredItem {
    pub item_type: String,
    pub item_name: String,
    pub module_name: String,
    pub source_file: String,
    pub source_line: usize,
    pub missing_elements: Vec<String>,
}

/// Coverage thresholds for different quality levels
#[derive(Debug, Clone)]
pub struct CoverageThresholds {
    pub minimum_coverage: f64,
    pub good_coverage: f64,
    pub excellent_coverage: f64,
    pub require_examples: bool,
    pub require_parameter_docs: bool,
}

/// Module-specific coverage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleCoverage {
    pub module_name: String,
    pub functions: CoverageMetrics,
    pub variables: CoverageMetrics,
    pub constants: CoverageMetrics,
    pub types: CoverageMetrics,
    pub overall: CoverageMetrics,
    pub quality_grade: CoverageGrade,
}

/// Global coverage across all modules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalCoverage {
    pub total_modules: usize,
    pub documented_modules: usize,
    pub total_functions: usize,
    pub documented_functions: usize,
    pub total_types: usize,
    pub documented_types: usize,
    pub overall_percentage: f64,
    pub quality_distribution: HashMap<CoverageGrade, usize>,
}

/// Coverage quality grades
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CoverageGrade {
    Excellent,  // 90%+
    Good,      // 70-89%
    Fair,      // 50-69%
    Poor,      // 30-49%
    Critical,  // <30%
}

/// Coverage report configuration
#[derive(Debug, Clone)]
pub struct CoverageReportConfig {
    pub include_missing_items: bool,
    pub include_quality_metrics: bool,
    pub include_suggestions: bool,
    pub format: ReportFormat,
    pub output_file: Option<String>,
}

/// Report output formats
#[derive(Debug, Clone)]
pub enum ReportFormat {
    Html,
    Markdown,
    Json,
    Console,
}

impl CoverageAnalyzer {
    /// Create a new coverage analyzer with default thresholds
    pub fn new() -> Self {
        Self {
            coverage_stats: HashMap::new(),
            uncovered_items: Vec::new(),
            coverage_thresholds: CoverageThresholds::default(),
            module_coverage: HashMap::new(),
            global_coverage: GlobalCoverage::default(),
        }
    }

    /// Create analyzer with custom thresholds
    pub fn with_thresholds(thresholds: CoverageThresholds) -> Self {
        Self {
            coverage_stats: HashMap::new(),
            uncovered_items: Vec::new(),
            coverage_thresholds: thresholds,
            module_coverage: HashMap::new(),
            global_coverage: GlobalCoverage::default(),
        }
    }

    /// Analyze documentation coverage for all modules
    pub fn analyze_documentation(&mut self, documentation: &Documentation) -> Result<(), CursedError> {
        // Reset state
        self.coverage_stats.clear();
        self.uncovered_items.clear();
        self.module_coverage.clear();

        // Analyze each module
        for module in &documentation.modules {
            let module_coverage = self.analyze_module_coverage(module)?;
            self.module_coverage.insert(module.name.clone(), module_coverage);
        }

        // Calculate global coverage
        self.calculate_global_coverage(documentation)?;

        Ok(())
    }

    /// Analyze coverage for a specific module
    pub fn analyze_module_coverage(&mut self, module: &DocumentedModule) -> Result<ModuleCoverage, CursedError> {
        let functions_coverage = self.analyze_functions_coverage(&module.functions, &module.name)?;
        let variables_coverage = self.analyze_variables_coverage(&module.variables, &module.name)?;
        let constants_coverage = self.analyze_constants_coverage(&module.constants, &module.name)?;
        let types_coverage = self.analyze_types_coverage(&module.types, &module.name)?;

        // Calculate overall module coverage
        let total_items = functions_coverage.total_items + variables_coverage.total_items + 
                         constants_coverage.total_items + types_coverage.total_items;
        let documented_items = functions_coverage.documented_items + variables_coverage.documented_items + 
                              constants_coverage.documented_items + types_coverage.documented_items;

        let overall_coverage = if total_items > 0 {
            (documented_items as f64 / total_items as f64) * 100.0
        } else {
            0.0
        };

        let overall_metrics = CoverageMetrics {
            total_items,
            documented_items,
            coverage_percentage: overall_coverage,
            missing_descriptions: 0, // Will be calculated separately
            missing_examples: 0,
            missing_parameters: 0,
            quality_score: self.calculate_quality_score(overall_coverage),
        };

        let quality_grade = self.calculate_coverage_grade(overall_coverage);

        Ok(ModuleCoverage {
            module_name: module.name.clone(),
            functions: functions_coverage,
            variables: variables_coverage,
            constants: constants_coverage,
            types: types_coverage,
            overall: overall_metrics,
            quality_grade,
        })
    }

    /// Analyze functions coverage
    fn analyze_functions_coverage(&mut self, functions: &[DocumentedFunction], module_name: &str) -> Result<CoverageMetrics, CursedError> {
        let total_items = functions.len();
        let mut documented_items = 0;
        let mut missing_descriptions = 0;
        let mut missing_examples = 0;
        let mut missing_parameters = 0;

        for function in functions {
            let mut is_documented = false;
            let mut missing_elements = Vec::new();

            // Check description
            if !function.description.is_empty() {
                is_documented = true;
            } else {
                missing_descriptions += 1;
                missing_elements.push("description".to_string());
            }

            // Check examples if required
            if self.coverage_thresholds.require_examples && function.examples.is_empty() {
                missing_examples += 1;
                missing_elements.push("examples".to_string());
            }

            // Check parameter documentation if required
            if self.coverage_thresholds.require_parameter_docs {
                for param in &function.parameters {
                    if param.description.is_empty() {
                        missing_parameters += 1;
                        missing_elements.push(format!("parameter: {}", param.name));
                    }
                }
            }

            if is_documented {
                documented_items += 1;
            }

            // Record uncovered item if necessary
            if !missing_elements.is_empty() {
                self.uncovered_items.push(UncoveredItem {
                    item_type: "function".to_string(),
                    item_name: function.name.clone(),
                    module_name: module_name.to_string(),
                    source_file: function.source_file.clone(),
                    source_line: function.source_line,
                    missing_elements,
                });
            }
        }

        let coverage_percentage = if total_items > 0 {
            (documented_items as f64 / total_items as f64) * 100.0
        } else {
            0.0
        };

        Ok(CoverageMetrics {
            total_items,
            documented_items,
            coverage_percentage,
            missing_descriptions,
            missing_examples,
            missing_parameters,
            quality_score: self.calculate_quality_score(coverage_percentage),
        })
    }

    /// Analyze variables coverage
    fn analyze_variables_coverage(&mut self, variables: &[DocumentedVariable], module_name: &str) -> Result<CoverageMetrics, CursedError> {
        let total_items = variables.len();
        let mut documented_items = 0;
        let mut missing_descriptions = 0;

        for variable in variables {
            let mut missing_elements = Vec::new();

            if !variable.description.is_empty() {
                documented_items += 1;
            } else {
                missing_descriptions += 1;
                missing_elements.push("description".to_string());

                self.uncovered_items.push(UncoveredItem {
                    item_type: "variable".to_string(),
                    item_name: variable.name.clone(),
                    module_name: module_name.to_string(),
                    source_file: variable.source_file.clone(),
                    source_line: variable.source_line,
                    missing_elements,
                });
            }
        }

        let coverage_percentage = if total_items > 0 {
            (documented_items as f64 / total_items as f64) * 100.0
        } else {
            0.0
        };

        Ok(CoverageMetrics {
            total_items,
            documented_items,
            coverage_percentage,
            missing_descriptions,
            missing_examples: 0,
            missing_parameters: 0,
            quality_score: self.calculate_quality_score(coverage_percentage),
        })
    }

    /// Analyze constants coverage
    fn analyze_constants_coverage(&mut self, constants: &[DocumentedConstant], module_name: &str) -> Result<CoverageMetrics, CursedError> {
        let total_items = constants.len();
        let mut documented_items = 0;
        let mut missing_descriptions = 0;

        for constant in constants {
            let mut missing_elements = Vec::new();

            if !constant.description.is_empty() {
                documented_items += 1;
            } else {
                missing_descriptions += 1;
                missing_elements.push("description".to_string());

                self.uncovered_items.push(UncoveredItem {
                    item_type: "constant".to_string(),
                    item_name: constant.name.clone(),
                    module_name: module_name.to_string(),
                    source_file: constant.source_file.clone(),
                    source_line: constant.source_line,
                    missing_elements,
                });
            }
        }

        let coverage_percentage = if total_items > 0 {
            (documented_items as f64 / total_items as f64) * 100.0
        } else {
            0.0
        };

        Ok(CoverageMetrics {
            total_items,
            documented_items,
            coverage_percentage,
            missing_descriptions,
            missing_examples: 0,
            missing_parameters: 0,
            quality_score: self.calculate_quality_score(coverage_percentage),
        })
    }

    /// Analyze types coverage
    fn analyze_types_coverage(&mut self, types: &[DocumentedType], module_name: &str) -> Result<CoverageMetrics, CursedError> {
        let total_items = types.len();
        let mut documented_items = 0;
        let mut missing_descriptions = 0;

        for doc_type in types {
            let mut missing_elements = Vec::new();

            if !doc_type.description.is_empty() {
                documented_items += 1;
            } else {
                missing_descriptions += 1;
                missing_elements.push("description".to_string());

                self.uncovered_items.push(UncoveredItem {
                    item_type: doc_type.type_kind.clone(),
                    item_name: doc_type.name.clone(),
                    module_name: module_name.to_string(),
                    source_file: doc_type.source_file.clone(),
                    source_line: doc_type.source_line,
                    missing_elements,
                });
            }
        }

        let coverage_percentage = if total_items > 0 {
            (documented_items as f64 / total_items as f64) * 100.0
        } else {
            0.0
        };

        Ok(CoverageMetrics {
            total_items,
            documented_items,
            coverage_percentage,
            missing_descriptions,
            missing_examples: 0,
            missing_parameters: 0,
            quality_score: self.calculate_quality_score(coverage_percentage),
        })
    }

    /// Calculate global coverage statistics
    fn calculate_global_coverage(&mut self, documentation: &Documentation) -> Result<(), CursedError> {
        let mut total_modules = documentation.modules.len();
        let mut documented_modules = 0;
        let mut total_functions = 0;
        let mut documented_functions = 0;
        let mut total_types = 0;
        let mut documented_types = 0;
        let mut quality_distribution = HashMap::new();

        // Initialize quality distribution
        quality_distribution.insert(CoverageGrade::Excellent, 0);
        quality_distribution.insert(CoverageGrade::Good, 0);
        quality_distribution.insert(CoverageGrade::Fair, 0);
        quality_distribution.insert(CoverageGrade::Poor, 0);
        quality_distribution.insert(CoverageGrade::Critical, 0);

        for module_coverage in self.module_coverage.values() {
            // Count documented modules (those with some documentation)
            if module_coverage.overall.documented_items > 0 {
                documented_modules += 1;
            }

            // Aggregate function and type counts
            total_functions += module_coverage.functions.total_items;
            documented_functions += module_coverage.functions.documented_items;
            total_types += module_coverage.types.total_items;
            documented_types += module_coverage.types.documented_items;

            // Update quality distribution
            *quality_distribution.get_mut(&module_coverage.quality_grade).unwrap() += 1;
        }

        let overall_percentage = if total_functions + total_types > 0 {
            ((documented_functions + documented_types) as f64 / (total_functions + total_types) as f64) * 100.0
        } else {
            0.0
        };

        self.global_coverage = GlobalCoverage {
            total_modules,
            documented_modules,
            total_functions,
            documented_functions,
            total_types,
            documented_types,
            overall_percentage,
            quality_distribution,
        };

        Ok(())
    }

    /// Calculate quality score based on various factors
    fn calculate_quality_score(&self, coverage_percentage: f64) -> f64 {
        // Base score from coverage percentage
        let mut score = coverage_percentage;

        // Quality score is capped at 100
        score.min(100.0)
    }

    /// Calculate coverage grade based on percentage
    fn calculate_coverage_grade(&self, coverage_percentage: f64) -> CoverageGrade {
        if coverage_percentage >= self.coverage_thresholds.excellent_coverage {
            CoverageGrade::Excellent
        } else if coverage_percentage >= self.coverage_thresholds.good_coverage {
            CoverageGrade::Good
        } else if coverage_percentage >= 50.0 {
            CoverageGrade::Fair
        } else if coverage_percentage >= 30.0 {
            CoverageGrade::Poor
        } else {
            CoverageGrade::Critical
        }
    }

    /// Generate comprehensive coverage report
    pub fn generate_report(&self, config: &CoverageReportConfig) -> Result<String, CursedError> {
        match config.format {
            ReportFormat::Html => self.generate_html_report(config),
            ReportFormat::Markdown => self.generate_markdown_report(config),
            ReportFormat::Json => self.generate_json_report(config),
            ReportFormat::Console => self.generate_console_report(config),
        }
    }

    /// Generate HTML coverage report
    fn generate_html_report(&self, config: &CoverageReportConfig) -> Result<String, CursedError> {
        let mut html = String::new();

        html.push_str(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Documentation Coverage Report</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 2rem; }
        .coverage-summary { background: #f8f9fa; padding: 1rem; border-radius: 4px; margin-bottom: 2rem; }
        .module-coverage { margin-bottom: 1rem; padding: 1rem; border: 1px solid #dee2e6; border-radius: 4px; }
        .coverage-bar { width: 100%; height: 20px; background: #e9ecef; border-radius: 10px; overflow: hidden; }
        .coverage-fill { height: 100%; transition: width 0.3s; }
        .excellent { background: #28a745; }
        .good { background: #17a2b8; }
        .fair { background: #ffc107; }
        .poor { background: #fd7e14; }
        .critical { background: #dc3545; }
        .uncovered-list { margin-top: 1rem; }
        .uncovered-item { background: #fff3cd; padding: 0.5rem; margin: 0.25rem 0; border-radius: 4px; }
    </style>
</head>
<body>
    <h1>Documentation Coverage Report</h1>
"#);

        // Global coverage summary
        html.push_str(&format!(r#"
    <div class="coverage-summary">
        <h2>Global Coverage: {:.1}%</h2>
        <div class="coverage-bar">
            <div class="coverage-fill {}" style="width: {:.1}%"></div>
        </div>
        <p>
            Modules: {}/{} documented ({:.1}%)<br>
            Functions: {}/{} documented ({:.1}%)<br>
            Types: {}/{} documented ({:.1}%)
        </p>
    </div>
"#, 
            self.global_coverage.overall_percentage,
            self.get_css_class_for_percentage(self.global_coverage.overall_percentage),
            self.global_coverage.overall_percentage,
            self.global_coverage.documented_modules,
            self.global_coverage.total_modules,
            if self.global_coverage.total_modules > 0 { 
                (self.global_coverage.documented_modules as f64 / self.global_coverage.total_modules as f64) * 100.0 
            } else { 0.0 },
            self.global_coverage.documented_functions,
            self.global_coverage.total_functions,
            if self.global_coverage.total_functions > 0 { 
                (self.global_coverage.documented_functions as f64 / self.global_coverage.total_functions as f64) * 100.0 
            } else { 0.0 },
            self.global_coverage.documented_types,
            self.global_coverage.total_types,
            if self.global_coverage.total_types > 0 { 
                (self.global_coverage.documented_types as f64 / self.global_coverage.total_types as f64) * 100.0 
            } else { 0.0 }
        ));

        // Module coverage details
        html.push_str("<h2>Module Coverage</h2>");
        for (module_name, coverage) in &self.module_coverage {
            html.push_str(&format!(r#"
    <div class="module-coverage">
        <h3>{}</h3>
        <div class="coverage-bar">
            <div class="coverage-fill {}" style="width: {:.1}%"></div>
        </div>
        <p>Overall: {:.1}% ({} documented / {} total)</p>
        <ul>
            <li>Functions: {:.1}% ({}/{})</li>
            <li>Variables: {:.1}% ({}/{})</li>
            <li>Constants: {:.1}% ({}/{})</li>
            <li>Types: {:.1}% ({}/{})</li>
        </ul>
    </div>
"#, 
                module_name,
                self.get_css_class_for_percentage(coverage.overall.coverage_percentage),
                coverage.overall.coverage_percentage,
                coverage.overall.coverage_percentage,
                coverage.overall.documented_items,
                coverage.overall.total_items,
                coverage.functions.coverage_percentage,
                coverage.functions.documented_items,
                coverage.functions.total_items,
                coverage.variables.coverage_percentage,
                coverage.variables.documented_items,
                coverage.variables.total_items,
                coverage.constants.coverage_percentage,
                coverage.constants.documented_items,
                coverage.constants.total_items,
                coverage.types.coverage_percentage,
                coverage.types.documented_items,
                coverage.types.total_items
            ));
        }

        // Uncovered items if requested
        if config.include_missing_items && !self.uncovered_items.is_empty() {
            html.push_str("<h2>Uncovered Items</h2>");
            html.push_str("<div class=\"uncovered-list\">");
            
            for item in &self.uncovered_items {
                html.push_str(&format!(r#"
    <div class="uncovered-item">
        <strong>{}::{}</strong> ({})<br>
        File: {} (line {})<br>
        Missing: {}
    </div>
"#, 
                    item.module_name,
                    item.item_name,
                    item.item_type,
                    item.source_file,
                    item.source_line,
                    item.missing_elements.join(", ")
                ));
            }
            
            html.push_str("</div>");
        }

        html.push_str("</body></html>");

        Ok(html)
    }

    /// Generate Markdown coverage report
    fn generate_markdown_report(&self, config: &CoverageReportConfig) -> Result<String, CursedError> {
        let mut report = String::new();

        report.push_str("# Documentation Coverage Report\n\n");

        // Global summary
        report.push_str(&format!("## Global Coverage: {:.1}%\n\n", self.global_coverage.overall_percentage));
        report.push_str(&format!("- **Modules**: {}/{} documented ({:.1}%)\n", 
            self.global_coverage.documented_modules, 
            self.global_coverage.total_modules,
            if self.global_coverage.total_modules > 0 { 
                (self.global_coverage.documented_modules as f64 / self.global_coverage.total_modules as f64) * 100.0 
            } else { 0.0 }
        ));
        report.push_str(&format!("- **Functions**: {}/{} documented ({:.1}%)\n", 
            self.global_coverage.documented_functions, 
            self.global_coverage.total_functions,
            if self.global_coverage.total_functions > 0 { 
                (self.global_coverage.documented_functions as f64 / self.global_coverage.total_functions as f64) * 100.0 
            } else { 0.0 }
        ));
        report.push_str(&format!("- **Types**: {}/{} documented ({:.1}%)\n\n", 
            self.global_coverage.documented_types, 
            self.global_coverage.total_types,
            if self.global_coverage.total_types > 0 { 
                (self.global_coverage.documented_types as f64 / self.global_coverage.total_types as f64) * 100.0 
            } else { 0.0 }
        ));

        // Module details
        report.push_str("## Module Coverage\n\n");
        report.push_str("| Module | Overall | Functions | Variables | Constants | Types | Grade |\n");
        report.push_str("|--------|---------|-----------|-----------|-----------|-------|-------|\n");

        for (module_name, coverage) in &self.module_coverage {
            report.push_str(&format!("| {} | {:.1}% | {:.1}% | {:.1}% | {:.1}% | {:.1}% | {:?} |\n",
                module_name,
                coverage.overall.coverage_percentage,
                coverage.functions.coverage_percentage,
                coverage.variables.coverage_percentage,
                coverage.constants.coverage_percentage,
                coverage.types.coverage_percentage,
                coverage.quality_grade
            ));
        }

        // Uncovered items if requested
        if config.include_missing_items && !self.uncovered_items.is_empty() {
            report.push_str("\n## Uncovered Items\n\n");
            
            for item in &self.uncovered_items {
                report.push_str(&format!("- **{}::{}** ({})\n", 
                    item.module_name, item.item_name, item.item_type));
                report.push_str(&format!("  - File: {} (line {})\n", 
                    item.source_file, item.source_line));
                report.push_str(&format!("  - Missing: {}\n\n", 
                    item.missing_elements.join(", ")));
            }
        }

        Ok(report)
    }

    /// Generate JSON coverage report
    fn generate_json_report(&self, _config: &CoverageReportConfig) -> Result<String, CursedError> {
        let report_data = serde_json::json!({
            "global_coverage": self.global_coverage,
            "module_coverage": self.module_coverage,
            "uncovered_items": self.uncovered_items,
            "thresholds": {
                "minimum": self.coverage_thresholds.minimum_coverage,
                "good": self.coverage_thresholds.good_coverage,
                "excellent": self.coverage_thresholds.excellent_coverage
            }
        });

        serde_json::to_string_pretty(&report_data)
            .map_err(|e| CursedError::IoError(format!("Failed to serialize JSON: {}", e)))
    }

    /// Generate console coverage report
    fn generate_console_report(&self, config: &CoverageReportConfig) -> Result<String, CursedError> {
        let mut report = String::new();

        report.push_str("═══════════════════════════════════════════════════════════════\n");
        report.push_str("                 DOCUMENTATION COVERAGE REPORT\n");
        report.push_str("═══════════════════════════════════════════════════════════════\n\n");

        // Global summary with visual bar
        report.push_str(&format!("Global Coverage: {:.1}% {}\n\n", 
            self.global_coverage.overall_percentage,
            self.create_console_progress_bar(self.global_coverage.overall_percentage)
        ));

        // Module breakdown
        report.push_str("Module Breakdown:\n");
        report.push_str("─────────────────────────────────────────────────────────────\n");
        
        for (module_name, coverage) in &self.module_coverage {
            let grade_icon = match coverage.quality_grade {
                CoverageGrade::Excellent => "🟢",
                CoverageGrade::Good => "🔵", 
                CoverageGrade::Fair => "🟡",
                CoverageGrade::Poor => "🟠",
                CoverageGrade::Critical => "🔴",
            };

            report.push_str(&format!("{} {} {:.1}% {}\n",
                grade_icon,
                module_name,
                coverage.overall.coverage_percentage,
                self.create_console_progress_bar(coverage.overall.coverage_percentage)
            ));
        }

        if config.include_missing_items && !self.uncovered_items.is_empty() {
            report.push_str("\nUncovered Items:\n");
            report.push_str("─────────────────────────────────────────────────────────────\n");
            
            for item in &self.uncovered_items {
                report.push_str(&format!("⚠️  {}::{} ({})\n", 
                    item.module_name, item.item_name, item.item_type));
                report.push_str(&format!("   Missing: {}\n", 
                    item.missing_elements.join(", ")));
            }
        }

        Ok(report)
    }

    /// Create console progress bar
    fn create_console_progress_bar(&self, percentage: f64) -> String {
        let bar_length = 20;
        let filled = ((percentage / 100.0) * bar_length as f64) as usize;
        let empty = bar_length - filled;
        
        format!("[{}{}] {:.1}%", 
            "█".repeat(filled), 
            "░".repeat(empty), 
            percentage)
    }

    /// Get CSS class for coverage percentage
    fn get_css_class_for_percentage(&self, percentage: f64) -> &str {
        if percentage >= self.coverage_thresholds.excellent_coverage {
            "excellent"
        } else if percentage >= self.coverage_thresholds.good_coverage {
            "good"
        } else if percentage >= 50.0 {
            "fair"
        } else if percentage >= 30.0 {
            "poor"
        } else {
            "critical"
        }
    }

    /// Save coverage report to file
    pub fn save_report(&self, config: &CoverageReportConfig) -> Result<(), CursedError> {
        if let Some(output_file) = &config.output_file {
            let report = self.generate_report(config)?;
            fs::write(output_file, report)
                .map_err(|e| CursedError::IoError(format!("Failed to write report: {}", e)))?;
        }
        Ok(())
    }
}

impl Default for CoverageThresholds {
    fn default() -> Self {
        Self {
            minimum_coverage: 30.0,
            good_coverage: 70.0,
            excellent_coverage: 90.0,
            require_examples: false,
            require_parameter_docs: true,
        }
    }
}

impl Default for GlobalCoverage {
    fn default() -> Self {
        Self {
            total_modules: 0,
            documented_modules: 0,
            total_functions: 0,
            documented_functions: 0,
            total_types: 0,
            documented_types: 0,
            overall_percentage: 0.0,
            quality_distribution: HashMap::new(),
        }
    }
}

impl Default for CoverageReportConfig {
    fn default() -> Self {
        Self {
            include_missing_items: true,
            include_quality_metrics: true,
            include_suggestions: true,
            format: ReportFormat::Console,
            output_file: None,
        }
    }
}
