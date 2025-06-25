use crate::error::CursedError;
/// HTTP/2 Protocol Implementation for CURSED
/// 
/// Provides comprehensive HTTP/2 client and server functionality including
/// multiplexing, server push, header compression, stream prioritization,
/// and flow control mechanisms for high-performance web applications.

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use super::{NetError, NetResult};

// =============================================================================
// HTTP/2 PROTOCOL CONSTANTS
// =============================================================================

const HTTP2_CONNECTION_PREFACE: &[u8] = b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n";
const HTTP2_FRAME_HEADER_LENGTH: usize = 9;
const HTTP2_DEFAULT_WINDOW_SIZE: u32 = 65535;
const HTTP2_MAX_FRAME_SIZE: u32 = 16777215; // 2^24 - 1

// =============================================================================
// HTTP/2 FRAME TYPES
// =============================================================================

/// HTTP/2 frame types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FrameType {
impl TryFrom<u8> for FrameType {
    type CursedError = NetError;
    
    fn try_from(value: u8) -> crate::error::Result<()> {
        match value {
            _ => Err(NetError::ProtocolError {
                protocol: "HTTP/2".to_string(),
        }
    }
// =============================================================================
// HTTP/2 FRAME STRUCTURE
// =============================================================================

/// HTTP/2 frame flags
#[derive(Debug, Clone, Copy)]
pub struct FrameFlags(pub u8);

impl FrameFlags {
    pub const NONE: FrameFlags = FrameFlags(0x0);
    pub const END_STREAM: FrameFlags = FrameFlags(0x1);
    pub const END_HEADERS: FrameFlags = FrameFlags(0x4);
    pub const PADDED: FrameFlags = FrameFlags(0x8);
    pub const PRIORITY: FrameFlags = FrameFlags(0x20);
    pub const ACK: FrameFlags = FrameFlags(0x1);
    
    pub fn contains(self, flag: FrameFlags) -> bool {
        (self.0 & flag.0) != 0
    pub fn set(&mut self, flag: FrameFlags) {
        self.0 |= flag.0;
    pub fn unset(&mut self, flag: FrameFlags) {
        self.0 &= !flag.0;
    }
}

/// HTTP/2 frame header
#[derive(Debug, Clone)]
pub struct FrameHeader {
impl FrameHeader {
    /// Creates a new frame header
    pub fn new(length: u32, frame_type: FrameType, flags: FrameFlags, stream_id: u32) -> Self {
        Self {
        }
    }
    
    /// Serializes frame header to bytes
    pub fn to_bytes(&self) -> [u8; HTTP2_FRAME_HEADER_LENGTH] {
        let mut bytes = [0u8; HTTP2_FRAME_HEADER_LENGTH];
        
        // Length (24 bits)
        bytes[0] = (self.length >> 16) as u8;
        bytes[1] = (self.length >> 8) as u8;
        bytes[2] = self.length as u8;
        
        // Type (8 bits)
        bytes[3] = self.frame_type as u8;
        
        // Flags (8 bits)
        bytes[4] = self.flags.0;
        
        // Stream ID (31 bits, reserved bit is 0)
        bytes[5] = (self.stream_id >> 24) as u8;
        bytes[6] = (self.stream_id >> 16) as u8;
        bytes[7] = (self.stream_id >> 8) as u8;
        bytes[8] = self.stream_id as u8;
        
        bytes
    /// Parses frame header from bytes
    pub fn from_bytes(bytes: &[u8]) -> NetResult<Self> {
        if bytes.len() < HTTP2_FRAME_HEADER_LENGTH {
            return Err(NetError::ProtocolError {
                protocol: "HTTP/2".to_string(),
            });
        let length = ((bytes[0] as u32) << 16) | ((bytes[1] as u32) << 8) | (bytes[2] as u32);
        let frame_type = FrameType::try_from(bytes[3])?;
        let flags = FrameFlags(bytes[4]);
        let stream_id = ((bytes[5] as u32) << 24) | ((bytes[6] as u32) << 16) | 
                       ((bytes[7] as u32) << 8) | (bytes[8] as u32);
        
        // Clear reserved bit
        let stream_id = stream_id & 0x7FFFFFFF;
        
        Ok(Self {
        })
    }
}

/// HTTP/2 frame
#[derive(Debug, Clone)]
pub struct Frame {
impl Frame {
    /// Creates a new frame
    pub fn new(frame_type: FrameType, flags: FrameFlags, stream_id: u32, payload: Vec<u8>) -> Self {
        let header = FrameHeader::new(payload.len() as u32, frame_type, flags, stream_id);
        Self { header, payload }
    }
    
    /// Creates a DATA frame
    pub fn data(stream_id: u32, data: Vec<u8>, end_stream: bool) -> Self {
        let mut flags = FrameFlags::NONE;
        if end_stream {
            flags.set(FrameFlags::END_STREAM);
        }
        Self::new(FrameType::Data, flags, stream_id, data)
    /// Creates a HEADERS frame
    pub fn headers(stream_id: u32, headers: Vec<u8>, end_stream: bool, end_headers: bool) -> Self {
        let mut flags = FrameFlags::NONE;
        if end_stream {
            flags.set(FrameFlags::END_STREAM);
        }
        if end_headers {
            flags.set(FrameFlags::END_HEADERS);
        }
        Self::new(FrameType::Headers, flags, stream_id, headers)
    /// Creates a SETTINGS frame
    pub fn settings(settings: Vec<u8>, ack: bool) -> Self {
        let flags = if ack { FrameFlags::ACK } else { FrameFlags::NONE };
        Self::new(FrameType::Settings, flags, 0, settings)
    /// Creates a PING frame
    pub fn ping(data: [u8; 8], ack: bool) -> Self {
        let flags = if ack { FrameFlags::ACK } else { FrameFlags::NONE };
        Self::new(FrameType::Ping, flags, 0, data.to_vec())
    /// Creates a WINDOW_UPDATE frame
    pub fn window_update(stream_id: u32, window_size_increment: u32) -> Self {
        let mut payload = Vec::with_capacity(4);
        payload.extend_from_slice(&window_size_increment.to_be_bytes());
        Self::new(FrameType::WindowUpdate, FrameFlags::NONE, stream_id, payload)
    /// Creates a RST_STREAM frame
    pub fn rst_stream(stream_id: u32, error_code: u32) -> Self {
        let mut payload = Vec::with_capacity(4);
        payload.extend_from_slice(&error_code.to_be_bytes());
        Self::new(FrameType::RstStream, FrameFlags::NONE, stream_id, payload)
    /// Serializes frame to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(HTTP2_FRAME_HEADER_LENGTH + self.payload.len());
        bytes.extend_from_slice(&self.header.to_bytes());
        bytes.extend_from_slice(&self.payload);
        bytes
    }
}

// =============================================================================
// HTTP/2 SETTINGS
// =============================================================================

/// HTTP/2 settings parameters
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum SettingsParameter {
impl TryFrom<u16> for SettingsParameter {
    type CursedError = NetError;
    
    fn try_from(value: u16) -> crate::error::Result<()> {
        match value {
            _ => Err(NetError::ProtocolError {
                protocol: "HTTP/2".to_string(),
        }
    }
/// HTTP/2 settings
#[derive(Debug, Clone)]
pub struct Settings {
impl Default for Settings {
    fn default() -> Self {
        Self {
        }
    }
impl Settings {
    /// Serializes settings to frame payload
    pub fn to_frame_payload(&self) -> Vec<u8> {
        let mut payload = Vec::new();
        
        // Each setting is 6 bytes: 2 bytes parameter ID + 4 bytes value
        payload.extend_from_slice(&(SettingsParameter::HeaderTableSize as u16).to_be_bytes());
        payload.extend_from_slice(&self.header_table_size.to_be_bytes());
        
        payload.extend_from_slice(&(SettingsParameter::EnablePush as u16).to_be_bytes());
        payload.extend_from_slice(&(if self.enable_push { 1u32 } else { 0u32 }).to_be_bytes());
        
        if let Some(max_streams) = self.max_concurrent_streams {
            payload.extend_from_slice(&(SettingsParameter::MaxConcurrentStreams as u16).to_be_bytes());
            payload.extend_from_slice(&max_streams.to_be_bytes());
        payload.extend_from_slice(&(SettingsParameter::InitialWindowSize as u16).to_be_bytes());
        payload.extend_from_slice(&self.initial_window_size.to_be_bytes());
        
        payload.extend_from_slice(&(SettingsParameter::MaxFrameSize as u16).to_be_bytes());
        payload.extend_from_slice(&self.max_frame_size.to_be_bytes());
        
        if let Some(max_header_size) = self.max_header_list_size {
            payload.extend_from_slice(&(SettingsParameter::MaxHeaderListSize as u16).to_be_bytes());
            payload.extend_from_slice(&max_header_size.to_be_bytes());
        payload
    /// Parses settings from frame payload
    pub fn from_frame_payload(payload: &[u8]) -> NetResult<Self> {
        if payload.len() % 6 != 0 {
            return Err(NetError::ProtocolError {
                protocol: "HTTP/2".to_string(),
            });
        let mut settings = Settings::default();
        
        for chunk in payload.chunks_exact(6) {
            let param_id = u16::from_be_bytes([chunk[0], chunk[1]]);
            let value = u32::from_be_bytes([chunk[2], chunk[3], chunk[4], chunk[5]]);
            
            match SettingsParameter::try_from(param_id)? {
            }
        }
        
        Ok(settings)
    }
}

// =============================================================================
// HTTP/2 STREAM
// =============================================================================

/// HTTP/2 stream state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamState {
/// HTTP/2 stream
#[derive(Debug)]
pub struct Stream {
impl Stream {
    /// Creates a new stream
    pub fn new(id: u32, initial_window_size: u32) -> Self {
        Self {
        }
    }
    
    /// Checks if stream can send data
    pub fn can_send_data(&self) -> bool {
        matches!(self.state, StreamState::Open | StreamState::HalfClosedRemote)
    /// Checks if stream can receive data
    pub fn can_receive_data(&self) -> bool {
        matches!(self.state, StreamState::Open | StreamState::HalfClosedLocal)
    /// Updates stream state based on frame
    pub fn update_state(&mut self, frame: &Frame) -> NetResult<()> {
        match frame.header.frame_type {
            FrameType::Headers => {
                if frame.header.flags.contains(FrameFlags::END_STREAM) {
                    match self.state {
                        _ => return Err(NetError::ProtocolError {
                            protocol: "HTTP/2".to_string(),
                    }
                } else {
                    match self.state {
                        _ => {}
                    }
                }
                self.headers_received = true;
            }
            FrameType::Data => {
                if frame.header.flags.contains(FrameFlags::END_STREAM) {
                    match self.state {
                        _ => return Err(NetError::ProtocolError {
                            protocol: "HTTP/2".to_string(),
                    }
                }
                
                // Update window size
                self.window_size -= frame.payload.len() as i32;
                
                // Add data to queue
                if !frame.payload.is_empty() {
                    self.data_queue.push_back(frame.payload.clone());
                }
            }
            FrameType::RstStream => {
                self.state = StreamState::Closed;
            }
            _ => {}
        Ok(())
    /// Gets all received data
    pub fn get_all_data(&mut self) -> Vec<u8> {
        let mut all_data = Vec::new();
        while let Some(data) = self.data_queue.pop_front() {
            all_data.extend(data);
        }
        all_data
    /// Checks if stream is closed
    pub fn is_closed(&self) -> bool {
        self.state == StreamState::Closed
    }
}

// =============================================================================
// HTTP/2 CONNECTION
// =============================================================================

/// HTTP/2 connection
#[derive(Debug)]
pub struct Http2Connection {
impl Http2Connection {
    /// Creates a new HTTP/2 connection
    pub fn new(is_server: bool) -> Self {
        Self {
        }
    }
    
    /// Creates a new stream
    pub fn create_stream(&mut self) -> u32 {
        let stream_id = self.next_stream_id;
        self.next_stream_id += 2; // Client uses odd IDs, server uses even IDs
        
        let stream = Stream::new(stream_id, self.local_settings.initial_window_size);
        self.streams.insert(stream_id, stream);
        
        stream_id
    /// Gets or creates a stream
    pub fn get_or_create_stream(&mut self, stream_id: u32) -> &mut Stream {
        if !self.streams.contains_key(&stream_id) {
            let stream = Stream::new(stream_id, self.local_settings.initial_window_size);
            self.streams.insert(stream_id, stream);
        }
        self.streams.get_mut(&stream_id).unwrap()
    /// Processes an incoming frame
    pub fn process_frame(&mut self, frame: Frame) -> NetResult<Vec<Frame>> {
        self.last_activity = Instant::now();
        let mut response_frames = Vec::new();
        
        match frame.header.frame_type {
            FrameType::Settings => {
                if frame.header.flags.contains(FrameFlags::ACK) {
                    // Settings ACK received
                } else {
                    // Process settings and send ACK
                    self.remote_settings = Settings::from_frame_payload(&frame.payload)?;
                    response_frames.push(Frame::settings(Vec::new(), true));
                }
            }
            FrameType::Ping => {
                if !frame.header.flags.contains(FrameFlags::ACK) {
                    // Send PING ACK
                    let ping_data: [u8; 8] = frame.payload.try_into().map_err(|_| {
                        NetError::ProtocolError {
                            protocol: "HTTP/2".to_string(),
                        }
                    })?;
                    response_frames.push(Frame::ping(ping_data, true));
                }
            }
            FrameType::WindowUpdate => {
                if frame.header.stream_id == 0 {
                    // Connection-level window update
                    let increment = u32::from_be_bytes(
                        frame.payload.try_into().map_err(|_| {
                            NetError::ProtocolError {
                                protocol: "HTTP/2".to_string(),
                            }
                        })?
                    );
                    self.connection_window_size += increment as i32;
                } else {
                    // Stream-level window update
                    if let Some(stream) = self.streams.get_mut(&frame.header.stream_id) {
                        let increment = u32::from_be_bytes(
                            frame.payload.try_into().map_err(|_| {
                                NetError::ProtocolError {
                                    protocol: "HTTP/2".to_string(),
                                }
                            })?
                        );
                        stream.window_size += increment as i32;
                    }
                }
            }
            FrameType::Data | FrameType::Headers => {
                let stream = self.get_or_create_stream(frame.header.stream_id);
                stream.update_state(&frame)?;
                
                // Send WINDOW_UPDATE if needed
                if frame.header.frame_type == FrameType::Data && !frame.payload.is_empty() {
                    let increment = frame.payload.len() as u32;
                    response_frames.push(Frame::window_update(frame.header.stream_id, increment));
                    response_frames.push(Frame::window_update(0, increment));
                }
            }
            FrameType::RstStream => {
                if let Some(stream) = self.streams.get_mut(&frame.header.stream_id) {
                    stream.state = StreamState::Closed;
                }
            }
            _ => {
                // Handle other frame types
            }
        }
        
        Ok(response_frames)
    /// Sends data on a stream
    pub fn send_data(&mut self, stream_id: u32, data: Vec<u8>, end_stream: bool) -> NetResult<Vec<Frame>> {
        let stream = self.streams.get_mut(&stream_id).ok_or_else(|| {
            NetError::ProtocolError {
                protocol: "HTTP/2".to_string(),
            }
        })?;
        
        if !stream.can_send_data() {
            return Err(NetError::ProtocolError {
                protocol: "HTTP/2".to_string(),
            });
        let mut frames = Vec::new();
        let max_frame_size = self.remote_settings.max_frame_size as usize;
        
        // Split data into multiple frames if necessary
        for chunk in data.chunks(max_frame_size) {
            let is_last_chunk = chunk.len() < max_frame_size || chunk.as_ptr() == data.chunks(max_frame_size).last().unwrap().as_ptr();
            let frame_end_stream = end_stream && is_last_chunk;
            
            frames.push(Frame::data(stream_id, chunk.to_vec(), frame_end_stream));
        // Update stream state
        if end_stream {
            match stream.state {
                _ => {}
            }
        Ok(frames)
    /// Sends headers on a stream
    pub fn send_headers(&mut self, stream_id: u32, headers: Vec<u8>, end_stream: bool) -> NetResult<Frame> {
        let stream = self.streams.get_mut(&stream_id).ok_or_else(|| {
            NetError::ProtocolError {
                protocol: "HTTP/2".to_string(),
            }
        })?;
        
        stream.headers_sent = true;
        
        if end_stream {
            match stream.state {
                _ => {}
            }
        } else if stream.state == StreamState::Idle {
            stream.state = StreamState::Open;
        Ok(Frame::headers(stream_id, headers, end_stream, true))
    /// Gets active streams
    pub fn active_streams(&self) -> Vec<u32> {
        self.streams
            .iter()
            .filter(|(_, stream)| !stream.is_closed())
            .map(|(&id, _)| id)
            .collect()
    /// Cleans up closed streams
    pub fn cleanup_closed_streams(&mut self) {
        self.streams.retain(|_, stream| !stream.is_closed());
    /// Generates the next PING ID
    pub fn next_ping_id(&mut self) -> u64 {
        self.ping_id += 1;
        self.ping_id
    /// Checks connection health
    pub fn is_healthy(&self, timeout: Duration) -> bool {
        self.last_activity.elapsed() < timeout
    }
}

// =============================================================================
// CONVENIENCE FUNCTIONS
// =============================================================================

/// Creates HTTP/2 connection preface
pub fn create_connection_preface() -> Vec<u8> {
    HTTP2_CONNECTION_PREFACE.to_vec()
/// Validates HTTP/2 connection preface
pub fn validate_connection_preface(data: &[u8]) -> bool {
    data == HTTP2_CONNECTION_PREFACE
/// Creates initial settings frame
pub fn create_initial_settings() -> Frame {
    let settings = Settings::default();
    Frame::settings(settings.to_frame_payload(), false)
