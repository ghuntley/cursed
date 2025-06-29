//! Mathematical functionality for utilities

use crate::error::CursedError;

/// Result type for math operations
pub type MathResult<T> = Result<T, CursedError>;

/// Mathematical operations
pub struct MathProcessor {
    precision: f64,
}

impl MathProcessor {
    /// Create a new math processor
    pub fn new() -> Self {
        Self {
            precision: 1e-10,
        }
    }
    
    /// Set precision for floating point operations
    pub fn precision(mut self, precision: f64) -> Self {
        self.precision = precision;
        self
    }
    
    /// Add two numbers
    pub fn add(&self, a: f64, b: f64) -> f64 {
        a + b
    }
    
    /// Subtract two numbers
    pub fn subtract(&self, a: f64, b: f64) -> f64 {
        a - b
    }
    
    /// Multiply two numbers
    pub fn multiply(&self, a: f64, b: f64) -> f64 {
        a * b
    }
    
    /// Divide two numbers
    pub fn divide(&self, a: f64, b: f64) -> MathResult<f64> {
        if b.abs() < self.precision {
            Err(CursedError::runtime_error("Division by zero"))
        } else {
            Ok(a / b)
        }
    }
    
    /// Calculate power
    pub fn power(&self, base: f64, exponent: f64) -> f64 {
        base.powf(exponent)
    }
    
    /// Calculate square root
    pub fn sqrt(&self, x: f64) -> MathResult<f64> {
        if x < 0.0 {
            Err(CursedError::runtime_error("Square root of negative number"))
        } else {
            Ok(x.sqrt())
        }
    }
}

impl Default for MathProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize math processing
pub fn init_utilities() -> MathResult<()> {
    let processor = MathProcessor::new();
    let result = processor.add(2.0, 3.0);
    if (result - 5.0).abs() > processor.precision {
        return Err(CursedError::runtime_error("Math test failed"));
    }
    println!("🔢 Math processing (utilities) initialized");
    Ok(())
}

/// Test math functionality
pub fn test_utilities() -> MathResult<()> {
    let processor = MathProcessor::new();
    let result = processor.multiply(6.0, 7.0);
    if (result - 42.0).abs() > processor.precision {
        return Err(CursedError::runtime_error("Math test failed"));
    }
    Ok(())
}
