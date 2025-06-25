use crate::error::CursedError;
/// Listener types for VibeNet
/// 
/// This module provides network listeners for accepting incoming connections
/// on TCP and Unix domain sockets.

use std::net::{TcpListener, SocketAddr};
use std::os::unix::net::UnixListener;
use std::time::SystemTime;
use super::addr::{AddrVibe, TCPAddrVibe, UnixAddrVibe};
use super::conn::{ConnVibe, TCPConnVibe, UnixConnVibe};
use super::error::{NetError, connection_failed_error};
use super::NetResult;

/// ListenerVibe trait represents a network listener
pub trait ListenerVibe: Send + Sync {
    /// Accept a connection
    fn accept(&mut self) -> NetResult<Box<dyn ConnVibe>>;
    
    /// Close the listener
    fn close(&mut self) -> NetResult<()>;
    
    /// Get the listener address
    fn addr(&self) -> NetResult<Box<dyn AddrVibe>>;
}

/// TCPListenerVibe represents a TCP network listener
#[derive(Debug)]
pub struct TCPListenerVibe {
    listener: TcpListener,
    addr: TCPAddrVibe,
}

impl TCPListenerVibe {
    /// Listen on a TCP address
    pub fn listen(network: &str, laddr: Option<&TCPAddrVibe>) -> NetResult<TCPListenerVibe> {
        let bind_addr = if let Some(addr) = laddr {
            addr.socket_addr()
        } else {
            match network {
                "tcp4" => "0.0.0.0:0".parse().unwrap(),
                "tcp6" => "[::]:0".parse().unwrap(),
                _ => "0.0.0.0:0".parse().unwrap(),
            }
        };
        
        let listener = TcpListener::bind(bind_addr)
            .map_err(|e| CursedError::from(connection_failed_error(&e.to_string())))?;
        
        let actual_addr = TCPAddrVibe::from_socket_addr(
            listener.local_addr()
                .map_err(|e| CursedError::from(connection_failed_error(&e.to_string())))?
        );
        
        Ok(TCPListenerVibe {
            listener,
            addr: actual_addr,
        })
    }
    
    /// Accept a TCP connection
    pub fn accept_tcp(&mut self) -> NetResult<TCPConnVibe> {
        let (stream, _addr) = self.listener.accept()
            .map_err(|e| CursedError::from(NetError::from(e)))?;
        TCPConnVibe::from_stream(stream)
    }
    
    /// Set deadline for accept operations
    pub fn set_deadline(&mut self, _t: SystemTime) -> NetResult<()> {
        // Note: std::net::TcpListener doesn't support timeouts directly
        // In a real implementation, we'd use non-blocking I/O and select/poll
        Ok(())
    }
}

impl ListenerVibe for TCPListenerVibe {
    fn accept(&mut self) -> NetResult<Box<dyn ConnVibe>> {
        let conn = self.accept_tcp()?;
        Ok(Box::new(conn))
    }
    
    fn close(&mut self) -> NetResult<()> {
        // TcpListener doesn't have explicit close - handled by drop
        Ok(())
    }
    
    fn addr(&self) -> NetResult<Box<dyn AddrVibe>> {
        Ok(Box::new(self.addr.clone()))
    }
}

/// UnixListenerVibe represents a Unix domain socket listener
#[derive(Debug)]
pub struct UnixListenerVibe {
    listener: UnixListener,
    addr: UnixAddrVibe,
}

impl UnixListenerVibe {
    /// Listen on a Unix address
    pub fn listen(network: &str, laddr: Option<&UnixAddrVibe>) -> NetResult<UnixListenerVibe> {
        let path = if let Some(addr) = laddr {
            addr.path().clone()
        } else {
            return Err(CursedError::from(connection_failed_error("Unix listener requires a path")));
        };
        
        let listener = UnixListener::bind(&path)
            .map_err(|e| CursedError::from(connection_failed_error(&e.to_string())))?;
        
        let addr = UnixAddrVibe::new(path, network);
        
        Ok(UnixListenerVibe {
            listener,
            addr,
        })
    }
    
    /// Accept a Unix connection
    pub fn accept_unix(&mut self) -> NetResult<UnixConnVibe> {
        let (stream, _addr) = self.listener.accept()
            .map_err(|e| CursedError::from(NetError::from(e)))?;
        
        UnixConnVibe::from_stream(stream, &self.addr.name(), "client")
    }
    
    /// Set deadline for accept operations
    pub fn set_deadline(&mut self, _t: SystemTime) -> NetResult<()> {
        // Note: std::os::unix::net::UnixListener doesn't support timeouts directly
        Ok(())
    }
}

impl ListenerVibe for UnixListenerVibe {
    fn accept(&mut self) -> NetResult<Box<dyn ConnVibe>> {
        let conn = self.accept_unix()?;
        Ok(Box::new(conn))
    }
    
    fn close(&mut self) -> NetResult<()> {
        // UnixListener doesn't have explicit close - handled by drop
        // In practice, we might want to remove the socket file
        Ok(())
    }
    
    fn addr(&self) -> NetResult<Box<dyn AddrVibe>> {
        Ok(Box::new(self.addr.clone()))
    }
}

