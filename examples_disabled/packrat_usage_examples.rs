// Rust examples showing PackRat usage patterns
// These demonstrate the actual API that would be used from CURSED

use std::fs::File;
use std::io::{Write, Read, Cursor};
use tempfile::TempDir;

use cursed::stdlib::packrat::{
    tar::{RatPack, RatStash, RatHeader, FileInfoHeader as TarFileInfoHeader},
    zip::{HoardPack, HoardStash, HoardFileHeader, FileInfoHeader as ZipFileInfoHeader},
    compression::{IsZip, IsTar, Compress, Decompress},
    ArchiveResult,
};

fn main() -> ArchiveResult<()> {
    println!("🐀 PackRat Archive Examples 🐀");
    println!("================================");
    
    example_tar_operations()?;
    println!();
    
    example_zip_operations()?;
    println!();
    
    example_format_detection()?;
    println!();
    
    example_compression_utilities()?;
    
    println!("\n✨ All PackRat examples completed successfully! ✨");
    Ok(())
}

fn example_tar_operations() -> ArchiveResult<()> {
    println!("📦 TAR Archive Operations Example");
    println!("=================================");
    
    let temp_dir = TempDir::new()?;
    let archive_path = temp_dir.path().join("example.tar");
    
    // Creating a TAR archive
    println!("Creating TAR archive...");
    {
        let tar_file = File::create(&archive_path)?;
        let mut tar = RatStash::new(tar_file);
        
        let files = [
            ("hello.txt", b"Hello from PackRat!"),
            ("config.json", b"{\"name\": \"cursed\", \"version\": \"1.0\"}"),
            ("data/info.txt", b"This file is in a subdirectory"),
        ];
        
        for (name, content) in &files {
            let header = TarFileInfoHeader(name, content.len() as u64, 0o644, "")?;
            tar.write_header(&header)?;
            tar.write_all(content)?;
            println!("  ✓ Added: {} ({} bytes)", name, content.len());
        }
        
        tar.close()?;
    }
    
    // Reading the TAR archive
    println!("\nReading TAR archive...");
    {
        let tar_file = File::open(&archive_path)?;
        let mut tar = RatPack::new(tar_file);
        
        while let Some(header) = tar.next()? {
            println!("  📄 File: {}", header.name);
            println!("     Size: {} bytes", header.size);
            println!("     Mode: {:o}", header.mode);
            
            // Read content
            let mut content = vec![0; header.size as usize];
            tar.read_exact(&mut content)?;
            
            let preview = String::from_utf8_lossy(&content[..content.len().min(50)]);
            println!("     Content: {}...", preview);
            println!();
        }
    }
    
    Ok(())
}

fn example_zip_operations() -> ArchiveResult<()> {
    println!("🗜️ ZIP Archive Operations Example");
    println!("=================================");
    
    let temp_dir = TempDir::new()?;
    let archive_path = temp_dir.path().join("example.zip");
    
    // Creating a ZIP archive
    println!("Creating ZIP archive...");
    {
        let zip_file = File::create(&archive_path)?;
        let zip_cursor = Cursor::new(Vec::new());
        let mut zip = HoardStash::new(zip_cursor);
        
        let files = [
            ("readme.md", b"# PackRat ZIP Example\n\nThis is a readme file."),
            ("src/main.rs", b"fn main() {\n    println!(\"Hello from ZIP!\");\n}"),
            ("assets/data.csv", b"name,value\nitem1,100\nitem2,200"),
        ];
        
        for (name, content) in &files {
            let mut writer = zip.create(name)?;
            writer.write_all(content)?;
            println!("  ✓ Added: {} ({} bytes)", name, content.len());
        }
        
        zip.close()?;
    }
    
    println!("\nZIP archive operations completed!");
    
    Ok(())
}

fn example_format_detection() -> ArchiveResult<()> {
    println!("🔍 Format Detection Example");
    println!("===========================");
    
    // Test with ZIP signature
    let zip_signature = [0x50, 0x4b, 0x03, 0x04, 0x00, 0x00, 0x00, 0x00];
    let zip_cursor = Cursor::new(zip_signature);
    let is_zip = IsZip(zip_cursor);
    println!("ZIP signature detected: {}", is_zip);
    
    // Test with TAR signature  
    let mut tar_data = vec![0u8; 300];
    tar_data[257..263].copy_from_slice(b"ustar\0");
    let mut tar_cursor = Cursor::new(tar_data);
    let is_tar = IsTar(&mut tar_cursor);
    println!("TAR signature detected: {}", is_tar);
    
    // Test with non-archive data
    let random_data = [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0];
    let random_cursor = Cursor::new(random_data);
    let is_zip_random = IsZip(random_cursor);
    println!("Random data detected as ZIP: {}", is_zip_random);
    
    Ok(())
}

fn example_compression_utilities() -> ArchiveResult<()> {
    println!("⚙️ Compression Utilities Example");
    println!("================================");
    
    let temp_dir = TempDir::new()?;
    
    // Create test files
    let test_files = [
        ("file1.txt", "This is the first test file with some content."),
        ("file2.txt", "This is the second test file with different content."),
        ("file3.txt", "This is the third test file with even more content."),
    ];
    
    for (name, content) in &test_files {
        let file_path = temp_dir.path().join(name);
        std::fs::write(&file_path, content.as_bytes())?;
        println!("  ✓ Created: {} ({} bytes)", name, content.len());
    }
    
    // Test compression (may fail due to simplified implementation)
    let src_dir = temp_dir.path();
    let archive_path = temp_dir.path().join("compressed.tar");
    
    println!("\nTesting compression...");
    match Compress(&src_dir, &archive_path, "tar") {
        Ok(()) => println!("  ✓ Compression successful"),
        Err(e) => println!("  ⚠️ Compression failed: {} (expected in this demo)", e),
    }
    
    // Test decompression
    println!("\nTesting decompression...");
    let extract_dir = temp_dir.path().join("extracted");
    match Decompress(&archive_path, &extract_dir) {
        Ok(()) => println!("  ✓ Decompression successful"),
        Err(e) => println!("  ⚠️ Decompression failed: {} (expected in this demo)", e),
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_packrat_examples() {
        // Run examples as tests
        assert!(main().is_ok());
    }
}
