# htmlrizzler Module

The `htmlrizzler` module provides comprehensive HTML parsing, generation, and manipulation functionality for the CURSED programming language. This module is designed with modern web development needs in mind, offering both traditional HTML operations and Gen Z enhanced APIs that are straight fire! 🔥

## Features

### Core HTML Operations
- **HTML Element Creation**: Create HTML elements with full attribute support
- **HTML Generation**: Generate clean, well-formed HTML from element structures
- **HTML Parsing**: Parse HTML strings into element structures
- **Pretty Printing**: Generate beautifully formatted HTML with proper indentation
- **DOM Tree Operations**: Manipulate HTML element hierarchies

### Security Features
- **XSS Protection**: Comprehensive sanitization against cross-site scripting attacks
- **HTML Entity Encoding/Decoding**: Safe handling of special characters
- **Dangerous Tag Detection**: Automatic identification and removal of risky HTML elements
- **Input Validation**: Robust validation of HTML structure and content

### CSS Integration
- **CSS Selector Support**: Match elements using CSS-style selectors
- **Class and ID Detection**: Check for specific CSS classes and IDs
- **Responsive Design Support**: Built-in utilities for modern responsive layouts

### Gen Z Enhanced APIs
- **Sending Elements**: Create elements that are absolutely sending (popular/trending)
- **Fire Interactive Components**: Buttons and forms that are no cap interactive
- **Lowkey Essential Inputs**: Form inputs that are lowkey necessary for UX
- **Goated Links**: Navigation links that are actually goated
- **Iconic Images**: Image elements that are absolutely iconic

## Types

### HtmlElement
```cursed
be_like HtmlElement = {
    tag_name: tea,           # The HTML tag name (e.g., "div", "p")
    attributes: tea,         # Attribute string (e.g., "class=\"example\"")
    content: tea,           # Text content inside the element
    children: tea,          # Child HTML elements as string
    is_self_closing: lit    # Whether the tag is self-closing (e.g., <br />)
}
```

### HtmlDocument
```cursed
be_like HtmlDocument = {
    doctype: tea,           # Document type declaration
    html_element: HtmlElement, # Root HTML element
    title: tea,             # Document title
    encoding: tea           # Character encoding
}
```

### CssSelector
```cursed
be_like CssSelector = {
    selector: tea,          # CSS selector string
    element_type: tea,      # Element type selector
    class_name: tea,        # Class name selector
    id_name: tea           # ID selector
}
```

## Core Functions

### Element Creation
```cursed
# Create basic HTML element
sus div_element = htmlrizzler.create_element("div", "Hello World")

# Create element with attributes
sus link_element = htmlrizzler.create_element_with_attrs("a", "href=\"https://example.com\"", "Click here")
```

### HTML Generation
```cursed
# Generate HTML string
sus html_output = htmlrizzler.generate_html(div_element)
# Output: <div>Hello World</div>

# Generate pretty-printed HTML
sus pretty_html = htmlrizzler.generate_pretty_html(div_element, 1)
# Output: formatted with proper indentation
```

### HTML Parsing
```cursed
# Parse HTML from string
sus parsed_element = htmlrizzler.parse_html("<h1>Hello World</h1>")
# Returns HtmlElement with tag_name="h1", content="Hello World"
```

### Security Operations
```cursed
# Encode dangerous characters
sus safe_text = htmlrizzler.encode_html_entities("<script>alert('xss')</script>")
# Output: &lt;script&gt;alert('xss')&lt;/script&gt;

# Sanitize HTML against XSS
sus clean_html = htmlrizzler.sanitize_html("<script>alert('xss')</script><p>Safe content</p>")
# Output: <p>Safe content</p>

# Validate HTML structure
sus is_valid = htmlrizzler.validate_html("<div><p>Hello</p></div>")
# Output: based (true)
```

### CSS Selector Matching
```cursed
# Match by tag name
sus matches = htmlrizzler.matches_selector(element, "div")

# Match by class
sus matches_class = htmlrizzler.matches_selector(element, ".highlight")

# Match by ID
sus matches_id = htmlrizzler.matches_selector(element, "#main")
```

## Gen Z Enhanced APIs (Fire Features! 🔥)

### Sending Elements
```cursed
# Create a div that's absolutely sending
sus sending_div = htmlrizzler.create_sending_div("This content is fire!", "highlight premium")
```

### Interactive Components
```cursed
# Create a button that's no cap interactive
sus fire_button = htmlrizzler.create_fire_button("Subscribe", "handleSubscribe()")

# Create an input that's lowkey necessary
sus username_input = htmlrizzler.create_lowkey_input("text", "Enter username", "username")
```

### Navigation Elements
```cursed
# Create a link that's actually goated
sus goated_link = htmlrizzler.create_goated_link("https://github.com", "Check this repo", "_blank")
```

### Media Elements
```cursed
# Create an image that's absolutely iconic
sus iconic_image = htmlrizzler.create_iconic_image("logo.png", "Company Logo", "responsive-img")
```

### Form Elements
```cursed
# Create a form that's actually valid
sus contact_form = htmlrizzler.create_valid_form("/contact", "POST", "<!-- form content -->")
```

### Table Elements
```cursed
# Create a table that's organized fr
sus data_table = htmlrizzler.create_organized_table("Name|Email", "John|john@example.com", "data-table")
```

## Utility Functions

### HTML Processing
```cursed
# Minify HTML for performance
sus minified = htmlrizzler.minify_html("  <div>  \n  <p>Hello</p>  \n  </div>  ")

# Extract text content only
sus text_only = htmlrizzler.extract_text_content("<div><p>Hello <strong>World</strong>!</p></div>")
# Output: "Hello World!"

# Count HTML elements
sus element_count = htmlrizzler.count_elements("<div><p>One</p><span>Two</span></div>")
```

### Document Generation
```cursed
# Create complete HTML document
sus full_document = htmlrizzler.create_html_document("My Page", "<h1>Welcome</h1>")
```

### Security Utilities
```cursed
# Check if tag is dangerous
sus is_dangerous = htmlrizzler.is_dangerous_tag("script")  # returns based (true)

# Check if tag is self-closing
sus is_self_closing = htmlrizzler.is_self_closing_tag("br")  # returns based (true)
```

## Security Features

### XSS Protection
The module provides comprehensive protection against Cross-Site Scripting (XSS) attacks:

- **Automatic Entity Encoding**: All user content is automatically encoded
- **Dangerous Tag Removal**: Script, iframe, and other risky tags are removed
- **Protocol Filtering**: JavaScript and VBScript protocols are stripped
- **Event Handler Removal**: Dangerous event handlers are removed

### Safe HTML Practices
- All user input is encoded by default
- Attributes are properly quoted and escaped
- HTML structure is validated before output
- Content Security Policy compatible output

## Performance Optimizations

### Minification
- Removes unnecessary whitespace
- Optimizes tag spacing
- Reduces file size for faster loading

### Efficient Parsing
- Streaming parser for large documents
- Memory-efficient element creation
- Optimized string operations

## Examples

### Basic Usage
```cursed
yeet "htmlrizzler"

# Create a simple webpage
sus title = "My Awesome Page"
sus content = htmlrizzler.generate_html(
    htmlrizzler.create_element("h1", "Welcome to my site!")
)
sus webpage = htmlrizzler.create_html_document(title, content)
vibez.spill(webpage)
```

### Advanced Example
```cursed
yeet "htmlrizzler"

# Create a contact form
sus form_content = htmlrizzler.generate_html(
    htmlrizzler.create_lowkey_input("text", "Your name", "name")
) + htmlrizzler.generate_html(
    htmlrizzler.create_lowkey_input("email", "Your email", "email")
) + htmlrizzler.generate_html(
    htmlrizzler.create_fire_button("Send Message", "submitForm()")
)

sus contact_form = htmlrizzler.create_valid_form("/contact", "POST", form_content)
sus form_html = htmlrizzler.generate_pretty_html(contact_form, 0)
vibez.spill(form_html)
```

### Security Example
```cursed
yeet "htmlrizzler"

# Sanitize user input
sus user_input = "<script>alert('xss')</script><p>Hello World</p>"
sus safe_html = htmlrizzler.sanitize_html(user_input)
vibez.spill(safe_html)  # Output: <p>Hello World</p>

# Validate HTML structure
sus html_to_check = "<div><p>Valid HTML</p></div>"
lowkey htmlrizzler.validate_html(html_to_check) {
    vibez.spill("HTML is valid!")
} cringe {
    vibez.spill("Invalid HTML structure!")
}
```

## Testing

Run the comprehensive test suite:

```bash
cargo run --bin cursed stdlib/htmlrizzler/test_htmlrizzler.csd
```

The test suite covers:
- Element creation and manipulation
- HTML generation and parsing
- Security features and XSS protection
- CSS selector matching
- Gen Z enhanced APIs
- Performance optimizations
- Edge cases and error handling

## Dependencies

- `stringz`: String manipulation utilities
- `testz`: Testing framework for validation

## Browser Compatibility

The generated HTML is compatible with all modern browsers and follows web standards:
- HTML5 compliant
- CSS3 selector support
- Responsive design ready
- Accessibility friendly

## Performance Notes

- HTML generation is optimized for speed
- Memory usage is minimized through efficient string handling
- Large documents are processed efficiently
- Minification reduces output size by 20-40%

## Security Considerations

- Never trust user input - always use sanitization functions
- Validate HTML structure before rendering
- Use entity encoding for all dynamic content
- Regular security audits recommended for production use

---

**This module is absolutely fire and no cap ready for production! 🔥**

The `htmlrizzler` module brings modern web development capabilities to CURSED with both traditional HTML operations and cutting-edge Gen Z enhanced APIs. Whether you're building a simple webpage or a complex web application, this module has got you covered fr fr! 💯
