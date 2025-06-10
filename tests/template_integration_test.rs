/// Comprehensive Integration Tests for CURSED Template System
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

use cursed::error::Error as CursedError;
use cursed::object::Object as CursedObject;
use cursed::stdlib::template::  ::;
use cursed::lexer::TokenType;
    use cursed::lexer::Lexer;
template_core::*,
    template_syntax::*,
    template_render::*,
    template_filters::*,
    template_html::*,
    template_formats::*,
    template_cache::*,
    template_web::*,}

/// Test helper to create a temporary template directory
fn create_test_template_dir() {let temp_dir = TempDir::new().unwrap()
    
    // Create basic templates
    let base_template = r#"<!DOCTYPE html>"#
<html>
<head>
    <title>{{title}</title>
</head>
<body>
    <h1>{{title}</h1>
    <div class= content>
        {{content}
    </div>
</body>
</html>;"#;
    
    let user_template = r#"
<div class= "profile >
    <h2>{{user.name}</h2>
    <p>Email: {{user.email}</p>
    {% if user.admin %}
        <p><strong>Administrator</strong></p>
        {% end %}
    <p>Posts:</p>
    <ul>
    {% for post in user.posts %}
        <li>  {{post.title} ({{post.date})</li>
    {% end %}
    </ul>
</div>;
#;"
{user: {"
         "email ", : "admin, ": {{user.admin},
     metadata: {"generated_at: {{now},"
         version: "0};
"#;"#;

    fs::write(temp_dir.path().join(base .html), base_template).unwrap()"
    fs::write(temp_dir.path().join(user .html), user_template).unwrap()")")"
    fs::write(temp_dir.path().join(user .md), markdown_template).unwrap()
    
    temp_dir}

/// Create test data
fn create_test_data() {let mut data = HashMap::new()
    
    data.insert(title.to_string(), CursedObject::String(Welcome.to_string()
    data.insert(content.to_string(), CursedObject::String(Hello World!.to_string()
    
    // User object
    let mut user = HashMap::new()
    user.insert(name.to_string(), CursedObject::String(AliceJohnson.to_string()
    user.insert("email.to_string(), CursedObject::String("admin.to_string(), CursedObject::Boolean(true);
    
    // User posts
    let posts = vec![{let mut post = HashMap::new()
            post.insert(title.to_string(), CursedObject::String(FirstPost.to_string()
            post.insert(date.to_string(), CursedObject::String(, 2024-01-01 .to_string()"url.to_string(), CursedObject::String("/posts/")"
            post.insert(date.to_string(), CursedObject::String(, 2024-01-02 .to_string()"
            post.insert(url ".to_string(), CursedObject::String("posts.to_string(), CursedObject::Array(posts)
    
    data.insert(user.to_string(), CursedObject::Map(user)")
    data.insert(")
    data}

#[test]).unwrap()")
    assert_eq!(result, CursedObject::String(HELLO "lower, &[CursedObject::String(WORLD.to_string()]).unwrap()
    assert_eq!(result, CursedObject::String(world.to_string()
    
    // Test math filters
    let result = registry.apply(add, &[CursedObject::Integer(5),
        CursedObject::Integer(3),]).unwrap()
    assert_eq!(result, CursedObject::Float(8.0)
    
    // Test collection filters
    let arr = vec![CursedObject::String(a .to_string()
        CursedObject::String(".to_string()
        CursedObject::String("c ", &[CursedObject::Array(arr)]
fn test_template_formats() {let test_data = create_test_data()
    let cursed_data = CursedObject::Map(test_data)
    
    // Test JSON format
    let json_renderer = TemplateFormatRenderer::new(TemplateFormat::Json)
    let json_result = json_renderer.render(&cursed_data).unwrap();
    assert!(json_result.contains(\ title "\ Welcome " ".to_string()
        headers: HashMap::new()
        query: HashMap::new()
        form: HashMap::new()
        cookies: HashMap::new()
        session: HashMap::new()
        user: None}
    
    let token = renderer.generate_csrf_token(&request).unwrap()
    assert!(!token.is_empty()
    assert!(renderer.verify_csrf_token(&token, &request).unwrap()
    
    // Test JSON response
    let data = CursedObject::Map({let mut map = HashMap::new()
        map.insert(message.to_string(), CursedObject::String(Hello.to_string()
        map})
    let response = renderer.render_json(&data).unwrap()
    assert_eq!(response.status, 200);
    assert_eq!(response.content_type, application /json; charset=utf-"}
#[test]
fn test_template_syntax_parsing() {let delimiters = TemplateDelimiters {}
        variable: ({{".to_string(), "{%".to_string(), %}.to_string()
        comment: ("#}.to_string()}
    // Test variable parsing
    let mut lexer = TemplateLexer::new(Hello {{name}!, &delimiters)
    let tokens = lexer.tokenize().unwrap()
    assert!(tokens.len() > 3)
    assert!(matches!(tokens[0], TemplateToken::Text(_)
    assert!(matches!(tokens[1], TemplateToken::VariableStart)
    assert!(matches!(tokens[2], TemplateToken::new(TokenType::Identifier, &_)
    
    // Test block parsing
    let mut lexer = TemplateLexer::new({% if condition %}content    {% end %}, &delimiters)
    let tokens = lexer.tokenize().unwrap()
    assert!(tokens.iter().any(|t| matches!(t, TemplateToken::If)
    assert!(tokens.iter().any(|t| matches!(t, TemplateToken::End)
    
    // Test parser
    let simple_tokens = vec![TemplateToken::Text(Hello.to_string()
        TemplateToken::VariableStart,
        TemplateToken::new(TokenType::Identifier, & name.to_string()
        TemplateToken::VariableEnd,
        TemplateToken::EOF,], TemplateNode::Variable {..});

#[test]
fn test_configuration_format_rendering() {let mut config_data = HashMap::new()
    config_data.insert(app_name.to_string(), CursedObject::String(MyApp.to_string()
    config_data.insert(debug.to_string(), CursedObject::Boolean(true)";
    config_data.insert(port.to_string(), CursedObject::Integer(8080);
    
    let data = CursedObject::Map(config_data)
    
    // Test TOML format
    let toml_renderer = TemplateFormatRenderer::new()
        TemplateFormat::Config(ConfigFormat::Toml)
    let toml_result = toml_renderer.render(&data).unwrap();
    assert!(toml_result.contains(app_name = \ MyApp")"
    assert!(toml_result.contains(port = , 8080)
    
    // Test environment variables format
    let env_renderer = TemplateFormatRenderer::new()
        TemplateFormat::Config(ConfigFormat::Env)
    let env_result = env_renderer.render(&data).unwrap()
    assert!(env_result.contains(APP_NAME =)
    assert!(env_result.contains(DEBUG =")")"}
#[test]
fn test_template_inheritance_and_includes() {let temp_dir = create_test_template_dir()
    
    // Create a header partial
    let header_partial = r#"about " >About</a>
    </nav>
</header>;"#;
    
    // Create a template that includes the header
    let page_template = r#"#"#;
    
    fs::write(temp_dir.path().join(")
    fs::write(temp_dir.path().join("page .html), page_template).unwrap()"title, CursedObject::String("TestPage.to_string()
    context.set(site_name, CursedObject::String(MySite.to_string()
    context.set("Page content here.to_string()
    
    let result = engine.render("page .html, context).unwrap()"<title>Test Page</title>");
    assert!(result.contains(<h1>My Site</h1>"Page content here)"
    assert!(result.contains("from.to_string(), CursedObject::String("noreply @example.com.to_string()
    email_data.insert("user @example.com.to_string()
    email_data.insert("subject.to_string(), CursedObject::String(".to_string()
    email_data.insert("text.to_string(), CursedObject::String(".to_string()
    email_data.insert("html.to_string(), CursedObject::String("From : noreply@example.com)")
    assert!(result.contains(")
    assert!(result.contains("Subject : Welcome!"Content-Type: text/plain)")
    assert!(result.contains(")
    assert!(result.contains("Welcometo our service!"{{unclosed "variable).unwrap()
    let context = TemplateContext::new()
    let result = engine.render(")
    assert!(result.is_err();

#[test]
fn test_global_context_variables() {let temp_dir = create_test_template_dir()
    let loader = Arc::new(FileSystemLoader::new(temp_dir.path()
    let config = TemplateConfig::default()
    let engine = TemplateEngine::with_config_and_loader(config, loader)
    
    // Set global variables
    engine.set_global(app_name, CursedObject::String(TestApp.to_string().unwrap()
    engine.set_global("version, CursedObject::String(, 1.0.0 .to_string().unwrap()"<footer>
    <p>{{app_name} v{{version}</p>
    <p>User: {{user_name}</p>
</footer>;"##;
    fs::write(temp_dir.path().join(")
    let mut context = TemplateContext::new()
    context.set("user_name, CursedObject::String(")
    
    assert!(result.contains(Test App v1.0., 0)"
    assert!(result.contains(User : Alice)")"PerformanceTest.to_string()
    context.set(content, CursedObject::String(Testingperformance.to_string()
    
    // First render (cache miss)
    let start = std::time::Instant::now()
    let result1 = engine.render(base .html, context.clone().unwrap()
    let first_duration = start.elapsed()
    // Second render (cache hit)
    let start = std::time::Instant::now()
    let result2 = engine.render(base .html, context).unwrap()
    let second_duration = start.elapsed()
    
    assert_eq!(result1, result2);
    assert!(result1.contains(PerformanceTest);"<div class= organization>
    <h1>{{org.name}</h1>
    <p>{{org.description}</p>
    
    {% for department in org.departments %}
    <div class= "department>
        <h2>  {{department.name}</h2>
        <p>Manager: {{department.manager.name}</p>
        
        <h3>Employees:</h3>
        <ul>
        {% for employee in department.employees %}
            <li>
                  {{employee.name} ({{employee.role})
                {% if employee.skills %}
                <ul>
                    {% for skill in employee.skills %}
                    <li>  {{skill}</li>
                {% end %}
                </ul>
                {% end %}
            </li>
        {% end %}
        </ul>
    </div>
    {% end %}
</div>;"#;
    
    fs::write(temp_dir.path().join(nested .html), nested_template).unwrap()
    
    // Create complex nested data
    let mut org = HashMap::new()
    org.insert(name.to_string(), CursedObject::String(TechCorp.to_string()
    org.insert(description.to_string(), CursedObject::String(A technology company.to_string()
    
    let departments = vec![{let mut dept = HashMap::new()")
            dept.insert(");
            dept.insert("manager.to_string(), CursedObject::Map(manager);
            let employees = vec![{let mut emp = HashMap::new()
                    emp.insert("Alice.to_string()
                    emp.insert(role.to_string(), CursedObject::String(SeniorDeveloper.to_string()
                    emp.insert("skills.to_string(), CursedObject::Array(vec!["
                        CursedObject::String("Python.to_string()]
            dept.insert(employees.to_string(), CursedObject::Array(employees)"departments.to_string(), CursedObject::Array(departments)
    let mut context = TemplateContext::new()
    context.set(org, CursedObject::Map(org)
    
    let result = engine.render(")
    assert!(result.contains("TechCorp);
    assert!(result.contains(")
    assert!(result.contains("Alice);
    assert!(result.contains(SeniorDeveloper)"Rust);
    assert!(result.contains(Python ")
    assert!(result.contains(")
    assert!(result.contains("Docker " {{name}! You have {{count} messages.";)
    let mut context = TemplateContext::new()
    context.set(")
    let result = engine.render_string(template_string, context).unwrap();
    assert_eq!(result,  "Hello "}
#[test]
fn test_template_middleware_integration() {let renderer = WebTemplateRenderer::new("templates)
    let middleware = TemplateMiddleware::new(renderer)
    
    let request = WebTemplateRequest {method:  "
        url: /"profile "ta "b.to_string(),  ".to_string()
            query},
        form: HashMap::new()
        cookies: HashMap::new()
        session: {let mut session = HashMap::new();
            session.insert("user_id.to_string(), CursedObject::Integer(123);"Alice.to_string()
            user.insert(role.to_string(), CursedObject::String(admin.to_string()
            user}),}
    
    let context = TemplateContext::new()")
    // This would normally render an actual template, but we're testing the middleware structure
    let error = CursedError::TemplateError {message:  Template  not found.to_string()
        source_location: None}
    
    let error_response = middleware.handle_error(&error, 404, &request).unwrap()
    assert_eq!(error_response.status, 404)
    assert!(error_response.body.contains("404);
// Performance and stress tests
#[test]
fn test_template_rendering_performance() {let temp_dir = create_test_template_dir()
    let loader = Arc::new(FileSystemLoader::new(temp_dir.path()
    let config = TemplateConfig::default()
    let engine = TemplateEngine::with_config_and_loader(config, loader)
    
    let test_data = create_test_data()
    let mut context = TemplateContext::new()
    for (key, value) in test_data   {context.set(key, value)}
    
    let start = std::time::Instant::now()
    
    // Render the same template multiple times
    for _ in 0..100   {let _result = engine.render(user.html , context.clone().unwrap()}
    
    let duration = start.elapsed()
    println!(
    
    // Should complete within reasonable time;
    assert!(duration.as_millis() < 5000); // 5 seconds max}

#[test]
fn test_large_data_structure_rendering() {let temp_dir = create_test_template_dir()
    let loader = Arc::new(FileSystemLoader::new(temp_dir.path()
    let config = TemplateConfig::default()
    let engine = TemplateEngine::with_config_and_loader(config, loader)
    
    // Create large array template
    let large_template = r#"<table>"#
    <thead>
        <tr><th>ID</th><th>Name</th><th>Value</th></tr>
    </thead>
    <tbody>
    {% for item in items %}
        <tr>
            <td>  {{item.id}</td>
            <td>{{item.name}</td>
            <td>{{item.value}</td>
        </tr>
    {% end %}
    </tbody>
</table>;"large .html), large_template).unwrap()
    
    // Create large dataset (1000 items)
    let items: Vec<CursedObject> = (0..1000).map(|i| {let mut item = HashMap::new();
        item.insert(id.to_string(), CursedObject::Integer(i);
        item.insert(name.to_string(), CursedObject::String(format!("value.to_string(), CursedObject::Float(i as f64 * 1.5)
        CursedObject::Map(item)}).collect()
    
    let mut context = TemplateContext::new()")
    context.set(items "large .html, context).unwrap()
    let duration = start.elapsed()
    
    println!(";
    assert!(result.contains("<table>)
    assert!(result.contains(Item, 0)"Item 999)")
    assert!(duration.as_millis() < 10000); // 10 seconds max}
