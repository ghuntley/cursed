/// Canonical OptimizationLevel enum for the CURSED programming language
/// 
/// This serves as the single source of truth for optimization levels across the entire codebase.
/// All other OptimizationLevel definitions should be replaced with imports from this module.

use std::fmt::{self, Display};
use std::str::FromStr;
use std::hash::{Hash, Hasher};
use serde::{Deserialize, Serialize};

/// Optimization level configuration
/// 
/// Defines the optimization levels available in the CURSED compiler, corresponding
/// to standard compiler optimization flags:
/// 
/// - `O0`: No optimization, fastest compilation, best debugging experience
/// - `O1`: Basic optimization, reasonable compilation time, some performance improvement
/// - `O2`: Standard optimization, balanced compilation time and runtime performance
/// - `O3`: Aggressive optimization, slower compilation, maximum runtime performance
/// - `Os`: Optimize for binary size while maintaining reasonable performance
/// - `Oz`: Aggressively optimize for minimum binary size
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OptimizationLevel {
    /// No optimization (-O0)
    /// 
    /// Fastest compilation time, no optimization passes applied.
    /// Best for debugging and development iterations.
    /// Preserves all debugging information and maintains predictable code layout.
    O0,
    
    /// Basic optimization (-O1)
    /// 
    /// Light optimization passes that don't significantly impact compilation time.
    /// Includes basic dead code elimination, constant propagation, and simple inlining.
    /// Good balance for development builds that need some performance.
    O1,
    
    /// Standard optimization (-O2)
    /// 
    /// Comprehensive optimization without aggressive transformations.
    /// Includes loop optimizations, advanced inlining, and most optimization passes.
    /// Recommended default for production builds.
    O2,
    
    /// Aggressive optimization (-O3)
    /// 
    /// Maximum performance optimization, may increase compilation time significantly.
    /// Includes vectorization, aggressive inlining, and speculative optimizations.
    /// Best for performance-critical production code.
    O3,
    
    /// Optimize for size (-Os)
    /// 
    /// Prioritizes binary size reduction while maintaining reasonable performance.
    /// Similar to O2 but with size-focused optimizations and reduced inlining.
    /// Good for embedded systems or memory-constrained environments.
    Os,
    
    /// Aggressively optimize for size (-Oz)
    /// 
    /// Maximum binary size reduction, may sacrifice performance.
    /// Applies all size-reduction optimizations including aggressive function merging.
    /// Best for extremely memory-constrained environments.
    Oz,
}

impl OptimizationLevel {
    /// Convert string representation to OptimizationLevel
    /// 
    /// Supports multiple formats:
    /// - Numeric: "0", "1", "2", "3"
    /// - GCC-style: "O0", "O1", "O2", "O3", "Os", "Oz"
    /// - Lowercase: "o0", "o1", "o2", "o3", "os", "oz"
    /// - Descriptive: "none", "basic", "default", "aggressive", "size", "minsize"
    /// 
    /// # Examples
    /// 
    /// ```
    /// use cursed::common::optimization_level::OptimizationLevel;
    /// 
    /// assert_eq!(OptimizationLevel::from_str("O2").unwrap(), OptimizationLevel::O2);
    /// assert_eq!(OptimizationLevel::from_str("3").unwrap(), OptimizationLevel::O3);
    /// assert_eq!(OptimizationLevel::from_str("size").unwrap(), OptimizationLevel::Os);
    /// ```
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            // Numeric formats
            "0" => Ok(OptimizationLevel::O0),
            "1" => Ok(OptimizationLevel::O1),
            "2" => Ok(OptimizationLevel::O2),
            "3" => Ok(OptimizationLevel::O3),
            
            // GCC-style formats (case insensitive)
            "o0" => Ok(OptimizationLevel::O0),
            "o1" => Ok(OptimizationLevel::O1),
            "o2" => Ok(OptimizationLevel::O2),
            "o3" => Ok(OptimizationLevel::O3),
            "os" => Ok(OptimizationLevel::Os),
            "oz" => Ok(OptimizationLevel::Oz),
            
            // Descriptive formats
            "none" | "debug" => Ok(OptimizationLevel::O0),
            "basic" | "less" | "light" => Ok(OptimizationLevel::O1),
            "default" | "standard" | "normal" => Ok(OptimizationLevel::O2),
            "aggressive" | "max" | "maximum" => Ok(OptimizationLevel::O3),
            "size" => Ok(OptimizationLevel::Os),
            "minsize" | "minimum-size" | "min-size" => Ok(OptimizationLevel::Oz),
            
            // Alternative size formats
            "s" => Ok(OptimizationLevel::Os),
            "z" => Ok(OptimizationLevel::Oz),
            
            _ => Err(format!("Invalid optimization level: '{}'. Valid options: O0, O1, O2, O3, Os, Oz", s)),
        }
    }
    
    /// Convert OptimizationLevel to string representation
    /// 
    /// Returns the canonical GCC-style string representation.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use cursed::common::optimization_level::OptimizationLevel;
    /// 
    /// assert_eq!(OptimizationLevel::O2.as_str(), "O2");
    /// assert_eq!(OptimizationLevel::Os.as_str(), "Os");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            OptimizationLevel::O0 => "O0",
            OptimizationLevel::O1 => "O1",
            OptimizationLevel::O2 => "O2",
            OptimizationLevel::O3 => "O3",
            OptimizationLevel::Os => "Os",
            OptimizationLevel::Oz => "Oz",
        }
    }
    
    /// Get a human-readable description of the optimization level
    /// 
    /// # Examples
    /// 
    /// ```
    /// use cursed::common::optimization_level::OptimizationLevel;
    /// 
    /// assert!(OptimizationLevel::O0.description().contains("No optimization"));
    /// assert!(OptimizationLevel::O3.description().contains("Aggressive"));
    /// ```
    pub fn description(&self) -> &'static str {
        match self {
            OptimizationLevel::O0 => "No optimization (-O0): Fastest compilation, best for debugging",
            OptimizationLevel::O1 => "Basic optimization (-O1): Light optimization, good compilation speed",
            OptimizationLevel::O2 => "Standard optimization (-O2): Balanced performance and compilation time",
            OptimizationLevel::O3 => "Aggressive optimization (-O3): Maximum performance, slower compilation",
            OptimizationLevel::Os => "Size optimization (-Os): Optimize for binary size",
            OptimizationLevel::Oz => "Aggressive size optimization (-Oz): Minimize binary size",
        }
    }
    
    /// Convert to LLVM's inkwell OptimizationLevel
    /// 
    /// Maps CURSED optimization levels to the corresponding inkwell/LLVM levels.
    /// Note that LLVM doesn't have distinct size optimization levels, so Os and Oz
    /// are mapped to Default and Aggressive respectively with size-focused passes.
    pub fn to_inkwell_level(&self) -> inkwell::OptimizationLevel {
        match self {
            OptimizationLevel::O0 => inkwell::OptimizationLevel::O0,
            OptimizationLevel::O1 => inkwell::OptimizationLevel::O1,
            OptimizationLevel::O2 => inkwell::OptimizationLevel::O2,
            OptimizationLevel::O3 => inkwell::OptimizationLevel::O3,
            OptimizationLevel::Os => inkwell::OptimizationLevel::O2, // Size optimization uses default level with size passes
            OptimizationLevel::Oz => inkwell::OptimizationLevel::O3, // Aggressive size optimization
        }
    }
    
    /// Convert to numeric representation
    /// 
    /// Returns the numeric equivalent commonly used in compiler toolchains.
    /// Size optimization levels return their closest performance equivalent.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use cursed::common::optimization_level::OptimizationLevel;
    /// 
    /// assert_eq!(OptimizationLevel::O2.to_numeric(), 2);
    /// assert_eq!(OptimizationLevel::Os.to_numeric(), 2); // Os is similar to O2
    /// ```
    pub fn to_numeric(&self) -> u32 {
        match self {
            OptimizationLevel::O0 => 0,
            OptimizationLevel::O1 => 1,
            OptimizationLevel::O2 => 2,
            OptimizationLevel::O3 => 3,
            OptimizationLevel::Os => 2, // Size optimization is similar to O2 in terms of optimization passes
            OptimizationLevel::Oz => 3, // Aggressive size optimization is similar to O3
        }
    }
    
    /// Check if this optimization level prioritizes size reduction
    /// 
    /// Returns true for Os and Oz levels.
    pub fn is_size_optimized(&self) -> bool {
        matches!(self, OptimizationLevel::Os | OptimizationLevel::Oz)
    }
    
    /// Check if this optimization level is suitable for debugging
    /// 
    /// Returns true for O0 and O1 levels where debugging information is well-preserved.
    pub fn is_debug_friendly(&self) -> bool {
        matches!(self, OptimizationLevel::O0 | OptimizationLevel::O1)
    }
    
    /// Check if this optimization level uses aggressive optimizations
    /// 
    /// Returns true for O3 and Oz levels that may significantly change code structure.
    pub fn is_aggressive(&self) -> bool {
        matches!(self, OptimizationLevel::O3 | OptimizationLevel::Oz)
    }
    
    /// Get the recommended number of parallel compilation threads for this optimization level
    /// 
    /// Higher optimization levels benefit from more parallelization but also use more resources.
    pub fn recommended_parallel_threads(&self) -> usize {
        match self {
            OptimizationLevel::O0 => 1,      // No parallel for debug builds
            OptimizationLevel::O1 => 4,     // Light parallelization
            OptimizationLevel::O2 => 8,     // Moderate parallelization
            OptimizationLevel::O3 => 16,    // Heavy parallelization
            OptimizationLevel::Os => 6,     // Moderate for size optimization
            OptimizationLevel::Oz => 8,     // More for aggressive size optimization
        }
    }
    
    /// Get the estimated relative compilation time multiplier
    /// 
    /// Returns an approximate multiplier compared to O0 compilation time.
    /// Useful for progress estimation and resource planning.
    pub fn compilation_time_multiplier(&self) -> f64 {
        match self {
            OptimizationLevel::O0 => 1.0,   // Baseline
            OptimizationLevel::O1 => 1.5,   // 50% longer
            OptimizationLevel::O2 => 2.5,   // 2.5x longer
            OptimizationLevel::O3 => 4.0,   // 4x longer
            OptimizationLevel::Os => 2.0,   // 2x longer (less aggressive than O2)
            OptimizationLevel::Oz => 3.0,   // 3x longer (more passes for size)
        }
    }
    
    /// Create default optimization level for development builds
    pub fn development_default() -> Self {
        OptimizationLevel::O1
    }
    
    /// Create default optimization level for release builds
    pub fn release_default() -> Self {
        OptimizationLevel::O2
    }
    
    /// Create default optimization level for production builds
    pub fn production_default() -> Self {
        OptimizationLevel::O3
    }
}

impl Default for OptimizationLevel {
    /// Default optimization level is O2 (balanced performance and compilation time)
    fn default() -> Self {
        OptimizationLevel::O2
    }
}

impl Display for OptimizationLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for OptimizationLevel {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        OptimizationLevel::from_str(s)
    }
}

/// Compatibility aliases for legacy code
/// 
/// These are deprecated and should be replaced with the canonical enum variants.
#[deprecated(note = "Use OptimizationLevel::O0 instead")]
pub const NONE: OptimizationLevel = OptimizationLevel::O0;

#[deprecated(note = "Use OptimizationLevel::O1 instead")]
pub const LESS: OptimizationLevel = OptimizationLevel::O1;

#[deprecated(note = "Use OptimizationLevel::O2 instead")]
pub const DEFAULT: OptimizationLevel = OptimizationLevel::O2;

#[deprecated(note = "Use OptimizationLevel::O3 instead")]
pub const AGGRESSIVE: OptimizationLevel = OptimizationLevel::O3;

#[deprecated(note = "Use OptimizationLevel::Os instead")]
pub const SIZE: OptimizationLevel = OptimizationLevel::Os;

#[deprecated(note = "Use OptimizationLevel::Oz instead")]
pub const SIZE_AGGRESSIVE: OptimizationLevel = OptimizationLevel::Oz;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_numeric() {
        assert_eq!(OptimizationLevel::from_str("0").unwrap(), OptimizationLevel::O0);
        assert_eq!(OptimizationLevel::from_str("1").unwrap(), OptimizationLevel::O1);
        assert_eq!(OptimizationLevel::from_str("2").unwrap(), OptimizationLevel::O2);
        assert_eq!(OptimizationLevel::from_str("3").unwrap(), OptimizationLevel::O3);
    }

    #[test]
    fn test_from_str_gcc_style() {
        assert_eq!(OptimizationLevel::from_str("O0").unwrap(), OptimizationLevel::O0);
        assert_eq!(OptimizationLevel::from_str("O1").unwrap(), OptimizationLevel::O1);
        assert_eq!(OptimizationLevel::from_str("O2").unwrap(), OptimizationLevel::O2);
        assert_eq!(OptimizationLevel::from_str("O3").unwrap(), OptimizationLevel::O3);
        assert_eq!(OptimizationLevel::from_str("Os").unwrap(), OptimizationLevel::Os);
        assert_eq!(OptimizationLevel::from_str("Oz").unwrap(), OptimizationLevel::Oz);
    }

    #[test]
    fn test_from_str_case_insensitive() {
        assert_eq!(OptimizationLevel::from_str("o0").unwrap(), OptimizationLevel::O0);
        assert_eq!(OptimizationLevel::from_str("o3").unwrap(), OptimizationLevel::O3);
        assert_eq!(OptimizationLevel::from_str("os").unwrap(), OptimizationLevel::Os);
        assert_eq!(OptimizationLevel::from_str("oz").unwrap(), OptimizationLevel::Oz);
    }

    #[test]
    fn test_from_str_descriptive() {
        assert_eq!(OptimizationLevel::from_str("none").unwrap(), OptimizationLevel::O0);
        assert_eq!(OptimizationLevel::from_str("basic").unwrap(), OptimizationLevel::O1);
        assert_eq!(OptimizationLevel::from_str("default").unwrap(), OptimizationLevel::O2);
        assert_eq!(OptimizationLevel::from_str("aggressive").unwrap(), OptimizationLevel::O3);
        assert_eq!(OptimizationLevel::from_str("size").unwrap(), OptimizationLevel::Os);
        assert_eq!(OptimizationLevel::from_str("minsize").unwrap(), OptimizationLevel::Oz);
    }

    #[test]
    fn test_from_str_alternative_formats() {
        assert_eq!(OptimizationLevel::from_str("s").unwrap(), OptimizationLevel::Os);
        assert_eq!(OptimizationLevel::from_str("z").unwrap(), OptimizationLevel::Oz);
        assert_eq!(OptimizationLevel::from_str("debug").unwrap(), OptimizationLevel::O0);
        assert_eq!(OptimizationLevel::from_str("maximum").unwrap(), OptimizationLevel::O3);
    }

    #[test]
    fn test_from_str_invalid() {
        assert!(OptimizationLevel::from_str("O4").is_err());
        assert!(OptimizationLevel::from_str("invalid").is_err());
        assert!(OptimizationLevel::from_str("").is_err());
    }

    #[test]
    fn test_as_str() {
        assert_eq!(OptimizationLevel::O0.as_str(), "O0");
        assert_eq!(OptimizationLevel::O1.as_str(), "O1");
        assert_eq!(OptimizationLevel::O2.as_str(), "O2");
        assert_eq!(OptimizationLevel::O3.as_str(), "O3");
        assert_eq!(OptimizationLevel::Os.as_str(), "Os");
        assert_eq!(OptimizationLevel::Oz.as_str(), "Oz");
    }

    #[test]
    fn test_description() {
        assert!(OptimizationLevel::O0.description().contains("No optimization"));
        assert!(OptimizationLevel::O1.description().contains("Basic"));
        assert!(OptimizationLevel::O2.description().contains("Standard"));
        assert!(OptimizationLevel::O3.description().contains("Aggressive"));
        assert!(OptimizationLevel::Os.description().contains("Size"));
        assert!(OptimizationLevel::Oz.description().contains("size"));
    }

    #[test]
    fn test_to_numeric() {
        assert_eq!(OptimizationLevel::O0.to_numeric(), 0);
        assert_eq!(OptimizationLevel::O1.to_numeric(), 1);
        assert_eq!(OptimizationLevel::O2.to_numeric(), 2);
        assert_eq!(OptimizationLevel::O3.to_numeric(), 3);
        assert_eq!(OptimizationLevel::Os.to_numeric(), 2);
        assert_eq!(OptimizationLevel::Oz.to_numeric(), 3);
    }

    #[test]
    fn test_is_size_optimized() {
        assert!(!OptimizationLevel::O0.is_size_optimized());
        assert!(!OptimizationLevel::O1.is_size_optimized());
        assert!(!OptimizationLevel::O2.is_size_optimized());
        assert!(!OptimizationLevel::O3.is_size_optimized());
        assert!(OptimizationLevel::Os.is_size_optimized());
        assert!(OptimizationLevel::Oz.is_size_optimized());
    }

    #[test]
    fn test_is_debug_friendly() {
        assert!(OptimizationLevel::O0.is_debug_friendly());
        assert!(OptimizationLevel::O1.is_debug_friendly());
        assert!(!OptimizationLevel::O2.is_debug_friendly());
        assert!(!OptimizationLevel::O3.is_debug_friendly());
        assert!(!OptimizationLevel::Os.is_debug_friendly());
        assert!(!OptimizationLevel::Oz.is_debug_friendly());
    }

    #[test]
    fn test_is_aggressive() {
        assert!(!OptimizationLevel::O0.is_aggressive());
        assert!(!OptimizationLevel::O1.is_aggressive());
        assert!(!OptimizationLevel::O2.is_aggressive());
        assert!(OptimizationLevel::O3.is_aggressive());
        assert!(!OptimizationLevel::Os.is_aggressive());
        assert!(OptimizationLevel::Oz.is_aggressive());
    }

    #[test]
    fn test_recommended_parallel_threads() {
        assert_eq!(OptimizationLevel::O0.recommended_parallel_threads(), 1);
        assert_eq!(OptimizationLevel::O1.recommended_parallel_threads(), 4);
        assert_eq!(OptimizationLevel::O2.recommended_parallel_threads(), 8);
        assert_eq!(OptimizationLevel::O3.recommended_parallel_threads(), 16);
        assert_eq!(OptimizationLevel::Os.recommended_parallel_threads(), 6);
        assert_eq!(OptimizationLevel::Oz.recommended_parallel_threads(), 8);
    }

    #[test]
    fn test_compilation_time_multiplier() {
        assert_eq!(OptimizationLevel::O0.compilation_time_multiplier(), 1.0);
        assert_eq!(OptimizationLevel::O1.compilation_time_multiplier(), 1.5);
        assert_eq!(OptimizationLevel::O2.compilation_time_multiplier(), 2.5);
        assert_eq!(OptimizationLevel::O3.compilation_time_multiplier(), 4.0);
        assert_eq!(OptimizationLevel::Os.compilation_time_multiplier(), 2.0);
        assert_eq!(OptimizationLevel::Oz.compilation_time_multiplier(), 3.0);
    }

    #[test]
    fn test_defaults() {
        assert_eq!(OptimizationLevel::development_default(), OptimizationLevel::O1);
        assert_eq!(OptimizationLevel::release_default(), OptimizationLevel::O2);
        assert_eq!(OptimizationLevel::production_default(), OptimizationLevel::O3);
        assert_eq!(OptimizationLevel::default(), OptimizationLevel::O2);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", OptimizationLevel::O2), "O2");
        assert_eq!(format!("{}", OptimizationLevel::Os), "Os");
    }

    #[test]
    fn test_from_str_trait() {
        let level: OptimizationLevel = "O3".parse().unwrap();
        assert_eq!(level, OptimizationLevel::O3);
        
        let level: Result<OptimizationLevel, _> = "invalid".parse();
        assert!(level.is_err());
    }

    #[test]
    fn test_serialization() {
        let level = OptimizationLevel::O3;
        let json = serde_json::to_string(&level).unwrap();
        let deserialized: OptimizationLevel = serde_json::from_str(&json).unwrap();
        assert_eq!(level, deserialized);
    }

    #[test]
    fn test_hash_and_equality() {
        use std::collections::HashMap;
        
        let mut map = HashMap::new();
        map.insert(OptimizationLevel::O2, "standard");
        map.insert(OptimizationLevel::O3, "aggressive");
        
        assert_eq!(map.get(&OptimizationLevel::O2), Some(&"standard"));
        assert_eq!(map.get(&OptimizationLevel::O3), Some(&"aggressive"));
    }

    #[test]
    fn test_inkwell_conversion() {
        use inkwell::OptimizationLevel as InkwellLevel;
        
        assert_eq!(OptimizationLevel::O0.to_inkwell_level(), InkwellLevel::None);
        assert_eq!(OptimizationLevel::O1.to_inkwell_level(), InkwellLevel::Less);
        assert_eq!(OptimizationLevel::O2.to_inkwell_level(), InkwellLevel::Default);
        assert_eq!(OptimizationLevel::O3.to_inkwell_level(), InkwellLevel::Aggressive);
        assert_eq!(OptimizationLevel::Os.to_inkwell_level(), InkwellLevel::Default);
        assert_eq!(OptimizationLevel::Oz.to_inkwell_level(), InkwellLevel::Aggressive);
    }
}
