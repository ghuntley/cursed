/// Simple Coverage Data Collection for CURSED
/// 
/// A streamlined coverage collector that works with the existing CURSED codebase
/// without requiring complex AST integration.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use serde_json;

use super::{CoverageData, FileCoverage, LineCoverage, FunctionCoverage, BranchCoverage, CoverageSummary, CoverageConfig};

/// Simple coverage collector that uses line-based analysis
pub struct SimpleCoverageCollector {
    config: CoverageConfig,
    line_hits: Arc<Mutex<HashMap<String, HashMap<u32, u64>>>>,
    function_hits: Arc<Mutex<HashMap<String, HashMap<String, u64>>>>,
    branch_hits: Arc<Mutex<HashMap<String, HashMap<String, (u64, u64)>>>>,
    test_run_id: Option<String>,
    start_time: Option<SystemTime>,
}

impl SimpleCoverageCollector {
    pub fn new(config: CoverageConfig) -> io::Result<Self> {
        let raw_dir = config.output_dir.join("raw");
        fs::create_dir_all(&raw_dir)?;
        
        Ok(Self {
            config,
            line_hits: Arc::new(Mutex::new(HashMap::new())),
            function_hits: Arc::new(Mutex::new(HashMap::new())),
            branch_hits: Arc::new(Mutex::new(HashMap::new())),
            test_run_id: None,
            start_time: None,
        })
    }

    pub async fn start_collection(&mut self, test_run_id: &str) -> io::Result<()> {
        self.test_run_id = Some(test_run_id.to_string());
        self.start_time = Some(SystemTime::now());
        
        self.discover_source_files().await?;
        self.initialize_coverage_tracking().await?;
        
        println!("📊 Coverage collection started for test run: {}", test_run_id);
        Ok(())
    }

    pub async fn stop_collection(&mut self) -> io::Result<CoverageData> {
        let test_run_id = self.test_run_id.clone().unwrap_or_else(|| "unknown".to_string());
        
        let coverage_data = self.compute_coverage_data(&test_run_id).await?;
        self.save_raw_coverage_data(&coverage_data).await?;
        
        println!("📈 Coverage collection completed: {:.2}% line coverage", 
                 coverage_data.summary.line_coverage_percentage);
        
        Ok(coverage_data)
    }

    async fn discover_source_files(&mut self) -> io::Result<()> {
        let source_dirs = self.config.source_dirs.clone();
        for source_dir in &source_dirs {
            self.discover_files_in_directory(source_dir).await?;
        }
        Ok(())
    }

    async fn discover_files_in_directory(&mut self, dir: &Path) -> io::Result<()> {
        if !dir.exists() {
            return Ok(());
        }
        
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                self.discover_files_in_directory(&path).await?;
            } else if self.should_include_file(&path) {
                self.initialize_file_coverage(&path).await?;
            }
        }
        
        Ok(())
    }

    fn should_include_file(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        
        // Check exclude patterns
        for pattern in &self.config.exclude_patterns {
            if glob_match(pattern, &path_str) {
                return false;
            }
        }
        
        // Check include patterns
        for pattern in &self.config.include_patterns {
            if glob_match(pattern, &path_str) {
                return true;
            }
        }
        
        false
    }

    async fn initialize_file_coverage(&self, file_path: &Path) -> io::Result<()> {
        let path_str = file_path.to_string_lossy().to_string();
        let content = fs::read_to_string(file_path)?;
        
        // Initialize line tracking
        let mut line_hits = self.line_hits.lock().unwrap();
        line_hits.insert(path_str.clone(), HashMap::new());
        
        // Initialize function tracking
        if self.config.collect_function_coverage {
            let mut function_hits = self.function_hits.lock().unwrap();
            function_hits.insert(path_str.clone(), HashMap::new());
            
            // Simple function detection for CURSED files
            if file_path.extension().and_then(|s| s.to_str()) == Some("csd") {
                self.extract_functions_simple(&content, &path_str, &mut function_hits);
            }
        }
        
        // Initialize branch tracking
        if self.config.collect_branch_coverage {
            let mut branch_hits = self.branch_hits.lock().unwrap();
            branch_hits.insert(path_str.clone(), HashMap::new());
            
            self.extract_branches_simple(&content, &path_str, &mut branch_hits);
        }
        
        Ok(())
    }

    async fn initialize_coverage_tracking(&self) -> io::Result<()> {
        // Already handled in discover_files_in_directory
        Ok(())
    }

    fn extract_functions_simple(&self, content: &str, file_path: &str, function_hits: &mut HashMap<String, HashMap<String, u64>>) {
        let file_functions = function_hits.get_mut(file_path).unwrap();
        
        for line in content.lines() {
            // Simple regex-based function detection
            if let Some(func_name) = self.extract_function_name_simple(line) {
                file_functions.insert(func_name, 0);
            }
        }
    }

    fn extract_function_name_simple(&self, line: &str) -> Option<String> {
        // Match function declarations: slay function_name(
        if let Some(slay_pos) = line.find("slay") {
            let after_slay = &line[slay_pos + 4..];
            if let Some(func_start) = after_slay.trim_start().find(|c: char| c.is_alphabetic()) {
                let func_part = &after_slay[func_start..];
                if let Some(func_end) = func_part.find('(') {
                    let func_name = func_part[..func_end].trim();
                    if !func_name.is_empty() {
                        return Some(func_name.to_string());
                    }
                }
            }
        }
        None
    }

    fn extract_branches_simple(&self, content: &str, file_path: &str, branch_hits: &mut HashMap<String, HashMap<String, (u64, u64)>>) {
        let file_branches = branch_hits.get_mut(file_path).unwrap();
        
        for (line_num, line) in content.lines().enumerate() {
            let line_number = line_num + 1;
            
            // Simple branch detection
            if line.contains("lowkey") {
                let branch_id = format!("{}:if", line_number);
                file_branches.insert(branch_id, (0, 0));
            }
            
            if line.contains("highkey") {
                let branch_id = format!("{}:else_if", line_number);
                file_branches.insert(branch_id, (0, 0));
            }
            
            if line.contains("around") {
                let branch_id = format!("{}:while", line_number);
                file_branches.insert(branch_id, (0, 0));
            }
            
            if line.contains("bestie") {
                let branch_id = format!("{}:for", line_number);
                file_branches.insert(branch_id, (0, 0));
            }
            
            if line.contains("ready") {
                let branch_id = format!("{}:select", line_number);
                file_branches.insert(branch_id, (0, 0));
            }
        }
    }

    async fn compute_coverage_data(&self, test_run_id: &str) -> io::Result<CoverageData> {
        let line_hits = self.line_hits.lock().unwrap();
        let function_hits = self.function_hits.lock().unwrap();
        let branch_hits = self.branch_hits.lock().unwrap();
        
        let mut files = HashMap::new();
        let mut total_lines = 0;
        let mut covered_lines = 0;
        let mut total_functions = 0;
        let mut covered_functions = 0;
        let mut total_branches = 0;
        let mut covered_branches = 0;
        
        // Get all files from line_hits (which contains all discovered files)
        for (file_path, file_line_hits) in line_hits.iter() {
            let content = match fs::read_to_string(file_path) {
                Ok(content) => content,
                Err(_) => continue,
            };
            
            // Compute line coverage
            let mut file_lines = HashMap::new();
            let mut file_covered_lines = 0;
            let mut file_total_lines = 0;
            
            for (line_num, line_content) in content.lines().enumerate() {
                let line_number = (line_num + 1) as u32;
                let is_executable = is_executable_line(line_content);
                
                if is_executable {
                    file_total_lines += 1;
                    total_lines += 1;
                    
                    let execution_count = file_line_hits.get(&line_number).copied().unwrap_or(0);
                    let is_covered = execution_count > 0;
                    
                    if is_covered {
                        file_covered_lines += 1;
                        covered_lines += 1;
                    }
                    
                    file_lines.insert(line_number, LineCoverage {
                        line_number,
                        execution_count,
                        is_executable,
                        is_covered,
                        source_line: line_content.to_string(),
                    });
                }
            }
            
            // Compute function coverage
            let mut file_functions = HashMap::new();
            if let Some(func_hits) = function_hits.get(file_path) {
                for (func_name, hit_count) in func_hits {
                    total_functions += 1;
                    let is_covered = *hit_count > 0;
                    if is_covered {
                        covered_functions += 1;
                    }
                    
                    file_functions.insert(func_name.clone(), FunctionCoverage {
                        name: func_name.clone(),
                        start_line: 0,
                        end_line: 0,
                        execution_count: *hit_count,
                        is_covered,
                        complexity: 1,
                    });
                }
            }
            
            // Compute branch coverage
            let mut file_branches = HashMap::new();
            if let Some(br_hits) = branch_hits.get(file_path) {
                for (branch_id, (true_count, false_count)) in br_hits {
                    total_branches += 1;
                    let is_covered = *true_count > 0 && *false_count > 0;
                    if is_covered {
                        covered_branches += 1;
                    }
                    
                    let line_number = branch_id.split(':').next()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(0);
                    
                    file_branches.insert(branch_id.clone(), BranchCoverage {
                        line_number,
                        branch_id: branch_id.clone(),
                        condition: "detected".to_string(),
                        true_count: *true_count,
                        false_count: *false_count,
                        is_covered,
                    });
                }
            }
            
            let coverage_percentage = if file_total_lines > 0 {
                (file_covered_lines as f64 / file_total_lines as f64) * 100.0
            } else {
                100.0
            };
            
            files.insert(file_path.clone(), FileCoverage {
                path: file_path.clone(),
                lines: file_lines,
                functions: file_functions,
                branches: file_branches,
                total_lines: file_total_lines,
                covered_lines: file_covered_lines,
                coverage_percentage,
            });
        }
        
        let summary = CoverageSummary {
            total_files: files.len() as u32,
            total_lines,
            covered_lines,
            line_coverage_percentage: if total_lines > 0 {
                (covered_lines as f64 / total_lines as f64) * 100.0
            } else {
                100.0
            },
            total_functions,
            covered_functions,
            function_coverage_percentage: if total_functions > 0 {
                (covered_functions as f64 / total_functions as f64) * 100.0
            } else {
                100.0
            },
            total_branches,
            covered_branches,
            branch_coverage_percentage: if total_branches > 0 {
                (covered_branches as f64 / total_branches as f64) * 100.0
            } else {
                100.0
            },
        };
        
        Ok(CoverageData {
            files,
            summary,
            timestamp: chrono::Utc::now().to_rfc3339(),
            test_run_id: test_run_id.to_string(),
        })
    }

    async fn save_raw_coverage_data(&self, coverage_data: &CoverageData) -> io::Result<()> {
        let raw_dir = self.config.output_dir.join("raw");
        let raw_file = raw_dir.join(format!("{}.json", coverage_data.test_run_id));
        
        let json_data = serde_json::to_string_pretty(coverage_data)?;
        fs::write(raw_file, json_data)?;
        
        Ok(())
    }

    /// Record a line hit during execution
    pub fn record_line_hit(&self, file_path: &str, line_number: u32) {
        if let Ok(mut line_hits) = self.line_hits.lock() {
            let file_hits = line_hits.entry(file_path.to_string()).or_insert_with(HashMap::new);
            *file_hits.entry(line_number).or_insert(0) += 1;
        }
    }

    /// Record a function hit during execution
    pub fn record_function_hit(&self, file_path: &str, function_name: &str) {
        if let Ok(mut function_hits) = self.function_hits.lock() {
            let file_hits = function_hits.entry(file_path.to_string()).or_insert_with(HashMap::new);
            *file_hits.entry(function_name.to_string()).or_insert(0) += 1;
        }
    }

    /// Record a branch hit during execution
    pub fn record_branch_hit(&self, file_path: &str, branch_id: &str, condition_result: bool) {
        if let Ok(mut branch_hits) = self.branch_hits.lock() {
            let file_hits = branch_hits.entry(file_path.to_string()).or_insert_with(HashMap::new);
            let (true_count, false_count) = file_hits.entry(branch_id.to_string()).or_insert((0, 0));
            
            if condition_result {
                *true_count += 1;
            } else {
                *false_count += 1;
            }
        }
    }
}

/// Check if a line of code is executable (not a comment or blank line)
fn is_executable_line(line: &str) -> bool {
    let trimmed = line.trim();
    
    // Skip empty lines
    if trimmed.is_empty() {
        return false;
    }
    
    // Skip comment lines
    if trimmed.starts_with("//") || trimmed.starts_with("fr fr") || trimmed.starts_with("no cap") {
        return false;
    }
    
    // Skip lines that are only braces
    if matches!(trimmed, "{" | "}") {
        return false;
    }
    
    // Skip import/package declarations
    if trimmed.starts_with("yeet") || trimmed.starts_with("vibe") || trimmed.starts_with("fam") {
        return false;
    }
    
    true
}

/// Simple glob pattern matching
fn glob_match(pattern: &str, text: &str) -> bool {
    if pattern.contains('*') {
        let parts: Vec<&str> = pattern.split('*').collect();
        if parts.len() == 2 {
            text.starts_with(parts[0]) && text.ends_with(parts[1])
        } else {
            text.contains(&pattern.replace('*', ""))
        }
    } else {
        text == pattern
    }
}
