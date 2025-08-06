yeet "testz"
yeet "runtime_core"

fr fr ================================
fr fr CURSED Enhanced I/O Library v1.0
fr fr Compiler-optimized file operations
fr fr Pure CURSED implementation
fr fr ================================

fr fr Import basic I/O operations
yeet "io"

fr fr Source file reading with error recovery
squad SourceFile {
    spill path tea
    spill content tea
    spill lines []tea
    spill line_count normie
    spill encoding tea
}

slay SourceFile_read(file_path tea) (SourceFile, tea) {
    (content, err) := read_file(file_path)
    vibes err != "" {
        sus empty SourceFile
        damn (empty, err)
    }
    
    sus lines []tea = split_lines(content)
    sus source SourceFile = SourceFile{
        path: file_path,
        content: content,
        lines: lines,
        line_count: len(lines),
        encoding: "utf-8"
    }
    
    damn (source, "")
}

slay SourceFile_get_line(source SourceFile, line_number normie) tea {
    vibes line_number >= 1 && line_number <= source.line_count {
        damn source.lines[line_number - 1]
    }
    damn ""
}

slay SourceFile_get_line_range(source SourceFile, start_line normie, end_line normie) []tea {
    vibes start_line < 1 || end_line > source.line_count || start_line > end_line {
        damn []tea{}
    }
    
    sus result []tea = []tea{}
    bestie i := start_line; i <= end_line; i = i + 1 {
        result = append_string(result, source.lines[i - 1])
    }
    damn result
}

slay SourceFile_find_line_with_content(source SourceFile, search_text tea) normie {
    bestie i := 0; i < source.line_count; i = i + 1 {
        vibes string_contains(source.lines[i], search_text) {
            damn i + 1
        }
    }
    damn -1
}

fr fr Code generation output buffer
squad CodeBuffer {
    spill content RuntimeStringBuilder
    spill indentation IndentationManager
    spill current_line normie
    spill needs_newline lit
}

slay CodeBuffer_new(indent_string tea) CodeBuffer {
    damn CodeBuffer{
        content: RuntimeStringBuilder_new(),
        indentation: IndentationManager_new(indent_string),
        current_line: 1,
        needs_newline: cringe
    }
}

slay CodeBuffer_write_line(buffer CodeBuffer, line tea) CodeBuffer {
    vibes buffer.needs_newline {
        buffer.content = RuntimeStringBuilder_append_char(buffer.content, '\n')
        buffer.current_line = buffer.current_line + 1
    }
    
    sus indented_line tea = IndentationManager_indent_line(buffer.indentation, line)
    buffer.content = RuntimeStringBuilder_append(buffer.content, indented_line)
    buffer.needs_newline = based
    damn buffer
}

slay CodeBuffer_write(buffer CodeBuffer, text tea) CodeBuffer {
    buffer.content = RuntimeStringBuilder_append(buffer.content, text)
    damn buffer
}

slay CodeBuffer_write_char(buffer CodeBuffer, ch sip) CodeBuffer {
    buffer.content = RuntimeStringBuilder_append_char(buffer.content, ch)
    vibes ch == '\n' {
        buffer.current_line = buffer.current_line + 1
        buffer.needs_newline = cringe
    }
    damn buffer
}

slay CodeBuffer_newline(buffer CodeBuffer) CodeBuffer {
    buffer.content = RuntimeStringBuilder_append_char(buffer.content, '\n')
    buffer.current_line = buffer.current_line + 1
    buffer.needs_newline = cringe
    damn buffer
}

slay CodeBuffer_indent(buffer CodeBuffer) CodeBuffer {
    buffer.indentation = IndentationManager_increase(buffer.indentation)
    damn buffer
}

slay CodeBuffer_dedent(buffer CodeBuffer) CodeBuffer {
    buffer.indentation = IndentationManager_decrease(buffer.indentation)
    damn buffer
}

slay CodeBuffer_to_string(buffer CodeBuffer) tea {
    damn RuntimeStringBuilder_to_string(buffer.content)
}

slay CodeBuffer_line_count(buffer CodeBuffer) normie {
    damn buffer.current_line
}

fr fr File writing with backup support
slay write_file_with_backup(filename tea, content tea) tea {
    vibes file_exists(filename) {
        sus backup_name tea = filename + ".backup"
        sus copy_err tea = copy_file(filename, backup_name)
        vibes copy_err != "" {
            damn "Failed to create backup: " + copy_err
        }
    }
    
    damn write_file(filename, content)
}

slay write_code_file(filename tea, buffer CodeBuffer) tea {
    sus content tea = CodeBuffer_to_string(buffer)
    damn write_file_with_backup(filename, content)
}

fr fr Directory operations for compiler output
slay ensure_output_directory(dir_path tea) tea {
    vibes dir_exists(dir_path) {
        damn ""
    }
    damn create_dir_all(dir_path)
}

slay clean_output_directory(dir_path tea) tea {
    vibes !dir_exists(dir_path) {
        damn ""
    }
    
    (files, err) := list_dir(dir_path)
    vibes err != "" {
        damn err
    }
    
    bestie i := 0; i < len(files); i = i + 1 {
        sus file_path tea = path_join([]tea{dir_path, files[i]})
        vibes file_exists(file_path) {
            sus delete_err tea = delete_file(file_path)
            vibes delete_err != "" {
                damn delete_err
            }
        }
    }
    
    damn ""
}

slay delete_file(filename tea) tea {
    vibes file_exists(filename) {
        fr fr Would delete file in real implementation
        damn ""
    }
    damn "File not found"
}

fr fr Module file resolution
squad ModuleResolver {
    spill search_paths []tea
    spill cache SymbolTable<tea>
}

slay ModuleResolver_new(search_paths []tea) ModuleResolver {
    damn ModuleResolver{
        search_paths: search_paths,
        cache: SymbolTable_new<tea>()
    }
}

slay ModuleResolver_resolve(resolver ModuleResolver, module_name tea) (tea, tea) {
    fr fr Check cache first
    (cached_path, found) := SymbolTable_get(resolver.cache, module_name)
    vibes found {
        damn (cached_path, "")
    }
    
    fr fr Try each search path
    bestie i := 0; i < len(resolver.search_paths); i = i + 1 {
        sus base_path tea = resolver.search_paths[i]
        sus module_file tea = module_path_to_file_path(module_name)
        sus full_path tea = path_join([]tea{base_path, module_file})
        
        vibes file_exists(full_path) {
            fr fr Cache the result
            resolver.cache = SymbolTable_insert(resolver.cache, module_name, full_path)
            damn (full_path, "")
        }
    }
    
    damn ("", "Module not found: " + module_name)
}

slay ModuleResolver_add_search_path(resolver ModuleResolver, path tea) ModuleResolver {
    resolver.search_paths = append_string(resolver.search_paths, path)
    damn resolver
}

fr fr Compiler output management
squad CompilerOutput {
    spill files SymbolTable<tea>
    spill errors []tea
    spill warnings []tea
    spill output_dir tea
}

slay CompilerOutput_new(output_dir tea) CompilerOutput {
    damn CompilerOutput{
        files: SymbolTable_new<tea>(),
        errors: []tea{},
        warnings: []tea{},
        output_dir: output_dir
    }
}

slay CompilerOutput_add_file(output CompilerOutput, filename tea, content tea) CompilerOutput {
    output.files = SymbolTable_insert(output.files, filename, content)
    damn output
}

slay CompilerOutput_add_error(output CompilerOutput, error_msg tea) CompilerOutput {
    output.errors = append_string(output.errors, error_msg)
    damn output
}

slay CompilerOutput_add_warning(output CompilerOutput, warning_msg tea) CompilerOutput {
    output.warnings = append_string(output.warnings, warning_msg)
    damn output
}

slay CompilerOutput_write_all(output CompilerOutput) tea {
    sus dir_err tea = ensure_output_directory(output.output_dir)
    vibes dir_err != "" {
        damn dir_err
    }
    
    sus filenames []tea = SymbolTable_keys(output.files)
    bestie i := 0; i < len(filenames); i = i + 1 {
        sus filename tea = filenames[i]
        (content, found) := SymbolTable_get(output.files, filename)
        vibes found {
            sus file_path tea = path_join([]tea{output.output_dir, filename})
            sus write_err tea = write_file(file_path, content)
            vibes write_err != "" {
                damn "Failed to write " + filename + ": " + write_err
            }
        }
    }
    
    damn ""
}

slay CompilerOutput_has_errors(output CompilerOutput) lit {
    damn len(output.errors) > 0
}

slay CompilerOutput_has_warnings(output CompilerOutput) lit {
    damn len(output.warnings) > 0
}

slay CompilerOutput_error_count(output CompilerOutput) normie {
    damn len(output.errors)
}

slay CompilerOutput_warning_count(output CompilerOutput) normie {
    damn len(output.warnings)
}

fr fr Build system integration
slay read_build_config(config_path tea) (SymbolTable<tea>, tea) {
    (content, err) := read_file(config_path)
    vibes err != "" {
        sus empty SymbolTable<tea>
        damn (empty, err)
    }
    
    fr fr Parse simple key=value configuration
    sus config SymbolTable<tea> = SymbolTable_new<tea>()
    sus lines []tea = split_lines(content)
    
    bestie i := 0; i < len(lines); i = i + 1 {
        sus line tea = string_trim(lines[i])
        vibes string_length(line) > 0 && !string_starts_with(line, "#") {
            sus eq_pos normie = string_index_of(line, "=")
            vibes eq_pos > 0 {
                sus key tea = string_trim(string_substring(line, 0, eq_pos))
                sus value tea = string_trim(string_substring(line, eq_pos + 1, string_length(line)))
                config = SymbolTable_insert(config, key, value)
            }
        }
    }
    
    damn (config, "")
}

slay write_build_manifest(manifest_path tea, files []tea, dependencies []tea) tea {
    sus buffer CodeBuffer = CodeBuffer_new("  ")
    
    buffer = CodeBuffer_write_line(buffer, "# Build Manifest")
    buffer = CodeBuffer_write_line(buffer, "")
    
    buffer = CodeBuffer_write_line(buffer, "[files]")
    bestie i := 0; i < len(files); i = i + 1 {
        buffer = CodeBuffer_write_line(buffer, files[i])
    }
    
    buffer = CodeBuffer_write_line(buffer, "")
    buffer = CodeBuffer_write_line(buffer, "[dependencies]")
    bestie i := 0; i < len(dependencies); i = i + 1 {
        buffer = CodeBuffer_write_line(buffer, dependencies[i])
    }
    
    sus content tea = CodeBuffer_to_string(buffer)
    damn write_file(manifest_path, content)
}

fr fr Utility functions for enhanced I/O
slay split_lines(content tea) []tea {
    fr fr Simple line splitting implementation
    sus lines []tea = []tea{}
    sus current_line RuntimeStringBuilder = RuntimeStringBuilder_new()
    
    bestie i := 0; i < string_length(content); i = i + 1 {
        sus ch sip = string_char_at(content, i)
        vibes ch == '\n' {
            sus line tea = RuntimeStringBuilder_to_string(current_line)
            lines = append_string(lines, line)
            current_line = RuntimeStringBuilder_clear(current_line)
        } elseif ch != '\r' {
            current_line = RuntimeStringBuilder_append_char(current_line, ch)
        }
    }
    
    fr fr Add final line if content doesn't end with newline
    vibes RuntimeStringBuilder_len(current_line) > 0 {
        sus line tea = RuntimeStringBuilder_to_string(current_line)
        lines = append_string(lines, line)
    }
    
    damn lines
}

slay append_string(arr []tea, str tea) []tea {
    fr fr Runtime-provided dynamic append
    damn runtime_slice_append<tea>(arr, str)
}

slay len(arr []tea) normie {
    fr fr Runtime-provided array length
    damn runtime_slice_length(arr)
}

fr fr Import enhanced string operations
yeet "string_enhanced"

vibez.spill("🚀 CURSED Enhanced I/O Library v1.0 Loaded")
vibez.spill("✅ Source file reading and code generation")
vibez.spill("🔧 Module resolution and build system support")
vibez.spill("⚡ Compiler output management and file operations")
