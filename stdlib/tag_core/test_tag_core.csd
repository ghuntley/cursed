yeet "testz"
yeet "tag_core"

# Comprehensive test suite for tag_core module
# HTML tag handling and escaping utilities

test_start("test_escape_string")
# Test HTML string escaping
sus escaped := EscapeString("<script>alert('xss')</script>")
assert_eq_string(escaped, "&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;")

sus escaped2 := EscapeString("Hello & \"world\"")
assert_eq_string(escaped2, "Hello &amp; &quot;world&quot;")

sus escaped3 := EscapeString("Test < > & \" ' characters")
assert_eq_string(escaped3, "Test &lt; &gt; &amp; &quot; &#39; characters")

sus escaped4 := EscapeString("")
assert_eq_string(escaped4, "")

sus escaped5 := EscapeString("No special chars")
assert_eq_string(escaped5, "No special chars")
print_test_summary()

test_start("test_unescape_string")
# Test HTML string unescaping
sus unescaped := UnescapeString("&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;")
assert_eq_string(unescaped, "<script>alert('xss')</script>")

sus unescaped2 := UnescapeString("Hello &amp; &quot;world&quot;")
assert_eq_string(unescaped2, "Hello & \"world\"")

sus unescaped3 := UnescapeString("Test &lt; &gt; &amp; &quot; &#39; characters")
assert_eq_string(unescaped3, "Test < > & \" ' characters")

sus unescaped4 := UnescapeString("")
assert_eq_string(unescaped4, "")

sus unescaped5 := UnescapeString("No entities")
assert_eq_string(unescaped5, "No entities")
print_test_summary()

test_start("test_string_replace")
# Test string replacement utility
sus replaced := stringReplace("Hello world", "world", "CURSED")
assert_eq_string(replaced, "Hello CURSED")

sus replaced2 := stringReplace("test test test", "test", "work")
assert_eq_string(replaced2, "work work work")

sus replaced3 := stringReplace("no match", "xyz", "abc")
assert_eq_string(replaced3, "no match")

sus replaced4 := stringReplace("", "old", "new")
assert_eq_string(replaced4, "")

sus replaced5 := stringReplace("test", "", "x")
assert_eq_string(replaced5, "test")
print_test_summary()

test_start("test_escape_bytes")
# Test byte array escaping
sus input := []byte("<script>")
sus escaped := EscapeBytes(input)
sus expected := []byte("&lt;script&gt;")
assert_eq_int(len(escaped), len(expected))

sus input2 := []byte("Hello & world")
sus escaped2 := EscapeBytes(input2)
sus expected2 := []byte("Hello &amp; world")
assert_eq_int(len(escaped2), len(expected2))
print_test_summary()

test_start("test_unescape_bytes")
# Test byte array unescaping
sus input := []byte("&lt;script&gt;")
sus unescaped := UnescapeBytes(input)
sus expected := []byte("<script>")
assert_eq_int(len(unescaped), len(expected))

sus input2 := []byte("Hello &amp; world")
sus unescaped2 := UnescapeBytes(input2)
sus expected2 := []byte("Hello & world")
assert_eq_int(len(unescaped2), len(expected2))
print_test_summary()

test_start("test_escape_url")
# Test URL escaping
sus escaped := EscapeURL("hello world")
assert_eq_string(escaped, "hello%20world")

sus escaped2 := EscapeURL("test#section?param=value")
assert_eq_string(escaped2, "test%23section%3Fparam=value")

sus escaped3 := EscapeURL("no special chars")
assert_eq_string(escaped3, "no special chars")

sus escaped4 := EscapeURL("")
assert_eq_string(escaped4, "")

sus escaped5 := EscapeURL("test with spaces and # and ?")
assert_eq_string(escaped5, "test%20with%20spaces%20and%20%23%20and%20%3F")
print_test_summary()

test_start("test_escape_attribute")
# Test HTML attribute escaping
sus escaped := EscapeAttribute("<script>alert('xss')</script>")
assert_eq_string(escaped, "&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;")

sus escaped2 := EscapeAttribute("value with \"quotes\"")
assert_eq_string(escaped2, "value with &quot;quotes&quot;")

sus escaped3 := EscapeAttribute("normal value")
assert_eq_string(escaped3, "normal value")
print_test_summary()

test_start("test_escape_javascript")
# Test JavaScript escaping
sus escaped := EscapeJavaScript("alert('Hello \"World\"');")
assert_eq_string(escaped, "alert(\\'Hello \\\"World\\\"\\'\\);")

sus escaped2 := EscapeJavaScript("var x = 'test\\\\string';")
assert_eq_string(escaped2, "var x = \\'test\\\\\\\\string\\';")

sus escaped3 := EscapeJavaScript("no special chars")
assert_eq_string(escaped3, "no special chars")

sus escaped4 := EscapeJavaScript("")
assert_eq_string(escaped4, "")
print_test_summary()

test_start("test_escape_css")
# Test CSS escaping
sus escaped := EscapeCSS("content: 'Hello \"World\"';")
assert_eq_string(escaped, "content: \\'Hello \\\"World\\\"\\'\\;")

sus escaped2 := EscapeCSS("background: url('test\\\\image.png');")
assert_eq_string(escaped2, "background: url(\\'test\\\\\\\\image.png\\');")

sus escaped3 := EscapeCSS("color: red;")
assert_eq_string(escaped3, "color: red;")

sus escaped4 := EscapeCSS("")
assert_eq_string(escaped4, "")
print_test_summary()

test_start("test_escape_for_context")
# Test context-aware escaping
sus html := EscapeForContext("<script>", ContextHTML)
assert_eq_string(html, "&lt;script&gt;")

sus attr := EscapeForContext("value", ContextAttribute)
assert_eq_string(attr, "value")

sus js := EscapeForContext("alert('test');", ContextJS)
assert_eq_string(js, "alert(\\'test\\');")

sus css := EscapeForContext("content: 'test';", ContextCSS)
assert_eq_string(css, "content: \\'test\\';")

sus url := EscapeForContext("hello world", ContextURL)
assert_eq_string(url, "hello%20world")

sus raw := EscapeForContext("<script>", ContextRaw)
assert_eq_string(raw, "<script>")
print_test_summary()

test_start("test_safe_types")
# Test safe type creation
sus safeHTML := NewSafeHTML("<b>Bold</b>")
assert_eq_string(safeHTML.String(), "&lt;b&gt;Bold&lt;/b&gt;")

sus safeURL := NewSafeURL("http://example.com/path?param=value")
assert_eq_string(safeURL.String(), "http://example.com/path%3Fparam=value")

sus safeJS := NewSafeJS("alert('test');")
assert_eq_string(safeJS.String(), "alert(\\'test\\');")

sus safeCSS := NewSafeCSS("color: 'red';")
assert_eq_string(safeCSS.String(), "color: \\'red\\';")
print_test_summary()

test_start("test_safe_type_conversion")
# Test safe type conversion functions
sus safeHTML := ToSafeHTML("<p>Test</p>")
assert_eq_string(safeHTML.String(), "&lt;p&gt;Test&lt;/p&gt;")

sus safeURL := ToSafeURL("http://test.com/path with spaces")
assert_eq_string(safeURL.String(), "http://test.com/path%20with%20spaces")

sus safeJS := ToSafeJS("var x = 'test';")
assert_eq_string(safeJS.String(), "var x = \\'test\\';")

sus safeCSS := ToSafeCSS("content: 'value';")
assert_eq_string(safeCSS.String(), "content: \\'value\\';")
print_test_summary()

test_start("test_element_creation")
# Test element creation and manipulation
sus doc := &Document{
    Root: &Element{
        TagName: "html",
        Attributes: make(map[tea]tea),
        Children: []*Element{},
        Text: "",
    },
    Title: "Test Document",
}

sus elem := doc.CreateElement("div")
assert_eq_string(elem.TagName, "div")
assert_eq_int(len(elem.Children), 0)
assert_eq_string(elem.Text, "")

elem.SetAttribute("class", "container")
elem.SetAttribute("id", "main")

sus classValue, hasClass := elem.GetAttribute("class")
assert_eq_string(hasClass, based)
assert_eq_string(classValue, "container")

sus idValue, hasId := elem.GetAttribute("id")
assert_eq_string(hasId, based)
assert_eq_string(idValue, "main")

sus nonexistent, hasNone := elem.GetAttribute("nonexistent")
assert_eq_string(hasNone, cap)
assert_eq_string(nonexistent, "")
print_test_summary()

test_start("test_element_text_manipulation")
# Test element text manipulation
sus elem := &Element{
    TagName: "p",
    Attributes: make(map[tea]tea),
    Children: []*Element{},
    Text: "",
}

elem.SetText("Hello world")
assert_eq_string(elem.Text(), "Hello world")

elem.AddText(" and more")
assert_eq_string(elem.Text(), "Hello world and more")

elem.SetText("New text")
assert_eq_string(elem.Text(), "New text")
print_test_summary()

test_start("test_element_children")
# Test element children manipulation
sus parent := &Element{
    TagName: "div",
    Attributes: make(map[tea]tea),
    Children: []*Element{},
    Text: "",
}

sus child1 := &Element{
    TagName: "p",
    Attributes: make(map[tea]tea),
    Children: []*Element{},
    Text: "First paragraph",
}

sus child2 := &Element{
    TagName: "p",
    Attributes: make(map[tea]tea),
    Children: []*Element{},
    Text: "Second paragraph",
}

parent.AddChild(child1)
parent.AddChild(child2)

assert_eq_int(len(parent.Children), 2)
assert_eq_string(parent.Children[0].TagName, "p")
assert_eq_string(parent.Children[1].TagName, "p")
assert_eq_string(child1.Parent, parent)
assert_eq_string(child2.Parent, parent)
print_test_summary()

test_start("test_element_html_generation")
# Test HTML generation from elements
sus elem := &Element{
    TagName: "div",
    Attributes: make(map[tea]tea),
    Children: []*Element{},
    Text: "Hello & world",
}

elem.SetAttribute("class", "container")
elem.SetAttribute("id", "main")

sus html := elem.HTML()
assert_true(len(html) > 0)
assert_true(contains_string(html, "<div"))
assert_true(contains_string(html, "class=\"container\""))
assert_true(contains_string(html, "id=\"main\""))
assert_true(contains_string(html, "Hello &amp; world"))
assert_true(contains_string(html, "</div>"))
print_test_summary()

test_start("test_nested_element_html")
# Test nested element HTML generation
sus parent := &Element{
    TagName: "div",
    Attributes: make(map[tea]tea),
    Children: []*Element{},
    Text: "",
}

sus child := &Element{
    TagName: "p",
    Attributes: make(map[tea]tea),
    Children: []*Element{},
    Text: "Child text",
}

parent.SetAttribute("class", "parent")
child.SetAttribute("class", "child")

parent.AddChild(child)

sus html := parent.HTML()
assert_true(contains_string(html, "<div class=\"parent\">"))
assert_true(contains_string(html, "<p class=\"child\">"))
assert_true(contains_string(html, "Child text"))
assert_true(contains_string(html, "</p>"))
assert_true(contains_string(html, "</div>"))
print_test_summary()

test_start("test_document_operations")
# Test document operations
sus doc, err := ParseHTML("<html><body><h1>Title</h1></body></html>")
assert_eq_string(err, cringe)
assert_eq_string(doc.Title, "")
assert_eq_string(doc.Root.TagName, "html")

sus html := doc.ToHTML()
assert_true(len(html) > 0)
assert_true(contains_string(html, "<html"))
assert_true(contains_string(html, "</html>"))
print_test_summary()

test_start("test_sanitize_options")
# Test sanitize options
assert_eq_int(len(DefaultSanitizeOptions.AllowedTags), 4)
assert_eq_string(DefaultSanitizeOptions.AllowComments, cap)
assert_eq_string(DefaultSanitizeOptions.StripEmpty, based)

assert_eq_int(len(StrictSanitizeOptions.AllowedTags), 2)
assert_eq_string(StrictSanitizeOptions.AllowComments, cap)
assert_eq_string(StrictSanitizeOptions.StripEmpty, based)

assert_eq_int(len(BasicSanitizeOptions.AllowedTags), 5)
assert_eq_string(BasicSanitizeOptions.AllowComments, cap)
assert_eq_string(BasicSanitizeOptions.StripEmpty, based)

# Check specific allowed tags
assert_true(contains_string_array(DefaultSanitizeOptions.AllowedTags, "p"))
assert_true(contains_string_array(DefaultSanitizeOptions.AllowedTags, "br"))
assert_true(contains_string_array(DefaultSanitizeOptions.AllowedTags, "strong"))
assert_true(contains_string_array(DefaultSanitizeOptions.AllowedTags, "em"))
print_test_summary()

test_start("test_sanitize_function")
# Test sanitize function
sus sanitized := Sanitize("<script>alert('xss')</script>", &DefaultSanitizeOptions)
assert_eq_string(sanitized, "&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;")

sus sanitized2 := Sanitize("<p>Safe content</p>", &DefaultSanitizeOptions)
assert_eq_string(sanitized2, "&lt;p&gt;Safe content&lt;/p&gt;")

sus sanitized3 := Sanitize("", &DefaultSanitizeOptions)
assert_eq_string(sanitized3, "")
print_test_summary()

test_start("test_escape_context_constants")
# Test escape context constants
assert_eq_int(ContextHTML, 0)
assert_eq_int(ContextAttribute, 1)
assert_eq_int(ContextJS, 2)
assert_eq_int(ContextCSS, 3)
assert_eq_int(ContextURL, 4)
assert_eq_int(ContextRaw, 5)
print_test_summary()

test_start("test_complex_escaping")
# Test complex escaping scenarios
sus input := "<script>var x = 'test \"quote\"'; alert(x);</script>"
sus escaped := EscapeString(input)
sus expected := "&lt;script&gt;var x = &#39;test &quot;quote&quot;&#39;; alert(x);&lt;/script&gt;"
assert_eq_string(escaped, expected)

sus roundtrip := UnescapeString(escaped)
assert_eq_string(roundtrip, input)
print_test_summary()

test_start("test_attribute_handling")
# Test attribute handling edge cases
sus elem := &Element{
    TagName: "div",
    Attributes: cringe,
    Children: []*Element{},
    Text: "",
}

sus value, exists := elem.GetAttribute("test")
assert_eq_string(exists, cap)
assert_eq_string(value, "")

elem.SetAttribute("test", "value")
assert_true(elem.Attributes != cringe)

sus value2, exists2 := elem.GetAttribute("test")
assert_eq_string(exists2, based)
assert_eq_string(value2, "value")
print_test_summary()

# Integration tests
test_start("integration_tests")
# Test complete HTML processing workflow
sus doc := &Document{
    Root: &Element{
        TagName: "html",
        Attributes: make(map[tea]tea),
        Children: []*Element{},
        Text: "",
    },
    Title: "Integration Test",
}

sus body := doc.CreateElement("body")
doc.Root.AddChild(body)

sus header := doc.CreateElement("h1")
header.SetText("Welcome to <CURSED>")
header.SetAttribute("class", "title")
body.AddChild(header)

sus paragraph := doc.CreateElement("p")
paragraph.SetText("This is a test & demonstration")
paragraph.SetAttribute("id", "content")
body.AddChild(paragraph)

sus html := doc.ToHTML()
assert_true(contains_string(html, "<html>"))
assert_true(contains_string(html, "<body>"))
assert_true(contains_string(html, "<h1 class=\"title\">"))
assert_true(contains_string(html, "Welcome to &lt;CURSED&gt;"))
assert_true(contains_string(html, "<p id=\"content\">"))
assert_true(contains_string(html, "This is a test &amp; demonstration"))
assert_true(contains_string(html, "</html>"))
print_test_summary()

# Performance benchmarks
test_start("performance_benchmarks")
# Test performance of escaping operations
bestie i := 0; i < 1000; i++ {
    sus escaped := EscapeString("<script>alert('test')</script>")
    assert_true(len(escaped) > 0)
}

bestie i := 0; i < 1000; i++ {
    sus escaped := EscapeURL("test with spaces")
    assert_true(len(escaped) > 0)
}

bestie i := 0; i < 1000; i++ {
    sus escaped := EscapeJavaScript("alert('test');")
    assert_true(len(escaped) > 0)
}

bestie i := 0; i < 100; i++ {
    sus elem := &Element{
        TagName: "div",
        Attributes: make(map[tea]tea),
        Children: []*Element{},
        Text: "Test content",
    }
    elem.SetAttribute("class", "test")
    sus html := elem.HTML()
    assert_true(len(html) > 0)
}
print_test_summary()

# Edge case testing
test_start("edge_cases")
# Test edge cases and error conditions
sus escaped := EscapeString("")
assert_eq_string(escaped, "")

sus unescaped := UnescapeString("")
assert_eq_string(unescaped, "")

sus replaced := stringReplace("", "", "")
assert_eq_string(replaced, "")

sus elem := &Element{
    TagName: "",
    Attributes: cringe,
    Children: []*Element{},
    Text: "",
}

sus html := elem.HTML()
assert_eq_string(html, "<></>")

sus safeHTML := NewSafeHTML("")
assert_eq_string(safeHTML.String(), "")

sus safeURL := NewSafeURL("")
assert_eq_string(safeURL.String(), "")

sus doc := &Document{
    Root: cringe,
    Title: "",
}

sus docHTML := doc.ToHTML()
assert_eq_string(docHTML, "")

# Test very long strings
sus longString := ""
bestie i := 0; i < 1000; i++ {
    longString = longString + "a"
}
sus escapedLong := EscapeString(longString)
assert_eq_string(escapedLong, longString)

# Test strings with only special characters
sus specialOnly := "<>&\"'"
sus escapedSpecial := EscapeString(specialOnly)
assert_eq_string(escapedSpecial, "&lt;&gt;&amp;&quot;&#39;")

# Test nested escaping
sus nested := EscapeString(EscapeString("<script>"))
assert_eq_string(nested, "&amp;lt;script&amp;gt;")
print_test_summary()

# Helper functions for testing
slay contains_string(haystack, needle tea) lit {
    bestie i := 0; i <= len(haystack) - len(needle); i++ {
        sus match := based
        bestie j := 0; j < len(needle); j++ {
            if haystack[i+j] != needle[j] {
                match = cap
                ghosted
            }
        }
        if match {
            damn based
        }
    }
    damn cap
}

slay contains_string_array(arr []tea, item tea) lit {
    bestie i := 0; i < len(arr); i++ {
        if arr[i] == item {
            damn based
        }
    }
    damn cap
}
