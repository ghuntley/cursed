use crate::error::CursedError;
// TAR format support for PackRat
// RatPack (reader), RatStash (writer), RatHeader

use std::collections::HashMap;
use std::io::{Read, Write, BufReader, BufWriter};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, error, info, warn};

// Note: YeetIO integration would be added in full implementation
use super::error::{ArchiveError, ArchiveResult, invalid_header, corrupt_archive, io_error};

// TAR format constants
const TAR_BLOCK_SIZE: usize = 512;
const TAR_NAME_SIZE: usize = 100;
const TAR_MAGIC_OFFSET: usize = 257;
const TAR_VERSION_OFFSET: usize = 263;
const TAR_USTAR_MAGIC: &[u8] = b"ustar\0";
const TAR_GNU_MAGIC: &[u8] = b"ustar  \0";

// TAR format variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    FormatUnknown,
    FormatLegacy,
    FormatPOSIX,
    FormatGNU,
    FormatOldVibe, // Custom format with Gen Z metadata
}

impl Default for Format {
    fn default() -> Self {
        Format::FormatPOSIX
    }
}

// TAR header structure
#[derive(Debug, Clone)]
pub struct RatHeader {
    pub name: String,
    pub mode: i64,
    pub uid: i32,
    pub gid: i32,
    pub size: i64,
    pub mod_time: SystemTime,
    pub typeflag: u8,
    pub linkname: String,
    pub uname: String,
    pub gname: String,
    pub devmajor: i64,
    pub devminor: i64,
    pub access_time: SystemTime,
    pub change_time: SystemTime,
    pub format: Format,
    
    // Extended attributes for OldVibe format
    pub gen_z_metadata: HashMap<String, String>,
}

impl Default for RatHeader {
    fn default() -> Self {
        RatHeader {
            name: String::new(),
            mode: 0o644,
            uid: 0,
            gid: 0,
            size: 0,
            mod_time: SystemTime::now(),
            typeflag: b'0', // Regular file
            linkname: String::new(),
            uname: String::new(),
            gname: String::new(),
            devmajor: 0,
            devminor: 0,
            access_time: SystemTime::now(),
            change_time: SystemTime::now(),
            format: Format::FormatPOSIX,
            gen_z_metadata: HashMap::new(),
        }
    }
}

impl RatHeader {
    // Create header from file info (simplified version)
    pub fn from_file_info(name: &str, size: u64, mode: u32) -> Self {
        let mut header = RatHeader::default();
        header.name = name.to_string();
        header.size = size as i64;
        header.mode = mode as i64;
        header.mod_time = SystemTime::now();
        header
    }
    
    // Validate header for security issues
    pub fn validate(&self) -> ArchiveResult<()> {
        // Check for path traversal
        if self.name.contains("..") || self.name.starts_with('/') {
            return Err(ArchiveError::PathTraversal(format!("Unsafe path: {}", self.name)));
        }
        
        // Check name length
        if self.name.len() > 255 {
            return Err(ArchiveError::NameTooLong(format!("Name too long: {}", self.name.len())));
        }
        
        // Check size is reasonable
        if self.size < 0 {
            return Err(invalid_header("Negative file size"));
        }
        
        Ok(())
    }
    
    // Convert to TAR header bytes
    pub fn to_bytes(&self) -> ArchiveResult<Vec<u8>> {
        let mut header = vec![0u8; TAR_BLOCK_SIZE];
        
        // Name (100 bytes)
        let name_bytes = self.name.as_bytes();
        if name_bytes.len() >= TAR_NAME_SIZE {
            return Err(ArchiveError::NameTooLong("Name too long for TAR format".to_string()));
        }
        header[..name_bytes.len()].copy_from_slice(name_bytes);
        
        // Mode (8 bytes octal)
        let mode_str = format!("{:07o}\0", self.mode);
        header[100..108].copy_from_slice(mode_str.as_bytes());
        
        // UID (8 bytes octal)
        let uid_str = format!("{:07o}\0", self.uid);
        header[108..116].copy_from_slice(uid_str.as_bytes());
        
        // GID (8 bytes octal)
        let gid_str = format!("{:07o}\0", self.gid);
        header[116..124].copy_from_slice(gid_str.as_bytes());
        
        // Size (12 bytes octal)
        let size_str = format!("{:011o}\0", self.size);
        header[124..136].copy_from_slice(size_str.as_bytes());
        
        // Modification time (12 bytes octal)
        let mtime = self.mod_time.duration_since(UNIX_EPOCH)
            .unwrap_or_default().as_secs();
        let mtime_str = format!("{:011o}\0", mtime);
        header[136..148].copy_from_slice(mtime_str.as_bytes());
        
        // Checksum (8 bytes) - calculate after setting other fields
        header[148..156].copy_from_slice(b"        ");
        
        // Typeflag (1 byte)
        header[156] = self.typeflag;
        
        // Linkname (100 bytes)
        let linkname_bytes = self.linkname.as_bytes();
        if linkname_bytes.len() < 100 {
            header[157..157 + linkname_bytes.len()].copy_from_slice(linkname_bytes);
        }
        
        // Magic (6 bytes)
        header[TAR_MAGIC_OFFSET..TAR_MAGIC_OFFSET + 6].copy_from_slice(b"ustar\0");
        
        // Version (2 bytes)
        header[TAR_VERSION_OFFSET..TAR_VERSION_OFFSET + 2].copy_from_slice(b"00");
        
        // User name (32 bytes)
        let uname_bytes = self.uname.as_bytes();
        if uname_bytes.len() < 32 {
            header[265..265 + uname_bytes.len()].copy_from_slice(uname_bytes);
        }
        
        // Group name (32 bytes)
        let gname_bytes = self.gname.as_bytes();
        if gname_bytes.len() < 32 {
            header[297..297 + gname_bytes.len()].copy_from_slice(gname_bytes);
        }
        
        // Calculate and set checksum
        let checksum = self.calculate_checksum(&header);
        let checksum_str = format!("{:06o}\0 ", checksum);
        header[148..156].copy_from_slice(checksum_str.as_bytes());
        
        Ok(header)
    }
    
    // Parse header from bytes
    pub fn from_bytes(data: &[u8]) -> ArchiveResult<Self> {
        if data.len() < TAR_BLOCK_SIZE {
            return Err(invalid_header("Header too short"));
        }
        
        // Check if this is an empty block (all zeros)
        if data.iter().all(|&b| b == 0) {
            return Err(invalid_header("Empty header block"));
        }
        
        let mut header = RatHeader::default();
        
        // Parse name
        header.name = parse_null_terminated_string(&data[0..100])?;
        
        // Parse mode
        header.mode = parse_octal(&data[100..108])? as i64;
        
        // Parse UID
        header.uid = parse_octal(&data[108..116])? as i32;
        
        // Parse GID
        header.gid = parse_octal(&data[116..124])? as i32;
        
        // Parse size
        header.size = parse_octal(&data[124..136])? as i64;
        
        // Parse modification time
        let mtime_secs = parse_octal(&data[136..148])? as u64;
        header.mod_time = UNIX_EPOCH + std::time::Duration::from_secs(mtime_secs);
        
        // Parse typeflag
        header.typeflag = data[156];
        
        // Parse linkname
        header.linkname = parse_null_terminated_string(&data[157..257])?;
        
        // Check magic and determine format
        let magic = &data[TAR_MAGIC_OFFSET..TAR_MAGIC_OFFSET + 6];
        header.format = if magic == TAR_USTAR_MAGIC {
            Format::FormatPOSIX
        } else if magic.starts_with(b"ustar") {
            Format::FormatGNU
        } else {
            Format::FormatLegacy
        };
        
        // Parse user and group names (POSIX/GNU only)
        if header.format != Format::FormatLegacy {
            header.uname = parse_null_terminated_string(&data[265..297])?;
            header.gname = parse_null_terminated_string(&data[297..329])?;
        }
        
        // Verify checksum
        let stored_checksum = parse_octal(&data[148..156])? as u32;
        let calculated_checksum = header.calculate_checksum(data);
        if stored_checksum != calculated_checksum {
            return Err(corrupt_archive("Header checksum mismatch"));
        }
        
        Ok(header)
    }
    
    // Calculate TAR header checksum
    fn calculate_checksum(&self, header: &[u8]) -> u32 {
        let mut checksum = 0u32;
        for (i, &byte) in header.iter().enumerate() {
            if i >= 148 && i < 156 {
                // Checksum field is treated as spaces
                checksum += b' ' as u32;
            } else {
                checksum += byte as u32;
            }
        }
        checksum
    }
}

// TAR reader (RatPack)
pub struct RatPack<R: Read> {
    reader: BufReader<R>,
    current_file_remaining: i64,
    finished: bool,
}

impl<R: Read> RatPack<R> {
    pub fn new(reader: R) -> Self {
        RatPack {
            reader: BufReader::new(reader),
            current_file_remaining: 0,
            finished: false,
        }
    }
    
    // Read next file header
    pub fn next(&mut self) -> ArchiveResult<Option<RatHeader>> {
        if self.finished {
            return Ok(None);
        }
        
        // Skip remaining bytes from current file
        if self.current_file_remaining > 0 {
            self.skip_current_file()?;
        }
        
        // Read header block
        let mut header_buf = vec![0u8; TAR_BLOCK_SIZE];
        match self.reader.read_exact(&mut header_buf) {
            Ok(()) => {},
            Err(ref e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                self.finished = true;
                return Ok(None);
            },
            Err(e) => return Err(e.into()),
        }
        
        // Check for end-of-archive (two empty blocks)
        if header_buf.iter().all(|&b| b == 0) {
            // Read second empty block to confirm end
            let mut empty_buf = vec![0u8; TAR_BLOCK_SIZE];
            match self.reader.read_exact(&mut empty_buf) {
                Ok(()) if empty_buf.iter().all(|&b| b == 0) => {
                    self.finished = true;
                    return Ok(None);
                },
                _ => return Err(corrupt_archive("Invalid end-of-archive marker")),
            }
        }
        
        // Parse header
        let header = RatHeader::from_bytes(&header_buf)?;
        header.validate()?;
        
        self.current_file_remaining = header.size;
        
        debug!("Read TAR header: {} ({} bytes)", header.name, header.size);
        Ok(Some(header))
    }
    
    // Skip current file data
    fn skip_current_file(&mut self) -> ArchiveResult<()> {
        if self.current_file_remaining <= 0 {
            return Ok(());
        }
        
        // Calculate blocks to skip (including padding)
        let blocks_to_skip = (self.current_file_remaining + TAR_BLOCK_SIZE as i64 - 1) / TAR_BLOCK_SIZE as i64;
        let bytes_to_skip = blocks_to_skip * TAR_BLOCK_SIZE as i64;
        
        // Skip in chunks to avoid large memory allocation
        let mut remaining = bytes_to_skip;
        let mut buffer = vec![0u8; 8192];
        
        while remaining > 0 {
            let to_read = std::cmp::min(remaining, buffer.len() as i64);
            self.reader.read_exact(&mut buffer[..to_read as usize])?;
            remaining -= to_read;
        }
        
        self.current_file_remaining = 0;
        Ok(())
    }
}

impl<R: Read> Read for RatPack<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.current_file_remaining <= 0 {
            return Ok(0);
        }
        
        let to_read = std::cmp::min(buf.len() as i64, self.current_file_remaining) as usize;
        let bytes_read = self.reader.read(&mut buf[..to_read])?;
        self.current_file_remaining -= bytes_read as i64;
        
        Ok(bytes_read)
    }
}

// TAR writer (RatStash)
pub struct RatStash<W: Write> {
    writer: BufWriter<W>,
    current_file_size: i64,
    current_file_written: i64,
    finished: bool,
}

impl<W: Write> RatStash<W> {
    pub fn new(writer: W) -> Self {
        RatStash {
            writer: BufWriter::new(writer),
            current_file_size: 0,
            current_file_written: 0,
            finished: false,
        }
    }
    
    // Write file header
    pub fn write_header(&mut self, header: &RatHeader) -> ArchiveResult<()> {
        if self.finished {
            return Err(io_error("Archive already finished"));
        }
        
        // Finish current file if needed
        if self.current_file_written < self.current_file_size {
            self.pad_current_file()?;
        }
        
        // Validate header
        header.validate()?;
        
        // Write header
        let header_bytes = header.to_bytes()?;
        self.writer.write_all(&header_bytes)?;
        
        self.current_file_size = header.size;
        self.current_file_written = 0;
        
        debug!("Wrote TAR header: {} ({} bytes)", header.name, header.size);
        Ok(())
    }
    
    // Pad current file to block boundary
    fn pad_current_file(&mut self) -> ArchiveResult<()> {
        let remaining = self.current_file_size - self.current_file_written;
        if remaining > 0 {
            // Write zeros for remaining file data
            let zeros = vec![0u8; remaining as usize];
            self.writer.write_all(&zeros)?;
        }
        
        // Pad to block boundary
        let padding_needed = (TAR_BLOCK_SIZE as i64 - (self.current_file_size % TAR_BLOCK_SIZE as i64)) % TAR_BLOCK_SIZE as i64;
        if padding_needed > 0 {
            let padding = vec![0u8; padding_needed as usize];
            self.writer.write_all(&padding)?;
        }
        
        self.current_file_written = self.current_file_size;
        Ok(())
    }
    
    // Flush buffered data
    pub fn flush(&mut self) -> ArchiveResult<()> {
        self.writer.flush()?;
        Ok(())
    }
    
    // Close archive (write end markers)
    pub fn close(&mut self) -> ArchiveResult<()> {
        if self.finished {
            return Ok(());
        }
        
        // Finish current file
        if self.current_file_written < self.current_file_size {
            self.pad_current_file()?;
        }
        
        // Write two empty blocks to mark end of archive
        let empty_block = vec![0u8; TAR_BLOCK_SIZE];
        self.writer.write_all(&empty_block)?;
        self.writer.write_all(&empty_block)?;
        
        self.writer.flush()?;
        self.finished = true;
        
        debug!("Closed TAR archive");
        Ok(())
    }
}

impl<W: Write> Write for RatStash<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if self.finished {
            return Err(std::io::Error::new(std::io::ErrorKind::BrokenPipe, "Archive finished"));
        }
        
        let remaining_space = self.current_file_size - self.current_file_written;
        if remaining_space <= 0 {
            return Ok(0);
        }
        
        let to_write = std::cmp::min(buf.len() as i64, remaining_space) as usize;
        let bytes_written = self.writer.write(&buf[..to_write])?;
        self.current_file_written += bytes_written as i64;
        
        Ok(bytes_written)
    }
    
    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

impl<W: Write> Drop for RatStash<W> {
    fn drop(&mut self) {
        if !self.finished {
            let _ = self.close();
        }
    }
}

// Helper functions for parsing TAR headers
fn parse_null_terminated_string(data: &[u8]) -> ArchiveResult<String> {
    let end = data.iter().position(|&b| b == 0).unwrap_or(data.len());
    String::from_utf8(data[..end].to_vec())
        .map_err(|_| invalid_header("Invalid UTF-8 in string field"))
}

fn parse_octal(data: &[u8]) -> ArchiveResult<u64> {
    let s = parse_null_terminated_string(data)?;
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Ok(0);
    }
    
    u64::from_str_radix(trimmed, 8)
        .map_err(|_| invalid_header(&format!("Invalid octal number: {}", trimmed)))
}

// Create header from file info (public function)
pub fn FileInfoHeader(name: &str, size: u64, mode: u32, link: &str) -> ArchiveResult<RatHeader> {
    let mut header = RatHeader::from_file_info(name, size, mode);
    header.linkname = link.to_string();
    Ok(header)
}

