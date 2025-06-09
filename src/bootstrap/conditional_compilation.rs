//! # Conditional Compilation System
//!
//! This module provides feature-conditional compilation directives that allow
//! CURSED programs to adapt their behavior based on available compiler capabilities.
//! It integrates with the feature detection system to enable graceful degradation.

use std::collections::HashMap;
use tracing::{debug, instrument, warn};
use crate::ast::traits::{Expression, Statement, Node};
use crate::bootstrap::feature_detection::{CompilerFeature, FeatureDetectionSystem, FeatureSupport};

/// Conditional compilation directive types
#[derive(Debug, Clone, PartialEq)]
pub enum ConditionalDirective {
    /// Enable code if feature is supported
    IfFeature(CompilerFeature),
    /// Enable code if feature is NOT supported  
    IfNotFeature(CompilerFeature),
    /// Enable code if feature has minimum support level
    IfFeatureLevel(CompilerFeature, FeatureSupport),
    /// Enable code if compiler version meets requirement
    IfVersion(String),
    /// Enable code if custom feature flag is set
    IfFlag(String),
    /// Enable code if ALL conditions are met
    IfAll(Vec<ConditionalDirective>),
    /// Enable code if ANY condition is met
    IfAny(Vec<ConditionalDirective>),
}

/// Conditional compilation block
#[derive(Debug)]
pub struct ConditionalBlock {
    pub condition: ConditionalDirective,
    pub then_statements: Vec<String>,  // Store as strings instead of AST nodes
    pub else_statements: Option<Vec<String>>,
}

/// Fallback strategy for unsupported features
#[derive(Debug, Clone, PartialEq)]
pub enum FallbackStrategy {
    /// Remove the code entirely
    Remove,
    /// Replace with no-op implementation
    NoOp,
    /// Replace with simpler implementation
    Simplify,
    /// Replace with runtime error
    RuntimeError,
    /// Replace with compile-time error
    CompileError(String),
}

/// Feature-specific fallback configuration
#[derive(Debug, Clone)]
pub struct FeatureFallback {
    pub feature: CompilerFeature,
    pub strategy: FallbackStrategy,
    pub replacement_code: Option<String>,
    pub warning_message: Option<String>,
}

/// Conditional compilation processor
#[derive(Debug)]
pub struct ConditionalCompiler {
    feature_system: Option<FeatureDetectionSystem>,
    fallback_strategies: HashMap<CompilerFeature, FeatureFallback>,
    enabled_flags: std::collections::HashSet<String>,
    processed_blocks: Vec<ConditionalBlock>,
}

impl ConditionalCompiler {
    /// Create a new conditional compiler
    pub fn new(feature_system: Option<FeatureDetectionSystem>) -> Self {
        let mut compiler = Self {
            feature_system,
            fallback_strategies: HashMap::new(),
            enabled_flags: std::collections::HashSet::new(),
            processed_blocks: Vec::new(),
        };
        
        compiler.setup_default_fallbacks();
        compiler
    }

    /// Set up default fallback strategies for common features
    fn setup_default_fallbacks(&mut self) {
        use CompilerFeature::*;
        use FallbackStrategy::*;

        let default_fallbacks = [
            (Goroutines, FeatureFallback {
                feature: Goroutines,
                strategy: Simplify,
                replacement_code: Some("// Sequential execution fallback".to_string()),
                warning_message: Some("Goroutines not supported, using sequential execution".to_string()),
            }),
            (Channels, FeatureFallback {
                feature: Channels,
                strategy: Simplify,
                replacement_code: Some("// Direct communication fallback".to_string()),
                warning_message: Some("Channels not supported, using direct communication".to_string()),
            }),
            (OptimizedCodegen, FeatureFallback {
                feature: OptimizedCodegen,
                strategy: NoOp,
                replacement_code: None,
                warning_message: Some("Optimized codegen not available, using standard codegen".to_string()),
            }),
            (JitCompilation, FeatureFallback {
                feature: JitCompilation,
                strategy: CompileError("JIT compilation required but not available".to_string()),
                replacement_code: None,
                warning_message: None,
            }),
            (AdvancedTypes, FeatureFallback {
                feature: AdvancedTypes,
                strategy: Simplify,
                replacement_code: Some("// Using basic types instead of advanced types".to_string()),
                warning_message: Some("Advanced types not supported, using basic types".to_string()),
            }),
        ];

        for (feature, fallback) in default_fallbacks {
            self.fallback_strategies.insert(feature, fallback);
        }
    }

    /// Evaluate a conditional directive
    #[instrument(level = "trace", skip(self))]
    pub fn evaluate_condition(&self, directive: &ConditionalDirective) -> bool {
        match directive {
            ConditionalDirective::IfFeature(feature) => {
                self.feature_system
                    .as_ref()
                    .map(|sys| sys.is_feature_supported(feature))
                    .unwrap_or(false)
            },
            ConditionalDirective::IfNotFeature(feature) => {
                !self.feature_system
                    .as_ref()
                    .map(|sys| sys.is_feature_supported(feature))
                    .unwrap_or(true)
            },
            ConditionalDirective::IfFeatureLevel(feature, min_level) => {
                self.feature_system
                    .as_ref()
                    .map(|sys| {
                        let current_level = sys.get_feature_support(feature);
                        self.support_level_meets_requirement(&current_level, min_level)
                    })
                    .unwrap_or(false)
            },
            ConditionalDirective::IfVersion(version_req) => {
                self.feature_system
                    .as_ref()
                    .map(|sys| self.version_meets_requirement(&sys.current_version.to_string(), version_req))
                    .unwrap_or(false)
            },
            ConditionalDirective::IfFlag(flag) => {
                self.enabled_flags.contains(flag)
            },
            ConditionalDirective::IfAll(conditions) => {
                conditions.iter().all(|cond| self.evaluate_condition(cond))
            },
            ConditionalDirective::IfAny(conditions) => {
                conditions.iter().any(|cond| self.evaluate_condition(cond))
            },
        }
    }

    /// Check if current support level meets minimum requirement
    fn support_level_meets_requirement(&self, current: &FeatureSupport, minimum: &FeatureSupport) -> bool {
        use FeatureSupport::*;
        match (current, minimum) {
            (Stable, _) => true,
            (Limited, Limited) | (Limited, Experimental) | (Limited, Unsupported) => true,
            (Experimental, Experimental) | (Experimental, Unsupported) => true,
            (Unsupported, Unsupported) => true,
            _ => false,
        }
    }

    /// Check if version meets requirement (simplified version comparison)
    fn version_meets_requirement(&self, current: &str, requirement: &str) -> bool {
        // Simple string comparison for now
        // In a real implementation, this would use proper semantic versioning
        current >= requirement
    }

    /// Process conditional compilation directives in source code
    #[instrument(level = "debug", skip(self, source))]
    pub fn process_source(&mut self, source: &str) -> Result<String, String> {
        let mut processed_source = String::new();
        let lines: Vec<(usize, &str)> = source.lines().enumerate().collect();
        let mut line_iter = lines.into_iter();
        
        while let Some((line_num, line)) = line_iter.next() {
            if line.trim().starts_with("#if_feature") || line.trim().starts_with("#ifdef") {
                match self.parse_conditional_block(&mut line_iter, line_num, line) {
                    Ok(processed_block) => {
                        processed_source.push_str(&processed_block);
                    },
                    Err(e) => {
                        warn!(line = line_num, error = %e, "Failed to process conditional block");
                        return Err(format!("Line {}: {}", line_num + 1, e));
                    }
                }
            } else {
                processed_source.push_str(line);
                processed_source.push('\n');
            }
        }
        
        debug!(original_lines = source.lines().count(), processed_lines = processed_source.lines().count(), "Source processing completed");
        Ok(processed_source)
    }

    /// Parse a conditional compilation block
    fn parse_conditional_block<'a>(&self, lines: &mut impl Iterator<Item = (usize, &'a str)>, start_line: usize, directive_line: &str) -> Result<String, String>
    {
        let directive = self.parse_directive(directive_line)?;
        let condition_met = self.evaluate_condition(&directive);
        
        let mut then_block = String::new();
        let mut else_block = String::new();
        let mut in_else = false;
        let mut nesting_level = 0;
        
        for (line_num, line) in lines {
            let trimmed = line.trim();
            
            if trimmed.starts_with("#if_feature") || trimmed.starts_with("#ifdef") {
                nesting_level += 1;
            } else if trimmed == "#else" && nesting_level == 0 {
                in_else = true;
                continue;
            } else if trimmed == "#endif" {
                if nesting_level == 0 {
                    break;
                } else {
                    nesting_level -= 1;
                }
            }
            
            if in_else {
                else_block.push_str(line);
                else_block.push('\n');
            } else {
                then_block.push_str(line);
                then_block.push('\n');
            }
        }
        
        let result = if condition_met {
            then_block
        } else if !else_block.is_empty() {
            else_block
        } else {
            // Apply fallback strategy if available
            self.apply_fallback_strategy(&directive)
        };
        
        debug!(directive = ?directive, condition_met, start_line, "Conditional block processed");
        Ok(result)
    }

    /// Parse a conditional directive from a line
    fn parse_directive(&self, line: &str) -> Result<ConditionalDirective, String> {
        let trimmed = line.trim();
        
        if let Some(feature_name) = trimmed.strip_prefix("#if_feature ") {
            let feature = self.parse_feature_name(feature_name.trim())?;
            Ok(ConditionalDirective::IfFeature(feature))
        } else if let Some(feature_name) = trimmed.strip_prefix("#if_not_feature ") {
            let feature = self.parse_feature_name(feature_name.trim())?;
            Ok(ConditionalDirective::IfNotFeature(feature))
        } else if let Some(flag_name) = trimmed.strip_prefix("#ifdef ") {
            Ok(ConditionalDirective::IfFlag(flag_name.trim().to_string()))
        } else if let Some(version_req) = trimmed.strip_prefix("#if_version ") {
            Ok(ConditionalDirective::IfVersion(version_req.trim().to_string()))
        } else {
            Err(format!("Unknown conditional directive: {}", trimmed))
        }
    }

    /// Parse feature name from string
    fn parse_feature_name(&self, name: &str) -> Result<CompilerFeature, String> {
        use CompilerFeature::*;
        
        match name.to_lowercase().as_str() {
            "basic_types" => Ok(BasicTypes),
            "advanced_types" => Ok(AdvancedTypes),
            "generics" => Ok(Generics),
            "interfaces" => Ok(Interfaces),
            "type_assertion" => Ok(TypeAssertion),
            "error_handling" => Ok(ErrorHandling),
            "garbage_collection" | "gc" => Ok(GarbageCollection),
            "memory_profiler" => Ok(MemoryProfiler),
            "leak_detection" => Ok(LeakDetection),
            "goroutines" => Ok(Goroutines),
            "channels" => Ok(Channels),
            "channel_buffering" => Ok(ChannelBuffering),
            "select_statement" => Ok(SelectStatement),
            "mutex_support" => Ok(MutexSupport),
            "llvm_codegen" => Ok(LlvmCodegen),
            "jit_compilation" => Ok(JitCompilation),
            "optimized_codegen" => Ok(OptimizedCodegen),
            "bitstream_output" => Ok(BitstreamOutput),
            "static_linking" => Ok(StaticLinking),
            "reflection" => Ok(Reflection),
            "meta_programming" => Ok(MetaProgramming),
            "compiler_plugins" => Ok(CompilerPlugins),
            "cross_compilation" => Ok(CrossCompilation),
            "stdlib_core" => Ok(StdlibCore),
            "stdlib_extended" => Ok(StdlibExtended),
            "stdlib_experimental" => Ok(StdlibExperimental),
            "debug_info" => Ok(DebugInfo),
            "profiling" => Ok(Profiling),
            "trace_generation" => Ok(TraceGeneration),
            "error_recovery" => Ok(ErrorRecovery),
            "language_server" => Ok(LanguageServer),
            "syntax_highlighting" => Ok(SyntaxHighlighting),
            "auto_complete" => Ok(AutoComplete),
            "refactoring" => Ok(Refactoring),
            _ => Err(format!("Unknown feature: {}", name)),
        }
    }

    /// Apply fallback strategy for unsupported features
    fn apply_fallback_strategy(&self, directive: &ConditionalDirective) -> String {
        let feature = match directive {
            ConditionalDirective::IfFeature(f) | ConditionalDirective::IfNotFeature(f) | 
            ConditionalDirective::IfFeatureLevel(f, _) => Some(f),
            _ => None,
        };
        
        if let Some(feature) = feature {
            if let Some(fallback) = self.fallback_strategies.get(feature) {
                if let Some(ref warning) = fallback.warning_message {
                    warn!(feature = ?feature, warning = %warning, "Applying fallback strategy");
                }
                
                match &fallback.strategy {
                    FallbackStrategy::Remove => String::new(),
                    FallbackStrategy::NoOp => "// Feature not available\n".to_string(),
                    FallbackStrategy::Simplify => {
                        fallback.replacement_code.clone().unwrap_or_else(|| {
                            format!("// Simplified implementation for {}\n", feature)
                        })
                    },
                    FallbackStrategy::RuntimeError => {
                        format!("panic!(\"Feature {} not supported at runtime\");\n", feature)
                    },
                    FallbackStrategy::CompileError(msg) => {
                        format!("compile_error!(\"{}\");\n", msg)
                    },
                }
            } else {
                format!("// No fallback strategy configured for {}\n", feature)
            }
        } else {
            "// Condition not met, no fallback available\n".to_string()
        }
    }

    /// Add a custom feature flag
    pub fn add_feature_flag(&mut self, flag: String) {
        self.enabled_flags.insert(flag);
    }

    /// Remove a feature flag
    pub fn remove_feature_flag(&mut self, flag: &str) {
        self.enabled_flags.remove(flag);
    }

    /// Set custom fallback strategy for a feature
    pub fn set_fallback_strategy(&mut self, feature: CompilerFeature, fallback: FeatureFallback) {
        self.fallback_strategies.insert(feature, fallback);
    }

    /// Get processing statistics
    pub fn get_statistics(&self) -> ConditionalCompilerStats {
        ConditionalCompilerStats {
            processed_blocks: self.processed_blocks.len(),
            enabled_flags: self.enabled_flags.len(),
            fallback_strategies: self.fallback_strategies.len(),
        }
    }
}

/// Statistics for conditional compilation processing
#[derive(Debug, Clone)]
pub struct ConditionalCompilerStats {
    pub processed_blocks: usize,
    pub enabled_flags: usize,
    pub fallback_strategies: usize,
}

/// Macro for feature-conditional compilation in Rust code
#[macro_export]
macro_rules! if_feature {
    ($feature:expr, $then:block) => {
        if $crate::bootstrap::feature_detection::is_feature_supported(&$feature) {
            $then
        }
    };
    ($feature:expr, $then:block, $else:block) => {
        if $crate::bootstrap::feature_detection::is_feature_supported(&$feature) {
            $then
        } else {
            $else
        }
    };
}

/// Macro for feature level conditional compilation
#[macro_export]
macro_rules! if_feature_level {
    ($feature:expr, $min_level:expr, $then:block) => {
        {
            let current_level = $crate::bootstrap::feature_detection::get_feature_support_level(&$feature);
            if $crate::bootstrap::conditional_compilation::support_level_sufficient(&current_level, &$min_level) {
                $then
            }
        }
    };
}

/// Helper function for support level checking
pub fn support_level_sufficient(current: &FeatureSupport, minimum: &FeatureSupport) -> bool {
    use FeatureSupport::*;
    match (current, minimum) {
        (Stable, _) => true,
        (Limited, Limited) | (Limited, Experimental) | (Limited, Unsupported) => true,
        (Experimental, Experimental) | (Experimental, Unsupported) => true,
        (Unsupported, Unsupported) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bootstrap::feature_detection::{BootstrapStage, CompilerVersion};

    #[test]
    fn test_conditional_compiler_creation() {
        let compiler = ConditionalCompiler::new(None);
        assert!(!compiler.fallback_strategies.is_empty());
    }

    #[test]
    fn test_condition_evaluation_without_feature_system() {
        let compiler = ConditionalCompiler::new(None);
        let condition = ConditionalDirective::IfFeature(CompilerFeature::BasicTypes);
        
        // Should return false when no feature system is available
        assert!(!compiler.evaluate_condition(&condition));
    }

    #[test]
    fn test_condition_evaluation_with_feature_system() {
        let version = CompilerVersion {
            major: 0,
            minor: 1,
            patch: 0,
            stage: BootstrapStage::Stage1,
            commit_hash: None,
            build_timestamp: None,
        };
        
        let feature_system = FeatureDetectionSystem::new(BootstrapStage::Stage1, version);
        let compiler = ConditionalCompiler::new(Some(feature_system));
        
        let condition = ConditionalDirective::IfFeature(CompilerFeature::BasicTypes);
        assert!(compiler.evaluate_condition(&condition));
        
        let condition = ConditionalDirective::IfFeature(CompilerFeature::MetaProgramming);
        // MetaProgramming is experimental in Stage1, so should be supported
        assert!(compiler.evaluate_condition(&condition));
    }

    #[test]
    fn test_feature_flag_evaluation() {
        let mut compiler = ConditionalCompiler::new(None);
        compiler.add_feature_flag("custom_feature".to_string());
        
        let condition = ConditionalDirective::IfFlag("custom_feature".to_string());
        assert!(compiler.evaluate_condition(&condition));
        
        let condition = ConditionalDirective::IfFlag("nonexistent_feature".to_string());
        assert!(!compiler.evaluate_condition(&condition));
    }

    #[test]
    fn test_complex_conditions() {
        let mut compiler = ConditionalCompiler::new(None);
        compiler.add_feature_flag("debug".to_string());
        
        let condition = ConditionalDirective::IfAll(vec![
            ConditionalDirective::IfFlag("debug".to_string()),
            ConditionalDirective::IfFlag("nonexistent".to_string()),
        ]);
        assert!(!compiler.evaluate_condition(&condition));
        
        let condition = ConditionalDirective::IfAny(vec![
            ConditionalDirective::IfFlag("debug".to_string()),
            ConditionalDirective::IfFlag("nonexistent".to_string()),
        ]);
        assert!(compiler.evaluate_condition(&condition));
    }

    #[test]
    fn test_feature_name_parsing() {
        let compiler = ConditionalCompiler::new(None);
        
        assert_eq!(compiler.parse_feature_name("basic_types").unwrap(), CompilerFeature::BasicTypes);
        assert_eq!(compiler.parse_feature_name("goroutines").unwrap(), CompilerFeature::Goroutines);
        assert_eq!(compiler.parse_feature_name("gc").unwrap(), CompilerFeature::GarbageCollection);
        
        assert!(compiler.parse_feature_name("unknown_feature").is_err());
    }

    #[test]
    fn test_source_processing() {
        let source = r#"
slay main() {
    #if_feature goroutines
    spn some_goroutine()
    #else
    some_function()
    #endif
    
    vibez.spill("Hello, World!")
}
"#;
        
        let mut compiler = ConditionalCompiler::new(None);
        let processed = compiler.process_source(source).unwrap();
        
        // Since goroutines are not supported without feature system, should use else block
        assert!(processed.contains("some_function()"));
        assert!(!processed.contains("spn some_goroutine()"));
    }

    #[test]
    fn test_fallback_strategy_application() {
        let compiler = ConditionalCompiler::new(None);
        let directive = ConditionalDirective::IfFeature(CompilerFeature::Goroutines);
        
        let fallback_result = compiler.apply_fallback_strategy(&directive);
        assert!(fallback_result.contains("Sequential execution fallback"));
    }
}
