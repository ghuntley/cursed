use crate::ast;
use crate::ast::control_flow::conditionals::IfStatement;
use crate::ast::expressions::{ArrayLiteral, Identifier, StringLiteral};
use crate::ast::statements::{
    block::BlockStatement,
    declarations::{LetStatement, ReturnStatement},
    fields,
};
use crate::ast::{Expression, Node, Statement};
use crate::core::type_checker::Type;
use crate::error::Error;
use std::collections::HashMap;

/// Implements generic type instantiation for CURSED
pub struct GenericInstantiator {
    // Maps type parameter names to concrete types
    type_map: HashMap<String, Type>,
}

impl GenericInstantiator {
    /// Create a new generic instantiator
    pub fn new() -> Self {
        GenericInstantiator {
            type_map: HashMap::new(),
        }
    }

    /// Add a type parameter mapping
    pub fn add_type_param(&mut self, param_name: &str, concrete_type: Type) {
        self.type_map.insert(param_name.to_string(), concrete_type);
    }

    /// Instantiate a type with concrete type arguments
    pub fn instantiate_type(&self, generic_type: &Type) -> Result<Type, Error> {
        // Use the enhanced nested generic instantiation functionality for better handling of complex nested types
        use crate::core::nested_generic_instantiation::NestedGenericSubstitution;
        
        // Convert our type map to the format expected by NestedGenericSubstitution
        let type_param_map = self.type_map.clone();
        
        // Call the enhanced implementation with a reasonable depth limit
        generic_type.substitute_nested_type_parameters(&type_param_map, 32)
    }

    // Original monomorphization functions are commented out for now as they have various issues
    // We'll implement them properly in a future update

    /// Generate a monomorphized version of a generic function
    pub fn monomorphize_function(
        &self,
        generic_function: &ast::FunctionStatement,
        type_args: &[Type],
    ) -> Result<ast::FunctionStatement, Error> {
        // Verify that the number of type arguments matches the function's type parameters
        if type_args.len() != generic_function.type_parameters.len() {
            return Err(Error::from_str(&format!(
                "Type argument count mismatch: expected {}, got {}",
                generic_function.type_parameters.len(),
                type_args.len()
            )));
        }

        // Create a new instantiator with the provided type arguments
        let mut instantiator = GenericInstantiator::new();

        // Map each type parameter to its concrete type
        for (i, param) in generic_function.type_parameters.iter().enumerate() {
            instantiator.add_type_param(&param.value, type_args[i].clone());
        }

        // Generate a specialized name that includes concrete type information
        let specialized_name = format!(
            "{}{}",
            generic_function.name.value,
            type_args
                .iter()
                .map(|t| format!("{}", t.to_string()))
                .collect::<Vec<_>>()
                .join("_")
        );

        // Create a specialized identifier for the function name
        let specialized_ident = Identifier {
            token: "IDENT".to_string(),
            value: specialized_name,
        };

        // Create new parameters with specialized types
        let mut specialized_params = Vec::new();
        for param in &generic_function.parameters {
            // Monomorphize the parameter type
            let param_type = self
                .extract_type_from_expression(&param.param_type)
                .unwrap_or(Type::Unknown);

            // Apply type substitutions for generic parameters
            let concrete_param_type = instantiator.instantiate_type(&param_type)?;

            // Create a new expression for the specialized type
            let concrete_type_expr = self.create_expression_from_type(&concrete_param_type)?;

            // Create the specialized parameter
            let specialized_param = ast::Parameter {
                token: param.token.clone(),
                name: param.name.clone(),
                param_type: concrete_type_expr,
            };

            specialized_params.push(specialized_param);
        }

        // Process return type if it exists
        let specialized_return_type = if let Some(ret_type_expr) = &generic_function.return_type {
            // Extract and instantiate the return type
            let ret_type = self
                .extract_type_from_expression(ret_type_expr)
                .unwrap_or(Type::Unknown);

            // Apply type substitutions
            let concrete_ret_type = instantiator.instantiate_type(&ret_type)?;

            // Create a new expression for the specialized return type
            let concrete_ret_expr = self.create_expression_from_type(&concrete_ret_type)?;

            Some(concrete_ret_expr)
        } else {
            None
        };

        // Process the function body to replace generic types
        let specialized_body =
            self.monomorphize_block_statement(&generic_function.body, &instantiator)?;

        // Create a new function with the specialized components
        let specialized_func = ast::FunctionStatement {
            token: generic_function.token.clone(),
            name: specialized_ident,
            parameters: specialized_params,
            body: specialized_body,
            return_type: specialized_return_type,
            type_parameters: Vec::new(), // No type parameters in specialized version
            generic_constraints: Vec::new(), // No constraints in specialized version
            doc: generic_function.doc.clone(), // Preserve documentation
        };

        Ok(specialized_func)
    }

    /// Transforms a block statement by specializing all contained statements
    fn monomorphize_block_statement(
        &self,
        block: &BlockStatement,
        instantiator: &GenericInstantiator,
    ) -> Result<BlockStatement, Error> {
        // Create a new block statement
        let mut specialized_block = BlockStatement {
            token: block.token.clone(),
            statements: Vec::new(),
        };

        // Process each statement in the block
        for statement in &block.statements {
            // Different statement types need different processing
            if let Some(let_stmt) = statement.as_any().downcast_ref::<LetStatement>() {
                // Handle variable declarations
                let specialized_let_stmt =
                    self.monomorphize_let_statement(let_stmt, instantiator)?;
                specialized_block
                    .statements
                    .push(Box::new(specialized_let_stmt));
            } else if let Some(return_stmt) = statement.as_any().downcast_ref::<ReturnStatement>() {
                // Handle return statements
                let specialized_return_stmt =
                    self.monomorphize_return_statement(return_stmt, instantiator)?;
                specialized_block
                    .statements
                    .push(Box::new(specialized_return_stmt));
            } else if let Some(if_stmt) = statement.as_any().downcast_ref::<IfStatement>() {
                // Handle if statements
                let specialized_if_stmt = self.monomorphize_if_statement(if_stmt, instantiator)?;
                specialized_block
                    .statements
                    .push(Box::new(specialized_if_stmt));
            } else if let Some(block_stmt) = statement.as_any().downcast_ref::<BlockStatement>() {
                // Handle nested blocks
                let specialized_block_stmt =
                    self.monomorphize_block_statement(block_stmt, instantiator)?;
                specialized_block
                    .statements
                    .push(Box::new(specialized_block_stmt));
            } else {
                // For other statement types, we need to create a new object
                // We can't directly clone the trait object, so we need to create a
                // new statement that has the same behavior
                let mut statement_clone: Option<Box<dyn Statement>> = None;

                // Check for various statement types
                // This approach is a bit verbose but necessary because we can't clone trait objects directly
                if let Some(expr_stmt) = statement
                    .as_any()
                    .downcast_ref::<ast::statements::ExpressionStatement>()
                {
                    // Create a new expression statement with processed expression
                    let processed_expr = if let Some(expr) = &expr_stmt.expression {
                        // Process the expression
                        Some(self.monomorphize_expression(expr, instantiator)?)
                    } else {
                        None
                    };

                    let new_expr_stmt = ast::statements::ExpressionStatement {
                        token: expr_stmt.token.clone(),
                        expression: processed_expr,
                    };
                    statement_clone = Some(Box::new(new_expr_stmt));
                }
                // Add more statement types here as needed

                // If we couldn't handle this statement type, log a warning and skip it
                if let Some(stmt) = statement_clone {
                    specialized_block.statements.push(stmt);
                } else {
                    // In a real implementation, we would add proper error handling here
                    println!("Warning: Skipping unhandled statement type in monomorphization");
                }
            }
        }

        Ok(specialized_block)
    }

    /// Transforms a let statement by specializing its type and value
    fn monomorphize_let_statement(
        &self,
        let_stmt: &LetStatement,
        instantiator: &GenericInstantiator,
    ) -> Result<LetStatement, Error> {
        // Extract the type annotation if present
        let specialized_type_annotation = let_stmt.type_annotation.clone();

        // If the variable has a value, transform it to replace any generic types
        let specialized_value = if let Some(value_expr) = &let_stmt.value {
            // In a full implementation, we would replace generic types in the expression
            // For now, we're just copying the expression as-is
            Some(self.monomorphize_expression(value_expr, instantiator)?)
        } else {
            None
        };

        // Create a new let statement with the specialized components
        Ok(LetStatement {
            token: let_stmt.token.clone(),
            name: let_stmt.name.clone(),
            value: specialized_value,
            type_annotation: specialized_type_annotation,
        })
    }

    /// Helper method to monomorphize an expression by replacing generic types with concrete types
    fn monomorphize_expression(
        &self,
        expr: &Box<dyn Expression>,
        instantiator: &GenericInstantiator,
    ) -> Result<Box<dyn Expression>, Error> {
        // Handle different expression types
        if let Some(identifier) = expr.as_any().downcast_ref::<Identifier>() {
            // If this is a generic type parameter, replace it with the concrete type
            if let Some(concrete_type) = instantiator.type_map.get(&identifier.value) {
                // Create an identifier with the concrete type name
                return Ok(Box::new(Identifier {
                    token: identifier.token.clone(),
                    value: concrete_type.to_string(),
                }));
            }
            // Otherwise, just clone the identifier
            return Ok(Box::new(Identifier {
                token: identifier.token.clone(),
                value: identifier.value.clone(),
            }));
        } else if let Some(string_literal) = expr.as_any().downcast_ref::<StringLiteral>() {
            // Handle string literals
            return Ok(Box::new(StringLiteral {
                token: string_literal.token.clone(),
                value: string_literal.value.clone(),
            }));
        } else if let Some(array_literal) = expr.as_any().downcast_ref::<ArrayLiteral>() {
            // Handle array literals - this would need more work to process each element
            // For now, just create a new array literal with the same properties
            let new_array = ArrayLiteral {
                token: array_literal.token.clone(),
                elements: Vec::new(), // In a full implementation, we'd process each element
            };
            return Ok(Box::new(new_array));
        }

        // For expressions we don't handle, log a warning and return a placeholder
        println!("Warning: Unhandled expression type in monomorphization");

        // Return a placeholder identifier representing the original expression
        Ok(Box::new(Identifier {
            token: "UNKNOWN_EXPR".to_string(),
            value: "unknown_expression".to_string(),
        }))
    }

    /// Transforms an expression statement by specializing its expression
    fn monomorphize_expression_statement(
        &self,
        expr_stmt: &ast::statements::ExpressionStatement,
        instantiator: &GenericInstantiator,
    ) -> Result<ast::statements::ExpressionStatement, Error> {
        // Create a new expression statement with a transformed expression
        let specialized_expr = if let Some(expr) = &expr_stmt.expression {
            // Transform the expression to replace any generic types
            Some(self.monomorphize_expression(expr, instantiator)?)
        } else {
            None
        };

        // Create a new expression statement with the specialized expression
        Ok(ast::statements::ExpressionStatement {
            token: expr_stmt.token.clone(),
            expression: specialized_expr,
        })
    }

    /// Transforms a return statement by specializing its return value
    fn monomorphize_return_statement(
        &self,
        return_stmt: &ReturnStatement,
        instantiator: &GenericInstantiator,
    ) -> Result<ReturnStatement, Error> {
        // Transform the return value expression if present
        let specialized_return_value = if let Some(value) = &return_stmt.return_value {
            // Transform the expression to replace any generic types
            Some(self.monomorphize_expression(value, instantiator)?)
        } else {
            None
        };

        // Create a new return statement with the specialized return value
        Ok(ReturnStatement {
            token: return_stmt.token.clone(),
            return_value: specialized_return_value,
        })
    }

    /// Transforms an if statement by specializing its condition and blocks
    fn monomorphize_if_statement(
        &self,
        if_stmt: &IfStatement,
        instantiator: &GenericInstantiator,
    ) -> Result<IfStatement, Error> {
        // Transform the condition expression
        let specialized_condition =
            self.monomorphize_expression(&if_stmt.condition, instantiator)?;

        // Process the consequence block
        let specialized_consequence =
            self.monomorphize_block_statement(&if_stmt.consequence, instantiator)?;

        // Process the alternative if present
        let specialized_alternative = if let Some(alt) = &if_stmt.alternative {
            // Process the alternative block
            let specialized_alt = self.monomorphize_block_statement(alt, instantiator)?;
            Some(Box::new(specialized_alt))
        } else {
            None
        };

        // Create the specialized if statement
        Ok(IfStatement {
            token: if_stmt.token.clone(),
            condition: specialized_condition,
            consequence: Box::new(specialized_consequence),
            alternative: specialized_alternative,
        })
    }

    /// Helper method to extract a Type from an Expression
    fn extract_type_from_expression(&self, expr: &Box<dyn Expression>) -> Option<Type> {
        // Try to extract the type from various expression types
        if let Some(identifier) = expr.as_any().downcast_ref::<Identifier>() {
            // Convert an identifier to a basic type
            return Some(Type::new_basic(&identifier.value));
        }

        // For other expressions, such as array literals, slice literals, etc.
        // we would need a more comprehensive implementation
        // For now, return Unknown for expressions we don't specifically handle
        Some(Type::Unknown)
    }

    /// Helper method to create an Expression from a Type
    fn create_expression_from_type(&self, typ: &Type) -> Result<Box<dyn Expression>, Error> {
        // Convert the type to a string representation
        let type_str = typ.to_string();

        // Create an appropriate expression based on the type
        match typ {
            Type::Array(elem_type, size) => {
                // In a complete implementation, we'd create an array type expression
                // For now, use a simple identifier with the type name
                let identifier = Identifier {
                    token: "IDENT".to_string(),
                    value: format!("[{}]{}", size, elem_type.to_string()),
                };
                Ok(Box::new(identifier))
            }
            Type::Slice(elem_type) => {
                // In a complete implementation, we'd create a slice type expression
                // For now, use a simple identifier with the type name
                let identifier = Identifier {
                    token: "IDENT".to_string(),
                    value: format!("][]{}", elem_type.to_string()),
                };
                Ok(Box::new(identifier))
            }
            Type::Map(key_type, value_type) => {
                // In a complete implementation, we'd create a map type expression
                // For now, use a simple identifier with the type name
                let identifier = Identifier {
                    token: "IDENT".to_string(),
                    value: format!("tea[{}]{}", key_type.to_string(), value_type.to_string()),
                };
                Ok(Box::new(identifier))
            }
            // For other types, create a simple identifier
            _ => {
                let identifier = Identifier {
                    token: "IDENT".to_string(),
                    value: type_str,
                };
                Ok(Box::new(identifier))
            }
        }
    }

    /// Generate a specialized version of a generic struct
    ///
    /// This method creates a monomorphized version of a generic struct by substituting
    /// concrete types for type parameters. It transforms the AST, replacing type
    /// parameters in field types with concrete types.
    ///
    /// # Arguments
    ///
    /// * `generic_struct` - The generic struct to monomorphize
    /// * `type_args` - The concrete types to substitute for type parameters
    ///
    /// # Returns
    ///
    /// A specialized version of the struct with concrete types
    pub fn monomorphize_struct(
        &self,
        generic_struct: &ast::SquadStatement,
        type_args: &[Type],
    ) -> Result<ast::SquadStatement, Error> {
        // Verify that the number of type arguments matches the struct's type parameters
        if type_args.len() != generic_struct.type_parameters.len() {
            return Err(Error::from_str(&format!(
                "Type argument count mismatch: expected {}, got {}",
                generic_struct.type_parameters.len(),
                type_args.len()
            )));
        }

        // Create a new instantiator with the provided type arguments
        let mut instantiator = GenericInstantiator::new();

        // Map each type parameter to its concrete type
        for (i, param) in generic_struct.type_parameters.iter().enumerate() {
            instantiator.add_type_param(&param.value, type_args[i].clone());
        }

        // For this implementation, create a specialized struct name with type parameters
        let specialized_name = format!(
            "{}{}",
            generic_struct.name.value,
            type_args
                .iter()
                .map(|t| format!("{}", t.to_string()))
                .collect::<Vec<_>>()
                .join("_")
        );

        // Create a specialized identifier for the struct name
        let specialized_ident = ast::expressions::Identifier {
            token: "IDENT".to_string(),
            value: specialized_name,
        };

        // Process all fields with concrete types
        let mut specialized_fields = Vec::new();
        for field in &generic_struct.fields {
            // Get the field's type name
            let field_type_name = field.type_name.string();
            
            // Check if this type name refers to a type parameter
            let field_type = if let Some(concrete_type) = self.type_map.get(&field_type_name) {
                // This is a direct type parameter reference (e.g., "T")
                concrete_type.clone()
            } else {
                // This could be a more complex type like "List[T]" or a primitive type
                // Parse the field type based on the name
                self.parse_type_name_with_params(&field_type_name)?
            };
            
            // Apply any remaining type parameter substitution for nested generics
            let concrete_field_type = self.instantiate_type(&field_type)?;
            
            // Create a new type expression for the concrete type
            let concrete_type_name = match &concrete_field_type {
                Type::Normie => "normie".to_string(),
                Type::Smol => "smol".to_string(),
                Type::Mid => "mid".to_string(),
                Type::Thicc => "thicc".to_string(),
                Type::Snack => "snack".to_string(),
                Type::Meal => "meal".to_string(),
                Type::Tea => "tea".to_string(),
                Type::Lit => "lit".to_string(),
                Type::Byte => "byte".to_string(),
                Type::Rune => "rune".to_string(),
                Type::Sip => "sip".to_string(),
                Type::Extra => "extra".to_string(),
                _ => concrete_field_type.to_string(),
            };

            let concrete_type_expr = ast::expressions::Identifier {
                token: "IDENT".to_string(),
                value: concrete_type_name,
            };
            
            // Create a specialized field using the ast::statements::fields::FieldStatement type
            // To avoid direct dependencies, we need to create the field statement through
            // the appropriate APIs
            let specialized_field = ast::statements::fields::FieldStatement {
                token: field.token.clone(),
                name: field.name.clone(),
                type_name: concrete_type_expr,
            };
            
            specialized_fields.push(specialized_field);
        }

        // Create a new struct with a specialized name, no type parameters, and concrete field types
        let specialized_struct = ast::SquadStatement {
            token: generic_struct.token.clone(),
            name: specialized_ident,
            type_parameters: Vec::new(), // No type parameters in specialized version
            generic_constraints: Vec::new(), // No constraints in specialized version
            fields: specialized_fields,  // Fields with concrete types
            doc: generic_struct.doc.clone(), // Preserve documentation
        };

        Ok(specialized_struct)
    }
    
    /// Parse a type from its string representation
    fn parse_type_from_name(&self, type_name: &str) -> Type {
        // Handle primitive types
        match type_name {
            "normie" => Type::Normie,
            "smol" => Type::Smol,
            "mid" => Type::Mid,
            "thicc" => Type::Thicc,
            "snack" => Type::Snack,
            "meal" => Type::Meal,
            "tea" => Type::Tea,
            "lit" => Type::Lit,
            "byte" => Type::Byte,
            "rune" | "sip" => Type::Rune,
            "extra" => Type::Extra,
            _ => {
                // Check if this is a type parameter in our map
                if let Some(concrete_type) = self.type_map.get(type_name) {
                    concrete_type.clone()
                } else {
                    // Handle other complex types or default to Named
                    Type::Named(type_name.to_string())
                }
            }
        }
    }
    
    /// Parse a complex type name that might contain type parameters
    /// For example: "List[T]", "Map[K, V]", etc.
    fn parse_type_name_with_params(&self, type_name: &str) -> Result<Type, Error> {
        // Check if this is a primitive type first
        let primitive = self.parse_type_from_name(type_name);
        if !matches!(primitive, Type::Named(_)) {
            // This is a primitive type, return it directly
            return Ok(primitive);
        }
        
        // Check if this has generic parameters [T, U, ...]
        if let Some(bracket_idx) = type_name.find('[') {
            if let Some(end_bracket_idx) = type_name.rfind(']') {
                // Extract the base type name and the type parameters
                let base_name = &type_name[0..bracket_idx];
                let params_str = &type_name[bracket_idx+1..end_bracket_idx];
                
                // Split the parameters string by commas
                let param_names: Vec<&str> = params_str.split(',').map(|s| s.trim()).collect();
                
                // Parse each parameter
                let mut type_params = Vec::new();
                for param_name in param_names {
                    // First check if it's a direct type parameter
                    if let Some(concrete_type) = self.type_map.get(param_name) {
                        type_params.push(Box::new(concrete_type.clone()));
                    } else {
                        // Otherwise, recursively parse it as a potentially nested type
                        let param_type = self.parse_type_name_with_params(param_name)?;
                        type_params.push(Box::new(param_type));
                    }
                }
                
                // Create a struct or interface type based on the base name
                return Ok(Type::Struct(base_name.to_string(), type_params));
            }
        }
        
        // If no special format, just return a named type
        Ok(Type::Named(type_name.to_string()))
    }

    /// Generate a specialized version of a generic interface
    pub fn monomorphize_interface(
        &self,
        generic_interface: &ast::CollabStatement,
        type_args: &[Type],
    ) -> Result<ast::CollabStatement, Error> {
        // Verify that the number of type arguments matches the interface's type parameters
        if type_args.len() != generic_interface.type_parameters.len() {
            return Err(Error::from_str(&format!(
                "Type argument count mismatch: expected {}, got {}",
                generic_interface.type_parameters.len(),
                type_args.len()
            )));
        }

        // Create a new instantiator with the provided type arguments
        let mut instantiator = GenericInstantiator::new();

        // Map each type parameter to its concrete type
        for (i, param) in generic_interface.type_parameters.iter().enumerate() {
            instantiator.add_type_param(&param.value, type_args[i].clone());
        }

        // For this implementation, create a specialized interface name with type parameters
        let specialized_name = format!(
            "{}{}",
            generic_interface.name.value,
            type_args
                .iter()
                .map(|t| format!("{}", t.to_string()))
                .collect::<Vec<_>>()
                .join("_")
        );

        // Create a specialized identifier for the interface name
        let specialized_ident = ast::expressions::Identifier {
            token: "IDENT".to_string(),
            value: specialized_name,
        };

        // Create a new interface with specialized name and no type parameters
        let specialized_interface = ast::CollabStatement {
            token: generic_interface.token.clone(),
            name: specialized_ident,
            type_parameters: Vec::new(), // No type parameters in specialized version
            generic_constraints: Vec::new(), // No constraints in specialized version
            methods: Vec::new(), // In a real implementation, we would process methods with concrete types
            doc: generic_interface.doc.clone(), // Preserve documentation
        };

        // Note: In a complete implementation, we would process each method
        // and transform its parameter types and return type with concrete types
        // for any type parameters used

        Ok(specialized_interface)
    }

    /// Generate a specialized version of an entire program AST by replacing generic types
    pub fn generate_instantiation(
        &self,
        generic_ast: &crate::ast::base::Program,
        type_map: &HashMap<String, Type>,
    ) -> Result<crate::ast::base::Program, Error> {
        // Create a new instantiator with the provided type map
        let mut instantiator = GenericInstantiator::new();

        // Add all type mappings
        for (param_name, concrete_type) in type_map {
            instantiator.add_type_param(param_name, concrete_type.clone());
        }

        // Create a new program for the specialized version
        let specialized_program = crate::ast::base::Program::default();

        // For this implementation, we'll simply return an empty program with a comment
        // In a real implementation, we would process each statement and generate
        // specialized versions based on the type arguments

        // Note: A complete implementation would:
        // 1. Examine each statement in the original program
        // 2. Identify generic functions, structs, and interfaces
        // 3. Create specialized versions with concrete types
        // 4. Add non-generic statements as-is
        // 5. Handle type-dependent expressions within statements

        Ok(specialized_program)
    }

    // Helper methods for converting between Type and Expression would go here
    // In a full implementation, these would handle the conversion between the AST's
    // expression types and the type system's Type enum

    /// Converts an Expression to a Type by analyzing the expression
    pub fn expression_to_type(&self, expr: &dyn crate::ast::Expression) -> Result<Type, Error> {
        // Handle different expression types
        if let Some(identifier) = expr.as_any().downcast_ref::<crate::ast::expressions::Identifier>() {
            // Convert identifier to type
            return Ok(Type::new_basic(&identifier.value));
        } else if let Some(string_literal) = expr.as_any().downcast_ref::<crate::ast::expressions::StringLiteral>() {
            // String literals represent the tea type
            return Ok(Type::Tea);
        } else if let Some(int_literal) = expr.as_any().downcast_ref::<crate::ast::expressions::IntegerLiteral>() {
            // Integer literals default to normie
            return Ok(Type::Normie);
        } else if let Some(float_literal) = expr.as_any().downcast_ref::<crate::ast::expressions::FloatLiteral>() {
            // Float literals default to snack
            return Ok(Type::Snack);
        } else if let Some(bool_literal) = expr.as_any().downcast_ref::<crate::ast::expressions::BooleanLiteral>() {
            // Boolean literals are lit type
            return Ok(Type::Lit);
        } else if let Some(array_literal) = expr.as_any().downcast_ref::<crate::ast::expressions::ArrayLiteral>() {
            // For array literals, try to determine element type from first element
            if !array_literal.elements.is_empty() {
                if let Some(first_elem) = array_literal.elements.first() {
                    let elem_type = self.expression_to_type(first_elem.as_ref())?;
                    return Ok(Type::Array(Box::new(elem_type), array_literal.elements.len()));
                }
            }
            // Default to unknown element type if array is empty
            return Ok(Type::Array(Box::new(Type::Unknown), 0));
        }
        
        // For other expression types, we need more context or default to Unknown
        Ok(Type::Unknown)
    }

    /// Converts a Type to an Expression by creating the appropriate expression
    /// that represents the given type
    pub fn type_to_expression(&self, typ: &Type) -> Result<Box<dyn crate::ast::Expression>, Error> {
        // Convert the type to the appropriate expression
        match typ {
            Type::Lit => {
                // Create an identifier for lit (boolean) type
                Ok(Box::new(crate::ast::expressions::Identifier {
                    token: "IDENT".to_string(),
                    value: "lit".to_string(),
                }))
            }
            Type::Smol => {
                Ok(Box::new(crate::ast::expressions::Identifier {
                    token: "IDENT".to_string(),
                    value: "smol".to_string(),
                }))
            }
            Type::Mid => {
                Ok(Box::new(crate::ast::expressions::Identifier {
                    token: "IDENT".to_string(),
                    value: "mid".to_string(),
                }))
            }
            Type::Normie => {
                Ok(Box::new(crate::ast::expressions::Identifier {
                    token: "IDENT".to_string(),
                    value: "normie".to_string(),
                }))
            }
            Type::Thicc => {
                Ok(Box::new(crate::ast::expressions::Identifier {
                    token: "IDENT".to_string(),
                    value: "thicc".to_string(),
                }))
            }
            Type::Snack => {
                Ok(Box::new(crate::ast::expressions::Identifier {
                    token: "IDENT".to_string(),
                    value: "snack".to_string(),
                }))
            }
            Type::Meal => {
                Ok(Box::new(crate::ast::expressions::Identifier {
                    token: "IDENT".to_string(),
                    value: "meal".to_string(),
                }))
            }
            Type::Tea => {
                Ok(Box::new(crate::ast::expressions::Identifier {
                    token: "IDENT".to_string(),
                    value: "tea".to_string(),
                }))
            }
            Type::Sip => {
                Ok(Box::new(crate::ast::expressions::Identifier {
                    token: "IDENT".to_string(),
                    value: "sip".to_string(),
                }))
            }
            Type::Byte => {
                Ok(Box::new(crate::ast::expressions::Identifier {
                    token: "IDENT".to_string(),
                    value: "byte".to_string(),
                }))
            }
            Type::Rune => {
                Ok(Box::new(crate::ast::expressions::Identifier {
                    token: "IDENT".to_string(),
                    value: "rune".to_string(),
                }))
            }
            Type::Extra => {
                Ok(Box::new(crate::ast::expressions::Identifier {
                    token: "IDENT".to_string(),
                    value: "extra".to_string(),
                }))
            }
            Type::Named(name) => {
                Ok(Box::new(crate::ast::expressions::Identifier {
                    token: "IDENT".to_string(),
                    value: name.clone(),
                }))
            }
            Type::TypeParam(name) => {
                Ok(Box::new(crate::ast::expressions::Identifier {
                    token: "IDENT".to_string(),
                    value: name.clone(),
                }))
            }
            Type::Array(elem_type, size) => {
                // Create an identifier representing array type syntax
                Ok(Box::new(crate::ast::expressions::Identifier {
                    token: "IDENT".to_string(),
                    value: format!("[{}]{}", size, elem_type.to_string()),
                }))
            }
            Type::Slice(elem_type) => {
                // Create an identifier representing slice type syntax
                Ok(Box::new(crate::ast::expressions::Identifier {
                    token: "IDENT".to_string(),
                    value: format!("][]{}", elem_type.to_string()),
                }))
            }
            Type::Map(key_type, value_type) => {
                // Create an identifier representing map type syntax
                Ok(Box::new(crate::ast::expressions::Identifier {
                    token: "IDENT".to_string(),
                    value: format!("tea[{}]{}", key_type.to_string(), value_type.to_string()),
                }))
            }
            Type::Pointer(target_type) => {
                // Create an identifier representing pointer type syntax
                Ok(Box::new(crate::ast::expressions::Identifier {
                    token: "IDENT".to_string(),
                    value: format!("@{}", target_type.to_string()),
                }))
            }
            Type::Function(param_types, return_type) => {
                // Create an identifier representing function type syntax
                let param_types_str = param_types
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                Ok(Box::new(crate::ast::expressions::Identifier {
                    token: "IDENT".to_string(),
                    value: format!("slay({}) {}", param_types_str, return_type.to_string()),
                }))
            }
            Type::Channel(elem_type) => {
                // Create an identifier representing channel type syntax
                Ok(Box::new(crate::ast::expressions::Identifier {
                    token: "IDENT".to_string(),
                    value: format!("dm<{}>", elem_type.to_string()),
                }))
            }
            Type::Struct(name, type_args) if !type_args.is_empty() => {
                // Create an identifier representing generic struct type syntax
                let type_args_str = type_args
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                Ok(Box::new(crate::ast::expressions::Identifier {
                    token: "IDENT".to_string(),
                    value: format!("{name}[{type_args_str}]"),
                }))
            }
            Type::Interface(name, type_args) if !type_args.is_empty() => {
                // Create an identifier representing generic interface type syntax
                let type_args_str = type_args
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                Ok(Box::new(crate::ast::expressions::Identifier {
                    token: "IDENT".to_string(),
                    value: format!("{name}[{type_args_str}]"),
                }))
            }
            Type::Struct(name, _) | Type::Interface(name, _) => {
                // Create an identifier for non-generic struct/interface
                Ok(Box::new(crate::ast::expressions::Identifier {
                    token: "IDENT".to_string(),
                    value: name.clone(),
                }))
            }
            Type::Generic(name, type_args) => {
                // Create an identifier representing generic type syntax
                let type_args_str = type_args
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                Ok(Box::new(crate::ast::expressions::Identifier {
                    token: "IDENT".to_string(),
                    value: format!("{name}[{type_args_str}]"),
                }))
            }
            Type::Unknown => {
                // Create an identifier for unknown type
                Ok(Box::new(crate::ast::expressions::Identifier {
                    token: "IDENT".to_string(),
                    value: "unknown".to_string(),
                }))
            }
        }
    }
}

/// Type checking functions for generics
pub trait GenericTypeChecker {
    /// Check if a generic type is valid
    fn check_generic_type(&self, generic_type: &Type, type_params: &[String]) -> Result<(), Error>;

    /// Check if generic type arguments are valid for a generic type
    fn check_generic_type_args(&self, generic_type: &Type, type_args: &[Type])
        -> Result<(), Error>;
}
