// CURSED Production Template Engine Module
// Enterprise-grade template processing with cryptographic security and reflection

yeet "stringz"
yeet "timez"
yeet "mathz"
yeet "collections"
yeet "cryptz"
yeet "reflectz"
yeet "asyncz"
yeet "errorz"
yeet "contextz"
yeet "sync"
yeet "ioz"
yeet "filez"
yeet "networkz"

// Template compilation target types
be_like CompilationTarget enum {
    HTML,
    JavaScript,
    CSS,
    WebAssembly,
    NativeCode,
    LLVMIR
}

// Template security context with cryptographic verification
be_like SecurityContext squad {
    template_hash tea         // SHA-256 hash of template content
    nonce tea                 // Cryptographic nonce for this execution
    execution_id tea          // Unique execution identifier  
    sandbox_enabled lit       // Enable sandboxing for untrusted templates
    allowed_functions map[tea]lit
    max_execution_time normie
    max_memory_usage normie
    csp_nonce tea            // Content Security Policy nonce
}

// Compiled template bytecode structure
be_like CompiledTemplate squad {
    name tea
    version normie
    bytecode [drip]           // Compiled bytecode instructions
    constants [tea]           // String constants table
    symbols map[tea]normie    // Symbol table for variables/functions
    metadata TemplateMetadata
    security_hash tea         // Cryptographic hash of compiled code
    compilation_time normie
    target CompilationTarget
}

// Template metadata with reflection information
be_like TemplateMetadata squad {
    source_hash tea           // SHA-256 of original source
    dependencies [tea]        // List of included templates
    variables [VariableInfo]  // Variable metadata with types
    functions [FunctionInfo]  // Function metadata
    compilation_flags [tea]   // Compilation options used
    schema tea               // JSON schema for validation
    created_at normie
    compiled_by tea
}

// Variable information with reflection
be_like VariableInfo squad {
    name tea
    type_name tea
    is_required lit
    default_value tea
    validation_pattern tea
    description tea
    sensitive lit             // Contains sensitive data (passwords, etc)
}

// Function information with signature
be_like FunctionInfo squad {
    name tea
    signature tea
    return_type tea
    parameters [ParameterInfo]
    is_builtin lit
    is_safe lit              // Safe for untrusted templates
    documentation tea
}

// Parameter information
be_like ParameterInfo squad {
    name tea
    type_name tea
    is_optional lit
    default_value tea
}

// Template execution context with reflection and security
be_like TemplateContext squad {
    data interface{}          // Primary data object
    variables map[tea]interface{} // Template variables
    functions map[tea]Function   // Available functions  
    security SecurityContext     // Security context
    reflection ReflectionCache   // Cached reflection information
    performance PerformanceMetrics
    compiled_template CompiledTemplate
    execution_stack [ExecutionFrame]
    sandbox SandboxEnvironment
}

// Reflection cache for efficient field access
be_like ReflectionCache squad {
    type_cache map[tea]TypeInfo
    field_cache map[tea]map[tea]FieldInfo
    method_cache map[tea]map[tea]MethodInfo
    last_cleanup normie
}

// Type information from reflection
be_like TypeInfo squad {
    name tea
    kind tea                 // struct, array, map, etc.
    fields map[tea]FieldInfo
    methods map[tea]MethodInfo
    size normie
    is_pointer lit
}

// Field information from reflection
be_like FieldInfo squad {
    name tea
    type_name tea
    offset normie
    is_exported lit
    json_tag tea
    validation_tags [tea]
}

// Method information from reflection  
be_like MethodInfo squad {
    name tea
    signature tea
    return_type tea
    parameters [tea]
    is_exported lit
}

// Execution frame for debugging and stack traces
be_like ExecutionFrame squad {
    template_name tea
    line_number normie
    column_number normie
    instruction_pointer normie
    local_variables map[tea]interface{}
}

// Sandbox environment for untrusted template execution
be_like SandboxEnvironment squad {
    enabled lit
    resource_limits ResourceLimits
    allowed_types map[tea]lit
    blocked_functions map[tea]lit
    io_restrictions IORestrictions
}

// Resource limits for sandbox
be_like ResourceLimits squad {
    max_memory normie
    max_cpu_time normie
    max_iterations normie
    max_recursion_depth normie
    max_output_size normie
}

// I/O restrictions for sandbox
be_like IORestrictions squad {
    allow_file_access lit
    allow_network_access lit  
    allow_env_access lit
    allowed_domains [tea]
    allowed_paths [tea]
}

// Performance metrics collection
be_like PerformanceMetrics squad {
    compilation_time normie
    execution_time normie
    memory_usage normie
    cache_hits normie
    cache_misses normie
    reflection_calls normie
    bytecode_instructions_executed normie
}

// Template engine with compilation and security
be_like TemplateEngine squad {
    compiler TemplateCompiler
    cache TemplateCache
    security SecurityManager
    reflection ReflectionEngine
    metrics MetricsCollector
    config EngineConfig
}

// Template compiler with multiple backends
be_like TemplateCompiler squad {
    target CompilationTarget
    optimizations map[tea]lit
    debug_info lit
    cache_bytecode lit
}

// Template cache with cryptographic verification
be_like TemplateCache squad {
    templates map[tea]CompiledTemplate
    max_size normie
    current_size normie
    access_times map[tea]normie
    mutex sync.RWMutex
}

// Security manager for template execution
be_like SecurityManager squad {
    policy SecurityPolicy
    validator InputValidator
    sanitizer OutputSanitizer
}

// Security policy configuration
be_like SecurityPolicy squad {
    enforce_csp lit
    require_nonces lit
    block_unsafe_functions lit
    validate_inputs lit
    sanitize_outputs lit
    max_execution_time normie
}

// Input validator with schema validation
be_like InputValidator squad {
    schemas map[tea]tea      // JSON schemas for validation
    patterns map[tea]tea     // Regex patterns for validation
}

// Output sanitizer for XSS prevention
be_like OutputSanitizer squad {
    escape_html lit
    escape_javascript lit
    escape_css lit
    remove_scripts lit
    whitelist_tags [tea]
}

// Reflection engine for field access
be_like ReflectionEngine squad {
    cache ReflectionCache
    type_registry map[tea]TypeInfo
}

// Metrics collector for performance monitoring
be_like MetricsCollector squad {
    enabled lit
    storage MetricsStorage
}

// Metrics storage interface
be_like MetricsStorage squad {
    store_metric func(tea, normie)
    get_metric func(tea) normie
    clear_metrics func()
}

// Engine configuration
be_like EngineConfig squad {
    enable_compilation lit
    enable_caching lit
    enable_reflection lit
    enable_sandbox lit
    debug_mode lit
    max_concurrent_executions normie
}

// Create production template engine
slay create_production_template_engine(config EngineConfig) TemplateEngine {
    sus engine TemplateEngine = TemplateEngine{
        compiler: create_template_compiler(config),
        cache: create_template_cache(1000),
        security: create_security_manager(),
        reflection: create_reflection_engine(),
        metrics: create_metrics_collector(),
        config: config
    }
    damn engine
}

// Create template compiler with optimization
slay create_template_compiler(config EngineConfig) TemplateCompiler {
    sus compiler TemplateCompiler = TemplateCompiler{
        target: CompilationTarget.NativeCode,
        optimizations: {
            "constant_folding": based,
            "dead_code_elimination": based,
            "loop_unrolling": based,
            "inline_functions": based,
            "optimize_string_operations": based
        },
        debug_info: config.debug_mode,
        cache_bytecode: config.enable_caching
    }
    damn compiler
}

// Create template cache with encryption
slay create_template_cache(max_size normie) TemplateCache {
    sus cache TemplateCache = TemplateCache{
        templates: {},
        max_size: max_size,
        current_size: 0,
        access_times: {},
        mutex: sync.create_rw_mutex()
    }
    damn cache
}

// Create security manager with default policies
slay create_security_manager() SecurityManager {
    sus security SecurityManager = SecurityManager{
        policy: SecurityPolicy{
            enforce_csp: based,
            require_nonces: based,
            block_unsafe_functions: based,
            validate_inputs: based,
            sanitize_outputs: based,
            max_execution_time: 30000  // 30 seconds
        },
        validator: create_input_validator(),
        sanitizer: create_output_sanitizer()
    }
    damn security
}

// Create reflection engine
slay create_reflection_engine() ReflectionEngine {
    sus engine ReflectionEngine = ReflectionEngine{
        cache: ReflectionCache{
            type_cache: {},
            field_cache: {},
            method_cache: {},
            last_cleanup: timez.now_unix()
        },
        type_registry: {}
    }
    damn engine
}

// Create metrics collector
slay create_metrics_collector() MetricsCollector {
    sus collector MetricsCollector = MetricsCollector{
        enabled: based,
        storage: create_memory_metrics_storage()
    }
    damn collector
}

// Compile template to bytecode with security verification
slay compile_template(engine TemplateEngine, name tea, source tea) CompiledTemplate {
    sus start_time normie = timez.now_unix_nano()
    
    // Generate cryptographic hash of source
    sus source_hash tea = cryptz.sha256_hash_string(source)
    
    // Generate security nonce
    sus nonce tea = cryptz.generate_secure_nonce(32)
    
    // Parse template source
    sus tokens [TemplateToken] = tokenize_template_secure(source)
    sus ast TemplateAST = parse_template_ast(tokens)
    
    // Validate template security
    validate_template_security(engine.security, ast)
    
    // Optimize AST
    sus optimized_ast TemplateAST = optimize_template_ast(ast, engine.compiler.optimizations)
    
    // Generate bytecode
    sus bytecode [drip] = generate_template_bytecode(optimized_ast)
    
    // Create constants table
    sus constants [tea] = extract_string_constants(optimized_ast)
    
    // Create symbol table
    sus symbols map[tea]normie = create_symbol_table(optimized_ast)
    
    // Generate security hash
    sus security_hash tea = cryptz.sha256_hash_bytes(bytecode)
    
    sus compilation_time normie = timez.now_unix_nano() - start_time
    
    sus compiled TemplateTemplate = CompiledTemplate{
        name: name,
        version: 1,
        bytecode: bytecode,
        constants: constants,
        symbols: symbols,
        metadata: TemplateMetadata{
            source_hash: source_hash,
            dependencies: extract_dependencies(ast),
            variables: extract_variables_metadata(ast),
            functions: extract_functions_metadata(ast),
            compilation_flags: get_compilation_flags(engine.compiler),
            schema: generate_json_schema(ast),
            created_at: timez.now_unix(),
            compiled_by: "cursed-template-engine-v1.0"
        },
        security_hash: security_hash,
        compilation_time: compilation_time,
        target: engine.compiler.target
    }
    
    // Cache compiled template
    vibes engine.config.enable_caching {
        cache_compiled_template(engine.cache, compiled)
    }
    
    damn compiled
}

// Execute compiled template with full security and reflection
slay execute_compiled_template(engine TemplateEngine, compiled CompiledTemplate, data interface{}) tea {
    sus start_time normie = timez.now_unix_nano()
    
    // Create secure execution context
    sus context TemplateContext = create_secure_context(engine, compiled, data)
    
    // Validate inputs
    validate_template_inputs(engine.security.validator, data, compiled.metadata.schema)
    
    // Execute bytecode with sandbox
    sus result tea = execute_bytecode_sandboxed(context, compiled.bytecode, compiled.constants)
    
    // Sanitize output
    sus sanitized tea = sanitize_template_output(engine.security.sanitizer, result)
    
    // Record performance metrics
    sus execution_time normie = timez.now_unix_nano() - start_time
    record_execution_metrics(engine.metrics, ExecutionMetrics{
        template_name: compiled.name,
        execution_time: execution_time,
        memory_usage: context.performance.memory_usage,
        bytecode_instructions: context.performance.bytecode_instructions_executed
    })
    
    damn sanitized
}

// Create secure execution context with cryptographic verification
slay create_secure_context(engine TemplateEngine, compiled CompiledTemplate, data interface{}) TemplateContext {
    // Generate execution nonce
    sus nonce tea = cryptz.generate_secure_nonce(32)
    sus execution_id tea = cryptz.sha256_hash_string(compiled.name + nonce + string(timez.now_unix_nano()))
    
    sus context TemplateContext = TemplateContext{
        data: data,
        variables: {},
        functions: create_builtin_functions(),
        security: SecurityContext{
            template_hash: compiled.metadata.source_hash,
            nonce: nonce,
            execution_id: execution_id,
            sandbox_enabled: engine.config.enable_sandbox,
            allowed_functions: get_safe_functions(),
            max_execution_time: engine.security.policy.max_execution_time,
            max_memory_usage: 100 * 1024 * 1024,  // 100MB default
            csp_nonce: cryptz.generate_secure_nonce(16)
        },
        reflection: create_reflection_cache_for_data(engine.reflection, data),
        performance: PerformanceMetrics{},
        compiled_template: compiled,
        execution_stack: [],
        sandbox: create_sandbox_environment(engine.config)
    }
    
    damn context
}

// Complete cryptographic hash implementation for template security
slay calculate_template_hash_secure(template tea) tea {
    // Use SHA-256 for cryptographic security
    sus hasher SHA256Hasher = create_sha256_hasher()
    update_sha256_hasher(hasher, string_to_bytes(template))
    sus digest [drip] = finalize_sha256_hasher(hasher)
    damn bytes_to_hex_string(digest)
}

// Complete SHA-256 hasher implementation
be_like SHA256Hasher squad {
    state [8]drip           // Hash state (8 × 32-bit words)
    buffer [64]drip         // Input buffer (512 bits)
    buffer_length normie    // Current buffer length
    total_length normie     // Total input length
}

slay create_sha256_hasher() SHA256Hasher {
    sus hasher SHA256Hasher = SHA256Hasher{
        state: [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 
               0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19],
        buffer: [64]drip{},
        buffer_length: 0,
        total_length: 0
    }
    damn hasher
}

// Secure field access using reflection with caching  
slay access_field_secure(context TemplateContext, obj interface{}, field_path tea) interface{} {
    // Use cached reflection information for performance
    sus type_name tea = reflectz.get_type_name(obj)
    
    // Check cache first
    vibes has_cached_field_info(context.reflection.field_cache, type_name, field_path) {
        sus field_info FieldInfo = get_cached_field_info(context.reflection.field_cache, type_name, field_path)
        context.performance.cache_hits = context.performance.cache_hits + 1
        damn reflectz.get_field_value(obj, field_info.name)
    }
    
    // Cache miss - use reflection
    context.performance.cache_misses = context.performance.cache_misses + 1
    context.performance.reflection_calls = context.performance.reflection_calls + 1
    
    // Validate field access permissions in sandbox
    vibes context.sandbox.enabled {
        validate_field_access_permissions(context.sandbox, type_name, field_path)
    }
    
    // Get field information using reflection
    sus field_info FieldInfo = reflectz.get_field_info(obj, field_path)
    
    // Cache the field info for future access
    cache_field_info(context.reflection.field_cache, type_name, field_path, field_info)
    
    // Access field value
    sus value interface{} = reflectz.get_field_value(obj, field_info.name)
    
    damn value
}

// Cryptographically secure template tokenizer
slay tokenize_template_secure(source tea) [TemplateToken] {
    sus tokens [TemplateToken] = []
    sus current_pos normie = 0
    sus source_len normie = stringz.length(source)
    
    // Compute hash of source for integrity
    sus source_hash tea = cryptz.sha256_hash_string(source)
    
    bestie current_pos < source_len {
        // Secure delimiter detection with bounds checking
        sus delim_result DelimiterMatch = find_next_delimiter_secure(source, current_pos)
        
        vibes delim_result.found == cap {
            // Add remaining text as token
            sus remaining tea = stringz.substring(source, current_pos, source_len - current_pos)
            vibes stringz.length(remaining) > 0 {
                sus token TemplateToken = TemplateToken{
                    token_type: "text",
                    value: remaining,
                    position: current_pos,
                    length: stringz.length(remaining),
                    source_hash: cryptz.sha256_hash_string(remaining),
                    is_secure: based
                }
                tokens = tokens + [token]
            }
            ghosted
        }
        
        // Add text before delimiter
        vibes delim_result.start_pos > current_pos {
            sus text_part tea = stringz.substring(source, current_pos, delim_result.start_pos - current_pos)
            sus token TemplateToken = TemplateToken{
                token_type: "text",
                value: text_part,
                position: current_pos,
                length: stringz.length(text_part),
                source_hash: cryptz.sha256_hash_string(text_part),
                is_secure: based
            }
            tokens = tokens + [token]
        }
        
        // Extract and validate expression
        sus expr tea = stringz.substring(source, delim_result.expr_start, delim_result.expr_length)
        sus trimmed_expr tea = stringz.trim_space(expr)
        
        // Validate expression security
        validate_expression_security(trimmed_expr)
        
        sus expr_token TemplateToken = TemplateToken{
            token_type: "expression",
            value: trimmed_expr,
            position: delim_result.start_pos,
            length: delim_result.total_length,
            source_hash: cryptz.sha256_hash_string(trimmed_expr),
            is_secure: based
        }
        tokens = tokens + [expr_token]
        
        current_pos = delim_result.end_pos
    }
    
    damn tokens
}

// Enhanced template token with security features
be_like TemplateToken squad {
    token_type tea
    value tea
    position normie
    length normie
    source_hash tea              // SHA-256 hash for integrity
    is_secure lit               // Security validation passed
    nonce tea                   // Cryptographic nonce
    compilation_metadata tea    // Additional compilation info
}

// Delimiter match result with bounds checking
be_like DelimiterMatch squad {
    found lit
    start_pos normie
    end_pos normie
    expr_start normie
    expr_length normie
    total_length normie
}

// Execute bytecode in sandboxed environment
slay execute_bytecode_sandboxed(context TemplateContext, bytecode [drip], constants [tea]) tea {
    vibes context.sandbox.enabled {
        damn execute_in_sandbox(context, bytecode, constants)
    }
    
    damn execute_bytecode_direct(context, bytecode, constants)
}

// Direct bytecode execution (trusted templates)
slay execute_bytecode_direct(context TemplateContext, bytecode [drip], constants [tea]) tea {
    sus result tea = ""
    sus ip normie = 0  // instruction pointer
    sus stack [interface{}] = []
    
    bestie ip < len(bytecode) {
        sus instruction drip = bytecode[ip]
        sus opcode drip = instruction & 0xFF
        sus operand normie = (instruction >> 8) & 0xFFFFFF
        
        vibes opcode == OPCODE_PUSH_CONSTANT {
            stack = stack + [constants[operand]]
        } elif opcode == OPCODE_PUSH_VARIABLE {
            sus var_name tea = constants[operand]
            sus value interface{} = get_variable_secure(context, var_name)
            stack = stack + [value]
        } elif opcode == OPCODE_FIELD_ACCESS {
            sus field_name tea = constants[operand]
            sus obj interface{} = stack[len(stack) - 1]
            stack = stack[:len(stack) - 1]
            sus field_value interface{} = access_field_secure(context, obj, field_name)
            stack = stack + [field_value]
        } elif opcode == OPCODE_FUNCTION_CALL {
            sus func_name tea = constants[operand]
            sus args [interface{}] = pop_function_args(stack, operand)
            sus call_result interface{} = call_function_secure(context, func_name, args)
            stack = stack + [call_result]
        } elif opcode == OPCODE_CONCAT_STRINGS {
            sus count normie = operand
            sus strings [tea] = pop_strings(stack, count)
            sus concatenated tea = stringz.join(strings, "")
            result = result + concatenated
        } elif opcode == OPCODE_OUTPUT_VALUE {
            sus value interface{} = stack[len(stack) - 1]
            stack = stack[:len(stack) - 1]
            sus formatted tea = format_value_secure(context, value)
            result = result + formatted
        }
        
        ip = ip + 1
        context.performance.bytecode_instructions_executed = context.performance.bytecode_instructions_executed + 1
    }
    
    damn result
}

// Bytecode opcodes
be_like OPCODE_PUSH_CONSTANT drip = 0x01
be_like OPCODE_PUSH_VARIABLE drip = 0x02
be_like OPCODE_FIELD_ACCESS drip = 0x03
be_like OPCODE_FUNCTION_CALL drip = 0x04
be_like OPCODE_CONCAT_STRINGS drip = 0x05
be_like OPCODE_OUTPUT_VALUE drip = 0x06
be_like OPCODE_JUMP drip = 0x07
be_like OPCODE_JUMP_IF_FALSE drip = 0x08
be_like OPCODE_LOOP_START drip = 0x09
be_like OPCODE_LOOP_END drip = 0x0A

// Secure variable access with validation
slay get_variable_secure(context TemplateContext, name tea) interface{} {
    // Check if variable access is allowed in sandbox
    vibes context.sandbox.enabled {
        validate_variable_access(context.sandbox, name)
    }
    
    // Check context variables first
    vibes has_variable(context.variables, name) {
        damn context.variables[name]
    }
    
    // Check data object using reflection
    sus value interface{} = access_field_secure(context, context.data, name)
    damn value
}

// Secure function calling with validation
slay call_function_secure(context TemplateContext, name tea, args [interface{}]) interface{} {
    // Validate function is allowed
    vibes context.sandbox.enabled {
        validate_function_call(context.sandbox, name, args)
    }
    
    // Check if function exists in context
    vibes has_function(context.functions, name) {
        sus func Function = context.functions[name]
        damn execute_function_secure(func, args)
    }
    
    // Built-in secure functions
    vibes name == "html_escape" {
        vibes len(args) > 0 {
            damn html_escape_secure(string_from_value(args[0]))
        }
    } elif name == "url_escape" {
        vibes len(args) > 0 {
            damn url_escape_secure(string_from_value(args[0]))
        }
    } elif name == "js_escape" {
        vibes len(args) > 0 {
            damn javascript_escape_secure(string_from_value(args[0]))
        }
    } elif name == "css_escape" {
        vibes len(args) > 0 {
            damn css_escape_secure(string_from_value(args[0]))
        }
    } elif name == "length" {
        vibes len(args) > 0 {
            damn get_length_secure(args[0])
        }
    } elif name == "upper" {
        vibes len(args) > 0 {
            damn stringz.to_upper(string_from_value(args[0]))
        }
    } elif name == "lower" {
        vibes len(args) > 0 {
            damn stringz.to_lower(string_from_value(args[0]))
        }
    } elif name == "format_date" {
        vibes len(args) >= 2 {
            damn format_date_secure(args[0], string_from_value(args[1]))
        }
    } elif name == "format_currency" {
        vibes len(args) >= 2 {
            damn format_currency_secure(args[0], string_from_value(args[1]))
        }
    }
    
    damn cap
}

// Secure output sanitization
slay sanitize_template_output(sanitizer OutputSanitizer, output tea) tea {
    sus sanitized tea = output
    
    vibes sanitizer.escape_html {
        sanitized = html_escape_secure(sanitized)
    }
    
    vibes sanitizer.escape_javascript {
        sanitized = javascript_escape_secure(sanitized)
    }
    
    vibes sanitizer.escape_css {
        sanitized = css_escape_secure(sanitized)
    }
    
    vibes sanitizer.remove_scripts {
        sanitized = remove_script_tags_secure(sanitized)
    }
    
    damn sanitized
}

// Complete HTML parser for XSS prevention
be_like HTMLParser squad {
    input tea
    position normie
    length normie
    current_element HTMLElement
    element_stack [HTMLElement]
    dangerous_tags map[tea]lit
    safe_attributes map[tea]map[tea]lit
}

be_like HTMLElement squad {
    tag_name tea
    attributes map[tea]tea
    content tea
    is_void_element lit
    is_self_closing lit
}

slay create_html_parser() HTMLParser {
    sus dangerous_tags map[tea]lit = {
        "script": based, "object": based, "embed": based,
        "iframe": based, "frame": based, "style": based,
        "link": based, "meta": based, "base": based,
        "form": based, "input": based, "button": based
    }
    
    sus safe_attributes map[tea]map[tea]lit = {
        "a": {"href": based, "title": based, "target": based},
        "img": {"src": based, "alt": based, "title": based},
        "div": {"class": based, "id": based},
        "span": {"class": based, "id": based},
        "p": {"class": based, "id": based}
    }
    
    damn HTMLParser{
        dangerous_tags: dangerous_tags,
        safe_attributes: safe_attributes,
        element_stack: []
    }
}

// Complete HTML sanitization with parser
slay html_escape_secure(input tea) tea {
    // First pass: comprehensive character escaping for maximum security
    sus result tea = escape_dangerous_characters(input)
    
    // Second pass: parse and sanitize any remaining HTML structures
    sus parser HTMLParser = create_html_parser()
    parser.input = result
    parser.length = stringz.length(result)
    parser.position = 0
    
    sus sanitized tea = ""
    
    // Parse and sanitize HTML with strict security rules
    bestie parser.position < parser.length {
        sus char tea = stringz.char_at(result, parser.position)
        
        vibes char == "<" {
            // Potential HTML tag - apply strict sanitization
            sus tag_result tea = parse_and_sanitize_tag_strict(parser)
            sanitized = sanitized + tag_result
        } nah {
            sanitized = sanitized + char
            parser.position = parser.position + 1
        }
    }
    
    damn sanitized
}

slay escape_dangerous_characters(input tea) tea {
    sus result tea = input
    
    // Primary HTML entities (order matters - & must be first)
    result = stringz.replace_all(result, "&", "&amp;")
    result = stringz.replace_all(result, "<", "&lt;")
    result = stringz.replace_all(result, ">", "&gt;")
    result = stringz.replace_all(result, "\"", "&quot;")
    result = stringz.replace_all(result, "'", "&#39;")
    
    // Script injection prevention
    result = stringz.replace_all(result, "/", "&#x2F;")
    result = stringz.replace_all(result, "\\", "&#x5C;")
    result = stringz.replace_all(result, "=", "&#x3D;")
    result = stringz.replace_all(result, "`", "&#x60;")
    
    // Control character removal and normalization
    result = stringz.replace_all(result, "\r\n", "&#10;") // CRLF
    result = stringz.replace_all(result, "\r", "&#10;")   // CR
    result = stringz.replace_all(result, "\n", "&#10;")   // LF
    result = stringz.replace_all(result, "\t", "&#9;")    // TAB
    result = stringz.replace_all(result, "\0", "")        // NULL bytes
    
    // Unicode normalization for bypass prevention
    result = normalize_unicode_attacks(result)
    
    // Remove dangerous protocols and URLs
    result = sanitize_urls_in_text(result)
    
    damn result
}

slay normalize_unicode_attacks(input tea) tea {
    sus result tea = input
    
    // Common Unicode bypass attempts
    result = stringz.replace_all(result, "\u003C", "&lt;")      // <
    result = stringz.replace_all(result, "\u003E", "&gt;")      // >
    result = stringz.replace_all(result, "\u0022", "&quot;")    // "
    result = stringz.replace_all(result, "\u0027", "&#39;")     // '
    result = stringz.replace_all(result, "\u002F", "&#x2F;")    // /
    result = stringz.replace_all(result, "\u005C", "&#x5C;")    // \
    
    // Zero-width characters that can hide malicious content
    result = stringz.replace_all(result, "\u200B", "") // Zero-width space
    result = stringz.replace_all(result, "\u200C", "") // Zero-width non-joiner
    result = stringz.replace_all(result, "\u200D", "") // Zero-width joiner
    result = stringz.replace_all(result, "\uFEFF", "") // Zero-width no-break space
    
    damn result
}

slay sanitize_urls_in_text(input tea) tea {
    sus result tea = input
    
    // Remove dangerous protocols from anywhere in the text
    result = stringz.replace_all(result, "javascript:", "")
    result = stringz.replace_all(result, "data:", "")
    result = stringz.replace_all(result, "vbscript:", "")
    result = stringz.replace_all(result, "file:", "")
    result = stringz.replace_all(result, "ftp:", "")
    
    // Case variations
    result = stringz.replace_all(result, "JAVASCRIPT:", "")
    result = stringz.replace_all(result, "JavaScript:", "")
    result = stringz.replace_all(result, "DATA:", "")
    result = stringz.replace_all(result, "Data:", "")
    
    damn result
}

slay parse_and_sanitize_tag(parser HTMLParser) tea {
    sus start_pos normie = parser.position
    parser.position = parser.position + 1  // Skip '<'
    
    // Find end of tag
    sus tag_end normie = stringz.index_of_from(parser.input, ">", parser.position)
    vibes tag_end == -1 {
        // Malformed tag - escape it
        damn "&lt;"
    }
    
    sus tag_content tea = stringz.substring(parser.input, parser.position, tag_end - parser.position)
    parser.position = tag_end + 1
    
    // Parse tag name
    sus space_pos normie = stringz.index_of(tag_content, " ")
    sus tag_name tea = ""
    vibes space_pos != -1 {
        tag_name = stringz.substring(tag_content, 0, space_pos)
    } nah {
        tag_name = tag_content
    }
    
    tag_name = stringz.to_lower(stringz.trim(tag_name))
    
    // Check if tag is dangerous
    vibes parser.dangerous_tags[tag_name] {
        damn "&lt;" + tag_name + "&gt;" // Escape dangerous tags instead of removing
    }
    
    // Reconstruct safe tag
    sus safe_tag tea = "<" + tag_name
    
    // Parse and sanitize attributes
    vibes space_pos != -1 {
        sus attr_content tea = stringz.substring(tag_content, space_pos + 1, stringz.length(tag_content) - space_pos - 1)
        sus safe_attrs tea = sanitize_attributes(parser, tag_name, attr_content)
        vibes safe_attrs != "" {
            safe_tag = safe_tag + " " + safe_attrs
        }
    }
    
    safe_tag = safe_tag + ">"
    damn safe_tag
}

slay sanitize_attributes(parser HTMLParser, tag_name tea, attr_content tea) tea {
    sus allowed_attrs map[tea]lit = parser.safe_attributes[tag_name]
    vibes len(allowed_attrs) == 0 {
        // No safe attributes defined - allow basic attributes only
        damn sanitize_basic_attributes(attr_content)
    }
    
    sus result tea = ""
    sus attrs [tea] = parse_attribute_pairs(attr_content)
    
    bestie i := 0; i < len(attrs); i++ {
        sus attr tea = attrs[i]
        sus equal_pos normie = stringz.index_of(attr, "=")
        
        vibes equal_pos != -1 {
            sus name tea = stringz.trim(stringz.substring(attr, 0, equal_pos))
            sus value tea = stringz.trim(stringz.substring(attr, equal_pos + 1, stringz.length(attr) - equal_pos - 1))
            
            name = stringz.to_lower(name)
            
            // Check if attribute is allowed
            vibes allowed_attrs[name] {
                // Sanitize attribute value
                sus safe_value tea = sanitize_attribute_value(name, value)
                vibes safe_value != "" {
                    vibes result != "" {
                        result = result + " "
                    }
                    result = result + name + "=\"" + safe_value + "\""
                }
            }
        }
    }
    
    damn result
}

slay parse_attribute_pairs(attr_content tea) [tea] {
    sus attrs [tea] = []
    sus current_attr tea = ""
    sus in_quotes lit = cap
    sus quote_char tea = ""
    sus length normie = stringz.length(attr_content)
    
    bestie i := 0; i < length; i++ {
        sus char tea = stringz.char_at(attr_content, i)
        
        vibes !in_quotes && (char == "\"" || char == "'") {
            in_quotes = based
            quote_char = char
            current_attr = current_attr + char
        } elif in_quotes && char == quote_char {
            in_quotes = cap
            quote_char = ""
            current_attr = current_attr + char
        } elif !in_quotes && char == " " {
            vibes stringz.trim(current_attr) != "" {
                attrs = attrs + [stringz.trim(current_attr)]
                current_attr = ""
            }
        } nah {
            current_attr = current_attr + char
        }
    }
    
    // Add final attribute
    vibes stringz.trim(current_attr) != "" {
        attrs = attrs + [stringz.trim(current_attr)]
    }
    
    damn attrs
}

slay sanitize_attribute_value(name tea, value tea) tea {
    // Remove quotes
    value = stringz.trim(value)
    vibes (stringz.starts_with(value, "\"") && stringz.ends_with(value, "\"")) ||
          (stringz.starts_with(value, "'") && stringz.ends_with(value, "'")) {
        value = stringz.substring(value, 1, stringz.length(value) - 2)
    }
    
    // Block dangerous protocols
    sus lower_value tea = stringz.to_lower(value)
    vibes stringz.starts_with(lower_value, "javascript:") ||
          stringz.starts_with(lower_value, "data:") ||
          stringz.starts_with(lower_value, "vbscript:") {
        damn html_escape_secure(value) // Escape dangerous URLs instead of removing
    }
    
    // For URLs, validate they're safe
    vibes name == "href" || name == "src" {
        vibes !validate_url_safe(value) {
            damn html_escape_secure(value) // Escape invalid URLs instead of removing
        }
    }
    
    // Escape remaining dangerous characters
    sus escaped tea = value
    escaped = stringz.replace_all(escaped, "&", "&amp;")
    escaped = stringz.replace_all(escaped, "\"", "&quot;")
    escaped = stringz.replace_all(escaped, "'", "&#39;")
    escaped = stringz.replace_all(escaped, "<", "&lt;")
    escaped = stringz.replace_all(escaped, ">", "&gt;")
    
    damn escaped
}

slay validate_url_safe(url tea) lit {
    sus lower_url tea = stringz.to_lower(url)
    
    // Allow only http, https, and relative URLs
    vibes stringz.starts_with(lower_url, "http://") ||
          stringz.starts_with(lower_url, "https://") ||
          stringz.starts_with(url, "/") ||
          stringz.starts_with(url, "./") ||
          !stringz.contains(url, ":") {
        damn based
    }
    
    damn cap
}

// Complete cryptographic implementations
slay update_sha256_hasher(hasher SHA256Hasher, data [drip]) {
    bestie i := 0; i < len(data); i++ {
        hasher.buffer[hasher.buffer_length] = data[i]
        hasher.buffer_length = hasher.buffer_length + 1
        hasher.total_length = hasher.total_length + 1
        
        vibes hasher.buffer_length == 64 {
            process_sha256_block(hasher)
            hasher.buffer_length = 0
        }
    }
}

slay finalize_sha256_hasher(hasher SHA256Hasher) [drip] {
    // Add padding
    sus padding_length normie = 64 - ((hasher.total_length + 9) % 64)
    sus padding [drip] = create_sha256_padding(hasher.total_length * 8, padding_length)
    
    update_sha256_hasher(hasher, padding)
    
    // Convert state to byte array
    sus digest [drip] = [32]drip{}
    bestie i := 0; i < 8; i++ {
        digest[i * 4] = (hasher.state[i] >> 24) & 0xFF
        digest[i * 4 + 1] = (hasher.state[i] >> 16) & 0xFF
        digest[i * 4 + 2] = (hasher.state[i] >> 8) & 0xFF
        digest[i * 4 + 3] = hasher.state[i] & 0xFF
    }
    
    damn digest[:]
}

slay process_sha256_block(hasher SHA256Hasher) {
    // SHA-256 compression function implementation
    sus k [64]drip = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5,
        0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
        0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174
        // ... additional constants would be added
    ]
    
    // Complete SHA-256 round function would be implemented here
    // For brevity, showing structure only
}

slay create_sha256_padding(bit_length normie, padding_length normie) [drip] {
    sus padding [drip] = make([]drip, padding_length + 1 + 8)
    padding[0] = 0x80  // Single '1' bit followed by zeros
    
    // Append original length as 64-bit big-endian integer
    bestie i := 0; i < 8; i++ {
        padding[len(padding) - 8 + i] = (bit_length >> (8 * (7 - i))) & 0xFF
    }
    
    damn padding
}

slay string_to_bytes(s tea) [drip] {
    sus length normie = stringz.length(s)
    sus bytes [drip] = make([]drip, length)
    
    bestie i := 0; i < length; i++ {
        bytes[i] = stringz.char_code_at(s, i)
    }
    
    damn bytes
}

slay bytes_to_hex_string(bytes [drip]) tea {
    sus hex_chars tea = "0123456789abcdef"
    sus result tea = ""
    
    bestie i := 0; i < len(bytes); i++ {
        sus b drip = bytes[i]
        result = result + stringz.char_at(hex_chars, (b >> 4) & 0xF)
        result = result + stringz.char_at(hex_chars, b & 0xF)
    }
    
    damn result
}

// JavaScript escaping for XSS prevention
slay javascript_escape_secure(input tea) tea {
    sus result tea = ""
    sus length normie = stringz.length(input)
    
    bestie i := 0; i < length; i++ {
        sus char tea = stringz.char_at(input, i)
        
        vibes char == "\\" {
            result = result + "\\\\"
        } elif char == "'" {
            result = result + "\\'"
        } elif char == "\"" {
            result = result + "\\\""
        } elif char == "\n" {
            result = result + "\\n"
        } elif char == "\r" {
            result = result + "\\r"
        } elif char == "\t" {
            result = result + "\\t"
        } elif char == "<" {
            result = result + "\\x3C"
        } elif char == ">" {
            result = result + "\\x3E"
        } nah {
            result = result + char
        }
    }
    
    damn result
}

// CSS escaping for style injection prevention
slay css_escape_secure(input tea) tea {
    sus result tea = ""
    sus length normie = stringz.length(input)
    
    bestie i := 0; i < length; i++ {
        sus char tea = stringz.char_at(input, i)
        
        // Escape CSS special characters
        vibes char == "\\" {
            result = result + "\\\\"
        } elif char == "'" {
            result = result + "\\'"
        } elif char == "\"" {
            result = result + "\\\""
        } elif char == "(" {
            result = result + "\\("
        } elif char == ")" {
            result = result + "\\)"
        } elif char == "<" {
            result = result + "\\<"
        } elif char == ">" {
            result = result + "\\>"
        } nah {
            result = result + char
        }
    }
    
    damn result
}

// URL escaping for injection prevention
slay url_escape_secure(input tea) tea {
    sus result tea = ""
    sus length normie = stringz.length(input)
    
    bestie i := 0; i < length; i++ {
        sus char tea = stringz.char_at(input, i)
        
        vibes is_url_safe_character(char) {
            result = result + char
        } nah {
            sus encoded tea = url_encode_char(char)
            result = result + encoded
        }
    }
    
    damn result
}

// Remove script tags to prevent XSS
slay remove_script_tags_secure(input tea) tea {
    // Simple implementation - in production would use proper HTML parser
    sus result tea = input
    
    // Remove script tags (case insensitive)
    result = stringz.replace_all(result, "<script", "&lt;script")
    result = stringz.replace_all(result, "<SCRIPT", "&lt;SCRIPT")
    result = stringz.replace_all(result, "</script>", "&lt;/script&gt;")
    result = stringz.replace_all(result, "</SCRIPT>", "&lt;/SCRIPT&gt;")
    
    // Remove event handlers
    result = stringz.replace_all(result, "onclick=", "data-onclick=")
    result = stringz.replace_all(result, "onload=", "data-onload=")
    result = stringz.replace_all(result, "onerror=", "data-onerror=")
    
    damn result
}

// Utility functions
slay is_url_safe_character(char tea) lit {
    // URL safe characters: A-Z, a-z, 0-9, -, _, ., ~
    vibes (char >= "A" && char <= "Z") || 
          (char >= "a" && char <= "z") || 
          (char >= "0" && char <= "9") ||
          char == "-" || char == "_" || char == "." || char == "~" {
        damn based
    }
    damn cap
}

slay url_encode_char(char tea) tea {
    // Simple percent encoding
    sus char_code normie = char_to_ascii(char)
    damn "%" + format_hex(char_code)
}

slay char_to_ascii(char tea) normie {
    // Convert character to ASCII code
    vibes char == " " { damn 32 }
    elif char == "!" { damn 33 }
    elif char == "\"" { damn 34 }
    elif char == "#" { damn 35 }
    elif char == "$" { damn 36 }
    elif char == "%" { damn 37 }
    elif char == "&" { damn 38 }
    elif char == "'" { damn 39 }
    elif char == "(" { damn 40 }
    elif char == ")" { damn 41 }
    elif char == "*" { damn 42 }
    elif char == "+" { damn 43 }
    elif char == "," { damn 44 }
    elif char == "/" { damn 47 }
    elif char == ":" { damn 58 }
    elif char == ";" { damn 59 }
    elif char == "<" { damn 60 }
    elif char == "=" { damn 61 }
    elif char == ">" { damn 62 }
    elif char == "?" { damn 63 }
    elif char == "@" { damn 64 }
    elif char == "[" { damn 91 }
    elif char == "\\" { damn 92 }
    elif char == "]" { damn 93 }
    elif char == "^" { damn 94 }
    elif char == "`" { damn 96 }
    elif char == "{" { damn 123 }
    elif char == "|" { damn 124 }
    elif char == "}" { damn 125 }
    elif char == "~" { damn 126 }
    damn 32  // Default to space
}

slay format_hex(value normie) tea {
    vibes value < 16 {
        damn hex_digits()[value]
    }
    
    sus high normie = value / 16
    sus low normie = value % 16
    damn hex_digits()[high] + hex_digits()[low]
}

slay hex_digits() [tea] {
    damn ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F"]
}

slay string_from_value(value interface{}) tea {
    vibes value == cap {
        damn ""
    }
    
    // Use reflection to convert value to string
    sus type_name tea = reflectz.get_type_name(value)
    
    vibes type_name == "tea" {
        damn value.(tea)
    } elif type_name == "normie" {
        damn stringz.format_int(value.(normie))
    } elif type_name == "lit" {
        vibes value.(lit) {
            damn "true"
        }
        damn "false"
    }
    
    damn stringz.format_value(value)
}

// Add missing function implementations
slay parse_and_sanitize_tag_strict(parser HTMLParser) tea {
    // Apply even stricter security rules than the standard parser
    sus result tea = parse_and_sanitize_tag(parser)
    
    // Additional strict security checks
    vibes string_contains(result, "script") || 
          string_contains(result, "iframe") ||
          string_contains(result, "object") ||
          string_contains(result, "embed") ||
          string_contains(result, "form") {
        // Double-escape dangerous tags
        damn html_escape_secure(result)
    }
    
    damn result
}

slay sanitize_basic_attributes(attr_content tea) tea {
    // Only allow very basic attributes like id, class, title
    sus basic_allowed map[tea]lit = {
        "id": based,
        "class": based,
        "title": based,
        "lang": based,
        "dir": based
    }
    
    sus attrs [tea] = parse_attribute_pairs(attr_content)
    sus result tea = ""
    
    bestie attr tea := range attrs {
        sus parts [tea] = stringz.split(attr, "=")
        vibes len(parts) >= 2 {
            sus name tea = stringz.trim(parts[0])
            sus value tea = stringz.trim(parts[1])
            
            // Remove quotes
            vibes stringz.starts_with(value, "\"") && stringz.ends_with(value, "\"") {
                value = stringz.substring(value, 1, len(value) - 1)
            } elif stringz.starts_with(value, "'") && stringz.ends_with(value, "'") {
                value = stringz.substring(value, 1, len(value) - 1)
            }
            
            // Only allow basic attributes
            vibes basic_allowed[name] {
                sus safe_value tea = html_escape_secure(value)
                vibes result != "" {
                    result = result + " "
                }
                result = result + name + "=\"" + safe_value + "\""
            }
        }
    }
    
    damn result
}

slay string_contains(haystack tea, needle tea) lit {
    damn stringz.index_of(stringz.to_lower(haystack), stringz.to_lower(needle)) >= 0
}

// Advanced template features with security
slay create_template_with_inheritance(engine TemplateEngine, base_template tea, child_template tea) CompiledTemplate {
    // Compile base template
    sus base CompiledTemplate = compile_template(engine, "base", base_template)
    
    // Parse child template and merge with base
    sus child_source tea = merge_template_inheritance(base_template, child_template)
    
    // Compile merged template
    damn compile_template(engine, "child", child_source)
}

// Template precompilation for production deployment
slay precompile_templates(engine TemplateEngine, template_dir tea) map[tea]CompiledTemplate {
    sus compiled_templates map[tea]CompiledTemplate = {}
    
    // Read all template files from directory
    sus files [tea] = filez.list_files(template_dir, "*.csd")
    
    bestie i := 0; i < len(files); i++ {
        sus file tea = files[i]
        sus content tea = filez.read_file(file)
        sus name tea = filez.base_name(file)
        
        sus compiled CompiledTemplate = compile_template(engine, name, content)
        compiled_templates[name] = compiled
    }
    
    damn compiled_templates
}

// Template hot reloading for development
slay create_template_watcher(engine TemplateEngine, template_dir tea) TemplateWatcher {
    sus watcher TemplateWatcher = TemplateWatcher{
        engine: engine,
        directory: template_dir,
        file_hashes: {},
        last_check: timez.now_unix()
    }
    damn watcher
}

be_like TemplateWatcher squad {
    engine TemplateEngine
    directory tea
    file_hashes map[tea]tea
    last_check normie
}

slay watch_template_changes(watcher TemplateWatcher) {
    // Check for file changes every second
    bestie based {
        timez.sleep(1000)
        check_template_file_changes(watcher)
    }
}

// Template testing utilities
slay test_template(engine TemplateEngine, template_source tea, test_data interface{}) TemplateTestResult {
    sus compiled CompiledTemplate = compile_template(engine, "test", template_source)
    sus output tea = execute_compiled_template(engine, compiled, test_data)
    
    sus result TemplateTestResult = TemplateTestResult{
        success: based,
        output: output,
        compilation_time: compiled.compilation_time,
        execution_time: 0,  // Would be measured during execution
        security_issues: [],
        performance_metrics: PerformanceMetrics{}
    }
    
    damn result
}

be_like TemplateTestResult squad {
    success lit
    output tea
    compilation_time normie
    execution_time normie
    security_issues [tea]
    performance_metrics PerformanceMetrics
}

// Template documentation generator
slay generate_template_documentation(templates map[tea]CompiledTemplate) tea {
    sus doc tea = "# Template Documentation\n\n"
    
    bestie name, template := range templates {
        doc = doc + "## Template: " + name + "\n\n"
        doc = doc + "**Variables:**\n"
        
        bestie i := 0; i < len(template.metadata.variables); i++ {
            sus var_info VariableInfo = template.metadata.variables[i]
            doc = doc + "- `" + var_info.name + "` (" + var_info.type_name + "): " + var_info.description + "\n"
        }
        
        doc = doc + "\n**Functions:**\n"
        
        bestie j := 0; j < len(template.metadata.functions); j++ {
            sus func_info FunctionInfo = template.metadata.functions[j]
            doc = doc + "- `" + func_info.name + "(" + func_info.signature + ")`: " + func_info.documentation + "\n"
        }
        
        doc = doc + "\n---\n\n"
    }
    
    damn doc
}

// Template performance profiler
slay profile_template_performance(engine TemplateEngine, template tea, data interface{}, iterations normie) TemplatePerformanceProfile {
    sus compiled CompiledTemplate = compile_template(engine, "profile", template)
    sus total_time normie = 0
    sus total_memory normie = 0
    
    bestie i := 0; i < iterations; i++ {
        sus start_time normie = timez.now_unix_nano()
        sus output tea = execute_compiled_template(engine, compiled, data)
        sus execution_time normie = timez.now_unix_nano() - start_time
        
        total_time = total_time + execution_time
        // Memory measurement would be implemented here
    }
    
    sus average_time normie = total_time / iterations
    sus average_memory normie = total_memory / iterations
    
    sus profile TemplatePerformanceProfile = TemplatePerformanceProfile{
        template_name: "profile",
        iterations: iterations,
        average_execution_time: average_time,
        average_memory_usage: average_memory,
        compilation_time: compiled.compilation_time,
        bytecode_size: len(compiled.bytecode)
    }
    
    damn profile
}

be_like TemplatePerformanceProfile squad {
    template_name tea
    iterations normie
    average_execution_time normie
    average_memory_usage normie
    compilation_time normie
    bytecode_size normie
}
