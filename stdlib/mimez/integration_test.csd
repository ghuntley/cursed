fr fr CURSED MIMEZ Module - Integration Test with Other Stdlib Modules
fr fr Tests interaction with vibez, filez, networkz, and other modules

yeet "mimez"
yeet "vibez"
yeet "stringz"
yeet "testz"

fr fr ===== INTEGRATION TEST SUITE =====

slay run_mimez_integration_tests() {
    vibez.spill("🔗 Running MIMEZ Integration Tests...")
    
    test_vibez_integration()
    test_stringz_integration()
    test_web_server_simulation()
    test_file_processing_pipeline()
    test_api_response_handling()
    
    print_test_summary()
}

fr fr ===== VIBEZ INTEGRATION TESTS =====

slay test_vibez_integration() {
    test_start("VIBEZ Integration")
    
    fr fr Test output formatting with MIME data
    sus mime tea = detect_mime_from_extension("data.json")
    sus description tea = get_mime_description(mime)
    
    vibez.spill("Detected MIME type:", mime)
    vibez.spill("Description:", description)
    
    assert_eq_string(mime, "application/json")
    assert_eq_string(description, "JSON data")
    
    fr fr Test error reporting through vibez
    clear_error()
    detect_mime_from_extension("")  fr fr This should generate an error
    sus error tea = get_last_error()
    
    ready error != "" {
        vibez.spill("Error detected and reported:", error)
    }
    
    vibez.spill("✅ VIBEZ integration tests passed")
}

fr fr ===== STRINGZ INTEGRATION TESTS =====

slay test_stringz_integration() {
    test_start("STRINGZ Integration")
    
    fr fr Test MIME detection with string manipulation
    sus filename tea = "DOCUMENT.PDF"
    sus lowercase_filename tea = string_to_lower(filename)
    sus mime tea = detect_mime_from_extension(lowercase_filename)
    
    assert_eq_string(mime, "application/pdf")
    
    fr fr Test Content-Type header parsing with string operations
    sus header tea = "text/html; charset=utf-8; boundary=test"
    sus parsed ContentTypeHeader = parse_content_type(header)
    
    assert_eq_bool(string_contains(header, parsed.media_type), based)
    assert_eq_bool(string_contains(header, parsed.charset), based)
    assert_eq_bool(string_contains(header, parsed.boundary), based)
    
    fr fr Test extension extraction
    sus full_path tea = "/path/to/document.json"
    sus dot_index drip = string_last_index(full_path, ".")
    sus extension tea = string_substring(full_path, dot_index + 1, -1)
    sus mime_from_ext tea = detect_mime_from_extension("file." + extension)
    
    assert_eq_string(mime_from_ext, "application/json")
    
    vibez.spill("✅ STRINGZ integration tests passed")
}

fr fr ===== WEB SERVER SIMULATION TESTS =====

slay test_web_server_simulation() {
    test_start("Web Server Simulation")
    
    fr fr Simulate HTTP response with proper Content-Type headers
    slay simulate_http_response(filename tea, content []drip) tea {
        sus mime_type tea = detect_mime_comprehensive(filename, content)
        sus content_type tea = get_content_type_for_file(filename)
        sus is_binary lit = is_binary_mime(mime_type)
        
        sus response tea = "HTTP/1.1 200 OK\r\n"
        response = response + "Content-Type: " + content_type + "\r\n"
        
        ready is_binary {
            response = response + "Content-Transfer-Encoding: binary\r\n"
        } otherwise {
            response = response + "Content-Transfer-Encoding: text\r\n"
        }
        
        response = response + "\r\n"
        damn response
    }
    
    fr fr Test various file types
    sus html_content []drip = [0x3C, 0x68, 0x74, 0x6D, 0x6C, 0x3E]  fr fr "<html>"
    sus html_response tea = simulate_http_response("index.html", html_content)
    assert_eq_bool(string_contains(html_response, "text/html"), based)
    assert_eq_bool(string_contains(html_response, "charset=utf-8"), based)
    
    sus jpeg_content []drip = [0xFF, 0xD8, 0xFF, 0xE0]
    sus jpeg_response tea = simulate_http_response("photo.jpg", jpeg_content)
    assert_eq_bool(string_contains(jpeg_response, "image/jpeg"), based)
    assert_eq_bool(string_contains(jpeg_response, "binary"), based)
    
    sus json_content []drip = [0x7B, 0x22, 0x74, 0x65, 0x73, 0x74, 0x22]  fr fr "{\"test\""
    sus json_response tea = simulate_http_response("api.json", json_content)
    assert_eq_bool(string_contains(json_response, "application/json"), based)
    assert_eq_bool(string_contains(json_response, "charset=utf-8"), based)
    
    vibez.spill("✅ Web server simulation tests passed")
}

fr fr ===== FILE PROCESSING PIPELINE TESTS =====

slay test_file_processing_pipeline() {
    test_start("File Processing Pipeline")
    
    fr fr Simulate a file processing pipeline
    slay process_file_batch(files []tea) {
        sus results []FileProcessResult = []
        
        bestie (i drip = 0; i < array_len(files); i++) {
            sus filename tea = files[i]
            sus mime_type tea = detect_mime_from_extension(filename)
            sus is_binary lit = is_binary_mime(mime_type)
            sus description tea = get_mime_description(mime_type)
            
            sus result FileProcessResult = FileProcessResult{
                filename: filename,
                mime_type: mime_type,
                is_binary: is_binary,
                description: description,
                processed: based
            }
            
            results = array_push(results, result)
        }
        
        damn results
    }
    
    sus test_files []tea = [
        "document.pdf",
        "image.png", 
        "data.json",
        "script.js",
        "style.css",
        "video.mp4",
        "audio.wav",
        "archive.zip",
        "source.csd"
    ]
    
    sus processing_results []FileProcessResult = process_file_batch(test_files)
    
    fr fr Verify all files were processed
    assert_eq_int(array_len(processing_results), array_len(test_files))
    
    fr fr Check specific results
    bestie (i drip = 0; i < array_len(processing_results); i++) {
        sus result FileProcessResult = processing_results[i]
        assert_eq_bool(result.processed, based)
        assert_not_equal_string(result.mime_type, "")
        assert_not_equal_string(result.description, "")
        
        ready result.filename == "document.pdf" {
            assert_eq_string(result.mime_type, "application/pdf")
            assert_eq_bool(result.is_binary, based)
        }
        
        ready result.filename == "data.json" {
            assert_eq_string(result.mime_type, "application/json") 
            assert_eq_bool(result.is_binary, cap)
        }
        
        ready result.filename == "source.csd" {
            assert_eq_string(result.mime_type, "text/x-cursed")
            assert_eq_bool(result.is_binary, cap)
        }
    }
    
    vibez.spill("Processed", array_len(processing_results), "files successfully")
    vibez.spill("✅ File processing pipeline tests passed")
}

fr fr ===== API RESPONSE HANDLING TESTS =====

slay test_api_response_handling() {
    test_start("API Response Handling")
    
    fr fr Simulate API endpoints returning different content types
    slay handle_api_endpoint(endpoint tea, accept_header tea) APIResponse {
        sus response_mime tea = ""
        sus response_content tea = ""
        
        ready endpoint == "/api/data" {
            response_mime = "application/json"
            response_content = "{\"status\": \"success\", \"data\": []}"
        } otherwise ready endpoint == "/api/export" {
            ready string_contains(accept_header, "text/csv") {
                response_mime = "text/csv"
                response_content = "name,value\ntest,123\n"
            } otherwise {
                response_mime = "application/json"
                response_content = "{\"export\": \"data\"}"
            }
        } otherwise ready endpoint == "/docs" {
            response_mime = "text/html"
            response_content = "<html><body>Documentation</body></html>"
        } otherwise {
            response_mime = "text/plain"
            response_content = "Not found"
        }
        
        sus content_type_header tea = ready string_starts_with(response_mime, "text/") {
            format_content_type(response_mime, "utf-8", "")
        } otherwise {
            response_mime
        }
        
        sus api_response APIResponse = APIResponse{
            status_code: 200,
            content_type: content_type_header,
            body: response_content
        }
        
        damn api_response
    }
    
    fr fr Test various API scenarios
    sus json_response APIResponse = handle_api_endpoint("/api/data", "application/json")
    assert_eq_string(json_response.content_type, "application/json; charset=utf-8")
    assert_eq_bool(string_contains(json_response.body, "status"), based)
    
    sus csv_response APIResponse = handle_api_endpoint("/api/export", "text/csv")
    assert_eq_string(csv_response.content_type, "text/csv; charset=utf-8")
    assert_eq_bool(string_contains(csv_response.body, "name,value"), based)
    
    sus html_response APIResponse = handle_api_endpoint("/docs", "text/html")
    assert_eq_string(html_response.content_type, "text/html; charset=utf-8")
    assert_eq_bool(string_contains(html_response.body, "<html>"), based)
    
    fr fr Test content negotiation
    sus json_fallback APIResponse = handle_api_endpoint("/api/export", "application/json")
    assert_eq_string(json_fallback.content_type, "application/json; charset=utf-8")
    
    vibez.spill("✅ API response handling tests passed")
}

fr fr ===== SUPPORTING DATA STRUCTURES =====

squad FileProcessResult {
    filename tea,
    mime_type tea,
    is_binary lit,
    description tea,
    processed lit
}

squad APIResponse {
    status_code drip,
    content_type tea,
    body tea
}

fr fr ===== INTEGRATION TEST EXECUTION =====

run_mimez_integration_tests()
