//! Relationship Extractor for CURSED Documentation
//! 
//! This module provides comprehensive extraction of relationships between
//! code elements, including inheritance, implementation, usage, and
//! cross-reference relationships for documentation generation.

use crate::ast::*;
use crate::error::Error;
use crate::documentation::extractors::ast_extractor::{RelationshipInfo, RelationshipType, RelationshipStrength};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use tracing::{debug, instrument};

/// Relationship extractor for code element analysis
pub struct RelationshipExtractor {
    /// Cache of discovered relationships
    relationship_cache: HashMap<String, Vec<RelationshipInfo>>,
    /// Known type names in the codebase
    known_types: HashSet<String>,
    /// Function call relationships
    call_graph: HashMap<String, Vec<String>>,
}

impl RelationshipExtractor {
    /// Create a new relationship extractor
    #[instrument]
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            relationship_cache: HashMap::new(),
            known_types: HashSet::new(),
            call_graph: HashMap::new(),
        })
    }

    /// Extract relationships for a module declaration
    #[instrument(skip(self, module_decl, source_code))]
    pub fn extract_module_relationships(
        &self,
        module_decl: &ModuleDeclaration,
        source_code: &str,
    ) -> Result<Vec<RelationshipInfo>, Error> {
        debug!("Extracting module relationships for: {}", module_decl.name);
        
        let mut relationships = Vec::new();

        // Extract import relationships
        relationships.extend(self.extract_module_imports(module_decl, source_code)?);

        // Extract export relationships
        relationships.extend(self.extract_module_exports(module_decl, source_code)?);

        // Extract nested module relationships
        if let Some(ref body) = module_decl.body {
            relationships.extend(self.extract_nested_relationships(body, source_code)?);
        }

        Ok(relationships)
    }

    /// Extract relationships for a function declaration
    #[instrument(skip(self, func_decl, source_code))]
    pub fn extract_function_relationships(
        &self,
        func_decl: &FunctionDeclaration,
        source_code: &str,
    ) -> Result<Vec<RelationshipInfo>, Error> {
        debug!("Extracting function relationships for: {}", func_decl.name);
        
        let mut relationships = Vec::new();

        // Extract parameter type relationships
        for param in &func_decl.parameters {
            if let Some(ref param_type) = param.param_type {
                relationships.extend(self.extract_type_usage_relationships(
                    &func_decl.name,
                    param_type,
                    RelationshipType::Uses,
                )?);
            }
        }

        // Extract return type relationships
        if let Some(ref return_type) = func_decl.return_type {
            relationships.extend(self.extract_type_usage_relationships(
                &func_decl.name,
                return_type,
                RelationshipType::Uses,
            )?);
        }

        // Extract function call relationships from body
        relationships.extend(self.extract_function_call_relationships(
            &func_decl.name,
            &func_decl.body,
        )?);

        // Extract generic constraint relationships
        if let Some(ref constraints) = func_decl.constraints {
            relationships.extend(self.extract_constraint_relationships(
                &func_decl.name,
                constraints,
            )?);
        }

        Ok(relationships)
    }

    /// Extract relationships for a struct declaration
    #[instrument(skip(self, struct_decl, source_code))]
    pub fn extract_struct_relationships(
        &self,
        struct_decl: &StructDeclaration,
        source_code: &str,
    ) -> Result<Vec<RelationshipInfo>, Error> {
        debug!("Extracting struct relationships for: {}", struct_decl.name);
        
        let mut relationships = Vec::new();

        // Extract field type relationships
        for field in &struct_decl.fields {
            if let Some(ref field_type) = field.field_type {
                relationships.extend(self.extract_type_usage_relationships(
                    &struct_decl.name,
                    field_type,
                    RelationshipType::Contains,
                )?);
            }
        }

        // Extract implementation relationships
        relationships.extend(self.extract_struct_implementations(
            &struct_decl.name,
            source_code,
        )?);

        // Extract inheritance relationships
        relationships.extend(self.extract_inheritance_relationships(
            &struct_decl.name,
            source_code,
        )?);

        // Extract generic constraint relationships
        if let Some(ref constraints) = struct_decl.constraints {
            relationships.extend(self.extract_constraint_relationships(
                &struct_decl.name,
                constraints,
            )?);
        }

        Ok(relationships)
    }

    /// Extract relationships for an interface declaration
    #[instrument(skip(self, interface_decl, source_code))]
    pub fn extract_interface_relationships(
        &self,
        interface_decl: &InterfaceDeclaration,
        source_code: &str,
    ) -> Result<Vec<RelationshipInfo>, Error> {
        debug!("Extracting interface relationships for: {}", interface_decl.name);
        
        let mut relationships = Vec::new();

        // Extract method signature relationships
        for method in &interface_decl.methods {
            // Parameter type relationships
            for param in &method.parameters {
                if let Some(ref param_type) = param.param_type {
                    relationships.extend(self.extract_type_usage_relationships(
                        &interface_decl.name,
                        param_type,
                        RelationshipType::References,
                    )?);
                }
            }

            // Return type relationships
            if let Some(ref return_type) = method.return_type {
                relationships.extend(self.extract_type_usage_relationships(
                    &interface_decl.name,
                    return_type,
                    RelationshipType::References,
                )?);
            }
        }

        // Extract interface inheritance relationships
        relationships.extend(self.extract_interface_inheritance(
            &interface_decl.name,
            source_code,
        )?);

        // Extract implementation relationships (who implements this interface)
        relationships.extend(self.extract_interface_implementors(
            &interface_decl.name,
            source_code,
        )?);

        Ok(relationships)
    }

    /// Extract type usage relationships from an expression
    #[instrument(skip(self, source_name, type_expr))]
    fn extract_type_usage_relationships(
        &self,
        source_name: &str,
        type_expr: &dyn Expression,
        relationship_type: RelationshipType,
    ) -> Result<Vec<RelationshipInfo>, Error> {
        let mut relationships = Vec::new();

        match &type_expr.expr_type {
            ExpressionType::Identifier(id) => {
                if self.is_known_type(&id.name) {
                    relationships.push(RelationshipInfo {
                        relationship_type: relationship_type.clone(),
                        target: id.name.clone(),
                        strength: RelationshipStrength::Strong,
                        context: Some(format!("Type usage in {}", source_name)),
                    });
                }
            }
            ExpressionType::FunctionCall(call) => {
                // Generic type instantiation
                relationships.extend(self.extract_type_usage_relationships(
                    source_name,
                    &call.function,
                    relationship_type.clone(),
                )?);

                // Type arguments
                for arg in &call.arguments {
                    relationships.extend(self.extract_type_usage_relationships(
                        source_name,
                        arg,
                        relationship_type.clone(),
                    )?);
                }
            }
            ExpressionType::ArrayAccess(arr) => {
                // Array element type
                relationships.extend(self.extract_type_usage_relationships(
                    source_name,
                    &arr.array,
                    relationship_type,
                )?);
            }
            ExpressionType::MemberAccess(member) => {
                // Object type
                relationships.extend(self.extract_type_usage_relationships(
                    source_name,
                    &member.object,
                    relationship_type,
                )?);
            }
            _ => {
                // Other expression types may not represent type relationships
            }
        }

        Ok(relationships)
    }

    /// Extract function call relationships from a block statement
    #[instrument(skip(self, caller_name, body))]
    fn extract_function_call_relationships(
        &self,
        caller_name: &str,
        body: &AstNode,
    ) -> Result<Vec<RelationshipInfo>, Error> {
        let mut relationships = Vec::new();

        // Recursively search for function calls in the AST
        self.search_function_calls(body, caller_name, &mut relationships)?;

        Ok(relationships)
    }

    /// Recursively search for function calls in AST nodes
    fn search_function_calls(
        &self,
        node: &AstNode,
        caller_name: &str,
        relationships: &mut Vec<RelationshipInfo>,
    ) -> Result<(), Error> {
        match &node.node_type {
            AstNodeType::ExpressionStatement(expr_stmt) => {
                self.extract_calls_from_expression(&expr_stmt.expression, caller_name, relationships)?;
            }
            AstNodeType::BlockStatement(block) => {
                for stmt in &block.statements {
                    self.search_function_calls(stmt, caller_name, relationships)?;
                }
            }
            AstNodeType::IfStatement(if_stmt) => {
                self.extract_calls_from_expression(&if_stmt.test, caller_name, relationships)?;
                self.search_function_calls(&if_stmt.consequent, caller_name, relationships)?;
                if let Some(ref alternate) = if_stmt.alternate {
                    self.search_function_calls(alternate, caller_name, relationships)?;
                }
            }
            AstNodeType::WhileStatement(while_stmt) => {
                self.extract_calls_from_expression(&while_stmt.test, caller_name, relationships)?;
                self.search_function_calls(&while_stmt.body, caller_name, relationships)?;
            }
            AstNodeType::ForStatement(for_stmt) => {
                if let Some(ref init) = for_stmt.init {
                    self.search_function_calls(init, caller_name, relationships)?;
                }
                if let Some(ref test) = for_stmt.test {
                    self.extract_calls_from_expression(test, caller_name, relationships)?;
                }
                if let Some(ref update) = for_stmt.update {
                    self.extract_calls_from_expression(update, caller_name, relationships)?;
                }
                self.search_function_calls(&for_stmt.body, caller_name, relationships)?;
            }
            AstNodeType::ReturnStatement(ret) => {
                if let Some(ref argument) = ret.argument {
                    self.extract_calls_from_expression(argument, caller_name, relationships)?;
                }
            }
            AstNodeType::VariableDeclaration(var_decl) => {
                if let Some(ref init) = var_decl.init {
                    self.extract_calls_from_expression(init, caller_name, relationships)?;
                }
            }
            _ => {
                // Other node types may contain expressions recursively
            }
        }

        Ok(())
    }

    /// Extract function calls from an expression
    fn extract_calls_from_expression(
        &self,
        expr: &dyn Expression,
        caller_name: &str,
        relationships: &mut Vec<RelationshipInfo>,
    ) -> Result<(), Error> {
        match &expr.expr_type {
            ExpressionType::FunctionCall(call) => {
                if let ExpressionType::Identifier(id) = &call.function.expr_type {
                    relationships.push(RelationshipInfo {
                        relationship_type: RelationshipType::Calls,
                        target: id.name.clone(),
                        strength: RelationshipStrength::Strong,
                        context: Some(format!("Function call from {}", caller_name)),
                    });
                }

                // Recursively check arguments
                for arg in &call.arguments {
                    self.extract_calls_from_expression(arg, caller_name, relationships)?;
                }
            }
            ExpressionType::MemberAccess(member) => {
                // Method calls
                self.extract_calls_from_expression(&member.object, caller_name, relationships)?;
                
                relationships.push(RelationshipInfo {
                    relationship_type: RelationshipType::Calls,
                    target: member.member.clone(),
                    strength: RelationshipStrength::Weak,
                    context: Some(format!("Method call from {}", caller_name)),
                });
            }
            ExpressionType::BinaryExpression(bin) => {
                self.extract_calls_from_expression(&bin.left, caller_name, relationships)?;
                self.extract_calls_from_expression(&bin.right, caller_name, relationships)?;
            }
            ExpressionType::UnaryExpression(unary) => {
                self.extract_calls_from_expression(&unary.operand, caller_name, relationships)?;
            }
            ExpressionType::AssignmentExpression(assign) => {
                self.extract_calls_from_expression(&assign.left, caller_name, relationships)?;
                self.extract_calls_from_expression(&assign.right, caller_name, relationships)?;
            }
            ExpressionType::ConditionalExpression(cond) => {
                self.extract_calls_from_expression(&cond.condition, caller_name, relationships)?;
                self.extract_calls_from_expression(&cond.true_expr, caller_name, relationships)?;
                self.extract_calls_from_expression(&cond.false_expr, caller_name, relationships)?;
            }
            ExpressionType::ArrayAccess(arr) => {
                self.extract_calls_from_expression(&arr.array, caller_name, relationships)?;
                self.extract_calls_from_expression(&arr.index, caller_name, relationships)?;
            }
            _ => {
                // Other expression types may not contain function calls
            }
        }

        Ok(())
    }

    /// Extract constraint relationships
    fn extract_constraint_relationships(
        &self,
        source_name: &str,
        constraints: &[GenericConstraint],
    ) -> Result<Vec<RelationshipInfo>, Error> {
        let mut relationships = Vec::new();

        for constraint in constraints {
            relationships.push(RelationshipInfo {
                relationship_type: RelationshipType::DependsOn,
                target: constraint.constraint_type.clone(),
                strength: RelationshipStrength::Strong,
                context: Some(format!("Generic constraint in {}", source_name)),
            });
        }

        Ok(relationships)
    }

    /// Extract module import relationships
    fn extract_module_imports(
        &self,
        module_decl: &ModuleDeclaration,
        source_code: &str,
    ) -> Result<Vec<RelationshipInfo>, Error> {
        let mut relationships = Vec::new();

        // Parse import statements from source code
        for line in source_code.lines() {
            let line = line.trim();
            if line.starts_with("import ") {
                if let Some(module_name) = self.extract_import_module_name(line) {
                    relationships.push(RelationshipInfo {
                        relationship_type: RelationshipType::DependsOn,
                        target: module_name,
                        strength: RelationshipStrength::Strong,
                        context: Some(format!("Import in module {}", module_decl.name)),
                    });
                }
            }
        }

        Ok(relationships)
    }

    /// Extract module export relationships
    fn extract_module_exports(
        &self,
        module_decl: &ModuleDeclaration,
        source_code: &str,
    ) -> Result<Vec<RelationshipInfo>, Error> {
        let mut relationships = Vec::new();

        // Parse export statements from source code
        for line in source_code.lines() {
            let line = line.trim();
            if line.starts_with("export ") || line.contains(" export ") {
                // This would require more sophisticated parsing
                // For now, we'll leave this as a placeholder
            }
        }

        Ok(relationships)
    }

    /// Extract nested relationships from module body
    fn extract_nested_relationships(
        &self,
        body: &AstNode,
        source_code: &str,
    ) -> Result<Vec<RelationshipInfo>, Error> {
        let mut relationships = Vec::new();

        // This would recursively analyze the module body
        // For now, return empty vector
        Ok(relationships)
    }

    /// Extract struct implementation relationships
    fn extract_struct_implementations(
        &self,
        struct_name: &str,
        source_code: &str,
    ) -> Result<Vec<RelationshipInfo>, Error> {
        let mut relationships = Vec::new();

        // Look for impl blocks in source code
        for line in source_code.lines() {
            let line = line.trim();
            if line.starts_with("impl ") && line.contains(struct_name) {
                if let Some(interface_name) = self.extract_impl_interface_name(line) {
                    relationships.push(RelationshipInfo {
                        relationship_type: RelationshipType::Implements,
                        target: interface_name,
                        strength: RelationshipStrength::Strong,
                        context: Some(format!("Implementation by {}", struct_name)),
                    });
                }
            }
        }

        Ok(relationships)
    }

    /// Extract inheritance relationships
    fn extract_inheritance_relationships(
        &self,
        struct_name: &str,
        source_code: &str,
    ) -> Result<Vec<RelationshipInfo>, Error> {
        let mut relationships = Vec::new();

        // Look for inheritance patterns in source code
        // This would depend on CURSED's inheritance syntax
        for line in source_code.lines() {
            let line = line.trim();
            if line.contains("extends") && line.contains(struct_name) {
                if let Some(parent_name) = self.extract_parent_class_name(line) {
                    relationships.push(RelationshipInfo {
                        relationship_type: RelationshipType::Extends,
                        target: parent_name,
                        strength: RelationshipStrength::Strong,
                        context: Some(format!("Inheritance by {}", struct_name)),
                    });
                }
            }
        }

        Ok(relationships)
    }

    /// Extract interface inheritance relationships
    fn extract_interface_inheritance(
        &self,
        interface_name: &str,
        source_code: &str,
    ) -> Result<Vec<RelationshipInfo>, Error> {
        let mut relationships = Vec::new();

        // Look for interface inheritance patterns
        for line in source_code.lines() {
            let line = line.trim();
            if line.contains("extends") && line.contains(interface_name) {
                if let Some(parent_interface) = self.extract_parent_interface_name(line) {
                    relationships.push(RelationshipInfo {
                        relationship_type: RelationshipType::Extends,
                        target: parent_interface,
                        strength: RelationshipStrength::Strong,
                        context: Some(format!("Interface inheritance by {}", interface_name)),
                    });
                }
            }
        }

        Ok(relationships)
    }

    /// Extract interface implementors
    fn extract_interface_implementors(
        &self,
        interface_name: &str,
        source_code: &str,
    ) -> Result<Vec<RelationshipInfo>, Error> {
        let mut relationships = Vec::new();

        // Look for types that implement this interface
        for line in source_code.lines() {
            let line = line.trim();
            if line.contains("impl ") && line.contains(interface_name) {
                if let Some(implementor_name) = self.extract_implementor_name(line) {
                    relationships.push(RelationshipInfo {
                        relationship_type: RelationshipType::Implements,
                        target: implementor_name,
                        strength: RelationshipStrength::Strong,
                        context: Some(format!("Implemented by types using {}", interface_name)),
                    });
                }
            }
        }

        Ok(relationships)
    }

    /// Check if a type name is known
    fn is_known_type(&self, type_name: &str) -> bool {
        // Check against known types or use heuristics
        self.known_types.contains(type_name) || 
        type_name.chars().next().map_or(false, |c| c.is_uppercase()) // Assume uppercase names are types
    }

    /// Extract module name from import statement
    fn extract_import_module_name(&self, import_line: &str) -> Option<String> {
        // Parse: import "module_name" or import module_name
        if let Some(start) = import_line.find('"') {
            if let Some(end) = import_line[start + 1..].find('"') {
                return Some(import_line[start + 1..start + 1 + end].to_string());
            }
        }
        
        // Handle unquoted imports
        let parts: Vec<&str> = import_line.split_whitespace().collect();
        if parts.len() >= 2 {
            Some(parts[1].to_string())
        } else {
            None
        }
    }

    /// Extract interface name from impl statement
    fn extract_impl_interface_name(&self, impl_line: &str) -> Option<String> {
        // Parse: impl InterfaceName for StructName
        if let Some(for_pos) = impl_line.find(" for ") {
            let interface_part = &impl_line[5..for_pos].trim(); // Skip "impl "
            Some(interface_part.to_string())
        } else {
            None
        }
    }

    /// Extract parent class name from inheritance statement
    fn extract_parent_class_name(&self, extends_line: &str) -> Option<String> {
        // Parse: class ChildName extends ParentName
        if let Some(extends_pos) = extends_line.find("extends ") {
            let parent_part = &extends_line[extends_pos + 8..].trim();
            Some(parent_part.split_whitespace().next()?.to_string())
        } else {
            None
        }
    }

    /// Extract parent interface name
    fn extract_parent_interface_name(&self, extends_line: &str) -> Option<String> {
        self.extract_parent_class_name(extends_line) // Same logic
    }

    /// Extract implementor name from impl statement
    fn extract_implementor_name(&self, impl_line: &str) -> Option<String> {
        // Parse: impl InterfaceName for StructName
        if let Some(for_pos) = impl_line.find(" for ") {
            let implementor_part = &impl_line[for_pos + 5..].trim();
            Some(implementor_part.split_whitespace().next()?.to_string())
        } else {
            None
        }
    }

    /// Add known type to the extractor
    pub fn add_known_type(&mut self, type_name: String) {
        self.known_types.insert(type_name);
    }

    /// Get relationship cache
    pub fn relationship_cache(&self) -> &HashMap<String, Vec<RelationshipInfo>> {
        &self.relationship_cache
    }

    /// Get call graph
    pub fn call_graph(&self) -> &HashMap<String, Vec<String>> {
        &self.call_graph
    }
}
