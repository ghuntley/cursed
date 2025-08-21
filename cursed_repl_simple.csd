fr fr Simple CURSED REPL - Working with current interpreter
fr Demonstrates basic REPL functionality in pure CURSED

yeet "vibez"

fr REPL session variables
sus session_variables facts<tea> = {}
sus command_history []tea = []
sus line_count drip = 1

fr Welcome message
vibez.spill("🔥 CURSED REPL v1.0.0 - Simple Implementation")
vibez.spill("Type 'help' for commands, 'quit' to exit")
vibez.spill("")

fr Show help information
slay show_help() {
    vibez.spill("Available commands:")
    vibez.spill("  help     - Show this help")
    vibez.spill("  quit     - Exit REPL")
    vibez.spill("  vars     - Show variables")
    vibez.spill("  history  - Show command history")
    vibez.spill("  clear    - Clear screen")
    vibez.spill("")
    vibez.spill("CURSED syntax:")
    vibez.spill("  sus x drip = 42        # Variable")
    vibez.spill("  vibez.spill(\"hello\")   # Print")
    vibez.spill("  x + 5                  # Expression")
}

fr Show current variables
slay show_variables() {
    vibez.spill("Current variables:")
    ready (session_variables.length() == 0) {
        vibez.spill("  (no variables defined)")
    } otherwise {
        bestie (name, value in session_variables) {
            vibez.spill("  " + name + " = " + value)
        }
    }
}

fr Show command history
slay show_command_history() {
    vibez.spill("Command history:")
    ready (command_history.length() == 0) {
        vibez.spill("  (no commands in history)")
    } otherwise {
        bestie (i, cmd in command_history) {
            vibez.spill("  " + (i + 1).toString() + ": " + cmd)
        }
    }
}

fr Clear screen
slay clear_screen() {
    vibez.spill("\n\n\n\n\n\n\n\n\n\n")  fr Simple clear
    vibez.spill("Screen cleared")
}

fr Add command to history
slay add_to_history(cmd tea) {
    ready (cmd.trim() != "" && command_history.length() < 100) {
        command_history.append(cmd)
    }
}

fr Simple expression evaluation
slay evaluate_simple_expression(expr tea) tea {
    fr Handle numeric values
    ready (expr == "42") damn "42"
    ready (expr == "10") damn "10"
    ready (expr == "5") damn "5"
    
    fr Handle string literals
    ready (expr.startsWith("\"") && expr.endsWith("\"")) {
        damn expr.substring(1, expr.length() - 1)
    }
    
    fr Handle variable lookups
    ready (session_variables.containsKey(expr)) {
        damn session_variables[expr]
    }
    
    fr Handle simple arithmetic
    ready (expr.indexOf("+") != -1) {
        sus parts []tea = stringz.split(expr, "+")
        ready (parts.length() == 2) {
            sus left tea = parts[0].trim()
            sus right tea = parts[1].trim()
            
            sus left_val drip = 0
            sus right_val drip = 0
            
            ready (stringz.isNumeric(left)) left_val = left.toNumber()
            ready (stringz.isNumeric(right)) right_val = right.toNumber()
            
            damn (left_val + right_val).toString()
        }
    }
    
    damn expr
}

fr Process user input
slay process_input(input tea) lit {
    sus trimmed tea = input.trim()
    
    fr Handle empty input
    ready (trimmed == "") {
        damn false
    }
    
    fr Add to history
    add_to_history(trimmed)
    
    fr Handle commands
    ready (trimmed == "help") {
        show_help()
        damn false
    }
    
    ready (trimmed == "quit" || trimmed == "exit") {
        vibez.spill("Goodbye!")
        damn true
    }
    
    ready (trimmed == "vars") {
        show_variables()
        damn false
    }
    
    ready (trimmed == "history") {
        show_command_history()
        damn false
    }
    
    ready (trimmed == "clear") {
        clear_screen()
        damn false
    }
    
    fr Handle variable declarations: sus x drip = value
    ready (trimmed.startsWith("sus ")) {
        sus declaration tea = trimmed.substring(4)
        sus equals_pos drip = declaration.indexOf("=")
        
        ready (equals_pos != -1) {
            sus left tea = declaration.substring(0, equals_pos).trim()
            sus right tea = declaration.substring(equals_pos + 1).trim()
            
            fr Parse variable name (ignore type for now)
            sus var_name tea = left.split(" ")[0]
            sus value tea = evaluate_simple_expression(right)
            
            session_variables[var_name] = value
            vibez.spill("Variable set: " + var_name + " = " + value)
        } otherwise {
            vibez.spill("Error: Invalid variable declaration")
        }
        damn false
    }
    
    fr Handle print statements
    ready (trimmed.indexOf("vibez.spill(") != -1) {
        sus start drip = trimmed.indexOf("(") + 1
        sus end drip = trimmed.lastIndexOf(")")
        
        ready (end > start) {
            sus content tea = trimmed.substring(start, end)
            sus evaluated tea = evaluate_simple_expression(content)
            vibez.spill(evaluated)
        } otherwise {
            vibez.spill("Error: Invalid print statement")
        }
        damn false
    }
    
    fr Handle simple expressions
    sus result tea = evaluate_simple_expression(trimmed)
    ready (result != trimmed) {
        vibez.spill(result)
    } otherwise {
        vibez.spill("Unknown command or syntax: " + trimmed)
    }
    
    damn false
}

fr Main REPL simulation loop
fr This simulates interactive input - in real implementation would read from stdin
sus demo_inputs []tea = [
    "help",
    "sus x drip = 42", 
    "sus name tea = \"CURSED\"",
    "vars",
    "x + 5",
    "vibez.spill(\"Hello from CURSED REPL!\")",
    "vibez.spill(name)",
    "history",
    "quit"
]

vibez.spill("Running REPL simulation with demo inputs...")
vibez.spill("In real implementation, this would read from terminal stdin")
vibez.spill("")

bestie (input in demo_inputs) {
    vibez.spill("cursed> " + input)
    ready (process_input(input)) {
        break
    }
    vibez.spill("")
}

vibez.spill("")
vibez.spill("✅ REPL simulation completed!")
vibez.spill("This demonstrates the foundation for interactive CURSED REPL")
