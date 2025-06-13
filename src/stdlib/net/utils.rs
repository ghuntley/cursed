/// Network utilities for CURSED networking
/// 
/// This module provides various network utility functions including
/// port scanning, ping, network diagnostics, and validation utilities.

use std::net::IpAddr;
use std::time::{Duration, Instant};
use std::thread;
use crate::stdlib::net::error::{NetError, NetResult, general_error, timeout_error};
use crate::stdlib::net::socket::{TcpSocket, UdpSocket};
use crate::stdlib::net::dns::resolve_hostname;

/// Check if a port is available (not in use)
pub fn is_port_available(port: u16) -> bool {
    is_port_available_on_host("127.0.0.1", port)
}

/// Check if a port is available on a specific host
pub fn is_port_available_on_host(host: &str, port: u16) -> bool {
    let addr = format!("{}:{}", host, port);
    TcpSocket::connect_timeout(&addr, Duration::from_millis(100)).is_err()
}

/// Scan ports on a host within a given range
pub fn scan_ports(host: &str, start_port: u16, end_port: u16) -> NetResult<Vec<u16>> {
    let mut open_ports = Vec::new();
    
    for port in start_port..=end_port {
        let addr = format!("{}:{}", host, port);
        if let Ok(socket) = TcpSocket::connect_timeout(&addr, Duration::from_millis(100)) {
            open_ports.push(port);
            let _ = socket.close();
        }
    }
    
    Ok(open_ports)
}

/// Scan ports with timeout and parallelism
pub fn scan_ports_parallel(host: &str, ports: &[u16], timeout: Duration, max_threads: usize) -> NetResult<Vec<u16>> {
    use std::sync::{Arc, Mutex};
    use std::thread;
    
    let open_ports = Arc::new(Mutex::new(Vec::new()));
    let host = host.to_string();
    let chunk_size = (ports.len() + max_threads - 1) / max_threads;
    
    let mut handles = Vec::new();
    
    for chunk in ports.chunks(chunk_size) {
        let host = host.clone();
        let ports = chunk.to_vec();
        let open_ports = Arc::clone(&open_ports);
        
        let handle = thread::spawn(move || {
            for port in ports {
                let addr = format!("{}:{}", host, port);
                if let Ok(socket) = TcpSocket::connect_timeout(&addr, timeout) {
                    open_ports.lock().unwrap().push(port);
                    let _ = socket.close();
                }
            }
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().map_err(|_| general_error("Thread join failed"))?;
    }
    
    let mut result = open_ports.lock().unwrap().clone();
    result.sort();
    Ok(result)
}

/// Ping a host using TCP connection attempt
pub fn ping_host(host: &str) -> NetResult<Duration> {
    ping_host_with_timeout(host, Duration::from_secs(5))
}

/// Ping a host with custom timeout
pub fn ping_host_with_timeout(host: &str, timeout: Duration) -> NetResult<Duration> {
    let start = Instant::now();
    
    // Try to resolve hostname first
    let ips = resolve_hostname(host)?;
    if ips.is_empty() {
        return Err(general_error("Host not found"));
    }
    
    // Try to connect to port 80 (HTTP) as a ping substitute
    let addr = format!("{}:80", ips[0]);
    match TcpSocket::connect_timeout(&addr, timeout) {
        Ok(socket) => {
            let duration = start.elapsed();
            let _ = socket.close();
            Ok(duration)
        },
        Err(_) => {
            // Try port 443 (HTTPS) as fallback
            let addr = format!("{}:443", ips[0]);
            match TcpSocket::connect_timeout(&addr, timeout) {
                Ok(socket) => {
                    let duration = start.elapsed();
                    let _ = socket.close();
                    Ok(duration)
                },
                Err(_) => Err(timeout_error("Host unreachable")),
            }
        }
    }
}

/// Trace route to a destination (simplified implementation)
pub fn trace_route(host: &str) -> NetResult<Vec<String>> {
    // This is a simplified implementation
    // A real implementation would use ICMP or UDP with increasing TTL values
    let ips = resolve_hostname(host)?;
    if ips.is_empty() {
        return Err(general_error("Host not found"));
    }
    
    // For now, just return the resolved IP as the only hop
    Ok(vec![ips[0].to_string()])
}

/// Get public IP address by connecting to external service
pub fn get_public_ip() -> NetResult<crate::stdlib::net::address::IpAddr> {
    // Try connecting to known external services to determine public IP
    let services = [
        "8.8.8.8:53",       // Google DNS
        "1.1.1.1:53",       // Cloudflare DNS
        "208.67.222.222:53", // OpenDNS
    ];
    
    for service in &services {
        if let Ok(socket) = TcpSocket::connect_timeout(service, Duration::from_secs(5)) {
            if let Some(local_addr) = socket.local_addr() {
                let _ = socket.close();
                return Ok(local_addr.ip());
            }
        }
    }
    
    Err(general_error("Could not determine public IP"))
}

/// Get local IP addresses
pub fn get_local_ips() -> NetResult<Vec<crate::stdlib::net::address::IpAddr>> {
    use crate::stdlib::net::interfaces::list_interfaces;
    
    let interfaces = list_interfaces()?;
    let mut ips = Vec::new();
    
    for interface in interfaces {
        if interface.is_active() && !interface.is_loopback() {
            ips.extend(interface.ip_addresses);
        }
    }
    
    Ok(ips)
}

/// Validate email address format
pub fn validate_email(email: &str) -> bool {
    // Basic email validation (simplified)
    if email.is_empty() || email.len() > 254 {
        return false;
    }
    
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }
    
    let local = parts[0];
    let domain = parts[1];
    
    // Check local part
    if local.is_empty() || local.len() > 64 {
        return false;
    }
    
    // Check domain part
    if domain.is_empty() || domain.len() > 253 {
        return false;
    }
    
    // Must contain at least one dot
    if !domain.contains('.') {
        return false;
    }
    
    // Basic character validation
    let valid_chars = |c: char| c.is_ascii_alphanumeric() || ".-_+".contains(c);
    local.chars().all(valid_chars) && domain.chars().all(|c| c.is_ascii_alphanumeric() || ".-".contains(c))
}

/// Validate URL format
pub fn validate_url(url: &str) -> bool {
    if url.is_empty() || url.len() > 2048 {
        return false;
    }
    
    // Must start with http:// or https://
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return false;
    }
    
    // Basic structure validation
    let without_scheme = if url.starts_with("https://") {
        &url[8..]
    } else {
        &url[7..]
    };
    
    if without_scheme.is_empty() {
        return false;
    }
    
    // Must contain a host part
    let parts: Vec<&str> = without_scheme.split('/').collect();
    if parts.is_empty() || parts[0].is_empty() {
        return false;
    }
    
    // Validate host part
    let host = parts[0];
    if host.contains(' ') {
        return false;
    }
    
    true
}

/// Parse URL into components
pub fn parse_url(url: &str) -> NetResult<UrlComponents> {
    if !validate_url(url) {
        return Err(general_error("Invalid URL format"));
    }
    
    let is_https = url.starts_with("https://");
    let scheme = if is_https { "https" } else { "http" };
    let default_port = if is_https { 443 } else { 80 };
    
    let without_scheme = if is_https {
        &url[8..]
    } else {
        &url[7..]
    };
    
    let (host_port, path) = if let Some(slash_pos) = without_scheme.find('/') {
        (&without_scheme[..slash_pos], &without_scheme[slash_pos..])
    } else {
        (without_scheme, "/")
    };
    
    let (host, port) = if let Some(colon_pos) = host_port.rfind(':') {
        let host = &host_port[..colon_pos];
        let port_str = &host_port[colon_pos + 1..];
        let port = port_str.parse::<u16>()
            .map_err(|_| general_error("Invalid port in URL"))?;
        (host.to_string(), port)
    } else {
        (host_port.to_string(), default_port)
    };
    
    let (path, query, fragment) = parse_url_path(path);
    
    Ok(UrlComponents {
        scheme: scheme.to_string(),
        host,
        port,
        path,
        query,
        fragment,
    })
}

fn parse_url_path(path: &str) -> (String, Option<String>, Option<String>) {
    let (path_and_query, fragment) = if let Some(hash_pos) = path.find('#') {
        (&path[..hash_pos], Some(path[hash_pos + 1..].to_string()))
    } else {
        (path, None)
    };
    
    let (path, query) = if let Some(question_pos) = path_and_query.find('?') {
        (&path_and_query[..question_pos], Some(path_and_query[question_pos + 1..].to_string()))
    } else {
        (path_and_query, None)
    };
    
    (path.to_string(), query, fragment)
}

/// URL components structure
#[derive(Debug, Clone)]
pub struct UrlComponents {
    pub scheme: String,
    pub host: String,
    pub port: u16,
    pub path: String,
    pub query: Option<String>,
    pub fragment: Option<String>,
}

/// Format bandwidth in human-readable format
pub fn format_bandwidth(bytes_per_second: f64) -> String {
    const UNITS: &[&str] = &["B/s", "KB/s", "MB/s", "GB/s", "TB/s"];
    let mut value = bytes_per_second;
    let mut unit_index = 0;
    
    while value >= 1024.0 && unit_index < UNITS.len() - 1 {
        value /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{:.0} {}", value, UNITS[unit_index])
    } else {
        format!("{:.2} {}", value, UNITS[unit_index])
    }
}

/// Network diagnostics information
#[derive(Debug, Clone)]
pub struct NetworkDiagnostics {
    pub local_ips: Vec<crate::stdlib::net::address::IpAddr>,
    pub public_ip: Option<crate::stdlib::net::address::IpAddr>,
    pub default_gateway: Option<crate::stdlib::net::address::IpAddr>,
    pub dns_servers: Vec<crate::stdlib::net::address::IpAddr>,
    pub active_connections: usize,
    pub network_interfaces: usize,
}

/// Gather network diagnostics
pub fn network_diagnostics() -> NetResult<NetworkDiagnostics> {
    let local_ips = get_local_ips().unwrap_or_default();
    let public_ip = get_public_ip().ok();
    
    // Get network interface information
    use crate::stdlib::net::interfaces::{list_interfaces, get_default_interface};
    let interfaces = list_interfaces().unwrap_or_default();
    let default_interface = get_default_interface().unwrap_or_default();
    
    let default_gateway = default_interface.and_then(|iface| iface.gateway);
    let dns_servers = default_interface.map(|iface| iface.dns_servers).unwrap_or_default();
    
    Ok(NetworkDiagnostics {
        local_ips,
        public_ip,
        default_gateway,
        dns_servers,
        active_connections: 0, // TODO: Implement connection counting
        network_interfaces: interfaces.len(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_port_availability() {
        // Port 0 should always be available (OS will assign)
        // We can't test specific ports as they might be in use
        assert!(is_port_available(0));
    }

    #[test]
    fn test_email_validation() {
        assert!(validate_email("user@example.com"));
        assert!(validate_email("test.email+tag@domain.co.uk"));
        assert!(!validate_email(""));
        assert!(!validate_email("invalid"));
        assert!(!validate_email("@domain.com"));
        assert!(!validate_email("user@"));
        assert!(!validate_email("user@domain"));
        assert!(!validate_email("user.domain.com"));
    }

    #[test]
    fn test_url_validation() {
        assert!(validate_url("http://example.com"));
        assert!(validate_url("https://example.com/path?query=value#fragment"));
        assert!(validate_url("http://subdomain.example.com:8080/path"));
        assert!(!validate_url(""));
        assert!(!validate_url("ftp://example.com"));
        assert!(!validate_url("http://"));
        assert!(!validate_url("not-a-url"));
    }

    #[test]
    fn test_url_parsing() {
        let url = parse_url("https://example.com:8080/path/to/page?query=value&other=data#section").unwrap();
        assert_eq!(url.scheme, "https");
        assert_eq!(url.host, "example.com");
        assert_eq!(url.port, 8080);
        assert_eq!(url.path, "/path/to/page");
        assert_eq!(url.query, Some("query=value&other=data".to_string()));
        assert_eq!(url.fragment, Some("section".to_string()));
        
        let simple_url = parse_url("http://example.com").unwrap();
        assert_eq!(simple_url.scheme, "http");
        assert_eq!(simple_url.host, "example.com");
        assert_eq!(simple_url.port, 80);
        assert_eq!(simple_url.path, "/");
        assert!(simple_url.query.is_none());
        assert!(simple_url.fragment.is_none());
    }

    #[test]
    fn test_bandwidth_formatting() {
        assert_eq!(format_bandwidth(1024.0), "1.00 KB/s");
        assert_eq!(format_bandwidth(1024.0 * 1024.0), "1.00 MB/s");
        assert_eq!(format_bandwidth(1024.0 * 1024.0 * 1024.0), "1.00 GB/s");
        assert_eq!(format_bandwidth(500.0), "500 B/s");
        assert_eq!(format_bandwidth(1536.0), "1.50 KB/s");
    }

    #[test]
    fn test_network_diagnostics() {
        // This should not panic even if network operations fail
        let _ = network_diagnostics();
    }

    #[test]
    fn test_port_scanning() {
        // Test scanning a small range on localhost
        let result = scan_ports("127.0.0.1", 1, 10);
        assert!(result.is_ok());
        
        // Parallel scan test
        let ports = vec![80, 443, 8080, 3000, 5000];
        let result = scan_ports_parallel("127.0.0.1", &ports, Duration::from_millis(100), 2);
        assert!(result.is_ok());
    }
}
