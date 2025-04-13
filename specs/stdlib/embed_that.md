# EmbedThat (embed package)

## Overview
EmbedThat provides access to files embedded in the compiled binary at build time. It's inspired by Go's embed package but with enhanced features for resource management and more flexible embedding options.

## Core Features

### File Embedding

```go
// Directives for embedding files
//go:embed [patterns...]
```

### Core Types

#### `ThatFile`
Represents a single embedded file.

```go
type ThatFile struct {}

// Methods
func (f ThatFile) Name() string
func (f ThatFile) Size() int64
func (f ThatFile) ModTime() time.Time
func (f ThatFile) Content() []byte
func (f ThatFile) ContentString() string
func (f ThatFile) Hash() string
func (f ThatFile) MIMEType() string
func (f ThatFile) Extension() string
func (f ThatFile) IsText() bool
func (f ThatFile) IsImage() bool
func (f ThatFile) IsAudio() bool
func (f ThatFile) IsVideo() bool
func (f ThatFile) Reader() io.Reader
```

#### `ThatFiles`
Represents a collection of embedded files.

```go
type ThatFiles struct {}

// Methods
func (f ThatFiles) Get(name string) (ThatFile, bool)
func (f ThatFiles) GetMust(name string) ThatFile // Panics if not found
func (f ThatFiles) Names() []string
func (f ThatFiles) List() []ThatFile
func (f ThatFiles) Count() int
func (f ThatFiles) TotalSize() int64
func (f ThatFiles) Filter(pattern string) ThatFiles
func (f ThatFiles) FilterByExt(ext string) ThatFiles
func (f ThatFiles) FilterByMIME(mimeType string) ThatFiles
func (f ThatFiles) MakeFS() FileSystemVibe
```

#### `ThatString`
Represents a string loaded from an embedded file.

```go
type ThatString string

// Methods
func (s ThatString) String() string
func (s ThatString) Bytes() []byte
func (s ThatString) Reader() io.Reader
func (s ThatString) Lines() []string
func (s ThatString) Split(sep string) []string
```

#### `ThatBytes`
Represents a byte slice loaded from an embedded file.

```go
type ThatBytes []byte

// Methods
func (b ThatBytes) String() string
func (b ThatBytes) Reader() io.Reader
func (b ThatBytes) WriteTo(w io.Writer) (int64, error)
func (b ThatBytes) Decode() (interface{}, error) // Auto-detects format (JSON, YAML, etc.)
```

### File System Interface

```go
type FileSystemVibe interface {
    Open(name string) (file dropz.File, err error)
    ReadFile(name string) ([]byte, error)
    ReadDir(name string) ([]dropz.DirEntry, error)
    Stat(name string) (dropz.FileInfo, error)
    Sub(dir string) (FileSystemVibe, error)
    Glob(pattern string) ([]string, error)
    Walk(root string, fn dropz.WalkDirFunc) error
}
```

## Enhanced Embedding Features

### Dynamic Resource Loading

```go
// Load embedded resources at runtime
func LoadThatFile(path string) (ThatFile, error)
func LoadThatDir(path string) (ThatFiles, error)
func LoadThatPattern(pattern string) (ThatFiles, error)
```

### Template Integration

```go
// Parse embedded templates
func ParseTemplates(patterns ...string) (*rizz_template.Template, error)
func ParseTemplatesWithFuncs(funcMap rizz_template.FuncMap, patterns ...string) (*rizz_template.Template, error)
```

### Specific Resource Types

```go
// Image loading
func LoadImage(path string) (image.Image, error)
func LoadImageFS(fs FileSystemVibe, path string) (image.Image, error)

// Config loading
func LoadJSON(path string, v interface{}) error
func LoadYAML(path string, v interface{}) error
func LoadTOML(path string, v interface{}) error
func LoadConfig(path string, v interface{}) error // Auto-detects format
```

### Resource Compression

```go
// Access compressed embedded resources
func DecompressFile(embeddedFile ThatFile) ([]byte, error)
func LoadCompressedFS(pattern string) (FileSystemVibe, error)
```

### Resource Cache

```go
type ResourceCache struct {}

// Constructors
func NewResourceCache() *ResourceCache
func NewResourceCacheWithExpiry(expiry time.Duration) *ResourceCache

// Methods
func (c *ResourceCache) Get(key string) (interface{}, bool)
func (c *ResourceCache) Set(key string, value interface{})
func (c *ResourceCache) Delete(key string)
func (c *ResourceCache) Clear()
func (c *ResourceCache) LoadFile(path string) (ThatFile, error)
func (c *ResourceCache) LoadJSON(path string, v interface{}) error
```

## Usage Example

```go
//go:embed static/logo.png
var logoBytes []byte

//go:embed static/style.css
var styleCSS string

//go:embed templates/*.html
var templateFiles embed_that.ThatFiles

//go:embed config.json
var configData embed_that.ThatFile

// Using embedded files
func main() {
    // Using embedded bytes
    img, err := embed_that.LoadImage(logoBytes)
    if err != nil {
        vibez.spill("Failed to load logo:", err)
        return
    }
    vibez.spill("Logo dimensions:", img.Bounds().Dx(), "x", img.Bounds().Dy())
    
    // Using embedded string
    vibez.spill("CSS content length:", len(styleCSS))
    
    // Using embedded files collection
    vibez.spill("Available templates:", templateFiles.Names())
    
    // Loading a specific template
    homeTemplate, found := templateFiles.Get("templates/home.html")
    if !found {
        vibez.spill("Home template not found")
        return
    }
    vibez.spill("Home template size:", homeTemplate.Size())
    
    // Parse all templates
    tmpl, err := embed_that.ParseTemplates("templates/*.html")
    if err != nil {
        vibez.spill("Failed to parse templates:", err)
        return
    }
    
    // Loading configuration
    var config struct {
        ServerPort int    `json:"serverPort"`
        ApiKey     string `json:"apiKey"`
    }
    
    if err := configData.Decode(&config); err != nil {
        vibez.spill("Failed to decode config:", err)
        return
    }
    
    vibez.spill("Server configured for port:", config.ServerPort)
    
    // Creating a file system from embedded files
    fs := templateFiles.MakeFS()
    templates, err := fs.ReadDir("templates")
    if err != nil {
        vibez.spill("Failed to read templates directory:", err)
        return
    }
    
    for _, entry := range templates {
        vibez.spill("Template file:", entry.Name())
    }
    
    // Using the cache
    cache := embed_that.NewResourceCache()
    
    // First access loads from embedded data
    configFromCache, err := cache.LoadJSON("config.json", &config)
    if err != nil {
        vibez.spill("Failed to load config from cache:", err)
        return
    }
    
    // Second access uses cached value
    configFromCache, found = cache.Get("config.json")
    if !found {
        vibez.spill("Config not found in cache")
        return
    }
}
```

## Code Sample: Template with Embedded Files

```go
package main

import (
    "embed_that"
    "glowup_http"
    "vibez"
)

//go:embed templates/*.html
var templates embed_that.ThatFiles

//go:embed static/*
var staticFiles embed_that.ThatFiles

func main() {
    // Create a template engine
    tmpl, err := embed_that.ParseTemplates("templates/*.html")
    if err != nil {
        vibez.spill("Failed to parse templates:", err)
        return
    }
    
    // Create a file server for static files
    staticFS := staticFiles.MakeFS()
    fileServer := glowup_http.NewVibeRouter().FileServer(staticFS, "/static/")
    
    // Set up HTTP routes
    router := glowup_http.NewVibeRouter()
    router.GET("/", func(w glowup_http.ResponderVibe, r *glowup_http.VibeRequest) {
        tmpl.ExecuteTemplate(w, "index.html", map[string]interface{}{
            "title": "Embedded Files Example",
            "message": "This page is served from embedded files!",
        })
    })
    
    router.UseMiddleware(fileServer)
    
    // Start the server
    vibez.spill("Server started on http://localhost:8080")
    glowup_http.Serve(":8080", router)
}
```

## Implementation Guidelines
1. Ensure efficient storage of embedded files in the binary
2. Optimize memory usage when accessing large embedded resources
3. Support transparent decompression for compressed resources
4. Provide clear error messages for missing or invalid embedded files
5. Ensure thread-safety for concurrent access to embedded resources
6. Support both development and production environments
7. Implement proper MIME type detection for embedded files
8. Support embedding directories with nested structure