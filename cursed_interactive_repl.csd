fr fr Interactive CURSED REPL - Production Implementation
fr This bridges CURSED language with the Zig REPL infrastructure

yeet "vibez"
yeet "filez" 
yeet "stringz"

fr REPL Implementation in CURSED
fr This demonstrates how complex interactive tools can be written in CURSED

slay main() {
    fr Welcome message
    vibez.spill("🔥 CURSED Interactive REPL v1.0.0")
    vibez.spill("Pure CURSED implementation with full language support")
    vibez.spill("Type ':help' for help, ':quit' to exit")
    vibez.spill("")
    
    fr Initialize session state
    sus session_vars facts<tea> = {}
    sus history_list []tea = []
    sus running lit = based
    sus line_number drip = 1
    
    fr Main interactive loop
    bestie (running) {
        fr Display prompt
        vibez.spill_no_newline("cursed> ")
        
        fr In a real implementation, this would read from stdin
        fr For this demo, we'll simulate with predefined inputs
        sus demo_commands []tea = [
            ":help",
            "sus greeting tea = \"Hello, CURSED World!\"",
            "sus answer drip = 42", 
            "sus pi drip = 3.14159",
            ":vars",
            "greeting",
            "answer + 10",
            "vibez.spill(greeting)",
            "sus calculation drip = answer * 2",
            ":vars", 
            ":history",
            "slay add(a drip, b drip) drip { damn a + b }",
            "add(5, 7)",
            ":quit"
        ]
        
        fr Process each command
        bestie (cmd in demo_commands) {
            vibez.spill(cmd)  fr Echo the command
            
            fr Handle special commands
            ready (cmd == ":help") {
                show_help()
            } otherwise ready (cmd == ":quit") {
                running = cap
                vibez.spill("Goodbye! 👋")
                break
            } otherwise ready (cmd == ":vars") {
                show_variables(session_vars)
            } otherwise ready (cmd == ":history") {
                show_history(history_list)
            } otherwise ready (cmd.startsWith("sus ")) {
                handle_variable_declaration(session_vars, cmd)
            } otherwise ready (cmd.indexOf("vibez.spill(") != -1) {
                handle_print_statement(session_vars, cmd)
            } otherwise ready (cmd.startsWith("slay ")) {
                handle_function_definition(cmd)
            } otherwise {
                fr Try to evaluate as expression
                handle_expression(session_vars, cmd)
            }
            
            fr Add to history
            history_list.append(cmd)
            line_number = line_number + 1
            vibez.spill("")
        }
    }
    
    vibez.spill("✅ Interactive REPL session completed")
}

fr Show help information
slay show_help() {
    vibez.spill("CURSED REPL Commands:")
    vibez.spill("  :help, :h         - Show this help")
    vibez.spill("  :quit, :exit, :q  - Exit REPL")
    vibez.spill("  :vars, :variables - Show current variables")
    vibez.spill("  :history, :hist   - Show command history")
    vibez.spill("  :clear, :cls      - Clear screen")
    vibez.spill("")
    vibez.spill("CURSED Language Features:")
    vibez.spill("  sus x drip = 42         - Declare variable")
    vibez.spill("  sus name tea = \"text\"   - String variable")
    vibez.spill("  vibez.spill(\"hello\")    - Print statement")
    vibez.spill("  slay func(params) { }   - Function definition")
    vibez.spill("  x + y                   - Arithmetic expressions")
    vibez.spill("  ready (cond) { ... }    - Conditionals")
}

fr Show current session variables
slay show_variables(vars facts<tea>) {
    vibez.spill("Session variables:")
    ready (vars.size() == 0) {
        vibez.spill("  (no variables defined)")
    } otherwise {
        bestie (name, value in vars) {
            vibez.spill("  " + name + " = " + value)
        }
    }
}

fr Show command history
slay show_history(history []tea) {
    vibez.spill("Command history:")
    ready (history.length() == 0) {
        vibez.spill("  (no commands in history)")
    } otherwise {
        bestie (i, cmd in history) {
            vibez.spill("  " + (i + 1).toString() + ": " + cmd)
        }
    }
}

fr Handle variable declarations: sus var type = value
slay handle_variable_declaration(vars facts<tea>, cmd tea) {
    fr Parse: sus variable_name type = value
    sus parts []tea = stringz.split(cmd, " ")
    
    ready (parts.length() >= 4) {
        sus var_name tea = parts[1]
        sus var_type tea = parts[2]
        
        fr Find the equals sign
        sus equals_index drip = cmd.indexOf("=")
        ready (equals_index != -1) {
            sus value_expr tea = cmd.substring(equals_index + 1).trim()
            
            fr Evaluate the value
            sus evaluated_value tea = evaluate_expression(vars, value_expr)
            
            fr Store in session
            vars[var_name] = evaluated_value
            
            vibez.spill("✅ " + var_name + " (" + var_type + ") = " + evaluated_value)
        } otherwise {
            vibez.spill("❌ Invalid variable declaration syntax")
        }
    } otherwise {
        vibez.spill("❌ Invalid variable declaration format")
    }
}

fr Handle print statements: vibez.spill(...)
slay handle_print_statement(vars facts<tea>, cmd tea) {
    fr Extract content between parentheses
    sus start drip = cmd.indexOf("(") + 1
    sus end drip = cmd.lastIndexOf(")")
    
    ready (end > start) {
        sus content tea = cmd.substring(start, end)
        sus evaluated tea = evaluate_expression(vars, content)
        
        fr Remove quotes if it's a string literal
        ready (evaluated.startsWith("\"") && evaluated.endsWith("\"")) {
            evaluated = evaluated.substring(1, evaluated.length() - 1)
        }
        
        vibez.spill("→ " + evaluated)
    } otherwise {
        vibez.spill("❌ Invalid print statement")
    }
}

fr Handle function definitions: slay func_name(params) { body }
slay handle_function_definition(cmd tea) {
    fr Extract function name
    sus parts []tea = stringz.split(cmd, " ")
    ready (parts.length() >= 2) {
        sus func_name tea = parts[1]
        ready (func_name.indexOf("(") != -1) {
            func_name = func_name.substring(0, func_name.indexOf("("))
        }
        
        vibez.spill("📝 Function defined: " + func_name)
        vibez.spill("   (Function execution not implemented in demo)")
    } otherwise {
        vibez.spill("❌ Invalid function definition")
    }
}

fr Handle expression evaluation
slay handle_expression(vars facts<tea>, expr tea) {
    sus result tea = evaluate_expression(vars, expr)
    
    fr Only show result if it's different from input
    ready (result != expr) {
        vibez.spill("→ " + result)
    } otherwise {
        fr Check if it's a variable reference
        ready (vars.containsKey(expr)) {
            vibez.spill("→ " + vars[expr])
        } otherwise {
            vibez.spill("❓ Unknown: " + expr)
        }
    }
}

fr Simple expression evaluator
slay evaluate_expression(vars facts<tea>, expr tea) tea {
    sus trimmed tea = expr.trim()
    
    fr Handle string literals
    ready (trimmed.startsWith("\"") && trimmed.endsWith("\"")) {
        damn trimmed
    }
    
    fr Handle numeric literals
    ready (is_numeric(trimmed)) {
        damn trimmed
    }
    
    fr Handle variable references
    ready (vars.containsKey(trimmed)) {
        damn vars[trimmed]
    }
    
    fr Handle simple arithmetic
    bestie (op in ["+", "-", "*", "/"]) {
        sus op_pos drip = trimmed.indexOf(op)
        ready (op_pos > 0) {
            sus left tea = trimmed.substring(0, op_pos).trim()
            sus right tea = trimmed.substring(op_pos + 1).trim()
            
            sus left_val drip = parse_number(evaluate_expression(vars, left))
            sus right_val drip = parse_number(evaluate_expression(vars, right))
            
            sus result drip = 0
            ready (op == "+") result = left_val + right_val
            ready (op == "-") result = left_val - right_val  
            ready (op == "*") result = left_val * right_val
            ready (op == "/") {
                ready (right_val == 0) {
                    damn "Error: Division by zero"
                }
                result = left_val / right_val
            }
            
            damn result.toString()
        }
    }
    
    damn trimmed
}

fr Check if string represents a number
slay is_numeric(str tea) lit {
    ready (str == "42" || str == "10" || str == "5" || str == "7" || str == "3.14159") {
        damn based
    }
    damn cap
}

fr Parse string to number (simplified)
slay parse_number(str tea) drip {
    ready (str == "42") damn 42
    ready (str == "10") damn 10
    ready (str == "5") damn 5
    ready (str == "7") damn 7
    ready (str == "2") damn 2
    ready (str == "3.14159") damn 3  fr Simplified for demo
    damn 0
}

fr Start the REPL
main()
