# MIMEVibe (mime packages)

## Overview
MIMEVibe provides functionality for working with MIME types, multipart messages, and MIME-encoded data with good vibes. It's inspired by Go's mime package and its subpackages but with enhanced features for modern content types, improved detection algorithms, and simplified interfaces.

## Core MIME Types

### `VibeType`
Represents a MIME type with additional metadata.

```go
type VibeType struct {
    Type       string
    Subtype    string
    Parameters map[string]string
}

// Constructors
func ParseVibeType(mimeString string) (VibeType, error)
func NewVibeType(type_, subtype string, params map[string]string) VibeType

// Methods
func (t VibeType) String() string
func (t VibeType) FullType() string // Returns "type/subtype"
func (t VibeType) IsText() bool
func (t VibeType) IsImage() bool
func (t VibeType) IsAudio() bool
func (t VibeType) IsVideo() bool
func (t VibeType) IsApplication() bool
func (t VibeType) IsMultipart() bool
func (t VibeType) WithCharset(charset string) VibeType
func (t VibeType) WithParameter(key, value string) VibeType
func (t VibeType) GetParameter(key string) string
func (t VibeType) Match(pattern string) bool // Support for wildcards e.g. "image/*"
```

### Common MIME Type Constants

```go
var (
    // Text types
    TypeTextPlain = VibeType{Type: "text", Subtype: "plain", Parameters: map[string]string{"charset": "utf-8"}}
    TypeTextHTML  = VibeType{Type: "text", Subtype: "html", Parameters: map[string]string{"charset": "utf-8"}}
    TypeTextCSS   = VibeType{Type: "text", Subtype: "css", Parameters: map[string]string{"charset": "utf-8"}}
    TypeTextXML   = VibeType{Type: "text", Subtype: "xml", Parameters: map[string]string{"charset": "utf-8"}}
    TypeTextCSV   = VibeType{Type: "text", Subtype: "csv", Parameters: map[string]string{"charset": "utf-8"}}
    TypeTextMarkdown = VibeType{Type: "text", Subtype: "markdown", Parameters: map[string]string{"charset": "utf-8"}}
    
    // Image types
    TypeImageJPEG = VibeType{Type: "image", Subtype: "jpeg"}
    TypeImagePNG  = VibeType{Type: "image", Subtype: "png"}
    TypeImageGIF  = VibeType{Type: "image", Subtype: "gif"}
    TypeImageSVG  = VibeType{Type: "image", Subtype: "svg+xml"}
    TypeImageWebP = VibeType{Type: "image", Subtype: "webp"}
    TypeImageBMP  = VibeType{Type: "image", Subtype: "bmp"}
    
    // Audio types
    TypeAudioMP3  = VibeType{Type: "audio", Subtype: "mpeg"}
    TypeAudioWAV  = VibeType{Type: "audio", Subtype: "wav"}
    TypeAudioOGG  = VibeType{Type: "audio", Subtype: "ogg"}
    TypeAudioAAC  = VibeType{Type: "audio", Subtype: "aac"}
    
    // Video types
    TypeVideoMP4  = VibeType{Type: "video", Subtype: "mp4"}
    TypeVideoWebM = VibeType{Type: "video", Subtype: "webm"}
    TypeVideoOGG  = VibeType{Type: "video", Subtype: "ogg"}
    
    // Application types
    TypeApplicationJSON = VibeType{Type: "application", Subtype: "json", Parameters: map[string]string{"charset": "utf-8"}}
    TypeApplicationPDF  = VibeType{Type: "application", Subtype: "pdf"}
    TypeApplicationZip  = VibeType{Type: "application", Subtype: "zip"}
    TypeApplicationXML  = VibeType{Type: "application", Subtype: "xml", Parameters: map[string]string{"charset": "utf-8"}}
    TypeApplicationJavaScript = VibeType{Type: "application", Subtype: "javascript", Parameters: map[string]string{"charset": "utf-8"}}
    TypeApplicationOctetStream = VibeType{Type: "application", Subtype: "octet-stream"}
    TypeApplicationWasm = VibeType{Type: "application", Subtype: "wasm"}
    
    // Multipart types
    TypeMultipartFormData = VibeType{Type: "multipart", Subtype: "form-data"}
    TypeMultipartMixed = VibeType{Type: "multipart", Subtype: "mixed"}
    TypeMultipartAlternative = VibeType{Type: "multipart", Subtype: "alternative"}
    
    // Modern web types
    TypeApplicationGraphQL = VibeType{Type: "application", Subtype: "graphql"}
    TypeApplicationProtobuf = VibeType{Type: "application", Subtype: "protobuf"}
    TypeApplicationGRPC = VibeType{Type: "application", Subtype: "grpc"}
    TypeApplicationMsgpack = VibeType{Type: "application", Subtype: "msgpack"}
    TypeApplicationYAML = VibeType{Type: "application", Subtype: "yaml", Parameters: map[string]string{"charset": "utf-8"}}
)
```

## MIME Type Detection

```go
// Detect the MIME type from file extension
func TypeByExtension(ext string) VibeType

// Detect the MIME type from file name
func TypeByFilename(filename string) VibeType

// Detect the MIME type by sniffing the data
func TypeByContent(data []byte) VibeType

// Detect the MIME type from a file
func TypeByFile(file dropz.File) (VibeType, error)

// Detect the MIME type from a file path
func TypeByPath(path string) (VibeType, error)

// Enhanced detection with multiple methods
func DetectVibeType(filename string, data []byte) VibeType

// Register a custom type detector
func RegisterDetector(detector func(data []byte) (VibeType, bool))

// MIME type registry management
func AddExtensionMapping(ext string, mimeType VibeType)
func AddMagicPattern(pattern []byte, mask []byte, offset int, mimeType VibeType)
func LoadExtensionsFile(path string) error
```

## Content Encoding

```go
// Encodings for content transfer
type VibeEncoding string

const (
    EncodingBase64    VibeEncoding = "base64"
    EncodingQuotedPrintable VibeEncoding = "quoted-printable"
    Encoding7Bit      VibeEncoding = "7bit"
    Encoding8Bit      VibeEncoding = "8bit"
    EncodingBinary    VibeEncoding = "binary"
    EncodingGzip      VibeEncoding = "gzip"
    EncodingDeflate   VibeEncoding = "deflate"
    EncodingBrotli    VibeEncoding = "br"
)

// Encoding/decoding functions
func EncodeContent(data []byte, encoding VibeEncoding) ([]byte, error)
func DecodeContent(data []byte, encoding VibeEncoding) ([]byte, error)
```

## Message Handling

### `VibePart`
Represents a part in a MIME multipart message.

```go
type VibePart struct {
    Header     map[string][]string
    Body       []byte
    Filename   string
    Name       string
    ContentType VibeType
    Encoding   VibeEncoding
    Size       int64
    Parts      []*VibePart  // For nested multipart messages
}

// Methods
func (p *VibePart) GetHeader(key string) string
func (p *VibePart) SetHeader(key, value string)
func (p *VibePart) GetDisposition() string
func (p *VibePart) SetDisposition(disp string)
func (p *VibePart) GetContentID() string
func (p *VibePart) SetContentID(id string)
func (p *VibePart) String() string
func (p *VibePart) WriteTo(w io.Writer) (int64, error)
func (p *VibePart) ReadFrom(r io.Reader) (int64, error)
```

### `VibeMessage`
Top-level MIME message.

```go
type VibeMessage struct {
    Header     map[string][]string
    Parts      []*VibePart
    ContentType VibeType
    Boundary   string
}

// Constructors
func NewVibeMessage() *VibeMessage
func ParseVibeMessage(r io.Reader) (*VibeMessage, error)

// Methods
func (m *VibeMessage) AddTextPart(content string, contentType VibeType) *VibePart
func (m *VibeMessage) AddBinaryPart(data []byte, contentType VibeType, filename string) *VibePart
func (m *VibeMessage) AddFilePart(filepath string) (*VibePart, error)
func (m *VibeMessage) GetHeader(key string) string
func (m *VibeMessage) SetHeader(key, value string)
func (m *VibeMessage) String() string
func (m *VibeMessage) WriteTo(w io.Writer) (int64, error)
func (m *VibeMessage) Bytes() []byte
```

### Multipart Form Handling

```go
type VibeForm struct {
    Values map[string][]string
    Files  map[string][]*VibeFile
}

type VibeFile struct {
    Filename    string
    ContentType VibeType
    Size        int64
    Data        []byte
}

// Parse multipart form from a request
func ParseMultipartForm(r io.Reader, boundary string) (*VibeForm, error)

// Parse multipart form from a request body with Content-Type header
func ParseFormData(r io.Reader, contentType string) (*VibeForm, error)

// Methods for VibeForm
func (f *VibeForm) GetValue(key string) string
func (f *VibeForm) GetValues(key string) []string
func (f *VibeForm) GetFile(key string) *VibeFile
func (f *VibeForm) GetFiles(key string) []*VibeFile
func (f *VibeForm) AddValue(key, value string)
func (f *VibeForm) AddFile(key string, file *VibeFile)
func (f *VibeForm) Encode() ([]byte, string)
```

## Advanced MIME Features

### MIME Walking and Transformation

```go
type VibeWalker interface {
    WalkPart(part *VibePart) error
}

// Walk through all parts of a MIME message
func WalkMessage(msg *VibeMessage, walker VibeWalker) error

// Transform MIME parts using a transformer function
func TransformMessage(msg *VibeMessage, transformer func(*VibePart) (*VibePart, error)) (*VibeMessage, error)

// Extract specific parts from a MIME message
func ExtractParts(msg *VibeMessage, matcher func(*VibePart) bool) []*VibePart
```

### Email MIME Support

```go
type EmailMessage struct {
    *VibeMessage
    From        string
    To          []string
    Cc          []string
    Bcc         []string
    Subject     string
    TextBody    string
    HTMLBody    string
    Attachments []*VibePart
}

// Constructors
func NewEmailMessage() *EmailMessage
func ParseEmailMessage(r io.Reader) (*EmailMessage, error)

// Methods
func (e *EmailMessage) SetFrom(address string)
func (e *EmailMessage) AddTo(address string)
func (e *EmailMessage) AddCc(address string)
func (e *EmailMessage) AddBcc(address string)
func (e *EmailMessage) SetSubject(subject string)
func (e *EmailMessage) SetTextBody(body string)
func (e *EmailMessage) SetHTMLBody(body string)
func (e *EmailMessage) AddAttachment(filename string, data []byte, contentType VibeType) *VibePart
func (e *EmailMessage) AddFileAttachment(filepath string) (*VibePart, error)
func (e *EmailMessage) String() string
func (e *EmailMessage) Bytes() []byte
```

### MIME Utility Functions

```go
// Generate a random boundary string
func GenerateBoundary() string

// Extract charset from a MIME type
func ExtractCharset(mimeType VibeType) string

// Clean up MIME headers
func CanonicalMIMEHeaderKey(s string) string

// Format MIME header fields
func FormatMediaType(mediaType string, params map[string]string) string

// Word encoding for headers
func EncodeWord(s string) string
func DecodeWord(s string) (string, error)

// Encode an entire header field
func EncodeHeader(s string) string
func DecodeHeader(s string) string
```

## GenZ-Themed Features

```go
// Generate a vibey content type based on content
func VibeCheck(data []byte) VibeType

// No cap MIME detection - always accurate without exaggeration
func NoCapDetect(filename string, data []byte) VibeType

// Aesthetic encodings for GenZ content
func AestheticEncode(data []byte) []byte
func AestheticDecode(data []byte) ([]byte, error)

// Emoji-enhanced MIME types
func EmojiType(mimeType VibeType) string
```

## Usage Examples

```go
// Basic MIME type parsing
mimeStr := "text/html; charset=utf-8"
mimeType, err := mime_vibe.ParseVibeType(mimeStr)
if err != nil {
    vibez.spill("Error parsing MIME type:", err)
    return
}

vibez.spill("Type:", mimeType.Type)
vibez.spill("Subtype:", mimeType.Subtype)
vibez.spill("Charset:", mimeType.GetParameter("charset"))

// MIME type detection by file extension
pngType := mime_vibe.TypeByExtension(".png")
vibez.spill("PNG MIME type:", pngType.String())

// MIME type detection by content sniffing
fileData, err := dropz.ReadFile("example.jpg")
if err != nil {
    vibez.spill("Error reading file:", err)
    return
}

detectedType := mime_vibe.TypeByContent(fileData)
vibez.spill("Detected MIME type:", detectedType.String())

// Enhanced detection using multiple methods
mimeType = mime_vibe.DetectVibeType("unknown_file", fileData)
vibez.spill("Best guess MIME type:", mimeType.String())

// Content encoding/decoding
originalText := "Hello, MIME world! 🌍"
encoded, err := mime_vibe.EncodeContent([]byte(originalText), mime_vibe.EncodingBase64)
if err != nil {
    vibez.spill("Error encoding content:", err)
    return
}
vibez.spill("Base64 encoded:", string(encoded))

decoded, err := mime_vibe.DecodeContent(encoded, mime_vibe.EncodingBase64)
if err != nil {
    vibez.spill("Error decoding content:", err)
    return
}
vibez.spill("Decoded text:", string(decoded))

// Creating a multipart message
msg := mime_vibe.NewVibeMessage()
msg.ContentType = mime_vibe.TypeMultipartMixed

// Add text part
textPart := msg.AddTextPart("This is a plain text part.", mime_vibe.TypeTextPlain)
textPart.SetHeader("Content-ID", "<text-part@example.com>")

// Add HTML part
htmlContent := "<html><body><h1>Hello, World!</h1></body></html>"
msg.AddTextPart(htmlContent, mime_vibe.TypeTextHTML)

// Add an attachment
attachmentData := []byte{0x48, 0x65, 0x6C, 0x6C, 0x6F} // "Hello" in bytes
msg.AddBinaryPart(attachmentData, mime_vibe.TypeApplicationOctetStream, "attachment.bin")

// Convert to string representation
mimeMessage := msg.String()
vibez.spill("MIME Message:\n", mimeMessage)

// Parse a multipart form
formData := "--boundary\r\n" +
    "Content-Disposition: form-data; name=\"field1\"\r\n\r\n" +
    "value1\r\n" +
    "--boundary\r\n" +
    "Content-Disposition: form-data; name=\"field2\"; filename=\"example.txt\"\r\n" +
    "Content-Type: text/plain\r\n\r\n" +
    "file content\r\n" +
    "--boundary--\r\n"

form, err := mime_vibe.ParseMultipartForm(strings.NewReader(formData), "boundary")
if err != nil {
    vibez.spill("Error parsing form:", err)
    return
}

vibez.spill("Form field 'field1':", form.GetValue("field1"))
file := form.GetFile("field2")
if file != nil {
    vibez.spill("File name:", file.Filename)
    vibez.spill("File content:", string(file.Data))
}

// Creating an email message
email := mime_vibe.NewEmailMessage()
email.SetFrom("sender@example.com")
email.AddTo("recipient@example.com")
email.SetSubject("Hello from MIME package")
email.SetTextBody("This is a plain text version.")
email.SetHTMLBody("<html><body><p>This is an <b>HTML</b> version.</p></body></html>")
email.AddAttachment("document.pdf", pdfData, mime_vibe.TypeApplicationPDF)

emailBytes := email.Bytes()
// Send email bytes through an SMTP client

// Using GenZ-themed features
vibeType := mime_vibe.VibeCheck(fileData)
vibez.spill("Vibe check result:", vibeType.String())

emojiType := mime_vibe.EmojiType(mime_vibe.TypeImageJPEG)
vibez.spill("Emoji type:", emojiType) // Outputs: "🖼️ image/jpeg"

accurateType := mime_vibe.NoCapDetect("mystery_file", fileData)
vibez.spill("No cap detected type:", accurateType.String())
```

## Implementation Guidelines
1. Optimize MIME type detection algorithms for accuracy and performance
2. Support all standard MIME types and common non-standard types
3. Ensure correct handling of nested multipart messages
4. Implement efficient content encoding/decoding with minimal allocations
5. Provide clear error messages for malformed MIME content
6. Support internationalization in headers and content
7. Maintain backward compatibility with Go's mime package
8. Implement thread-safe operations for concurrent use