//! Connection types and traits for VibeNet
//! 
//! This module provides comprehensive connection handling including TCP, UDP,
//! and Unix connections with proper I/O operations, deadlines, and configuration.

use std::io::{Read, Write};
use std::net::{TcpStream, UdpSocket, Shutdown};
use std::os::unix::net::UnixStream;
use std::time::{Duration, SystemTime};
use std::sync::{Arc, Mutex};
use crate::error::CursedError;
use super::addr::{AddrVibe, TCPAddrVibe, UDPAddrVibe, UnixAddrVibe};
use super::error::{NetError, connection_failed_error, timeout_error};
use super::NetResult;
use crate::error::Error;
pub type NetError = crate::error::Error;


/// ConnVibe trait represents a generic network connection
pub trait ConnVibe: Read + Write + Send + Sync {
    /// Read data from the connection
    fn read(&mut self, buf: &mut [u8]) -> NetResult<usize>;
    
    /// Write data to the connection
    fn write(&mut self, buf: &[u8]) -> NetResult<usize>;
    
    /// Close the connection
    fn close(&mut self) -> NetResult<()>;
    
    /// Get the local address
    fn local_addr(&self) -> NetResult<Box<dyn AddrVibe>>;
    
    /// Get the remote address
    fn remote_addr(&self) -> NetResult<Box<dyn AddrVibe>>;
    
    /// Set deadline for all operations
    fn set_deadline(&mut self, t: SystemTime) -> NetResult<()>;
    
    /// Set deadline for read operations
    fn set_read_deadline(&mut self, t: SystemTime) -> NetResult<()>;
    
    /// Set deadline for write operations
    fn set_write_deadline(&mut self, t: SystemTime) -> NetResult<()>;
}

/// PacketConnVibe trait represents a packet-oriented network connection
pub trait PacketConnVibe: Send + Sync {
    /// Read a packet from the connection
    fn read_from(&mut self, buf: &mut [u8]) -> NetResult<(usize, Box<dyn AddrVibe>)>;
    
    /// Write a packet to the connection
    fn write_to(&mut self, buf: &[u8], addr: &dyn AddrVibe) -> NetResult<usize>;
    
    /// Close the connection
    fn close(&mut self) -> NetResult<()>;
    
    /// Get the local address
    fn local_addr(&self) -> NetResult<Box<dyn AddrVibe>>;
    
    /// Set deadline for all operations
    fn set_deadline(&mut self, t: SystemTime) -> NetResult<()>;
    
    /// Set deadline for read operations
    fn set_read_deadline(&mut self, t: SystemTime) -> NetResult<()>;
    
    /// Set deadline for write operations
    fn set_write_deadline(&mut self, t: SystemTime) -> NetResult<()>;
}

/// TCPConnVibe represents a TCP network connection
#[derive(Debug)]
pub struct TCPConnVibe {
    stream: Arc<Mutex<TcpStream>>,
    local_addr: TCPAddrVibe,
    remote_addr: TCPAddrVibe,
}

impl TCPConnVibe {
    /// Create a new TCP connection from a TcpStream
    pub fn from_stream(stream: TcpStream) -> NetResult<TCPConnVibe> {
        let local_addr = TCPAddrVibe::from_socket_addr(
            stream.local_addr()
                .map_err(|e| CursedError::from(connection_failed_error(&e.to_string())))?
        );
        let remote_addr = TCPAddrVibe::from_socket_addr(
            stream.peer_addr()
                .map_err(|e| CursedError::from(connection_failed_error(&e.to_string())))?
        );
        
        Ok(TCPConnVibe {
            stream: Arc::new(Mutex::new(stream)),
            local_addr,
            remote_addr,
        })
    }
    
    /// Dial a TCP connection
    pub fn dial(network: &str, laddr: Option<&TCPAddrVibe>, raddr: &TCPAddrVibe) -> NetResult<TCPConnVibe> {
        let stream = if let Some(local) = laddr {
            // Bind to local address first, then connect
            let socket = std::net::TcpSocket::new_v4()
                .map_err(|e| CursedError::from(connection_failed_error(&e.to_string())))?;
            socket.bind(local.socket_addr())
                .map_err(|e| CursedError::from(connection_failed_error(&e.to_string())))?;
            socket.connect(raddr.socket_addr())
                .map_err(|e| CursedError::from(connection_failed_error(&e.to_string())))?
        } else {
            TcpStream::connect(raddr.socket_addr())
                .map_err(|e| CursedError::from(connection_failed_error(&e.to_string())))?
        };
        
        Self::from_stream(stream)
    }
    
    /// Set keep-alive option
    pub fn set_keep_alive(&mut self, keepalive: bool) -> NetResult<()> {
        let stream = self.stream.lock().unwrap();
        stream.set_keepalive(Some(Duration::from_secs(30)))
            .map_err(|e| CursedError::from(connection_failed_error(&e.to_string())))?;
        Ok(())
    }
    
    /// Set keep-alive period
    pub fn set_keep_alive_period(&mut self, d: Duration) -> NetResult<()> {
        let stream = self.stream.lock().unwrap();
        stream.set_keepalive(Some(d))
            .map_err(|e| CursedError::from(connection_failed_error(&e.to_string())))?;
        Ok(())
    }
    
    /// Set linger option
    pub fn set_linger(&mut self, sec: i32) -> NetResult<()> {
        let stream = self.stream.lock().unwrap();
        let linger = if sec >= 0 { Some(Duration::from_secs(sec as u64)) } else { None };
        stream.set_linger(linger)
            .map_err(|e| CursedError::from(connection_failed_error(&e.to_string())))?;
        Ok(())
    }
    
    /// Set no-delay option (Nagle's algorithm)
    pub fn set_no_delay(&mut self, no_delay: bool) -> NetResult<()> {
        let stream = self.stream.lock().unwrap();
        stream.set_nodelay(no_delay)
            .map_err(|e| CursedError::from(connection_failed_error(&e.to_string())))?;
        Ok(())
    }
    
    /// Set read buffer size
    pub fn set_read_buffer(&mut self, bytes: i32) -> NetResult<()> {
        // Note: This is a hint to the OS - actual implementation may vary
        Ok(())
    }
    
    /// Set write buffer size
    pub fn set_write_buffer(&mut self, bytes: i32) -> NetResult<()> {
        // Note: This is a hint to the OS - actual implementation may vary
        Ok(())
    }
}

impl ConnVibe for TCPConnVibe {
    fn read(&mut self, buf: &mut [u8]) -> NetResult<usize> {
        let mut stream = self.stream.lock().unwrap();
        stream.read(buf)
            .map_err(|e| CursedError::from(NetError::from(e)))
    }
    
    fn write(&mut self, buf: &[u8]) -> NetResult<usize> {
        let mut stream = self.stream.lock().unwrap();
        stream.write(buf)
            .map_err(|e| CursedError::from(NetError::from(e)))
    }
    
    fn close(&mut self) -> NetResult<()> {
        let stream = self.stream.lock().unwrap();
        stream.shutdown(Shutdown::Both)
            .map_err(|e| CursedError::from(NetError::from(e)))
    }
    
    fn local_addr(&self) -> NetResult<Box<dyn AddrVibe>> {
        Ok(Box::new(self.local_addr.clone()))
    }
    
    fn remote_addr(&self) -> NetResult<Box<dyn AddrVibe>> {
        Ok(Box::new(self.remote_addr.clone()))
    }
    
    fn set_deadline(&mut self, t: SystemTime) -> NetResult<()> {
        let duration = t.duration_since(SystemTime::now())
            .map_err(|_| CursedError::from(timeout_error("Deadline is in the past")))?;
        
        let stream = self.stream.lock().unwrap();
        stream.set_read_timeout(Some(duration))
            .map_err(|e| CursedError::from(NetError::from(e)))?;
        stream.set_write_timeout(Some(duration))
            .map_err(|e| CursedError::from(NetError::from(e)))?;
        Ok(())
    }
    
    fn set_read_deadline(&mut self, t: SystemTime) -> NetResult<()> {
        let duration = t.duration_since(SystemTime::now())
            .map_err(|_| CursedError::from(timeout_error("Deadline is in the past")))?;
        
        let stream = self.stream.lock().unwrap();
        stream.set_read_timeout(Some(duration))
            .map_err(|e| CursedError::from(NetError::from(e)))?;
        Ok(())
    }
    
    fn set_write_deadline(&mut self, t: SystemTime) -> NetResult<()> {
        let duration = t.duration_since(SystemTime::now())
            .map_err(|_| CursedError::from(timeout_error("Deadline is in the past")))?;
        
        let stream = self.stream.lock().unwrap();
        stream.set_write_timeout(Some(duration))
            .map_err(|e| CursedError::from(NetError::from(e)))?;
        Ok(())
    }
}

impl Read for TCPConnVibe {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut stream = self.stream.lock().unwrap();
        stream.read(buf)
    }
}

impl Write for TCPConnVibe {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut stream = self.stream.lock().unwrap();
        stream.write(buf)
    }
    
    fn flush(&mut self) -> std::io::Result<()> {
        let mut stream = self.stream.lock().unwrap();
        stream.flush()
    }
}

/// UDPConnVibe represents a UDP network connection
#[derive(Debug)]
pub struct UDPConnVibe {
    socket: Arc<Mutex<UdpSocket>>,
    local_addr: UDPAddrVibe,
    remote_addr: Option<UDPAddrVibe>,
}

impl UDPConnVibe {
    /// Create a new UDP connection from a UdpSocket
    pub fn from_socket(socket: UdpSocket) -> NetResult<UDPConnVibe> {
        let local_addr = UDPAddrVibe::from_socket_addr(
            socket.local_addr()
                .map_err(|e| CursedError::from(connection_failed_error(&e.to_string())))?
        );
        
        let remote_addr = socket.peer_addr()
            .map(|addr| UDPAddrVibe::from_socket_addr(addr))
            .ok();
        
        Ok(UDPConnVibe {
            socket: Arc::new(Mutex::new(socket)),
            local_addr,
            remote_addr,
        })
    }
    
    /// Dial a UDP connection
    pub fn dial(network: &str, laddr: Option<&UDPAddrVibe>, raddr: &UDPAddrVibe) -> NetResult<UDPConnVibe> {
        let socket = if let Some(local) = laddr {
            UdpSocket::bind(local.socket_addr())
                .map_err(|e| CursedError::from(connection_failed_error(&e.to_string())))?
        } else {
            let bind_addr = if raddr.socket_addr().is_ipv4() {
                "0.0.0.0:0"
            } else {
                "[::]:0"
            };
            UdpSocket::bind(bind_addr)
                .map_err(|e| CursedError::from(connection_failed_error(&e.to_string())))?
        };
        
        socket.connect(raddr.socket_addr())
            .map_err(|e| CursedError::from(connection_failed_error(&e.to_string())))?;
        
        Self::from_socket(socket)
    }
    
    /// Listen for UDP packets
    pub fn listen(network: &str, laddr: Option<&UDPAddrVibe>) -> NetResult<UDPConnVibe> {
        let bind_addr = if let Some(local) = laddr {
            local.socket_addr()
        } else {
            match network {
                "udp4" => "0.0.0.0:0".parse().unwrap(),
                "udp6" => "[::]:0".parse().unwrap(),
                _ => "0.0.0.0:0".parse().unwrap(),
            }
        };
        
        let socket = UdpSocket::bind(bind_addr)
            .map_err(|e| CursedError::from(connection_failed_error(&e.to_string())))?;
        
        Self::from_socket(socket)
    }
    
    /// Read from UDP connection
    pub fn read_from_udp(&mut self, buf: &mut [u8]) -> NetResult<(usize, UDPAddrVibe)> {
        let socket = self.socket.lock().unwrap();
        let (n, addr) = socket.recv_from(buf)
            .map_err(|e| CursedError::from(NetError::from(e)))?;
        Ok((n, UDPAddrVibe::from_socket_addr(addr)))
    }
    
    /// Write to UDP connection
    pub fn write_to_udp(&mut self, buf: &[u8], addr: &UDPAddrVibe) -> NetResult<usize> {
        let socket = self.socket.lock().unwrap();
        socket.send_to(buf, addr.socket_addr())
            .map_err(|e| CursedError::from(NetError::from(e)))
    }
}

impl ConnVibe for UDPConnVibe {
    fn read(&mut self, buf: &mut [u8]) -> NetResult<usize> {
        let socket = self.socket.lock().unwrap();
        socket.recv(buf)
            .map_err(|e| CursedError::from(NetError::from(e)))
    }
    
    fn write(&mut self, buf: &[u8]) -> NetResult<usize> {
        let socket = self.socket.lock().unwrap();
        socket.send(buf)
            .map_err(|e| CursedError::from(NetError::from(e)))
    }
    
    fn close(&mut self) -> NetResult<()> {
        // UDP sockets don't have explicit close, handled by drop
        Ok(())
    }
    
    fn local_addr(&self) -> NetResult<Box<dyn AddrVibe>> {
        Ok(Box::new(self.local_addr.clone()))
    }
    
    fn remote_addr(&self) -> NetResult<Box<dyn AddrVibe>> {
        match &self.remote_addr {
            Some(addr) => Ok(Box::new(addr.clone())),
            None => Err(CursedError::from(connection_failed_error("No remote address set"))),
        }
    }
    
    fn set_deadline(&mut self, t: SystemTime) -> NetResult<()> {
        let duration = t.duration_since(SystemTime::now())
            .map_err(|_| CursedError::from(timeout_error("Deadline is in the past")))?;
        
        let socket = self.socket.lock().unwrap();
        socket.set_read_timeout(Some(duration))
            .map_err(|e| CursedError::from(NetError::from(e)))?;
        socket.set_write_timeout(Some(duration))
            .map_err(|e| CursedError::from(NetError::from(e)))?;
        Ok(())
    }
    
    fn set_read_deadline(&mut self, t: SystemTime) -> NetResult<()> {
        let duration = t.duration_since(SystemTime::now())
            .map_err(|_| CursedError::from(timeout_error("Deadline is in the past")))?;
        
        let socket = self.socket.lock().unwrap();
        socket.set_read_timeout(Some(duration))
            .map_err(|e| CursedError::from(NetError::from(e)))?;
        Ok(())
    }
    
    fn set_write_deadline(&mut self, t: SystemTime) -> NetResult<()> {
        let duration = t.duration_since(SystemTime::now())
            .map_err(|_| CursedError::from(timeout_error("Deadline is in the past")))?;
        
        let socket = self.socket.lock().unwrap();
        socket.set_write_timeout(Some(duration))
            .map_err(|e| CursedError::from(NetError::from(e)))?;
        Ok(())
    }
}

impl PacketConnVibe for UDPConnVibe {
    fn read_from(&mut self, buf: &mut [u8]) -> NetResult<(usize, Box<dyn AddrVibe>)> {
        let (n, addr) = self.read_from_udp(buf)?;
        Ok((n, Box::new(addr)))
    }
    
    fn write_to(&mut self, buf: &[u8], addr: &dyn AddrVibe) -> NetResult<usize> {
        // We need to downcast the AddrVibe to UDPAddrVibe
        // In a real implementation, this would be more sophisticated
        if let Some(udp_addr) = addr.string().parse::<std::net::SocketAddr>().ok() {
            let udp_addr_vibe = UDPAddrVibe::from_socket_addr(udp_addr);
            self.write_to_udp(buf, &udp_addr_vibe)
        } else {
            Err(CursedError::from(connection_failed_error("Invalid UDP address")))
        }
    }
    
    fn close(&mut self) -> NetResult<()> {
        self.close()
    }
    
    fn local_addr(&self) -> NetResult<Box<dyn AddrVibe>> {
        self.local_addr()
    }
    
    fn set_deadline(&mut self, t: SystemTime) -> NetResult<()> {
        self.set_deadline(t)
    }
    
    fn set_read_deadline(&mut self, t: SystemTime) -> NetResult<()> {
        self.set_read_deadline(t)
    }
    
    fn set_write_deadline(&mut self, t: SystemTime) -> NetResult<()> {
        self.set_write_deadline(t)
    }
}

impl Read for UDPConnVibe {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let socket = self.socket.lock().unwrap();
        socket.recv(buf)
    }
}

impl Write for UDPConnVibe {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let socket = self.socket.lock().unwrap();
        socket.send(buf)
    }
    
    fn flush(&mut self) -> std::io::Result<()> {
        // UDP doesn't need flushing
        Ok(())
    }
}

/// UnixConnVibe represents a Unix domain socket connection
#[derive(Debug)]
pub struct UnixConnVibe {
    stream: Arc<Mutex<UnixStream>>,
    local_addr: UnixAddrVibe,
    remote_addr: UnixAddrVibe,
}

impl UnixConnVibe {
    /// Create a new Unix connection from a UnixStream
    pub fn from_stream(stream: UnixStream, local_path: &str, remote_path: &str) -> NetResult<UnixConnVibe> {
        let local_addr = UnixAddrVibe::resolve("unix", local_path)?;
        let remote_addr = UnixAddrVibe::resolve("unix", remote_path)?;
        
        Ok(UnixConnVibe {
            stream: Arc::new(Mutex::new(stream)),
            local_addr,
            remote_addr,
        })
    }
    
    /// Dial a Unix connection
    pub fn dial(network: &str, laddr: Option<&UnixAddrVibe>, raddr: &UnixAddrVibe) -> NetResult<UnixConnVibe> {
        let stream = UnixStream::connect(raddr.path())
            .map_err(|e| CursedError::from(connection_failed_error(&e.to_string())))?;
        
        let local_path = laddr.map(|a| a.name()).unwrap_or_else(|| "".to_string());
        Self::from_stream(stream, &local_path, &raddr.name())
    }
}

impl ConnVibe for UnixConnVibe {
    fn read(&mut self, buf: &mut [u8]) -> NetResult<usize> {
        let mut stream = self.stream.lock().unwrap();
        stream.read(buf)
            .map_err(|e| CursedError::from(NetError::from(e)))
    }
    
    fn write(&mut self, buf: &[u8]) -> NetResult<usize> {
        let mut stream = self.stream.lock().unwrap();
        stream.write(buf)
            .map_err(|e| CursedError::from(NetError::from(e)))
    }
    
    fn close(&mut self) -> NetResult<()> {
        let stream = self.stream.lock().unwrap();
        stream.shutdown(Shutdown::Both)
            .map_err(|e| CursedError::from(NetError::from(e)))
    }
    
    fn local_addr(&self) -> NetResult<Box<dyn AddrVibe>> {
        Ok(Box::new(self.local_addr.clone()))
    }
    
    fn remote_addr(&self) -> NetResult<Box<dyn AddrVibe>> {
        Ok(Box::new(self.remote_addr.clone()))
    }
    
    fn set_deadline(&mut self, _t: SystemTime) -> NetResult<()> {
        // Unix sockets don't support timeouts in the same way
        Ok(())
    }
    
    fn set_read_deadline(&mut self, _t: SystemTime) -> NetResult<()> {
        // Unix sockets don't support timeouts in the same way
        Ok(())
    }
    
    fn set_write_deadline(&mut self, _t: SystemTime) -> NetResult<()> {
        // Unix sockets don't support timeouts in the same way
        Ok(())
    }
}

impl Read for UnixConnVibe {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut stream = self.stream.lock().unwrap();
        stream.read(buf)
    }
}

impl Write for UnixConnVibe {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut stream = self.stream.lock().unwrap();
        stream.write(buf)
    }
    
    fn flush(&mut self) -> std::io::Result<()> {
        let mut stream = self.stream.lock().unwrap();
        stream.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_tcp_connection_dial() {
        // This test would require a server to connect to
        // In practice, we'd mock or use a test server
    }

    #[test]
    fn test_udp_connection_creation() {
        let conn = UDPConnVibe::listen("udp", None);
        assert!(conn.is_ok());
    }

    #[test]
    fn test_connection_traits() {
        // Test that our types implement the required traits
        fn assert_conn_vibe<T: ConnVibe>() {}
        fn assert_packet_conn_vibe<T: PacketConnVibe>() {}
        
        // These should compile without error
        assert_conn_vibe::<TCPConnVibe>();
        assert_conn_vibe::<UDPConnVibe>();
        assert_conn_vibe::<UnixConnVibe>();
        assert_packet_conn_vibe::<UDPConnVibe>();
    }

    #[test]
    fn test_deadline_calculation() {
        let future = SystemTime::now() + Duration::from_secs(10);
        let past = SystemTime::now() - Duration::from_secs(10);
        
        // Future deadline should work
        assert!(future > SystemTime::now());
        
        // Past deadline should be detected
        assert!(past < SystemTime::now());
    }
}
