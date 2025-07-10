# Advanced CURSED Standard Library Modules

This document describes the advanced stdlib modules implemented in pure CURSED language without FFI dependencies.

## 1. Image Processing Module (`stdlib/image_processing/`)

### Overview
Comprehensive image processing capabilities supporting JPEG, PNG, and GIF formats.

### Key Features
- **Image Loading/Saving**: Load and save images in multiple formats
- **Image Manipulation**: Resize, crop, rotate, flip operations
- **Color Adjustments**: Brightness, contrast, saturation control
- **Filters**: Blur, sharpen, edge detection, emboss
- **Format Conversion**: Convert between image formats
- **Batch Processing**: Process multiple images simultaneously
- **Metadata Handling**: Read and write image metadata
- **Compression**: Image compression with quality control

### Usage Example
```cursed
yeet "image_processing"

// Load an image
image_load("photo.jpg")

// Apply transformations
image_resize(800, 600)
image_adjust_brightness(1.2)
image_apply_blur(3)

// Save processed image
image_save("processed_photo.jpg", 85)
```

### Testing
```bash
cargo run --bin cursed stdlib/image_processing/test_image_processing.csd
```

## 2. Template Engine Module (`stdlib/template_engine/`)

### Overview
Full-featured template engine supporting HTML and text templating with variable substitution, blocks, and filters.

### Key Features
- **Template Loading**: Load templates from files or strings
- **Variable Substitution**: Replace variables with values
- **Block Processing**: Handle conditional blocks and loops
- **HTML Operations**: HTML escaping, unescaping, tag stripping
- **Template Inheritance**: Extend and include templates
- **Macro System**: Define and use template macros
- **Filters**: Apply filters to template variables
- **Caching**: Template compilation and caching
- **Validation**: Template syntax validation

### Usage Example
```cursed
yeet "template_engine"

// Load template
template_load_string("<h1>{{title}}</h1><p>{{content}}</p>")

// Set variables
template_set_var("title", "My Page")
template_set_var("content", "Welcome to my website!")

// Render template
sus output tea = template_render()
vibez.spill(output)
```

### Testing
```bash
cargo run --bin cursed stdlib/template_engine/test_template_engine.csd
```

## 3. Archive Handling Module (`stdlib/archive_handling/`)

### Overview
Complete archive management supporting TAR, ZIP, GZIP, and BZIP2 formats.

### Key Features
- **Archive Creation**: Create new archives in multiple formats
- **File Management**: Add, remove, extract files from archives
- **Archive Information**: List contents, get file sizes, metadata
- **Compression**: Configurable compression levels
- **Validation**: Archive integrity testing and repair
- **Password Protection**: Encrypt/decrypt archives
- **Format Conversion**: Convert between archive formats
- **Batch Operations**: Process multiple archives simultaneously
- **Split/Merge**: Handle large archives in parts

### Usage Example
```cursed
yeet "archive_handling"

// Create archive
archive_create("backup.zip", "zip")

// Add files
archive_add_file("document.txt", "docs/document.txt")
archive_add_directory("images/", "backup_images/")

// Set compression
archive_set_compression_level(6)

// Extract to directory
archive_extract_all("restore/")
```

### Testing
```bash
cargo run --bin cursed stdlib/archive_handling/test_archive_handling.csd
```

## 4. URL Parsing Module (`stdlib/url_parsing/`)

### Overview
Comprehensive URL parsing and manipulation library supporting all URL components.

### Key Features
- **URL Parsing**: Parse URLs into components (scheme, host, path, query, fragment)
- **Component Access**: Get and set individual URL components
- **URL Building**: Construct URLs from components
- **Query Parameters**: Manage URL query parameters
- **URL Validation**: Validate URL syntax and format
- **URL Manipulation**: Resolve relative URLs, normalize URLs
- **Encoding/Decoding**: URL encoding and decoding functions
- **Comparison**: Compare URLs and check same origin
- **Utility Functions**: Extract domain, subdomain, file extension

### Usage Example
```cursed
yeet "url_parsing"

// Parse URL
url_parse("https://example.com:8080/path/to/resource?param=value#section")

// Access components
vibez.spill("Host: " + url_get_host())
vibez.spill("Path: " + url_get_path())
vibez.spill("Query: " + url_get_query())

// Modify and rebuild
url_set_port(9000)
url_add_query_param("new_param", "new_value")
sus modified_url tea = url_build()
```

### Testing
```bash
cargo run --bin cursed stdlib/url_parsing/test_url_parsing.csd
```

## 5. Command Line Module (`stdlib/command_line/`)

### Overview
Command-line argument parsing and flag management system.

### Key Features
- **Flag Parsing**: Parse command-line flags and options
- **Argument Types**: Support for boolean, string, and integer flags
- **Validation**: Validate required flags and arguments
- **Help Generation**: Automatic help text generation
- **Subcommands**: Support for subcommand parsing
- **Positional Arguments**: Handle non-flag arguments
- **Configuration**: Flexible flag configuration system

### Usage Example
```cursed
yeet "command_line"

// Initialize CLI parser
cli_init("myapp", "--verbose --input data.txt --output result.txt")

// Parse arguments
cli_parse()

// Check flags
bestie cli_has_flag("verbose") {
    vibez.spill("Verbose mode enabled")
}

// Get flag values
sus input_file tea = cli_get_flag_value("input")
sus output_file tea = cli_get_flag_value("output")
```

### Testing
```bash
cargo run --bin cursed stdlib/command_line/test_command_line.csd
```

## 6. Reflection Module (`stdlib/reflection/`)

### Overview
Runtime reflection system providing type information and dynamic operations.

### Key Features
- **Type Inspection**: Get type information at runtime
- **Field Reflection**: Access and modify struct fields dynamically
- **Method Reflection**: Call methods dynamically
- **Interface Checking**: Check interface implementations
- **Type Conversion**: Convert between types dynamically
- **Dynamic Creation**: Create instances at runtime
- **Metadata Access**: Access struct tags and metadata
- **Package Information**: Get module and package information

### Usage Example
```cursed
yeet "reflection"

// Load type information
reflect_load_type("MyStruct")

// Get field information
sus field_names tea = reflect_get_field_names()
sus field_count normie = reflect_get_field_count()

// Get method information
sus method_names tea = reflect_get_method_names()

// Call method dynamically
sus result tea = reflect_call_method("processData", "arg1,arg2")
```

### Testing
```bash
cargo run --bin cursed stdlib/reflection/test_reflection.csd
```

## 7. Plugin System Module (`stdlib/plugin_system/`)

### Overview
Dynamic plugin system for loading and managing plugins at runtime.

### Key Features
- **Plugin Registration**: Register and manage plugins
- **Dynamic Loading**: Load and unload plugins at runtime
- **Plugin Communication**: Inter-plugin communication system
- **Event System**: Plugin event handling and triggering
- **Dependency Management**: Handle plugin dependencies
- **Configuration**: Plugin configuration management
- **Security**: Plugin sandboxing and permission checking
- **Statistics**: Plugin usage and performance statistics

### Usage Example
```cursed
yeet "plugin_system"

// Initialize plugin system
plugin_system_init()

// Register plugin
plugin_register("auth_plugin", "plugins/auth.csd")

// Load plugin
plugin_load("auth_plugin")

// Call plugin function
sus result tea = plugin_call_function("auth_plugin", "authenticate", "user,pass")

// Send message to plugin
plugin_send_message("auth_plugin", "user_logged_in")
```

### Testing
```bash
cargo run --bin cursed stdlib/plugin_system/test_plugin_system.csd
```

## Testing All Advanced Modules

To test all advanced modules simultaneously:

```bash
# Test individual modules
cargo run --bin cursed stdlib/image_processing/test_image_processing.csd
cargo run --bin cursed stdlib/template_engine/test_template_engine.csd
cargo run --bin cursed stdlib/archive_handling/test_archive_handling.csd
cargo run --bin cursed stdlib/url_parsing/test_url_parsing.csd
cargo run --bin cursed stdlib/command_line/test_command_line.csd
cargo run --bin cursed stdlib/reflection/test_reflection.csd
cargo run --bin cursed stdlib/plugin_system/test_plugin_system.csd

# Test all stdlib modules
cargo run --bin cursed test --test-dir stdlib --parallel
```

## Implementation Notes

### Pure CURSED Implementation
- All modules are implemented in pure CURSED language
- No FFI dependencies or external libraries required
- Full compatibility with both interpretation and compilation modes
- Consistent error handling and validation

### Performance Considerations
- Efficient string manipulation and parsing
- Minimal memory allocation and cleanup
- Optimized for common use cases
- Suitable for production deployment

### Security Features
- Input validation and sanitization
- Safe string handling and manipulation
- Protection against common vulnerabilities
- Proper error handling and recovery

### Extensibility
- Modular design for easy extension
- Well-defined interfaces and contracts
- Plugin-based architecture where applicable
- Comprehensive documentation and examples

## Future Enhancements

### Planned Features
1. **Database ORM**: Enhanced database abstraction layer
2. **Web Framework**: Complete web application framework
3. **Machine Learning**: Basic ML algorithms and utilities
4. **Graphics**: 2D/3D graphics and rendering
5. **Audio Processing**: Audio file manipulation and processing
6. **Network Protocols**: Additional network protocol support
7. **Cryptographic**: Advanced cryptographic algorithms
8. **Compression**: Additional compression algorithms

### Performance Improvements
- Native implementations for critical paths
- Memory pool allocation for high-frequency operations
- Parallel processing for batch operations
- Caching mechanisms for repeated operations

### Integration Features
- IDE integration and debugging support
- Build system integration
- Package manager integration
- Testing framework enhancements

This advanced stdlib provides a comprehensive foundation for building sophisticated applications in the CURSED programming language, covering essential functionality from image processing to plugin systems while maintaining the language's design principles and performance characteristics.
