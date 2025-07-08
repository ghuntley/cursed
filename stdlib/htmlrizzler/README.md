# htmlrizzler - HTML Parsing Module

A pure CURSED HTML parsing module for DOM tree manipulation, element selection, and attribute handling.

## Features

- **DOM Tree Parsing**: Parse HTML strings into structured DOM trees
- **Element Selection**: Find elements by tag, class, ID, and other criteria
- **Attribute Handling**: Get, set, and check HTML attributes
- **Text Content Extraction**: Extract clean text from HTML elements
- **HTML Generation**: Convert DOM structures back to HTML strings
- **Pure CURSED Implementation**: No FFI dependencies, fully native

## Core Types

### HTMLElement
```cursed
struct HTMLElement {
    tag tea,                    // HTML tag name (e.g., "div", "p")
    content tea,               // Element content/text
    attributes map[tea]tea,    // HTML attributes as key-value pairs
    children []HTMLElement,    // Child elements
    parent *HTMLElement        // Parent element reference
}
```

## Core Functions

### HTML Parsing
```cursed
// Parse HTML string into DOM tree
slay parse_html(html tea) HTMLElement

// Parse HTML attributes from tag string
slay parse_attributes(tag_str tea) map[tea]tea
```

### Element Selection
```cursed
// Find first element by tag name
slay find_element_by_tag(element HTMLElement, tag tea) *HTMLElement

// Find element by ID attribute
slay find_element_by_id(element HTMLElement, id tea) *HTMLElement

// Find all elements with specific class
slay find_elements_by_class(element HTMLElement, class_name tea) []HTMLElement

// Get all elements with specific tag
slay get_elements_by_tag(element HTMLElement, tag tea) []HTMLElement
```

### Attribute Management
```cursed
// Get element attribute value
slay get_attribute(element HTMLElement, key tea) tea

// Set element attribute
slay set_attribute(element *HTMLElement, key tea, value tea)

// Check if element has attribute
slay has_attribute(element HTMLElement, key tea) lit
```

### Content Extraction
```cursed
// Get clean text content (strips HTML tags)
slay get_text_content(element HTMLElement) tea

// Get inner HTML of element
slay get_inner_html(element HTMLElement) tea

// Convert element to HTML string
slay element_to_string(element HTMLElement) tea
```

## Usage Examples

### Basic HTML Parsing
```cursed
yeet "htmlrizzler"

// Parse simple HTML
sus html tea = "<div class=\"container\">Hello World</div>"
sus element HTMLElement = parse_html(html)

vibez.spill("Tag:", element.tag)              // Output: div
vibez.spill("Class:", get_attribute(element, "class"))  // Output: container
vibez.spill("Content:", get_text_content(element))      // Output: Hello World
```

### Element Selection
```cursed
// Find elements by various criteria
sus doc HTMLElement = parse_html("<html><body><div id=\"main\"><p class=\"highlight\">Text</p></div></body></html>")

// Find by tag
sus body *HTMLElement = find_element_by_tag(doc, "body")

// Find by ID
sus main_div *HTMLElement = find_element_by_id(doc, "main")

// Find by class
sus highlights []HTMLElement = find_elements_by_class(doc, "highlight")
```

### Attribute Manipulation
```cursed
// Create element with attributes
sus img HTMLElement
img.tag = "img"
img.attributes = make_map()

// Set attributes
set_attribute(&img, "src", "image.jpg")
set_attribute(&img, "alt", "Description")
set_attribute(&img, "width", "300")

// Check attributes
yeet has_attribute(img, "src") {
    sus src tea = get_attribute(img, "src")
    vibez.spill("Image source:", src)
}
```

### HTML Generation
```cursed
// Build HTML programmatically
sus link HTMLElement
link.tag = "a"
link.content = "Visit Example"
link.attributes = make_map()
set_attribute(&link, "href", "https://example.com")
set_attribute(&link, "target", "_blank")

// Generate HTML string
sus html tea = element_to_string(link)
vibez.spill(html)  // Output: <a href="https://example.com" target="_blank">Visit Example</a>
```

### Complex DOM Traversal
```cursed
// Parse complex HTML structure
sus complex_html tea = `
<div class="container">
    <header>
        <h1>Title</h1>
        <nav>
            <a href="/home">Home</a>
            <a href="/about">About</a>
        </nav>
    </header>
    <main>
        <article class="post">
            <h2>Article Title</h2>
            <p>Article content...</p>
        </article>
    </main>
</div>
`

sus doc HTMLElement = parse_html(complex_html)

// Find all links
sus links []HTMLElement = get_elements_by_tag(doc, "a")
bestie i := 0; i < len(links); i++ {
    sus href tea = get_attribute(links[i], "href")
    sus text tea = get_text_content(links[i])
    vibez.spill("Link:", text, "->", href)
}

// Find articles
sus articles []HTMLElement = find_elements_by_class(doc, "post")
bestie i := 0; i < len(articles); i++ {
    sus title *HTMLElement = find_element_by_tag(articles[i], "h2")
    yeet title != cringe {
        vibez.spill("Article:", get_text_content(*title))
    }
}
```

## Utility Functions

The module includes several utility functions for string manipulation:

```cursed
// String utilities
slay find_char(str tea, ch sip) normie           // Find character position
slay split_string(str tea, delimiter sip) []tea  // Split string by delimiter
slay trim_string(str tea) tea                    // Remove leading/trailing spaces
slay contains_string(str tea, substr tea) lit    // Check if string contains substring
```

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/htmlrizzler/test_htmlrizzler.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/htmlrizzler/test_htmlrizzler.csd
./test_htmlrizzler

# Run specific HTML parsing tests
cargo run --bin cursed test --filter htmlrizzler
```

## Implementation Notes

- **Pure CURSED**: No FFI dependencies, fully implemented in CURSED
- **Memory Safe**: Uses CURSED's memory management and type system
- **Extensible**: Easy to add new parsing features and selectors
- **Performance**: Optimized for both interpretation and compilation modes
- **Standards Compliant**: Follows HTML parsing best practices

## Limitations

- Basic HTML parsing (not full HTML5 spec compliance)
- Limited CSS selector support (tag, class, ID only)
- No JavaScript execution or DOM events
- Simple attribute parsing (handles quoted attributes)

## Future Enhancements

- CSS selector engine
- HTML5 semantic parsing
- DOM manipulation methods
- XML/XHTML support
- Performance optimizations for large documents

## Security Considerations

- Input sanitization for XSS prevention
- Safe attribute handling
- Proper character encoding support
- HTML entity decoding
