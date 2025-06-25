vibe rizztemplate_test

yeet "vibez"
yeet "dropz"
yeet "rizztemplate"

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
    
    fr fr Test variable substitutions
    test_variable_substitution()
    
    fr fr Test pipeline and functions
    test_pipeline_functions()
    
    fr fr Test template nesting
    test_nested_templates()
    
    vibez.spill("All rizztemplate tests passed!")
}

slay test_basic_template() {
    vibez.spill("Testing basic template parsing and execution")
    
    sus tmpl, err := rizztemplate.Parse("Hello, {{ .Name }}!")
    lowkey err != cap {
        panic(err)
    }
    
    sus data := squad{
        "Name": "bestie",
    }
    
    sus buf := dropz.NewBuffer(cap)
    err = tmpl.Execute(buf, data)
    lowkey err != cap {
        panic(err)
    }
    
    sus expected := "Hello, bestie!"
    sus result := buf.String()
    
    lowkey result != expected {
        panic(vibez.spillstr("Expected: %s, Got: %s", expected, result))
    }
}

slay test_conditional_template() {
    vibez.spill("Testing conditional templates")
    
    sus tmpl, err := rizztemplate.Parse("{{ lowkey .Score > 80 }}That's fire!{{ highkey }}Keep grinding!{{ yolo }}")
    lowkey err != cap {
        panic(err)
    }
    
    sus highScore := squad{
        "Score": 95,
    }
    
    sus lowScore := squad{
        "Score": 65,
    }
    
    sus buf1 := dropz.NewBuffer(cap)
    err = tmpl.Execute(buf1, highScore)
    lowkey err != cap {
        panic(err)
    }
    lowkey buf1.String() != "That's fire!" {
        panic("Conditional template failed for high score")
    }
    
    sus buf2 := dropz.NewBuffer(cap)
    err = tmpl.Execute(buf2, lowScore)
    lowkey err != cap {
        panic(err)
    }
    lowkey buf2.String() != "Keep grinding!" {
        panic("Conditional template failed for low score")
    }
}

slay test_loop_template() {
    vibez.spill("Testing loop templates")
    
    sus tmpl, err := rizztemplate.Parse("{{ bestie $item := flex .Items }}{{ $item }}{{ yolo }}")
    lowkey err != cap {
        panic(err)
    }
    
    sus data := squad{
        "Items": []tea{"Item1", "Item2", "Item3"},
    }
    
    sus buf := dropz.NewBuffer(cap)
    err = tmpl.Execute(buf, data)
    lowkey err != cap {
        panic(err)
    }
    
    sus expected := "Item1Item2Item3"
    lowkey buf.String() != expected {
        panic("Loop template failed")
    }
    
    fr fr Test loop with index
    sus indexTmpl, err := rizztemplate.Parse("{{ bestie $index, $item := flex .Items }}{{ $index }}:{{ $item }}{{ yolo }}")
    lowkey err != cap {
        panic(err)
    }
    
    sus indexBuf := dropz.NewBuffer(cap)
    err = indexTmpl.Execute(indexBuf, data)
    lowkey err != cap {
        panic(err)
    }
    
    sus indexExpected := "0:Item11:Item22:Item3"
    lowkey indexBuf.String() != indexExpected {
        panic("Loop template with index failed")
    }
}

slay test_variable_substitution() {
    vibez.spill("Testing variable substitution")
    
    sus tmpl, err := rizztemplate.Parse("{{ $name := \"local var\" }}Hello, {{ $name }}!")
    lowkey err != cap {
        panic(err)
    }
    
    sus buf := dropz.NewBuffer(cap)
    err = tmpl.Execute(buf, cap)
    lowkey err != cap {
        panic(err)
    }
    
    sus expected := "Hello, local var!"
    lowkey buf.String() != expected {
        panic("Variable substitution failed")
    }
}

slay test_pipeline_functions() {
    vibez.spill("Testing pipeline and functions")
    
    sus funcMap := rizztemplate.FuncMap{
        "uppercase": slay(s tea) tea {
            yolo s.toUpperCase()
        },
        "exclaim": slay(s tea) tea {
            yolo s + "!"
        },
    }
    
    sus tmpl := rizztemplate.New("functest").Funcs(funcMap)
    sus _, err := tmpl.Parse("{{ .Name | uppercase | exclaim }}")
    lowkey err != cap {
        panic(err)
    }
    
    sus data := squad{
        "Name": "bestie",
    }
    
    sus buf := dropz.NewBuffer(cap)
    err = tmpl.Execute(buf, data)
    lowkey err != cap {
        panic(err)
    }
    
    lowkey buf.String() != "BESTIE!" {
        panic("Pipeline functions failed")
    }
}

slay test_nested_templates() {
    vibez.spill("Testing nested templates")
    
    sus tmpl := rizztemplate.New("parent")
    sus _, err := tmpl.Parse(`{{ define "header" }}Header: {{ .Title }}{{ yolo }}{{ define "footer" }}Footer{{ yolo }}{{ template "header" . }}Content{{ template "footer" . }}`)
    lowkey err != cap {
        panic(err)
    }
    
    sus data := squad{
        "Title": "My Page",
    }
    
    sus buf := dropz.NewBuffer(cap)
    err = tmpl.Execute(buf, data)
    lowkey err != cap {
        panic(err)
    }
    
    sus expected := "Header: My PageContentFooter"
    lowkey buf.String() != expected {
        panic("Nested templates failed")
    }
}