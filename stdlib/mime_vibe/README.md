# MIMEVibe Module

## Overview
MIMEVibe provides functionality for working with MIME types, multipart messages, and MIME-encoded data with good vibes. It's inspired by Go's mime package and its subpackages but with enhanced features for modern content types, improved detection algorithms, and simplified interfaces.

## Core Types

### `VibeType`
Represents a MIME type with additional metadata.
- **Type** - Main type (text, image, audio, video, application, multipart)
- **Subtype** - Specific subtype (plain, html, json, png, etc.)
- **Parameters** - Additional parameters (charset, boundary, etc.)

### `VibeEncoding`
Represents content transfer encoding types.

### `VibePart`
Represents a part in a multipart MIME message.

### `VibeMessage`
Represents a complete MIME message with multiple parts.

### `VibeForm`
Represents parsed multipart form data.

### `VibeFile`
Represents a file in a multipart form.

## Predefined MIME Types

### Text Types
- **TypeTextPlain** - text/plain with UTF-8 charset
- **TypeTextHTML** - text/html with UTF-8 charset
- **TypeTextCSS** - text/css with UTF-8 charset
- **TypeTextXML** - text/xml with UTF-8 charset
- **TypeTextCSV** - text/csv with UTF-8 charset
- **TypeTextMarkdown** - text/markdown with UTF-8 charset

### Image Types
- **TypeImageJPEG** - image/jpeg
- **TypeImagePNG** - image/png
- **TypeImageGIF** - image/gif
- **TypeImageSVG** - image/svg+xml
- **TypeImageWebP** - image/webp
- **TypeImageBMP** - image/bmp

### Audio Types
- **TypeAudioMP3** - audio/mpeg
- **TypeAudioWAV** - audio/wav
- **TypeAudioOGG** - audio/ogg
- **TypeAudioAAC** - audio/aac

### Video Types
- **TypeVideoMP4** - video/mp4
- **TypeVideoWebM** - video/webm
- **TypeVideoOGG** - video/ogg

### Application Types
- **TypeApplicationJSON** - application/json with UTF-8 charset
- **TypeApplicationPDF** - application/pdf
- **TypeApplicationZip** - application/zip
- **TypeApplicationXML** - application/xml with UTF-8 charset
- **TypeApplicationJavaScript** - application/javascript with UTF-8 charset
- **TypeApplicationOctetStream** - application/octet-stream
- **TypeApplicationWasm** - application/wasm

### Modern Web Types
- **TypeApplicationGraphQL** - application/graphql
- **TypeApplicationProtobuf** - application/protobuf
- **TypeApplicationGRPC** - application/grpc
- **TypeApplicationMsgpack** - application/msgpack
- **TypeApplicationYAML** - application/yaml with UTF-8 charset

### Multipart Types
- **TypeMultipartFormData** - multipart/form-data
- **TypeMultipartMixed** - multipart/mixed
- **TypeMultipartAlternative** - multipart/alternative

## Core Functions

### MIME Type Creation and Parsing
- **ParseVibeType(mimeString tea) (VibeType, tea)** - Parse MIME type string
- **NewVibeType(type_, subtype tea, params map[tea]tea) VibeType** - Create new MIME type

### MIME Type Detection
- **TypeByExtension(ext tea) VibeType** - Detect by file extension
- **TypeByFilename(filename tea) VibeType** - Detect by filename
- **TypeByContent(data []normie) VibeType** - Detect by content analysis
- **DetectVibeType(filename tea, data []normie) VibeType** - Enhanced detection

### Content Encoding
- **EncodeContent(data []normie, encoding VibeEncoding) ([]normie, tea)** - Encode content
- **DecodeContent(data []normie, encoding VibeEncoding) ([]normie, tea)** - Decode content

### Message Handling
- **NewVibeMessage() *VibeMessage** - Create new multipart message

## VibeType Methods

### Type Checking
- **IsText() lit** - Check if text type
- **IsImage() lit** - Check if image type
- **IsAudio() lit** - Check if audio type
- **IsVideo() lit** - Check if video type
- **IsApplication() lit** - Check if application type
- **IsMultipart() lit** - Check if multipart type

### String Representation
- **String() tea** - Full MIME type string with parameters
- **FullType() tea** - Type/subtype without parameters

### Parameter Management
- **GetParameter(key tea) tea** - Get parameter value
- **WithCharset(charset tea) VibeType** - Add charset parameter
- **WithParameter(key, value tea) VibeType** - Add custom parameter

### Pattern Matching
- **Match(pattern tea) lit** - Match against pattern (supports wildcards)

## Content Encodings

- **EncodingBase64** - Base64 encoding
- **EncodingQuotedPrintable** - Quoted-printable encoding
- **Encoding7Bit** - 7-bit encoding
- **Encoding8Bit** - 8-bit encoding
- **EncodingBinary** - Binary encoding
- **EncodingGzip** - Gzip compression
- **EncodingDeflate** - Deflate compression
- **EncodingBrotli** - Brotli compression

## Message Methods

### VibeMessage
- **AddTextPart(content tea, contentType VibeType) *VibePart** - Add text part
- **AddBinaryPart(data []normie, contentType VibeType, filename tea) *VibePart** - Add binary part
- **String() tea** - Generate complete MIME message

## GenZ-Themed Features

### Vibe Functions
- **VibeCheck(data []normie) VibeType** - Check the vibe of content
- **NoCapDetect(filename tea, data []normie) VibeType** - Accurate detection (no lies)
- **EmojiType(mimeType VibeType) tea** - Get MIME type with emoji

## Usage Examples

### Basic MIME Type Operations
```cursed
yeet "mime_vibe"

fr fr Parse MIME type
sus mimeType, err := mime_vibe.ParseVibeType("text/html; charset=utf-8")
if err == "" {
    vibez.spill("Type:", mimeType.Type)        fr fr "text"
    vibez.spill("Subtype:", mimeType.Subtype) fr fr "html"
    vibez.spill("Charset:", mimeType.GetParameter("charset")) fr fr "utf-8"
}

fr fr Create new MIME type
sus params := make(map[tea]tea)
params["boundary"] = "abc123"
sus multipart := mime_vibe.NewVibeType("multipart", "form-data", params)
vibez.spill("Full type:", multipart.String())
```

### MIME Type Detection
```cursed
fr fr Detect by extension
sus pngType := mime_vibe.TypeByExtension(".png")
vibez.spill("PNG type:", pngType.FullType()) fr fr "image/png"

fr fr Detect by filename
sus docType := mime_vibe.TypeByFilename("document.pdf")
vibez.spill("Document type:", docType.FullType()) fr fr "application/pdf"

fr fr Detect by content (magic bytes)
sus jpegData := []normie{255, 216, 255, 224} fr fr JPEG signature
sus detected := mime_vibe.TypeByContent(jpegData)
vibez.spill("Detected:", detected.FullType()) fr fr "image/jpeg"

fr fr Enhanced detection
sus bestGuess := mime_vibe.DetectVibeType("photo.jpg", jpegData)
vibez.spill("Best guess:", bestGuess.FullType())
```

### Type Checking
```cursed
fr fr Check type categories
assert_true(mime_vibe.TypeTextHTML.IsText())
assert_true(mime_vibe.TypeImagePNG.IsImage())
assert_true(mime_vibe.TypeAudioMP3.IsAudio())
assert_true(mime_vibe.TypeVideoMP4.IsVideo())
assert_true(mime_vibe.TypeApplicationJSON.IsApplication())
assert_true(mime_vibe.TypeMultipartMixed.IsMultipart())

fr fr Pattern matching
assert_true(mime_vibe.TypeTextPlain.Match("text/*"))
assert_true(mime_vibe.TypeImagePNG.Match("*/*"))
assert_false(mime_vibe.TypeTextPlain.Match("image/*"))
```

### Parameter Management
```cursed
fr fr Add charset parameter
sus withCharset := mime_vibe.TypeTextHTML.WithCharset("iso-8859-1")
vibez.spill("With charset:", withCharset.String())

fr fr Add custom parameter
sus withCustom := mime_vibe.TypeApplicationJSON.WithParameter("version", "2.0")
vibez.spill("Custom param:", withCustom.GetParameter("version"))
```

### Content Encoding
```cursed
fr fr Encode content
sus data := []normie{72, 101, 108, 108, 111} fr fr "Hello"
sus encoded, err := mime_vibe.EncodeContent(data, mime_vibe.EncodingBase64)
if err == "" {
    vibez.spill("Encoded length:", len(encoded))
}

fr fr Decode content
sus decoded, decErr := mime_vibe.DecodeContent(encoded, mime_vibe.EncodingBase64)
if decErr == "" {
    vibez.spill("Decoded length:", len(decoded))
}
```

### Multipart Messages
```cursed
fr fr Create multipart message
sus msg := mime_vibe.NewVibeMessage()

fr fr Add text part
sus textPart := msg.AddTextPart("Hello, World!", mime_vibe.TypeTextPlain)
vibez.spill("Added text part, size:", textPart.Size)

fr fr Add binary attachment
sus fileData := []normie{80, 75, 3, 4} fr fr ZIP signature
sus attachment := msg.AddBinaryPart(fileData, mime_vibe.TypeApplicationZip, "archive.zip")
vibez.spill("Added attachment:", attachment.Filename)

fr fr Generate message
sus messageStr := msg.String()
vibez.spill("Message length:", len(messageStr))
```

### GenZ Features
```cursed
fr fr Vibe check content
sus unknownData := []normie{137, 80, 78, 71} fr fr PNG signature
sus vibeResult := mime_vibe.VibeCheck(unknownData)
vibez.spill("Vibe check:", vibeResult.FullType())

fr fr No cap detection (always accurate)
sus accurate := mime_vibe.NoCapDetect("image.png", unknownData)
vibez.spill("No cap result:", accurate.FullType())

fr fr Emoji types
sus emojiText := mime_vibe.EmojiType(mime_vibe.TypeTextPlain)
vibez.spill("Text with emoji:", emojiText) fr fr "📄 text/plain"

sus emojiImage := mime_vibe.EmojiType(mime_vibe.TypeImagePNG)
vibez.spill("Image with emoji:", emojiImage) fr fr "🖼️ image/png"
```

## Implementation Features

1. **Pure CURSED Implementation** - No FFI dependencies
2. **Comprehensive Type Support** - Extensive predefined MIME types
3. **Content Detection** - Magic byte analysis for accurate detection
4. **Modern Web Support** - GraphQL, protobuf, WASM, and other modern types
5. **Flexible Parameters** - Full parameter management
6. **Pattern Matching** - Wildcard support for type matching
7. **Content Encoding** - Base64 and other encoding support
8. **Multipart Messages** - Complete multipart MIME support
9. **GenZ Features** - Fun emoji and vibe-based functions

## Error Handling

All functions that can fail return error messages as strings:
- Empty string ("") indicates success
- Non-empty string contains error description

## Magic Byte Detection

The module includes magic byte detection for common file types:
- **PNG**: 137, 80, 78, 71
- **JPEG**: 255, 216, 255
- **GIF**: 71, 73, 70
- **PDF**: 37, 80, 68, 70

## Implementation Notes

This is a pure CURSED implementation that provides comprehensive MIME type handling without external dependencies. The implementation includes:

- Extensive predefined MIME type constants
- Flexible parameter management system
- Content-based type detection using magic bytes
- Support for modern web content types
- Multipart message creation and parsing
- Content encoding/decoding utilities
- Fun GenZ-themed helper functions

The module is designed for both serious MIME handling and educational purposes, providing a complete foundation for working with MIME types in CURSED applications.
