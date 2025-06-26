//! Type inference engine for CURSED

use crate::error::CursedError;
use crate::core::Type;
use std::collections::HashMap;

#[derive(Debug)]
pub struct TypeInference {
    type_constraints: Vec<TypeConstraint>,
    type_variables: HashMap<String, Type>,
    substitutions: HashMap<String, Type>,
}

#[derive(Debug, Clone)]
pub struct TypeConstraint {
    pub left: Type,
    pub right: Type,
    pub location: String,
}

impl TypeInference {
    pub fn new() -> Self {
        Self {
            type_constraints: Vec::new(),
            type_variables: HashMap::new(),
            substitutions: HashMap::new(),
        }
    }

    pub fn add_constraint(&mut self, left: Type, right: Type, location: String) {
        self.type_constraints.push(TypeConstraint { left, right, location });
    }

    pub fn infer_types(&mut self) -> Result<HashMap<String, Type>, CursedError> {
        // Simplified type inference algorithm
        let constraints = self.type_constraints.clone();
        for constraint in constraints.iter() {
            self.unify(&constraint.left, &constraint.right)?;
        }
        Ok(self.substitutions.clone())
    }

    fn unify(&mut self, left: &Type, right: &Type) -> Result<(), CursedError> {
        match (left, right) {
            (Type::Unknown, t) | (t, Type::Unknown) => {
                // Handle type variables
                Ok(())
            },
            (Type::Int, Type::Int) => Ok(()),
            (Type::Float, Type::Float) => Ok(()),
            (Type::String, Type::String) => Ok(()),
            (Type::Bool, Type::Bool) => Ok(()),
            _ => Err(CursedError::type_error(&format!("Cannot unify {:?} with {:?}", left, right))),
        }
    }
}

impl Default for TypeInference {
    fn default() -> Self {
        Self::new()
    }
}
