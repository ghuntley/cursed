yeet "template_engine/mod"
yeet "vibez"

slay main() {
    vibez.spill("=== Template Security Test ===")
    
    // Test basic XSS prevention
    test_basic_xss()
    
    // Test URL sanitization 
    test_url_sanitization()
    
    // Test attribute sanitization
    test_attribute_sanitization()
    
    vibez.spill("=== Security Tests Complete ===")
}

slay test_basic_xss() {
    vibez.spill("Testing basic XSS prevention...")
    
    // Test script tag injection
    sus malicious tea = "<script>alert('XSS')</script>"
    sus escaped tea = html_escape_secure(malicious)
    vibez.spill("Input: " + malicious)
    vibez.spill("Escaped: " + escaped)
    
    // Verify it's properly escaped
    vibes !string_contains_unsafe(escaped, "script") {
        vibez.spill("✓ Script tags properly escaped")
    } nah {
        vibez.spill("✗ Script tags NOT properly escaped")
    }
    
    // Test mixed case bypass attempt
    malicious = "<ScRiPt>alert('XSS')</ScRiPt>"
    escaped = html_escape_secure(malicious)
    vibez.spill("Mixed case input: " + malicious)
    vibez.spill("Escaped: " + escaped)
}

slay test_url_sanitization() {
    vibez.spill("Testing URL sanitization...")
    
    // Test javascript: URL
    sus dangerous_url tea = "javascript:alert('XSS')"
    sus safe_url tea = sanitize_urls_in_text(dangerous_url)
    vibez.spill("Dangerous URL: " + dangerous_url)
    vibez.spill("Sanitized: " + safe_url)
    
    // Test data: URL
    dangerous_url = "data:text/html,<script>alert('XSS')</script>"
    safe_url = sanitize_urls_in_text(dangerous_url)
    vibez.spill("Data URL: " + dangerous_url)
    vibez.spill("Sanitized: " + safe_url)
    
    // Test safe URL (should remain unchanged)
    sus good_url tea = "https://example.com/safe"
    sus preserved_url tea = sanitize_urls_in_text(good_url)
    vibez.spill("Good URL: " + good_url)
    vibez.spill("Preserved: " + preserved_url)
}

slay test_attribute_sanitization() {
    vibez.spill("Testing attribute sanitization...")
    
    // Create a mock parser for testing
    sus parser HTMLParser = create_html_parser()
    initialize_security_maps(parser)
    
    // Test dangerous attribute
    sus malicious_attr tea = "onload=\"alert('XSS')\""
    sus safe_attr tea = sanitize_basic_attributes(malicious_attr)
    vibez.spill("Malicious attr: " + malicious_attr)
    vibez.spill("Sanitized: " + safe_attr)
    
    // Test safe attribute
    sus good_attr tea = "class=\"safe-class\""
    sus preserved_attr tea = sanitize_basic_attributes(good_attr)
    vibez.spill("Good attr: " + good_attr)  
    vibez.spill("Preserved: " + preserved_attr)
}

slay string_contains_unsafe(haystack tea, needle tea) lit {
    sus lower_haystack tea = stringz.to_lower(haystack)
    sus lower_needle tea = stringz.to_lower(needle)
    damn stringz.index_of(lower_haystack, lower_needle) >= 0
}

slay initialize_security_maps(parser HTMLParser) {
    // Initialize the security maps that would normally be done in create_html_parser
    parser.dangerous_tags = {
        "script": based,
        "iframe": based,
        "object": based,
        "embed": based,
        "form": based,
        "input": based,
        "textarea": based,
        "button": based,
        "link": based,
        "meta": based,
        "style": based
    }
    
    parser.safe_attributes = {
        "div": {"class": based, "id": based},
        "span": {"class": based, "id": based},
        "p": {"class": based, "id": based},
        "a": {"href": based, "title": based},
        "img": {"src": based, "alt": based}
    }
}
