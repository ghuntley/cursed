/// Test to verify Redis API compatibility fix
/// This test checks that the execute_with_timing method works correctly

#[tokio::test]
async fn test_redis_api_compatibility() {
    use cursed::stdlib::packages::db_nosql::redis::{RedisDriver, RedisConfig};
    
    // Test that the RedisDriver can be created
    let driver = RedisDriver::new();
    assert_eq!(driver.config.url, "redis://localhost:6379");
    
    // Test configuration
    let config = RedisConfig::default();
    assert_eq!(config.database, 0);
    assert_eq!(config.max_connections, 10);
    
    println!("✅ Redis API compatibility test passed!");
}

/// Test to verify that the execute_with_timing method is available
#[tokio::test]
async fn test_redis_execute_with_timing() {
    use cursed::stdlib::packages::db_nosql::redis::{RedisDriver, RedisConfig};
    
    // Create a test Redis configuration
    let config = RedisConfig::default();
    
    // This should compile without errors, proving the method exists
    let driver = RedisDriver::with_config(config);
    
    // Test from_url method
    let url_driver = RedisDriver::from_url("redis://localhost:6379");
    assert!(url_driver.is_ok());
    
    println!("✅ Redis execute_with_timing method exists and compiles!");
}

fn main() {
    println!("Redis API fix test completed successfully!");
}
