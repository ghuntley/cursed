yeet "dropz"
yeet "stringz"
yeet "json_tea"
yeet "tab_aesthetic"
yeet "pathing"
yeet "testz"

# Documentation Generation Tools
# Comprehensive toolkit for generating, validating, and maintaining documentation

struct DocumentationGenerator {
    source_dir tea
    output_dir tea
    config DocConfig
    templates map[tea]tea
    plugins []DocPlugin
}

struct DocConfig {
    title tea
    description tea
    version tea
    author tea
    theme tea
    output_formats []tea
    include_patterns []tea
    exclude_patterns []tea
    auto_generate_toc lit
    enable_search lit
    enable_pdf_export lit
}

struct DocPlugin {
    name tea
    version tea
    processor slay(content tea) tea
    enabled lit
}

struct DocumentationSite {
    pages []DocPage
    navigation Navigation
    search_index SearchIndex
    metadata SiteMetadata
}

struct DocPage {
    title tea
    path tea
    content tea
    metadata PageMetadata
    toc []TocEntry
}

struct PageMetadata {
    author tea
    date tea
    tags []tea
    category tea
    description tea
}

struct TocEntry {
    title tea
    anchor tea
    level normie
    children []TocEntry
}

struct Navigation {
    sections []NavSection
    breadcrumbs []Breadcrumb
}

struct NavSection {
    title tea
    items []NavItem
}

struct NavItem {
    title tea
    url tea
    icon tea
    children []NavItem
}

struct Breadcrumb {
    title tea
    url tea
}

struct SearchIndex {
    documents []SearchDocument
    index map[tea][]normie
}

struct SearchDocument {
    id normie
    title tea
    content tea
    url tea
    keywords []tea
}

struct SiteMetadata {
    title tea
    description tea
    version tea
    build_date tea
    generator tea
}

# Create new documentation generator
slay new_doc_generator(source_dir tea, output_dir tea) DocumentationGenerator {
    sus config DocConfig = DocConfig{
        title: "CURSED Documentation",
        description: "Comprehensive documentation for CURSED programming language",
        version: "1.0.0",
        author: "CURSED Team",
        theme: "default",
        output_formats: []tea{"html", "markdown", "pdf"},
        include_patterns: []tea{"**/*.md", "**/*.csd"},
        exclude_patterns: []tea{"**/.*", "**/node_modules/**"},
        auto_generate_toc: based,
        enable_search: based,
        enable_pdf_export: based,
    }
    
    damn DocumentationGenerator{
        source_dir: source_dir,
        output_dir: output_dir,
        config: config,
        templates: make(map[tea]tea),
        plugins: make([]DocPlugin, 0),
    }
}

# Generate complete documentation site
slay (gen *DocumentationGenerator) generate_site() DocumentationSite {
    vibez.spill("Generating documentation site...")
    
    # Load templates
    gen.load_templates()
    
    # Discover source files
    sus source_files []tea = gen.discover_source_files()
    
    # Process files into pages
    sus pages []DocPage = gen.process_files(source_files)
    
    # Generate navigation
    sus navigation Navigation = gen.generate_navigation(pages)
    
    # Create search index
    sus search_index SearchIndex = gen.create_search_index(pages)
    
    # Create site metadata
    sus metadata SiteMetadata = gen.create_site_metadata()
    
    sus site DocumentationSite = DocumentationSite{
        pages: pages,
        navigation: navigation,
        search_index: search_index,
        metadata: metadata,
    }
    
    # Generate output files
    gen.generate_output_files(site)
    
    vibez.spill("Documentation site generated successfully")
    damn site
}

# Load HTML templates
slay (gen *DocumentationGenerator) load_templates() {
    # Load default templates
    gen.templates["page"] = gen.load_page_template()
    gen.templates["index"] = gen.load_index_template()
    gen.templates["navigation"] = gen.load_navigation_template()
    gen.templates["search"] = gen.load_search_template()
    gen.templates["api"] = gen.load_api_template()
    gen.templates["tutorial"] = gen.load_tutorial_template()
}

# Discover source files
slay (gen *DocumentationGenerator) discover_source_files() []tea {
    sus files []tea = make([]tea, 0)
    
    # Walk through source directory
    sus all_files []tea = pathing.walk_directory(gen.source_dir)
    
    bestie file <- all_files {
        lowkey gen.should_include_file(file) {
            files = append(files, file)
        }
    }
    
    damn files
}

# Check if file should be included
slay (gen *DocumentationGenerator) should_include_file(file tea) lit {
    # Check include patterns
    bestie pattern <- gen.config.include_patterns {
        lowkey pathing.match_pattern(file, pattern) {
            # Check exclude patterns
            bestie exclude_pattern <- gen.config.exclude_patterns {
                lowkey pathing.match_pattern(file, exclude_pattern) {
                    damn cap
                }
            }
            damn based
        }
    }
    damn cap
}

# Process files into documentation pages
slay (gen *DocumentationGenerator) process_files(files []tea) []DocPage {
    sus pages []DocPage = make([]DocPage, 0)
    
    bestie file <- files {
        sus page DocPage = gen.process_file(file)
        pages = append(pages, page)
    }
    
    damn pages
}

# Process individual file
slay (gen *DocumentationGenerator) process_file(file tea) DocPage {
    sus content tea = dropz.read_file(file)
    sus relative_path tea = pathing.relative_path(gen.source_dir, file)
    
    # Extract metadata
    sus metadata PageMetadata = gen.extract_metadata(content)
    
    # Process content through plugins
    bestie plugin <- gen.plugins {
        lowkey plugin.enabled {
            content = plugin.processor(content)
        }
    }
    
    # Generate table of contents
    sus toc []TocEntry = gen.generate_toc(content)
    
    # Extract title
    sus title tea = gen.extract_title(content)
    
    damn DocPage{
        title: title,
        path: relative_path,
        content: content,
        metadata: metadata,
        toc: toc,
    }
}

# Extract metadata from content
slay (gen *DocumentationGenerator) extract_metadata(content tea) PageMetadata {
    # Parse frontmatter if present
    sus metadata PageMetadata = PageMetadata{
        author: gen.config.author,
        date: get_current_date(),
        tags: make([]tea, 0),
        category: "general",
        description: "",
    }
    
    # Extract from YAML frontmatter or comments
    lowkey stringz.has_prefix(content, "---") {
        sus frontmatter tea = gen.extract_frontmatter(content)
        metadata = gen.parse_frontmatter(frontmatter)
    }
    
    damn metadata
}

# Generate table of contents
slay (gen *DocumentationGenerator) generate_toc(content tea) []TocEntry {
    sus toc []TocEntry = make([]TocEntry, 0)
    
    # Parse markdown headers
    sus lines []tea = stringz.split(content, "\n")
    
    bestie line <- lines {
        lowkey stringz.has_prefix(line, "#") {
            sus entry TocEntry = gen.parse_header(line)
            toc = append(toc, entry)
        }
    }
    
    damn toc
}

# Parse markdown header
slay (gen *DocumentationGenerator) parse_header(line tea) TocEntry {
    sus level normie = 0
    sus title tea = line
    
    # Count # characters
    bestie i := 0; i < len(line); i++ {
        lowkey line[i] == '#' {
            level++
        } highkey {
            title = stringz.trim(line[i:])
            ghosted
        }
    }
    
    # Generate anchor
    sus anchor tea = gen.generate_anchor(title)
    
    damn TocEntry{
        title: title,
        anchor: anchor,
        level: level,
        children: make([]TocEntry, 0),
    }
}

# Generate navigation structure
slay (gen *DocumentationGenerator) generate_navigation(pages []DocPage) Navigation {
    sus sections []NavSection = make([]NavSection, 0)
    
    # Group pages by category
    sus categories map[tea][]DocPage = make(map[tea][]DocPage)
    
    bestie page <- pages {
        sus category tea = page.metadata.category
        lowkey categories[category] == cringe {
            categories[category] = make([]DocPage, 0)
        }
        categories[category] = append(categories[category], page)
    }
    
    # Create navigation sections
    bestie category, category_pages <- categories {
        sus section NavSection = NavSection{
            title: stringz.title_case(category),
            items: make([]NavItem, 0),
        }
        
        bestie page <- category_pages {
            sus item NavItem = NavItem{
                title: page.title,
                url: page.path,
                icon: gen.get_page_icon(page),
                children: make([]NavItem, 0),
            }
            section.items = append(section.items, item)
        }
        
        sections = append(sections, section)
    }
    
    damn Navigation{
        sections: sections,
        breadcrumbs: make([]Breadcrumb, 0),
    }
}

# Create search index
slay (gen *DocumentationGenerator) create_search_index(pages []DocPage) SearchIndex {
    sus documents []SearchDocument = make([]SearchDocument, 0)
    sus index map[tea][]normie = make(map[tea][]normie)
    
    bestie i, page <- pages {
        sus doc SearchDocument = SearchDocument{
            id: i,
            title: page.title,
            content: gen.extract_text_content(page.content),
            url: page.path,
            keywords: gen.extract_keywords(page.content),
        }
        
        documents = append(documents, doc)
        
        # Index keywords
        bestie keyword <- doc.keywords {
            lowkey index[keyword] == cringe {
                index[keyword] = make([]normie, 0)
            }
            index[keyword] = append(index[keyword], i)
        }
    }
    
    damn SearchIndex{
        documents: documents,
        index: index,
    }
}

# Generate output files
slay (gen *DocumentationGenerator) generate_output_files(site DocumentationSite) {
    # Create output directory
    pathing.create_directory(gen.output_dir)
    
    # Generate HTML files
    lowkey gen.should_generate_format("html") {
        gen.generate_html_files(site)
    }
    
    # Generate markdown files
    lowkey gen.should_generate_format("markdown") {
        gen.generate_markdown_files(site)
    }
    
    # Generate PDF files
    lowkey gen.should_generate_format("pdf") {
        gen.generate_pdf_files(site)
    }
    
    # Copy static assets
    gen.copy_static_assets()
}

# Generate HTML files
slay (gen *DocumentationGenerator) generate_html_files(site DocumentationSite) {
    sus html_dir tea = gen.output_dir + "/html"
    pathing.create_directory(html_dir)
    
    # Generate index page
    sus index_html tea = gen.render_template("index", site)
    dropz.write_file(html_dir + "/index.html", index_html)
    
    # Generate individual pages
    bestie page <- site.pages {
        sus page_html tea = gen.render_page_template(page, site)
        sus filename tea = html_dir + "/" + pathing.change_extension(page.path, ".html")
        dropz.write_file(filename, page_html)
    }
    
    # Generate navigation
    sus nav_html tea = gen.render_template("navigation", site)
    dropz.write_file(html_dir + "/navigation.html", nav_html)
    
    # Generate search page
    sus search_html tea = gen.render_template("search", site)
    dropz.write_file(html_dir + "/search.html", search_html)
    
    # Generate search index JSON
    sus search_json tea = json_tea.marshal(site.search_index)
    dropz.write_file(html_dir + "/search_index.json", search_json)
}

# Render template with data
slay (gen *DocumentationGenerator) render_template(template_name tea, data interface{}) tea {
    sus template tea = gen.templates[template_name]
    
    # Simple template rendering (in production, use proper template engine)
    sus rendered tea = template
    
    # Replace placeholders
    rendered = stringz.replace(rendered, "{{.Title}}", gen.config.title)
    rendered = stringz.replace(rendered, "{{.Description}}", gen.config.description)
    rendered = stringz.replace(rendered, "{{.Version}}", gen.config.version)
    
    damn rendered
}

# Plugin system
slay (gen *DocumentationGenerator) add_plugin(plugin DocPlugin) {
    gen.plugins = append(gen.plugins, plugin)
}

# Built-in plugins
slay create_syntax_highlighter_plugin() DocPlugin {
    damn DocPlugin{
        name: "syntax_highlighter",
        version: "1.0.0",
        processor: slay(content tea) tea {
            # Add syntax highlighting to code blocks
            damn gen.highlight_code_blocks(content)
        },
        enabled: based,
    }
}

slay create_link_checker_plugin() DocPlugin {
    damn DocPlugin{
        name: "link_checker",
        version: "1.0.0",
        processor: slay(content tea) tea {
            # Check for broken links
            gen.check_links(content)
            damn content
        },
        enabled: based,
    }
}

slay create_api_doc_plugin() DocPlugin {
    damn DocPlugin{
        name: "api_doc_generator",
        version: "1.0.0",
        processor: slay(content tea) tea {
            # Generate API documentation from source code
            damn gen.generate_api_docs(content)
        },
        enabled: based,
    }
}

# Template definitions
slay (gen *DocumentationGenerator) load_page_template() tea {
    damn `
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{.Title}} - CURSED Documentation</title>
    <link rel="stylesheet" href="styles.css">
</head>
<body>
    <nav class="sidebar">
        {{.Navigation}}
    </nav>
    <main class="content">
        <article>
            {{.Content}}
        </article>
    </main>
    <script src="search.js"></script>
</body>
</html>
    `
}

slay (gen *DocumentationGenerator) load_index_template() tea {
    damn `
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{.Title}}</title>
    <link rel="stylesheet" href="styles.css">
</head>
<body>
    <header class="hero">
        <h1>{{.Title}}</h1>
        <p>{{.Description}}</p>
    </header>
    <main class="index-content">
        {{.IndexContent}}
    </main>
</body>
</html>
    `
}

# Helper functions
slay get_current_date() tea {
    damn "2023-12-01"
}

slay (gen *DocumentationGenerator) extract_title(content tea) tea {
    sus lines []tea = stringz.split(content, "\n")
    bestie line <- lines {
        lowkey stringz.has_prefix(line, "# ") {
            damn stringz.trim(line[2:])
        }
    }
    damn "Untitled"
}

slay (gen *DocumentationGenerator) generate_anchor(title tea) tea {
    # Convert title to URL-friendly anchor
    sus anchor tea = stringz.to_lower(title)
    anchor = stringz.replace(anchor, " ", "-")
    anchor = stringz.replace(anchor, ".", "")
    damn anchor
}

slay (gen *DocumentationGenerator) get_page_icon(page DocPage) tea {
    lowkey stringz.contains(page.path, "tutorial") {
        damn "📚"
    }
    lowkey stringz.contains(page.path, "api") {
        damn "🔧"
    }
    lowkey stringz.contains(page.path, "example") {
        damn "💡"
    }
    damn "📄"
}

slay (gen *DocumentationGenerator) should_generate_format(format tea) lit {
    bestie supported_format <- gen.config.output_formats {
        lowkey supported_format == format {
            damn based
        }
    }
    damn cap
}

# Testing
slay test_doc_generator() {
    test_start("Documentation Generator Tests")
    
    sus generator DocumentationGenerator = new_doc_generator("test_docs", "output")
    
    # Test configuration
    assert_eq_string(generator.config.title, "CURSED Documentation")
    assert_true(generator.config.enable_search)
    
    # Test plugin system
    sus plugin DocPlugin = create_syntax_highlighter_plugin()
    generator.add_plugin(plugin)
    assert_eq_int(len(generator.plugins), 1)
    
    # Test template loading
    generator.load_templates()
    assert_true(len(generator.templates) > 0)
    
    vibez.spill("Documentation generator tests completed")
    print_test_summary()
}

# Main function
slay main() {
    vibez.spill("CURSED Documentation Generator")
    vibez.spill("==============================")
    
    # Run tests
    test_doc_generator()
    
    # Generate documentation
    sus generator DocumentationGenerator = new_doc_generator("docs", "output")
    
    # Add plugins
    generator.add_plugin(create_syntax_highlighter_plugin())
    generator.add_plugin(create_link_checker_plugin())
    generator.add_plugin(create_api_doc_plugin())
    
    # Generate site
    sus site DocumentationSite = generator.generate_site()
    
    vibez.spill("Documentation generated:")
    vibez.spill("  Pages: " + len(site.pages).(tea))
    vibez.spill("  Navigation sections: " + len(site.navigation.sections).(tea))
    vibez.spill("  Search documents: " + len(site.search_index.documents).(tea))
    
    vibez.spill("Output directory: output/")
}

# Run the generator
main()
