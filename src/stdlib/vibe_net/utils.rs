use crate::error::Error;
/// # Network Utility Functions
/// 
/// This module provides network utility functions including IP address manipulation,
/// network address calculations, bandwidth measurement, and network topology utilities.

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::str::FromStr;
use std::time::{Duration, Instant, SystemTime};
use crate::error::CursedError;
use crate::stdlib::vibe_net::NetResult;

/// Network bandwidth measurement and calculation utilities
pub struct BandwidthMeter {
    measurements: Vec<BandwidthMeasurement>,
    window_size: Duration,
    max_measurements: usize,
}

#[derive(Debug, Clone)]
pub struct BandwidthMeasurement {
    pub timestamp: Instant,
    pub bytes_transferred: u64,
    pub duration: Duration,
    pub direction: TransferDirection,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransferDirection {
    Upload,
    Download,
    Bidirectional,
}

impl BandwidthMeter {
    pub fn new(window_size: Duration, max_measurements: usize) -> Self {
        Self {
            measurements: Vec::new(),
            window_size,
            max_measurements,
        }
    }

    /// Record a bandwidth measurement
    pub fn record_transfer(&mut self, bytes: u64, duration: Duration, direction: TransferDirection) {
        let measurement = BandwidthMeasurement {
            timestamp: Instant::now(),
            bytes_transferred: bytes,
            duration,
            direction,
        };

        self.measurements.push(measurement);
        self.cleanup_old_measurements();
    }

    /// Calculate current bandwidth in bytes per second
    pub fn current_bandwidth(&self, direction: Option<TransferDirection>) -> f64 {
        let cutoff = Instant::now() - self.window_size;
        let recent_measurements: Vec<&BandwidthMeasurement> = self.measurements
            .iter()
            .filter(|m| m.timestamp > cutoff)
            .filter(|m| direction.as_ref().map_or(true, |d| &m.direction == d))
            .collect();

        if recent_measurements.is_empty() {
            return 0.0;
        }

        let total_bytes: u64 = recent_measurements.iter().map(|m| m.bytes_transferred).sum();
        let total_duration = self.window_size.as_secs_f64();
        
        total_bytes as f64 / total_duration
    }

    /// Get bandwidth statistics
    pub fn bandwidth_stats(&self) -> BandwidthStats {
        let upload_bw = self.current_bandwidth(Some(TransferDirection::Upload));
        let download_bw = self.current_bandwidth(Some(TransferDirection::Download));
        let total_bw = self.current_bandwidth(None);

        BandwidthStats {
            upload_bytes_per_sec: upload_bw,
            download_bytes_per_sec: download_bw,
            total_bytes_per_sec: total_bw,
            measurement_count: self.measurements.len(),
            window_duration: self.window_size,
        }
    }

    fn cleanup_old_measurements(&mut self) {
        let cutoff = Instant::now() - self.window_size;
        self.measurements.retain(|m| m.timestamp > cutoff);
        
        // Also limit by max count
        if self.measurements.len() > self.max_measurements {
            let excess = self.measurements.len() - self.max_measurements;
            self.measurements.drain(0..excess);
        }
    }
}

#[derive(Debug, Clone)]
pub struct BandwidthStats {
    pub upload_bytes_per_sec: f64,
    pub download_bytes_per_sec: f64,
    pub total_bytes_per_sec: f64,
    pub measurement_count: usize,
    pub window_duration: Duration,
}

/// Network address utilities and calculations
pub struct NetworkUtils;

impl NetworkUtils {
    /// Parse CIDR notation and return network and host parts
    pub fn parse_cidr(cidr: &str) -> NetResult<(IpAddr, u8, IpAddr, IpAddr)> {
        let parts: Vec<&str> = cidr.split('/').collect();
        if parts.len() != 2 {
            return Err(CursedError::new("Invalid CIDR notation"));
        }

        let ip = IpAddr::from_str(parts[0])
            .map_err(|_| CursedError::new("Invalid IP address in CIDR"))?;
        let prefix_len = parts[1].parse::<u8>()
            .map_err(|_| CursedError::new("Invalid prefix length in CIDR"))?;

        let (network, broadcast) = match ip {
            IpAddr::V4(ipv4) => {
                let (net, bcast) = Self::ipv4_network_info(ipv4, prefix_len)?;
                (IpAddr::V4(net), IpAddr::V4(bcast))
            }
            IpAddr::V6(ipv6) => {
                let (net, bcast) = Self::ipv6_network_info(ipv6, prefix_len)?;
                (IpAddr::V6(net), IpAddr::V6(bcast))
            }
        };

        Ok((ip, prefix_len, network, broadcast))
    }

    /// Calculate IPv4 network and broadcast addresses
    pub fn ipv4_network_info(ip: Ipv4Addr, prefix_len: u8) -> NetResult<(Ipv4Addr, Ipv4Addr)> {
        if prefix_len > 32 {
            return Err(CursedError::new("Invalid IPv4 prefix length"));
        }

        let ip_u32 = u32::from(ip);
        let mask = if prefix_len == 0 { 0 } else { !((1u32 << (32 - prefix_len)) - 1) };
        
        let network_u32 = ip_u32 & mask;
        let broadcast_u32 = network_u32 | !mask;

        Ok((Ipv4Addr::from(network_u32), Ipv4Addr::from(broadcast_u32)))
    }

    /// Calculate IPv6 network information
    pub fn ipv6_network_info(ip: Ipv6Addr, prefix_len: u8) -> NetResult<(Ipv6Addr, Ipv6Addr)> {
        if prefix_len > 128 {
            return Err(CursedError::new("Invalid IPv6 prefix length"));
        }

        let ip_segments = ip.segments();
        let mut network_segments = [0u16; 8];
        let mut broadcast_segments = [0u16; 8];

        let full_segments = prefix_len / 16;
        let remaining_bits = prefix_len % 16;

        // Copy full segments
        for i in 0..full_segments as usize {
            network_segments[i] = ip_segments[i];
            broadcast_segments[i] = ip_segments[i];
        }

        // Handle partial segment
        if full_segments < 8 && remaining_bits > 0 {
            let mask = !((1u16 << (16 - remaining_bits)) - 1);
            network_segments[full_segments as usize] = ip_segments[full_segments as usize] & mask;
            broadcast_segments[full_segments as usize] = ip_segments[full_segments as usize] | !mask;
        }

        // Set remaining segments for broadcast
        for i in (full_segments as usize + if remaining_bits > 0 { 1 } else { 0 })..8 {
            broadcast_segments[i] = 0xFFFF;
        }

        Ok((
            Ipv6Addr::from(network_segments),
            Ipv6Addr::from(broadcast_segments)
        ))
    }

    /// Check if an IP address is in a given network
    pub fn ip_in_network(ip: IpAddr, network: IpAddr, prefix_len: u8) -> NetResult<bool> {
        match (ip, network) {
            (IpAddr::V4(ip4), IpAddr::V4(net4)) => {
                Self::ipv4_in_network(ip4, net4, prefix_len)
            }
            (IpAddr::V6(ip6), IpAddr::V6(net6)) => {
                Self::ipv6_in_network(ip6, net6, prefix_len)
            }
            _ => Ok(false), // Different IP versions
        }
    }

    /// Check if IPv4 address is in network
    pub fn ipv4_in_network(ip: Ipv4Addr, network: Ipv4Addr, prefix_len: u8) -> NetResult<bool> {
        if prefix_len > 32 {
            return Err(CursedError::new("Invalid IPv4 prefix length"));
        }

        let ip_u32 = u32::from(ip);
        let net_u32 = u32::from(network);
        let mask = if prefix_len == 0 { 0 } else { !((1u32 << (32 - prefix_len)) - 1) };

        Ok((ip_u32 & mask) == (net_u32 & mask))
    }

    /// Check if IPv6 address is in network
    pub fn ipv6_in_network(ip: Ipv6Addr, network: Ipv6Addr, prefix_len: u8) -> NetResult<bool> {
        if prefix_len > 128 {
            return Err(CursedError::new("Invalid IPv6 prefix length"));
        }

        let ip_segments = ip.segments();
        let net_segments = network.segments();

        let full_segments = prefix_len / 16;
        let remaining_bits = prefix_len % 16;

        // Check full segments
        for i in 0..full_segments as usize {
            if ip_segments[i] != net_segments[i] {
                return Ok(false);
            }
        }

        // Check partial segment
        if full_segments < 8 && remaining_bits > 0 {
            let mask = !((1u16 << (16 - remaining_bits)) - 1);
            if (ip_segments[full_segments as usize] & mask) != (net_segments[full_segments as usize] & mask) {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Calculate the number of host addresses in a network
    pub fn host_count(prefix_len: u8, ipv6: bool) -> NetResult<u128> {
        let max_bits = if ipv6 { 128 } else { 32 };
        
        if prefix_len > max_bits {
            return Err(CursedError::new("Invalid prefix length"));
        }

        let host_bits = max_bits - prefix_len;
        if host_bits == 0 {
            Ok(1) // Single host
        } else if host_bits >= 64 && !ipv6 {
            Err(CursedError::new("Host count too large for u128"))
        } else {
            Ok(1u128 << host_bits)
        }
    }

    /// Convert bytes to human-readable format
    pub fn format_bytes(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];
        let mut size = bytes as f64;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        if unit_index == 0 {
            format!("{} {}", bytes, UNITS[unit_index])
        } else {
            format!("{:.2} {}", size, UNITS[unit_index])
        }
    }

    /// Convert bandwidth to human-readable format
    pub fn format_bandwidth(bytes_per_sec: f64) -> String {
        let formatted_bytes = Self::format_bytes(bytes_per_sec as u64);
        format!("{}/s", formatted_bytes)
    }

    /// Parse a socket address from string with default port
    pub fn parse_socket_addr(addr: &str, default_port: u16) -> NetResult<SocketAddr> {
        // Try parsing as-is first
        if let Ok(socket_addr) = SocketAddr::from_str(addr) {
            return Ok(socket_addr);
        }

        // Try parsing as IP and add default port
        if let Ok(ip) = IpAddr::from_str(addr) {
            return Ok(SocketAddr::new(ip, default_port));
        }

        // Try parsing as hostname:port
        if let Some(colon_pos) = addr.rfind(':') {
            let host = &addr[..colon_pos];
            let port_str = &addr[colon_pos + 1..];
            
            if let Ok(port) = port_str.parse::<u16>() {
                if let Ok(ip) = IpAddr::from_str(host) {
                    return Ok(SocketAddr::new(ip, port));
                }
            }
        }

        Err(CursedError::new(&format!("Invalid socket address: {}", addr)))
    }

    /// Get local network interfaces (simplified)
    pub fn get_local_interfaces() -> NetResult<Vec<NetworkInterface>> {
        // This is a simplified implementation
        // In a real implementation, this would use system calls to get actual interfaces
        let mut interfaces = Vec::new();
        
        // Add localhost
        interfaces.push(NetworkInterface {
            name: "lo".to_string(),
            ip_addr: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            is_up: true,
            is_loopback: true,
            mtu: 65536,
        });

        // Add common private network interfaces (this would be detected in real implementation)
        interfaces.push(NetworkInterface {
            name: "eth0".to_string(),
            ip_addr: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
            is_up: true,
            is_loopback: false,
            mtu: 1500,
        });

        Ok(interfaces)
    }
}

#[derive(Debug, Clone)]
pub struct NetworkInterface {
    pub name: String,
    pub ip_addr: IpAddr,
    pub is_up: bool,
    pub is_loopback: bool,
    pub mtu: u32,
}

/// Network topology discovery utilities
pub struct TopologyDiscovery {
    discovered_hosts: HashMap<IpAddr, HostInfo>,
    scan_timeout: Duration,
}

#[derive(Debug, Clone)]
pub struct HostInfo {
    pub ip_addr: IpAddr,
    pub hostname: Option<String>,
    pub mac_address: Option<String>,
    pub open_ports: Vec<u16>,
    pub last_seen: SystemTime,
    pub response_time: Option<Duration>,
}

impl TopologyDiscovery {
    pub fn new() -> Self {
        Self {
            discovered_hosts: HashMap::new(),
            scan_timeout: Duration::from_secs(3),
        }
    }

    /// Discover hosts in a network range (simplified implementation)
    pub fn discover_network(&mut self, network: &str) -> NetResult<Vec<HostInfo>> {
        // Parse the network CIDR
        let (base_ip, prefix_len, network_addr, _broadcast_addr) = NetworkUtils::parse_cidr(network)?;

        // For simplicity, only handle small IPv4 networks
        if let IpAddr::V4(ipv4_net) = network_addr {
            if prefix_len < 24 {
                return Err(CursedError::new("Network too large for discovery"));
            }

            let base_u32 = u32::from(ipv4_net);
            let host_count = (1u32 << (32 - prefix_len)).min(254); // Limit scan size

            let mut discovered = Vec::new();

            for i in 1..=host_count {
                let ip = Ipv4Addr::from(base_u32 + i);
                
                // Simulate host discovery (in real implementation, this would ping or probe)
                if self.simulate_host_probe(IpAddr::V4(ip)) {
                    let host_info = HostInfo {
                        ip_addr: IpAddr::V4(ip),
                        hostname: None, // Would be resolved via reverse DNS
                        mac_address: None, // Would be obtained via ARP
                        open_ports: vec![], // Would be scanned
                        last_seen: SystemTime::now(),
                        response_time: Some(Duration::from_millis(10)), // Simulated
                    };

                    self.discovered_hosts.insert(IpAddr::V4(ip), host_info.clone());
                    discovered.push(host_info);
                }
            }

            Ok(discovered)
        } else {
            Err(CursedError::new("IPv6 network discovery not implemented"))
        }
    }

    /// Get discovered hosts
    pub fn get_discovered_hosts(&self) -> Vec<&HostInfo> {
        self.discovered_hosts.values().collect()
    }

    /// Simulate host probe (placeholder for actual ping/probe implementation)
    fn simulate_host_probe(&self, _ip: IpAddr) -> bool {
        // Simulate some hosts being available
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        _ip.hash(&mut hasher);
        let hash = hasher.finish();
        
        // ~30% of IPs are "available"
        (hash % 100) < 30
    }
}

/// Network performance testing utilities
pub struct NetworkTester {
    pub test_duration: Duration,
    pub packet_size: usize,
    pub concurrent_streams: usize,
}

impl NetworkTester {
    pub fn new() -> Self {
        Self {
            test_duration: Duration::from_secs(10),
            packet_size: 1024,
            concurrent_streams: 1,
        }
    }

    /// Perform a bandwidth test (simplified simulation)
    pub fn test_bandwidth(&self, target: &str) -> NetResult<BandwidthTestResult> {
        // Simulate bandwidth test
        let start_time = Instant::now();
        
        // Simulate some processing time
        std::thread::sleep(Duration::from_millis(100));
        
        let elapsed = start_time.elapsed();
        let bytes_transferred = self.packet_size as u64 * 1000; // Simulate 1000 packets
        
        Ok(BandwidthTestResult {
            target: target.to_string(),
            duration: elapsed,
            bytes_sent: bytes_transferred,
            bytes_received: bytes_transferred,
            upload_bandwidth: bytes_transferred as f64 / elapsed.as_secs_f64(),
            download_bandwidth: bytes_transferred as f64 / elapsed.as_secs_f64(),
            latency: Duration::from_millis(10), // Simulated
            packet_loss: 0.0, // Simulated
        })
    }
}

#[derive(Debug, Clone)]
pub struct BandwidthTestResult {
    pub target: String,
    pub duration: Duration,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub upload_bandwidth: f64,
    pub download_bandwidth: f64,
    pub latency: Duration,
    pub packet_loss: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bandwidth_meter() {
        let mut meter = BandwidthMeter::new(Duration::from_secs(1), 100);
        meter.record_transfer(1000, Duration::from_millis(100), TransferDirection::Download);
        
        let stats = meter.bandwidth_stats();
        assert!(stats.download_bytes_per_sec > 0.0);
    }

    #[test]
    fn test_cidr_parsing() {
        let result = NetworkUtils::parse_cidr("192.168.1.0/24").unwrap();
        assert_eq!(result.1, 24);
    }

    #[test]
    fn test_ipv4_in_network() {
        let ip = Ipv4Addr::new(192, 168, 1, 100);
        let network = Ipv4Addr::new(192, 168, 1, 0);
        let result = NetworkUtils::ipv4_in_network(ip, network, 24).unwrap();
        assert!(result);
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(NetworkUtils::format_bytes(1024), "1.00 KB");
        assert_eq!(NetworkUtils::format_bytes(1048576), "1.00 MB");
    }

    #[test]
    fn test_host_count() {
        let count = NetworkUtils::host_count(24, false).unwrap();
        assert_eq!(count, 256);
    }

    #[test]
    fn test_socket_addr_parsing() {
        let addr = NetworkUtils::parse_socket_addr("127.0.0.1", 8080).unwrap();
        assert_eq!(addr.port(), 8080);
    }

    #[test]
    fn test_topology_discovery() {
        let mut discovery = TopologyDiscovery::new();
        assert_eq!(discovery.get_discovered_hosts().len(), 0);
    }

    #[test]
    fn test_network_tester() {
        let tester = NetworkTester::new();
        assert_eq!(tester.packet_size, 1024);
    }
}
