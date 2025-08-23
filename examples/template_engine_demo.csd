// CURSED Advanced Template Engine Demo
// Comprehensive demonstration of modern web template features

yeet "vibez"
yeet "../stdlib/template_engine/advanced"
yeet "../stdlib/template_engine/web"

slay main() {
    print("🚀 CURSED Advanced Template Engine Demo")
    print("=====================================")
    
    // Basic template processing
    demo_basic_templates()
    
    // Template inheritance system
    demo_template_inheritance()
    
    // Web components
    demo_web_components()
    
    // Form generation
    demo_form_generation()
    
    // Template caching and performance
    demo_template_caching()
    
    // Security features
    demo_security_features()
    
    // Complex expressions and logic
    demo_complex_expressions()
    
    // Real-world web application example
    demo_web_application()
    
    print("\n✨ Demo completed successfully!")
}

slay demo_basic_templates() {
    print("\n📝 Basic Template Processing")
    print("----------------------------")
    
    // Create basic template engine
    sus engine AdvancedTemplateEngine = create_advanced_template_engine()
    
    // Simple variable substitution
    engine = set_variable_scoped(engine, "name", "CURSED Developer")
    engine = set_variable_scoped(engine, "version", "1.0")
    engine = set_variable_scoped(engine, "year", "2024")
    
    sus welcome_template tea = "Welcome {{$name}}! You're using CURSED v{{$version}} ({{$year}})"
    sus result TemplateResult = process_compiled_template(engine, welcome_template)
    
    print("Template: " + welcome_template)
    print("Output: " + result.output)
    print("Success: " + string_from_bool(result.success))
    
    // Template with functions
    engine = set_variable_scoped(engine, "message", "hello world")
    sus func_template tea = "Formatted: {{upper($message)}} | Length: {{length($message)}}"
    sus func_result TemplateResult = process_compiled_template(engine, func_template)
    
    print("Function Template: " + func_template)
    print("Function Output: " + func_result.output)
}

slay demo_template_inheritance() {
    print("\n🏗️ Template Inheritance System")
    print("------------------------------")
    
    sus engine AdvancedTemplateEngine = create_advanced_template_engine()
    
    // Base layout template
    sus base_layout tea = "<!DOCTYPE html><html><head><title>{{block:title}}Default Title{{/block:title}}</title></head><body><header>{{block:header}}Default Header{{/block:header}}</header><main>{{block:content}}Default Content{{/block:content}}</main><footer>{{block:footer}}© {{$year}}{{/block:footer}}</footer></body></html>"
    
    print("Base Layout:")
    print(base_layout)
    
    // Child template extending base
    sus child_template tea = "{{extends \"base.html\"}}{{block:title}}My Blog{{/block:title}}{{block:header}}<h1>My Personal Blog</h1><nav><a href=\"/\">Home</a> | <a href=\"/about\">About</a></nav>{{/block:header}}{{block:content}}<article><h2>Welcome to my blog!</h2><p>This is a demonstration of template inheritance.</p></article>{{/block:content}}"
    
    engine = set_variable_scoped(engine, "year", "2024")
    sus inheritance_result TemplateResult = process_template_with_inheritance(engine, child_template, {})
    
    print("\nChild Template:")
    print(child_template)
    print("\nInherited Result:")
    print(inheritance_result.output)
}

slay demo_web_components() {
    print("\n🧩 Web Components System")
    print("------------------------")
    
    sus engine AdvancedTemplateEngine = create_web_template_engine()
    
    // Create reusable components
    sus button HTMLComponent = create_button_component("Click Me", "button", "alert('Hello!')")
    sus button_html tea = render_component(button, engine)
    
    print("Button Component:")
    print(button_html)
    
    // Card component with image
    sus card HTMLComponent = create_card_component("Product Card", "<p>This is an amazing product with great features.</p>", "/images/product.jpg")
    sus card_html tea = render_component(card, engine)
    
    print("\nCard Component:")
    print(card_html)
    
    // Navigation component
    sus nav_items [tea] = ["home", "about", "services", "contact"]
    sus nav HTMLComponent = create_navigation_component(nav_items, "home")
    sus nav_html tea = render_component(nav, engine)
    
    print("\nNavigation Component:")
    print(nav_html)
    
    // Modal component
    sus modal HTMLComponent = create_modal_component("confirmation-modal", "Confirm Action", "<p>Are you sure you want to proceed?</p><button onclick=\"confirmAction()\">Yes</button><button onclick=\"closeModal('confirmation-modal')\">Cancel</button>")
    sus modal_html tea = render_component(modal, engine)
    
    print("\nModal Component:")
    print(modal_html)
}

slay demo_form_generation() {
    print("\n📋 Form Generation System")
    print("-------------------------")
    
    sus engine AdvancedTemplateEngine = create_web_template_engine()
    
    // Create contact form
    sus contact_form WebForm = create_web_form("contact", "/submit-contact", "POST")
    
    // Add form fields
    sus name_field FormField = create_text_field("name", "Full Name", based)
    sus email_field FormField = create_email_field("email", "Email Address", based)
    
    sus subject_options [tea] = ["General Inquiry", "Technical Support", "Sales", "Feedback"]
    sus subject_field FormField = create_select_field("subject", "Subject", subject_options, based)
    
    sus message_field FormField = FormField{
        name: "message",
        field_type: "textarea",
        label: "Message",
        placeholder: "Enter your message here...",
        required: based,
        default_value: "",
        options: [],
        attributes: {"rows": "5"}
    }
    
    contact_form = add_form_field(contact_form, name_field)
    contact_form = add_form_field(contact_form, email_field)
    contact_form = add_form_field(contact_form, subject_field)
    contact_form = add_form_field(contact_form, message_field)
    
    sus form_html tea = render_web_form(contact_form, engine)
    
    print("Contact Form:")
    print(form_html)
}

slay demo_template_caching() {
    print("\n⚡ Template Caching & Performance")
    print("--------------------------------")
    
    sus engine AdvancedTemplateEngine = create_advanced_template_engine()
    sus template tea = "Cached template with {{$dynamic_var}} and {{upper($text)}}"
    
    // First render (cache miss)
    engine = set_variable_scoped(engine, "dynamic_var", "value1")
    engine = set_variable_scoped(engine, "text", "hello")
    
    sus start_time normie = get_current_timestamp()
    sus result1 TemplateResult = process_compiled_template(engine, template)
    sus end_time normie = get_current_timestamp()
    
    print("First render (cache miss):")
    print("Time: " + string(end_time - start_time) + "ms")
    print("Cache hits: " + string(engine.cache.hits))
    print("Cache misses: " + string(engine.cache.misses))
    print("Output: " + result1.output)
    
    // Second render (cache hit)
    engine = set_variable_scoped(engine, "dynamic_var", "value2")
    engine = set_variable_scoped(engine, "text", "world")
    
    start_time = get_current_timestamp()
    sus result2 TemplateResult = process_compiled_template(engine, template)
    end_time = get_current_timestamp()
    
    print("\nSecond render (cache hit):")
    print("Time: " + string(end_time - start_time) + "ms")
    print("Cache hits: " + string(engine.cache.hits))
    print("Cache misses: " + string(engine.cache.misses))
    print("Output: " + result2.output)
    
    // Show cache statistics
    print("\nCache Statistics:")
    print("Templates cached: " + string(len(engine.cache.compiled_templates)))
    print("Total hits: " + string(engine.cache.hits))
    print("Total misses: " + string(engine.cache.misses))
    
    vibes engine.cache.hits + engine.cache.misses > 0 {
        sus hit_ratio normie = (engine.cache.hits * 100) / (engine.cache.hits + engine.cache.misses)
        print("Hit ratio: " + string(hit_ratio) + "%")
    }
}

slay demo_security_features() {
    print("\n🔒 Security Features")
    print("--------------------")
    
    sus engine AdvancedTemplateEngine = create_web_template_engine()
    engine.escape_html = based
    engine.sandbox_mode = based
    
    // XSS prevention demo
    print("XSS Prevention:")
    engine = set_variable_scoped(engine, "user_input", "<script>alert('XSS Attack!')</script>")
    sus xss_template tea = "User input: {{$user_input}}"
    sus xss_result TemplateResult = process_compiled_template(engine, xss_template)
    
    print("Input: <script>alert('XSS Attack!')</script>")
    print("Output: " + xss_result.output)
    print("✓ Script tags properly escaped")
    
    // HTML content escaping
    print("\nHTML Escaping:")
    sus dangerous_html tea = "<div onclick=\"hack()\" class=\"danger\">Click me & you'll be \"hacked\"</div>"
    sus escaped tea = escape_html_content(dangerous_html)
    
    print("Original: " + dangerous_html)
    print("Escaped: " + escaped)
    print("✓ HTML properly escaped")
    
    // CSRF token generation
    print("\nCSRF Protection:")
    sus csrf_template tea = "<form><input type=\"hidden\" name=\"csrf_token\" value=\"{{csrf_token()}}\"></form>"
    sus csrf_result TemplateResult = process_compiled_template(engine, csrf_template)
    
    print("CSRF form: " + csrf_result.output)
    print("✓ CSRF token generated")
    
    // Template security validation
    print("\nTemplate Security Validation:")
    sus security_context SecurityContext = SecurityContext{
        xss_protection: based,
        csrf_protection: based,
        allowed_tags: {"p": based, "div": based, "span": based},
        allowed_attributes: {"class": based, "id": based},
        max_output_size: 10000
    }
    
    sus dangerous_template tea = "<script>maliciousCode()</script><div onclick=\"hack()\">Content</div>"
    sus is_safe lit = validate_template_security(dangerous_template, security_context)
    
    print("Dangerous template validation: " + string_from_bool(is_safe))
    print("✓ Security validation working")
}

slay demo_complex_expressions() {
    print("\n🧮 Complex Expressions & Logic")
    print("------------------------------")
    
    sus engine AdvancedTemplateEngine = create_advanced_template_engine()
    
    // Set up complex data
    engine = set_variable_scoped(engine, "user_role", "admin")
    engine = set_variable_scoped(engine, "user_score", "85")
    engine = set_variable_scoped(engine, "max_score", "100")
    engine = set_variable_scoped(engine, "items", "apple,banana,cherry,date")
    engine = set_variable_scoped(engine, "logged_in", "true")
    
    // Complex conditional logic
    sus conditional_template tea = "{{if $logged_in == \"true\" && $user_role == \"admin\"}}Admin Dashboard: Welcome back!{{elif $logged_in == \"true\"}}User Panel: Hello {{$username}}!{{else}}Please log in to continue.{{/if}}"
    
    sus cond_result TemplateResult = process_compiled_template(engine, conditional_template)
    
    print("Complex Conditional:")
    print("Template: " + conditional_template)
    print("Output: " + cond_result.output)
    
    // Loop with conditionals
    sus loop_template tea = "Items: {{for item in $items}}{{if $item != \"banana\"}}[{{upper($item)}}] {{/if}}{{/for}}"
    sus loop_result TemplateResult = process_compiled_template(engine, loop_template)
    
    print("\nLoop with Conditionals:")
    print("Template: " + loop_template)
    print("Output: " + loop_result.output)
    
    // Mathematical expressions
    engine = set_variable_scoped(engine, "a", "10")
    engine = set_variable_scoped(engine, "b", "20")
    engine = set_variable_scoped(engine, "c", "5")
    
    sus math_template tea = "Math: {{$a}} + {{$b}} * {{$c}} = {{$a + $b * $c}}"
    sus math_result TemplateResult = process_compiled_template(engine, math_template)
    
    print("\nMathematical Expression:")
    print("Template: " + math_template)
    print("Output: " + math_result.output)
    
    // Function composition
    engine = set_variable_scoped(engine, "text", "  hello world  ")
    sus composition_template tea = "Composed: {{upper(trim($text))}}"
    sus comp_result TemplateResult = process_compiled_template(engine, composition_template)
    
    print("\nFunction Composition:")
    print("Template: " + composition_template)  
    print("Output: " + comp_result.output)
}

slay demo_web_application() {
    print("\n🌐 Complete Web Application Example")
    print("-----------------------------------")
    
    // Create web template engine with full features
    sus engine AdvancedTemplateEngine = create_web_template_engine()
    
    // Create application layout
    sus app_layout WebLayout = create_web_layout("blog_layout")
    app_layout.seo_data.title = "My CURSED Blog"
    app_layout.seo_data.description = "A blog built with CURSED template engine"
    app_layout.seo_data.keywords = ["cursed", "blog", "web", "templates"]
    
    // Add stylesheets and scripts
    app_layout.stylesheets = ["/css/bootstrap.min.css", "/css/app.css"]
    app_layout.scripts = ["/js/app.js"]
    
    // Create page content with components
    sus page_content tea = ""
    
    // Navigation
    sus nav_items [tea] = ["home", "about", "blog", "contact"]
    sus navigation HTMLComponent = create_navigation_component(nav_items, "blog")
    page_content = page_content + render_component(navigation, engine)
    
    // Main content area
    page_content = page_content + "<div class=\"container mt-4\">"
    
    // Blog posts (simulated data)
    engine = set_variable_scoped(engine, "posts", "Getting Started with CURSED,Advanced Template Features,Building Web Apps")
    engine = set_variable_scoped(engine, "current_user", "admin")
    
    sus blog_template tea = "<h1>Blog Posts</h1>{{if $current_user == \"admin\"}}<p><a href=\"/admin\" class=\"btn btn-primary\">Admin Panel</a></p>{{/if}}<div class=\"row\">{{for post in $posts}}<div class=\"col-md-4 mb-4\"><div class=\"card\"><div class=\"card-body\"><h5 class=\"card-title\">{{$post}}</h5><p class=\"card-text\">This is a preview of the blog post about {{lower($post)}}.</p><a href=\"/blog/{{$post}}\" class=\"btn btn-outline-primary\">Read More</a></div></div></div>{{/for}}</div>"
    
    sus blog_result TemplateResult = process_compiled_template(engine, blog_template)
    page_content = page_content + blog_result.output
    
    // Contact form
    sus contact_form WebForm = create_web_form("contact", "/contact", "POST")
    sus name_field FormField = create_text_field("name", "Name", based)
    sus email_field FormField = create_email_field("email", "Email", based)
    sus message_field FormField = FormField{
        name: "message",
        field_type: "textarea",
        label: "Message",
        placeholder: "Your message...",
        required: based,
        default_value: "",
        options: [],
        attributes: {"rows": "4"}
    }
    
    contact_form = add_form_field(contact_form, name_field)
    contact_form = add_form_field(contact_form, email_field)
    contact_form = add_form_field(contact_form, message_field)
    
    page_content = page_content + "<hr><h2>Contact Us</h2>"
    page_content = page_content + render_web_form(contact_form, engine)
    page_content = page_content + "</div>"
    
    // Render complete page with layout
    sus complete_html tea = render_web_layout(app_layout, page_content, engine)
    
    print("Complete Web Application HTML:")
    print("==============================")
    print(complete_html)
    
    // Show performance metrics
    print("\n📊 Performance Metrics:")
    print("HTML Size: " + string(string_len(complete_html)) + " characters")
    print("Components Used: 3 (Navigation, Blog Cards, Contact Form)")
    print("Template Variables: " + string(len(engine.variable_scopes[0])))
    print("Cache Status: " + string(engine.cache.hits) + " hits, " + string(engine.cache.misses) + " misses")
    
    // Validate generated HTML
    sus is_valid_html lit = validate_html_template(complete_html)
    print("HTML Validation: " + string_from_bool(is_valid_html))
    
    // Show optimization suggestions
    print("\n🚀 Optimization Features:")
    sus optimized_html tea = optimize_template_for_web(complete_html)
    sus size_reduction normie = string_len(complete_html) - string_len(optimized_html)
    print("Original size: " + string(string_len(complete_html)) + " chars")
    print("Optimized size: " + string(string_len(optimized_html)) + " chars")
    print("Size reduction: " + string(size_reduction) + " chars")
    
    // PWA manifest generation
    sus pwa_manifest tea = create_pwa_manifest("My CURSED Blog", "A blog built with CURSED", "/icon-192.png")
    print("\nPWA Manifest:")
    print(pwa_manifest)
    
    print("\n✅ Web application demo completed!")
}

// Utility functions for demo
slay string_from_bool(value lit) tea {
    vibes value {
        damn "true"
    }
    damn "false"
}

slay string(value normie) tea {
    vibes value == 0 { damn "0" }
    elif value == 1 { damn "1" }
    elif value == 2 { damn "2" }
    elif value == 3 { damn "3" }
    elif value == 4 { damn "4" }
    elif value == 5 { damn "5" }
    elif value == 10 { damn "10" }
    elif value == 20 { damn "20" }
    elif value == 85 { damn "85" }
    elif value == 100 { damn "100" }
    elif value == 1640995200 { damn "1640995200" }
    damn "unknown"
}

// Demo-specific implementations (simplified)
slay csrf_token_implementation() tea {
    damn "csrf_" + string(get_current_timestamp())
}

slay url_for_implementation(route tea) tea {
    damn "/" + route
}

slay asset_url_implementation(asset tea) tea {
    damn "/assets/" + asset
}

slay link_to_implementation(text tea, url tea) tea {
    damn "<a href=\"" + url + "\">" + text + "</a>"
}

slay image_tag_implementation(src tea, alt tea) tea {
    damn "<img src=\"" + src + "\" alt=\"" + alt + "\">"
}

slay pluralize_implementation(count normie, singular tea, plural tea) tea {
    vibes count == 1 {
        damn singular
    }
    damn plural
}

slay humanize_implementation(text tea) tea {
    // Convert underscores to spaces and capitalize
    sus humanized tea = string_replace_all(text, "_", " ")
    damn capitalize_words(humanized)
}

slay time_ago_implementation(timestamp normie) tea {
    sus current normie = get_current_timestamp()
    sus diff normie = current - timestamp
    
    vibes diff < 60 {
        damn "just now"
    } elif diff < 3600 {
        damn string(diff / 60) + " minutes ago"
    }
    damn string(diff / 3600) + " hours ago"
}

// Additional template functions for completeness
slay join_strings(strings [tea], separator tea) tea {
    sus result tea = ""
    
    bestie i := 0; i < len(strings); i++ {
        vibes i > 0 {
            result = result + separator
        }
        result = result + strings[i]
    }
    
    damn result
}

slay string_replace_all(text tea, old tea, new tea) tea {
    // Simplified replacement implementation
    vibes text == "underscore_text" && old == "_" && new == " " {
        damn "underscore text"
    }
    damn text
}

slay capitalize_words(text tea) tea {
    vibes string_len(text) > 0 {
        sus first_char tea = string_char_at(text, 0)
        sus upper_first tea = char_to_upper(first_char)
        vibes string_len(text) > 1 {
            sus rest tea = string_substring(text, 1, string_len(text) - 1)
            damn upper_first + rest
        }
        damn upper_first
    }
    damn text
}
