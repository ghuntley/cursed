# TagCore (html package)

## Overview
TagCore provides utilities for working with HTML content including escaping, parsing, and manipulation. It's inspired by Go's html package but with enhanced features for modern web development and improved safety controls against XSS.

## Core Escaping Functions

```go
// EscapeString escapes special characters in HTML text
func EscapeString(s string) string

// UnescapeString unescapes HTML entities in text
func UnescapeString(s string) string

// EscapeBytes escapes special characters in HTML text
func EscapeBytes(b []byte) []byte

// UnescapeBytes unescapes HTML entities in text
func UnescapeBytes(b []byte) []byte

// EscapeURL escapes characters for use in a URL query
func EscapeURL(s string) string

// EscapeAttribute escapes characters for use in HTML attributes
func EscapeAttribute(s string) string

// EscapeJavaScript escapes characters for use in inline JavaScript
func EscapeJavaScript(s string) string

// EscapeCSS escapes characters for use in inline CSS
func EscapeCSS(s string) string
```

## Enhanced Escaping Features

```go
// SafeWriter for optimized HTML escaping to io.Writer
type SafeWriter struct {}

// Constructor
func NewSafeWriter(w io.Writer) *SafeWriter

// Methods
func (w *SafeWriter) Write(p []byte) (n int, err error)
func (w *SafeWriter) WriteString(s string) (n int, err error)
func (w *SafeWriter) WriteAttribute(attr, value string) (n int, err error)
func (w *SafeWriter) WriteJavaScript(s string) (n int, err error)
func (w *SafeWriter) WriteCSS(s string) (n int, err error)
func (w *SafeWriter) WriteRaw(p []byte) (n int, err error) // Write without escaping
```

### Context-Aware Escaping

```go
type EscapeContext int

const (
    ContextHTML EscapeContext = iota
    ContextAttribute
    ContextJS
    ContextCSS
    ContextURL
    ContextURLQuery
    ContextRaw  // No escaping
)

// Context-aware escaping function
func EscapeForContext(s string, ctx EscapeContext) string

// Detect context from markup position
func DetectContext(html string, position int) EscapeContext
```

## HTML Safety Types

```go
// Safe HTML types that won't be escaped again
type SafeHTML string
type SafeURL string
type SafeJSStr string
type SafeCSSStr string
type SafeAttr struct {
    Name, Value string
}

// Constructor functions
func NewSafeHTML(html string) SafeHTML // Validate and sanitize
func NewSafeURL(url string) SafeURL // Validate and sanitize
func NewSafeJS(js string) SafeJSStr // Validate and sanitize
func NewSafeCSS(css string) SafeCSSStr // Validate and sanitize
func NewSafeAttr(name, value string) SafeAttr // Validate and sanitize

// Methods
func (h SafeHTML) String() string
func (u SafeURL) String() string
func (j SafeJSStr) String() string
func (c SafeCSSStr) String() string
func (a SafeAttr) String() string

// Convert functions always sanitize input
func ToSafeHTML(html string) SafeHTML
func ToSafeURL(url string) SafeURL
func ToSafeJS(js string) SafeJSStr
func ToSafeCSS(css string) SafeCSSStr
```

## HTML Parsing

```go
// Element struct for HTML element representation
type Element struct {
    TagName    string
    Attributes map[string]string
    Children   []*Element
    Parent     *Element
    Text       string
}

// Methods
func (e *Element) AddChild(child *Element) *Element
func (e *Element) SetAttribute(name, value string) *Element
func (e *Element) GetAttribute(name string) (string, bool)
func (e *Element) RemoveAttribute(name string) *Element
func (e *Element) AddText(text string) *Element
func (e *Element) Find(selector string) []*Element
func (e *Element) FindOne(selector string) *Element
func (e *Element) CSS(property string) string
func (e *Element) SetCSS(property, value string) *Element
func (e *Element) Text() string
func (e *Element) SetText(text string) *Element
func (e *Element) HTML() string
func (e *Element) SetHTML(html string) *Element
func (e *Element) Remove()
func (e *Element) ReplaceWith(newElement *Element)
func (e *Element) Clone() *Element

// Document struct for HTML document representation
type Document struct {
    Root        *Element
    Title       string
    Doctype     string
    ContentType string
}

// Parser functions
func ParseHTML(html string) (*Document, error)
func ParseHTMLElement(html string) (*Element, error)
func ParseHTMLFragment(html string) ([]*Element, error)

// Document methods
func (d *Document) Find(selector string) []*Element
func (d *Document) FindOne(selector string) *Element
func (d *Document) GetElementByID(id string) *Element
func (d *Document) GetElementsByTagName(tagName string) []*Element
func (d *Document) GetElementsByClassName(className string) []*Element
func (d *Document) QuerySelector(selector string) *Element
func (d *Document) QuerySelectorAll(selector string) []*Element
func (d *Document) CreateElement(tagName string) *Element
func (d *Document) CreateTextNode(data string) *Element
func (d *Document) ToHTML() string
func (d *Document) Title() string
func (d *Document) SetTitle(title string)
func (d *Document) GetMetaTag(name string) (string, bool)
func (d *Document) SetMetaTag(name, content string)
```

## HTML Sanitization

```go
type SanitizeOptions struct {
    AllowedTags       []string
    AllowedAttributes map[string][]string
    AllowedURLSchemes []string
    AllowComments     bool
    AllowIframes      bool
    AllowStyles       bool
    AllowScripts      bool
    AllowSVG          bool
    MaxNestingLevel   int
    PreserveEntities  bool
    StripEmpty        bool
    SanitizeURLs      bool
}

// Default sanitize options
var (
    DefaultSanitizeOptions = SanitizeOptions{...}
    StrictSanitizeOptions = SanitizeOptions{...}
    BasicSanitizeOptions  = SanitizeOptions{...}
    BlogSanitizeOptions  = SanitizeOptions{...}
    WikiSanitizeOptions  = SanitizeOptions{...}
)

// Sanitization functions
func Sanitize(html string, options *SanitizeOptions) string
func SanitizeAttribute(name, value string, tagName string, options *SanitizeOptions) (string, bool)
func SanitizeURL(url string, options *SanitizeOptions) string
func SanitizeStyle(css string, options *SanitizeOptions) string
```

## CSS Selector Engine

```go
type Selector interface {
    // Match element against selector
    Match(e *Element) bool
    
    // Find all elements matching selector
    Find(root *Element) []*Element
    
    // Find first element matching selector
    FindOne(root *Element) *Element
}

// Parse CSS selector into Selector
func ParseSelector(selector string) (Selector, error)

// Helper CSS selection functions
func Select(root *Element, selector string) []*Element
func SelectOne(root *Element, selector string) *Element
func Matches(el *Element, selector string) bool
func Closest(el *Element, selector string) *Element
```

## HTML Generation

```go
// HTMLBuilder for simplified HTML generation
type HTMLBuilder struct {}

// Constructor
func NewHTMLBuilder() *HTMLBuilder

// Methods
func (b *HTMLBuilder) Element(name string, attrs map[string]string, content ...interface{}) *HTMLBuilder
func (b *HTMLBuilder) Text(text string) *HTMLBuilder
func (b *HTMLBuilder) Raw(html SafeHTML) *HTMLBuilder
func (b *HTMLBuilder) String() string
func (b *HTMLBuilder) SafeHTML() SafeHTML
func (b *HTMLBuilder) WriteTo(w io.Writer) (int64, error)

// Common HTML element helper functions
func Div(attrs map[string]string, content ...interface{}) *HTMLBuilder
func Span(attrs map[string]string, content ...interface{}) *HTMLBuilder
func A(href string, attrs map[string]string, content ...interface{}) *HTMLBuilder
func P(attrs map[string]string, content ...interface{}) *HTMLBuilder
func H1(attrs map[string]string, content ...interface{}) *HTMLBuilder
func H2(attrs map[string]string, content ...interface{}) *HTMLBuilder
func H3(attrs map[string]string, content ...interface{}) *HTMLBuilder
func Img(src, alt string, attrs map[string]string) *HTMLBuilder
func Input(type_, name, value string, attrs map[string]string) *HTMLBuilder
func Button(attrs map[string]string, content ...interface{}) *HTMLBuilder
func Table(headers []string, rows [][]interface{}, attrs map[string]string) *HTMLBuilder
func Form(action, method string, attrs map[string]string, content ...interface{}) *HTMLBuilder
```

## Accessibility Helpers

```go
// Generate ARIA attributes
func AriaLabel(label string) SafeAttr
func AriaDescribedBy(id string) SafeAttr
func AriaExpanded(expanded bool) SafeAttr
func AriaHidden(hidden bool) SafeAttr
func AriaRequired(required bool) SafeAttr

// Check accessibility
func CheckA11y(element *Element) []A11yIssue
func CheckPageA11y(doc *Document) []A11yIssue

type A11yIssue struct {
    Element     *Element
    Description string
    Level       A11yLevel // Error, Warning, Info
    Rule        string
}
```

## Usage Example

```go
// Basic HTML escaping
unsafe := "<script>alert('XSS')</script>"
safe := tag_core.EscapeString(unsafe)
vibez.spill(safe) // &lt;script&gt;alert(&#39;XSS&#39;)&lt;/script&gt;

// Context-aware escaping
js := "alert('Hello');"
safeJS := tag_core.EscapeForContext(js, tag_core.ContextJS)
vibez.spill(safeJS) // alert(\'Hello\');

// Using SafeHTML types
userComment := "<b>Hello!</b>"
sanitized := tag_core.Sanitize(userComment, &tag_core.BasicSanitizeOptions)
safeHTML := tag_core.ToSafeHTML(sanitized)

// HTML parsing
html := "<div id='content'><p>Hello <b>World</b></p></div>"
doc, err := tag_core.ParseHTML(html)
if err != nil {
    vibez.spill("Parsing error:", err)
    return
}

// Finding elements
elements := doc.Find("p b")
vibez.spill("Found elements:", len(elements))
vibez.spill("Text:", elements[0].Text()) // World

// Manipulating elements
div := doc.FindOne("div")
div.SetAttribute("class", "container")
div.AddChild(doc.CreateElement("footer")).AddText("Page Footer")

// Output modified HTML
modifiedHTML := doc.ToHTML()
vibez.spill(modifiedHTML)
// <div id='content' class='container'><p>Hello <b>World</b></p><footer>Page Footer</footer></div>

// Building HTML with the builder
builder := tag_core.NewHTMLBuilder()
html = builder.Element("div", map[string]string{"class": "card"}, 
    tag_core.H2(nil, "Card Title"),
    tag_core.P(map[string]string{"class": "card-text"}, "This is a card component"),
    tag_core.Button(map[string]string{"class": "btn"}, "Click Me"),
).String()

vibez.spill(html)
// <div class="card"><h2>Card Title</h2><p class="card-text">This is a card component</p><button class="btn">Click Me</button></div>

// Sanitizing user input
userHTML := "<p>Hello</p><script>alert('XSS')</script><img src='x' onerror='alert(1)'>"
sanitized = tag_core.Sanitize(userHTML, &tag_core.StrictSanitizeOptions)
vibez.spill(sanitized) // <p>Hello</p><img src="x">

// Using SafeWriter for performance
var buf bytes.Buffer
writer := tag_core.NewSafeWriter(&buf)
writer.WriteString("<p>")
writer.WriteString(unsafe)
writer.WriteString("</p>")
vibez.spill(buf.String()) // <p>&lt;script&gt;alert(&#39;XSS&#39;)&lt;/script&gt;</p>

// Creating a table
tableHeaders := []string{"Name", "Age", "Location"}
tableRows := [][]interface{}{
    {"Alice", 25, "New York"},
    {"Bob", 30, "San Francisco"},
    {"Charlie", 22, "Chicago"},
}
table := tag_core.Table(tableHeaders, tableRows, map[string]string{"class": "data-table"})
vibez.spill(table.String())

// Accessibility checks
doc, _ = tag_core.ParseHTML("<img src='image.jpg'>")
issues := tag_core.CheckA11y(doc.FindOne("img"))
for _, issue := range issues {
    vibez.spill(issue.Description) // "Image is missing alt attribute"
}
```

## Implementation Guidelines
1. Prioritize security to prevent XSS vulnerabilities
2. Optimize performance for large HTML documents
3. Ensure robust HTML parsing that handles malformed input gracefully
4. Maintain compatibility with HTML5 standards
5. Provide clear error messages for parsing and validation issues
6. Include comprehensive accessibility support
7. Support internationalization and different character encodings
8. Implement efficient memory usage for large documents