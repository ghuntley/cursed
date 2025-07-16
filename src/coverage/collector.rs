/// Coverage data collection for CURSED source files
/// 
/// Collects runtime coverage information during test execution
/// for both interpreted and compiled CURSED code.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::fs;
use std::io::{self, Write, BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use serde_json;

use super::{CoverageData, FileCoverage, LineCoverage, FunctionCoverage, BranchCoverage, CoverageSummary, CoverageConfig};
use crate::lexer::{Lexer, Token, Position};
use crate::parser::{Parser, AstNode};

/// Collects coverage data during program execution
pub struct CoverageCollector {
    config: CoverageConfig,
    active_files: HashSet<PathBuf>,
    line_hits: Arc<Mutex<HashMap<String, HashMap<u32, u64>>>>,
    function_hits: Arc<Mutex<HashMap<String, HashMap<String, u64>>>>,
    branch_hits: Arc<Mutex<HashMap<String, HashMap<String, (u64, u64)>>>>,
    test_run_id: Option<String>,
    start_time: Option<SystemTime>,
}

impl CoverageCollector {
    pub fn new(config: CoverageConfig) -> io::Result<Self> {
        // Create raw coverage output directory
        let raw_dir = config.output_dir.join("raw");
        fs::create_dir_all(&raw_dir)?;
        
        Ok(Self {
            config,
            active_files: HashSet::new(),
            line_hits: Arc::new(Mutex::new(HashMap::new())),
            function_hits: Arc::new(Mutex::new(HashMap::new())),
            branch_hits: Arc::new(Mutex::new(HashMap::new())),
            test_run_id: None,
            start_time: None,
        })
    }

    /// Start coverage collection for a test run
    pub async fn start_collection(&mut self, test_run_id: &str) -> io::Result<()> {
        self.test_run_id = Some(test_run_id.to_string());
        self.start_time = Some(SystemTime::now());
        
        // Discover source files to monitor
        self.discover_source_files()?;
        
        // Initialize coverage tracking for all files
        self.initialize_coverage_tracking()?;
        
        println!("📊 Coverage collection started for {} files", self.active_files.len());
        Ok(())
    }

    /// Stop coverage collection and compute final coverage data
    pub async fn stop_collection(&mut self) -> io::Result<CoverageData> {
        let test_run_id = self.test_run_id.clone().unwrap_or_else(|| "unknown".to_string());
        
        // Collect coverage data from all sources
        let mut coverage_data = self.compute_coverage_data(&test_run_id).await?;
        
        // Save raw coverage data
        self.save_raw_coverage_data(&coverage_data).await?;
        
        println!("📈 Coverage collection completed: {:.2}% line coverage", 
                 coverage_data.summary.line_coverage_percentage);
        
        Ok(coverage_data)
    }

    /// Discover all source files that should be included in coverage
    fn discover_source_files(&mut self) -> io::Result<()> {
        for source_dir in &self.config.source_dirs {
            self.discover_files_in_directory(source_dir)?;
        }
        Ok(())
    }

    fn discover_files_in_directory(&mut self, dir: &Path) -> io::Result<()> {
        if !dir.exists() {
            return Ok(());
        }
        
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                self.discover_files_in_directory(&path)?;
            } else if self.should_include_file(&path) {
                self.active_files.insert(path);
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

    /// Initialize coverage tracking structures for all active files
    fn initialize_coverage_tracking(&self) -> io::Result<()> {
        let mut line_hits = self.line_hits.lock().unwrap();
        let mut function_hits = self.function_hits.lock().unwrap();
        let mut branch_hits = self.branch_hits.lock().unwrap();
        
        for file_path in &self.active_files {
            let path_str = file_path.to_string_lossy().to_string();
            
            // Initialize line tracking
            line_hits.insert(path_str.clone(), HashMap::new());
            
            // Initialize function tracking
            if self.config.collect_function_coverage {
                function_hits.insert(path_str.clone(), HashMap::new());
                self.analyze_functions_in_file(file_path, &mut function_hits)?;
            }
            
            // Initialize branch tracking
            if self.config.collect_branch_coverage {
                branch_hits.insert(path_str.clone(), HashMap::new());
                self.analyze_branches_in_file(file_path, &mut branch_hits)?;
            }
        }
        
        Ok(())
    }

    /// Analyze functions in a CURSED source file
    fn analyze_functions_in_file(
        &self, 
        file_path: &Path, 
        function_hits: &mut HashMap<String, HashMap<String, u64>>
    ) -> io::Result<()> {
        let content = fs::read_to_string(file_path)?;
        let path_str = file_path.to_string_lossy().to_string();
        
        if file_path.extension().and_then(|s| s.to_str()) == Some("csd") {
            // Parse CURSED file to find functions
            let mut lexer = Lexer::new(&content);
            let tokens = lexer.tokenize().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            
            let mut parser = Parser::new(tokens);
            if let Ok(ast) = parser.parse() {
                self.extract_functions_from_ast(&ast, &path_str, function_hits);
            }
        } else if file_path.extension().and_then(|s| s.to_str()) == Some("rs") {
            // Parse Rust file to find functions
            self.extract_rust_functions(&content, &path_str, function_hits)?;
        }
        
        Ok(())
    }

    fn extract_functions_from_ast(
        &self,
        ast: &AstNode,
        file_path: &str,
        function_hits: &mut HashMap<String, HashMap<String, u64>>
    ) {
        match ast {
            AstNode::FunctionDeclaration { name, body, .. } => {
                if let Some(file_functions) = function_hits.get_mut(file_path) {
                    file_functions.insert(name.clone(), 0);
                }
            }
            _ => {
                // Recursively search for functions in child nodes
                for child in ast.children() {
                    self.extract_functions_from_ast(child, file_path, function_hits);
                }
            }
        }
    }

    fn extract_rust_functions(
        &self,
        content: &str,
        file_path: &str,
        function_hits: &mut HashMap<String, HashMap<String, u64>>
    ) -> io::Result<()> {
        // Simple regex-based function extraction for Rust
        use regex::Regex;
        
        let fn_regex = Regex::new(r"(?m)^\s*(?:pub\s+)?(?:async\s+)?fn\s+(\w+)").unwrap();
        
        if let Some(file_functions) = function_hits.get_mut(file_path) {
            for caps in fn_regex.captures_iter(content) {
                if let Some(fn_name) = caps.get(1) {
                    file_functions.insert(fn_name.as_str().to_string(), 0);
                }
            }
        }
        
        Ok(())
    }

    /// Analyze branches in a source file
    fn analyze_branches_in_file(
        &self,
        file_path: &Path,
        branch_hits: &mut HashMap<String, HashMap<String, (u64, u64)>>
    ) -> io::Result<()> {
        let content = fs::read_to_string(file_path)?;
        let path_str = file_path.to_string_lossy().to_string();
        
        // Find conditional statements and loops that create branches
        let branch_patterns = [
            r"lowkey\s+.*\{",  // if statements
            r"highkey\s+.*\{", // else if statements
            r"bestie\s+.*\{",  // for loops
            r"around\s+.*\{",  // while loops
            r"ready\s+\{",     // select statements
        ];
        
        if let Some(file_branches) = branch_hits.get_mut(&path_str) {
            for (line_num, line) in content.lines().enumerate() {
                for pattern in &branch_patterns {
                    if regex::Regex::new(pattern).unwrap().is_match(line) {
                        let branch_id = format!("{}:{}", line_num + 1, pattern);
                        file_branches.insert(branch_id, (0, 0));
                    }
                }
            }
        }
        
        Ok(())
    }

    /// Compute final coverage data from collected hits
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
        
        for file_path in &self.active_files {
            let path_str = file_path.to_string_lossy().to_string();
            let content = fs::read_to_string(file_path)?;
            
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
                    
                    let execution_count = line_hits
                        .get(&path_str)
                        .and_then(|file_hits| file_hits.get(&line_number))
                        .copied()
                        .unwrap_or(0);
                    
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
            if let Some(func_hits) = function_hits.get(&path_str) {
                for (func_name, hit_count) in func_hits {
                    total_functions += 1;
                    let is_covered = *hit_count > 0;
                    if is_covered {
                        covered_functions += 1;
                    }
                    
                    file_functions.insert(func_name.clone(), FunctionCoverage {
                        name: func_name.clone(),
                        start_line: 0, // TODO: Extract from AST
                        end_line: 0,   // TODO: Extract from AST
                        execution_count: *hit_count,
                        is_covered,
                        complexity: 1, // TODO: Calculate cyclomatic complexity
                    });
                }
            }
            
            // Compute branch coverage
            let mut file_branches = HashMap::new();
            if let Some(br_hits) = branch_hits.get(&path_str) {
                for (branch_id, (true_count, false_count)) in br_hits {
                    total_branches += 1;
                    let is_covered = *true_count > 0 && *false_count > 0;
                    if is_covered {
                        covered_branches += 1;
                    }
                    
                    file_branches.insert(branch_id.clone(), BranchCoverage {
                        line_number: 0, // TODO: Extract from branch_id
                        branch_id: branch_id.clone(),
                        condition: "unknown".to_string(), // TODO: Extract condition
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
            
            files.insert(path_str.clone(), FileCoverage {
                path: path_str,
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

    /// Save raw coverage data to disk
    async fn save_raw_coverage_data(&self, coverage_data: &CoverageData) -> io::Result<()> {
        let raw_dir = self.config.output_dir.join("raw");
        let raw_file = raw_dir.join(format!("{}.json", coverage_data.test_run_id));
        
        let json_data = serde_json::to_string_pretty(coverage_data)?;
        fs::write(raw_file, json_data)?;
        
        Ok(())
    }

    /// Record a line hit during execution (called by instrumented code)
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
    // Simple implementation - could be enhanced with proper glob matching
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
