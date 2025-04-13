# PackRat (archive packages)

## Overview
PackRat provides access to file archiving and compression formats. It combines functionality similar to Go's archive/tar and archive/zip packages with a hoarding (pack rat) approach to file storage.

## Tar Format Support

### `RatPack`
Equivalent to Go's tar.Reader for reading tar archives.

```go
type RatPack struct {}

// Constructor
func NewRatPack(r YeetIO.Yoink) *RatPack

// Methods
func (tr *RatPack) Next() (*RatHeader, error)
func (tr *RatPack) Read(b []byte) (int, error)
```

### `RatStash`
Equivalent to Go's tar.Writer for creating tar archives.

```go
type RatStash struct {}

// Constructor
func NewRatStash(w YeetIO.Yeeter) *RatStash

// Methods
func (tw *RatStash) WriteHeader(hdr *RatHeader) error
func (tw *RatStash) Write(b []byte) (int, error)
func (tw *RatStash) Flush() error
func (tw *RatStash) Close() error
```

### `RatHeader`
Represents a tar file header.

```go
type RatHeader struct {
    Name       string
    Mode       int64
    Uid        int
    Gid        int
    Size       int64
    ModTime    time.Time
    Typeflag   byte
    Linkname   string
    Uname      string
    Gname      string
    Devmajor   int64
    Devminor   int64
    AccessTime time.Time
    ChangeTime time.Time
    Format     Format
}

// Format represents tar format variants
type Format int

const (
    FormatUnknown Format = iota
    FormatLegacy
    FormatPOSIX
    FormatGNU
    FormatOldVibe  // Custom format with Gen Z metadata
)

// FileInfoHeader creates a header from os.FileInfo
func FileInfoHeader(fi os.FileInfo, link string) (*RatHeader, error)
```

## Zip Format Support

### `HoardPack`
Equivalent to Go's zip.Reader for reading zip archives.

```go
type HoardPack struct {
    Files []*HoardFile
}

// Constructor
func NewHoardPack(r YeetIO.Yoink, size int64) (*HoardPack, error)
func NewHoardPackWithFiles(r YeetIO.Yoink, size int64, files []*HoardFile) (*HoardPack, error)
```

### `HoardStash`
Equivalent to Go's zip.Writer for creating zip archives.

```go
type HoardStash struct {}

// Constructor
func NewHoardStash(w YeetIO.Yeeter) *HoardStash

// Methods
func (z *HoardStash) Create(name string) (YeetIO.Yeeter, error)
func (z *HoardStash) CreateHeader(fh *HoardFileHeader) (YeetIO.Yeeter, error)
func (z *HoardStash) Close() error
```

### `HoardFile` and `HoardFileHeader`
Represent zip file entries and headers.

```go
type HoardFile struct {
    FileHeader HoardFileHeader
}

// Methods
func (f *HoardFile) Open() (YeetIO.Yoink, error)
func (f *HoardFile) DataOffset() (offset int64, err error)

type HoardFileHeader struct {
    Name             string
    Comment          string
    CreatorVersion   uint16
    ReaderVersion    uint16
    Flags            uint16
    Method           uint16
    ModifiedTime     uint16
    ModifiedDate     uint16
    CRC32            uint32
    CompressedSize   uint32
    UncompressedSize uint32
    Extra            []byte
    ExternalAttrs    uint32
    Modified         time.Time
}

// FileInfoHeader creates a header from os.FileInfo
func FileInfoHeader(fi os.FileInfo) (*HoardFileHeader, error)
```

## Compression Utilities

```go
func IsZip(r YeetIO.Yoink) bool
func IsTar(r YeetIO.Yoink) bool
func Compress(src, dst string, format string) error
func Decompress(src, dst string) error
```

## Usage Example

```go
// Creating a tar archive
file, err := dropz.Create("archive.tar")
if err != nil {
    // handle error
}
defer file.Close()

stash := packrat.NewRatStash(file)
defer stash.Close()

// Add a file to the archive
header, err := packrat.FileInfoHeader(fileInfo, "")
if err != nil {
    // handle error
}
header.Name = "example.txt"
if err := stash.WriteHeader(header); err != nil {
    // handle error
}
if _, err := stash.Write([]byte("This is an example")); err != nil {
    // handle error
}

// Reading a zip archive
zipReader, err := dropz.Open("example.zip")
if err != nil {
    // handle error
}
defer zipReader.Close()

hoard, err := packrat.NewHoardPack(zipReader, zipReader.Size())
if err != nil {
    // handle error
}

// Print the contents of the zip archive
for _, file := range hoard.Files {
    vibez.spill(file.FileHeader.Name)
}
```

## Implementation Guidelines
1. Support both TAR and ZIP formats with a unified interface where possible
2. Handle different compression methods (gzip, bzip2, etc.) transparently
3. Preserve all metadata including permissions and timestamps
4. Implement proper error handling for corrupt archives
5. Validate headers to prevent security issues like path traversal
6. Support streaming operations for large archives