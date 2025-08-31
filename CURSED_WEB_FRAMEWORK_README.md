# CURSED Web Framework & Template Engine

A complete web development framework built entirely in CURSED, providing modern web application capabilities with Gen Z energy.

## 🚀 Features

### Web Framework (`web_framework` module)
- **HTTP Server**: Create and manage HTTP servers on any port
- **Routing System**: Support for GET, POST, PUT, DELETE with parameter extraction
- **Middleware Support**: CORS, logging, authentication, and custom middleware
- **Static File Serving**: Serve CSS, JS, images, and other assets
- **JSON API**: Built-in JSON response helpers and API endpoint support
- **Cookie Handling**: Set and retrieve cookies with security options
- **Request/Response**: Parse HTTP requests and create structured responses

### Template Engine (`template` module)
- **Variable Substitution**: `{{variable}}` syntax with nested object support
- **Conditional Rendering**: `{{#if condition}}...{{/if}}` blocks
- **Loop Processing**: `{{#each items}}...{{/each}}` with index and position
- **Partial Templates**: `{{> partial_name}}` for reusable components
- **Built-in Helpers**: Date formatting, string manipulation, and more
- **Template Compilation**: Compile templates for improved performance

### Integration Features
- **Full Stack**: Complete integration between web framework and templates
- **Database Ready**: Mock data layer with real database module integration
- **Authentication**: Login/logout handling with session management
- **Admin Interface**: Content management capabilities
- **RESTful APIs**: Standard REST endpoint patterns

## 📁 Module Structure

```
stdlib/
├── web_framework/mod.csd    # Main web framework module
├── template/mod.csd         # Template engine module
└── [existing modules]       # json_tea, stringz, main_character, etc.

examples/
└── cursed_blog_app/         # Complete blog application example
    ├── main.csd            # Main application code
    └── public/             # Static assets
        ├── css/style.css   # Stylesheet
        └── js/app.js       # Client-side JavaScript
```

## 🔧 Quick Start

### 1. Create a Simple Web Server

```cursed
yeet "web_framework"

slay main() cringe {
    fr fr Create server on port 8080
    sus server_key tea = web_framework.create_server(8080)
    
    fr fr Add a simple route
    web_framework.add_get_route(server_key, "/", home_handler)
    
    fr fr Start the server
    web_framework.start_server(server_key)
    damn nil
}

slay home_handler(ctx *web_framework.Context) cringe {
    web_framework.create_response(ctx, 200, "<h1>Hello, CURSED!</h1>", "text/html")
    damn nil
}
```

### 2. Use Templates

```cursed
yeet "template"

slay template_example() {
    sus html_template tea = `
    <html>
    <head><title>{{title}}</title></head>
    <body>
        <h1>Welcome {{name}}!</h1>
        <ul>
        {{#each items}}
        <li>{{this}}</li>
        {{/each}}
        </ul>
    </body>
    </html>`
    
    sus data tea = `{
        "title": "My Page",
        "name": "CURSED Developer", 
        "items": ["Feature 1", "Feature 2", "Feature 3"]
    }`
    
    sus rendered tea = template.render_template_string(html_template, data)
    print(rendered)
}
```

### 3. Create JSON APIs

```cursed
slay setup_api_routes(server_key tea) {
    web_framework.add_get_route(server_key, "/api/users", get_users_handler)
    web_framework.add_post_route(server_key, "/api/users", create_user_handler)
    web_framework.add_get_route(server_key, "/api/users/:id", get_user_handler)
}

slay get_users_handler(ctx *web_framework.Context) cringe {
    sus users tea = `[
        {"id": 1, "name": "Alice", "email": "alice@example.com"},
        {"id": 2, "name": "Bob", "email": "bob@example.com"}
    ]`
    web_framework.json_success(ctx, users)
    damn nil
}

slay get_user_handler(ctx *web_framework.Context) cringe {
    sus user_id tea = web_framework.get_param(ctx, "id")
    sus user tea = `{"id": ` + user_id + `, "name": "User ` + user_id + `"}`
    web_framework.json_success(ctx, user)
    damn nil
}
```

## 🌟 Complete Blog Application Example

The `examples/cursed_blog_app/` demonstrates a full-featured blog with:

- **Homepage**: Recent posts with card layout
- **Post Listing**: All published posts with pagination
- **Post Detail**: Individual post view with comments
- **Admin Panel**: Create, edit, delete posts
- **Authentication**: Login/logout functionality
- **JSON API**: RESTful endpoints for all operations
- **Responsive Design**: Mobile-friendly CSS

### Running the Blog Example

```bash
# Run in interpreter mode
cursed examples/cursed_blog_app/main.csd

# Try compilation (experimental)
cursed compile examples/cursed_blog_app/main.csd -o blog_app
./blog_app
```

Visit `http://localhost:8080` to see the blog in action.

## 🎯 API Reference

### Web Framework Functions

#### Server Management
- `create_server(port normie) tea` - Create HTTP server
- `start_server(server_key tea) cringe` - Start server
- `stop_server(server_key tea) cringe` - Stop server

#### Routing
- `add_get_route(server_key tea, path tea, handler slay) cringe` - Add GET route
- `add_post_route(server_key tea, path tea, handler slay) cringe` - Add POST route
- `add_put_route(server_key tea, path tea, handler slay) cringe` - Add PUT route
- `add_delete_route(server_key tea, path tea, handler slay) cringe` - Add DELETE route

#### Request/Response
- `create_response(ctx *Context, status normie, body tea, content_type tea)` - Create HTTP response
- `get_param(ctx *Context, key tea) tea` - Get route parameter
- `get_query(ctx *Context, key tea) tea` - Get query parameter
- `get_header(ctx *Context, key tea) tea` - Get request header
- `set_header(ctx *Context, key tea, value tea)` - Set response header

#### JSON Helpers
- `json_success(ctx *Context, data tea)` - Send JSON success response
- `json_error(ctx *Context, status normie, message tea)` - Send JSON error response

#### Static Files
- `serve_static(server_key tea, url_path tea, file_path tea)` - Serve static files

#### Middleware
- `use_middleware(server_key tea, middleware slay)` - Add middleware
- `cors_middleware(ctx *Context)` - Built-in CORS middleware
- `logging_middleware(ctx *Context)` - Built-in logging middleware

### Template Engine Functions

#### Template Compilation
- `compile_template(content tea) tea` - Compile template string
- `compile_template_from_file(path tea) tea` - Compile from file

#### Template Rendering
- `render_template(template_id tea, data_json tea) tea` - Render compiled template
- `render_template_string(content tea, data_json tea) tea` - Compile and render

#### Variable Processing
- `parse_variables(template tea) []tea` - Extract template variables
- `substitute_value(template tea, key tea, value tea) tea` - Replace single variable

#### Partials
- `register_partial(name tea, content tea)` - Register reusable partial
- `register_partial_from_file(name tea, path tea)` - Register from file

#### Helpers
- `register_helper(name tea, helper_func slay)` - Register template helper

## 🔧 Template Syntax

### Variables
```html
<h1>{{title}}</h1>
<p>Welcome {{user.name}}!</p>
<span>{{price | currency}}</span>
```

### Conditionals
```html
{{#if user.isLoggedIn}}
<p>Welcome back, {{user.name}}!</p>
{{/if}}

{{#if posts.length}}
<div class="posts">...</div>
{{else}}
<p>No posts yet.</p>
{{/if}}
```

### Loops
```html
{{#each posts}}
<article class="post">
    <h2>{{title}}</h2>
    <p>By {{author}} - {{@index}}/{{@total}}</p>
    {{#if @first}}<hr>{{/if}}
</article>
{{/each}}
```

### Partials
```html
{{> header}}
<main>
    {{> post_card}}
</main>
{{> footer}}
```

### Built-in Variables in Loops
- `{{this}}` - Current item
- `{{@index}}` - Zero-based index
- `{{@first}}` - True for first item
- `{{@last}}` - True for last item

### Built-in Formatters
- `{{text | upper}}` - Uppercase
- `{{text | lower}}` - Lowercase  
- `{{text | capitalize}}` - Capitalize first letter
- `{{text | trim}}` - Remove whitespace
- `{{text | length}}` - String length

## 🎨 Styling & Assets

### CSS Framework Integration
The framework works with any CSS framework:

```css
/* Modern CSS with CSS custom properties */
:root {
    --primary-color: #6366f1;
    --success-color: #10b981;
}

.post-card {
    background: white;
    border-radius: 0.75rem;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
    transition: transform 0.2s;
}

.post-card:hover {
    transform: translateY(-2px);
}
```

### Static File Organization
```
public/
├── css/
│   ├── style.css      # Main styles
│   └── admin.css      # Admin-specific styles
├── js/
│   ├── app.js         # Main JavaScript
│   └── admin.js       # Admin functionality
├── images/
│   ├── logo.png
│   └── hero-bg.jpg
└── fonts/
    └── custom-font.woff2
```

## 🧪 Testing

### Run Framework Tests
```bash
# Test in interpreter mode
cursed test_web_framework.csd

# Test compilation (may have limitations)
cursed compile test_web_framework.csd -o test_framework
./test_framework
```

### Test Coverage
- ✅ Server creation and management
- ✅ Route registration and matching
- ✅ Request parsing and response creation
- ✅ Template compilation and rendering
- ✅ Variable substitution and loops
- ✅ JSON API functionality
- ✅ Static file serving
- ✅ Middleware execution
- ✅ Integration between framework and templates

## 🚀 Production Considerations

### Performance
- Template compilation improves rendering speed
- Static file serving with proper MIME types
- Connection pooling in underlying net module
- Middleware caching for repeated operations

### Security
- CORS middleware included
- Cookie security flags (HttpOnly, Secure)
- Request validation and sanitization
- Protection against common web vulnerabilities

### Scalability
- Stateless design for horizontal scaling
- Database abstraction for various backends
- Session management for user state
- Load balancer compatibility

## 🔮 Future Enhancements

### Planned Features
- **WebSocket Support**: Real-time communication
- **File Upload**: Multipart form handling
- **Rate Limiting**: Request throttling
- **Caching**: Response and template caching
- **SSL/TLS**: HTTPS support
- **Database ORM**: Object-relational mapping
- **Testing Framework**: Unit and integration tests
- **CLI Generator**: Scaffold new applications

### Advanced Templates
- **Template Inheritance**: Layout extends
- **Custom Filters**: User-defined formatters
- **Async Partials**: Dynamic content loading
- **Template Macros**: Reusable code blocks

## 📚 Learning Resources

### Example Applications
1. **Simple Blog** - `examples/cursed_blog_app/`
2. **REST API** - JSON-only backend service
3. **Static Site** - Template-based site generator
4. **Admin Panel** - Data management interface

### Best Practices
1. **Separation of Concerns**: Keep templates, logic, and data separate
2. **Error Handling**: Use proper HTTP status codes
3. **Validation**: Validate all user input
4. **Documentation**: Comment your routes and handlers
5. **Testing**: Write tests for all functionality

## 🤝 Contributing

The CURSED Web Framework is built to be extensible:

1. **Add Middleware**: Create custom middleware functions
2. **Template Helpers**: Register new formatting functions
3. **Route Patterns**: Extend pattern matching capabilities
4. **Static Optimization**: Improve asset serving
5. **Database Integration**: Connect with various databases

---

## 🎉 Conclusion

The CURSED Web Framework brings modern web development capabilities to the CURSED language with:

- **Full-Stack Solution**: From HTTP servers to template rendering
- **Modern Architecture**: Middleware, routing, and RESTful APIs
- **Developer Experience**: Intuitive APIs and comprehensive examples
- **Production Ready**: Security, performance, and scalability considerations
- **Extensible Design**: Easy to add new features and integrations

Start building your next web application with CURSED today! 🚀

### Quick Commands
```bash
# Test the framework
cursed test_web_framework.csd

# Run the blog example
cursed examples/cursed_blog_app/main.csd

# Build for production (experimental)
cursed compile examples/cursed_blog_app/main.csd -o blog_server
./blog_server
```

Visit the running application at `http://localhost:8080` and experience the power of CURSED web development! 💪
