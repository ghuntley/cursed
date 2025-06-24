use crate::error::Error;
/// Network address types for VibeNet
/// 
/// This module provides comprehensive network address handling including TCP, UDP,
/// and Unix socket addresses with proper resolution and formatting capabilities.

use std::fmt;
use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6, ToSocketAddrs};
use std::path::PathBuf;
use crate::error::CursedError;
use super::ip::IPVibe;
use super::error::{NetError, address_resolution_error};
use super::NetResult;

/// AddrVibe trait represents a network endpoint address
pub trait AddrVibe: fmt::Display + fmt::Debug + Send + Sync {
    /// Get the network type
    fn network(&self) -> String;
    
    /// Get string representation of the address
    fn string(&self) -> String;
}

/// TCPAddrVibe represents a TCP network endpoint address
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TCPAddrVibe {
    addr: SocketAddr,
    zone: Option<String>, // IPv6 zone identifier
}

impl TCPAddrVibe {
    /// Create a new TCP address from IP and port
    pub fn new(ip: IPVibe, port: u16) -> TCPAddrVibe {
        let socket_addr = match ip.inner() {
            std::net::IpAddr::V4(ipv4) => SocketAddr::V4(SocketAddrV4::new(*ipv4, port)),
            std::net::IpAddr::V6(ipv6) => SocketAddr::V6(SocketAddrV6::new(*ipv6, port, 0, 0)),
        };
        
        TCPAddrVibe {
            addr: socket_addr,
            zone: None,
        }
    }
    
    /// Create from a SocketAddr
    pub fn from_socket_addr(addr: SocketAddr) -> TCPAddrVibe {
        TCPAddrVibe {
            addr,
            zone: None,
        }
    }
    
    /// Resolve a TCP address from network and address string
    pub fn resolve(network: &str, address: &str) -> NetResult<TCPAddrVibe> {
        // Validate network type
        match network {
            "tcp" | "tcp4" | "tcp6" => {},
            _ => return Err(CursedError::from(address_resolution_error(&format!("Invalid TCP network type: {}", network)))),
        }
        
        // Parse the address
        let socket_addrs: Vec<SocketAddr> = address.to_socket_addrs()
            .map_err(|e| CursedError::from(address_resolution_error(&format!("Failed to resolve TCP address '{}': {}", address, e))))?
            .collect();
        
        if socket_addrs.is_empty() {
            return Err(CursedError::from(address_resolution_error(&format!("No addresses found for '{}'", address))));
        }
        
        // Filter by network type if specified
        let filtered_addr = match network {
            "tcp4" => socket_addrs.iter().find(|addr| addr.is_ipv4()),
            "tcp6" => socket_addrs.iter().find(|addr| addr.is_ipv6()),
            _ => socket_addrs.first(),
        };
        
        match filtered_addr {
            Some(addr) => Ok(TCPAddrVibe::from_socket_addr(*addr)),
            None => Err(CursedError::from(address_resolution_error(&format!("No suitable {} address found for '{}'", network, address)))),
        }
    }
    
    /// Get the IP address
    pub fn ip(&self) -> IPVibe {
        IPVibe::parse_ip(&self.addr.ip().to_string()).unwrap()
    }
    
    /// Get the port number
    pub fn port(&self) -> i32 {
        self.addr.port() as i32
    }
    
    /// Get the IPv6 zone identifier
    pub fn zone(&self) -> String {
        self.zone.clone().unwrap_or_default()
    }
    
    /// Set the IPv6 zone identifier
    pub fn set_zone(&mut self, zone: String) {
        self.zone = Some(zone);
    }
    
    /// Get the underlying SocketAddr
    pub fn socket_addr(&self) -> SocketAddr {
        self.addr
    }
}

impl AddrVibe for TCPAddrVibe {
    fn network(&self) -> String {
        if self.addr.is_ipv4() {
            "tcp4".to_string()
        } else {
            "tcp6".to_string()
        }
    }
    
    fn string(&self) -> String {
        if let Some(zone) = &self.zone {
            format!("{}%{}", self.addr, zone)
        } else {
            self.addr.to_string()
        }
    }
}

impl fmt::Display for TCPAddrVibe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.string())
    }
}

/// UDPAddrVibe represents a UDP network endpoint address
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UDPAddrVibe {
    addr: SocketAddr,
    zone: Option<String>,
}

impl UDPAddrVibe {
    /// Create a new UDP address from IP and port
    pub fn new(ip: IPVibe, port: u16) -> UDPAddrVibe {
        let socket_addr = match ip.inner() {
            std::net::IpAddr::V4(ipv4) => SocketAddr::V4(SocketAddrV4::new(*ipv4, port)),
            std::net::IpAddr::V6(ipv6) => SocketAddr::V6(SocketAddrV6::new(*ipv6, port, 0, 0)),
        };
        
        UDPAddrVibe {
            addr: socket_addr,
            zone: None,
        }
    }
    
    /// Create from a SocketAddr
    pub fn from_socket_addr(addr: SocketAddr) -> UDPAddrVibe {
        UDPAddrVibe {
            addr,
            zone: None,
        }
    }
    
    /// Resolve a UDP address from network and address string
    pub fn resolve(network: &str, address: &str) -> NetResult<UDPAddrVibe> {
        // Validate network type
        match network {
            "udp" | "udp4" | "udp6" => {},
            _ => return Err(CursedError::from(address_resolution_error(&format!("Invalid UDP network type: {}", network)))),
        }
        
        // Parse the address
        let socket_addrs: Vec<SocketAddr> = address.to_socket_addrs()
            .map_err(|e| CursedError::from(address_resolution_error(&format!("Failed to resolve UDP address '{}': {}", address, e))))?
            .collect();
        
        if socket_addrs.is_empty() {
            return Err(CursedError::from(address_resolution_error(&format!("No addresses found for '{}'", address))));
        }
        
        // Filter by network type if specified
        let filtered_addr = match network {
            "udp4" => socket_addrs.iter().find(|addr| addr.is_ipv4()),
            "udp6" => socket_addrs.iter().find(|addr| addr.is_ipv6()),
            _ => socket_addrs.first(),
        };
        
        match filtered_addr {
            Some(addr) => Ok(UDPAddrVibe::from_socket_addr(*addr)),
            None => Err(CursedError::from(address_resolution_error(&format!("No suitable {} address found for '{}'", network, address)))),
        }
    }
    
    /// Get the IP address
    pub fn ip(&self) -> IPVibe {
        IPVibe::parse_ip(&self.addr.ip().to_string()).unwrap()
    }
    
    /// Get the port number
    pub fn port(&self) -> i32 {
        self.addr.port() as i32
    }
    
    /// Get the IPv6 zone identifier
    pub fn zone(&self) -> String {
        self.zone.clone().unwrap_or_default()
    }
    
    /// Set the IPv6 zone identifier
    pub fn set_zone(&mut self, zone: String) {
        self.zone = Some(zone);
    }
    
    /// Get the underlying SocketAddr
    pub fn socket_addr(&self) -> SocketAddr {
        self.addr
    }
}

impl AddrVibe for UDPAddrVibe {
    fn network(&self) -> String {
        if self.addr.is_ipv4() {
            "udp4".to_string()
        } else {
            "udp6".to_string()
        }
    }
    
    fn string(&self) -> String {
        if let Some(zone) = &self.zone {
            format!("{}%{}", self.addr, zone)
        } else {
            self.addr.to_string()
        }
    }
}

impl fmt::Display for UDPAddrVibe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.string())
    }
}

/// UnixAddrVibe represents a Unix domain socket address
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnixAddrVibe {
    path: PathBuf,
    network_type: String,
}

impl UnixAddrVibe {
    /// Create a new Unix address from path
    pub fn new(path: PathBuf, network_type: &str) -> UnixAddrVibe {
        UnixAddrVibe {
            path,
            network_type: network_type.to_string(),
        }
    }
    
    /// Resolve a Unix address from network and address string
    pub fn resolve(network: &str, address: &str) -> NetResult<UnixAddrVibe> {
        // Validate network type
        match network {
            "unix" | "unixgram" | "unixpacket" => {},
            _ => return Err(CursedError::from(address_resolution_error(&format!("Invalid Unix network type: {}", network)))),
        }
        
        let path = PathBuf::from(address);
        Ok(UnixAddrVibe::new(path, network))
    }
    
    /// Get the socket path name
    pub fn name(&self) -> String {
        self.path.to_string_lossy().to_string()
    }
    
    /// Get the path as PathBuf
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
    
    /// Get the network type
    pub fn get_network_type(&self) -> &str {
        &self.network_type
    }
}

impl AddrVibe for UnixAddrVibe {
    fn network(&self) -> String {
        self.network_type.clone()
    }
    
    fn string(&self) -> String {
        self.name()
    }
}

impl fmt::Display for UnixAddrVibe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tcp_addr_creation() {
        let ip = IPVibe::parse_ip("192.168.1.1").unwrap();
        let addr = TCPAddrVibe::new(ip, 8080);
        assert_eq!(addr.port(), 8080);
        assert_eq!(addr.network(), "tcp4");
    }

    #[test]
    fn test_tcp_addr_resolve() {
        let addr = TCPAddrVibe::resolve("tcp", "127.0.0.1:8080").unwrap();
        assert_eq!(addr.port(), 8080);
        assert!(addr.ip().is_ipv4());
    }

    #[test]
    fn test_udp_addr_creation() {
        let ip = IPVibe::parse_ip("::1").unwrap();
        let addr = UDPAddrVibe::new(ip, 9090);
        assert_eq!(addr.port(), 9090);
        assert_eq!(addr.network(), "udp6");
    }

    #[test]
    fn test_udp_addr_resolve() {
        let addr = UDPAddrVibe::resolve("udp", "localhost:5000").unwrap();
        assert_eq!(addr.port(), 5000);
    }

    #[test]
    fn test_unix_addr_creation() {
        let addr = UnixAddrVibe::resolve("unix", "/tmp/test.sock").unwrap();
        assert_eq!(addr.name(), "/tmp/test.sock");
        assert_eq!(addr.network(), "unix");
    }

    #[test]
    fn test_invalid_network_type() {
        let result = TCPAddrVibe::resolve("invalid", "127.0.0.1:8080");
        assert!(result.is_err());
    }

    #[test]
    fn test_zone_handling() {
        let ip = IPVibe::parse_ip("::1").unwrap();
        let mut addr = TCPAddrVibe::new(ip, 8080);
        addr.set_zone("eth0".to_string());
        assert_eq!(addr.zone(), "eth0");
    }

    #[test]
    fn test_addr_vibe_trait() {
        let ip = IPVibe::parse_ip("127.0.0.1").unwrap();
        let addr = TCPAddrVibe::new(ip, 8080);
        let addr_vibe: &dyn AddrVibe = &addr;
        
        assert_eq!(addr_vibe.network(), "tcp4");
        assert!(addr_vibe.string().contains("127.0.0.1:8080"));
    }
}
