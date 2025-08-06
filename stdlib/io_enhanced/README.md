# io_enhanced

Enhanced I/O library optimized for CURSED compiler operations including source file handling, code generation, module resolution, and build system integration. Features compiler-specific optimizations and developer tooling.

## Overview

The `io_enhanced` module provides:
- Source file reading with error recovery and line tracking
- Code generation with indentation management
- Module resolution and dependency handling
- Compiler output management and file operations
- Build system integration and configuration parsing
- Enhanced string operations and utility functions

## Core Components

### Source File Management

#### `SourceFile`
Comprehensive source file representation with metadata.

```cursed
squad SourceFile {
    spill path tea
    spill content tea
    spill lines []tea
    spill line_count normie
    spill encoding tea
}
```

#### Source File Operations

#### `SourceFile_read(file_path: tea) -> (SourceFile, tea)`
Reads source file with complete metadata extraction.

**Parameters:**
- `file_path`: Path to source file

**Returns:** Tuple of (SourceFile, error_message)

**Features:**
- Automatic line splitting for compiler use
- UTF-8 encoding detection
- Line count tracking for error reporting
- Complete content preservation

#### `SourceFile_get_line(source: SourceFile, line_number: normie) -> tea`
Retrieves specific line by number (1-indexed).

#### `SourceFile_get_line_range(source: SourceFile, start_line: normie, end_line: normie) -> []tea`
Extracts range of lines for context display.

#### `SourceFile_find_line_with_content(source: SourceFile, search_text: tea) -> normie`
Searches for first line containing specified text.

### Code Generation

#### `CodeBuffer`
Advanced buffer for generating formatted code with indentation.

```cursed
squad CodeBuffer {
    spill content RuntimeStringBuilder
    spill indentation IndentationManager
    spill current_line normie
    spill needs_newline lit
}
```

#### Code Generation Operations

#### `CodeBuffer_new(indent_string: tea) -> CodeBuffer`
Creates code buffer with specified indentation style.

**Parameters:**
- `indent_string`: String used for each indentation level (e.g., "  ", "\t")

#### `CodeBuffer_write_line(buffer: CodeBuffer, line: tea) -> CodeBuffer`
Writes complete line with automatic indentation.

**Features:**
- Automatic indentation based on current level
- Newline management
- Line number tracking

#### `CodeBuffer_write(buffer: CodeBuffer, text: tea) -> CodeBuffer`
Writes text without automatic newlines.

#### `CodeBuffer_indent(buffer: CodeBuffer) -> CodeBuffer`
Increases indentation level for nested blocks.

#### `CodeBuffer_dedent(buffer: CodeBuffer) -> CodeBuffer`
Decreases indentation level.

#### `CodeBuffer_to_string(buffer: CodeBuffer) -> tea`
Converts buffer contents to final string.

### File Operations with Backup

#### `write_file_with_backup(filename: tea, content: tea) -> tea`
Writes file with automatic backup creation.

**Process:**
1. Check if target file exists
2. Create backup with .backup extension
3. Write new content to target
4. Return error status

#### `write_code_file(filename: tea, buffer: CodeBuffer) -> tea`
Writes code buffer to file with backup support.

### Directory Management

#### `ensure_output_directory(dir_path: tea) -> tea`
Creates output directory structure if it doesn't exist.

#### `clean_output_directory(dir_path: tea) -> tea`
Removes all files from output directory.

**Safety Features:**
- Checks directory existence before cleaning
- Lists files before deletion
- Error handling for permission issues

### Module Resolution

#### `ModuleResolver`
Intelligent module resolution with caching.

```cursed
squad ModuleResolver {
    spill search_paths []tea
    spill cache SymbolTable<tea>
}
```

#### Module Resolution Operations

#### `ModuleResolver_new(search_paths: []tea) -> ModuleResolver`
Creates resolver with specified search paths.

#### `ModuleResolver_resolve(resolver: ModuleResolver, module_name: tea) -> (tea, tea)`
Resolves module name to file path with caching.

**Algorithm:**
1. Check cache for previously resolved modules
2. Try each search path in order
3. Convert module name to file path
4. Check file existence
5. Cache successful resolution
6. Return path or error

#### `ModuleResolver_add_search_path(resolver: ModuleResolver, path: tea) -> ModuleResolver`
Adds new search path to resolver.

### Compiler Output Management

#### `CompilerOutput`
Manages all compiler-generated files and diagnostics.

```cursed
squad CompilerOutput {
    spill files SymbolTable<tea>
    spill errors []tea
    spill warnings []tea
    spill output_dir tea
}
```

#### Compiler Output Operations

#### `CompilerOutput_new(output_dir: tea) -> CompilerOutput`
Creates output manager for specified directory.

#### `CompilerOutput_add_file(output: CompilerOutput, filename: tea, content: tea) -> CompilerOutput`
Registers file for output generation.

#### `CompilerOutput_add_error(output: CompilerOutput, error_msg: tea) -> CompilerOutput`
Records compilation error.

#### `CompilerOutput_add_warning(output: CompilerOutput, warning_msg: tea) -> CompilerOutput`
Records compilation warning.

#### `CompilerOutput_write_all(output: CompilerOutput) -> tea`
Writes all registered files to output directory.

#### `CompilerOutput_has_errors(output: CompilerOutput) -> lit`
Checks if any errors were recorded.

## Usage Examples

### Source File Processing

```cursed
yeet "io_enhanced"

// Read source file with error handling
(sus source SourceFile, sus err tea) = SourceFile_read("main.csd")
lowkey err != "" {
    vibez.spill("Failed to read source: " + err)
    damn
}

vibez.spill("Loaded " + string(source.line_count) + " lines from " + source.path)

// Get specific lines for error reporting
sus error_line tea = SourceFile_get_line(source, 15)
vibez.spill("Line 15: " + error_line)

// Get context around error
sus context []tea = SourceFile_get_line_range(source, 13, 17)
bestie i := 0; i < len(context); i = i + 1 {
    vibez.spill("  " + string(i + 13) + ": " + context[i])
}

// Search for specific patterns
sus function_line normie = SourceFile_find_line_with_content(source, "slay main")
lowkey function_line > 0 {
    vibez.spill("Found main function at line " + string(function_line))
}
```

### Code Generation

```cursed
// Create code buffer with 2-space indentation
sus buffer CodeBuffer = CodeBuffer_new("  ")

// Generate function with proper indentation
buffer = CodeBuffer_write_line(buffer, "slay generate_example() {")
buffer = CodeBuffer_indent(buffer)

buffer = CodeBuffer_write_line(buffer, "sus value normie = 42")
buffer = CodeBuffer_write_line(buffer, "lowkey value > 0 {")
buffer = CodeBuffer_indent(buffer)

buffer = CodeBuffer_write_line(buffer, "vibez.spill(\"Positive value: \" + string(value))")
buffer = CodeBuffer_dedent(buffer)

buffer = CodeBuffer_write_line(buffer, "}")
buffer = CodeBuffer_write_line(buffer, "damn value")
buffer = CodeBuffer_dedent(buffer)

buffer = CodeBuffer_write_line(buffer, "}")

// Write generated code to file
sus generated_code tea = CodeBuffer_to_string(buffer)
sus write_err tea = write_code_file("generated.csd", buffer)
lowkey write_err == "" {
    vibez.spill("Generated code written successfully")
}
```

### Module Resolution

```cursed
// Set up module resolver with multiple search paths
sus search_paths []tea = []tea{
    "stdlib/",
    "lib/",
    "vendor/",
    "/usr/local/lib/cursed/"
}

sus resolver ModuleResolver = ModuleResolver_new(search_paths)

// Resolve module dependencies
sus modules []tea = []tea{"testz", "stringz", "mathz", "custom_module"}

bestie i := 0; i < len(modules); i = i + 1 {
    sus module_name tea = modules[i]
    (sus file_path tea, sus err tea) = ModuleResolver_resolve(resolver, module_name)
    
    lowkey err == "" {
        vibez.spill("Resolved " + module_name + " -> " + file_path)
    } yikes {
        vibez.spill("Failed to resolve " + module_name + ": " + err)
    }
}

// Add project-specific search path
resolver = ModuleResolver_add_search_path(resolver, "project/modules/")
```

### Compiler Output Management

```cursed
// Create compiler output manager
sus output CompilerOutput = CompilerOutput_new("build/")

// Generate multiple output files
output = CompilerOutput_add_file(output, "main.ll", llvm_ir_code)
output = CompilerOutput_add_file(output, "main.s", assembly_code)
output = CompilerOutput_add_file(output, "symbols.json", symbol_table_json)

// Record compilation diagnostics
output = CompilerOutput_add_warning(output, "Unused variable 'temp' at line 45")
output = CompilerOutput_add_error(output, "Undefined function 'missing_func' at line 67")

// Check compilation status
lowkey CompilerOutput_has_errors(output) {
    vibez.spill("Compilation failed with " + string(CompilerOutput_error_count(output)) + " errors")
} yikes {
    // Write all files if compilation succeeded
    sus write_err tea = CompilerOutput_write_all(output)
    lowkey write_err == "" {
        vibez.spill("All output files written successfully")
    }
}
```

### Build Configuration

```cursed
// Read build configuration
(sus config SymbolTable<tea>, sus err tea) = read_build_config("CursedBuild.toml")
lowkey err == "" {
    (sus project_name tea, sus found lit) = SymbolTable_get(config, "name")
    lowkey found {
        vibez.spill("Building project: " + project_name)
    }
    
    (sus source_dir tea, sus found lit) = SymbolTable_get(config, "source_dir")
    lowkey found {
        vibez.spill("Source directory: " + source_dir)
    }
}

// Write build manifest
sus files []tea = []tea{"main.csd", "utils.csd", "types.csd"}
sus dependencies []tea = []tea{"testz", "stringz", "io"}
sus manifest_err tea = write_build_manifest("build/manifest.txt", files, dependencies)
```

## Advanced Features

### Indentation Management

#### `IndentationManager`
Sophisticated indentation handling for various coding styles.

```cursed
squad IndentationManager {
    spill indent_string tea
    spill current_level normie
    spill style tea  // "spaces", "tabs", "mixed"
}
```

**Features:**
- Configurable indentation style (spaces, tabs, mixed)
- Automatic level tracking
- Consistent formatting across generated code
- Support for nested indentation patterns

### Enhanced String Operations

```cursed
// Advanced string processing for code generation
slay module_path_to_file_path(module_name tea) tea {
    // Convert "foo.bar.baz" to "foo/bar/baz.csd"
    sus path tea = string_replace(module_name, ".", "/")
    damn path + ".csd"
}

slay split_lines(content tea) []tea {
    // Efficient line splitting with multiple line ending support
    // Handles \n, \r\n, and \r
    sus lines []tea = []tea{}
    // ... implementation details
    damn lines
}
```

### Build System Integration

```cursed
// Configuration file parsing
slay parse_toml_config(content tea) SymbolTable<tea> {
    sus config SymbolTable<tea> = SymbolTable_new<tea>()
    
    // Parse simple key=value pairs
    // Support for sections [build], [dependencies], etc.
    // Comment handling with # prefix
    
    damn config
}

// Dependency tracking
slay track_file_dependencies(source_files []tea) SymbolTable<[]tea> {
    sus dependencies SymbolTable<[]tea> = SymbolTable_new<[]tea>()
    
    // Analyze import statements
    // Build dependency graph
    // Detect circular dependencies
    
    damn dependencies
}
```

## Performance Optimizations

### Memory Management

- **String Builder**: Efficient string concatenation for large code generation
- **Line Caching**: Cache frequently accessed source lines
- **Buffer Pooling**: Reuse code buffers for multiple generations
- **Path Caching**: Cache resolved module paths

### I/O Optimizations

- **Batch File Operations**: Group file writes for better performance
- **Streaming**: Process large files without loading entirely into memory
- **Compression**: Optional compression for generated files
- **Async I/O**: Framework ready for asynchronous file operations

### Compiler Integration

```cursed
// Optimized for compiler workloads
slay compiler_optimized_read(file_path tea) SourceFile {
    // Pre-allocate line array based on file size estimation
    // Use memory-mapped files for large sources
    // Cache frequently accessed files
    damn source
}

slay compiler_optimized_write(files []CompilerFile) tea {
    // Batch write operations
    // Parallel file generation where safe
    // Atomic writes with temporary files
    damn ""
}
```

## Error Handling

### Comprehensive Error Management

```cursed
// Error types for different I/O operations
facts ERROR_FILE_NOT_FOUND tea = "file_not_found"
facts ERROR_PERMISSION_DENIED tea = "permission_denied"
facts ERROR_DISK_FULL tea = "disk_full"
facts ERROR_INVALID_PATH tea = "invalid_path"

slay handle_io_error(operation tea, error tea) tea {
    match error {
        ERROR_FILE_NOT_FOUND -> {
            damn "File not found during " + operation
        }
        ERROR_PERMISSION_DENIED -> {
            damn "Permission denied for " + operation
        }
        ERROR_DISK_FULL -> {
            damn "Disk full, cannot complete " + operation
        }
        default -> {
            damn "Unknown error during " + operation + ": " + error
        }
    }
}
```

### Recovery Strategies

```cursed
// Graceful degradation for I/O failures
slay safe_write_with_retry(filename tea, content tea, max_retries normie) tea {
    sus attempts normie = 0
    
    bestie attempts < max_retries {
        sus err tea = write_file_with_backup(filename, content)
        lowkey err == "" {
            damn ""  // Success
        }
        
        attempts = attempts + 1
        lowkey attempts < max_retries {
            // Wait before retry
            sleep_ms(1000 * attempts)  // Exponential backoff
        }
    }
    
    damn "Failed after " + string(max_retries) + " attempts"
}
```

## Testing

### Comprehensive Test Suite

```bash
# Run I/O enhancement tests
zig build test
./zig-out/bin/cursed-zig stdlib/io_enhanced/test_io_enhanced.csd
```

### Test Categories

1. **Source File Tests**
   - File reading with various encodings
   - Line-by-line access
   - Error handling for missing files

2. **Code Generation Tests**
   - Indentation correctness
   - Large code buffer performance
   - Output formatting validation

3. **Module Resolution Tests**
   - Path resolution with multiple search directories
   - Caching behavior verification
   - Error handling for missing modules

4. **Build Integration Tests**
   - Configuration file parsing
   - Output directory management
   - File backup and recovery

## Dependencies

```cursed
yeet "testz"           // Testing framework
yeet "runtime_core"    // Runtime utilities
yeet "hash_map_enhanced" // For symbol tables
yeet "string_enhanced"   // Enhanced string operations
```

## Integration Points

### Compiler Pipeline

```cursed
// Complete compiler I/O workflow
slay compile_project(config_path tea) tea {
    // 1. Read configuration
    (sus config SymbolTable<tea>, sus err tea) = read_build_config(config_path)
    lowkey err != "" {
        damn err
    }
    
    // 2. Set up module resolution
    sus resolver ModuleResolver = create_resolver_from_config(config)
    
    // 3. Read source files
    sus sources []SourceFile = read_all_sources(config)
    
    // 4. Generate output
    sus output CompilerOutput = CompilerOutput_new("build/")
    
    // 5. Process and write results
    damn process_compilation(sources, output, resolver)
}
```

### IDE Integration

```cursed
// Language server protocol support
slay provide_line_diagnostics(source SourceFile, line_num normie) tea {
    sus line tea = SourceFile_get_line(source, line_num)
    sus context []tea = SourceFile_get_line_range(source, line_num - 2, line_num + 2)
    
    // Return formatted diagnostic information
    damn format_diagnostic(line, context, line_num)
}
```

## Architecture

### Layered Design

1. **File Layer**: Basic file operations with enhanced error handling
2. **Source Layer**: Compiler-specific source file management
3. **Generation Layer**: Code generation and formatting
4. **Build Layer**: Project-level build system integration

### Extension Points

- Custom indentation styles
- Alternative file formats (YAML, JSON configuration)
- Plugin system for code generators
- Integration with version control systems
- Support for remote module resolution

The module provides a comprehensive I/O foundation specifically optimized for CURSED compiler infrastructure and development tooling.
