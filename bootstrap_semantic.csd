fr fr Bootstrap Semantic Analyzer in Pure CURSED
fr fr Type checking and semantic validation for CURSED AST

fr fr Type constants
sus TYPE_UNKNOWN normie = 0
sus TYPE_NORMIE normie = 1    fr fr int
sus TYPE_TEA normie = 2       fr fr string
sus TYPE_LIT normie = 3       fr fr bool
sus TYPE_MEAL normie = 4      fr fr float
sus TYPE_VOID normie = 5      fr fr void/no return

fr fr AST Node types (copy from parser)
sus AST_PROGRAM normie = 100
sus AST_FUNCTION normie = 101
sus AST_VARIABLE normie = 102
sus AST_ASSIGNMENT normie = 103
sus AST_CALL normie = 104
sus AST_BLOCK normie = 105
sus AST_RETURN normie = 106

fr fr Symbol table entry
squad Symbol {
    spill name tea
    spill symbol_type normie
    spill is_function lit
    spill line normie
    spill column normie
}

fr fr Simple symbol table
squad SymbolTable {
    spill symbols []Symbol
    spill parent SymbolTable
}

fr fr Semantic analyzer state
squad SemanticAnalyzer {
    spill current_scope SymbolTable
    spill errors []tea
    spill current_function tea
}

fr fr AST Node structure (copy from parser)
squad ASTNode {
    spill node_type normie
    spill value tea
    spill children []ASTNode
    spill line normie
    spill column normie
}

fr fr Create new symbol table
slay new_symbol_table(parent SymbolTable) SymbolTable {
    sus symbols []Symbol = []Symbol{}
    damn SymbolTable{
        symbols: symbols,
        parent: parent
    }
}

fr fr Add symbol to table
slay add_symbol(table SymbolTable, name tea, symbol_type normie, is_function lit, line normie, column normie) {
    sus symbol Symbol = Symbol{
        name: name,
        symbol_type: symbol_type,
        is_function: is_function,
        line: line,
        column: column
    }
    table.symbols.push(symbol)
}

fr fr Lookup symbol in table
slay lookup_symbol(table SymbolTable, name tea) Symbol {
    fr fr Search current scope
    bestie (i := 0; i < table.symbols.len(); i = i + 1) {
        sus symbol Symbol = table.symbols[i]
        bestie (symbol.name == name) {
            damn symbol
        }
    }
    
    fr fr Symbol not found - return unknown symbol
    damn Symbol{
        name: name,
        symbol_type: TYPE_UNKNOWN,
        is_function: cringe,
        line: 0,
        column: 0
    }
}

fr fr Create new semantic analyzer
slay new_semantic_analyzer() SemanticAnalyzer {
    sus global_scope SymbolTable = new_symbol_table(SymbolTable{})
    sus errors []tea = []tea{}
    
    fr fr Add built-in functions
    add_symbol(global_scope, "vibez.spill", TYPE_VOID, based, 0, 0)
    
    damn SemanticAnalyzer{
        current_scope: global_scope,
        errors: errors,
        current_function: ""
    }
}

fr fr Add error to analyzer
slay add_error(analyzer SemanticAnalyzer, message tea) {
    analyzer.errors.push(message)
}

fr fr Get type from type name string
slay get_type_from_name(type_name tea) normie {
    bestie (type_name == "normie") {
        damn TYPE_NORMIE
    } capish bestie (type_name == "tea") {
        damn TYPE_TEA
    } capish bestie (type_name == "lit") {
        damn TYPE_LIT
    } capish bestie (type_name == "meal") {
        damn TYPE_MEAL
    } capish {
        damn TYPE_UNKNOWN
    }
}

fr fr Get type name from type constant
slay get_type_name(type_id normie) tea {
    bestie (type_id == TYPE_NORMIE) {
        damn "normie"
    } capish bestie (type_id == TYPE_TEA) {
        damn "tea"
    } capish bestie (type_id == TYPE_LIT) {
        damn "lit"
    } capish bestie (type_id == TYPE_MEAL) {
        damn "meal"
    } capish bestie (type_id == TYPE_VOID) {
        damn "void"
    } capish {
        damn "unknown"
    }
}

fr fr Analyze AST node
slay analyze_node(analyzer SemanticAnalyzer, node ASTNode) normie {
    bestie (node.node_type == AST_PROGRAM) {
        damn analyze_program(analyzer, node)
    } capish bestie (node.node_type == AST_FUNCTION) {
        damn analyze_function(analyzer, node)
    } capish bestie (node.node_type == AST_VARIABLE) {
        damn analyze_variable(analyzer, node)
    } capish bestie (node.node_type == AST_BLOCK) {
        damn analyze_block(analyzer, node)
    } capish bestie (node.node_type == AST_RETURN) {
        damn analyze_return(analyzer, node)
    } capish {
        damn TYPE_UNKNOWN
    }
}

fr fr Analyze program
slay analyze_program(analyzer SemanticAnalyzer, node ASTNode) normie {
    vibez.spill("Analyzing program node")
    
    fr fr Analyze all top-level declarations
    bestie (i := 0; i < node.children.len(); i = i + 1) {
        analyze_node(analyzer, node.children[i])
    }
    
    damn TYPE_VOID
}

fr fr Analyze function declaration
slay analyze_function(analyzer SemanticAnalyzer, node ASTNode) normie {
    vibez.spill("Analyzing function node")
    
    bestie (node.children.len() > 0) {
        sus name_node ASTNode = node.children[0]
        sus func_name tea = name_node.value
        analyzer.current_function = func_name
        
        vibez.spill("Analyzing function:", func_name)
        
        fr fr Add function to symbol table
        add_symbol(analyzer.current_scope, func_name, TYPE_VOID, based, node.line, node.column)
        
        fr fr Analyze function body if present
        bestie (node.children.len() > 1) {
            analyze_node(analyzer, node.children[1])
        }
    }
    
    damn TYPE_VOID
}

fr fr Analyze variable
slay analyze_variable(analyzer SemanticAnalyzer, node ASTNode) normie {
    vibez.spill("Analyzing variable:", node.value)
    
    fr fr For variable declarations, infer type from value
    bestie (node.children.len() > 0) {
        sus value_node ASTNode = node.children[0]
        sus inferred_type normie = analyze_node(analyzer, value_node)
        
        fr fr Add variable to symbol table
        add_symbol(analyzer.current_scope, node.value, inferred_type, cringe, node.line, node.column)
        
        vibez.spill("Variable", node.value, "has type", get_type_name(inferred_type))
        damn inferred_type
    } capish {
        fr fr Variable reference - lookup in symbol table
        sus symbol Symbol = lookup_symbol(analyzer.current_scope, node.value)
        bestie (symbol.symbol_type == TYPE_UNKNOWN) {
            add_error(analyzer, "Undefined variable: " + node.value)
        }
        damn symbol.symbol_type
    }
}

fr fr Analyze block
slay analyze_block(analyzer SemanticAnalyzer, node ASTNode) normie {
    vibez.spill("Analyzing block")
    
    fr fr Create new scope for block
    sus old_scope SymbolTable = analyzer.current_scope
    sus new_scope SymbolTable = new_symbol_table(old_scope)
    analyzer.current_scope = new_scope
    
    fr fr Analyze all statements in block
    bestie (i := 0; i < node.children.len(); i = i + 1) {
        analyze_node(analyzer, node.children[i])
    }
    
    fr fr Restore previous scope
    analyzer.current_scope = old_scope
    
    damn TYPE_VOID
}

fr fr Analyze return statement
slay analyze_return(analyzer SemanticAnalyzer, node ASTNode) normie {
    vibez.spill("Analyzing return statement")
    
    bestie (node.children.len() > 0) {
        sus return_type normie = analyze_node(analyzer, node.children[0])
        vibez.spill("Return type:", get_type_name(return_type))
        damn return_type
    } capish {
        damn TYPE_VOID
    }
}

fr fr Infer type from literal value
slay infer_literal_type(value tea) normie {
    fr fr Check if it's a number
    bestie (value >= "0" && value <= "9") {
        fr fr Simple heuristic - if contains dot, it's float
        bestie (value.contains(".")) {
            damn TYPE_MEAL
        } capish {
            damn TYPE_NORMIE
        }
    } capish bestie (value == "based" || value == "cringe") {
        damn TYPE_LIT
    } capish bestie (value.starts_with("\"")) {
        damn TYPE_TEA
    } capish {
        damn TYPE_UNKNOWN
    }
}

fr fr Run semantic analysis
slay run_semantic_analysis(ast ASTNode) SemanticAnalyzer {
    vibez.spill("Starting semantic analysis")
    
    sus analyzer SemanticAnalyzer = new_semantic_analyzer()
    analyze_node(analyzer, ast)
    
    vibez.spill("Semantic analysis complete")
    vibez.spill("Errors found:", analyzer.errors.len())
    
    fr fr Print errors
    bestie (i := 0; i < analyzer.errors.len(); i = i + 1) {
        vibez.spill("Error:", analyzer.errors[i])
    }
    
    damn analyzer
}

fr fr Test semantic analyzer with simple AST
slay test_semantic_analyzer() {
    vibez.spill("Bootstrap Semantic Analyzer Test")
    
    fr fr Create a simple AST manually for testing
    sus program ASTNode = ASTNode{
        node_type: AST_PROGRAM,
        value: "program",
        children: []ASTNode{},
        line: 1,
        column: 1
    }
    
    fr fr Function: slay hello() { }
    sus func_node ASTNode = ASTNode{
        node_type: AST_FUNCTION,
        value: "function",
        children: []ASTNode{},
        line: 1,
        column: 1
    }
    
    sus name_node ASTNode = ASTNode{
        node_type: AST_VARIABLE,
        value: "hello",
        children: []ASTNode{},
        line: 1,
        column: 6
    }
    
    sus block_node ASTNode = ASTNode{
        node_type: AST_BLOCK,
        value: "block",
        children: []ASTNode{},
        line: 1,
        column: 14
    }
    
    fr fr Variable: sus x normie = 42
    sus var_node ASTNode = ASTNode{
        node_type: AST_VARIABLE,
        value: "x",
        children: []ASTNode{},
        line: 1,
        column: 20
    }
    
    sus value_node ASTNode = ASTNode{
        node_type: AST_VARIABLE,
        value: "42",
        children: []ASTNode{},
        line: 1,
        column: 31
    }
    
    fr fr Build AST structure
    var_node.children.push(value_node)
    block_node.children.push(var_node)
    func_node.children.push(name_node)
    func_node.children.push(block_node)
    program.children.push(func_node)
    
    fr fr Run semantic analysis
    sus analyzer SemanticAnalyzer = run_semantic_analysis(program)
    
    vibez.spill("Test complete")
}

fr fr Main function
slay main() {
    test_semantic_analyzer()
}

main()
