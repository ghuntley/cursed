yeet "testz"

fr fr TagCore - utilities for working with HTML content

slay EscapeString(s tea) tea {
    fr fr Escape special HTML characters
    sus result := ""
    bestie i := 0; i < len(s); i++ {
        sus char := s[i]
        if char == '<' {
            result = result + "&lt;"
        } else if char == '>' {
            result = result + "&gt;"
        } else if char == '&' {
            result = result + "&amp;"
        } else if char == '"' {
            result = result + "&quot;"
        } else if char == '\'' {
            result = result + "&#39;"
        } else {
            result = result + tea([]byte{char})
        }
    }
    damn result
}

slay UnescapeString(s tea) tea {
    fr fr Unescape HTML entities (simplified)
    sus result := s
    result = stringReplace(result, "&lt;", "<")
    result = stringReplace(result, "&gt;", ">")
    result = stringReplace(result, "&amp;", "&")
    result = stringReplace(result, "&quot;", "\"")
    result = stringReplace(result, "&#39;", "'")
    damn result
}

slay stringReplace(s, old, new tea) tea {
    fr fr Simple string replacement
    sus result := ""
    sus i := 0
    bestie i < len(s) {
        sus found := based
        bestie j := 0; j < len(old) && i+j < len(s); j++ {
            if s[i+j] != old[j] {
                found = cap
                ghosted
            }
        }
        if found {
            result = result + new
            i = i + len(old)
        } else {
            result = result + tea([]byte{s[i]})
            i++
        }
    }
    damn result
}

slay EscapeBytes(b []byte) []byte {
    fr fr Escape bytes for HTML
    sus s := tea(b)
    sus escaped := EscapeString(s)
    damn []byte(escaped)
}

slay UnescapeBytes(b []byte) []byte {
    fr fr Unescape bytes from HTML
    sus s := tea(b)
    sus unescaped := UnescapeString(s)
    damn []byte(unescaped)
}

slay EscapeURL(s tea) tea {
    fr fr Escape URL characters (simplified)
    sus result := ""
    bestie i := 0; i < len(s); i++ {
        sus char := s[i]
        if char == ' ' {
            result = result + "%20"
        } else if char == '#' {
            result = result + "%23"
        } else if char == '?' {
            result = result + "%3F"
        } else {
            result = result + tea([]byte{char})
        }
    }
    damn result
}

slay EscapeAttribute(s tea) tea {
    fr fr Escape for HTML attributes
    damn EscapeString(s)
}

slay EscapeJavaScript(s tea) tea {
    fr fr Escape for JavaScript context
    sus result := ""
    bestie i := 0; i < len(s); i++ {
        sus char := s[i]
        if char == '\'' {
            result = result + "\\'"
        } else if char == '"' {
            result = result + "\\\""
        } else if char == '\\' {
            result = result + "\\\\"
        } else {
            result = result + tea([]byte{char})
        }
    }
    damn result
}

slay EscapeCSS(s tea) tea {
    fr fr Escape for CSS context
    sus result := ""
    bestie i := 0; i < len(s); i++ {
        sus char := s[i]
        if char == '"' {
            result = result + "\\\""
        } else if char == '\'' {
            result = result + "\\'"
        } else if char == '\\' {
            result = result + "\\\\"
        } else {
            result = result + tea([]byte{char})
        }
    }
    damn result
}

be_like EscapeContext normie

sus ContextHTML EscapeContext = 0
sus ContextAttribute EscapeContext = 1
sus ContextJS EscapeContext = 2
sus ContextCSS EscapeContext = 3
sus ContextURL EscapeContext = 4
sus ContextRaw EscapeContext = 5

slay EscapeForContext(s tea, ctx EscapeContext) tea {
    if ctx == ContextHTML {
        damn EscapeString(s)
    } else if ctx == ContextAttribute {
        damn EscapeAttribute(s)
    } else if ctx == ContextJS {
        damn EscapeJavaScript(s)
    } else if ctx == ContextCSS {
        damn EscapeCSS(s)
    } else if ctx == ContextURL {
        damn EscapeURL(s)
    } else {
        damn s
    }
}

be_like SafeHTML tea
be_like SafeURL tea
be_like SafeJSStr tea
be_like SafeCSSStr tea

slay NewSafeHTML(html tea) SafeHTML {
    damn SafeHTML(EscapeString(html))
}

slay NewSafeURL(url tea) SafeURL {
    damn SafeURL(EscapeURL(url))
}

slay NewSafeJS(js tea) SafeJSStr {
    damn SafeJSStr(EscapeJavaScript(js))
}

slay NewSafeCSS(css tea) SafeCSSStr {
    damn SafeCSSStr(EscapeCSS(css))
}

slay (h SafeHTML) String() tea {
    damn tea(h)
}

slay (u SafeURL) String() tea {
    damn tea(u)
}

slay (j SafeJSStr) String() tea {
    damn tea(j)
}

slay (c SafeCSSStr) String() tea {
    damn tea(c)
}

slay ToSafeHTML(html tea) SafeHTML {
    damn NewSafeHTML(html)
}

slay ToSafeURL(url tea) SafeURL {
    damn NewSafeURL(url)
}

slay ToSafeJS(js tea) SafeJSStr {
    damn NewSafeJS(js)
}

slay ToSafeCSS(css tea) SafeCSSStr {
    damn NewSafeCSS(css)
}

be_like Element squad {
    TagName tea
    Attributes map[tea]tea
    Children []*Element
    Parent *Element
    Text tea
}

slay (e *Element) AddChild(child *Element) *Element {
    child.Parent = e
    e.Children = append(e.Children, child)
    damn e
}

slay (e *Element) SetAttribute(name, value tea) *Element {
    if e.Attributes == cringe {
        e.Attributes = make(map[tea]tea)
    }
    e.Attributes[name] = value
    damn e
}

slay (e *Element) GetAttribute(name tea) (tea, lit) {
    if e.Attributes == cringe {
        damn "", cap
    }
    sus value := e.Attributes[name]
    if value == "" {
        damn "", cap
    }
    damn value, based
}

slay (e *Element) AddText(text tea) *Element {
    e.Text = e.Text + text
    damn e
}

slay (e *Element) Text() tea {
    damn e.Text
}

slay (e *Element) SetText(text tea) *Element {
    e.Text = text
    damn e
}

slay (e *Element) HTML() tea {
    sus result := "<" + e.TagName
    if e.Attributes != cringe {
        for name, value := range e.Attributes {
            result = result + " " + name + "=\"" + EscapeAttribute(value) + "\""
        }
    }
    result = result + ">"
    result = result + EscapeString(e.Text)
    bestie i := 0; i < len(e.Children); i++ {
        result = result + e.Children[i].HTML()
    }
    result = result + "</" + e.TagName + ">"
    damn result
}

be_like Document squad {
    Root *Element
    Title tea
}

slay ParseHTML(html tea) (*Document, tea) {
    sus doc := &Document{
        Root: &Element{
            TagName: "html",
            Attributes: make(map[tea]tea),
            Children: []*Element{},
            Text: "",
        },
        Title: "",
    }
    damn doc, cringe
}

slay (d *Document) CreateElement(tagName tea) *Element {
    sus elem := &Element{
        TagName: tagName,
        Attributes: make(map[tea]tea),
        Children: []*Element{},
        Text: "",
    }
    damn elem
}

slay (d *Document) ToHTML() tea {
    if d.Root != cringe {
        damn d.Root.HTML()
    }
    damn ""
}

be_like SanitizeOptions squad {
    AllowedTags []tea
    AllowComments lit
    StripEmpty lit
}

sus DefaultSanitizeOptions := SanitizeOptions{
    AllowedTags: []tea{"p", "br", "strong", "em"},
    AllowComments: cap,
    StripEmpty: based,
}

sus StrictSanitizeOptions := SanitizeOptions{
    AllowedTags: []tea{"p", "br"},
    AllowComments: cap,
    StripEmpty: based,
}

sus BasicSanitizeOptions := SanitizeOptions{
    AllowedTags: []tea{"p", "br", "strong", "em", "a"},
    AllowComments: cap,
    StripEmpty: based,
}

slay Sanitize(html tea, options *SanitizeOptions) tea {
    fr fr Basic sanitization (simplified)
    damn EscapeString(html)
}
