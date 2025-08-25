// CURSED Template Engine Comprehensive Test Suite
// Tests for production-grade template processing with security and reflection

yeet "template_engine"
yeet "testz"
yeet "stringz"
yeet "cryptz"
yeet "reflectz"

// Test data structures
be_like TestUser squad {
    name tea
    email tea
    age normie
    is_admin lit
    profile TestProfile
}

be_like TestProfile squad {
    bio tea
    avatar_url tea
    social_links [tea]
}

// Test cryptographic security in templates
slay test_cryptographic_security() {
    testz.test_group("Cryptographic Security")
    
    sus config ProcessorConfig = ProcessorConfig{
        enable_caching: based,
        enable_security: based,
        max_content_size: 1024 * 1024,
        cache_size: 100,
        unicode_normalization: based
    }
    
    sus engine TemplateEngine = create_production_template_engine(config)
    
    // Test template content hashing
    sus template_source tea = "Hello {{.name}}! Your email is {{.email}}"
    sus compiled CompiledTemplate = compile_template(engine, "test_secure", template_source)
    
    testz.assert_not_empty(compiled.metadata.source_hash, "Template should have SHA-256 hash")
    testz.assert_not_empty(compiled.security_hash, "Compiled template should have security hash")
    testz.assert_equal_int(len(compiled.security_hash), 64, "Security hash should be 64 characters (SHA-256 hex)")
    
    // Test nonce generation
    sus nonce1 tea = cryptz.generate_secure_nonce(32)
    sus nonce2 tea = cryptz.generate_secure_nonce(32)
    testz.assert_not_equal(nonce1, nonce2, "Nonces should be unique")
    testz.assert_equal_int(len(nonce1), 64, "32-byte nonce should be 64 hex characters")
    
    testz.test_complete("Cryptographic Security")
}

// Test reflection-based field access
slay test_reflection_field_access() {
    testz.test_group("Reflection Field Access")
    
    sus user TestUser = TestUser{
        name: "John Doe",
        email: "john@example.com",
        age: 30,
        is_admin: based,
        profile: TestProfile{
            bio: "Software engineer with 10 years experience",
            avatar_url: "https://example.com/avatar.jpg",
            social_links: ["https://twitter.com/johndoe", "https://github.com/johndoe"]
        }
    }
    
    sus engine TemplateEngine = create_production_template_engine(ProcessorConfig{
        enable_caching: based,
        enable_reflection: based,
        enable_security: based,
        max_content_size: 1024,
        cache_size: 50,
        unicode_normalization: based
    })
    
    // Test basic field access
    sus template1 tea = "User: {{.name}} ({{.email}})"
    sus compiled1 CompiledTemplate = compile_template(engine, "user_basic", template1)
    sus result1 tea = execute_compiled_template(engine, compiled1, user)
    
    testz.assert_contains(result1, "John Doe", "Should access user name field")
    testz.assert_contains(result1, "john@example.com", "Should access user email field")
    
    // Test nested field access  
    sus template2 tea = "Bio: {{.profile.bio}}"
    sus compiled2 CompiledTemplate = compile_template(engine, "user_nested", template2)
    sus result2 tea = execute_compiled_template(engine, compiled2, user)
    
    testz.assert_contains(result2, "Software engineer", "Should access nested profile bio field")
    
    // Test array field access
    sus template3 tea = "Social: {{.profile.social_links[0]}}"
    sus compiled3 CompiledTemplate = compile_template(engine, "user_array", template3)
    sus result3 tea = execute_compiled_template(engine, compiled3, user)
    
    testz.assert_contains(result3, "twitter.com/johndoe", "Should access array element")
    
    testz.test_complete("Reflection Field Access")
}

// Test bytecode compilation and execution
slay test_bytecode_compilation() {
    testz.test_group("Bytecode Compilation")
    
    sus engine TemplateEngine = create_production_template_engine(ProcessorConfig{
        enable_caching: based,
        enable_security: based,
        max_content_size: 2048,
        cache_size: 100,
        unicode_normalization: based
    })
    
    // Test simple template compilation
    sus template_source tea = "Hello {{.name}}! Welcome to {{.site_name}}."
    sus compiled CompiledTemplate = compile_template(engine, "welcome", template_source)
    
    testz.assert_true(compiled.version > 0, "Compiled template should have version")
    testz.assert_true(len(compiled.bytecode) > 0, "Compiled template should have bytecode")
    testz.assert_true(len(compiled.constants) > 0, "Compiled template should have constants table")
    testz.assert_not_empty(compiled.metadata.source_hash, "Template should have metadata")
    
    // Test constants extraction
    sus expected_constants [tea] = ["Hello ", "! Welcome to ", "."]
    bestie i := 0; i < len(expected_constants); i++ {
        sus expected tea = expected_constants[i]
        sus found lit = cap
        
        bestie j := 0; j < len(compiled.constants); j++ {
            vibes compiled.constants[j] == expected {
                found = based
                ghosted
            }
        }
        
        testz.assert_true(found, "Should find constant: " + expected)
    }
    
    // Test execution with data
    sus data map[tea]interface{} = {
        "name": "Alice",
        "site_name": "CURSED Templates"
    }
    
    sus result tea = execute_compiled_template(engine, compiled, data)
    testz.assert_contains(result, "Hello Alice", "Should substitute name variable")
    testz.assert_contains(result, "CURSED Templates", "Should substitute site_name variable")
    
    testz.test_complete("Bytecode Compilation")
}

// Test advanced template features
slay test_advanced_template_features() {
    testz.test_group("Advanced Template Features")
    
    sus engine TemplateEngine = create_production_template_engine(ProcessorConfig{
        enable_caching: based,
        enable_security: based,
        max_content_size: 4096,
        cache_size: 200,
        unicode_normalization: based
    })
    
    // Test conditional rendering
    sus conditional_template tea = "{{if .is_admin}}Admin Panel{{else}}User Dashboard{{end}}"
    sus admin_data map[tea]interface{} = {"is_admin": based}
    sus user_data map[tea]interface{} = {"is_admin": cap}
    
    sus compiled_cond CompiledTemplate = compile_template(engine, "conditional", conditional_template)
    sus admin_result tea = execute_compiled_template(engine, compiled_cond, admin_data)
    sus user_result tea = execute_compiled_template(engine, compiled_cond, user_data)
    
    testz.assert_contains(admin_result, "Admin Panel", "Should show admin content for admin user")
    testz.assert_contains(user_result, "User Dashboard", "Should show user content for regular user")
    
    // Test loop rendering
    sus loop_template tea = "Users: {{range .users}}{{.name}}, {{end}}"
    sus users_data map[tea]interface{} = {
        "users": [
            {"name": "Alice"},
            {"name": "Bob"},
            {"name": "Charlie"}
        ]
    }
    
    sus compiled_loop CompiledTemplate = compile_template(engine, "users_loop", loop_template)
    sus loop_result tea = execute_compiled_template(engine, compiled_loop, users_data)
    
    testz.assert_contains(loop_result, "Alice", "Should render first user")
    testz.assert_contains(loop_result, "Bob", "Should render second user")
    testz.assert_contains(loop_result, "Charlie", "Should render third user")
    
    // Test function calls
    sus function_template tea = "{{upper .message}} - {{len .items}} items"
    sus function_data map[tea]interface{} = {
        "message": "hello world",
        "items": ["a", "b", "c", "d"]
    }
    
    sus compiled_func CompiledTemplate = compile_template(engine, "functions", function_template)
    sus func_result tea = execute_compiled_template(engine, compiled_func, function_data)
    
    testz.assert_contains(func_result, "HELLO WORLD", "Should apply upper function")
    testz.assert_contains(func_result, "4 items", "Should apply len function")
    
    testz.test_complete("Advanced Template Features")
}

// Test security features
slay test_security_features() {
    testz.test_group("Security Features")
    
    sus engine TemplateEngine = create_production_template_engine(ProcessorConfig{
        enable_caching: based,
        enable_security: based,
        max_content_size: 1024,
        cache_size: 50,
        unicode_normalization: based
    })
    
    // Test HTML escaping
    sus xss_template tea = "Message: {{.user_input}}"
    sus xss_data map[tea]interface{} = {
        "user_input": "<script>alert('XSS')</script>"
    }
    
    sus compiled_xss CompiledTemplate = compile_template(engine, "xss_test", xss_template)
    sus xss_result tea = execute_compiled_template(engine, compiled_xss, xss_data)
    
    testz.assert_contains(xss_result, "&lt;script&gt;", "Should escape HTML tags")
    testz.assert_not_contains(xss_result, "<script>", "Should not contain unescaped script tags")
    
    // Test JavaScript escaping
    sus js_template tea = "var data = '{{js_escape .data}}';"
    sus js_data map[tea]interface{} = {
        "data": "'; alert('XSS'); var x='"
    }
    
    sus compiled_js CompiledTemplate = compile_template(engine, "js_escape", js_template)
    sus js_result tea = execute_compiled_template(engine, compiled_js, js_data)
    
    testz.assert_contains(js_result, "\\'", "Should escape single quotes")
    testz.assert_not_contains(js_result, "'; alert('XSS');", "Should prevent JS injection")
    
    // Test CSS escaping
    sus css_template tea = "color: {{css_escape .color}};"
    sus css_data map[tea]interface{} = {
        "color": "red; background: url('javascript:alert(1)')"
    }
    
    sus compiled_css CompiledTemplate = compile_template(engine, "css_escape", css_template)
    sus css_result tea = execute_compiled_template(engine, compiled_css, css_data)
    
    testz.assert_contains(css_result, "\\;", "Should escape CSS semicolons")
    testz.assert_not_contains(css_result, "javascript:", "Should prevent CSS injection")
    
    testz.test_complete("Security Features")
}

// Test template caching with integrity
slay test_template_caching() {
    testz.test_group("Template Caching")
    
    sus engine TemplateEngine = create_production_template_engine(ProcessorConfig{
        enable_caching: based,
        enable_security: based,
        max_content_size: 1024,
        cache_size: 10,
        unicode_normalization: based
    })
    
    // Compile same template twice
    sus template_source tea = "Cached: {{.value}}"
    sus compiled1 CompiledTemplate = compile_template(engine, "cache_test", template_source)
    sus compiled2 CompiledTemplate = compile_template(engine, "cache_test", template_source)
    
    testz.assert_equal(compiled1.security_hash, compiled2.security_hash, "Cached templates should have same security hash")
    testz.assert_equal(compiled1.metadata.source_hash, compiled2.metadata.source_hash, "Cached templates should have same source hash")
    
    // Test cache hit performance
    sus start_time1 normie = timez.now_unix_nano()
    sus result1 CompiledTemplate = compile_template(engine, "perf_test", "Performance {{.test}}")
    sus compilation_time1 normie = timez.now_unix_nano() - start_time1
    
    sus start_time2 normie = timez.now_unix_nano()
    sus result2 CompiledTemplate = compile_template(engine, "perf_test", "Performance {{.test}}")
    sus compilation_time2 normie = timez.now_unix_nano() - start_time2
    
    // Second compilation should be faster (cache hit)
    testz.assert_true(compilation_time2 < compilation_time1, "Cached compilation should be faster")
    
    testz.test_complete("Template Caching")
}

// Test performance profiling
slay test_performance_profiling() {
    testz.test_group("Performance Profiling")
    
    sus engine TemplateEngine = create_production_template_engine(ProcessorConfig{
        enable_caching: based,
        enable_security: based,
        max_content_size: 8192,
        cache_size: 100,
        unicode_normalization: based
    })
    
    // Create complex template for performance testing
    sus complex_template tea = `
    <html>
    <head><title>{{.title}}</title></head>
    <body>
        <h1>{{upper .heading}}</h1>
        {{if .show_users}}
        <ul>
        {{range .users}}
            <li>{{.name}} - {{.email}} (Age: {{.age}})</li>
        {{end}}
        </ul>
        {{end}}
        <p>Generated at: {{format_date .timestamp "2006-01-02 15:04:05"}}</p>
        <footer>{{html_escape .footer_text}}</footer>
    </body>
    </html>`
    
    sus test_data map[tea]interface{} = {
        "title": "User List",
        "heading": "registered users",
        "show_users": based,
        "users": [
            {"name": "Alice Johnson", "email": "alice@example.com", "age": 28},
            {"name": "Bob Smith", "email": "bob@example.com", "age": 35},
            {"name": "Carol Wilson", "email": "carol@example.com", "age": 42}
        ],
        "timestamp": 1609459200,  // 2021-01-01 00:00:00
        "footer_text": "© 2024 Example Corp. All rights reserved."
    }
    
    // Profile template compilation and execution
    sus profile TemplatePerformanceProfile = profile_template_performance(engine, complex_template, test_data, 100)
    
    testz.assert_true(profile.iterations == 100, "Should run specified number of iterations")
    testz.assert_true(profile.average_execution_time > 0, "Should measure execution time")
    testz.assert_true(profile.compilation_time > 0, "Should measure compilation time")
    testz.assert_true(profile.bytecode_size > 0, "Should report bytecode size")
    
    // Ensure reasonable performance thresholds
    testz.assert_true(profile.average_execution_time < 1000000, "Average execution should be under 1ms")
    testz.assert_true(profile.compilation_time < 10000000, "Compilation should be under 10ms")
    
    testz.test_complete("Performance Profiling")
}

// Test template inheritance
slay test_template_inheritance() {
    testz.test_group("Template Inheritance")
    
    sus engine TemplateEngine = create_production_template_engine(ProcessorConfig{
        enable_caching: based,
        enable_security: based,
        max_content_size: 2048,
        cache_size: 50,
        unicode_normalization: based
    })
    
    // Base template
    sus base_template tea = `
    <!DOCTYPE html>
    <html>
    <head>
        <title>{{block "title"}}Default Title{{end}}</title>
    </head>
    <body>
        <header>{{block "header"}}Default Header{{end}}</header>
        <main>{{block "content"}}Default Content{{end}}</main>
        <footer>{{block "footer"}}Default Footer{{end}}</footer>
    </body>
    </html>`
    
    // Child template
    sus child_template tea = `
    {{define "title"}}Custom Page Title{{end}}
    {{define "content"}}
        <h1>Welcome {{.username}}</h1>
        <p>This is custom content for {{.page_name}}</p>
    {{end}}
    {{define "footer"}}© 2024 Custom Footer{{end}}`
    
    sus combined_template CompiledTemplate = create_template_with_inheritance(engine, base_template, child_template)
    
    sus inheritance_data map[tea]interface{} = {
        "username": "John Doe",
        "page_name": "Dashboard"
    }
    
    sus result tea = execute_compiled_template(engine, combined_template, inheritance_data)
    
    testz.assert_contains(result, "Custom Page Title", "Should override title block")
    testz.assert_contains(result, "Welcome John Doe", "Should render custom content with variables")
    testz.assert_contains(result, "© 2024 Custom Footer", "Should override footer block")
    testz.assert_contains(result, "Default Header", "Should use default header when not overridden")
    
    testz.test_complete("Template Inheritance")
}

// Test Unicode and internationalization
slay test_unicode_support() {
    testz.test_group("Unicode Support")
    
    sus engine TemplateEngine = create_production_template_engine(ProcessorConfig{
        enable_caching: based,
        enable_security: based,
        max_content_size: 1024,
        cache_size: 50,
        unicode_normalization: based
    })
    
    // Test various Unicode characters
    sus unicode_template tea = "Hello {{.name}}! Message: {{.message}}"
    sus unicode_data map[tea]interface{} = {
        "name": "José María",
        "message": "こんにちは世界 🌍 Здравствуй мир! مرحبا بالعالم"
    }
    
    sus compiled CompiledTemplate = compile_template(engine, "unicode_test", unicode_template)
    sus result tea = execute_compiled_template(engine, compiled, unicode_data)
    
    testz.assert_contains(result, "José María", "Should handle Latin characters with accents")
    testz.assert_contains(result, "こんにちは", "Should handle Japanese characters")
    testz.assert_contains(result, "🌍", "Should handle emoji")
    testz.assert_contains(result, "Здравствуй", "Should handle Cyrillic characters")
    testz.assert_contains(result, "مرحبا", "Should handle Arabic characters")
    
    // Test Unicode-aware string length
    sus emoji_template tea = "Length: {{len .emoji_string}}"
    sus emoji_data map[tea]interface{} = {
        "emoji_string": "👨‍👩‍👧‍👦🚀"  // Family emoji + rocket emoji
    }
    
    sus emoji_compiled CompiledTemplate = compile_template(engine, "emoji_test", emoji_template)
    sus emoji_result tea = execute_compiled_template(engine, emoji_compiled, emoji_data)
    
    testz.assert_contains(emoji_result, "Length:", "Should calculate length of Unicode string")
    
    testz.test_complete("Unicode Support")
}

// Test error handling and validation
slay test_error_handling() {
    testz.test_group("Error Handling")
    
    sus engine TemplateEngine = create_production_template_engine(ProcessorConfig{
        enable_caching: based,
        enable_security: based,
        max_content_size: 512,
        cache_size: 25,
        unicode_normalization: based
    })
    
    // Test invalid template syntax
    sus invalid_template tea = "Hello {{.name unclosed"
    sus error_occurred lit = cap
    
    yikes {
        sus compiled CompiledTemplate = compile_template(engine, "invalid", invalid_template)
    } fam err {
        error_occurred = based
    }
    
    testz.assert_true(error_occurred, "Should handle syntax errors gracefully")
    
    // Test missing data field
    sus missing_field_template tea = "Hello {{.missing_field}}"
    sus incomplete_data map[tea]interface{} = {"name": "John"}
    
    sus compiled_missing CompiledTemplate = compile_template(engine, "missing_field", missing_field_template)
    sus result_missing tea = execute_compiled_template(engine, compiled_missing, incomplete_data)
    
    // Should handle missing field gracefully (empty string or error message)
    testz.assert_not_empty(result_missing, "Should handle missing fields gracefully")
    
    // Test template size limits
    sus large_template tea = ""
    bestie i := 0; i < 1000; i++ {
        large_template = large_template + "This is a very long template content. "
    }
    
    sus size_error_occurred lit = cap
    yikes {
        sus large_compiled CompiledTemplate = compile_template(engine, "too_large", large_template)
    } fam err {
        size_error_occurred = based
    }
    
    testz.assert_true(size_error_occurred, "Should enforce template size limits")
    
    testz.test_complete("Error Handling")
}

// Main test runner
slay run_all_tests() {
    testz.test_start("Template Engine Comprehensive Test Suite")
    
    test_cryptographic_security()
    test_reflection_field_access()
    test_bytecode_compilation()
    test_advanced_template_features()
    test_security_features()
    test_template_caching()
    test_performance_profiling()
    test_template_inheritance()
    test_unicode_support()
    test_error_handling()
    
    testz.print_test_summary()
}

// Execute tests
run_all_tests()
