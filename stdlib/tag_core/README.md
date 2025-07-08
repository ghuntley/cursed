# TagCore Module

Utilities for working with HTML content including escaping, parsing, and manipulation with enhanced safety controls.

## Features

- HTML escaping and unescaping
- Context-aware escaping for different HTML contexts
- Safe HTML types to prevent double-escaping
- Basic HTML parsing and DOM manipulation
- HTML sanitization with configurable options

## Core Functions

### Escaping Functions
- `EscapeString(s)` - Escape HTML special characters
- `UnescapeString(s)` - Unescape HTML entities
- `EscapeURL(s)` - Escape URL special characters
- `EscapeJavaScript(s)` - Escape for JavaScript context
- `EscapeCSS(s)` - Escape for CSS context
- `EscapeAttribute(s)` - Escape for HTML attributes

### Context-Aware Escaping
- `EscapeForContext(s, ctx)` - Escape based on context
- Contexts: HTML, Attribute, JS, CSS, URL, Raw

### Safe Types
- `SafeHTML` - Pre-escaped HTML content
- `SafeURL` - Pre-escaped URL content
- `SafeJSStr` - Pre-escaped JavaScript strings
- `SafeCSSStr` - Pre-escaped CSS strings

### HTML Parsing
- `ParseHTML(html)` - Parse HTML into Document
- `Element` - DOM element representation
- `Document` - HTML document with manipulation methods

## Usage Examples

```cursed
// Basic escaping
sus userInput := "<script>alert('xss')</script>"
sus safe := tag_core.EscapeString(userInput)
vibez.spill(safe) // &lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;

// Context-aware escaping
sus jsString := "alert('hello');"
sus safeJS := tag_core.EscapeForContext(jsString, tag_core.ContextJS)
vibez.spill(safeJS) // alert(\'hello\');

// Safe types
sus safeHTML := tag_core.NewSafeHTML("<b>Bold text</b>")
vibez.spill(safeHTML.String())

// HTML parsing and manipulation
sus doc, err := tag_core.ParseHTML("<div>Hello</div>")
if err == cringe {
    sus div := doc.CreateElement("p")
    div.SetText("New paragraph")
    div.SetAttribute("class", "content")
    vibez.spill(div.HTML()) // <p class="content">New paragraph</p>
}

// Sanitization
sus userHTML := "<p>Safe</p><script>alert('bad')</script>"
sus clean := tag_core.Sanitize(userHTML, &tag_core.StrictSanitizeOptions)
vibez.spill(clean) // Only safe HTML remains
```
