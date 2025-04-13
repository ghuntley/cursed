# RizzTemplate (template package)

## Overview
RizzTemplate provides a data-driven templating system for generating textual output with powerful "rizz" (style and charisma). It's inspired by Go's template package but with enhanced features and a focus on dynamic content generation.

## Core Types

### `Template`
Represents a compiled template.

```go
type Template struct {}

// Constructors and Parsing
func New(name string) *Template
func ParseFiles(filenames ...string) (*Template, error)
func ParseGlob(pattern string) (*Template, error)
func ParseFS(fs fs.FS, patterns ...string) (*Template, error)
func Must(t *Template, err error) *Template

// Methods for parsing
func (t *Template) Parse(text string) (*Template, error)
func (t *Template) ParseFiles(filenames ...string) (*Template, error)
func (t *Template) ParseGlob(pattern string) (*Template, error)
func (t *Template) ParseFS(fs fs.FS, patterns ...string) (*Template, error)

// Methods for cloning and lookup
func (t *Template) Clone() (*Template, error)
func (t *Template) Name() string
func (t *Template) Templates() []*Template
func (t *Template) Lookup(name string) *Template
func (t *Template) DefinedTemplates() string

// Methods for execution
func (t *Template) Execute(wr io.Writer, data interface{}) error
func (t *Template) ExecuteTemplate(wr io.Writer, name string, data interface{}) error

// Enhanced methods
func (t *Template) AddFuncs(funcMap FuncMap) *Template
func (t *Template) ExecuteToString(data interface{}) (string, error)
func (t *Template) ExecuteTemplateToString(name string, data interface{}) (string, error)
func (t *Template) ExecuteWithContext(ctx VibeContext, wr io.Writer, data interface{}) error
```

### `FuncMap`
Map of functions available to templates during execution.

```go
type FuncMap map[string]interface{}
```

## Options and Configuration

### `TemplateOptions`
Options for configuring template behavior.

```go
type TemplateOptions struct {
    LeftDelim  string
    RightDelim string
    StrictMode bool
    EscapeHTML bool
    Funcs      FuncMap
    MaxExecTime time.Duration
    CustomErrorHandler func(err error)
}

// Apply options to a template
func (t *Template) WithOptions(opts TemplateOptions) *Template
```

## Built-in Template Functions

RizzTemplate provides a variety of built-in functions for templating:

### Text Manipulation

```go
// String operations
func lower(s string) string
func upper(s string) string
func title(s string) string
func trim(s string) string
func trimSpace(s string) string
func trimPrefix(s, prefix string) string
func trimSuffix(s, suffix string) string
func replace(s, old, new string) string
func replaceAll(s, old, new string) string
func split(s, sep string) []string
func join(a []string, sep string) string
func contains(s, substr string) bool
func hasPrefix(s, prefix string) bool
func hasSuffix(s, suffix string) bool
func substr(s string, start, end int) string
func repeat(s string, count int) string
func runeCount(s string) int
func index(s, substr string) int
func lastIndex(s, substr string) int
```

### Formatting

```go
func printf(format string, args ...interface{}) string
func sprintf(format string, args ...interface{}) string
func numFormat(n interface{}, precision int) string
func currency(n float64) string
func byteSize(bytes int64) string
func percentage(n float64) string
func dateFormat(t time.Time, layout string) string
func relativeTime(t time.Time) string
func plural(count int, singular, plural string) string
```

### Collection Operations

```go
func len(v interface{}) int
func slice(v interface{}, start, end int) interface{}
func map(collection interface{}, fn interface{}) interface{}
func filter(collection interface{}, fn interface{}) interface{}
func reduce(collection interface{}, initialValue interface{}, fn interface{}) interface{}
func sort(v interface{}) interface{}
func sortBy(v interface{}, key string) interface{}
func reverse(v interface{}) interface{}
func first(v interface{}) interface{}
func last(v interface{}) interface{}
func keys(v interface{}) []string
func values(v interface{}) []interface{}
func groupBy(collection interface{}, key string) map[string]interface{}
```

### Data Conversion

```go
func toJSON(v interface{}) string
func fromJSON(s string) interface{}
func toYAML(v interface{}) string
func fromYAML(s string) interface{}
func toBase64(s string) string
func fromBase64(s string) string
func toBool(v interface{}) bool
func toString(v interface{}) string
func toInt(v interface{}) int
func toFloat(v interface{}) float64
```

### Control Flow

```go
func eq(a, b interface{}) bool
func ne(a, b interface{}) bool
func lt(a, b interface{}) bool
func le(a, b interface{}) bool
func gt(a, b interface{}) bool
func ge(a, b interface{}) bool
func and(a, b bool) bool
func or(a, b bool) bool
func not(a bool) bool
func ternary(cond bool, t, f interface{}) interface{}
func isZero(v interface{}) bool
func isNil(v interface{}) bool
func isEmpty(v interface{}) bool
```

### URL and HTML

```go
func urlEncode(s string) string
func urlDecode(s string) string
func htmlEscape(s string) string
func htmlUnescape(s string) string
func pathEscape(s string) string
func queryEscape(s string) string
func cssEscape(s string) string
func jsEscape(s string) string
func safeHTML(s string) any // Marks string as safe HTML
func safeURL(s string) any // Marks string as safe URL
func safeJS(s string) any // Marks string as safe JavaScript
func safeCSS(s string) any // Marks string as safe CSS
```

### Random and Math

```go
func randomInt(min, max int) int
func randomString(length int) string
func uuid() string
func now() time.Time
func timeAdd(t time.Time, duration string) time.Time
func timeSub(t1, t2 time.Time) time.Duration
func add(a, b interface{}) interface{}
func sub(a, b interface{}) interface{}
func mul(a, b interface{}) interface{}
func div(a, b interface{}) interface{}
func mod(a, b interface{}) interface{}
func max(a, b interface{}) interface{}
func min(a, b interface{}) interface{}
func round(n interface{}, precision int) float64
func ceil(n interface{}) float64
func floor(n interface{}) float64
```

## Enhanced Features

### `TemplateCache`
Provides caching capabilities for templates.

```go
type TemplateCache struct {}

// Constructor
func NewTemplateCache() *TemplateCache

// Methods
func (c *TemplateCache) Get(name string) (*Template, bool)
func (c *TemplateCache) Set(name string, t *Template)
func (c *TemplateCache) Del(name string)
func (c *TemplateCache) Clear()
func (c *TemplateCache) Load(dir string) error
func (c *TemplateCache) WatchAndReload(dir string) error
```

### `TemplateRenderer`
Provides high-level rendering capabilities.

```go
type TemplateRenderer struct {}

// Constructor
func NewTemplateRenderer(dir string, opts TemplateOptions) *TemplateRenderer

// Methods
func (r *TemplateRenderer) Render(w io.Writer, name string, data interface{}) error
func (r *TemplateRenderer) RenderToString(name string, data interface{}) (string, error)
func (r *TemplateRenderer) AddGlobal(key string, value interface{})
func (r *TemplateRenderer) AddFunc(name string, fn interface{})
func (r *TemplateRenderer) AddFuncs(funcs FuncMap)
func (r *TemplateRenderer) Reload() error
func (r *TemplateRenderer) EnableAutoReload(interval time.Duration)
```

### `RizzLayout`
Provides layout/inheritance capabilities.

```go
type RizzLayout struct {
    Name string
    Template *Template
    Content map[string]string
}

// Constructor
func NewRizzLayout(name string, t *Template) *RizzLayout

// Methods
func (l *RizzLayout) SetContent(blockName, content string)
func (l *RizzLayout) Execute(w io.Writer, data interface{}) error
func (l *RizzLayout) String() (string, error)
```

## Syntax Extensions

In addition to standard Go template syntax, RizzTemplate supports:

### Layout and Blocks

```
{{#layout "base"}}
  {{#block "title"}}Page Title{{/block}}
  {{#block "content"}}
    <p>Page content goes here</p>
  {{/block}}
{{/layout}}
```

### Partials with Local Variables

```
{{#partial "header" headerText="Welcome!" headerSize=1}}
  <h{{headerSize}}>{{headerText}}</h{{headerSize}}>
{{/partial}}
```

### Enhanced Conditionals

```
{{#when user.admin}}
  <p>Admin controls:</p>
  {{#if user.superadmin}}
    <p>Super admin features</p>
  {{else if user.moderator}}
    <p>Moderator features</p>
  {{else}}
    <p>Basic admin features</p>
  {{/if}}
{{/when}}
```

### Iteration Helpers

```
{{#loop 5}}
  <p>Item {{@index}}</p>
{{/loop}}

{{#each users}}
  <p>{{@index}}: {{name}} {{#if @first}}(first){{/if}} {{#if @last}}(last){{/if}}</p>
{{/each}}

{{#for i=0 to=10 step=2}}
  <p>Counter: {{i}}</p>
{{/for}}
```

## Usage Example

```go
// Simple template parsing and execution
tmpl, err := rizz_template.New("greeting").Parse("Hello, {{.Name}}! {{if .Admin}}You're an admin!{{end}}")
if err != nil {
    vibez.spill("Parse error:", err)
    return
}

data := struct {
    Name  string
    Admin bool
}{
    Name:  "Alice",
    Admin: true,
}

var buf bytes.Buffer
if err := tmpl.Execute(&buf, data); err != nil {
    vibez.spill("Execution error:", err)
    return
}

vibez.spill(buf.String()) // "Hello, Alice! You're an admin!"

// Using function maps
funcs := rizz_template.FuncMap{
    "formatName": func(name string) string {
        return strings.ToUpper(name)
    },
}

tmpl, err = rizz_template.New("fancyGreeting").AddFuncs(funcs).Parse("Hello, {{formatName .Name}}!")
if err != nil {
    vibez.spill("Parse error:", err)
    return
}

result, err := tmpl.ExecuteToString(data)
if err != nil {
    vibez.spill("Execution error:", err)
    return
}

vibez.spill(result) // "Hello, ALICE!"

// Using template renderer
renderer := rizz_template.NewTemplateRenderer("./templates", rizz_template.TemplateOptions{
    LeftDelim:  "{{{",
    RightDelim: "}}}",
    StrictMode: true,
})

renderer.AddGlobal("siteName", "My Awesome Site")
renderer.AddFunc("currentYear", func() int {
    return time.Now().Year()
})

html, err := renderer.RenderToString("pages/home.tmpl", map[string]interface{}{
    "title": "Home Page",
    "user":  user,
})
if err != nil {
    vibez.spill("Render error:", err)
    return
}

// Using layout system
layout := rizz_template.NewRizzLayout("base", baseTemplate)
layout.SetContent("title", "<h1>Welcome to my site</h1>")
layout.SetContent("content", "<p>This is the main content area.</p>")

html, err = layout.String(map[string]interface{}{
    "user": currentUser,
})
if err != nil {
    vibez.spill("Layout error:", err)
    return
}
```

## Template Syntax Examples

### Basic Variable Interpolation

```
{{.Name}} - Simple variable
{{.User.Profile.Email}} - Nested properties
{{$variable := .Count}} - Variable assignment
{{$variable}} - Variable usage
```

### Conditionals

```
{{if .IsAdmin}}
  <p>Admin content</p>
{{else if .IsModerator}}
  <p>Moderator content</p>
{{else}}
  <p>User content</p>
{{end}}

{{#when .IsLoggedIn}}
  <p>You are logged in as {{.Username}}</p>
{{/when}}
```

### Loops and Iterations

```
{{range .Items}}
  <li>{{.Name}}: {{.Price}}</li>
{{else}}
  <li>No items found</li>
{{end}}

{{#each .Users}}
  <tr class="{{#if @even}}even{{else}}odd{{/if}}">
    <td>{{@index}}</td>
    <td>{{.Name}}</td>
  </tr>
{{/each}}
```

### Template Composition

```
{{template "header" .}}
<main>
  {{template "content" .}}
</main>
{{template "footer" .}}

{{define "header"}}
  <header>{{.Title}}</header>
{{end}}
```

### Function Calls

```
{{upper .Name}}
{{add 5 10}}
{{dateFormat .Timestamp "Jan 2, 2006"}}
{{slice .Items 0 5}}
{{toJSON .Data}}
```

## Implementation Guidelines
1. Ensure thread-safe template execution for concurrent rendering
2. Optimize parsing and execution performance for large templates
3. Provide clear error messages with line numbers for debugging
4. Implement proper HTML escaping by default to prevent XSS vulnerabilities
5. Support nested template composition for reusability
6. Include comprehensive documentation with examples
7. Implement caching mechanisms for compiled templates
8. Ensure compatibility with Go's template syntax while adding enhancements