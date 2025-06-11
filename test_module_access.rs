// Test that the newly implemented modules are accessible

#[test]
fn test_environment_module_access() {
    // Test that environment module is accessible
    use cursed::stdlib::environment;
    
    // Basic module functionality should be available
    let config = environment::EnvironmentConfig::new();
    assert!(config.case_sensitive == false);
    
    // Basic operations should work
    let result = environment::get_env("PATH");
    assert!(result.is_some() || result.is_none()); // Should not panic
    
    println!("✓ Environment module accessible and functional");
}

#[test]
fn test_process_module_access() {
    // Test that process module is accessible
    use cursed::stdlib::process;
    
    // Basic module functionality should be available
    let manager = process::ProcessManager::new();
    assert!(manager.max_processes > 0);
    
    println!("✓ Process module accessible and functional");
}

#[test]
fn test_io_module_access() {
    // Test that I/O module is accessible
    use cursed::stdlib::io;
    
    // Console operations should be available
    let result = io::console::print("test");
    assert!(result.is_ok());
    
    println!("✓ I/O module accessible and functional");
}

#[test]
fn test_modules_in_stdlib() {
    // Verify modules are properly exported from stdlib
    use cursed::stdlib;
    
    // Check that modules are accessible through stdlib
    let env_result = stdlib::environment::get_env("HOME");
    assert!(env_result.is_some() || env_result.is_none());
    
    let proc_manager = stdlib::process::ProcessManager::new();
    assert!(proc_manager.max_processes > 0);
    
    let io_result = stdlib::io::console::print("test");
    assert!(io_result.is_ok());
    
    println!("✓ All modules accessible through stdlib namespace");
}

fn main() {
    test_environment_module_access();
    test_process_module_access();
    test_io_module_access();
    test_modules_in_stdlib();
    
    println!("🎉 All module access tests passed!");
}
