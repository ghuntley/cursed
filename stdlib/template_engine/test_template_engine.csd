yeet "testz"
yeet "template_engine"

// Test Template Loading
test_start("template_load")
sus result lit = template_load("test.html")
assert_true(result)
assert_true(template_is_loaded())

test_start("template_load_string")
template_clear()
sus result2 lit = template_load_string("<h1>{{title}}</h1>")
assert_true(result2)
assert_true(template_is_loaded())

test_start("template_get_content")
template_load_string("<h1>{{title}}</h1>")
sus content tea = template_get_content()
assert_eq_string(content, "<h1>{{title}}</h1>")

test_start("template_clear")
template_load("test.html")
assert_true(template_is_loaded())
template_clear()
assert_false(template_is_loaded())

// Test Variable Management
test_start("template_set_var")
template_load_string("<h1>{{title}}</h1>")
sus var_result lit = template_set_var("title", "My Title")
assert_true(var_result)

test_start("template_has_var")
template_load_string("<h1>{{title}}</h1>")
template_set_var("title", "My Title")
assert_true(template_has_var("title"))
assert_false(template_has_var("nonexistent"))

test_start("template_get_var")
template_load_string("<h1>{{title}}</h1>")
template_set_var("title", "My Title")
sus var_value tea = template_get_var("title")
assert_eq_string(var_value, "variable_value_title")

test_start("template_remove_var")
template_load_string("<h1>{{title}}</h1>")
template_set_var("title", "My Title")
sus remove_result lit = template_remove_var("title")
assert_true(remove_result)

// Test Template Rendering
test_start("template_render")
template_load("test.html")
template_set_var("title", "My Page")
template_set_var("header", "Welcome")
template_set_var("content", "Hello World")
sus rendered tea = template_render()
assert_true(rendered.contains("My Web Page"))
assert_true(rendered.contains("Welcome"))

test_start("template_render_to_file")
template_load("test.html")
template_set_var("title", "My Page")
sus file_result lit = template_render_to_file("output.html")
assert_true(file_result)

// Test Block Processing
test_start("template_process_blocks")
sus block_content tea = "<ul>{{#items}}<li>{{name}}</li>{{/items}}</ul>"
sus processed tea = template_process_blocks(block_content)
assert_true(processed.contains("Item 1"))
assert_true(processed.contains("Item 2"))

test_start("template_process_conditionals")
sus cond_content tea = "{{#if_user}}Hello User{{/if_user}}"
sus processed_cond tea = template_process_conditionals(cond_content)
assert_true(processed_cond != "")

test_start("template_process_loops")
sus loop_content tea = "{{#each}}{{name}}{{/each}}"
sus processed_loop tea = template_process_loops(loop_content)
assert_true(processed_loop != "")

// Test Template Validation
test_start("template_validate_valid")
template_load_string("<h1>{{title}}</h1>")
sus valid_result lit = template_validate()
assert_true(valid_result)

test_start("template_validate_invalid")
template_load_string("<h1>{{title</h1>")
sus invalid_result lit = template_validate()
assert_false(invalid_result)

test_start("template_find_variables")
template_load_string("<h1>{{title}}</h1><p>{{content}}</p>")
sus variables tea = template_find_variables()
assert_true(variables.contains("title"))
assert_true(variables.contains("content"))

test_start("template_find_blocks")
template_load_string("<ul>{{#items}}<li>{{name}}</li>{{/items}}</ul>")
sus blocks tea = template_find_blocks()
assert_true(blocks.contains("items"))

// Test HTML Functions
test_start("template_html_escape")
sus escaped tea = template_html_escape("<script>alert('test')</script>")
assert_true(escaped.contains("&lt;"))
assert_true(escaped.contains("&gt;"))
assert_true(escaped.contains("&#39;"))

test_start("template_html_unescape")
sus unescaped tea = template_html_unescape("&lt;script&gt;")
assert_true(unescaped.contains("<"))
assert_true(unescaped.contains(">"))

test_start("template_html_strip_tags")
sus stripped tea = template_html_strip_tags("<p>Hello <b>World</b></p>")
assert_true(stripped.contains("Hello"))
assert_true(stripped.contains("World"))

// Test Template Caching
test_start("template_enable_cache")
sus cache_result lit = template_enable_cache()
assert_true(cache_result)

test_start("template_disable_cache")
sus disable_result lit = template_disable_cache()
assert_true(disable_result)

test_start("template_clear_cache")
sus clear_result lit = template_clear_cache()
assert_true(clear_result)

test_start("template_get_cache_size")
sus cache_size normie = template_get_cache_size()
assert_eq_int(cache_size, 0)

// Test Template Inheritance
test_start("template_extend")
template_load_string("<h1>{{title}}</h1>")
sus extend_result lit = template_extend("base.html")
assert_true(extend_result)

test_start("template_include")
template_load_string("<h1>{{title}}</h1>")
sus include_result lit = template_include("header.html")
assert_true(include_result)

// Test Template Macros
test_start("template_define_macro")
template_load_string("<h1>{{title}}</h1>")
sus macro_result lit = template_define_macro("greeting", "<p>Hello {{name}}</p>")
assert_true(macro_result)

test_start("template_use_macro")
template_load_string("<h1>{{title}}</h1>")
sus use_result lit = template_use_macro("greeting")
assert_true(use_result)

// Test Template Filters
test_start("template_apply_filter_uppercase")
sus filtered tea = template_apply_filter("hello world", "uppercase")
assert_eq_string(filtered, "HELLO WORLD")

test_start("template_apply_filter_lowercase")
sus filtered2 tea = template_apply_filter("HELLO WORLD", "lowercase")
assert_eq_string(filtered2, "hello world")

test_start("template_apply_filter_capitalize")
sus filtered3 tea = template_apply_filter("hello world", "capitalize")
assert_true(filtered3.contains("Hello"))

test_start("template_apply_filter_reverse")
sus filtered4 tea = template_apply_filter("hello", "reverse")
assert_true(filtered4.contains("reversed_"))

// Test Template Statistics
test_start("template_get_stats")
template_load_string("<h1>{{title}}</h1>")
template_set_var("title", "Test")
sus stats tea = template_get_stats()
assert_true(stats.contains("variables:"))
assert_true(stats.contains("content_length:"))

test_start("template_get_render_time")
sus render_time normie = template_get_render_time()
assert_eq_int(render_time, 42)

// Test Template Compilation
test_start("template_compile")
template_load_string("<h1>{{title}}</h1>")
sus compile_result lit = template_compile()
assert_true(compile_result)

test_start("template_is_compiled")
template_load_string("<h1>{{title}}</h1>")
template_compile()
assert_true(template_is_compiled())

// Test Error Handling
test_start("operations_without_loaded_template")
template_clear()
assert_false(template_set_var("title", "Test"))
assert_false(template_remove_var("title"))
assert_eq_string(template_render(), "")
assert_false(template_render_to_file("output.html"))
assert_false(template_validate())
assert_eq_string(template_find_variables(), "")
assert_eq_string(template_find_blocks(), "")
assert_eq_string(template_get_stats(), "")
assert_false(template_compile())

// Test Complex Template
test_start("complex_template_render")
template_load_string("<!DOCTYPE html><html><head><title>{{title}}</title></head><body><h1>{{header}}</h1><p>{{content}}</p></body></html>")
template_set_var("title", "Complex Page")
template_set_var("header", "Main Header")
template_set_var("content", "Page content here")
sus complex_rendered tea = template_render()
assert_true(complex_rendered.contains("<!DOCTYPE html>"))
assert_true(complex_rendered.contains("<html>"))
assert_true(complex_rendered.contains("</html>"))

// Test Variable Replacement
test_start("variable_replacement")
template_load_string("Hello {{name}}, welcome to {{site}}!")
template_set_var("name", "John")
template_set_var("site", "MyWebsite")
sus replaced tea = template_render()
assert_true(replaced.contains("Hello"))
assert_true(replaced.contains("welcome"))

print_test_summary()
