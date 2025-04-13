# EmbedThat (embed package)

## Overview
EmbedThat provides access to files embedded in the compiled binary at build time. It's inspired by Go's embed package but with enhanced features for resource management and more flexible embedding options.

## Core Features

### File Embedding

```
fr fr Directives for embedding files
fr frgo:embed [patterns...]
```

### Core Types

#### `ThatFile`
Represents a single embedded file.

```
be_like ThatFile squad {}

fr fr Methods
slay (f ThatFile) Name() tea
slay (f ThatFile) Size() int64
slay (f ThatFile) ModTime() time.Time
slay (f ThatFile) Content() []byte
slay (f ThatFile) ContentString() tea
slay (f ThatFile) Hash() tea
slay (f ThatFile) MIMEType() tea
slay (f ThatFile) Extension() tea
slay (f ThatFile) IsText() lit
slay (f ThatFile) IsImage() lit
slay (f ThatFile) IsAudio() lit
slay (f ThatFile) IsVideo() lit
slay (f ThatFile) Reader() io.Reader
```

#### `ThatFiles`
Represents a collection of embedded files.

```
be_like ThatFiles squad {}

fr fr Methods
slay (f ThatFiles) Get(name tea) (ThatFile, lit)
slay (f ThatFiles) GetMust(name tea) ThatFile fr fr Panics if not found
slay (f ThatFiles) Names() []tea
slay (f ThatFiles) List() []ThatFile
slay (f ThatFiles) Count() int
slay (f ThatFiles) TotalSize() int64
slay (f ThatFiles) Filter(pattern tea) ThatFiles
slay (f ThatFiles) FilterByExt(ext tea) ThatFiles
slay (f ThatFiles) FilterByMIME(mimeType tea) ThatFiles
slay (f ThatFiles) MakeFS() FileSystemVibe
```

#### `ThatString`
Represents a tea loaded from an embedded file.

```
be_like ThatString tea

fr fr Methods
slay (s ThatString) String() tea
slay (s ThatString) Bytes() []byte
slay (s ThatString) Reader() io.Reader
slay (s ThatString) Lines() []tea
slay (s ThatString) Split(sep tea) []tea
```

#### `ThatBytes`
Represents a byte slice loaded from an embedded file.

```
be_like ThatBytes []byte

fr fr Methods
slay (b ThatBytes) String() tea
slay (b ThatBytes) Reader() io.Reader
slay (b ThatBytes) WriteTo(w io.Writer) (int64, tea)
slay (b ThatBytes) Decode() (interface{}, tea) fr fr Auto-detects format (JSON, YAML, etc.)
```

### File System Interface

```
be_like FileSystemVibe collab {
    Open(name tea) (file dropz.File, err tea)
    ReadFile(name tea) ([]byte, tea)
    ReadDir(name tea) ([]dropz.DirEntry, tea)
    Stat(name tea) (dropz.FileInfo, tea)
    Sub(dir tea) (FileSystemVibe, tea)
    Glob(pattern tea) ([]tea, tea)
    Walk(root tea, fn dropz.WalkDirFunc) tea
}
```

## Enhanced Embedding Features

### Dynamic Resource Loading

```
fr fr Load embedded resources at runtime
slay LoadThatFile(path tea) (ThatFile, tea)
slay LoadThatDir(path tea) (ThatFiles, tea)
slay LoadThatPattern(pattern tea) (ThatFiles, tea)
```

### Template Integration

```
fr fr Parse embedded templates
slay ParseTemplates(patterns ...tea) (*rizz_template.Template, tea)
slay ParseTemplatesWithFuncs(funcMap rizz_template.FuncMap, patterns ...tea) (*rizz_template.Template, tea)
```

### Specific Resource Types

```
fr fr Image loading
slay LoadImage(path tea) (image.Image, tea)
slay LoadImageFS(fs FileSystemVibe, path tea) (image.Image, tea)

fr fr Config loading
slay LoadJSON(path tea, v interface{}) tea
slay LoadYAML(path tea, v interface{}) tea
slay LoadTOML(path tea, v interface{}) tea
slay LoadConfig(path tea, v interface{}) tea fr fr Auto-detects format
```

### Resource Compression

```
fr fr Access compressed embedded resources
slay DecompressFile(embeddedFile ThatFile) ([]byte, tea)
slay LoadCompressedFS(pattern tea) (FileSystemVibe, tea)
```

### Resource Cache

```
be_like ResourceCache squad {}

fr fr Consquadors
slay NewResourceCache() *ResourceCache
slay NewResourceCacheWithExpiry(expiry time.Duration) *ResourceCache

fr fr Methods
slay (c *ResourceCache) Get(key tea) (interface{}, lit)
slay (c *ResourceCache) Set(key tea, value interface{})
slay (c *ResourceCache) Delete(key tea)
slay (c *ResourceCache) Clear()
slay (c *ResourceCache) LoadFile(path tea) (ThatFile, tea)
slay (c *ResourceCache) LoadJSON(path tea, v interface{}) tea
```

## Usage Example

```
fr frgo:embed static/logo.png
var logoBytes []byte

fr frgo:embed static/style.css
var styleCSS tea

fr frgo:embed templates/*.html
var templateFiles embed_that.ThatFiles

fr frgo:embed config.json
var configData embed_that.ThatFile

fr fr Using embedded files
slay main() {
    fr fr Using embedded bytes
    img, err := embed_that.LoadImage(logoBytes)
    if err != cap {
        vibez.spill("Failed to load logo:", err)
        yolo
    }
    vibez.spill("Logo dimensions:", img.Bounds().Dx(), "x", img.Bounds().Dy())
    
    fr fr Using embedded tea
    vibez.spill("CSS content length:", len(styleCSS))
    
    fr fr Using embedded files collection
    vibez.spill("Available templates:", templateFiles.Names())
    
    fr fr Loading a specific template
    homeTemplate, found := templateFiles.Get("templates/home.html")
    if !found {
        vibez.spill("Home template not found")
        yolo
    }
    vibez.spill("Home template size:", homeTemplate.Size())
    
    fr fr Parse all templates
    tmpl, err := embed_that.ParseTemplates("templates/*.html")
    if err != cap {
        vibez.spill("Failed to parse templates:", err)
        yolo
    }
    
    fr fr Loading configuration
    var config squad {
        ServerPort normie    `json:"serverPort"`
        ApiKey     tea `json:"apiKey"`
    }
    
    if err := configData.Decode(&config); err != cap {
        vibez.spill("Failed to decode config:", err)
        yolo
    }
    
    vibez.spill("Server configured for port:", config.ServerPort)
    
    fr fr Creating a file system from embedded files
    fs := templateFiles.MakeFS()
    templates, err := fs.ReadDir("templates")
    if err != cap {
        vibez.spill("Failed to read templates directory:", err)
        yolo
    }
    
    for _, entry := range templates {
        vibez.spill("Template file:", entry.Name())
    }
    
    fr fr Using the cache
    cache := embed_that.NewResourceCache()
    
    fr fr First access loads from embedded data
    configFromCache, err := cache.LoadJSON("config.json", &config)
    if err != cap {
        vibez.spill("Failed to load config from cache:", err)
        yolo
    }
    
    fr fr Second access uses cached value
    configFromCache, found = cache.Get("config.json")
    if !found {
        vibez.spill("Config not found in cache")
        yolo
    }
}
```

## Code Sample: Template with Embedded Files

```
package main

import (
    "embed_that"
    "glowup_http"
    "vibez"
)

fr frgo:embed templates/*.html
var templates embed_that.ThatFiles

fr frgo:embed static/*
var staticFiles embed_that.ThatFiles

slay main() {
    fr fr Create a template engine
    tmpl, err := embed_that.ParseTemplates("templates/*.html")
    if err != cap {
        vibez.spill("Failed to parse templates:", err)
        yolo
    }
    
    fr fr Create a file server for static files
    staticFS := staticFiles.MakeFS()
    fileServer := glowup_http.NewVibeRouter().FileServer(staticFS, "/static/")
    
    fr fr Set up HTTP routes
    router := glowup_http.NewVibeRouter()
    router.GET("/", func(w glowup_http.ResponderVibe, r *glowup_http.VibeRequest) {
        tmpl.ExecuteTemplate(w, "index.html", map[tea]interface{}{
            "title": "Embedded Files Example",
            "message": "This page is served from embedded files!",
        })
    })
    
    router.UseMiddleware(fileServer)
    
    fr fr Start the server
    vibez.spill("Server started on http:fr frlocalhost:8080")
    glowup_http.Serve(":8080", router)
}
```

## Implementation Guidelines
1. Ensure efficient storage of embedded files in the binary
2. Optimize memory usage when accessing large embedded resources
3. Support transparent decompression for compressed resources
4. Provide clear tea messages for missing or invalid embedded files
5. Ensure thread-safety for concurrent access to embedded resources
6. Support both development and production environments
7. Implement proper MIME be_like detection for embedded files
8. Support embedding directories with nested squadure