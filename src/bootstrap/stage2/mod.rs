// Stage 2 Bootstrap Compiler Module
// Provides interface to the self-hosting CURSED compiler

use std::path::Path;
use std::process::Command;
use crate::error::Error;

/// Configuration for Stage 2 compiler
#[derive(Debug, Clone)]
pub struct Stage2Config {
    pub compiler_path: String,
    pub output_dir: String,
    pub optimization_level: u8,
    pub debug_info: bool,
    pub verbose: bool,
}

impl Default for Stage2Config {
    fn default() -> Self {
        Self {
            compiler_path: "target/debug/cursed-stage2".to_string(),
            output_dir: "target".to_string(),
            optimization_level: 0,
            debug_info: false,
            verbose: false,
        }
    }
}

/// Stage 2 compiler interface
pub struct Stage2Compiler {
    config: Stage2Config,
}

impl Stage2Compiler {
    /// Create new Stage 2 compiler instance
    pub fn new(config: Stage2Config) -> Self {
        Self { config }
    }
    
    /// Check if Stage 2 compiler is available
    pub fn is_available(&self) -> bool {
        Path::new(&self.config.compiler_path).exists()
    }
    
    /// Compile CURSED source file using Stage 2 compiler
    pub fn compile_file(&self, source_path: &str, output_path: &str) -> Result<(), Error> {
        if !self.is_available() {
            return Err(crate::error::compilation_error(&format!(
                "Stage 2 compiler not found at: {}", 
                self.config.compiler_path
            )));
        }
        
        let mut cmd = Command::new(&self.config.compiler_path);
        cmd.arg(source_path);
        cmd.arg(output_path);
        
        if self.config.verbose {
            cmd.arg("--verbose");
        }
        
        if self.config.debug_info {
            cmd.arg("--debug");
        }
        
        if self.config.optimization_level > 0 {
            cmd.arg("--optimize");
            cmd.arg(self.config.optimization_level.to_string());
        }
        
        let output = cmd.output()
            .map_err(|e| crate::error::system_error(&format!("Failed to execute Stage 2 compiler: {}", e)))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(crate::error::compilation_error(&format!("Stage 2 compilation failed:\n{}", stderr)));
        }
        
        if self.config.verbose {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("Stage 2 compiler output:\n{}", stdout);
        }
        
        Ok(())
    }
    
    /// Compile multiple CURSED source files
    pub fn compile_files(&self, source_files: &[&str], output_path: &str) -> Result<(), Error> {
        // For now, compile files sequentially
        // In a full implementation, this could handle linking multiple modules
        if source_files.is_empty() {
            return Err(crate::error::compilation_error("No source files provided"));
        }
        
        // Use the first file as main and compile it
        self.compile_file(source_files[0], output_path)?;
        
        // TODO: Handle linking of multiple modules
        if source_files.len() > 1 {
            println!("Warning: Multiple file compilation not fully implemented in Stage 2");
        }
        
        Ok(())
    }
    
    /// Get Stage 2 compiler version info
    pub fn version(&self) -> Result<String, Error> {
        if !self.is_available() {
            return Err(crate::error::compilation_error("Stage 2 compiler not available"));
        }
        
        let output = Command::new(&self.config.compiler_path)
            .arg("--version")
            .output()
            .map_err(|e| crate::error::system_error(&format!("Failed to get version: {}", e)))?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Ok("CURSED Stage 2 Compiler v0.1.0".to_string())
        }
    }
}

/// Helper function to check if Stage 2 should be used
pub fn should_use_stage2() -> bool {
    std::env::var("CURSED_USE_STAGE2").is_ok() || 
    std::env::var("CURSED_SELF_HOSTING").is_ok()
}

/// Create default Stage 2 compiler instance
pub fn create_stage2_compiler() -> Stage2Compiler {
    let config = Stage2Config::default();
    Stage2Compiler::new(config)
}

/// Compile using Stage 2 if available, otherwise fall back to Stage 1
pub fn compile_with_fallback(
    source_path: &str, 
    output_path: &str,
    stage1_fallback: impl Fn(&str, &str) -> Result<(), Error>
) -> Result<(), Error> {
    if should_use_stage2() {
        let stage2 = create_stage2_compiler();
        
        if stage2.is_available() {
            println!("Using Stage 2 CURSED compiler");
            return stage2.compile_file(source_path, output_path);
        } else {
            println!("Stage 2 compiler not available, falling back to Stage 1");
        }
    }
    
    // Fall back to Stage 1 Rust compiler
    stage1_fallback(source_path, output_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_stage2_config_default() {
        let config = Stage2Config::default();
        assert_eq!(config.optimization_level, 0);
        assert!(!config.debug_info);
        assert!(!config.verbose);
    }
    
    #[test]
    fn test_stage2_compiler_creation() {
        let config = Stage2Config::default();
        let compiler = Stage2Compiler::new(config);
        
        // Should not panic
        let _ = compiler.is_available();
    }
    
    #[test]
    fn test_should_use_stage2() {
        // Test when environment variable is not set
        let result = should_use_stage2();
        // Result depends on environment, so we just test it doesn't panic
        let _ = result;
    }
}
