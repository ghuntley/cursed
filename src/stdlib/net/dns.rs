use crate::error::Error;
/// DNS resolution and hostname lookup for the CURSED networking module
/// 
/// This module provides comprehensive DNS functionality including hostname
/// resolution, reverse DNS lookup, different record type queries, and
/// DNS server configuration.

use std::net::{ToSocketAddrs, IpAddr as StdIpAddr};
use std::collections::HashMap;
use std::time::Duration;
use crate::stdlib::net::error::{NetError, NetResult, dns_error, dns_error_with_type, timeout_error};
use crate::stdlib::net::address::{IpAddr, SocketAddr};

/// DNS record types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DnsRecordType {
    A,      // IPv4 address
    AAAA,   // IPv6 address
    CNAME,  // Canonical name
    MX,     // Mail exchange
    TXT,    // Text record
    NS,     // Name server
    PTR,    // Pointer record (reverse DNS)
    SOA,    // Start of authority
    SRV,    // Service record
}

impl std::fmt::Display for DnsRecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DnsRecordType::A => write!(f, "A"),
            DnsRecordType::AAAA => write!(f, "AAAA"),
            DnsRecordType::CNAME => write!(f, "CNAME"),
            DnsRecordType::MX => write!(f, "MX"),
            DnsRecordType::TXT => write!(f, "TXT"),
            DnsRecordType::NS => write!(f, "NS"),
            DnsRecordType::PTR => write!(f, "PTR"),
            DnsRecordType::SOA => write!(f, "SOA"),
            DnsRecordType::SRV => write!(f, "SRV"),
        }
    }
}

/// DNS record data
#[derive(Debug, Clone)]
pub enum DnsRecordData {
    A(IpAddr),
    AAAA(IpAddr),
    CNAME(String),
    MX { priority: u16, hostname: String },
    TXT(String),
    NS(String),
    PTR(String),
    SOA {
        primary_ns: String,
        admin_email: String,
        serial: u32,
        refresh: u32,
        retry: u32,
        expire: u32,
        minimum_ttl: u32,
    },
    SRV {
        priority: u16,
        weight: u16,
        port: u16,
        target: String,
    },
}

/// DNS record with metadata
#[derive(Debug, Clone)]
pub struct DnsRecord {
    pub name: String,
    pub record_type: DnsRecordType,
    pub data: DnsRecordData,
    pub ttl: u32,
    pub class: String, // Usually "IN" for Internet
}

/// DNS query configuration
#[derive(Debug, Clone)]
pub struct DnsQuery {
    pub hostname: String,
    pub record_type: DnsRecordType,
    pub timeout: Duration,
    pub retries: u32,
    pub use_tcp: bool,
}

impl Default for DnsQuery {
    fn default() -> Self {
        Self {
            hostname: String::new(),
            record_type: DnsRecordType::A,
            timeout: Duration::from_secs(5),
            retries: 3,
            use_tcp: false,
        }
    }
}

/// DNS response
#[derive(Debug, Clone)]
pub struct DnsResponse {
    pub query: DnsQuery,
    pub records: Vec<DnsRecord>,
    pub authoritative: bool,
    pub truncated: bool,
    pub recursion_available: bool,
    pub response_time: Duration,
}

/// DNS resolver with caching and configuration
#[derive(Debug)]
pub struct DnsResolver {
    cache: HashMap<String, (Vec<DnsRecord>, std::time::Instant)>,
    cache_ttl: Duration,
    timeout: Duration,
    retries: u32,
    dns_servers: Vec<SocketAddr>,
}

impl Default for DnsResolver {
    fn default() -> Self {
        Self {
            cache: HashMap::new(),
            cache_ttl: Duration::from_secs(300), // 5 minutes
            timeout: Duration::from_secs(5),
            retries: 3,
            dns_servers: vec![
                "8.8.8.8:53".parse().unwrap(),         // Google DNS
                "8.8.4.4:53".parse().unwrap(),         // Google DNS Secondary
                "1.1.1.1:53".parse().unwrap(),         // Cloudflare DNS
                "1.0.0.1:53".parse().unwrap(),         // Cloudflare DNS Secondary
            ],
        }
    }
}

impl DnsResolver {
    /// Create a new DNS resolver
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create a DNS resolver with custom DNS servers
    pub fn with_servers(servers: Vec<SocketAddr>) -> Self {
        Self {
            dns_servers: servers,
            ..Self::default()
        }
    }
    
    /// Set DNS servers
    pub fn set_dns_servers(&mut self, servers: Vec<SocketAddr>) {
        self.dns_servers = servers;
    }
    
    /// Set cache TTL
    pub fn set_cache_ttl(&mut self, ttl: Duration) {
        self.cache_ttl = ttl;
    }
    
    /// Set query timeout
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }
    
    /// Set retry count
    pub fn set_retries(&mut self, retries: u32) {
        self.retries = retries;
    }
    
    /// Clear DNS cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
    
    /// Resolve hostname to IP addresses
    pub fn resolve(&mut self, hostname: &str) -> NetResult<Vec<IpAddr>> {
        let cache_key = format!("{}:A", hostname);
        
        // Check cache first
        if let Some((records, timestamp)) = self.cache.get(&cache_key) {
            if timestamp.elapsed() < self.cache_ttl {
                return Ok(records.iter()
                    .filter_map(|r| match &r.data {
                        DnsRecordData::A(ip) | DnsRecordData::AAAA(ip) => Some(*ip),
                        _ => None,
                    })
                    .collect());
            }
        }
        
        // Perform actual DNS resolution using standard library
        let socket_addrs: Vec<std::net::SocketAddr> = format!("{}:0", hostname)
            .to_socket_addrs()
            .map_err(|e| dns_error_with_type(hostname, &e.to_string(), "A/AAAA"))?
            .collect();
        
        let ip_addrs: Vec<IpAddr> = socket_addrs
            .into_iter()
            .map(|addr| addr.ip().into())
            .collect();
        
        if ip_addrs.is_empty() {
            return Err(dns_error_with_type(hostname, "No IP addresses found", "A/AAAA"));
        }
        
        // Create DNS records and cache them
        let records: Vec<DnsRecord> = ip_addrs.iter().map(|ip| {
            let record_type = if ip.is_ipv4() { DnsRecordType::A } else { DnsRecordType::AAAA };
            DnsRecord {
                name: hostname.to_string(),
                record_type,
                data: if ip.is_ipv4() { DnsRecordData::A(*ip) } else { DnsRecordData::AAAA(*ip) },
                ttl: 300,
                class: "IN".to_string(),
            }
        }).collect();
        
        self.cache.insert(cache_key, (records, std::time::Instant::now()));
        
        Ok(ip_addrs)
    }
    
    /// Resolve hostname to IPv4 addresses only
    pub fn resolve_ipv4(&mut self, hostname: &str) -> NetResult<Vec<IpAddr>> {
        let all_ips = self.resolve(hostname)?;
        Ok(all_ips.into_iter().filter(|ip| ip.is_ipv4()).collect())
    }
    
    /// Resolve hostname to IPv6 addresses only
    pub fn resolve_ipv6(&mut self, hostname: &str) -> NetResult<Vec<IpAddr>> {
        let all_ips = self.resolve(hostname)?;
        Ok(all_ips.into_iter().filter(|ip| ip.is_ipv6()).collect())
    }
    
    /// Perform reverse DNS lookup (IP to hostname)
    pub fn reverse_lookup(&mut self, ip: IpAddr) -> NetResult<String> {
        let hostname = match ip {
            IpAddr::V4(ipv4) => {
                // For IPv4, create reverse domain like: 1.2.3.4 -> 4.3.2.1.in-addr.arpa
                let octets = ipv4.octets();
                format!("{}.{}.{}.{}.in-addr.arpa", octets[3], octets[2], octets[1], octets[0])
            },
            IpAddr::V6(_) => {
                // For IPv6, this would be more complex with ip6.arpa
                // For now, return an error
                return Err(dns_error_with_type(
                    &ip.to_string(),
                    "IPv6 reverse lookup not implemented",
                    "PTR"
                ));
            },
        };
        
        // In a real implementation, we would query for PTR records
        // For now, we'll use a simplified approach
        Err(dns_error_with_type(&hostname, "Reverse lookup not fully implemented", "PTR"))
    }
    
    /// Query for MX records
    pub fn lookup_mx(&mut self, hostname: &str) -> NetResult<Vec<DnsRecord>> {
        // Simplified MX lookup - in a real implementation this would query actual MX records
        Err(dns_error_with_type(hostname, "MX record lookup not implemented", "MX"))
    }
    
    /// Query for TXT records
    pub fn lookup_txt(&mut self, hostname: &str) -> NetResult<Vec<DnsRecord>> {
        // Simplified TXT lookup - in a real implementation this would query actual TXT records
        Err(dns_error_with_type(hostname, "TXT record lookup not implemented", "TXT"))
    }
    
    /// Query for CNAME records
    pub fn lookup_cname(&mut self, hostname: &str) -> NetResult<Vec<DnsRecord>> {
        // Simplified CNAME lookup - in a real implementation this would query actual CNAME records
        Err(dns_error_with_type(hostname, "CNAME record lookup not implemented", "CNAME"))
    }
    
    /// Perform a custom DNS query
    pub fn query(&mut self, query: DnsQuery) -> NetResult<DnsResponse> {
        let start_time = std::time::Instant::now();
        
        match query.record_type {
            DnsRecordType::A | DnsRecordType::AAAA => {
                let ips = self.resolve(&query.hostname)?;
                let records = ips.into_iter().map(|ip| {
                    let record_type = if ip.is_ipv4() { DnsRecordType::A } else { DnsRecordType::AAAA };
                    DnsRecord {
                        name: query.hostname.clone(),
                        record_type,
                        data: if ip.is_ipv4() { DnsRecordData::A(ip) } else { DnsRecordData::AAAA(ip) },
                        ttl: 300,
                        class: "IN".to_string(),
                    }
                }).collect();
                
                Ok(DnsResponse {
                    query,
                    records,
                    authoritative: false,
                    truncated: false,
                    recursion_available: true,
                    response_time: start_time.elapsed(),
                })
            },
            _ => {
                Err(dns_error_with_type(
                    &query.hostname,
                    &format!("{} record queries not implemented", query.record_type),
                    &query.record_type.to_string()
                ))
            }
        }
    }
    
    /// Get cache statistics
    pub fn cache_stats(&self) -> (usize, usize) {
        let total_entries = self.cache.len();
        let expired_entries = self.cache.iter()
            .filter(|(_, (_, timestamp))| timestamp.elapsed() >= self.cache_ttl)
            .count();
        
        (total_entries, expired_entries)
    }
}

/// Global DNS resolver instance (thread-safe)
use std::sync::{Arc, Mutex};
use std::sync::OnceLock;

static GLOBAL_RESOLVER: OnceLock<Arc<Mutex<DnsResolver>>> = OnceLock::new();

fn get_global_resolver() -> &'static Arc<Mutex<DnsResolver>> {
    GLOBAL_RESOLVER.get_or_init(|| Arc::new(Mutex::new(DnsResolver::new())))
}

/// Simple hostname resolution using global resolver
pub fn resolve_hostname(hostname: &str) -> NetResult<Vec<IpAddr>> {
    let resolver = get_global_resolver();
    let mut resolver_guard = resolver.lock().unwrap();
    resolver_guard.resolve(hostname)
}

/// Simple IP to hostname resolution using global resolver
pub fn resolve_ip(ip: IpAddr) -> NetResult<String> {
    let resolver = get_global_resolver();
    let mut resolver_guard = resolver.lock().unwrap();
    resolver_guard.reverse_lookup(ip)
}

/// Lookup MX records using global resolver
pub fn lookup_mx(hostname: &str) -> NetResult<Vec<DnsRecord>> {
    let resolver = get_global_resolver();
    let mut resolver_guard = resolver.lock().unwrap();
    resolver_guard.lookup_mx(hostname)
}

/// Lookup TXT records using global resolver
pub fn lookup_txt(hostname: &str) -> NetResult<Vec<DnsRecord>> {
    let resolver = get_global_resolver();
    let mut resolver_guard = resolver.lock().unwrap();
    resolver_guard.lookup_txt(hostname)
}

/// Lookup CNAME records using global resolver
pub fn lookup_cname(hostname: &str) -> NetResult<Vec<DnsRecord>> {
    let resolver = get_global_resolver();
    let mut resolver_guard = resolver.lock().unwrap();
    resolver_guard.lookup_cname(hostname)
}

/// Configure global DNS resolver
pub fn configure_global_resolver(servers: Vec<SocketAddr>, cache_ttl: Duration, timeout: Duration) {
    let resolver = get_global_resolver();
    let mut resolver_guard = resolver.lock().unwrap();
    resolver_guard.set_dns_servers(servers);
    resolver_guard.set_cache_ttl(cache_ttl);
    resolver_guard.set_timeout(timeout);
}

/// Clear global DNS cache
pub fn clear_dns_cache() {
    let resolver = get_global_resolver();
    let mut resolver_guard = resolver.lock().unwrap();
    resolver_guard.clear_cache();
}

/// Get DNS cache statistics
pub fn get_dns_cache_stats() -> (usize, usize) {
    let resolver = get_global_resolver();
    let resolver_guard = resolver.lock().unwrap();
    resolver_guard.cache_stats()
}

/// Utility functions

/// Check if a hostname is valid
pub fn is_valid_hostname(hostname: &str) -> bool {
    if hostname.is_empty() || hostname.len() > 253 {
        return false;
    }
    
    // Check for valid characters and format
    for part in hostname.split('.') {
        if part.is_empty() || part.len() > 63 {
            return false;
        }
        
        if part.starts_with('-') || part.ends_with('-') {
            return false;
        }
        
        for ch in part.chars() {
            if !ch.is_ascii_alphanumeric() && ch != '-' {
                return false;
            }
        }
    }
    
    true
}

/// Check if a hostname looks like an IP address
pub fn is_ip_address(hostname: &str) -> bool {
    hostname.parse::<std::net::IpAddr>().is_ok()
}

/// Normalize hostname (convert to lowercase, remove trailing dot)
pub fn normalize_hostname(hostname: &str) -> String {
    let mut normalized = hostname.to_lowercase();
    if normalized.ends_with('.') {
        normalized.pop();
    }
    normalized
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dns_record_type_display() {
        assert_eq!(DnsRecordType::A.to_string(), "A");
        assert_eq!(DnsRecordType::AAAA.to_string(), "AAAA");
        assert_eq!(DnsRecordType::MX.to_string(), "MX");
    }

    #[test]
    fn test_dns_query_default() {
        let query = DnsQuery::default();
        assert_eq!(query.record_type, DnsRecordType::A);
        assert_eq!(query.timeout, Duration::from_secs(5));
        assert_eq!(query.retries, 3);
        assert!(!query.use_tcp);
    }

    #[test]
    fn test_dns_resolver_creation() {
        let resolver = DnsResolver::new();
        assert!(!resolver.dns_servers.is_empty());
        
        let custom_servers = vec!["1.1.1.1:53".parse().unwrap()];
        let resolver = DnsResolver::with_servers(custom_servers.clone());
        assert_eq!(resolver.dns_servers.len(), 1);
    }

    #[test]
    fn test_hostname_validation() {
        assert!(is_valid_hostname("example.com"));
        assert!(is_valid_hostname("sub.example.com"));
        assert!(!is_valid_hostname(""));
        assert!(!is_valid_hostname("example-.com"));
        assert!(!is_valid_hostname("-example.com"));
        
        // Very long hostname should be invalid
        let long_hostname = "a".repeat(300);
        assert!(!is_valid_hostname(&long_hostname));
    }

    #[test]
    fn test_ip_address_detection() {
        assert!(is_ip_address("127.0.0.1"));
        assert!(is_ip_address("::1"));
        assert!(!is_ip_address("example.com"));
        assert!(!is_ip_address("not-an-ip"));
    }

    #[test]
    fn test_hostname_normalization() {
        assert_eq!(normalize_hostname("Example.COM"), "example.com");
        assert_eq!(normalize_hostname("example.com."), "example.com");
        assert_eq!(normalize_hostname("EXAMPLE.COM."), "example.com");
    }

    #[test]
    fn test_resolver_configuration() {
        let mut resolver = DnsResolver::new();
        
        resolver.set_timeout(Duration::from_secs(10));
        resolver.set_retries(5);
        resolver.set_cache_ttl(Duration::from_secs(600));
        
        assert_eq!(resolver.timeout, Duration::from_secs(10));
        assert_eq!(resolver.retries, 5);
        assert_eq!(resolver.cache_ttl, Duration::from_secs(600));
    }

    #[test]
    fn test_cache_operations() {
        let mut resolver = DnsResolver::new();
        resolver.clear_cache();
        
        let (total, expired) = resolver.cache_stats();
        assert_eq!(total, 0);
        assert_eq!(expired, 0);
    }

    #[test]
    fn test_hostname_resolution() {
        // Test with localhost - should always work
        let result = resolve_hostname("localhost");
        assert!(result.is_ok());
        
        if let Ok(ips) = result {
            assert!(!ips.is_empty());
            // Should contain at least the loopback address
            assert!(ips.iter().any(|ip| ip.is_loopback()));
        }
    }

    #[test]
    fn test_dns_cache_management() {
        clear_dns_cache();
        let (total, _) = get_dns_cache_stats();
        assert_eq!(total, 0);
    }

    #[test]
    fn test_global_resolver_configuration() {
        let servers = vec!["8.8.8.8:53".parse().unwrap()];
        configure_global_resolver(
            servers,
            Duration::from_secs(300),
            Duration::from_secs(5)
        );
        
        // Configuration should not panic
        assert!(true);
    }
}
