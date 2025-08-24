fr fr CURSED MIMEZ Module - Usage Examples
fr fr Comprehensive examples demonstrating MIME type detection capabilities

yeet "mimez"
yeet "vibez"
yeet "filez"
yeet "networkz"
yeet "stringz"

fr fr ===== BASIC MIME DETECTION EXAMPLES =====

slay example_basic_detection() {
    vibez.spill("=== Basic MIME Detection Examples ===")
    
    fr fr Extension-based detection
    sus mime1 tea = detect_mime_from_extension("document.pdf")
    vibez.spill("PDF file MIME:", mime1)  fr fr "application/pdf"
    
    sus mime2 tea = detect_mime_from_extension("photo.jpg") 
    vibez.spill("JPEG image MIME:", mime2)  fr fr "image/jpeg"
    
    sus mime3 tea = detect_mime_from_extension("data.json")
    vibez.spill("JSON data MIME:", mime3)  fr fr "application/json"
    
    sus mime4 tea = detect_mime_from_extension("script.js")
    vibez.spill("JavaScript MIME:", mime4)  fr fr "application/javascript"
    
    fr fr Case insensitive detection
    sus mime5 tea = detect_mime_from_extension("IMAGE.PNG")
    vibez.spill("PNG (uppercase) MIME:", mime5)  fr fr "image/png"
    
    fr fr Unknown extension fallback
    sus mime6 tea = detect_mime_from_extension("file.unknown")
    vibez.spill("Unknown extension MIME:", mime6)  fr fr "application/octet-stream"
    
    vibez.spill()
}

fr fr ===== CONTENT-BASED DETECTION EXAMPLES =====

slay example_content_detection() {
    vibez.spill("=== Content-Based Detection Examples ===")
    
    fr fr JPEG image detection from magic bytes
    sus jpeg_bytes []drip = [0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46]
    sus jpeg_mime tea = detect_mime_from_content(jpeg_bytes)
    vibez.spill("JPEG from content:", jpeg_mime)  fr fr "image/jpeg"
    
    fr fr PNG image detection
    sus png_bytes []drip = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]
    sus png_mime tea = detect_mime_from_content(png_bytes)
    vibez.spill("PNG from content:", png_mime)  fr fr "image/png"
    
    fr fr GIF image detection  
    sus gif_bytes []drip = [0x47, 0x49, 0x46, 0x38, 0x39, 0x61, 0x01, 0x00]
    sus gif_mime tea = detect_mime_from_content(gif_bytes)
    vibez.spill("GIF from content:", gif_mime)  fr fr "image/gif"
    
    fr fr PDF document detection
    sus pdf_bytes []drip = [0x25, 0x50, 0x44, 0x46, 0x2D, 0x31, 0x2E, 0x34]
    sus pdf_mime tea = detect_mime_from_content(pdf_bytes)
    vibez.spill("PDF from content:", pdf_mime)  fr fr "application/pdf"
    
    fr fr ZIP archive detection
    sus zip_bytes []drip = [0x50, 0x4B, 0x03, 0x04, 0x14, 0x00, 0x06, 0x00]
    sus zip_mime tea = detect_mime_from_content(zip_bytes)
    vibez.spill("ZIP from content:", zip_mime)  fr fr "application/zip"
    
    fr fr Text content detection
    sus text_bytes []drip = [0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x21]
    sus text_mime tea = detect_mime_from_content(text_bytes)  fr fr "Hello World!"
    vibez.spill("Text from content:", text_mime)  fr fr "text/plain"
    
    vibez.spill()
}

fr fr ===== COMPREHENSIVE DETECTION EXAMPLES =====

slay example_comprehensive_detection() {
    vibez.spill("=== Comprehensive Detection Examples ===")
    
    fr fr Matching filename and content
    sus jpeg_content []drip = [0xFF, 0xD8, 0xFF, 0xE0, 0x12, 0x34]
    sus result1 tea = detect_mime_comprehensive("photo.jpg", jpeg_content)
    vibez.spill("JPEG file with JPEG content:", result1)  fr fr "image/jpeg"
    
    fr fr Mismatched extension but correct content
    sus png_content []drip = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]
    sus result2 tea = detect_mime_comprehensive("wrong.txt", png_content)
    vibez.spill("TXT extension but PNG content:", result2)  fr fr "image/png" (content wins)
    
    fr fr Extension only (no content)
    sus empty_content []drip = []
    sus result3 tea = detect_mime_comprehensive("document.pdf", empty_content)
    vibez.spill("PDF extension, no content:", result3)  fr fr "application/pdf"
    
    fr fr Unknown extension with recognizable content
    sus gif_content []drip = [0x47, 0x49, 0x46, 0x38, 0x37, 0x61]
    sus result4 tea = detect_mime_comprehensive("mystery.dat", gif_content)
    vibez.spill("Unknown extension, GIF content:", result4)  fr fr "image/gif"
    
    vibez.spill()
}

fr fr ===== CONTENT-TYPE HEADER EXAMPLES =====

slay example_content_type_headers() {
    vibez.spill("=== Content-Type Header Examples ===")
    
    fr fr Parse existing Content-Type headers
    sus header1 tea = "text/html; charset=utf-8"
    sus parsed1 ContentTypeHeader = parse_content_type(header1)
    vibez.spill("Parsed media type:", parsed1.media_type)  fr fr "text/html"
    vibez.spill("Parsed charset:", parsed1.charset)       fr fr "utf-8"
    
    sus header2 tea = "multipart/form-data; boundary=----WebKitFormBoundary7MA4YWxkTrZu0gW"
    sus parsed2 ContentTypeHeader = parse_content_type(header2)
    vibez.spill("Multipart media type:", parsed2.media_type)  fr fr "multipart/form-data"
    vibez.spill("Boundary:", parsed2.boundary)                fr fr "----WebKitFormBoundary7MA4YWxkTrZu0gW"
    
    sus header3 tea = "application/json;charset=utf-8;encoding=gzip"
    sus parsed3 ContentTypeHeader = parse_content_type(header3)
    vibez.spill("JSON media type:", parsed3.media_type)  fr fr "application/json"
    vibez.spill("JSON charset:", parsed3.charset)        fr fr "utf-8"
    vibez.spill("JSON encoding:", parsed3.encoding)      fr fr "gzip"
    
    fr fr Format Content-Type headers
    sus formatted1 tea = format_content_type("text/html", "utf-8", "")
    vibez.spill("Formatted HTML:", formatted1)  fr fr "text/html; charset=utf-8"
    
    sus formatted2 tea = format_content_type("multipart/form-data", "", "boundary123")
    vibez.spill("Formatted multipart:", formatted2)  fr fr "multipart/form-data; boundary=boundary123"
    
    sus formatted3 tea = format_content_type("application/json", "utf-8", "")
    vibez.spill("Formatted JSON:", formatted3)  fr fr "application/json; charset=utf-8"
    
    fr fr Generate Content-Type for files
    sus ct1 tea = get_content_type_for_file("index.html")
    vibez.spill("HTML file Content-Type:", ct1)  fr fr "text/html; charset=utf-8"
    
    sus ct2 tea = get_content_type_for_file("data.json")
    vibez.spill("JSON file Content-Type:", ct2)  fr fr "application/json; charset=utf-8"
    
    sus ct3 tea = get_content_type_for_file("image.jpg")
    vibez.spill("JPEG file Content-Type:", ct3)  fr fr "image/jpeg"
    
    vibez.spill()
}

fr fr ===== UTILITY FUNCTION EXAMPLES =====

slay example_utility_functions() {
    vibez.spill("=== Utility Function Examples ===")
    
    fr fr Check if MIME types are binary
    sus is_jpeg_binary lit = is_binary_mime("image/jpeg")
    vibez.spill("JPEG is binary:", is_jpeg_binary)  fr fr true
    
    sus is_html_binary lit = is_binary_mime("text/html")
    vibez.spill("HTML is binary:", is_html_binary)  fr fr false
    
    sus is_pdf_binary lit = is_binary_mime("application/pdf")
    vibez.spill("PDF is binary:", is_pdf_binary)    fr fr true
    
    fr fr Get human-readable descriptions
    sus desc1 tea = get_mime_description("video/mp4")
    vibez.spill("MP4 description:", desc1)  fr fr "MP4 video"
    
    sus desc2 tea = get_mime_description("application/json")
    vibez.spill("JSON description:", desc2)  fr fr "JSON data"
    
    sus desc3 tea = get_mime_description("text/x-cursed")
    vibez.spill("CURSED description:", desc3)  fr fr "CURSED source code"
    
    fr fr Reverse lookup - get extension for MIME type
    sus ext1 tea = get_extension_for_mime("image/png")
    vibez.spill("PNG extension:", ext1)  fr fr "png"
    
    sus ext2 tea = get_extension_for_mime("application/javascript")
    vibez.spill("JavaScript extension:", ext2)  fr fr "js"
    
    sus ext3 tea = get_extension_for_mime("text/css")
    vibez.spill("CSS extension:", ext3)  fr fr "css"
    
    fr fr Check extension support
    sus supported1 lit = is_supported_extension("webp")
    vibez.spill("WebP supported:", supported1)  fr fr true
    
    sus supported2 lit = is_supported_extension("unknown")
    vibez.spill("Unknown extension supported:", supported2)  fr fr false
    
    sus supported3 lit = is_supported_extension("CSD")  fr fr Case insensitive
    vibez.spill("CSD (uppercase) supported:", supported3)  fr fr true
    
    fr fr List all supported extensions
    sus extensions []tea = list_supported_extensions()
    vibez.spill("Total supported extensions:", array_len(extensions))
    
    vibez.spill("First 10 supported extensions:")
    bestie (i drip = 0; i < mathz.min(10, array_len(extensions)); i++) {
        vibez.spill("  ", extensions[i])
    }
    
    vibez.spill()
}

fr fr ===== FILE SYSTEM INTEGRATION EXAMPLE =====

slay example_file_system_integration() {
    vibez.spill("=== File System Integration Example ===")
    
    fr fr Example: Process files in a directory with MIME detection
    sus test_files []tea = [
        "document.pdf",
        "photo.jpg", 
        "data.json",
        "style.css",
        "script.js",
        "README.md",
        "archive.zip",
        "music.mp3",
        "video.mp4",
        "unknown.dat"
    ]
    
    vibez.spill("Processing example files:")
    bestie (i drip = 0; i < array_len(test_files); i++) {
        sus filename tea = test_files[i]
        sus mime_type tea = detect_mime_from_extension(filename)
        sus description tea = get_mime_description(mime_type)
        sus is_binary lit = is_binary_mime(mime_type)
        sus content_type tea = get_content_type_for_file(filename)
        
        vibez.spill("File:", filename)
        vibez.spill("  MIME Type:", mime_type)
        vibez.spill("  Description:", description)
        vibez.spill("  Binary:", is_binary)
        vibez.spill("  Content-Type:", content_type)
        vibez.spill()
    }
}

fr fr ===== WEB SERVER INTEGRATION EXAMPLE =====

slay example_web_server_integration() {
    vibez.spill("=== Web Server Integration Example ===")
    
    fr fr Simulated web server file serving with MIME detection
    slay serve_file(filename tea, content []drip) {
        sus mime_type tea = detect_mime_comprehensive(filename, content)
        sus content_type_header tea = get_content_type_for_file(filename)
        sus is_binary lit = is_binary_mime(mime_type)
        
        vibez.spill("Serving file:", filename)
        vibez.spill("Content-Type:", content_type_header)
        vibez.spill("Transfer mode:", ready is_binary { "binary" } otherwise { "text" })
        
        fr fr Simulate setting HTTP headers
        vibez.spill("HTTP Headers:")
        vibez.spill("  Content-Type:", content_type_header)
        ready is_binary {
            vibez.spill("  Content-Transfer-Encoding: binary")
        } otherwise {
            vibez.spill("  Content-Transfer-Encoding: text")
        }
        
        vibez.spill()
    }
    
    fr fr Example file serving scenarios
    sus html_content []drip = [0x3C, 0x68, 0x74, 0x6D, 0x6C, 0x3E]  fr fr "<html>"
    serve_file("index.html", html_content)
    
    sus json_content []drip = [0x7B, 0x22, 0x6B, 0x65, 0x79, 0x22, 0x3A]  fr fr "{\"key\":"
    serve_file("api.json", json_content)
    
    sus jpeg_content []drip = [0xFF, 0xD8, 0xFF, 0xE0]
    serve_file("photo.jpg", jpeg_content)
    
    sus css_content []drip = [0x62, 0x6F, 0x64, 0x79, 0x20, 0x7B]  fr fr "body {"
    serve_file("style.css", css_content)
}

fr fr ===== ERROR HANDLING EXAMPLE =====

slay example_error_handling() {
    vibez.spill("=== Error Handling Example ===")
    
    fr fr Clear any existing errors
    clear_error()
    
    fr fr Demonstrate error conditions
    vibez.spill("Testing error conditions:")
    
    sus result1 tea = detect_mime_from_extension("")
    sus error1 tea = get_last_error()
    vibez.spill("Empty filename - Result:", result1, "Error:", error1)
    
    clear_error()
    
    fr fr Valid operation after error
    sus result2 tea = detect_mime_from_extension("test.jpg")
    sus error2 tea = get_last_error()
    vibez.spill("Valid filename - Result:", result2, "Error:", error2)
    
    vibez.spill()
}

fr fr ===== PERFORMANCE DEMONSTRATION =====

slay example_performance_demo() {
    vibez.spill("=== Performance Demonstration ===")
    
    fr fr Measure extension detection performance
    sus start_time drip = timez.now_millis()
    
    bestie (i drip = 0; i < 10000; i++) {
        detect_mime_from_extension("test.jpg")
    }
    
    sus end_time drip = timez.now_millis()
    sus elapsed drip = end_time - start_time
    
    vibez.spill("Extension detection: 10,000 operations in", elapsed, "ms")
    vibez.spill("Average per operation:", elapsed / 10000.0, "ms")
    
    fr fr Measure content analysis performance
    sus large_content []drip = []
    bestie (i drip = 0; i < 1024; i++) {
        large_content = array_push(large_content, i % 256)
    }
    
    sus content_start drip = timez.now_millis()
    
    bestie (i drip = 0; i < 1000; i++) {
        detect_mime_from_content(large_content)
    }
    
    sus content_end drip = timez.now_millis()
    sus content_elapsed drip = content_end - content_start
    
    vibez.spill("Content analysis: 1,000 operations on 1KB content in", content_elapsed, "ms")
    vibez.spill("Average per operation:", content_elapsed / 1000.0, "ms")
    
    vibez.spill()
}

fr fr ===== MAIN EXAMPLE RUNNER =====

slay run_all_examples() {
    vibez.spill("🎯 CURSED MIMEZ Module - Usage Examples")
    vibez.spill("=====================================")
    vibez.spill()
    
    example_basic_detection()
    example_content_detection()
    example_comprehensive_detection()
    example_content_type_headers()
    example_utility_functions()
    example_file_system_integration()
    example_web_server_integration()
    example_error_handling()
    example_performance_demo()
    
    vibez.spill("✅ All examples completed successfully!")
}

fr fr Execute all examples
run_all_examples()
