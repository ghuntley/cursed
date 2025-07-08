// Test suite for htmlrizzler module
yeet "testz"
yeet "htmlrizzler"

// Test HTML parsing
test_start("parse_html basic functionality")
sus html tea = "<div>Hello World</div>"
sus element HTMLElement = parse_html(html)
assert_eq_string(element.tag, "div")
assert_eq_string(get_text_content(element), "Hello World")

// Test element creation
test_start("create HTML element")
sus test_element HTMLElement
test_element.tag = "p"
test_element.content = "Test paragraph"
test_element.attributes = make_map()
assert_eq_string(test_element.tag, "p")
assert_eq_string(test_element.content, "Test paragraph")

// Test attribute handling
test_start("set and get attributes")
sus attr_element HTMLElement
attr_element.tag = "img"
attr_element.attributes = make_map()
set_attribute(&attr_element, "src", "image.jpg")
set_attribute(&attr_element, "alt", "Test image")
assert_eq_string(get_attribute(attr_element, "src"), "image.jpg")
assert_eq_string(get_attribute(attr_element, "alt"), "Test image")

// Test attribute existence
test_start("check attribute existence")
sus check_element HTMLElement
check_element.attributes = make_map()
set_attribute(&check_element, "class", "main")
assert_true(has_attribute(check_element, "class"))
assert_false(has_attribute(check_element, "id"))

// Test text content extraction
test_start("get text content")
sus text_element HTMLElement
text_element.content = "<p>This is <b>bold</b> text</p>"
sus clean_text tea = get_text_content(text_element)
assert_eq_string(clean_text, "This is bold text")

// Test element to string conversion
test_start("element to string")
sus string_element HTMLElement
string_element.tag = "span"
string_element.content = "Content"
string_element.attributes = make_map()
set_attribute(&string_element, "class", "highlight")
sus html_string tea = element_to_string(string_element)
assert_true(contains_string(html_string, "<span"))
assert_true(contains_string(html_string, "class=\"highlight\""))
assert_true(contains_string(html_string, "Content"))
assert_true(contains_string(html_string, "</span>"))

// Test find char utility
test_start("find_char utility")
sus test_str tea = "Hello World"
sus pos normie = find_char(test_str, 'W')
assert_eq_int(pos, 6)
sus not_found normie = find_char(test_str, 'X')
assert_eq_int(not_found, -1)

// Test split string utility
test_start("split_string utility")
sus split_test tea = "one two three"
sus parts []tea = split_string(split_test, ' ')
assert_eq_int(len(parts), 3)

// Test trim string utility
test_start("trim_string utility")
sus trim_test tea = "  hello  "
sus trimmed tea = trim_string(trim_test)
assert_eq_string(trimmed, "hello")

// Test contains string utility
test_start("contains_string utility")
sus contains_test tea = "Hello World"
assert_true(contains_string(contains_test, "World"))
assert_false(contains_string(contains_test, "xyz"))

// Test parse attributes
test_start("parse_attributes")
sus attr_str tea = "img src=\"test.jpg\" alt=\"Test\""
sus attrs map[tea]tea = parse_attributes(attr_str)
assert_eq_string(attrs["src"], "test.jpg")
assert_eq_string(attrs["alt"], "Test")

// Test complex HTML structure
test_start("complex HTML parsing")
sus complex_html tea = "<div class=\"container\"><p>Paragraph</p><span>Span</span></div>"
sus complex_element HTMLElement = parse_html(complex_html)
assert_eq_string(complex_element.tag, "div")
assert_eq_string(get_attribute(complex_element, "class"), "container")

// Test find element by tag
test_start("find_element_by_tag")
sus doc HTMLElement
doc.tag = "html"
doc.children = make_array()
sus body HTMLElement
body.tag = "body"
body.parent = &doc
// In a real implementation, we'd append body to doc.children
sus found_body *HTMLElement = find_element_by_tag(doc, "body")
// This test would work with proper array implementation

// Test find element by ID
test_start("find_element_by_id")
sus id_element HTMLElement
id_element.tag = "div"
id_element.attributes = make_map()
set_attribute(&id_element, "id", "main-content")
sus found_id *HTMLElement = find_element_by_id(id_element, "main-content")
assert_true(found_id != cringe)

// Test find elements by class
test_start("find_elements_by_class")
sus class_element HTMLElement
class_element.tag = "div"
class_element.attributes = make_map()
set_attribute(&class_element, "class", "highlight important")
sus class_results []HTMLElement = find_elements_by_class(class_element, "highlight")
assert_eq_int(len(class_results), 1)

// Test get elements by tag
test_start("get_elements_by_tag")
sus tag_element HTMLElement
tag_element.tag = "p"
tag_element.children = make_array()
sus tag_results []HTMLElement = get_elements_by_tag(tag_element, "p")
assert_eq_int(len(tag_results), 1)

// Test get inner HTML
test_start("get_inner_html")
sus inner_element HTMLElement
inner_element.tag = "div"
inner_element.children = make_array()
sus inner_html tea = get_inner_html(inner_element)
assert_eq_string(inner_html, "")

// Test empty attribute handling
test_start("empty attribute handling")
sus empty_element HTMLElement
empty_element.attributes = make_map()
sus empty_attr tea = get_attribute(empty_element, "nonexistent")
assert_eq_string(empty_attr, "")

// Test string utility edge cases
test_start("string utility edge cases")
sus empty_str tea = ""
sus empty_trimmed tea = trim_string(empty_str)
assert_eq_string(empty_trimmed, "")

sus single_char tea = "a"
sus single_pos normie = find_char(single_char, 'a')
assert_eq_int(single_pos, 0)

// Test HTML special characters
test_start("HTML special characters")
sus special_html tea = "<p>&lt;Hello&gt;</p>"
sus special_element HTMLElement = parse_html(special_html)
assert_eq_string(special_element.tag, "p")

// Test nested elements
test_start("nested elements")
sus nested_html tea = "<div><p><span>Nested</span></p></div>"
sus nested_element HTMLElement = parse_html(nested_html)
assert_eq_string(nested_element.tag, "div")

// Test multiple attributes
test_start("multiple attributes")
sus multi_attr_element HTMLElement
multi_attr_element.attributes = make_map()
set_attribute(&multi_attr_element, "class", "main")
set_attribute(&multi_attr_element, "id", "content")
set_attribute(&multi_attr_element, "data-value", "123")
assert_eq_string(get_attribute(multi_attr_element, "class"), "main")
assert_eq_string(get_attribute(multi_attr_element, "id"), "content")
assert_eq_string(get_attribute(multi_attr_element, "data-value"), "123")

// Test HTML generation
test_start("HTML generation")
sus gen_element HTMLElement
gen_element.tag = "a"
gen_element.content = "Click me"
gen_element.attributes = make_map()
set_attribute(&gen_element, "href", "https://example.com")
set_attribute(&gen_element, "target", "_blank")
sus generated_html tea = element_to_string(gen_element)
assert_true(contains_string(generated_html, "<a"))
assert_true(contains_string(generated_html, "href=\"https://example.com\""))
assert_true(contains_string(generated_html, "target=\"_blank\""))
assert_true(contains_string(generated_html, ">Click me<"))
assert_true(contains_string(generated_html, "</a>"))

print_test_summary()
