/// Comprehensive template system tests for CURSED programming language
#[path = "common.rs"]
pub mod common;

use cursed::object::Object as CursedObject;
use cursed::stdlib::template::*;
use cursed::error::Error as CursedError;
use std::collections::HashMap;
use std::path::Path;
use std::fs;
use std::io::Write;
use tempfile::TempDir;
use common::init_tracing;

/// Test basic template parsing and rendering
#[test]
fn test_basic_template_rendering() {
    init_tracing!();
    
    let engine = TemplateEngine::new();
    let mut context = TemplateContext::new();
    context.set("name", CursedObject::String("Alice".to_string()));
    context.set("age", CursedObject::Integer(25));
    
    let template_source = "Hello {{ name }}, you are {{ age }} years old!";
    let result = engine.render_string(template_source, context).unwrap();
    
    assert_eq!(result, "Hello Alice, you are 25 years old!");
}

/// Test variable interpolation with filters
#[test]
fn test_variable_filters() {
    init_tracing!();
    
    let engine = TemplateEngine::new();
    let mut context = TemplateContext::new();
    context.set("text", CursedObject::String("hello world".to_string()));
    context.set("number", CursedObject::Float(3.14159));
    
    let template_source = "{{ text | upper }} and {{ number | numFormat 2 }}";
    let result = engine.render_string(template_source, context).unwrap();
    
    assert!(result.contains("HELLO WORLD"));
    assert!(result.contains("3.14"));
}

/// Test conditional blocks
#[test]
fn test_conditional_blocks() {
    init_tracing!();
    
    let engine = TemplateEngine::new();
    
    // Test truthy condition
    let mut context = TemplateContext::new();
    context.set("show_message", CursedObject::Boolean(true));
    context.set("message", CursedObject::String("Welcome!".to_string()));
    
    let template_source = "{% if show_message %}{{ message }}{% else %}No message{% end %}";
    let result = engine.render_string(template_source, context).unwrap();
    assert_eq!(result, "Welcome!");
    
    // Test falsy condition
    let mut context = TemplateContext::new();
    context.set("show_message", CursedObject::Boolean(false));
    
    let result = engine.render_string(template_source, context).unwrap();
    assert_eq!(result, "No message");
}

/// Test loop blocks
#[test]
fn test_loop_blocks() {
    init_tracing!();
    
    let engine = TemplateEngine::new();
    let mut context = TemplateContext::new();
    
    let items = vec![
        CursedObject::String("apple".to_string()),
        CursedObject::String("banana".to_string()),
        CursedObject::String("cherry".to_string()),
    ];
    context.set("fruits", CursedObject::Array(items));
    
    let template_source = "{% for fruit in fruits %}{{ fruit }}{% end %}";
    let result = engine.render_string(template_source, context).unwrap();
    
    assert!(result.contains("apple"));
    assert!(result.contains("banana"));
    assert!(result.contains("cherry"));
}

/// Test loop variables and helpers
#[test]
fn test_loop_variables() {
    init_tracing!();
    
    let engine = TemplateEngine::new();
    let mut context = TemplateContext::new();
    
    let numbers = vec![
        CursedObject::Integer(1),
        CursedObject::Integer(2),
        CursedObject::Integer(3),
    ];
    context.set("numbers", CursedObject::Array(numbers));
    
    let template_source = "{% for num in numbers %}{{ @index }}: {{ num }}{% if @last %} (last){% end %}{% end %}";
    let result = engine.render_string(template_source, context).unwrap();
    
    assert!(result.contains("0: 1"));
    assert!(result.contains("1: 2"));
    assert!(result.contains("2: 3 (last)"));
}

/// Test template includes
#[test]
fn test_template_includes() {
    init_tracing!();
    
    let temp_dir = TempDir::new().unwrap();
    let templates_path = temp_dir.path();
    
    // Create header template
    let header_path = templates_path.join("header.html");
    fs::write(&header_path, "<h1>{{ title }}</h1>").unwrap();
    
    // Create main template
    let main_path = templates_path.join("main.html");
    fs::write(&main_path, "{% include \"header.html\" %}<p>{{ content }}</p>").unwrap();
    
    let loader = Arc::new(FileSystemLoader::new(templates_path));
    let config = TemplateConfig::default();
    let engine = TemplateEngine::with_config_and_loader(config, loader);
    
    let mut context = TemplateContext::new();
    context.set("title", CursedObject::String("Welcome".to_string()));
    context.set("content", CursedObject::String("Hello, World!".to_string()));
    
    let result = engine.render("main.html", context).unwrap();
    
    assert!(result.contains("<h1>Welcome</h1>"));
    assert!(result.contains("<p>Hello, World!</p>"));
}

/// Test HTML escaping and safety
#[test]
fn test_html_escaping() {
    init_tracing!();
    
    let engine = TemplateEngine::new();
    let mut context = TemplateContext::new();
    context.set("unsafe_content", CursedObject::String("<script>alert('xss')</script>".to_string()));
    
    let template_source = "{{ unsafe_content }}";
    let result = engine.render_string(template_source, context).unwrap();
    
    // Should be escaped by default
    assert!(result.contains("&lt;script&gt;"));
    assert!(!result.contains("<script>"));
}

/// Test range loops
#[test]
fn test_range_loops() {
    init_tracing!();
    
    let engine = TemplateEngine::new();
    let context = TemplateContext::new();
    
    let template_source = "{% for i=1 to=5 %}{{ i }}{% end %}";
    let result = engine.render_string(template_source, context).unwrap();
    
    assert!(result.contains("1"));
    assert!(result.contains("2"));
    assert!(result.contains("3"));
    assert!(result.contains("4"));
    assert!(result.contains("5"));
}

/// Test filter registry and custom filters
#[test]
fn test_filter_registry() {
    init_tracing!();
    
    let registry = FilterRegistry::new();
    
    // Test built-in filters
    let result = registry.apply("upper", &[CursedObject::String("hello".to_string())]).unwrap();
    assert_eq!(result, CursedObject::String("HELLO".to_string()));
    
    let result = registry.apply("len", &[CursedObject::String("test".to_string())]).unwrap();
    assert_eq!(result, CursedObject::Integer(4));
    
    // Test math filters
    let result = registry.apply("add", &[CursedObject::Integer(5), CursedObject::Integer(3)]).unwrap();
    assert_eq!(result, CursedObject::Float(8.0));
}

/// Test template caching
#[test]
fn test_template_caching() {
    init_tracing!();
    
    let cache = TemplateCache::new(10);
    
    let ast = TemplateAst {
        nodes: vec![TemplateNode::Text("Hello World".to_string())],
    };
    
    let source_hash = CacheKeyGenerator::hash_source("test template");
    
    // Test cache put and get
    cache.put("test".to_string(), ast.clone(), source_hash).unwrap();
    let retrieved = cache.get("test");
    assert!(retrieved.is_some());
    
    // Test cache validation
    assert!(cache.validate_entry("test", source_hash));
    assert!(!cache.validate_entry("test", 12345));
    
    // Test cache statistics
    let (entries, max) = cache.stats();
    assert_eq!(entries, 1);
    assert_eq!(max, 10);
}

/// Test HTML template features
#[test]
fn test_html_template_features() {
    init_tracing!();
    
    let context = HtmlTemplateContext::new();
    let escaper = HtmlEscaper::new(context);
    
    // Test HTML escaping
    let content = "<script>alert('test')</script>";
    let escaped = escaper.escape(content, EscapeContext::Html).unwrap();
    assert!(escaped.contains("&lt;script&gt;"));
    
    // Test JavaScript escaping
    let js_content = "alert('hello\nworld')";
    let escaped_js = escaper.escape(js_content, EscapeContext::JavaScript).unwrap();
    assert!(escaped_js.contains("\\'"));
    assert!(escaped_js.contains("\\n"));
    
    // Test URL escaping
    let url_content = "hello world & more";
    let escaped_url = escaper.escape(url_content, EscapeContext::Url).unwrap();
    assert!(escaped_url.contains("%20"));
    assert!(escaped_url.contains("%26"));
}

/// Test template format rendering
#[test]
fn test_template_formats() {
    init_tracing!();
    
    let mut data = HashMap::new();
    data.insert("name".to_string(), CursedObject::String("Alice".to_string()));
    data.insert("age".to_string(), CursedObject::Integer(25));
    data.insert("active".to_string(), CursedObject::Boolean(true));
    let cursed_data = CursedObject::Map(data);
    
    // Test JSON format
    let json_renderer = TemplateFormatRenderer::new(TemplateFormat::Json);
    let json_result = json_renderer.render(&cursed_data).unwrap();
    assert!(json_result.contains("\"name\""));
    assert!(json_result.contains("\"Alice\""));
    assert!(json_result.contains("\"age\""));
    assert!(json_result.contains("25"));
    
    // Test HTML format
    let html_renderer = TemplateFormatRenderer::new(TemplateFormat::Html);
    let html_result = html_renderer.render(&cursed_data).unwrap();
    assert!(html_result.contains("<dl>"));
    assert!(html_result.contains("<dt>name</dt>"));
    assert!(html_result.contains("<dd>Alice</dd>"));
    
    // Test XML format
    let xml_renderer = TemplateFormatRenderer::new(TemplateFormat::Xml);
    let xml_result = xml_renderer.render(&cursed_data).unwrap();
    assert!(xml_result.contains("<?xml version"));
    assert!(xml_result.contains("<name>Alice</name>"));
    
    // Test YAML format
    let yaml_renderer = TemplateFormatRenderer::new(TemplateFormat::Yaml);
    let yaml_result = yaml_renderer.render(&cursed_data).unwrap();
    assert!(yaml_result.contains("name: Alice"));
    assert!(yaml_result.contains("age: 25"));
}

/// Test web template features
#[test]
fn test_web_template_features() {
    init_tracing!();
    
    let renderer = WebTemplateRenderer::new("templates");
    
    // Test CSRF token generation
    let request = WebTemplateRequest {
        method: "GET".to_string(),
        url: "/test".to_string(),
        headers: HashMap::new(),
        query: HashMap::new(),
        form: HashMap::new(),
        cookies: HashMap::new(),
        session: HashMap::new(),
        user: None,
    };
    
    let token = renderer.generate_csrf_token(&request).unwrap();
    assert!(!token.is_empty());
    assert!(renderer.verify_csrf_token(&token, &request).unwrap());
    
    // Test JSON response rendering
    let mut data = HashMap::new();
    data.insert("message".to_string(), CursedObject::String("Hello".to_string()));
    data.insert("status".to_string(), CursedObject::Integer(200));
    let cursed_data = CursedObject::Map(data);
    
    let response = renderer.render_json(&cursed_data).unwrap();
    assert_eq!(response.status, 200);
    assert_eq!(response.content_type, "application/json; charset=utf-8");
    assert!(response.body.contains("Hello"));
}

/// Test configuration file formats
#[test]
fn test_config_formats() {
    init_tracing!();
    
    let mut data = HashMap::new();
    data.insert("title".to_string(), CursedObject::String("My App".to_string()));
    data.insert("debug".to_string(), CursedObject::Boolean(true));
    data.insert("port".to_string(), CursedObject::Integer(8080));
    let cursed_data = CursedObject::Map(data);
    
    // Test TOML format
    let toml_renderer = TemplateFormatRenderer::new(
        TemplateFormat::Config(ConfigFormat::Toml)
    );
    let toml_result = toml_renderer.render(&cursed_data).unwrap();
    assert!(toml_result.contains("title = \"My App\""));
    assert!(toml_result.contains("debug = true"));
    assert!(toml_result.contains("port = 8080"));
    
    // Test INI format
    let ini_renderer = TemplateFormatRenderer::new(
        TemplateFormat::Config(ConfigFormat::Ini)
    );
    let ini_result = ini_renderer.render(&cursed_data).unwrap();
    assert!(ini_result.contains("title = My App"));
    assert!(ini_result.contains("debug = true"));
    
    // Test Environment format
    let env_renderer = TemplateFormatRenderer::new(
        TemplateFormat::Config(ConfigFormat::Env)
    );
    let env_result = env_renderer.render(&cursed_data).unwrap();
    assert!(env_result.contains("TITLE=My App"));
    assert!(env_result.contains("DEBUG=true"));
    assert!(env_result.contains("PORT=8080"));
}

/// Test CSV format rendering
#[test]
fn test_csv_format() {
    init_tracing!();
    
    // Test array of maps (typical CSV data)
    let mut row1 = HashMap::new();
    row1.insert("name".to_string(), CursedObject::String("Alice".to_string()));
    row1.insert("age".to_string(), CursedObject::Integer(25));
    row1.insert("city".to_string(), CursedObject::String("New York".to_string()));
    
    let mut row2 = HashMap::new();
    row2.insert("name".to_string(), CursedObject::String("Bob".to_string()));
    row2.insert("age".to_string(), CursedObject::Integer(30));
    row2.insert("city".to_string(), CursedObject::String("London".to_string()));
    
    let data = CursedObject::Array(vec![
        CursedObject::Map(row1),
        CursedObject::Map(row2),
    ]);
    
    let csv_renderer = TemplateFormatRenderer::new(TemplateFormat::Csv);
    let csv_result = csv_renderer.render(&data).unwrap();
    
    // Should have header and data rows
    assert!(csv_result.contains("age,city,name")); // Headers (sorted)
    assert!(csv_result.contains("25,New York,Alice"));
    assert!(csv_result.contains("30,London,Bob"));
}

/// Test email template format
#[test]
fn test_email_format() {
    init_tracing!();
    
    let mut email_data = HashMap::new();
    email_data.insert("from".to_string(), CursedObject::String("sender@example.com".to_string()));
    email_data.insert("to".to_string(), CursedObject::String("recipient@example.com".to_string()));
    email_data.insert("subject".to_string(), CursedObject::String("Test Email".to_string()));
    email_data.insert("text".to_string(), CursedObject::String("Hello from text!".to_string()));
    email_data.insert("html".to_string(), CursedObject::String("<h1>Hello from HTML!</h1>".to_string()));
    
    let data = CursedObject::Map(email_data);
    
    let email_renderer = TemplateFormatRenderer::new(TemplateFormat::Email);
    let email_result = email_renderer.render(&data).unwrap();
    
    assert!(email_result.contains("From: sender@example.com"));
    assert!(email_result.contains("To: recipient@example.com"));
    assert!(email_result.contains("Subject: Test Email"));
    assert!(email_result.contains("Hello from text!"));
    assert!(email_result.contains("<h1>Hello from HTML!</h1>"));
    assert!(email_result.contains("Content-Type: text/plain"));
    assert!(email_result.contains("Content-Type: text/html"));
}

/// Test template error handling
#[test]
fn test_template_error_handling() {
    init_tracing!();
    
    let engine = TemplateEngine::new();
    let context = TemplateContext::new();
    
    // Test undefined variable in strict mode
    let mut strict_config = TemplateConfig::default();
    strict_config.strict_mode = true;
    
    // Test malformed template syntax
    let malformed_template = "{{ unclosed_var";
    let result = engine.render_string(malformed_template, context.clone());
    assert!(result.is_err());
    
    // Test invalid filter
    let invalid_filter_template = "{{ name | nonexistent_filter }}";
    let result = engine.render_string(invalid_filter_template, context);
    assert!(result.is_err());
}

/// Test complex nested templates
#[test]
fn test_complex_nested_templates() {
    init_tracing!();
    
    let engine = TemplateEngine::new();
    let mut context = TemplateContext::new();
    
    // Complex nested data structure
    let mut user = HashMap::new();
    user.insert("name".to_string(), CursedObject::String("Alice".to_string()));
    user.insert("age".to_string(), CursedObject::Integer(25));
    
    let mut address = HashMap::new();
    address.insert("street".to_string(), CursedObject::String("123 Main St".to_string()));
    address.insert("city".to_string(), CursedObject::String("New York".to_string()));
    user.insert("address".to_string(), CursedObject::Map(address));
    
    let hobbies = vec![
        CursedObject::String("reading".to_string()),
        CursedObject::String("coding".to_string()),
        CursedObject::String("gaming".to_string()),
    ];
    user.insert("hobbies".to_string(), CursedObject::Array(hobbies));
    
    context.set("user", CursedObject::Map(user));
    context.set("show_hobbies", CursedObject::Boolean(true));
    
    let complex_template = r#"
Name: {{ user.name }}
Age: {{ user.age }}
Address: {{ user.address.street }}, {{ user.address.city }}
{% if show_hobbies %}
Hobbies:
{% for hobby in user.hobbies %}
- {{ hobby | title }}
{% end %}
{% end %}
"#;
    
    let result = engine.render_string(complex_template, context).unwrap();
    
    assert!(result.contains("Name: Alice"));
    assert!(result.contains("Age: 25"));
    assert!(result.contains("Address: 123 Main St, New York"));
    assert!(result.contains("Hobbies:"));
    assert!(result.contains("- Reading"));
    assert!(result.contains("- Coding"));
    assert!(result.contains("- Gaming"));
}

/// Test performance with large templates
#[test]
fn test_template_performance() {
    init_tracing!();
    
    let engine = TemplateEngine::new();
    let mut context = TemplateContext::new();
    
    // Create large data set
    let mut items = Vec::new();
    for i in 0..1000 {
        let mut item = HashMap::new();
        item.insert("id".to_string(), CursedObject::Integer(i));
        item.insert("name".to_string(), CursedObject::String(format!("Item {}", i)));
        item.insert("active".to_string(), CursedObject::Boolean(i % 2 == 0));
        items.push(CursedObject::Map(item));
    }
    context.set("items", CursedObject::Array(items));
    
    let large_template = r#"
<ul>
{% for item in items %}
<li>{{ item.id }}: {{ item.name }}{% if item.active %} (active){% end %}</li>
{% end %}
</ul>
"#;
    
    let start_time = std::time::Instant::now();
    let result = engine.render_string(large_template, context).unwrap();
    let duration = start_time.elapsed();
    
    // Should render efficiently (under 1 second for 1000 items)
    assert!(duration.as_secs() < 1);
    assert!(result.contains("<ul>"));
    assert!(result.contains("</ul>"));
    assert!(result.contains("Item 999"));
}

use std::sync::Arc;
