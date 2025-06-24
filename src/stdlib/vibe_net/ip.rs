use crate::error::Error;
/// IP address types and utilities for VibeNet
/// 
/// This module provides comprehensive IP address handling including IPv4 and IPv6
/// support, network address parsing, and CIDR notation handling.

use std::fmt;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;
use crate::error::CursedError;
use super::error::{NetError, address_resolution_error};
use super::NetResult;

/// IPVibe represents an IP address (IPv4 or IPv6)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IPVibe {
    addr: IpAddr,
}

impl IPVibe {
    /// Parse an IP address from a string
    pub fn parse_ip(s: &str) -> NetResult<IPVibe> {
        match IpAddr::from_str(s) {
            Ok(addr) => Ok(IPVibe { addr }),
            Err(e) => Err(CursedError::from(address_resolution_error(&format!("Failed to parse IP address '{}': {}", s, e))))
        }
    }
    
    /// Create an IPv4 address from four octets
    pub fn ipv4(a: u8, b: u8, c: u8, d: u8) -> IPVibe {
        IPVibe {
            addr: IpAddr::V4(Ipv4Addr::new(a, b, c, d))
        }
    }
    
    /// Create an IPv6 address from eight 16-bit segments
    pub fn ipv6(segments: [u16; 8]) -> IPVibe {
        IPVibe {
            addr: IpAddr::V6(Ipv6Addr::new(
                segments[0], segments[1], segments[2], segments[3],
                segments[4], segments[5], segments[6], segments[7]
            ))
        }
    }
    
    /// Get the underlying IP address
    pub fn inner(&self) -> &IpAddr {
        &self.addr
    }
    
    /// Convert to string representation
    pub fn string(&self) -> String {
        self.addr.to_string()
    }
    
    /// Check if this is a loopback address
    pub fn is_loopback(&self) -> bool {
        self.addr.is_loopback()
    }
    
    /// Check if this is a multicast address
    pub fn is_multicast(&self) -> bool {
        self.addr.is_multicast()
    }
    
    /// Check if this is a global unicast address
    pub fn is_global_unicast(&self) -> bool {
        match &self.addr {
            IpAddr::V4(addr) => !addr.is_private() && !addr.is_loopback() && !addr.is_multicast(),
            IpAddr::V6(addr) => addr.is_unicast_global(),
        }
    }
    
    /// Check if this is a link-local unicast address
    pub fn is_link_local_unicast(&self) -> bool {
        match &self.addr {
            IpAddr::V4(addr) => addr.octets()[0] == 169 && addr.octets()[1] == 254,
            IpAddr::V6(addr) => addr.is_unicast_link_local(),
        }
    }
    
    /// Check if this is a link-local multicast address
    pub fn is_link_local_multicast(&self) -> bool {
        match &self.addr {
            IpAddr::V4(addr) => {
                let octets = addr.octets();
                octets[0] == 224 && octets[1] == 0 && octets[2] == 0
            },
            IpAddr::V6(addr) => {
                let segments = addr.segments();
                segments[0] & 0xff0f == 0xff02
            }
        }
    }
    
    /// Check if this is an interface-local multicast address (IPv6 only)
    pub fn is_interface_local_multicast(&self) -> bool {
        match &self.addr {
            IpAddr::V4(_) => false,
            IpAddr::V6(addr) => {
                let segments = addr.segments();
                segments[0] & 0xff0f == 0xff01
            }
        }
    }
    
    /// Check if this is a private address
    pub fn is_private(&self) -> bool {
        match &self.addr {
            IpAddr::V4(addr) => addr.is_private(),
            IpAddr::V6(addr) => {
                // IPv6 unique local addresses (fc00::/7)
                let segments = addr.segments();
                (segments[0] & 0xfe00) == 0xfc00
            }
        }
    }
    
    /// Check if this is an unspecified address
    pub fn is_unspecified(&self) -> bool {
        self.addr.is_unspecified()
    }
    
    /// Convert to IPv4 address (returns None if not IPv4)
    pub fn to4(&self) -> Option<IPVibe> {
        match &self.addr {
            IpAddr::V4(_) => Some(self.clone()),
            IpAddr::V6(addr) => {
                if let Some(ipv4) = addr.to_ipv4() {
                    Some(IPVibe { addr: IpAddr::V4(ipv4) })
                } else {
                    None
                }
            }
        }
    }
    
    /// Convert to IPv6 address (maps IPv4 if necessary)
    pub fn to16(&self) -> IPVibe {
        match &self.addr {
            IpAddr::V4(addr) => IPVibe { addr: IpAddr::V6(addr.to_ipv6_mapped()) },
            IpAddr::V6(_) => self.clone(),
        }
    }
    
    /// Check if two IP addresses are equal
    pub fn equal(&self, other: &IPVibe) -> bool {
        self.addr == other.addr
    }
    
    /// Check if this is an IPv4 address
    pub fn is_ipv4(&self) -> bool {
        matches!(self.addr, IpAddr::V4(_))
    }
    
    /// Check if this is an IPv6 address
    pub fn is_ipv6(&self) -> bool {
        matches!(self.addr, IpAddr::V6(_))
    }
    
    /// Marshal to text representation
    pub fn marshal_text(&self) -> NetResult<Vec<u8>> {
        Ok(self.string().into_bytes())
    }
    
    /// Unmarshal from text representation
    pub fn unmarshal_text(text: &[u8]) -> NetResult<IPVibe> {
        let s = std::str::from_utf8(text)
            .map_err(|e| CursedError::from(address_resolution_error(&format!("Invalid UTF-8 in IP address: {}", e))))?;
        Self::parse_ip(s)
    }
}

impl fmt::Display for IPVibe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.addr)
    }
}

impl FromStr for IPVibe {
    type Err = CursedError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_ip(s)
    }
}

/// IPMaskVibe represents an IP subnet mask
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IPMaskVibe {
    mask: Vec<u8>,
}

impl IPMaskVibe {
    /// Create an IPv4 subnet mask from four octets
    pub fn ipv4_mask(a: u8, b: u8, c: u8, d: u8) -> IPMaskVibe {
        IPMaskVibe {
            mask: vec![a, b, c, d],
        }
    }
    
    /// Create a subnet mask from CIDR notation
    pub fn cidr_mask(ones: i32, bits: i32) -> NetResult<IPMaskVibe> {
        if ones < 0 || bits <= 0 || ones > bits {
            return Err(CursedError::from(address_resolution_error("Invalid CIDR mask parameters")));
        }
        
        let bytes_count = (bits + 7) / 8;
        let mut mask = vec![0u8; bytes_count as usize];
        
        let mut remaining_ones = ones;
        for i in 0..bytes_count {
            if remaining_ones >= 8 {
                mask[i as usize] = 0xff;
                remaining_ones -= 8;
            } else if remaining_ones > 0 {
                mask[i as usize] = (0xff << (8 - remaining_ones)) & 0xff;
                remaining_ones = 0;
            }
        }
        
        Ok(IPMaskVibe { mask })
    }
    
    /// Convert to string representation
    pub fn string(&self) -> String {
        if self.mask.len() == 4 {
            format!("{}.{}.{}.{}", self.mask[0], self.mask[1], self.mask[2], self.mask[3])
        } else {
            // IPv6 mask representation
            self.mask.iter()
                .map(|b| format!("{:02x}", b))
                .collect::<Vec<_>>()
                .join(":")
        }
    }
    
    /// Get the size of the subnet mask
    pub fn size(&self) -> (i32, i32) {
        let bits = self.mask.len() as i32 * 8;
        let mut ones = 0;
        
        for &byte in &self.mask {
            ones += byte.count_ones() as i32;
        }
        
        (ones, bits)
    }
    
    /// Get the raw mask bytes
    pub fn bytes(&self) -> &[u8] {
        &self.mask
    }
}

impl fmt::Display for IPMaskVibe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.string())
    }
}

/// IPNetVibe represents an IP network (IP address + subnet mask)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IPNetVibe {
    pub ip: IPVibe,
    pub mask: IPMaskVibe,
    pub prefix_len: i32,
}

impl IPNetVibe {
    /// Parse a CIDR notation string into IP and network
    pub fn parse_cidr(s: &str) -> NetResult<(IPVibe, IPNetVibe)> {
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() != 2 {
            return Err(CursedError::from(address_resolution_error("Invalid CIDR notation")));
        }
        
        let ip = IPVibe::parse_ip(parts[0])?;
        let prefix_len: i32 = parts[1].parse()
            .map_err(|_| CursedError::from(address_resolution_error("Invalid prefix length")))?;
        
        let bits = match ip.addr {
            IpAddr::V4(_) => 32,
            IpAddr::V6(_) => 128,
        };
        
        if prefix_len < 0 || prefix_len > bits {
            return Err(CursedError::from(address_resolution_error("Prefix length out of range")));
        }
        
        let mask = IPMaskVibe::cidr_mask(prefix_len, bits)?;
        
        let net = IPNetVibe {
            ip: ip.clone(),
            mask,
            prefix_len,
        };
        
        Ok((ip, net))
    }
    
    /// Check if the network contains the given IP address
    pub fn contains(&self, ip: &IPVibe) -> bool {
        // Both addresses must be the same type (IPv4 or IPv6)
        if self.ip.is_ipv4() != ip.is_ipv4() {
            return false;
        }
        
        match (&self.ip.addr, &ip.addr) {
            (IpAddr::V4(net_ip), IpAddr::V4(test_ip)) => {
                let net_octets = net_ip.octets();
                let test_octets = test_ip.octets();
                let mask_bytes = &self.mask.mask;
                
                if mask_bytes.len() != 4 {
                    return false;
                }
                
                for i in 0..4 {
                    if (net_octets[i] & mask_bytes[i]) != (test_octets[i] & mask_bytes[i]) {
                        return false;
                    }
                }
                true
            },
            (IpAddr::V6(net_ip), IpAddr::V6(test_ip)) => {
                let net_octets = net_ip.octets();
                let test_octets = test_ip.octets();
                let mask_bytes = &self.mask.mask;
                
                if mask_bytes.len() != 16 {
                    return false;
                }
                
                for i in 0..16 {
                    if (net_octets[i] & mask_bytes[i]) != (test_octets[i] & mask_bytes[i]) {
                        return false;
                    }
                }
                true
            },
            _ => false,
        }
    }
    
    /// Get the network identifier
    pub fn network(&self) -> String {
        if self.ip.is_ipv4() {
            "ip+net".to_string()
        } else {
            "ip6+net".to_string()
        }
    }
    
    /// Convert to string representation
    pub fn string(&self) -> String {
        format!("{}/{}", self.ip, self.prefix_len)
    }
}

impl fmt::Display for IPNetVibe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ipv4() {
        let ip = IPVibe::parse_ip("192.168.1.1").unwrap();
        assert!(ip.is_ipv4());
        assert_eq!(ip.string(), "192.168.1.1");
    }

    #[test]
    fn test_parse_ipv6() {
        let ip = IPVibe::parse_ip("2001:db8::1").unwrap();
        assert!(ip.is_ipv6());
        assert_eq!(ip.string(), "2001:db8::1");
    }

    #[test]
    fn test_ipv4_creation() {
        let ip = IPVibe::ipv4(192, 168, 1, 1);
        assert!(ip.is_ipv4());
        assert_eq!(ip.string(), "192.168.1.1");
    }

    #[test]
    fn test_ip_properties() {
        let localhost = IPVibe::parse_ip("127.0.0.1").unwrap();
        assert!(localhost.is_loopback());
        assert!(!localhost.is_multicast());

        let private = IPVibe::parse_ip("192.168.1.1").unwrap();
        assert!(private.is_private());
        assert!(!private.is_global_unicast());
    }

    #[test]
    fn test_cidr_parsing() {
        let (ip, net) = IPNetVibe::parse_cidr("192.168.1.0/24").unwrap();
        assert_eq!(ip.string(), "192.168.1.0");
        assert_eq!(net.prefix_len, 24);
        assert_eq!(net.string(), "192.168.1.0/24");
    }

    #[test]
    fn test_network_contains() {
        let (_, net) = IPNetVibe::parse_cidr("192.168.1.0/24").unwrap();
        let ip1 = IPVibe::parse_ip("192.168.1.100").unwrap();
        let ip2 = IPVibe::parse_ip("192.168.2.100").unwrap();
        
        assert!(net.contains(&ip1));
        assert!(!net.contains(&ip2));
    }

    #[test]
    fn test_ipv4_mask() {
        let mask = IPMaskVibe::ipv4_mask(255, 255, 255, 0);
        assert_eq!(mask.string(), "255.255.255.0");
        let (ones, bits) = mask.size();
        assert_eq!(ones, 24);
        assert_eq!(bits, 32);
    }

    #[test]
    fn test_cidr_mask() {
        let mask = IPMaskVibe::cidr_mask(24, 32).unwrap();
        let (ones, bits) = mask.size();
        assert_eq!(ones, 24);
        assert_eq!(bits, 32);
    }
}
