use crate::error::CursedError;
/// DNS resolution and hostname lookup for the CURSED networking module
/// 
/// This module provides comprehensive DNS functionality including hostname
/// resolution, reverse DNS lookup, different record type queries, and
/// DNS server configuration.

use std::net::{ToSocketAddrs, IpAddr as StdIpAddr};
use std::collections::HashMap;
use std::time::Duration;
// use crate::stdlib::net::error::{NetError, NetResult, dns_error, dns_error_with_type, timeout_error};
// use crate::stdlib::net::address::{IpAddr, SocketAddr};

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
impl std::fmt::Display for DnsRecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
        }
    }
/// DNS record data
#[derive(Debug, Clone)]
pub enum DnsRecordData {
    SOA {
    SRV {
/// DNS record with metadata
#[derive(Debug, Clone)]
pub struct DnsRecord {
    pub class: String, // Usually "IN" for Internet
/// DNS query configuration
#[derive(Debug, Clone)]
pub struct DnsQuery {
impl Default for DnsQuery {
    fn default() -> Self {
        Self {
        }
    }
/// DNS response
#[derive(Debug, Clone)]
pub struct DnsResponse {
/// DNS resolver with caching and configuration
#[derive(Debug)]
pub struct DnsResolver {
impl Default for DnsResolver {
    fn default() -> Self {
        Self {
            cache_ttl: Duration::from_secs(300), // 5 minutes
            dns_servers: vec![
                "8.8.8.8:53".parse().unwrap(),         // Google DNS
                "8.8.4.4:53".parse().unwrap(),         // Google DNS Secondary
                "1.1.1.1:53".parse().unwrap(),         // Cloudflare DNS
                "1.0.0.1:53".parse().unwrap(),         // Cloudflare DNS Secondary
        }
    }
impl DnsResolver {
    /// Create a new DNS resolver
    pub fn new() -> Self {
        Self::default()
    /// Create a DNS resolver with custom DNS servers
    pub fn with_servers(servers: Vec<SocketAddr>) -> Self {
        Self {
            ..Self::default()
        }
    }
    
    /// Set DNS servers
    pub fn set_dns_servers(&mut self, servers: Vec<SocketAddr>) {
        self.dns_servers = servers;
    /// Set cache TTL
    pub fn set_cache_ttl(&mut self, ttl: Duration) {
        self.cache_ttl = ttl;
    /// Set query timeout
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    /// Set retry count
    pub fn set_retries(&mut self, retries: u32) {
        self.retries = retries;
    /// Clear DNS cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    /// Resolve hostname to IP addresses
    pub fn resolve(&mut self, hostname: &str) -> NetResult<Vec<IpAddr>> {
        let cache_key = format!("{}:A", hostname);
        
        // Check cache first
        if let Some((records, timestamp)) = self.cache.get(&cache_key) {
            if timestamp.elapsed() < self.cache_ttl {
                return Ok(records.iter()
                    .filter_map(|r| match &r.data {
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
        // Create DNS records and cache them
        let records: Vec<DnsRecord> = ip_addrs.iter().map(|ip| {
            let record_type = if ip.is_ipv4() { DnsRecordType::A } else { DnsRecordType::AAAA };
            DnsRecord {
            }
        }).collect();
        
        self.cache.insert(cache_key, (records, std::time::Instant::now()));
        
        Ok(ip_addrs)
    /// Resolve hostname to IPv4 addresses only
    pub fn resolve_ipv4(&mut self, hostname: &str) -> NetResult<Vec<IpAddr>> {
        let all_ips = self.resolve(hostname)?;
        Ok(all_ips.into_iter().filter(|ip| ip.is_ipv4()).collect())
    /// Resolve hostname to IPv6 addresses only
    pub fn resolve_ipv6(&mut self, hostname: &str) -> NetResult<Vec<IpAddr>> {
        let all_ips = self.resolve(hostname)?;
        Ok(all_ips.into_iter().filter(|ip| ip.is_ipv6()).collect())
    /// Perform reverse DNS lookup (IP to hostname)
    pub fn reverse_lookup(&mut self, ip: IpAddr) -> NetResult<String> {
        let hostname = match ip {
            IpAddr::V4(ipv4) => {
                // For IPv4, create reverse domain like: 1.2.3.4 -> 4.3.2.1.in-addr.arpa
                let octets = ipv4.octets();
                format!("{}.{}.{}.{}.in-addr.arpa", octets[3], octets[2], octets[1], octets[0])
            IpAddr::V6(_) => {
                // For IPv6, this would be more complex with ip6.arpa
                // For now, return an error
                return Err(dns_error_with_type(
                    "PTR"
                ));
        
        // In a real implementation, we would query for PTR records
        // For now, we'll use a simplified approach
        Err(dns_error_with_type(&hostname, "Reverse lookup not fully implemented", "PTR"))
    /// Query for MX records
    pub fn lookup_mx(&mut self, hostname: &str) -> NetResult<Vec<DnsRecord>> {
        // Simplified MX lookup - in a real implementation this would query actual MX records
        Err(dns_error_with_type(hostname, "MX record lookup not implemented", "MX"))
    /// Query for TXT records
    pub fn lookup_txt(&mut self, hostname: &str) -> NetResult<Vec<DnsRecord>> {
        // Simplified TXT lookup - in a real implementation this would query actual TXT records
        Err(dns_error_with_type(hostname, "TXT record lookup not implemented", "TXT"))
    /// Query for CNAME records
    pub fn lookup_cname(&mut self, hostname: &str) -> NetResult<Vec<DnsRecord>> {
        // Simplified CNAME lookup - in a real implementation this would query actual CNAME records
        Err(dns_error_with_type(hostname, "CNAME record lookup not implemented", "CNAME"))
    /// Perform a custom DNS query
    pub fn query(&mut self, query: DnsQuery) -> NetResult<DnsResponse> {
        let start_time = std::time::Instant::now();
        
        match query.record_type {
            DnsRecordType::A | DnsRecordType::AAAA => {
                let ips = self.resolve(&query.hostname)?;
                let records = ips.into_iter().map(|ip| {
                    let record_type = if ip.is_ipv4() { DnsRecordType::A } else { DnsRecordType::AAAA };
                    DnsRecord {
                    }
                }).collect();
                
                Ok(DnsResponse {
                })
            _ => {
                Err(dns_error_with_type(
                    &query.record_type.to_string()
                ))
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
/// Simple hostname resolution using global resolver
pub fn resolve_hostname(hostname: &str) -> NetResult<Vec<IpAddr>> {
    let resolver = get_global_resolver();
    let mut resolver_guard = resolver.lock().unwrap();
    resolver_guard.resolve(hostname)
/// Simple IP to hostname resolution using global resolver
pub fn resolve_ip(ip: IpAddr) -> NetResult<String> {
    let resolver = get_global_resolver();
    let mut resolver_guard = resolver.lock().unwrap();
    resolver_guard.reverse_lookup(ip)
/// Lookup MX records using global resolver
pub fn lookup_mx(hostname: &str) -> NetResult<Vec<DnsRecord>> {
    let resolver = get_global_resolver();
    let mut resolver_guard = resolver.lock().unwrap();
    resolver_guard.lookup_mx(hostname)
/// Lookup TXT records using global resolver
pub fn lookup_txt(hostname: &str) -> NetResult<Vec<DnsRecord>> {
    let resolver = get_global_resolver();
    let mut resolver_guard = resolver.lock().unwrap();
    resolver_guard.lookup_txt(hostname)
/// Lookup CNAME records using global resolver
pub fn lookup_cname(hostname: &str) -> NetResult<Vec<DnsRecord>> {
    let resolver = get_global_resolver();
    let mut resolver_guard = resolver.lock().unwrap();
    resolver_guard.lookup_cname(hostname)
/// Configure global DNS resolver
pub fn configure_global_resolver(servers: Vec<SocketAddr>, cache_ttl: Duration, timeout: Duration) {
    let resolver = get_global_resolver();
    let mut resolver_guard = resolver.lock().unwrap();
    resolver_guard.set_dns_servers(servers);
    resolver_guard.set_cache_ttl(cache_ttl);
    resolver_guard.set_timeout(timeout);
/// Clear global DNS cache
pub fn clear_dns_cache() {
        // TODO: implement
    }
    let resolver = get_global_resolver();
    let mut resolver_guard = resolver.lock().unwrap();
    resolver_guard.clear_cache();
/// Get DNS cache statistics
pub fn get_dns_cache_stats() -> (usize, usize) {
    let resolver = get_global_resolver();
    let resolver_guard = resolver.lock().unwrap();
    resolver_guard.cache_stats()
/// Utility functions

/// Check if a hostname is valid
pub fn is_valid_hostname(hostname: &str) -> bool {
    if hostname.is_empty() || hostname.len() > 253 {
        return false;
    // Check for valid characters and format
    for part in hostname.split('.') {
        if part.is_empty() || part.len() > 63 {
            return false;
        if part.starts_with('-') || part.ends_with('-') {
            return false;
        for ch in part.chars() {
            if !ch.is_ascii_alphanumeric() && ch != '-' {
                return false;
            }
        }
    true
/// Check if a hostname looks like an IP address
pub fn is_ip_address(hostname: &str) -> bool {
    hostname.parse::<std::net::IpAddr>().is_ok()
/// Normalize hostname (convert to lowercase, remove trailing dot)
pub fn normalize_hostname(hostname: &str) -> String {
    let mut normalized = hostname.to_lowercase();
    if normalized.ends_with('.') {
        normalized.pop();
    }
    normalized
