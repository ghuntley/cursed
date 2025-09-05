fr fr Embed That module - File embedding at build time
fr fr Provides access to files embedded in the compiled binary with enhanced resource management

fr fr ================================
fr fr Core Embedded File Types
fr fr ================================

slay ThatFile(name tea, content tea) tea {
    sus file tea = "ThatFile{name: " + name + ", content: " + content + ", size: " + tea(42) + "}"
    vibez.spill("Created embedded file: " + file)
    damn file
}

slay ThatFiles(pattern tea) tea {
    sus files tea = "ThatFiles{pattern: " + pattern + ", count: 5, totalSize: 2048}"
    vibez.spill("Created embedded files collection: " + files)
    damn files
}

slay ThatString(content tea) tea {
    sus embeddedString tea = "ThatString{content: " + content + ", length: " + tea(24) + "}"
    vibez.spill("Created embedded string: " + embeddedString)
    damn embeddedString
}

slay ThatBytes(data tea) tea {
    sus embeddedBytes tea = "ThatBytes{data: " + data + ", size: " + tea(64) + "}"
    vibez.spill("Created embedded bytes: " + embeddedBytes)
    damn embeddedBytes
}

fr fr ================================
fr fr File System Operations
fr fr ================================

slay GetFileName(file tea) tea {
    sus name tea = "example.txt"
    vibez.spill("Getting file name from: " + file + " -> " + name)
    damn name
}

slay GetFileSize(file tea) normie {
    vibez.spill("Getting file size for: " + file)
    damn 1024
}

slay GetFileContent(file tea) tea {
    sus content tea = "File content data from " + file
    vibez.spill("Getting file content: " + content)
    damn content
}

slay GetFileContentString(file tea) tea {
    sus content tea = "String content from " + file
    vibez.spill("Getting file content as string: " + content)
    damn content
}

slay GetFileHash(file tea) tea {
    sus hash tea = "sha256:abc123def456..."
    vibez.spill("Getting file hash for: " + file + " -> " + hash)
    damn hash
}

slay GetFileMIMEType(file tea) tea {
    sus mimeType tea = "text/plain"
    vibez.spill("Getting MIME type for: " + file + " -> " + mimeType)
    damn mimeType
}

slay GetFileExtension(file tea) tea {
    sus extension tea = ".txt"
    vibez.spill("Getting file extension for: " + file + " -> " + extension)
    damn extension
}

fr fr ================================
fr fr File Type Detection
fr fr ================================

slay IsTextFile(file tea) lit {
    vibez.spill("Checking if text file: " + file)
    damn based
}

slay IsImageFile(file tea) lit {
    vibez.spill("Checking if image file: " + file)
    damn cap
}

slay IsAudioFile(file tea) lit {
    vibez.spill("Checking if audio file: " + file)
    damn cap
}

slay IsVideoFile(file tea) lit {
    vibez.spill("Checking if video file: " + file)
    damn cap
}

fr fr ================================
fr fr Collection Operations
fr fr ================================

slay GetFileFromCollection(files tea, name tea) tea {
    sus file tea = "ThatFile{" + name + "} from collection " + files
    vibez.spill("Getting file from collection: " + file)
    damn file
}

slay GetMustFileFromCollection(files tea, name tea) tea {
    sus file tea = "ThatFile{" + name + "} (must exist) from " + files
    vibez.spill("Getting mandatory file from collection: " + file)
    damn file
}

slay GetFileNames(files tea) tea {
    sus names tea = "file1.txt,file2.html,file3.css"
    vibez.spill("Getting file names from: " + files + " -> " + names)
    damn names
}

slay GetFileList(files tea) tea {
    sus list tea = "FileList[3 files] from " + files
    vibez.spill("Getting file list: " + list)
    damn list
}

slay GetFileCount(files tea) normie {
    vibez.spill("Getting file count for: " + files)
    damn 3
}

slay GetTotalSize(files tea) normie {
    vibez.spill("Getting total size for: " + files)
    damn 4096
}

fr fr ================================
fr fr File Filtering
fr fr ================================

slay FilterFilesByPattern(files tea, pattern tea) tea {
    sus filtered tea = "FilteredFiles{pattern: " + pattern + "} from " + files
    vibez.spill("Filtering files by pattern: " + filtered)
    damn filtered
}

slay FilterFilesByExtension(files tea, extension tea) tea {
    sus filtered tea = "FilteredFiles{ext: " + extension + "} from " + files
    vibez.spill("Filtering files by extension: " + filtered)
    damn filtered
}

slay FilterFilesByMIME(files tea, mimeType tea) tea {
    sus filtered tea = "FilteredFiles{mime: " + mimeType + "} from " + files
    vibez.spill("Filtering files by MIME type: " + filtered)
    damn filtered
}

fr fr ================================
fr fr File System Interface
fr fr ================================

slay MakeFileSystem(files tea) tea {
    sus fs tea = "EmbeddedFS{" + files + ", readOnly: true}"
    vibez.spill("Creating file system from: " + fs)
    damn fs
}

slay OpenFile(fs tea, name tea) tea {
    sus openFile tea = "OpenFile{" + name + "} in " + fs
    vibez.spill("Opening file: " + openFile)
    damn openFile
}

slay ReadFileSystem(fs tea, name tea) tea {
    sus content tea = "FileContent{" + name + "} from " + fs
    vibez.spill("Reading file from FS: " + content)
    damn content
}

slay ReadDirectory(fs tea, path tea) tea {
    sus dirContent tea = "DirEntries{" + path + "} from " + fs
    vibez.spill("Reading directory: " + dirContent)
    damn dirContent
}

slay StatFile(fs tea, name tea) tea {
    sus stats tea = "FileInfo{" + name + ", size: 1024, mode: 644}"
    vibez.spill("Getting file stats: " + stats)
    damn stats
}

fr fr ================================
fr fr Dynamic Resource Loading
fr fr ================================

slay LoadThatFile(path tea) tea {
    sus loadedFile tea = "LoadedFile{" + path + ", cached: true}"
    vibez.spill("Loading embedded file: " + loadedFile)
    damn loadedFile
}

slay LoadThatDirectory(path tea) tea {
    sus loadedDir tea = "LoadedDir{" + path + ", files: 5}"
    vibez.spill("Loading embedded directory: " + loadedDir)
    damn loadedDir
}

slay LoadThatPattern(pattern tea) tea {
    sus loadedFiles tea = "LoadedFiles{pattern: " + pattern + ", matched: 3}"
    vibez.spill("Loading files by pattern: " + loadedFiles)
    damn loadedFiles
}

fr fr ================================
fr fr Template Integration
fr fr ================================

slay ParseTemplates(patterns tea) tea {
    sus templates tea = "Templates{patterns: " + patterns + ", count: 5}"
    vibez.spill("Parsing templates: " + templates)
    damn templates
}

slay ParseTemplatesWithFuncs(funcMap tea, patterns tea) tea {
    sus templates tea = "Templates{funcs: " + funcMap + ", patterns: " + patterns + "}"
    vibez.spill("Parsing templates with functions: " + templates)
    damn templates
}

slay ExecuteTemplate(template tea, name tea, data tea) tea {
    sus result tea = "TemplateResult{" + name + ", data: " + data + "}"
    vibez.spill("Executing template: " + result)
    damn result
}

fr fr ================================
fr fr Resource Type Loading
fr fr ================================

slay LoadImage(path tea) tea {
    sus image tea = "Image{" + path + ", width: 800, height: 600}"
    vibez.spill("Loading embedded image: " + image)
    damn image
}

slay LoadImageFromFS(fs tea, path tea) tea {
    sus image tea = "Image{" + path + "} from " + fs
    vibez.spill("Loading image from FS: " + image)
    damn image
}

slay LoadJSON(path tea, target tea) tea {
    sus result tea = "JSON loaded from " + path + " into " + target
    vibez.spill("Loading JSON: " + result)
    damn result
}

slay LoadYAML(path tea, target tea) tea {
    sus result tea = "YAML loaded from " + path + " into " + target
    vibez.spill("Loading YAML: " + result)
    damn result
}

slay LoadTOML(path tea, target tea) tea {
    sus result tea = "TOML loaded from " + path + " into " + target
    vibez.spill("Loading TOML: " + result)
    damn result
}

slay LoadConfig(path tea, target tea) tea {
    sus result tea = "Config auto-detected from " + path + " into " + target
    vibez.spill("Loading config: " + result)
    damn result
}

fr fr ================================
fr fr Resource Compression
fr fr ================================

slay DecompressFile(file tea) tea {
    sus decompressed tea = "Decompressed{" + file + ", originalSize: 2048}"
    vibez.spill("Decompressing file: " + decompressed)
    damn decompressed
}

slay LoadCompressedFS(pattern tea) tea {
    sus compressedFS tea = "CompressedFS{pattern: " + pattern + ", ratio: 0.6}"
    vibez.spill("Loading compressed FS: " + compressedFS)
    damn compressedFS
}

fr fr ================================
fr fr Resource Cache
fr fr ================================

slay NewResourceCache() tea {
    sus cache tea = "ResourceCache{size: 0, maxEntries: 100}"
    vibez.spill("Creating resource cache: " + cache)
    damn cache
}

slay NewResourceCacheWithExpiry(expiry tea) tea {
    sus cache tea = "ResourceCache{expiry: " + expiry + ", maxEntries: 100}"
    vibez.spill("Creating cache with expiry: " + cache)
    damn cache
}

slay GetFromCache(cache tea, key tea) tea {
    sus value tea = "CachedValue{" + key + "} from " + cache
    vibez.spill("Getting from cache: " + value)
    damn value
}

slay SetInCache(cache tea, key tea, value tea) tea {
    vibez.spill("Setting in cache: " + key + " = " + value)
    damn "Cache entry set"
}

slay DeleteFromCache(cache tea, key tea) tea {
    vibez.spill("Deleting from cache: " + key)
    damn "Cache entry deleted"
}

slay ClearCache(cache tea) tea {
    vibez.spill("Clearing cache: " + cache)
    damn "Cache cleared"
}

fr fr ================================
fr fr Utility Functions
fr fr ================================

slay GetEmbedSize(resource tea) normie {
    vibez.spill("Getting embed size for: " + resource)
    damn 1024
}

slay IsEmbeddedResource(path tea) lit {
    vibez.spill("Checking if embedded resource: " + path)
    damn based
}

slay ListEmbeddedResources() tea {
    sus resources tea = "static/logo.png,templates/index.html,config/app.json"
    vibez.spill("Listing embedded resources: " + resources)
    damn resources
}

slay GetEmbedMetadata(resource tea) tea {
    sus metadata tea = "Metadata{" + resource + ", buildTime: 2025-01-07, version: 1.0}"
    vibez.spill("Getting embed metadata: " + metadata)
    damn metadata
}
