#!/usr/bin/env cursed
# CURSED Self-Hosting Compiler - Code Generation Module
# Generates C code from CURSED AST

yeet "parser"
yeet "stringz"
yeet "arrayz"
yeet "testz"

# Code generation context
squad CodeGen {
    spill output tea                # generated code
    spill indent_level normie       # current indentation
    spill temp_var_count normie     # for generating temp variables
    spill function_scope lit        # whether we're in a function
    spill includes []tea           # required includes
}

# Initialize code generator
slay new_codegen() CodeGen {
    damn CodeGen{
        output: "",
        indent_level: 0,
        temp_var_count: 0,
        function_scope: cringe,
        includes: []
    }
}

# Add indentation to output
slay add_indent(codegen CodeGen) {
    bestie i := 0; i < codegen.indent_level; i = i + 1 {
        codegen.output = codegen.output + "    "
    }
}

# Emit code with indentation
slay emit(codegen CodeGen, code tea) {
    add_indent(codegen)
    codegen.output = codegen.output + code + "\n"
}

# Emit code without indentation or newline
slay emit_raw(codegen CodeGen, code tea) {
    codegen.output = codegen.output + code
}

# Increment indentation
slay indent_push(codegen CodeGen) {
    codegen.indent_level = codegen.indent_level + 1
}

# Decrement indentation
slay indent_pop(codegen CodeGen) {
    lowkey (codegen.indent_level > 0) {
        codegen.indent_level = codegen.indent_level - 1
    }
}

# Generate unique temporary variable name
slay gen_temp_var(codegen CodeGen) tea {
    sus var_name tea = "_temp_" + codegen.temp_var_count
    codegen.temp_var_count = codegen.temp_var_count + 1
    damn var_name
}

# Add required include
slay add_include(codegen CodeGen, include_name tea) {
    # Check if already included
    bestie i := 0; i < arrayz.array_length(codegen.includes); i = i + 1 {
        sus existing tea = arrayz.array_get(codegen.includes, i)
        lowkey (existing == include_name) {
            damn  # already included
        }
    }
    
    arrayz.array_push(codegen.includes, include_name)
}

# Generate C type from CURSED type
slay cursed_type_to_c(type tea) tea {
    vibe_check (type) {
        mood "normie" {
            damn "int"
        }
        mood "thicc" {
            damn "long long"
        }
        mood "smol" {
            damn "short"
        }
        mood "meal" {
            damn "double"
        }
        mood "tea" {
            damn "char*"
        }
        mood "lit" {
            damn "int"  # boolean as int
        }
        mood "drip" {
            damn "void*"  # auto type as void pointer
        }
        basic {
            damn type  # assume it's a custom type
        }
    }
}

# Generate function signature
slay generate_function_signature(codegen CodeGen, node ASTNode) tea {
    # Function name
    sus name_node ASTNode = arrayz.array_get(node.children, 0)
    sus func_name tea = name_node.value
    
    # Parameters
    sus params_node ASTNode = arrayz.array_get(node.children, 1)
    sus param_list tea = ""
    
    lowkey (arrayz.array_length(params_node.children) == 0) {
        param_list = "void"
    } highkey {
        bestie i := 0; i < arrayz.array_length(params_node.children); i = i + 1 {
            sus param ASTNode = arrayz.array_get(params_node.children, i)
            sus param_name tea = param.value
            
            sus param_type tea = "int"  # default type
            lowkey (arrayz.array_length(param.children) > 0) {
                sus type_node ASTNode = arrayz.array_get(param.children, 0)
                param_type = cursed_type_to_c(type_node.value)
            }
            
            lowkey (i > 0) {
                param_list = param_list + ", "
            }
            param_list = param_list + param_type + " " + param_name
        }
    }
    
    # Return type
    sus return_type tea = "void"
    lowkey (arrayz.array_length(node.children) > 2) {
        sus last_child ASTNode = arrayz.array_get(node.children, arrayz.array_length(node.children) - 1)
        lowkey (last_child.node_type != NodeType.BLOCK_STATEMENT) {
            # Return type specified
            return_type = cursed_type_to_c(last_child.value)
        }
    }
    
    damn return_type + " " + func_name + "(" + param_list + ")"
}

# Generate C code from AST node
slay generate_node(codegen CodeGen, node ASTNode) tea {
    vibe_check (node.node_type) {
        mood NodeType.PROGRAM {
            damn generate_program(codegen, node)
        }
        mood NodeType.FUNCTION_DECLARATION {
            damn generate_function(codegen, node)
        }
        mood NodeType.VARIABLE_DECLARATION {
            damn generate_variable_declaration(codegen, node)
        }
        mood NodeType.BLOCK_STATEMENT {
            damn generate_block(codegen, node)
        }
        mood NodeType.EXPRESSION_STATEMENT {
            damn generate_expression_statement(codegen, node)
        }
        mood NodeType.RETURN_STATEMENT {
            damn generate_return_statement(codegen, node)
        }
        mood NodeType.IF_STATEMENT {
            damn generate_if_statement(codegen, node)
        }
        mood NodeType.WHILE_STATEMENT {
            damn generate_while_statement(codegen, node)
        }
        mood NodeType.FOR_STATEMENT {
            damn generate_for_statement(codegen, node)
        }
        mood NodeType.CALL_EXPRESSION {
            damn generate_call_expression(codegen, node)
        }
        mood NodeType.BINARY_EXPRESSION {
            damn generate_binary_expression(codegen, node)
        }
        mood NodeType.UNARY_EXPRESSION {
            damn generate_unary_expression(codegen, node)
        }
        mood NodeType.IDENTIFIER {
            damn node.value
        }
        mood NodeType.INTEGER_LITERAL {
            damn node.value
        }
        mood NodeType.FLOAT_LITERAL {
            damn node.value
        }
        mood NodeType.STRING_LITERAL {
            damn "\"" + node.value + "\""
        }
        mood NodeType.BOOLEAN_LITERAL {
            lowkey (node.value == "true") {
                damn "1"
            } highkey {
                damn "0"
            }
        }
        mood NodeType.STRUCT_DECLARATION {
            damn generate_struct_declaration(codegen, node)
        }
        basic {
            damn "/* Unsupported node type: " + node.node_type + " */"
        }
    }
}

# Generate program
slay generate_program(codegen CodeGen, node ASTNode) tea {
    # Add standard includes
    add_include(codegen, "#include <stdio.h>")
    add_include(codegen, "#include <stdlib.h>")
    add_include(codegen, "#include <string.h>")
    add_include(codegen, "#include <stdbool.h>")
    
    # Generate includes
    bestie i := 0; i < arrayz.array_length(codegen.includes); i = i + 1 {
        sus include tea = arrayz.array_get(codegen.includes, i)
        emit(codegen, include)
    }
    
    emit(codegen, "")  # empty line
    
    # Add vibez.spill function (CURSED stdlib bridge)
    emit(codegen, "// CURSED Stdlib Bridge Functions")
    emit(codegen, "void vibez_spill(const char* message) {")
    indent_push(codegen)
    emit(codegen, "printf(\"%s\\n\", message);")
    indent_pop(codegen)
    emit(codegen, "}")
    emit(codegen, "")
    
    # Generate forward declarations
    bestie i := 0; i < arrayz.array_length(node.children); i = i + 1 {
        sus child ASTNode = arrayz.array_get(node.children, i)
        lowkey (child.node_type == NodeType.FUNCTION_DECLARATION) {
            sus signature tea = generate_function_signature(codegen, child)
            emit(codegen, signature + ";")
        }
    }
    
    emit(codegen, "")  # empty line
    
    # Generate all declarations
    bestie i := 0; i < arrayz.array_length(node.children); i = i + 1 {
        sus child ASTNode = arrayz.array_get(node.children, i)
        generate_node(codegen, child)
        emit(codegen, "")  # empty line between declarations
    }
    
    damn ""  # return value not used
}

# Generate function
slay generate_function(codegen CodeGen, node ASTNode) tea {
    sus signature tea = generate_function_signature(codegen, node)
    emit(codegen, signature + " {")
    
    indent_push(codegen)
    codegen.function_scope = based
    
    # Generate function body
    sus body_index normie = arrayz.array_length(node.children) - 1
    sus body_node ASTNode = arrayz.array_get(node.children, body_index)
    generate_node(codegen, body_node)
    
    codegen.function_scope = cringe
    indent_pop(codegen)
    emit(codegen, "}")
    
    damn ""
}

# Generate variable declaration
slay generate_variable_declaration(codegen CodeGen, node ASTNode) tea {
    sus name_node ASTNode = arrayz.array_get(node.children, 0)
    sus var_name tea = name_node.value
    
    sus var_type tea = "int"  # default type
    sus init_expr tea = ""
    
    # Check for type annotation
    lowkey (arrayz.array_length(node.children) >= 2) {
        sus second_child ASTNode = arrayz.array_get(node.children, 1)
        lowkey (second_child.node_type == NodeType.IDENTIFIER) {
            var_type = cursed_type_to_c(second_child.value)
            
            # Check for initializer
            lowkey (arrayz.array_length(node.children) >= 3) {
                sus init_node ASTNode = arrayz.array_get(node.children, 2)
                init_expr = " = " + generate_node(codegen, init_node)
            }
        } highkey {
            # No type, just initializer
            init_expr = " = " + generate_node(codegen, second_child)
        }
    }
    
    emit(codegen, var_type + " " + var_name + init_expr + ";")
    damn ""
}

# Generate block statement
slay generate_block(codegen CodeGen, node ASTNode) tea {
    bestie i := 0; i < arrayz.array_length(node.children); i = i + 1 {
        sus child ASTNode = arrayz.array_get(node.children, i)
        generate_node(codegen, child)
    }
    
    damn ""
}

# Generate expression statement
slay generate_expression_statement(codegen CodeGen, node ASTNode) tea {
    lowkey (arrayz.array_length(node.children) > 0) {
        sus expr_node ASTNode = arrayz.array_get(node.children, 0)
        sus expr_code tea = generate_node(codegen, expr_node)
        emit(codegen, expr_code + ";")
    }
    
    damn ""
}

# Generate return statement
slay generate_return_statement(codegen CodeGen, node ASTNode) tea {
    lowkey (arrayz.array_length(node.children) > 0) {
        sus expr_node ASTNode = arrayz.array_get(node.children, 0)
        sus expr_code tea = generate_node(codegen, expr_node)
        emit(codegen, "return " + expr_code + ";")
    } highkey {
        emit(codegen, "return;")
    }
    
    damn ""
}

# Generate if statement
slay generate_if_statement(codegen CodeGen, node ASTNode) tea {
    sus condition_node ASTNode = arrayz.array_get(node.children, 0)
    sus condition_code tea = generate_node(codegen, condition_node)
    
    emit(codegen, "if (" + condition_code + ") {")
    indent_push(codegen)
    
    sus then_node ASTNode = arrayz.array_get(node.children, 1)
    generate_node(codegen, then_node)
    
    indent_pop(codegen)
    
    lowkey (arrayz.array_length(node.children) > 2) {
        emit(codegen, "} else {")
        indent_push(codegen)
        
        sus else_node ASTNode = arrayz.array_get(node.children, 2)
        generate_node(codegen, else_node)
        
        indent_pop(codegen)
    }
    
    emit(codegen, "}")
    damn ""
}

# Generate while statement
slay generate_while_statement(codegen CodeGen, node ASTNode) tea {
    sus condition_node ASTNode = arrayz.array_get(node.children, 0)
    sus condition_code tea = generate_node(codegen, condition_node)
    
    emit(codegen, "while (" + condition_code + ") {")
    indent_push(codegen)
    
    sus body_node ASTNode = arrayz.array_get(node.children, 1)
    generate_node(codegen, body_node)
    
    indent_pop(codegen)
    emit(codegen, "}")
    
    damn ""
}

# Generate for statement
slay generate_for_statement(codegen CodeGen, node ASTNode) tea {
    # For now, treat CURSED 'bestie' loops as while loops
    sus condition_node ASTNode = arrayz.array_get(node.children, 0)
    sus condition_code tea = generate_node(codegen, condition_node)
    
    emit(codegen, "while (" + condition_code + ") {")
    indent_push(codegen)
    
    sus body_node ASTNode = arrayz.array_get(node.children, 1)
    generate_node(codegen, body_node)
    
    indent_pop(codegen)
    emit(codegen, "}")
    
    damn ""
}

# Generate call expression
slay generate_call_expression(codegen CodeGen, node ASTNode) tea {
    sus function_node ASTNode = arrayz.array_get(node.children, 0)
    sus function_name tea = generate_node(codegen, function_node)
    
    # Handle special CURSED functions
    lowkey (function_name == "vibez.spill") {
        function_name = "vibez_spill"
    }
    
    sus args tea = ""
    
    # Generate arguments
    bestie i := 1; i < arrayz.array_length(node.children); i = i + 1 {
        sus arg_node ASTNode = arrayz.array_get(node.children, i)
        sus arg_code tea = generate_node(codegen, arg_node)
        
        lowkey (i > 1) {
            args = args + ", "
        }
        args = args + arg_code
    }
    
    damn function_name + "(" + args + ")"
}

# Generate binary expression
slay generate_binary_expression(codegen CodeGen, node ASTNode) tea {
    sus left_node ASTNode = arrayz.array_get(node.children, 0)
    sus right_node ASTNode = arrayz.array_get(node.children, 1)
    
    sus left_code tea = generate_node(codegen, left_node)
    sus right_code tea = generate_node(codegen, right_node)
    sus operator tea = node.value
    
    # Handle member access
    lowkey (operator == ".") {
        damn left_code + "." + right_code
    }
    
    # Handle string concatenation
    lowkey (operator == "+") {
        # TODO: Check if operands are strings and use strcat
        damn "(" + left_code + " " + operator + " " + right_code + ")"
    }
    
    damn "(" + left_code + " " + operator + " " + right_code + ")"
}

# Generate unary expression
slay generate_unary_expression(codegen CodeGen, node ASTNode) tea {
    sus operand_node ASTNode = arrayz.array_get(node.children, 0)
    sus operand_code tea = generate_node(codegen, operand_node)
    sus operator tea = node.value
    
    damn "(" + operator + operand_code + ")"
}

# Generate struct declaration
slay generate_struct_declaration(codegen CodeGen, node ASTNode) tea {
    sus name_node ASTNode = arrayz.array_get(node.children, 0)
    sus struct_name tea = name_node.value
    
    emit(codegen, "typedef struct {")
    indent_push(codegen)
    
    # Generate fields
    bestie i := 1; i < arrayz.array_length(node.children); i = i + 1 {
        sus field_node ASTNode = arrayz.array_get(node.children, i)
        sus field_name tea = field_node.value
        
        sus field_type tea = "int"  # default
        lowkey (arrayz.array_length(field_node.children) > 0) {
            sus type_node ASTNode = arrayz.array_get(field_node.children, 0)
            field_type = cursed_type_to_c(type_node.value)
        }
        
        emit(codegen, field_type + " " + field_name + ";")
    }
    
    indent_pop(codegen)
    emit(codegen, "} " + struct_name + ";")
    
    damn ""
}

# Main code generation function
slay generate_code(ast ASTNode) tea {
    sus codegen CodeGen = new_codegen()
    generate_node(codegen, ast)
    damn codegen.output
}

# Write generated code to file
slay write_code_to_file(code tea, filename tea) {
    # TODO: Implement file writing when CURSED has file I/O
    vibez.spill("Generated C code:")
    vibez.spill("================")
    vibez.spill(code)
    vibez.spill("================")
}

# Compile CURSED source to C
slay compile_to_c(source tea, output_file tea) lit {
    vibez.spill("🔄 Compiling CURSED to C...")
    
    # Parse CURSED source
    sus ast ASTNode = parse(source)
    
    # Generate C code
    sus c_code tea = generate_code(ast)
    
    # Write to file (simulated)
    write_code_to_file(c_code, output_file)
    
    vibez.spill("✅ Compilation successful!")
    damn based
}
