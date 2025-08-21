# CURSED Hygienic Macro System

The CURSED programming language features a powerful hygienic macro system that enables advanced metaprogramming while maintaining type safety and avoiding common macro pitfalls.

## Table of Contents

1. [Overview](#overview)
2. [Declarative Macros](#declarative-macros)
3. [Pattern Matching](#pattern-matching)
4. [Hygiene System](#hygiene-system)
5. [Procedural Macros](#procedural-macros)
6. [Built-in Macros](#built-in-macros)
7. [Compile-time Execution](#compile-time-execution)
8. [Debugging and Error Reporting](#debugging-and-error-reporting)
9. [Best Practices](#best-practices)
10. [Examples](#examples)

## Overview

The CURSED macro system provides three types of macros:

- **Declarative Macros**: Pattern-based code transformation using `slay_macro!`
- **Procedural Macros**: Full AST manipulation at compile time
- **Built-in Macros**: Common patterns implemented natively

All macros are hygienic by default, preventing variable capture and name collisions.

## Declarative Macros

Declarative macros use pattern matching to transform code at compile time.

### Basic Syntax

```cursed
slay_macro! macro_name {
    (pattern1) => {
        expansion1
    },
    (pattern2) => {
        expansion2
    }
}
```

### Pattern Elements

- **Literals**: Exact token matches
- **Captures**: `$name:type` - capture tokens of specific types
- **Repetitions**: `$(...)*` - match zero or more repetitions
- **Alternations**: Multiple patterns with different expansions

### Capture Types

| Type | Description | Example |
|------|-------------|---------|
| `expr` | Expression | `$x:expr` matches `2 + 3` |
| `stmt` | Statement | `$s:stmt` matches `sus x = 5` |
| `ident` | Identifier | `$name:ident` matches `variable_name` |
| `ty` | Type | `$t:ty` matches `drip` or `tea` |
| `literal` | Literal value | `$val:literal` matches `42` or `"hello"` |
| `block` | Code block | `$code:block` matches `{ ... }` |

### Examples

#### Simple Debug Macro

```cursed
slay_macro! debug_print {
    ($msg:expr) => {
        ready (DEBUG_MODE) {
            vibez.spill("[DEBUG]", $msg)
        }
    }
}

// Usage
debug_print!("Hello, world!")
```

#### Math Operations

```cursed
slay_macro! math_op {
    (add $a:expr, $b:expr) => { $a + $b },
    (mul $a:expr, $b:expr) => { $a * $b },
    (square $x:expr) => { $x * $x }
}

// Usage
sus result = math_op!(add 5, 3)  // Expands to: 5 + 3
sus squared = math_op!(square 4)  // Expands to: 4 * 4
```

#### Vector Creation

```cursed
slay_macro! vec {
    () => { [] },
    ($($item:expr),*) => { [$($item),*] }
}

// Usage
sus empty = vec!()           // Expands to: []
sus numbers = vec!(1, 2, 3)  // Expands to: [1, 2, 3]
```

## Pattern Matching

The pattern matching system supports advanced features:

### Repetition Patterns

```cursed
slay_macro! struct_new {
    ($name:ident { $($field:ident: $value:expr),* }) => {
        $name {
            $($field: $value,)*
        }
    }
}

// Usage
sus point = struct_new!(Point { x: 10, y: 20 })
```

### Optional Elements

```cursed
slay_macro! function_def {
    (slay $name:ident($($param:ident: $ty:ty),*) $(-> $ret:ty)? $body:block) => {
        slay $name($($param: $ty),*) $($ret)? $body
    }
}
```

### Pattern Guards

```cursed
slay_macro! conditional_op {
    ($x:expr where $x > 0) => { $x * 2 },
    ($x:expr) => { 0 }
}
```

## Hygiene System

The hygiene system prevents common macro problems:

### Variable Capture Prevention

```cursed
slay_macro! with_temp {
    ($expr:expr) => {
        {
            sus temp = 42  // This won't capture outer 'temp'
            $expr + temp
        }
    }
}

slay test_hygiene() {
    sus temp = 100
    sus result = with_temp!(10)
    // temp is still 100, not affected by macro
    assert!(temp == 100)
}
```

### Symbol Renaming

The hygiene system automatically renames symbols to prevent conflicts:

```cursed
// Original macro expansion
sus temp = 42

// After hygiene (conceptual)
sus temp__hyg_0_1 = 42
```

### Intentional Capture

When you need to capture variables, use explicit naming:

```cursed
slay_macro! use_outer {
    ($var:ident) => {
        captured_$var  // Explicitly marked as capture
    }
}
```

## Procedural Macros

Procedural macros provide full AST manipulation capabilities:

```cursed
@proc_macro
slay derive_json(input: StructDefinition) FunctionDefinition {
    // Custom code generation logic
    damn generate_json_serializer(input)
}

// Usage
@derive_json
squad User {
    name: tea,
    age: drip,
}
```

### Proc Macro Types

1. **Function-like**: `custom_macro!(tokens)`
2. **Derive**: `@derive_trait`
3. **Attribute**: `@custom_attribute`

## Built-in Macros

CURSED provides several built-in macros for common patterns:

### Debug Macros

```cursed
debug_print!("Debug message")
assert!(condition)
assert!(condition, "Custom message")
```

### Collection Macros

```cursed
vec![1, 2, 3, 4, 5]        // Array creation
map!{key1: value1, key2: value2}  // Map creation
```

### Code Generation

```cursed
derive_json!(struct_definition)
derive_debug!(struct_definition)
derive_eq!(struct_definition)
```

### Format Macros

```cursed
format!("Hello, {}!", name)
stringify!(expression)
```

## Compile-time Execution

Macros execute during compilation, enabling:

### Constant Evaluation

```cursed
slay_macro! compile_time_calc {
    ($expr:expr) => {
        const_eval!($expr)
    }
}

sus result = compile_time_calc!(2 + 3 * 4)  // Computed at compile time
```

### Code Validation

```cursed
slay_macro! validate_sql {
    ($query:literal) => {
        compile_time_check_sql!($query)
        $query
    }
}
```

### Resource Embedding

```cursed
slay_macro! include_file {
    ($path:literal) => {
        compile_time_read_file!($path)
    }
}
```

## Debugging and Error Reporting

### Macro Expansion Tracing

Enable macro debugging with compiler flags:

```bash
cursed-zig --macro-debug file.csd
```

### Error Messages

The macro system provides detailed error messages:

```
Error: No matching pattern for macro 'math_op'
  --> file.csd:10:5
   |
10 |     math_op!(invalid 1, 2)
   |     ^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: Available patterns:
     - (add $a:expr, $b:expr)
     - (mul $a:expr, $b:expr)
     - (square $x:expr)
```

### Hygiene Violation Reports

```
Warning: Potential hygiene violation in macro 'unsafe_macro'
  --> file.csd:5:9
   |
5  |         sus temp = $value
   |         ^^^^
   |
   = note: Variable 'temp' may shadow outer scope
   = help: Use explicit naming or enable hygiene
```

## Best Practices

### 1. Use Descriptive Names

```cursed
// Good
slay_macro! create_getter {
    ($field:ident, $type:ty) => { ... }
}

// Avoid
slay_macro! cg {
    ($f:ident, $t:ty) => { ... }
}
```

### 2. Provide Multiple Patterns

```cursed
slay_macro! assert {
    ($condition:expr) => {
        assert!($condition, "Assertion failed")
    },
    ($condition:expr, $message:expr) => {
        ready (!($condition)) {
            yikes $message
        }
    }
}
```

### 3. Document Your Macros

```cursed
/// Creates a debug print statement that only executes in debug mode
/// 
/// # Examples
/// 
/// ```cursed
/// debug_print!("Debug info: {}", value)
/// ```
slay_macro! debug_print {
    // implementation
}
```

### 4. Use Type-Safe Patterns

```cursed
// Prefer specific capture types
slay_macro! safe_div {
    ($a:expr, $b:expr) => {
        ready ($b != 0) {
            $a / $b
        } otherwise {
            yikes "Division by zero"
        }
    }
}
```

### 5. Test Edge Cases

```cursed
// Test with empty inputs
vec!()

// Test with single elements  
vec!(42)

// Test with complex expressions
vec!(func_call(), 2 + 3, other_macro!())
```

## Examples

### JSON Serialization Macro

```cursed
slay_macro! derive_json {
    (squad $name:ident { $($field:ident: $type:ty),* }) => {
        slay to_json(self: $name) tea {
            sus result = "{"
            sus first = based
            
            $({
                ready (!first) { result += ", " }
                result += "\"" + stringify!($field) + "\": "
                result += json_value!(self.$field)
                first = cringe
            })*
            
            result += "}"
            damn result
        }
        
        slay from_json(json: tea) $name {
            sus obj = $name{}
            sus parsed = parse_json!(json)
            
            $({
                ready (parsed.has(stringify!($field))) {
                    obj.$field = parsed.get(stringify!($field))
                }
            })*
            
            damn obj
        }
    }
}
```

### Test Case Generator

```cursed
slay_macro! test_case {
    ($name:literal, $body:block) => {
        slay test_$name() {
            vibez.spill("Running test:", $name)
            
            later {
                vibez.spill("Test completed:", $name)
            }
            
            $body
        }
    }
}

// Usage
test_case!("addition", {
    assert!(2 + 2 == 4)
})
```

### Builder Pattern Generator

```cursed
slay_macro! generate_builder {
    (squad $name:ident { $($field:ident: $type:ty),* }) => {
        squad $name ## Builder {
            $($field: ?$type),*
        }
        
        impl $name ## Builder {
            slay new() $name ## Builder {
                $name ## Builder {
                    $($field: nah),*
                }
            }
            
            $(
                slay $field(self: &$name ## Builder, value: $type) &$name ## Builder {
                    self.$field = value
                    damn self
                }
            )*
            
            slay build(self: $name ## Builder) $name {
                $name {
                    $($field: self.$field expect("Field '$field' not set")),*
                }
            }
        }
    }
}
```

## Performance Considerations

1. **Compile Time**: Large macro expansions increase compilation time
2. **Code Size**: Repetitive macro expansions can increase binary size
3. **Debug Info**: Complex macros may complicate debugging

## Limitations

1. **Parsing Context**: Macros cannot access type information during expansion
2. **Recursive Limits**: Recursion depth is limited to prevent infinite expansion
3. **Hygiene Scope**: Some advanced hygiene features require explicit annotation

## Future Enhancements

- **Macro 2.0**: More powerful procedural macro system
- **IDE Integration**: Better macro expansion visualization
- **Cross-crate Macros**: Macro export/import system
- **Async Macros**: Macros for async/await code generation

This macro system makes CURSED highly expressive while maintaining safety and performance characteristics essential for systems programming.
