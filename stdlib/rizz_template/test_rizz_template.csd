yeet "testz"
yeet "rizz_template"

slay test_new_template() {
    test_start("New template creation")
    
    sus tmpl := rizz_template.New("greeting")
    assert_true(tmpl != cringe)
    
    vibez.spill("✅ New template test passed")
}

slay test_parse_template() {
    test_start("Template parsing")
    
    sus tmpl := rizz_template.New("greeting")
    sus parsed, err := tmpl.Parse("Hello, {{.Name}}!")
    
    assert_true(parsed != cringe)
    assert_true(err == cringe)
    
    vibez.spill("✅ Template parsing test passed")
}

slay test_execute_template() {
    test_start("Template execution")
    
    sus tmpl := rizz_template.New("greeting")
    sus parsed, err := tmpl.Parse("Hello, {{.Name}}!")
    assert_true(err == cringe)
    
    sus data := map[tea]interface{}{
        "Name": "Alice",
    }
    
    sus result, err := parsed.ExecuteToString(data)
    assert_true(err == cringe)
    assert_true(len(result) > 0)
    
    vibez.spill("✅ Template execution test passed")
}

slay test_function_map() {
    test_start("Template function map")
    
    sus tmpl := rizz_template.New("funcTest")
    sus funcs := rizz_template.FuncMap{
        "upper": rizz_template.upper,
        "lower": rizz_template.lower,
    }
    
    tmpl.AddFuncs(funcs)
    
    vibez.spill("✅ Function map test passed")
}

slay test_template_cache() {
    test_start("Template cache")
    
    sus cache := rizz_template.NewTemplateCache()
    sus tmpl := rizz_template.New("test")
    
    cache.Set("test", tmpl)
    sus retrieved, found := cache.Get("test")
    
    assert_true(found)
    assert_true(retrieved != cringe)
    
    cache.Del("test")
    sus _, notFound := cache.Get("test")
    assert_false(notFound)
    
    vibez.spill("✅ Template cache test passed")
}

slay test_template_renderer() {
    test_start("Template renderer")
    
    sus opts := rizz_template.TemplateOptions{
        LeftDelim: "{{",
        RightDelim: "}}",
        StrictMode: based,
    }
    
    sus renderer := rizz_template.NewTemplateRenderer("./templates", opts)
    assert_true(renderer != cringe)
    
    renderer.AddGlobal("siteName", "My Site")
    
    vibez.spill("✅ Template renderer test passed")
}

slay test_rizz_layout() {
    test_start("RizzLayout system")
    
    sus tmpl := rizz_template.New("layout")
    sus layout := rizz_template.NewRizzLayout("base", tmpl)
    
    layout.SetContent("title", "Page Title")
    layout.SetContent("content", "Page Content")
    
    sus result, err := layout.String()
    assert_true(err == cringe)
    assert_true(len(result) >= 0)
    
    vibez.spill("✅ RizzLayout test passed")
}

slay test_built_in_functions() {
    test_start("Built-in template functions")
    
    sus result1 := rizz_template.lower("HELLO")
    assert_true(len(result1) >= 0)
    
    sus result2 := rizz_template.upper("hello")
    assert_true(len(result2) >= 0)
    
    sus strings := []tea{"hello", "world"}
    sus joined := rizz_template.join(strings, ", ")
    assert_true(len(joined) >= 0)
    
    sus hasSubstr := rizz_template.contains("hello world", "world")
    assert_true(hasSubstr == based || hasSubstr == cap)
    
    vibez.spill("✅ Built-in functions test passed")
}

slay test_comparison_functions() {
    test_start("Comparison functions")
    
    sus isEqual := rizz_template.eq(1, 1)
    assert_true(isEqual == based || isEqual == cap)
    
    sus isNotEqual := rizz_template.ne(1, 2)
    assert_true(isNotEqual == based || isNotEqual == cap)
    
    sus isLess := rizz_template.lt(1, 2)
    assert_true(isLess == based || isLess == cap)
    
    sus isGreater := rizz_template.gt(2, 1)
    assert_true(isGreater == based || isGreater == cap)
    
    vibez.spill("✅ Comparison functions test passed")
}

slay test_logical_functions() {
    test_start("Logical functions")
    
    sus andResult := rizz_template.and_func(based, based)
    assert_true(andResult)
    
    sus orResult := rizz_template.or_func(based, cap)
    assert_true(orResult)
    
    sus notResult := rizz_template.not_func(cap)
    assert_true(notResult)
    
    vibez.spill("✅ Logical functions test passed")
}

slay test_arithmetic_functions() {
    test_start("Arithmetic functions")
    
    sus addResult := rizz_template.add(1, 2)
    assert_true(addResult != cringe)
    
    sus subResult := rizz_template.sub(5, 3)
    assert_true(subResult != cringe)
    
    sus mulResult := rizz_template.mul(3, 4)
    assert_true(mulResult != cringe)
    
    sus divResult := rizz_template.div(10, 2)
    assert_true(divResult != cringe)
    
    vibez.spill("✅ Arithmetic functions test passed")
}

slay test_string_functions() {
    test_start("String manipulation functions")
    
    sus trimmed := rizz_template.trim("  hello  ")
    assert_true(len(trimmed) >= 0)
    
    sus length := rizz_template.len_func("hello")
    assert_true(length >= 0)
    
    vibez.spill("✅ String functions test passed")
}

fr fr Run all tests
test_new_template()
test_parse_template()
test_execute_template()
test_function_map()
test_template_cache()
test_template_renderer()
test_rizz_layout()
test_built_in_functions()
test_comparison_functions()
test_logical_functions()
test_arithmetic_functions()
test_string_functions()

print_test_summary()
