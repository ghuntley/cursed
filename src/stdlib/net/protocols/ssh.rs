/// SSH client implementation with complete SSH/SCP functionality
/// 
/// This module provides a comprehensive SSH client implementation supporting:
/// - Password and key-based authentication
/// - Command execution with proper output handling
/// - File transfer via SCP
/// - Connection management with timeouts
/// - Comprehensive error handling

// use crate::stdlib::net::protocols::{ProtocolError, ProtocolResult};
use crate::error::CursedError;
use ssh2::{Session, Channel, Sftp, ScpFileStat};
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::path::Path;
use std::time::Duration;

/// SSH key types supporting different cryptographic algorithms
#[derive(Debug, Clone)]
pub enum SshKey {
    /// RSA private key in OpenSSH or PEM format
    Rsa(Vec<u8>),
    /// Ed25519 private key
    Ed25519(Vec<u8>),
    /// ECDSA private key
    Ecdsa(Vec<u8>),
}

/// SSH command execution result with comprehensive output capture
#[derive(Debug, Clone)]
pub struct SshCommand {
    /// The command that was executed
    pub command: String,
    /// Standard output from the command
    pub stdout: String,
    /// Standard error from the command
    pub stderr: String,
    /// Exit code of the command (0 typically means success)
    pub exit_code: i32,
}

/// SSH client configuration with authentication and connection options
#[derive(Debug, Clone)]
pub struct SshConfig {
    /// Remote host address (hostname or IP)
    pub host: String,
    /// SSH port (default: 22)
    pub port: u16,
    /// Username for authentication
    pub username: String,
    /// Password for password-based authentication
    pub password: Option<String>,
    /// Private key for key-based authentication
    pub private_key: Option<SshKey>,
    /// Connection timeout in seconds
    pub connect_timeout: Option<Duration>,
    /// Command execution timeout in seconds
    pub command_timeout: Option<Duration>,
}

impl Default for SshConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 22,
            username: "user".to_string(),
            password: None,
            private_key: None,
            connect_timeout: Some(Duration::from_secs(30)),
            command_timeout: Some(Duration::from_secs(300)),
        }
    }
}

/// SSH client with connection management and operation support
pub struct SshClient {
    config: SshConfig,
    session: Option<Session>,
    tcp_stream: Option<TcpStream>,
}

impl std::fmt::Debug for SshClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SshClient")
            .field("config", &self.config)
            .field("connected", &self.session.is_some())
            .finish()
    }
}

impl SshClient {
    /// Create a new SSH client with the given configuration
    pub fn new(config: SshConfig) -> Self {
        Self {
            config,
            session: None,
            tcp_stream: None,
        }
    }

    /// Establish SSH connection and authenticate
    /// 
    /// This method:
    /// 1. Establishes a TCP connection to the remote host
    /// 2. Performs SSH handshake
    /// 3. Authenticates using password or private key
    /// 
    /// # Returns
    /// - `Ok(())` if connection and authentication succeed
    /// - `Err(ProtocolError)` if any step fails
    pub fn connect(&mut self) -> ProtocolResult<()> {
        // Create socket address
        let addr = format!("{}:{}", self.config.host, self.config.port)
            .to_socket_addrs()
            .map_err(|e| ProtocolError::Connection(format!("Invalid address: {}", e)))?
            .next()
            .ok_or_else(|| ProtocolError::Connection("Could not resolve address".to_string()))?;

        // Establish TCP connection with timeout
        let tcp = if let Some(timeout) = self.config.connect_timeout {
            TcpStream::connect_timeout(&addr, timeout)
                .map_err(|e| ProtocolError::Connection(format!("TCP connection failed: {}", e)))?
        } else {
            TcpStream::connect(&addr)
                .map_err(|e| ProtocolError::Connection(format!("TCP connection failed: {}", e)))?
        };

        // Set TCP stream options for better performance
        tcp.set_nodelay(true)
            .map_err(|e| ProtocolError::Connection(format!("Failed to set TCP options: {}", e)))?;

        // Create SSH session
        let mut session = Session::new()
            .map_err(|e| ProtocolError::Protocol(format!("Failed to create SSH session: {}", e)))?;

        // Set timeout if configured
        if let Some(timeout) = self.config.command_timeout {
            session.set_timeout(timeout.as_millis() as u32);
        }

        // Perform SSH handshake
        session.set_tcp_stream(tcp);
        session.handshake()
            .map_err(|e| ProtocolError::Protocol(format!("SSH handshake failed: {}", e)))?;

        // Authenticate
        self.authenticate(&mut session)?;

        // Store successful connection
        self.session = Some(session);
        
        Ok(())
    }

    /// Authenticate with the SSH server
    fn authenticate(&self, session: &mut Session) -> ProtocolResult<()> {
        let username = &self.config.username;

        // Try public key authentication first if available
        if let Some(ref key) = self.config.private_key {
            match self.authenticate_with_key(session, username, key) {
                Ok(()) => return Ok(()),
                Err(e) => {
                    // Log key auth failure but continue to password auth if available
                    eprintln!("Key authentication failed: {}", e);
                }
            }
        }

        // Try password authentication if available
        if let Some(ref password) = self.config.password {
            session.userauth_password(username, password)
                .map_err(|e| ProtocolError::Authentication(format!("Password authentication failed: {}", e)))?;
            
            if !session.authenticated() {
                return Err(ProtocolError::Authentication("Authentication failed".to_string()));
            }
            
            return Ok(());
        }

        // No authentication method succeeded
        Err(ProtocolError::Authentication(
            "No valid authentication method available".to_string()
        ))
    }

    /// Authenticate using private key
    fn authenticate_with_key(&self, session: &mut Session, username: &str, key: &SshKey) -> ProtocolResult<()> {
        match key {
            SshKey::Rsa(key_data) | SshKey::Ed25519(key_data) | SshKey::Ecdsa(key_data) => {
                // Try to authenticate with the key data
                // First try as OpenSSH format, then as PEM
                let key_str = String::from_utf8_lossy(key_data);
                
                // Use memory-based key authentication
                session.userauth_pubkey_memory(username, None, &key_str, None)
                    .map_err(|e| ProtocolError::Authentication(format!("Key authentication failed: {}", e)))?;
                
                if !session.authenticated() {
                    return Err(ProtocolError::Authentication("Key authentication failed".to_string()));
                }
                
                Ok(())
            }
        }
    }

    /// Execute a command on the remote server
    /// 
    /// # Arguments
    /// - `command` - The command to execute
    /// 
    /// # Returns
    /// - `Ok(SshCommand)` with execution results
    /// - `Err(ProtocolError)` if execution fails
    pub fn execute_command(&mut self, command: &str) -> ProtocolResult<SshCommand> {
        let session = self.session.as_mut()
            .ok_or_else(|| ProtocolError::Connection("Not connected".to_string()))?;

        // Create channel for command execution
        let mut channel = session.channel_session()
            .map_err(|e| ProtocolError::Protocol(format!("Failed to create channel: {}", e)))?;

        // Execute the command
        channel.exec(command)
            .map_err(|e| ProtocolError::Protocol(format!("Failed to execute command: {}", e)))?;

        // Read stdout
        let mut stdout = String::new();
        channel.read_to_string(&mut stdout)
            .map_err(|e| ProtocolError::Protocol(format!("Failed to read stdout: {}", e)))?;

        // Read stderr  
        let mut stderr = String::new();
        channel.stderr().read_to_string(&mut stderr)
            .map_err(|e| ProtocolError::Protocol(format!("Failed to read stderr: {}", e)))?;

        // Wait for command completion and get exit status
        channel.wait_close()
            .map_err(|e| ProtocolError::Protocol(format!("Failed to close channel: {}", e)))?;

        let exit_code = channel.exit_status()
            .map_err(|e| ProtocolError::Protocol(format!("Failed to get exit status: {}", e)))?;

        Ok(SshCommand {
            command: command.to_string(),
            stdout,
            stderr,
            exit_code,
        })
    }

    /// Upload a local file to the remote server via SCP
    /// 
    /// # Arguments
    /// - `local_path` - Path to the local file
    /// - `remote_path` - Destination path on the remote server
    /// 
    /// # Returns
    /// - `Ok(())` if upload succeeds
    /// - `Err(ProtocolError)` if upload fails
    pub fn upload_file(&mut self, local_path: &str, remote_path: &str) -> ProtocolResult<()> {
        let session = self.session.as_mut()
            .ok_or_else(|| ProtocolError::Connection("Not connected".to_string()))?;

        // Read local file
        let local_data = std::fs::read(local_path)
            .map_err(|e| ProtocolError::Protocol(format!("Failed to read local file: {}", e)))?;

        // Get file metadata
        let metadata = std::fs::metadata(local_path)
            .map_err(|e| ProtocolError::Protocol(format!("Failed to get file metadata: {}", e)))?;

        // Create SCP channel for upload
        let mut channel = session.scp_send(Path::new(remote_path), 0o644, local_data.len() as u64, None)
            .map_err(|e| ProtocolError::Protocol(format!("Failed to create SCP upload channel: {}", e)))?;

        // Write file data
        channel.write_all(&local_data)
            .map_err(|e| ProtocolError::Protocol(format!("Failed to write file data: {}", e)))?;

        // Finalize transfer
        channel.send_eof()
            .map_err(|e| ProtocolError::Protocol(format!("Failed to send EOF: {}", e)))?;

        channel.wait_eof()
            .map_err(|e| ProtocolError::Protocol(format!("Failed to wait for EOF: {}", e)))?;

        channel.close()
            .map_err(|e| ProtocolError::Protocol(format!("Failed to close SCP channel: {}", e)))?;

        channel.wait_close()
            .map_err(|e| ProtocolError::Protocol(format!("Failed to wait for close: {}", e)))?;

        Ok(())
    }

    /// Download a file from the remote server via SCP
    /// 
    /// # Arguments
    /// - `remote_path` - Path to the remote file
    /// - `local_path` - Destination path for the downloaded file
    /// 
    /// # Returns
    /// - `Ok(())` if download succeeds
    /// - `Err(ProtocolError)` if download fails
    pub fn download_file(&mut self, remote_path: &str, local_path: &str) -> ProtocolResult<()> {
        let session = self.session.as_mut()
            .ok_or_else(|| ProtocolError::Connection("Not connected".to_string()))?;

        // Create SCP channel for download
        let (mut channel, stat) = session.scp_recv(Path::new(remote_path))
            .map_err(|e| ProtocolError::Protocol(format!("Failed to create SCP download channel: {}", e)))?;

        // Read file data
        let mut file_data = Vec::with_capacity(stat.size() as usize);
        channel.read_to_end(&mut file_data)
            .map_err(|e| ProtocolError::Protocol(format!("Failed to read file data: {}", e)))?;

        // Write to local file
        std::fs::write(local_path, &file_data)
            .map_err(|e| ProtocolError::Protocol(format!("Failed to write local file: {}", e)))?;

        // Finalize transfer
        channel.send_eof()
            .map_err(|e| ProtocolError::Protocol(format!("Failed to send EOF: {}", e)))?;

        channel.wait_eof()
            .map_err(|e| ProtocolError::Protocol(format!("Failed to wait for EOF: {}", e)))?;

        channel.close()
            .map_err(|e| ProtocolError::Protocol(format!("Failed to close SCP channel: {}", e)))?;

        channel.wait_close()
            .map_err(|e| ProtocolError::Protocol(format!("Failed to wait for close: {}", e)))?;

        Ok(())
    }

    /// Check if the client is currently connected
    pub fn is_connected(&self) -> bool {
        self.session.is_some()
    }

    /// Disconnect from the remote server
    pub fn disconnect(&mut self) -> ProtocolResult<()> {
        if let Some(mut session) = self.session.take() {
            session.disconnect(None, "Client disconnecting", None)
                .map_err(|e| ProtocolError::Protocol(format!("Failed to disconnect: {}", e)))?;
        }
        self.tcp_stream = None;
        Ok(())
    }

    /// Get connection information
    pub fn connection_info(&self) -> Option<String> {
        if self.is_connected() {
            Some(format!("{}@{}:{}", self.config.username, self.config.host, self.config.port))
        } else {
            None
        }
    }
}

impl Drop for SshClient {
    fn drop(&mut self) {
        let _ = self.disconnect();
    }
}
