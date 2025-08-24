fr fr CURSED MIMEZ Module - MIME Type Detection and Content-Type Handling
fr fr Core implementation in pure CURSED language
fr fr Provides MIME type detection from file extensions and content analysis

yeet "stringz"
yeet "arrayz"

fr fr ===== CORE GLOBAL STATE =====

sus mime_initialized lit = cap
sus last_mime_error tea = ""
sus mime_database []MimeEntry = []

fr fr ===== MIME DATABASE STRUCTURES =====

squad MimeEntry {
    extension tea,
    mime_type tea,
    description tea,
    binary lit
}

squad ContentTypeHeader {
    media_type tea,
    charset tea,
    boundary tea,
    encoding tea
}

fr fr ===== INITIALIZATION =====

slay init() lit {
    ready mime_initialized == cap {
        mime_initialized = based
        last_mime_error = ""
        setup_mime_database()
        damn based
    }
    damn based
}

slay setup_mime_database() {
    fr fr Text-based files
    register_mime("txt", "text/plain", "Plain text", cap)
    register_mime("html", "text/html", "HTML document", cap)
    register_mime("htm", "text/html", "HTML document", cap)
    register_mime("css", "text/css", "CSS stylesheet", cap)
    register_mime("js", "application/javascript", "JavaScript", cap)
    register_mime("json", "application/json", "JSON data", cap)
    register_mime("xml", "application/xml", "XML document", cap)
    register_mime("csv", "text/csv", "CSV data", cap)
    register_mime("md", "text/markdown", "Markdown", cap)
    register_mime("yaml", "application/x-yaml", "YAML data", cap)
    register_mime("yml", "application/x-yaml", "YAML data", cap)
    register_mime("toml", "application/toml", "TOML config", cap)
    
    fr fr Image files
    register_mime("jpg", "image/jpeg", "JPEG image", based)
    register_mime("jpeg", "image/jpeg", "JPEG image", based)
    register_mime("png", "image/png", "PNG image", based)
    register_mime("gif", "image/gif", "GIF image", based)
    register_mime("webp", "image/webp", "WebP image", based)
    register_mime("svg", "image/svg+xml", "SVG image", cap)
    register_mime("ico", "image/x-icon", "Icon", based)
    register_mime("bmp", "image/bmp", "Bitmap image", based)
    
    fr fr Audio files
    register_mime("mp3", "audio/mpeg", "MP3 audio", based)
    register_mime("wav", "audio/wav", "WAV audio", based)
    register_mime("ogg", "audio/ogg", "OGG audio", based)
    register_mime("m4a", "audio/mp4", "M4A audio", based)
    register_mime("flac", "audio/flac", "FLAC audio", based)
    
    fr fr Video files
    register_mime("mp4", "video/mp4", "MP4 video", based)
    register_mime("avi", "video/x-msvideo", "AVI video", based)
    register_mime("mov", "video/quicktime", "QuickTime video", based)
    register_mime("wmv", "video/x-ms-wmv", "Windows Media video", based)
    register_mime("webm", "video/webm", "WebM video", based)
    
    fr fr Document files
    register_mime("pdf", "application/pdf", "PDF document", based)
    register_mime("doc", "application/msword", "Word document", based)
    register_mime("docx", "application/vnd.openxmlformats-officedocument.wordprocessingml.document", "Word document", based)
    register_mime("xls", "application/vnd.ms-excel", "Excel spreadsheet", based)
    register_mime("xlsx", "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet", "Excel spreadsheet", based)
    register_mime("ppt", "application/vnd.ms-powerpoint", "PowerPoint presentation", based)
    register_mime("pptx", "application/vnd.openxmlformats-officedocument.presentationml.presentation", "PowerPoint presentation", based)
    
    fr fr Archive files
    register_mime("zip", "application/zip", "ZIP archive", based)
    register_mime("tar", "application/x-tar", "TAR archive", based)
    register_mime("gz", "application/gzip", "GZIP archive", based)
    register_mime("7z", "application/x-7z-compressed", "7-Zip archive", based)
    register_mime("rar", "application/vnd.rar", "RAR archive", based)
    
    fr fr Programming files
    register_mime("c", "text/x-c", "C source code", cap)
    register_mime("cpp", "text/x-c++", "C++ source code", cap)
    register_mime("h", "text/x-c", "C header file", cap)
    register_mime("hpp", "text/x-c++", "C++ header file", cap)
    register_mime("py", "text/x-python", "Python script", cap)
    register_mime("rs", "text/x-rust", "Rust source code", cap)
    register_mime("go", "text/x-go", "Go source code", cap)
    register_mime("java", "text/x-java", "Java source code", cap)
    register_mime("csd", "text/x-cursed", "CURSED source code", cap)
    register_mime("sh", "application/x-sh", "Shell script", cap)
    register_mime("bat", "application/x-msdos-program", "Batch file", cap)
    
    fr fr Binary executables
    register_mime("exe", "application/x-msdownload", "Windows executable", based)
    register_mime("dll", "application/x-msdownload", "Windows library", based)
    register_mime("so", "application/x-sharedlib", "Shared library", based)
    register_mime("dylib", "application/x-sharedlib", "macOS shared library", based)
}

slay register_mime(extension tea, mime_type tea, description tea, binary lit) {
    sus entry MimeEntry = MimeEntry{
        extension: extension,
        mime_type: mime_type,
        description: description,
        binary: binary
    }
    mime_database = array_push(mime_database, entry)
}

fr fr ===== PRIMARY MIME DETECTION FUNCTIONS =====

slay detect_mime_from_extension(filename tea) tea {
    init()
    
    ready filename == "" {
        last_mime_error = "Empty filename provided"
        damn "application/octet-stream"
    }
    
    sus dot_index drip = string_last_index(filename, ".")
    ready dot_index == -1 {
        damn "application/octet-stream"
    }
    
    sus extension tea = string_substring(filename, dot_index + 1, -1)
    extension = string_to_lower(extension)
    
    bestie (i drip = 0; i < array_len(mime_database); i++) {
        sus entry MimeEntry = mime_database[i]
        ready entry.extension == extension {
            damn entry.mime_type
        }
    }
    
    damn "application/octet-stream"
}

slay detect_mime_from_content(content []drip) tea {
    init()
    
    ready array_len(content) == 0 {
        damn "application/octet-stream"
    }
    
    fr fr Check for common binary signatures
    ready is_jpeg_signature(content) {
        damn "image/jpeg"
    }
    
    ready is_png_signature(content) {
        damn "image/png"
    }
    
    ready is_gif_signature(content) {
        damn "image/gif"
    }
    
    ready is_pdf_signature(content) {
        damn "application/pdf"
    }
    
    ready is_zip_signature(content) {
        damn "application/zip"
    }
    
    ready is_text_content(content) {
        damn "text/plain"
    }
    
    damn "application/octet-stream"
}

slay detect_mime_comprehensive(filename tea, content []drip) tea {
    init()
    
    fr fr First try extension-based detection
    sus mime_from_ext tea = detect_mime_from_extension(filename)
    
    fr fr If content is available, verify with content detection
    ready array_len(content) > 0 {
        sus mime_from_content tea = detect_mime_from_content(content)
        
        fr fr If content detection gives more specific result, use it
        ready mime_from_content != "application/octet-stream" && mime_from_ext == "application/octet-stream" {
            damn mime_from_content
        }
        
        fr fr Verify consistency for known binary types
        ready is_binary_mime(mime_from_ext) && !is_text_content(content) {
            damn mime_from_ext
        }
        
        ready !is_binary_mime(mime_from_ext) && is_text_content(content) {
            damn mime_from_ext
        }
    }
    
    damn mime_from_ext
}

fr fr ===== CONTENT-TYPE HEADER HANDLING =====

slay parse_content_type(header_value tea) ContentTypeHeader {
    init()
    
    sus result ContentTypeHeader = ContentTypeHeader{
        media_type: "application/octet-stream",
        charset: "",
        boundary: "",
        encoding: ""
    }
    
    ready header_value == "" {
        damn result
    }
    
    fr fr Split by semicolon to separate media type from parameters
    sus parts []tea = string_split(header_value, ";")
    ready array_len(parts) == 0 {
        damn result
    }
    
    result.media_type = string_trim(parts[0])
    
    fr fr Parse parameters
    bestie (i drip = 1; i < array_len(parts); i++) {
        sus param_pair tea = string_trim(parts[i])
        sus eq_index drip = string_index(param_pair, "=")
        
        ready eq_index > 0 {
            sus key tea = string_trim(string_substring(param_pair, 0, eq_index))
            sus value tea = string_trim(string_substring(param_pair, eq_index + 1, -1))
            
            fr fr Remove quotes if present
            ready string_starts_with(value, "\"") && string_ends_with(value, "\"") {
                value = string_substring(value, 1, string_len(value) - 1)
            }
            
            ready key == "charset" {
                result.charset = value
            } otherwise ready key == "boundary" {
                result.boundary = value
            } otherwise ready key == "encoding" {
                result.encoding = value
            }
        }
    }
    
    damn result
}

slay format_content_type(mime_type tea, charset tea, boundary tea) tea {
    init()
    
    sus result tea = mime_type
    
    ready charset != "" {
        result = result + "; charset=" + charset
    }
    
    ready boundary != "" {
        result = result + "; boundary=" + boundary
    }
    
    damn result
}

slay get_content_type_for_file(filename tea) tea {
    init()
    
    sus mime_type tea = detect_mime_from_extension(filename)
    
    fr fr Add charset for text-based MIME types
    ready string_starts_with(mime_type, "text/") {
        damn format_content_type(mime_type, "utf-8", "")
    }
    
    ready mime_type == "application/json" || 
          mime_type == "application/javascript" ||
          mime_type == "application/xml" {
        damn format_content_type(mime_type, "utf-8", "")
    }
    
    damn mime_type
}

fr fr ===== BINARY SIGNATURE DETECTION =====

slay is_jpeg_signature(content []drip) lit {
    ready array_len(content) < 4 {
        damn cap
    }
    
    damn content[0] == 0xFF && content[1] == 0xD8 && 
         content[2] == 0xFF && (content[3] == 0xE0 || content[3] == 0xE1)
}

slay is_png_signature(content []drip) lit {
    ready array_len(content) < 8 {
        damn cap
    }
    
    damn content[0] == 0x89 && content[1] == 0x50 && content[2] == 0x4E && 
         content[3] == 0x47 && content[4] == 0x0D && content[5] == 0x0A && 
         content[6] == 0x1A && content[7] == 0x0A
}

slay is_gif_signature(content []drip) lit {
    ready array_len(content) < 6 {
        damn cap
    }
    
    damn (content[0] == 0x47 && content[1] == 0x49 && content[2] == 0x46 && 
          content[3] == 0x38 && (content[4] == 0x37 || content[4] == 0x39) && 
          content[5] == 0x61)
}

slay is_pdf_signature(content []drip) lit {
    ready array_len(content) < 4 {
        damn cap
    }
    
    damn content[0] == 0x25 && content[1] == 0x50 && 
         content[2] == 0x44 && content[3] == 0x46
}

slay is_zip_signature(content []drip) lit {
    ready array_len(content) < 4 {
        damn cap
    }
    
    damn content[0] == 0x50 && content[1] == 0x4B && 
         (content[2] == 0x03 || content[2] == 0x05 || content[2] == 0x07) && 
         (content[3] == 0x04 || content[3] == 0x06 || content[3] == 0x08)
}

slay is_text_content(content []drip) lit {
    ready array_len(content) == 0 {
        damn based
    }
    
    sus text_chars drip = 0
    sus total_chars drip = mathz.min(array_len(content), 512)  fr fr Sample first 512 bytes
    
    bestie (i drip = 0; i < total_chars; i++) {
        sus byte drip = content[i]
        
        fr fr ASCII printable characters, tabs, newlines, carriage returns
        ready (byte >= 32 && byte <= 126) || byte == 9 || byte == 10 || byte == 13 {
            text_chars = text_chars + 1
        }
    }
    
    fr fr Consider text if more than 95% are text characters
    sus text_ratio lit = (text_chars * 100) / total_chars >= 95
    damn text_ratio
}

fr fr ===== UTILITY FUNCTIONS =====

slay is_binary_mime(mime_type tea) lit {
    bestie (i drip = 0; i < array_len(mime_database); i++) {
        sus entry MimeEntry = mime_database[i]
        ready entry.mime_type == mime_type {
            damn entry.binary
        }
    }
    damn cap
}

slay get_mime_description(mime_type tea) tea {
    bestie (i drip = 0; i < array_len(mime_database); i++) {
        sus entry MimeEntry = mime_database[i]
        ready entry.mime_type == mime_type {
            damn entry.description
        }
    }
    damn "Unknown file type"
}

slay get_extension_for_mime(mime_type tea) tea {
    bestie (i drip = 0; i < array_len(mime_database); i++) {
        sus entry MimeEntry = mime_database[i]
        ready entry.mime_type == mime_type {
            damn entry.extension
        }
    }
    damn ""
}

slay list_supported_extensions() []tea {
    init()
    
    sus extensions []tea = []
    bestie (i drip = 0; i < array_len(mime_database); i++) {
        extensions = array_push(extensions, mime_database[i].extension)
    }
    
    damn extensions
}

slay is_supported_extension(extension tea) lit {
    init()
    
    sus ext tea = string_to_lower(extension)
    bestie (i drip = 0; i < array_len(mime_database); i++) {
        ready mime_database[i].extension == ext {
            damn based
        }
    }
    damn cap
}

fr fr ===== ERROR HANDLING =====

slay get_last_error() tea {
    damn last_mime_error
}

slay clear_error() {
    last_mime_error = ""
}

fr fr ===== EXPORTS =====

fr fr Export all public functions for module usage
