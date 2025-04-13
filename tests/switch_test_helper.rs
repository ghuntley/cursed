use cursed::ast::control_flow::{CaseStatement, SwitchCase};
use cursed::ast::statements::block::BlockStatement;
use cursed::ast::{Expression, Statement};

/// Helper function to convert a CaseStatement to a SwitchCase
pub fn convert_to_switch_case(case: CaseStatement) -> SwitchCase {
    // For simple cases with a single expression
    // We can't clone a Box<dyn Expression> directly, so we need to create a new StringLiteral
    // as a temporary workaround for this test
    let value = if !case.expressions.is_empty() {
        // Use the first expression's string representation to create a new StringLiteral
        Box::new(cursed::ast::expressions::StringLiteral {
            token: "\"string\"".to_string(),
            value: case.expressions[0].string(),
        }) as Box<dyn Expression>
    } else {
        // Provide a fallback value if there are no expressions
        panic!("CaseStatement must have at least one expression")
    };
    
    // Convert BlockStatement to Vec<Box<dyn Statement>>
    let statements = case.body.statements;
    
    SwitchCase {
        value,
        statements,
    }
}

/// Helper function to convert a BlockStatement to a SwitchCase with a default value
pub fn convert_block_to_default_case(block: BlockStatement, default_expr: Box<dyn Expression>) -> SwitchCase {
    SwitchCase {
        value: default_expr,
        statements: block.statements,
    }
}