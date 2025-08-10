//! CURSED Test Runner
//! 
//! Temporarily disabled to focus on composite literal implementation

use crate::error::CursedError;

#[derive(Debug, Clone)]
pub struct TestConfig {
    pub test_dir: String,
    pub pattern: String,
    pub filter: Option<String>,
    pub parallel: bool,
    pub verbose: bool,
    pub fail_fast: bool,
    pub timeout: u64,
    pub format: String,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            test_dir: "stdlib".to_string(),
            pattern: "test_*.csd".to_string(),
            filter: None,
            parallel: false,
            verbose: false,
            fail_fast: false,
            timeout: 30,
            format: "text".to_string(),
        }
    }
}

// Test runner temporarily commented out for compilation fix
// Will be re-enabled in future update
