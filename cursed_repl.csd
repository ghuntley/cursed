fr fr CURSED Interactive REPL - Pure CURSED Implementation
fr This is the first native REPL written entirely in CURSED language
fr Designed to be self-hosted and demonstrate advanced CURSED features

yeet "vibez"
yeet "stringz"
yeet "arrayz"
yeet "filez"
yeet "termz"

fr REPL Session State
squad ReplSession {
    sus variables facts<tea> = {}
    sus history []tea = []
    sus line_number drip = 1
    sus verbose lit = false
    sus running lit = true
    sus prompt tea = "cursed> "
    sus history_file tea = ".cursed_history"
}

fr Welcome message for REPL
slay show_welcome() {
    vibez.spill("🔥 CURSED REPL v1.0.0 - Pure CURSED Implementation")
    vibez.spill("Interactive CURSED language shell written in CURSED")
    vibez.spill("Type :help for help, :quit to exit")
    vibez.spill("")
}

fr Help message with all available commands
slay show_help() {
    vibez.spill("")
    vibez.spill("CURSED REPL Commands:")
    vibez.spill("  :help, :h         - Show this help message")
    vibez.spill("  :quit, :exit, :q  - Exit the REPL")
    vibez.spill("  :vars, :variables - Show current variables")  
    vibez.spill("  :history, :hist   - Show command history")
    vibez.spill("  :clear, :cls      - Clear the screen")
    vibez.spill("  :version          - Show version information")
    vibez.spill("  :save             - Save session to file")
    vibez.spill("  :load             - Load session from file")
    vibez.spill("")
    vibez.spill("CURSED Language Features:")
    vibez.spill("  Variables:  sus x drip = 42")
    vibez.spill("  Functions:  slay add(a drip, b drip) drip { damn a + b }")
    vibez.spill("  Arrays:     sus arr []drip = [1, 2, 3]")
    vibez.spill("  Print:      vibez.spill(\"Hello, world!\")")
    vibez.spill("  Import:     yeet \"stdlib_module\"")
    vibez.spill("  Control:    ready (condition) { ... }")
    vibez.spill("              bestie (condition) { ... }")
    vibez.spill("")
}

fr Show current session variables
slay show_variables(session *ReplSession) {
    ready (session.variables.length() == 0) {
        vibez.spill("  No variables defined")
        damn
    }
    
    vibez.spill("  Current variables:")
    bestie (var_name, var_value in session.variables) {
        vibez.spill("    " + var_name + " = " + var_value)
    }
}

fr Show command history
slay show_history(session *ReplSession) {
    ready (session.history.length() == 0) {
        vibez.spill("  No command history")
        damn
    }
    
    vibez.spill("  Command history:")
    bestie (i, command in session.history) {
        vibez.spill("  " + (i + 1).toString() + ": " + command)
    }
}

fr Clear screen using ANSI escape sequences
slay clear_screen() {
    vibez.spill("\x1B[2J\x1B[1;1H")
}

fr Show version information
slay show_version() {
    vibez.spill("CURSED REPL v1.0.0 (Pure CURSED Implementation)")
    vibez.spill("Built with CURSED standard library")
    vibez.spill("Runtime: CURSED Interpreter")
}

fr Add command to history with duplicate prevention
slay add_to_history(session *ReplSession, command tea) {
    fr Don't add empty commands
    ready (command.trim() == "") {
        damn
    }
    
    fr Don't add duplicates of the last command
    ready (session.history.length() > 0 && session.history[-1] == command) {
        damn
    }
    
    fr Add to history array
    session.history.append(command)
    
    fr Keep history size manageable (last 1000 entries)
    ready (session.history.length() > 1000) {
        session.history = session.history.slice(-1000)
    }
}

fr Save session history to file
slay save_history(session *ReplSession) {
    ready (session.history.length() == 0) {
        damn
    }
    
    sus history_content tea = ""
    bestie (command in session.history) {
        history_content += command + "\n"
    }
    
    filez.writeFile(session.history_file, history_content) fam {
        when _ -> {
            vibez.spill("⚠️  Warning: Could not save history")
        }
    }
    
    ready (session.verbose) {
        vibez.spill("💾 History saved (" + session.history.length().toString() + " entries)")
    }
}

fr Load session history from file
slay load_history(session *ReplSession) {
    sus content tea = filez.readFile(session.history_file) fam {
        when _ -> {
            fr File doesn't exist yet, that's fine
            damn
        }
    }
    
    sus lines []tea = stringz.split(content, "\n")
    bestie (line in lines) {
        sus trimmed tea = line.trim()
        ready (trimmed != "") {
            session.history.append(trimmed)
        }
    }
    
    ready (session.verbose && session.history.length() > 0) {
        vibez.spill("📜 Loaded " + session.history.length().toString() + " history entries")
    }
}

fr Parse and handle variable declarations: sus x drip = 42
slay handle_variable_declaration(session *ReplSession, input tea) tea {
    fr Remove "sus " prefix
    sus declaration tea = input.substring(4).trim()
    
    fr Find the equals sign
    sus equals_pos drip = declaration.indexOf("=")
    ready (equals_pos == -1) {
        damn "Error: Invalid variable declaration syntax"
    }
    
    fr Extract variable name, type, and value
    sus left_side tea = declaration.substring(0, equals_pos).trim()
    sus value_expr tea = declaration.substring(equals_pos + 1).trim()
    
    fr Parse variable name and optional type
    sus parts []tea = stringz.split(left_side, " ")
    sus var_name tea = parts[0]
    sus var_type tea = ready (parts.length() > 1) parts[1] otherwise ""
    
    fr Evaluate the value expression (simplified)
    sus value tea = evaluate_expression(session, value_expr)
    
    fr Store the variable
    session.variables[var_name] = value
    
    ready (session.verbose) {
        vibez.spill("  ✅ Variable declared: " + var_name + " = " + value)
    }
    
    damn value
}

fr Simple expression evaluator for basic arithmetic and values
slay evaluate_expression(session *ReplSession, expr tea) tea {
    sus trimmed tea = expr.trim()
    
    fr Handle string literals
    ready (trimmed.startsWith("\"") && trimmed.endsWith("\"")) {
        damn trimmed.substring(1, trimmed.length() - 1)
    }
    
    fr Handle numeric literals
    ready (stringz.isNumeric(trimmed)) {
        damn trimmed
    }
    
    fr Handle boolean literals
    ready (trimmed == "based" || trimmed == "cap") {
        damn trimmed
    }
    
    fr Handle variable references
    ready (session.variables.containsKey(trimmed)) {
        damn session.variables[trimmed]
    }
    
    fr Handle simple arithmetic (a + b)
    bestie (op in ["+", "-", "*", "/"]) {
        sus op_pos drip = trimmed.indexOf(op)
        ready (op_pos > 0) {
            sus left tea = trimmed.substring(0, op_pos).trim()
            sus right tea = trimmed.substring(op_pos + 1).trim()
            
            sus left_val drip = parseNumber(evaluate_expression(session, left))
            sus right_val drip = parseNumber(evaluate_expression(session, right))
            
            ready (op == "+") damn (left_val + right_val).toString()
            ready (op == "-") damn (left_val - right_val).toString()
            ready (op == "*") damn (left_val * right_val).toString()
            ready (op == "/") {
                ready (right_val == 0) damn "Error: Division by zero"
                damn (left_val / right_val).toString()
            }
        }
    }
    
    fr Handle array literals: [1, 2, 3]
    ready (trimmed.startsWith("[") && trimmed.endsWith("]")) {
        sus inner tea = trimmed.substring(1, trimmed.length() - 1).trim()
        sus elements []tea = stringz.split(inner, ",")
        sus result []tea = []
        
        bestie (elem in elements) {
            result.append(evaluate_expression(session, elem.trim()))
        }
        
        damn "[" + stringz.join(result, ", ") + "]"
    }
    
    fr Default: return as-is
    damn trimmed
}

fr Parse a string as a number, return 0 if not numeric
slay parseNumber(str tea) drip {
    ready (stringz.isNumeric(str)) {
        damn str.toNumber()
    }
    damn 0
}

fr Handle print statements: vibez.spill(...)
slay handle_print_statement(session *ReplSession, input tea) tea {
    fr Simple regex-like parsing for vibez.spill(content)
    sus start_pos drip = input.indexOf("vibez.spill(")
    ready (start_pos == -1) {
        damn "Error: Invalid print statement"
    }
    
    start_pos += 12  fr Length of "vibez.spill("
    sus end_pos drip = input.lastIndexOf(")")
    
    ready (end_pos <= start_pos) {
        damn "Error: Unclosed print statement"
    }
    
    sus content tea = input.substring(start_pos, end_pos).trim()
    sus evaluated tea = evaluate_expression(session, content)
    
    fr Remove quotes if it's a string literal
    ready (evaluated.startsWith("\"") && evaluated.endsWith("\"")) {
        evaluated = evaluated.substring(1, evaluated.length() - 1)
    }
    
    vibez.spill(evaluated)
    damn ""
}

fr Handle assignment: var = value
slay handle_assignment(session *ReplSession, input tea) tea {
    sus equals_pos drip = input.indexOf("=")
    sus var_name tea = input.substring(0, equals_pos).trim()
    sus value_expr tea = input.substring(equals_pos + 1).trim()
    
    sus value tea = evaluate_expression(session, value_expr)
    session.variables[var_name] = value
    
    ready (session.verbose) {
        vibez.spill("  ✅ Assignment: " + var_name + " = " + value)
    }
    
    damn value
}

fr Main evaluation function for user input
slay evaluate_input(session *ReplSession, input tea) tea {
    sus trimmed tea = input.trim()
    ready (trimmed == "") {
        damn ""
    }
    
    fr Add to command history
    add_to_history(session, trimmed)
    
    fr Handle variable declarations: sus x drip = 42
    ready (trimmed.startsWith("sus ")) {
        damn handle_variable_declaration(session, trimmed)
    }
    
    fr Handle print statements: vibez.spill(...)
    ready (trimmed.indexOf("vibez.spill(") != -1) {
        damn handle_print_statement(session, trimmed)
    }
    
    fr Handle assignments: var = value
    ready (trimmed.indexOf("=") != -1) {
        damn handle_assignment(session, trimmed)
    }
    
    fr Handle function definitions: slay func(...) { ... }
    ready (trimmed.startsWith("slay ")) {
        vibez.spill("📝 Function definition recognized (advanced parsing needed)")
        damn "Function definition placeholder"
    }
    
    fr Handle module imports: yeet "module"
    ready (trimmed.startsWith("yeet ")) {
        sus module_name tea = trimmed.substring(5).trim()
        ready (module_name.startsWith("\"") && module_name.endsWith("\"")) {
            module_name = module_name.substring(1, module_name.length() - 1)
            vibez.spill("📦 Module '" + module_name + "' imported (placeholder)")
            damn ""
        }
        damn "Error: Invalid import syntax"
    }
    
    fr Default: try to evaluate as expression
    damn evaluate_expression(session, trimmed)
}

fr Handle special REPL commands (starting with :)
slay handle_special_command(session *ReplSession, input tea) lit {
    sus trimmed tea = input.trim()
    
    ready (trimmed == ":quit" || trimmed == ":exit" || trimmed == ":q") {
        session.running = false
        damn true
    }
    
    ready (trimmed == ":help" || trimmed == ":h") {
        show_help()
        damn true
    }
    
    ready (trimmed == ":vars" || trimmed == ":variables") {
        show_variables(session)
        damn true
    }
    
    ready (trimmed == ":history" || trimmed == ":hist") {
        show_history(session)
        damn true
    }
    
    ready (trimmed == ":clear" || trimmed == ":cls") {
        clear_screen()
        damn true
    }
    
    ready (trimmed == ":version") {
        show_version()
        damn true
    }
    
    ready (trimmed == ":save") {
        save_history(session)
        vibez.spill("💾 Session saved")
        damn true
    }
    
    ready (trimmed == ":load") {
        load_history(session)
        vibez.spill("📂 Session loaded")
        damn true
    }
    
    ready (trimmed.startsWith(":")) {
        vibez.spill("Unknown command: " + trimmed + ". Type :help for available commands.")
        damn true
    }
    
    damn false
}

fr Main REPL loop function
slay run_repl() {
    fr Initialize REPL session
    sus session ReplSession = ReplSession{
        variables: {},
        history: [],
        line_number: 1,
        verbose: false,
        running: true,
        prompt: "cursed> ",
        history_file: ".cursed_history"
    }
    
    fr Load existing history
    load_history(&session)
    
    fr Show welcome message
    show_welcome()
    
    fr Main input loop
    bestie (session.running) {
        fr Display prompt and read input
        vibez.spill_no_newline(session.prompt)
        
        fr Read input from terminal (simplified - would use termz in real implementation)
        sus input tea = termz.readline() fam {
            when _ -> {
                fr EOF or error occurred
                session.running = false
                break
            }
        }
        
        fr Handle empty input
        ready (input.trim() == "") {
            continue
        }
        
        fr Handle special commands
        ready (handle_special_command(&session, input)) {
            continue
        }
        
        fr Evaluate regular CURSED input
        sus result tea = evaluate_input(&session, input) fam {
            when error -> {
                vibez.spill("Error: " + error.toString())
                continue
            }
        }
        
        fr Display result if not empty
        ready (result.trim() != "") {
            vibez.spill(result)
        }
        
        session.line_number += 1
    }
    
    fr Save history on exit
    save_history(&session)
    vibez.spill("Goodbye! 👋")
}

fr Entry point - start the REPL
run_repl()
