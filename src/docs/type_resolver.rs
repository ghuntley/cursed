//! Type system integration for CURSED documentation generation
//!
//! This module provides type resolution capabilities for generating comprehensive
//! documentation that includes resolved type information, generic constraints,
//! and type hierarchies for the CURSED programming language.

use crate::ast::*;
use crate::docs::{DocError, DocResult};
use std::collections::{HashMap, HashSet};
use tracing::{debug, instrument};

/// Helper trait for joining parameter lists
trait VecParameterJoinExt {
    fn join(&self, separator: &str) -> String;
}

impl VecParameterJoinExt for Vec<Parameter> {
    fn join(&self, separator: &str) -> String {
        self.iter()
            .map(|p| format!("{} {}", p.name.string(), p.param_type.string()))
            .collect::<Vec<_>>()
            .join(separator)
    }
}

/// Type resolution context for documentation generation
#[derive(Debug, Clone)]
pub struct TypeResolver {
    /// Known type definitions in the current scope
    type_definitions: HashMap<String, ResolvedType>,
    /// Generic type constraints
    constraints: HashMap<String, Vec<String>>,
    /// Module import information
    imports: HashMap<String, String>,
    /// Type aliases
    aliases: HashMap<String, String>,
    /// Interface implementations
    implementations: HashMap<String, Vec<String>>,
}

/// Resolved type information for documentation
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedType {
    /// Base type name
    pub name: String,
    /// Package or module origin
    pub origin: Option<String>,
    /// Generic type parameters
    pub type_parameters: Vec<String>,
    /// Type kind (struct, interface, primitive, etc.)
    pub kind: TypeKind,
    /// Detailed type signature
    pub signature: String,
    /// Documentation comment
    pub documentation: Option<String>,
    /// Fields (for structs) or methods (for interfaces)
    pub members: Vec<TypeMember>,
    /// Generic constraints
    pub constraints: Vec<String>,
}

/// Type hierarchy information for cross-referencing
#[derive(Debug, Clone)]
pub struct TypeHierarchy {
    /// Types that implement this interface
    pub implementations: Vec<String>,
    /// Interfaces that this type implements
    pub implements: Vec<String>,
    /// Type dependencies
    pub dependencies: Vec<String>,
    /// Types that depend on this one
    pub dependents: Vec<String>,
}

/// Kind of type for documentation categorization
#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind {
    Struct,      // squad
    Interface,   // collab
    Primitive,   // normie, facts_string, etc.
    Generic,     // Generic type parameter
    Slice,       // []Type
    Map,         // map[K]V
    Channel,     // chan Type
    Pointer,     // @Type
    Function,    // func(params) -> return
    Alias,       // type alias
}

/// Type member (field or method) information
#[derive(Debug, Clone, PartialEq)]
pub struct TypeMember {
    /// Member name
    pub name: String,
    /// Member type
    pub member_type: String,
    /// Visibility (public/private)
    pub visibility: String,
    /// Documentation comment
    pub documentation: Option<String>,
    /// For methods: parameter information
    pub parameters: Vec<(String, String)>,
    /// For methods: return type
    pub return_type: Option<String>,
}

impl TypeResolver {
    /// Create a new type resolver
    pub fn new() -> Self {
        Self {
            type_definitions: HashMap::new(),
            constraints: HashMap::new(),
            imports: HashMap::new(),
            aliases: HashMap::new(),
            implementations: HashMap::new(),
        }
    }

    /// Resolve type from program AST
    #[instrument(skip(self, program))]
    pub fn resolve_from_program(&mut self, program: &Program) -> DocResult<()> {
        debug!("Resolving types from program AST");
        
        // First pass: collect all type definitions
        for statement in &program.statements {
            self.collect_type_definitions(statement)?;
        }

        // Second pass: resolve type relationships and constraints
        for statement in &program.statements {
            self.resolve_type_relationships(statement)?;
        }

        debug!("Resolved {} type definitions", self.type_definitions.len());
        Ok(())
    }

    /// Collect type definitions from statements
    fn collect_type_definitions(&mut self, statement: &Box<dyn Statement>) -> DocResult<()> {
        // Handle struct (squad) definitions
        if let Some(squad) = statement.as_any().downcast_ref::<SquadStatement>() {
            let resolved_type = self.resolve_squad_type(squad)?;
            self.type_definitions.insert(squad.name.string(), resolved_type);
        }
        
        // Handle interface (collab) definitions
        else if let Some(collab) = statement.as_any().downcast_ref::<CollabStatement>() {
            let resolved_type = self.resolve_collab_type(collab)?;
            self.type_definitions.insert(collab.name.string(), resolved_type);
        }
        
        // Handle function definitions for function types
        else if let Some(func) = statement.as_any().downcast_ref::<FunctionStatement>() {
            if !func.type_parameters.is_empty() {
                // This is a generic function, track its constraints
                let constraints: Vec<String> = func.generic_constraints
                    .iter()
                    .map(|c| format!("{}:{}", c.parameter_name, c.interface_name))
                    .collect();
                self.constraints.insert(func.name.string(), constraints);
            }
        }

        Ok(())
    }

    /// Resolve type relationships and constraints
    fn resolve_type_relationships(&mut self, statement: &Box<dyn Statement>) -> DocResult<()> {
        // Track interface implementations
        if let Some(squad) = statement.as_any().downcast_ref::<SquadStatement>() {
            // For now, we'll need to infer implementations from usage
            // This would require more sophisticated analysis in a full implementation
            self.track_implementations(&squad.name.string());
        }

        Ok(())
    }

    /// Resolve struct (squad) type information
    fn resolve_squad_type(&self, squad: &SquadStatement) -> DocResult<ResolvedType> {
        let mut members = Vec::new();
        
        // Extract field information
        for field in &squad.fields {
            let member = TypeMember {
                name: field.name.string(),
                member_type: field.type_name.string(),
                visibility: "public".to_string(), // Default in CURSED
                documentation: None, // Would be extracted from comments
                parameters: Vec::new(),
                return_type: None,
            };
            members.push(member);
        }

        // Extract generic parameters
        let type_parameters: Vec<String> = squad.type_parameters
            .iter()
            .map(|tp| tp.string())
            .collect();

        // Generate signature
        let signature = self.generate_squad_signature(squad);

        Ok(ResolvedType {
            name: squad.name.string(),
            origin: None, // Would be set based on package context
            type_parameters,
            kind: TypeKind::Struct,
            signature,
            documentation: None, // Would be extracted from comments
            members,
            constraints: Vec::new(),
        })
    }

    /// Resolve interface (collab) type information
    fn resolve_collab_type(&self, collab: &CollabStatement) -> DocResult<ResolvedType> {
        let mut members = Vec::new();
        
        // Extract method information
        for method in &collab.methods {
            let parameters: Vec<(String, String)> = method.parameters
                .iter()
                .map(|p| (p.name.string(), p.param_type.string()))
                .collect();

            let member = TypeMember {
                name: method.name.string(),
                member_type: "function".to_string(),
                visibility: "public".to_string(),
                documentation: None,
                parameters,
                return_type: method.return_type.as_ref().map(|rt| rt.string()),
            };
            members.push(member);
        }

        // Extract generic parameters
        let type_parameters: Vec<String> = collab.type_parameters
            .iter()
            .map(|tp| tp.string())
            .collect();

        // Generate signature
        let signature = self.generate_collab_signature(collab);

        Ok(ResolvedType {
            name: collab.name.string(),
            origin: None,
            type_parameters,
            kind: TypeKind::Interface,
            signature,
            documentation: None,
            members,
            constraints: Vec::new(),
        })
    }

    /// Generate signature for struct (squad)
    fn generate_squad_signature(&self, squad: &SquadStatement) -> String {
        let mut signature = String::new();
        
        signature.push_str("squad ");
        signature.push_str(&squad.name.string());
        
        // Add generic parameters
        if !squad.type_parameters.is_empty() {
            signature.push('[');
            let type_params: Vec<String> = squad.type_parameters
                .iter()
                .map(|tp| tp.string())
                .collect();
            signature.push_str(&type_params.join(", "));
            signature.push(']');
        }
        
        signature.push_str(" {\n");
        for field in &squad.fields {
            signature.push_str(&format!("  {} {}\n", field.name.string(), field.type_name.string()));
        }
        signature.push('}');
        
        signature
    }

    /// Generate signature for interface (collab)
    fn generate_collab_signature(&self, collab: &CollabStatement) -> String {
        let mut signature = String::new();
        
        signature.push_str("collab ");
        signature.push_str(&collab.name.string());
        
        // Add generic parameters
        if !collab.type_parameters.is_empty() {
            signature.push('[');
            let type_params: Vec<String> = collab.type_parameters
                .iter()
                .map(|tp| tp.string())
                .collect();
            signature.push_str(&type_params.join(", "));
            signature.push(']');
        }
        
        signature.push_str(" {\n");
        for method in &collab.methods {
            signature.push_str(&format!(
                "  {}({}) {}\n",
                method.name.string(),
                method.parameters.join(", "),
                method.return_type.as_ref().map(|rt| rt.string()).unwrap_or_else(|| "void".to_string())
            ));
        }
        signature.push('}');
        
        signature
    }

    /// Track interface implementations
    fn track_implementations(&mut self, type_name: &str) {
        // This would be enhanced to actually detect implementations
        // For now, just create an empty entry
        self.implementations.entry(type_name.to_string()).or_insert_with(Vec::new);
    }

    /// Resolve a type name to its full information
    pub fn resolve_type(&self, type_name: &str) -> Option<&ResolvedType> {
        self.type_definitions.get(type_name)
    }

    /// Get type hierarchy for a given type
    pub fn get_type_hierarchy(&self, type_name: &str) -> TypeHierarchy {
        let implementations = self.implementations
            .get(type_name)
            .cloned()
            .unwrap_or_default();

        // Find what this type implements
        let implements = self.implementations
            .iter()
            .filter_map(|(interface, impls)| {
                if impls.contains(&type_name.to_string()) {
                    Some(interface.clone())
                } else {
                    None
                }
            })
            .collect();

        TypeHierarchy {
            implementations,
            implements,
            dependencies: self.get_dependencies(type_name),
            dependents: self.get_dependents(type_name),
        }
    }

    /// Get type dependencies
    fn get_dependencies(&self, type_name: &str) -> Vec<String> {
        if let Some(resolved_type) = self.type_definitions.get(type_name) {
            let mut deps = HashSet::new();
            
            // Add dependencies from members
            for member in &resolved_type.members {
                deps.insert(member.member_type.clone());
            }
            
            // Add generic parameter constraints
            for constraint in &resolved_type.constraints {
                deps.insert(constraint.clone());
            }
            
            deps.into_iter().collect()
        } else {
            Vec::new()
        }
    }

    /// Get types that depend on this one
    fn get_dependents(&self, type_name: &str) -> Vec<String> {
        self.type_definitions
            .iter()
            .filter_map(|(name, resolved_type)| {
                let depends_on_type = resolved_type.members
                    .iter()
                    .any(|member| member.member_type == type_name)
                    || resolved_type.constraints
                        .iter()
                        .any(|constraint| constraint == type_name);
                
                if depends_on_type {
                    Some(name.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Resolve complex type expressions
    pub fn resolve_complex_type(&self, type_expr: &str) -> DocResult<ResolvedType> {
        // Handle slice types
        if type_expr.starts_with("[]") {
            let element_type = &type_expr[2..];
            return Ok(ResolvedType {
                name: type_expr.to_string(),
                origin: None,
                type_parameters: vec![element_type.to_string()],
                kind: TypeKind::Slice,
                signature: format!("[]{}", element_type),
                documentation: None,
                members: Vec::new(),
                constraints: Vec::new(),
            });
        }

        // Handle map types
        if type_expr.starts_with("map[") {
            return self.resolve_map_type(type_expr);
        }

        // Handle channel types
        if type_expr.starts_with("chan ") {
            let element_type = &type_expr[5..];
            return Ok(ResolvedType {
                name: type_expr.to_string(),
                origin: None,
                type_parameters: vec![element_type.to_string()],
                kind: TypeKind::Channel,
                signature: format!("chan {}", element_type),
                documentation: None,
                members: Vec::new(),
                constraints: Vec::new(),
            });
        }

        // Handle pointer types
        if type_expr.starts_with("@") {
            let pointed_type = &type_expr[1..];
            return Ok(ResolvedType {
                name: type_expr.to_string(),
                origin: None,
                type_parameters: vec![pointed_type.to_string()],
                kind: TypeKind::Pointer,
                signature: format!("@{}", pointed_type),
                documentation: None,
                members: Vec::new(),
                constraints: Vec::new(),
            });
        }

        // Default to looking up in type definitions
        if let Some(resolved) = self.type_definitions.get(type_expr) {
            Ok(resolved.clone())
        } else {
            // Create a placeholder for unknown types
            Ok(ResolvedType {
                name: type_expr.to_string(),
                origin: None,
                type_parameters: Vec::new(),
                kind: TypeKind::Primitive,
                signature: type_expr.to_string(),
                documentation: None,
                members: Vec::new(),
                constraints: Vec::new(),
            })
        }
    }

    /// Resolve map type expressions
    fn resolve_map_type(&self, type_expr: &str) -> DocResult<ResolvedType> {
        // Parse map[K]V syntax
        if let Some(closing_bracket) = type_expr.find(']') {
            let key_type = &type_expr[4..closing_bracket];
            let value_type = &type_expr[closing_bracket + 1..];
            
            return Ok(ResolvedType {
                name: type_expr.to_string(),
                origin: None,
                type_parameters: vec![key_type.to_string(), value_type.to_string()],
                kind: TypeKind::Map,
                signature: format!("map[{}]{}", key_type, value_type),
                documentation: None,
                members: Vec::new(),
                constraints: Vec::new(),
            });
        }

        Err(DocError::AstError(format!("Invalid map type syntax: {}", type_expr)))
    }

    /// Get all resolved types
    pub fn get_all_types(&self) -> &HashMap<String, ResolvedType> {
        &self.type_definitions
    }

    /// Add type alias
    pub fn add_alias(&mut self, alias: String, target: String) {
        self.aliases.insert(alias, target);
    }

    /// Add import mapping
    pub fn add_import(&mut self, import_name: String, package_path: String) {
        self.imports.insert(import_name, package_path);
    }
}

impl Default for TypeResolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::declarations::*;
    use crate::ast::expressions::identifiers::Identifier;

    #[test]
    fn test_type_resolver_creation() {
        let resolver = TypeResolver::new();
        assert_eq!(resolver.type_definitions.len(), 0);
    }

    #[test]
    fn test_resolve_complex_types() {
        let resolver = TypeResolver::new();
        
        // Test slice type
        let slice_type = resolver.resolve_complex_type("[]normie").unwrap();
        assert_eq!(slice_type.kind, TypeKind::Slice);
        assert_eq!(slice_type.type_parameters[0], "normie");
        
        // Test map type
        let map_type = resolver.resolve_complex_type("map[facts_string]normie").unwrap();
        assert_eq!(map_type.kind, TypeKind::Map);
        assert_eq!(map_type.type_parameters[0], "facts_string");
        assert_eq!(map_type.type_parameters[1], "normie");
        
        // Test channel type
        let chan_type = resolver.resolve_complex_type("chan normie").unwrap();
        assert_eq!(chan_type.kind, TypeKind::Channel);
        assert_eq!(chan_type.type_parameters[0], "normie");
        
        // Test pointer type
        let ptr_type = resolver.resolve_complex_type("@Person").unwrap();
        assert_eq!(ptr_type.kind, TypeKind::Pointer);
        assert_eq!(ptr_type.type_parameters[0], "Person");
    }

    #[test]
    fn test_type_hierarchy() {
        let mut resolver = TypeResolver::new();
        
        // Add some mock implementations
        resolver.implementations.insert(
            "Drawable".to_string(),
            vec!["Circle".to_string(), "Rectangle".to_string()]
        );
        
        let hierarchy = resolver.get_type_hierarchy("Drawable");
        assert_eq!(hierarchy.implementations.len(), 2);
        assert!(hierarchy.implementations.contains(&"Circle".to_string()));
        assert!(hierarchy.implementations.contains(&"Rectangle".to_string()));
    }
}
