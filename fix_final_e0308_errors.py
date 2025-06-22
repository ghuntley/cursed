#!/usr/bin/env python3

import os

def fix_session_timeout_final():
    """Fix remaining session timeout async issues"""
    session_timeout_path = "src/stdlib/web_vibez/session_timeout.rs"
    
    with open(session_timeout_path, 'r') as f:
        content = f.read()
    
    # Look for the specific problematic timeout_middleware calls
    # These need to be properly awaited and the async context fixed
    
    # Fix specific instances by adding .await to the end properly
    content = content.replace(
        ').await\n    }',
        ').await?\n    }'
    )
    
    with open(session_timeout_path, 'w') as f:
        f.write(content)

def fix_async_io_result_type():
    """Fix async I/O result type mismatch"""
    async_io_path = "src/stdlib/async/io.rs"
    
    with open(async_io_path, 'r') as f:
        content = f.read()
    
    # Fix the spawn_blocking_io call to match expected Result type
    content = content.replace(
        'spawn_blocking_io(f).await',
        'spawn_blocking_io(f).await.map_err(|e| crate::error::CursedError::General(e.to_string()))'
    )
    
    with open(async_io_path, 'w') as f:
        f.write(content)

def fix_ast_extractor_location():
    """Fix AST extractor location handling"""
    extractor_path = "src/documentation/extractors/ast_extractor.rs"
    
    with open(extractor_path, 'r') as f:
        content = f.read()
    
    # Fix interface location access
    content = content.replace(
        'interface_decl.location.clone()',
        'interface_decl.location.clone().unwrap_or_default()'
    )
    
    with open(extractor_path, 'w') as f:
        f.write(content)

def fix_live_server_join_handle():
    """Fix live server JoinHandle type mismatch"""
    live_server_path = "src/documentation/live_server.rs"
    
    with open(live_server_path, 'r') as f:
        content = f.read()
    
    # Fix JoinHandle type by mapping the result
    content = content.replace(
        '*queue = Some(generation_task);',
        '*queue = Some(tokio::spawn(async move { let _ = generation_task.await; }));'
    )
    
    with open(live_server_path, 'w') as f:
        f.write(content)

def fix_lsp_symbols_type_conversion():
    """Fix LSP symbols type conversion issues"""
    lsp_path = "src/lsp/enhanced_symbols.rs"
    
    with open(lsp_path, 'r') as f:
        content = f.read()
    
    # Add conversion functions for ImportStatement/ImportDeclaration
    conversion_functions = '''
    fn convert_import_statement_to_declaration(&self, import_stmt: &ImportStatement) -> ImportDeclaration {
        ImportDeclaration {
            module_path: import_stmt.module_path.clone(),
            items: import_stmt.items.clone(),
            location: import_stmt.location.clone(),
        }
    }

    fn convert_package_statement_to_declaration(&self, package_stmt: &PackageStatement) -> PackageDeclaration {
        PackageDeclaration {
            name: package_stmt.name.clone(),
            version: package_stmt.version.clone(),
            location: package_stmt.location.clone(),
        }
    }
'''
    
    # Add conversion functions if not already present
    if 'convert_import_statement_to_declaration' not in content:
        content = content.replace(
            'impl EnhancedSymbolProvider {',
            f'impl EnhancedSymbolProvider {{{conversion_functions}'
        )
    
    # Fix method calls
    content = content.replace(
        'self.create_import_symbol(import_decl)',
        'self.create_import_symbol(&self.convert_import_statement_to_declaration(import_decl))'
    )
    
    content = content.replace(
        'self.create_package_symbol(package_decl)',
        'self.create_package_symbol(&self.convert_package_statement_to_declaration(package_decl))'
    )
    
    with open(lsp_path, 'w') as f:
        f.write(content)

def add_missing_types():
    """Add missing type definitions for conversions"""
    
    # Add ImportDeclaration if missing
    import_decl_content = '''
#[derive(Debug, Clone)]
pub struct ImportDeclaration {
    pub module_path: String,
    pub items: Vec<String>,
    pub location: Option<SourceLocation>,
}

#[derive(Debug, Clone)]
pub struct PackageDeclaration {
    pub name: String,
    pub version: Option<String>, 
    pub location: Option<SourceLocation>,
}
'''
    
    # Check if we need to add these to core_types
    core_types_path = "src/types/core_types.rs"
    if os.path.exists(core_types_path):
        with open(core_types_path, 'r') as f:
            content = f.read()
        
        if 'ImportDeclaration' not in content:
            content += import_decl_content
            
            with open(core_types_path, 'w') as f:
                f.write(content)

def main():
    """Fix remaining 8 E0308 errors"""
    print("Fixing final 8 E0308 type mismatch errors...")
    
    print("1. Fixing session timeout final async issues...")
    fix_session_timeout_final()
    
    print("2. Fixing async I/O result type mismatch...")
    fix_async_io_result_type()
    
    print("3. Fixing AST extractor location handling...")
    fix_ast_extractor_location()
    
    print("4. Fixing live server JoinHandle type...")
    fix_live_server_join_handle()
    
    print("5. Fixing LSP symbols type conversions...")
    fix_lsp_symbols_type_conversion()
    
    print("6. Adding missing type definitions...")
    add_missing_types()
    
    print("All final E0308 fixes completed!")

if __name__ == "__main__":
    main()
