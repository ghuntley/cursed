/// Coverage analyzer for documentation generation
use crate::error::CursedError;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CoverageAnalyzer {
    pub coverage_stats: HashMap<String, f64>,
    pub uncovered_items: Vec<String>,
}

impl CoverageAnalyzer {
    pub fn new() -> Self {
        Self {
            coverage_stats: HashMap::new(),
            uncovered_items: Vec::new(),
        }
    }

    pub fn analyze_coverage(&mut self, items: &[String]) -> Result<f64, CursedError> {
        let total_items = items.len();
        if total_items == 0 {
            return Ok(0.0);
        }

        let covered_items = items.len() - self.uncovered_items.len();
        let coverage = (covered_items as f64) / (total_items as f64) * 100.0;
        Ok(coverage)
    }

    pub fn generate_report(&self) -> Result<String, CursedError> {
        let mut report = String::new();
        report.push_str("# Coverage Report\n\n");
        
        for (module, coverage) in &self.coverage_stats {
            report.push_str(&format!("- {}: {:.2}%\n", module, coverage));
        }
        
        Ok(report)
    }
}
