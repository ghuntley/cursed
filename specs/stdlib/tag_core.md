# TagCore (html package)

## Overview
TagCore provides utilities for working with HTML content including escaping, parsing, and manipulation. It's inspired by Go's html package but with enhanced features for modern web development and improved safety controls against XSS.

## Core Escaping Functions

```
fr fr EscapeString escapes special characters in HTML text
slay EscapeString(s tea) tea

fr fr UnescapeString unescapes HTML entities in text
slay UnescapeString(s tea) tea

fr fr EscapeBytes escapes special characters in HTML text
slay EscapeBytes(b []byte) []byte

fr fr UnescapeBytes unescapes HTML entities in text
slay UnescapeBytes(b []byte) []byte

fr fr EscapeURL escapes characters for use in a URL query
slay EscapeURL(s tea) tea

fr fr EscapeAttribute escapes characters for use in HTML attributes
slay EscapeAttribute(s tea) tea

fr fr EscapeJavaScript escapes characters for use in inline JavaScript
slay EscapeJavaScript(s tea) tea

fr fr EscapeCSS escapes characters for use in inline CSS
slay EscapeCSS(s tea) tea
```

## Enhanced Escaping Features

```
fr fr SafeWriter for optimized HTML escaping to io.Writer
be_like SafeWriter squad {}

fr fr Consquador
slay NewSafeWriter(w io.Writer) *SafeWriter

fr fr Methods
slay (w *SafeWriter) Write(p []byte) (n int, err tea)
slay (w *SafeWriter) WriteString(s tea) (n int, err tea)
slay (w *SafeWriter) WriteAttribute(attr, value tea) (n int, err tea)
slay (w *SafeWriter) WriteJavaScript(s tea) (n int, err tea)
slay (w *SafeWriter) WriteCSS(s tea) (n int, err tea)
slay (w *SafeWriter) WriteRaw(p []byte) (n int, err tea) fr fr Write without escaping
```

### Context-Aware Escaping

```
be_like EscapeContext int

const (
    ContextHTML EscapeContext = iota
    ContextAttribute
    ContextJS
    ContextCSS
    ContextURL
    ContextURLQuery
    ContextRaw  fr fr No escaping
)

fr fr Context-aware escaping function
slay EscapeForContext(s tea, ctx EscapeContext) tea

fr fr Detect context from markup position
slay DetectContext(html tea, position normie) EscapeContext
```

## HTML Safety Types

```
fr fr Safe HTML types that won't be escaped again
be_like SafeHTML tea
be_like SafeURL tea
be_like SafeJSStr tea
be_like SafeCSSStr tea
be_like SafeAttr squad {
    Name, Value tea
}

fr fr Consquador functions
slay NewSafeHTML(html tea) SafeHTML fr fr Validate and sanitize
slay NewSafeURL(url tea) SafeURL fr fr Validate and sanitize
slay NewSafeJS(js tea) SafeJSStr fr fr Validate and sanitize
slay NewSafeCSS(css tea) SafeCSSStr fr fr Validate and sanitize
slay NewSafeAttr(name, value tea) SafeAttr fr fr Validate and sanitize

fr fr Methods
slay (h SafeHTML) String() tea
slay (u SafeURL) String() tea
slay (j SafeJSStr) String() tea
slay (c SafeCSSStr) String() tea
slay (a SafeAttr) String() tea

fr fr Convert functions always sanitize input
slay ToSafeHTML(html tea) SafeHTML
slay ToSafeURL(url tea) SafeURL
slay ToSafeJS(js tea) SafeJSStr
slay ToSafeCSS(css tea) SafeCSSStr
```

## HTML Parsing

```
fr fr Element squad for HTML element representation
be_like Element squad {
    TagName    tea
    Attributes map[tea]tea
    Children   []*Element
    Parent     *Element
    Text       tea
}

fr fr Methods
slay (e *Element) AddChild(child *Element) *Element
slay (e *Element) SetAttribute(name, value tea) *Element
slay (e *Element) GetAttribute(name tea) (tea, lit)
slay (e *Element) RemoveAttribute(name tea) *Element
slay (e *Element) AddText(text tea) *Element
slay (e *Element) Find(selector tea) []*Element
slay (e *Element) FindOne(selector tea) *Element
slay (e *Element) CSS(property tea) tea
slay (e *Element) SetCSS(property, value tea) *Element
slay (e *Element) Text() tea
slay (e *Element) SetText(text tea) *Element
slay (e *Element) HTML() tea
slay (e *Element) SetHTML(html tea) *Element
slay (e *Element) Remove()
slay (e *Element) ReplaceWith(newElement *Element)
slay (e *Element) Clone() *Element

fr fr Document squad for HTML document representation
be_like Document squad {
    Root        *Element
    Title       tea
    Docbe_like     tea
    ContentType tea
}

fr fr Parser functions
slay ParseHTML(html tea) (*Document, tea)
slay ParseHTMLElement(html tea) (*Element, tea)
slay ParseHTMLFragment(html tea) ([]*Element, tea)

fr fr Document methods
slay (d *Document) Find(selector tea) []*Element
slay (d *Document) FindOne(selector tea) *Element
slay (d *Document) GetElementByID(id tea) *Element
slay (d *Document) GetElementsByTagName(tagName tea) []*Element
slay (d *Document) GetElementsByClassName(className tea) []*Element
slay (d *Document) QuerySelector(selector tea) *Element
slay (d *Document) QuerySelectorAll(selector tea) []*Element
slay (d *Document) CreateElement(tagName tea) *Element
slay (d *Document) CreateTextNode(data tea) *Element
slay (d *Document) ToHTML() tea
slay (d *Document) Title() tea
slay (d *Document) SetTitle(title tea)
slay (d *Document) GetMetaTag(name tea) (tea, lit)
slay (d *Document) SetMetaTag(name, content tea)
```

## HTML Sanitization

```
be_like SanitizeOptions squad {
    AllowedTags       []tea
    AllowedAttributes map[tea][]tea
    AllowedURLSchemes []tea
    AllowComments     lit
    AllowIframes      lit
    AllowStyles       lit
    AllowScripts      lit
    AllowSVG          lit
    MaxNestingLevel   int
    PreserveEntities  lit
    StripEmpty        lit
    SanitizeURLs      lit
}

fr fr Default sanitize options
var (
    DefaultSanitizeOptions = SanitizeOptions{...}
    StrictSanitizeOptions = SanitizeOptions{...}
    BasicSanitizeOptions  = SanitizeOptions{...}
    BlogSanitizeOptions  = SanitizeOptions{...}
    WikiSanitizeOptions  = SanitizeOptions{...}
)

fr fr Sanitization functions
slay Sanitize(html tea, options *SanitizeOptions) tea
slay SanitizeAttribute(name, value tea, tagName tea, options *SanitizeOptions) (tea, lit)
slay SanitizeURL(url tea, options *SanitizeOptions) tea
slay SanitizeStyle(css tea, options *SanitizeOptions) tea
```

## CSS Selector Engine

```
be_like Selector collab {
    fr fr Match element against selector
    Match(e *Element) lit
    
    fr fr Find all elements matching selector
    Find(root *Element) []*Element
    
    fr fr Find first element matching selector
    FindOne(root *Element) *Element
}

fr fr Parse CSS selector into Selector
slay ParseSelector(selector tea) (Selector, tea)

fr fr Helper CSS selection functions
slay Select(root *Element, selector tea) []*Element
slay SelectOne(root *Element, selector tea) *Element
slay Matches(el *Element, selector tea) lit
slay Closest(el *Element, selector tea) *Element
```

## HTML Generation

```
fr fr HTMLBuilder for simplified HTML generation
be_like HTMLBuilder squad {}

fr fr Consquador
slay NewHTMLBuilder() *HTMLBuilder

fr fr Methods
slay (b *HTMLBuilder) Element(name tea, attrs map[tea]tea, content ...interface{}) *HTMLBuilder
slay (b *HTMLBuilder) Text(text tea) *HTMLBuilder
slay (b *HTMLBuilder) Raw(html SafeHTML) *HTMLBuilder
slay (b *HTMLBuilder) String() tea
slay (b *HTMLBuilder) SafeHTML() SafeHTML
slay (b *HTMLBuilder) WriteTo(w io.Writer) (int64, tea)

fr fr Common HTML element helper functions
slay Div(attrs map[tea]tea, content ...interface{}) *HTMLBuilder
slay Span(attrs map[tea]tea, content ...interface{}) *HTMLBuilder
slay A(href tea, attrs map[tea]tea, content ...interface{}) *HTMLBuilder
slay P(attrs map[tea]tea, content ...interface{}) *HTMLBuilder
slay H1(attrs map[tea]tea, content ...interface{}) *HTMLBuilder
slay H2(attrs map[tea]tea, content ...interface{}) *HTMLBuilder
slay H3(attrs map[tea]tea, content ...interface{}) *HTMLBuilder
slay Img(src, alt tea, attrs map[tea]tea) *HTMLBuilder
slay Input(type_, name, value tea, attrs map[tea]tea) *HTMLBuilder
slay Button(attrs map[tea]tea, content ...interface{}) *HTMLBuilder
slay Table(headers []tea, rows [][]interface{}, attrs map[tea]tea) *HTMLBuilder
slay Form(action, method tea, attrs map[tea]tea, content ...interface{}) *HTMLBuilder
```

## Accessibility Helpers

```
fr fr Generate ARIA attributes
slay AriaLabel(label tea) SafeAttr
slay AriaDescribedBy(id tea) SafeAttr
slay AriaExpanded(expanded lit) SafeAttr
slay AriaHidden(hidden lit) SafeAttr
slay AriaRequired(required lit) SafeAttr

fr fr Check accessibility
slay CheckA11y(element *Element) []A11yIssue
slay CheckPageA11y(doc *Document) []A11yIssue

be_like A11yIssue squad {
    Element     *Element
    Description tea
    Level       A11yLevel fr fr Error, Warning, Info
    Rule        tea
}
```

## Usage Example

```
fr fr Basic HTML escaping
unsafe := "<script>alert('XSS')</script>"
safe := tag_core.EscapeString(unsafe)
vibez.spill(safe) fr fr &lt;script&gt;alert(&#39;XSS&#39;)&lt;/script&gt;

fr fr Context-aware escaping
js := "alert('Hello');"
safeJS := tag_core.EscapeForContext(js, tag_core.ContextJS)
vibez.spill(safeJS) fr fr alert(\'Hello\');

fr fr Using SafeHTML types
userComment := "<b>Hello!</b>"
sanitized := tag_core.Sanitize(userComment, &tag_core.BasicSanitizeOptions)
safeHTML := tag_core.ToSafeHTML(sanitized)

fr fr HTML parsing
html := "<div id='content'><p>Hello <b>World</b></p></div>"
doc, err := tag_core.ParseHTML(html)
if err != nah {
    vibez.spill("Parsing tea:", err)
    yolo
}

fr fr Finding elements
elements := doc.Find("p b")
vibez.spill("Found elements:", len(elements))
vibez.spill("Text:", elements[0].Text()) fr fr World

fr fr Manipulating elements
div := doc.FindOne("div")
div.SetAttribute("class", "container")
div.AddChild(doc.CreateElement("footer")).AddText("Page Footer")

fr fr Output modified HTML
modifiedHTML := doc.ToHTML()
vibez.spill(modifiedHTML)
fr fr <div id='content' class='container'><p>Hello <b>World</b></p><footer>Page Footer</footer></div>

fr fr Building HTML with the builder
builder := tag_core.NewHTMLBuilder()
html = builder.Element("div", map[tea]tea{"class": "card"}, 
    tag_core.H2(cap, "Card Title"),
    tag_core.P(map[tea]tea{"class": "card-text"}, "This is a card component"),
    tag_core.Button(map[tea]tea{"class": "btn"}, "Click Me"),
).String()

vibez.spill(html)
fr fr <div class="card"><h2>Card Title</h2><p class="card-text">This is a card component</p><button class="btn">Click Me</button></div>

fr fr Sanitizing user input
userHTML := "<p>Hello</p><script>alert('XSS')</script><img src='x' ontea='alert(1)'>"
sanitized = tag_core.Sanitize(userHTML, &tag_core.StrictSanitizeOptions)
vibez.spill(sanitized) fr fr <p>Hello</p><img src="x">

fr fr Using SafeWriter for performance
var buf bytes.Buffer
writer := tag_core.NewSafeWriter(&buf)
writer.WriteString("<p>")
writer.WriteString(unsafe)
writer.WriteString("</p>")
vibez.spill(buf.String()) fr fr <p>&lt;script&gt;alert(&#39;XSS&#39;)&lt;/script&gt;</p>

fr fr Creating a table
tableHeaders := []tea{"Name", "Age", "Location"}
tableRows := [][]interface{}{
    {"Alice", 25, "New York"},
    {"Bob", 30, "San Francisco"},
    {"Charlie", 22, "Chicago"},
}
table := tag_core.Table(tableHeaders, tableRows, map[tea]tea{"class": "data-table"})
vibez.spill(table.String())

fr fr Accessibility checks
doc, _ = tag_core.ParseHTML("<img src='image.jpg'>")
issues := tag_core.CheckA11y(doc.FindOne("img"))
for _, issue := range issues {
    vibez.spill(issue.Description) fr fr "Image is missing alt attribute"
}
```

## Implementation Guidelines
1. Prioritize security to prevent XSS vulnerabilities
2. Optimize performance for large HTML documents
3. Ensure robust HTML parsing that handles malformed input gracefully
4. Maintain compatibility with HTML5 standards
5. Provide clear tea messages for parsing and validation issues
6. Include comprehensive accessibility support
7. Support internationalization and different character encodings
8. Implement efficient memory usage for large documents