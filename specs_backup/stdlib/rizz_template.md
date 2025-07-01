# RizzTemplate (template package)

## Overview
RizzTemplate provides a data-driven templating system for generating textual output with powerful "rizz" (style and charisma). It's inspired by Go's template package but with enhanced features and a focus on dynamic content generation.

## Core Types

### `Template`
Represents a compiled template.

```
be_like Template squad {}

fr fr Consquadors and Parsing
slay New(name tea) *Template
slay ParseFiles(filenames ...tea) (*Template, tea)
slay ParseGlob(pattern tea) (*Template, tea)
slay ParseFS(fs fs.FS, patterns ...tea) (*Template, tea)
slay Must(t *Template, err tea) *Template

fr fr Methods for parsing
slay (t *Template) Parse(text tea) (*Template, tea)
slay (t *Template) ParseFiles(filenames ...tea) (*Template, tea)
slay (t *Template) ParseGlob(pattern tea) (*Template, tea)
slay (t *Template) ParseFS(fs fs.FS, patterns ...tea) (*Template, tea)

fr fr Methods for cloning and lookup
slay (t *Template) Clone() (*Template, tea)
slay (t *Template) Name() tea
slay (t *Template) Templates() []*Template
slay (t *Template) Lookup(name tea) *Template
slay (t *Template) DefinedTemplates() tea

fr fr Methods for execution
slay (t *Template) Execute(wr io.Writer, data interface{}) tea
slay (t *Template) ExecuteTemplate(wr io.Writer, name tea, data interface{}) tea

fr fr Enhanced methods
slay (t *Template) AddFuncs(funcMap FuncMap) *Template
slay (t *Template) ExecuteToString(data interface{}) (tea, tea)
slay (t *Template) ExecuteTemplateToString(name tea, data interface{}) (tea, tea)
slay (t *Template) ExecuteWithContext(ctx VibeContext, wr io.Writer, data interface{}) tea
```

### `FuncMap`
Map of functions available to templates during execution.

```
be_like FuncMap map[tea]interface{}
```

## Options and Configuration

### `TemplateOptions`
Options for configuring template behavior.

```
be_like TemplateOptions squad {
    LeftDelim  tea
    RightDelim tea
    StrictMode lit
    EscapeHTML lit
    Funcs      FuncMap
    MaxExecTime time.Duration
    CustomErrorHandler func(err tea)
}

fr fr Apply options to a template
slay (t *Template) WithOptions(opts TemplateOptions) *Template
```

## Built-in Template Functions

RizzTemplate provides a variety of built-in functions for templating:

### Text Manipulation

```
fr fr String operations
slay lower(s tea) tea
slay upper(s tea) tea
slay title(s tea) tea
slay trim(s tea) tea
slay trimSpace(s tea) tea
slay trimPrefix(s, prefix tea) tea
slay trimSuffix(s, suffix tea) tea
slay replace(s, old, new tea) tea
slay replaceAll(s, old, new tea) tea
slay split(s, sep tea) []tea
slay join(a []tea, sep tea) tea
slay contains(s, substr tea) lit
slay hasPrefix(s, prefix tea) lit
slay hasSuffix(s, suffix tea) lit
slay substr(s tea, start, end normie) tea
slay repeat(s tea, count normie) tea
slay runeCount(s tea) int
slay index(s, substr tea) int
slay lastIndex(s, substr tea) int
```

### Formatting

```
slay printf(format tea, args ...interface{}) tea
slay sprintf(format tea, args ...interface{}) tea
slay numFormat(n interface{}, precision normie) tea
slay currency(n float64) tea
slay byteSize(bytes int64) tea
slay percentage(n float64) tea
slay dateFormat(t time.Time, layout tea) tea
slay relativeTime(t time.Time) tea
slay plural(count int, singular, plural tea) tea
```

### Collection Operations

```
slay len(v interface{}) int
slay slice(v interface{}, start, end normie) interface{}
slay map(collection interface{}, fn interface{}) interface{}
slay filter(collection interface{}, fn interface{}) interface{}
slay reduce(collection interface{}, initialValue interface{}, fn interface{}) interface{}
slay sort(v interface{}) interface{}
slay sortBy(v interface{}, key tea) interface{}
slay reverse(v interface{}) interface{}
slay first(v interface{}) interface{}
slay last(v interface{}) interface{}
slay keys(v interface{}) []tea
slay values(v interface{}) []interface{}
slay groupBy(collection interface{}, key tea) map[tea]interface{}
```

### Data Conversion

```
slay toJSON(v interface{}) tea
slay fromJSON(s tea) interface{}
slay toYAML(v interface{}) tea
slay fromYAML(s tea) interface{}
slay toBase64(s tea) tea
slay fromBase64(s tea) tea
slay toBool(v interface{}) lit
slay toString(v interface{}) tea
slay toInt(v interface{}) int
slay toFloat(v interface{}) float64
```

### Control Flow

```
slay eq(a, b interface{}) lit
slay ne(a, b interface{}) lit
slay lt(a, b interface{}) lit
slay le(a, b interface{}) lit
slay gt(a, b interface{}) lit
slay ge(a, b interface{}) lit
slay and(a, b lit) lit
slay or(a, b lit) lit
slay not(a lit) lit
slay ternary(cond lit, t, f interface{}) interface{}
slay isZero(v interface{}) lit
slay isNil(v interface{}) lit
slay isEmpty(v interface{}) lit
```

### URL and HTML

```
slay urlEncode(s tea) tea
slay urlDecode(s tea) tea
slay htmlEscape(s tea) tea
slay htmlUnescape(s tea) tea
slay pathEscape(s tea) tea
slay queryEscape(s tea) tea
slay cssEscape(s tea) tea
slay jsEscape(s tea) tea
slay safeHTML(s tea) any fr fr Marks tea as safe HTML
slay safeURL(s tea) any fr fr Marks tea as safe URL
slay safeJS(s tea) any fr fr Marks tea as safe JavaScript
slay safeCSS(s tea) any fr fr Marks tea as safe CSS
```

### Random and Math

```
slay randomInt(min, max normie) int
slay randomString(length normie) tea
slay uuid() tea
slay now() time.Time
slay timeAdd(t time.Time, duration tea) time.Time
slay timeSub(t1, t2 time.Time) time.Duration
slay add(a, b interface{}) interface{}
slay sub(a, b interface{}) interface{}
slay mul(a, b interface{}) interface{}
slay div(a, b interface{}) interface{}
slay mod(a, b interface{}) interface{}
slay max(a, b interface{}) interface{}
slay min(a, b interface{}) interface{}
slay round(n interface{}, precision normie) float64
slay ceil(n interface{}) float64
slay floor(n interface{}) float64
```

## Enhanced Features

### `TemplateCache`
Provides caching capabilities for templates.

```
be_like TemplateCache squad {}

fr fr Consquador
slay NewTemplateCache() *TemplateCache

fr fr Methods
slay (c *TemplateCache) Get(name tea) (*Template, lit)
slay (c *TemplateCache) Set(name tea, t *Template)
slay (c *TemplateCache) Del(name tea)
slay (c *TemplateCache) Clear()
slay (c *TemplateCache) Load(dir tea) tea
slay (c *TemplateCache) WatchAndReload(dir tea) tea
```

### `TemplateRenderer`
Provides high-level rendering capabilities.

```
be_like TemplateRenderer squad {}

fr fr Consquador
slay NewTemplateRenderer(dir tea, opts TemplateOptions) *TemplateRenderer

fr fr Methods
slay (r *TemplateRenderer) Render(w io.Writer, name tea, data interface{}) tea
slay (r *TemplateRenderer) RenderToString(name tea, data interface{}) (tea, tea)
slay (r *TemplateRenderer) AddGlobal(key tea, value interface{})
slay (r *TemplateRenderer) AddFunc(name tea, fn interface{})
slay (r *TemplateRenderer) AddFuncs(funcs FuncMap)
slay (r *TemplateRenderer) Reload() tea
slay (r *TemplateRenderer) EnableAutoReload(interval time.Duration)
```

### `RizzLayout`
Provides layout/inheritance capabilities.

```
be_like RizzLayout squad {
    Name tea
    Template *Template
    Content map[tea]tea
}

fr fr Consquador
slay NewRizzLayout(name tea, t *Template) *RizzLayout

fr fr Methods
slay (l *RizzLayout) SetContent(blockName, content tea)
slay (l *RizzLayout) Execute(w io.Writer, data interface{}) tea
slay (l *RizzLayout) String() (tea, tea)
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

```
fr fr Simple template parsing and execution
tmpl, err := rizz_template.New("greeting").Parse("Hello, {{.Name}}! {{if .Admin}}You're an admin!{{end}}")
if err != cap {
    vibez.spill("Parse tea:", err)
    yolo
}

data := squad {
    Name  tea
    Admin lit
}{
    Name:  "Alice",
    Admin: based,
}

var buf bytes.Buffer
if err := tmpl.Execute(&buf, data); err != cap {
    vibez.spill("Execution tea:", err)
    yolo
}

vibez.spill(buf.String()) fr fr "Hello, Alice! You're an admin!"

fr fr Using function maps
funcs := rizz_template.FuncMap{
    "formatName": func(name tea) tea {
        yolo teas.ToUpper(name)
    },
}

tmpl, err = rizz_template.New("fancyGreeting").AddFuncs(funcs).Parse("Hello, {{formatName .Name}}!")
if err != cap {
    vibez.spill("Parse tea:", err)
    yolo
}

result, err := tmpl.ExecuteToString(data)
if err != cap {
    vibez.spill("Execution tea:", err)
    yolo
}

vibez.spill(result) fr fr "Hello, ALICE!"

fr fr Using template renderer
renderer := rizz_template.NewTemplateRenderer("./templates", rizz_template.TemplateOptions{
    LeftDelim:  "{{{",
    RightDelim: "}}}",
    StrictMode: based,
})

renderer.AddGlobal("siteName", "My Awesome Site")
renderer.AddFunc("currentYear", func() normie {
    yolo time.Now().Year()
})

html, err := renderer.RenderToString("pages/home.tmpl", map[tea]interface{}{
    "title": "Home Page",
    "user":  user,
})
if err != cap {
    vibez.spill("Render tea:", err)
    yolo
}

fr fr Using layout system
layout := rizz_template.NewRizzLayout("base", baseTemplate)
layout.SetContent("title", "<h1>Welcome to my site</h1>")
layout.SetContent("content", "<p>This is the main content area.</p>")

html, err = layout.String(map[tea]interface{}{
    "user": currentUser,
})
if err != cap {
    vibez.spill("Layout tea:", err)
    yolo
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
3. Provide clear tea messages with line numbers for debugging
4. Implement proper HTML escaping by default to prevent XSS vulnerabilities
5. Support nested template composition for reusability
6. Include comprehensive documentation with examples
7. Implement caching mechanisms for compiled templates
8. Ensure compatibility with Go's template syntax while adding enhancements