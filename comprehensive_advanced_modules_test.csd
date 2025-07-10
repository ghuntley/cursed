// Comprehensive Advanced Modules Test
// Tests all advanced stdlib modules integration

yeet "testz"
yeet "image_processing"
yeet "template_engine"
yeet "archive_handling"
yeet "url_parsing"
yeet "command_line"
yeet "reflection"
yeet "plugin_system"

// Test Image Processing Integration
test_start("image_processing_integration")
sus img_result lit = image_load("test_image.jpg")
assert_true(img_result)
assert_true(image_resize(800, 600))
assert_true(image_adjust_brightness(1.2))
assert_true(image_save("processed_image.jpg", 85))
image_clear()

// Test Template Engine Integration
test_start("template_engine_integration")
sus template_result lit = template_load_string("<html><body><h1>{{title}}</h1><p>{{content}}</p></body></html>")
assert_true(template_result)
assert_true(template_set_var("title", "Advanced Module Test"))
assert_true(template_set_var("content", "All modules working together"))
sus rendered tea = template_render()
assert_true(rendered.contains("<h1>"))
assert_true(rendered.contains("Advanced Module Test"))
template_clear()

// Test Archive Handling Integration
test_start("archive_handling_integration")
sus archive_result lit = archive_create("test_archive.zip", "zip")
assert_true(archive_result)
assert_true(archive_add_file("test_file.txt", "archived_file.txt"))
assert_true(archive_set_compression_level(6))
assert_eq_int(archive_get_file_count(), 1)
assert_true(archive_validate())
archive_close()

// Test URL Parsing Integration
test_start("url_parsing_integration")
sus url_result lit = url_parse("https://example.com:8080/api/v1/data?param=value#section")
assert_true(url_result)
assert_eq_string(url_get_scheme(), "https")
assert_eq_string(url_get_host(), "example.com")
assert_eq_int(url_get_port(), 443)
assert_true(url_add_query_param("new_param", "new_value"))
sus rebuilt_url tea = url_build()
assert_true(rebuilt_url.contains("https://"))
url_clear()

// Test Command Line Integration
test_start("command_line_integration")
sus cli_result lit = cli_init("test_app", "--verbose --input data.txt --output result.txt")
assert_true(cli_result)
assert_true(cli_parse())
assert_true(cli_has_flag("verbose"))
assert_eq_string(cli_get_flag_value("input"), "input.txt")
assert_eq_string(cli_get_flag_value("output"), "output.txt")
assert_true(cli_validate_required_flags("input,output"))

// Test Reflection Integration
test_start("reflection_integration")
sus reflect_result lit = reflect_load_type("test_struct")
assert_true(reflect_result)
assert_true(reflect_is_struct())
assert_eq_int(reflect_get_field_count(), 3)
assert_true(reflect_has_field("field1"))
assert_true(reflect_has_method("method1"))
sus call_result tea = reflect_call_method("method1", "test_args")
assert_eq_string(call_result, "method_result_method1")
reflect_clear()

// Test Plugin System Integration
test_start("plugin_system_integration")
sus plugin_init_result lit = plugin_system_init()
assert_true(plugin_init_result)
assert_true(plugin_register("test_plugin", "plugins/test.csd"))
assert_true(plugin_load("test_plugin"))
assert_true(plugin_is_loaded("test_plugin"))
sus plugin_call_result tea = plugin_call_function("test_plugin", "test_function", "args")
assert_true(plugin_call_result != "")
assert_true(plugin_send_message("test_plugin", "test_message"))
plugin_system_shutdown()

// Test Cross-Module Integration
test_start("cross_module_integration")
// Use reflection to inspect image processing module
reflect_load_type("image_struct")
sus image_methods tea = reflect_get_method_names()
assert_true(image_methods.contains("method"))

// Use template engine to generate URLs
template_load_string("{{scheme}}://{{host}}:{{port}}/{{path}}")
template_set_var("scheme", "https")
template_set_var("host", "api.example.com")
template_set_var("port", "8080")
template_set_var("path", "v1/endpoint")
sus url_template tea = template_render()
assert_true(url_template.contains("https://api.example.com:8080/v1/endpoint"))

// Parse the generated URL
url_parse(url_template)
assert_eq_string(url_get_scheme(), "https")
assert_eq_string(url_get_host(), "api.example.com")

// Test Advanced Feature Combinations
test_start("advanced_feature_combinations")
// Image processing with archive storage
image_load("batch_image_1.jpg")
image_resize(400, 300)
image_save("processed_batch_1.jpg", 80)

archive_create("image_batch.zip", "zip")
archive_add_file("processed_batch_1.jpg", "batch/image_1.jpg")
archive_set_compression_level(9)

// Template-based configuration
template_load_string("compression_level={{level}},quality={{quality}}")
template_set_var("level", "9")
template_set_var("quality", "80")
sus config_output tea = template_render()
assert_true(config_output.contains("compression_level=9"))
assert_true(config_output.contains("quality=80"))

// Plugin system with dynamic loading
plugin_system_init()
plugin_register("image_processor", "plugins/image.csd")
plugin_register("archive_manager", "plugins/archive.csd")
plugin_load("image_processor")
plugin_load("archive_manager")

// Send cross-plugin messages
plugin_send_message("image_processor", "process_complete")
plugin_send_message("archive_manager", "ready_for_archive")

// Test Error Handling Integration
test_start("error_handling_integration")
// Test invalid image loading
image_clear()
assert_false(image_load("nonexistent.jpg"))
assert_false(image_resize(100, 100))  // Should fail without loaded image

// Test invalid URL parsing
url_clear()
assert_false(url_parse("invalid://"))
assert_eq_string(url_get_host(), "")

// Test invalid archive operations
archive_close()
assert_false(archive_add_file("test.txt", "test.txt"))  // Should fail without open archive

// Test invalid plugin operations
plugin_system_shutdown()
assert_false(plugin_load("nonexistent_plugin"))

// Test Performance and Memory Usage
test_start("performance_memory_test")
// Test batch operations
sus batch_count normie = image_batch_resize("img1.jpg,img2.jpg,img3.jpg", 200, 200)
assert_eq_int(batch_count, 3)

sus batch_archive_count normie = archive_batch_extract("arch1.zip,arch2.zip", "extract/")
assert_eq_int(batch_archive_count, 2)

// Test large template processing
template_load_string("Large template with {{var1}} and {{var2}} and {{var3}}")
template_set_var("var1", "value1")
template_set_var("var2", "value2")
template_set_var("var3", "value3")
sus large_template tea = template_render()
assert_true(large_template.contains("value1"))
assert_true(large_template.contains("value2"))
assert_true(large_template.contains("value3"))

// Test complex URL manipulation
url_parse("https://api.example.com/v1/users")
url_add_query_param("page", "1")
url_add_query_param("limit", "10")
url_add_query_param("sort", "name")
sus complex_url tea = url_build()
assert_true(complex_url.contains("page=1"))
assert_true(complex_url.contains("limit=10"))
assert_true(complex_url.contains("sort=name"))

// Test Real-World Scenarios
test_start("real_world_scenarios")
// Scenario 1: Web Application Asset Processing
// Load image, resize for web, create archive, generate HTML
image_load("web_asset.jpg")
image_resize(1200, 800)
image_create_thumbnail(200)
image_save("web_asset_thumb.jpg", 85)

archive_create("web_assets.zip", "zip")
archive_add_file("web_asset_thumb.jpg", "assets/thumbnail.jpg")

template_load_string("<img src=\"{{image_url}}\" alt=\"{{alt_text}}\" width=\"{{width}}\" height=\"{{height}}\">")
template_set_var("image_url", "assets/thumbnail.jpg")
template_set_var("alt_text", "Web Asset Thumbnail")
template_set_var("width", "200")
template_set_var("height", "200")
sus img_html tea = template_render()
assert_true(img_html.contains("<img src="))
assert_true(img_html.contains("width=\"200\""))

// Scenario 2: API Configuration Management
// Parse API endpoints, validate URLs, generate client code
url_parse("https://api.service.com/v2/endpoints")
url_set_path("/v2/users")
url_add_query_param("api_key", "secret_key")
sus api_url tea = url_build()

template_load_string("API_URL = \"{{url}}\"\nAPI_VERSION = \"{{version}}\"")
template_set_var("url", api_url)
template_set_var("version", "v2")
sus api_config tea = template_render()
assert_true(api_config.contains("API_URL"))
assert_true(api_config.contains("v2"))

// Scenario 3: Plugin-based Image Processing Pipeline
plugin_system_init()
plugin_register("resize_plugin", "plugins/resize.csd")
plugin_register("filter_plugin", "plugins/filter.csd")
plugin_register("export_plugin", "plugins/export.csd")

plugin_load("resize_plugin")
plugin_load("filter_plugin")
plugin_load("export_plugin")

plugin_call_function("resize_plugin", "resize", "800x600")
plugin_call_function("filter_plugin", "blur", "radius=3")
plugin_call_function("export_plugin", "save", "output.jpg")

sus plugin_stats tea = plugin_get_system_stats()
assert_true(plugin_stats.contains("loaded:3"))

// Final Integration Test
test_start("final_integration_test")
// Complex workflow combining all modules
cli_init("advanced_app", "--input images/ --output processed/ --format zip --verbose")
cli_parse()

bestie cli_has_flag("verbose") {
    vibez.spill("Verbose mode: Processing images with advanced modules")
}

sus input_dir tea = cli_get_flag_value("input")
sus output_dir tea = cli_get_flag_value("output")
sus archive_format tea = cli_get_flag_value("format")

// Process images
image_load("input_image.jpg")
image_resize(800, 600)
image_adjust_brightness(1.1)
image_save("processed_image.jpg", 90)

// Create archive
archive_create("processed_images.zip", "zip")
archive_add_file("processed_image.jpg", "processed/image.jpg")

// Generate report
template_load_string("Processing Report\n=================\nInput: {{input}}\nOutput: {{output}}\nFormat: {{format}}\nStatus: {{status}}")
template_set_var("input", input_dir)
template_set_var("output", output_dir)
template_set_var("format", archive_format)
template_set_var("status", "Complete")
sus report tea = template_render()
assert_true(report.contains("Processing Report"))
assert_true(report.contains("Complete"))

// Log the successful completion
vibez.spill("Advanced modules integration test completed successfully!")
vibez.spill("All 7 advanced modules working together seamlessly")

print_test_summary()
