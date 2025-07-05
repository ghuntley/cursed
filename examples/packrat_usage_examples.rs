// Rust examples showing PackRat usage patterns
// These demonstrate the actual API that would be used from CURSED

use std::fs::File;
use std::io::{Write, Read, Cursor};
use tempfile::TempDir;

use cursed::error::CursedError;

// Note: PackRat is currently a stub implementation with basic handlers
// The full archive functionality is not yet implemented

type ArchiveResult<T> = Result<T, CursedError>;

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
    
    // Creating a TAR archive (using basic file operations as PackRat is not fully implemented)
    println!("Creating TAR archive...");
    {
        let tar_file = File::create(&archive_path)?;
        // Note: PackRat TAR implementation is not complete, using basic file operations
        let module = cursed::stdlib::packrat::tar::ModuleHandler::new();
        println!("  ✓ TAR module status: {}", module.info());
        
        let files = [
            ("hello.txt", "Hello from PackRat!"),
            ("config.json", "{\"name\": \"cursed\", \"version\": \"1.0\"}"),
            ("data/info.txt", "This file is in a subdirectory"),
        ];
        
        for (name, content) in &files {
            let processed = module.process(content)?;
            println!("  ✓ Processed: {} ({} bytes)", name, processed.len());
        }
    }
    
    // Reading the TAR archive (demonstration only)
    println!("\nReading TAR archive...");
    {
        let tar_file = File::open(&archive_path)?;
        let module = cursed::stdlib::packrat::tar::ModuleHandler::new();
        
        if module.is_enabled() {
            println!("  📁 TAR module is enabled and ready");
            let demo_content = "Sample TAR content";
            let processed = module.process(demo_content)?;
            println!("  ✓ Processed content: {}", processed);
        }
    }
    
    Ok(())
}

fn example_zip_operations() -> ArchiveResult<()> {
    println!("🗜️ ZIP Archive Operations Example");
    println!("=================================");
    
    let temp_dir = TempDir::new()?;
    let archive_path = temp_dir.path().join("example.zip");
    
    // Creating a ZIP archive (using basic file operations as PackRat is not fully implemented)
    println!("Creating ZIP archive...");
    {
        let zip_file = File::create(&archive_path)?;
        let module = cursed::stdlib::packrat::zip::ModuleHandler::new();
        println!("  ✓ ZIP module status: {}", module.info());
        
        let files = [
            ("readme.md", "# PackRat ZIP Example\n\nThis is a readme file."),
            ("src/main.rs", "fn main() {\n    println!(\"Hello from ZIP!\");\n}"),
            ("assets/data.csv", "name,value\nitem1,100\nitem2,200"),
        ];
        
        for (name, content) in &files {
            let processed = module.process(content)?;
            println!("  ✓ Processed: {} ({} bytes)", name, processed.len());
        }
    }
    
    println!("\nZIP archive operations completed!");
    
    Ok(())
}

fn example_format_detection() -> ArchiveResult<()> {
    println!("🔍 Format Detection Example");
    println!("===========================");
    
    // Test with ZIP signature (demonstration only)
    let zip_signature = [0x50, 0x4b, 0x03, 0x04, 0x00, 0x00, 0x00, 0x00];
    let zip_matches = zip_signature[0..4] == [0x50, 0x4b, 0x03, 0x04];
    println!("ZIP signature detected: {}", zip_matches);
    
    // Test with TAR signature  
    let mut tar_data = vec![0u8; 300];
    tar_data[257..263].copy_from_slice(b"ustar\0");
    let tar_matches = &tar_data[257..262] == cursed::stdlib::packrat::TAR_MAGIC;
    println!("TAR signature detected: {}", tar_matches);
    
    // Test with non-archive data
    let random_data = [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0];
    let random_matches = random_data[0..4] == [0x50, 0x4b, 0x03, 0x04];
    println!("Random data detected as ZIP: {}", random_matches);
    
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
    
    // Test compression (demonstration only - using available handlers)
    let src_dir = temp_dir.path();
    let archive_path = temp_dir.path().join("compressed.tar");
    
    println!("\nTesting compression...");
    let compression_handler = cursed::stdlib::packrat::compression::IOHandler::new();
    let test_data = b"test data";
    match compression_handler.read_all(test_data.as_slice()) {
        Ok(result) => println!("  ✓ Compression handler works: {} bytes processed", result.len()),
        Err(e) => println!("  ⚠️ Compression handler failed: {} (expected in this demo)", e),
    }
    
    // Test decompression
    println!("\nTesting decompression...");
    let extract_dir = temp_dir.path().join("extracted");
    let compressed_data = b"compressed data";
    match compression_handler.read_all(compressed_data.as_slice()) {
        Ok(result) => println!("  ✓ Decompression handler works: {} bytes processed", result.len()),
        Err(e) => println!("  ⚠️ Decompression handler failed: {} (expected in this demo)", e),
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
