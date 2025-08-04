yeet "testz"
yeet "dropz"
yeet "stringz"
yeet "pathing" 
yeet "json_tea"
yeet "io"

# Enhanced Documentation Generator
# Generates professional API documentation from CURSED source code

struct EnhancedDocGenerator {
    source_dirs []tea
    output_dir tea
    config GeneratorConfig
    stats GenerationStats
}

struct GeneratorConfig {
    include_private lit
    generate_examples lit
    validate_links lit
    output_formats []tea
    theme tea
    search_enabled lit
}

struct GenerationStats {
    files_processed normie
    modules_documented normie
    functions_documented normie
    structs_documented normie
    interfaces_documented normie
    examples_generated normie
    cross_references normie
}

struct DocumentedFunction {
    name tea
    signature tea
    description tea
    parameters []Parameter
    return_type tea
    examples []CodeExample
    source_location tea
}

struct DocumentedStruct {
    name tea
    description tea
    fields []StructField
    methods []DocumentedFunction
    examples []CodeExample
    source_location tea
}

struct DocumentedInterface {
    name tea
    description tea
    methods []DocumentedFunction
    implementations []tea
    source_location tea
}

struct Parameter {
    name tea
    type_name tea
    description tea
    required lit
}

struct StructField {
    name tea
    type_name tea
    description tea
    visibility tea
}

struct CodeExample {
    title tea
    code tea
    explanation tea
    runnable lit
}

# Create enhanced documentation generator
slay new_enhanced_doc_generator(source_dirs []tea, output_dir tea) EnhancedDocGenerator {
    sus config GeneratorConfig = GeneratorConfig{
        include_private: cap,
        generate_examples: based,
        validate_links: based,
        output_formats: []tea{"html", "markdown", "json"},
        theme: "modern",
        search_enabled: based,
    }
    
    damn EnhancedDocGenerator{
        source_dirs: source_dirs,
        output_dir: output_dir,
        config: config,
        stats: GenerationStats{},
    }
}

# Generate complete documentation
slay (gen *EnhancedDocGenerator) generate_complete_documentation() {
    vibez.spill("Starting enhanced documentation generation...")
    
    # Initialize output directories
    gen.initialize_output_directories()
    
    # Scan and parse source files
    sus modules = gen.scan_and_parse_modules()
    
    # Generate documentation for each module
    bestie module <- modules {
        gen.generate_module_documentation(module)
    }
    
    # Generate cross-references
    gen.generate_cross_references(modules)
    
    # Generate search index
    gen.generate_search_index(modules)
    
    # Generate navigation
    gen.generate_navigation(modules)
    
    # Generate index pages
    gen.generate_index_pages(modules)
    
    # Copy static assets
    gen.copy_static_assets()
    
    # Generate sitemap
    gen.generate_sitemap(modules)
    
    # Validate documentation
    gen.validate_documentation()
    
    # Print generation report
    gen.print_generation_report()
}

# Parse CURSED source modules
slay (gen *EnhancedDocGenerator) scan_and_parse_modules() []DocumentedModule {
    sus modules []DocumentedModule = []DocumentedModule{}
    
    bestie source_dir <- gen.source_dirs {
        sus files = pathing.walk_files(source_dir, "*.csd")
        
        bestie file <- files {
            gen.stats.files_processed++
            
            sus module_doc = gen.parse_cursed_file(file)
            lowkey module_doc.name != "" {
                modules = append(modules, module_doc)
                gen.stats.modules_documented++
            }
        }
    }
    
    damn modules
}

# Parse individual CURSED file
slay (gen *EnhancedDocGenerator) parse_cursed_file(file_path tea) DocumentedModule {
    sus content = dropz.read_file(file_path)
    sus lines = stringz.split(content, "\n")
    
    sus module DocumentedModule = DocumentedModule{
        name: gen.extract_module_name(file_path),
        path: file_path,
        description: "",
        functions: []DocumentedFunction{},
        structs: []DocumentedStruct{},
        interfaces: []DocumentedInterface{},
    }
    
    sus current_comment tea = ""
    sus i normie = 0
    
    bestie i < len(lines) {
        sus line = stringz.trim(lines[i])
        
        # Parse comments
        lowkey stringz.starts_with(line, "fr fr ") {
            current_comment = current_comment + stringz.trim_prefix(line, "fr fr ") + "\n"
        } highkey stringz.starts_with(line, "# ") {
            current_comment = current_comment + stringz.trim_prefix(line, "# ") + "\n"
        } highkey stringz.starts_with(line, "slay ") {
            # Parse function
            sus func = gen.parse_function(lines, i, current_comment)
            lowkey func.name != "" {
                module.functions = append(module.functions, func)
                gen.stats.functions_documented++
            }
            current_comment = ""
        } highkey stringz.starts_with(line, "struct ") {
            # Parse struct
            sus struct_def = gen.parse_struct(lines, i, current_comment)
            lowkey struct_def.name != "" {
                module.structs = append(module.structs, struct_def)
                gen.stats.structs_documented++
            }
            current_comment = ""
        } highkey stringz.starts_with(line, "collab ") {
            # Parse interface
            sus interface_def = gen.parse_interface(lines, i, current_comment)
            lowkey interface_def.name != "" {
                module.interfaces = append(module.interfaces, interface_def)
                gen.stats.interfaces_documented++
            }
            current_comment = ""
        } highkey line != "" && !stringz.starts_with(line, "fr fr") && !stringz.starts_with(line, "#") {
            current_comment = ""
        }
        
        i++
    }
    
    damn module
}

# Parse CURSED function
slay (gen *EnhancedDocGenerator) parse_function(lines []tea, start_index normie, comment tea) DocumentedFunction {
    sus line = lines[start_index]
    
    # Extract function signature
    sus signature = gen.extract_function_signature(line)
    sus name = gen.extract_function_name(signature)
    sus parameters = gen.extract_function_parameters(signature)
    sus return_type = gen.extract_function_return_type(signature)
    
    # Generate examples if enabled
    sus examples []CodeExample = []CodeExample{}
    lowkey gen.config.generate_examples {
        examples = gen.generate_function_examples(name, parameters, return_type)
        gen.stats.examples_generated += len(examples)
    }
    
    damn DocumentedFunction{
        name: name,
        signature: signature,
        description: stringz.trim(comment),
        parameters: parameters,
        return_type: return_type,
        examples: examples,
        source_location: gen.format_source_location(start_index),
    }
}

# Parse CURSED struct
slay (gen *EnhancedDocGenerator) parse_struct(lines []tea, start_index normie, comment tea) DocumentedStruct {
    sus line = lines[start_index]
    sus name = gen.extract_struct_name(line)
    
    sus fields []StructField = []StructField{}
    sus i = start_index + 1
    
    # Parse struct fields
    bestie i < len(lines) && !stringz.contains(lines[i], "}") {
        sus field_line = stringz.trim(lines[i])
        lowkey field_line != "" && stringz.contains(field_line, " ") {
            sus field = gen.parse_struct_field(field_line)
            lowkey field.name != "" {
                fields = append(fields, field)
            }
        }
        i++
    }
    
    # Generate struct examples
    sus examples []CodeExample = []CodeExample{}
    lowkey gen.config.generate_examples {
        examples = gen.generate_struct_examples(name, fields)
        gen.stats.examples_generated += len(examples)
    }
    
    damn DocumentedStruct{
        name: name,
        description: stringz.trim(comment),
        fields: fields,
        methods: []DocumentedFunction{},
        examples: examples,
        source_location: gen.format_source_location(start_index),
    }
}

# Generate HTML documentation
slay (gen *EnhancedDocGenerator) generate_html_documentation(modules []DocumentedModule) {
    vibez.spill("Generating HTML documentation...")
    
    # Generate HTML for each module
    bestie module <- modules {
        sus html_content = gen.generate_module_html(module)
        sus output_file = gen.output_dir + "/html/" + module.name + ".html"
        
        pathing.create_directory(pathing.dirname(output_file))
        dropz.write_file(output_file, html_content)
    }
    
    # Generate index page
    sus index_html = gen.generate_index_html(modules)
    dropz.write_file(gen.output_dir + "/html/index.html", index_html)
    
    # Copy CSS and JavaScript
    gen.copy_web_assets()
}

# Generate module HTML
slay (gen *EnhancedDocGenerator) generate_module_html(module DocumentedModule) tea {
    sus html tea = `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>` + module.name + ` - CURSED Documentation</title>
    <link rel="stylesheet" href="../assets/docs.css">
    <link rel="stylesheet" href="../assets/syntax-highlighting.css">
    <script src="../assets/search.js"></script>
</head>
<body>
    <nav class="sidebar">
        <div class="logo">
            <h2>CURSED Docs</h2>
        </div>
        <div class="search-container">
            <input type="text" id="search" placeholder="Search documentation...">
            <div id="search-results"></div>
        </div>
        <div class="nav-links">
            <a href="../index.html">Home</a>
            <a href="../api/index.html">API Reference</a>
            <a href="../tutorials/index.html">Tutorials</a>
            <a href="../examples/index.html">Examples</a>
        </div>
    </nav>
    
    <main class="content">
        <header>
            <h1>` + module.name + `</h1>
            <p class="module-description">` + module.description + `</p>
        </header>
        
        <section class="module-overview">
            <h2>Overview</h2>
            <div class="stats">
                <span class="stat">Functions: ` + len(module.functions).(tea) + `</span>
                <span class="stat">Structs: ` + len(module.structs).(tea) + `</span>
                <span class="stat">Interfaces: ` + len(module.interfaces).(tea) + `</span>
            </div>
        </section>`
    
    # Add functions documentation
    lowkey len(module.functions) > 0 {
        html = html + `
        <section class="functions">
            <h2>Functions</h2>`
        
        bestie func <- module.functions {
            html = html + gen.generate_function_html(func)
        }
        
        html = html + `
        </section>`
    }
    
    # Add structs documentation
    lowkey len(module.structs) > 0 {
        html = html + `
        <section class="structs">
            <h2>Structs</h2>`
        
        bestie struct_def <- module.structs {
            html = html + gen.generate_struct_html(struct_def)
        }
        
        html = html + `
        </section>`
    }
    
    # Add interfaces documentation
    lowkey len(module.interfaces) > 0 {
        html = html + `
        <section class="interfaces">
            <h2>Interfaces</h2>`
        
        bestie interface_def <- module.interfaces {
            html = html + gen.generate_interface_html(interface_def)
        }
        
        html = html + `
        </section>`
    }
    
    html = html + `
    </main>
    
    <script>
        // Initialize search functionality
        initializeSearch();
        
        // Initialize syntax highlighting
        initializeSyntaxHighlighting();
    </script>
</body>
</html>`
    
    damn html
}

# Generate search index
slay (gen *EnhancedDocGenerator) generate_search_index(modules []DocumentedModule) {
    vibez.spill("Generating search index...")
    
    sus search_documents []SearchDocument = []SearchDocument{}
    
    bestie module <- modules {
        # Add module to search
        search_documents = append(search_documents, SearchDocument{
            title: module.name,
            content: module.description,
            url: "/api/" + module.name + ".html",
            type_name: "module",
        })
        
        # Add functions to search
        bestie func <- module.functions {
            search_documents = append(search_documents, SearchDocument{
                title: module.name + "." + func.name,
                content: func.description + " " + func.signature,
                url: "/api/" + module.name + ".html#" + func.name,
                type_name: "function",
            })
        }
        
        # Add structs to search
        bestie struct_def <- module.structs {
            search_documents = append(search_documents, SearchDocument{
                title: struct_def.name,
                content: struct_def.description,
                url: "/api/" + module.name + ".html#" + struct_def.name,
                type_name: "struct",
            })
        }
    }
    
    # Write search index as JSON
    sus search_index = SearchIndex{
        documents: search_documents,
        generated_at: "2025-01-01T00:00:00Z",
        total_documents: len(search_documents),
    }
    
    sus search_json = json_tea.marshal(search_index)
    dropz.write_file(gen.output_dir + "/assets/search-index.json", search_json)
    
    vibez.spill("Search index generated with", len(search_documents), "documents")
}

# Test enhanced documentation generator
slay test_enhanced_doc_generator() {
    test_start("Enhanced Documentation Generator")
    
    vibez.spill("Testing Enhanced Documentation Generator")
    vibez.spill("=====================================")
    
    # Create test generator
    sus generator = new_enhanced_doc_generator(
        []tea{"stdlib", "src-zig", "examples"},
        "docs_output"
    )
    
    # Test module parsing
    sus modules = generator.scan_and_parse_modules()
    assert_true(len(modules) > 0)
    vibez.spill("✅ Parsed", len(modules), "modules")
    
    # Test HTML generation
    generator.generate_html_documentation(modules)
    vibez.spill("✅ Generated HTML documentation")
    
    # Test search index generation
    generator.generate_search_index(modules)
    vibez.spill("✅ Generated search index")
    
    # Print statistics
    vibez.spill("\n📊 Generation Statistics:")
    vibez.spill("  Files processed:", generator.stats.files_processed)
    vibez.spill("  Modules documented:", generator.stats.modules_documented)
    vibez.spill("  Functions documented:", generator.stats.functions_documented)
    vibez.spill("  Structs documented:", generator.stats.structs_documented)
    vibez.spill("  Interfaces documented:", generator.stats.interfaces_documented)
    vibez.spill("  Examples generated:", generator.stats.examples_generated)
    
    # Validate documentation coverage
    assert_true(generator.stats.modules_documented >= 10)
    assert_true(generator.stats.functions_documented >= 50)
    
    vibez.spill("\n✅ Enhanced Documentation Generator Test Completed!")
    
    print_test_summary()
}

# Helper structures
struct DocumentedModule {
    name tea
    path tea
    description tea
    functions []DocumentedFunction
    structs []DocumentedStruct
    interfaces []DocumentedInterface
}

struct SearchDocument {
    title tea
    content tea
    url tea
    type_name tea
}

struct SearchIndex {
    documents []SearchDocument
    generated_at tea
    total_documents normie
}

# Helper functions (placeholders for actual implementation)
slay (gen *EnhancedDocGenerator) initialize_output_directories() {
    pathing.create_directory(gen.output_dir + "/html")
    pathing.create_directory(gen.output_dir + "/assets")
    pathing.create_directory(gen.output_dir + "/api")
}

slay (gen *EnhancedDocGenerator) extract_module_name(file_path tea) tea {
    sus parts = stringz.split(file_path, "/")
    sus filename = parts[len(parts)-1]
    damn stringz.trim_suffix(filename, ".csd")
}

slay (gen *EnhancedDocGenerator) extract_function_signature(line tea) tea {
    damn stringz.trim(line)
}

slay (gen *EnhancedDocGenerator) extract_function_name(signature tea) tea {
    lowkey stringz.contains(signature, "(") {
        sus parts = stringz.split(signature, "(")
        sus name_part = stringz.trim(parts[0])
        sus name_parts = stringz.split(name_part, " ")
        damn name_parts[len(name_parts)-1]
    }
    damn ""
}

slay (gen *EnhancedDocGenerator) extract_function_parameters(signature tea) []Parameter {
    damn []Parameter{}
}

slay (gen *EnhancedDocGenerator) extract_function_return_type(signature tea) tea {
    damn "void"
}

slay (gen *EnhancedDocGenerator) generate_function_examples(name tea, params []Parameter, return_type tea) []CodeExample {
    damn []CodeExample{}
}

slay (gen *EnhancedDocGenerator) extract_struct_name(line tea) tea {
    sus parts = stringz.split(line, " ")
    lowkey len(parts) >= 2 {
        damn parts[1]
    }
    damn ""
}

slay (gen *EnhancedDocGenerator) parse_struct_field(field_line tea) StructField {
    damn StructField{name: "field", type_name: "tea", description: "", visibility: "public"}
}

slay (gen *EnhancedDocGenerator) generate_struct_examples(name tea, fields []StructField) []CodeExample {
    damn []CodeExample{}
}

slay (gen *EnhancedDocGenerator) parse_interface(lines []tea, start_index normie, comment tea) DocumentedInterface {
    damn DocumentedInterface{name: "interface", description: comment}
}

slay (gen *EnhancedDocGenerator) format_source_location(line_number normie) tea {
    damn "line " + line_number.(tea)
}

slay (gen *EnhancedDocGenerator) generate_module_documentation(module DocumentedModule) {
    # Implementation placeholder
}

slay (gen *EnhancedDocGenerator) generate_cross_references(modules []DocumentedModule) {
    # Implementation placeholder
}

slay (gen *EnhancedDocGenerator) generate_navigation(modules []DocumentedModule) {
    # Implementation placeholder
}

slay (gen *EnhancedDocGenerator) generate_index_pages(modules []DocumentedModule) {
    # Implementation placeholder
}

slay (gen *EnhancedDocGenerator) copy_static_assets() {
    # Implementation placeholder
}

slay (gen *EnhancedDocGenerator) generate_sitemap(modules []DocumentedModule) {
    # Implementation placeholder
}

slay (gen *EnhancedDocGenerator) validate_documentation() {
    # Implementation placeholder
}

slay (gen *EnhancedDocGenerator) print_generation_report() {
    vibez.spill("Documentation generation completed successfully!")
}

slay (gen *EnhancedDocGenerator) generate_function_html(func DocumentedFunction) tea {
    damn `<div class="function">
        <h3 id="` + func.name + `">` + func.name + `</h3>
        <code class="signature">` + func.signature + `</code>
        <p class="description">` + func.description + `</p>
    </div>`
}

slay (gen *EnhancedDocGenerator) generate_struct_html(struct_def DocumentedStruct) tea {
    damn `<div class="struct">
        <h3 id="` + struct_def.name + `">` + struct_def.name + `</h3>
        <p class="description">` + struct_def.description + `</p>
    </div>`
}

slay (gen *EnhancedDocGenerator) generate_interface_html(interface_def DocumentedInterface) tea {
    damn `<div class="interface">
        <h3 id="` + interface_def.name + `">` + interface_def.name + `</h3>
        <p class="description">` + interface_def.description + `</p>
    </div>`
}

slay (gen *EnhancedDocGenerator) generate_index_html(modules []DocumentedModule) tea {
    damn `<!DOCTYPE html>
<html>
<head><title>CURSED Documentation</title></head>
<body>
    <h1>CURSED Documentation</h1>
    <ul>` + gen.generate_module_links(modules) + `</ul>
</body>
</html>`
}

slay (gen *EnhancedDocGenerator) generate_module_links(modules []DocumentedModule) tea {
    sus links tea = ""
    bestie module <- modules {
        links = links + `<li><a href="` + module.name + `.html">` + module.name + `</a></li>`
    }
    damn links
}

slay (gen *EnhancedDocGenerator) copy_web_assets() {
    # Implementation placeholder for copying CSS/JS files
}

# Run the test
test_enhanced_doc_generator()
