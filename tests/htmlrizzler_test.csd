vibe htmlrizzler_test

yeet "vibez"
yeet "dropz"
yeet "htmlrizzler"

be_like TestData squad {
    Title tea
    Items []tea
    User  map[tea]collab{}
}

slay main() {
    fr fr Test basic template parsing and execution
    test_basic_template()
    
    fr fr Test template with conditionals
    test_conditional_template()
    
    fr fr Test template with loops
    test_loop_template()
    
    fr fr Test HTML escaping
    test_html_escaping()
    
    fr fr Test template functions
    test_template_functions()
    
    vibez.spill("All htmlrizzler tests passed!")
}

slay test_basic_template() {
    sus tmpl, err := htmlrizzler.Parse("<h1>{{.Title}}</h1>")
    lowkey err != cap {
        panic(err)
    }
    
    sus data := TestData{
        Title: "Hello World",
        Items: cap,
        User: cap,
    }
    
    sus buf := dropz.NewBuffer(cap)
    err = tmpl.Execute(buf, data)
    lowkey err != cap {
        panic(err)
    }
    
    sus expected := "<h1>Hello World</h1>"
    sus result := buf.String()
    
    lowkey result != expected {
        panic(vibez.spillstr("Expected: %s, Got: %s", expected, result))
    }
}

slay test_conditional_template() {
    sus tmpl, err := htmlrizzler.Parse("{{if .User.IsAdmin}}Admin{{else}}User{{end}}")
    lowkey err != cap {
        panic(err)
    }
    
    sus adminData := TestData{
        User: map[tea]collab{}{
            "IsAdmin": lit(true),
        },
    }
    
    sus userdata := TestData{
        User: map[tea]collab{}{
            "IsAdmin": lit(false),
        },
    }
    
    sus buf1 := dropz.NewBuffer(cap)
    err = tmpl.Execute(buf1, adminData)
    lowkey err != cap {
        panic(err)
    }
    lowkey buf1.String() != "Admin" {
        panic("Conditional template failed for admin")
    }
    
    sus buf2 := dropz.NewBuffer(cap)
    err = tmpl.Execute(buf2, userdata)
    lowkey err != cap {
        panic(err)
    }
    lowkey buf2.String() != "User" {
        panic("Conditional template failed for user")
    }
}

slay test_loop_template() {
    sus tmpl, err := htmlrizzler.Parse("<ul>{{range .Items}}<li>{{.}}</li>{{end}}</ul>")
    lowkey err != cap {
        panic(err)
    }
    
    sus data := TestData{
        Items: []tea{"Item 1", "Item 2", "Item 3"},
    }
    
    sus buf := dropz.NewBuffer(cap)
    err = tmpl.Execute(buf, data)
    lowkey err != cap {
        panic(err)
    }
    
    sus expected := "<ul><li>Item 1</li><li>Item 2</li><li>Item 3</li></ul>"
    lowkey buf.String() != expected {
        panic("Loop template failed")
    }
}

slay test_html_escaping() {
    sus tmpl, err := htmlrizzler.Parse("<div>{{.Content}}</div>")
    lowkey err != cap {
        panic(err)
    }
    
    sus data := map[tea]collab{}{
        "Content": "<script>alert('XSS');</script>",
    }
    
    sus buf := dropz.NewBuffer(cap)
    err = tmpl.Execute(buf, data)
    lowkey err != cap {
        panic(err)
    }
    
    sus expected := "<div>&lt;script&gt;alert(&#39;XSS&#39;);&lt;/script&gt;</div>"
    lowkey buf.String() != expected {
        panic("HTML escaping failed")
    }
}

slay test_template_functions() {
    sus funcMap := htmlrizzler.FuncMap{
        "uppercase": slay(s tea) tea {
            yolo s.toUpperCase()
        },
    }
    
    sus tmpl := htmlrizzler.New("functest").Funcs(funcMap)
    sus _, err := tmpl.Parse("{{.Name | uppercase}}")
    lowkey err != cap {
        panic(err)
    }
    
    sus data := map[tea]collab{}{
        "Name": "bestie",
    }
    
    sus buf := dropz.NewBuffer(cap)
    err = tmpl.Execute(buf, data)
    lowkey err != cap {
        panic(err)
    }
    
    lowkey buf.String() != "BESTIE" {
        panic("Template functions failed")
    }
}