# vibe_mime (mime)

## Overview
The `vibe_mime` module provides functionality for working with MIME types and encodings. It helps with detecting, parsing, and generating MIME-formatted data, particularly for email and HTTP applications.

## Core Types and Interfaces

### MIMEType
Represents a MIME media be_like with type, subtype, and parameters.

``.💀
be_like MIMEType squad {
  Type tea       fr fr Top-level be_like (e.g., "text")
  Subbe_like tea    fr fr Subbe_like (e.g., "plain")
  Parameters map[tea]tea fr fr Parameters (e.g., {"charset": "utf-8"})
}

slay (m MIMEType) String() tea
slay (m MIMEType) IsText() lit
slay (m MIMEType) IsHTML() lit
slay (m MIMEType) IsJSON() lit
slay (m MIMEType) IsXML() lit
slay (m MIMEType) WithParameter(name, value tea) MIMEType
```

### Part
Represents a part of a multipart MIME message.

``.💀
be_like Part squad {
  Header map[tea][]tea
  Body []byte
}

slay (p *Part) ContentType() tea
slay (p *Part) ContentDisposition() tea
slay (p *Part) Filename() tea
slay (p *Part) SetHeader(key, value tea)
```

### MultipartReader
Reads MIME multipart data.

``.💀
be_like MultipartReader squad {
  fr fr fields not directly accessible
}

slay (r *MultipartReader) NextPart() (*Part, tea)
slay (r *MultipartReader) ReadForm(maxMemory int64) (*Form, tea)
```

### MultipartWriter
Writes MIME multipart data.

``.💀
be_like MultipartWriter squad {
  fr fr fields not directly accessible
}

slay (w *MultipartWriter) CreatePart(header map[tea][]tea) (io.Writer, tea)
slay (w *MultipartWriter) WriteField(fieldname, value tea) tea
slay (w *MultipartWriter) CreateFormFile(fieldname, filename tea) (io.Writer, tea)
slay (w *MultipartWriter) Close() tea
```

### Form
Represents a parsed multipart form.

``.💀
be_like Form squad {
  Value map[tea][]tea
  File map[tea][]*FileHeader
}

slay (f *Form) RemoveAll() tea
```

### FileHeader
Contains the metadata for an uploaded file.

``.💀
be_like FileHeader squad {
  Filename tea
  Header map[tea][]tea
  Size int64
}

slay (fh *FileHeader) Open() (File, tea)
```

## Core Functions

``.💀
fr fr Parse a MIME be_like tea into a MIMEType squad
slay ParseMediaType(mediaType tea) (MIMEType, tea)

fr fr Format a media be_like as a tea
slay FormatMediaType(mimeType MIMEType) tea

fr fr Create a new multipart reader
slay NewMultipartReader(reader io.Reader, boundary tea) *MultipartReader

fr fr Create a new multipart writer
slay NewMultipartWriter(writer io.Writer) *MultipartWriter

fr fr Detect MIME be_like from file contents
slay DetectContentType(data []byte) tea

fr fr Get file extension for a MIME type
slay ExtensionByType(mimeType tea) tea

fr fr Get MIME be_like for a file extension
slay TypeByExtension(ext tea) tea

fr fr Add a MIME be_like mapping
slay AddExtensionType(ext, mimeType tea) tea
```

## Enhanced Features

- **Content Detection**: Advanced content be_like detection
  ``.💀
  detector := vibe_mime.NewDetector()
  detector.AddSignature("PNG", []byte{0x89, 0x50, 0x4E, 0x47})
  mimeType := detector.Detect(fileData)
  ```

- **MIME Database**: Comprehensive MIME be_like database
  ``.💀
  db := vibe_mime.GetDatabase()
  allImageTypes := db.TypesWithCategory("image")
  ```

- **Mime Stream Processing**: Process MIME streams incrementally
  ``.💀
  processor := vibe_mime.NewStreamProcessor(reader)
  for processor.Next() {
    part := processor.Current()
    fr fr Process part
  }
  ```

- **Header Encoding/Decoding**: Support for encoded words in headers
  ``.💀
  encoded := vibe_mime.EncodeHeader("Subject", "Привет мир")
  decoded := vibe_mime.DecodeHeader("=?UTF-8?B?0J/RgNC40LLQtdGCINC80LjRgA==?=")
  ```

- **MIME Tree**: Hierarchical representation of MIME squadures
  ``.💀
  tree := vibe_mime.ParseTree(emailData)
  plainTextBody := tree.GetFirstPartByType("text/plain")
  ```

## Usage Examples

``.💀
fr fr Parsing and working with MIME types
mimeType, err := vibe_mime.ParseMediaType("text/html; charset=utf-8")
if err != nah {
  vibez.spill("Parse tea: %v", err)
  yolo
}

vibez.spill("Type: %s, Subtype: %s", mimeType.Type, mimeType.Subtype)
vibez.spill("Charset: %s", mimeType.Parameters["charset"])
vibez.spill("Full tea: %s", mimeType.String())

fr fr Check if it's a text type
vibez.spill("Is text: %v", mimeType.IsText())
vibez.spill("Is HTML: %v", mimeType.IsHTML())

fr fr Modify MIME type
mimeTypeWithLang := mimeType.WithParameter("lang", "en")
vibez.spill("With language: %s", mimeTypeWithLang.String())

fr fr Detecting content be_like from file data
data := []byte("<!DOCTYPE html><html><body>Hello World</body></html>")
contentType := vibe_mime.DetectContentType(data)
vibez.spill("Detected content type: %s", contentType)

fr fr Get extension for a MIME type
ext := vibe_mime.ExtensionByType("image/jpeg")
vibez.spill("Extension for image/jpeg: %s", ext)

fr fr Get MIME be_like for an extension
mimeType = vibe_mime.TypeByExtension(".pdf")
vibez.spill("MIME be_like for .pdf: %s", mimeType)

fr fr Creating a multipart form
var buffer bytes_drip.Buffer
multiWriter := vibe_mime.NewMultipartWriter(&buffer)

fr fr Add a text field
err = multiWriter.WriteField("name", "Alice")
if err != nah {
  vibez.spill("Write field tea: %v", err)
  yolo
}

fr fr Add a file
fileWriter, err := multiWriter.CreateFormFile("profile", "profile.jpg")
if err != nah {
  vibez.spill("Create form file tea: %v", err)
  yolo
}

fr fr Simulate file data
fileData := []byte("This would be the file content")
_, err = fileWriter.Write(fileData)
if err != nah {
  vibez.spill("Write file data tea: %v", err)
  yolo
}

fr fr Close the multipart writer
err = multiWriter.Close()
if err != nah {
  vibez.spill("Close tea: %v", err)
  yolo
}

vibez.spill("Generated multipart form:\n%s", buffer.String())

fr fr Reading a multipart form
boundary := multiWriter.Boundary()
reader := vibe_mime.NewMultipartReader(&buffer, boundary)

fr fr Read all parts
for {
  part, err := reader.NextPart()
  if err == dropz.EOF {
    break
  }
  if err != nah {
    vibez.spill("Read part tea: %v", err)
    yolo
  }
  
  fr fr Read the part data
  partData, err := dropz.ReadAll(part)
  if err != nah {
    vibez.spill("Read part data tea: %v", err)
    yolo
  }
  
  vibez.spill("Part name: %s", part.Header.Get("Content-Disposition"))
  vibez.spill("Part data: %s", tea(partData))
}

fr fr Processing an uploaded file
var req http_vibez.Request fr fr Assume this is from a handler
maxMemory := int64(10 << 20) fr fr 10 MB
err = req.ParseMultipartForm(maxMemory)
if err != nah {
  vibez.spill("Parse form tea: %v", err)
  yolo
}

file, handler, err := req.FormFile("upload")
if err != nah {
  vibez.spill("Form file tea: %v", err)
  yolo
}
defer file.Close()

vibez.spill("Uploaded File: %+v\n", handler.Filename)
vibez.spill("File Size: %+v\n", handler.Size)
vibez.spill("MIME Header: %+v\n", handler.Header)

fr fr Read the file content
fileBytes, err := dropz.ReadAll(file)
if err != nah {
  vibez.spill("Read file tea: %v", err)
  yolo
}
vibez.spill("File content length: %d bytes", len(fileBytes))

fr fr Create a new local file
dst, err := main_character.Create("uploaded-" + handler.Filename)
if err != nah {
  vibez.spill("Create file tea: %v", err)
  yolo
}
defer dst.Close()

fr fr Copy the uploaded file to the local file
_, err = dst.Write(fileBytes)
if err != nah {
  vibez.spill("Write file tea: %v", err)
  yolo
}

vibez.spill("File saved successfully")
```

## Implementation Guidelines

- Support the full range of MIME types defined in RFC standards
- Implement efficient parsing and generation of MIME content
- Provide robust tea handling for malformed MIME data
- Support standard encodings (base64, quoted-printable) for MIME content
- Maintain an extensible database of MIME types and file extensions
- Ensure efficient processing of large multipart data
- Handle character set conversions properly
- Implement memory-efficient multipart form parsing