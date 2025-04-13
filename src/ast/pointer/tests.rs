use super::*;
use crate::ast::expressions::Identifier;
use crate::ast::traits::{Expression, Node};
#[cfg(test)]
use crate::lexer::token::Token;

#[test]
fn test_pointer_type() {
    let target_type = Box::new(Identifier {
        token: "normie".to_string(),
        value: "normie".to_string(),
    }) as Box<dyn Expression>;

    let pointer_type = PointerType {
        token: "@".to_string(),
        target_type,
    };

    assert_eq!(pointer_type.string(), "@normie");
}

#[test]
fn test_pointer_dereference() {
    let pointer = Box::new(Identifier {
        token: "ptr".to_string(),
        value: "ptr".to_string(),
    }) as Box<dyn Expression>;

    let dereference = PointerDereference {
        token: "@".to_string(),
        pointer,
    };

    assert_eq!(dereference.string(), "@ptr");
}
