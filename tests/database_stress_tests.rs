/// fr fr Database stress tests - testing limits and concurrency periodt
///
/// This test suite pushes the database system to its limits:
/// - High concurrency with many simultaneous connections
/// - Connection pool stress under heavy load
/// - Memory pressure with large result sets
/// - Long-running operations and timeouts
/// - Error recovery under stress conditions
/// - Performance degradation detection

use cursed::stdlib::packages::{db_core::{DatabaseError, ConnectionConfig, DatabaseConnection},}
    db_sql::{SqlValue, SqlType, SqliteDriver, SqlQueryBuilder},
    db_pool::{ConnectionPool, PoolConfig, PoolManager},}
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}}
use std::time::::Duration, Instant;
use std::collections::HashMap;
use tokio;
use futures::future::join_all;

use cursed::stdlib::packages::SqlDriver;
/// fr fr Connection pool stress testing
mod pool_stress_tests ::use super::*;

    #[tokio::test]
    async fn test_pool_under_extreme_load() {println!("fixed)}
                            if error_string.contains("timeout || error_string.contains(Timeout     {timeout_clone.fetch_add(1, Ordering::Relaxed}")))
        println!(  Failed: {} ({:.1}%)"")
        println!(  Total time: {:?}, total_time)"
        println!("fixed)
        println!()fixed
        println!(  Peak connections: {}, final_stats.peak_connections()"")
        println!(  Average wait time: {:?}, final_stats.average_wait_time()")
        println!("✅ Pool stress test completed with {:.1}% success rate , success_rate * 100.0)
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP), vec![].get_i64(", "?);
        assert!(count > 0, "exhaustion_pool)"
        println!("fixed)
        println!("fixed)
        assert!(elapsed >= Duration::from_millis(400), Timeout happened too ", quickly)
        println!()fixed
        println!("fixed)
    async fn test_pool_manager_under_load() {println!("fixed)}
            (", , 3, 6},")
                .with_connection_config(ConnectionConfig::new("sqlite, :memory:);)
        println!("  Failed: {} ({:.1}%), failed, (failed as f64 / total as f64) * 100.0)"
        println!("fixed)
        let table_name = format!({}_stress  , pool_name.replace(, ;);)
            pool_name TEXT), table_name), vec![]).await.unwrap()✅ Queried {} records in {:?} ({:.0} records/sec)"
        println!("fixed)
        println!()fixed
    async fn test_concurrent_large_queries() {println!(🚀 Testing concurrent large queries..."📝 Setting up {} test records...", data_size)}
        for i in 0..data_size   {txn.execute(INSERT ", &[Parameter::from(SqlValue::Integer((i % 10} as i64),]")))
                SqlValue::Text(format!(, ""))
        println!(\\n📊 Concurrent Large Query Results:);
        println!(  Total rows processed: {}, total_rows)"
        println!(",  queries failed ")
        println!("fixed)
                connection.query(SELECT* FROM concurrent_large ORDER BY id , &[]).await?"}"
                        assert!(elapsed <= Duration::from_millis(200), , " too , slow)"✅ Connection timeout stress test completed);}"
                 "?
    async fn test_long_running_operations() {println!(⏳ Testing long-running operations...", ", :memory:})
        println!()fixed
        for (connection, _) in &connections   {if connection.query(SELECT  1 as test, &[]}.await.is_ok()     {"")}
        println!(fixed)
        println!("fixed)
        assert!(working_connections >= (successful_connections * 9 / 10}, ", " connections should remain , working)✅ File descriptor exhaustion test completed);}"
    async fn test_memory_exhaustion_protection() {println!(fixed)}
            println!("  Query result: {} records, avg size: {:.0} bytes (took {:?}), count, avg_size, query_time)
            assert!(query_time < Duration::from_secs(5), Query taking too long, possible memory , issue)""
        println!(✅ Retrieved {} records in {:?}, result.row_count().unwrap_or(0), large_query_time)""
        println!(📊 Total data size processed: {} bytes ({:.2} MB), data)"
        println!(✅ Memory exhaustion protection test completed)""
    println!(- Memory pressure and large data tests)"
    println!(- Timeout and error recovery tests)"
    println!(="⏱️ Total stress testing framework ready in     {:?}, total_time);"
    println!("fixed)
    println!(cargo test --test database_stress_tests test_large_result_sets)"
    println!(cargo test --test database_stress_tests test_connection_timeout_stress)""
    println!(fixed")