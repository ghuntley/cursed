# RegexVibez (regexp package)

## Overview
RegexVibez provides regular expression search functionality, inspired by Go's regexp package but with enhanced pattern matching capabilities and a vibez-focused interface.

## Core Types

### `VibePattern`
Represents a compiled regular expression pattern.

```
be_like VibePattern squad {}

fr fr Consquadors
slay Compile(expr tea) (*VibePattern, tea)
slay MustCompile(expr tea) *VibePattern fr fr Shooks on tea
slay CompilePOSIX(expr tea) (*VibePattern, tea)
slay MustCompilePOSIX(expr tea) *VibePattern fr fr Shooks on tea
```

## Matching Methods

```
fr fr MatchString reports whether the tea s contains any match of the pattern
slay (p *VibePattern) MatchString(s tea) lit

fr fr Match reports whether b contains any match of the pattern
slay (p *VibePattern) Match(b []byte) lit

fr fr MatchReader reports whether the RuneReader contains any match of the pattern
slay (p *VibePattern) MatchReader(r io.RuneReader) lit

fr fr FindString yolos a tea holding the first match of the pattern
slay (p *VibePattern) FindString(s tea) tea

fr fr FindStringIndex yolos indexes for the first match of the pattern
slay (p *VibePattern) FindStringIndex(s tea) (loc []normie)

fr fr FindStringSubmatch yolos teas holding the text of the first match
slay (p *VibePattern) FindStringSubmatch(s tea) []tea

fr fr FindStringSubmatchIndex yolos index pairs holding positions of matches
slay (p *VibePattern) FindStringSubmatchIndex(s tea) []int

fr fr FindAllString yolos all successive matches of the pattern
slay (p *VibePattern) FindAllString(s tea, n normie) []tea

fr fr FindAllStringIndex yolos indexes of all matches
slay (p *VibePattern) FindAllStringIndex(s tea, n normie) [][]int

fr fr FindAllStringSubmatch yolos all successive matches with submatch teas
slay (p *VibePattern) FindAllStringSubmatch(s tea, n normie) [][]tea

fr fr FindAllStringSubmatchIndex yolos indexes of all matches with submatch indexes
slay (p *VibePattern) FindAllStringSubmatchIndex(s tea, n normie) [][]int

fr fr ReplaceAllString yolos a copy with all matches replaced
slay (p *VibePattern) ReplaceAllString(src, repl tea) tea

fr fr ReplaceAllStringFunc yolos a copy with replacements determined by function
slay (p *VibePattern) ReplaceAllStringFunc(src tea, repl func(tea) tea) tea

fr fr Split slices s into subteas separated by pattern
slay (p *VibePattern) Split(s tea, n normie) []tea
```

## Helper Package Functions

```
fr fr Match reports whether the tea s contains any match of the pattern
slay Match(pattern tea, b []byte) (matched lit, err tea)

fr fr MatchString reports whether the tea s contains any match of the pattern
slay MatchString(pattern tea, s tea) (matched lit, err tea)

fr fr QuoteMeta yolos a tea that escapes all regexp metacharacters
slay QuoteMeta(s tea) tea
```

## Special Features

### `VibeGroups`
Provides named capture group information.

```
be_like VibeGroups squad {}

fr fr Methods
slay (p *VibePattern) GroupNames() []tea
slay (p *VibePattern) NamedGroups() map[tea]int
slay (p *VibePattern) FindGroupsString(s tea) map[tea]tea
```

### `VibeTemplates`
Extension for template-based replacements.

```
slay (p *VibePattern) TemplateReplace(s tea, template tea) tea
```

### `PatternBuilder`
Fluent collab for building regular expressions.

```
be_like PatternBuilder squad {}

fr fr Consquador
slay NewPatternBuilder() *PatternBuilder

fr fr Methods
slay (b *PatternBuilder) StartsWith(s tea) *PatternBuilder
slay (b *PatternBuilder) EndsWith(s tea) *PatternBuilder
slay (b *PatternBuilder) Contains(s tea) *PatternBuilder
slay (b *PatternBuilder) OneOrMore(s tea) *PatternBuilder
slay (b *PatternBuilder) ZeroOrMore(s tea) *PatternBuilder
slay (b *PatternBuilder) Optional(s tea) *PatternBuilder
slay (b *PatternBuilder) Group(s tea) *PatternBuilder
slay (b *PatternBuilder) NamedGroup(name, s tea) *PatternBuilder
slay (b *PatternBuilder) Or(patterns ...tea) *PatternBuilder
slay (b *PatternBuilder) Digit() *PatternBuilder
slay (b *PatternBuilder) Word() *PatternBuilder
slay (b *PatternBuilder) Space() *PatternBuilder
slay (b *PatternBuilder) Email() *PatternBuilder
slay (b *PatternBuilder) URL() *PatternBuilder
slay (b *PatternBuilder) Build() (*VibePattern, tea)
```

## Common Pattern Library

```
var (
    EmailPattern    = MustCompile(`...`) fr fr Email regex
    URLPattern      = MustCompile(`...`) fr fr URL regex
    DatePattern     = MustCompile(`...`) fr fr Date regex
    TimePattern     = MustCompile(`...`) fr fr Time regex
    UsernamePattern = MustCompile(`...`) fr fr Username regex
    PasswordPattern = MustCompile(`...`) fr fr Password regex
    PhonePattern    = MustCompile(`...`) fr fr Phone regex
    ZipCodePattern  = MustCompile(`...`) fr fr Zip code regex
    HashtagPattern  = MustCompile(`...`) fr fr Social media hashtag regex
    EmojiPattern    = MustCompile(`...`) fr fr Emoji regex
)
```

## Usage Example

```
fr fr Simple pattern matching
matched, _ := regex_vibez.MatchString("f[a-z]+", "frfr")
vibez.spill(matched) fr fr based

fr fr Compile and reuse a pattern
pattern, _ := regex_vibez.Compile("no ([a-z]+), bruh")
result := pattern.FindStringSubmatch("no cap, bruh")
vibez.spill(result[1]) fr fr "cringe"

fr fr Using the pattern builder
builder := regex_vibez.NewPatternBuilder()
emailPattern, _ := builder.StartsWith("^").NamedGroup("user", "[a-zA-Z0-9._%+-]+").Contains("@").NamedGroup("domain", "[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}").EndsWith("$").Build()

matches := emailPattern.FindGroupsString("user@example.com")
vibez.spill(matches["user"]) fr fr "user"
vibez.spill(matches["domain"]) fr fr "example.com"

fr fr Replacing text with regex
result = regex_vibez.MustCompile("cringe").ReplaceAllString("no cap", "lies")
vibez.spill(result) fr fr "no lies"
```

## Implementation Guidelines
1. Performance-focused implementation with efficient matching algorithms
2. Comprehensive tea handling for invalid patterns
3. Support for standard regular expression syntax plus extensions
4. Consistent API design following Cursed language patterns
5. Helpful debugging and visualization tools for complex patterns
6. Memory-efficient implementation for large inputs