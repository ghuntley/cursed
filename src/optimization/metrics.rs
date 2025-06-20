/// Optimization Metrics and Compilation Units
/// 
/// Defines structures for tracking compilation units and their optimization metrics.

use std::collections::HashMap;
use std::time::Duration;
use serde::{Deserialize, Serialize};

/// System-level performance statistics
#[derive(Debug, Clone, Default)]
pub struct SystemStatistics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub io_throughput: f64,
    pub cache_hit_rate: f64,
}

/// Resource utilization statistics
#[derive(Debug, Clone, Default)]
pub struct ResourceStatistics {
    pub total_memory: u64,
    pub used_memory: u64,
    pub cpu_cores: u32,
    pub thread_count: u32,
}

/// Compilation unit for optimization tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationUnit {
    /// Unique name/identifier for the compilation unit
    pub name: String,
    
    /// Source files that make up this unit
    pub source_files: Vec<String>,
    
    /// Dependencies on other compilation units
    pub dependencies: Vec<String>,
    
    /// Estimated size in bytes
    pub estimated_size_bytes: usize,
    
    /// Optimization metadata
    pub optimization_metadata: HashMap<String, String>,
    
    /// Compilation timing information
    pub timing_info: Option<CompilationTiming>,
    
    /// Unit type (library, binary, test, etc.)
    pub unit_type: CompilationUnitType,
    
    /// Target platform information
    pub target_info: Option<TargetInfo>,
}

/// Type of compilation unit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompilationUnitType {
    /// Binary executable
    Binary,
    /// Library
    Library,
    /// Test suite
    Test,
    /// Example code
    Example,
    /// Benchmark
    Benchmark,
}

/// Target platform information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetInfo {
    /// Target architecture (x86_64, aarch64, etc.)
    pub architecture: String,
    /// Target operating system
    pub operating_system: String,
    /// Additional target features
    pub features: Vec<String>,
}

/// Compilation timing information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationTiming {
    /// Total compilation time
    pub total_time: Duration,
    /// Time spent on parsing
    pub parse_time: Duration,
    /// Time spent on type checking
    pub typecheck_time: Duration,
    /// Time spent on optimization
    pub optimization_time: Duration,
    /// Time spent on code generation
    pub codegen_time: Duration,
    /// Time spent on linking
    pub link_time: Duration,
}

impl CompilationUnit {
    /// Create a new compilation unit with the given name
    pub fn new(name: String) -> Self {
        Self {
            name,
            source_files: Vec::new(),
            dependencies: Vec::new(),
            estimated_size_bytes: 0,
            optimization_metadata: HashMap::new(),
            timing_info: None,
            unit_type: CompilationUnitType::Binary,
            target_info: None,
        }
    }
    
    /// Create a library compilation unit
    pub fn new_library(name: String) -> Self {
        Self {
            unit_type: CompilationUnitType::Library,
            ..Self::new(name)
        }
    }
    
    /// Create a test compilation unit
    pub fn new_test(name: String) -> Self {
        Self {
            unit_type: CompilationUnitType::Test,
            ..Self::new(name)
        }
    }
    
    /// Add a source file to this unit
    pub fn add_source_file(&mut self, file_path: String) {
        self.source_files.push(file_path);
    }
    
    /// Add a dependency on another unit
    pub fn add_dependency(&mut self, dependency: String) {
        self.dependencies.push(dependency);
    }
    
    /// Set optimization metadata
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.optimization_metadata.insert(key, value);
    }
    
    /// Get optimization metadata
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.optimization_metadata.get(key)
    }
    
    /// Set timing information
    pub fn set_timing(&mut self, timing: CompilationTiming) {
        self.timing_info = Some(timing);
    }
    
    /// Calculate total dependency count (recursive)
    pub fn total_dependency_count(&self, all_units: &[CompilationUnit]) -> usize {
        let mut visited = std::collections::HashSet::new();
        self.count_dependencies_recursive(all_units, &mut visited)
    }
    
    fn count_dependencies_recursive(
        &self,
        all_units: &[CompilationUnit],
        visited: &mut std::collections::HashSet<String>,
    ) -> usize {
        if visited.contains(&self.name) {
            return 0; // Avoid cycles
        }
        
        visited.insert(self.name.clone());
        let mut count = self.dependencies.len();
        
        for dep_name in &self.dependencies {
            if let Some(dep_unit) = all_units.iter().find(|u| u.name == *dep_name) {
                count += dep_unit.count_dependencies_recursive(all_units, visited);
            }
        }
        
        count
    }
    
    /// Check if this unit depends on another unit (directly or indirectly)
    pub fn depends_on(&self, target: &str, all_units: &[CompilationUnit]) -> bool {
        let mut visited = std::collections::HashSet::new();
        self.depends_on_recursive(target, all_units, &mut visited)
    }
    
    fn depends_on_recursive(
        &self,
        target: &str,
        all_units: &[CompilationUnit],
        visited: &mut std::collections::HashSet<String>,
    ) -> bool {
        if visited.contains(&self.name) {
            return false; // Avoid cycles
        }
        
        visited.insert(self.name.clone());
        
        // Direct dependency
        if self.dependencies.contains(&target.to_string()) {
            return true;
        }
        
        // Indirect dependency
        for dep_name in &self.dependencies {
            if let Some(dep_unit) = all_units.iter().find(|u| u.name == *dep_name) {
                if dep_unit.depends_on_recursive(target, all_units, visited) {
                    return true;
                }
            }
        }
        
        false
    }
    
    /// Estimate compilation complexity
    pub fn estimate_complexity(&self) -> CompilationComplexity {
        let source_file_count = self.source_files.len();
        let dependency_count = self.dependencies.len();
        let size_factor = self.estimated_size_bytes / 1000; // KB
        
        let complexity_score = source_file_count + dependency_count * 2 + size_factor;
        
        match complexity_score {
            0..=10 => CompilationComplexity::Low,
            11..=50 => CompilationComplexity::Medium,
            51..=200 => CompilationComplexity::High,
            _ => CompilationComplexity::VeryHigh,
        }
    }
    
    /// Get unit statistics
    pub fn get_statistics(&self) -> CompilationUnitStats {
        CompilationUnitStats {
            source_file_count: self.source_files.len(),
            dependency_count: self.dependencies.len(),
            estimated_size_bytes: self.estimated_size_bytes,
            complexity: self.estimate_complexity(),
            has_timing_info: self.timing_info.is_some(),
            metadata_count: self.optimization_metadata.len(),
        }
    }
}

/// Compilation complexity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompilationComplexity {
    Low,
    Medium,
    High,
    VeryHigh,
}

impl CompilationComplexity {
    /// Get the complexity as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            CompilationComplexity::Low => "Low",
            CompilationComplexity::Medium => "Medium",
            CompilationComplexity::High => "High",
            CompilationComplexity::VeryHigh => "Very High",
        }
    }
    
    /// Get estimated compilation time multiplier
    pub fn time_multiplier(&self) -> f64 {
        match self {
            CompilationComplexity::Low => 1.0,
            CompilationComplexity::Medium => 2.0,
            CompilationComplexity::High => 4.0,
            CompilationComplexity::VeryHigh => 8.0,
        }
    }
}

/// Statistics about a compilation unit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationUnitStats {
    pub source_file_count: usize,
    pub dependency_count: usize,
    pub estimated_size_bytes: usize,
    pub complexity: CompilationComplexity,
    pub has_timing_info: bool,
    pub metadata_count: usize,
}

impl Default for CompilationUnitType {
    fn default() -> Self {
        CompilationUnitType::Binary
    }
}

impl Default for TargetInfo {
    fn default() -> Self {
        Self {
            architecture: "x86_64".to_string(),
            operating_system: std::env::consts::OS.to_string(),
            features: Vec::new(),
        }
    }
}

impl Default for CompilationTiming {
    fn default() -> Self {
        Self {
            total_time: Duration::from_secs(0),
            parse_time: Duration::from_secs(0),
            typecheck_time: Duration::from_secs(0),
            optimization_time: Duration::from_secs(0),
            codegen_time: Duration::from_secs(0),
            link_time: Duration::from_secs(0),
        }
    }
}

/// Collection of compilation units for analysis
#[derive(Debug, Clone)]
pub struct CompilationUnitCollection {
    units: Vec<CompilationUnit>,
}

impl CompilationUnitCollection {
    /// Create new collection
    pub fn new() -> Self {
        Self {
            units: Vec::new(),
        }
    }
    
    /// Add a compilation unit
    pub fn add_unit(&mut self, unit: CompilationUnit) {
        self.units.push(unit);
    }
    
    /// Get all units
    pub fn units(&self) -> &[CompilationUnit] {
        &self.units
    }
    
    /// Get mutable units
    pub fn units_mut(&mut self) -> &mut [CompilationUnit] {
        &mut self.units
    }
    
    /// Find unit by name
    pub fn find_unit(&self, name: &str) -> Option<&CompilationUnit> {
        self.units.iter().find(|u| u.name == name)
    }
    
    /// Find unit by name (mutable)
    pub fn find_unit_mut(&mut self, name: &str) -> Option<&mut CompilationUnit> {
        self.units.iter_mut().find(|u| u.name == name)
    }
    
    /// Get collection statistics
    pub fn get_statistics(&self) -> CollectionStats {
        let total_units = self.units.len();
        let total_source_files: usize = self.units.iter().map(|u| u.source_files.len()).sum();
        let total_dependencies: usize = self.units.iter().map(|u| u.dependencies.len()).sum();
        let total_size: usize = self.units.iter().map(|u| u.estimated_size_bytes).sum();
        
        let complexity_counts = self.units.iter().fold(
            HashMap::new(),
            |mut acc, unit| {
                let complexity = unit.estimate_complexity();
                *acc.entry(complexity).or_insert(0) += 1;
                acc
            }
        );
        
        let units_with_timing = self.units.iter().filter(|u| u.timing_info.is_some()).count();
        
        CollectionStats {
            total_units,
            total_source_files,
            total_dependencies,
            total_estimated_size_bytes: total_size,
            complexity_distribution: complexity_counts,
            units_with_timing_info: units_with_timing,
            average_dependencies_per_unit: if total_units > 0 {
                total_dependencies as f64 / total_units as f64
            } else {
                0.0
            },
        }
    }
    
    /// Sort units by compilation order (dependencies first)
    pub fn sort_by_compilation_order(&mut self) {
        // Simple topological sort
        let mut sorted = Vec::new();
        let mut remaining: Vec<_> = self.units.drain(..).collect();
        
        while !remaining.is_empty() {
            let mut progress = false;
            
            remaining.retain(|unit| {
                // Check if all dependencies are already sorted
                let deps_satisfied = unit.dependencies.iter().all(|dep| {
                    sorted.iter().any(|sorted_unit: &CompilationUnit| sorted_unit.name == *dep)
                });
                
                if deps_satisfied {
                    sorted.push(unit.clone());
                    progress = true;
                    false // Remove from remaining
                } else {
                    true // Keep in remaining
                }
            });
            
            // If no progress, there might be circular dependencies
            if !progress && !remaining.is_empty() {
                // Add remaining units anyway to avoid infinite loop
                sorted.extend(remaining.drain(..));
                break;
            }
        }
        
        self.units = sorted;
    }
}

/// Statistics about a collection of compilation units
#[derive(Debug, Clone)]
pub struct CollectionStats {
    pub total_units: usize,
    pub total_source_files: usize,
    pub total_dependencies: usize,
    pub total_estimated_size_bytes: usize,
    pub complexity_distribution: HashMap<CompilationComplexity, usize>,
    pub units_with_timing_info: usize,
    pub average_dependencies_per_unit: f64,
}

impl Default for CompilationUnitCollection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_compilation_unit_creation() {
        let unit = CompilationUnit::new("test_unit".to_string());
        assert_eq!(unit.name, "test_unit");
        assert!(unit.source_files.is_empty());
        assert!(unit.dependencies.is_empty());
    }
    
    #[test]
    fn test_compilation_unit_dependencies() {
        let mut main_unit = CompilationUnit::new("main".to_string());
        let mut utils_unit = CompilationUnit::new("utils".to_string());
        let core_unit = CompilationUnit::new("core".to_string());
        
        main_unit.add_dependency("utils".to_string());
        utils_unit.add_dependency("core".to_string());
        
        let all_units = vec![main_unit.clone(), utils_unit, core_unit];
        
        assert!(main_unit.depends_on("utils", &all_units));
        assert!(main_unit.depends_on("core", &all_units));
        assert_eq!(main_unit.total_dependency_count(&all_units), 2);
    }
    
    #[test]
    fn test_complexity_estimation() {
        let mut unit = CompilationUnit::new("complex_unit".to_string());
        unit.add_source_file("file1.csd".to_string());
        unit.add_source_file("file2.csd".to_string());
        unit.add_dependency("dep1".to_string());
        unit.estimated_size_bytes = 50000; // 50KB
        
        let complexity = unit.estimate_complexity();
        assert!(matches!(complexity, CompilationComplexity::Medium | CompilationComplexity::High));
    }
    
    #[test]
    fn test_compilation_unit_collection() {
        let mut collection = CompilationUnitCollection::new();
        
        let unit1 = CompilationUnit::new("unit1".to_string());
        let unit2 = CompilationUnit::new("unit2".to_string());
        
        collection.add_unit(unit1);
        collection.add_unit(unit2);
        
        let stats = collection.get_statistics();
        assert_eq!(stats.total_units, 2);
    }
    
    #[test]
    fn test_collection_sorting() {
        let mut collection = CompilationUnitCollection::new();
        
        let mut main_unit = CompilationUnit::new("main".to_string());
        let utils_unit = CompilationUnit::new("utils".to_string());
        
        main_unit.add_dependency("utils".to_string());
        
        collection.add_unit(main_unit);
        collection.add_unit(utils_unit);
        
        collection.sort_by_compilation_order();
        
        // utils should come before main
        let units = collection.units();
        assert_eq!(units[0].name, "utils");
        assert_eq!(units[1].name, "main");
    }
}
