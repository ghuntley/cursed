use crate::error::CursedError;
// Compression utilities for PackRat

use std::fs::File;
use std::io::{Read, Write, BufReader, BufWriter};
use std::path::Path;
use tracing::{debug, error, info, warn};

// Note: YeetIO integration would be added in full implementation
use super::error::{ArchiveError, ArchiveResult, io_error, invalid_format, compression_error, decompression_error};
use super::tar::{RatPack, RatStash};
use super::zip::{HoardPack, HoardStash};

// Format detection constants
const ZIP_SIGNATURE: &[u8] = &[0x50, 0x4b, 0x03, 0x04]; // "PK\x03\x04"
const TAR_USTAR_MAGIC: &[u8] = b"ustar\0";
const GZIP_MAGIC: &[u8] = &[0x1f, 0x8b];
const BZIP2_MAGIC: &[u8] = b"BZ";

// Compression format enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionFormat {
// Archive format enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveFormat {
// Check if data is ZIP format
pub fn IsZip<R: Read>(mut reader: R) -> bool {
    let mut magic = [0u8; 4];
    match reader.read_exact(&mut magic) {
    }
}

// Check if data is TAR format
pub fn IsTar<R: Read + std::io::Seek>(mut reader: R) -> bool {
    // TAR magic is at offset 257
    if reader.seek(std::io::SeekFrom::Start(257)).is_err() {
        return false;
    let mut magic = [0u8; 6];
    match reader.read_exact(&mut magic) {
    }
}

// Detect archive format from file
pub fn detect_format<P: AsRef<Path>>(path: P) -> ArchiveResult<ArchiveFormat> {
    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);
    detect_format_from_reader(&mut reader)
// Detect format from reader
pub fn detect_format_from_reader<R: Read + std::io::Seek>(reader: &mut R) -> ArchiveResult<ArchiveFormat> {
    let mut magic = [0u8; 8];
    reader.seek(std::io::SeekFrom::Start(0))?;
    
    match reader.read(&mut magic) {
        Ok(n) if n >= 4 => {
            // Check ZIP signature
            if magic[..4] == *ZIP_SIGNATURE {
                return Ok(ArchiveFormat::Zip);
            // Check GZIP signature
            if n >= 2 && magic[..2] == *GZIP_MAGIC {
                // Could be tar.gz, check further
                return detect_compressed_tar_format(reader, CompressionFormat::Gzip);
            // Check BZIP2 signature
            if n >= 2 && magic[..2] == *BZIP2_MAGIC {
                return detect_compressed_tar_format(reader, CompressionFormat::Bzip2);
            // Check TAR format (magic at offset 257)
            reader.seek(std::io::SeekFrom::Start(257))?;
            let mut tar_magic = [0u8; 6];
            if reader.read_exact(&mut tar_magic).is_ok() && tar_magic == *TAR_USTAR_MAGIC {
                return Ok(ArchiveFormat::Tar);
            Ok(ArchiveFormat::Unknown)
    }
}

// Detect compressed TAR format
fn detect_compressed_tar_format<R: Read + std::io::Seek>(
    compression: CompressionFormat
) -> ArchiveResult<ArchiveFormat> {
    // For now, assume compressed files are TAR archives
    // In a full implementation, we would decompress and check
    match compression {
    }
}

// Generic compression function
pub fn Compress<P: AsRef<Path>>(src: P, dst: P, format: &str) -> ArchiveResult<()> {
    let src_path = src.as_ref();
    let dst_path = dst.as_ref();
    
           src_path.display(), dst_path.display(), format);
    
    match format.to_lowercase().as_str() {
    }
}

// Generic decompression function
pub fn Decompress<P: AsRef<Path>>(src: P, dst: P) -> ArchiveResult<()> {
    let src_path = src.as_ref();
    let dst_path = dst.as_ref();
    
    debug!("Decompressing {} to {}", src_path.display(), dst_path.display());
    
    // Detect format
    let format = detect_format(src_path)?;
    
    match format {
    }
}

// TAR compression
fn compress_tar<P: AsRef<Path>>(src: P, dst: P) -> ArchiveResult<()> {
    let dst_file = File::create(dst)?;
    let mut tar = RatStash::new(dst_file);
    
    // Add files to TAR (simplified - would recursively walk directory in full implementation)
    if src.as_ref().is_file() {
        add_file_to_tar(&mut tar, src.as_ref())?;
    } else {
        add_directory_to_tar(&mut tar, src.as_ref())?;
    tar.close()?;
    Ok(())
// ZIP compression
fn compress_zip<P: AsRef<Path>>(src: P, dst: P) -> ArchiveResult<()> {
    let dst_file = File::create(dst)?;
    let dst_cursor = std::io::Cursor::new(Vec::new());
    let mut zip = HoardStash::new(dst_cursor);
    
    // Add files to ZIP (simplified - would recursively walk directory in full implementation)
    if src.as_ref().is_file() {
        add_file_to_zip(&mut zip, src.as_ref())?;
    } else {
        add_directory_to_zip(&mut zip, src.as_ref())?;
    zip.close()?;
    Ok(())
// TAR.GZ compression (placeholder - would use actual gzip in full implementation)
fn compress_tar_gz<P: AsRef<Path>>(src: P, dst: P) -> ArchiveResult<()> {
    // For now, just create TAR file
    // In full implementation, would pipe through gzip encoder
    warn!("TAR.GZ compression not fully implemented, falling back to TAR");
    compress_tar(src, dst)
// TAR.BZ2 compression (placeholder)
fn compress_tar_bz2<P: AsRef<Path>>(src: P, dst: P) -> ArchiveResult<()> {
    warn!("TAR.BZ2 compression not fully implemented, falling back to TAR");
    compress_tar(src, dst)
// TAR decompression
fn decompress_tar<P: AsRef<Path>>(src: P, dst: P) -> ArchiveResult<()> {
    let src_file = File::open(src)?;
    let mut tar = RatPack::new(src_file);
    
    extract_tar(&mut tar, dst.as_ref())?;
    Ok(())
// ZIP decompression
fn decompress_zip<P: AsRef<Path>>(src: P, dst: P) -> ArchiveResult<()> {
    let src_file = File::open(src)?;
    let file_size = src_file.metadata()?.len() as i64;
    let zip = HoardPack::new(src_file, file_size)?;
    
    extract_zip(&zip, dst.as_ref())?;
    Ok(())
// TAR.GZ decompression (placeholder)
fn decompress_tar_gz<P: AsRef<Path>>(src: P, dst: P) -> ArchiveResult<()> {
    warn!("TAR.GZ decompression not fully implemented, falling back to TAR");
    decompress_tar(src, dst)
// TAR.BZ2 decompression (placeholder)
fn decompress_tar_bz2<P: AsRef<Path>>(src: P, dst: P) -> ArchiveResult<()> {
    warn!("TAR.BZ2 decompression not fully implemented, falling back to TAR");
    decompress_tar(src, dst)
// TAR.XZ decompression (placeholder)
fn decompress_tar_xz<P: AsRef<Path>>(src: P, dst: P) -> ArchiveResult<()> {
    warn!("TAR.XZ decompression not fully implemented, falling back to TAR");
    decompress_tar(src, dst)
// Helper functions for adding files to archives
fn add_file_to_tar<W: Write>(tar: &mut RatStash<W>, file_path: &Path) -> ArchiveResult<()> {
    let file = File::open(file_path)?;
    let metadata = file.metadata()?;
    let file_name = file_path.file_name()
        .ok_or_else(|| io_error("Invalid file name"))?
        .to_string_lossy();
    
    let header = super::tar::FileInfoHeader(
        ""
    )?;
    
    tar.write_header(&header)?;
    
    let mut reader = BufReader::new(file);
    std::io::copy(&mut reader, tar)?;
    
    Ok(())
fn add_directory_to_tar<W: Write>(tar: &mut RatStash<W>, dir_path: &Path) -> ArchiveResult<()> {
    // Simplified directory handling - would recursively walk in full implementation
    for entry in std::fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            add_file_to_tar(tar, &path)?;
        }
    }
    
    Ok(())
fn add_file_to_zip<W: Write + std::io::Seek>(zip: &mut HoardStash<W>, file_path: &Path) -> ArchiveResult<()> {
    let mut file = File::open(file_path)?;
    let file_name = file_path.file_name()
        .ok_or_else(|| io_error("Invalid file name"))?
        .to_string_lossy();
    
    let mut writer = zip.create(&file_name)?;
    std::io::copy(&mut file, &mut writer)?;
    
    Ok(())
fn add_directory_to_zip<W: Write + std::io::Seek>(zip: &mut HoardStash<W>, dir_path: &Path) -> ArchiveResult<()> {
    // Simplified directory handling
    for entry in std::fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            add_file_to_zip(zip, &path)?;
        }
    }
    
    Ok(())
// Helper functions for extracting from archives
fn extract_tar<R: Read>(tar: &mut RatPack<R>, dst_dir: &Path) -> ArchiveResult<()> {
    std::fs::create_dir_all(dst_dir)?;
    
    while let Some(header) = tar.next()? {
        let file_path = dst_dir.join(&header.name);
        
        // Ensure parent directory exists
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent)?;
        // Extract file
        let mut output_file = File::create(&file_path)?;
        std::io::copy(tar, &mut output_file)?;
        
        debug!("Extracted: {}", file_path.display());
    Ok(())
fn extract_zip<R: Read + std::io::Seek>(zip: &HoardPack<R>, dst_dir: &Path) -> ArchiveResult<()> {
    std::fs::create_dir_all(dst_dir)?;
    
    for file in &zip.files {
        let file_path = dst_dir.join(&file.file_header.name);
        
        // Ensure parent directory exists
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent)?;
        // Extract file
        let mut reader = file.open()?;
        let mut output_file = File::create(&file_path)?;
        std::io::copy(&mut reader, &mut output_file)?;
        
        debug!("Extracted: {}", file_path.display());
    Ok(())
