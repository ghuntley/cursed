yeet "testz"
yeet "rizz_template"

# Test basic template creation
test_start("Template Creation")
sus template RizzTemplate = rizz_template_new("Hello {{name}}!")
assert_eq_string(template.template_content, "Hello {{name}}!")
vibez.spill("✅ Template creation test passed")

# Test variable setting and getting
test_start("Variable Management")
rizz_template_set_var(&template, "name", "CURSED")
sus retrieved tea = rizz_template_get_var(&template, "name")
assert_eq_string(retrieved, "CURSED")
vibez.spill("✅ Variable management test passed")

# Test basic variable interpolation
test_start("Variable Interpolation")
sus rendered tea = rizz_template_render(&template)
assert_eq_string(rendered, "Hello CURSED!")
vibez.spill("✅ Variable interpolation test passed")

# Test multiple variables
test_start("Multiple Variables")
sus multi_template RizzTemplate = rizz_template_new("{{greeting}} {{name}}, welcome to {{place}}!")
rizz_template_set_var(&multi_template, "greeting", "Hello")
rizz_template_set_var(&multi_template, "name", "Developer")
rizz_template_set_var(&multi_template, "place", "CURSED")
sus multi_rendered tea = rizz_template_render(&multi_template)
assert_eq_string(multi_rendered, "Hello Developer, welcome to CURSED!")
vibez.spill("✅ Multiple variables test passed")

# Test conditional rendering - true condition
test_start("Conditional Rendering - True")
sus cond_template RizzTemplate = rizz_template_new("{{if show_message}}This message is shown{{endif}}")
rizz_template_set_var(&cond_template, "show_message", "true")
sus cond_rendered tea = rizz_template_render(&cond_template)
assert_eq_string(cond_rendered, "This message is shown")
vibez.spill("✅ Conditional rendering (true) test passed")

# Test conditional rendering - false condition
test_start("Conditional Rendering - False")
sus cond_template2 RizzTemplate = rizz_template_new("{{if show_message}}This message is shown{{endif}}")
rizz_template_set_var(&cond_template2, "show_message", "false")
sus cond_rendered2 tea = rizz_template_render(&cond_template2)
assert_eq_string(cond_rendered2, "")
vibez.spill("✅ Conditional rendering (false) test passed")

# Test conditional with equality
test_start("Conditional Equality")
sus eq_template RizzTemplate = rizz_template_new("{{if status == \"active\"}}User is active{{endif}}")
rizz_template_set_var(&eq_template, "status", "active")
sus eq_rendered tea = rizz_template_render(&eq_template)
assert_eq_string(eq_rendered, "User is active")
vibez.spill("✅ Conditional equality test passed")

# Test loop rendering
test_start("Loop Rendering")
sus loop_template RizzTemplate = rizz_template_new("{{for item in items}}Item: {{item}}\n{{endfor}}")
rizz_template_set_var(&loop_template, "items", "apple,banana,cherry")
sus loop_rendered tea = rizz_template_render(&loop_template)
assert_eq_string(loop_rendered, "Item: apple\nItem: banana\nItem: cherry\n")
vibez.spill("✅ Loop rendering test passed")

# Test complex template with all features
test_start("Complex Template")
sus complex_content tea = `
Welcome {{name}}!

{{if is_premium}}
You are a premium user.
{{endif}}

Your items:
{{for item in items}}
- {{item}}
{{endfor}}

Thank you for using {{service_name}}!
`
sus complex_template RizzTemplate = rizz_template_new(complex_content)
rizz_template_set_var(&complex_template, "name", "John")
rizz_template_set_var(&complex_template, "is_premium", "true")
rizz_template_set_var(&complex_template, "items", "Task1,Task2,Task3")
rizz_template_set_var(&complex_template, "service_name", "CURSED")
sus complex_rendered tea = rizz_template_render(&complex_template)
assert_true(rizz_contains(complex_rendered, "Welcome John!"))
assert_true(rizz_contains(complex_rendered, "You are a premium user."))
assert_true(rizz_contains(complex_rendered, "- Task1"))
assert_true(rizz_contains(complex_rendered, "Thank you for using CURSED!"))
vibez.spill("✅ Complex template test passed")

# Test template validation
test_start("Template Validation")
sus valid_result lit
sus valid_error tea
(valid_result, valid_error) = rizz_template_validate("Hello {{name}}!")
assert_true(valid_result)
assert_eq_string(valid_error, "")
vibez.spill("✅ Template validation (valid) test passed")

# Test template validation with errors
test_start("Template Validation - Errors")
sus invalid_result lit
sus invalid_error tea
(invalid_result, invalid_error) = rizz_template_validate("Hello {{name}!")
assert_false(invalid_result)
assert_true(len(invalid_error) > 0)
vibez.spill("✅ Template validation (invalid) test passed")

# Test layout rendering
test_start("Layout Rendering")
sus content_template RizzTemplate = rizz_template_new("This is the main content about {{topic}}.")
rizz_template_set_var(&content_template, "topic", "Templates")
sus layout_content tea = `
<!DOCTYPE html>
<html>
<head><title>{{title}}</title></head>
<body>
<h1>{{title}}</h1>
{{content}}
</body>
</html>
`
rizz_template_set_var(&content_template, "title", "My Page")
sus layout_rendered tea = rizz_template_render_with_layout(&content_template, layout_content)
assert_true(rizz_contains(layout_rendered, "<title>My Page</title>"))
assert_true(rizz_contains(layout_rendered, "This is the main content about Templates."))
vibez.spill("✅ Layout rendering test passed")

# Test include functionality
test_start("Include Functionality")
sus main_template RizzTemplate = rizz_template_new("Header: {{header_content}}")
rizz_template_set_var(&main_template, "site_name", "CURSED Site")
sus include_content tea = "Welcome to {{site_name}}!"
sus include_rendered tea = rizz_template_include(&main_template, include_content)
assert_eq_string(include_rendered, "Welcome to CURSED Site!")
vibez.spill("✅ Include functionality test passed")

# Test helper functions
test_start("Helper Functions")
assert_true(rizz_starts_with_at("Hello World", 0, "Hello"))
assert_false(rizz_starts_with_at("Hello World", 0, "World"))
assert_eq_string(rizz_trim_whitespace("  test  "), "test")
assert_true(rizz_contains("Hello World", "World"))
assert_false(rizz_contains("Hello World", "xyz"))
vibez.spill("✅ Helper functions test passed")

# Test string splitting
test_start("String Splitting")
sus parts []tea = rizz_split("a,b,c", ",")
assert_eq_int(len(parts), 3)
assert_eq_string(parts[0], "a")
assert_eq_string(parts[1], "b")
assert_eq_string(parts[2], "c")
vibez.spill("✅ String splitting test passed")

# Test quote removal
test_start("Quote Removal")
assert_eq_string(rizz_remove_quotes("\"hello\""), "hello")
assert_eq_string(rizz_remove_quotes("hello"), "hello")
vibez.spill("✅ Quote removal test passed")

# Test nested conditions
test_start("Nested Conditions")
sus nested_template RizzTemplate = rizz_template_new("{{if user_logged_in}}Welcome {{name}}!{{if is_admin}} (Admin){{endif}}{{endif}}")
rizz_template_set_var(&nested_template, "user_logged_in", "true")
rizz_template_set_var(&nested_template, "name", "John")
rizz_template_set_var(&nested_template, "is_admin", "true")
sus nested_rendered tea = rizz_template_render(&nested_template)
assert_eq_string(nested_rendered, "Welcome John! (Admin)")
vibez.spill("✅ Nested conditions test passed")

# Test loop with conditions
test_start("Loop with Conditions")
sus loop_cond_template RizzTemplate = rizz_template_new("{{for item in items}}{{if item == \"special\"}}Special: {{item}}{{endif}}{{endfor}}")
rizz_template_set_var(&loop_cond_template, "items", "normal,special,other")
sus loop_cond_rendered tea = rizz_template_render(&loop_cond_template)
assert_eq_string(loop_cond_rendered, "Special: special")
vibez.spill("✅ Loop with conditions test passed")

# Test whitespace handling
test_start("Whitespace Handling")
sus ws_template RizzTemplate = rizz_template_new("{{  name  }}")
rizz_template_set_var(&ws_template, "name", "Test")
sus ws_rendered tea = rizz_template_render(&ws_template)
assert_eq_string(ws_rendered, "Test")
vibez.spill("✅ Whitespace handling test passed")

# Test empty template
test_start("Empty Template")
sus empty_template RizzTemplate = rizz_template_new("")
sus empty_rendered tea = rizz_template_render(&empty_template)
assert_eq_string(empty_rendered, "")
vibez.spill("✅ Empty template test passed")

# Test template compilation
test_start("Template Compilation")
sus compile_template RizzTemplate = rizz_template_new("Hello {{name}}!")
sus compile_result lit = rizz_template_compile(&compile_template)
assert_true(compile_result)
vibez.spill("✅ Template compilation test passed")

# Performance test
test_start("Performance Test")
sus perf_template RizzTemplate = rizz_template_new("{{for i in numbers}}Number: {{i}}\n{{endfor}}")
rizz_template_set_var(&perf_template, "numbers", "1,2,3,4,5,6,7,8,9,10")
sus perf_rendered tea = rizz_template_render(&perf_template)
assert_true(len(perf_rendered) > 0)
vibez.spill("✅ Performance test passed")

print_test_summary()
