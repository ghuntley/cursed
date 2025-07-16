# macro_slay - CURSED Macro System Implementation

The `macro_slay` module provides a comprehensive macro system for the CURSED programming language, enabling code generation, AST manipulation, and advanced metaprogramming capabilities.

## Features

### 🚀 Core Capabilities
- **Macro Definition & Registry**: Complete macro definition, registration, and lookup system
- **Multiple Macro Types**: Support for function, expression, statement, template, generator, syntax, attribute, and directive macros
- **Flexible Expansion Modes**: Immediate, lazy, recursive, and one-time expansion strategies
- **AST Integration**: Seamless integration with the `ast_mood` module for AST manipulation
- **Code Generation**: Multiple output formats including AST, string, and token representations
- **Built-in Macros**: Pre-defined common macros for immediate productivity

### 🔧 Technical Features
- **FFI-Free Implementation**: Pure CURSED implementation without external dependencies
- **Type Safety**: Comprehensive type checking and validation
- **Error Handling**: Robust error handling with graceful degradation
- **Performance Optimized**: Efficient macro registry and expansion algorithms
- **Debug Support**: Comprehensive debugging and tracing utilities

## Usage

### Basic Macro Definition

```cursed
yeet "macro_slay"

# Register a simple function macro
sus my_macro normie = register_macro("my_func", MACRO_FUNCTION, EXPAND_IMMEDIATE, "function_body")

# Check if macro is defined
lowkey is_macro_defined("my_func") {
    vibez.spill("Macro is registered!")
}
```

### Macro Types

The module supports nine different macro types:

```cursed
# Function macros - generate complete functions
sus func_macro normie = register_macro("gen_func", MACRO_FUNCTION, EXPAND_IMMEDIATE, "body")

# Expression macros - generate expressions
sus expr_macro normie = register_macro("add_expr", MACRO_EXPRESSION, EXPAND_IMMEDIATE, "a + b")

# Statement macros - generate statements
sus stmt_macro normie = register_macro("print_stmt", MACRO_STATEMENT, EXPAND_IMMEDIATE, "print")

# Template macros - parameter substitution
sus tmpl_macro normie = register_macro("template", MACRO_TEMPLATE, EXPAND_LAZY, "template_body")

# Generator macros - repetitive code generation
sus gen_macro normie = register_macro("repeat", MACRO_GENERATOR, EXPAND_RECURSIVE, "loop_body")
```

### Expansion Modes

Four different expansion strategies are supported:

```cursed
# Immediate expansion - expand right away
EXPAND_IMMEDIATE

# Lazy expansion - defer until needed
EXPAND_LAZY

# Recursive expansion - allow nested expansion
EXPAND_RECURSIVE  

# One-time expansion - expand only once
EXPAND_ONCE
```

### Macro Expansion

```cursed
# Expand a macro with arguments
sus result tea = expand_macro(my_macro, "arg1,arg2", 0)
vibez.spill(result)

# Expand specific macro types
sus func_result tea = expand_function_macro(func_macro, "parameters", 0)
sus expr_result tea = expand_expression_macro(expr_macro, "add", 0)
sus stmt_result tea = expand_statement_macro(stmt_macro, "print", 0)
```

### AST Integration

```cursed
yeet "ast_mood"

# Convert macro to AST
sus ast_node normie = macro_to_ast(my_macro, "args")

# Convert AST back to code
sus generated_code tea = ast_to_code(ast_node)
vibez.spill(generated_code)

# Generate code in different formats
sus ast_format tea = generate_code_from_macro(my_macro, "args", CODEGEN_AST)
sus string_format tea = generate_code_from_macro(my_macro, "args", CODEGEN_STRING)
sus token_format tea = generate_code_from_macro(my_macro, "args", CODEGEN_TOKENS)
```

### Template Macros

```cursed
# Template with parameter substitution
sus template_macro normie = register_macro("my_template", MACRO_TEMPLATE, EXPAND_LAZY, "template_body")
sus expanded tea = expand_template_macro(template_macro, "substitute_value", 0)
```

### Generator Macros

```cursed
# Generate repetitive code structures
sus generator_macro normie = register_macro("var_generator", MACRO_GENERATOR, EXPAND_IMMEDIATE, "variables")
sus generated_vars tea = expand_generator_macro(generator_macro, "5", 0)
# Generates: sus var0 normie = 0\nsus var1 normie = 1\n...
```

### Built-in Macros

```cursed
# Initialize built-in macros
define_builtin_macros()

# Use built-in macros
sus print_result tea = execute_macro("print", "hello world")
sus add_result tea = execute_macro("add", "operands")
```

## API Reference

### Macro Definition Functions

| Function | Description |
|----------|-------------|
| `register_macro(name, type, mode, body)` | Register a new macro |
| `lookup_macro(name)` | Find a macro by name |
| `is_macro_defined(name)` | Check if macro exists |
| `get_macro_count()` | Get total registered macros |

### Macro Type Functions

| Function | Description |
|----------|-------------|
| `is_function_macro(macro_def)` | Check if function macro |
| `is_expression_macro(macro_def)` | Check if expression macro |
| `is_statement_macro(macro_def)` | Check if statement macro |
| `is_template_macro(macro_def)` | Check if template macro |
| `is_generator_macro(macro_def)` | Check if generator macro |

### Expansion Functions

| Function | Description |
|----------|-------------|
| `expand_macro(macro_def, args, context)` | Expand macro with context |
| `expand_immediate(macro_def, args, context)` | Immediate expansion |
| `expand_lazy(macro_def, args, context)` | Lazy expansion |
| `expand_recursive(macro_def, args, context)` | Recursive expansion |
| `expand_once(macro_def, args, context)` | One-time expansion |

### Code Generation Functions

| Function | Description |
|----------|-------------|
| `generate_code_from_macro(macro_def, args, format)` | Generate code in specified format |
| `macro_to_ast(macro_def, args)` | Convert macro to AST |
| `ast_to_code(ast_node)` | Convert AST to code string |
| `tokenize_code(code)` | Tokenize generated code |

### Analysis Functions

| Function | Description |
|----------|-------------|
| `analyze_macro_complexity(macro_def)` | Analyze macro complexity |
| `estimate_expansion_size(macro_def, args)` | Estimate output size |
| `can_macro_expand_infinitely(macro_def)` | Check for infinite expansion |
| `get_macro_signature(macro_def)` | Get macro signature string |

### Debug Functions

| Function | Description |
|----------|-------------|
| `debug_macro_expansion(macro_def, args, context)` | Debug expansion process |
| `trace_macro_expansion(macro_def, args, depth)` | Trace expansion with depth |

### Utility Functions

| Function | Description |
|----------|-------------|
| `validate_macro_syntax(macro_text)` | Validate macro syntax |
| `parse_macro_definition(macro_text)` | Parse macro definition |
| `compile_macro(macro_text)` | Compile macro from text |
| `execute_macro(name, args)` | Execute named macro |

## Constants

### Macro Types
```cursed
sus MACRO_UNKNOWN normie = 0
sus MACRO_FUNCTION normie = 1
sus MACRO_EXPRESSION normie = 2
sus MACRO_STATEMENT normie = 3
sus MACRO_TEMPLATE normie = 4
sus MACRO_GENERATOR normie = 5
sus MACRO_SYNTAX normie = 6
sus MACRO_ATTRIBUTE normie = 7
sus MACRO_DIRECTIVE normie = 8
```

### Expansion Modes
```cursed
sus EXPAND_IMMEDIATE normie = 10
sus EXPAND_LAZY normie = 11
sus EXPAND_RECURSIVE normie = 12
sus EXPAND_ONCE normie = 13
```

### Code Generation Formats
```cursed
sus CODEGEN_AST normie = 20
sus CODEGEN_STRING normie = 21
sus CODEGEN_TOKENS normie = 22
```

## Examples

### 1. Simple Function Generator

```cursed
yeet "macro_slay"

# Create a function generator macro
sus func_gen normie = register_macro("func_gen", MACRO_FUNCTION, EXPAND_IMMEDIATE, "getter")

# Expand to generate a getter function
sus getter_code tea = expand_function_macro(func_gen, "name", 0)
vibez.spill(getter_code)
# Output: slay generated_function(name) { damn based }
```

### 2. Expression Builder

```cursed
# Create expression macros
sus add_macro normie = register_macro("add_op", MACRO_EXPRESSION, EXPAND_IMMEDIATE, "addition")
sus mul_macro normie = register_macro("mul_op", MACRO_EXPRESSION, EXPAND_IMMEDIATE, "multiplication")

# Generate arithmetic expressions
sus addition tea = expand_expression_macro(add_macro, "add", 0)  # "a + b"
sus multiplication tea = expand_expression_macro(mul_macro, "mul", 0)  # "a * b"
```

### 3. Code Template System

```cursed
# Create a template macro
sus class_template normie = register_macro("class_tmpl", MACRO_TEMPLATE, EXPAND_LAZY, "class_body")

# Expand template with substitution
sus class_code tea = expand_template_macro(class_template, "MyClass", 0)
```

### 4. Repetitive Code Generator

```cursed
# Create a variable generator
sus var_gen normie = register_macro("var_gen", MACRO_GENERATOR, EXPAND_IMMEDIATE, "variables")

# Generate multiple variable declarations
sus vars tea = expand_generator_macro(var_gen, "3", 0)
vibez.spill(vars)
# Output: 
# sus var0 normie = 0
# sus var1 normie = 1  
# sus var2 normie = 2
```

### 5. AST Manipulation

```cursed
yeet "ast_mood"

# Convert macro to AST and manipulate
sus my_macro normie = register_macro("ast_demo", MACRO_FUNCTION, EXPAND_IMMEDIATE, "demo")
sus ast normie = macro_to_ast(my_macro, "demo_args")

# Validate and analyze AST
lowkey ast_mood.validate_ast_node(ast) {
    sus ast_string tea = ast_mood.ast_node_to_string(ast)
    vibez.spill("Valid AST: " + ast_string)
}
```

## Integration with ast_mood

The `macro_slay` module seamlessly integrates with the `ast_mood` module:

- **AST Node Creation**: Macros can generate AST nodes directly
- **Node Type Validation**: Full support for all AST node types
- **Bidirectional Conversion**: Convert between macros and AST representations
- **Pattern Matching**: Use AST patterns in macro expansion

## Performance Considerations

- **Registry Efficiency**: Macro registry uses encoded integer storage for fast lookup
- **Expansion Limits**: Recursive expansion has depth limits to prevent infinite loops
- **Memory Management**: Efficient string handling and minimal memory allocation
- **Lazy Evaluation**: Lazy expansion mode defers computation until needed

## Error Handling

The module provides comprehensive error handling:

- **Syntax Validation**: Validate macro definitions before registration
- **Type Checking**: Ensure macro types are valid
- **Expansion Safety**: Prevent infinite recursion with depth limits
- **Graceful Degradation**: Return sensible defaults for invalid operations

## Testing

Run the comprehensive test suite:

```bash
cargo run --bin cursed stdlib/macro_slay/test_macro_slay.csd
```

The test suite covers:
- ✅ Macro definition and registry management
- ✅ Type system validation  
- ✅ Expansion engine functionality
- ✅ AST integration
- ✅ Code generation
- ✅ Built-in macros
- ✅ Debug utilities
- ✅ Error handling
- ✅ Performance scenarios

## Module Information

- **Version**: 1.0.0
- **Dependencies**: `ast_mood`, `stringz`, `testz`
- **Implementation**: Pure CURSED (FFI-free)
- **Status**: Production ready

## Future Enhancements

Potential areas for expansion:
- **Hygiene System**: Macro hygiene to prevent variable capture
- **Pattern Matching**: Advanced pattern matching in macro definitions
- **Compile-time Evaluation**: Execute macros at compile time
- **Macro Libraries**: Importable macro libraries
- **IDE Integration**: Syntax highlighting and completion for macros

---

For more information about the CURSED macro system, see the language specification and the `ast_mood` module documentation.
