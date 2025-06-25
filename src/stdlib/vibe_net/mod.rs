// # VibeNet - Network I/O Package
// 
// VibeNet provides a comprehensive networking library for the CURSED programming language,
// offering TCP/IP, UDP, domain name resolution, and socket programming capabilities.
// 
// This module is inspired by Go's net package but enhanced with modern usability,
// extended protocols, and performance optimizations tailored for CURSED applications.

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use std::io::{Read, Write};
use crate::error::CursedError;

pub mod error;
pub mod ip;
pub mod addr;
pub mod conn;
pub mod listener;
pub mod dns;
pub mod dialer;
pub mod pool;
pub mod circuit_breaker;
pub mod rate_limiter;
pub mod protocol;
pub mod interface;
pub mod enhanced;
pub mod utils;
pub mod security;
pub mod monitoring;

// Re-export core types for convenient access
pub use ip::{IPVibe, IPNetVibe, IPMaskVibe};
pub use addr::{AddrVibe, TCPAddrVibe, UDPAddrVibe, UnixAddrVibe};
pub use conn::{ConnVibe, TCPConnVibe, UDPConnVibe, UnixConnVibe, PacketConnVibe};
pub use listener::{ListenerVibe, TCPListenerVibe, UnixListenerVibe};
pub use dns::{DNSResolverVibe, MXVibe, NSVibe, SRVVibe};
pub use dialer::DialerVibe;
pub use pool::{ConnPoolVibe, ConnPoolStats};
pub use circuit_breaker::{CircuitBreakerVibe, CircuitBreakerState};
pub use rate_limiter::{RateLimiterVibe, Reservation};
pub use protocol::{WebSocketConnVibe, MQTTConnVibe, HTTP2ConnVibe, HTTP2StreamVibe};
pub use interface::{InterfaceVibe, InterfaceFlags, HardwareAddrVibe};
pub use enhanced::{EnhancedConnVibe, RetryConfig, NetworkQualityTracker, LoadBalancer, LoadBalanceStrategy, ProtocolNegotiator, ConnectionMultiplexer};
pub use utils::{BandwidthMeter, NetworkUtils, TopologyDiscovery, NetworkTester, BandwidthStats, NetworkInterface};
pub use security::{TlsConfig, TlsVersion, CipherSuite, Certificate, CertificateValidator, SecurityScanner, SecureChannel};
pub use monitoring::{NetworkMonitor, ConnectionHealthChecker, HealthCheckTarget, HealthStatus, NetworkEvent};

/// Result type for network operations using CURSED error handling
pub type NetResult<T> = std::result::Result<T, NetError>;

/// Context type for cancellation and timeouts
pub struct VibeContext {
    deadline: Option<SystemTime>,
    cancelled: Arc<Mutex<bool>>,
}

impl VibeContext {
    pub fn new() -> Self {
        Self {
            deadline: None,
            cancelled: Arc::new(Mutex::new(false)),
        }
    }
    
    pub fn with_deadline(deadline: SystemTime) -> Self {
        Self {
            deadline: Some(deadline),
            cancelled: Arc::new(Mutex::new(false)),
        }
    }
    
    pub fn with_timeout(timeout: Duration) -> Self {
        Self {
            deadline: Some(SystemTime::now() + timeout),
            cancelled: Arc::new(Mutex::new(false)),
        }
    }
    
    pub fn is_cancelled(&self) -> bool {
        *self.cancelled.lock().unwrap()
    }
    
    pub fn cancel(&self) {
        *self.cancelled.lock().unwrap() = true;
    }
    
    pub fn deadline(&self) -> Option<SystemTime> {
        self.deadline
    }
    
    pub fn is_expired(&self) -> bool {
        if let Some(deadline) = self.deadline {
            SystemTime::now() > deadline
        } else {
            false
        }
    }
}

// High-Level Networking Functions

/// Dial connects to the address on the named network
/// 
/// # Arguments
/// * `network` - The network type ("tcp", "udp", "unix", etc.)
/// * `address` - The network address to connect to
/// 
/// # Returns
/// A connection implementing ConnVibe or an error
pub fn dial(network: &str, address: &str) -> NetResult<Box<dyn ConnVibe>> {
    let dialer = DialerVibe::new();
    dialer.dial(network, address)
}

/// DialTimeout connects to the address with a timeout
/// 
/// # Arguments
/// * `network` - The network type ("tcp", "udp", "unix", etc.)
/// * `address` - The network address to connect to
/// * `timeout` - Connection timeout duration
/// 
/// # Returns
/// A connection implementing ConnVibe or an error
pub fn dial_timeout(network: &str, address: &str, timeout: Duration) -> NetResult<Box<dyn ConnVibe>> {
    let mut dialer = DialerVibe::new();
    dialer.set_timeout(timeout);
    dialer.dial(network, address)
}

/// Listen announces on the local network address
/// 
/// # Arguments
/// * `network` - The network type ("tcp", "unix", etc.)
/// * `address` - The local address to listen on
/// 
/// # Returns
/// A listener implementing ListenerVibe or an error
pub fn listen(network: &str, address: &str) -> NetResult<Box<dyn ListenerVibe>> {
    match network {
        "tcp" | "tcp4" | "tcp6" => {
            let addr = resolve_tcp_addr(network, address)?;
            let listener = TCPListenerVibe::listen(network, Some(&addr))?;
            Ok(Box::new(listener))
        }
        "unix" | "unixpacket" => {
            let addr = resolve_unix_addr(network, address)?;
            let listener = UnixListenerVibe::listen(network, Some(&addr))?;
            Ok(Box::new(listener))
        }
        _ => Err(CursedError::new(&format!("Unsupported network type: {}", network)))
    }
}

/// ListenPacket listens for packets on the local network address
/// 
/// # Arguments
/// * `network` - The network type ("udp", "unixgram", etc.)
/// * `address` - The local address to listen on
/// 
/// # Returns
/// A packet connection implementing PacketConnVibe or an error
pub fn listen_packet(network: &str, address: &str) -> NetResult<Box<dyn PacketConnVibe>> {
    match network {
        "udp" | "udp4" | "udp6" => {
            let addr = resolve_udp_addr(network, address)?;
            let conn = UDPConnVibe::listen(network, Some(&addr))?;
            Ok(Box::new(conn))
        }
        _ => Err(CursedError::new(&format!("Unsupported packet network type: {}", network)))
    }
}

/// ResolveTCPAddr resolves a TCP address
/// 
/// # Arguments
/// * `network` - The network type ("tcp", "tcp4", "tcp6")
/// * `address` - The address string to resolve
/// 
/// # Returns
/// A resolved TCP address or an error
pub fn resolve_tcp_addr(network: &str, address: &str) -> NetResult<TCPAddrVibe> {
    TCPAddrVibe::resolve(network, address)
}

/// ResolveUDPAddr resolves a UDP address
/// 
/// # Arguments
/// * `network` - The network type ("udp", "udp4", "udp6")
/// * `address` - The address string to resolve
/// 
/// # Returns
/// A resolved UDP address or an error
pub fn resolve_udp_addr(network: &str, address: &str) -> NetResult<UDPAddrVibe> {
    UDPAddrVibe::resolve(network, address)
}

/// ResolveUnixAddr resolves a Unix address
/// 
/// # Arguments
/// * `network` - The network type ("unix", "unixgram", "unixpacket")
/// * `address` - The address string to resolve
/// 
/// # Returns
/// A resolved Unix address or an error
pub fn resolve_unix_addr(network: &str, address: &str) -> NetResult<UnixAddrVibe> {
    UnixAddrVibe::resolve(network, address)
}

/// DialTCP connects to the TCP address
/// 
/// # Arguments
/// * `network` - The network type ("tcp", "tcp4", "tcp6")
/// * `laddr` - Local address (optional)
/// * `raddr` - Remote address
/// 
/// # Returns
/// A TCP connection or an error
pub fn dial_tcp(network: &str, laddr: Option<&TCPAddrVibe>, raddr: &TCPAddrVibe) -> NetResult<TCPConnVibe> {
    TCPConnVibe::dial(network, laddr, raddr)
}

/// DialUDP connects to the UDP address
/// 
/// # Arguments
/// * `network` - The network type ("udp", "udp4", "udp6")
/// * `laddr` - Local address (optional)
/// * `raddr` - Remote address
/// 
/// # Returns
/// A UDP connection or an error
pub fn dial_udp(network: &str, laddr: Option<&UDPAddrVibe>, raddr: &UDPAddrVibe) -> NetResult<UDPConnVibe> {
    UDPConnVibe::dial(network, laddr, raddr)
}

/// DialUnix connects to the Unix address
/// 
/// # Arguments
/// * `network` - The network type ("unix", "unixgram", "unixpacket")
/// * `laddr` - Local address (optional)
/// * `raddr` - Remote address
/// 
/// # Returns
/// A Unix connection or an error
pub fn dial_unix(network: &str, laddr: Option<&UnixAddrVibe>, raddr: &UnixAddrVibe) -> NetResult<UnixConnVibe> {
    UnixConnVibe::dial(network, laddr, raddr)
}

// DNS and Host Resolution Functions

/// LookupHost looks up the given host using the local resolver
/// 
/// # Arguments
/// * `host` - The hostname to look up
/// 
/// # Returns
/// A list of addresses or an error
pub fn lookup_host(host: &str) -> NetResult<Vec<String>> {
    let resolver = DNSResolverVibe::new();
    resolver.lookup_host(host)
}

/// LookupIP looks up host using the local resolver
/// 
/// # Arguments
/// * `host` - The hostname to look up
/// 
/// # Returns
/// A list of IP addresses or an error
pub fn lookup_ip(host: &str) -> NetResult<Vec<IPVibe>> {
    let resolver = DNSResolverVibe::new();
    resolver.lookup_ip(host)
}

/// LookupPort looks up the port for the given network and service
/// 
/// # Arguments
/// * `network` - The network type
/// * `service` - The service name
/// 
/// # Returns
/// The port number or an error
pub fn lookup_port(network: &str, service: &str) -> NetResult<i32> {
    let resolver = DNSResolverVibe::new();
    resolver.lookup_port(network, service)
}

/// LookupCNAME returns the canonical name for the given host
/// 
/// # Arguments
/// * `host` - The hostname to look up
/// 
/// # Returns
/// The canonical name or an error
pub fn lookup_cname(host: &str) -> NetResult<String> {
    let resolver = DNSResolverVibe::new();
    resolver.lookup_cname(host)
}

/// LookupSRV tries to resolve an SRV query of the given service, protocol, and domain name
/// 
/// # Arguments
/// * `service` - The service name
/// * `proto` - The protocol name
/// * `name` - The domain name
/// 
/// # Returns
/// The canonical name and SRV records or an error
pub fn lookup_srv(service: &str, proto: &str, name: &str) -> NetResult<(String, Vec<SRVVibe>)> {
    let resolver = DNSResolverVibe::new();
    resolver.lookup_srv(service, proto, name)
}

/// LookupMX returns the DNS MX records for the given domain name
/// 
/// # Arguments
/// * `name` - The domain name
/// 
/// # Returns
/// A list of MX records or an error
pub fn lookup_mx(name: &str) -> NetResult<Vec<MXVibe>> {
    let resolver = DNSResolverVibe::new();
    resolver.lookup_mx(name)
}

/// LookupNS returns the DNS NS records for the given domain name
/// 
/// # Arguments
/// * `name` - The domain name
/// 
/// # Returns
/// A list of NS records or an error
pub fn lookup_ns(name: &str) -> NetResult<Vec<NSVibe>> {
    let resolver = DNSResolverVibe::new();
    resolver.lookup_ns(name)
}

/// LookupTXT returns the DNS TXT records for the given domain name
/// 
/// # Arguments
/// * `name` - The domain name
/// 
/// # Returns
/// A list of TXT records or an error
pub fn lookup_txt(name: &str) -> NetResult<Vec<String>> {
    let resolver = DNSResolverVibe::new();
    resolver.lookup_txt(name)
}

/// LookupAddr performs a reverse lookup for the given address
/// 
/// # Arguments
/// * `addr` - The IP address to look up
/// 
/// # Returns
/// A list of hostnames or an error
pub fn lookup_addr(addr: &str) -> NetResult<Vec<String>> {
    let resolver = DNSResolverVibe::new();
    resolver.lookup_addr(addr)
}

// Enhanced IPv6 Support Functions

/// IsIPv6Enabled returns whether IPv6 is enabled on this system
pub fn is_ipv6_enabled() -> bool {
    // Implementation would check system IPv6 support
    true // Placeholder - would need actual system check
}

/// PreferIPv6 returns whether IPv6 is preferred over IPv4
pub fn prefer_ipv6() -> bool {
    // Implementation would check system preferences
    false // Placeholder - would need actual preference check
}

/// SetPreferIPv6 sets whether to prefer IPv6 over IPv4
/// 
/// # Arguments
/// * `prefer` - Whether to prefer IPv6
pub fn set_prefer_ipv6(prefer: bool) {
    // Implementation would set system preference
    // This is a placeholder - would need actual preference setting
}

/// IPv6InterfaceAddrs returns a list of the system's IPv6 interface addresses
/// 
/// # Returns
/// A list of IPv6 addresses or an error
pub fn ipv6_interface_addrs() -> NetResult<Vec<IPVibe>> {
    // Implementation would query system interfaces
    Ok(vec![]) // Placeholder - would need actual interface querying
}

// Network Interface Functions

/// Interfaces returns a list of the system's network interfaces
/// 
/// # Returns
/// A list of network interfaces or an error
pub fn interfaces() -> NetResult<Vec<InterfaceVibe>> {
    InterfaceVibe::list()
}

/// InterfaceByIndex returns the interface specified by index
/// 
/// # Arguments
/// * `index` - The interface index
/// 
/// # Returns
/// The network interface or an error
pub fn interface_by_index(index: i32) -> NetResult<InterfaceVibe> {
    InterfaceVibe::by_index(index)
}

/// InterfaceByName returns the interface specified by name
/// 
/// # Arguments
/// * `name` - The interface name
/// 
/// # Returns
/// The network interface or an error
pub fn interface_by_name(name: &str) -> NetResult<InterfaceVibe> {
    InterfaceVibe::by_name(name)
}

// Module initialization and utility functions

/// Initialize the vibe_net module
pub fn init() -> NetResult<()> {
    // Perform any necessary initialization
    // This could include setting up global state, checking system capabilities, etc.
    Ok(())
}

/// Get version information for the vibe_net module
pub fn version() -> &'static str {
    "1.0.0"
}

/// Get information about supported features
pub fn features() -> HashMap<String, bool> {
    let mut features = HashMap::new();
    features.insert("tcp".to_string(), true);
    features.insert("udp".to_string(), true);
    features.insert("unix".to_string(), cfg!(unix));
    features.insert("ipv6".to_string(), is_ipv6_enabled());
    features.insert("dns".to_string(), true);
    features.insert("websocket".to_string(), true);
    features.insert("mqtt".to_string(), true);
    features.insert("http2".to_string(), true);
    features.insert("connection_pool".to_string(), true);
    features.insert("circuit_breaker".to_string(), true);
    features.insert("rate_limiter".to_string(), true);
    
    // Enhanced features
    features.insert("enhanced_connections".to_string(), true);
    features.insert("retry_mechanisms".to_string(), true);
    features.insert("load_balancing".to_string(), true);
    features.insert("network_quality_tracking".to_string(), true);
    features.insert("protocol_negotiation".to_string(), true);
    features.insert("connection_multiplexing".to_string(), true);
    
    // Utility features
    features.insert("bandwidth_monitoring".to_string(), true);
    features.insert("network_utilities".to_string(), true);
    features.insert("topology_discovery".to_string(), true);
    features.insert("network_testing".to_string(), true);
    features.insert("cidr_utilities".to_string(), true);
    
    // Security features
    features.insert("tls_support".to_string(), true);
    features.insert("certificate_validation".to_string(), true);
    features.insert("security_scanning".to_string(), true);
    features.insert("encrypted_channels".to_string(), true);
    features.insert("vulnerability_assessment".to_string(), true);
    
    // Monitoring features
    features.insert("network_monitoring".to_string(), true);
    features.insert("health_checking".to_string(), true);
    features.insert("metrics_collection".to_string(), true);
    features.insert("event_handling".to_string(), true);
    features.insert("performance_tracking".to_string(), true);
    
    features
}

