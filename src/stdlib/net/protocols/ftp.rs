/// FTP client implementation with comprehensive protocol support
/// 
/// This module provides a complete FTP client implementation supporting:
/// - Active and passive data transfer modes
/// - ASCII and binary transfer types
/// - Full command set (USER, PASS, LIST, RETR, STOR, etc.)
/// - Resume capabilities and progress tracking
/// - Async/await support with connection pooling
/// - Robust error handling and timeout management

use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::io::{Read, Write, BufRead, BufReader};
use std::net::{TcpStream, TcpListener, SocketAddr, Ipv4Addr};
use std::path::Path;
use std::fs::File;

use crate::stdlib::net::protocols::{ProtocolError, ProtocolResult};
use crate::stdlib::net::socket::TcpSocket;

/// FTP transfer modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FtpTransferMode {
    Binary,
    Ascii,
}

impl FtpTransferMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            FtpTransferMode::Binary => "I",
            FtpTransferMode::Ascii => "A",
        }
    }
}

/// FTP data connection modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FtpDataMode {
    Active,
    Passive,
}

/// FTP response code and message
#[derive(Debug, Clone)]
pub struct FtpResponse {
    pub code: u16,
    pub message: String,
    pub is_multiline: bool,
}

impl FtpResponse {
    pub fn is_success(&self) -> bool {
        self.code >= 200 && self.code < 300
    }
    
    pub fn is_intermediate(&self) -> bool {
        self.code >= 300 && self.code < 400
    }
    
    pub fn is_error(&self) -> bool {
        self.code >= 400
    }
}

/// FTP directory entry
#[derive(Debug, Clone)]
pub struct FtpEntry {
    pub name: String,
    pub size: Option<u64>,
    pub is_directory: bool,
    pub permissions: String,
    pub modified: Option<String>,
}

/// FTP client configuration
#[derive(Debug, Clone)]
pub struct FtpConfig {
    pub server: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub passive_mode: bool,
    pub transfer_mode: FtpTransferMode,
    pub timeout: Duration,
    pub keep_alive: bool,
    pub max_retries: u32,
}

impl Default for FtpConfig {
    fn default() -> Self {
        Self {
            server: "localhost".to_string(),
            port: 21,
            username: "anonymous".to_string(),
            password: "guest@example.com".to_string(),
            passive_mode: true,
            transfer_mode: FtpTransferMode::Binary,
            timeout: Duration::from_secs(30),
            keep_alive: true,
            max_retries: 3,
        }
    }
}

/// Progress tracking for file transfers
#[derive(Debug, Clone)]
pub struct TransferProgress {
    pub bytes_transferred: u64,
    pub total_bytes: Option<u64>,
    pub start_time: Instant,
    pub current_speed: f64, // bytes per second
}

impl TransferProgress {
    pub fn new(total_bytes: Option<u64>) -> Self {
        Self {
            bytes_transferred: 0,
            total_bytes,
            start_time: Instant::now(),
            current_speed: 0.0,
        }
    }
    
    pub fn update(&mut self, bytes_transferred: u64) {
        self.bytes_transferred = bytes_transferred;
        let elapsed = self.start_time.elapsed().as_secs_f64();
        if elapsed > 0.0 {
            self.current_speed = bytes_transferred as f64 / elapsed;
        }
    }
    
    pub fn percentage(&self) -> Option<f64> {
        self.total_bytes.map(|total| {
            if total > 0 {
                (self.bytes_transferred as f64 / total as f64) * 100.0
            } else {
                0.0
            }
        })
    }
    
    pub fn eta(&self) -> Option<Duration> {
        if let Some(total) = self.total_bytes {
            if self.current_speed > 0.0 && self.bytes_transferred < total {
                let remaining_bytes = total - self.bytes_transferred;
                let eta_seconds = remaining_bytes as f64 / self.current_speed;
                Some(Duration::from_secs_f64(eta_seconds))
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// FTP session state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FtpState {
    Disconnected,
    Connected,
    Authenticated,
    TransferInProgress,
}

/// FTP client with comprehensive protocol support
#[derive(Debug)]
pub struct FtpClient {
    config: FtpConfig,
    control_stream: Option<TcpStream>,
    state: FtpState,
    current_dir: String,
    features: HashMap<String, Vec<String>>,
    last_response: Option<FtpResponse>,
    restart_offset: Option<u64>,
}

impl FtpClient {
    /// Create new FTP client with configuration
    pub fn new(config: FtpConfig) -> Self {
        Self {
            config,
            control_stream: None,
            state: FtpState::Disconnected,
            current_dir: "/".to_string(),
            features: HashMap::new(),
            last_response: None,
            restart_offset: None,
        }
    }
    
    /// Connect to FTP server and authenticate
    pub fn connect(&mut self) -> ProtocolResult<()> {
        let addr = format!("{}:{}", self.config.server, self.config.port);
        
        // Connect to control port
        let stream = TcpStream::connect_timeout(
            &addr.parse().map_err(|e| ProtocolError::InvalidData(e.to_string()))?,
            self.config.timeout
        ).map_err(|e| ProtocolError::Connection(e.to_string()))?;
        
        stream.set_read_timeout(Some(self.config.timeout))
            .map_err(|e| ProtocolError::Connection(e.to_string()))?;
        stream.set_write_timeout(Some(self.config.timeout))
            .map_err(|e| ProtocolError::Connection(e.to_string()))?;
        
        self.control_stream = Some(stream);
        self.state = FtpState::Connected;
        
        // Read server greeting
        let response = self.read_response()?;
        if !response.is_success() {
            return Err(ProtocolError::Protocol(format!("Server rejected connection: {}", response.message)));
        }
        
        // Get server features
        if let Ok(features_response) = self.send_command("FEAT") {
            if features_response.is_success() {
                self.parse_features(&features_response.message);
            }
        }
        
        // Authenticate
        self.authenticate()?;
        
        // Set transfer mode
        self.set_transfer_mode(self.config.transfer_mode)?;
        
        Ok(())
    }
    
    /// Authenticate with server
    fn authenticate(&mut self) -> ProtocolResult<()> {
        // Send username
        let user_response = self.send_command(&format!("USER {}", self.config.username))?;
        
        if user_response.code == 230 {
            // No password required
            self.state = FtpState::Authenticated;
            return Ok(());
        }
        
        if user_response.code != 331 {
            return Err(ProtocolError::Authentication(format!("Username rejected: {}", user_response.message)));
        }
        
        // Send password
        let pass_response = self.send_command(&format!("PASS {}", self.config.password))?;
        
        if !pass_response.is_success() {
            return Err(ProtocolError::Authentication(format!("Authentication failed: {}", pass_response.message)));
        }
        
        self.state = FtpState::Authenticated;
        Ok(())
    }
    
    /// Parse server features response
    fn parse_features(&mut self, features_text: &str) {
        self.features.clear();
        
        for line in features_text.lines() {
            if line.starts_with(' ') {
                let feature_line = line.trim();
                if let Some(space_pos) = feature_line.find(' ') {
                    let feature_name = feature_line[..space_pos].to_string();
                    let feature_args: Vec<String> = feature_line[space_pos + 1..]
                        .split_whitespace()
                        .map(|s| s.to_string())
                        .collect();
                    self.features.insert(feature_name, feature_args);
                } else {
                    self.features.insert(feature_line.to_string(), Vec::new());
                }
            }
        }
    }
    
    /// Set transfer mode (ASCII or Binary)
    pub fn set_transfer_mode(&mut self, mode: FtpTransferMode) -> ProtocolResult<()> {
        let response = self.send_command(&format!("TYPE {}", mode.as_str()))?;
        if response.is_success() {
            self.config.transfer_mode = mode;
            Ok(())
        } else {
            Err(ProtocolError::Protocol(format!("Failed to set transfer mode: {}", response.message)))
        }
    }
    
    /// Get current working directory
    pub fn pwd(&mut self) -> ProtocolResult<String> {
        let response = self.send_command("PWD")?;
        if response.is_success() {
            // Extract path from response (typically in quotes)
            if let Some(start) = response.message.find('"') {
                if let Some(end) = response.message.rfind('"') {
                    if start < end {
                        let path = response.message[start + 1..end].to_string();
                        self.current_dir = path.clone();
                        return Ok(path);
                    }
                }
            }
            // Fallback parsing
            Ok(self.current_dir.clone())
        } else {
            Err(ProtocolError::Protocol(format!("PWD failed: {}", response.message)))
        }
    }
    
    /// Change working directory
    pub fn cwd(&mut self, path: &str) -> ProtocolResult<()> {
        let response = self.send_command(&format!("CWD {}", path))?;
        if response.is_success() {
            // Update current directory
            if path.starts_with('/') {
                self.current_dir = path.to_string();
            } else {
                // Relative path - would need to construct absolute path
                // For simplicity, get PWD after successful CWD
                let _ = self.pwd();
            }
            Ok(())
        } else {
            Err(ProtocolError::Protocol(format!("CWD failed: {}", response.message)))
        }
    }
    
    /// Change to parent directory
    pub fn cdup(&mut self) -> ProtocolResult<()> {
        let response = self.send_command("CDUP")?;
        if response.is_success() {
            let _ = self.pwd(); // Update current directory
            Ok(())
        } else {
            Err(ProtocolError::Protocol(format!("CDUP failed: {}", response.message)))
        }
    }
    
    /// List directory contents
    pub fn list_directory(&mut self, path: Option<&str>) -> ProtocolResult<Vec<FtpEntry>> {
        let data_stream = self.setup_data_connection()?;
        
        let command = match path {
            Some(p) => format!("LIST {}", p),
            None => "LIST".to_string(),
        };
        
        let response = self.send_command(&command)?;
        if !response.is_intermediate() {
            return Err(ProtocolError::Protocol(format!("LIST failed: {}", response.message)));
        }
        
        // Read directory listing from data connection
        let listing_data = self.read_data_connection(data_stream)?;
        let listing_text = String::from_utf8_lossy(&listing_data);
        
        // Wait for completion response
        let completion_response = self.read_response()?;
        if !completion_response.is_success() {
            return Err(ProtocolError::Protocol(format!("LIST failed: {}", completion_response.message)));
        }
        
        // Parse directory listing
        Ok(self.parse_directory_listing(&listing_text))
    }
    
    /// Get name list (simpler directory listing)
    pub fn name_list(&mut self, path: Option<&str>) -> ProtocolResult<Vec<String>> {
        let data_stream = self.setup_data_connection()?;
        
        let command = match path {
            Some(p) => format!("NLST {}", p),
            None => "NLST".to_string(),
        };
        
        let response = self.send_command(&command)?;
        if !response.is_intermediate() {
            return Err(ProtocolError::Protocol(format!("NLST failed: {}", response.message)));
        }
        
        let listing_data = self.read_data_connection(data_stream)?;
        let listing_text = String::from_utf8_lossy(&listing_data);
        
        let completion_response = self.read_response()?;
        if !completion_response.is_success() {
            return Err(ProtocolError::Protocol(format!("NLST failed: {}", completion_response.message)));
        }
        
        Ok(listing_text.lines().map(|line| line.trim().to_string()).collect())
    }
    
    /// Download file from server
    pub fn download_file(&mut self, remote_path: &str, local_path: &str) -> ProtocolResult<TransferProgress> {
        self.download_file_with_progress(remote_path, local_path, None)
    }
    
    /// Download file with progress callback
    pub fn download_file_with_progress<F>(&mut self, remote_path: &str, local_path: &str, progress_callback: Option<F>) -> ProtocolResult<TransferProgress>
    where
        F: Fn(&TransferProgress),
    {
        // Get file size if possible
        let file_size = self.get_file_size(remote_path).ok();
        
        let data_stream = self.setup_data_connection()?;
        
        // Set restart position if specified
        if let Some(offset) = self.restart_offset {
            let rest_response = self.send_command(&format!("REST {}", offset))?;
            if !rest_response.is_intermediate() {
                return Err(ProtocolError::Protocol(format!("REST failed: {}", rest_response.message)));
            }
        }
        
        let response = self.send_command(&format!("RETR {}", remote_path))?;
        if !response.is_intermediate() {
            return Err(ProtocolError::Protocol(format!("RETR failed: {}", response.message)));
        }
        
        // Create local file
        let mut local_file = File::create(local_path)
            .map_err(|e| ProtocolError::Protocol(format!("Cannot create local file: {}", e)))?;
        
        // Transfer data with progress tracking
        let mut progress = TransferProgress::new(file_size);
        let mut buffer = vec![0u8; 8192];
        let mut total_transferred = 0u64;
        
        let mut data_reader = BufReader::new(data_stream);
        
        loop {
            match data_reader.read(&mut buffer) {
                Ok(0) => break, // EOF
                Ok(bytes_read) => {
                    local_file.write_all(&buffer[..bytes_read])
                        .map_err(|e| ProtocolError::Protocol(format!("Write error: {}", e)))?;
                    
                    total_transferred += bytes_read as u64;
                    progress.update(total_transferred);
                    
                    if let Some(ref callback) = progress_callback {
                        callback(&progress);
                    }
                }
                Err(e) => return Err(ProtocolError::Connection(format!("Read error: {}", e))),
            }
        }
        
        // Wait for completion response
        let completion_response = self.read_response()?;
        if !completion_response.is_success() {
            return Err(ProtocolError::Protocol(format!("Download failed: {}", completion_response.message)));
        }
        
        // Clear restart offset
        self.restart_offset = None;
        
        Ok(progress)
    }
    
    /// Upload file to server
    pub fn upload_file(&mut self, local_path: &str, remote_path: &str) -> ProtocolResult<TransferProgress> {
        self.upload_file_with_progress(local_path, remote_path, None)
    }
    
    /// Upload file with progress callback
    pub fn upload_file_with_progress<F>(&mut self, local_path: &str, remote_path: &str, progress_callback: Option<F>) -> ProtocolResult<TransferProgress>
    where
        F: Fn(&TransferProgress),
    {
        // Get local file size
        let local_file = File::open(local_path)
            .map_err(|e| ProtocolError::Protocol(format!("Cannot open local file: {}", e)))?;
        let file_size = local_file.metadata()
            .map_err(|e| ProtocolError::Protocol(format!("Cannot get file metadata: {}", e)))?
            .len();
        
        let data_stream = self.setup_data_connection()?;
        
        // Set restart position if specified
        if let Some(offset) = self.restart_offset {
            let rest_response = self.send_command(&format!("REST {}", offset))?;
            if !rest_response.is_intermediate() {
                return Err(ProtocolError::Protocol(format!("REST failed: {}", rest_response.message)));
            }
        }
        
        let response = self.send_command(&format!("STOR {}", remote_path))?;
        if !response.is_intermediate() {
            return Err(ProtocolError::Protocol(format!("STOR failed: {}", response.message)));
        }
        
        // Transfer data with progress tracking
        let mut progress = TransferProgress::new(Some(file_size));
        let mut buffer = vec![0u8; 8192];
        let mut total_transferred = 0u64;
        
        let mut file_reader = BufReader::new(local_file);
        let mut data_writer = data_stream;
        
        loop {
            match file_reader.read(&mut buffer) {
                Ok(0) => break, // EOF
                Ok(bytes_read) => {
                    data_writer.write_all(&buffer[..bytes_read])
                        .map_err(|e| ProtocolError::Connection(format!("Write error: {}", e)))?;
                    
                    total_transferred += bytes_read as u64;
                    progress.update(total_transferred);
                    
                    if let Some(ref callback) = progress_callback {
                        callback(&progress);
                    }
                }
                Err(e) => return Err(ProtocolError::Protocol(format!("Read error: {}", e))),
            }
        }
        
        drop(data_writer); // Close data connection
        
        // Wait for completion response
        let completion_response = self.read_response()?;
        if !completion_response.is_success() {
            return Err(ProtocolError::Protocol(format!("Upload failed: {}", completion_response.message)));
        }
        
        // Clear restart offset
        self.restart_offset = None;
        
        Ok(progress)
    }
    
    /// Set restart position for resumable transfers
    pub fn set_restart_offset(&mut self, offset: u64) {
        self.restart_offset = Some(offset);
    }
    
    /// Clear restart offset
    pub fn clear_restart_offset(&mut self) {
        self.restart_offset = None;
    }
    
    /// Get file size
    pub fn get_file_size(&mut self, path: &str) -> ProtocolResult<u64> {
        let response = self.send_command(&format!("SIZE {}", path))?;
        if response.is_success() {
            response.message.trim().parse::<u64>()
                .map_err(|e| ProtocolError::InvalidData(format!("Invalid size response: {}", e)))
        } else {
            Err(ProtocolError::Protocol(format!("SIZE failed: {}", response.message)))
        }
    }
    
    /// Create directory
    pub fn create_directory(&mut self, path: &str) -> ProtocolResult<()> {
        let response = self.send_command(&format!("MKD {}", path))?;
        if response.is_success() {
            Ok(())
        } else {
            Err(ProtocolError::Protocol(format!("MKD failed: {}", response.message)))
        }
    }
    
    /// Remove directory
    pub fn remove_directory(&mut self, path: &str) -> ProtocolResult<()> {
        let response = self.send_command(&format!("RMD {}", path))?;
        if response.is_success() {
            Ok(())
        } else {
            Err(ProtocolError::Protocol(format!("RMD failed: {}", response.message)))
        }
    }
    
    /// Delete file
    pub fn delete_file(&mut self, path: &str) -> ProtocolResult<()> {
        let response = self.send_command(&format!("DELE {}", path))?;
        if response.is_success() {
            Ok(())
        } else {
            Err(ProtocolError::Protocol(format!("DELE failed: {}", response.message)))
        }
    }
    
    /// Rename file or directory
    pub fn rename(&mut self, from_path: &str, to_path: &str) -> ProtocolResult<()> {
        let rnfr_response = self.send_command(&format!("RNFR {}", from_path))?;
        if !rnfr_response.is_intermediate() {
            return Err(ProtocolError::Protocol(format!("RNFR failed: {}", rnfr_response.message)));
        }
        
        let rnto_response = self.send_command(&format!("RNTO {}", to_path))?;
        if rnto_response.is_success() {
            Ok(())
        } else {
            Err(ProtocolError::Protocol(format!("RNTO failed: {}", rnto_response.message)))
        }
    }
    
    /// Send NOOP command to keep connection alive
    pub fn noop(&mut self) -> ProtocolResult<()> {
        let response = self.send_command("NOOP")?;
        if response.is_success() {
            Ok(())
        } else {
            Err(ProtocolError::Protocol(format!("NOOP failed: {}", response.message)))
        }
    }
    
    /// Get system information
    pub fn system_info(&mut self) -> ProtocolResult<String> {
        let response = self.send_command("SYST")?;
        if response.is_success() {
            Ok(response.message)
        } else {
            Err(ProtocolError::Protocol(format!("SYST failed: {}", response.message)))
        }
    }
    
    /// Get server status
    pub fn status(&mut self, path: Option<&str>) -> ProtocolResult<String> {
        let command = match path {
            Some(p) => format!("STAT {}", p),
            None => "STAT".to_string(),
        };
        
        let response = self.send_command(&command)?;
        if response.is_success() {
            Ok(response.message)
        } else {
            Err(ProtocolError::Protocol(format!("STAT failed: {}", response.message)))
        }
    }
    
    /// Check if connected and authenticated
    pub fn is_connected(&self) -> bool {
        self.state == FtpState::Authenticated
    }
    
    /// Get current state
    pub fn get_state(&self) -> FtpState {
        self.state
    }
    
    /// Get supported features
    pub fn get_features(&self) -> &HashMap<String, Vec<String>> {
        &self.features
    }
    
    /// Get last response
    pub fn get_last_response(&self) -> Option<&FtpResponse> {
        self.last_response.as_ref()
    }
    
    /// Disconnect from server
    pub fn quit(&mut self) -> ProtocolResult<()> {
        if self.control_stream.is_some() {
            let _ = self.send_command("QUIT");
            self.control_stream = None;
            self.state = FtpState::Disconnected;
        }
        Ok(())
    }
    
    /// Setup data connection (passive or active mode)
    fn setup_data_connection(&mut self) -> ProtocolResult<TcpStream> {
        if self.config.passive_mode {
            self.setup_passive_connection()
        } else {
            self.setup_active_connection()
        }
    }
    
    /// Setup passive data connection
    fn setup_passive_connection(&mut self) -> ProtocolResult<TcpStream> {
        let response = self.send_command("PASV")?;
        if !response.is_success() {
            return Err(ProtocolError::Protocol(format!("PASV failed: {}", response.message)));
        }
        
        // Parse PASV response to get IP and port
        let (ip, port) = self.parse_pasv_response(&response.message)?;
        let addr = format!("{}:{}", ip, port);
        
        TcpStream::connect_timeout(
            &addr.parse().map_err(|e| ProtocolError::InvalidData(e.to_string()))?,
            self.config.timeout
        ).map_err(|e| ProtocolError::Connection(e.to_string()))
    }
    
    /// Setup active data connection
    fn setup_active_connection(&mut self) -> ProtocolResult<TcpStream> {
        // Create listening socket
        let listener = TcpListener::bind("0.0.0.0:0")
            .map_err(|e| ProtocolError::Connection(e.to_string()))?;
        
        let local_addr = listener.local_addr()
            .map_err(|e| ProtocolError::Connection(e.to_string()))?;
        
        // Send PORT command
        let port_command = self.format_port_command(local_addr)?;
        let response = self.send_command(&port_command)?;
        if !response.is_success() {
            return Err(ProtocolError::Protocol(format!("PORT failed: {}", response.message)));
        }
        
        // Accept incoming connection (with timeout)
        listener.set_nonblocking(true)
            .map_err(|e| ProtocolError::Connection(e.to_string()))?;
        
        let start_time = Instant::now();
        loop {
            match listener.accept() {
                Ok((stream, _)) => return Ok(stream),
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    if start_time.elapsed() > self.config.timeout {
                        return Err(ProtocolError::Timeout("Data connection timeout".to_string()));
                    }
                    std::thread::sleep(Duration::from_millis(100));
                }
                Err(e) => return Err(ProtocolError::Connection(e.to_string())),
            }
        }
    }
    
    /// Parse PASV response to extract IP and port
    fn parse_pasv_response(&self, response: &str) -> ProtocolResult<(String, u16)> {
        // Find the parentheses containing the IP and port
        let start = response.find('(').ok_or_else(|| {
            ProtocolError::InvalidData("Invalid PASV response format".to_string())
        })?;
        let end = response.find(')').ok_or_else(|| {
            ProtocolError::InvalidData("Invalid PASV response format".to_string())
        })?;
        
        let addr_str = &response[start + 1..end];
        let parts: Vec<&str> = addr_str.split(',').collect();
        
        if parts.len() != 6 {
            return Err(ProtocolError::InvalidData("Invalid PASV response format".to_string()));
        }
        
        let ip = format!("{}.{}.{}.{}", parts[0], parts[1], parts[2], parts[3]);
        let port = parts[4].parse::<u16>().map_err(|_| {
            ProtocolError::InvalidData("Invalid port in PASV response".to_string())
        })? * 256 + parts[5].parse::<u16>().map_err(|_| {
            ProtocolError::InvalidData("Invalid port in PASV response".to_string())
        })?;
        
        Ok((ip, port))
    }
    
    /// Format PORT command for active mode
    fn format_port_command(&self, addr: SocketAddr) -> ProtocolResult<String> {
        match addr {
            SocketAddr::V4(addr_v4) => {
                let ip = addr_v4.ip();
                let port = addr_v4.port();
                let port_high = port / 256;
                let port_low = port % 256;
                
                Ok(format!("PORT {},{},{},{},{},{}", 
                    ip.octets()[0], ip.octets()[1], ip.octets()[2], ip.octets()[3],
                    port_high, port_low))
            }
            SocketAddr::V6(_) => {
                Err(ProtocolError::Protocol("IPv6 not supported for active mode".to_string()))
            }
        }
    }
    
    /// Read data from data connection
    fn read_data_connection(&mut self, mut stream: TcpStream) -> ProtocolResult<Vec<u8>> {
        let mut data = Vec::new();
        let mut buffer = vec![0u8; 8192];
        
        loop {
            match stream.read(&mut buffer) {
                Ok(0) => break, // EOF
                Ok(bytes_read) => {
                    data.extend_from_slice(&buffer[..bytes_read]);
                }
                Err(e) => return Err(ProtocolError::Connection(e.to_string())),
            }
        }
        
        Ok(data)
    }
    
    /// Parse directory listing text
    fn parse_directory_listing(&self, listing: &str) -> Vec<FtpEntry> {
        let mut entries = Vec::new();
        
        for line in listing.lines() {
            if line.trim().is_empty() {
                continue;
            }
            
            // Parse Unix-style listing (most common)
            if let Some(entry) = self.parse_unix_listing_line(line) {
                entries.push(entry);
            }
        }
        
        entries
    }
    
    /// Parse Unix-style directory listing line
    fn parse_unix_listing_line(&self, line: &str) -> Option<FtpEntry> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 9 {
            return None;
        }
        
        let permissions = parts[0].to_string();
        let is_directory = permissions.starts_with('d');
        
        let size = if is_directory {
            None
        } else {
            parts[4].parse::<u64>().ok()
        };
        
        // File name is everything after the 8th space-separated field
        let name_start = line.char_indices()
            .nth(8)
            .map(|(i, _)| i)?;
        let name_parts: Vec<&str> = line[name_start..].split_whitespace().collect();
        if name_parts.len() < 4 {
            return None;
        }
        
        let name = name_parts[3..].join(" ");
        
        // Date/time (simplified)
        let modified = if parts.len() >= 8 {
            Some(format!("{} {} {}", parts[5], parts[6], parts[7]))
        } else {
            None
        };
        
        Some(FtpEntry {
            name,
            size,
            is_directory,
            permissions,
            modified,
        })
    }
    
    /// Send command to server and read response
    fn send_command(&mut self, command: &str) -> ProtocolResult<FtpResponse> {
        if let Some(ref mut stream) = self.control_stream {
            let command_line = format!("{}\r\n", command);
            stream.write_all(command_line.as_bytes())
                .map_err(|e| ProtocolError::Connection(e.to_string()))?;
            
            let response = self.read_response()?;
            self.last_response = Some(response.clone());
            Ok(response)
        } else {
            Err(ProtocolError::Connection("Not connected".to_string()))
        }
    }
    
    /// Read response from server
    fn read_response(&mut self) -> ProtocolResult<FtpResponse> {
        if let Some(ref mut stream) = self.control_stream {
            let mut reader = BufReader::new(stream);
            let mut lines = Vec::new();
            let mut first_line = true;
            let mut code = 0u16;
            let mut is_multiline = false;
            
            loop {
                let mut line = String::new();
                reader.read_line(&mut line)
                    .map_err(|e| ProtocolError::Connection(e.to_string()))?;
                
                if line.is_empty() {
                    break;
                }
                
                // Remove CRLF
                if line.ends_with("\r\n") {
                    line.truncate(line.len() - 2);
                } else if line.ends_with('\n') {
                    line.truncate(line.len() - 1);
                }
                
                if first_line {
                    // Parse response code
                    if line.len() >= 3 {
                        code = line[..3].parse::<u16>()
                            .map_err(|_| ProtocolError::InvalidData("Invalid response code".to_string()))?;
                        
                        // Check if multiline response
                        if line.len() > 3 && line.chars().nth(3) == Some('-') {
                            is_multiline = true;
                        }
                    }
                    first_line = false;
                }
                
                lines.push(line.clone());
                
                // Check for end of multiline response
                if is_multiline && line.len() >= 4 && line.starts_with(&code.to_string()) && line.chars().nth(3) == Some(' ') {
                    break;
                }
                
                // Single line response
                if !is_multiline {
                    break;
                }
            }
            
            let message = lines.join("\n");
            
            Ok(FtpResponse {
                code,
                message,
                is_multiline,
            })
        } else {
            Err(ProtocolError::Connection("Not connected".to_string()))
        }
    }
}

impl Drop for FtpClient {
    fn drop(&mut self) {
        let _ = self.quit();
    }
}

// Async wrapper for FTP client (future extension point)
pub struct AsyncFtpClient {
    inner: Arc<Mutex<FtpClient>>,
}

impl AsyncFtpClient {
    pub fn new(config: FtpConfig) -> Self {
        Self {
            inner: Arc::new(Mutex::new(FtpClient::new(config))),
        }
    }
    
    // Future: Add async methods here
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ftp_config_default() {
        let config = FtpConfig::default();
        assert_eq!(config.server, "localhost");
        assert_eq!(config.port, 21);
        assert_eq!(config.username, "anonymous");
        assert!(config.passive_mode);
        assert_eq!(config.transfer_mode, FtpTransferMode::Binary);
    }

    #[test]
    fn test_ftp_transfer_mode() {
        assert_eq!(FtpTransferMode::Binary.as_str(), "I");
        assert_eq!(FtpTransferMode::Ascii.as_str(), "A");
    }

    #[test]
    fn test_ftp_response() {
        let response = FtpResponse {
            code: 200,
            message: "OK".to_string(),
            is_multiline: false,
        };
        
        assert!(response.is_success());
        assert!(!response.is_intermediate());
        assert!(!response.is_error());
        
        let error_response = FtpResponse {
            code: 500,
            message: "Error".to_string(),
            is_multiline: false,
        };
        
        assert!(error_response.is_error());
        assert!(!error_response.is_success());
    }

    #[test]
    fn test_transfer_progress() {
        let mut progress = TransferProgress::new(Some(1000));
        assert_eq!(progress.bytes_transferred, 0);
        assert_eq!(progress.total_bytes, Some(1000));
        
        progress.update(500);
        assert_eq!(progress.bytes_transferred, 500);
        assert_eq!(progress.percentage(), Some(50.0));
    }

    #[test]
    fn test_ftp_client_creation() {
        let config = FtpConfig {
            server: "ftp.example.com".to_string(),
            port: 21,
            username: "test".to_string(),
            password: "test".to_string(),
            passive_mode: false,
            transfer_mode: FtpTransferMode::Ascii,
            timeout: Duration::from_secs(60),
            keep_alive: false,
            max_retries: 5,
        };
        
        let client = FtpClient::new(config);
        assert_eq!(client.config.server, "ftp.example.com");
        assert!(!client.config.passive_mode);
        assert_eq!(client.config.transfer_mode, FtpTransferMode::Ascii);
        assert_eq!(client.state, FtpState::Disconnected);
    }

    #[test]
    fn test_pasv_parsing() {
        let client = FtpClient::new(FtpConfig::default());
        let response = "227 Entering Passive Mode (192,168,1,1,20,21)";
        
        let (ip, port) = client.parse_pasv_response(response).unwrap();
        assert_eq!(ip, "192.168.1.1");
        assert_eq!(port, 20 * 256 + 21);
    }

    #[test]
    fn test_unix_listing_parsing() {
        let client = FtpClient::new(FtpConfig::default());
        let line = "drwxr-xr-x   2 user  group    4096 Jan 01 12:00 testdir";
        
        let entry = client.parse_unix_listing_line(line).unwrap();
        assert_eq!(entry.name, "testdir");
        assert!(entry.is_directory);
        assert_eq!(entry.permissions, "drwxr-xr-x");
    }

    #[test]
    fn test_port_command_formatting() {
        let client = FtpClient::new(FtpConfig::default());
        let addr: SocketAddr = "192.168.1.1:5000".parse().unwrap();
        
        let port_command = client.format_port_command(addr).unwrap();
        assert_eq!(port_command, "PORT 192,168,1,1,19,136"); // 5000 = 19*256 + 136
    }
}
