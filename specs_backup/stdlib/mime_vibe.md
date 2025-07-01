# MIMEVibe (mime packages)

## Overview
MIMEVibe provides functionality for working with MIME types, multipart messages, and MIME-encoded data with good vibes. It's inspired by Go's mime package and its subpackages but with enhanced features for modern content types, improved detection algorithms, and simplified interfaces.

## Core MIME Types

### `VibeType`
Represents a MIME be_like with additional metadata.

```
be_like VibeType squad {
    Type       tea
    Subbe_like    tea
    Parameters map[tea]tea
}

fr fr Consquadors
slay ParseVibeType(mimeString tea) (VibeType, tea)
slay NewVibeType(type_, subbe_like tea, params map[tea]tea) VibeType

fr fr Methods
slay (t VibeType) String() tea
slay (t VibeType) FullType() tea fr fr Returns "type/subtype"
slay (t VibeType) IsText() lit
slay (t VibeType) IsImage() lit
slay (t VibeType) IsAudio() lit
slay (t VibeType) IsVideo() lit
slay (t VibeType) IsApplication() lit
slay (t VibeType) IsMultipart() lit
slay (t VibeType) WithCharset(charset tea) VibeType
slay (t VibeType) WithParameter(key, value tea) VibeType
slay (t VibeType) GetParameter(key tea) tea
slay (t VibeType) Match(pattern tea) lit fr fr Support for wildcards e.g. "image/*"
```

### Common MIME Type Constants

```
var (
    fr fr Text types
    TypeTextPlain = VibeType{Type: "text", Subtype: "plain", Parameters: map[tea]tea{"charset": "utf-8"}}
    TypeTextHTML  = VibeType{Type: "text", Subtype: "html", Parameters: map[tea]tea{"charset": "utf-8"}}
    TypeTextCSS   = VibeType{Type: "text", Subtype: "css", Parameters: map[tea]tea{"charset": "utf-8"}}
    TypeTextXML   = VibeType{Type: "text", Subtype: "xml", Parameters: map[tea]tea{"charset": "utf-8"}}
    TypeTextCSV   = VibeType{Type: "text", Subtype: "csv", Parameters: map[tea]tea{"charset": "utf-8"}}
    TypeTextMarkdown = VibeType{Type: "text", Subtype: "markdown", Parameters: map[tea]tea{"charset": "utf-8"}}
    
    fr fr Image types
    TypeImageJPEG = VibeType{Type: "image", Subtype: "jpeg"}
    TypeImagePNG  = VibeType{Type: "image", Subtype: "png"}
    TypeImageGIF  = VibeType{Type: "image", Subtype: "gif"}
    TypeImageSVG  = VibeType{Type: "image", Subtype: "svg+xml"}
    TypeImageWebP = VibeType{Type: "image", Subtype: "webp"}
    TypeImageBMP  = VibeType{Type: "image", Subtype: "bmp"}
    
    fr fr Audio types
    TypeAudioMP3  = VibeType{Type: "audio", Subtype: "mpeg"}
    TypeAudioWAV  = VibeType{Type: "audio", Subtype: "wav"}
    TypeAudioOGG  = VibeType{Type: "audio", Subtype: "ogg"}
    TypeAudioAAC  = VibeType{Type: "audio", Subtype: "aac"}
    
    fr fr Video types
    TypeVideoMP4  = VibeType{Type: "video", Subtype: "mp4"}
    TypeVideoWebM = VibeType{Type: "video", Subtype: "webm"}
    TypeVideoOGG  = VibeType{Type: "video", Subtype: "ogg"}
    
    fr fr Application types
    TypeApplicationJSON = VibeType{Type: "application", Subtype: "json", Parameters: map[tea]tea{"charset": "utf-8"}}
    TypeApplicationPDF  = VibeType{Type: "application", Subtype: "pdf"}
    TypeApplicationZip  = VibeType{Type: "application", Subtype: "zip"}
    TypeApplicationXML  = VibeType{Type: "application", Subtype: "xml", Parameters: map[tea]tea{"charset": "utf-8"}}
    TypeApplicationJavaScript = VibeType{Type: "application", Subtype: "javascript", Parameters: map[tea]tea{"charset": "utf-8"}}
    TypeApplicationOctetStream = VibeType{Type: "application", Subtype: "octet-stream"}
    TypeApplicationWasm = VibeType{Type: "application", Subtype: "wasm"}
    
    fr fr Multipart types
    TypeMultipartFormData = VibeType{Type: "multipart", Subtype: "form-data"}
    TypeMultipartMixed = VibeType{Type: "multipart", Subtype: "mixed"}
    TypeMultipartAlternative = VibeType{Type: "multipart", Subtype: "alternative"}
    
    fr fr Modern web types
    TypeApplicationGraphQL = VibeType{Type: "application", Subtype: "graphql"}
    TypeApplicationProtobuf = VibeType{Type: "application", Subtype: "protobuf"}
    TypeApplicationGRPC = VibeType{Type: "application", Subtype: "grpc"}
    TypeApplicationMsgpack = VibeType{Type: "application", Subtype: "msgpack"}
    TypeApplicationYAML = VibeType{Type: "application", Subtype: "yaml", Parameters: map[tea]tea{"charset": "utf-8"}}
)
```

## MIME Type Detection

```
fr fr Detect the MIME be_like from file extension
slay TypeByExtension(ext tea) VibeType

fr fr Detect the MIME be_like from file name
slay TypeByFilename(filename tea) VibeType

fr fr Detect the MIME be_like by sniffing the data
slay TypeByContent(data []byte) VibeType

fr fr Detect the MIME be_like from a file
slay TypeByFile(file dropz.File) (VibeType, tea)

fr fr Detect the MIME be_like from a file path
slay TypeByPath(path tea) (VibeType, tea)

fr fr Enhanced detection with multiple methods
slay DetectVibeType(filename tea, data []byte) VibeType

fr fr Register a custom be_like detector
slay RegisterDetector(detector func(data []byte) (VibeType, lit))

fr fr MIME be_like registry management
slay AddExtensionMapping(ext tea, mimeType VibeType)
slay AddMagicPattern(pattern []byte, mask []byte, offset int, mimeType VibeType)
slay LoadExtensionsFile(path tea) tea
```

## Content Encoding

```
fr fr Encodings for content transfer
be_like VibeEncoding tea

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

fr fr Encoding/decoding functions
slay EncodeContent(data []byte, encoding VibeEncoding) ([]byte, tea)
slay DecodeContent(data []byte, encoding VibeEncoding) ([]byte, tea)
```

## Message Handling

### `VibePart`
Represents a part in a MIME multipart message.

```
be_like VibePart squad {
    Header     map[tea][]tea
    Body       []byte
    Filename   tea
    Name       tea
    ContentType VibeType
    Encoding   VibeEncoding
    Size       int64
    Parts      []*VibePart  fr fr For nested multipart messages
}

fr fr Methods
slay (p *VibePart) GetHeader(key tea) tea
slay (p *VibePart) SetHeader(key, value tea)
slay (p *VibePart) GetDisposition() tea
slay (p *VibePart) SetDisposition(disp tea)
slay (p *VibePart) GetContentID() tea
slay (p *VibePart) SetContentID(id tea)
slay (p *VibePart) String() tea
slay (p *VibePart) WriteTo(w io.Writer) (int64, tea)
slay (p *VibePart) ReadFrom(r io.Reader) (int64, tea)
```

### `VibeMessage`
Top-level MIME message.

```
be_like VibeMessage squad {
    Header     map[tea][]tea
    Parts      []*VibePart
    ContentType VibeType
    Boundary   tea
}

fr fr Consquadors
slay NewVibeMessage() *VibeMessage
slay ParseVibeMessage(r io.Reader) (*VibeMessage, tea)

fr fr Methods
slay (m *VibeMessage) AddTextPart(content tea, contentType VibeType) *VibePart
slay (m *VibeMessage) AddBinaryPart(data []byte, contentType VibeType, filename tea) *VibePart
slay (m *VibeMessage) AddFilePart(filepath tea) (*VibePart, tea)
slay (m *VibeMessage) GetHeader(key tea) tea
slay (m *VibeMessage) SetHeader(key, value tea)
slay (m *VibeMessage) String() tea
slay (m *VibeMessage) WriteTo(w io.Writer) (int64, tea)
slay (m *VibeMessage) Bytes() []byte
```

### Multipart Form Handling

```
be_like VibeForm squad {
    Values map[tea][]tea
    Files  map[tea][]*VibeFile
}

be_like VibeFile squad {
    Filename    tea
    ContentType VibeType
    Size        int64
    Data        []byte
}

fr fr Parse multipart form from a request
slay ParseMultipartForm(r io.Reader, boundary tea) (*VibeForm, tea)

fr fr Parse multipart form from a request body with Content-Type header
slay ParseFormData(r io.Reader, contentType tea) (*VibeForm, tea)

fr fr Methods for VibeForm
slay (f *VibeForm) GetValue(key tea) tea
slay (f *VibeForm) GetValues(key tea) []tea
slay (f *VibeForm) GetFile(key tea) *VibeFile
slay (f *VibeForm) GetFiles(key tea) []*VibeFile
slay (f *VibeForm) AddValue(key, value tea)
slay (f *VibeForm) AddFile(key tea, file *VibeFile)
slay (f *VibeForm) Encode() ([]byte, tea)
```

## Advanced MIME Features

### MIME Walking and Transformation

```
be_like VibeWalker collab {
    WalkPart(part *VibePart) tea
}

fr fr Walk through all parts of a MIME message
slay WalkMessage(msg *VibeMessage, walker VibeWalker) tea

fr fr Transform MIME parts using a transformer function
slay TransformMessage(msg *VibeMessage, transformer func(*VibePart) (*VibePart, tea)) (*VibeMessage, tea)

fr fr Extract specific parts from a MIME message
slay ExtractParts(msg *VibeMessage, matcher func(*VibePart) lit) []*VibePart
```

### Email MIME Support

```
be_like EmailMessage squad {
    *VibeMessage
    From        tea
    To          []tea
    Cc          []tea
    Bcc         []tea
    Subject     tea
    TextBody    tea
    HTMLBody    tea
    Attachments []*VibePart
}

fr fr Consquadors
slay NewEmailMessage() *EmailMessage
slay ParseEmailMessage(r io.Reader) (*EmailMessage, tea)

fr fr Methods
slay (e *EmailMessage) SetFrom(address tea)
slay (e *EmailMessage) AddTo(address tea)
slay (e *EmailMessage) AddCc(address tea)
slay (e *EmailMessage) AddBcc(address tea)
slay (e *EmailMessage) SetSubject(subject tea)
slay (e *EmailMessage) SetTextBody(body tea)
slay (e *EmailMessage) SetHTMLBody(body tea)
slay (e *EmailMessage) AddAttachment(filename tea, data []byte, contentType VibeType) *VibePart
slay (e *EmailMessage) AddFileAttachment(filepath tea) (*VibePart, tea)
slay (e *EmailMessage) String() tea
slay (e *EmailMessage) Bytes() []byte
```

### MIME Utility Functions

```
fr fr Generate a random boundary tea
slay GenerateBoundary() tea

fr fr Extract charset from a MIME type
slay ExtractCharset(mimeType VibeType) tea

fr fr Clean up MIME headers
slay CanonicalMIMEHeaderKey(s tea) tea

fr fr Format MIME header fields
slay FormatMediaType(mediaType tea, params map[tea]tea) tea

fr fr Word encoding for headers
slay EncodeWord(s tea) tea
slay DecodeWord(s tea) (tea, tea)

fr fr Encode an entire header field
slay EncodeHeader(s tea) tea
slay DecodeHeader(s tea) tea
```

## GenZ-Themed Features

```
fr fr Generate a vibey content be_like based on content
slay VibeCheck(data []byte) VibeType

fr fr No cap MIME detection - always accurate without exaggeration
slay NoCapDetect(filename tea, data []byte) VibeType

fr fr Aesthetic encodings for GenZ content
slay AestheticEncode(data []byte) []byte
slay AestheticDecode(data []byte) ([]byte, tea)

fr fr Emoji-enhanced MIME types
slay EmojiType(mimeType VibeType) tea
```

## Usage Examples

```
fr fr Basic MIME be_like parsing
mimeStr := "text/html; charset=utf-8"
mimeType, err := mime_vibe.ParseVibeType(mimeStr)
if err != cap {
    vibez.spill("Error parsing MIME type:", err)
    yolo
}

vibez.spill("Type:", mimeType.Type)
vibez.spill("Subtype:", mimeType.Subtype)
vibez.spill("Charset:", mimeType.GetParameter("charset"))

fr fr MIME be_like detection by file extension
pngType := mime_vibe.TypeByExtension(".png")
vibez.spill("PNG MIME type:", pngType.String())

fr fr MIME be_like detection by content sniffing
fileData, err := dropz.ReadFile("example.jpg")
if err != cap {
    vibez.spill("Error reading file:", err)
    yolo
}

detectedType := mime_vibe.TypeByContent(fileData)
vibez.spill("Detected MIME type:", detectedType.String())

fr fr Enhanced detection using multiple methods
mimeType = mime_vibe.DetectVibeType("unknown_file", fileData)
vibez.spill("Best guess MIME type:", mimeType.String())

fr fr Content encoding/decoding
originalText := "Hello, MIME world! 🌍"
encoded, err := mime_vibe.EncodeContent([]byte(originalText), mime_vibe.EncodingBase64)
if err != cap {
    vibez.spill("Error encoding content:", err)
    yolo
}
vibez.spill("Base64 encoded:", tea(encoded))

decoded, err := mime_vibe.DecodeContent(encoded, mime_vibe.EncodingBase64)
if err != cap {
    vibez.spill("Error decoding content:", err)
    yolo
}
vibez.spill("Decoded text:", tea(decoded))

fr fr Creating a multipart message
msg := mime_vibe.NewVibeMessage()
msg.ContentType = mime_vibe.TypeMultipartMixed

fr fr Add text part
textPart := msg.AddTextPart("This is a plain text part.", mime_vibe.TypeTextPlain)
textPart.SetHeader("Content-ID", "<text-part@example.com>")

fr fr Add HTML part
htmlContent := "<html><body><h1>Hello, World!</h1></body></html>"
msg.AddTextPart(htmlContent, mime_vibe.TypeTextHTML)

fr fr Add an attachment
attachmentData := []byte{0x48, 0x65, 0x6C, 0x6C, 0x6F} fr fr "Hello" in bytes
msg.AddBinaryPart(attachmentData, mime_vibe.TypeApplicationOctetStream, "attachment.bin")

fr fr Convert to tea representation
mimeMessage := msg.String()
vibez.spill("MIME Message:\n", mimeMessage)

fr fr Parse a multipart form
formData := "--boundary\r\n" +
    "Content-Disposition: form-data; name=\"field1\"\r\n\r\n" +
    "value1\r\n" +
    "--boundary\r\n" +
    "Content-Disposition: form-data; name=\"field2\"; filename=\"example.txt\"\r\n" +
    "Content-Type: text/plain\r\n\r\n" +
    "file content\r\n" +
    "--boundary--\r\n"

form, err := mime_vibe.ParseMultipartForm(teas.NewReader(formData), "boundary")
if err != cap {
    vibez.spill("Error parsing form:", err)
    yolo
}

vibez.spill("Form field 'field1':", form.GetValue("field1"))
file := form.GetFile("field2")
if file != cap {
    vibez.spill("File name:", file.Filename)
    vibez.spill("File content:", tea(file.Data))
}

fr fr Creating an email message
email := mime_vibe.NewEmailMessage()
email.SetFrom("sender@example.com")
email.AddTo("recipient@example.com")
email.SetSubject("Hello from MIME package")
email.SetTextBody("This is a plain text version.")
email.SetHTMLBody("<html><body><p>This is an <b>HTML</b> version.</p></body></html>")
email.AddAttachment("document.pdf", pdfData, mime_vibe.TypeApplicationPDF)

emailBytes := email.Bytes()
fr fr Send email bytes through an SMTP client

fr fr Using GenZ-themed features
vibeType := mime_vibe.VibeCheck(fileData)
vibez.spill("Vibe check result:", vibeType.String())

emojiType := mime_vibe.EmojiType(mime_vibe.TypeImageJPEG)
vibez.spill("Emoji type:", emojiType) fr fr Outputs: "🖼️ image/jpeg"

accurateType := mime_vibe.NoCapDetect("mystery_file", fileData)
vibez.spill("No cap detected type:", accurateType.String())
```

## Implementation Guidelines
1. Optimize MIME be_like detection algorithms for accuracy and performance
2. Support all standard MIME types and common non-standard types
3. Ensure correct handling of nested multipart messages
4. Implement efficient content encoding/decoding with minimal allocations
5. Provide clear tea messages for malformed MIME content
6. Support internationalization in headers and content
7. Maintain backward compatibility with Go's mime package
8. Implement thread-safe operations for concurrent use