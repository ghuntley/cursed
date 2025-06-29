//! Unit tests for CURSED net standard library

use cursed::stdlib::net;

#[test]
fn test_network_statistics_tracking() {
    // Reset statistics before test
    net::reset_network_statistics();
    
    let initial_stats = net::get_network_statistics();
    assert_eq!(initial_stats.active_connections, 0);
    assert_eq!(initial_stats.total_bytes_sent, 0);
    assert_eq!(initial_stats.total_bytes_received, 0);
    assert_eq!(initial_stats.dns_queries, 0);
    assert_eq!(initial_stats.failed_connections, 0);
}

#[test]
fn test_connection_tracking() {
    net::reset_network_statistics();
    
    // Track connections
    net::track_connection_opened();
    net::track_connection_opened();
    
    let stats = net::get_network_statistics();
    assert_eq!(stats.active_connections, 2);
    
    // Track closed connection
    net::track_connection_closed();
    
    let stats = net::get_network_statistics();
    assert_eq!(stats.active_connections, 1);
    
    // Track failed connection
    net::track_connection_failed();
    
    let stats = net::get_network_statistics();
    assert_eq!(stats.failed_connections, 1);
}

#[test]
fn test_bytes_tracking() {
    net::reset_network_statistics();
    
    net::track_bytes_sent(1024);
    net::track_bytes_received(512);
    
    let stats = net::get_network_statistics();
    assert_eq!(stats.total_bytes_sent, 1024);
    assert_eq!(stats.total_bytes_received, 512);
    
    // Track more bytes
    net::track_bytes_sent(256);
    net::track_bytes_received(128);
    
    let stats = net::get_network_statistics();
    assert_eq!(stats.total_bytes_sent, 1280);
    assert_eq!(stats.total_bytes_received, 640);
}

#[test]
fn test_dns_query_tracking() {
    net::reset_network_statistics();
    
    net::track_dns_query();
    net::track_dns_query();
    net::track_dns_query();
    
    let stats = net::get_network_statistics();
    assert_eq!(stats.dns_queries, 3);
}

#[test]
fn test_network_module_initialization() {
    // Test that initialization doesn't panic
    assert!(net::initialize().is_ok());
    assert!(net::shutdown().is_ok());
}
