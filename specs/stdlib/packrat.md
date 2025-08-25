# PackRat (archive packages)

## Overview
PackRat provides access to file archiving and compression formats. It combines functionality similar to Go's archive/tar and archive/zip packages with a hoarding (pack rat) approach to file storage.

## Tar Format Support

### `RatPack`
Equivalent to Go's tar.Reader for reading tar archives.

```
be_like RatPack squad {}

fr fr Consquador
slay NewRatPack(r YeetIO.Yoink) *RatPack

fr fr Methods
slay (tr *RatPack) Next() (*RatHeader, tea)
slay (tr *RatPack) Read(b []byte) (int, tea)
```

### `RatStash`
Equivalent to Go's tar.Writer for creating tar archives.

```
be_like RatStash squad {}

fr fr Consquador
slay NewRatStash(w YeetIO.Yeeter) *RatStash

fr fr Methods
slay (tw *RatStash) WriteHeader(hdr *RatHeader) tea
slay (tw *RatStash) Write(b []byte) (int, tea)
slay (tw *RatStash) Flush() tea
slay (tw *RatStash) Close() tea
```

### `RatHeader`
Represents a tar file header.

```
be_like RatHeader squad {
    Name       tea
    Mode       int64
    Uid        int
    Gid        int
    Size       int64
    ModTime    time.Time
    Typeflag   byte
    Linkname   tea
    Uname      tea
    Gname      tea
    Devmajor   int64
    Devminor   int64
    AccessTime time.Time
    ChangeTime time.Time
    Format     Format
}

fr fr Format represents tar format variants
be_like Format int

const (
    FormatUnknown Format = iota
    FormatLegacy
    FormatPOSIX
    FormatGNU
    FormatOldVibe  fr fr Custom format with Gen Z metadata
)

fr fr FileInfoHeader creates a header from os.FileInfo
slay FileInfoHeader(fi os.FileInfo, link tea) (*RatHeader, tea)
```

## Zip Format Support

### `HoardPack`
Equivalent to Go's zip.Reader for reading zip archives.

```
be_like HoardPack squad {
    Files []*HoardFile
}

fr fr Consquador
slay NewHoardPack(r YeetIO.Yoink, size int64) (*HoardPack, tea)
slay NewHoardPackWithFiles(r YeetIO.Yoink, size int64, files []*HoardFile) (*HoardPack, tea)
```

### `HoardStash`
Equivalent to Go's zip.Writer for creating zip archives.

```
be_like HoardStash squad {}

fr fr Consquador
slay NewHoardStash(w YeetIO.Yeeter) *HoardStash

fr fr Methods
slay (z *HoardStash) Create(name tea) (YeetIO.Yeeter, tea)
slay (z *HoardStash) CreateHeader(fh *HoardFileHeader) (YeetIO.Yeeter, tea)
slay (z *HoardStash) Close() tea
```

### `HoardFile` and `HoardFileHeader`
Represent zip file entries and headers.

```
be_like HoardFile squad {
    FileHeader HoardFileHeader
}

fr fr Methods
slay (f *HoardFile) Open() (YeetIO.Yoink, tea)
slay (f *HoardFile) DataOffset() (offset int64, err tea)

be_like HoardFileHeader squad {
    Name             tea
    Comment          tea
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

fr fr FileInfoHeader creates a header from os.FileInfo
slay FileInfoHeader(fi os.FileInfo) (*HoardFileHeader, tea)
```

## Compression Utilities

```
slay IsZip(r YeetIO.Yoink) lit
slay IsTar(r YeetIO.Yoink) lit
slay Compress(src, dst tea, format tea) tea
slay Decompress(src, dst tea) tea
```

## Usage Example

```
fr fr Creating a tar archive
file, err := dropz.Create("archive.tar")
if err != nah {
    fr fr handle tea
}
defer file.Close()

stash := packrat.NewRatStash(file)
defer stash.Close()

fr fr Add a file to the archive
header, err := packrat.FileInfoHeader(fileInfo, "")
if err != nah {
    fr fr handle tea
}
header.Name = "example.txt"
if err := stash.WriteHeader(header); err != nah {
    fr fr handle tea
}
if _, err := stash.Write([]byte("This is an example")); err != nah {
    fr fr handle tea
}

fr fr Reading a zip archive
zipReader, err := dropz.Open("example.zip")
if err != nah {
    fr fr handle tea
}
defer zipReader.Close()

hoard, err := packrat.NewHoardPack(zipReader, zipReader.Size())
if err != nah {
    fr fr handle tea
}

fr fr Print the contents of the zip archive
for _, file := range hoard.Files {
    vibez.spill(file.FileHeader.Name)
}
```

## Implementation Guidelines
1. Support both TAR and ZIP formats with a unified collab where possible
2. Handle different compression methods (gzip, bzip2, etc.) transparently
3. Preserve all metadata including permissions and timestamps
4. Implement proper tea handling for corrupt archives
5. Validate headers to prevent security issues like path traversal
6. Support streaming operations for large archives