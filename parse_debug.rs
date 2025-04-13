use std::fs::File;
use std::io::Read;

use cursed::ast::expressions::DotExpression;
use cursed::ast::traits::{Node, Expression, Statement};
use cursed::lexer::Lexer;
use cursed::parser::Parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = args.get(1).unwrap_or(&"examples/simplest.csd".to_string());
    
    // Read file
    let mut file = File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    
    // Create lexer
    let mut lexer = Lexer::new(&contents);
    
    // Create parser
    let mut parser = Parser::new(&mut lexer).expect("Failed to create parser");
    
    // Parse the program
    let program = parser.parse_program().expect("Failed to parse program");
    
    // Print any parser errors
    if !parser.errors().is_empty() {
        println!("Parser errors:");
        for err in parser.errors() {
            println!("{}", err);
        }
        return;
    }
    
    println!("Parsed program with {} statements", program.statements.len());
    
    // Examine each statement in detail
    for (i, stmt) in program.statements.iter().enumerate() {
        println!("\n=== Statement {} ===\n", i);
        println!("Type: {}", type_name_of_val(&**stmt));
        println!("String representation: {}", stmt.string());
        
        // Check if it's an expression statement
        if let Some(expr_stmt) = stmt.as_any().downcast_ref::<cursed::ast::statements::ExpressionStatement>() {
            if let Some(expr) = &expr_stmt.expression {
                println!("\n  Expression type: {}", type_name_of_val(&**expr));
                println!("  Expression string: {}", expr.string());
                
                // Check if it's a dot expression
                if let Some(dot_expr) = expr.as_any().downcast_ref::<DotExpression>() {
                    println!("\n    Found DotExpression!");
                    println!("    Object: {} (type: {})", dot_expr.object.string(), 
                             type_name_of_val(&*dot_expr.object));
                    println!("    Property: {}", dot_expr.property);
                }
                
                // Check nested objects using recursion
                examine_expression(&**expr, 1);
            }
        }
    }
}

fn examine_expression(expr: &dyn Expression, depth: usize) {
    let indent = "  ".repeat(depth + 1);
    let expr_type = type_name_of_val(expr);
    println!("{indent}Examining expression of type: {expr_type}");
    
    // Check if it's a call expression
    if let Some(call_expr) = expr.as_any().downcast_ref::<cursed::ast::expressions::CallExpression>() {
        println!("{indent}Found CallExpression!");
        println!("{indent}Function: {} (type: {})", call_expr.function.string(), 
                 type_name_of_val(&*call_expr.function));
        
        // Examine the function expression
        examine_expression(&*call_expr.function, depth + 1);
    }
    
    // Check if it's a dot expression
    if let Some(dot_expr) = expr.as_any().downcast_ref::<DotExpression>() {
        println!("{indent}Found DotExpression!");
        println!("{indent}Object: {} (type: {})", dot_expr.object.string(), 
                 type_name_of_val(&*dot_expr.object));
        println!("{indent}Property: {}", dot_expr.property);
        
        // Examine the object expression
        examine_expression(&*dot_expr.object, depth + 1);
    }
}

fn type_name_of_val<T: ?Sized>(_val: &T) -> &'static str {
    std::any::type_name::<T>()
}