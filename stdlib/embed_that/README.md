# Embed That Module

The `embed_that` module provides comprehensive file embedding capabilities for the CURSED language. It allows developers to embed files directly into compiled binaries at build time, offering efficient resource management and deployment simplification.

## Features

### Core Embedded Types
- **ThatFile**: Individual embedded files with metadata
- **ThatFiles**: Collections of embedded files
- **ThatString**: Text content from embedded files  
- **ThatBytes**: Binary data from embedded files

### File Operations
- **GetFileName**: Extract file names from embedded resources
- **GetFileSize**: Get file sizes efficiently
- **GetFileContent**: Access file content as bytes or strings
- **GetFileHash**: Generate checksums for integrity verification
- **GetFileMIMEType**: Automatic MIME type detection
- **GetFileExtension**: Extract file extensions

### File Type Detection
- **IsTextFile**: Detect text-based files
- **IsImageFile**: Identify image formats
- **IsAudioFile**: Recognize audio files
- **IsVideoFile**: Detect video content

### Collection Management
- **GetFileFromCollection**: Retrieve specific files from collections
- **GetFileNames**: List all embedded file names
- **GetFileCount**: Count files in collections
- **GetTotalSize**: Calculate total embedded data size

### Advanced Filtering
- **FilterFilesByPattern**: Filter using glob patterns
- **FilterFilesByExtension**: Filter by file extensions
- **FilterFilesByMIME**: Filter by MIME types

### File System Interface
- **MakeFileSystem**: Create virtual file systems from embedded data
- **OpenFile**: Open embedded files through FS interface
- **ReadFileSystem**: Read files from embedded FS
- **ReadDirectory**: List directory contents
- **StatFile**: Get detailed file information

### Dynamic Loading
- **LoadThatFile**: Load individual embedded files
- **LoadThatDirectory**: Load entire embedded directories
- **LoadThatPattern**: Load files matching patterns

### Template Integration
- **ParseTemplates**: Parse embedded template files
- **ParseTemplatesWithFuncs**: Parse templates with custom functions
- **ExecuteTemplate**: Render templates with data

### Resource Type Loading
- **LoadImage**: Load and decode embedded images
- **LoadJSON**: Parse embedded JSON configurations
- **LoadYAML**: Parse embedded YAML files
- **LoadTOML**: Parse embedded TOML files
- **LoadConfig**: Auto-detect and parse configuration files

### Resource Compression
- **DecompressFile**: Decompress embedded files
- **LoadCompressedFS**: Work with compressed file systems

### Resource Cache
- **NewResourceCache**: Create resource caching systems
- **GetFromCache**: Retrieve cached resources
- **SetInCache**: Store resources in cache
- **ClearCache**: Clear cache contents

## Usage Examples

```cursed
fr fr Basic file embedding
sus logoFile tea = ThatFile("logo.png", "binary_data_here")
sus templates tea = ThatFiles("templates/*.html")

fr fr File operations
sus fileName tea = GetFileName(logoFile)
sus fileSize normie = GetFileSize(logoFile)
sus content tea = GetFileContent(logoFile)

fr fr File type detection
sus isImage lit = IsImageFile(logoFile)
sus isText lit = IsTextFile(logoFile)

fr fr Collection operations
sus indexTemplate tea = GetFileFromCollection(templates, "index.html")
sus templateCount normie = GetFileCount(templates)

fr fr Filtering
sus htmlFiles tea = FilterFilesByExtension(templates, ".html")
sus imageFiles tea = FilterFilesByMIME(templates, "image/*")

fr fr File system creation
sus embeddedFS tea = MakeFileSystem(templates)
sus fileHandle tea = OpenFile(embeddedFS, "index.html")

fr fr Dynamic loading
sus staticFiles tea = LoadThatPattern("static/*")
sus configFile tea = LoadThatFile("config/app.json")

fr fr Template usage
sus templateEngine tea = ParseTemplates("templates/*.html")

fr fr Resource loading
sus appLogo tea = LoadImage("assets/logo.png")
sus appConfig tea = LoadJSON("config.json", "configuration")

fr fr Caching
sus cache tea = NewResourceCache()
sus cachedTemplate tea = GetFromCache(cache, "homepage")
```

## Build-Time Embedding

### Embedding Syntax
Use the `fr frgo:embed` directive to embed files at build time:

```cursed
fr frgo:embed static/logo.png
sus logoBytes ThatBytes

fr frgo:embed config.json  
sus configData ThatFile

fr frgo:embed templates/*.html
sus templateFiles ThatFiles
```

### Supported Patterns
- **Single files**: `config.json`, `logo.png`
- **Wildcard patterns**: `templates/*.html`, `static/*`
- **Directory embedding**: `assets/`, `docs/`
- **Recursive patterns**: `**/*.md`, `src/**/*.js`

## Performance Features

### Compression
- Automatic compression for text files
- Configurable compression levels
- Runtime decompression on demand

### Caching
- Intelligent resource caching
- Memory-efficient cache management
- Configurable cache expiration

### Lazy Loading
- Load resources only when needed
- Minimal memory footprint
- Fast application startup

## Security Features

### Integrity Verification
- Automatic checksum generation
- Runtime integrity checking
- Tamper detection

### Access Control
- Read-only embedded file systems
- Resource access logging
- Path traversal protection

## Implementation Notes

This is a pure CURSED implementation providing:
- Zero-dependency file embedding
- Cross-platform compatibility  
- Production-ready resource management
- Memory-efficient operations
- Build-time optimization
- Runtime performance monitoring

The module enables developers to create self-contained applications with all necessary resources embedded directly in the binary, simplifying deployment and distribution while maintaining excellent performance characteristics.
