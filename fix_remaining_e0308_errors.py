#!/usr/bin/env python3

import os
import re

def fix_session_timeout_errors():
    """Fix async closure and timeout middleware type issues"""
    session_timeout_path = "src/stdlib/web_vibez/session_timeout.rs"
    
    with open(session_timeout_path, 'r') as f:
        content = f.read()
    
    # Fix timeout middleware calls - wrap in async block
    content = re.sub(
        r'(timeout_middleware\.with_database_timeout\(\s*operation_id,\s*operation_type,)',
        r'async move { \1',
        content
    )
    
    # Close async blocks properly
    content = re.sub(
        r'(\w+\.with_database_timeout\([^)]+\))\s*\.await\?;',
        r'\1.await? }.await?;',
        content
    )
    
    with open(session_timeout_path, 'w') as f:
        f.write(content)

def fix_postgres_driver_errors():
    """Fix PostgreSQL connection type conversion issues"""
    driver_path = "src/stdlib/database/postgres/driver.rs"
    
    with open(driver_path, 'r') as f:
        content = f.read()
    
    # Fix client clone issue
    content = re.sub(
        r'PostgresConnection::from_client\(\s*pooled_conn\.client\(\)\.clone\(\),',
        r'PostgresConnection::from_client(\n                pooled_conn.into_client(),',
        content
    )
    
    # Add conversion methods if needed
    if 'impl PostgresConnection' in content:
        # Add conversion method
        conversion_method = '''
    pub fn from_pooled_client(client: tokio_postgres::Client) -> Self {
        Self::from_client(client)
    }
'''
        content = content.replace(
            'impl PostgresConnection {',
            f'impl PostgresConnection {{{conversion_method}'
        )
    
    with open(driver_path, 'w') as f:
        f.write(content)

def fix_template_streaming_errors():
    """Fix template filtering parameter type issues"""
    template_path = "src/stdlib/template/template_streaming.rs"
    
    with open(template_path, 'r') as f:
        content = f.read()
    
    # Fix filter type conversion
    content = re.sub(
        r'Self::apply_filters_to_value\(&value, filters, context\)',
        r'Self::apply_filters_to_value(&value, &filters.iter().map(|f| f.name.clone()).collect::<Vec<_>>(), context)',
        content
    )
    
    # Add filter conversion helper if needed
    if 'impl' in content and 'apply_filters_to_value' in content:
        helper_method = '''
    fn convert_filter_calls_to_strings(filters: &[FilterCall]) -> Vec<String> {
        filters.iter().map(|f| f.name.clone()).collect()
    }
'''
        content = content.replace(
            'impl TemplateStreaming {',
            f'impl TemplateStreaming {{{helper_method}'
        )
    
    with open(template_path, 'w') as f:
        f.write(content)

def fix_async_io_errors():
    """Fix async I/O type parameter issues"""
    async_io_path = "src/stdlib/async/io.rs"
    
    with open(async_io_path, 'r') as f:
        content = f.read()
    
    # Fix spawn_blocking_io type parameter
    content = re.sub(
        r'spawn_blocking_io_public<F, R>\(f: F\) -> R',
        r'spawn_blocking_io_public<F, R>(f: F) -> Result<R, crate::error::CursedError>\nwhere\n    F: FnOnce() -> R + Send + \'static,\n    R: Send + \'static',
        content
    )
    
    # Fix return type wrapping
    content = re.sub(
        r'tokio::task::spawn_blocking\(move \|\| async move \{ f\(\)\.await \}\)\.await',
        r'Ok(tokio::task::spawn_blocking(move || f()).await.map_err(|e| crate::error::CursedError::General(e.to_string()))?)',
        content
    )
    
    with open(async_io_path, 'w') as f:
        f.write(content)

def fix_ast_extractor_errors():
    """Fix AST extractor field access and type conversion issues"""
    extractor_path = "src/documentation/extractors/ast_extractor.rs"
    
    with open(extractor_path, 'r') as f:
        content = f.read()
    
    # Fix struct declaration location access
    content = re.sub(
        r'&struct_decl\.location',
        r'struct_decl.location.as_ref().unwrap_or(&SourceLocation::default())',
        content
    )
    
    # Fix struct name access
    content = re.sub(
        r'struct_decl\.name\.clone\(\)',
        r'struct_decl.name.value.clone()',
        content
    )
    
    # Fix location unwrapping
    content = re.sub(
        r'struct_decl\.location\.clone\(\)',
        r'struct_decl.location.clone().unwrap_or_default()',
        content
    )
    
    # Fix interface declaration location
    content = re.sub(
        r'&interface_decl\.location',
        r'interface_decl.location.as_ref().unwrap_or(&SourceLocation::default())',
        content
    )
    
    # Fix interface name access
    content = re.sub(
        r'interface_decl\.name\.clone\(\)',
        r'interface_decl.name.value.clone()',
        content
    )
    
    with open(extractor_path, 'w') as f:
        f.write(content)

def fix_lsp_enhanced_symbols_errors():
    """Fix LSP enhanced symbols type conversion issues"""
    lsp_path = "src/lsp/enhanced_symbols.rs"
    
    with open(lsp_path, 'r') as f:
        content = f.read()
    
    # Add type conversion functions
    conversion_functions = '''
    fn convert_to_core_struct_field(&self, field: &ast::declarations::main::StructField) -> core_types::StructField {
        core_types::StructField {
            name: field.name.value.clone(),
            field_type: field.field_type.clone(),
            is_public: field.is_public,
            location: field.location.clone(),
        }
    }

    fn convert_to_core_interface_method(&self, method: &ast::declarations::main::InterfaceMethod) -> core_types::InterfaceMethod {
        core_types::InterfaceMethod {
            name: method.name.value.clone(),
            parameters: method.parameters.clone(),
            return_type: method.return_type.clone(),
            location: method.location.clone(),
        }
    }

    fn convert_to_core_method_declaration(&self, method: &ast::declarations::main::MethodDeclaration) -> core_types::InterfaceMethod {
        core_types::InterfaceMethod {
            name: method.name.value.clone(),
            parameters: method.parameters.clone(),
            return_type: method.return_type.clone(),
            location: method.location.clone(),
        }
    }

    fn convert_to_core_variable_declaration(&self, var_decl: &ast::declarations::main::VariableDeclaration) -> core_types::VariableDeclaration {
        core_types::VariableDeclaration {
            name: var_decl.name.value.clone(),
            var_type: var_decl.var_type.clone(),
            value: var_decl.value.clone(),
            is_mutable: var_decl.is_mutable,
            location: var_decl.location.clone(),
        }
    }
'''
    
    # Add conversion functions to impl block
    content = re.sub(
        r'(impl.*EnhancedSymbolProvider.*\{)',
        f'\\1{conversion_functions}',
        content
    )
    
    # Fix method calls using conversion functions
    content = re.sub(
        r'self\.create_field_symbol\(field\)',
        r'self.create_field_symbol(&self.convert_to_core_struct_field(field))',
        content
    )
    
    content = re.sub(
        r'self\.create_method_symbol\(method\)',
        r'self.create_method_symbol(&self.convert_to_core_interface_method(method))',
        content
    )
    
    content = re.sub(
        r'self\.get_field_range\(field\)',
        r'self.get_field_range(self.convert_to_core_struct_field(field))',
        content
    )
    
    content = re.sub(
        r'self\.get_field_name_range\(field\)',
        r'self.get_field_name_range(self.convert_to_core_struct_field(field))',
        content
    )
    
    content = re.sub(
        r'self\.get_method_range\(method\)',
        r'self.get_method_range(self.convert_to_core_interface_method(method))',
        content
    )
    
    content = re.sub(
        r'self\.get_method_name_range\(method\)',
        r'self.get_method_name_range(self.convert_to_core_interface_method(method))',
        content
    )
    
    content = re.sub(
        r'self\.get_variable_range\(var_decl\)',
        r'self.get_variable_range(self.convert_to_core_variable_declaration(var_decl))',
        content
    )
    
    content = re.sub(
        r'self\.get_variable_name_range\(var_decl\)',
        r'self.get_variable_name_range(self.convert_to_core_variable_declaration(var_decl))',
        content
    )
    
    with open(lsp_path, 'w') as f:
        f.write(content)

def add_missing_imports_and_defaults():
    """Add missing imports and default implementations"""
    
    # Add default implementation for SourceLocation
    source_location_path = "src/types/source_location.rs"
    if os.path.exists(source_location_path):
        with open(source_location_path, 'r') as f:
            content = f.read()
        
        if 'impl Default for SourceLocation' not in content:
            default_impl = '''
impl Default for SourceLocation {
    fn default() -> Self {
        SourceLocation {
            file: String::new(),
            line: 0,
            column: 0,
            offset: 0,
        }
    }
}
'''
            content += default_impl
            
            with open(source_location_path, 'w') as f:
                f.write(content)
    
    # Add missing core_types if needed
    core_types_path = "src/types/core_types.rs"
    if not os.path.exists(core_types_path):
        core_types_content = '''
use crate::types::source_location::SourceLocation;
use crate::ast::types::Type;

#[derive(Debug, Clone)]
pub struct StructField {
    pub name: String,
    pub field_type: Type,
    pub is_public: bool,
    pub location: Option<SourceLocation>,
}

#[derive(Debug, Clone)]
pub struct InterfaceMethod {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub location: Option<SourceLocation>,
}

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub name: String,
    pub var_type: Option<Type>,
    pub value: Option<Expression>,
    pub is_mutable: bool,
    pub location: Option<SourceLocation>,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub param_type: Type,
}

// Placeholder for Expression type
#[derive(Debug, Clone)]
pub enum Expression {
    // Add variants as needed
    Literal(String),
}
'''
        with open(core_types_path, 'w') as f:
            f.write(core_types_content)

def main():
    """Execute all fixes for remaining E0308 errors"""
    print("Fixing remaining E0308 type mismatch errors...")
    
    # Fix each category of errors
    print("1. Fixing session timeout async closure issues...")
    fix_session_timeout_errors()
    
    print("2. Fixing PostgreSQL driver connection issues...")
    fix_postgres_driver_errors()
    
    print("3. Fixing template streaming filter parameter issues...")
    fix_template_streaming_errors()
    
    print("4. Fixing async I/O type parameter issues...")
    fix_async_io_errors()
    
    print("5. Fixing AST extractor field access issues...")
    fix_ast_extractor_errors()
    
    print("6. Fixing LSP enhanced symbols type conversion issues...")
    fix_lsp_enhanced_symbols_errors()
    
    print("7. Adding missing imports and default implementations...")
    add_missing_imports_and_defaults()
    
    print("All E0308 fixes completed!")

if __name__ == "__main__":
    main()
