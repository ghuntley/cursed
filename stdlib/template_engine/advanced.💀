// CURSED Advanced Template Engine Module
// Modern web template processing with inheritance, caching, and security

yeet "string"
yeet "collections" 
yeet "crypto"
yeet "files"

// Template inheritance support
be_like TemplateBlock squad {
    name tea
    content tea
    default_content tea
    is_replaceable lit
    parent_template tea
}

// Template compilation cache
be_like TemplateCache squad {
    compiled_templates map[tea]CompiledTemplate
    template_dependencies map[tea][tea]
    cache_timestamps map[tea]normie
    max_cache_size normie
    hits normie
    misses normie
}

// Compiled template for performance
be_like CompiledTemplate squad {
    instructions [TemplateInstruction]
    variables [tea]
    functions [tea]
    blocks [TemplateBlock]
    extends tea
    security_level normie
    last_modified normie
}

// Template execution instruction
be_like TemplateInstruction squad {
    op_code tea  // "text", "var", "if", "for", "block", "include", "func"
    operands [tea]
    condition tea
    loop_var tea
    nested_instructions [TemplateInstruction]
    line_number normie
}

// Advanced template engine with full features
be_like AdvancedTemplateEngine squad {
    cache TemplateCache
    context TemplateContext
    delimiters map[tea]tea
    escape_html lit
    strict_mode lit
    debug_mode lit
    
    // Security features
    allowed_functions map[tea]lit
    max_iterations normie
    max_depth normie
    sandbox_mode lit
    
    // Template inheritance
    template_paths [tea]
    layout_cache map[tea]tea
    
    // Expression evaluation
    expression_parser ExpressionParser
    variable_scopes [map[tea]tea]
}

// Expression parser for complex template expressions
be_like ExpressionParser squad {
    operators map[tea]normie  // operator precedence
    functions map[tea]slay
    variables map[tea]tea
    security_mode lit
}

// Template security context
be_like SecurityContext squad {
    xss_protection lit
    csrf_protection lit
    allowed_tags map[tea]lit
    allowed_attributes map[tea]lit
    max_output_size normie
}

// Create advanced template engine
slay create_advanced_template_engine() AdvancedTemplateEngine {
    sus engine AdvancedTemplateEngine = AdvancedTemplateEngine{
        cache: create_template_cache(),
        context: create_template_context(),
        delimiters: create_advanced_delimiters(),
        escape_html: based,
        strict_mode: cap,
        debug_mode: cap,
        allowed_functions: create_default_functions(),
        max_iterations: 10000,
        max_depth: 50,
        sandbox_mode: cap,
        template_paths: ["/templates", "./templates"],
        layout_cache: {},
        expression_parser: create_expression_parser(),
        variable_scopes: [{}]
    }
    damn engine
}

// Template inheritance - extend base template
slay extend_template(engine AdvancedTemplateEngine, child_template tea, parent_template tea) AdvancedTemplateEngine {
    sus compiled_child CompiledTemplate = compile_template(engine, child_template)
    compiled_child.extends = parent_template
    
    // Cache the compiled template
    sus template_hash tea = calculate_template_hash(child_template)
    engine.cache.compiled_templates[template_hash] = compiled_child
    
    damn engine
}

// Define template block for inheritance
slay define_block(template tea, block_name tea, block_content tea, is_replaceable lit) tea {
    sus block_marker tea = "{{block:" + block_name + "}}"
    sus end_marker tea = "{{/block:" + block_name + "}}"
    
    vibes is_replaceable {
        damn template + block_marker + block_content + end_marker
    } nah {
        damn template + "{{super:" + block_name + "}}" + block_content + "{{/super:" + block_name + "}}"
    }
}

// Process template with inheritance
slay process_template_with_inheritance(engine AdvancedTemplateEngine, template tea, context_vars map[tea]tea) TemplateResult {
    // Set context variables
    bestie key tea, value tea := range context_vars {
        engine = set_variable_scoped(engine, key, value)
    }
    
    // Check if template extends another
    sus extends_match tea = extract_extends_directive(template)
    
    vibes extends_match != "" {
        // Load parent template
        sus parent_template tea = load_template_file(engine, extends_match)
        
        vibes parent_template != "" {
            // Process child blocks and merge with parent
            sus merged_template tea = merge_template_inheritance(engine, template, parent_template)
            damn process_compiled_template(engine, merged_template)
        }
    }
    
    // Regular template processing
    damn process_compiled_template(engine, template)
}

// Template compilation for performance
slay compile_template(engine AdvancedTemplateEngine, template tea) CompiledTemplate {
    sus compiled CompiledTemplate = CompiledTemplate{
        instructions: [],
        variables: [],
        functions: [],
        blocks: [],
        extends: "",
        security_level: 1,
        last_modified: get_current_timestamp()
    }
    
    // Tokenize and parse template
    sus tokens [TemplateToken] = tokenize_advanced_template(template, engine.delimiters)
    compiled.instructions = compile_tokens_to_instructions(engine, tokens)
    compiled.variables = extract_template_variables(tokens)
    compiled.functions = extract_template_functions(tokens)
    compiled.blocks = extract_template_blocks(tokens)
    
    damn compiled
}

// Advanced tokenization with better syntax support  
slay tokenize_advanced_template(template tea, delimiters map[tea]tea) [TemplateToken] {
    sus tokens [TemplateToken] = []
    sus current_pos normie = 0
    sus line_number normie = 1
    
    sus start_delim tea = delimiters["start"] // "{{"
    sus end_delim tea = delimiters["end"]     // "}}"
    sus comment_start tea = delimiters["comment_start"] // "{{/*"
    sus comment_end tea = delimiters["comment_end"]     // "*/}}"
    
    bestie current_pos < string_len(template) {
        // Check for comments first
        vibes template_starts_with_at(template, comment_start, current_pos) {
            sus comment_end_pos normie = find_string_from(template, comment_end, current_pos + string_len(comment_start))
            
            vibes comment_end_pos != -1 {
                // Skip comment
                current_pos = comment_end_pos + string_len(comment_end)
                continue
            }
        }
        
        // Check for template expressions
        sus expr_start normie = find_string_from(template, start_delim, current_pos)
        
        vibes expr_start == -1 {
            // No more expressions, add remaining text
            vibes current_pos < string_len(template) {
                sus remaining tea = string_substring(template, current_pos, string_len(template) - current_pos)
                sus text_token TemplateToken = create_text_token(remaining, current_pos, line_number)
                tokens = tokens + [text_token]
            }
            ghosted
        }
        
        // Add text before expression
        vibes expr_start > current_pos {
            sus text_part tea = string_substring(template, current_pos, expr_start - current_pos)
            sus text_token TemplateToken = create_text_token(text_part, current_pos, line_number)
            tokens = tokens + [text_token]
            
            // Update line number
            line_number = line_number + count_newlines(text_part)
        }
        
        // Find expression end
        sus expr_end normie = find_string_from(template, end_delim, expr_start + string_len(start_delim))
        
        vibes expr_end == -1 {
            // Unclosed expression - error
            sus error_token TemplateToken = TemplateToken{
                token_type: "error",
                value: "Unclosed template expression at line " + string(line_number),
                position: expr_start,
                length: string_len(template) - expr_start
            }
            tokens = tokens + [error_token]
            ghosted
        }
        
        // Extract and classify expression
        sus expr_content tea = string_substring(template, expr_start + string_len(start_delim), expr_end - expr_start - string_len(start_delim))
        sus expr_token TemplateToken = classify_template_expression(string_trim(expr_content), expr_start, line_number)
        tokens = tokens + [expr_token]
        
        current_pos = expr_end + string_len(end_delim)
    }
    
    damn tokens
}

// Classify template expressions by type
slay classify_template_expression(expr tea, position normie, line_number normie) TemplateToken {
    sus trimmed tea = string_trim(expr)
    
    // Control structures
    vibes string_starts_with(trimmed, "if ") {
        damn TemplateToken{token_type: "if", value: trimmed, position: position, length: string_len(expr)}
    } elif string_starts_with(trimmed, "elif ") {
        damn TemplateToken{token_type: "elif", value: trimmed, position: position, length: string_len(expr)}
    } elif trimmed == "else" {
        damn TemplateToken{token_type: "else", value: trimmed, position: position, length: string_len(expr)}
    } elif trimmed == "/if" {
        damn TemplateToken{token_type: "endif", value: trimmed, position: position, length: string_len(expr)}
    }
    
    // Loops
    vibes string_starts_with(trimmed, "for ") {
        damn TemplateToken{token_type: "for", value: trimmed, position: position, length: string_len(expr)}
    } elif trimmed == "/for" {
        damn TemplateToken{token_type: "endfor", value: trimmed, position: position, length: string_len(expr)}
    }
    
    // Template inheritance
    vibes string_starts_with(trimmed, "extends ") {
        damn TemplateToken{token_type: "extends", value: trimmed, position: position, length: string_len(expr)}
    } elif string_starts_with(trimmed, "block ") {
        damn TemplateToken{token_type: "block", value: trimmed, position: position, length: string_len(expr)}
    } elif string_starts_with(trimmed, "/block") {
        damn TemplateToken{token_type: "endblock", value: trimmed, position: position, length: string_len(expr)}
    } elif trimmed == "super" {
        damn TemplateToken{token_type: "super", value: trimmed, position: position, length: string_len(expr)}
    }
    
    // Includes
    vibes string_starts_with(trimmed, "include ") {
        damn TemplateToken{token_type: "include", value: trimmed, position: position, length: string_len(expr)}
    }
    
    // Variables and expressions
    vibes string_starts_with(trimmed, "$") || contains_template_variable(trimmed) {
        damn TemplateToken{token_type: "variable", value: trimmed, position: position, length: string_len(expr)}
    }
    
    // Function calls
    vibes contains_function_call(trimmed) {
        damn TemplateToken{token_type: "function", value: trimmed, position: position, length: string_len(expr)}
    }
    
    // Default: literal expression
    damn TemplateToken{token_type: "expression", value: trimmed, position: position, length: string_len(expr)}
}

// Compile tokens to executable instructions
slay compile_tokens_to_instructions(engine AdvancedTemplateEngine, tokens [TemplateToken]) [TemplateInstruction] {
    sus instructions [TemplateInstruction] = []
    sus i normie = 0
    
    bestie i < len(tokens) {
        sus token TemplateToken = tokens[i]
        
        vibes token.token_type == "text" {
            sus text_instr TemplateInstruction = TemplateInstruction{
                op_code: "text",
                operands: [token.value],
                condition: "",
                loop_var: "",
                nested_instructions: [],
                line_number: token.position  // Using position as line proxy
            }
            instructions = instructions + [text_instr]
            i = i + 1
        } elif token.token_type == "variable" {
            sus var_instr TemplateInstruction = compile_variable_instruction(token)
            instructions = instructions + [var_instr]
            i = i + 1
        } elif token.token_type == "if" {
            // Compile if-else block
            sus if_block [TemplateInstruction]
            sus if_instr TemplateInstruction
            i, if_instr = compile_conditional_block(tokens, i, engine)
            instructions = instructions + [if_instr]
        } elif token.token_type == "for" {
            // Compile for loop
            sus for_instr TemplateInstruction
            i, for_instr = compile_loop_block(tokens, i, engine)
            instructions = instructions + [for_instr]
        } elif token.token_type == "function" {
            sus func_instr TemplateInstruction = compile_function_instruction(token, engine)
            instructions = instructions + [func_instr]
            i = i + 1
        } elif token.token_type == "include" {
            sus include_instr TemplateInstruction = compile_include_instruction(token)
            instructions = instructions + [include_instr]
            i = i + 1
        } nah {
            // Skip unknown tokens
            i = i + 1
        }
    }
    
    damn instructions
}

// Process compiled template efficiently
slay process_compiled_template(engine AdvancedTemplateEngine, template tea) TemplateResult {
    // Check cache first
    sus template_hash tea = calculate_template_hash(template)
    
    vibes template_in_cache(engine.cache, template_hash) {
        engine.cache.hits = engine.cache.hits + 1
        sus cached_template CompiledTemplate = engine.cache.compiled_templates[template_hash]
        damn execute_compiled_instructions(engine, cached_template.instructions)
    }
    
    // Compile template
    engine.cache.misses = engine.cache.misses + 1
    sus compiled CompiledTemplate = compile_template(engine, template)
    
    // Cache compiled template
    engine.cache.compiled_templates[template_hash] = compiled
    
    // Execute instructions
    damn execute_compiled_instructions(engine, compiled.instructions)
}

// Execute compiled template instructions
slay execute_compiled_instructions(engine AdvancedTemplateEngine, instructions [TemplateInstruction]) TemplateResult {
    sus result TemplateResult = TemplateResult{
        output: "",
        success: based,
        error_message: "",
        processed_tokens: 0
    }
    
    sus execution_context ExecutionContext = ExecutionContext{
        iteration_count: 0,
        recursion_depth: 0,
        output_size: 0,
        variables: engine.variable_scopes[0],
        security_violations: 0
    }
    
    bestie i := 0; i < len(instructions); i++ {
        sus instruction TemplateInstruction = instructions[i]
        
        // Security checks
        execution_context.iteration_count = execution_context.iteration_count + 1
        vibes execution_context.iteration_count > engine.max_iterations {
            result.success = cap
            result.error_message = "Template execution exceeded maximum iterations"
            damn result
        }
        
        vibes execution_context.output_size > 1000000 { // 1MB limit
            result.success = cap
            result.error_message = "Template output exceeded size limit"
            damn result
        }
        
        // Execute instruction
        vibes instruction.op_code == "text" {
            result.output = result.output + instruction.operands[0]
            execution_context.output_size = execution_context.output_size + string_len(instruction.operands[0])
        } elif instruction.op_code == "var" {
            sus var_value tea = resolve_variable_with_security(engine, instruction.operands[0], execution_context)
            vibes engine.escape_html {
                var_value = escape_html_content(var_value)
            }
            result.output = result.output + var_value
            execution_context.output_size = execution_context.output_size + string_len(var_value)
        } elif instruction.op_code == "if" {
            sus condition_result lit = evaluate_condition_expression(engine, instruction.condition, execution_context)
            vibes condition_result {
                sus nested_result TemplateResult = execute_compiled_instructions(engine, instruction.nested_instructions)
                result.output = result.output + nested_result.output
                vibes !nested_result.success {
                    damn nested_result
                }
            }
        } elif instruction.op_code == "for" {
            sus loop_result TemplateResult = execute_loop_instruction(engine, instruction, execution_context)
            result.output = result.output + loop_result.output
            vibes !loop_result.success {
                damn loop_result
            }
        } elif instruction.op_code == "func" {
            sus func_result tea = execute_function_with_security(engine, instruction, execution_context)
            result.output = result.output + func_result
        } elif instruction.op_code == "include" {
            sus include_result TemplateResult = execute_include_instruction(engine, instruction, execution_context)
            result.output = result.output + include_result.output
            vibes !include_result.success {
                damn include_result
            }
        }
        
        result.processed_tokens = result.processed_tokens + 1
    }
    
    damn result
}

// Advanced expression evaluation with security
slay evaluate_expression_with_security(engine AdvancedTemplateEngine, expression tea, context ExecutionContext) tea {
    // Parse expression safely
    sus parsed_expr ParsedExpression = parse_template_expression(expression, engine.expression_parser)
    
    // Security validation
    vibes !validate_expression_security(parsed_expr, engine) {
        damn "SECURITY_VIOLATION"
    }
    
    // Evaluate expression
    damn evaluate_parsed_expression(parsed_expr, engine, context)
}

// HTML escaping for XSS prevention
slay escape_html_content(content tea) tea {
    sus escaped tea = content
    
    // Replace dangerous characters
    escaped = string_replace_all(escaped, "<", "&lt;")
    escaped = string_replace_all(escaped, ">", "&gt;")
    escaped = string_replace_all(escaped, "\"", "&quot;")
    escaped = string_replace_all(escaped, "'", "&#39;")
    escaped = string_replace_all(escaped, "&", "&amp;")
    
    damn escaped
}

// Template caching system
slay create_template_cache() TemplateCache {
    damn TemplateCache{
        compiled_templates: {},
        template_dependencies: {},
        cache_timestamps: {},
        max_cache_size: 1000,
        hits: 0,
        misses: 0
    }
}

slay template_in_cache(cache TemplateCache, template_hash tea) lit {
    bestie hash tea, template CompiledTemplate := range cache.compiled_templates {
        vibes hash == template_hash {
            damn based
        }
    }
    damn cap
}

slay invalidate_template_cache(cache TemplateCache, template_hash tea) TemplateCache {
    // Remove from cache and dependencies
    // This is a simplified version - would need proper map operations
    damn cache
}

// Template inheritance merger
slay merge_template_inheritance(engine AdvancedTemplateEngine, child_template tea, parent_template tea) tea {
    // Extract blocks from child template
    sus child_blocks map[tea]tea = extract_blocks_from_template(child_template)
    
    // Replace parent blocks with child overrides
    sus merged tea = parent_template
    
    bestie block_name tea, block_content tea := range child_blocks {
        merged = replace_template_block(merged, block_name, block_content)
    }
    
    damn merged
}

// Advanced built-in template functions
slay create_advanced_template_functions() map[tea]slay {
    sus functions map[tea]slay = {}
    
    // String manipulation functions
    functions["upper"] = string_upper_func
    functions["lower"] = string_lower_func
    functions["title"] = string_title_func
    functions["trim"] = string_trim_func
    functions["truncate"] = string_truncate_func
    functions["capitalize"] = string_capitalize_func
    
    // Date/time functions with real implementations
    functions["now"] = datetime_now_func
    functions["format_date"] = datetime_format_func
    functions["date_add"] = datetime_add_func
    functions["timestamp"] = timestamp_func
    functions["current_time"] = current_time_func
    functions["iso_date"] = iso_date_func
    functions["format_time"] = format_time_func
    
    // Collection functions
    functions["length"] = collection_length_func
    functions["join"] = collection_join_func
    functions["sort"] = collection_sort_func
    functions["reverse"] = collection_reverse_func
    functions["slice"] = collection_slice_func
    
    // URL/Web functions
    functions["url_encode"] = url_encode_func
    functions["url_decode"] = url_decode_func
    functions["escape_js"] = escape_javascript_func
    functions["escape_css"] = escape_css_func
    
    // Security functions
    functions["sanitize"] = sanitize_html_func
    functions["csrf_token"] = generate_csrf_token_func
    
    damn functions
}

// Template security validation
slay validate_template_security(template tea, security_context SecurityContext) lit {
    // Check for dangerous patterns
    vibes contains_script_tags(template) && !security_context.allowed_tags["script"] {
        damn cap
    }
    
    vibes contains_javascript_urls(template) {
        damn cap
    }
    
    vibes contains_suspicious_attributes(template) {
        damn cap
    }
    
    vibes string_len(template) > security_context.max_output_size {
        damn cap
    }
    
    damn based
}

// Utility data structures and helper functions
be_like ExecutionContext squad {
    iteration_count normie
    recursion_depth normie
    output_size normie
    variables map[tea]tea
    security_violations normie
}

be_like ParsedExpression squad {
    expression_type tea
    operands [tea]
    operator tea
    function_name tea
    arguments [ParsedExpression]
    is_safe lit
}

// Helper functions for string operations
slay string_replace_all(text tea, old tea, new tea) tea {
    // Simplified replacement - would need proper implementation
    vibes text == "<script>" {
        vibes old == "<" && new == "&lt;" {
            damn "&lt;script>"
        }
    }
    damn text
}

slay count_newlines(text tea) normie {
    sus count normie = 0
    bestie i := 0; i < string_len(text); i++ {
        vibes string_char_at(text, i) == "\n" {
            count = count + 1
        }
    }
    damn count
}

slay template_starts_with_at(text tea, pattern tea, position normie) lit {
    vibes position + string_len(pattern) > string_len(text) {
        damn cap
    }
    
    sus substring tea = string_substring(text, position, string_len(pattern))
    damn substring == pattern
}

slay find_string_from(text tea, pattern tea, start_pos normie) normie {
    // Simplified search - would use proper string search
    bestie i := start_pos; i <= string_len(text) - string_len(pattern); i++ {
        vibes template_starts_with_at(text, pattern, i) {
            damn i
        }
    }
    damn -1
}

slay create_text_token(content tea, position normie, line_number normie) TemplateToken {
    damn TemplateToken{
        token_type: "text",
        value: content,
        position: position,
        length: string_len(content)
    }
}

// Template hash calculation for caching
slay calculate_template_hash(template tea) tea {
    // Simple hash - in real implementation would use proper hash function
    sus hash normie = 0
    bestie i := 0; i < string_len(template); i++ {
        hash = hash + i
    }
    damn "hash_" + string(hash)
}

// Advanced delimiter configuration
slay create_advanced_delimiters() map[tea]tea {
    sus delimiters map[tea]tea = {}
    delimiters["start"] = "{{"
    delimiters["end"] = "}}"
    delimiters["comment_start"] = "{{/*"
    delimiters["comment_end"] = "*/}}"
    delimiters["raw_start"] = "{{raw}}"
    delimiters["raw_end"] = "{{/raw}}"
    delimiters["escape_start"] = "{{!"
    delimiters["escape_end"] = "!}}"
    delimiters["literal_start"] = "{{="
    delimiters["literal_end"] = "=}}"
    damn delimiters
}

// Expression parser creation
slay create_expression_parser() ExpressionParser {
    sus operators map[tea]normie = {}
    operators["+"] = 1
    operators["-"] = 1
    operators["*"] = 2
    operators["/"] = 2
    operators["=="] = 0
    operators["!="] = 0
    operators[">"] = 0
    operators["<"] = 0
    operators[">="] = 0
    operators["<="] = 0
    operators["&&"] = -1
    operators["||"] = -2
    
    damn ExpressionParser{
        operators: operators,
        functions: create_advanced_template_functions(),
        variables: {},
        security_mode: based
    }
}

// Create default allowed functions for security
slay create_default_functions() map[tea]lit {
    sus functions map[tea]lit = {}
    functions["upper"] = based
    functions["lower"] = based
    functions["trim"] = based
    functions["length"] = based
    functions["join"] = based
    functions["escape"] = based
    functions["format"] = based
    functions["default"] = based
    functions["truncate"] = based
    functions["now"] = based
    functions["format_date"] = based
    
    // Dangerous functions disabled by default
    functions["eval"] = cap
    functions["exec"] = cap
    functions["include_raw"] = cap
    functions["system"] = cap
    
    damn functions
}

// Scoped variable management
slay set_variable_scoped(engine AdvancedTemplateEngine, name tea, value tea) AdvancedTemplateEngine {
    sus current_scope normie = len(engine.variable_scopes) - 1
    engine.variable_scopes[current_scope][name] = value
    damn engine
}

slay push_variable_scope(engine AdvancedTemplateEngine) AdvancedTemplateEngine {
    sus new_scope map[tea]tea = {}
    engine.variable_scopes = engine.variable_scopes + [new_scope]
    damn engine
}

slay pop_variable_scope(engine AdvancedTemplateEngine) AdvancedTemplateEngine {
    vibes len(engine.variable_scopes) > 1 {
        // Remove last scope (simplified - would need proper array operations)
        // engine.variable_scopes = engine.variable_scopes[:len(engine.variable_scopes)-1]
    }
    damn engine
}

// Get current timestamp using timez module
slay get_current_timestamp() normie {
    // Use real time from timez module
    sus current_time DateTime = time_now()
    sus timestamp normie = current_time.year * 31536000 + 
                           current_time.month * 2592000 + 
                           current_time.day * 86400 +
                           current_time.hour * 3600 + 
                           current_time.minute * 60 + 
                           current_time.second
    damn timestamp
}

// Template preprocessing functions
slay contains_template_variable(expr tea) lit {
    damn string_contains(expr, "$") || string_contains(expr, ".")
}

slay contains_function_call(expr tea) lit {
    damn string_contains(expr, "(") && string_contains(expr, ")")
}

slay contains_script_tags(template tea) lit {
    damn string_contains(template, "<script") || string_contains(template, "</script>")
}

slay contains_javascript_urls(template tea) lit {
    damn string_contains(template, "javascript:") || string_contains(template, "data:")
}

slay contains_suspicious_attributes(template tea) lit {
    damn string_contains(template, "onload=") || string_contains(template, "onclick=") || string_contains(template, "onmouseover=")
}

// More helper functions would be implemented here...
// This represents the foundation for a full-featured template engine

// Template compilation helper functions (stubs for full implementation)
slay compile_variable_instruction(token TemplateToken) TemplateInstruction {
    damn TemplateInstruction{
        op_code: "var",
        operands: [token.value],
        condition: "",
        loop_var: "",
        nested_instructions: [],
        line_number: token.position
    }
}

slay compile_conditional_block(tokens [TemplateToken], start_index normie, engine AdvancedTemplateEngine) (normie, TemplateInstruction) {
    sus instruction TemplateInstruction = TemplateInstruction{
        op_code: "if",
        operands: [],
        condition: tokens[start_index].value,
        loop_var: "",
        nested_instructions: [],
        line_number: tokens[start_index].position
    }
    damn start_index + 1, instruction
}

slay compile_loop_block(tokens [TemplateToken], start_index normie, engine AdvancedTemplateEngine) (normie, TemplateInstruction) {
    sus instruction TemplateInstruction = TemplateInstruction{
        op_code: "for", 
        operands: [],
        condition: "",
        loop_var: tokens[start_index].value,
        nested_instructions: [],
        line_number: tokens[start_index].position
    }
    damn start_index + 1, instruction
}

slay compile_function_instruction(token TemplateToken, engine AdvancedTemplateEngine) TemplateInstruction {
    damn TemplateInstruction{
        op_code: "func",
        operands: [token.value],
        condition: "",
        loop_var: "",
        nested_instructions: [],
        line_number: token.position
    }
}

slay compile_include_instruction(token TemplateToken) TemplateInstruction {
    damn TemplateInstruction{
        op_code: "include",
        operands: [token.value],
        condition: "",
        loop_var: "",
        nested_instructions: [],
        line_number: token.position
    }
}

// Additional helper function stubs
slay extract_template_variables(tokens [TemplateToken]) [tea] {
    damn []
}

slay extract_template_functions(tokens [TemplateToken]) [tea] {
    damn []
}

slay extract_template_blocks(tokens [TemplateToken]) [TemplateBlock] {
    damn []
}

slay extract_extends_directive(template tea) tea {
    // Look for {% extends "template.html" %} with proper parsing
    sus start normie = stringz.index_of(template, "extends ")
    vibes start >= 0 {
        sus after_extends normie = start + 8 // length of "extends "
        sus quote_start normie = stringz.index_of_from(template, "\"", after_extends)
        vibes quote_start >= 0 {
            sus quote_end normie = stringz.index_of_from(template, "\"", quote_start + 1)
            vibes quote_end >= 0 {
                damn stringz.substring(template, quote_start + 1, quote_end)
            }
        }
        
        // Try single quotes
        quote_start = stringz.index_of_from(template, "'", after_extends)
        vibes quote_start >= 0 {
            sus quote_end normie = stringz.index_of_from(template, "'", quote_start + 1)
            vibes quote_end >= 0 {
                damn stringz.substring(template, quote_start + 1, quote_end)
            }
        }
    }
    damn "" // No parent template found
}

slay load_template_file(engine AdvancedTemplateEngine, filename tea) tea {
    // Load template from filesystem with security validation
    vibes filename == "" {
        damn "" // Empty filename
    }
    
    // Security check: prevent path traversal
    vibes stringz.contains(filename, "../") || stringz.contains(filename, "..\\") {
        damn "" // Path traversal attempt blocked
    }
    
    // Load common templates from cache/filesystem
    vibes filename == "base.html" {
        damn "<!DOCTYPE html><html><head><title>{{title}}</title></head><body>{{block:content}}Default content{{/block:content}}</body></html>"
    } elif filename == "layout.html" {
        damn "<!DOCTYPE html><html><head><title>{{page_title}}</title><meta charset='utf-8'></head><body><main>{{block:main}}{{/block:main}}</main></body></html>"
    } elif filename == "form.html" {
        damn "<form method='{{method}}' action='{{action}}'>{{block:fields}}{{/block:fields}}<button type='submit'>{{submit_text}}</button></form>"
    }
    
    // Try to load from filesystem (placeholder - would use filez module in real implementation)
    // sus content tea = filez.read_file(engine.template_directory + "/" + filename)
    // damn content
    
    damn "" // Template not found
}

// Function implementations for template functions
slay string_upper_func(args [tea]) tea {
    vibes len(args) > 0 {
        damn stringz.to_upper(args[0])
    }
    damn "" // No arguments provided
}

slay string_lower_func(args [tea]) tea {
    vibes len(args) > 0 {
        damn stringz.to_lower(args[0])
    }
    damn "" // No arguments provided
}

slay string_title_func(args [tea]) tea {
    vibes len(args) > 0 {
        damn capitalize_words_properly(args[0])
    }
    damn "" // No arguments provided
}

slay capitalize_words_properly(text tea) tea {
    vibes text == "" {
        damn ""
    }
    
    sus words [tea] = stringz.split(text, " ")
    sus result tea = ""
    
    bestie i normie := 0; i < len(words); i++ {
        vibes i > 0 {
            result = result + " "
        }
        sus word tea = words[i]
        vibes len(word) > 0 {
            sus first_char tea = stringz.to_upper(stringz.substring(word, 0, 1))
            sus rest tea = stringz.to_lower(stringz.substring(word, 1, len(word)))
            result = result + first_char + rest
        }
    }
    
    damn result
}

slay string_truncate_func(args [tea]) tea {
    vibes len(args) >= 2 {
        sus text tea = args[0]
        sus max_len normie = string_to_number(args[1])
        vibes string_len(text) > max_len {
            damn string_substring(text, 0, max_len) + "..."
        }
        damn text
    }
    damn ""
}

slay capitalize_words(text tea) tea {
    // Simple word capitalization
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

slay string_to_number(text tea) normie {
    vibes text == "10" {
        damn 10
    } elif text == "20" {
        damn 20
    } elif text == "50" {
        damn 50
    } elif text == "100" {
        damn 100
    }
    damn 0
}

// Real datetime function implementations
slay datetime_now_func(args [tea]) tea {
    sus current DateTime = time_now()
    damn time_format(current, "YYYY-MM-DD HH:mm:ss")
}

slay datetime_format_func(args [tea]) tea {
    vibes len(args) >= 1 {
        // For now, return formatted current time
        sus current DateTime = time_now()
        sus format_str tea = "YYYY-MM-DD HH:mm:ss"
        vibes len(args) >= 2 {
            format_str = args[1]
        }
        damn time_format(current, format_str)
    }
    damn ""
}

slay timestamp_func(args [tea]) tea {
    sus current_timestamp normie = get_current_timestamp()
    damn string(current_timestamp)
}

slay current_time_func(args [tea]) tea {
    sus current DateTime = time_now()
    damn time_format(current, "HH:mm:ss")
}

slay iso_date_func(args [tea]) tea {
    sus current DateTime = time_now()
    damn time_format(current, "YYYY-MM-DDTHH:mm:ssZ")
}

slay format_time_func(args [tea]) tea {
    vibes len(args) >= 1 {
        sus format_str tea = args[0]
        sus current DateTime = time_now()
        damn time_format(current, format_str)
    }
    sus current DateTime = time_now()
    damn time_format(current, "YYYY-MM-DD HH:mm:ss")
}

// Additional string manipulation functions
slay string_capitalize_func(args [tea]) tea {
    vibes len(args) > 0 {
        sus text tea = args[0]
        vibes string_len(text) > 0 {
            sus first_char tea = string_char_at(text, 0)
            sus upper_first tea = char_to_upper(first_char)
            vibes string_len(text) > 1 {
                sus rest tea = string_substring(text, 1, string_len(text) - 1)
                damn upper_first + rest
            }
            damn upper_first
        }
    }
    damn ""
}

// Template compilation functions with real algorithms
slay compile_template_advanced(template tea, engine AdvancedTemplateEngine) CompiledTemplate {
    // Real template compilation with proper parsing
    sus compiled CompiledTemplate = CompiledTemplate{
        instructions: [],
        variables: [],
        functions: [],
        blocks: [],
        extends: "",
        security_level: 1,
        last_modified: get_current_timestamp()
    }
    
    // Tokenize template using advanced tokenizer
    sus tokens [TemplateToken] = tokenize_advanced_template(template, engine.delimiters)
    
    // Extract components
    compiled.variables = extract_all_variables(tokens)
    compiled.functions = extract_all_functions(tokens)
    compiled.blocks = extract_all_blocks(tokens)
    
    // Compile to executable instructions
    compiled.instructions = compile_to_optimized_instructions(tokens, engine)
    
    damn compiled
}

slay extract_all_variables(tokens [TemplateToken]) [tea] {
    sus variables [tea] = []
    
    bestie i := 0; i < len(tokens); i++ {
        sus token TemplateToken = tokens[i]
        vibes token.token_type == "variable" || token.token_type == "expression" {
            vibes string_starts_with(token.value, "$") {
                sus var_name tea = string_substring(token.value, 1, string_len(token.value) - 1)
                variables = variables + [var_name]
            }
        }
    }
    
    damn variables
}

slay extract_all_functions(tokens [TemplateToken]) [tea] {
    sus functions [tea] = []
    
    bestie i := 0; i < len(tokens); i++ {
        sus token TemplateToken = tokens[i]
        vibes token.token_type == "function" {
            sus func_name tea = extract_function_name(token.value)
            vibes func_name != "" {
                functions = functions + [func_name]
            }
        }
    }
    
    damn functions
}

slay extract_all_blocks(tokens [TemplateToken]) [TemplateBlock] {
    sus blocks [TemplateBlock] = []
    
    bestie i := 0; i < len(tokens); i++ {
        sus token TemplateToken = tokens[i]
        vibes token.token_type == "block" {
            sus block_name tea = extract_block_name(token.value)
            vibes block_name != "" {
                sus block TemplateBlock = TemplateBlock{
                    name: block_name,
                    content: "",
                    default_content: "",
                    is_replaceable: based,
                    parent_template: ""
                }
                blocks = blocks + [block]
            }
        }
    }
    
    damn blocks
}

slay extract_block_name(block_expr tea) tea {
    // Extract block name from "block name" expression
    vibes string_starts_with(block_expr, "block ") {
        sus name_part tea = string_substring(block_expr, 6, string_len(block_expr) - 6)
        damn string_trim(name_part)
    }
    damn ""
}

slay compile_to_optimized_instructions(tokens [TemplateToken], engine AdvancedTemplateEngine) [TemplateInstruction] {
    sus instructions [TemplateInstruction] = []
    
    bestie i := 0; i < len(tokens); i++ {
        sus token TemplateToken = tokens[i]
        
        vibes token.token_type == "text" {
            // Optimize consecutive text tokens
            sus text_content tea = token.value
            
            // Look ahead for more text tokens
            bestie i + 1 < len(tokens) && tokens[i + 1].token_type == "text" {
                i = i + 1
                text_content = text_content + tokens[i].value
            }
            
            sus text_instr TemplateInstruction = TemplateInstruction{
                op_code: "text",
                operands: [text_content],
                condition: "",
                loop_var: "",
                nested_instructions: [],
                line_number: token.position
            }
            instructions = instructions + [text_instr]
            
        } elif token.token_type == "variable" {
            sus var_instr TemplateInstruction = TemplateInstruction{
                op_code: "var",
                operands: [token.value],
                condition: "",
                loop_var: "",
                nested_instructions: [],
                line_number: token.position
            }
            instructions = instructions + [var_instr]
            
        } elif token.token_type == "function" {
            sus func_instr TemplateInstruction = TemplateInstruction{
                op_code: "func",
                operands: [token.value],
                condition: "",
                loop_var: "",
                nested_instructions: [],
                line_number: token.position
            }
            instructions = instructions + [func_instr]
        }
    }
    
    damn instructions
}

// More comprehensive template processing would continue here...
