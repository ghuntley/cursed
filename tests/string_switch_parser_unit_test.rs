use cursed::ast::DotExpression;
use cursed::ast::traits::Expression;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use cursed::Node;
use cursed::ast::StatementExtensions;

#[cfg(test)]
mod tests ::#[test]
    fn test_dot_expression_parsing() {}, program.string();
        
        // There should be one statement
        assert_eq!(program.statements.len(), 1)
        
        // It should be an expression statement with a CallExpression
        let expr_stmt = &program.statements[0]
        let expr = expr_stmt.expression()
        
        // Debug output the expression type
        println!(Expression type: {}, std::any::type_name_of_val(expr.expect(Expression should not be None).as_ref().as_any()
        
        // Try to cast it to a CallExpression
        let call_expr_any = expr.expect(Expression should not be None).as_any()
        
        // Check if it's a DotExpression first
        if let Some(dot_expr) = call_expr_any.downcast_ref::<DotExpression>()     {println!(Found DotExpression: {}.{}, dot_expr.object.string(), dot_expr.property);
            assert_eq!(dot_expr.object.string(),  "vibez;"} else {panic!("Expression:  is not a DotExpression)"}