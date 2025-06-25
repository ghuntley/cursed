use crate::error::CursedError;
/// IP address and socket address handling for the CURSED networking module
/// 
/// This module provides comprehensive IP address handling including IPv4, IPv6,
/// socket addresses, address parsing, validation, and utility functions.

use std::fmt;
use std::str::FromStr;
use std::net::{IpAddr as StdIpAddr, Ipv4Addr as StdIpv4Addr, Ipv6Addr as StdIpv6Addr};
use std::net::{SocketAddr as StdSocketAddr, SocketAddrV4 as StdSocketAddrV4, SocketAddrV6 as StdSocketAddrV6};
// use crate::stdlib::net::error::{NetError, NetResult, address_error};

/// IP address enumeration supporting both IPv4 and IPv6
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IpAddr {
    V4(IpAddrV4),
    V6(IpAddrV6),
}

/// IPv4 address representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IpAddrV4 {
    octets: [u8; 4],
}

/// IPv6 address representation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IpAddrV6 {
    segments: [u16; 8],
}

/// Socket address enumeration supporting both IPv4 and IPv6
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SocketAddr {
    V4(SocketAddrV4),
    V6(SocketAddrV6),
}

/// IPv4 socket address (IP + port)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SocketAddrV4 {
    ip: IpAddrV4,
    port: u16,
}

/// IPv6 socket address (IP + port + flow info + scope ID)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SocketAddrV6 {
    ip: IpAddrV6,
    port: u16,
    flowinfo: u32,
    scope_id: u32,
}

// IPv4 Address Implementation
impl IpAddrV4 {
    /// Create a new IPv4 address from four octets
    pub fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        Self { octets: [a, b, c, d] }
    }
    
    /// Create IPv4 address from octets array
    pub fn from_octets(octets: [u8; 4]) -> Self {
        Self { octets }
    }
    
    /// Get the octets of the IPv4 address
    pub fn octets(&self) -> [u8; 4] {
        self.octets
    }
    
    /// Convert to 32-bit integer (network byte order)
    pub fn to_u32(&self) -> u32 {
        u32::from_be_bytes(self.octets)
    }
    
    /// Create IPv4 address from 32-bit integer (network byte order)
    pub fn from_u32(addr: u32) -> Self {
        Self { octets: addr.to_be_bytes() }
    }
    
    /// Check if this is a loopback address (127.0.0.0/8)
    pub fn is_loopback(&self) -> bool {
        self.octets[0] == 127
    }
    
    /// Check if this is a private address (RFC 1918)
    pub fn is_private(&self) -> bool {
        match self.octets {
            [10, ..] => true,
            [172, b, ..] if b >= 16 && b <= 31 => true,
            [192, 168, ..] => true,
            _ => false,
        }
    }
    
    /// Check if this is a link-local address (169.254.0.0/16)
    pub fn is_link_local(&self) -> bool {
        matches!(self.octets, [169, 254, ..])
    }
    
    /// Check if this is a multicast address (224.0.0.0/4)
    pub fn is_multicast(&self) -> bool {
        self.octets[0] >= 224 && self.octets[0] <= 239
    }
    
    /// Check if this is a broadcast address (255.255.255.255)
    pub fn is_broadcast(&self) -> bool {
        self.octets == [255, 255, 255, 255]
    }
    
    /// Check if this is an unspecified address (0.0.0.0)
    pub fn is_unspecified(&self) -> bool {
        self.octets == [0, 0, 0, 0]
    }
    
    /// Common IPv4 addresses
    pub const LOCALHOST: IpAddrV4 = IpAddrV4 { octets: [127, 0, 0, 1] };
    pub const UNSPECIFIED: IpAddrV4 = IpAddrV4 { octets: [0, 0, 0, 0] };
    pub const BROADCAST: IpAddrV4 = IpAddrV4 { octets: [255, 255, 255, 255] };
}

// IPv6 Address Implementation
impl IpAddrV6 {
    /// Create a new IPv6 address from eight 16-bit segments
    pub fn new(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, g: u16, h: u16) -> Self {
        Self { segments: [a, b, c, d, e, f, g, h] }
    }
    
    /// Create IPv6 address from segments array
    pub fn from_segments(segments: [u16; 8]) -> Self {
        Self { segments }
    }
    
    /// Get the segments of the IPv6 address
    pub fn segments(&self) -> [u16; 8] {
        self.segments
    }
    
    /// Convert to 128-bit integer as bytes
    pub fn to_bytes(&self) -> [u8; 16] {
        let mut bytes = [0u8; 16];
        for (i, &segment) in self.segments.iter().enumerate() {
            let segment_bytes = segment.to_be_bytes();
            bytes[i * 2] = segment_bytes[0];
            bytes[i * 2 + 1] = segment_bytes[1];
        }
        bytes
    }
    
    /// Create IPv6 address from 128-bit bytes
    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        let mut segments = [0u16; 8];
        for i in 0..8 {
            segments[i] = u16::from_be_bytes([bytes[i * 2], bytes[i * 2 + 1]]);
        }
        Self { segments }
    }
    
    /// Check if this is a loopback address (::1)
    pub fn is_loopback(&self) -> bool {
        self.segments == [0, 0, 0, 0, 0, 0, 0, 1]
    }
    
    /// Check if this is an unspecified address (::)
    pub fn is_unspecified(&self) -> bool {
        self.segments == [0, 0, 0, 0, 0, 0, 0, 0]
    }
    
    /// Check if this is a multicast address (ff00::/8)
    pub fn is_multicast(&self) -> bool {
        (self.segments[0] & 0xff00) == 0xff00
    }
    
    /// Check if this is a link-local address (fe80::/10)
    pub fn is_link_local(&self) -> bool {
        (self.segments[0] & 0xffc0) == 0xfe80
    }
    
    /// Check if this is a unique local address (fc00::/7)
    pub fn is_unique_local(&self) -> bool {
        (self.segments[0] & 0xfe00) == 0xfc00
    }
    
    /// Check if this is an IPv4-mapped IPv6 address (::ffff:0:0/96)
    pub fn is_ipv4_mapped(&self) -> bool {
        self.segments[0..5] == [0, 0, 0, 0, 0] && self.segments[5] == 0xffff
    }
    
    /// Extract IPv4 address from IPv4-mapped IPv6 address
    pub fn to_ipv4(&self) -> Option<IpAddrV4> {
        if self.is_ipv4_mapped() {
            let a = (self.segments[6] >> 8) as u8;
            let b = (self.segments[6] & 0xff) as u8;
            let c = (self.segments[7] >> 8) as u8;
            let d = (self.segments[7] & 0xff) as u8;
            Some(IpAddrV4::new(a, b, c, d))
        } else {
            None
        }
    }
    
    /// Common IPv6 addresses
    pub const LOCALHOST: IpAddrV6 = IpAddrV6 { segments: [0, 0, 0, 0, 0, 0, 0, 1] };
    pub const UNSPECIFIED: IpAddrV6 = IpAddrV6 { segments: [0, 0, 0, 0, 0, 0, 0, 0] };
}

// IP Address Implementation
impl IpAddr {
    /// Check if this is an IPv4 address
    pub fn is_ipv4(&self) -> bool {
        matches!(self, IpAddr::V4(_))
    }
    
    /// Check if this is an IPv6 address
    pub fn is_ipv6(&self) -> bool {
        matches!(self, IpAddr::V6(_))
    }
    
    /// Check if this is a loopback address
    pub fn is_loopback(&self) -> bool {
        match self {
            IpAddr::V4(ip) => ip.is_loopback(),
            IpAddr::V6(ip) => ip.is_loopback(),
        }
    }
    
    /// Check if this is an unspecified address
    pub fn is_unspecified(&self) -> bool {
        match self {
            IpAddr::V4(ip) => ip.is_unspecified(),
            IpAddr::V6(ip) => ip.is_unspecified(),
        }
    }
    
    /// Check if this is a multicast address
    pub fn is_multicast(&self) -> bool {
        match self {
            IpAddr::V4(ip) => ip.is_multicast(),
            IpAddr::V6(ip) => ip.is_multicast(),
        }
    }
}

// Socket Address Implementations
impl SocketAddrV4 {
    /// Create a new IPv4 socket address
    pub fn new(ip: IpAddrV4, port: u16) -> Self {
        Self { ip, port }
    }
    
    /// Get the IP address
    pub fn ip(&self) -> &IpAddrV4 {
        &self.ip
    }
    
    /// Get the port number
    pub fn port(&self) -> u16 {
        self.port
    }
    
    /// Set the IP address
    pub fn set_ip(&mut self, ip: IpAddrV4) {
        self.ip = ip;
    }
    
    /// Set the port number
    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }
}

impl SocketAddrV6 {
    /// Create a new IPv6 socket address
    pub fn new(ip: IpAddrV6, port: u16, flowinfo: u32, scope_id: u32) -> Self {
        Self { ip, port, flowinfo, scope_id }
    }
    
    /// Get the IP address
    pub fn ip(&self) -> &IpAddrV6 {
        &self.ip
    }
    
    /// Get the port number
    pub fn port(&self) -> u16 {
        self.port
    }
    
    /// Get the flow info
    pub fn flowinfo(&self) -> u32 {
        self.flowinfo
    }
    
    /// Get the scope ID
    pub fn scope_id(&self) -> u32 {
        self.scope_id
    }
    
    /// Set the IP address
    pub fn set_ip(&mut self, ip: IpAddrV6) {
        self.ip = ip;
    }
    
    /// Set the port number
    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }
    
    /// Set the flow info
    pub fn set_flowinfo(&mut self, flowinfo: u32) {
        self.flowinfo = flowinfo;
    }
    
    /// Set the scope ID
    pub fn set_scope_id(&mut self, scope_id: u32) {
        self.scope_id = scope_id;
    }
}

impl SocketAddr {
    /// Check if this is an IPv4 socket address
    pub fn is_ipv4(&self) -> bool {
        matches!(self, SocketAddr::V4(_))
    }
    
    /// Check if this is an IPv6 socket address
    pub fn is_ipv6(&self) -> bool {
        matches!(self, SocketAddr::V6(_))
    }
    
    /// Get the IP address
    pub fn ip(&self) -> IpAddr {
        match self {
            SocketAddr::V4(addr) => IpAddr::V4(*addr.ip()),
            SocketAddr::V6(addr) => IpAddr::V6(*addr.ip()),
        }
    }
    
    /// Get the port number
    pub fn port(&self) -> u16 {
        match self {
            SocketAddr::V4(addr) => addr.port(),
            SocketAddr::V6(addr) => addr.port(),
        }
    }
    
    /// Set the port number
    pub fn set_port(&mut self, port: u16) {
        match self {
            SocketAddr::V4(addr) => addr.set_port(port),
            SocketAddr::V6(addr) => addr.set_port(port),
        }
    }
}

// Display implementations
impl fmt::Display for IpAddrV4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}.{}", self.octets[0], self.octets[1], self.octets[2], self.octets[3])
    }
}

impl fmt::Display for IpAddrV6 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Simplified IPv6 formatting (not handling compression)
        write!(f, "{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}",
               self.segments[0], self.segments[1], self.segments[2], self.segments[3],
               self.segments[4], self.segments[5], self.segments[6], self.segments[7])
    }
}

impl fmt::Display for IpAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IpAddr::V4(ip) => ip.fmt(f),
            IpAddr::V6(ip) => ip.fmt(f),
        }
    }
}

impl fmt::Display for SocketAddrV4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.ip, self.port)
    }
}

impl fmt::Display for SocketAddrV6 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]:{}", self.ip, self.port)
    }
}

impl fmt::Display for SocketAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SocketAddr::V4(addr) => addr.fmt(f),
            SocketAddr::V6(addr) => addr.fmt(f),
        }
    }
}

// String parsing implementations
impl FromStr for IpAddrV4 {
    type Err = NetError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 4 {
            return Err(address_error(s, "IPv4 address must have 4 octets"));
        }
        
        let mut octets = [0u8; 4];
        for (i, part) in parts.iter().enumerate() {
            match part.parse::<u8>() {
                Ok(octet) => octets[i] = octet,
                Err(_) => return Err(address_error(s, "Invalid octet in IPv4 address")),
            }
        }
        
        Ok(IpAddrV4::from_octets(octets))
    }
}

impl FromStr for IpAddrV6 {
    type Err = NetError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Simplified IPv6 parsing (not handling compression)
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 8 {
            return Err(address_error(s, "IPv6 address must have 8 segments (simplified parsing)"));
        }
        
        let mut segments = [0u16; 8];
        for (i, part) in parts.iter().enumerate() {
            match u16::from_str_radix(part, 16) {
                Ok(segment) => segments[i] = segment,
                Err(_) => return Err(address_error(s, "Invalid segment in IPv6 address")),
            }
        }
        
        Ok(IpAddrV6::from_segments(segments))
    }
}

impl FromStr for IpAddr {
    type Err = NetError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(':') {
            Ok(IpAddr::V6(s.parse()?))
        } else {
            Ok(IpAddr::V4(s.parse()?))
        }
    }
}

impl FromStr for SocketAddr {
    type Err = NetError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('[') {
            // IPv6 format: [ip]:port
            if let Some(end_bracket) = s.find(']') {
                let ip_str = &s[1..end_bracket];
                let rest = &s[end_bracket + 1..];
                if rest.starts_with(':') {
                    let port_str = &rest[1..];
                    match port_str.parse::<u16>() {
                        Ok(port) => {
                            let ip = ip_str.parse::<IpAddrV6>()?;
                            Ok(SocketAddr::V6(SocketAddrV6::new(ip, port, 0, 0)))
                        },
                        Err(_) => Err(address_error(s, "Invalid port in IPv6 socket address")),
                    }
                } else {
                    Err(address_error(s, "IPv6 socket address must have format [ip]:port"))
                }
            } else {
                Err(address_error(s, "Invalid IPv6 socket address format"))
            }
        } else {
            // IPv4 format: ip:port
            if let Some(colon_pos) = s.rfind(':') {
                let ip_str = &s[..colon_pos];
                let port_str = &s[colon_pos + 1..];
                match port_str.parse::<u16>() {
                    Ok(port) => {
                        let ip = ip_str.parse::<IpAddrV4>()?;
                        Ok(SocketAddr::V4(SocketAddrV4::new(ip, port)))
                    },
                    Err(_) => Err(address_error(s, "Invalid port in IPv4 socket address")),
                }
            } else {
                Err(address_error(s, "Socket address must contain port"))
            }
        }
    }
}

// Conversions to/from standard library types
impl From<StdIpv4Addr> for IpAddrV4 {
    fn from(addr: StdIpv4Addr) -> Self {
        Self::from_octets(addr.octets())
    }
}

impl From<IpAddrV4> for StdIpv4Addr {
    fn from(addr: IpAddrV4) -> Self {
        StdIpv4Addr::from(addr.octets())
    }
}

impl From<StdIpv6Addr> for IpAddrV6 {
    fn from(addr: StdIpv6Addr) -> Self {
        Self::from_segments(addr.segments())
    }
}

impl From<IpAddrV6> for StdIpv6Addr {
    fn from(addr: IpAddrV6) -> Self {
        StdIpv6Addr::from(addr.segments())
    }
}

impl From<StdIpAddr> for IpAddr {
    fn from(addr: StdIpAddr) -> Self {
        match addr {
            StdIpAddr::V4(ip) => IpAddr::V4(ip.into()),
            StdIpAddr::V6(ip) => IpAddr::V6(ip.into()),
        }
    }
}

impl From<IpAddr> for StdIpAddr {
    fn from(addr: IpAddr) -> Self {
        match addr {
            IpAddr::V4(ip) => StdIpAddr::V4(ip.into()),
            IpAddr::V6(ip) => StdIpAddr::V6(ip.into()),
        }
    }
}

impl From<StdSocketAddr> for SocketAddr {
    fn from(addr: StdSocketAddr) -> Self {
        match addr {
            StdSocketAddr::V4(addr) => SocketAddr::V4(SocketAddrV4::new(
                addr.ip().clone().into(),
                addr.port(),
            )),
            StdSocketAddr::V6(addr) => SocketAddr::V6(SocketAddrV6::new(
                addr.ip().clone().into(),
                addr.port(),
                addr.flowinfo(),
                addr.scope_id(),
            )),
        }
    }
}

impl From<SocketAddr> for StdSocketAddr {
    fn from(addr: SocketAddr) -> Self {
        match addr {
            SocketAddr::V4(addr) => StdSocketAddr::V4(StdSocketAddrV4::new(
                (*addr.ip()).into(),
                addr.port(),
            )),
            SocketAddr::V6(addr) => StdSocketAddr::V6(StdSocketAddrV6::new(
                (*addr.ip()).into(),
                addr.port(),
                addr.flowinfo(),
                addr.scope_id(),
            )),
        }
    }
}

/// Utility functions for address handling

/// Parse a host:port string into a socket address
pub fn parse_socket_addr(input: &str) -> NetResult<SocketAddr> {
    input.parse()
}

/// Parse an IP address string
pub fn parse_ip_addr(input: &str) -> NetResult<IpAddr> {
    input.parse()
}

/// Create a socket address from IP and port
pub fn socket_addr_from_ip_port(ip: IpAddr, port: u16) -> SocketAddr {
    match ip {
        IpAddr::V4(ip) => SocketAddr::V4(SocketAddrV4::new(ip, port)),
        IpAddr::V6(ip) => SocketAddr::V6(SocketAddrV6::new(ip, port, 0, 0)),
    }
}

/// Validate that a string is a valid IP address
pub fn is_valid_ip(input: &str) -> bool {
    parse_ip_addr(input).is_ok()
}

/// Validate that a string is a valid socket address
pub fn is_valid_socket_addr(input: &str) -> bool {
    parse_socket_addr(input).is_ok()
}

/// Check if an IP address is in a given CIDR range
pub fn ip_in_cidr(ip: IpAddr, cidr: &str) -> NetResult<bool> {
    // Simplified CIDR checking for IPv4 only
    if let IpAddr::V4(ip) = ip {
        if let Some(slash_pos) = cidr.find('/') {
            let network_str = &cidr[..slash_pos];
            let prefix_len_str = &cidr[slash_pos + 1..];
            
            let network: IpAddrV4 = network_str.parse()?;
            let prefix_len: u8 = prefix_len_str.parse()
                .map_err(|_| address_error(cidr, "Invalid prefix length in CIDR"))?;
            
            if prefix_len > 32 {
                return Err(address_error(cidr, "Prefix length must be <= 32 for IPv4"));
            }
            
            let mask = if prefix_len == 0 { 0 } else { !0u32 << (32 - prefix_len) };
            let network_addr = network.to_u32() & mask;
            let ip_addr = ip.to_u32() & mask;
            
            Ok(network_addr == ip_addr)
        } else {
            Err(address_error(cidr, "CIDR must contain /"))
        }
    } else {
        Err(address_error("ipv6", "IPv6 CIDR checking not implemented"))
    }
}

