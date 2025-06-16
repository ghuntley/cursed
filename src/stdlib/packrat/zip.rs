// ZIP format support for PackRat
// HoardPack (reader), HoardStash (writer), HoardFile, HoardFileHeader

use std::collections::HashMap;
use std::io::{Read, Write, Seek, SeekFrom, BufReader, BufWriter, Cursor};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, error, info, warn};

// Note: YeetIO integration would be added in full implementation
use super::error::{ArchiveError, ArchiveResult, invalid_header, corrupt_archive, io_error, invalid_format};

// ZIP format constants
const ZIP_LOCAL_HEADER_SIGNATURE: u32 = 0x04034b50;
const ZIP_CENTRAL_HEADER_SIGNATURE: u32 = 0x02014b50;
const ZIP_END_CENTRAL_DIR_SIGNATURE: u32 = 0x06054b50;
const ZIP_DATA_DESCRIPTOR_SIGNATURE: u32 = 0x08074b50;

const ZIP_VERSION_MADE_BY: u16 = 20; // Version 2.0
const ZIP_VERSION_NEEDED: u16 = 20;  // Version 2.0

// Compression methods
const COMPRESSION_STORED: u16 = 0;  // No compression
const COMPRESSION_DEFLATE: u16 = 8; // Deflate compression

// ZIP file header
#[derive(Debug, Clone)]
pub struct HoardFileHeader {
    pub name: String,
    pub comment: String,
    pub creator_version: u16,
    pub reader_version: u16,
    pub flags: u16,
    pub method: u16,
    pub modified_time: u16,
    pub modified_date: u16,
    pub crc32: u32,
    pub compressed_size: u32,
    pub uncompressed_size: u32,
    pub extra: Vec<u8>,
    pub external_attrs: u32,
    pub modified: SystemTime,
    
    // Internal fields
    pub(crate) offset: u64,
    pub(crate) header_offset: u64,
}

impl Default for HoardFileHeader {
    fn default() -> Self {
        let now = SystemTime::now();
        let (dos_time, dos_date) = system_time_to_dos_time(now);
        
        HoardFileHeader {
            name: String::new(),
            comment: String::new(),
            creator_version: ZIP_VERSION_MADE_BY,
            reader_version: ZIP_VERSION_NEEDED,
            flags: 0,
            method: COMPRESSION_STORED,
            modified_time: dos_time,
            modified_date: dos_date,
            crc32: 0,
            compressed_size: 0,
            uncompressed_size: 0,
            extra: Vec::new(),
            external_attrs: 0,
            modified: now,
            offset: 0,
            header_offset: 0,
        }
    }
}

impl HoardFileHeader {
    pub fn new(name: &str) -> Self {
        let mut header = HoardFileHeader::default();
        header.name = name.to_string();
        header
    }
    
    pub fn validate(&self) -> ArchiveResult<()> {
        // Check for path traversal
        if self.name.contains("..") || self.name.starts_with('/') {
            return Err(ArchiveError::PathTraversal(format!("Unsafe path: {}", self.name)));
        }
        
        // Check name length
        if self.name.len() > 65535 {
            return Err(ArchiveError::NameTooLong(format!("Name too long: {}", self.name.len())));
        }
        
        // Check sizes are reasonable
        if self.compressed_size as u64 > i64::MAX as u64 {
            return Err(invalid_header("Compressed size too large"));
        }
        
        if self.uncompressed_size as u64 > i64::MAX as u64 {
            return Err(invalid_header("Uncompressed size too large"));
        }
        
        Ok(())
    }
    
    // Write local file header
    pub(crate) fn write_local_header<W: Write>(&self, writer: &mut W) -> ArchiveResult<()> {
        let name_bytes = self.name.as_bytes();
        
        // Local file header signature
        writer.write_all(&ZIP_LOCAL_HEADER_SIGNATURE.to_le_bytes())?;
        
        // Version needed to extract
        writer.write_all(&self.reader_version.to_le_bytes())?;
        
        // General purpose bit flag
        writer.write_all(&self.flags.to_le_bytes())?;
        
        // Compression method
        writer.write_all(&self.method.to_le_bytes())?;
        
        // Last mod file time/date
        writer.write_all(&self.modified_time.to_le_bytes())?;
        writer.write_all(&self.modified_date.to_le_bytes())?;
        
        // CRC-32, compressed size, uncompressed size
        writer.write_all(&self.crc32.to_le_bytes())?;
        writer.write_all(&self.compressed_size.to_le_bytes())?;
        writer.write_all(&self.uncompressed_size.to_le_bytes())?;
        
        // File name length, extra field length
        writer.write_all(&(name_bytes.len() as u16).to_le_bytes())?;
        writer.write_all(&(self.extra.len() as u16).to_le_bytes())?;
        
        // File name
        writer.write_all(name_bytes)?;
        
        // Extra field
        writer.write_all(&self.extra)?;
        
        Ok(())
    }
    
    // Write central directory header
    pub(crate) fn write_central_header<W: Write>(&self, writer: &mut W) -> ArchiveResult<()> {
        let name_bytes = self.name.as_bytes();
        let comment_bytes = self.comment.as_bytes();
        
        // Central file header signature
        writer.write_all(&ZIP_CENTRAL_HEADER_SIGNATURE.to_le_bytes())?;
        
        // Version made by
        writer.write_all(&self.creator_version.to_le_bytes())?;
        
        // Version needed to extract
        writer.write_all(&self.reader_version.to_le_bytes())?;
        
        // General purpose bit flag
        writer.write_all(&self.flags.to_le_bytes())?;
        
        // Compression method
        writer.write_all(&self.method.to_le_bytes())?;
        
        // Last mod file time/date
        writer.write_all(&self.modified_time.to_le_bytes())?;
        writer.write_all(&self.modified_date.to_le_bytes())?;
        
        // CRC-32, compressed size, uncompressed size
        writer.write_all(&self.crc32.to_le_bytes())?;
        writer.write_all(&self.compressed_size.to_le_bytes())?;
        writer.write_all(&self.uncompressed_size.to_le_bytes())?;
        
        // File name length, extra field length, comment length
        writer.write_all(&(name_bytes.len() as u16).to_le_bytes())?;
        writer.write_all(&(self.extra.len() as u16).to_le_bytes())?;
        writer.write_all(&(comment_bytes.len() as u16).to_le_bytes())?;
        
        // Disk number start, internal file attributes, external file attributes
        writer.write_all(&0u16.to_le_bytes())?; // Disk number
        writer.write_all(&0u16.to_le_bytes())?; // Internal attrs
        writer.write_all(&self.external_attrs.to_le_bytes())?;
        
        // Relative offset of local header
        writer.write_all(&(self.header_offset as u32).to_le_bytes())?;
        
        // File name
        writer.write_all(name_bytes)?;
        
        // Extra field
        writer.write_all(&self.extra)?;
        
        // File comment
        writer.write_all(comment_bytes)?;
        
        Ok(())
    }
}

// ZIP file entry
#[derive(Debug)]
pub struct HoardFile {
    pub file_header: HoardFileHeader,
    pub(crate) data: Vec<u8>,
}

impl HoardFile {
    pub fn new(header: HoardFileHeader, data: Vec<u8>) -> Self {
        HoardFile {
            file_header: header,
            data,
        }
    }
    
    // Open file for reading
    pub fn open(&self) -> ArchiveResult<Box<dyn Read>> {
        // For now, return cursor over stored data
        // In full implementation, would handle compression
        Ok(Box::new(Cursor::new(self.data.clone())))
    }
    
    // Get data offset (for compatibility)
    pub fn data_offset(&self) -> ArchiveResult<i64> {
        Ok(self.file_header.offset as i64)
    }
}

// ZIP reader (HoardPack)
pub struct HoardPack<R: Read + Seek> {
    reader: R,
    pub files: Vec<HoardFile>,
    central_dir_offset: u64,
    central_dir_size: u64,
}

impl<R: Read + Seek> HoardPack<R> {
    pub fn new(mut reader: R, size: i64) -> ArchiveResult<Self> {
        Self::new_with_files(reader, size, Vec::new())
    }
    
    pub fn new_with_files(mut reader: R, size: i64, files: Vec<HoardFile>) -> ArchiveResult<Self> {
        let mut pack = HoardPack {
            reader,
            files,
            central_dir_offset: 0,
            central_dir_size: 0,
        };
        
        // Find and parse central directory
        pack.read_central_directory(size as u64)?;
        
        Ok(pack)
    }
    
    fn read_central_directory(&mut self, archive_size: u64) -> ArchiveResult<()> {
        // Find end of central directory record
        let eocd_offset = self.find_eocd_record(archive_size)?;
        
        // Read EOCD record
        self.reader.seek(SeekFrom::Start(eocd_offset))?;
        let eocd = self.read_eocd_record()?;
        
        self.central_dir_offset = eocd.central_dir_offset;
        self.central_dir_size = eocd.central_dir_size;
        
        // Read central directory entries
        self.reader.seek(SeekFrom::Start(self.central_dir_offset))?;
        for _ in 0..eocd.total_entries {
            let file = self.read_central_dir_entry()?;
            self.files.push(file);
        }
        
        debug!("Read {} files from ZIP central directory", self.files.len());
        Ok(())
    }
    
    fn find_eocd_record(&mut self, archive_size: u64) -> ArchiveResult<u64> {
        // Search backwards from end of file for EOCD signature
        const MAX_COMMENT_SIZE: u64 = 65535;
        const EOCD_SIZE: u64 = 22;
        
        let search_start = if archive_size > MAX_COMMENT_SIZE + EOCD_SIZE {
            archive_size - MAX_COMMENT_SIZE - EOCD_SIZE
        } else {
            0
        };
        
        self.reader.seek(SeekFrom::Start(search_start))?;
        let mut buffer = Vec::new();
        self.reader.read_to_end(&mut buffer)?;
        
        // Search for EOCD signature
        for i in (0..buffer.len().saturating_sub(4)).rev() {
            let signature = u32::from_le_bytes([
                buffer[i], buffer[i+1], buffer[i+2], buffer[i+3]
            ]);
            
            if signature == ZIP_END_CENTRAL_DIR_SIGNATURE {
                return Ok(search_start + i as u64);
            }
        }
        
        Err(corrupt_archive("Could not find end of central directory record"))
    }
    
    fn read_eocd_record(&mut self) -> ArchiveResult<EndOfCentralDir> {
        let mut buf = [0u8; 22];
        self.reader.read_exact(&mut buf)?;
        
        let signature = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
        if signature != ZIP_END_CENTRAL_DIR_SIGNATURE {
            return Err(corrupt_archive("Invalid EOCD signature"));
        }
        
        Ok(EndOfCentralDir {
            disk_number: u16::from_le_bytes([buf[4], buf[5]]),
            central_dir_disk: u16::from_le_bytes([buf[6], buf[7]]),
            disk_entries: u16::from_le_bytes([buf[8], buf[9]]),
            total_entries: u16::from_le_bytes([buf[10], buf[11]]),
            central_dir_size: u32::from_le_bytes([buf[12], buf[13], buf[14], buf[15]]) as u64,
            central_dir_offset: u32::from_le_bytes([buf[16], buf[17], buf[18], buf[19]]) as u64,
            comment_length: u16::from_le_bytes([buf[20], buf[21]]),
        })
    }
    
    fn read_central_dir_entry(&mut self) -> ArchiveResult<HoardFile> {
        let mut buf = [0u8; 46];
        self.reader.read_exact(&mut buf)?;
        
        let signature = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
        if signature != ZIP_CENTRAL_HEADER_SIGNATURE {
            return Err(corrupt_archive("Invalid central directory header signature"));
        }
        
        let name_len = u16::from_le_bytes([buf[28], buf[29]]) as usize;
        let extra_len = u16::from_le_bytes([buf[30], buf[31]]) as usize;
        let comment_len = u16::from_le_bytes([buf[32], buf[33]]) as usize;
        
        // Read variable-length fields
        let mut name_buf = vec![0u8; name_len];
        self.reader.read_exact(&mut name_buf)?;
        let name = String::from_utf8(name_buf)
            .map_err(|_| invalid_format("Invalid UTF-8 in file name"))?;
        
        let mut extra = vec![0u8; extra_len];
        self.reader.read_exact(&mut extra)?;
        
        let mut comment_buf = vec![0u8; comment_len];
        self.reader.read_exact(&mut comment_buf)?;
        let comment = String::from_utf8(comment_buf)
            .map_err(|_| invalid_format("Invalid UTF-8 in comment"))?;
        
        let header = HoardFileHeader {
            name,
            comment,
            creator_version: u16::from_le_bytes([buf[4], buf[5]]),
            reader_version: u16::from_le_bytes([buf[6], buf[7]]),
            flags: u16::from_le_bytes([buf[8], buf[9]]),
            method: u16::from_le_bytes([buf[10], buf[11]]),
            modified_time: u16::from_le_bytes([buf[12], buf[13]]),
            modified_date: u16::from_le_bytes([buf[14], buf[15]]),
            crc32: u32::from_le_bytes([buf[16], buf[17], buf[18], buf[19]]),
            compressed_size: u32::from_le_bytes([buf[20], buf[21], buf[22], buf[23]]),
            uncompressed_size: u32::from_le_bytes([buf[24], buf[25], buf[26], buf[27]]),
            extra,
            external_attrs: u32::from_le_bytes([buf[38], buf[39], buf[40], buf[41]]),
            modified: dos_time_to_system_time(
                u16::from_le_bytes([buf[12], buf[13]]),
                u16::from_le_bytes([buf[14], buf[15]])
            ),
            offset: u32::from_le_bytes([buf[42], buf[43], buf[44], buf[45]]) as u64,
            header_offset: u32::from_le_bytes([buf[42], buf[43], buf[44], buf[45]]) as u64,
        };
        
        // For now, create empty file data (would read actual data in full implementation)
        Ok(HoardFile::new(header, Vec::new()))
    }
}

// ZIP writer (HoardStash)
pub struct HoardStash<W: Write + Seek> {
    writer: W,
    files: Vec<HoardFileHeader>,
    current_offset: u64,
    finished: bool,
}

impl<W: Write + Seek> HoardStash<W> {
    pub fn new(writer: W) -> Self {
        HoardStash {
            writer,
            files: Vec::new(),
            current_offset: 0,
            finished: false,
        }
    }
    
    // Create a new file entry
    pub fn create(&mut self, name: &str) -> ArchiveResult<ZipFileWriter<'_, W>> {
        if self.finished {
            return Err(io_error("Archive already finished"));
        }
        
        let header = HoardFileHeader::new(name);
        self.create_header(&header)
    }
    
    // Create file with custom header
    pub fn create_header(&mut self, header: &HoardFileHeader) -> ArchiveResult<ZipFileWriter<'_, W>> {
        if self.finished {
            return Err(io_error("Archive already finished"));
        }
        
        header.validate()?;
        
        let mut file_header = header.clone();
        file_header.header_offset = self.current_offset;
        
        // Write local file header
        file_header.write_local_header(&mut self.writer)?;
        
        let header_size = 30 + file_header.name.len() + file_header.extra.len();
        self.current_offset += header_size as u64;
        
        Ok(ZipFileWriter {
            stash: self,
            header: file_header,
            data_start: self.current_offset,
            crc: crc32fast::Hasher::new(),
            data: Vec::new(),
        })
    }
    
    // Close archive
    pub fn close(&mut self) -> ArchiveResult<()> {
        if self.finished {
            return Ok(());
        }
        
        let central_dir_start = self.current_offset;
        
        // Write central directory
        for header in &self.files {
            header.write_central_header(&mut self.writer)?;
            let entry_size = 46 + header.name.len() + header.extra.len() + header.comment.len();
            self.current_offset += entry_size as u64;
        }
        
        let central_dir_size = self.current_offset - central_dir_start;
        
        // Write end of central directory record
        self.write_eocd_record(central_dir_start, central_dir_size)?;
        
        self.finished = true;
        debug!("Closed ZIP archive with {} files", self.files.len());
        Ok(())
    }
    
    fn write_eocd_record(&mut self, central_dir_offset: u64, central_dir_size: u64) -> ArchiveResult<()> {
        // End of central directory signature
        self.writer.write_all(&ZIP_END_CENTRAL_DIR_SIGNATURE.to_le_bytes())?;
        
        // Disk numbers
        self.writer.write_all(&0u16.to_le_bytes())?; // This disk
        self.writer.write_all(&0u16.to_le_bytes())?; // Central dir disk
        
        // Number of entries
        let num_files = self.files.len() as u16;
        self.writer.write_all(&num_files.to_le_bytes())?; // Entries on this disk
        self.writer.write_all(&num_files.to_le_bytes())?; // Total entries
        
        // Central directory size and offset
        self.writer.write_all(&(central_dir_size as u32).to_le_bytes())?;
        self.writer.write_all(&(central_dir_offset as u32).to_le_bytes())?;
        
        // Comment length (0)
        self.writer.write_all(&0u16.to_le_bytes())?;
        
        Ok(())
    }
    
    fn add_file(&mut self, mut header: HoardFileHeader) {
        header.offset = header.header_offset;
        self.files.push(header);
    }
}

impl<W: Write + Seek> Drop for HoardStash<W> {
    fn drop(&mut self) {
        if !self.finished {
            let _ = self.close();
        }
    }
}

// ZIP file writer helper
pub struct ZipFileWriter<'a, W: Write + Seek> {
    stash: &'a mut HoardStash<W>,
    header: HoardFileHeader,
    data_start: u64,
    crc: crc32fast::Hasher,
    data: Vec<u8>,
}

impl<'a, W: Write + Seek> Write for ZipFileWriter<'a, W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.data.extend_from_slice(buf);
        Ok(buf.len())
    }
    
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl<'a, W: Write + Seek> Drop for ZipFileWriter<'a, W> {
    fn drop(&mut self) {
        // Write data and finalize file
        if let Err(e) = self.finish() {
            error!("Error finishing ZIP file: {}", e);
        }
    }
}

impl<'a, W: Write + Seek> ZipFileWriter<'a, W> {
    fn finish(&mut self) -> ArchiveResult<()> {
        // Write file data
        self.stash.writer.write_all(&self.data)?;
        
        // Update header with actual sizes and CRC
        self.header.uncompressed_size = self.data.len() as u32;
        self.header.compressed_size = self.data.len() as u32; // No compression for now
        self.crc.update(&self.data);
        self.header.crc32 = self.crc.finalize();
        
        // Update current offset
        self.stash.current_offset += self.data.len() as u64;
        
        // Add to file list
        self.stash.add_file(self.header.clone());
        
        Ok(())
    }
}

// Helper structures
#[derive(Debug)]
struct EndOfCentralDir {
    disk_number: u16,
    central_dir_disk: u16,
    disk_entries: u16,
    total_entries: u16,
    central_dir_size: u64,
    central_dir_offset: u64,
    comment_length: u16,
}

// Create header from file info (public function)  
pub fn FileInfoHeader(name: &str, size: u64, mode: u32) -> ArchiveResult<HoardFileHeader> {
    let mut header = HoardFileHeader::new(name);
    header.uncompressed_size = size as u32;
    header.external_attrs = (mode << 16) as u32;
    Ok(header)
}

// DOS time conversion helpers
fn system_time_to_dos_time(time: SystemTime) -> (u16, u16) {
    let duration = time.duration_since(UNIX_EPOCH).unwrap_or_default();
    let secs = duration.as_secs();
    
    // Convert to DOS date/time format (simplified)
    let dos_time = ((secs % 86400) / 2) as u16; // Seconds since midnight / 2
    let dos_date = ((secs / 86400) + 719163) as u16; // Days since 1980-01-01
    
    (dos_time, dos_date)
}

fn dos_time_to_system_time(dos_time: u16, dos_date: u16) -> SystemTime {
    // Convert DOS date/time to SystemTime (simplified)
    let days_since_epoch = dos_date as u64 - 719163;
    let secs_in_day = (dos_time as u64) * 2;
    let total_secs = days_since_epoch * 86400 + secs_in_day;
    
    UNIX_EPOCH + std::time::Duration::from_secs(total_secs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use tracing::info;
    
    #[test]
    fn test_hoard_file_header_creation() {
        info!("Testing HoardFileHeader creation");
        
        let header = HoardFileHeader::new("test.txt");
        assert_eq!(header.name, "test.txt");
        assert_eq!(header.method, COMPRESSION_STORED);
        
        info!("HoardFileHeader creation test passed");
    }
    
    #[test]
    fn test_hoard_file_header_validation() {
        info!("Testing HoardFileHeader validation");
        
        let mut header = HoardFileHeader::new("test.txt");
        assert!(header.validate().is_ok());
        
        // Test path traversal
        header.name = "../evil.txt".to_string();
        assert!(header.validate().is_err());
        
        // Test absolute path
        header.name = "/etc/passwd".to_string();
        assert!(header.validate().is_err());
        
        info!("HoardFileHeader validation tests passed");
    }
    
    #[test]
    fn test_hoard_file_creation() {
        info!("Testing HoardFile creation");
        
        let header = HoardFileHeader::new("test.txt");
        let data = b"Hello, World!".to_vec();
        let file = HoardFile::new(header, data);
        
        assert_eq!(file.file_header.name, "test.txt");
        
        info!("HoardFile creation test passed");
    }
    
    #[test]
    fn test_hoard_stash_creation() {
        info!("Testing HoardStash creation");
        
        let buffer = Cursor::new(Vec::new());
        let mut stash = HoardStash::new(buffer);
        
        let mut writer = stash.create("test.txt").unwrap();
        writer.write_all(b"Hello, World!").unwrap();
        drop(writer);
        
        stash.close().unwrap();
        
        info!("HoardStash creation test passed");
    }
    
    #[test]
    fn test_dos_time_conversion() {
        info!("Testing DOS time conversion");
        
        let now = SystemTime::now();
        let (dos_time, dos_date) = system_time_to_dos_time(now);
        let converted = dos_time_to_system_time(dos_time, dos_date);
        
        // Should be roughly the same (within a day due to precision loss)
        let diff = now.duration_since(converted).unwrap_or_else(|_| {
            converted.duration_since(now).unwrap()
        });
        assert!(diff.as_secs() < 86400);
        
        info!("DOS time conversion tests passed");
    }
    
    #[test]
    fn test_file_info_header() {
        info!("Testing FileInfoHeader function");
        
        let header = FileInfoHeader("test.txt", 100, 0o644).unwrap();
        assert_eq!(header.name, "test.txt");
        assert_eq!(header.uncompressed_size, 100);
        
        info!("FileInfoHeader function test passed");
    }
}
