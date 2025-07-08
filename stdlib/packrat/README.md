# PackRat Module

## Overview
PackRat provides access to file archiving and compression formats. It combines functionality similar to Go's archive/tar and archive/zip packages with a hoarding (pack rat) approach to file storage.

## Core Types

### Tar Format Support

#### `RatPack` (Tar Reader)
Equivalent to Go's tar.Reader for reading tar archives.
- **NewRatPack(data []normie) *RatPack** - Create new tar reader
- **Next() (*RatHeader, tea)** - Read next file header
- **Read(b []normie) (normie, tea)** - Read file data

#### `RatStash` (Tar Writer)
Equivalent to Go's tar.Writer for creating tar archives.
- **NewRatStash() *RatStash** - Create new tar writer
- **WriteHeader(hdr *RatHeader) tea** - Write file header
- **Write(b []normie) (normie, tea)** - Write file data
- **Flush() tea** - Flush buffered data
- **Close() tea** - Close archive

#### `RatHeader`
Represents a tar file header.
- **Name** - File name
- **Mode** - File permissions
- **Uid/Gid** - User/group IDs
- **Size** - File size in bytes
- **ModTime** - Modification time
- **Typeflag** - File type (regular, directory, etc.)
- **Linkname** - Link target for symbolic links
- **Uname/Gname** - User/group names
- **Format** - Tar format variant

### Zip Format Support

#### `HoardPack` (Zip Reader)
Equivalent to Go's zip.Reader for reading zip archives.
- **NewHoardPack(data []normie, size normie) (*HoardPack, tea)** - Create new zip reader
- **Files** - Array of files in the archive

#### `HoardStash` (Zip Writer)
Equivalent to Go's zip.Writer for creating zip archives.
- **NewHoardStash() *HoardStash** - Create new zip writer
- **Create(name tea) ([]normie, tea)** - Create file in archive
- **CreateHeader(fh *HoardFileHeader) ([]normie, tea)** - Create file with custom header
- **Close() tea** - Close archive

#### `HoardFile` and `HoardFileHeader`
Represent zip file entries and headers.
- **Open() ([]normie, tea)** - Open file for reading
- **DataOffset() (normie, tea)** - Get data offset in archive

### Format Constants

#### `Format`
Represents tar format variants:
- **FormatUnknown** - Unknown format
- **FormatLegacy** - Legacy tar format
- **FormatPOSIX** - POSIX tar format
- **FormatGNU** - GNU tar format
- **FormatOldVibe** - Custom format with Gen Z metadata

## Utility Functions

### Format Detection
- **IsZip(data []normie) lit** - Check if data is ZIP format
- **IsTar(data []normie) lit** - Check if data is TAR format
- **ValidateArchive(data []normie) (tea, tea)** - Detect archive format

### Compression Operations
- **Compress(src, dst, format tea) tea** - Compress file to archive
- **Decompress(src, dst tea) tea** - Decompress archive

### Header Creation
- **FileInfoHeader(name tea, size normie) (*RatHeader, tea)** - Create tar header
- **ZipFileInfoHeader(name tea, size normie) (*HoardFileHeader, tea)** - Create zip header

### Archive Information
- **GetArchiveInfo(data []normie) (ArchiveInfo, tea)** - Get archive metadata

## Usage Examples

### Creating Tar Archives
```cursed
yeet "packrat"

fr fr Create tar writer
sus writer := packrat.NewRatStash()

fr fr Create file header
sus header, err := packrat.FileInfoHeader("document.txt", 13)
if err == "" {
    vibez.spill("Header created for:", header.Name)
    
    fr fr Write header to archive
    sus writeErr := writer.WriteHeader(header)
    if writeErr == "" {
        fr fr Write file data
        sus data := []normie{72, 101, 108, 108, 111}  fr fr "Hello"
        sus bytesWritten, dataErr := writer.Write(data)
        if dataErr == "" {
            vibez.spill("Wrote", bytesWritten, "bytes")
        }
    }
}

fr fr Close the archive
sus closeErr := writer.Close()
if closeErr == "" {
    vibez.spill("Archive created successfully")
}
```

### Reading Tar Archives
```cursed
fr fr Create tar reader from data
sus reader := packrat.NewRatPack(archiveData)

fr fr Read files from archive
while based {
    sus header, err := reader.Next()
    if err != "" {
        break  fr fr End of archive
    }
    
    vibez.spill("File:", header.Name)
    vibez.spill("Size:", header.Size)
    vibez.spill("Mode:", header.Mode)
    
    fr fr Read file data
    sus buffer := make([]normie, header.Size)
    sus bytesRead, readErr := reader.Read(buffer)
    if readErr == "" {
        vibez.spill("Read", bytesRead, "bytes")
    }
}
```

### Creating Zip Archives
```cursed
fr fr Create zip writer
sus zipWriter := packrat.NewHoardStash()

fr fr Add files to zip
sus fileData, createErr := zipWriter.Create("readme.txt")
if createErr == "" {
    vibez.spill("Created file in zip")
}

fr fr Add file with custom header
sus header, headerErr := packrat.ZipFileInfoHeader("data.bin", 1024)
if headerErr == "" {
    sus customData, customErr := zipWriter.CreateHeader(header)
    if customErr == "" {
        vibez.spill("Added custom file to zip")
    }
}

fr fr Finalize zip
sus zipCloseErr := zipWriter.Close()
if zipCloseErr == "" {
    vibez.spill("Zip archive completed")
}
```

### Reading Zip Archives
```cursed
fr fr Create zip reader
sus zipReader, err := packrat.NewHoardPack(zipData, len(zipData))
if err == "" {
    vibez.spill("Zip archive contains", len(zipReader.Files), "files")
    
    fr fr Process each file
    for i := 0; i < len(zipReader.Files); i++ {
        sus file := zipReader.Files[i]
        vibez.spill("File:", file.FileHeader.Name)
        vibez.spill("Compressed size:", file.FileHeader.CompressedSize)
        vibez.spill("Uncompressed size:", file.FileHeader.UncompressedSize)
        
        fr fr Open and read file
        sus content, openErr := file.Open()
        if openErr == "" {
            vibez.spill("File opened successfully")
        }
        
        fr fr Get data offset
        sus offset, offsetErr := file.DataOffset()
        if offsetErr == "" {
            vibez.spill("Data offset:", offset)
        }
    }
}
```

### Format Detection
```cursed
fr fr Detect archive format
sus format, err := packrat.ValidateArchive(unknownData)
if err == "" {
    vibez.spill("Archive format:", format)
    
    switch format {
    case "zip":
        vibez.spill("Processing ZIP archive")
    case "tar":
        vibez.spill("Processing TAR archive")
    default:
        vibez.spill("Unknown format")
    }
}

fr fr Check specific formats
if packrat.IsZip(data) {
    vibez.spill("This is a ZIP file")
}

if packrat.IsTar(data) {
    vibez.spill("This is a TAR file")
}
```

### Archive Information
```cursed
fr fr Get archive metadata
sus info, err := packrat.GetArchiveInfo(archiveData)
if err == "" {
    vibez.spill("Format:", info.Format)
    vibez.spill("File count:", info.FileCount)
    vibez.spill("Total size:", info.TotalSize)
    vibez.spill("Compressed:", info.Compressed)
}
```

### Compression Operations
```cursed
fr fr Compress files
sus compressErr := packrat.Compress("source.txt", "archive.tar", "tar")
if compressErr == "" {
    vibez.spill("File compressed to TAR")
}

sus zipErr := packrat.Compress("data/", "backup.zip", "zip")
if zipErr == "" {
    vibez.spill("Directory compressed to ZIP")
}

fr fr Decompress archives
sus decompressErr := packrat.Decompress("backup.zip", "restored/")
if decompressErr == "" {
    vibez.spill("Archive decompressed")
}
```

### Header Customization
```cursed
fr fr Create custom tar header
sus customHeader := &packrat.RatHeader{
    Name: "custom/path/file.txt",
    Mode: 755,
    Uid: 1000,
    Gid: 1000,
    Size: 2048,
    Typeflag: 0,  fr fr Regular file
    Format: packrat.FormatPOSIX
}

fr fr Create custom zip header
sus zipHeader := &packrat.HoardFileHeader{
    Name: "compressed/file.dat",
    Method: 8,  fr fr Deflate compression
    UncompressedSize: 4096,
    CompressedSize: 2048,
    CreatorVersion: 20,
    ReaderVersion: 20
}
```

## Implementation Features

1. **Pure CURSED Implementation** - No FFI dependencies
2. **Dual Format Support** - Both TAR and ZIP formats
3. **Magic Byte Detection** - Automatic format recognition
4. **Header Preservation** - Complete metadata support
5. **Streaming Operations** - Efficient memory usage
6. **Error Handling** - Comprehensive error reporting
7. **Format Validation** - Security against malformed archives

## Magic Bytes

### ZIP Format
- **Signature**: `PK\003\004` (bytes: 80, 75, 3, 4)
- **Detection**: Checks first 4 bytes of file

### TAR Format  
- **Magic**: `ustar\000` at offset 257
- **Detection**: Checks bytes 257-262 in header block

## Error Handling

All functions return error messages as strings:
- Empty string ("") indicates success
- Non-empty string contains error description

Common errors:
- "EOF" - End of archive reached
- "no current header" - Reading without valid header
- "header cannot be nil" - Invalid header parameter
- "unsupported format" - Unknown archive format

## Implementation Notes

This is a pure CURSED implementation that provides essential archive handling functionality without external dependencies. The implementation includes:

- Complete TAR and ZIP format support
- Magic byte detection for format identification
- Header serialization and parsing
- Streaming read/write operations
- Archive validation and metadata extraction
- Compression format abstraction

The module focuses on:
- Practical archive manipulation
- Format compatibility
- Memory efficiency
- Error resilience
- Educational value for understanding archive formats

### Limitations

- Simplified compression algorithms (for demonstration)
- Basic timestamp handling
- Limited compression method support
- Simplified directory handling

For production use, this module provides a solid foundation for archive operations while maintaining format compatibility and demonstrating key archiving concepts.
