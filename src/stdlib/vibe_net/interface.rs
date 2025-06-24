// Network interface information for VibeNet

use crate::error::CursedError;
use crate::error::Error;
use super::addr::AddrVibe;
use super::NetResult;

/// InterfaceFlags represents network interface flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InterfaceFlags(u32);

impl InterfaceFlags {
    pub const UP: InterfaceFlags = InterfaceFlags(1);
    pub const BROADCAST: InterfaceFlags = InterfaceFlags(2);
    pub const LOOPBACK: InterfaceFlags = InterfaceFlags(4);
    pub const MULTICAST: InterfaceFlags = InterfaceFlags(8);
}

/// HardwareAddrVibe represents a hardware (MAC) address
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HardwareAddrVibe {
    addr: Vec<u8>,
}

impl HardwareAddrVibe {
    /// Create a new hardware address
    pub fn new(addr: Vec<u8>) -> HardwareAddrVibe {
        HardwareAddrVibe { addr }
    }
    
    /// Get the address bytes
    pub fn bytes(&self) -> &[u8] {
        &self.addr
    }
}

/// InterfaceVibe represents a network interface
#[derive(Debug, Clone)]
pub struct InterfaceVibe {
    pub index: i32,
    pub mtu: i32,
    pub name: String,
    pub hardware_addr: HardwareAddrVibe,
    pub flags: InterfaceFlags,
}

impl InterfaceVibe {
    /// Get all network interfaces
    pub fn list() -> NetResult<Vec<InterfaceVibe>> {
        Ok(vec![])
    }
    
    /// Get interface by index
    pub fn by_index(index: i32) -> NetResult<InterfaceVibe> {
        Err(CursedError::new("Not implemented"))
    }
    
    /// Get interface by name
    pub fn by_name(name: &str) -> NetResult<InterfaceVibe> {
        Err(CursedError::new("Not implemented"))
    }
    
    /// Get addresses for this interface
    pub fn addrs(&self) -> NetResult<Vec<Box<dyn AddrVibe>>> {
        Ok(vec![])
    }
    
    /// Get multicast addresses for this interface
    pub fn multicast_addrs(&self) -> NetResult<Vec<Box<dyn AddrVibe>>> {
        Ok(vec![])
    }
}
