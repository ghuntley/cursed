vibe template_web_demo

yeet "stdlib::template"
yeet "stdlib::io"
yeet "stdlib::collections"

fr fr Web template system demonstration
slay main() {
    fr fr Create web template renderer
    sus web_renderer = WebTemplateRenderer::new("templates")
    
    fr fr Create mock web request
    sus request = WebTemplateRequest {
        method: "GET",
        url: "/dashboard",
        headers: Map::new(),
        query: Map::new(),
        form: Map::new(),
        cookies: Map::new(),
        session: Map::new(),
        user: nil
    }
    
    fr fr Add session data
    request.session.set("user_id", 123)
    request.session.set("username", "alice_dev")
    request.session.set("logged_in", based)
    
    fr fr Add query parameters
    request.query.set("page", "1")
    request.query.set("filter", "active")
    
    fr fr Create user data for template
    sus user_data = Map::new()
    user_data.set("name", "Alice Johnson")
    user_data.set("email", "alice@example.com")
    user_data.set("role", "developer")
    user_data.set("projects", 15)
    user_data.set("last_login", "2024-01-15")
    
    request.user = user_data
    
    fr fr Create template context
    sus context = TemplateContext::new()
    context.set("page_title", "User Dashboard")
    context.set("welcome_message", "Welcome back!")
    
    fr fr Add dashboard statistics
    sus stats = Map::new()
    stats.set("total_users", 1250)
    stats.set("active_projects", 87)
    stats.set("pending_tasks", 23)
    stats.set("completion_rate", 94.5)
    context.set("stats", stats)
    
    fr fr Generate CSRF token
    sus csrf_token = web_renderer.generate_csrf_token(request)
    println("Generated CSRF Token: " + csrf_token)
    
    fr fr Verify CSRF token
    sus is_valid = web_renderer.verify_csrf_token(csrf_token, request)
    println("CSRF Token Valid: " + is_valid.toString())
    println("")
    
    fr fr Create JSON API response
    sus api_data = Map::new()
    api_data.set("status", "success")
    api_data.set("user", user_data)
    api_data.set("stats", stats)
    api_data.set("timestamp", 1640995200)
    
    sus json_response = web_renderer.render_json(api_data)
    println("JSON Response:")
    println("Status: " + json_response.status.toString())
    println("Content-Type: " + json_response.content_type)
    println("Body: " + json_response.body)
    println("")
    
    fr fr Demonstrate error page rendering
    sus error = TemplateError::new("User not found")
    sus error_response = web_renderer.render_error(error, 404, request)
    
    println("Error Response:")
    println("Status: " + error_response.status.toString())
    println("Headers: " + error_response.headers.toString())
    println("Error Page Preview:")
    println(error_response.body.substring(0, 200) + "...")
    
    vibez.spill("Web template demo completed successfully!")
}
