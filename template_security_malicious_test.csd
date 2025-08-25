yeet "template_engine/mod_enhanced"
yeet "testz"

slay main() {
    test_start("Template Security - Malicious Input Tests")
    
    // Test XSS attacks
    test_xss_script_injection()
    test_xss_attribute_injection()
    test_xss_url_injection()
    test_xss_css_injection()
    test_html_entity_bypass()
    test_malformed_html()
    test_nested_attacks()
    
    print_test_summary()
}

slay test_xss_script_injection() {
    test_group("XSS Script Injection")
    
    sus engine TemplateEngineEnhanced = create_template_engine_enhanced()
    sus context map[tea]tea = {}
    
    // Basic script injection
    context["malicious"] = "<script>alert('XSS')</script>"
    sus result tea = render_template_secure(engine, "Hello {{malicious}}", context)
    assert_not_contains(result, "<script>")
    assert_not_contains(result, "alert")
    
    // Script with attributes
    context["malicious"] = "<script type='text/javascript'>document.location='http://evil.com'</script>"
    result = render_template_secure(engine, "{{malicious}}", context)
    assert_not_contains(result, "javascript")
    assert_not_contains(result, "document.location")
    
    // Encoded script injection
    context["malicious"] = "&lt;script&gt;alert('XSS')&lt;/script&gt;"
    result = render_template_secure(engine, "{{malicious}}", context)
    assert_not_contains(result, "alert")
    
    // Mixed case bypass attempt
    context["malicious"] = "<ScRiPt>alert('XSS')</ScRiPt>"
    result = render_template_secure(engine, "{{malicious}}", context)
    assert_not_contains(result, "ScRiPt")
    assert_not_contains(result, "alert")
}

slay test_xss_attribute_injection() {
    test_group("XSS Attribute Injection")
    
    sus engine TemplateEngineEnhanced = create_template_engine_enhanced()
    sus context map[tea]tea = {}
    
    // Event handler injection
    context["malicious"] = "' onload='alert(1)'"
    sus result tea = render_template_secure(engine, "<img src='{{malicious}}'>", context)
    assert_not_contains(result, "onload")
    assert_not_contains(result, "alert")
    
    // JavaScript URL injection
    context["malicious"] = "javascript:alert('XSS')"
    result = render_template_secure(engine, "<a href='{{malicious}}'>Link</a>", context)
    assert_not_contains(result, "javascript:")
    assert_not_contains(result, "alert")
    
    // Data URL injection
    context["malicious"] = "data:text/html,<script>alert('XSS')</script>"
    result = render_template_secure(engine, "<img src='{{malicious}}'>", context)
    assert_not_contains(result, "data:")
    assert_not_contains(result, "script")
    
    // VBScript injection
    context["malicious"] = "vbscript:MsgBox('XSS')"
    result = render_template_secure(engine, "<a href='{{malicious}}'>Link</a>", context)
    assert_not_contains(result, "vbscript:")
    assert_not_contains(result, "MsgBox")
}

slay test_xss_url_injection() {
    test_group("XSS URL Injection")
    
    sus engine TemplateEngineEnhanced = create_template_engine_enhanced()
    sus context map[tea]tea = {}
    
    // Protocol injection
    context["url"] = "javascript:void(0)"
    sus result tea = render_template_secure(engine, "<a href='{{url}}'>Link</a>", context)
    assert_string_equals(result, "<a href=''>Link</a>") // Should be sanitized to empty
    
    // File protocol injection
    context["url"] = "file:///etc/passwd"
    result = render_template_secure(engine, "<img src='{{url}}'>", context)
    assert_string_equals(result, "<img src=''>")
    
    // FTP protocol injection
    context["url"] = "ftp://evil.com/malicious"
    result = render_template_secure(engine, "<a href='{{url}}'>Link</a>", context)
    assert_string_equals(result, "<a href=''>Link</a>")
    
    // Valid URLs should work
    context["url"] = "https://example.com/safe"
    result = render_template_secure(engine, "<a href='{{url}}'>Link</a>", context)
    assert_contains(result, "https://example.com/safe")
    
    // Relative URLs should work
    context["url"] = "/safe/path"
    result = render_template_secure(engine, "<a href='{{url}}'>Link</a>", context)
    assert_contains(result, "/safe/path")
}

slay test_xss_css_injection() {
    test_group("XSS CSS Injection")
    
    sus engine TemplateEngineEnhanced = create_template_engine_enhanced()
    sus context map[tea]tea = {}
    
    // CSS expression injection
    context["style"] = "color: expression(alert('XSS'))"
    sus result tea = render_template_secure(engine, "<div style='{{style}}'>Content</div>", context)
    assert_not_contains(result, "expression")
    assert_not_contains(result, "alert")
    
    // CSS import injection
    context["style"] = "background: url('javascript:alert(1)')"
    result = render_template_secure(engine, "<div style='{{style}}'>Content</div>", context)
    assert_not_contains(result, "javascript:")
    
    // CSS behavior injection (IE specific)
    context["style"] = "behavior: url('evil.htc')"
    result = render_template_secure(engine, "<div style='{{style}}'>Content</div>", context)
    assert_not_contains(result, "behavior:")
}

slay test_html_entity_bypass() {
    test_group("HTML Entity Bypass Attempts")
    
    sus engine TemplateEngineEnhanced = create_template_engine_enhanced()
    sus context map[tea]tea = {}
    
    // HTML entities that might bypass filters
    context["malicious"] = "&#60;script&#62;alert('XSS')&#60;/script&#62;"
    sus result tea = render_template_secure(engine, "{{malicious}}", context)
    assert_not_contains(result, "script")
    assert_not_contains(result, "alert")
    
    // Hex entities
    context["malicious"] = "&#x3C;script&#x3E;alert('XSS')&#x3C;/script&#x3E;"
    result = render_template_secure(engine, "{{malicious}}", context)
    assert_not_contains(result, "script")
    assert_not_contains(result, "alert")
    
    // Unicode entities
    context["malicious"] = "\u003Cscript\u003Ealert('XSS')\u003C/script\u003E"
    result = render_template_secure(engine, "{{malicious}}", context)
    assert_not_contains(result, "script")
    assert_not_contains(result, "alert")
}

slay test_malformed_html() {
    test_group("Malformed HTML Attacks")
    
    sus engine TemplateEngineEnhanced = create_template_engine_enhanced()
    sus context map[tea]tea = {}
    
    // Unclosed tags
    context["malicious"] = "<script>alert('XSS')"
    sus result tea = render_template_secure(engine, "{{malicious}}", context)
    assert_not_contains(result, "script")
    assert_not_contains(result, "alert")
    
    // Nested quotes
    context["malicious"] = "\" onload=\"alert('XSS')"
    result = render_template_secure(engine, "<img src='safe.jpg' alt='{{malicious}}'>", context)
    assert_not_contains(result, "onload")
    assert_not_contains(result, "alert")
    
    // Comment injection
    context["malicious"] = "<!--<script>alert('XSS')</script>-->"
    result = render_template_secure(engine, "{{malicious}}", context)
    assert_not_contains(result, "script")
    assert_not_contains(result, "alert")
    
    // CDATA injection
    context["malicious"] = "<![CDATA[<script>alert('XSS')</script>]]>"
    result = render_template_secure(engine, "{{malicious}}", context)
    assert_not_contains(result, "CDATA")
    assert_not_contains(result, "script")
}

slay test_nested_attacks() {
    test_group("Nested Attack Vectors")
    
    sus engine TemplateEngineEnhanced = create_template_engine_enhanced()
    sus context map[tea]tea = {}
    
    // Double encoding
    context["malicious"] = "%253Cscript%253Ealert('XSS')%253C/script%253E"
    sus result tea = render_template_secure(engine, "{{malicious}}", context)
    assert_not_contains(result, "script")
    assert_not_contains(result, "alert")
    
    // Mixed encoding
    context["malicious"] = "&lt;scr&amp;#105;pt&gt;alert('XSS')&lt;/script&gt;"
    result = render_template_secure(engine, "{{malicious}}", context)
    assert_not_contains(result, "script")
    assert_not_contains(result, "alert")
    
    // Whitespace evasion
    context["malicious"] = "< script >alert('XSS')< / script >"
    result = render_template_secure(engine, "{{malicious}}", context)
    assert_not_contains(result, "script")
    assert_not_contains(result, "alert")
    
    // Tab/newline evasion
    context["malicious"] = "<\tscript\n>alert('XSS')<\t/script\n>"
    result = render_template_secure(engine, "{{malicious}}", context)
    assert_not_contains(result, "script")
    assert_not_contains(result, "alert")
}

// Helper test functions
slay assert_not_contains(text tea, substring tea) {
    vibes string_contains(text, substring) {
        vibez.spill("FAILED: Text contains dangerous substring:", substring)
        vibez.spill("Full text:", text)
        assert_eq_int(0, 1) // Force failure
    }
}

slay assert_contains(text tea, substring tea) {
    vibes !string_contains(text, substring) {
        vibez.spill("FAILED: Text should contain substring:", substring)
        vibez.spill("Full text:", text)
        assert_eq_int(0, 1) // Force failure
    }
}

slay assert_string_equals(actual tea, expected tea) {
    vibes actual != expected {
        vibez.spill("FAILED: String mismatch")
        vibez.spill("Expected:", expected)
        vibez.spill("Actual:", actual)
        assert_eq_int(0, 1) // Force failure
    }
}

slay string_contains(haystack tea, needle tea) lit {
    damn stringz.index_of(haystack, needle) >= 0
}
