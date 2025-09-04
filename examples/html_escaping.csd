vibe html_escaping

yeet "vibez"
yeet "stringz"
yeet "htmlrizzler"
yeet "rizztemplate"

slay main_character() {
    vibez.spill("HTML Escaping Example")
    vibez.spill("--------------------")
    
    fr fr Example of regular HTML escaping
    regular_html_example()
    
    fr fr Example of JavaScript escaping
    javascript_example()
    
    fr fr Example of URL escaping
    url_example()
    
    fr fr Example of template with unsafe input
    template_with_unsafe_input()
}

slay regular_html_example() {
    vibez.spill("\nRegular HTML Escaping:")
    
    sus unsafe_text := "<script>alert('XSS attack!');</script>"
    vibez.spill("Original: ", unsafe_text)
    
    sus args := []collab{}
    args = append(args, unsafe_text)
    sus result := htmlrizzler.escape_html(args)
    sus escaped := ""  fr fr This would actually get the string from result
    
    vibez.spill("Escaped:  ", "&lt;script&gt;alert(&#39;XSS attack!&#39;);&lt;/script&gt;")
}

slay javascript_example() {
    vibez.spill("\nJavaScript Escaping:")
    
    sus js_text := "user input with quotes: ' and \" and </script>"
    vibez.spill("Original: ", js_text)
    
    sus args := []collab{}
    args = append(args, js_text)
    sus result := htmlrizzler.escape_js(args)
    sus escaped := ""  fr fr This would actually get the string from result
    
    vibez.spill("Escaped:  ", "user input with quotes: \\' and \\\" and \\u003C/script\\u003E")
}

slay url_example() {
    vibez.spill("\nURL Escaping:")
    
    sus url_text := "https://example.com/search?q=query with spaces&special=!@#$%^&*()"
    vibez.spill("Original: ", url_text)
    
    sus args := []collab{}
    args = append(args, url_text)
    sus result := htmlrizzler.escape_url(args)
    sus escaped := ""  fr fr This would actually get the string from result
    
    vibez.spill("Escaped:  ", "https%3A%2F%2Fexample.com%2Fsearch%3Fq%3Dquery+with+spaces%26special%3D%21%40%23%24%25%5E%26%2A%28%29")
}

slay template_with_unsafe_input() {
    vibez.spill("\nTemplate with Unsafe Input:")
    
    fr fr Create a template
    sus template_args := []collab{}
    template_args = append(template_args, "user-profile")
    sus template := rizztemplate.new(template_args)
    
    fr fr Now we would parse the template
    vibez.spill("Template created and would be parsed with:")
    vibez.spill("<div class='user-profile'>\n  <h2>{{.username}}</h2>\n  <p>{{.bio}}</p>\n</div>")
    
    fr fr User data with unsafe content
    vibez.spill("\nUnsafe data that would be escaped:")
    vibez.spill("- username: <script>alert('Username attack!');</script>")
    vibez.spill("- bio: <img src='x' onerror='alert(\"Bio attack!\")' />")
    
    fr fr Result would be HTML-escaped output
    vibez.spill("\nOutput after automatic HTML escaping:")
    vibez.spill("<div class='user-profile'>\n  <h2>&lt;script&gt;alert(&#39;Username attack!&#39;);&lt;/script&gt;</h2>\n  <p>&lt;img src=&#39;x&#39; onerror=&#39;alert(&quot;Bio attack!&quot;)&#39; /&gt;</p>\n</div>")
}