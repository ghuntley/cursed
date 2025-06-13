/// Network interface enumeration and management for CURSED networking
/// 
/// This module provides functionality to discover and query network interfaces
/// on the local system, including interface properties, statistics, and
/// configuration details.

use std::collections::HashMap;
use crate::stdlib::net::error::{NetError, NetResult, general_error};
use crate::stdlib::net::address::{IpAddr, SocketAddr};

/// Network interface type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceType {
    Loopback,
    Ethernet,
    Wireless,
    Ppp,
    Tunnel,
    Bridge,
    Virtual,
    Unknown,
}

impl std::fmt::Display for InterfaceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterfaceType::Loopback => write!(f, "Loopback"),
            InterfaceType::Ethernet => write!(f, "Ethernet"),
            InterfaceType::Wireless => write!(f, "Wireless"),
            InterfaceType::Ppp => write!(f, "PPP"),
            InterfaceType::Tunnel => write!(f, "Tunnel"),
            InterfaceType::Bridge => write!(f, "Bridge"),
            InterfaceType::Virtual => write!(f, "Virtual"),
            InterfaceType::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Network interface statistics
#[derive(Debug, Clone)]
pub struct InterfaceStats {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub packets_sent: u64,
    pub packets_received: u64,
    pub errors_in: u64,
    pub errors_out: u64,
    pub dropped_in: u64,
    pub dropped_out: u64,
    pub collisions: u64,
}

impl Default for InterfaceStats {
    fn default() -> Self {
        Self {
            bytes_sent: 0,
            bytes_received: 0,
            packets_sent: 0,
            packets_received: 0,
            errors_in: 0,
            errors_out: 0,
            dropped_in: 0,
            dropped_out: 0,
            collisions: 0,
        }
    }
}

/// Network interface configuration
#[derive(Debug, Clone)]
pub struct InterfaceConfig {
    pub mtu: u32,
    pub up: bool,
    pub running: bool,
    pub multicast: bool,
    pub broadcast: bool,
    pub point_to_point: bool,
    pub loopback: bool,
}

impl Default for InterfaceConfig {
    fn default() -> Self {
        Self {
            mtu: 1500,
            up: false,
            running: false,
            multicast: false,
            broadcast: false,
            point_to_point: false,
            loopback: false,
        }
    }
}

/// Network interface information
#[derive(Debug, Clone)]
pub struct NetworkInterface {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub interface_type: InterfaceType,
    pub hardware_address: Option<String>, // MAC address
    pub ip_addresses: Vec<IpAddr>,
    pub netmask: Option<IpAddr>,
    pub broadcast_address: Option<IpAddr>,
    pub gateway: Option<IpAddr>,
    pub dns_servers: Vec<IpAddr>,
    pub config: InterfaceConfig,
    pub stats: InterfaceStats,
    pub index: u32,
}

impl NetworkInterface {
    /// Create a new network interface
    pub fn new(name: String, index: u32) -> Self {
        Self {
            name: name.clone(),
            display_name: name.clone(),
            description: String::new(),
            interface_type: InterfaceType::Unknown,
            hardware_address: None,
            ip_addresses: Vec::new(),
            netmask: None,
            broadcast_address: None,
            gateway: None,
            dns_servers: Vec::new(),
            config: InterfaceConfig::default(),
            stats: InterfaceStats::default(),
            index,
        }
    }
    
    /// Check if interface is up and running
    pub fn is_active(&self) -> bool {
        self.config.up && self.config.running
    }
    
    /// Check if interface is a loopback interface
    pub fn is_loopback(&self) -> bool {
        self.interface_type == InterfaceType::Loopback || self.config.loopback
    }
    
    /// Get primary IP address (first non-loopback if available)
    pub fn primary_ip(&self) -> Option<IpAddr> {
        if self.is_loopback() {
            self.ip_addresses.first().copied()
        } else {
            self.ip_addresses.iter()
                .find(|ip| !ip.is_loopback())
                .copied()
                .or_else(|| self.ip_addresses.first().copied())
        }
    }
    
    /// Get IPv4 addresses only
    pub fn ipv4_addresses(&self) -> Vec<IpAddr> {
        self.ip_addresses.iter()
            .filter(|ip| ip.is_ipv4())
            .copied()
            .collect()
    }
    
    /// Get IPv6 addresses only
    pub fn ipv6_addresses(&self) -> Vec<IpAddr> {
        self.ip_addresses.iter()
            .filter(|ip| ip.is_ipv6())
            .copied()
            .collect()
    }
    
    /// Calculate total bandwidth (bytes/second estimate)
    pub fn bandwidth_estimate(&self) -> f64 {
        // Simple estimate based on interface type
        match self.interface_type {
            InterfaceType::Loopback => 1_000_000_000.0, // 1 Gbps
            InterfaceType::Ethernet => 100_000_000.0,   // 100 Mbps
            InterfaceType::Wireless => 54_000_000.0,    // 54 Mbps
            InterfaceType::Ppp => 1_544_000.0,          // T1 line
            _ => 10_000_000.0,                          // 10 Mbps default
        }
    }
    
    /// Format interface information as string
    pub fn format_info(&self) -> String {
        let mut info = format!("Interface: {} ({})\n", self.name, self.display_name);
        info.push_str(&format!("  Type: {}\n", self.interface_type));
        info.push_str(&format!("  Index: {}\n", self.index));
        info.push_str(&format!("  Status: {}\n", if self.is_active() { "UP" } else { "DOWN" }));
        
        if let Some(mac) = &self.hardware_address {
            info.push_str(&format!("  MAC: {}\n", mac));
        }
        
        info.push_str(&format!("  MTU: {}\n", self.config.mtu));
        
        if !self.ip_addresses.is_empty() {
            info.push_str("  IP Addresses:\n");
            for ip in &self.ip_addresses {
                info.push_str(&format!("    {}\n", ip));
            }
        }
        
        if let Some(gateway) = &self.gateway {
            info.push_str(&format!("  Gateway: {}\n", gateway));
        }
        
        if !self.dns_servers.is_empty() {
            info.push_str("  DNS Servers:\n");
            for dns in &self.dns_servers {
                info.push_str(&format!("    {}\n", dns));
            }
        }
        
        info
    }
}

/// Network interface manager
#[derive(Debug)]
pub struct InterfaceManager {
    interfaces: HashMap<String, NetworkInterface>,
    last_update: std::time::Instant,
    cache_duration: std::time::Duration,
}

impl InterfaceManager {
    /// Create a new interface manager
    pub fn new() -> Self {
        Self {
            interfaces: HashMap::new(),
            last_update: std::time::Instant::now() - std::time::Duration::from_secs(3600),
            cache_duration: std::time::Duration::from_secs(60), // Cache for 1 minute
        }
    }
    
    /// Refresh interface list
    pub fn refresh(&mut self) -> NetResult<()> {
        self.interfaces.clear();
        self.discover_interfaces()?;
        self.last_update = std::time::Instant::now();
        Ok(())
    }
    
    /// Get all interfaces (refreshes if cache is stale)
    pub fn get_interfaces(&mut self) -> NetResult<&HashMap<String, NetworkInterface>> {
        if self.last_update.elapsed() > self.cache_duration {
            self.refresh()?;
        }
        Ok(&self.interfaces)
    }
    
    /// Get interface by name
    pub fn get_interface(&mut self, name: &str) -> NetResult<Option<&NetworkInterface>> {
        if self.last_update.elapsed() > self.cache_duration {
            self.refresh()?;
        }
        Ok(self.interfaces.get(name))
    }
    
    /// Get all active interfaces
    pub fn get_active_interfaces(&mut self) -> NetResult<Vec<&NetworkInterface>> {
        let interfaces = self.get_interfaces()?;
        Ok(interfaces.values()
            .filter(|iface| iface.is_active())
            .collect())
    }
    
    /// Get default interface (usually the one with default route)
    pub fn get_default_interface(&mut self) -> NetResult<Option<&NetworkInterface>> {
        let interfaces = self.get_interfaces()?;
        
        // Look for interface with gateway configured
        for iface in interfaces.values() {
            if iface.is_active() && !iface.is_loopback() && iface.gateway.is_some() {
                return Ok(Some(iface));
            }
        }
        
        // Fallback to first active non-loopback interface
        for iface in interfaces.values() {
            if iface.is_active() && !iface.is_loopback() {
                return Ok(Some(iface));
            }
        }
        
        Ok(None)
    }
    
    /// Platform-specific interface discovery
    fn discover_interfaces(&mut self) -> NetResult<()> {
        // This is a simplified implementation
        // In a real implementation, this would use platform-specific APIs
        
        #[cfg(unix)]
        {
            self.discover_unix_interfaces()?;
        }
        
        #[cfg(windows)]
        {
            self.discover_windows_interfaces()?;
        }
        
        // Always add loopback interface as fallback
        if !self.interfaces.contains_key("lo") && !self.interfaces.contains_key("loopback") {
            self.add_loopback_interface();
        }
        
        Ok(())
    }
    
    #[cfg(unix)]
    fn discover_unix_interfaces(&mut self) -> NetResult<()> {
        use std::process::Command;
        
        // Try to use 'ip' command to get interface information
        if let Ok(output) = Command::new("ip")
            .args(&["addr", "show"])
            .output()
        {
            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                self.parse_ip_addr_output(&output_str)?;
                return Ok(());
            }
        }
        
        // Fallback to parsing /proc/net/dev for basic interface names
        if let Ok(contents) = std::fs::read_to_string("/proc/net/dev") {
            self.parse_proc_net_dev(&contents)?;
        }
        
        Ok(())
    }
    
    #[cfg(windows)]
    fn discover_windows_interfaces(&mut self) -> NetResult<()> {
        use std::process::Command;
        
        // Try to use ipconfig to get interface information
        if let Ok(output) = Command::new("ipconfig")
            .args(&["/all"])
            .output()
        {
            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                self.parse_ipconfig_output(&output_str)?;
            }
        }
        
        Ok(())
    }
    
    fn parse_ip_addr_output(&mut self, output: &str) -> NetResult<()> {
        let mut current_interface: Option<NetworkInterface> = None;
        
        for line in output.lines() {
            let line = line.trim();
            
            // New interface line: "2: eth0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500"
            if line.contains(": ") && line.contains("mtu") {
                if let Some(iface) = current_interface.take() {
                    self.interfaces.insert(iface.name.clone(), iface);
                }
                
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let name_part = parts[1];
                    let name = name_part.trim_end_matches(':');
                    let index = parts[0].trim_end_matches(':').parse().unwrap_or(0);
                    
                    let mut iface = NetworkInterface::new(name.to_string(), index);
                    
                    // Parse flags
                    if line.contains("UP") {
                        iface.config.up = true;
                    }
                    if line.contains("RUNNING") {
                        iface.config.running = true;
                    }
                    if line.contains("LOOPBACK") {
                        iface.interface_type = InterfaceType::Loopback;
                        iface.config.loopback = true;
                    } else if name.starts_with("eth") || name.starts_with("enp") {
                        iface.interface_type = InterfaceType::Ethernet;
                    } else if name.starts_with("wlan") || name.starts_with("wlp") {
                        iface.interface_type = InterfaceType::Wireless;
                    }
                    
                    // Parse MTU
                    if let Some(mtu_pos) = line.find("mtu ") {
                        let mtu_str = &line[mtu_pos + 4..];
                        if let Some(space_pos) = mtu_str.find(' ') {
                            if let Ok(mtu) = mtu_str[..space_pos].parse::<u32>() {
                                iface.config.mtu = mtu;
                            }
                        }
                    }
                    
                    current_interface = Some(iface);
                }
            }
            // IP address line: "inet 192.168.1.100/24 brd 192.168.1.255 scope global eth0"
            else if line.starts_with("inet ") {
                if let Some(ref mut iface) = current_interface {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        let addr_with_prefix = parts[1];
                        if let Some(slash_pos) = addr_with_prefix.find('/') {
                            let addr_str = &addr_with_prefix[..slash_pos];
                            if let Ok(ip) = addr_str.parse::<IpAddr>() {
                                iface.ip_addresses.push(ip);
                            }
                        }
                    }
                }
            }
            // IPv6 address line: "inet6 ::1/128 scope host"
            else if line.starts_with("inet6 ") {
                if let Some(ref mut iface) = current_interface {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        let addr_with_prefix = parts[1];
                        if let Some(slash_pos) = addr_with_prefix.find('/') {
                            let addr_str = &addr_with_prefix[..slash_pos];
                            if let Ok(ip) = addr_str.parse::<IpAddr>() {
                                iface.ip_addresses.push(ip);
                            }
                        }
                    }
                }
            }
            // MAC address line: "link/ether 00:11:22:33:44:55 brd ff:ff:ff:ff:ff:ff"
            else if line.starts_with("link/ether ") {
                if let Some(ref mut iface) = current_interface {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        iface.hardware_address = Some(parts[1].to_string());
                    }
                }
            }
        }
        
        // Don't forget the last interface
        if let Some(iface) = current_interface {
            self.interfaces.insert(iface.name.clone(), iface);
        }
        
        Ok(())
    }
    
    fn parse_proc_net_dev(&mut self, contents: &str) -> NetResult<()> {
        for (index, line) in contents.lines().enumerate().skip(2) { // Skip header lines
            if let Some(colon_pos) = line.find(':') {
                let name = line[..colon_pos].trim().to_string();
                let mut iface = NetworkInterface::new(name.clone(), index as u32);
                
                // Determine interface type from name
                if name == "lo" {
                    iface.interface_type = InterfaceType::Loopback;
                    iface.config.loopback = true;
                } else if name.starts_with("eth") || name.starts_with("enp") {
                    iface.interface_type = InterfaceType::Ethernet;
                } else if name.starts_with("wlan") || name.starts_with("wlp") {
                    iface.interface_type = InterfaceType::Wireless;
                }
                
                // Assume interface is up (we can't tell from /proc/net/dev alone)
                iface.config.up = true;
                iface.config.running = true;
                
                self.interfaces.insert(name, iface);
            }
        }
        
        Ok(())
    }
    
    #[cfg(windows)]
    fn parse_ipconfig_output(&mut self, output: &str) -> NetResult<()> {
        let mut current_interface: Option<NetworkInterface> = None;
        let mut index = 0u32;
        
        for line in output.lines() {
            let line = line.trim();
            
            // New adapter line
            if line.starts_with("Ethernet adapter") || line.starts_with("Wireless LAN adapter") {
                if let Some(iface) = current_interface.take() {
                    self.interfaces.insert(iface.name.clone(), iface);
                }
                
                let name = if let Some(start) = line.find(' ') {
                    let rest = &line[start + 1..];
                    if let Some(end) = rest.find(':') {
                        rest[..end].to_string()
                    } else {
                        rest.to_string()
                    }
                } else {
                    format!("adapter{}", index)
                };
                
                let mut iface = NetworkInterface::new(name.clone(), index);
                iface.display_name = name.clone();
                
                if line.starts_with("Ethernet adapter") {
                    iface.interface_type = InterfaceType::Ethernet;
                } else if line.starts_with("Wireless LAN adapter") {
                    iface.interface_type = InterfaceType::Wireless;
                }
                
                current_interface = Some(iface);
                index += 1;
            }
            // IP address
            else if line.contains("IPv4 Address") {
                if let Some(ref mut iface) = current_interface {
                    if let Some(colon_pos) = line.find(':') {
                        let addr_part = line[colon_pos + 1..].trim();
                        // Remove any suffix like "(Preferred)"
                        let addr_str = addr_part.split('(').next().unwrap_or(addr_part).trim();
                        if let Ok(ip) = addr_str.parse::<IpAddr>() {
                            iface.ip_addresses.push(ip);
                        }
                    }
                }
            }
            // MAC address
            else if line.contains("Physical Address") {
                if let Some(ref mut iface) = current_interface {
                    if let Some(colon_pos) = line.find(':') {
                        let mac = line[colon_pos + 1..].trim().to_string();
                        iface.hardware_address = Some(mac);
                    }
                }
            }
        }
        
        // Don't forget the last interface
        if let Some(iface) = current_interface {
            self.interfaces.insert(iface.name.clone(), iface);
        }
        
        Ok(())
    }
    
    fn add_loopback_interface(&mut self) {
        let mut loopback = NetworkInterface::new("loopback".to_string(), 0);
        loopback.interface_type = InterfaceType::Loopback;
        loopback.config.loopback = true;
        loopback.config.up = true;
        loopback.config.running = true;
        
        // Add standard loopback addresses
        loopback.ip_addresses.push("127.0.0.1".parse().unwrap());
        loopback.ip_addresses.push("::1".parse().unwrap());
        
        self.interfaces.insert("loopback".to_string(), loopback);
    }
}

// Global interface manager
use std::sync::{Arc, Mutex};
use std::sync::OnceLock;

static GLOBAL_INTERFACE_MANAGER: OnceLock<Arc<Mutex<InterfaceManager>>> = OnceLock::new();

fn get_global_interface_manager() -> &'static Arc<Mutex<InterfaceManager>> {
    GLOBAL_INTERFACE_MANAGER.get_or_init(|| Arc::new(Mutex::new(InterfaceManager::new())))
}

/// List all network interfaces
pub fn list_interfaces() -> NetResult<Vec<NetworkInterface>> {
    let manager = get_global_interface_manager();
    let mut manager_guard = manager.lock().unwrap();
    let interfaces = manager_guard.get_interfaces()?;
    Ok(interfaces.values().cloned().collect())
}

/// Get interface by name
pub fn get_interface_by_name(name: &str) -> NetResult<Option<NetworkInterface>> {
    let manager = get_global_interface_manager();
    let mut manager_guard = manager.lock().unwrap();
    Ok(manager_guard.get_interface(name)?.cloned())
}

/// Get default network interface
pub fn get_default_interface() -> NetResult<Option<NetworkInterface>> {
    let manager = get_global_interface_manager();
    let mut manager_guard = manager.lock().unwrap();
    Ok(manager_guard.get_default_interface()?.cloned())
}

/// Get all active interfaces
pub fn get_active_interfaces() -> NetResult<Vec<NetworkInterface>> {
    let manager = get_global_interface_manager();
    let mut manager_guard = manager.lock().unwrap();
    let active_interfaces = manager_guard.get_active_interfaces()?;
    Ok(active_interfaces.into_iter().cloned().collect())
}

/// Refresh interface cache
pub fn refresh_interface_cache() -> NetResult<()> {
    let manager = get_global_interface_manager();
    let mut manager_guard = manager.lock().unwrap();
    manager_guard.refresh()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interface_type_display() {
        assert_eq!(InterfaceType::Ethernet.to_string(), "Ethernet");
        assert_eq!(InterfaceType::Wireless.to_string(), "Wireless");
        assert_eq!(InterfaceType::Loopback.to_string(), "Loopback");
    }

    #[test]
    fn test_interface_stats_default() {
        let stats = InterfaceStats::default();
        assert_eq!(stats.bytes_sent, 0);
        assert_eq!(stats.bytes_received, 0);
        assert_eq!(stats.packets_sent, 0);
    }

    #[test]
    fn test_interface_config_default() {
        let config = InterfaceConfig::default();
        assert_eq!(config.mtu, 1500);
        assert!(!config.up);
        assert!(!config.running);
    }

    #[test]
    fn test_network_interface_creation() {
        let iface = NetworkInterface::new("eth0".to_string(), 1);
        assert_eq!(iface.name, "eth0");
        assert_eq!(iface.index, 1);
        assert_eq!(iface.interface_type, InterfaceType::Unknown);
        assert!(!iface.is_active());
    }

    #[test]
    fn test_interface_properties() {
        let mut iface = NetworkInterface::new("lo".to_string(), 0);
        iface.interface_type = InterfaceType::Loopback;
        iface.config.loopback = true;
        iface.config.up = true;
        iface.config.running = true;
        iface.ip_addresses.push("127.0.0.1".parse().unwrap());
        
        assert!(iface.is_loopback());
        assert!(iface.is_active());
        assert_eq!(iface.primary_ip(), Some("127.0.0.1".parse().unwrap()));
        assert_eq!(iface.ipv4_addresses().len(), 1);
        assert_eq!(iface.ipv6_addresses().len(), 0);
    }

    #[test]
    fn test_interface_manager_creation() {
        let mut manager = InterfaceManager::new();
        assert!(manager.interfaces.is_empty());
        
        // Test refresh - should not panic
        let _ = manager.refresh();
    }

    #[test]
    fn test_interface_bandwidth_estimate() {
        let mut iface = NetworkInterface::new("eth0".to_string(), 1);
        iface.interface_type = InterfaceType::Ethernet;
        assert!(iface.bandwidth_estimate() > 0.0);
        
        iface.interface_type = InterfaceType::Wireless;
        assert!(iface.bandwidth_estimate() > 0.0);
    }

    #[test]
    fn test_interface_format_info() {
        let mut iface = NetworkInterface::new("eth0".to_string(), 1);
        iface.interface_type = InterfaceType::Ethernet;
        iface.config.up = true;
        iface.config.running = true;
        iface.ip_addresses.push("192.168.1.100".parse().unwrap());
        
        let info = iface.format_info();
        assert!(info.contains("eth0"));
        assert!(info.contains("Ethernet"));
        assert!(info.contains("UP"));
        assert!(info.contains("192.168.1.100"));
    }

    #[test]
    fn test_global_interface_functions() {
        // These should not panic
        let _ = list_interfaces();
        let _ = get_interface_by_name("nonexistent");
        let _ = get_default_interface();
        let _ = get_active_interfaces();
        let _ = refresh_interface_cache();
    }
}
