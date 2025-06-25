// Type Information Extractor for CURSED Documentation
// 
// This module provides comprehensive type information extraction for
// documentation generation, including complete type signatures, generic
// parameters, constraints, and relationships.

use crate::ast::*;
use crate::documentation::extractors::ast_node_support::{ExpressionType, Literal};
use crate::error::CursedError;
use crate::documentation::extractors::ast_extractor::{CompleteTypeInfo, TypeKind, SizeInfo};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use tracing::{debug, instrument};

/// Type information extractor for documentation
pub struct TypeExtractor {
    /// Cache for resolved types
    /// Known primitive types
    /// Type size information
impl TypeExtractor {
    /// Create a new type extractor
    #[instrument]
    pub fn new() -> crate::error::Result<()> {
        let mut primitives = HashSet::new();
        
        // CURSED primitive types
        primitives.insert("i8".to_string());
        primitives.insert("i16".to_string());
        primitives.insert("i32".to_string());
        primitives.insert("i64".to_string());
        primitives.insert("u8".to_string());
        primitives.insert("u16".to_string());
        primitives.insert("u32".to_string());
        primitives.insert("u64".to_string());
        primitives.insert("f32".to_string());
        primitives.insert("f64".to_string());
        primitives.insert("bool".to_string());
        primitives.insert("char".to_string());
        primitives.insert("string".to_string());
        primitives.insert("str".to_string());
        primitives.insert("unit".to_string());
        primitives.insert("never".to_string());
        primitives.insert("any".to_string());

        let mut size_info = HashMap::new();
        
        // Size information for primitive types
        size_info.insert("i8".to_string(), SizeInfo {
        });
        size_info.insert("i16".to_string(), SizeInfo {
        });
        size_info.insert("i32".to_string(), SizeInfo {
        });
        size_info.insert("i64".to_string(), SizeInfo {
        });
        size_info.insert("bool".to_string(), SizeInfo {
        });
        size_info.insert("unit".to_string(), SizeInfo {
        });

        Ok(Self {
        })
    /// Extract complete function type information
    #[instrument(skip(self, func_decl))]
    pub fn extract_function_type_info(
    ) -> crate::error::Result<()> {
        debug!("Extracting function type info for: {}", func_decl.to_string());

        // Build function signature
        let mut signature_parts = Vec::new();
        
        // Add async keyword if applicable
        if func_decl.is_async {
            signature_parts.push("async".to_string());
        signature_parts.push("slay".to_string());
        signature_parts.push(func_decl.to_string().clone());

        // Add generic parameters
        if let Some(ref generics) = func_decl.generic_params {
            if !generics.is_empty() {
                signature_parts.push(format!("<{}>", generics.join(", ")));
            }
        }

        // Add parameters
        let param_strings: Vec<String> = func_decl.parameters.iter()
            .map(|param| {
                let type_str = param.param_type.as_ref()
                    .map(|t| self.format_type_expression(t))
                    .transpose()?
                    .unwrap_or_else(|| "any".to_string());
                Ok(format!("{}: {}", param.to_string(), type_str))
            })
            .collect::<crate::error::Result<()>>()?;

        signature_parts.push(format!("({})", param_strings.join(", ")));

        // Add return type
        if let Some(ref return_type) = func_decl.return_type {
            signature_parts.push("->".to_string());
            signature_parts.push(self.format_type_expression(return_type)?);
        let type_signature = signature_parts.join(" ");

        // Extract parameter types for nested type information
        let mut nested_types = Vec::new();
        for param in &func_decl.parameters {
            if let Some(ref param_type) = param.param_type {
                nested_types.push(self.extract_type_info_from_expression(param_type)?);
            }
        }

        // Add return type to nested types
        if let Some(ref return_type) = func_decl.return_type {
            nested_types.push(self.extract_type_info_from_expression(return_type)?);
        Ok(CompleteTypeInfo {
            constraints: Vec::new(), // Would extract from generic constraints
            size_info: Some(SizeInfo {
                size_bytes: Some(std::mem::size_of::<*const ()>()), // Function pointer size
        })
    /// Extract struct type information
    #[instrument(skip(self, struct_decl))]
    pub fn extract_struct_type_info(
    ) -> crate::error::Result<()> {
        debug!("Extracting struct type info for: {}", struct_decl.to_string());

        // Build struct signature
        let mut signature_parts = Vec::new();
        signature_parts.push("squad".to_string());
        signature_parts.push(struct_decl.to_string().clone());

        // Add generic parameters
        if let Some(ref generics) = struct_decl.generic_params {
            if !generics.is_empty() {
                signature_parts.push(format!("<{}>", generics.join(", ")));
            }
        }

        // Add field information
        let field_strings: Vec<String> = struct_decl.fields.iter()
            .map(|field| {
                let type_str = field.field_type.as_ref()
                    .map(|t| self.format_type_expression(t))
                    .transpose()?
                    .unwrap_or_else(|| "any".to_string());
                Ok(format!("{}: {}", field.to_string(), type_str))
            })
            .collect::<crate::error::Result<()>>()?;

        signature_parts.push(format!("{{ {} }}", field_strings.join(", ")));

        let type_signature = signature_parts.join(" ");

        // Extract field types for nested type information
        let mut nested_types = Vec::new();
        for field in &struct_decl.fields {
            if let Some(ref field_type) = field.field_type {
                nested_types.push(self.extract_type_info_from_expression(field_type)?);
            }
        }

        // Calculate approximate size (simplified)
        let size_info = self.calculate_struct_size(struct_decl)?;

        Ok(CompleteTypeInfo {
        })
    /// Extract interface type information
    #[instrument(skip(self, interface_decl))]
    pub fn extract_interface_type_info(
    ) -> crate::error::Result<()> {
        debug!("Extracting interface type info for: {}", interface_decl.to_string());

        // Build interface signature
        let mut signature_parts = Vec::new();
        signature_parts.push("collab".to_string());
        signature_parts.push(interface_decl.to_string().clone());

        // Add generic parameters
        if let Some(ref generics) = interface_decl.generic_params {
            if !generics.is_empty() {
                signature_parts.push(format!("<{}>", generics.join(", ")));
            }
        }

        // Add method signatures
        let method_strings: Vec<String> = interface_decl.methods.iter()
            .map(|method| {
                let param_strings: Vec<String> = method.parameters.iter()
                    .map(|param| {
                        let type_str = param.param_type.as_ref()
                            .map(|t| self.format_type_expression(t))
                            .transpose()?
                            .unwrap_or_else(|| "any".to_string());
                        Ok(format!("{}: {}", param.to_string(), type_str))
                    })
                    .collect::<crate::error::Result<()>>()?;

                let return_str = method.return_type.as_ref()
                    .map(|t| format!(" -> {}", self.format_type_expression(t)?))
                    .transpose()?
                    .unwrap_or_default();

                Ok(format!("{}({}){}", method.to_string(), param_strings.join(", "), return_str))
            })
            .collect::<crate::error::Result<()>>()?;

        signature_parts.push(format!("{{ {} }}", method_strings.join("; ")));

        let type_signature = signature_parts.join(" ");

        // Extract method types for nested type information
        let nested_types = Vec::new(); // Methods are handled separately

        Ok(CompleteTypeInfo {
            size_info: Some(SizeInfo {
                size_bytes: Some(std::mem::size_of::<*const ()>() * 2), // vtable pointer + data pointer
        })
    /// Extract enum type information
    #[instrument(skip(self, enum_decl))]
    pub fn extract_enum_type_info(
    ) -> crate::error::Result<()> {
        debug!("Extracting enum type info for: {}", enum_decl.to_string());

        // Build enum signature
        let mut signature_parts = Vec::new();
        signature_parts.push("enum".to_string());
        signature_parts.push(enum_decl.to_string().clone());

        let variant_strings: Vec<String> = enum_decl.variants.iter()
            .map(|variant| {
                if variant.fields.is_empty() {
                    variant.to_string().clone()
                } else {
                    let field_strings: Vec<String> = variant.fields.iter()
                        .map(|field| {
                            field.field_type.as_ref()
                                .map(|t| self.format_type_expression(t))
                                .transpose()
                                .map(|opt| opt.unwrap_or_else(|| "any".to_string()))
                        })
                        .collect::<crate::error::Result<()>>()?;
                    Ok(format!("{}({})", variant.to_string(), field_strings.join(", ")))
                }
            })
            .collect::<crate::error::Result<()>>()?;

        signature_parts.push(format!("{{ {} }}", variant_strings.join(" | ")));

        let type_signature = signature_parts.join(" ");

        // Extract variant field types for nested type information
        let mut nested_types = Vec::new();
        for variant in &enum_decl.variants {
            for field in &variant.fields {
                if let Some(ref field_type) = field.field_type {
                    nested_types.push(self.extract_type_info_from_expression(field_type)?);
                }
            }
        Ok(CompleteTypeInfo {
        })
    /// Extract type alias information
    #[instrument(skip(self, type_alias))]
    pub fn extract_type_alias_info(
    ) -> crate::error::Result<()> {
        debug!("Extracting type alias info for: {}", type_alias.to_string());

        let target_type_str = self.format_type_expression(&type_alias.target_type)?;
        let type_signature = format!("type {} = {}", type_alias.to_string(), target_type_str);

        let nested_types = vec![self.extract_type_info_from_expression(&type_alias.target_type)?];

        Ok(CompleteTypeInfo {
            size_info: None, // Inherits from target type
        })
    /// Extract type information from an expression
    #[instrument(skip(self, expr))]
    pub fn extract_type_info_from_expression(
    ) -> crate::error::Result<()> {
        match &expr.expr_type {
            ExpressionType::Identifier(id) => {
                self.extract_identifier_type_info(&id.to_string())
            }
            ExpressionType::ArrayAccess(arr) => {
                let element_type = self.extract_type_info_from_expression(&arr.array)?;
                Ok(CompleteTypeInfo {
                    size_info: Some(SizeInfo {
                        size_bytes: None, // Dynamic size
                })
            }
            ExpressionType::FunctionCall(call) => {
                // Generic type instantiation like Vec<T>
                let base_type = self.extract_type_info_from_expression(&call.function)?;
                let arg_types: Vec<CompleteTypeInfo> = call.arguments.iter()
                    .map(|arg| self.extract_type_info_from_expression(arg))
                    .collect::<crate::error::Result<()>>()?;

                let type_args: Vec<String> = arg_types.iter()
                    .map(|t| t.type_name.clone())
                    .collect();

                Ok(CompleteTypeInfo {
                    size_info: None, // Depends on instantiation
                })
            }
            ExpressionType::MemberAccess(member) => {
                let object_type = self.extract_type_info_from_expression(&member.object)?;
                Ok(CompleteTypeInfo {
                })
            }
            ExpressionType::Literal(lit) => {
                self.extract_literal_type_info(lit)
            }
            _ => {
                // Default case for unknown expressions
                Ok(CompleteTypeInfo {
                })
            }
        }
    /// Extract type information for an identifier
    fn extract_identifier_type_info(&self, name: &str) -> crate::error::Result<()> {
        let type_kind = if self.primitives.contains(name) {
            TypeKind::Primitive
        } else {
            TypeKind::Custom

        Ok(CompleteTypeInfo {
        })
    /// Extract type information for a literal
    fn extract_literal_type_info(&self, literal: &Literal) -> crate::error::Result<()> {
        let (type_name, type_kind, size_info) = match literal {
            Literal::String(_) => ("string", TypeKind::Primitive, Some(SizeInfo {
                size_bytes: None, // Variable size
            Literal::Number(n) => {
                if n.contains('.') {
                    ("f64", TypeKind::Primitive, self.size_info.get("f64").cloned())
                } else {
                    ("i32", TypeKind::Primitive, self.size_info.get("i32").cloned())
                }
            }
            Literal::Null => ("null", TypeKind::Primitive, Some(SizeInfo {
            Literal::Array(arr) => {
                let element_types: Vec<CompleteTypeInfo> = arr.iter()
                    .map(|elem| self.extract_type_info_from_expression(elem))
                    .collect::<crate::error::Result<()>>()?;

                return Ok(CompleteTypeInfo {
                    size_info: Some(SizeInfo {
                        size_bytes: None, // Depends on element type and count
                });
            }
            Literal::Object(_) => ("object", TypeKind::Custom, Some(SizeInfo {
                size_bytes: None, // Variable size

        Ok(CompleteTypeInfo {
        })
    /// Format a type expression as a string
    #[instrument(skip(self, expr))]
    pub fn format_type_expression(&self, expr: &dyn Expression) -> crate::error::Result<()> {
        match &expr.expr_type {
            ExpressionType::ArrayAccess(arr) => {
                Ok(format!("{}[]", self.format_type_expression(&arr.array)?))
            }
            ExpressionType::FunctionCall(call) => {
                let args: Vec<String> = call.arguments.iter()
                    .map(|arg| self.format_type_expression(arg))
                    .collect::<crate::error::Result<()>>()?;
                    args.join(", ")))
            }
            ExpressionType::MemberAccess(member) => {
                    member.member))
            }
            ExpressionType::Literal(lit) => {
                match lit {
                    Literal::Array(arr) => {
                        let elements: Vec<String> = arr.iter()
                            .map(|elem| self.format_type_expression(elem))
                            .collect::<crate::error::Result<()>>()?;
                        Ok(format!("[{}]", elements.join(", ")))
                    }
                    Literal::Object(obj) => {
                        let fields: Vec<String> = obj.iter()
                            .map(|(k, v)| Ok(format!("{}: {}", k, self.format_type_expression(v)?)))
                            .collect::<crate::error::Result<()>>()?;
                        Ok(format!("{{{}}}", fields.join(", ")))
                    }
                }
            }
        }
    }

    /// Calculate struct size (simplified)
    fn calculate_struct_size(&self, struct_decl: &StructDeclaration) -> crate::error::Result<()> {
        let mut total_size = 0;
        let mut max_alignment = 1;
        let mut has_dst = false;

        for field in &struct_decl.fields {
            if let Some(ref field_type) = field.field_type {
                let field_type_info = self.extract_type_info_from_expression(field_type)?;
                if let Some(ref size_info) = field_type_info.size_info {
                    if let Some(size) = size_info.size_bytes {
                        total_size += size;
                    } else {
                        has_dst = true;
                    }
                    if let Some(alignment) = size_info.alignment {
                        max_alignment = max_alignment.max(alignment);
                    }
                    if size_info.is_dst {
                        has_dst = true;
                    }
                }
            }
        }

        // Add padding for alignment
        if total_size > 0 && max_alignment > 1 {
            total_size = (total_size + max_alignment - 1) & !(max_alignment - 1);
        Ok(SizeInfo {
        })
    /// Calculate enum size (simplified)
    fn calculate_enum_size(&self, enum_decl: &EnumDeclaration) -> crate::error::Result<()> {
        let discriminant_size = std::mem::size_of::<u32>(); // Enum discriminant
        let mut max_variant_size = 0;
        let mut max_alignment = std::mem::align_of::<u32>();

        for variant in &enum_decl.variants {
            let mut variant_size = 0;
            for field in &variant.fields {
                if let Some(ref field_type) = field.field_type {
                    let field_type_info = self.extract_type_info_from_expression(field_type)?;
                    if let Some(ref size_info) = field_type_info.size_info {
                        if let Some(size) = size_info.size_bytes {
                            variant_size += size;
                        }
                        if let Some(alignment) = size_info.alignment {
                            max_alignment = max_alignment.max(alignment);
                        }
                    }
                }
            }
            max_variant_size = max_variant_size.max(variant_size);
        let total_size = discriminant_size + max_variant_size;

        Ok(Some(SizeInfo {
        }))
    }
}
