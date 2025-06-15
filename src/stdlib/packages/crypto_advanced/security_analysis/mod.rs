/// fr fr Security Analysis Module - Comprehensive Cryptographic Security Testing
/// 
/// This module provides state-of-the-art security analysis capabilities including:
/// - Timing attack detection and side-channel analysis
/// - Entropy validation and randomness quality testing  
/// - Cryptographic parameter verification
/// - Vulnerability scanning and security metrics
/// - Real-time security monitoring and reporting
/// 
/// All implementations follow cryptographic best practices with production-ready security.

// Core analysis modules
pub mod timing_analysis;
pub mod side_channel;
pub mod entropy_validation;
pub mod parameter_verification;
pub mod vulnerability_scanner;

// Re-export all public types for convenience
pub use timing_analysis::*;
pub use side_channel::*;
pub use entropy_validation::*;
pub use parameter_verification::*;
pub use vulnerability_scanner::*;
