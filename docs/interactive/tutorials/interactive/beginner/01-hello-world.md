# Hello World - Your First CURSED Program

Welcome to CURSED! Let's start with the classic "Hello, World!" program and learn the fundamentals.

## Learning Objectives
- Write your first CURSED program
- Understand basic syntax
- Use the `vibez` module for output
- Run and execute CURSED code

## The Hello World Program

```cursed
# Import the vibez module for output
yeet "vibez"

# Print "Hello, World!" to the console
vibez.spill("Hello, World!")
```

### Try It Yourself

<interactive-editor>
yeet "vibez"

vibez.spill("Hello, World!")
</interactive-editor>

Click the "Run" button to execute this code!

## Breaking It Down

Let's understand each part of this program:

### 1. Importing Modules
```cursed
yeet "vibez"
```
- `yeet` is CURSED's import keyword
- `"vibez"` is the standard I/O module
- This gives us access to output functions

### 2. Printing Output
```cursed
vibez.spill("Hello, World!")
```
- `vibez.spill()` prints text to the console
- Similar to `print()` in Python or `println!()` in Rust
- The text is enclosed in double quotes

## Variations to Try

### Multiple Lines
<interactive-editor>
yeet "vibez"

vibez.spill("Hello, World!")
vibez.spill("Welcome to CURSED!")
vibez.spill("This is pretty cool, right?")
</interactive-editor>

### Using Variables
<interactive-editor>
yeet "vibez"

sus message tea = "Hello, CURSED!"
vibez.spill(message)
</interactive-editor>

### With Expressions
<interactive-editor>
yeet "vibez"

sus name tea = "Developer"
vibez.spill("Hello,", name, "!")
vibez.spill("2 + 2 =", 2 + 2)
</interactive-editor>

## Key Concepts Learned

1. **Module Import**: Use `yeet "module_name"` to import modules
2. **Function Calls**: Use dot notation to call module functions
3. **String Literals**: Text in double quotes
4. **Comments**: Use `#` for single-line comments

## Common Mistakes

### ❌ Forgetting to Import
```cursed
# This will cause an error!
vibez.spill("Hello, World!")  # vibez is not imported
```

### ❌ Wrong Syntax
```cursed
yeet "vibez"
# Missing parentheses
vibez.spill "Hello, World!"  # Should have ()
```

### ✅ Correct Version
```cursed
yeet "vibez"
vibez.spill("Hello, World!")
```

## Challenge

Try to create a program that prints your name and a fun fact about yourself:

<interactive-editor>
yeet "vibez"

# Your code here
# Example:
# sus name tea = "Alex"
# sus fact tea = "I love programming in CURSED!"
# vibez.spill("Hi, I'm", name)
# vibez.spill("Fun fact:", fact)
</interactive-editor>

## What's Next?

Great job! You've written your first CURSED program. In the next tutorial, we'll learn about:
- Variables and types
- Different data types in CURSED
- Basic arithmetic operations

[Continue to Variables and Types →](02-variables-types.md)

## Quick Reference

| Concept | Syntax | Example |
|---------|--------|---------|
| Import module | `yeet "module"` | `yeet "vibez"` |
| Print output | `vibez.spill(value)` | `vibez.spill("Hello!")` |
| Comment | `# comment` | `# This is a comment` |
| String literal | `"text"` | `"Hello, World!"` |

## Need Help?

- Check the [CURSED syntax reference](../../api/syntax.md)
- Join our [community Discord](../../community/discord.md)
- Browse [common issues](../../support/troubleshooting.md)
