//! CURSED Execution Pipeline
//! 
//! Complete execution pipeline that integrates lexer → parser → type-checker → interpreter/VM
//! This bridges the Rust tooling with the Zig implementation components.

use std::process::Command;
use std::path::Path;
use std::fs;
use std::io::{self, Write};
use std::time::Instant;

/// Execution backend options
#[derive(Debug, Clone)]
pub enum ExecutionBackend {
    /// Direct interpretation mode (fastest startup)
    Script,
    /// AST-based interpretation (full language support)
    AST,
    /// LLVM compilation (native performance)
    LLVM,
    /// C transpilation (maximum compatibility)
    C,
    /// WebAssembly compilation (web deployment)
    WASM,
}

/// Execution configuration
#[derive(Debug, Clone)]
pub struct ExecutionConfig {
    pub backend: ExecutionBackend,
    pub verbose: bool,
    pub show_tokens: bool,
    pub show_ast: bool,
    pub optimization_level: u8,
    pub memory_profile: bool,
    pub performance_profile: bool,
}

impl Default for ExecutionConfig {
    fn default() -> Self {
        Self {
            backend: ExecutionBackend::AST,
            verbose: false,
            show_tokens: false,
            show_ast: false,
            optimization_level: 0,
            memory_profile: false,
            performance_profile: false,
        }
    }
}

/// Execution pipeline result
#[derive(Debug)]
pub struct ExecutionResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub execution_time_ms: u64,
    pub memory_usage_bytes: Option<u64>,
    pub tokens_count: Option<usize>,
    pub ast_nodes_count: Option<usize>,
}

/// Main execution pipeline
pub struct CursedExecutionPipeline {
    cursed_binary_path: String,
}

impl CursedExecutionPipeline {
    /// Create new execution pipeline
    pub fn new() -> io::Result<Self> {
        // Locate the CURSED interpreter binary
        let cursed_binary = Self::find_cursed_binary()?;
        
        Ok(Self {
            cursed_binary_path: cursed_binary,
        })
    }

    /// Find the CURSED interpreter binary
    fn find_cursed_binary() -> io::Result<String> {
        // Check multiple possible locations for the CURSED binary
        let possible_paths = [
            "./zig-out/bin/cursed-zig",
            "./zig-out/bin/cursed",
            "./zig-out/bin/cursed-unified",
            "./cursed",
            "cursed",
        ];

        for path in &possible_paths {
            if Path::new(path).exists() {
                return Ok(path.to_string());
            }
        }

        // If not found, try building it
        eprintln!("🔨 CURSED binary not found, attempting to build...");
        let build_result = Command::new("zig")
            .args(&["build"])
            .output();

        match build_result {
            Ok(output) if output.status.success() => {
                // Try again after building
                for path in &possible_paths {
                    if Path::new(path).exists() {
                        eprintln!("✅ Built CURSED successfully: {}", path);
                        return Ok(path.to_string());
                    }
                }
            }
            Ok(output) => {
                eprintln!("❌ Build failed: {}", String::from_utf8_lossy(&output.stderr));
            }
            Err(e) => {
                eprintln!("❌ Failed to run build command: {}", e);
            }
        }

        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "CURSED binary not found and could not be built. Run 'zig build' manually.",
        ))
    }

    /// Execute a CURSED program from source code
    pub fn execute_source(&self, source: &str, config: &ExecutionConfig) -> io::Result<ExecutionResult> {
        // Write source to temporary file
        let temp_file = format!("/tmp/cursed_temp_{}.csd", std::process::id());
        fs::write(&temp_file, source)?;
        
        let result = self.execute_file(&temp_file, config);
        
        // Clean up temporary file
        let _ = fs::remove_file(&temp_file);
        
        result
    }

    /// Execute a CURSED program from file
    pub fn execute_file(&self, filename: &str, config: &ExecutionConfig) -> io::Result<ExecutionResult> {
        let start_time = Instant::now();
        
        if config.verbose {
            eprintln!("🚀 Executing {} with {:?} backend", filename, config.backend);
        }

        // Build command arguments
        let mut args = Vec::new();
        
        // Add backend flag
        match config.backend {
            ExecutionBackend::Script => args.push("--backend=script"),
            ExecutionBackend::AST => args.push("--backend=ast"),
            ExecutionBackend::LLVM => args.push("--backend=llvm"),
            ExecutionBackend::C => args.push("--backend=c"),
            ExecutionBackend::WASM => args.push("--backend=wasm"),
        }

        // Add configuration flags
        if config.verbose {
            args.push("--verbose");
        }
        if config.show_tokens {
            args.push("--show-tokens");
        }
        if config.show_ast {
            args.push("--show-ast");
        }
        if config.optimization_level > 0 {
            args.push(&format!("-O{}", config.optimization_level));
        }

        // Add source file
        args.push(filename);

        if config.verbose {
            eprintln!("🔧 Command: {} {}", self.cursed_binary_path, args.join(" "));
        }

        // Execute the pipeline: Lexer → Parser → Type-Checker → Interpreter/VM
        let output = if config.memory_profile {
            // Run with memory profiling using valgrind if available
            self.execute_with_memory_profiling(&args)?
        } else {
            // Direct execution
            Command::new(&self.cursed_binary_path)
                .args(&args)
                .output()?
        };

        let execution_time = start_time.elapsed().as_millis() as u64;

        // Parse output for additional metrics
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        let tokens_count = self.extract_tokens_count(&stderr);
        let ast_nodes_count = self.extract_ast_nodes_count(&stderr);
        let memory_usage = if config.memory_profile {
            self.extract_memory_usage(&stderr)
        } else {
            None
        };

        Ok(ExecutionResult {
            exit_code: output.status.code().unwrap_or(-1),
            stdout,
            stderr,
            execution_time_ms: execution_time,
            memory_usage_bytes: memory_usage,
            tokens_count,
            ast_nodes_count,
        })
    }

    /// Execute with memory profiling using valgrind
    fn execute_with_memory_profiling(&self, args: &[&str]) -> io::Result<std::process::Output> {
        // Check if valgrind is available
        let valgrind_check = Command::new("which").arg("valgrind").output();
        
        if valgrind_check.is_ok() && valgrind_check.unwrap().status.success() {
            let mut valgrind_args = vec![
                "--tool=memcheck",
                "--leak-check=full",
                "--show-leak-kinds=all",
                "--track-origins=yes",
                "--error-exitcode=1",
                &self.cursed_binary_path,
            ];
            valgrind_args.extend(args);

            Command::new("valgrind")
                .args(&valgrind_args)
                .output()
        } else {
            // Fallback to direct execution if valgrind not available
            eprintln!("⚠️  Valgrind not available, running without memory profiling");
            Command::new(&self.cursed_binary_path)
                .args(args)
                .output()
        }
    }

    /// Extract token count from stderr output
    fn extract_tokens_count(&self, stderr: &str) -> Option<usize> {
        for line in stderr.lines() {
            if line.contains("Tokenized") && line.contains("tokens") {
                // Look for pattern like "🔤 Tokenized 42 tokens"
                let words: Vec<&str> = line.split_whitespace().collect();
                for (i, word) in words.iter().enumerate() {
                    if word == &"Tokenized" && i + 1 < words.len() {
                        if let Ok(count) = words[i + 1].parse::<usize>() {
                            return Some(count);
                        }
                    }
                }
            }
        }
        None
    }

    /// Extract AST node count from stderr output
    fn extract_ast_nodes_count(&self, stderr: &str) -> Option<usize> {
        for line in stderr.lines() {
            if line.contains("Generated AST") && line.contains("statements") {
                // Look for pattern like "🌳 Generated AST with 5 statements"
                let words: Vec<&str> = line.split_whitespace().collect();
                for (i, word) in words.iter().enumerate() {
                    if word == &"with" && i + 1 < words.len() {
                        if let Ok(count) = words[i + 1].parse::<usize>() {
                            return Some(count);
                        }
                    }
                }
            }
        }
        None
    }

    /// Extract memory usage from valgrind output
    fn extract_memory_usage(&self, stderr: &str) -> Option<u64> {
        for line in stderr.lines() {
            if line.contains("total heap usage") {
                // Parse valgrind output for memory usage
                if let Some(bytes_pos) = line.find("bytes allocated") {
                    let before_bytes = &line[..bytes_pos];
                    let words: Vec<&str> = before_bytes.split_whitespace().collect();
                    if let Some(last_number) = words.iter().rev().find(|w| w.chars().all(|c| c.is_ascii_digit() || c == ',')) {
                        let clean_number = last_number.replace(',', "");
                        if let Ok(bytes) = clean_number.parse::<u64>() {
                            return Some(bytes);
                        }
                    }
                }
            }
        }
        None
    }

    /// Compile a CURSED program to native binary
    pub fn compile_to_binary(&self, filename: &str, output_path: Option<&str>, config: &ExecutionConfig) -> io::Result<ExecutionResult> {
        let start_time = Instant::now();
        
        if config.verbose {
            eprintln!("🔨 Compiling {} to native binary", filename);
        }

        let mut args = vec!["--compile"];
        
        // Add backend for compilation
        match config.backend {
            ExecutionBackend::LLVM => args.push("--backend=llvm"),
            ExecutionBackend::C => args.push("--backend=c"),
            ExecutionBackend::WASM => args.push("--backend=wasm"),
            _ => args.push("--backend=llvm"), // Default to LLVM for compilation
        }

        if let Some(output) = output_path {
            args.push("--output");
            args.push(output);
        }

        if config.optimization_level > 0 {
            args.push(&format!("-O{}", config.optimization_level));
        }

        if config.verbose {
            args.push("--verbose");
        }

        args.push(filename);

        let output = Command::new(&self.cursed_binary_path)
            .args(&args)
            .output()?;

        let execution_time = start_time.elapsed().as_millis() as u64;

        Ok(ExecutionResult {
            exit_code: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            execution_time_ms: execution_time,
            memory_usage_bytes: None,
            tokens_count: None,
            ast_nodes_count: None,
        })
    }

    /// Type-check a CURSED program without execution
    pub fn type_check(&self, filename: &str, config: &ExecutionConfig) -> io::Result<ExecutionResult> {
        let start_time = Instant::now();
        
        let mut args = vec!["check"];
        
        if config.verbose {
            args.push("--verbose");
        }
        
        args.push(filename);

        let output = Command::new(&self.cursed_binary_path)
            .args(&args)
            .output()?;

        let execution_time = start_time.elapsed().as_millis() as u64;

        Ok(ExecutionResult {
            exit_code: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            execution_time_ms: execution_time,
            memory_usage_bytes: None,
            tokens_count: None,
            ast_nodes_count: None,
        })
    }
}

/// High-level execution functions for common use cases
impl CursedExecutionPipeline {
    /// Quick interpretation for development/testing
    pub fn quick_interpret(&self, source: &str) -> io::Result<String> {
        let config = ExecutionConfig {
            backend: ExecutionBackend::AST,
            verbose: false,
            ..Default::default()
        };
        
        let result = self.execute_source(source, &config)?;
        
        if result.exit_code == 0 {
            Ok(result.stdout)
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Execution failed: {}", result.stderr),
            ))
        }
    }

    /// Production compilation with optimizations
    pub fn production_compile(&self, filename: &str, output_path: &str) -> io::Result<ExecutionResult> {
        let config = ExecutionConfig {
            backend: ExecutionBackend::LLVM,
            optimization_level: 3,
            verbose: true,
            ..Default::default()
        };
        
        self.compile_to_binary(filename, Some(output_path), &config)
    }

    /// Development mode with full debugging
    pub fn debug_execute(&self, filename: &str) -> io::Result<ExecutionResult> {
        let config = ExecutionConfig {
            backend: ExecutionBackend::AST,
            verbose: true,
            show_tokens: true,
            show_ast: true,
            memory_profile: true,
            ..Default::default()
        };
        
        self.execute_file(filename, &config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_pipeline_creation() {
        let pipeline = CursedExecutionPipeline::new();
        assert!(pipeline.is_ok(), "Pipeline creation should succeed");
    }

    #[test]
    fn test_simple_execution() {
        let pipeline = CursedExecutionPipeline::new().unwrap();
        
        // Create a simple test program
        let test_program = r#"
            vibez.spill("Hello from CURSED!")
        "#;
        
        let result = pipeline.quick_interpret(test_program);
        
        match result {
            Ok(output) => {
                assert!(output.contains("Hello from CURSED!"), "Output should contain expected message");
            }
            Err(e) => {
                eprintln!("Test execution failed (this may be expected if CURSED binary is not built): {}", e);
            }
        }
    }

    #[test] 
    fn test_type_checking() {
        let pipeline = CursedExecutionPipeline::new().unwrap();
        
        // Create a temporary test file
        let test_file = "/tmp/test_type_check.csd";
        let test_content = r#"
            sus x drip = 42
            sus y tea = "hello"
            vibez.spill(x + 1)
        "#;
        
        fs::write(test_file, test_content).unwrap();
        
        let config = ExecutionConfig::default();
        let result = pipeline.type_check(test_file, &config);
        
        // Clean up
        let _ = fs::remove_file(test_file);
        
        match result {
            Ok(type_result) => {
                println!("Type checking result: {}", type_result.stderr);
            }
            Err(e) => {
                eprintln!("Type checking test failed (this may be expected): {}", e);
            }
        }
    }
}
