# MIMEZ Module - MIME Type Detection and Content-Type Handling

A comprehensive MIME type detection package for the CURSED standard library, implemented in 100% pure CURSED language.

## Overview

The `mimez` module provides robust MIME type detection capabilities including:

- **File Extension Detection**: Identify MIME types from file extensions
- **Content-Based Detection**: Analyze file headers and content patterns
- **Content-Type Headers**: Parse and format HTTP Content-Type headers
- **Binary Signature Recognition**: Detect file types from magic bytes
- **Comprehensive MIME Database**: Support for 50+ common file types

## Features

### Core Functions

- `detect_mime_from_extension(filename tea) tea` - Detect MIME type from file extension
- `detect_mime_from_content(content []drip) tea` - Analyze content bytes for MIME type
- `detect_mime_comprehensive(filename tea, content []drip) tea` - Combined detection approach
- `get_content_type_for_file(filename tea) tea` - Get complete Content-Type header value

### Content-Type Header Support

- `parse_content_type(header_value tea) ContentTypeHeader` - Parse Content-Type headers
- `format_content_type(mime_type tea, charset tea, boundary tea) tea` - Format headers
- Content-Type parameter extraction (charset, boundary, encoding)

### Binary Signature Detection

- JPEG (`image/jpeg`) - FFD8FF magic bytes
- PNG (`image/png`) - 89504E47 magic bytes  
- GIF (`image/gif`) - GIF87a/GIF89a signatures
- PDF (`application/pdf`) - %PDF signature
- ZIP (`application/zip`) - PK magic bytes

### Utility Functions

- `get_mime_description(mime_type tea) tea` - Human-readable descriptions
- `get_extension_for_mime(mime_type tea) tea` - Reverse extension lookup
- `is_binary_mime(mime_type tea) lit` - Check if MIME type is binary
- `list_supported_extensions() []tea` - Get all supported extensions

## Supported File Types

### Text Files
- `txt` → `text/plain`
- `html`, `htm` → `text/html`
- `css` → `text/css`
- `js` → `application/javascript`
- `json` → `application/json`
- `xml` → `application/xml`
- `csv` → `text/csv`
- `md` → `text/markdown`
- `yaml`, `yml` → `application/x-yaml`
- `toml` → `application/toml`

### Images
- `jpg`, `jpeg` → `image/jpeg`
- `png` → `image/png`
- `gif` → `image/gif`
- `webp` → `image/webp`
- `svg` → `image/svg+xml`
- `ico` → `image/x-icon`
- `bmp` → `image/bmp`

### Audio
- `mp3` → `audio/mpeg`
- `wav` → `audio/wav`
- `ogg` → `audio/ogg`
- `m4a` → `audio/mp4`
- `flac` → `audio/flac`

### Video
- `mp4` → `video/mp4`
- `avi` → `video/x-msvideo`
- `mov` → `video/quicktime`
- `wmv` → `video/x-ms-wmv`
- `webm` → `video/webm`

### Documents
- `pdf` → `application/pdf`
- `doc` → `application/msword`
- `docx` → `application/vnd.openxmlformats-officedocument.wordprocessingml.document`
- `xls` → `application/vnd.ms-excel`
- `xlsx` → `application/vnd.openxmlformats-officedocument.spreadsheetml.sheet`

### Programming Languages
- `c` → `text/x-c`
- `cpp` → `text/x-c++`
- `py` → `text/x-python`
- `rs` → `text/x-rust`
- `go` → `text/x-go`
- `java` → `text/x-java`
- `csd` → `text/x-cursed`

### Archives
- `zip` → `application/zip`
- `tar` → `application/x-tar`
- `gz` → `application/gzip`
- `7z` → `application/x-7z-compressed`

## Usage Examples

### Basic MIME Detection

```cursed
yeet "mimez"

# Extension-based detection
sus mime1 tea = detect_mime_from_extension("document.pdf")
vibez.spill("PDF MIME type:", mime1)  # "application/pdf"

# Content-based detection
sus file_content []drip = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]
sus mime2 tea = detect_mime_from_content(file_content)
vibez.spill("PNG detected:", mime2)  # "image/png"

# Comprehensive detection
sus mime3 tea = detect_mime_comprehensive("image.png", file_content)
vibez.spill("Comprehensive:", mime3)  # "image/png"
```

### Content-Type Headers

```cursed
yeet "mimez"

# Parse Content-Type header
sus header tea = "text/html; charset=utf-8; boundary=something"
sus parsed ContentTypeHeader = parse_content_type(header)
vibez.spill("Media type:", parsed.media_type)  # "text/html"
vibez.spill("Charset:", parsed.charset)        # "utf-8"
vibez.spill("Boundary:", parsed.boundary)      # "something"

# Format Content-Type header
sus formatted tea = format_content_type("application/json", "utf-8", "")
vibez.spill("Formatted:", formatted)  # "application/json; charset=utf-8"

# Get Content-Type for file
sus content_type tea = get_content_type_for_file("data.json")
vibez.spill("JSON Content-Type:", content_type)  # "application/json; charset=utf-8"
```

### Binary Detection

```cursed
yeet "mimez"

# Check if MIME type is binary
sus is_bin lit = is_binary_mime("image/jpeg")
vibez.spill("JPEG is binary:", is_bin)  # true

sus is_text lit = is_binary_mime("text/plain")
vibez.spill("Text is binary:", is_text)  # false

# Get description
sus desc tea = get_mime_description("video/mp4")
vibez.spill("MP4 description:", desc)  # "MP4 video"
```

### Utility Functions

```cursed
yeet "mimez"

# Get extension for MIME type
sus ext tea = get_extension_for_mime("image/jpeg")
vibez.spill("JPEG extension:", ext)  # "jpg"

# Check if extension is supported
sus supported lit = is_supported_extension("webp")
vibez.spill("WebP supported:", supported)  # true

# List all supported extensions
sus extensions []tea = list_supported_extensions()
vibez.spill("Total extensions:", array_len(extensions))  # 50+
```

## Error Handling

The module provides error handling through:

```cursed
yeet "mimez"

# Check for errors after operations
sus mime tea = detect_mime_from_extension("")  # Empty filename
sus error tea = get_last_error()
ready error != "" {
    vibez.spill("Error occurred:", error)
    clear_error()
}
```

## Data Structures

### MimeEntry
```cursed
squad MimeEntry {
    extension tea,      # File extension (e.g., "jpg")
    mime_type tea,      # MIME type (e.g., "image/jpeg")
    description tea,    # Human description (e.g., "JPEG image")
    binary lit         # Whether file type is binary
}
```

### ContentTypeHeader
```cursed
squad ContentTypeHeader {
    media_type tea,     # Primary media type
    charset tea,        # Character encoding (e.g., "utf-8")
    boundary tea,       # Multipart boundary
    encoding tea        # Content encoding
}
```

## Integration

The `mimez` module integrates seamlessly with other CURSED stdlib modules:

- **networkz**: HTTP Content-Type header handling
- **filez**: File type detection for filesystem operations
- **vibez**: I/O operations with MIME-aware content handling
- **webz**: Web server MIME type resolution

## Performance

- **Fast Extension Lookup**: O(n) linear search through MIME database
- **Efficient Content Analysis**: Samples only first 512 bytes for text detection
- **Memory Efficient**: Static MIME database, minimal memory allocation
- **Zero Dependencies**: Pure CURSED implementation, no FFI calls

## Standards Compliance

- **RFC 2046**: MIME Part One - Format of Internet Message Bodies
- **RFC 2047**: MIME Part Two - Message Header Extensions  
- **RFC 6838**: Media Type Specifications and Registration Procedures
- **Common Magic Bytes**: Industry-standard binary signatures

## Future Enhancements

- Magic byte database expansion
- MIME type registration system  
- Custom MIME type definitions
- Content encoding detection
- Locale-specific MIME mappings

---

**Module Status**: ✅ Production Ready  
**Dependencies**: `stringz`, `arrayz`, `mathz`  
**FFI Dependencies**: None (100% Pure CURSED)  
**Standards**: RFC 2046, RFC 6838 compliant
