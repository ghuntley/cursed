// CURSED Stage 2 Type Checker
// Semantic analysis and type checking for the CURSED programming language
// Validates AST nodes for type safety and semantic correctness

vibe "cursed::stage2::type_checker";

yeet "std::collections";
yeet "cursed::stage2::parser";
yeet "cursed::stage2::error";

// Type information
enum TypeKind {
    Normie,    // int
    Float,     // float
    Tea,       // string
    Cap,       // bool
    Void,      // void/no return
    Auto,      // auto-inferred
    Unknown,   // unknown/error type
    Function,  // function type
    Struct,    // struct type
    Interface, // interface type
}

squad TypeInfo {
    kind: TypeKind,
    name: tea,
    size: normie,
    is_mutable: cap,
}

// Symbol table entry
squad Symbol {
    name: tea,
    type_info: TypeInfo,
    is_defined: cap,
    scope_level: normie,
}

// Scope for symbol resolution
squad Scope {
    symbols: collections::Map<tea, Symbol>,
    parent: Scope?,
    level: normie,
}

// Type checker state
squad TypeChecker {
    current_scope: Scope,
    global_scope: Scope,
    errors: tea[],
    warnings: tea[],
    function_return_type: TypeInfo?,
}

// Type checking result
squad TypeCheckResult {
    success: cap,
    errors: tea[],
    warnings: tea[],
    symbol_table: collections::Map<tea, Symbol>,
}

// Create a new type checker
slay new_type_checker() -> TypeChecker {
    sus global_scope = Scope {
        symbols: collections::Map<tea, Symbol>(),
        parent: nocap,
        level: 0,
    };
    
    // Add built-in types and functions
    add_builtin_symbols(global_scope);
    
    yolo TypeChecker {
        current_scope: global_scope,
        global_scope: global_scope,
        errors: tea[],
        warnings: tea[],
        function_return_type: nocap,
    };
}

// Add built-in symbols to global scope
slay add_builtin_symbols(scope: Scope) {
    // Built-in types
    scope.symbols.insert("normie", Symbol {
        name: "normie",
        type_info: TypeInfo {
            kind: TypeKind::Normie,
            name: "normie",
            size: 4,
            is_mutable: facts,
        },
        is_defined: truth,
        scope_level: 0,
    });
    
    scope.symbols.insert("tea", Symbol {
        name: "tea",
        type_info: TypeInfo {
            kind: TypeKind::Tea,
            name: "tea",
            size: 8,
            is_mutable: facts,
        },
        is_defined: truth,
        scope_level: 0,
    });
    
    scope.symbols.insert("cap", Symbol {
        name: "cap",
        type_info: TypeInfo {
            kind: TypeKind::Cap,
            name: "cap",
            size: 1,
            is_mutable: facts,
        },
        is_defined: truth,
        scope_level: 0,
    });
    
    scope.symbols.insert("void", Symbol {
        name: "void",
        type_info: TypeInfo {
            kind: TypeKind::Void,
            name: "void",
            size: 0,
            is_mutable: facts,
        },
        is_defined: truth,
        scope_level: 0,
    });
    
    // Built-in functions would go here
    // println, etc.
}

// Enter a new scope
slay enter_scope(checker: TypeChecker) -> Scope {
    sus new_scope = Scope {
        symbols: collections::Map<tea, Symbol>(),
        parent: checker.current_scope,
        level: checker.current_scope.level + 1,
    };
    
    checker.current_scope = new_scope;
    yolo new_scope;
}

// Exit current scope
slay exit_scope(checker: TypeChecker) {
    bestie (checker.current_scope.parent != nocap) {
        checker.current_scope = checker.current_scope.parent;
    }
}

// Look up symbol in current scope chain
slay lookup_symbol(checker: TypeChecker, name: tea) -> Symbol? {
    sus scope = checker.current_scope;
    
    periodt (scope != nocap) {
        bestie (scope.symbols.contains_key(name)) {
            yolo scope.symbols.get(name);
        }
        scope = scope.parent;
    }
    
    yolo nocap;
}

// Add symbol to current scope
slay add_symbol(checker: TypeChecker, symbol: Symbol) -> cap {
    bestie (checker.current_scope.symbols.contains_key(symbol.name)) {
        sus error = "Symbol '" + symbol.name + "' already defined in current scope";
        checker.errors.push(error);
        yolo facts;
    }
    
    checker.current_scope.symbols.insert(symbol.name, symbol);
    yolo truth;
}

// Get type info for type name
slay get_type_info(name: tea) -> TypeInfo {
    vibe_check (name) {
        mood "normie" {
            yolo TypeInfo {
                kind: TypeKind::Normie,
                name: "normie",
                size: 4,
                is_mutable: facts,
            };
        }
        
        mood "tea" {
            yolo TypeInfo {
                kind: TypeKind::Tea,
                name: "tea",
                size: 8,
                is_mutable: facts,
            };
        }
        
        mood "cap" {
            yolo TypeInfo {
                kind: TypeKind::Cap,
                name: "cap",
                size: 1,
                is_mutable: facts,
            };
        }
        
        mood "void" {
            yolo TypeInfo {
                kind: TypeKind::Void,
                name: "void",
                size: 0,
                is_mutable: facts,
            };
        }
        
        mood "auto" {
            yolo TypeInfo {
                kind: TypeKind::Auto,
                name: "auto",
                size: 0,
                is_mutable: facts,
            };
        }
        
        basic {
            yolo TypeInfo {
                kind: TypeKind::Unknown,
                name: name,
                size: 0,
                is_mutable: facts,
            };
        }
    }
}

// Check if types are compatible
slay types_compatible(type1: TypeInfo, type2: TypeInfo) -> cap {
    // Auto type can be assigned from any type
    bestie (type1.kind == TypeKind::Auto || type2.kind == TypeKind::Auto) {
        yolo truth;
    }
    
    // Same types are compatible
    bestie (type1.kind == type2.kind) {
        yolo truth;
    }
    
    // Check for valid conversions
    vibe_check (type1.kind) {
        mood TypeKind::Float {
            // Float can accept int
            yolo type2.kind == TypeKind::Normie;
        }
        
        mood TypeKind::Tea {
            // String can accept most types for conversion
            yolo type2.kind == TypeKind::Normie || 
                 type2.kind == TypeKind::Float ||
                 type2.kind == TypeKind::Cap;
        }
        
        basic {
            yolo facts;
        }
    }
}

// Main type checking entry point
slay check(program: Program) -> TypeCheckResult? {
    sus checker = new_type_checker();
    check_program(checker, program);
    
    yolo TypeCheckResult {
        success: checker.errors.length() == 0,
        errors: checker.errors,
        warnings: checker.warnings,
        symbol_table: checker.global_scope.symbols,
    };
}

// Check program node
slay check_program(checker: TypeChecker, program: Program) {
    lowkey (sus stmt in program.statements) {
        check_statement(checker, stmt);
    }
}

// Check statement
slay check_statement(checker: TypeChecker, stmt: ASTNode) {
    vibe_check (stmt.node_type()) {
        mood NodeType::Function {
            sus func = stmt as FunctionDecl;
            check_function_declaration(checker, func);
        }
        
        mood NodeType::Variable {
            sus var_decl = stmt as VariableDecl;
            check_variable_declaration(checker, var_decl);
        }
        
        mood NodeType::IfStatement {
            sus if_stmt = stmt as IfStatement;
            check_if_statement(checker, if_stmt);
        }
        
        mood NodeType::WhileStatement {
            sus while_stmt = stmt as WhileStatement;
            check_while_statement(checker, while_stmt);
        }
        
        mood NodeType::ReturnStatement {
            sus ret_stmt = stmt as ReturnStatement;
            check_return_statement(checker, ret_stmt);
        }
        
        mood NodeType::Block {
            sus block = stmt as Block;
            check_block(checker, block);
        }
        
        basic {
            check_expression(checker, stmt);
        }
    }
}

// Check function declaration
slay check_function_declaration(checker: TypeChecker, func: FunctionDecl) {
    // Create function symbol
    sus func_type = TypeInfo {
        kind: TypeKind::Function,
        name: func.name,
        size: 8,
        is_mutable: facts,
    };
    
    sus func_symbol = Symbol {
        name: func.name,
        type_info: func_type,
        is_defined: truth,
        scope_level: checker.current_scope.level,
    };
    
    add_symbol(checker, func_symbol);
    
    // Enter function scope
    enter_scope(checker);
    
    // Set function return type
    checker.function_return_type = get_type_info(func.return_type);
    
    // Add parameters to scope
    lowkey (sus param in func.parameters) {
        sus param_type = get_type_info(param.param_type);
        sus param_symbol = Symbol {
            name: param.name,
            type_info: param_type,
            is_defined: truth,
            scope_level: checker.current_scope.level,
        };
        add_symbol(checker, param_symbol);
    }
    
    // Check function body
    check_block(checker, func.body);
    
    // Exit function scope
    checker.function_return_type = nocap;
    exit_scope(checker);
}

// Check variable declaration
slay check_variable_declaration(checker: TypeChecker, var_decl: VariableDecl) {
    // Check the value expression first
    sus value_type = check_expression(checker, var_decl.value);
    
    // Determine variable type
    sus var_type = get_type_info(var_decl.var_type);
    
    // If type is auto, infer from value
    bestie (var_type.kind == TypeKind::Auto) {
        var_type = value_type;
    }
    
    // Check type compatibility
    bestie (!types_compatible(var_type, value_type)) {
        sus error = "Type mismatch in variable '" + var_decl.name + 
                   "': expected " + var_type.name + ", got " + value_type.name;
        checker.errors.push(error);
    }
    
    // Add variable to scope
    sus var_symbol = Symbol {
        name: var_decl.name,
        type_info: var_type,
        is_defined: truth,
        scope_level: checker.current_scope.level,
    };
    
    add_symbol(checker, var_symbol);
}

// Check if statement
slay check_if_statement(checker: TypeChecker, if_stmt: IfStatement) {
    // Check condition is boolean
    sus condition_type = check_expression(checker, if_stmt.condition);
    bestie (condition_type.kind != TypeKind::Cap) {
        sus error = "If condition must be boolean, got " + condition_type.name;
        checker.errors.push(error);
    }
    
    // Check then block
    check_block(checker, if_stmt.then_block);
    
    // Check else block if present
    bestie (if_stmt.else_block != nocap) {
        check_block(checker, if_stmt.else_block);
    }
}

// Check while statement
slay check_while_statement(checker: TypeChecker, while_stmt: WhileStatement) {
    // Check condition is boolean
    sus condition_type = check_expression(checker, while_stmt.condition);
    bestie (condition_type.kind != TypeKind::Cap) {
        sus error = "While condition must be boolean, got " + condition_type.name;
        checker.errors.push(error);
    }
    
    // Check body
    check_block(checker, while_stmt.body);
}

// Check return statement
slay check_return_statement(checker: TypeChecker, ret_stmt: ReturnStatement) {
    bestie (checker.function_return_type == nocap) {
        checker.errors.push("Return statement outside of function");
        yolo;
    }
    
    bestie (ret_stmt.value != nocap) {
        sus return_type = check_expression(checker, ret_stmt.value);
        bestie (!types_compatible(checker.function_return_type, return_type)) {
            sus error = "Return type mismatch: expected " + 
                       checker.function_return_type.name + ", got " + return_type.name;
            checker.errors.push(error);
        }
    } highkey {
        bestie (checker.function_return_type.kind != TypeKind::Void) {
            sus error = "Missing return value for function returning " + 
                       checker.function_return_type.name;
            checker.errors.push(error);
        }
    }
}

// Check block statement
slay check_block(checker: TypeChecker, block: Block) {
    enter_scope(checker);
    
    lowkey (sus stmt in block.statements) {
        check_statement(checker, stmt);
    }
    
    exit_scope(checker);
}

// Check expression and return its type
slay check_expression(checker: TypeChecker, expr: ASTNode) -> TypeInfo {
    vibe_check (expr.node_type()) {
        mood NodeType::Identifier {
            sus ident = expr as Identifier;
            sus symbol = lookup_symbol(checker, ident.name);
            bestie (symbol == nocap) {
                sus error = "Undefined variable: " + ident.name;
                checker.errors.push(error);
                yolo get_type_info("unknown");
            }
            yolo symbol.type_info;
        }
        
        mood NodeType::IntegerLiteral {
            yolo get_type_info("normie");
        }
        
        mood NodeType::FloatLiteral {
            yolo get_type_info("float");
        }
        
        mood NodeType::StringLiteral {
            yolo get_type_info("tea");
        }
        
        mood NodeType::BooleanLiteral {
            yolo get_type_info("cap");
        }
        
        mood NodeType::BinaryExpression {
            sus bin_expr = expr as BinaryExpression;
            yolo check_binary_expression(checker, bin_expr);
        }
        
        basic {
            checker.errors.push("Unknown expression type");
            yolo get_type_info("unknown");
        }
    }
}

// Check binary expression
slay check_binary_expression(checker: TypeChecker, expr: BinaryExpression) -> TypeInfo {
    sus left_type = check_expression(checker, expr.left);
    sus right_type = check_expression(checker, expr.right);
    
    vibe_check (expr.operator) {
        mood "+", "-", "*", "/", "%" {
            // Arithmetic operators
            bestie (left_type.kind == TypeKind::Normie && right_type.kind == TypeKind::Normie) {
                yolo get_type_info("normie");
            } highkey bestie (left_type.kind == TypeKind::Float || right_type.kind == TypeKind::Float) {
                yolo get_type_info("float");
            } highkey bestie (expr.operator == "+" && 
                            (left_type.kind == TypeKind::Tea || right_type.kind == TypeKind::Tea)) {
                // String concatenation
                yolo get_type_info("tea");
            } highkey {
                sus error = "Invalid operands for operator " + expr.operator + 
                           ": " + left_type.name + " and " + right_type.name;
                checker.errors.push(error);
                yolo get_type_info("unknown");
            }
        }
        
        mood "==", "!=" {
            // Equality operators
            bestie (types_compatible(left_type, right_type)) {
                yolo get_type_info("cap");
            } highkey {
                sus error = "Cannot compare incompatible types: " + 
                           left_type.name + " and " + right_type.name;
                checker.errors.push(error);
                yolo get_type_info("cap");
            }
        }
        
        mood "<", "<=", ">", ">=" {
            // Comparison operators
            bestie ((left_type.kind == TypeKind::Normie || left_type.kind == TypeKind::Float) &&
                    (right_type.kind == TypeKind::Normie || right_type.kind == TypeKind::Float)) {
                yolo get_type_info("cap");
            } highkey bestie (left_type.kind == TypeKind::Tea && right_type.kind == TypeKind::Tea) {
                yolo get_type_info("cap");
            } highkey {
                sus error = "Cannot compare types: " + left_type.name + " and " + right_type.name;
                checker.errors.push(error);
                yolo get_type_info("cap");
            }
        }
        
        mood "&&", "||" {
            // Logical operators
            bestie (left_type.kind == TypeKind::Cap && right_type.kind == TypeKind::Cap) {
                yolo get_type_info("cap");
            } highkey {
                sus error = "Logical operators require boolean operands, got " + 
                           left_type.name + " and " + right_type.name;
                checker.errors.push(error);
                yolo get_type_info("cap");
            }
        }
        
        basic {
            sus error = "Unknown binary operator: " + expr.operator;
            checker.errors.push(error);
            yolo get_type_info("unknown");
        }
    }
}
