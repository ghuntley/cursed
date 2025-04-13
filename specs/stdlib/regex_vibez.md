# RegexVibez (regexp package)

## Overview
RegexVibez provides regular expression search functionality, inspired by Go's regexp package but with enhanced pattern matching capabilities and a vibez-focused interface.

## Core Types

### `VibePattern`
Represents a compiled regular expression pattern.

```go
type VibePattern struct {}

// Constructors
func Compile(expr string) (*VibePattern, error)
func MustCompile(expr string) *VibePattern // Panics on error
func CompilePOSIX(expr string) (*VibePattern, error)
func MustCompilePOSIX(expr string) *VibePattern // Panics on error
```

## Matching Methods

```go
// MatchString reports whether the string s contains any match of the pattern
func (p *VibePattern) MatchString(s string) bool

// Match reports whether b contains any match of the pattern
func (p *VibePattern) Match(b []byte) bool

// MatchReader reports whether the RuneReader contains any match of the pattern
func (p *VibePattern) MatchReader(r io.RuneReader) bool

// FindString returns a string holding the first match of the pattern
func (p *VibePattern) FindString(s string) string

// FindStringIndex returns indexes for the first match of the pattern
func (p *VibePattern) FindStringIndex(s string) (loc []int)

// FindStringSubmatch returns strings holding the text of the first match
func (p *VibePattern) FindStringSubmatch(s string) []string

// FindStringSubmatchIndex returns index pairs holding positions of matches
func (p *VibePattern) FindStringSubmatchIndex(s string) []int

// FindAllString returns all successive matches of the pattern
func (p *VibePattern) FindAllString(s string, n int) []string

// FindAllStringIndex returns indexes of all matches
func (p *VibePattern) FindAllStringIndex(s string, n int) [][]int

// FindAllStringSubmatch returns all successive matches with submatch strings
func (p *VibePattern) FindAllStringSubmatch(s string, n int) [][]string

// FindAllStringSubmatchIndex returns indexes of all matches with submatch indexes
func (p *VibePattern) FindAllStringSubmatchIndex(s string, n int) [][]int

// ReplaceAllString returns a copy with all matches replaced
func (p *VibePattern) ReplaceAllString(src, repl string) string

// ReplaceAllStringFunc returns a copy with replacements determined by function
func (p *VibePattern) ReplaceAllStringFunc(src string, repl func(string) string) string

// Split slices s into substrings separated by pattern
func (p *VibePattern) Split(s string, n int) []string
```

## Helper Package Functions

```go
// Match reports whether the string s contains any match of the pattern
func Match(pattern string, b []byte) (matched bool, err error)

// MatchString reports whether the string s contains any match of the pattern
func MatchString(pattern string, s string) (matched bool, err error)

// QuoteMeta returns a string that escapes all regexp metacharacters
func QuoteMeta(s string) string
```

## Special Features

### `VibeGroups`
Provides named capture group information.

```go
type VibeGroups struct {}

// Methods
func (p *VibePattern) GroupNames() []string
func (p *VibePattern) NamedGroups() map[string]int
func (p *VibePattern) FindGroupsString(s string) map[string]string
```

### `VibeTemplates`
Extension for template-based replacements.

```go
func (p *VibePattern) TemplateReplace(s string, template string) string
```

### `PatternBuilder`
Fluent interface for building regular expressions.

```go
type PatternBuilder struct {}

// Constructor
func NewPatternBuilder() *PatternBuilder

// Methods
func (b *PatternBuilder) StartsWith(s string) *PatternBuilder
func (b *PatternBuilder) EndsWith(s string) *PatternBuilder
func (b *PatternBuilder) Contains(s string) *PatternBuilder
func (b *PatternBuilder) OneOrMore(s string) *PatternBuilder
func (b *PatternBuilder) ZeroOrMore(s string) *PatternBuilder
func (b *PatternBuilder) Optional(s string) *PatternBuilder
func (b *PatternBuilder) Group(s string) *PatternBuilder
func (b *PatternBuilder) NamedGroup(name, s string) *PatternBuilder
func (b *PatternBuilder) Or(patterns ...string) *PatternBuilder
func (b *PatternBuilder) Digit() *PatternBuilder
func (b *PatternBuilder) Word() *PatternBuilder
func (b *PatternBuilder) Space() *PatternBuilder
func (b *PatternBuilder) Email() *PatternBuilder
func (b *PatternBuilder) URL() *PatternBuilder
func (b *PatternBuilder) Build() (*VibePattern, error)
```

## Common Pattern Library

```go
var (
    EmailPattern    = MustCompile(`...`) // Email regex
    URLPattern      = MustCompile(`...`) // URL regex
    DatePattern     = MustCompile(`...`) // Date regex
    TimePattern     = MustCompile(`...`) // Time regex
    UsernamePattern = MustCompile(`...`) // Username regex
    PasswordPattern = MustCompile(`...`) // Password regex
    PhonePattern    = MustCompile(`...`) // Phone regex
    ZipCodePattern  = MustCompile(`...`) // Zip code regex
    HashtagPattern  = MustCompile(`...`) // Social media hashtag regex
    EmojiPattern    = MustCompile(`...`) // Emoji regex
)
```

## Usage Example

```go
// Simple pattern matching
matched, _ := regex_vibez.MatchString("f[a-z]+", "frfr")
vibez.spill(matched) // true

// Compile and reuse a pattern
pattern, _ := regex_vibez.Compile("no ([a-z]+), bruh")
result := pattern.FindStringSubmatch("no cap, bruh")
vibez.spill(result[1]) // "cap"

// Using the pattern builder
builder := regex_vibez.NewPatternBuilder()
emailPattern, _ := builder.StartsWith("^").NamedGroup("user", "[a-zA-Z0-9._%+-]+").Contains("@").NamedGroup("domain", "[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}").EndsWith("$").Build()

matches := emailPattern.FindGroupsString("user@example.com")
vibez.spill(matches["user"]) // "user"
vibez.spill(matches["domain"]) // "example.com"

// Replacing text with regex
result = regex_vibez.MustCompile("cap").ReplaceAllString("no cap", "lies")
vibez.spill(result) // "no lies"
```

## Implementation Guidelines
1. Performance-focused implementation with efficient matching algorithms
2. Comprehensive error handling for invalid patterns
3. Support for standard regular expression syntax plus extensions
4. Consistent API design following Cursed language patterns
5. Helpful debugging and visualization tools for complex patterns
6. Memory-efficient implementation for large inputs