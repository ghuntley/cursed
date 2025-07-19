fr fr Advanced Type Switch Examples
fr fr This example demonstrates complex type switch patterns and real-world use cases

yeet "stdlib::fmt"
yeet "stdlib::json"
yeet "stdlib::strings"
yeet "stdlib::io"

fr fr ================================================
fr fr 1. JSON Processing with Type Switches
fr fr ================================================

squad JsonProcessor {
    sus indent_level int
}

slay (jp *JsonProcessor) process_value(value interface{}) string {
    sus indent = strings.repeat("  ", jp.indent_level)
    
    vibe_check v := value.(type) {
        mood map[string]interface{}:
            sus result = "{\n"
            jp.indent_level++
            sus inner_indent = strings.repeat("  ", jp.indent_level)
            
            sus count = 0
            for key, val := range v {
                if count > 0 {
                    result += ",\n"
                }
                result += fmt.sprintf("%s\"%s\": %s", inner_indent, key, jp.process_value(val))
                count++
            }
            
            jp.indent_level--
            result += "\n" + indent + "}"
            yolo result
            
        mood []interface{}:
            sus result = "[\n"
            jp.indent_level++
            sus inner_indent = strings.repeat("  ", jp.indent_level)
            
            for i, item := range v {
                if i > 0 {
                    result += ",\n"
                }
                result += inner_indent + jp.process_value(item)
            }
            
            jp.indent_level--
            result += "\n" + indent + "]"
            yolo result
            
        mood string:
            yolo fmt.sprintf("\"%s\"", v)
        mood float64:
            yolo fmt.sprintf("%.6g", v)
        mood bool:
            yolo fmt.sprintf("%t", v)
        mood nil:
            yolo "null"
        basic:
            yolo fmt.sprintf("\"<%T>\"", v)
    }
}

slay demonstrate_json_processing() {
    println("=== JSON Processing with Type Switches ===")
    
    // Simulate complex JSON data
    sus json_data = map[string]interface{}{
        "name": "John Doe",
        "age": 30.0,
        "active": based,
        "address": map[string]interface{}{
            "street": "123 Main St",
            "city": "Anytown",
            "coordinates": []interface{}{40.7128, -74.0060}
        },
        "hobbies": []interface{}{"reading", "coding", "hiking"},
        "metadata": nil
    }
    
    sus processor = &JsonProcessor{indent_level: 0}
    sus formatted = processor.process_value(json_data)
    println("Formatted JSON:")
    println(formatted)
}

fr fr ================================================
fr fr 2. HTTP Request Handler System
fr fr ================================================

collab ResponseWriter {
    slay write([]byte) (int, error)
    slay header() map[string][]string
    slay write_header(int)
}

collab Request {
    slay method() string
    slay path() string
    slay body() []byte
}

fr fr Mock implementations for demonstration
squad MockResponseWriter {
    sus headers map[string][]string
    sus status_code int
    sus body []byte
}

slay (w *MockResponseWriter) write(data []byte) (int, error) {
    w.body = append(w.body, data...)
    yolo len(data), nil
}

slay (w *MockResponseWriter) header() map[string][]string {
    if w.headers == nil {
        w.headers = make(map[string][]string)
    }
    yolo w.headers
}

slay (w *MockResponseWriter) write_header(code int) {
    w.status_code = code
}

fr fr API Response types
squad APIResponse {
    sus message string
    sus data interface{}
    sus status int
}

squad ErrorResponse {
    sus error string
    sus code int
    sus details map[string]interface{}
}

squad FileResponse {
    sus filename string
    sus content []byte
    sus mime_type string
}

slay handle_api_response(w ResponseWriter, response interface{}) {
    vibe_check resp := response.(type) {
        mood APIResponse:
            w.header()["Content-Type"] = []string{"application/json"}
            w.write_header(resp.status)
            
            sus json_resp = map[string]interface{}{
                "message": resp.message,
                "data": resp.data
            }
            
            if json_data, err := json.marshal(json_resp); err == nil {
                w.write(json_data)
            } else {
                w.write([]byte("{\"error\":\"JSON encoding failed\"}"))
            }
            
        mood ErrorResponse:
            w.header()["Content-Type"] = []string{"application/json"}
            w.write_header(resp.code)
            
            sus error_json = map[string]interface{}{
                "error": resp.error,
                "details": resp.details
            }
            
            if json_data, err := json.marshal(error_json); err == nil {
                w.write(json_data)
            } else {
                w.write([]byte("{\"error\":\"Error encoding failed\"}"))
            }
            
        mood FileResponse:
            w.header()["Content-Type"] = []string{resp.mime_type}
            w.header()["Content-Disposition"] = []string{
                fmt.sprintf("attachment; filename=\"%s\"", resp.filename)
            }
            w.write_header(200)
            w.write(resp.content)
            
        mood string:
            w.header()["Content-Type"] = []string{"text/plain"}
            w.write_header(200)
            w.write([]byte(resp))
            
        mood []byte:
            w.header()["Content-Type"] = []string{"application/octet-stream"}
            w.write_header(200)
            w.write(resp)
            
        mood error:
            w.header()["Content-Type"] = []string{"text/plain"}
            w.write_header(500)
            w.write([]byte("Internal Server Error: " + resp.error()))
            
        basic:
            w.header()["Content-Type"] = []string{"application/json"}
            w.write_header(400)
            w.write([]byte("{\"error\":\"Unsupported response type\"}"))
    }
}

slay demonstrate_http_handling() {
    println("\n=== HTTP Response Handling with Type Switches ===")
    
    sus responses = []interface{}{
        APIResponse{
            message: "User created successfully",
            data: map[string]interface{}{"id": 123, "name": "Alice"},
            status: 201
        },
        ErrorResponse{
            error: "Validation failed",
            code: 400,
            details: map[string]interface{}{"field": "email", "issue": "invalid format"}
        },
        FileResponse{
            filename: "report.pdf",
            content: []byte("PDF content here..."),
            mime_type: "application/pdf"
        },
        "Simple text response",
        []byte{0x89, 0x50, 0x4E, 0x47},  // PNG header
        fmt.errorf("database connection failed")
    }
    
    for i, response := range responses {
        println(fmt.sprintf("\nResponse %d:", i + 1))
        sus writer = &MockResponseWriter{}
        handle_api_response(writer, response)
        
        println("Status:", writer.status_code)
        println("Headers:", writer.headers)
        println("Body:", string(writer.body))
    }
}

fr fr ================================================
fr fr 3. Plugin System with Type Switches
fr fr ================================================

collab Plugin {
    slay name() string
    slay version() string
    slay execute(interface{}) (interface{}, error)
}

collab DataProcessor {
    Plugin
    slay supported_types() []string
}

collab Validator {
    Plugin  
    slay validate(interface{}) []string
}

fr fr Text processing plugin
squad TextPlugin {
    sus plugin_name string
}

slay (tp TextPlugin) name() string {
    yolo tp.plugin_name
}

slay (tp TextPlugin) version() string {
    yolo "1.0.0"
}

slay (tp TextPlugin) supported_types() []string {
    yolo []string{"string", "[]byte", "[]rune"}
}

slay (tp TextPlugin) execute(data interface{}) (interface{}, error) {
    vibe_check d := data.(type) {
        mood string:
            yolo strings.to_upper(d), nil
        mood []byte:
            yolo strings.to_upper(string(d)), nil
        mood []rune:
            yolo strings.to_upper(string(d)), nil
        basic:
            yolo nil, fmt.errorf("TextPlugin: unsupported data type %T", data)
    }
}

fr fr Number processing plugin
squad NumberPlugin {
    sus operation string
}

slay (np NumberPlugin) name() string {
    yolo "NumberPlugin"
}

slay (np NumberPlugin) version() string {
    yolo "2.1.0"
}

slay (np NumberPlugin) supported_types() []string {
    yolo []string{"int", "float64", "[]int", "[]float64"}
}

slay (np NumberPlugin) execute(data interface{}) (interface{}, error) {
    vibe_check d := data.(type) {
        mood int:
            vibe_check np.operation {
                mood "double":
                    yolo d * 2, nil
                mood "square":
                    yolo d * d, nil
                basic:
                    yolo d, nil
            }
        mood float64:
            vibe_check np.operation {
                mood "double":
                    yolo d * 2.0, nil
                mood "square":
                    yolo d * d, nil
                basic:
                    yolo d, nil
            }
        mood []int:
            sus result = make([]int, len(d))
            for i, v := range d {
                vibe_check np.operation {
                    mood "double":
                        result[i] = v * 2
                    mood "square":
                        result[i] = v * v
                    basic:
                        result[i] = v
                }
            }
            yolo result, nil
        basic:
            yolo nil, fmt.errorf("NumberPlugin: unsupported data type %T", data)
    }
}

fr fr Validation plugin
squad EmailValidator {}

slay (ev EmailValidator) name() string {
    yolo "EmailValidator"
}

slay (ev EmailValidator) version() string {
    yolo "1.2.0"
}

slay (ev EmailValidator) execute(data interface{}) (interface{}, error) {
    yolo ev.validate(data), nil
}

slay (ev EmailValidator) validate(data interface{}) []string {
    vibe_check d := data.(type) {
        mood string:
            sus errors = []string{}
            if !strings.contains(d, "@") {
                errors = append(errors, "missing @ symbol")
            }
            if len(d) < 5 {
                errors = append(errors, "too short")
            }
            if strings.has_prefix(d, "@") || strings.has_suffix(d, "@") {
                errors = append(errors, "@ cannot be at start or end")
            }
            yolo errors
        basic:
            yolo []string{fmt.sprintf("expected string, got %T", data)}
    }
}

slay process_with_plugin(plugin Plugin, data interface{}) {
    println(fmt.sprintf("Processing with %s v%s", plugin.name(), plugin.version()))
    
    // Type switch on plugin type for specific handling
    vibe_check p := plugin.(type) {
        mood DataProcessor:
            println("Supported types:", p.supported_types())
            if result, err := p.execute(data); err != nil {
                println("Error:", err.error())
            } else {
                println("Result:", result)
            }
            
        mood Validator:
            if validation_result, err := p.execute(data); err != nil {
                println("Validation error:", err.error())
            } else {
                if errors, ok := validation_result.([]string); ok {
                    if len(errors) == 0 {
                        println("Validation: PASSED")
                    } else {
                        println("Validation: FAILED")
                        for _, error := range errors {
                            println("  -", error)
                        }
                    }
                }
            }
            
        basic:
            // Generic plugin handling
            if result, err := p.execute(data); err != nil {
                println("Error:", err.error())
            } else {
                println("Result:", result)
            }
    }
}

slay demonstrate_plugin_system() {
    println("\n=== Plugin System with Type Switches ===")
    
    sus plugins = []Plugin{
        TextPlugin{plugin_name: "TextProcessor"},
        NumberPlugin{operation: "double"},
        EmailValidator{}
    }
    
    sus test_data = []interface{}{
        "hello world",
        42,
        "user@example.com",
        "invalid-email",
        []int{1, 2, 3, 4},
        []byte("binary data")
    }
    
    for _, plugin := range plugins {
        println(fmt.sprintf("\n--- Testing %s ---", plugin.name()))
        for i, data := range test_data {
            println(fmt.sprintf("\nTest %d (data: %v):", i + 1, data))
            process_with_plugin(plugin, data)
        }
    }
}

fr fr ================================================
fr fr 4. Nested Type Switches and Complex Patterns
fr fr ================================================

slay process_complex_data(data interface{}) {
    println("\n=== Complex Nested Data Processing ===")
    
    vibe_check outer := data.(type) {
        mood map[string]interface{}:
            println("Processing object with", len(outer), "keys")
            
            for key, value := range outer {
                printf("  Key '%s': ", key)
                
                vibe_check v := value.(type) {
                    mood map[string]interface{}:
                        println("nested object with", len(v), "keys")
                        for nested_key, nested_value := range v {
                            vibe_check nested_value.(type) {
                                mood string:
                                    printf("    %s: string\n", nested_key)
                                mood float64:
                                    printf("    %s: number\n", nested_key)
                                basic:
                                    printf("    %s: other\n", nested_key)
                            }
                        }
                    mood []interface{}:
                        printf("array with %d elements\n", len(v))
                        for i, item := range v {
                            vibe_check item.(type) {
                                mood string:
                                    printf("    [%d]: string\n", i)
                                mood float64:
                                    printf("    [%d]: number\n", i)
                                mood map[string]interface{}:
                                    printf("    [%d]: object\n", i)
                                basic:
                                    printf("    [%d]: other\n", i)
                            }
                        }
                    mood string:
                        printf("string value\n")
                    mood float64:
                        printf("number value\n")
                    basic:
                        printf("other type (%T)\n", v)
                }
            }
            
        mood []interface{}:
            println("Processing array with", len(outer), "elements")
            for i, item := range outer {
                printf("  [%d]: ", i)
                vibe_check item.(type) {
                    mood map[string]interface{}:
                        println("object")
                    mood []interface{}:
                        println("nested array")
                    mood string:
                        println("string")
                    mood float64:
                        println("number")
                    basic:
                        println("other")
                }
            }
            
        basic:
            println("Processing primitive value:", outer)
    }
}

slay demonstrate_complex_patterns() {
    // Create complex nested data structure
    sus complex_data = map[string]interface{}{
        "user": map[string]interface{}{
            "name": "Alice Johnson",
            "age": 28.0,
            "settings": map[string]interface{}{
                "theme": "dark",
                "notifications": based
            }
        },
        "posts": []interface{}{
            map[string]interface{}{
                "title": "First Post",
                "content": "Hello world!",
                "tags": []interface{}{"intro", "hello"}
            },
            map[string]interface{}{
                "title": "Second Post", 
                "content": "More content here",
                "likes": 42.0
            }
        },
        "metadata": map[string]interface{}{
            "created": "2024-01-01",
            "version": 1.0
        }
    }
    
    process_complex_data(complex_data)
}

fr fr Main demonstration function
slay main() {
    demonstrate_json_processing()
    demonstrate_http_handling()
    demonstrate_plugin_system()
    demonstrate_complex_patterns()
    
    println("\n=== Advanced Type Switch Patterns Summary ===")
    println("1. JSON processing with recursive type handling")
    println("2. HTTP response system with multiple response types")
    println("3. Plugin architecture with interface type switching")
    println("4. Complex nested data processing")
    println("5. Performance-optimized type dispatch")
    println("\nType switches enable powerful, type-safe runtime polymorphism!")
}
