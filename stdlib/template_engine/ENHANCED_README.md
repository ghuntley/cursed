# CURSED Template Engine - Production Ready

A comprehensive, high-performance template engine for web development and code generation with advanced features including template inheritance, caching, security, and real-time processing.

## 🚀 Key Features

### ✅ Real Implementations (No More Placeholders!)

- **Real Time Functions**: Uses actual `timez` module for timestamps, current time, and formatting
- **Advanced String Processing**: Proper string manipulation using `stringz` module  
- **Template Compilation**: Actual compilation to optimized instructions with caching
- **Security Features**: XSS protection, CSRF tokens, input validation
- **Web Components**: Complete HTML component system with props and events
- **Performance Optimizations**: Template caching, instruction optimization, asset management

### Core Modules

- `mod.💀` - Basic template engine with variable substitution and control flow
- `advanced.💀` - Advanced features including inheritance, compilation, and security
- `web.💀` - Web-specific templates, components, forms, and layouts

## 🔧 Usage Examples

### Basic Template Processing

```cursed
yeet "template_engine"

sus engine TemplateEngine = create_template_engine()
engine = set_variable(engine, "name", "CURSED")
engine = set_variable(engine, "version", "1.0")

sus template tea = "Hello {{$name}}! Version: {{$version}}"
sus result TemplateResult = process_template(engine, template)
// Output: "Hello CURSED! Version: 1.0"
```

### Advanced Templates with Time Functions

```cursed
yeet "template_engine/advanced"

sus engine AdvancedTemplateEngine = create_advanced_template_engine()
sus template tea = "Generated at: {{now()}} | Timestamp: {{timestamp()}}"
sus result TemplateResult = process_compiled_template(engine, template)
// Output: "Generated at: 2025-01-08 10:30:45 | Timestamp: 1736341845"
```

### Web Components

```cursed
yeet "template_engine/web"

sus button HTMLComponent = create_button_component("Submit", "submit", "handleSubmit()")
sus html tea = render_component(button, engine)
// Output: <button type="submit" onclick="handleSubmit()" class="btn">Submit</button>
```

### Form Generation

```cursed
sus form WebForm = create_web_form("contact", "/submit", "POST")
sus name_field FormField = create_text_field("name", "Full Name", based)
sus email_field FormField = create_email_field("email", "Email", based)

form = add_form_field(form, name_field)
form = add_form_field(form, email_field)
sus form_html tea = render_web_form(form, engine)
```

## 🌟 Enhanced Features

### 1. Real Time Integration
- `{{now()}}` - Current date/time formatted
- `{{timestamp()}}` - Unix timestamp
- `{{iso_date()}}` - ISO 8601 format
- `{{format_time("YYYY-MM-DD")}}` - Custom formatting

### 2. String Functions
- `{{upper($var)}}` - Uppercase conversion
- `{{lower($var)}}` - Lowercase conversion  
- `{{trim($var)}}` - Whitespace removal
- `{{truncate($var, 50)}}` - Text truncation
- `{{capitalize($var)}}` - First letter uppercase

### 3. Template Compilation
- Tokenization and parsing
- Instruction optimization
- Variable extraction
- Function detection
- Caching for performance

### 4. Security Features
- XSS protection with HTML escaping
- CSRF token generation
- Input validation and sanitization
- Script tag filtering
- Maximum output size limits

### 5. Web Layout System
- HTML document templates
- SEO metadata integration
- Asset management (CSS/JS)
- Responsive breakpoints
- Progressive Web App support

### 6. Component System
- Reusable HTML components
- Props and event handling
- CSS class management
- Nested component support
- Void element handling

## 🔒 Security

The template engine includes comprehensive security features:

- **XSS Prevention**: Automatic HTML escaping for variables
- **CSRF Protection**: Token generation and validation
- **Input Sanitization**: Dangerous content filtering
- **Sandbox Mode**: Restricted function execution
- **Content Security**: Maximum output size limits

## ⚡ Performance

Optimized for production use with:

- **Template Compilation**: Pre-parsed instructions
- **Intelligent Caching**: Compiled template storage
- **Instruction Optimization**: Consecutive text merging
- **Asset Versioning**: Cache busting with timestamps
- **Lazy Loading**: On-demand template loading

## 🧪 Testing

Run the comprehensive test suite:

```bash
./zig-out/bin/cursed-zig stdlib/template_engine/comprehensive_test.💀
```

This tests all enhanced features including:
- Real time functions
- String manipulation  
- Template compilation
- Web components
- Form generation
- Layout systems
- Security validation
- Performance caching

## 🎯 Use Cases

Perfect for:
- **Web Applications**: Dynamic HTML generation
- **Email Templates**: Transactional and marketing emails
- **Code Generation**: Automated code scaffolding
- **Documentation**: Dynamic documentation systems
- **Configuration Files**: Template-driven config generation
- **Static Site Generation**: Blog and content sites

## 📚 API Reference

### Basic Engine Functions
- `create_template_engine()` - Create basic engine
- `set_variable(engine, name, value)` - Set template variable
- `process_template(engine, template)` - Process template string

### Advanced Engine Functions  
- `create_advanced_template_engine()` - Create advanced engine
- `compile_template_advanced(template, engine)` - Compile template
- `process_compiled_template(engine, template)` - Execute compiled template

### Web Functions
- `create_web_template_engine()` - Create web-optimized engine
- `create_html_component(name, template)` - Create component
- `create_web_form(name, action, method)` - Create form
- `create_web_layout(name)` - Create layout

## 🚀 Production Ready

This template engine is production-ready with:

✅ **No placeholder implementations**  
✅ **Real time and date functions**  
✅ **Comprehensive string processing**  
✅ **Template compilation and caching**  
✅ **Security features enabled**  
✅ **Web development optimized**  
✅ **Performance benchmarked**  
✅ **Extensively tested**  

Ready for use in web applications, code generation, and content management systems!
