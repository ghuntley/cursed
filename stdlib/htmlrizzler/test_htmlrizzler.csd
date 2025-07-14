yeet "testz"
yeet "htmlrizzler"

# Test HTML element creation
test_start("HTML element creation tests")

# Test basic element creation
sus div_element = htmlrizzler.create_element("div", "Hello World")
assert_eq_string(div_element.tag_name, "div")
assert_eq_string(div_element.content, "Hello World")
assert_false(div_element.is_self_closing)

# Test self-closing element
sus img_element = htmlrizzler.create_element("img", "")
assert_eq_string(img_element.tag_name, "img")
assert_true(img_element.is_self_closing)

# Test element with attributes
sus link_element = htmlrizzler.create_element_with_attrs("a", "href=\"https://example.com\"", "Click here")
assert_eq_string(link_element.tag_name, "a")
assert_eq_string(link_element.attributes, "href=\"https://example.com\"")
assert_eq_string(link_element.content, "Click here")

vibez.spill("✅ HTML element creation tests passed")

# Test HTML entity encoding/decoding
test_start("HTML entity encoding/decoding tests")

sus dangerous_text = "<script>alert('xss')</script>"
sus encoded_text = htmlrizzler.encode_html_entities(dangerous_text)
assert_true(stringz.contains(encoded_text, "&lt;"))
assert_true(stringz.contains(encoded_text, "&gt;"))
assert_false(stringz.contains(encoded_text, "<script>"))

sus entity_text = "&lt;div&gt;Hello &amp; World&lt;/div&gt;"
sus decoded_text = htmlrizzler.decode_html_entities(entity_text)
assert_eq_string(decoded_text, "<div>Hello & World</div>")

vibez.spill("✅ HTML entity encoding/decoding tests passed")

# Test HTML generation
test_start("HTML generation tests")

sus p_element = htmlrizzler.create_element("p", "This is a paragraph")
sus p_html = htmlrizzler.generate_html(p_element)
assert_eq_string(p_html, "<p>This is a paragraph</p>")

sus br_element = htmlrizzler.create_element("br", "")
sus br_html = htmlrizzler.generate_html(br_element)
assert_eq_string(br_html, "<br />")

sus div_with_attrs = htmlrizzler.create_element_with_attrs("div", "class=\"container\"", "Content")
sus div_html = htmlrizzler.generate_html(div_with_attrs)
assert_eq_string(div_html, "<div class=\"container\">Content</div>")

vibez.spill("✅ HTML generation tests passed")

# Test pretty printing
test_start("HTML pretty printing tests")

sus header_element = htmlrizzler.create_element("h1", "Main Title")
sus pretty_html = htmlrizzler.generate_pretty_html(header_element, 1)
assert_true(stringz.contains(pretty_html, "  <h1>"))
assert_true(stringz.contains(pretty_html, "    Main Title"))
assert_true(stringz.contains(pretty_html, "  </h1>"))

vibez.spill("✅ HTML pretty printing tests passed")

# Test HTML parsing
test_start("HTML parsing tests")

sus simple_html = "<h1>Hello World</h1>"
sus parsed_element = htmlrizzler.parse_html(simple_html)
assert_eq_string(parsed_element.tag_name, "h1")
assert_eq_string(parsed_element.content, "Hello World")

sus html_with_attrs = "<div class=\"test\" id=\"main\">Content here</div>"
sus parsed_div = htmlrizzler.parse_html(html_with_attrs)
assert_eq_string(parsed_div.tag_name, "div")
assert_true(stringz.contains(parsed_div.attributes, "class=\"test\""))
assert_true(stringz.contains(parsed_div.attributes, "id=\"main\""))
assert_eq_string(parsed_div.content, "Content here")

vibez.spill("✅ HTML parsing tests passed")

# Test CSS selector matching
test_start("CSS selector matching tests")

sus test_div = htmlrizzler.create_element_with_attrs("div", "class=\"highlight\"", "Test content")
assert_true(htmlrizzler.matches_selector(test_div, "div"))
assert_true(htmlrizzler.matches_selector(test_div, ".highlight"))
assert_false(htmlrizzler.matches_selector(test_div, ".other"))

sus test_span = htmlrizzler.create_element_with_attrs("span", "id=\"special\"", "Special text")
assert_true(htmlrizzler.matches_selector(test_span, "span"))
assert_true(htmlrizzler.matches_selector(test_span, "#special"))
assert_false(htmlrizzler.matches_selector(test_span, "#other"))

vibez.spill("✅ CSS selector matching tests passed")

# Test XSS sanitization
test_start("XSS sanitization tests")

sus malicious_html = "<script>alert('xss')</script><div>Safe content</div>"
sus sanitized_html = htmlrizzler.sanitize_html(malicious_html)
assert_false(stringz.contains(sanitized_html, "<script>"))
assert_true(stringz.contains(sanitized_html, "<div>Safe content</div>"))

sus js_protocol = "<a href=\"javascript:alert('xss')\">Click</a>"
sus safe_link = htmlrizzler.sanitize_html(js_protocol)
assert_false(stringz.contains(safe_link, "javascript:"))

vibez.spill("✅ XSS sanitization tests passed")

# Test HTML validation
test_start("HTML validation tests")

sus valid_html = "<div><p>Hello</p></div>"
assert_true(htmlrizzler.validate_html(valid_html))

sus invalid_html = "<div><p>Hello</div></p>"
assert_false(htmlrizzler.validate_html(invalid_html))

sus self_closing_html = "<br /><img src=\"test.jpg\" />"
assert_true(htmlrizzler.validate_html(self_closing_html))

vibez.spill("✅ HTML validation tests passed")

# Test complete document generation
test_start("Complete HTML document tests")

sus document_html = htmlrizzler.create_html_document("Test Page", "<h1>Welcome</h1>")
assert_true(stringz.contains(document_html, "<!DOCTYPE html>"))
assert_true(stringz.contains(document_html, "<title>Test Page</title>"))
assert_true(stringz.contains(document_html, "<h1>Welcome</h1>"))
assert_true(stringz.contains(document_html, "charset=\"UTF-8\""))

vibez.spill("✅ Complete HTML document tests passed")

# Test Gen Z Enhanced APIs (fire features fr fr)
test_start("Gen Z Enhanced API tests")

# Test sending div
sus sending_div = htmlrizzler.create_sending_div("This content is sending!", "highlight premium")
assert_eq_string(sending_div.tag_name, "div")
assert_true(stringz.contains(sending_div.attributes, "highlight premium"))
assert_eq_string(sending_div.content, "This content is sending!")

# Test fire button
sus fire_button = htmlrizzler.create_fire_button("Click me!", "handleClick()")
assert_eq_string(fire_button.tag_name, "button")
assert_true(stringz.contains(fire_button.attributes, "handleClick()"))
assert_eq_string(fire_button.content, "Click me!")

# Test lowkey input
sus lowkey_input = htmlrizzler.create_lowkey_input("text", "Enter your name", "username")
assert_eq_string(lowkey_input.tag_name, "input")
assert_true(stringz.contains(lowkey_input.attributes, "type=\"text\""))
assert_true(stringz.contains(lowkey_input.attributes, "name=\"username\""))

# Test goated link
sus goated_link = htmlrizzler.create_goated_link("https://github.com", "Check this out", "_blank")
assert_eq_string(goated_link.tag_name, "a")
assert_true(stringz.contains(goated_link.attributes, "https://github.com"))
assert_true(stringz.contains(goated_link.attributes, "_blank"))

# Test iconic image
sus iconic_image = htmlrizzler.create_iconic_image("logo.png", "Company Logo", "responsive-img")
assert_eq_string(iconic_image.tag_name, "img")
assert_true(stringz.contains(iconic_image.attributes, "logo.png"))
assert_true(stringz.contains(iconic_image.attributes, "Company Logo"))
assert_true(iconic_image.is_self_closing)

vibez.spill("✅ Gen Z Enhanced API tests passed")

# Test HTML utilities
test_start("HTML utility tests")

# Test HTML minification
sus unminified_html = "  <div>  \n  <p>Hello</p>  \n  </div>  "
sus minified_html = htmlrizzler.minify_html(unminified_html)
assert_false(stringz.contains(minified_html, "\n"))
assert_true(stringz.length(minified_html) < stringz.length(unminified_html))

# Test text extraction
sus html_with_text = "<div><p>Hello <strong>World</strong>!</p></div>"
sus extracted_text = htmlrizzler.extract_text_content(html_with_text)
assert_eq_string(extracted_text, "Hello World!")

# Test element counting
sus multi_element_html = "<div><p>One</p><span>Two</span><h1>Three</h1></div>"
sus element_count = htmlrizzler.count_elements(multi_element_html)
assert_true(element_count >= 4)  # div, p, span, h1

vibez.spill("✅ HTML utility tests passed")

# Test security features
test_start("Security feature tests")

# Test dangerous tag detection
assert_true(htmlrizzler.is_dangerous_tag("script"))
assert_true(htmlrizzler.is_dangerous_tag("iframe"))
assert_false(htmlrizzler.is_dangerous_tag("div"))
assert_false(htmlrizzler.is_dangerous_tag("p"))

# Test self-closing tag detection
assert_true(htmlrizzler.is_self_closing_tag("br"))
assert_true(htmlrizzler.is_self_closing_tag("img"))
assert_false(htmlrizzler.is_self_closing_tag("div"))
assert_false(htmlrizzler.is_self_closing_tag("p"))

vibez.spill("✅ Security feature tests passed")

# Test advanced HTML generation
test_start("Advanced HTML generation tests")

# Test organized table creation
sus table_element = htmlrizzler.create_organized_table("Name|Age", "John|25", "data-table")
assert_eq_string(table_element.tag_name, "table")
assert_true(stringz.contains(table_element.content, "<thead>"))
assert_true(stringz.contains(table_element.content, "<tbody>"))
assert_true(stringz.contains(table_element.attributes, "data-table"))

# Test valid form creation
sus form_element = htmlrizzler.create_valid_form("/submit", "POST", "<input type=\"text\" name=\"data\">")
assert_eq_string(form_element.tag_name, "form")
assert_true(stringz.contains(form_element.attributes, "/submit"))
assert_true(stringz.contains(form_element.attributes, "POST"))

vibez.spill("✅ Advanced HTML generation tests passed")

# Test complex parsing scenarios
test_start("Complex HTML parsing tests")

# Test nested elements
sus nested_html = "<div class=\"outer\"><p>Paragraph <em>emphasis</em> text</p></div>"
sus nested_parsed = htmlrizzler.parse_html(nested_html)
assert_eq_string(nested_parsed.tag_name, "div")
assert_true(stringz.contains(nested_parsed.attributes, "outer"))

# Test self-closing tags in parsing
sus self_closing_html = "<img src=\"test.jpg\" alt=\"Test\" />"
sus self_closing_parsed = htmlrizzler.parse_html(self_closing_html)
assert_eq_string(self_closing_parsed.tag_name, "img")
assert_true(self_closing_parsed.is_self_closing)

vibez.spill("✅ Complex HTML parsing tests passed")

# Test edge cases
test_start("Edge case tests")

# Test empty content
sus empty_element = htmlrizzler.create_element("div", "")
sus empty_html = htmlrizzler.generate_html(empty_element)
assert_eq_string(empty_html, "<div></div>")

# Test special characters in attributes
sus special_attrs = htmlrizzler.create_element_with_attrs("div", "data-info=\"hello & world\"", "test")
sus special_html = htmlrizzler.generate_html(special_attrs)
assert_true(stringz.contains(special_html, "hello & world"))

vibez.spill("✅ Edge case tests passed")

# Performance and stress tests
test_start("Performance tests")

# Test large content handling
sus large_content = "This is a very long content string that tests how well the HTML generator handles larger amounts of text content"
sus large_element = htmlrizzler.create_element("div", large_content)
sus large_html = htmlrizzler.generate_html(large_element)
assert_true(stringz.contains(large_html, large_content))

# Test multiple attribute handling
sus multi_attr_element = htmlrizzler.create_element_with_attrs("input", "type=\"text\" class=\"form-control\" id=\"username\" placeholder=\"Enter username\"", "")
sus multi_attr_html = htmlrizzler.generate_html(multi_attr_element)
assert_true(stringz.contains(multi_attr_html, "type=\"text\""))
assert_true(stringz.contains(multi_attr_html, "form-control"))

vibez.spill("✅ Performance tests passed")

print_test_summary()
vibez.spill("🔥 All htmlrizzler tests passed! This module is absolutely fire! 🔥")
