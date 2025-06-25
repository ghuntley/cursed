// Integration tests for PackRat archive/compression package

use std::io::{Cursor, Write};
use tempfile::TempDir;
use tracing::info;

use cursed::stdlib::packrat::{
    tar::{RatPack, RatStash, RatHeader, Format, FileInfoHeader as TarFileInfoHeader},
    zip::{HoardPack, HoardStash, HoardFile, HoardFileHeader, FileInfoHeader as ZipFileInfoHeader},
    compression::{IsZip, IsTar, Compress, Decompress, detect_format, ArchiveFormat},
    error::{ArchiveError, ArchiveResult},
};

#[path = "common.rs"]
mod common;

#[test]
fn test_rat_header_creation_and_validation() {
    init_tracing!();
    info!("Testing RatHeader creation and validation");
    
    // Test basic header creation
    let header = RatHeader::from_file_info("test.txt", 100, 0o644);
    assert_eq!(header.name, "test.txt");
    assert_eq!(header.size, 100);
    assert_eq!(header.mode, 0o644);
    assert_eq!(header.format, Format::FormatPOSIX);
    
    // Test header validation - valid path
    assert!(header.validate().is_ok());
    
    // Test path traversal detection
    let mut bad_header = header.clone();
    bad_header.name = "../etc/passwd".to_string();
    assert!(bad_header.validate().is_err());
    
    // Test absolute path detection
    bad_header.name = "/etc/passwd".to_string();
    assert!(bad_header.validate().is_err());
    
    // Test name too long
    bad_header.name = "a".repeat(300);
    assert!(bad_header.validate().is_err());
    
    info!("RatHeader creation and validation tests passed");
}

#[test]
fn test_rat_header_serialization() {
    init_tracing!();
    info!("Testing RatHeader serialization and deserialization");
    
    let original = RatHeader::from_file_info("example.txt", 1234, 0o755);
    
    // Serialize to bytes
    let bytes = original.to_bytes().unwrap();
    assert_eq!(bytes.len(), 512); // TAR block size
    
    // Deserialize back
    let deserialized = RatHeader::from_bytes(&bytes).unwrap();
    assert_eq!(deserialized.name, original.name);
    assert_eq!(deserialized.size, original.size);
    assert_eq!(deserialized.mode, original.mode);
    
    info!("RatHeader serialization tests passed");
}

#[test]
fn test_rat_pack_tar_reading() {
    init_tracing!();
    info!("Testing RatPack TAR reading functionality");
    
    // Create a minimal TAR archive in memory
    let mut tar_data = Vec::new();
    let mut tar = RatStash::new(&mut tar_data);
    
    // Add a test file
    let header = RatHeader::from_file_info("hello.txt", 5, 0o644);
    tar.write_header(&header).unwrap();
    tar.write_all(b"hello").unwrap();
    tar.close().unwrap();
    
    // Read it back
    let cursor = Cursor::new(tar_data);
    let mut reader = RatPack::new(cursor);
    
    let read_header = reader.next().unwrap().unwrap();
    assert_eq!(read_header.name, "hello.txt");
    assert_eq!(read_header.size, 5);
    
    let mut content = String::new();
    std::io::Read::read_to_string(&mut reader, &mut content).unwrap();
    assert_eq!(content, "hello");
    
    // Should be no more files
    assert!(reader.next().unwrap().is_none());
    
    info!("RatPack TAR reading tests passed");
}

#[test]
fn test_rat_stash_tar_writing() {
    init_tracing!();
    info!("Testing RatStash TAR writing functionality");
    
    let mut tar_data = Vec::new();
    let mut tar = RatStash::new(&mut tar_data);
    
    // Write multiple files
    let files = [
        ("file1.txt", b"First file content"),
        ("file2.txt", b"Second file content"),
        ("subdir/file3.txt", b"Third file in subdirectory"),
    ];
    
    for (name, content) in &files {
        let header = RatHeader::from_file_info(name, content.len() as u64, 0o644);
        tar.write_header(&header).unwrap();
        tar.write_all(content).unwrap();
    }
    
    tar.close().unwrap();
    
    // Verify archive is properly formed
    assert!(!tar_data.is_empty());
    
    // Should end with two zero blocks (1024 bytes of zeros)
    let end = &tar_data[tar_data.len() - 1024..];
    assert!(end.iter().all(|&b| b == 0));
    
    info!("RatStash TAR writing tests passed");
}

#[test]
fn test_hoard_file_header_creation_and_validation() {
    init_tracing!();
    info!("Testing HoardFileHeader creation and validation");
    
    // Test basic header creation
    let header = HoardFileHeader::new("test.zip");
    assert_eq!(header.name, "test.zip");
    assert_eq!(header.method, 0); // COMPRESSION_STORED
    
    // Test header validation - valid path
    assert!(header.validate().is_ok());
    
    // Test path traversal detection
    let mut bad_header = header.clone();
    bad_header.name = "../etc/passwd".to_string();
    assert!(bad_header.validate().is_err());
    
    // Test absolute path detection
    bad_header.name = "/etc/passwd".to_string();
    assert!(bad_header.validate().is_err());
    
    info!("HoardFileHeader creation and validation tests passed");
}

#[test]
fn test_hoard_stash_zip_writing() {
    init_tracing!();
    info!("Testing HoardStash ZIP writing functionality");
    
    let zip_data = Cursor::new(Vec::new());
    let mut zip = HoardStash::new(zip_data);
    
    // Add test files
    let files = [
        ("readme.txt", b"This is a readme file"),
        ("data.csv", b"col1,col2,col3\n1,2,3\n4,5,6"),
        ("config.json", b"{\"name\": \"test\", \"version\": \"1.0\"}"),
    ];
    
    for (name, content) in &files {
        let mut writer = zip.create(name).unwrap();
        writer.write_all(content).unwrap();
    }
    
    zip.close().unwrap();
    
    info!("HoardStash ZIP writing tests passed");
}

#[test]
fn test_hoard_file_operations() {
    init_tracing!();
    info!("Testing HoardFile operations");
    
    let header = HoardFileHeader::new("test.txt");
    let content = b"Hello, HoardFile!".to_vec();
    let file = HoardFile::new(header, content.clone());
    
    assert_eq!(file.file_header.name, "test.txt");
    
    // Test opening file for reading
    let mut reader = file.open().unwrap();
    let mut read_content = Vec::new();
    std::io::Read::read_to_end(&mut reader, &mut read_content).unwrap();
    assert_eq!(read_content, content);
    
    // Test data offset
    let offset = file.data_offset().unwrap();
    assert_eq!(offset, 0); // Default offset
    
    info!("HoardFile operations tests passed");
}

#[test]
fn test_format_detection() {
    init_tracing!();
    info!("Testing archive format detection");
    
    // Test ZIP signature detection
    let zip_data = [0x50, 0x4b, 0x03, 0x04, 0x00, 0x00, 0x00, 0x00];
    let cursor = Cursor::new(zip_data);
    assert!(IsZip(cursor));
    
    // Test non-ZIP data
    let other_data = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
    let cursor = Cursor::new(other_data);
    assert!(!IsZip(cursor));
    
    // Test TAR signature detection
    let mut tar_data = vec![0u8; 300];
    tar_data[257..263].copy_from_slice(b"ustar\0");
    let mut cursor = Cursor::new(tar_data);
    assert!(IsTar(&mut cursor));
    
    // Test format detection from reader
    let zip_data = [0x50, 0x4b, 0x03, 0x04, 0x00, 0x00, 0x00, 0x00];
    let mut cursor = Cursor::new(zip_data);
    let format = detect_format(&mut cursor).unwrap();
    assert_eq!(format, ArchiveFormat::Zip);
    
    info!("Format detection tests passed");
}

#[test]
fn test_tar_format_variants() {
    init_tracing!();
    info!("Testing TAR format variants");
    
    // Test format enumeration
    assert_eq!(Format::default(), Format::FormatPOSIX);
    assert_ne!(Format::FormatGNU, Format::FormatLegacy);
    
    // Test format assignment
    let mut header = RatHeader::default();
    header.format = Format::FormatGNU;
    assert_eq!(header.format, Format::FormatGNU);
    
    header.format = Format::FormatOldVibe;
    assert_eq!(header.format, Format::FormatOldVibe);
    
    info!("TAR format variants tests passed");
}

#[test]
fn test_file_info_header_functions() {
    init_tracing!();
    info!("Testing FileInfoHeader utility functions");
    
    // Test TAR FileInfoHeader
    let tar_header = TarFileInfoHeader("example.txt", 1000, 0o755, "").unwrap();
    assert_eq!(tar_header.name, "example.txt");
    assert_eq!(tar_header.size, 1000);
    assert_eq!(tar_header.mode, 0o755);
    
    // Test ZIP FileInfoHeader
    let zip_header = ZipFileInfoHeader("example.zip", 2000, 0o644).unwrap();
    assert_eq!(zip_header.name, "example.zip");
    assert_eq!(zip_header.uncompressed_size, 2000);
    
    info!("FileInfoHeader utility function tests passed");
}

#[test]
fn test_error_handling() {
    init_tracing!();
    info!("Testing PackRat error handling");
    
    // Test invalid header parsing
    let invalid_header = vec![0u8; 100]; // Too short for TAR header
    let result = RatHeader::from_bytes(&invalid_header);
    assert!(result.is_err());
    
    // Test path validation errors
    let mut header = RatHeader::default();
    header.name = "../../../etc/passwd".to_string();
    let result = header.validate();
    assert!(result.is_err());
    
    // Test compression with unsupported format
    let temp_dir = TempDir::new().unwrap();
    let src = temp_dir.path().join("test.txt");
    let dst = temp_dir.path().join("test.unknown");
    std::fs::write(&src, b"test content").unwrap();
    
    let result = Compress(&src, &dst, "unknown_format");
    assert!(result.is_err());
    
    info!("Error handling tests passed");
}

#[test]
fn test_archive_round_trip() {
    init_tracing!();
    info!("Testing archive round-trip operations");
    
    let temp_dir = TempDir::new().unwrap();
    
    // Create test files
    let test_files = [
        ("file1.txt", b"First test file"),
        ("file2.txt", b"Second test file"),
        ("subdir/file3.txt", b"File in subdirectory"),
    ];
    
    for (name, content) in &test_files {
        let file_path = temp_dir.path().join(name);
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(&file_path, content).unwrap();
    }
    
    // Test TAR round-trip
    let tar_path = temp_dir.path().join("test.tar");
    let extract_dir = temp_dir.path().join("extracted_tar");
    
    // Create TAR archive
    let tar_file = std::fs::File::create(&tar_path).unwrap();
    let mut tar = RatStash::new(tar_file);
    
    for (name, content) in &test_files {
        let header = RatHeader::from_file_info(name, content.len() as u64, 0o644);
        tar.write_header(&header).unwrap();
        tar.write_all(content).unwrap();
    }
    tar.close().unwrap();
    
    // Read TAR archive
    let tar_file = std::fs::File::open(&tar_path).unwrap();
    let mut tar_reader = RatPack::new(tar_file);
    
    std::fs::create_dir_all(&extract_dir).unwrap();
    
    while let Some(header) = tar_reader.next().unwrap() {
        let file_path = extract_dir.join(&header.name);
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        
        let mut file = std::fs::File::create(&file_path).unwrap();
        std::io::copy(&mut tar_reader, &mut file).unwrap();
    }
    
    // Verify extracted files
    for (name, expected_content) in &test_files {
        let extracted_path = extract_dir.join(name);
        let actual_content = std::fs::read(&extracted_path).unwrap();
        assert_eq!(&actual_content, expected_content);
    }
    
    info!("Archive round-trip tests passed");
}

#[test]
fn test_large_file_handling() {
    init_tracing!();
    info!("Testing large file handling");
    
    // Create a moderately large file (1MB)
    let large_content = vec![b'A'; 1024 * 1024];
    
    // Test TAR with large file
    let mut tar_data = Vec::new();
    let mut tar = RatStash::new(&mut tar_data);
    
    let header = RatHeader::from_file_info("large_file.txt", large_content.len() as u64, 0o644);
    tar.write_header(&header).unwrap();
    tar.write_all(&large_content).unwrap();
    tar.close().unwrap();
    
    // Read it back
    let cursor = Cursor::new(tar_data);
    let mut reader = RatPack::new(cursor);
    
    let read_header = reader.next().unwrap().unwrap();
    assert_eq!(read_header.name, "large_file.txt");
    assert_eq!(read_header.size, large_content.len() as i64);
    
    let mut read_content = Vec::new();
    std::io::Read::read_to_end(&mut reader, &mut read_content).unwrap();
    assert_eq!(read_content.len(), large_content.len());
    
    info!("Large file handling tests passed");
}

#[test]
fn test_empty_archive_handling() {
    init_tracing!();
    info!("Testing empty archive handling");
    
    // Test empty TAR archive
    let mut tar_data = Vec::new();
    let mut tar = RatStash::new(&mut tar_data);
    tar.close().unwrap();
    
    let cursor = Cursor::new(tar_data);
    let mut reader = RatPack::new(cursor);
    
    // Should immediately return None for empty archive
    assert!(reader.next().unwrap().is_none());
    
    // Test empty ZIP archive
    let zip_data = Cursor::new(Vec::new());
    let mut zip = HoardStash::new(zip_data);
    zip.close().unwrap();
    
    info!("Empty archive handling tests passed");
}

#[test]
fn test_archive_metadata_preservation() {
    init_tracing!();
    info!("Testing archive metadata preservation");
    
    use std::time::{SystemTime, Duration};
    
    // Create header with specific metadata
    let mut header = RatHeader::from_file_info("metadata_test.txt", 100, 0o755);
    header.uid = 1000;
    header.gid = 1000;
    header.uname = "testuser".to_string();
    header.gname = "testgroup".to_string();
    header.mod_time = SystemTime::UNIX_EPOCH + Duration::from_secs(1234567890);
    
    // Serialize and deserialize
    let bytes = header.to_bytes().unwrap();
    let restored = RatHeader::from_bytes(&bytes).unwrap();
    
    // Verify metadata is preserved
    assert_eq!(restored.name, header.name);
    assert_eq!(restored.mode, header.mode);
    assert_eq!(restored.uid, header.uid);
    assert_eq!(restored.gid, header.gid);
    assert_eq!(restored.uname, header.uname);
    assert_eq!(restored.gname, header.gname);
    
    // Time comparison (within 1 second due to precision)
    let time_diff = header.mod_time.duration_since(restored.mod_time)
        .unwrap_or_else(|_| restored.mod_time.duration_since(header.mod_time).unwrap());
    assert!(time_diff.as_secs() <= 1);
    
    info!("Archive metadata preservation tests passed");
}

#[test]
fn test_concurrent_archive_operations() {
    init_tracing!();
    info!("Testing concurrent archive operations");
    
    use std::sync::{Arc, Mutex};
    use std::thread;
    
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    
    // Create multiple archives concurrently
    for i in 0..4 {
        let results = Arc::clone(&results);
        let handle = thread::spawn(move || {
            let mut tar_data = Vec::new();
            let mut tar = RatStash::new(&mut tar_data);
            
            let file_name = format!("concurrent_file_{}.txt", i);
            let content = format!("Content from thread {}", i);
            
            let header = RatHeader::from_file_info(&file_name, content.len() as u64, 0o644);
            tar.write_header(&header).unwrap();
            tar.write_all(content.as_bytes()).unwrap();
            tar.close().unwrap();
            
            results.lock().unwrap().push((i, tar_data.len()));
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    let results = results.lock().unwrap();
    assert_eq!(results.len(), 4);
    
    // Verify all archives were created successfully
    for (thread_id, size) in results.iter() {
        assert!(*size > 0, "Thread {} created empty archive", thread_id);
    }
    
    info!("Concurrent archive operations tests passed");
}
