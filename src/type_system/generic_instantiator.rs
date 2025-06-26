//! Generic type instantiation for CURSED

use crate::error::CursedError;
use crate::core::Type;
use std::collections::HashMap;

#[derive(Debug)]
pub struct GenericInstantiator {
    type_parameters: HashMap<String, Type>,
    instantiations: Vec<TypeInstantiation>,
}

#[derive(Debug, Clone)]
pub struct TypeInstantiation {
    pub generic_type: String,
    pub concrete_type: Type,
    pub context: String,
}

impl GenericInstantiator {
    pub fn new() -> Self {
        Self {
            type_parameters: HashMap::new(),
            instantiations: Vec::new(),
        }
    }

    pub fn add_type_parameter(&mut self, name: String, bound: Type) {
        self.type_parameters.insert(name, bound);
    }

    pub fn instantiate(&mut self, generic_type: String, concrete_type: Type, context: String) -> Result<Type, CursedError> {
        let instantiation = TypeInstantiation {
            generic_type: generic_type.clone(),
            concrete_type: concrete_type.clone(),
            context,
        };
        
        self.instantiations.push(instantiation);
        Ok(concrete_type)
    }

    pub fn get_instantiations(&self) -> &[TypeInstantiation] {
        &self.instantiations
    }
}

impl Default for GenericInstantiator {
    fn default() -> Self {
        Self::new()
    }
}
