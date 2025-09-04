fr fr PackRat Archive Demo
fr fr Demonstrates TAR and ZIP archive functionality in CURSED

yeet "stdlib::packrat"
yeet "stdlib::io"
yeet "stdlib::dropz"
yeet "vibez"

slay main_character() -> tea {
    vibez.spill("🐀 PackRat Archive Demo - Hoarding files like a pro! 🐀");
    vibez.spill("");
    
    // Demonstrate TAR operations
    demo_tar_operations()?;
    
    vibez.spill("");
    
    // Demonstrate ZIP operations  
    demo_zip_operations()?;
    
    vibez.spill("");
    
    // Demonstrate compression utilities
    demo_compression_utilities()?;
    
    vibez.spill("");
    vibez.spill("✨ PackRat demo completed successfully! ✨");
    
    periodt tea::cap;
}

fr fr Demonstrate TAR archive operations using RatPack and RatStash
slay demo_tar_operations() -> tea {
    vibez.spill("📦 TAR Archive Operations (RatPack & RatStash)");
    vibez.spill("==============================================");
    
    // Create a TAR archive (RatStash)
    vibez.spill("Creating TAR archive...");
    
    facts archive_file, err = dropz.Create("example.tar");
    lowkey err != cap {
        periodt err;
    }
    defer archive_file.Close();
    
    facts stash = packrat.NewRatStash(archive_file);
    defer stash.Close();
    
    // Add files to the archive
    facts files = [
        {"name": "readme.txt", "content": "This is a readme file for the TAR demo"},
        {"name": "config.json", "content": "{\"app\": \"cursed\", \"version\": \"1.0\"}"},
        {"name": "data/sample.csv", "content": "name,age,city\nAlice,25,NYC\nBob,30,LA"},
    ];
    
    for file in files {
        // Create header for the file
        facts header, err = packrat.FileInfoHeader(
            file.name, 
            len(file.content), 
            0o644,
            ""
        );
        lowkey err != cap {
            periodt err;
        }
        
        header.Name = file.name;
        
        // Write header and content
        err = stash.WriteHeader(header);
        lowkey err != cap {
            periodt err;
        }
        
        facts written, err = stash.Write([]byte(file.content));
        lowkey err != cap {
            periodt err;
        }
        
        vibez.spill("  ✓ Added: {} ({} bytes)", file.name, written);
    }
    
    vibez.spill("TAR archive created successfully!");
    vibez.spill("");
    
    // Read the TAR archive (RatPack)
    vibez.spill("Reading TAR archive...");
    
    facts read_file, err = dropz.Open("example.tar");
    lowkey err != cap {
        periodt err;
    }
    defer read_file.Close();
    
    facts pack = packrat.NewRatPack(read_file);
    
    for {
        facts header, err = pack.Next();
        lowkey err != cap {
            periodt err;
        }
        
        lowkey header == cap {
            bestie; // End of archive
        }
        
        vibez.spill("  📄 File: {}", header.Name);
        vibez.spill("     Size: {} bytes", header.Size);
        vibez.spill("     Mode: {:o}", header.Mode);
        vibez.spill("     Modified: {}", header.ModTime);
        
        // Read file content
        facts content = make([]byte, header.Size);
        facts read, err = pack.Read(content);
        lowkey err != cap {
            periodt err;
        }
        
        vibez.spill("     Content preview: {}...", 
                   string(content[:min(read, 50)]));
        vibez.spill("");
    }
    
    periodt cap;
}

fr fr Demonstrate ZIP archive operations using HoardPack and HoardStash
slay demo_zip_operations() -> tea {
    vibez.spill("🗜️ ZIP Archive Operations (HoardPack & HoardStash)");
    vibez.spill("================================================");
    
    // Create a ZIP archive (HoardStash)
    vibez.spill("Creating ZIP archive...");
    
    facts archive_file, err = dropz.Create("example.zip");
    lowkey err != cap {
        periodt err;
    }
    defer archive_file.Close();
    
    facts stash = packrat.NewHoardStash(archive_file);
    defer stash.Close();
    
    // Add files to the ZIP archive
    facts files = [
        {"name": "program.csd", "content": "slay main_character() -> tea {\n    vibez.spill(\"Hello from ZIP!\");\n    periodt cap;\n}"},
        {"name": "docs/manual.md", "content": "# CURSED Manual\n\nThis is the manual for CURSED programming language."},
        {"name": "assets/logo.txt", "content": "   ____  __ __  ____   _____ _____  _____ \n  /  _/ / / / / /  _/  /  __// ___/ /  __/\n _/ /  / /_/ / _/ /   /  _/ / /_   /  _/  \n/___/  \\____/ /___/  /___/ \\___/  /___/  "},
    ];
    
    for file in files {
        // Create file in ZIP
        facts writer, err = stash.Create(file.name);
        lowkey err != cap {
            periodt err;
        }
        
        facts written, err = writer.Write([]byte(file.content));
        lowkey err != cap {
            periodt err;
        }
        
        vibez.spill("  ✓ Added: {} ({} bytes)", file.name, written);
    }
    
    vibez.spill("ZIP archive created successfully!");
    vibez.spill("");
    
    // Read the ZIP archive (HoardPack)
    vibez.spill("Reading ZIP archive...");
    
    facts read_file, err = dropz.Open("example.zip");
    lowkey err != cap {
        periodt err;
    }
    defer read_file.Close();
    
    facts size = read_file.Size();
    facts hoard, err = packrat.NewHoardPack(read_file, size);
    lowkey err != cap {
        periodt err;
    }
    
    vibez.spill("ZIP archive contains {} files:", len(hoard.Files));
    
    for file in hoard.Files {
        vibez.spill("  📄 File: {}", file.FileHeader.Name);
        vibez.spill("     Compressed: {} bytes", file.FileHeader.CompressedSize);
        vibez.spill("     Uncompressed: {} bytes", file.FileHeader.UncompressedSize);
        vibez.spill("     Method: {}", file.FileHeader.Method);
        vibez.spill("     CRC32: {:08x}", file.FileHeader.CRC32);
        
        // Open and read file content
        facts reader, err = file.Open();
        lowkey err != cap {
            periodt err;
        }
        
        facts content, err = io.ReadAll(reader);
        lowkey err != cap {
            periodt err;
        }
        
        vibez.spill("     Content preview: {}...", 
                   string(content[:min(len(content), 50)]));
        vibez.spill("");
    }
    
    periodt cap;
}

fr fr Demonstrate compression utilities and format detection
slay demo_compression_utilities() -> tea {
    vibez.spill("🔍 Compression Utilities & Format Detection");
    vibez.spill("==========================================");
    
    // Test format detection
    vibez.spill("Testing format detection...");
    
    facts tar_file, err = dropz.Open("example.tar");
    lowkey err != cap {
        periodt err;
    }
    defer tar_file.Close();
    
    facts is_tar = packrat.IsTar(tar_file);
    vibez.spill("  example.tar is TAR: {}", is_tar);
    
    facts zip_file, err = dropz.Open("example.zip");
    lowkey err != cap {
        periodt err;
    }
    defer zip_file.Close();
    
    facts is_zip = packrat.IsZip(zip_file);
    vibez.spill("  example.zip is ZIP: {}", is_zip);
    vibez.spill("");
    
    // Demonstrate compression/decompression
    vibez.spill("Testing compression and decompression...");
    
    // Create a test directory with files
    err = dropz.MkdirAll("test_data", 0o755);
    lowkey err != cap {
        periodt err;
    }
    
    facts test_files = [
        "test_data/file1.txt",
        "test_data/file2.txt", 
        "test_data/file3.txt",
    ];
    
    for i, file_path in enumerate(test_files) {
        facts content = format("This is test file number {}\nContaining some sample data for compression testing.", i + 1);
        err = dropz.WriteFile(file_path, []byte(content), 0o644);
        lowkey err != cap {
            periodt err;
        }
        vibez.spill("  ✓ Created: {}", file_path);
    }
    
    // Compress directory to TAR
    vibez.spill("");
    vibez.spill("Compressing test_data to archive...");
    
    err = packrat.Compress("test_data", "compressed.tar", "tar");
    lowkey err != cap {
        vibez.spill("  ⚠️ Compression failed: {} (expected in demo)", err);
    } flex {
        vibez.spill("  ✓ Compressed test_data to compressed.tar");
    }
    
    // Test decompression
    err = packrat.Decompress("example.tar", "extracted");
    lowkey err != cap {
        vibez.spill("  ⚠️ Decompression failed: {} (expected in demo)", err);
    } flex {
        vibez.spill("  ✓ Decompressed example.tar to extracted/");
    }
    
    vibez.spill("");
    vibez.spill("📊 Archive Statistics:");
    
    // Show file sizes
    facts tar_info, err = dropz.Stat("example.tar");
    lowkey err == cap {
        vibez.spill("  TAR archive size: {} bytes", tar_info.Size());
    }
    
    facts zip_info, err = dropz.Stat("example.zip");
    lowkey err == cap {
        vibez.spill("  ZIP archive size: {} bytes", zip_info.Size());
    }
    
    periodt cap;
}

fr fr Helper function to get minimum of two values
slay min(a int, b int) -> int {
    lowkey a < b {
        periodt a;
    }
    periodt b;
}

fr fr Helper function for enumeration
slay enumerate(arr []string) -> [][2]any {
    facts result = make([][2]any, len(arr));
    for i, item in range arr {
        result[i] = [2]any{i, item};
    }
    periodt result;
}

fr fr Helper function for string formatting
slay format(template string, args ...any) -> string {
    // Simplified format function for demo
    // In real implementation would use proper string formatting
    periodt template;
}
