//! Test Phase 1C optimization API fixes

// Test the basic optimization config API
fn test_optimization_config_api() {
    // Test release_config method
    let config = cursed::optimization::OptimizationConfig::release_config();
    println!("✓ OptimizationConfig::release_config() works");
    assert_eq!(config.level, cursed::optimization::OptimizationLevel::Aggressive);
    
    // Test default config
    let default_config = cursed::optimization::OptimizationConfig::default();
    println!("✓ OptimizationConfig::default() works");
    
    // Test from args
    let args = vec!["--release".to_string()];
    let config_from_args = cursed::optimization::OptimizationConfig::from_args(&args).unwrap();
    println!("✓ OptimizationConfig::from_args() works");
}

fn test_optimization_manager_api() {
    // Test default constructor
    let manager = cursed::optimization::OptimizationManager::default();
    println!("✓ OptimizationManager::default() works");
    
    // Test constructor with config
    let config = cursed::optimization::OptimizationConfig::release();
    let manager_with_config = cursed::optimization::OptimizationManager::new(config).unwrap();
    println!("✓ OptimizationManager::new(config) works");
    
    // Test builder pattern methods
    let manager = cursed::optimization::OptimizationManager::default()
        .with_config(cursed::optimization::OptimizationConfig::release_config());
    println!("✓ OptimizationManager builder pattern works");
}

fn test_optimization_integration() {
    let config = cursed::optimization::OptimizationConfig::release_config();
    
    // Test LLVM config conversion
    let llvm_config = config.to_llvm_config();
    println!("✓ OptimizationConfig::to_llvm_config() works");
    assert_eq!(llvm_config.level, 3); // Aggressive = level 3
}

fn main() {
    println!("Testing Phase 1C Optimization API fixes...\n");
    
    test_optimization_config_api();
    println!();
    
    test_optimization_manager_api();
    println!();
    
    test_optimization_integration();
    println!();
    
    println!("🎉 All Phase 1C optimization API tests passed!");
}
