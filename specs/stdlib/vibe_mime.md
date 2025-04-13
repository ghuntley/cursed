# vibe_mime (mime)

## Overview
The `vibe_mime` module provides functionality for working with MIME types and encodings. It helps with detecting, parsing, and generating MIME-formatted data, particularly for email and HTTP applications.

## Core Types and Interfaces

### MIMEType
Represents a MIME media type with type, subtype, and parameters.

```csd
type MIMEType struct {
  Type string       // Top-level type (e.g., "text")
  Subtype string    // Subtype (e.g., "plain")
  Parameters map[string]string // Parameters (e.g., {"charset": "utf-8"})
}

func (m MIMEType) String() string
func (m MIMEType) IsText() bool
func (m MIMEType) IsHTML() bool
func (m MIMEType) IsJSON() bool
func (m MIMEType) IsXML() bool
func (m MIMEType) WithParameter(name, value string) MIMEType
```

### Part
Represents a part of a multipart MIME message.

```csd
type Part struct {
  Header map[string][]string
  Body []byte
}

func (p *Part) ContentType() string
func (p *Part) ContentDisposition() string
func (p *Part) Filename() string
func (p *Part) SetHeader(key, value string)
```

### MultipartReader
Reads MIME multipart data.

```csd
type MultipartReader struct {
  // fields not directly accessible
}

func (r *MultipartReader) NextPart() (*Part, error)
func (r *MultipartReader) ReadForm(maxMemory int64) (*Form, error)
```

### MultipartWriter
Writes MIME multipart data.

```csd
type MultipartWriter struct {
  // fields not directly accessible
}

func (w *MultipartWriter) CreatePart(header map[string][]string) (io.Writer, error)
func (w *MultipartWriter) WriteField(fieldname, value string) error
func (w *MultipartWriter) CreateFormFile(fieldname, filename string) (io.Writer, error)
func (w *MultipartWriter) Close() error
```

### Form
Represents a parsed multipart form.

```csd
type Form struct {
  Value map[string][]string
  File map[string][]*FileHeader
}

func (f *Form) RemoveAll() error
```

### FileHeader
Contains the metadata for an uploaded file.

```csd
type FileHeader struct {
  Filename string
  Header map[string][]string
  Size int64
}

func (fh *FileHeader) Open() (File, error)
```

## Core Functions

```csd
// Parse a MIME type string into a MIMEType struct
func ParseMediaType(mediaType string) (MIMEType, error)

// Format a media type as a string
func FormatMediaType(mimeType MIMEType) string

// Create a new multipart reader
func NewMultipartReader(reader io.Reader, boundary string) *MultipartReader

// Create a new multipart writer
func NewMultipartWriter(writer io.Writer) *MultipartWriter

// Detect MIME type from file contents
func DetectContentType(data []byte) string

// Get file extension for a MIME type
func ExtensionByType(mimeType string) string

// Get MIME type for a file extension
func TypeByExtension(ext string) string

// Add a MIME type mapping
func AddExtensionType(ext, mimeType string) error
```

## Enhanced Features

- **Content Detection**: Advanced content type detection
  ```csd
  detector := vibe_mime.NewDetector()
  detector.AddSignature("PNG", []byte{0x89, 0x50, 0x4E, 0x47})
  mimeType := detector.Detect(fileData)
  ```

- **MIME Database**: Comprehensive MIME type database
  ```csd
  db := vibe_mime.GetDatabase()
  allImageTypes := db.TypesWithCategory("image")
  ```

- **Mime Stream Processing**: Process MIME streams incrementally
  ```csd
  processor := vibe_mime.NewStreamProcessor(reader)
  for processor.Next() {
    part := processor.Current()
    // Process part
  }
  ```

- **Header Encoding/Decoding**: Support for encoded words in headers
  ```csd
  encoded := vibe_mime.EncodeHeader("Subject", "Привет мир")
  decoded := vibe_mime.DecodeHeader("=?UTF-8?B?0J/RgNC40LLQtdGCINC80LjRgA==?=")
  ```

- **MIME Tree**: Hierarchical representation of MIME structures
  ```csd
  tree := vibe_mime.ParseTree(emailData)
  plainTextBody := tree.GetFirstPartByType("text/plain")
  ```

## Usage Examples

```csd
// Parsing and working with MIME types
mimeType, err := vibe_mime.ParseMediaType("text/html; charset=utf-8")
if err != nil {
  vibez.spill("Parse error: %v", err)
  return
}

vibez.spill("Type: %s, Subtype: %s", mimeType.Type, mimeType.Subtype)
vibez.spill("Charset: %s", mimeType.Parameters["charset"])
vibez.spill("Full string: %s", mimeType.String())

// Check if it's a text type
vibez.spill("Is text: %v", mimeType.IsText())
vibez.spill("Is HTML: %v", mimeType.IsHTML())

// Modify MIME type
mimeTypeWithLang := mimeType.WithParameter("lang", "en")
vibez.spill("With language: %s", mimeTypeWithLang.String())

// Detecting content type from file data
data := []byte("<!DOCTYPE html><html><body>Hello World</body></html>")
contentType := vibe_mime.DetectContentType(data)
vibez.spill("Detected content type: %s", contentType)

// Get extension for a MIME type
ext := vibe_mime.ExtensionByType("image/jpeg")
vibez.spill("Extension for image/jpeg: %s", ext)

// Get MIME type for an extension
mimeType = vibe_mime.TypeByExtension(".pdf")
vibez.spill("MIME type for .pdf: %s", mimeType)

// Creating a multipart form
var buffer bytes_drip.Buffer
multiWriter := vibe_mime.NewMultipartWriter(&buffer)

// Add a text field
err = multiWriter.WriteField("name", "Alice")
if err != nil {
  vibez.spill("Write field error: %v", err)
  return
}

// Add a file
fileWriter, err := multiWriter.CreateFormFile("profile", "profile.jpg")
if err != nil {
  vibez.spill("Create form file error: %v", err)
  return
}

// Simulate file data
fileData := []byte("This would be the file content")
_, err = fileWriter.Write(fileData)
if err != nil {
  vibez.spill("Write file data error: %v", err)
  return
}

// Close the multipart writer
err = multiWriter.Close()
if err != nil {
  vibez.spill("Close error: %v", err)
  return
}

vibez.spill("Generated multipart form:\n%s", buffer.String())

// Reading a multipart form
boundary := multiWriter.Boundary()
reader := vibe_mime.NewMultipartReader(&buffer, boundary)

// Read all parts
for {
  part, err := reader.NextPart()
  if err == dropz.EOF {
    break
  }
  if err != nil {
    vibez.spill("Read part error: %v", err)
    return
  }
  
  // Read the part data
  partData, err := dropz.ReadAll(part)
  if err != nil {
    vibez.spill("Read part data error: %v", err)
    return
  }
  
  vibez.spill("Part name: %s", part.Header.Get("Content-Disposition"))
  vibez.spill("Part data: %s", string(partData))
}

// Processing an uploaded file
var req http_vibez.Request // Assume this is from a handler
maxMemory := int64(10 << 20) // 10 MB
err = req.ParseMultipartForm(maxMemory)
if err != nil {
  vibez.spill("Parse form error: %v", err)
  return
}

file, handler, err := req.FormFile("upload")
if err != nil {
  vibez.spill("Form file error: %v", err)
  return
}
defer file.Close()

vibez.spill("Uploaded File: %+v\n", handler.Filename)
vibez.spill("File Size: %+v\n", handler.Size)
vibez.spill("MIME Header: %+v\n", handler.Header)

// Read the file content
fileBytes, err := dropz.ReadAll(file)
if err != nil {
  vibez.spill("Read file error: %v", err)
  return
}
vibez.spill("File content length: %d bytes", len(fileBytes))

// Create a new local file
dst, err := main_character.Create("uploaded-" + handler.Filename)
if err != nil {
  vibez.spill("Create file error: %v", err)
  return
}
defer dst.Close()

// Copy the uploaded file to the local file
_, err = dst.Write(fileBytes)
if err != nil {
  vibez.spill("Write file error: %v", err)
  return
}

vibez.spill("File saved successfully")
```

## Implementation Guidelines

- Support the full range of MIME types defined in RFC standards
- Implement efficient parsing and generation of MIME content
- Provide robust error handling for malformed MIME data
- Support standard encodings (base64, quoted-printable) for MIME content
- Maintain an extensible database of MIME types and file extensions
- Ensure efficient processing of large multipart data
- Handle character set conversions properly
- Implement memory-efficient multipart form parsing