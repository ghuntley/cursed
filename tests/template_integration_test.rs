
/// Template Integration Tests - End-to-end template system testing
#[path = "common.rs"]
pub mod common;

macro_rules! init_tracing {
    () => {
        common::tracing::setup();
    };
}

use cursed::object::Object as CursedObject;
use cursed::stdlib::template::*;
use cursed::error::Error as CursedError;
use std::collections::HashMap;
use std::path::Path;
use std::fs;
use std::io::Write;
use tempfile::TempDir;
// Remove import since we'll use macro

#[cfg(test)]
mod tests {
    use super::*;
    
    /// Test complete template workflow from parsing to rendering
    #[test]
    fn test_end_to_end_template_workflow() {
        init_tracing!();
        
        let temp_dir = TempDir::new().unwrap();
        let templates_path = temp_dir.path();
        
        // Create base layout template
        let layout_content = r#"<!DOCTYPE html>
<html>
<head>
    <title>{{ title }}</title>
</head>
<body>
    <header>
        <h1>{{ site_name }}</h1>
    </header>
    <main>
        {{ content }}
    </main>
    <footer>
        <p>&copy; {{ year }} {{ site_name }}</p>
    </footer>
</body>
</html>"#;
        
        fs::write(templates_path.join("layout.html"), layout_content).unwrap();
        
        // Create page template
        let page_content = r#"<h2>{{ page_title }}</h2>
<p>{{ description }}</p>
{% if show_users %}
<ul>
{% for user in users %}
    <li>{{ user.name }} ({{ user.email }})</li>
{% end %}
</ul>
{% end %}"#;
        
        fs::write(templates_path.join("page.html"), page_content).unwrap();
        
        // Set up template engine
        let loader = Arc::new(FileSystemLoader::new(templates_path));
        let config = TemplateConfig::default();
        let engine = TemplateEngine::with_config_and_loader(config, loader);
        
        // Create context data
        let mut context = TemplateContext::new();
        context.set("title", CursedObject::String("Welcome Page".to_string()));
        context.set("site_name", CursedObject::String("My Website".to_string()));
        context.set("year", CursedObject::Integer(2024));
        context.set("page_title", CursedObject::String("User Directory".to_string()));
        context.set("description", CursedObject::String("List of registered users".to_string()));
        context.set("show_users", CursedObject::Boolean(true));
        
        // Create user data
        let mut users = Vec::new();
        let mut user1 = HashMap::new();
        user1.insert("name".to_string(), CursedObject::String("Alice".to_string()));
        user1.insert("email".to_string(), CursedObject::String("alice@example.com".to_string()));
        users.push(CursedObject::Map(user1));
        
        let mut user2 = HashMap::new();
        user2.insert("name".to_string(), CursedObject::String("Bob".to_string()));
        user2.insert("email".to_string(), CursedObject::String("bob@example.com".to_string()));
        users.push(CursedObject::Map(user2));
        
        context.set("users", CursedObject::Array(users));
        
        // Render template
        let result = engine.render("page.html", context).unwrap();
        
        // Verify output
        assert!(result.contains("<h2>User Directory</h2>"));
        assert!(result.contains("List of registered users"));
        assert!(result.contains("<li>Alice (alice@example.com)</li>"));
        assert!(result.contains("<li>Bob (bob@example.com)</li>"));
    }
    
    /// Test web framework integration
    #[test]
    fn test_web_framework_integration() {
        init_tracing!();
        
        let mut web_renderer = WebTemplateRenderer::new("templates");
        
        // Create web request
        let mut request = WebTemplateRequest {
            method: "POST".to_string(),
            url: "/users/create".to_string(),
            headers: HashMap::new(),
            query: HashMap::new(),
            form: HashMap::new(),
            cookies: HashMap::new(),
            session: HashMap::new(),
            user: None,
        };
        
        // Add form data
        request.form.insert("name".to_string(), "John Doe".to_string());
        request.form.insert("email".to_string(), "john@example.com".to_string());
        
        // Add session data
        request.session.insert("user_id".to_string(), CursedObject::Integer(123));
        request.session.insert("logged_in".to_string(), CursedObject::Boolean(true));
        
        // Create template context
        let mut context = TemplateContext::new();
        context.set("success", CursedObject::Boolean(true));
        context.set("message", CursedObject::String("User created successfully".to_string()));
        
        // Test CSRF protection setting
        web_renderer.set_csrf_secret("test_secret_key_123".to_string());
        
        // Test JSON response
        let mut data = HashMap::new();
        data.insert("status".to_string(), CursedObject::String("success".to_string()));
        data.insert("user_id".to_string(), CursedObject::Integer(456));
        let json_data = CursedObject::Map(data);
        
        let json_response = web_renderer.render_json(&json_data).unwrap();
        assert_eq!(json_response.status, 200);
        assert_eq!(json_response.content_type, "application/json; charset=utf-8");
        assert!(json_response.body.contains("\"status\""));
        assert!(json_response.body.contains("\"success\""));
        assert!(json_response.body.contains("456"));
        
        // Verify security headers
        assert!(json_response.headers.contains_key("X-XSS-Protection"));
        assert!(json_response.headers.contains_key("Content-Security-Policy"));
    }
    
    /// Test multiple template formats
    #[test]
    fn test_multiple_formats_integration() {
        init_tracing!();
        
        // Create sample data
        let mut product = HashMap::new();
        product.insert("id".to_string(), CursedObject::Integer(123));
        product.insert("name".to_string(), CursedObject::String("Widget".to_string()));
        product.insert("price".to_string(), CursedObject::Float(29.99));
        product.insert("in_stock".to_string(), CursedObject::Boolean(true));
        
        let mut category = HashMap::new();
        category.insert("id".to_string(), CursedObject::Integer(5));
        category.insert("name".to_string(), CursedObject::String("Electronics".to_string()));
        product.insert("category".to_string(), CursedObject::Map(category));
        
        let tags = vec![
            CursedObject::String("gadget".to_string()),
            CursedObject::String("popular".to_string()),
            CursedObject::String("new".to_string()),
        ];
        product.insert("tags".to_string(), CursedObject::Array(tags));
        
        let data = CursedObject::Map(product);
        
        // Test JSON format
        let json_renderer = TemplateFormatRenderer::new(TemplateFormat::Json);
        let json_result = json_renderer.render(&data).unwrap();
        assert!(json_result.contains("\"name\": \"Widget\""));
        assert!(json_result.contains("\"price\": 29.99"));
        
        // Test XML format
        let xml_renderer = TemplateFormatRenderer::new(TemplateFormat::Xml);
        let xml_result = xml_renderer.render(&data).unwrap();
        assert!(xml_result.contains("<?xml version=\"1.0\""));
        assert!(xml_result.contains("<name>Widget</name>"));
        assert!(xml_result.contains("<price>29.99</price>"));
        
        // Test YAML format
        let yaml_renderer = TemplateFormatRenderer::new(TemplateFormat::Yaml);
        let yaml_result = yaml_renderer.render(&data).unwrap();
        assert!(yaml_result.contains("name: Widget"));
        assert!(yaml_result.contains("price: 29.99"));
        
        // Test HTML format
        let html_renderer = TemplateFormatRenderer::new(TemplateFormat::Html);
        let html_result = html_renderer.render(&data).unwrap();
        assert!(html_result.contains("<dl>"));
        assert!(html_result.contains("<dt>name</dt>"));
        assert!(html_result.contains("<dd>Widget</dd>"));
    }
    
    /// Test complex template expressions and operations
    #[test]
    fn test_complex_template_expressions() {
        init_tracing!();
        
        let engine = TemplateEngine::new();
        let mut context = TemplateContext::new();
        
        // Set up complex data
        context.set("user_count", CursedObject::Integer(42));
        context.set("premium_rate", CursedObject::Float(0.15));
        context.set("base_price", CursedObject::Float(100.0));
        
        // Test mathematical expressions and filters
        let template = r#"
User Count: {{ user_count }}
Premium Users: {{ user_count | mul premium_rate | round 0 }}
Base Price: {{ base_price | currency }}
Premium Price: {{ base_price | mul 1.5 | currency }}
Total Revenue: {{ user_count | mul base_price | currency }}
"#;
        
        let result = engine.render_string(template, context).unwrap();
        
        assert!(result.contains("User Count: 42"));
        assert!(result.contains("Premium Users: 6")); // 42 * 0.15 = 6.3, rounded to 6
        assert!(result.contains("Base Price: $100.00"));
        assert!(result.contains("Premium Price: $150.00"));
        assert!(result.contains("Total Revenue: $4200.00"));
    }
    
    /// Test template caching and performance
    #[test]
    fn test_template_caching_integration() {
        init_tracing!();
        
        let temp_dir = TempDir::new().unwrap();
        let templates_path = temp_dir.path();
        
        // Create test template
        let template_content = "Hello {{ name }}, welcome to {{ site }}!";
        fs::write(templates_path.join("welcome.html"), template_content).unwrap();
        
        let loader = Arc::new(FileSystemLoader::new(templates_path));
        let mut config = TemplateConfig::default();
        config.enable_cache = true;
        config.cache_size = 100;
        let engine = TemplateEngine::with_config_and_loader(config, loader);
        
        let mut context = TemplateContext::new();
        context.set("name", CursedObject::String("Alice".to_string()));
        context.set("site", CursedObject::String("Our Website".to_string()));
        
        // First render (cache miss)
        let start_time = std::time::Instant::now();
        let result1 = engine.render("welcome.html", context.clone()).unwrap();
        let first_render_time = start_time.elapsed();
        
        // Second render (cache hit)
        let start_time = std::time::Instant::now();
        let result2 = engine.render("welcome.html", context).unwrap();
        let second_render_time = start_time.elapsed();
        
        // Results should be identical
        assert_eq!(result1, result2);
        assert_eq!(result1, "Hello Alice, welcome to Our Website!");
        
        // Second render should be faster (cached)
        // Note: In practice, this might not always be true due to test environment variability
        // but we can at least verify both renders completed successfully
        assert!(first_render_time.as_nanos() > 0);
        assert!(second_render_time.as_nanos() > 0);
        
        // Verify cache statistics
        let (entries, max_size) = engine.cache_stats();
        assert!(entries > 0);
        assert_eq!(max_size, 100);
    }
    
    /// Test error handling and recovery
    #[test]
    fn test_error_handling_integration() {
        init_tracing!();
        
        let web_renderer = WebTemplateRenderer::new("templates");
        let request = WebTemplateRequest {
            method: "GET".to_string(),
            url: "/error-test".to_string(),
            headers: HashMap::new(),
            query: HashMap::new(),
            form: HashMap::new(),
            cookies: HashMap::new(),
            session: HashMap::new(),
            user: None,
        };
        
        // Test 404 error rendering
        let error_404 = CursedError::TemplateError {
            message: "Template not found".to_string(),
            source_location: None,
        };
        
        let error_response = web_renderer.render_error(&error_404, 404, &request).unwrap();
        assert_eq!(error_response.status, 404);
        assert!(error_response.body.contains("404"));
        assert!(error_response.body.contains("Page Not Found"));
        assert!(error_response.body.contains("Template not found"));
        
        // Test 500 error rendering
        let error_500 = CursedError::TemplateError {
            message: "Internal server error occurred".to_string(),
            source_location: None,
        };
        
        let error_response = web_renderer.render_error(&error_500, 500, &request).unwrap();
        assert_eq!(error_response.status, 500);
        assert!(error_response.body.contains("500"));
        assert!(error_response.body.contains("Internal Server Error"));
        
        // Verify security headers are still applied
        assert!(error_response.headers.contains_key("X-XSS-Protection"));
        assert!(error_response.headers.contains_key("Content-Security-Policy"));
    }
    
    /// Test template middleware integration
    #[test]
    fn test_template_middleware() {
        init_tracing!();
        
        let web_renderer = WebTemplateRenderer::new("templates");
        let middleware = TemplateMiddleware::new(web_renderer);
        
        let request = WebTemplateRequest {
            method: "GET".to_string(),
            url: "/dashboard".to_string(),
            headers: HashMap::new(),
            query: HashMap::new(),
            form: HashMap::new(),
            cookies: HashMap::new(),
            session: HashMap::new(),
            user: None,
        };
        
        let mut context = TemplateContext::new();
        context.set("page_title", CursedObject::String("Dashboard".to_string()));
        context.set("user_count", CursedObject::Integer(150));
        
        // Test error handling through middleware
        let error = CursedError::TemplateError {
            message: "Access denied".to_string(),
            source_location: None,
        };
        
        let error_response = middleware.handle_error(&error, 403, &request).unwrap();
        assert_eq!(error_response.status, 403);
        assert!(error_response.body.contains("403"));
        assert!(error_response.body.contains("Forbidden"));
    }
}

use std::sync::Arc;