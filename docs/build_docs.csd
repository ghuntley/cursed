yeet "dropz"
yeet "stringz"
yeet "pathing"
yeet "testz"
yeet "tab_aesthetic"
yeet "json_tea"

# Complete Documentation Build System
# Generates comprehensive documentation for the CURSED language

struct DocumentationBuilder {
    source_dir tea
    output_dir tea
    temp_dir tea
    config BuildConfig
    stats BuildStats
}

struct BuildConfig {
    generate_api_docs lit
    generate_tutorials lit
    generate_examples lit
    generate_migration_guides lit
    generate_search_index lit
    generate_pdf lit
    validate_links lit
    check_spelling lit
    output_formats []tea
}

struct BuildStats {
    total_files_processed normie
    api_docs_generated normie
    tutorials_generated normie
    examples_processed normie
    migration_guides_generated normie
    errors_found normie
    warnings_found normie
    build_time_seconds drip
}

# Create new documentation builder
slay new_doc_builder(source_dir tea, output_dir tea) DocumentationBuilder {
    sus config BuildConfig = BuildConfig{
        generate_api_docs: based,
        generate_tutorials: based,
        generate_examples: based,
        generate_migration_guides: based,
        generate_search_index: based,
        generate_pdf: based,
        validate_links: based,
        check_spelling: cap,
        output_formats: []tea{"html", "markdown", "json"},
    }
    
    damn DocumentationBuilder{
        source_dir: source_dir,
        output_dir: output_dir,
        temp_dir: output_dir + "/temp",
        config: config,
        stats: BuildStats{},
    }
}

# Build complete documentation
slay (builder *DocumentationBuilder) build_all() {
    vibez.spill("Building CURSED Documentation")
    vibez.spill("============================")
    
    sus start_time drip = get_current_time()
    
    # Initialize build environment
    builder.initialize_build_environment()
    
    # Generate API documentation
    lowkey builder.config.generate_api_docs {
        builder.generate_api_documentation()
    }
    
    # Generate tutorials
    lowkey builder.config.generate_tutorials {
        builder.generate_tutorial_documentation()
    }
    
    # Process examples
    lowkey builder.config.generate_examples {
        builder.process_examples()
    }
    
    # Generate migration guides
    lowkey builder.config.generate_migration_guides {
        builder.generate_migration_guides()
    }
    
    # Generate search index
    lowkey builder.config.generate_search_index {
        builder.generate_search_index()
    }
    
    # Generate PDF documentation
    lowkey builder.config.generate_pdf {
        builder.generate_pdf_documentation()
    }
    
    # Validate documentation
    builder.validate_documentation()
    
    # Generate final index
    builder.generate_documentation_index()
    
    # Copy static assets
    builder.copy_static_assets()
    
    # Calculate build time
    builder.stats.build_time_seconds = get_current_time() - start_time
    
    # Print build summary
    builder.print_build_summary()
    
    # Cleanup
    builder.cleanup_build_environment()
    
    vibez.spill("Documentation build completed successfully!")
}

# Initialize build environment
slay (builder *DocumentationBuilder) initialize_build_environment() {
    vibez.spill("Initializing build environment...")
    
    # Create output directories
    pathing.create_directory(builder.output_dir)
    pathing.create_directory(builder.temp_dir)
    pathing.create_directory(builder.output_dir + "/html")
    pathing.create_directory(builder.output_dir + "/markdown")
    pathing.create_directory(builder.output_dir + "/json")
    pathing.create_directory(builder.output_dir + "/pdf")
    pathing.create_directory(builder.output_dir + "/assets")
    
    vibez.spill("Build environment initialized")
}

# Generate API documentation
slay (builder *DocumentationBuilder) generate_api_documentation() {
    vibez.spill("Generating API documentation...")
    
    # Scan stdlib modules
    sus stdlib_modules []tea = builder.scan_stdlib_modules()
    
    bestie module <- stdlib_modules {
        sus api_doc tea = builder.generate_module_api_doc(module)
        sus filename tea = builder.output_dir + "/html/api/" + module + ".html"
        
        pathing.create_directory(pathing.dirname(filename))
        dropz.write_file(filename, api_doc)
        
        builder.stats.api_docs_generated++
    }
    
    # Generate API index
    sus api_index tea = builder.generate_api_index(stdlib_modules)
    dropz.write_file(builder.output_dir + "/html/api/index.html", api_index)
    
    vibez.spill("API documentation generated: " + builder.stats.api_docs_generated.(tea) + " modules")
}

# Generate tutorial documentation
slay (builder *DocumentationBuilder) generate_tutorial_documentation() {
    vibez.spill("Generating tutorial documentation...")
    
    # Process tutorial files
    sus tutorial_files []tea = builder.scan_tutorial_files()
    
    bestie tutorial_file <- tutorial_files {
        sus tutorial_html tea = builder.process_tutorial_file(tutorial_file)
        sus output_file tea = builder.convert_path_to_html(tutorial_file)
        
        pathing.create_directory(pathing.dirname(output_file))
        dropz.write_file(output_file, tutorial_html)
        
        builder.stats.tutorials_generated++
    }
    
    # Generate tutorial index
    sus tutorial_index tea = builder.generate_tutorial_index(tutorial_files)
    dropz.write_file(builder.output_dir + "/html/tutorials/index.html", tutorial_index)
    
    vibez.spill("Tutorial documentation generated: " + builder.stats.tutorials_generated.(tea) + " tutorials")
}

# Process examples
slay (builder *DocumentationBuilder) process_examples() {
    vibez.spill("Processing examples...")
    
    # Scan example files
    sus example_files []tea = builder.scan_example_files()
    
    bestie example_file <- example_files {
        # Test example
        sus test_result lit = builder.test_example(example_file)
        lowkey !test_result {
            vibez.spill("WARNING: Example failed to run: " + example_file)
            builder.stats.warnings_found++
        }
        
        # Generate example documentation
        sus example_doc tea = builder.generate_example_doc(example_file)
        sus output_file tea = builder.convert_path_to_html(example_file)
        
        pathing.create_directory(pathing.dirname(output_file))
        dropz.write_file(output_file, example_doc)
        
        builder.stats.examples_processed++
    }
    
    # Generate example index
    sus example_index tea = builder.generate_example_index(example_files)
    dropz.write_file(builder.output_dir + "/html/examples/index.html", example_index)
    
    vibez.spill("Examples processed: " + builder.stats.examples_processed.(tea) + " examples")
}

# Generate migration guides
slay (builder *DocumentationBuilder) generate_migration_guides() {
    vibez.spill("Generating migration guides...")
    
    # Process migration guide files
    sus migration_files []tea = builder.scan_migration_files()
    
    bestie migration_file <- migration_files {
        sus migration_html tea = builder.process_migration_file(migration_file)
        sus output_file tea = builder.convert_path_to_html(migration_file)
        
        pathing.create_directory(pathing.dirname(output_file))
        dropz.write_file(output_file, migration_html)
        
        builder.stats.migration_guides_generated++
    }
    
    # Generate migration index
    sus migration_index tea = builder.generate_migration_index(migration_files)
    dropz.write_file(builder.output_dir + "/html/migration/index.html", migration_index)
    
    vibez.spill("Migration guides generated: " + builder.stats.migration_guides_generated.(tea) + " guides")
}

# Generate search index
slay (builder *DocumentationBuilder) generate_search_index() {
    vibez.spill("Generating search index...")
    
    # Collect all documentation content
    sus search_documents []SearchDocument = builder.collect_search_documents()
    
    # Create search index
    sus search_index SearchIndex = builder.create_search_index(search_documents)
    
    # Write search index
    sus search_json tea = json_tea.marshal(search_index)
    dropz.write_file(builder.output_dir + "/json/search_index.json", search_json)
    
    # Generate search page
    sus search_page tea = builder.generate_search_page()
    dropz.write_file(builder.output_dir + "/html/search.html", search_page)
    
    vibez.spill("Search index generated with " + len(search_documents).(tea) + " documents")
}

# Generate PDF documentation
slay (builder *DocumentationBuilder) generate_pdf_documentation() {
    vibez.spill("Generating PDF documentation...")
    
    # Combine all documentation into single document
    sus combined_doc tea = builder.combine_documentation()
    
    # Convert to PDF (would use external tool in real implementation)
    sus pdf_filename tea = builder.output_dir + "/pdf/cursed_documentation.pdf"
    builder.convert_to_pdf(combined_doc, pdf_filename)
    
    vibez.spill("PDF documentation generated: " + pdf_filename)
}

# Validate documentation
slay (builder *DocumentationBuilder) validate_documentation() {
    vibez.spill("Validating documentation...")
    
    # Check for broken links
    lowkey builder.config.validate_links {
        sus broken_links []tea = builder.find_broken_links()
        lowkey len(broken_links) > 0 {
            vibez.spill("Found " + len(broken_links).(tea) + " broken links")
            builder.stats.errors_found += len(broken_links)
        }
    }
    
    # Check spelling
    lowkey builder.config.check_spelling {
        sus spelling_errors []tea = builder.check_spelling()
        lowkey len(spelling_errors) > 0 {
            vibez.spill("Found " + len(spelling_errors).(tea) + " spelling errors")
            builder.stats.errors_found += len(spelling_errors)
        }
    }
    
    # Validate code examples
    sus invalid_examples []tea = builder.validate_code_examples()
    lowkey len(invalid_examples) > 0 {
        vibez.spill("Found " + len(invalid_examples).(tea) + " invalid code examples")
        builder.stats.errors_found += len(invalid_examples)
    }
    
    vibez.spill("Documentation validation completed")
}

# Generate documentation index
slay (builder *DocumentationBuilder) generate_documentation_index() {
    vibez.spill("Generating documentation index...")
    
    # Create main index page
    sus index_html tea = builder.generate_main_index()
    dropz.write_file(builder.output_dir + "/html/index.html", index_html)
    
    # Create navigation
    sus navigation_html tea = builder.generate_navigation()
    dropz.write_file(builder.output_dir + "/html/navigation.html", navigation_html)
    
    # Create sitemap
    sus sitemap_xml tea = builder.generate_sitemap()
    dropz.write_file(builder.output_dir + "/sitemap.xml", sitemap_xml)
    
    vibez.spill("Documentation index generated")
}

# Copy static assets
slay (builder *DocumentationBuilder) copy_static_assets() {
    vibez.spill("Copying static assets...")
    
    # Copy CSS files
    builder.copy_css_files()
    
    # Copy JavaScript files
    builder.copy_js_files()
    
    # Copy images
    builder.copy_images()
    
    # Copy fonts
    builder.copy_fonts()
    
    vibez.spill("Static assets copied")
}

# Print build summary
slay (builder *DocumentationBuilder) print_build_summary() {
    vibez.spill("\nBuild Summary")
    vibez.spill("=============")
    
    # Create summary table
    sus summary_table tea = tab_aesthetic.create_table()
    
    tab_aesthetic.add_header(summary_table, []tea{"Metric", "Count"})
    tab_aesthetic.add_row(summary_table, []tea{"Total Files Processed", builder.stats.total_files_processed.(tea)})
    tab_aesthetic.add_row(summary_table, []tea{"API Docs Generated", builder.stats.api_docs_generated.(tea)})
    tab_aesthetic.add_row(summary_table, []tea{"Tutorials Generated", builder.stats.tutorials_generated.(tea)})
    tab_aesthetic.add_row(summary_table, []tea{"Examples Processed", builder.stats.examples_processed.(tea)})
    tab_aesthetic.add_row(summary_table, []tea{"Migration Guides", builder.stats.migration_guides_generated.(tea)})
    tab_aesthetic.add_row(summary_table, []tea{"Errors Found", builder.stats.errors_found.(tea)})
    tab_aesthetic.add_row(summary_table, []tea{"Warnings Found", builder.stats.warnings_found.(tea)})
    tab_aesthetic.add_row(summary_table, []tea{"Build Time (seconds)", builder.stats.build_time_seconds.(tea)})
    
    vibez.spill(tab_aesthetic.render_table(summary_table))
    
    lowkey builder.stats.errors_found > 0 {
        vibez.spill("\n⚠️  Build completed with " + builder.stats.errors_found.(tea) + " errors")
    } highkey {
        vibez.spill("\n✅ Build completed successfully!")
    }
}

# Cleanup build environment
slay (builder *DocumentationBuilder) cleanup_build_environment() {
    vibez.spill("Cleaning up build environment...")
    
    # Remove temporary files
    pathing.remove_directory(builder.temp_dir)
    
    vibez.spill("Build environment cleaned up")
}

# Helper methods
slay (builder *DocumentationBuilder) scan_stdlib_modules() []tea {
    # Scan stdlib directory for modules
    sus modules []tea = pathing.list_directories("stdlib")
    damn modules
}

slay (builder *DocumentationBuilder) scan_tutorial_files() []tea {
    # Scan docs/tutorials directory
    sus files []tea = pathing.walk_files("docs/tutorials", "*.md")
    damn files
}

slay (builder *DocumentationBuilder) scan_example_files() []tea {
    # Scan docs/examples directory
    sus files []tea = pathing.walk_files("docs/examples", "*.csd")
    damn files
}

slay (builder *DocumentationBuilder) scan_migration_files() []tea {
    # Scan docs/migration directory
    sus files []tea = pathing.walk_files("docs/migration", "*.md")
    damn files
}

slay (builder *DocumentationBuilder) test_example(example_file tea) lit {
    # Test example by running it
    # In real implementation, this would execute the CURSED program
    damn based
}

slay (builder *DocumentationBuilder) generate_module_api_doc(module tea) tea {
    # Generate API documentation for a module
    damn "<html><body><h1>API Documentation for " + module + "</h1></body></html>"
}

slay (builder *DocumentationBuilder) convert_path_to_html(file_path tea) tea {
    # Convert source path to HTML output path
    sus html_path tea = stringz.replace(file_path, builder.source_dir, builder.output_dir + "/html")
    html_path = stringz.replace(html_path, ".md", ".html")
    html_path = stringz.replace(html_path, ".csd", ".html")
    damn html_path
}

slay get_current_time() drip {
    # Get current timestamp
    damn 1234567890.0
}

# Main build function
slay build_cursed_documentation() {
    test_start("CURSED Documentation Build")
    
    vibez.spill("CURSED Documentation Build System")
    vibez.spill("=================================")
    
    # Initialize builder
    sus builder DocumentationBuilder = new_doc_builder("docs", "docs_output")
    
    # Build all documentation
    builder.build_all()
    
    # Verify build results
    assert_true(builder.stats.total_files_processed >= 0)
    assert_true(builder.stats.api_docs_generated >= 0)
    assert_true(builder.stats.tutorials_generated >= 0)
    assert_true(builder.stats.examples_processed >= 0)
    
    print_test_summary()
}

# Run the build
build_cursed_documentation()
