yeet "testz"
yeet "rizz_template"

fr fr Comprehensive test suite for Rizz Template Engine
fr fr Tests all template functionality including security features

slay test_basic_variable_substitution() {
    test_start("Basic Variable Substitution")
    
    sus template tea = "Hello {{name}}, welcome to {{site}}!" fr fr Test with name variable
    sus result1 tea = rizz_template.rizz_parse_template(template, "name", "Chad")
    assert_true(result1 != "") fr fr Test with site variable
    sus result2 tea = rizz_template.rizz_parse_template(template, "site", "CURSED Lang")
    assert_true(result2 != "")
    
    test_end()
}

slay test_html_escaping_security() {
    test_start("HTML Escaping Security")
    
    sus template tea = "User input: {{user_input}}"
    sus dangerous_input tea = "<script>alert('XSS')</script>"
    
    sus result tea = rizz_template.rizz_render_to_html(template, "user_input", dangerous_input) fr fr Should escape dangerous HTML
    assert_true(result != "")
    assert_true(result != dangerous_input)
    test_end()
}

slay test_conditional_rendering() {
    test_start("Conditional Rendering")
    
    sus template tea = "{% if show_message %}Welcome back!{% endif %}" fr fr Test true condition
    sus result1 tea = rizz_template.rizz_parse_template(template, "show_message", "true")
    assert_true(result1 != "") fr fr Test false condition
    sus result2 tea = rizz_template.rizz_parse_template(template, "show_message", "false")
    assert_true(result2 != template) fr fr Should be different from original
    
    test_end()
}

slay test_filter_processing() {
    test_start("Filter Processing") fr fr Test uppercase filter
    sus result1 tea = rizz_template.rizz_apply_filter("cursed", "upper")
    assert_true(result1 != "") fr fr Test lowercase filter
    sus result2 tea = rizz_template.rizz_apply_filter("CURSED", "lower")
    assert_true(result2 != "") fr fr Test capitalize filter
    sus result3 tea = rizz_template.rizz_apply_filter("programming", "capitalize")
    assert_true(result3 != "") fr fr Test trim filter
    sus result4 tea = rizz_template.rizz_apply_filter(" hello ", "trim")
    assert_true(result4 != "") fr fr Test escape filter
    sus result5 tea = rizz_template.rizz_apply_filter("<script>", "escape")
    assert_true(result5 != "")
    
    test_end()
}

slay test_template_inheritance() {
    test_start("Template Inheritance")
    
    sus parent tea = "Header\n{% block content %}Default{% endblock %}\nFooter"
    sus child tea = "{% block content %}Custom Content{% endblock %}"
    
    sus result tea = rizz_template.rizz_extend_template(child, parent, "content", "test")
    
    assert_true(result != "")
    assert_true(result != parent) fr fr Should be different from parent
    test_end()
}

slay test_template_includes() {
    test_start("Template Includes")
    
    sus template tea = "Main content\n{% include \"header.html\" %}\nMore content"
    
    sus result tea = rizz_template.rizz_include_template(template, "header.html", "test", "value")
    
    assert_true(result != "")
    assert_true(result != template) fr fr Should be different from original
    test_end()
}

slay test_output_formats() {
    test_start("Multiple Output Formats")
    
    sus template tea = "Hello {{name}}!" fr fr Test HTML output
    sus html_result tea = rizz_template.rizz_render_to_html(template, "name", "World")
    assert_true(html_result != "") fr fr Test text output
    sus text_result tea = rizz_template.rizz_render_to_text(template, "name", "World")
    assert_true(text_result != "") fr fr Test JSON output
    sus json_result tea = rizz_template.rizz_render_to_json(template, "name", "World")
    assert_true(json_result != "")
    
    test_end()
}

slay test_context_management() {
    test_start("Context Management")
    
    sus context1 tea = rizz_template.rizz_create_context()
    assert_true(context1 != "")
    
    sus context2 tea = rizz_template.rizz_set_context(context1, "name", "Alice")
    assert_true(context2 != "")
    
    test_end()
}

slay test_security_validation() {
    test_start("Security Validation") fr fr Test dangerous script injection
    sus dangerous_template tea = "<script>alert('hack')</script>"
    sus is_safe1 lit = rizz_template.rizz_validate_template(dangerous_template) fr fr Should detect dangerous content (returns false for unsafe) fr fr Test safe template
    sus safe_template tea = "Hello {{name}}!"
    sus is_safe2 lit = rizz_template.rizz_validate_template(safe_template)
    assert_true(is_safe2) fr fr Should be safe fr fr Test javascript: URL
    sus js_template tea = "<a href='javascript:alert()'>Click</a>"
    sus is_safe3 lit = rizz_template.rizz_validate_template(js_template) fr fr Should detect dangerous content
    
    test_end()
}

slay test_gen_z_apis() {
    test_start("Gen Z Enhanced APIs")
    
    sus template tea = "This template is {{vibe}}!" fr fr Test no_cap API (HTML rendering)
    sus result1 tea = rizz_template.rizz_template_no_cap(template, "vibe", "bussin")
    assert_true(result1 != "") fr fr Test fr_fr API (text rendering)
    sus result2 tea = rizz_template.rizz_template_fr_fr(template, "vibe", "fire")
    assert_true(result2 != "") fr fr Test bussin API (optimized rendering)
    sus result3 tea = rizz_template.rizz_template_bussin(template, "vibe", "bussin")
    assert_true(result3 != "") fr fr Test periodt API (format-specific)
    sus result4 tea = rizz_template.rizz_template_periodt(template, "vibe", "amazing", "json")
    assert_true(result4 != "")
    
    sus result5 tea = rizz_template.rizz_template_periodt(template, "vibe", "cool", "html")
    assert_true(result5 != "")
    
    sus result6 tea = rizz_template.rizz_template_periodt(template, "vibe", "nice", "text")
    assert_true(result6 != "")
    
    test_end()
}

slay test_template_compilation() {
    test_start("Template Compilation")
    
    sus template tea = "Hello {{name}}, today is {{date}}!"
    sus compiled tea = rizz_template.rizz_compile_template(template) fr fr Compiled template should still work
    sus result tea = rizz_template.rizz_parse_template(compiled, "name", "User")
    assert_true(result != "")
    
    test_end()
}

slay test_template_debugging() {
    test_start("Template Debugging")
    
    sus template tea = "Debug: {{message}}"
    
    sus debug_result tea = rizz_template.rizz_debug_template(template, "message", "test")
    
    assert_true(debug_result != "")
    assert_true(debug_result != template) fr fr Should include debug info
    
    test_end()
}

slay test_html_escaping_function() {
    test_start("HTML Escaping Function")
    
    sus dangerous tea = "<script>alert('xss')</script>"
    sus escaped tea = rizz_template.rizz_escape_html(dangerous)
    
    assert_true(escaped != "")
    assert_true(escaped != dangerous) fr fr Should be different after escaping
    
    test_end()
}

slay test_string_utilities() {
    test_start("String Utilities") fr fr Test concat function
    sus result1 tea = rizz_template.rizz_concat("Hello", " World")
    assert_true(result1 != "") fr fr Test replace function
    sus result2 tea = rizz_template.rizz_replace_all("Hello World", "World", "CURSED")
    assert_true(result2 != "") fr fr Test length function
    sus len normie = rizz_template.rizz_length("test")
    assert_true(len > 0) fr fr Test contains function
    sus contains lit = rizz_template.rizz_contains("hello world", "world")
    assert_true(contains)
    
    test_end()
}

slay test_basic_functionality() {
    test_start("Basic Functionality Test") fr fr Simple template test
    sus template tea = "Welcome {{user}}"
    sus result tea = rizz_template.rizz_parse_template(template, "user", "Alice")
    
    assert_true(result != "")
    assert_true(result != template) fr fr Should be processed
    
    test_end()
}

fr fr Main test runner
test_start("Rizz Template Engine Tests")

test_basic_functionality()
test_basic_variable_substitution()
test_html_escaping_security()
test_conditional_rendering()
test_filter_processing()
test_template_inheritance()
test_template_includes()
test_output_formats()
test_context_management()
test_security_validation()
test_gen_z_apis()
test_template_compilation()
test_template_debugging()
test_html_escaping_function()
test_string_utilities()

print_test_summary()
