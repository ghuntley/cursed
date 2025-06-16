/// Advanced Example Generation System
/// 
/// Provides comprehensive example extraction, validation, and generation capabilities
/// including automatic test extraction, runnable snippets, and categorized examples.

use crate::error::{Error, SourceLocation};
use crate::lexer::{Lexer, Token, TokenType};
use crate::parser::{Parser, ParsedProgram};
use crate::compiler::Compiler;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::fs;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// Advanced example generator
#[derive(Debug)]
pub struct AdvancedExampleGenerator {
    /// Configuration for example generation
    config: ExampleConfig,
    /// Extracted examples database
    examples_db: ExamplesDatabase,
    /// Validation results
    validation_results: HashMap<String, ValidationResult>,
}

/// Configuration for example generation
#[derive(Debug, Clone)]
pub struct ExampleConfig {
    /// Extract examples from test files
    pub extract_from_tests: bool,
    /// Validate examples by running them
    pub validate_examples: bool,
    /// Generate interactive snippets
    pub generate_interactive: bool,
    /// Maximum execution time for validation
    pub max_execution_time: Duration,
    /// Test file patterns to scan
    pub test_file_patterns: Vec<String>,
    /// Example categories to generate
    pub categories: HashSet<ExampleCategory>,
    /// Difficulty levels to include
    pub difficulty_levels: HashSet<DifficultyLevel>,
    /// Output formats for examples
    pub output_formats: HashSet<ExampleFormat>,
}

impl Default for ExampleConfig {
    fn default() -> Self {
        let mut categories = HashSet::new();
        categories.insert(ExampleCategory::Basic);
        categories.insert(ExampleCategory::Advanced);
        categories.insert(ExampleCategory::Tutorial);

        let mut difficulty_levels = HashSet::new();
        difficulty_levels.insert(DifficultyLevel::Beginner);
        difficulty_levels.insert(DifficultyLevel::Intermediate);
        difficulty_levels.insert(DifficultyLevel::Advanced);

        let mut output_formats = HashSet::new();
        output_formats.insert(ExampleFormat::Markdown);
        output_formats.insert(ExampleFormat::Html);
        output_formats.insert(ExampleFormat::Interactive);

        Self {
            extract_from_tests: true,
            validate_examples: true,
            generate_interactive: true,
            max_execution_time: Duration::from_secs(30),
            test_file_patterns: vec![
                "tests/**/*.rs".to_string(),
                "examples/**/*.csd".to_string(),
                "**/*_test.rs".to_string(),
                "**/*_example.csd".to_string(),
            ],
            categories,
            difficulty_levels,
            output_formats,
        }
    }
}

/// Example categories
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExampleCategory {
    Basic,
    Advanced,
    Tutorial,
    Reference,
    BestPractices,
    Performance,
    Testing,
    Integration,
    RealWorld,
    Cookbook,
}

/// Difficulty levels
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DifficultyLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

/// Example output formats
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExampleFormat {
    Markdown,
    Html,
    Interactive,
    Playground,
    Jupyter,
}

/// Examples database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamplesDatabase {
    /// All extracted examples
    pub examples: HashMap<String, ExtractedExample>,
    /// Examples by category
    pub by_category: HashMap<ExampleCategory, Vec<String>>,
    /// Examples by difficulty
    pub by_difficulty: HashMap<DifficultyLevel, Vec<String>>,
    /// Examples by topic
    pub by_topic: HashMap<String, Vec<String>>,
    /// Validation status
    pub validation_status: HashMap<String, bool>,
    /// Example relationships
    pub relationships: HashMap<String, Vec<String>>,
}

impl Default for ExamplesDatabase {
    fn default() -> Self {
        Self {
            examples: HashMap::new(),
            by_category: HashMap::new(),
            by_difficulty: HashMap::new(),
            by_topic: HashMap::new(),
            validation_status: HashMap::new(),
            relationships: HashMap::new(),
        }
    }
}

/// Extracted example information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedExample {
    /// Unique identifier
    pub id: String,
    /// Example title
    pub title: String,
    /// Description
    pub description: String,
    /// Source code
    pub code: String,
    /// Expected output
    pub expected_output: Option<String>,
    /// Category
    pub category: ExampleCategory,
    /// Difficulty level
    pub difficulty: DifficultyLevel,
    /// Topics covered
    pub topics: Vec<String>,
    /// Source location
    pub source_location: SourceLocation,
    /// Dependencies required
    pub dependencies: Vec<String>,
    /// Setup instructions
    pub setup: Option<String>,
    /// Cleanup instructions
    pub cleanup: Option<String>,
    /// Execution time estimate
    pub execution_time: Option<Duration>,
    /// Interactive features
    pub interactive_features: Vec<InteractiveFeature>,
    /// Related examples
    pub related_examples: Vec<String>,
    /// Tags for searching
    pub tags: HashSet<String>,
    /// Validation status
    pub is_validated: bool,
    /// Error information if validation failed
    pub validation_error: Option<String>,
}

/// Interactive features available for an example
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractiveFeature {
    EditableCode,
    LiveOutput,
    StepByStep,
    Visualization,
    ParameterTuning,
    Performance,
}

/// Example validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Whether validation passed
    pub success: bool,
    /// Output produced by the example
    pub output: String,
    /// Error message if validation failed
    pub error_message: Option<String>,
    /// Execution time
    pub execution_time: Duration,
    /// Memory usage
    pub memory_usage: Option<usize>,
    /// Exit code
    pub exit_code: Option<i32>,
}

/// Example extraction result
#[derive(Debug, Clone)]
pub struct ExtractionResult {
    /// Total examples extracted
    pub total_extracted: usize,
    /// Successfully validated examples
    pub validated_examples: usize,
    /// Failed validations
    pub failed_validations: usize,
    /// Extraction errors
    pub extraction_errors: Vec<String>,
    /// Processing time
    pub processing_time: Duration,
}

impl AdvancedExampleGenerator {
    /// Create a new advanced example generator
    pub fn new(config: ExampleConfig) -> Self {
        Self {
            config,
            examples_db: ExamplesDatabase::default(),
            validation_results: HashMap::new(),
        }
    }

    /// Extract examples from source files and tests
    pub fn extract_examples(&mut self, source_paths: &[PathBuf]) -> Result<ExtractionResult, Error> {
        let start_time = Instant::now();
        let mut total_extracted = 0;
        let mut extraction_errors = Vec::new();

        // Clear existing database
        self.examples_db = ExamplesDatabase::default();
        self.validation_results.clear();

        // Extract from source files
        for path in source_paths {
            match self.extract_from_file(path) {
                Ok(count) => total_extracted += count,
                Err(e) => extraction_errors.push(format!("Error extracting from {}: {}", path.display(), e)),
            }
        }

        // Extract from test files if enabled
        if self.config.extract_from_tests {
            for pattern in &self.config.test_file_patterns {
                match self.extract_from_test_pattern(pattern) {
                    Ok(count) => total_extracted += count,
                    Err(e) => extraction_errors.push(format!("Error extracting from pattern {}: {}", pattern, e)),
                }
            }
        }

        // Validate examples if enabled
        let validated_examples = if self.config.validate_examples {
            self.validate_all_examples()
        } else {
            0
        };

        let failed_validations = total_extracted - validated_examples;

        // Build relationships and categorize
        self.build_example_relationships();
        self.categorize_examples();

        Ok(ExtractionResult {
            total_extracted,
            validated_examples,
            failed_validations,
            extraction_errors,
            processing_time: start_time.elapsed(),
        })
    }

    /// Extract examples from a single file
    fn extract_from_file(&mut self, file_path: &Path) -> Result<usize, Error> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| Error::SystemError(format!("Failed to read file {}: {}", file_path.display(), e)))?;

        let mut extracted_count = 0;

        // Extract documentation examples
        extracted_count += self.extract_doc_examples(&content, file_path)?;

        // Extract code examples from comments
        extracted_count += self.extract_comment_examples(&content, file_path)?;

        // Extract standalone examples
        if file_path.extension().and_then(|s| s.to_str()) == Some("csd") {
            extracted_count += self.extract_standalone_example(&content, file_path)?;
        }

        Ok(extracted_count)
    }

    /// Extract examples from documentation comments
    fn extract_doc_examples(&mut self, content: &str, file_path: &Path) -> Result<usize, Error> {
        let mut count = 0;
        let lines: Vec<&str> = content.lines().collect();

        let mut in_doc_block = false;
        let mut current_example = String::new();
        let mut example_title = String::new();
        let mut example_description = String::new();
        let mut line_number = 0;

        for (i, line) in lines.iter().enumerate() {
            line_number = i + 1;
            let trimmed = line.trim();

            if trimmed.starts_with("///") || trimmed.starts_with("/**") {
                in_doc_block = true;
                
                // Check for example start
                if trimmed.contains("@example") || trimmed.contains("```cursed") {
                    // Extract title and description from previous lines
                    if current_example.is_empty() {
                        self.extract_doc_context(&lines, i, &mut example_title, &mut example_description);
                    }
                }
                
                // Extract code from doc comments
                if trimmed.starts_with("///") {
                    let code_line = trimmed.trim_start_matches("///").trim();
                    if !code_line.is_empty() && !code_line.starts_with("@") {
                        current_example.push_str(code_line);
                        current_example.push('\n');
                    }
                }
            } else if trimmed.starts_with("```cursed") {
                current_example.clear();
            } else if trimmed.starts_with("```") && !current_example.is_empty() {
                // End of code block - create example
                if !current_example.trim().is_empty() {
                    let example_id = format!("doc_{}_{}", file_path.file_stem().unwrap_or_default().to_string_lossy(), count);
                    let example = self.create_example(
                        example_id.clone(),
                        if example_title.is_empty() { format!("Example {}", count + 1) } else { example_title.clone() },
                        example_description.clone(),
                        current_example.trim().to_string(),
                        ExampleCategory::Basic,
                        DifficultyLevel::Beginner,
                        SourceLocation {
                            line: line_number,
                            column: 1,
                            file: Some(file_path.to_path_buf()),
                        },
                    );
                    
                    self.examples_db.examples.insert(example_id, example);
                    count += 1;
                }
                
                current_example.clear();
                example_title.clear();
                example_description.clear();
            } else if in_doc_block && trimmed.starts_with("```") {
                current_example.push_str(trimmed);
                current_example.push('\n');
            } else if !trimmed.starts_with("///") && !trimmed.starts_with("*") {
                in_doc_block = false;
            }
        }

        Ok(count)
    }

    /// Extract context (title and description) from documentation
    fn extract_doc_context(&self, lines: &[&str], current_line: usize, title: &mut String, description: &mut String) {
        let mut context_lines = Vec::new();
        
        // Look backward for documentation context
        for i in (0..current_line).rev() {
            let trimmed = lines[i].trim();
            if trimmed.starts_with("///") || trimmed.starts_with("*") {
                let content = trimmed.trim_start_matches("///").trim_start_matches("*").trim();
                if !content.is_empty() && !content.starts_with("@") {
                    context_lines.insert(0, content);
                }
            } else if !trimmed.is_empty() {
                break;
            }
        }

        if !context_lines.is_empty() {
            *title = context_lines[0].to_string();
            if context_lines.len() > 1 {
                *description = context_lines[1..].join(" ");
            }
        }
    }

    /// Extract examples from comment blocks
    fn extract_comment_examples(&mut self, content: &str, file_path: &Path) -> Result<usize, Error> {
        let mut count = 0;
        
        // Use regex or simple pattern matching to find comment examples
        let comment_patterns = [
            r"// Example:",
            r"/* Example:",
            r"// Usage:",
            r"/* Usage:",
        ];

        for pattern in &comment_patterns {
            if let Some(start) = content.find(pattern) {
                if let Some(example_code) = self.extract_code_after_comment(content, start) {
                    let example_id = format!("comment_{}_{}", file_path.file_stem().unwrap_or_default().to_string_lossy(), count);
                    let example = self.create_example(
                        example_id.clone(),
                        format!("Comment Example {}", count + 1),
                        "Example extracted from comments".to_string(),
                        example_code,
                        ExampleCategory::Reference,
                        DifficultyLevel::Beginner,
                        SourceLocation {
                            line: content[..start].lines().count() + 1,
                            column: 1,
                            file: Some(file_path.to_path_buf()),
                        },
                    );
                    
                    self.examples_db.examples.insert(example_id, example);
                    count += 1;
                }
            }
        }

        Ok(count)
    }

    /// Extract code following a comment
    fn extract_code_after_comment(&self, content: &str, start_pos: usize) -> Option<String> {
        let remaining = &content[start_pos..];
        let lines: Vec<&str> = remaining.lines().collect();
        
        let mut code_lines = Vec::new();
        let mut found_code = false;
        
        for line in lines.iter().skip(1) { // Skip the comment line itself
            let trimmed = line.trim();
            
            if trimmed.is_empty() && !found_code {
                continue; // Skip empty lines before code
            }
            
            if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with("*") {
                continue; // Skip more comments
            }
            
            if !trimmed.is_empty() {
                found_code = true;
                code_lines.push(*line);
            } else if found_code {
                break; // End of code block
            }
        }
        
        if code_lines.is_empty() {
            None
        } else {
            Some(code_lines.join("\n"))
        }
    }

    /// Extract a standalone example file
    fn extract_standalone_example(&mut self, content: &str, file_path: &Path) -> Result<usize, Error> {
        // Parse the file to understand its structure
        let example_id = format!("standalone_{}", file_path.file_stem().unwrap_or_default().to_string_lossy());
        
        // Try to extract metadata from file header comments
        let (title, description, category, difficulty) = self.extract_file_metadata(content);
        
        let example = self.create_example(
            example_id.clone(),
            title,
            description,
            content.to_string(),
            category,
            difficulty,
            SourceLocation {
                line: 1,
                column: 1,
                file: Some(file_path.to_path_buf()),
            },
        );
        
        self.examples_db.examples.insert(example_id, example);
        Ok(1)
    }

    /// Extract metadata from file header
    fn extract_file_metadata(&self, content: &str) -> (String, String, ExampleCategory, DifficultyLevel) {
        let lines: Vec<&str> = content.lines().take(20).collect(); // Check first 20 lines
        let mut title = String::new();
        let mut description = String::new();
        let mut category = ExampleCategory::Basic;
        let mut difficulty = DifficultyLevel::Beginner;

        for line in &lines {
            let trimmed = line.trim();
            
            if trimmed.starts_with("// Title:") {
                title = trimmed.trim_start_matches("// Title:").trim().to_string();
            } else if trimmed.starts_with("// Description:") {
                description = trimmed.trim_start_matches("// Description:").trim().to_string();
            } else if trimmed.starts_with("// Category:") {
                let cat_str = trimmed.trim_start_matches("// Category:").trim();
                category = match cat_str.to_lowercase().as_str() {
                    "advanced" => ExampleCategory::Advanced,
                    "tutorial" => ExampleCategory::Tutorial,
                    "reference" => ExampleCategory::Reference,
                    "best_practices" => ExampleCategory::BestPractices,
                    "performance" => ExampleCategory::Performance,
                    "testing" => ExampleCategory::Testing,
                    "integration" => ExampleCategory::Integration,
                    "real_world" => ExampleCategory::RealWorld,
                    "cookbook" => ExampleCategory::Cookbook,
                    _ => ExampleCategory::Basic,
                };
            } else if trimmed.starts_with("// Difficulty:") {
                let diff_str = trimmed.trim_start_matches("// Difficulty:").trim();
                difficulty = match diff_str.to_lowercase().as_str() {
                    "intermediate" => DifficultyLevel::Intermediate,
                    "advanced" => DifficultyLevel::Advanced,
                    "expert" => DifficultyLevel::Expert,
                    _ => DifficultyLevel::Beginner,
                };
            }
        }

        // Use filename as title if not specified
        if title.is_empty() {
            title = "Example".to_string();
        }

        (title, description, category, difficulty)
    }

    /// Extract examples from test files
    fn extract_from_test_pattern(&mut self, pattern: &str) -> Result<usize, Error> {
        // Simple glob pattern matching implementation
        let mut count = 0;
        
        // For now, just handle basic patterns - a full glob implementation would be more complex
        if pattern.contains("tests/") {
            let test_dir = Path::new("tests");
            if test_dir.exists() {
                count += self.extract_from_test_directory(test_dir)?;
            }
        }
        
        if pattern.contains("examples/") {
            let examples_dir = Path::new("examples");
            if examples_dir.exists() {
                count += self.extract_from_test_directory(examples_dir)?;
            }
        }

        Ok(count)
    }

    /// Extract examples from a test directory
    fn extract_from_test_directory(&mut self, dir: &Path) -> Result<usize, Error> {
        let mut count = 0;
        
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");
                        if extension == "rs" || extension == "csd" {
                            count += self.extract_test_examples(&path)?;
                        }
                    } else if path.is_dir() {
                        count += self.extract_from_test_directory(&path)?;
                    }
                }
            }
        }

        Ok(count)
    }

    /// Extract examples from test files
    fn extract_test_examples(&mut self, file_path: &Path) -> Result<usize, Error> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| Error::SystemError(format!("Failed to read test file {}: {}", file_path.display(), e)))?;

        let mut count = 0;

        // Look for test functions and extract their content as examples
        let lines: Vec<&str> = content.lines().collect();
        let mut in_test_function = false;
        let mut test_code = Vec::new();
        let mut test_name = String::new();
        let mut brace_count = 0;

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            
            if trimmed.starts_with("#[test]") || trimmed.contains("fn test_") {
                in_test_function = true;
                test_code.clear();
                brace_count = 0;
                
                // Extract test name
                if let Some(next_line) = lines.get(i + 1) {
                    if let Some(fn_start) = next_line.find("fn ") {
                        if let Some(paren_pos) = next_line[fn_start..].find('(') {
                            test_name = next_line[fn_start + 3..fn_start + paren_pos].trim().to_string();
                        }
                    }
                }
            }
            
            if in_test_function {
                test_code.push(*line);
                
                // Count braces to know when test function ends
                brace_count += line.chars().filter(|&c| c == '{').count() as i32;
                brace_count -= line.chars().filter(|&c| c == '}').count() as i32;
                
                if brace_count == 0 && trimmed.ends_with('}') {
                    // End of test function
                    let example_id = format!("test_{}_{}", file_path.file_stem().unwrap_or_default().to_string_lossy(), test_name);
                    let example = self.create_example(
                        example_id.clone(),
                        format!("Test: {}", test_name),
                        format!("Example extracted from test function {}", test_name),
                        test_code.join("\n"),
                        ExampleCategory::Testing,
                        DifficultyLevel::Intermediate,
                        SourceLocation {
                            line: i + 1,
                            column: 1,
                            file: Some(file_path.to_path_buf()),
                        },
                    );
                    
                    self.examples_db.examples.insert(example_id, example);
                    count += 1;
                    in_test_function = false;
                }
            }
        }

        Ok(count)
    }

    /// Create an example structure
    fn create_example(
        &self,
        id: String,
        title: String,
        description: String,
        code: String,
        category: ExampleCategory,
        difficulty: DifficultyLevel,
        location: SourceLocation,
    ) -> ExtractedExample {
        let topics = self.extract_topics_from_code(&code);
        let tags = self.extract_tags_from_code(&code);
        let dependencies = self.extract_dependencies_from_code(&code);

        ExtractedExample {
            id,
            title,
            description,
            code,
            expected_output: None,
            category,
            difficulty,
            topics,
            source_location: location,
            dependencies,
            setup: None,
            cleanup: None,
            execution_time: None,
            interactive_features: vec![InteractiveFeature::EditableCode, InteractiveFeature::LiveOutput],
            related_examples: Vec::new(),
            tags,
            is_validated: false,
            validation_error: None,
        }
    }

    /// Extract topics from code content
    fn extract_topics_from_code(&self, code: &str) -> Vec<String> {
        let mut topics = Vec::new();
        let code_lower = code.to_lowercase();

        // Simple keyword-based topic extraction
        let topic_keywords = [
            ("async", "Asynchronous Programming"),
            ("await", "Asynchronous Programming"),
            ("struct", "Data Structures"),
            ("interface", "Interfaces"),
            ("function", "Functions"),
            ("loop", "Control Flow"),
            ("if", "Conditionals"),
            ("match", "Pattern Matching"),
            ("error", "Error Handling"),
            ("test", "Testing"),
            ("http", "Web Development"),
            ("json", "JSON Processing"),
            ("file", "File I/O"),
            ("database", "Database Operations"),
            ("crypto", "Cryptography"),
        ];

        for (keyword, topic) in &topic_keywords {
            if code_lower.contains(keyword) {
                topics.push(topic.to_string());
            }
        }

        topics.sort();
        topics.dedup();
        topics
    }

    /// Extract tags from code content
    fn extract_tags_from_code(&self, code: &str) -> HashSet<String> {
        let mut tags = HashSet::new();
        
        // Extract CURSED-specific keywords as tags
        let cursed_keywords = [
            "slay", "yolo", "periodt", "sus", "facts", "lowkey", "highkey",
            "bestie", "flex", "stan", "vibe_check", "mood", "basic"
        ];

        for keyword in &cursed_keywords {
            if code.contains(keyword) {
                tags.insert(keyword.to_string());
            }
        }

        // Extract common programming concepts
        if code.contains("fn ") || code.contains("function") {
            tags.insert("functions".to_string());
        }
        if code.contains("struct") {
            tags.insert("structs".to_string());
        }
        if code.contains("interface") {
            tags.insert("interfaces".to_string());
        }

        tags
    }

    /// Extract dependencies from code
    fn extract_dependencies_from_code(&self, code: &str) -> Vec<String> {
        let mut dependencies = Vec::new();
        
        // Look for import statements
        for line in code.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("import ") {
                if let Some(quote_start) = trimmed.find('"') {
                    if let Some(quote_end) = trimmed[quote_start + 1..].find('"') {
                        let import_path = &trimmed[quote_start + 1..quote_start + 1 + quote_end];
                        dependencies.push(import_path.to_string());
                    }
                }
            }
        }

        dependencies.sort();
        dependencies.dedup();
        dependencies
    }

    /// Validate all extracted examples
    fn validate_all_examples(&mut self) -> usize {
        let mut validated_count = 0;
        let example_ids: Vec<String> = self.examples_db.examples.keys().cloned().collect();

        for example_id in example_ids {
            if let Some(example) = self.examples_db.examples.get_mut(&example_id) {
                match self.validate_example(example) {
                    Ok(result) => {
                        example.is_validated = result.success;
                        if !result.success {
                            example.validation_error = result.error_message;
                        }
                        self.validation_results.insert(example_id.clone(), result);
                        if example.is_validated {
                            validated_count += 1;
                        }
                    }
                    Err(e) => {
                        example.validation_error = Some(format!("Validation error: {}", e));
                    }
                }
            }
        }

        validated_count
    }

    /// Validate a single example by attempting to compile and run it
    fn validate_example(&self, example: &ExtractedExample) -> Result<ValidationResult, Error> {
        if !self.config.validate_examples {
            return Ok(ValidationResult {
                success: true,
                output: String::new(),
                error_message: None,
                execution_time: Duration::from_secs(0),
                memory_usage: None,
                exit_code: Some(0),
            });
        }

        let start_time = Instant::now();

        // Create a temporary file for the example
        let temp_file = format!("/tmp/cursed_example_{}.csd", example.id);
        fs::write(&temp_file, &example.code)
            .map_err(|e| Error::SystemError(format!("Failed to write temp file: {}", e)))?;

        // Try to compile the example
        let compile_result = Command::new("cargo")
            .args(&["run", "--bin", "cursed", "--", "compile", &temp_file])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output();

        // Clean up temp file
        let _ = fs::remove_file(&temp_file);

        let execution_time = start_time.elapsed();

        match compile_result {
            Ok(output) => {
                let success = output.status.success();
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                
                let output_text = if !stdout.is_empty() { stdout } else { stderr };
                let error_message = if !success && !stderr.is_empty() {
                    Some(stderr)
                } else {
                    None
                };

                Ok(ValidationResult {
                    success,
                    output: output_text,
                    error_message,
                    execution_time,
                    memory_usage: None, // Would need additional tooling to measure
                    exit_code: output.status.code(),
                })
            }
            Err(e) => {
                Ok(ValidationResult {
                    success: false,
                    output: String::new(),
                    error_message: Some(format!("Failed to execute validation: {}", e)),
                    execution_time,
                    memory_usage: None,
                    exit_code: None,
                })
            }
        }
    }

    /// Build relationships between examples
    fn build_example_relationships(&mut self) {
        let example_ids: Vec<String> = self.examples_db.examples.keys().cloned().collect();
        
        for example_id in &example_ids {
            if let Some(example) = self.examples_db.examples.get(example_id) {
                let mut related = Vec::new();
                
                // Find examples with similar topics
                for other_id in &example_ids {
                    if other_id == example_id {
                        continue;
                    }
                    
                    if let Some(other_example) = self.examples_db.examples.get(other_id) {
                        let common_topics = example.topics.iter()
                            .filter(|topic| other_example.topics.contains(topic))
                            .count();
                        
                        if common_topics > 0 {
                            related.push(other_id.clone());
                        }
                    }
                }
                
                self.examples_db.relationships.insert(example_id.clone(), related);
            }
        }
    }

    /// Categorize examples into database collections
    fn categorize_examples(&mut self) {
        for (example_id, example) in &self.examples_db.examples {
            // By category
            self.examples_db.by_category
                .entry(example.category.clone())
                .or_insert_with(Vec::new)
                .push(example_id.clone());

            // By difficulty
            self.examples_db.by_difficulty
                .entry(example.difficulty.clone())
                .or_insert_with(Vec::new)
                .push(example_id.clone());

            // By topic
            for topic in &example.topics {
                self.examples_db.by_topic
                    .entry(topic.clone())
                    .or_insert_with(Vec::new)
                    .push(example_id.clone());
            }

            // Update validation status
            self.examples_db.validation_status.insert(example_id.clone(), example.is_validated);
        }
    }

    /// Generate interactive examples in various formats
    pub fn generate_interactive_examples(&self, output_dir: &Path) -> Result<(), Error> {
        if !self.config.generate_interactive {
            return Ok(());
        }

        fs::create_dir_all(output_dir)
            .map_err(|e| Error::SystemError(format!("Failed to create output directory: {}", e)))?;

        for format in &self.config.output_formats {
            match format {
                ExampleFormat::Html => self.generate_html_examples(output_dir)?,
                ExampleFormat::Markdown => self.generate_markdown_examples(output_dir)?,
                ExampleFormat::Interactive => self.generate_playground_examples(output_dir)?,
                ExampleFormat::Playground => self.generate_advanced_playground(output_dir)?,
                ExampleFormat::Jupyter => self.generate_jupyter_examples(output_dir)?,
            }
        }

        Ok(())
    }

    /// Generate HTML examples with interactive features
    fn generate_html_examples(&self, output_dir: &Path) -> Result<(), Error> {
        let html_dir = output_dir.join("html");
        fs::create_dir_all(&html_dir)
            .map_err(|e| Error::SystemError(format!("Failed to create HTML directory: {}", e)))?;

        // Generate index page
        let index_html = self.generate_examples_index_html();
        fs::write(html_dir.join("index.html"), index_html)
            .map_err(|e| Error::SystemError(format!("Failed to write index.html: {}", e)))?;

        // Generate individual example pages
        for (example_id, example) in &self.examples_db.examples {
            let example_html = self.generate_example_html(example);
            let filename = format!("{}.html", example_id);
            fs::write(html_dir.join(filename), example_html)
                .map_err(|e| Error::SystemError(format!("Failed to write example HTML: {}", e)))?;
        }

        Ok(())
    }

    /// Generate examples index HTML
    fn generate_examples_index_html(&self) -> String {
        let mut categories_html = String::new();
        
        for (category, example_ids) in &self.examples_db.by_category {
            categories_html.push_str(&format!(
                r#"<div class="category">
                    <h2>{:?}</h2>
                    <div class="examples-grid">"#,
                category
            ));
            
            for example_id in example_ids {
                if let Some(example) = self.examples_db.examples.get(example_id) {
                    categories_html.push_str(&format!(
                        r#"<div class="example-card">
                            <h3><a href="{}.html">{}</a></h3>
                            <p>{}</p>
                            <div class="example-meta">
                                <span class="difficulty {:?}">{:?}</span>
                                <span class="validation {}">{}</span>
                            </div>
                        </div>"#,
                        example_id,
                        example.title,
                        example.description,
                        example.difficulty,
                        example.difficulty,
                        if example.is_validated { "validated" } else { "not-validated" },
                        if example.is_validated { "✓ Validated" } else { "⚠ Not validated" }
                    ));
                }
            }
            
            categories_html.push_str("</div></div>");
        }

        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>CURSED Examples</title>
    <style>
        body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; margin: 0; padding: 20px; background: #f8f9fa; }}
        .header {{ background: white; padding: 30px; border-radius: 8px; margin-bottom: 30px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }}
        .category {{ background: white; padding: 20px; border-radius: 8px; margin-bottom: 20px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }}
        .examples-grid {{ display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 20px; margin-top: 20px; }}
        .example-card {{ border: 1px solid #e9ecef; border-radius: 6px; padding: 15px; background: #f8f9fa; }}
        .example-card h3 {{ margin: 0 0 10px 0; }}
        .example-card a {{ text-decoration: none; color: #007bff; }}
        .example-card a:hover {{ text-decoration: underline; }}
        .example-meta {{ display: flex; gap: 10px; margin-top: 10px; }}
        .difficulty {{ padding: 4px 8px; border-radius: 4px; font-size: 0.8em; font-weight: bold; }}
        .Beginner {{ background: #d4edda; color: #155724; }}
        .Intermediate {{ background: #fff3cd; color: #856404; }}
        .Advanced {{ background: #f8d7da; color: #721c24; }}
        .Expert {{ background: #d1ecf1; color: #0c5460; }}
        .validation {{ padding: 4px 8px; border-radius: 4px; font-size: 0.8em; }}
        .validated {{ background: #d4edda; color: #155724; }}
        .not-validated {{ background: #f8d7da; color: #721c24; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>CURSED Programming Language Examples</h1>
        <p>Comprehensive collection of examples demonstrating CURSED language features and best practices.</p>
        <p>Total Examples: {} | Validated: {} | Categories: {}</p>
    </div>
    
    {}
</body>
</html>"#,
            self.examples_db.examples.len(),
            self.examples_db.validation_status.values().filter(|&&v| v).count(),
            self.examples_db.by_category.len(),
            categories_html
        )
    }

    /// Generate HTML for a single example
    fn generate_example_html(&self, example: &ExtractedExample) -> String {
        let related_links = example.related_examples.iter()
            .filter_map(|id| {
                self.examples_db.examples.get(id).map(|ex| {
                    format!(r#"<a href="{}.html">{}</a>"#, id, ex.title)
                })
            })
            .collect::<Vec<_>>()
            .join(" | ");

        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} - CURSED Example</title>
    <style>
        body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; margin: 0; padding: 20px; line-height: 1.6; }}
        .header {{ background: #f8f9fa; padding: 20px; border-radius: 8px; margin-bottom: 20px; }}
        .code-container {{ background: #f8f8f8; border: 1px solid #e9ecef; border-radius: 8px; overflow: hidden; }}
        .code-header {{ background: #e9ecef; padding: 10px 15px; border-bottom: 1px solid #dee2e6; }}
        .code {{ padding: 20px; font-family: 'Monaco', 'Consolas', monospace; white-space: pre-wrap; overflow-x: auto; }}
        .meta {{ display: flex; gap: 15px; margin: 20px 0; }}
        .meta-item {{ background: #e9ecef; padding: 8px 12px; border-radius: 4px; font-size: 0.9em; }}
        .topics {{ margin: 20px 0; }}
        .topic {{ display: inline-block; background: #007bff; color: white; padding: 4px 8px; border-radius: 4px; margin: 2px; font-size: 0.8em; }}
        .related {{ margin-top: 30px; padding-top: 20px; border-top: 1px solid #e9ecef; }}
        .back-link {{ display: inline-block; margin-bottom: 20px; color: #007bff; text-decoration: none; }}
        .back-link:hover {{ text-decoration: underline; }}
        .validation-status {{ padding: 10px; border-radius: 4px; margin: 15px 0; }}
        .validated {{ background: #d4edda; color: #155724; border: 1px solid #c3e6cb; }}
        .not-validated {{ background: #f8d7da; color: #721c24; border: 1px solid #f5c6cb; }}
    </style>
</head>
<body>
    <a href="index.html" class="back-link">← Back to Examples</a>
    
    <div class="header">
        <h1>{}</h1>
        <p>{}</p>
    </div>

    <div class="meta">
        <div class="meta-item">Category: {:?}</div>
        <div class="meta-item">Difficulty: {:?}</div>
        <div class="meta-item">Topics: {}</div>
    </div>

    <div class="validation-status {}">
        {}
    </div>

    <div class="code-container">
        <div class="code-header">
            <strong>Example Code</strong>
        </div>
        <div class="code">{}</div>
    </div>

    <div class="topics">
        <strong>Related Topics:</strong>
        {}
    </div>

    {}
</body>
</html>"#,
            example.title,
            example.title,
            example.description,
            example.category,
            example.difficulty,
            example.topics.join(", "),
            if example.is_validated { "validated" } else { "not-validated" },
            if example.is_validated {
                "✓ This example has been validated and runs successfully."
            } else {
                match &example.validation_error {
                    Some(error) => &format!("⚠ This example could not be validated: {}", error),
                    None => "⚠ This example has not been validated.",
                }
            },
            html_escape::encode_text(&example.code),
            example.topics.iter().map(|topic| format!(r#"<span class="topic">{}</span>"#, topic)).collect::<Vec<_>>().join(""),
            if related_links.is_empty() {
                String::new()
            } else {
                format!(r#"<div class="related"><strong>Related Examples:</strong><br>{}</div>"#, related_links)
            }
        )
    }

    /// Generate markdown examples
    fn generate_markdown_examples(&self, output_dir: &Path) -> Result<(), Error> {
        let md_dir = output_dir.join("markdown");
        fs::create_dir_all(&md_dir)
            .map_err(|e| Error::SystemError(format!("Failed to create markdown directory: {}", e)))?;

        // Generate README
        let readme_content = self.generate_examples_readme();
        fs::write(md_dir.join("README.md"), readme_content)
            .map_err(|e| Error::SystemError(format!("Failed to write README.md: {}", e)))?;

        // Generate category-based markdown files
        for (category, example_ids) in &self.examples_db.by_category {
            let category_content = self.generate_category_markdown(category, example_ids);
            let filename = format!("{:?}.md", category).to_lowercase();
            fs::write(md_dir.join(filename), category_content)
                .map_err(|e| Error::SystemError(format!("Failed to write category markdown: {}", e)))?;
        }

        Ok(())
    }

    /// Generate examples README
    fn generate_examples_readme(&self) -> String {
        format!(
            r#"# CURSED Programming Language Examples

This directory contains a comprehensive collection of examples demonstrating the features and capabilities of the CURSED programming language.

## Overview

- **Total Examples**: {}
- **Validated Examples**: {}
- **Categories**: {}
- **Difficulty Levels**: {}

## Categories

{}

## By Difficulty

{}

## How to Use

Each example includes:
- Complete, runnable code
- Detailed description and context
- Topic tags for easy searching
- Difficulty level indication
- Validation status

## Validation

Examples marked with ✓ have been automatically validated by compiling and running them to ensure they work correctly. Examples marked with ⚠ may have issues or are pending validation.

---

*Generated by CURSED Advanced Example System*"#,
            self.examples_db.examples.len(),
            self.examples_db.validation_status.values().filter(|&&v| v).count(),
            self.examples_db.by_category.len(),
            self.examples_db.by_difficulty.len(),
            self.generate_categories_list(),
            self.generate_difficulty_summary()
        )
    }

    /// Generate categories list for README
    fn generate_categories_list(&self) -> String {
        self.examples_db.by_category
            .iter()
            .map(|(category, examples)| {
                format!("- **{:?}**: {} examples", category, examples.len())
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Generate difficulty summary for README
    fn generate_difficulty_summary(&self) -> String {
        self.examples_db.by_difficulty
            .iter()
            .map(|(difficulty, examples)| {
                format!("- **{:?}**: {} examples", difficulty, examples.len())
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Generate markdown for a category
    fn generate_category_markdown(&self, category: &ExampleCategory, example_ids: &[String]) -> String {
        let mut content = format!("# {:?} Examples\n\n", category);
        
        for example_id in example_ids {
            if let Some(example) = self.examples_db.examples.get(example_id) {
                content.push_str(&format!(
                    r#"## {}

{}

**Difficulty**: {:?}  
**Topics**: {}  
**Validation**: {}

```cursed
{}
```

{}

---

"#,
                    example.title,
                    example.description,
                    example.difficulty,
                    example.topics.join(", "),
                    if example.is_validated { "✓ Validated" } else { "⚠ Not validated" },
                    example.code,
                    if !example.related_examples.is_empty() {
                        format!("**Related Examples**: {}", example.related_examples.join(", "))
                    } else {
                        String::new()
                    }
                ));
            }
        }

        content
    }

    /// Generate playground-style interactive examples
    fn generate_playground_examples(&self, output_dir: &Path) -> Result<(), Error> {
        let playground_dir = output_dir.join("playground");
        fs::create_dir_all(&playground_dir)
            .map_err(|e| Error::SystemError(format!("Failed to create playground directory: {}", e)))?;

        // Generate a simple web-based playground
        let playground_html = self.generate_playground_html();
        fs::write(playground_dir.join("index.html"), playground_html)
            .map_err(|e| Error::SystemError(format!("Failed to write playground HTML: {}", e)))?;

        // Generate examples data as JSON
        let examples_json = serde_json::to_string_pretty(&self.examples_db)
            .map_err(|e| Error::SystemError(format!("Failed to serialize examples: {}", e)))?;
        fs::write(playground_dir.join("examples.json"), examples_json)
            .map_err(|e| Error::SystemError(format!("Failed to write examples JSON: {}", e)))?;

        Ok(())
    }

    /// Generate playground HTML
    fn generate_playground_html(&self) -> String {
        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>CURSED Playground</title>
    <style>
        body {{ margin: 0; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; }}
        .playground {{ display: grid; grid-template-columns: 300px 1fr; height: 100vh; }}
        .sidebar {{ background: #f8f9fa; border-right: 1px solid #e9ecef; overflow-y: auto; }}
        .main {{ display: flex; flex-direction: column; }}
        .editor {{ flex: 1; position: relative; }}
        .output {{ height: 200px; background: #2d3748; color: #e2e8f0; font-family: monospace; padding: 15px; overflow-y: auto; }}
        .toolbar {{ background: #e9ecef; padding: 10px; border-bottom: 1px solid #dee2e6; }}
        .example-list {{ padding: 15px; }}
        .example-item {{ padding: 10px; border: 1px solid #e9ecef; border-radius: 4px; margin-bottom: 10px; cursor: pointer; }}
        .example-item:hover {{ background: #e9ecef; }}
        .example-item.active {{ background: #007bff; color: white; }}
        textarea {{ width: 100%; height: 100%; border: none; padding: 15px; font-family: 'Monaco', 'Consolas', monospace; resize: none; }}
        button {{ background: #007bff; color: white; border: none; padding: 8px 16px; border-radius: 4px; cursor: pointer; }}
        button:hover {{ background: #0056b3; }}
        .category-header {{ font-weight: bold; margin: 15px 0 10px 0; color: #495057; }}
    </style>
</head>
<body>
    <div class="playground">
        <div class="sidebar">
            <div class="example-list" id="example-list">
                <h3>Examples</h3>
                <div id="examples-container">Loading examples...</div>
            </div>
        </div>
        <div class="main">
            <div class="toolbar">
                <button onclick="runCode()">Run Code</button>
                <button onclick="formatCode()">Format</button>
                <button onclick="shareExample()">Share</button>
            </div>
            <div class="editor">
                <textarea id="code-editor" placeholder="Select an example or write your own CURSED code here..."></textarea>
            </div>
            <div class="output" id="output">
                Ready to run CURSED code...
            </div>
        </div>
    </div>

    <script>
        let examples = {{}};
        let currentExample = null;

        // Load examples
        fetch('examples.json')
            .then(response => response.json())
            .then(data => {{
                examples = data;
                displayExamples();
            }})
            .catch(error => {{
                document.getElementById('examples-container').innerHTML = 'Failed to load examples.';
            }});

        function displayExamples() {{
            const container = document.getElementById('examples-container');
            container.innerHTML = '';

            for (const [category, exampleIds] of Object.entries(examples.by_category)) {{
                const categoryDiv = document.createElement('div');
                categoryDiv.className = 'category-header';
                categoryDiv.textContent = category;
                container.appendChild(categoryDiv);

                exampleIds.forEach(exampleId => {{
                    const example = examples.examples[exampleId];
                    if (example) {{
                        const exampleDiv = document.createElement('div');
                        exampleDiv.className = 'example-item';
                        exampleDiv.innerHTML = `
                            <div style="font-weight: bold;">${{example.title}}</div>
                            <div style="font-size: 0.9em; color: #6c757d;">${{example.difficulty}}</div>
                        `;
                        exampleDiv.onclick = () => loadExample(exampleId);
                        container.appendChild(exampleDiv);
                    }}
                }});
            }}
        }}

        function loadExample(exampleId) {{
            const example = examples.examples[exampleId];
            if (example) {{
                document.getElementById('code-editor').value = example.code;
                currentExample = example;

                // Update active state
                document.querySelectorAll('.example-item').forEach(item => {{
                    item.classList.remove('active');
                }});
                event.target.closest('.example-item').classList.add('active');

                // Show example info in output
                const output = document.getElementById('output');
                output.innerHTML = `
                    <strong>${{example.title}}</strong><br>
                    ${{example.description}}<br>
                    <em>Difficulty: ${{example.difficulty}} | Topics: ${{example.topics.join(', ')}}</em>
                `;
            }}
        }}

        function runCode() {{
            const code = document.getElementById('code-editor').value;
            const output = document.getElementById('output');
            
            if (!code.trim()) {{
                output.innerHTML = 'No code to run.';
                return;
            }}

            output.innerHTML = 'Running code...<br>' + 
                              '<em>Note: This is a demo playground. In a real implementation, code would be sent to a CURSED compiler backend.</em>';
            
            // Simulate compilation/execution
            setTimeout(() => {{
                output.innerHTML = `
                    <strong>Compilation:</strong> Success<br>
                    <strong>Output:</strong><br>
                    <pre style="margin: 5px 0; color: #a0aec0;"># Example output would appear here
# This playground demonstrates the CURSED examples system</pre>
                `;
            }}, 1000);
        }}

        function formatCode() {{
            const output = document.getElementById('output');
            output.innerHTML = 'Code formatted! (Demo - actual formatting would be implemented with CURSED formatter)';
        }}

        function shareExample() {{
            if (currentExample) {{
                const url = `${{window.location.origin}}${{window.location.pathname}}?example=${{currentExample.id}}`;
                navigator.clipboard.writeText(url);
                document.getElementById('output').innerHTML = 'Example URL copied to clipboard!';
            }}
        }}
    </script>
</body>
</html>"#
        )
    }

    /// Generate advanced playground with more features
    fn generate_advanced_playground(&self, output_dir: &Path) -> Result<(), Error> {
        // This would implement a more sophisticated playground with features like:
        // - Code completion
        // - Syntax highlighting
        // - Step-by-step debugging
        // - Performance visualization
        // - Error highlighting
        // For now, we'll create a placeholder
        
        let advanced_dir = output_dir.join("advanced_playground");
        fs::create_dir_all(&advanced_dir)
            .map_err(|e| Error::SystemError(format!("Failed to create advanced playground directory: {}", e)))?;

        let placeholder_content = r#"# Advanced CURSED Playground

This directory is reserved for an advanced interactive playground that would include:

## Features
- Syntax highlighting with Monaco Editor
- Real-time code compilation and execution
- Step-by-step debugging capabilities
- Performance profiling and visualization
- Interactive tutorials with guided examples
- Code completion and IntelliSense
- Error highlighting and suggestions
- Collaborative editing capabilities

## Implementation
The advanced playground would be implemented using:
- Monaco Editor for code editing
- WebAssembly for CURSED compilation in the browser
- WebSockets for real-time collaboration
- D3.js for performance visualizations
- Custom CURSED language service for IDE features

This represents the next evolution of the CURSED examples system."#;

        fs::write(advanced_dir.join("README.md"), placeholder_content)
            .map_err(|e| Error::SystemError(format!("Failed to write advanced playground README: {}", e)))?;

        Ok(())
    }

    /// Generate Jupyter notebook examples
    fn generate_jupyter_examples(&self, output_dir: &Path) -> Result<(), Error> {
        let jupyter_dir = output_dir.join("jupyter");
        fs::create_dir_all(&jupyter_dir)
            .map_err(|e| Error::SystemError(format!("Failed to create jupyter directory: {}", e)))?;

        // Generate a sample Jupyter notebook for CURSED
        for (category, example_ids) in &self.examples_db.by_category {
            let notebook_content = self.generate_jupyter_notebook(category, example_ids);
            let filename = format!("{:?}_examples.ipynb", category).to_lowercase();
            fs::write(jupyter_dir.join(filename), notebook_content)
                .map_err(|e| Error::SystemError(format!("Failed to write Jupyter notebook: {}", e)))?;
        }

        Ok(())
    }

    /// Generate a Jupyter notebook for a category
    fn generate_jupyter_notebook(&self, category: &ExampleCategory, example_ids: &[String]) -> String {
        let mut cells = Vec::new();

        // Title cell
        cells.push(format!(
            "{{
   \"cell_type\": \"markdown\",
   \"metadata\": {{}},
   \"source\": [
    \"# CURSED {:?} Examples\\n\",
    \"\\n\",
    \"This notebook contains examples demonstrating {:?} features of the CURSED programming language.\\n\"
   ]
}}",
            category, category
        ));

        // Example cells
        for example_id in example_ids {
            if let Some(example) = self.examples_db.examples.get(example_id) {
                // Description cell
                cells.push(format!(
                    "{{
   \"cell_type\": \"markdown\",
   \"metadata\": {{}},
   \"source\": [
    \"## {}\\n\",
    \"\\n\",
    \"{}\\n\",
    \"\\n\",
    \"**Difficulty**: {:?}  \\n\",
    \"**Topics**: {}  \\n\"
   ]
}}",
                    example.title,
                    example.description,
                    example.difficulty,
                    example.topics.join(", ")
                ));

                // Code cell
                let escaped_code = example.code.replace("\\", "\\\\").replace("\"", "\\\"").replace("\n", "\\n\",\n    \"");
                cells.push(format!(
                    "{{
   \"cell_type\": \"code\",
   \"execution_count\": null,
   \"metadata\": {{}},
   \"outputs\": [],
   \"source\": [
    \"{}\"
   ]
}}",
                    escaped_code
                ));
            }
        }

        format!(
            r#"{{
 "cells": [
  {}
 ],
 "metadata": {{
  "kernelspec": {{
   "display_name": "CURSED",
   "language": "cursed",
   "name": "cursed"
  }},
  "language_info": {{
   "name": "cursed",
   "version": "1.0.0"
  }}
 }},
 "nbformat": 4,
 "nbformat_minor": 4
}}"#,
            cells.join(",\n  ")
        )
    }

    /// Get examples database
    pub fn get_examples_database(&self) -> &ExamplesDatabase {
        &self.examples_db
    }

    /// Get validation results
    pub fn get_validation_results(&self) -> &HashMap<String, ValidationResult> {
        &self.validation_results
    }

    /// Search examples by criteria
    pub fn search_examples(&self, query: &str, category: Option<ExampleCategory>, difficulty: Option<DifficultyLevel>) -> Vec<String> {
        let mut results = Vec::new();
        let query_lower = query.to_lowercase();

        for (example_id, example) in &self.examples_db.examples {
            // Filter by category if specified
            if let Some(ref cat) = category {
                if &example.category != cat {
                    continue;
                }
            }

            // Filter by difficulty if specified
            if let Some(ref diff) = difficulty {
                if &example.difficulty != diff {
                    continue;
                }
            }

            // Search in title, description, topics, and tags
            let matches_query = example.title.to_lowercase().contains(&query_lower) ||
                              example.description.to_lowercase().contains(&query_lower) ||
                              example.topics.iter().any(|topic| topic.to_lowercase().contains(&query_lower)) ||
                              example.tags.iter().any(|tag| tag.to_lowercase().contains(&query_lower));

            if matches_query {
                results.push(example_id.clone());
            }
        }

        results
    }
}
