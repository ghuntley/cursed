// Optimization level definitions for the CURSED compiler
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OptimizationLevel {
    // Development and debugging
    O0,     // No optimization
    O1,     // Basic optimization
    O2,     // Standard optimization
    O3,     // Aggressive optimization
    
    // Size optimization
    Os,     // Optimize for size
    Oz,     // Optimize for size more aggressively
    
    // Special levels
    Debug,
    Release,
    
    // Compatibility aliases
    None,
    Less,
    Default,
    Aggressive,
    Size,
    SizeAggressive,
    
    // Advanced levels
    Basic,
    Production,
    Speed,
}

impl OptimizationLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            OptimizationLevel::O0 => "O0",
            OptimizationLevel::O1 => "O1",
            OptimizationLevel::O2 => "O2",
            OptimizationLevel::O3 => "O3",
            OptimizationLevel::Os => "Os",
            OptimizationLevel::Oz => "Oz",
            OptimizationLevel::Debug => "debug",
            OptimizationLevel::Release => "release",
            OptimizationLevel::None => "none",
            OptimizationLevel::Less => "less",
            OptimizationLevel::Default => "default",
            OptimizationLevel::Aggressive => "aggressive",
            OptimizationLevel::Size => "size",
            OptimizationLevel::SizeAggressive => "size-aggressive",
            OptimizationLevel::Basic => "basic",
            OptimizationLevel::Production => "production",
            OptimizationLevel::Speed => "speed",
        }
    }
    
    pub fn to_llvm_level(&self) -> u32 {
        match self {
            OptimizationLevel::O0 | OptimizationLevel::None | OptimizationLevel::Debug => 0,
            OptimizationLevel::O1 | OptimizationLevel::Less | OptimizationLevel::Basic => 1,
            OptimizationLevel::O2 | OptimizationLevel::Default | OptimizationLevel::Release => 2,
            OptimizationLevel::O3 | OptimizationLevel::Aggressive | OptimizationLevel::Production | OptimizationLevel::Speed => 3,
            OptimizationLevel::Os | OptimizationLevel::Size => 2,
            OptimizationLevel::Oz | OptimizationLevel::SizeAggressive => 3,
        }
    }
    
    pub fn from_u32(level: u32) -> Self {
        match level {
            0 => OptimizationLevel::O0,
            1 => OptimizationLevel::O1,
            2 => OptimizationLevel::O2,
            3 => OptimizationLevel::O3,
            _ => OptimizationLevel::O2,
        }
    }
    
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "o0" | "0" | "none" | "debug" => Ok(OptimizationLevel::O0),
            "o1" | "1" | "less" | "basic" => Ok(OptimizationLevel::O1),
            "o2" | "2" | "default" | "release" => Ok(OptimizationLevel::O2),
            "o3" | "3" | "aggressive" | "production" | "speed" => Ok(OptimizationLevel::O3),
            "os" | "size" => Ok(OptimizationLevel::Os),
            "oz" | "size-aggressive" => Ok(OptimizationLevel::Oz),
            _ => Err(format!("Invalid optimization level: {}", s)),
        }
    }
    
    pub fn compilation_time_multiplier(&self) -> f64 {
        match self {
            OptimizationLevel::O0 | OptimizationLevel::None | OptimizationLevel::Debug => 1.0,
            OptimizationLevel::O1 | OptimizationLevel::Less | OptimizationLevel::Basic => 1.5,
            OptimizationLevel::O2 | OptimizationLevel::Default | OptimizationLevel::Release => 2.0,
            OptimizationLevel::O3 | OptimizationLevel::Aggressive | OptimizationLevel::Production => 3.0,
            OptimizationLevel::Os | OptimizationLevel::Size => 2.5,
            OptimizationLevel::Oz | OptimizationLevel::SizeAggressive => 3.5,
            OptimizationLevel::Speed => 2.8,
        }
    }
    
    pub fn recommended_parallel_threads(&self) -> usize {
        match self {
            OptimizationLevel::O0 | OptimizationLevel::None | OptimizationLevel::Debug => 1,
            OptimizationLevel::O1 | OptimizationLevel::Less | OptimizationLevel::Basic => 2,
            OptimizationLevel::O2 | OptimizationLevel::Default | OptimizationLevel::Release => 4,
            OptimizationLevel::O3 | OptimizationLevel::Aggressive | OptimizationLevel::Production => 8,
            OptimizationLevel::Os | OptimizationLevel::Size => 4,
            OptimizationLevel::Oz | OptimizationLevel::SizeAggressive => 6,
            OptimizationLevel::Speed => 8,
        }
    }
}

impl fmt::Display for OptimizationLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for OptimizationLevel {
    fn default() -> Self {
        OptimizationLevel::O2
    }
}

impl std::str::FromStr for OptimizationLevel {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}

impl PartialOrd for OptimizationLevel {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OptimizationLevel {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_llvm_level().cmp(&other.to_llvm_level())
    }
}

