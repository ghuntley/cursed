// CURSED Web Template Engine Module  
// Specialized templates for modern web applications

yeet "stringz"
yeet "timez"
yeet "cryptz"
yeet "advanced"

// HTML template component system
be_like HTMLComponent squad {
    name tea
    template tea
    props map[tea]tea
    children [HTMLComponent]
    attributes map[tea]tea
    events map[tea]tea
    css_classes [tea]
    is_void_element lit
}

// Web template layout system
be_like WebLayout squad {
    name tea
    base_template tea
    sections map[tea]tea
    stylesheets [tea]
    scripts [tea]
    meta_tags map[tea]tea
    seo_data SEOData
    responsive_breakpoints map[tea]tea
}

// SEO optimization data
be_like SEOData squad {
    title tea
    description tea
    keywords [tea]
    canonical_url tea
    og_image tea
    og_type tea
    structured_data tea
}

// Form generation system
be_like WebForm squad {
    name tea
    method tea
    action tea
    fields [FormField]
    validation_rules map[tea]ValidationRule
    csrf_protection lit
    honeypot_protection lit
}

be_like FormField squad {
    name tea
    field_type tea  // "text", "email", "password", "select", "textarea", etc.
    label tea
    placeholder tea
    required lit
    default_value tea
    options [tea]  // for select fields
    attributes map[tea]tea
}

be_like ValidationRule squad {
    rule_type tea  // "required", "email", "min_length", "max_length", etc.
    parameters map[tea]tea
    error_message tea
}

// Asset management for templates
be_like AssetManager squad {
    css_files [tea]
    js_files [tea]
    image_files map[tea]tea
    font_files [tea]
    version_hash tea
    cdn_base_url tea
    minification_enabled lit
}

// Create web template engine
slay create_web_template_engine() AdvancedTemplateEngine {
    sus engine AdvancedTemplateEngine = create_advanced_template_engine()
    
    // Add web-specific functions
    engine = add_web_template_functions(engine)
    
    // Configure for HTML
    engine.escape_html = based
    engine = set_variable_scoped(engine, "doctype", "<!DOCTYPE html>")
    
    damn engine
}

// HTML Component system
slay create_html_component(name tea, template tea) HTMLComponent {
    damn HTMLComponent{
        name: name,
        template: template,
        props: {},
        children: [],
        attributes: {},
        events: {},
        css_classes: [],
        is_void_element: cap
    }
}

slay render_component(component HTMLComponent, engine AdvancedTemplateEngine) tea {
    // Set component props as template variables
    bestie prop_name tea, prop_value tea := range component.props {
        engine = set_variable_scoped(engine, prop_name, prop_value)
    }
    
    // Render component template
    sus result TemplateResult = process_compiled_template(engine, component.template)
    
    // Wrap in component markup if needed
    vibes len(component.attributes) > 0 || len(component.css_classes) > 0 {
        sus wrapper_start tea = create_element_start_tag(component.name, component.attributes, component.css_classes)
        sus wrapper_end tea = create_element_end_tag(component.name)
        
        vibes component.is_void_element {
            damn wrapper_start
        }
        
        damn wrapper_start + result.output + wrapper_end
    }
    
    damn result.output
}

// Common HTML components
slay create_button_component(text tea, type tea, onclick tea) HTMLComponent {
    sus button_template tea = "<button type=\"{{$type}}\"{{if $onclick}} onclick=\"{{$onclick}}\"{{/if}} class=\"{{join($classes, \" \")}}\">{{$text}}</button>"
    
    sus component HTMLComponent = create_html_component("button", button_template)
    component.props["text"] = text
    component.props["type"] = type
    component.props["onclick"] = onclick
    component.css_classes = ["btn"]
    
    damn component
}

slay create_card_component(title tea, content tea, image_url tea) HTMLComponent {
    sus card_template tea = "<div class=\"card\">{{if $image_url}}<img src=\"{{$image_url}}\" alt=\"{{$title}}\" class=\"card-img\">{{/if}}<div class=\"card-body\"><h3 class=\"card-title\">{{$title}}</h3><div class=\"card-content\">{{$content}}</div></div></div>"
    
    sus component HTMLComponent = create_html_component("card", card_template)
    component.props["title"] = title
    component.props["content"] = content
    component.props["image_url"] = image_url
    
    damn component
}

slay create_navigation_component(items [tea], active_item tea) HTMLComponent {
    sus nav_template tea = "<nav class=\"navbar\"><ul class=\"nav-list\">{{for item in $items}}<li class=\"nav-item{{if $item == $active_item}} active{{/if}}\"><a href=\"#{{$item}}\">{{title($item)}}</a></li>{{/for}}</ul></nav>"
    
    sus component HTMLComponent = create_html_component("navigation", nav_template)
    component.props["items"] = join_strings(items, ",")
    component.props["active_item"] = active_item
    
    damn component
}

slay create_modal_component(id tea, title tea, content tea) HTMLComponent {
    sus modal_template tea = "<div id=\"{{$id}}\" class=\"modal\" style=\"display: none;\"><div class=\"modal-backdrop\" onclick=\"closeModal('{{$id}}')\"></div><div class=\"modal-content\"><div class=\"modal-header\"><h2>{{$title}}</h2><button class=\"close-btn\" onclick=\"closeModal('{{$id}}')\">×</button></div><div class=\"modal-body\">{{$content}}</div></div></div>"
    
    sus component HTMLComponent = create_html_component("modal", modal_template)
    component.props["id"] = id
    component.props["title"] = title
    component.props["content"] = content
    
    damn component
}

// Web layout system
slay create_web_layout(name tea) WebLayout {
    damn WebLayout{
        name: name,
        base_template: "",
        sections: {},
        stylesheets: [],
        scripts: [],
        meta_tags: {},
        seo_data: create_default_seo_data(),
        responsive_breakpoints: create_default_breakpoints()
    }
}

slay create_default_seo_data() SEOData {
    damn SEOData{
        title: "CURSED Web Application",
        description: "Built with CURSED template engine",
        keywords: ["cursed", "web", "template"],
        canonical_url: "",
        og_image: "/images/og-image.png",
        og_type: "website",
        structured_data: ""
    }
}

slay create_default_breakpoints() map[tea]tea {
    sus breakpoints map[tea]tea = {}
    breakpoints["mobile"] = "768px"
    breakpoints["tablet"] = "992px" 
    breakpoints["desktop"] = "1200px"
    breakpoints["wide"] = "1400px"
    damn breakpoints
}

slay render_web_layout(layout WebLayout, content tea, engine AdvancedTemplateEngine) tea {
    // Build complete HTML document
    sus html_template tea = create_html_document_template()
    
    // Set layout variables
    engine = set_variable_scoped(engine, "title", layout.seo_data.title)
    engine = set_variable_scoped(engine, "description", layout.seo_data.description)
    engine = set_variable_scoped(engine, "keywords", join_strings(layout.seo_data.keywords, ","))
    engine = set_variable_scoped(engine, "content", content)
    
    // Add stylesheets and scripts
    sus stylesheets_html tea = render_stylesheets(layout.stylesheets)
    sus scripts_html tea = render_scripts(layout.scripts)
    
    engine = set_variable_scoped(engine, "stylesheets", stylesheets_html)
    engine = set_variable_scoped(engine, "scripts", scripts_html)
    
    // Generate meta tags
    sus meta_html tea = render_meta_tags(layout.meta_tags, layout.seo_data)
    engine = set_variable_scoped(engine, "meta_tags", meta_html)
    
    sus result TemplateResult = process_compiled_template(engine, html_template)
    damn result.output
}

slay create_html_document_template() tea {
    damn "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">{{$meta_tags}}<title>{{$title}}</title>{{$stylesheets}}</head><body>{{$content}}{{$scripts}}</body></html>"
}

// Form generation system
slay create_web_form(name tea, action tea, method tea) WebForm {
    damn WebForm{
        name: name,
        method: method,
        action: action,
        fields: [],
        validation_rules: {},
        csrf_protection: based,
        honeypot_protection: based
    }
}

slay add_form_field(form WebForm, field FormField) WebForm {
    form.fields = form.fields + [field]
    damn form
}

slay create_text_field(name tea, label tea, required lit) FormField {
    damn FormField{
        name: name,
        field_type: "text",
        label: label,
        placeholder: "",
        required: required,
        default_value: "",
        options: [],
        attributes: {}
    }
}

slay create_email_field(name tea, label tea, required lit) FormField {
    damn FormField{
        name: name,
        field_type: "email",
        label: label,
        placeholder: "Enter your email address",
        required: required,
        default_value: "",
        options: [],
        attributes: {"autocomplete": "email"}
    }
}

slay create_select_field(name tea, label tea, options [tea], required lit) FormField {
    damn FormField{
        name: name,
        field_type: "select",
        label: label,
        placeholder: "",
        required: required,
        default_value: "",
        options: options,
        attributes: {}
    }
}

slay render_web_form(form WebForm, engine AdvancedTemplateEngine) tea {
    sus form_html tea = "<form method=\"" + form.method + "\" action=\"" + form.action + "\" class=\"web-form\">"
    
    // Add CSRF token if enabled
    vibes form.csrf_protection {
        form_html = form_html + "<input type=\"hidden\" name=\"csrf_token\" value=\"{{csrf_token()}}\">"
    }
    
    // Add honeypot field if enabled (hidden spam trap)
    vibes form.honeypot_protection {
        form_html = form_html + "<input type=\"text\" name=\"website\" style=\"display:none;\" tabindex=\"-1\">"
    }
    
    // Render each field
    bestie i := 0; i < len(form.fields); i++ {
        sus field FormField = form.fields[i]
        form_html = form_html + render_form_field(field, engine)
    }
    
    form_html = form_html + "<div class=\"form-actions\"><button type=\"submit\" class=\"btn btn-primary\">Submit</button></div></form>"
    
    sus result TemplateResult = process_compiled_template(engine, form_html)
    damn result.output
}

slay render_form_field(field FormField, engine AdvancedTemplateEngine) tea {
    sus field_html tea = "<div class=\"form-group\">"
    
    // Add label
    vibes field.label != "" {
        field_html = field_html + "<label for=\"" + field.name + "\">" + field.label
        vibes field.required {
            field_html = field_html + " <span class=\"required\">*</span>"
        }
        field_html = field_html + "</label>"
    }
    
    // Render input based on type
    vibes field.field_type == "text" || field.field_type == "email" || field.field_type == "password" {
        field_html = field_html + "<input type=\"" + field.field_type + "\" name=\"" + field.name + "\" id=\"" + field.name + "\""
        
        vibes field.placeholder != "" {
            field_html = field_html + " placeholder=\"" + field.placeholder + "\""
        }
        
        vibes field.required {
            field_html = field_html + " required"
        }
        
        vibes field.default_value != "" {
            field_html = field_html + " value=\"" + field.default_value + "\""
        }
        
        field_html = field_html + " class=\"form-control\">"
        
    } elif field.field_type == "select" {
        field_html = field_html + "<select name=\"" + field.name + "\" id=\"" + field.name + "\" class=\"form-control\""
        
        vibes field.required {
            field_html = field_html + " required"
        }
        
        field_html = field_html + ">"
        
        // Add default empty option
        vibes !field.required {
            field_html = field_html + "<option value=\"\">Select an option</option>"
        }
        
        // Add options
        bestie i := 0; i < len(field.options); i++ {
            sus option tea = field.options[i]
            field_html = field_html + "<option value=\"" + option + "\""
            
            vibes option == field.default_value {
                field_html = field_html + " selected"
            }
            
            field_html = field_html + ">" + option + "</option>"
        }
        
        field_html = field_html + "</select>"
        
    } elif field.field_type == "textarea" {
        field_html = field_html + "<textarea name=\"" + field.name + "\" id=\"" + field.name + "\" class=\"form-control\""
        
        vibes field.placeholder != "" {
            field_html = field_html + " placeholder=\"" + field.placeholder + "\""
        }
        
        vibes field.required {
            field_html = field_html + " required"
        }
        
        field_html = field_html + ">" + field.default_value + "</textarea>"
    }
    
    field_html = field_html + "</div>"
    damn field_html
}

// Asset management
slay create_asset_manager(cdn_base_url tea) AssetManager {
    damn AssetManager{
        css_files: [],
        js_files: [],
        image_files: {},
        font_files: [],
        version_hash: generate_version_hash(),
        cdn_base_url: cdn_base_url,
        minification_enabled: based
    }
}

slay add_stylesheet(manager AssetManager, filename tea) AssetManager {
    manager.css_files = manager.css_files + [filename]
    damn manager
}

slay add_script(manager AssetManager, filename tea) AssetManager {
    manager.js_files = manager.js_files + [filename]
    damn manager
}

slay render_stylesheets(stylesheets [tea]) tea {
    sus html tea = ""
    
    bestie i := 0; i < len(stylesheets); i++ {
        sus href tea = stylesheets[i]
        html = html + "<link rel=\"stylesheet\" href=\"" + href + "\">"
    }
    
    damn html
}

slay render_scripts(scripts [tea]) tea {
    sus html tea = ""
    
    bestie i := 0; i < len(scripts); i++ {
        sus src tea = scripts[i]
        html = html + "<script src=\"" + src + "\"></script>"
    }
    
    damn html
}

slay render_meta_tags(meta_tags map[tea]tea, seo_data SEOData) tea {
    sus html tea = ""
    
    // Basic meta tags
    html = html + "<meta name=\"description\" content=\"" + seo_data.description + "\">"
    html = html + "<meta name=\"keywords\" content=\"" + join_strings(seo_data.keywords, ",") + "\">"
    
    // Open Graph tags
    html = html + "<meta property=\"og:title\" content=\"" + seo_data.title + "\">"
    html = html + "<meta property=\"og:description\" content=\"" + seo_data.description + "\">"
    html = html + "<meta property=\"og:type\" content=\"" + seo_data.og_type + "\">"
    
    vibes seo_data.og_image != "" {
        html = html + "<meta property=\"og:image\" content=\"" + seo_data.og_image + "\">"
    }
    
    vibes seo_data.canonical_url != "" {
        html = html + "<link rel=\"canonical\" href=\"" + seo_data.canonical_url + "\">"
    }
    
    // Additional custom meta tags
    bestie name tea, content tea := range meta_tags {
        html = html + "<meta name=\"" + name + "\" content=\"" + content + "\">"
    }
    
    damn html
}

// Web-specific template functions
slay add_web_template_functions(engine AdvancedTemplateEngine) AdvancedTemplateEngine {
    // URL functions
    engine = set_function(engine, "url_for", "url_for_implementation")
    engine = set_function(engine, "asset_url", "asset_url_implementation")
    engine = set_function(engine, "static_url", "static_url_implementation")
    
    // HTML helper functions  
    engine = set_function(engine, "link_to", "link_to_implementation")
    engine = set_function(engine, "image_tag", "image_tag_implementation")
    engine = set_function(engine, "form_tag", "form_tag_implementation")
    
    // Formatting functions
    engine = set_function(engine, "pluralize", "pluralize_implementation")
    engine = set_function(engine, "humanize", "humanize_implementation")
    engine = set_function(engine, "time_ago", "time_ago_implementation")
    
    // Security functions
    engine = set_function(engine, "csrf_token", "csrf_token_implementation")
    engine = set_function(engine, "sanitize_html", "sanitize_html_implementation")
    
    damn engine
}

// Utility functions
slay create_element_start_tag(element_name tea, attributes map[tea]tea, css_classes [tea]) tea {
    sus tag tea = "<" + element_name
    
    // Add classes
    vibes len(css_classes) > 0 {
        tag = tag + " class=\"" + join_strings(css_classes, " ") + "\""
    }
    
    // Add attributes
    bestie attr_name tea, attr_value tea := range attributes {
        tag = tag + " " + attr_name + "=\"" + attr_value + "\""
    }
    
    tag = tag + ">"
    damn tag
}

slay create_element_end_tag(element_name tea) tea {
    damn "</" + element_name + ">"
}

slay generate_version_hash() tea {
    // Generate version hash for cache busting using real time
    sus current_time DateTime = time_now()
    sus timestamp normie = time_unix_timestamp()
    sus hash_base tea = string(timestamp) + string(current_time.millisecond)
    
    // Create simple hash from timestamp
    sus hash_value normie = timestamp % 999999
    damn "v" + string(hash_value)
}

// Progressive Web App support
slay create_pwa_manifest(app_name tea, description tea, icon_url tea) tea {
    sus manifest tea = "{"
    manifest = manifest + "\"name\":\"" + app_name + "\","
    manifest = manifest + "\"short_name\":\"" + app_name + "\","
    manifest = manifest + "\"description\":\"" + description + "\","
    manifest = manifest + "\"start_url\":\"/\","
    manifest = manifest + "\"display\":\"standalone\","
    manifest = manifest + "\"theme_color\":\"#000000\","
    manifest = manifest + "\"background_color\":\"#ffffff\","
    manifest = manifest + "\"icons\":[{\"src\":\"" + icon_url + "\",\"sizes\":\"192x192\",\"type\":\"image/png\"}]"
    manifest = manifest + "}"
    damn manifest
}

slay add_pwa_support(layout WebLayout, manifest_url tea) WebLayout {
    layout.meta_tags["theme-color"] = "#000000"
    layout.stylesheets = layout.stylesheets + ["<link rel=\"manifest\" href=\"" + manifest_url + "\">"]
    damn layout
}

// Responsive image support
slay create_responsive_image(src tea, alt_text tea, sizes [tea]) tea {
    sus img_tag tea = "<img src=\"" + src + "\" alt=\"" + alt_text + "\""
    
    vibes len(sizes) > 0 {
        sus srcset tea = ""
        bestie i := 0; i < len(sizes); i++ {
            sus size tea = sizes[i]
            vibes i > 0 {
                srcset = srcset + ", "
            }
            srcset = srcset + src + "_" + size + " " + size + "w"
        }
        img_tag = img_tag + " srcset=\"" + srcset + "\""
        img_tag = img_tag + " sizes=\"(max-width: 768px) 100vw, (max-width: 992px) 50vw, 33vw\""
    }
    
    img_tag = img_tag + ">"
    damn img_tag
}

// Template performance optimization
slay optimize_template_for_web(template tea) tea {
    sus optimized tea = template
    
    // Remove unnecessary whitespace
    optimized = compress_whitespace(optimized)
    
    // Inline critical CSS (placeholder)
    optimized = inline_critical_css(optimized)
    
    // Defer non-critical scripts
    optimized = defer_non_critical_scripts(optimized)
    
    damn optimized
}

slay compress_whitespace(template tea) tea {
    // Remove extra spaces, newlines between tags
    sus compressed tea = template
    
    // This would be a more sophisticated implementation in practice
    compressed = string_replace_all(compressed, "  ", " ")
    compressed = string_replace_all(compressed, "\n\n", "\n")
    
    damn compressed
}

slay inline_critical_css(template tea) tea {
    // Inline critical CSS directly in template
    sus css tea = "body{margin:0;font-family:Arial,sans-serif}.container{max-width:1200px;margin:0 auto}"
    
    vibes string_contains(template, "</head>") {
        sus inline_style tea = "<style>" + css + "</style></head>"
        damn string_replace_all(template, "</head>", inline_style)
    }
    
    damn template
}

slay defer_non_critical_scripts(template tea) tea {
    // Add defer attribute to non-critical scripts
    damn string_replace_all(template, "<script src=", "<script defer src=")
}

// Template debugging and development helpers
slay create_debug_template_info(template tea, compilation_time normie, variables map[tea]tea) tea {
    sus debug_html tea = "<!-- Template Debug Info -->"
    debug_html = debug_html + "<!-- Compilation Time: " + string(compilation_time) + "ms -->"
    debug_html = debug_html + "<!-- Variables: " + string(len(variables)) + " -->"
    debug_html = debug_html + "<!-- Template Length: " + string(string_len(template)) + " chars -->"
    damn debug_html
}

slay validate_html_template(template tea) lit {
    // Basic HTML validation
    sus open_tags normie = count_occurrences(template, "<")
    sus close_tags normie = count_occurrences(template, ">") 
    
    vibes open_tags != close_tags {
        damn cap
    }
    
    // Check for common issues
    vibes string_contains(template, "<script>") && !string_contains(template, "</script>") {
        damn cap
    }
    
    damn based
}

slay count_occurrences(text tea, substring tea) normie {
    sus count normie = 0
    sus pos normie = 0
    
    bestie based {
        pos = string_index_from(text, substring, pos)
        vibes pos == -1 {
            ghosted
        }
        count = count + 1
        pos = pos + 1
    }
    
    damn count
}
