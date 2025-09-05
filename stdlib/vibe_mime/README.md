# vibe_mime Module

MIME type handling and multipart processing for CURSED applications.

## Overview

The `vibe_mime` module provides comprehensive functionality for working with MIME types and encodings. It helps with detecting, parsing, and generating MIME-formatted data, particularly for email and HTTP applications.

## Core Features

### MIME Type Management
- **Parsing**: Parse MIME type strings with parameters
- **Detection**: Automatic content type detection from data
- **Database**: Comprehensive mapping between file extensions and MIME types
- **Validation**: Type checking and validation utilities

### Multipart Processing
- **Reading**: Parse multipart MIME messages and forms
- **Writing**: Generate multipart content with proper boundaries
- **Streaming**: Process large multipart data incrementally
- **Form Handling**: Parse HTML form uploads and data

### Content Analysis
- **Signature Detection**: Custom file signature detection
- **Tree Parsing**: Hierarchical MIME structure analysis
- **Header Processing**: Encode/decode MIME headers
- **Content Classification**: Categorize content by type

## Usage Examples

### Basic MIME Type Operations
```cursed
# Parse MIME type
mimeType, err := vibe_mime.ParseMediaType("text/html; charset=utf-8")
if err == "" {
    vibez.spill("Type: %s/%s", mimeType.Type, mimeType.Subtype)
    vibez.spill("Charset: %s", mimeType.Parameters["charset"])
}

# Check type properties
if mimeType.IsHTML() {
    vibez.spill("This is HTML content")
}

if mimeType.IsText() {
    vibez.spill("This is text content")
}

# Add parameters
newType := mimeType.WithParameter("boundary", "boundary123")
vibez.spill("Full type: %s", newType.String())
```

### Content Type Detection
```cursed
# Detect from file content
fileData := readFileData("image.png")
contentType := vibe_mime.DetectContentType(fileData)
vibez.spill("Detected type: %s", contentType)

# Get type by extension
mimeType := vibe_mime.TypeByExtension(".pdf")
vibez.spill("PDF type: %s", mimeType)

# Get extension by type
ext := vibe_mime.ExtensionByType("image/jpeg")
vibez.spill("JPEG extension: %s", ext)

# Add custom mapping
vibe_mime.AddExtensionType(".custom", "application/x-custom")
```

### Multipart Writing
```cursed
# Create multipart writer
var buffer bytes.Buffer
writer := vibe_mime.NewMultipartWriter(&buffer)

# Write form field
err := writer.WriteField("username", "alice")
if err != "" {
    vibez.spill("Error: %s", err)
}

# Create file field
fileWriter, err := writer.CreateFormFile("upload", "document.pdf")
if err != "" {
    vibez.spill("Error: %s", err)
}

# Write file content
fileData := []byte("PDF content here")
fileWriter.Write(fileData)

# Close writer
writer.Close()

# Get boundary for Content-Type header
boundary := writer.Boundary()
contentType := "multipart/form-data; boundary=" + boundary
```

### Multipart Reading
```cursed
# Parse multipart data
reader := vibe_mime.NewMultipartReader(dataReader, boundary)

# Read all parts
for {
    part, err := reader.NextPart()
    if err == "EOF" {
        break
    }
    if err != "" {
        vibez.spill("Error: %s", err)
        break
    }
    
    # Process part
    contentType := part.ContentType()
    disposition := part.ContentDisposition()
    filename := part.Filename()
    
    vibez.spill("Part: type=%s, disposition=%s, filename=%s", 
                contentType, disposition, filename)
    
    # Read part data
    partData := part.Body
    vibez.spill("Part size: %d bytes", len(partData))
}

# Read as form
form, err := reader.ReadForm(10485760) # 10MB max
if err == "" {
    for fieldName, values := range form.Value {
        vibez.spill("Field %s: %v", fieldName, values)
    }
    
    for fieldName, files := range form.File {
        for _, file := range files {
            vibez.spill("File %s: %s (%d bytes)", 
                        fieldName, file.Filename, file.Size)
        }
    }
}
```

### Custom Content Detection
```cursed
# Create custom detector
detector := vibe_mime.NewDetector()

# Add custom signatures
detector.AddSignature("PNG", []byte{0x89, 0x50, 0x4E, 0x47})
detector.AddSignature("JPEG", []byte{0xFF, 0xD8, 0xFF})
detector.AddSignature("PDF", []byte{0x25, 0x50, 0x44, 0x46})

# Detect content
data := readFileData("unknown.file")
fileType := detector.Detect(data)
vibez.spill("Detected: %s", fileType)
```

### MIME Database Operations
```cursed
# Get database
db := vibe_mime.GetDatabase()

# Get types by category
imageTypes := db.TypesWithCategory("image")
for _, mimeType := range imageTypes {
    vibez.spill("Image type: %s", mimeType)
}

textTypes := db.TypesWithCategory("text")
audioTypes := db.TypesWithCategory("audio")
videoTypes := db.TypesWithCategory("video")
```

### Stream Processing
```cursed
# Process MIME stream incrementally
processor := vibe_mime.NewStreamProcessor(reader)

for processor.Next() {
    part := processor.Current()
    
    # Process part incrementally
    contentType := part.ContentType()
    vibez.spill("Processing part: %s", contentType)
    
    # Handle different content types
    if string.Contains(contentType, "image/") {
        processImage(part.Body)
    } else if string.Contains(contentType, "text/") {
        processText(part.Body)
    }
}
```

### MIME Tree Analysis
```cursed
# Parse email or complex MIME structure
emailData := readEmailData("message.eml")
tree := vibe_mime.ParseTree(emailData)

# Find specific parts
textPart := tree.GetFirstPartByType("text/plain")
if textPart != cap {
    vibez.spill("Plain text: %s", tea(textPart.Body))
}

htmlPart := tree.GetFirstPartByType("text/html")
if htmlPart != cap {
    vibez.spill("HTML content found")
}

# Walk tree structure
processNode(tree)

slay processNode(node *vibe_mime.TreeNode) {
    vibez.spill("Node: %s", node.Part.ContentType())
    
    for _, child := range node.Children {
        processNode(child)
    }
}
```

### Header Encoding/Decoding
```cursed
# Encode headers with special characters
encoded := vibe_mime.EncodeHeader("Subject", "Привет мир")
vibez.spill("Encoded: %s", encoded)

# Decode encoded headers
decoded := vibe_mime.DecodeHeader("=?UTF-8?B?0J/RgNC40LLQtdGCINC80LjRgA==?=")
vibez.spill("Decoded: %s", decoded)
```

## MIME Type Constants

The module provides constants for common MIME types:

### Text Types
- `TypeTextPlain` - "text/plain"
- `TypeTextHTML` - "text/html"
- `TypeTextCSS` - "text/css"
- `TypeTextJavaScript` - "text/javascript"

### Application Types
- `TypeApplicationJSON` - "application/json"
- `TypeApplicationXML` - "application/xml"
- `TypeApplicationPDF` - "application/pdf"
- `TypeApplicationZip` - "application/zip"
- `TypeApplicationOctetStream` - "application/octet-stream"

### Image Types
- `TypeImageJPEG` - "image/jpeg"
- `TypeImagePNG` - "image/png"
- `TypeImageGIF` - "image/gif"
- `TypeImageWebP` - "image/webp"

### Audio/Video Types
- `TypeAudioMP3` - "audio/mpeg"
- `TypeAudioWAV` - "audio/wav"
- `TypeVideoMP4` - "video/mp4"
- `TypeVideoWebM` - "video/webm"

### Multipart Types
- `TypeMultipartFormData` - "multipart/form-data"
- `TypeMultipartMixed` - "multipart/mixed"

## API Reference

### Core Functions
- `ParseMediaType(tea) (MIMEType, tea)` - Parse MIME type string
- `FormatMediaType(MIMEType) tea` - Format MIME type to string
- `DetectContentType([]byte) tea` - Detect content type from data
- `TypeByExtension(tea) tea` - Get MIME type for extension
- `ExtensionByType(tea) tea` - Get extension for MIME type
- `AddExtensionType(tea, tea) tea` - Add custom mapping

### Multipart Functions
- `NewMultipartReader(io.Reader, tea) *MultipartReader` - Create reader
- `NewMultipartWriter(io.Writer) *MultipartWriter` - Create writer

### Detection Functions
- `NewDetector() *Detector` - Create custom detector
- `GetDatabase() *Database` - Get MIME database
- `NewStreamProcessor(io.Reader) *StreamProcessor` - Create stream processor

### Analysis Functions
- `ParseTree([]byte) *TreeNode` - Parse MIME tree
- `EncodeHeader(tea, tea) tea` - Encode header
- `DecodeHeader(tea) tea` - Decode header

### Type Methods
- `String() tea` - Convert to string representation
- `IsText() lit` - Check if text type
- `IsHTML() lit` - Check if HTML type
- `IsJSON() lit` - Check if JSON type
- `IsXML() lit` - Check if XML type
- `WithParameter(tea, tea) MIMEType` - Add parameter

## File Signatures

The module recognizes these file signatures:
- PNG: `0x89, 0x50, 0x4E, 0x47`
- JPEG: `0xFF, 0xD8, 0xFF`
- GIF: `0x47, 0x49, 0x46`
- PDF: `0x25, 0x50, 0x44, 0x46`

## Testing

Run the test suite:
```bash
cargo run --bin cursed stdlib/vibe_mime/test_vibe_mime.💀
```

Test both interpretation and compilation modes:
```bash
cargo run --bin cursed stdlib/vibe_mime/test_vibe_mime.💀
cargo run --bin cursed -- compile stdlib/vibe_mime/test_vibe_mime.💀
./test_vibe_mime
```

## Dependencies

- `io` - I/O operations
- `string` - String processing
- `testz` - Testing framework

## Implementation Notes

- Efficient parsing and generation of MIME content
- Memory-efficient multipart processing for large files
- Extensible MIME type database
- Support for custom file signatures
- Proper handling of character encodings
- Robust error handling for malformed data
- Thread-safe operations for concurrent use

## Integration

The `vibe_mime` module integrates well with:
- `web_vibez` - For HTTP request/response processing
- `smtp_tea` - For email message handling
- `glowup_http` - For HTTP multipart uploads
- `fs` - For file type detection
- `crypto` - For content validation and signing
