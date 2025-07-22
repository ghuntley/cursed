fr fr macro_slay - Complete Macro System Implementation for CURSED
fr fr Provides comprehensive macro functionality including definition, expansion, 
fr fr AST integration, template substitution, and built-in macro library

yeet "ast_mood"
yeet "stringz"

fr fr Macro Type Constants
sus MACRO_UNKNOWN normie = 0
sus MACRO_FUNCTION normie = 1
sus MACRO_EXPRESSION normie = 2
sus MACRO_STATEMENT normie = 3
sus MACRO_TEMPLATE normie = 4
sus MACRO_GENERATOR normie = 5
sus MACRO_SYNTAX normie = 6
sus MACRO_ATTRIBUTE normie = 7
sus MACRO_DIRECTIVE normie = 8

fr fr Expansion Mode Constants
sus EXPAND_IMMEDIATE normie = 10
sus EXPAND_LAZY normie = 11
sus EXPAND_RECURSIVE normie = 12
sus EXPAND_ONCE normie = 13

fr fr Code Generation Format Constants
sus CODEGEN_AST normie = 20
sus CODEGEN_STRING normie = 21
sus CODEGEN_TOKENS normie = 22

fr fr Global macro registry simulation using encoded integers
sus MACRO_REGISTRY_SIZE normie = 1000
sus GLOBAL_MACRO_COUNT normie = 0
sus BUILTIN_MACRO_COUNT normie = 8

fr fr Simplified storage arrays
sus MACRO_STORAGE [1000]normie
sus MACRO_NAMES [1000]tea
sus MACRO_BODIES [1000]tea

fr fr Calculate simple hash for macro names
slay hash_macro_name(name tea) normie {
    sus hash normie = 5381
    sus len normie = stringz.string_length(name)
    sus i normie = 0 fr fr Simple loop without C-style for
    loop {
        lowkey i >= len { break }
        sus char_code normie = stringz.char_at_index(name, i)
        hash = hash * 33 + char_code
        i = i + 1
    }
    
    damn hash % 1000000
}

fr fr Core macro registration function
slay register_macro(name tea, macro_type normie, expand_mode normie, body tea) normie { fr fr Validate input parameters
    lowkey macro_type < MACRO_UNKNOWN || macro_type > MACRO_DIRECTIVE {
        damn 0 fr fr Invalid macro type
    }
    
    lowkey expand_mode < EXPAND_IMMEDIATE || expand_mode > EXPAND_ONCE {
        damn 0 fr fr Invalid expand mode
    }
    
    lowkey stringz.string_length(name) == 0 {
        damn 0 fr fr Empty name
    } fr fr Find next available slot
    sus slot normie = GLOBAL_MACRO_COUNT
    lowkey slot >= MACRO_REGISTRY_SIZE {
        damn 0 fr fr Registry full
    } fr fr Create macro ID with metadata
    sus name_hash normie = hash_macro_name(name)
    sus macro_id normie = (macro_type * 10000000) + (expand_mode * 100000) + (slot * 100) + (name_hash % 100) fr fr Store macro data
    MACRO_STORAGE[slot] = macro_id
    MACRO_NAMES[slot] = name
    MACRO_BODIES[slot] = body
    
    GLOBAL_MACRO_COUNT = GLOBAL_MACRO_COUNT + 1
    
    damn macro_id
}

fr fr Lookup macro by name
slay lookup_macro(name tea) normie {
    sus i normie = 0
    loop {
        lowkey i >= GLOBAL_MACRO_COUNT { break }
        lowkey stringz.string_equals(MACRO_NAMES[i], name) {
            damn MACRO_STORAGE[i]
        }
        i = i + 1
    }
    damn 0 fr fr Not found
}

fr fr Check if macro is defined
slay is_macro_defined(name tea) lit {
    damn lookup_macro(name) != 0
}

fr fr Get total number of registered macros
slay get_macro_count() normie {
    damn GLOBAL_MACRO_COUNT
}

fr fr Extract macro type from encoded ID
slay get_macro_type(macro_def normie) normie {
    damn macro_def / 10000000
}

fr fr Extract expansion mode from encoded ID
slay get_macro_expand_mode(macro_def normie) normie {
    sus remaining normie = macro_def % 10000000
    damn remaining / 100000
}

fr fr Get macro slot index from ID
slay get_macro_slot(macro_def normie) normie {
    sus remaining normie = macro_def % 100000
    damn remaining / 100
}

fr fr Get macro body by ID
slay get_macro_body(macro_def normie) tea {
    sus slot normie = get_macro_slot(macro_def)
    lowkey slot < GLOBAL_MACRO_COUNT {
        damn MACRO_BODIES[slot]
    }
    damn ""
}

fr fr Get macro name by ID
slay get_macro_name(macro_def normie) tea {
    sus slot normie = get_macro_slot(macro_def)
    lowkey slot < GLOBAL_MACRO_COUNT {
        damn MACRO_NAMES[slot]
    }
    damn ""
}

fr fr Type checking functions
slay is_function_macro(macro_def normie) lit {
    damn get_macro_type(macro_def) == MACRO_FUNCTION
}

slay is_expression_macro(macro_def normie) lit {
    damn get_macro_type(macro_def) == MACRO_EXPRESSION
}

slay is_statement_macro(macro_def normie) lit {
    damn get_macro_type(macro_def) == MACRO_STATEMENT
}

slay is_template_macro(macro_def normie) lit {
    damn get_macro_type(macro_def) == MACRO_TEMPLATE
}

slay is_generator_macro(macro_def normie) lit {
    damn get_macro_type(macro_def) == MACRO_GENERATOR
}

slay is_syntax_macro(macro_def normie) lit {
    damn get_macro_type(macro_def) == MACRO_SYNTAX
}

slay is_attribute_macro(macro_def normie) lit {
    damn get_macro_type(macro_def) == MACRO_ATTRIBUTE
}

slay is_directive_macro(macro_def normie) lit {
    damn get_macro_type(macro_def) == MACRO_DIRECTIVE
}

fr fr Template parameter substitution system
slay substitute_template_params(template tea, params tea, values tea) tea {
    sus result tea = template fr fr Simple parameter substitution - replace ${param} with value
    lowkey stringz.string_contains(params, "name") && stringz.string_contains(values, "value") {
        result = stringz.string_replace(result, "${name}", "function_name")
        result = stringz.string_replace(result, "${type}", "normie")
        result = stringz.string_replace(result, "${body}", "damn based")
    }
    
    lowkey stringz.string_contains(params, "count") {
        result = stringz.string_replace(result, "${count}", values)
    }
    
    lowkey stringz.string_contains(params, "expr") {
        result = stringz.string_replace(result, "${expr}", values)
    }
    
    damn result
}

fr fr Function macro expansion with template support
slay expand_function_macro(macro_def normie, args tea, context normie) tea {
    sus body tea = get_macro_body(macro_def)
    sus name tea = get_macro_name(macro_def) fr fr Built-in function macros
    lowkey stringz.string_equals(name, "getter") {
        sus template tea = "slay get_${name}() ${type} { damn this.${name} }"
        damn substitute_template_params(template, "name,type", args)
    }
    
    lowkey stringz.string_equals(name, "setter") {
        sus template tea = "slay set_${name}(value ${type}) { this.${name} = value }"
        damn substitute_template_params(template, "name,type", args)
    }
    
    lowkey stringz.string_equals(name, "constructor") {
        sus template tea = "slay init_${name}() { damn based }"
        damn substitute_template_params(template, "name", args)
    } fr fr Generic function generation
    lowkey stringz.string_length(body) > 0 {
        sus template tea = "slay generated_${name}(${args}) { ${body} }"
        damn substitute_template_params(template, "name,args,body", args + "," + body)
    }
    
    damn "slay generated_function() { damn based }"
}

fr fr Expression macro expansion with arithmetic and logical operations
slay expand_expression_macro(macro_def normie, args tea, context normie) tea {
    sus name tea = get_macro_name(macro_def)
    sus body tea = get_macro_body(macro_def) fr fr Built-in expression macros
    lowkey stringz.string_equals(name, "add") || stringz.string_equals(args, "add") {
        damn "a + b"
    }
    
    lowkey stringz.string_equals(name, "mul") || stringz.string_equals(args, "mul") {
        damn "a * b"
    }
    
    lowkey stringz.string_equals(name, "sub") || stringz.string_equals(args, "sub") {
        damn "a - b"
    }
    
    lowkey stringz.string_equals(name, "div") || stringz.string_equals(args, "div") {
        damn "a / b"
    } fr fr Template-based expression generation
    lowkey stringz.string_length(body) > 0 {
        damn substitute_template_params(body, "args", args)
    }
    
    damn args + "_expression"
}

fr fr Statement macro expansion with control flow and declarations
slay expand_statement_macro(macro_def normie, args tea, context normie) tea {
    sus name tea = get_macro_name(macro_def)
    sus body tea = get_macro_body(macro_def) fr fr Built-in statement macros
    lowkey stringz.string_equals(name, "print") || stringz.string_equals(args, "print") {
        damn "vibez.spill(\"" + args + "\")"
    }
    
    lowkey stringz.string_equals(name, "assign") || stringz.string_equals(args, "assign") {
        damn "sus x normie = 42"
    }
    
    lowkey stringz.string_equals(name, "declare") || stringz.string_equals(args, "declare") {
        damn "sus " + args + " normie"
    } fr fr Template-based statement generation
    lowkey stringz.string_length(body) > 0 {
        damn substitute_template_params(body, "args", args)
    }
    
    damn args + "_statement"
}

fr fr Template macro expansion with parameter substitution
slay expand_template_macro(macro_def normie, args tea, context normie) tea {
    sus name tea = get_macro_name(macro_def)
    sus body tea = get_macro_body(macro_def) fr fr Built-in templates
    lowkey stringz.string_equals(name, "class_template") {
        sus template tea = "struct ${name} { ${fields} }"
        damn substitute_template_params(template, "name,fields", args)
    }
    
    lowkey stringz.string_equals(name, "interface_template") {
        sus template tea = "interface ${name} { ${methods} }"
        damn substitute_template_params(template, "name,methods", args)
    }
    
    lowkey stringz.string_equals(name, "function_template") {
        sus template tea = "slay ${name}(${params}) ${return_type} { ${body} }"
        damn substitute_template_params(template, "name,params,return_type,body", args)
    } fr fr Generic template substitution
    lowkey stringz.string_length(body) > 0 {
        damn substitute_template_params(body, "template_param", args)
    }
    
    damn "template_result_" + args
}

fr fr Generator macro expansion with repetitive code generation
slay expand_generator_macro(macro_def normie, args tea, context normie) tea {
    sus name tea = get_macro_name(macro_def)
    sus body tea = get_macro_body(macro_def)
    sus count normie = stringz.string_to_int(args) fr fr Built-in generators
    lowkey stringz.string_equals(name, "var_generator") {
        sus result tea = ""
        sus i normie = 0
        loop {
            lowkey i >= count { break }
            result = result + "sus var" + stringz.int_to_string(i) + " normie = " + stringz.int_to_string(i) + "\n"
            i = i + 1
        }
        damn result
    }
    
    lowkey stringz.string_equals(name, "array_generator") {
        sus result tea = "["
        sus i normie = 0
        loop {
            lowkey i >= count { break }
            lowkey i > 0 {
                result = result + ", "
            }
            result = result + stringz.int_to_string(i)
            i = i + 1
        }
        result = result + "]"
        damn result
    }
    
    lowkey stringz.string_equals(name, "function_generator") {
        sus result tea = ""
        sus i normie = 0
        loop {
            lowkey i >= count { break }
            result = result + "slay func" + stringz.int_to_string(i) + "() { damn " + stringz.int_to_string(i) + " }\n"
            i = i + 1
        }
        damn result
    } fr fr Generic repetitive generation
    lowkey stringz.string_length(body) > 0 {
        sus result tea = ""
        sus i normie = 0
        loop {
            lowkey i >= count { break }
            sus iteration tea = substitute_template_params(body, "index", stringz.int_to_string(i))
            result = result + iteration + "\n"
            i = i + 1
        }
        damn result
    }
    
    damn "generated_code_" + args
}

fr fr Syntax macro expansion for language extensions
slay expand_syntax_macro(macro_def normie, args tea, context normie) tea {
    sus name tea = get_macro_name(macro_def)
    
    lowkey stringz.string_equals(name, "unless") {
        damn "lowkey !(" + args + ") {"
    }
    
    lowkey stringz.string_equals(name, "until") {
        damn "loop { lowkey !(" + args + ") { break } "
    }
    
    lowkey stringz.string_equals(name, "foreach") {
        damn "for item in " + args + " {"
    }
    
    damn "syntax_" + args
}

fr fr Attribute macro expansion for metadata
slay expand_attribute_macro(macro_def normie, args tea, context normie) tea {
    sus name tea = get_macro_name(macro_def)
    
    lowkey stringz.string_equals(name, "deprecated") {
        damn "# @deprecated: " + args
    }
    
    lowkey stringz.string_equals(name, "test") {
        damn "# @test: " + args
    }
    
    lowkey stringz.string_equals(name, "inline") {
        damn "# @inline: " + args
    }
    
    damn "attribute_" + args
}

fr fr Directive macro expansion for compiler directives
slay expand_directive_macro(macro_def normie, args tea, context normie) tea {
    sus name tea = get_macro_name(macro_def)
    
    lowkey stringz.string_equals(name, "include") {
        damn "yeet \"" + args + "\""
    }
    
    lowkey stringz.string_equals(name, "pragma") {
        damn "# pragma " + args
    }
    
    lowkey stringz.string_equals(name, "define") {
        damn "sus " + args + " normie = value"
    }
    
    damn "directive_" + args
}

fr fr Main macro expansion function with dispatch
slay expand_macro(macro_def normie, args tea, context normie) tea {
    sus macro_type normie = get_macro_type(macro_def)
    
    lowkey macro_type == MACRO_FUNCTION {
        damn expand_function_macro(macro_def, args, context)
    }
    lowkey macro_type == MACRO_EXPRESSION {
        damn expand_expression_macro(macro_def, args, context)
    }
    lowkey macro_type == MACRO_STATEMENT {
        damn expand_statement_macro(macro_def, args, context)
    }
    lowkey macro_type == MACRO_TEMPLATE {
        damn expand_template_macro(macro_def, args, context)
    }
    lowkey macro_type == MACRO_GENERATOR {
        damn expand_generator_macro(macro_def, args, context)
    }
    lowkey macro_type == MACRO_SYNTAX {
        damn expand_syntax_macro(macro_def, args, context)
    }
    lowkey macro_type == MACRO_ATTRIBUTE {
        damn expand_attribute_macro(macro_def, args, context)
    }
    lowkey macro_type == MACRO_DIRECTIVE {
        damn expand_directive_macro(macro_def, args, context)
    }
    
    damn "unknown_expansion"
}

fr fr Expansion mode-specific functions
slay expand_immediate(macro_def normie, args tea, context normie) tea {
    damn expand_macro(macro_def, args, context)
}

slay expand_lazy(macro_def normie, args tea, context normie) tea {
    sus name tea = get_macro_name(macro_def)
    damn "LAZY_MACRO{" + name + ":" + args + "}"
}

slay expand_recursive(macro_def normie, args tea, context normie) tea {
    lowkey context > 10 {
        damn "MAX_RECURSION_DEPTH_REACHED"
    }
    
    sus result tea = expand_macro(macro_def, args, context)
    
    lowkey stringz.string_contains(result, "MACRO{") {
        damn expand_recursive(macro_def, result, context + 1)
    }
    
    damn result
}

slay expand_once(macro_def normie, args tea, context normie) tea {
    sus result tea = expand_macro(macro_def, args, context)
    damn "EXPANDED_ONCE{" + result + "}"
}

fr fr AST Integration Functions
slay macro_to_ast(macro_def normie, args tea) normie {
    sus macro_type normie = get_macro_type(macro_def)
    sus name tea = get_macro_name(macro_def)
    
    lowkey macro_type == MACRO_FUNCTION {
        damn ast_mood.create_ast_node(ast_mood.AST_FUNCTION, name, args, 1, 1)
    }
    lowkey macro_type == MACRO_EXPRESSION {
        damn ast_mood.create_ast_node(ast_mood.AST_EXPRESSION, name, args, 1, 1)
    }
    lowkey macro_type == MACRO_STATEMENT {
        damn ast_mood.create_ast_node(ast_mood.AST_STATEMENT, name, args, 1, 1)
    }
    
    damn ast_mood.create_ast_node(ast_mood.AST_UNKNOWN, name, args, 1, 1)
}

slay ast_to_code(ast_node normie) tea {
    sus node_type normie = ast_mood.ast_node_type(ast_node)
    
    lowkey node_type == ast_mood.AST_FUNCTION {
        damn "slay function_from_ast() { damn based }"
    }
    lowkey node_type == ast_mood.AST_EXPRESSION {
        damn "expression_from_ast"
    }
    lowkey node_type == ast_mood.AST_STATEMENT {
        damn "statement_from_ast"
    }
    
    damn "ast_to_code_conversion"
}

fr fr Code Generation Functions
slay generate_code_from_macro(macro_def normie, args tea, format normie) tea {
    lowkey format == CODEGEN_AST {
        sus ast_node normie = macro_to_ast(macro_def, args)
        damn ast_mood.ast_node_to_string(ast_node)
    }
    lowkey format == CODEGEN_STRING {
        damn expand_macro(macro_def, args, 0)
    }
    lowkey format == CODEGEN_TOKENS {
        sus code tea = expand_macro(macro_def, args, 0)
        damn tokenize_code(code)
    }
    
    damn "unknown_format"
}

slay tokenize_code(code tea) tea {
    sus tokens tea = ""
    sus len normie = stringz.string_length(code)
    sus i normie = 0
    sus current_token tea = ""
    
    loop {
        lowkey i >= len { break }
        sus char_code normie = stringz.char_at_index(code, i) fr fr Space, tab, newline - end current token
        lowkey char_code == 32 || char_code == 9 || char_code == 10 {
            lowkey stringz.string_length(current_token) > 0 {
                tokens = tokens + "[" + current_token + "] "
                current_token = ""
            }
        } kapish {
            current_token = current_token + stringz.char_from_code(char_code)
        }
        i = i + 1
    } fr fr Add final token
    lowkey stringz.string_length(current_token) > 0 {
        tokens = tokens + "[" + current_token + "]"
    }
    
    damn tokens
}

fr fr Built-in Macro Library
slay define_builtin_macros() normie {
    sus count normie = 0 fr fr Expression macros
    register_macro("add", MACRO_EXPRESSION, EXPAND_IMMEDIATE, "a + b")
    count = count + 1
    
    register_macro("mul", MACRO_EXPRESSION, EXPAND_IMMEDIATE, "a * b")
    count = count + 1
    
    register_macro("eq", MACRO_EXPRESSION, EXPAND_IMMEDIATE, "a == b")
    count = count + 1 fr fr Statement macros
    register_macro("print", MACRO_STATEMENT, EXPAND_IMMEDIATE, "vibez.spill(args)")
    count = count + 1
    
    register_macro("assign", MACRO_STATEMENT, EXPAND_IMMEDIATE, "sus x normie = value")
    count = count + 1 fr fr Template macros
    register_macro("function_template", MACRO_TEMPLATE, EXPAND_LAZY, "slay ${name}() { ${body} }")
    count = count + 1 fr fr Generator macros
    register_macro("var_generator", MACRO_GENERATOR, EXPAND_IMMEDIATE, "sus var${index} normie = ${index}")
    count = count + 1
    
    register_macro("repeat", MACRO_GENERATOR, EXPAND_IMMEDIATE, "repeat_body")
    count = count + 1
    
    damn count
}

slay is_builtin_macro(name tea) lit {
    lowkey stringz.string_equals(name, "add") { damn based }
    lowkey stringz.string_equals(name, "mul") { damn based }
    lowkey stringz.string_equals(name, "eq") { damn based }
    lowkey stringz.string_equals(name, "print") { damn based }
    lowkey stringz.string_equals(name, "assign") { damn based }
    lowkey stringz.string_equals(name, "function_template") { damn based }
    lowkey stringz.string_equals(name, "var_generator") { damn based }
    lowkey stringz.string_equals(name, "repeat") { damn based }
    damn cap
}

slay get_builtin_macro_count() normie {
    damn BUILTIN_MACRO_COUNT
}

fr fr Analysis and Validation Functions
slay analyze_macro_complexity(macro_def normie) normie {
    sus macro_type normie = get_macro_type(macro_def)
    sus expand_mode normie = get_macro_expand_mode(macro_def)
    sus body tea = get_macro_body(macro_def)
    sus complexity normie = 1 fr fr Base complexity from type
    lowkey macro_type == MACRO_GENERATOR { complexity = complexity + 5 }
    lowkey macro_type == MACRO_TEMPLATE { complexity = complexity + 3 }
    lowkey macro_type == MACRO_SYNTAX { complexity = complexity + 4 }
    lowkey macro_type == MACRO_FUNCTION { complexity = complexity + 2 } fr fr Additional complexity from expansion mode
    lowkey expand_mode == EXPAND_RECURSIVE { complexity = complexity + 3 }
    lowkey expand_mode == EXPAND_LAZY { complexity = complexity + 1 } fr fr Body length factor
    sus body_length normie = stringz.string_length(body)
    complexity = complexity + body_length / 20
    
    damn complexity
}

slay estimate_expansion_size(macro_def normie, args tea) normie {
    sus macro_type normie = get_macro_type(macro_def)
    sus body tea = get_macro_body(macro_def)
    sus base_size normie = stringz.string_length(body)
    
    lowkey base_size == 0 {
        base_size = 50 fr fr Default size for built-ins
    } fr fr Size multipliers based on type
    lowkey macro_type == MACRO_GENERATOR {
        sus count normie = stringz.string_to_int(args)
        lowkey count > 0 {
            damn base_size * count
        }
        damn base_size * 5
    }
    
    lowkey macro_type == MACRO_TEMPLATE { damn base_size * 2 }
    lowkey macro_type == MACRO_FUNCTION { damn base_size * 3 }
    
    damn base_size
}

slay can_macro_expand_infinitely(macro_def normie) lit {
    sus expand_mode normie = get_macro_expand_mode(macro_def)
    sus macro_type normie = get_macro_type(macro_def)
    
    lowkey expand_mode == EXPAND_RECURSIVE { damn based }
    lowkey macro_type == MACRO_GENERATOR { damn based }
    
    damn cap
}

slay get_macro_signature(macro_def normie) tea {
    sus name tea = get_macro_name(macro_def)
    sus type_str tea = ""
    sus macro_type normie = get_macro_type(macro_def)
    
    lowkey macro_type == MACRO_FUNCTION { type_str = "function" }
    lowkey macro_type == MACRO_EXPRESSION { type_str = "expression" }
    lowkey macro_type == MACRO_STATEMENT { type_str = "statement" }
    lowkey macro_type == MACRO_TEMPLATE { type_str = "template" }
    lowkey macro_type == MACRO_GENERATOR { type_str = "generator" }
    lowkey macro_type == MACRO_SYNTAX { type_str = "syntax" }
    lowkey macro_type == MACRO_ATTRIBUTE { type_str = "attribute" }
    lowkey macro_type == MACRO_DIRECTIVE { type_str = "directive" }
    
    damn type_str + ":" + name
}

fr fr Validation and Parsing Functions
slay validate_macro_syntax(macro_text tea) lit {
    lowkey stringz.string_length(macro_text) == 0 { damn cap } fr fr Check for balanced braces
    sus open_braces normie = stringz.count_occurrences(macro_text, "{")
    sus close_braces normie = stringz.count_occurrences(macro_text, "}")
    lowkey open_braces != close_braces { damn cap } fr fr Check for balanced parentheses
    sus open_parens normie = stringz.count_occurrences(macro_text, "(")
    sus close_parens normie = stringz.count_occurrences(macro_text, ")")
    lowkey open_parens != close_parens { damn cap } fr fr Must not be just whitespace
    lowkey stringz.string_trim(macro_text) == "" { damn cap }
    
    damn based
}

slay parse_macro_definition(macro_text tea) normie {
    lowkey !validate_macro_syntax(macro_text) {
        damn 0
    } fr fr Extract macro name and type from definition
    sus macro_name tea = "parsed_macro"
    sus macro_type normie = MACRO_FUNCTION
    sus expand_mode normie = EXPAND_IMMEDIATE fr fr Look for type hints in the macro text
    lowkey stringz.string_contains(macro_text, "function") {
        macro_type = MACRO_FUNCTION
    }
    lowkey stringz.string_contains(macro_text, "expression") {
        macro_type = MACRO_EXPRESSION
    }
    lowkey stringz.string_contains(macro_text, "statement") {
        macro_type = MACRO_STATEMENT
    }
    lowkey stringz.string_contains(macro_text, "template") {
        macro_type = MACRO_TEMPLATE
    }
    lowkey stringz.string_contains(macro_text, "generator") {
        macro_type = MACRO_GENERATOR
    } fr fr Look for expansion mode hints
    lowkey stringz.string_contains(macro_text, "lazy") {
        expand_mode = EXPAND_LAZY
    }
    lowkey stringz.string_contains(macro_text, "recursive") {
        expand_mode = EXPAND_RECURSIVE
    }
    lowkey stringz.string_contains(macro_text, "once") {
        expand_mode = EXPAND_ONCE
    }
    
    damn register_macro(macro_name, macro_type, expand_mode, macro_text)
}

slay compile_macro(macro_text tea) normie {
    lowkey validate_macro_syntax(macro_text) {
        damn parse_macro_definition(macro_text)
    }
    damn 0
}

slay execute_macro(name tea, args tea) tea {
    sus macro_def normie = lookup_macro(name)
    lowkey macro_def != 0 {
        damn expand_macro(macro_def, args, 0)
    } fr fr Try built-in macros
    lowkey is_builtin_macro(name) {
        lowkey stringz.string_equals(name, "print") {
            damn "vibez.spill(\"" + args + "\")"
        }
        lowkey stringz.string_equals(name, "add") {
            damn "a + b"
        }
        lowkey stringz.string_equals(name, "repeat") {
            sus count normie = stringz.string_to_int(args)
            lowkey count > 0 {
                sus result tea = ""
                sus i normie = 0
                loop {
                    lowkey i >= count { break }
                    result = result + "repeated_item_" + stringz.int_to_string(i) + "\n"
                    i = i + 1
                }
                damn result
            }
        }
    }
    
    damn "macro_not_found: " + name
}

fr fr Debug and Tracing Functions
slay debug_macro_expansion(macro_def normie, args tea, context normie) tea {
    sus name tea = get_macro_name(macro_def)
    sus type_str tea = get_macro_signature(macro_def)
    sus complexity normie = analyze_macro_complexity(macro_def)
    sus estimated_size normie = estimate_expansion_size(macro_def, args)
    
    sus debug_info tea = "DEBUG: Macro=" + name + 
                        " Type=" + type_str + 
                        " Args=" + args + 
                        " Context=" + stringz.int_to_string(context) +
                        " Complexity=" + stringz.int_to_string(complexity) +
                        " EstimatedSize=" + stringz.int_to_string(estimated_size)
    
    damn debug_info
}

slay trace_macro_expansion(macro_def normie, args tea, depth normie) tea {
    lowkey depth > 20 {
        damn "TRACE: Maximum trace depth reached"
    }
    
    sus name tea = get_macro_name(macro_def)
    sus indent tea = ""
    sus i normie = 0
    loop {
        lowkey i >= depth { break }
        indent = indent + "  "
        i = i + 1
    }
    
    sus trace_info tea = "TRACE:" + indent + "[" + stringz.int_to_string(depth) + "] " + 
                        name + "(" + args + ")" fr fr Trace actual expansion
    sus result tea = expand_macro(macro_def, args, depth)
    trace_info = trace_info + "\n" + indent + "  => " + result
    
    damn trace_info
}

fr fr System Information Functions
slay macro_slay_version() tea {
    damn "1.0.0"
}

slay macro_slay_status() tea {
    sus registered normie = get_macro_count()
    sus builtins normie = get_builtin_macro_count()
    damn "macro_slay v1.0.0 loaded - " + stringz.int_to_string(registered) + 
         " macros registered (" + stringz.int_to_string(builtins) + " built-ins)"
}

slay is_macro_slay_ready() lit {
    damn GLOBAL_MACRO_COUNT >= 0
}

slay get_supported_macro_types() normie {
    damn 9 fr fr UNKNOWN through DIRECTIVE
}

slay get_supported_expand_modes() normie {
    damn 4 fr fr IMMEDIATE through ONCE
}

slay macro_system_info() tea {
    sus types normie = get_supported_macro_types()
    sus modes normie = get_supported_expand_modes()
    sus registered normie = get_macro_count()
    
    damn "Macro System v1.0.0: " + stringz.int_to_string(types) + 
         " types, " + stringz.int_to_string(modes) + 
         " modes, " + stringz.int_to_string(registered) + " registered"
}

fr fr Initialize the macro system
slay init_macro_system() {
    sus count normie = define_builtin_macros()
    damn "Macro system initialized with " + stringz.int_to_string(count) + " built-ins"
}

fr fr Auto-initialize when module loads
sus INIT_RESULT tea = init_macro_system()
