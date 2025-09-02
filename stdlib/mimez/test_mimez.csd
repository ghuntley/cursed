fr fr CURSED MIMEZ Module - Comprehensive Test Suite
fr fr Tests for MIME type detection, content analysis, and Content-Type headers

yeet "mimez"
yeet "testz"
yeet "vibez"
yeet "stringz"
yeet "arrayz"

fr fr ===== TEST INITIALIZATION =====

slay run_mimez_tests() {
    vibez.spill("🧪 Running MIMEZ Module Tests...")
    
    test_mime_extension_detection()
    test_binary_signature_detection()
    test_content_type_parsing()
    test_content_type_formatting()
    test_comprehensive_detection()
    test_utility_functions()
    test_error_handling()
    test_edge_cases()
    test_performance_scenarios()
    
    print_test_summary()
}

fr fr ===== EXTENSION-BASED DETECTION TESTS =====

slay test_mime_extension_detection() {
    test_start("MIME Extension Detection")
    
    fr fr Text files
    assert_eq_string(detect_mime_from_extension("document.txt"), "text/plain")
    assert_eq_string(detect_mime_from_extension("index.html"), "text/html")
    assert_eq_string(detect_mime_from_extension("page.htm"), "text/html")
    assert_eq_string(detect_mime_from_extension("style.css"), "text/css")
    assert_eq_string(detect_mime_from_extension("script.js"), "application/javascript")
    assert_eq_string(detect_mime_from_extension("data.json"), "application/json")
    assert_eq_string(detect_mime_from_extension("config.xml"), "application/xml")
    assert_eq_string(detect_mime_from_extension("data.csv"), "text/csv")
    assert_eq_string(detect_mime_from_extension("README.md"), "text/markdown")
    assert_eq_string(detect_mime_from_extension("config.yaml"), "application/x-yaml")
    assert_eq_string(detect_mime_from_extension("config.yml"), "application/x-yaml")
    assert_eq_string(detect_mime_from_extension("app.toml"), "application/toml")
    
    fr fr Image files
    assert_eq_string(detect_mime_from_extension("photo.jpg"), "image/jpeg")
    assert_eq_string(detect_mime_from_extension("image.jpeg"), "image/jpeg")
    assert_eq_string(detect_mime_from_extension("icon.png"), "image/png")
    assert_eq_string(detect_mime_from_extension("animation.gif"), "image/gif")
    assert_eq_string(detect_mime_from_extension("modern.webp"), "image/webp")
    assert_eq_string(detect_mime_from_extension("vector.svg"), "image/svg+xml")
    assert_eq_string(detect_mime_from_extension("favicon.ico"), "image/x-icon")
    assert_eq_string(detect_mime_from_extension("bitmap.bmp"), "image/bmp")
    
    fr fr Audio files
    assert_eq_string(detect_mime_from_extension("song.mp3"), "audio/mpeg")
    assert_eq_string(detect_mime_from_extension("sound.wav"), "audio/wav")
    assert_eq_string(detect_mime_from_extension("music.ogg"), "audio/ogg")
    assert_eq_string(detect_mime_from_extension("track.m4a"), "audio/mp4")
    assert_eq_string(detect_mime_from_extension("lossless.flac"), "audio/flac")
    
    fr fr Video files
    assert_eq_string(detect_mime_from_extension("movie.mp4"), "video/mp4")
    assert_eq_string(detect_mime_from_extension("video.avi"), "video/x-msvideo")
    assert_eq_string(detect_mime_from_extension("clip.mov"), "video/quicktime")
    assert_eq_string(detect_mime_from_extension("presentation.wmv"), "video/x-ms-wmv")
    assert_eq_string(detect_mime_from_extension("stream.webm"), "video/webm")
    
    fr fr Programming files
    assert_eq_string(detect_mime_from_extension("program.c"), "text/x-c")
    assert_eq_string(detect_mime_from_extension("class.cpp"), "text/x-c++")
    assert_eq_string(detect_mime_from_extension("header.h"), "text/x-c")
    assert_eq_string(detect_mime_from_extension("header.hpp"), "text/x-c++")
    assert_eq_string(detect_mime_from_extension("script.py"), "text/x-python")
    assert_eq_string(detect_mime_from_extension("main.rs"), "text/x-rust")
    assert_eq_string(detect_mime_from_extension("server.go"), "text/x-go")
    assert_eq_string(detect_mime_from_extension("App.java"), "text/x-java")
    assert_eq_string(detect_mime_from_extension("program.csd"), "text/x-cursed")
    assert_eq_string(detect_mime_from_extension("build.sh"), "application/x-sh")
    
    fr fr Case insensitivity
    assert_eq_string(detect_mime_from_extension("FILE.JPG"), "image/jpeg")
    assert_eq_string(detect_mime_from_extension("Document.PDF"), "application/pdf")
    assert_eq_string(detect_mime_from_extension("Archive.ZIP"), "application/zip")
    
    fr fr Unknown extensions
    assert_eq_string(detect_mime_from_extension("file.unknown"), "application/octet-stream")
    assert_eq_string(detect_mime_from_extension("noextension"), "application/octet-stream")
    
    vibez.spill("✅ Extension detection tests passed")
}

fr fr ===== BINARY SIGNATURE DETECTION TESTS =====

slay test_binary_signature_detection() {
    test_start("Binary Signature Detection")
    
    fr fr JPEG signature (FFD8FF)
    sus jpeg_bytes drip[value] = [0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10]
    assert_eq_string(detect_mime_from_content(jpeg_bytes), "image/jpeg")
    assert_eq_bool(is_jpeg_signature(jpeg_bytes), based)
    
    sus jpeg_bytes2 drip[value] = [0xFF, 0xD8, 0xFF, 0xE1, 0x12, 0x34]  
    assert_eq_string(detect_mime_from_content(jpeg_bytes2), "image/jpeg")
    assert_eq_bool(is_jpeg_signature(jpeg_bytes2), based)
    
    fr fr PNG signature (89504E47)
    sus png_bytes drip[value] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]
    assert_eq_string(detect_mime_from_content(png_bytes), "image/png")
    assert_eq_bool(is_png_signature(png_bytes), based)
    
    fr fr GIF87a signature
    sus gif87_bytes drip[value] = [0x47, 0x49, 0x46, 0x38, 0x37, 0x61]
    assert_eq_string(detect_mime_from_content(gif87_bytes), "image/gif")
    assert_eq_bool(is_gif_signature(gif87_bytes), based)
    
    fr fr GIF89a signature  
    sus gif89_bytes drip[value] = [0x47, 0x49, 0x46, 0x38, 0x39, 0x61]
    assert_eq_string(detect_mime_from_content(gif89_bytes), "image/gif")
    assert_eq_bool(is_gif_signature(gif89_bytes), based)
    
    fr fr PDF signature (%PDF)
    sus pdf_bytes drip[value] = [0x25, 0x50, 0x44, 0x46, 0x2D, 0x31, 0x2E, 0x34]
    assert_eq_string(detect_mime_from_content(pdf_bytes), "application/pdf")
    assert_eq_bool(is_pdf_signature(pdf_bytes), based)
    
    fr fr ZIP signature variants
    sus zip_bytes1 drip[value] = [0x50, 0x4B, 0x03, 0x04]
    assert_eq_string(detect_mime_from_content(zip_bytes1), "application/zip")
    assert_eq_bool(is_zip_signature(zip_bytes1), based)
    
    sus zip_bytes2 drip[value] = [0x50, 0x4B, 0x05, 0x06]
    assert_eq_string(detect_mime_from_content(zip_bytes2), "application/zip")
    assert_eq_bool(is_zip_signature(zip_bytes2), based)
    
    fr fr Text content detection
    sus text_bytes drip[value] = [0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64]  fr fr "Hello World"
    assert_eq_string(detect_mime_from_content(text_bytes), "text/plain")
    assert_eq_bool(is_text_content(text_bytes), based)
    
    fr fr Invalid/insufficient signatures
    sus short_bytes drip[value] = [0xFF, 0xD8]  fr fr Too short for JPEG
    assert_eq_bool(is_jpeg_signature(short_bytes), cap)
    
    sus wrong_signature drip[value] = [0x12, 0x34, 0x56, 0x78]
    assert_eq_string(detect_mime_from_content(wrong_signature), "application/octet-stream")
    
    vibez.spill("✅ Binary signature detection tests passed")
}

fr fr ===== CONTENT-TYPE PARSING TESTS =====

slay test_content_type_parsing() {
    test_start("Content-Type Header Parsing")
    
    fr fr Basic media type only
    sus basic ContentTypeHeader = parse_content_type("text/html")
    assert_eq_string(basic.media_type, "text/html")
    assert_eq_string(basic.charset, "")
    assert_eq_string(basic.boundary, "")
    assert_eq_string(basic.encoding, "")
    
    fr fr With charset parameter
    sus with_charset ContentTypeHeader = parse_content_type("text/html; charset=utf-8")
    assert_eq_string(with_charset.media_type, "text/html")
    assert_eq_string(with_charset.charset, "utf-8")
    assert_eq_string(with_charset.boundary, "")
    
    fr fr With multiple parameters
    sus multi_param ContentTypeHeader = parse_content_type("multipart/form-data; charset=utf-8; boundary=something123")
    assert_eq_string(multi_param.media_type, "multipart/form-data")
    assert_eq_string(multi_param.charset, "utf-8")
    assert_eq_string(multi_param.boundary, "something123")
    
    fr fr With quoted values
    sus quoted ContentTypeHeader = parse_content_type("text/plain; charset=\"iso-8859-1\"")
    assert_eq_string(quoted.media_type, "text/plain")
    assert_eq_string(quoted.charset, "iso-8859-1")
    
    fr fr With spacing variations
    sus spaced ContentTypeHeader = parse_content_type("application/json;charset=utf-8;encoding=gzip")
    assert_eq_string(spaced.media_type, "application/json")
    assert_eq_string(spaced.charset, "utf-8")
    assert_eq_string(spaced.encoding, "gzip")
    
    fr fr Empty header
    sus empty ContentTypeHeader = parse_content_type("")
    assert_eq_string(empty.media_type, "application/octet-stream")
    assert_eq_string(empty.charset, "")
    
    fr fr Malformed header
    sus malformed ContentTypeHeader = parse_content_type("text/html; invalid_param_no_equals")
    assert_eq_string(malformed.media_type, "text/html")
    assert_eq_string(malformed.charset, "")
    
    vibez.spill("✅ Content-Type parsing tests passed")
}

fr fr ===== CONTENT-TYPE FORMATTING TESTS =====

slay test_content_type_formatting() {
    test_start("Content-Type Header Formatting")
    
    fr fr Basic MIME type only
    sus basic_format tea = format_content_type("text/html", "", "")
    assert_eq_string(basic_format, "text/html")
    
    fr fr With charset
    sus with_charset tea = format_content_type("text/html", "utf-8", "")
    assert_eq_string(with_charset, "text/html; charset=utf-8")
    
    fr fr With boundary
    sus with_boundary tea = format_content_type("multipart/form-data", "", "boundary123")
    assert_eq_string(with_boundary, "multipart/form-data; boundary=boundary123")
    
    fr fr With both charset and boundary
    sus full_format tea = format_content_type("multipart/form-data", "utf-8", "boundary123")
    assert_eq_string(full_format, "multipart/form-data; charset=utf-8; boundary=boundary123")
    
    fr fr File-based Content-Type generation
    sus json_ct tea = get_content_type_for_file("data.json")
    assert_eq_string(json_ct, "application/json; charset=utf-8")
    
    sus html_ct tea = get_content_type_for_file("index.html")
    assert_eq_string(html_ct, "text/html; charset=utf-8")
    
    sus js_ct tea = get_content_type_for_file("script.js")
    assert_eq_string(js_ct, "application/javascript; charset=utf-8")
    
    sus xml_ct tea = get_content_type_for_file("config.xml")
    assert_eq_string(xml_ct, "application/xml; charset=utf-8")
    
    sus binary_ct tea = get_content_type_for_file("photo.jpg")
    assert_eq_string(binary_ct, "image/jpeg")
    
    vibez.spill("✅ Content-Type formatting tests passed")
}

fr fr ===== COMPREHENSIVE DETECTION TESTS =====

slay test_comprehensive_detection() {
    test_start("Comprehensive MIME Detection")
    
    fr fr Matching extension and content
    sus jpeg_content drip[value] = [0xFF, 0xD8, 0xFF, 0xE0, 0x12, 0x34]
    sus jpeg_result tea = detect_mime_comprehensive("photo.jpg", jpeg_content)
    assert_eq_string(jpeg_result, "image/jpeg")
    
    fr fr Extension says one thing, content says another (content wins)
    sus png_content drip[value] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]
    sus mismatch_result tea = detect_mime_comprehensive("wrong.txt", png_content)
    assert_eq_string(mismatch_result, "image/png")
    
    fr fr No content provided (extension-based)
    sus empty_content drip[value] = []
    sus ext_only tea = detect_mime_comprehensive("document.pdf", empty_content)
    assert_eq_string(ext_only, "application/pdf")
    
    fr fr Unknown extension with recognizable content
    sus gif_content drip[value] = [0x47, 0x49, 0x46, 0x38, 0x37, 0x61]
    sus unknown_ext tea = detect_mime_comprehensive("file.unknown", gif_content)
    assert_eq_string(unknown_ext, "image/gif")
    
    fr fr Text content with correct extension
    sus text_content drip[value] = [0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64]
    sus text_result tea = detect_mime_comprehensive("hello.txt", text_content)
    assert_eq_string(text_result, "text/plain")
    
    fr fr Binary content with binary extension
    sus pdf_content drip[value] = [0x25, 0x50, 0x44, 0x46, 0x2D, 0x31, 0x2E, 0x34]
    sus pdf_result tea = detect_mime_comprehensive("document.pdf", pdf_content)
    assert_eq_string(pdf_result, "application/pdf")
    
    vibez.spill("✅ Comprehensive detection tests passed")
}

fr fr ===== UTILITY FUNCTION TESTS =====

slay test_utility_functions() {
    test_start("Utility Functions")
    
    fr fr Binary MIME type checking
    assert_eq_bool(is_binary_mime("image/jpeg"), based)
    assert_eq_bool(is_binary_mime("video/mp4"), based)
    assert_eq_bool(is_binary_mime("application/pdf"), based)
    assert_eq_bool(is_binary_mime("application/zip"), based)
    
    assert_eq_bool(is_binary_mime("text/plain"), cap)
    assert_eq_bool(is_binary_mime("text/html"), cap)
    assert_eq_bool(is_binary_mime("application/json"), cap)
    assert_eq_bool(is_binary_mime("text/x-cursed"), cap)
    
    fr fr MIME descriptions
    assert_eq_string(get_mime_description("image/jpeg"), "JPEG image")
    assert_eq_string(get_mime_description("video/mp4"), "MP4 video")
    assert_eq_string(get_mime_description("text/plain"), "Plain text")
    assert_eq_string(get_mime_description("application/json"), "JSON data")
    assert_eq_string(get_mime_description("unknown/type"), "Unknown file type")
    
    fr fr Extension for MIME type (reverse lookup)
    assert_eq_string(get_extension_for_mime("image/jpeg"), "jpg")
    assert_eq_string(get_extension_for_mime("text/html"), "html")
    assert_eq_string(get_extension_for_mime("application/json"), "json")
    assert_eq_string(get_extension_for_mime("text/x-cursed"), "csd")
    assert_eq_string(get_extension_for_mime("unknown/type"), "")
    
    fr fr Extension support checking
    assert_eq_bool(is_supported_extension("jpg"), based)
    assert_eq_bool(is_supported_extension("html"), based)
    assert_eq_bool(is_supported_extension("csd"), based)
    assert_eq_bool(is_supported_extension("unknown"), cap)
    assert_eq_bool(is_supported_extension("JPG"), based)  fr fr Case insensitive
    
    fr fr List supported extensions
    sus extensions tea[value] = list_supported_extensions()
    assert_greater_than_int(array_len(extensions), 50)  fr fr Should have 50+ extensions
    
    fr fr Verify some key extensions are included
    sus found_jpg lit = cap
    sus found_html lit = cap
    sus found_csd lit = cap
    
    bestie (i drip = 0; i < array_len(extensions); i++) {
        ready extensions[i] == "jpg" { found_jpg = based }
        ready extensions[i] == "html" { found_html = based }
        ready extensions[i] == "csd" { found_csd = based }
    }
    
    assert_eq_bool(found_jpg, based)
    assert_eq_bool(found_html, based) 
    assert_eq_bool(found_csd, based)
    
    vibez.spill("✅ Utility function tests passed")
}

fr fr ===== ERROR HANDLING TESTS =====

slay test_error_handling() {
    test_start("Error Handling")
    
    fr fr Clear any existing errors
    clear_error()
    assert_eq_string(get_last_error(), "")
    
    fr fr Test empty filename error
    sus result tea = detect_mime_from_extension("")
    sus error tea = get_last_error()
    assert_eq_string(result, "application/octet-stream")
    assert_not_equal_string(error, "")
    
    fr fr Clear error and verify
    clear_error()
    assert_eq_string(get_last_error(), "")
    
    fr fr Test with valid input after error
    sus valid_result tea = detect_mime_from_extension("test.jpg")
    assert_eq_string(valid_result, "image/jpeg")
    assert_eq_string(get_last_error(), "")  fr fr Should be no error
    
    vibez.spill("✅ Error handling tests passed")
}

fr fr ===== EDGE CASE TESTS =====

slay test_edge_cases() {
    test_start("Edge Cases")
    
    fr fr Empty arrays and strings
    sus empty_content drip[value] = []
    assert_eq_string(detect_mime_from_content(empty_content), "application/octet-stream")
    
    fr fr Very short content
    sus short_content drip[value] = [0x12]
    assert_eq_string(detect_mime_from_content(short_content), "application/octet-stream")
    
    fr fr Filenames with multiple extensions
    assert_eq_string(detect_mime_from_extension("file.tar.gz"), "application/gzip")
    assert_eq_string(detect_mime_from_extension("backup.sql.zip"), "application/zip")
    
    fr fr Filenames with no extension
    assert_eq_string(detect_mime_from_extension("README"), "application/octet-stream")
    assert_eq_string(detect_mime_from_extension("Makefile"), "application/octet-stream")
    
    fr fr Filenames starting with dot
    assert_eq_string(detect_mime_from_extension(".gitignore"), "application/octet-stream")
    assert_eq_string(detect_mime_from_extension(".htaccess"), "application/octet-stream")
    
    fr fr Mixed case extensions
    assert_eq_string(detect_mime_from_extension("File.HTML"), "text/html")
    assert_eq_string(detect_mime_from_extension("IMAGE.Png"), "image/png")
    assert_eq_string(detect_mime_from_extension("SCRIPT.Js"), "application/javascript")
    
    fr fr Content with mixed binary/text characters
    sus mixed_content drip[value] = [0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x00, 0xFF, 0x57, 0x6F, 0x72, 0x6C, 0x64]
    sus is_mixed_text lit = is_text_content(mixed_content)
    fr fr Should detect as binary due to null bytes and high bytes
    
    fr fr Very large text content (test sampling)
    sus large_text_content drip[value] = []
    bestie (i drip = 0; i < 1000; i++) {
        large_text_content = array_push(large_text_content, 0x41)  fr fr 'A'
    }
    assert_eq_bool(is_text_content(large_text_content), based)
    
    vibez.spill("✅ Edge case tests passed")
}

fr fr ===== PERFORMANCE TEST SCENARIOS =====

slay test_performance_scenarios() {
    test_start("Performance Scenarios")
    
    fr fr Test detection speed with many extensions
    sus start_time drip = timez.now_millis()
    
    bestie (i drip = 0; i < 1000; i++) {
        detect_mime_from_extension("test.jpg")
        detect_mime_from_extension("document.pdf") 
        detect_mime_from_extension("data.json")
        detect_mime_from_extension("style.css")
        detect_mime_from_extension("script.js")
    }
    
    sus end_time drip = timez.now_millis()
    sus elapsed drip = end_time - start_time
    
    vibez.spill("Extension detection performance: ", elapsed, "ms for 5000 operations")
    assert_less_than_int(elapsed, 1000)  fr fr Should complete in under 1 second
    
    fr fr Test content analysis with large binary content
    sus large_binary drip[value] = []
    bestie (i drip = 0; i < 10000; i++) {
        large_binary = array_push(large_binary, i % 256)
    }
    
    sus content_start drip = timez.now_millis()
    sus large_mime tea = detect_mime_from_content(large_binary)
    sus content_end drip = timez.now_millis()
    sus content_elapsed drip = content_end - content_start
    
    vibez.spill("Content analysis performance: ", content_elapsed, "ms for 10KB content")
    assert_less_than_int(content_elapsed, 100)  fr fr Should analyze quickly
    
    fr fr Test comprehensive detection performance
    sus comp_start drip = timez.now_millis()
    
    bestie (i drip = 0; i < 100; i++) {
        sus jpeg_content drip[value] = [0xFF, 0xD8, 0xFF, 0xE0]
        detect_mime_comprehensive("photo.jpg", jpeg_content)
    }
    
    sus comp_end drip = timez.now_millis()
    sus comp_elapsed drip = comp_end - comp_start
    
    vibez.spill("Comprehensive detection performance: ", comp_elapsed, "ms for 100 operations")
    assert_less_than_int(comp_elapsed, 200)
    
    vibez.spill("✅ Performance scenario tests passed")
}

fr fr ===== TEST EXECUTION =====

run_mimez_tests()
