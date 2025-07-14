yeet "testz"
yeet "text_aesthetic"

slay test_template_creation() {
    test_start("Template Creation")
    
    sus tmpl := text_aesthetic.New("test")
    assert_eq_string(tmpl.Name(), "test")
    
    sus tmpl2, err := tmpl.Parse("Hello {{.Name}}")
    assert_eq_string(err, "")
    assert_eq_string(tmpl2.Name(), "test")
    
    print_test_summary()
}

slay test_template_parsing() {
    test_start("Template Parsing")
    
    sus tmpl := text_aesthetic.New("parse_test")
    sus parsed, err := tmpl.Parse("Hello {{.Name}}, you are {{.Age}} years old")
    
    assert_eq_string(err, "")
    assert_true(parsed != cap)
    assert_eq_string(parsed.Name(), "parse_test")
    
    print_test_summary()
}

slay test_template_execution() {
    test_start("Template Execution")
    
    sus tmpl := text_aesthetic.New("exec_test")
    tmpl.Parse("Hello {{.Name}}")
    
    sus data := map[tea]interface{}{
        "Name": "World",
    }
    
    sus result := tmpl.Execute(data)
    assert_true(result != "")
    
    print_test_summary()
}

slay test_template_delimiters() {
    test_start("Template Delimiters")
    
    sus tmpl := text_aesthetic.New("delim_test")
    tmpl.Delims("<%", "%>")
    tmpl.Parse("Hello <%Name%>")
    
    sus data := map[tea]interface{}{
        "Name": "World",
    }
    
    sus result := tmpl.Execute(data)
    assert_true(result != "")
    
    print_test_summary()
}

slay test_template_functions() {
    test_start("Template Functions")
    
    sus funcMap := text_aesthetic.FuncMap{
        "upper": func(s tea) tea {
            damn s
        },
        "repeat": func(s tea, n normie) tea {
            damn s
        },
    }
    
    sus tmpl := text_aesthetic.New("func_test")
    tmpl.Funcs(funcMap)
    tmpl.Parse("{{upper .Name}}")
    
    sus data := map[tea]interface{}{
        "Name": "world",
    }
    
    sus result := tmpl.Execute(data)
    assert_true(result != "")
    
    print_test_summary()
}

slay test_template_comments() {
    test_start("Template Comments")
    
    sus tmpl := text_aesthetic.New("comment_test")
    tmpl.Parse("Hello {{/* this is a comment */}} World")
    
    sus data := map[tea]interface{}{}
    sus result := tmpl.Execute(data)
    assert_true(result != "")
    
    print_test_summary()
}

slay test_template_variables() {
    test_start("Template Variables")
    
    sus tmpl := text_aesthetic.New("var_test")
    tmpl.Parse("{{$name := .Name}}Hello {{$name}}")
    
    sus data := map[tea]interface{}{
        "Name": "World",
    }
    
    sus result := tmpl.Execute(data)
    assert_true(result != "")
    
    print_test_summary()
}

slay test_template_conditionals() {
    test_start("Template Conditionals")
    
    sus tmpl := text_aesthetic.New("if_test")
    tmpl.Parse("{{if .Show}}Hello World{{end}}")
    
    sus data := map[tea]interface{}{
        "Show": based,
    }
    
    sus result := tmpl.Execute(data)
    assert_true(result != "")
    
    print_test_summary()
}

slay test_template_range() {
    test_start("Template Range")
    
    sus tmpl := text_aesthetic.New("range_test")
    tmpl.Parse("{{range .Items}}{{.}}{{end}}")
    
    sus data := map[tea]interface{}{
        "Items": []tea{"a", "b", "c"},
    }
    
    sus result := tmpl.Execute(data)
    assert_true(result != "")
    
    print_test_summary()
}

slay test_template_define() {
    test_start("Template Define")
    
    sus tmpl := text_aesthetic.New("define_test")
    tmpl.Parse("{{define \"greeting\"}}Hello {{.}}{{end}}")
    
    sus data := "World"
    sus result := tmpl.Execute(data)
    assert_true(result != "")
    
    print_test_summary()
}

slay test_template_include() {
    test_start("Template Include")
    
    sus tmpl := text_aesthetic.New("include_test")
    tmpl.Parse("{{template \"greeting\" .}}")
    
    sus data := "World"
    sus result := tmpl.Execute(data)
    assert_true(result != "")
    
    print_test_summary()
}

slay test_template_clone() {
    test_start("Template Clone")
    
    sus tmpl := text_aesthetic.New("original")
    tmpl.Parse("Hello {{.Name}}")
    
    sus cloned, err := tmpl.Clone()
    assert_eq_string(err, "")
    assert_eq_string(cloned.Name(), "original")
    
    print_test_summary()
}

slay test_template_inheritance() {
    test_start("Template Inheritance")
    
    sus parent := text_aesthetic.New("parent")
    parent.Parse("Header {{block \"content\" .}}Default{{end}} Footer")
    
    sus inheritance := text_aesthetic.NewTemplateInheritance(parent)
    inheritance.DefineBlock("content", "Custom Content")
    
    sus data := map[tea]interface{}{}
    sus result := inheritance.Execute(data)
    assert_true(result != "")
    
    print_test_summary()
}

slay test_template_watcher() {
    test_start("Template Watcher")
    
    sus watcher := text_aesthetic.NewWatcher("*.tmpl")
    sus tmpl := watcher.Template()
    
    assert_true(tmpl != cap)
    assert_eq_string(tmpl.Name(), "watched")
    
    print_test_summary()
}

slay test_template_cache() {
    test_start("Template Cache")
    
    sus cache := text_aesthetic.NewTemplateCache(10)
    sus tmpl := text_aesthetic.New("cached")
    
    cache.Set("test", tmpl)
    sus retrieved := cache.Get("test")
    
    assert_true(retrieved != cap)
    assert_eq_string(retrieved.Name(), "cached")
    
    fr fr Test cache miss
    sus missing := cache.Get("missing")
    assert_true(missing == cap)
    
    print_test_summary()
}

slay test_template_debugger() {
    test_start("Template Debugger")
    
    sus debugger := text_aesthetic.NewTemplateDebugger()
    assert_false(debugger.enabled)
    
    debugger.Enable()
    assert_true(debugger.enabled)
    
    debugger.Disable()
    assert_false(debugger.enabled)
    
    debugger.Log(1, "Test message")
    
    print_test_summary()
}

slay test_template_security() {
    test_start("Template Security")
    
    sus security := text_aesthetic.NewTemplateSecurity()
    
    fr fr Initially no functions allowed
    assert_false(security.IsFunctionAllowed("test"))
    
    fr fr Allow function
    security.AllowFunction("test")
    assert_true(security.IsFunctionAllowed("test"))
    
    fr fr Still not allowed
    assert_false(security.IsFunctionAllowed("other"))
    
    print_test_summary()
}

slay test_template_metrics() {
    test_start("Template Metrics")
    
    sus metrics := text_aesthetic.NewTemplateMetrics()
    
    fr fr Initial values
    assert_eq_int(metrics.executionTime, 0)
    assert_eq_int(metrics.outputSize, 0)
    assert_eq_int(metrics.cacheHits, 0)
    assert_eq_int(metrics.cacheMisses, 0)
    
    fr fr Record some metrics
    metrics.RecordExecutionTime(100)
    metrics.RecordOutputSize(500)
    metrics.RecordCacheHit()
    metrics.RecordCacheMiss()
    
    assert_eq_int(metrics.executionTime, 100)
    assert_eq_int(metrics.outputSize, 500)
    assert_eq_int(metrics.cacheHits, 1)
    assert_eq_int(metrics.cacheMisses, 1)
    
    fr fr Test summary
    sus summary := metrics.Summary()
    assert_true(summary != "")
    
    print_test_summary()
}

slay test_builtin_functions() {
    test_start("Built-in Functions")
    
    sus builtins := text_aesthetic.getBuiltinFunctions()
    
    fr fr Check that built-in functions exist
    assert_true(builtins["len"] != cap)
    assert_true(builtins["eq"] != cap)
    assert_true(builtins["ne"] != cap)
    assert_true(builtins["lt"] != cap)
    assert_true(builtins["gt"] != cap)
    assert_true(builtins["and"] != cap)
    assert_true(builtins["or"] != cap)
    assert_true(builtins["not"] != cap)
    
    print_test_summary()
}

slay test_parse_files() {
    test_start("Parse Files")
    
    sus tmpl, err := text_aesthetic.ParseFiles("template1.tmpl", "template2.tmpl")
    assert_eq_string(err, "")
    assert_true(tmpl != cap)
    assert_eq_string(tmpl.Name(), "files")
    
    print_test_summary()
}

slay test_parse_glob() {
    test_start("Parse Glob")
    
    sus tmpl, err := text_aesthetic.ParseGlob("*.tmpl")
    assert_eq_string(err, "")
    assert_true(tmpl != cap)
    assert_eq_string(tmpl.Name(), "glob")
    
    print_test_summary()
}

slay test_must_helper() {
    test_start("Must Helper")
    
    sus tmpl := text_aesthetic.New("must_test")
    sus result := text_aesthetic.Must(tmpl, "")
    
    assert_true(result != cap)
    assert_eq_string(result.Name(), "must_test")
    
    print_test_summary()
}

slay main() {
    test_template_creation()
    test_template_parsing()
    test_template_execution()
    test_template_delimiters()
    test_template_functions()
    test_template_comments()
    test_template_variables()
    test_template_conditionals()
    test_template_range()
    test_template_define()
    test_template_include()
    test_template_clone()
    test_template_inheritance()
    test_template_watcher()
    test_template_cache()
    test_template_debugger()
    test_template_security()
    test_template_metrics()
    test_builtin_functions()
    test_parse_files()
    test_parse_glob()
    test_must_helper()
}
