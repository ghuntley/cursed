#!/usr/bin/env python3

import re
import os

def fix_llvm_integration_issues():
    """Fix LLVM integration issues in error_propagation.rs and related files"""
    
    # Fix error_propagation.rs API issues
    error_prop_file = "src/codegen/llvm/error_propagation.rs"
    
    if os.path.exists(error_prop_file):
        with open(error_prop_file, 'r') as f:
            content = f.read()
        
        # Fix builder field access -> method calls
        content = re.sub(r'self\.builder\.', 'self.builder().', content)
        
        # Fix context field access -> add proper method calls
        content = re.sub(r'self\.context\.', 'self.get_context().', content)
        
        # Fix module field access -> add proper method calls  
        content = re.sub(r'self\.module\.', 'self.get_module().', content)
        
        # Fix location field access -> method calls
        content = re.sub(r'expr\.location\.', 'expr.location().', content)
        
        # Fix inner_expression field access
        content = re.sub(r'expr\.inner_expression', 'expr.expression', content)
        
        # Fix current_function field access
        content = re.sub(r'self\.current_function', 'self.get_current_function()', content)
        
        # Fix get_element_type method call
        content = re.sub(r'\.get_element_type\(\)', '.get_pointee_type().unwrap()', content)
        
        # Fix type mismatches in infer_value_type calls
        content = re.sub(
            r'self\.infer_value_type\(&check_result\.error_value\)',
            'self.infer_value_type(&check_result.error_value.into_pointer_value())',
            content
        )
        
        with open(error_prop_file, 'w') as f:
            f.write(content)
        
        print(f"✅ Fixed LLVM API issues in {error_prop_file}")
    
    # Fix parser error_propagation.rs issues
    parser_error_file = "src/parser/error_propagation.rs"
    
    if os.path.exists(parser_error_file):
        with open(parser_error_file, 'r') as f:
            content = f.read()
        
        # Fix Parser::new calls that need a lexer
        content = re.sub(
            r'Parser::new\(/\* test lexer \*/\)',
            'Parser::new(crate::lexer::Lexer::new("").unwrap()).unwrap()',
            content
        )
        
        # Fix parser method calls on Result
        content = re.sub(
            r'assert!\(parser\.is_propagatable_type',
            'assert!(parser.is_propagatable_type',
            content
        )
        content = re.sub(
            r'assert!\(!parser\.is_propagatable_type',
            'assert!(!parser.is_propagatable_type',
            content
        )
        
        with open(parser_error_file, 'w') as f:
            f.write(content)
        
        print(f"✅ Fixed parser issues in {parser_error_file}")
    
    # Fix runtime error_propagation.rs type annotation issues
    runtime_error_file = "src/runtime/error_propagation.rs"
    
    if os.path.exists(runtime_error_file):
        with open(runtime_error_file, 'r') as f:
            content = f.read()
        
        # Fix type annotations for CursedResult
        content = re.sub(
            r'let result = CursedResult::Ok\(42\);',
            'let result: types::result::Result<i32, String> = CursedResult::Ok(42);',
            content
        )
        content = re.sub(
            r'let result = CursedResult::Err\("test error"\.to_string\(\)\);',
            'let result: types::result::Result<i32, String> = CursedResult::Err("test error".to_string());',
            content
        )
        
        with open(runtime_error_file, 'w') as f:
            f.write(content)
        
        print(f"✅ Fixed type annotations in {runtime_error_file}")

def add_missing_methods_to_llvm_generator():
    """Add missing methods to LlvmCodeGenerator"""
    
    llvm_file = "src/codegen/llvm.rs"
    
    with open(llvm_file, 'r') as f:
        content = f.read()
    
    # Add context and module fields to struct
    struct_pattern = r'(pub struct LlvmCodeGenerator \{[^}]+)'
    if 'context:' not in content and 'module:' not in content:
        replacement = r'\1    context: DummyContext,\n    module: DummyModule,\n    current_function: Option<DummyFunction>,\n'
        content = re.sub(struct_pattern, replacement, content, flags=re.DOTALL)
    
    # Add missing methods
    missing_methods = '''
    
    /// Get LLVM context reference
    pub fn get_context(&self) -> &DummyContext {
        &self.context
    }
    
    /// Get LLVM module reference  
    pub fn get_module(&self) -> &DummyModule {
        &self.module
    }
    
    /// Get current function reference
    pub fn get_current_function(&self) -> Option<&DummyFunction> {
        self.current_function.as_ref()
    }
    
    /// Set current function
    pub fn set_current_function(&mut self, function: DummyFunction) {
        self.current_function = Some(function);
    }
'''
    
    # Add methods before the closing impl block
    if 'get_context' not in content:
        content = content.replace('}\n\nstruct DummyContext', missing_methods + '}\n\nstruct DummyContext')
    
    # Update constructor to initialize new fields
    content = re.sub(
        r'(\s+)debug_generator: LlvmDebugCodeGenerator::new\(DebugConfig::default\(\)\),\s*module_name: None,',
        r'\1debug_generator: LlvmDebugCodeGenerator::new(DebugConfig::default()),\n\1module_name: None,\n\1context: DummyContext::new(),\n\1module: DummyModule::new(),\n\1current_function: None,',
        content
    )
    
    content = re.sub(
        r'(\s+)debug_generator: LlvmDebugCodeGenerator::new\(debug_config\),\s*module_name: None,',
        r'\1debug_generator: LlvmDebugCodeGenerator::new(debug_config),\n\1module_name: None,\n\1context: DummyContext::new(),\n\1module: DummyModule::new(),\n\1current_function: None,',
        content
    )
    
    with open(llvm_file, 'w') as f:
        f.write(content)
    
    print(f"✅ Added missing methods to LlvmCodeGenerator")

def add_dummy_types():
    """Add missing dummy types for LLVM integration"""
    
    llvm_file = "src/codegen/llvm.rs"
    
    with open(llvm_file, 'r') as f:
        content = f.read()
    
    # Add DummyFunction if not present
    if 'struct DummyFunction' not in content:
        dummy_function = '''
#[derive(Debug, Clone)]
pub struct DummyFunction {
    name: String,
}

impl DummyFunction {
    pub const fn new() -> Self {
        Self { name: String::new() }
    }
    
    pub fn with_name(name: String) -> Self {
        Self { name }
    }
}
'''
        content = content.replace('struct DummyContext', dummy_function + '\nstruct DummyContext')
    
    with open(llvm_file, 'w') as f:
        f.write(content)
    
    print(f"✅ Added DummyFunction type")

def fix_debug_builder_error():
    """Fix BuilderError conversion issue in debug.rs"""
    
    debug_file = "src/codegen/llvm/debug.rs"
    
    if os.path.exists(debug_file):
        with open(debug_file, 'r') as f:
            content = f.read()
        
        # Fix BuilderError conversion
        content = re.sub(
            r'\?\s*;\s*$/gm',
            '.map_err(|e| Error::General(format!("Builder error: {:?}", e)))?;',
            content
        )
        
        # Or more simply, replace the error handling
        content = re.sub(
            r'(\s+)\)\?;',
            r'\1).map_err(|e| Error::General(format!("Builder error: {:?}", e)))?;',
            content
        )
        
        with open(debug_file, 'w') as f:
            f.write(content)
        
        print(f"✅ Fixed BuilderError conversion in {debug_file}")

def fix_debug_lifetime_issues():
    """Fix module lifetime issues in debug files"""
    
    debug_files = [
        "src/codegen/llvm/debug.rs",
        "src/codegen/llvm/debug_info.rs"
    ]
    
    for debug_file in debug_files:
        if os.path.exists(debug_file):
            with open(debug_file, 'r') as f:
                content = f.read()
            
            # Fix lifetime issues by making module owned in tests
            content = re.sub(
                r'let module = context\.create_module\("([^"]+)"\);',
                r'let module = Box::new(context.create_module("\1"));',
                content
            )
            
            content = re.sub(
                r'&module',
                r'module.as_ref()',
                content
            )
            
            with open(debug_file, 'w') as f:
                f.write(content)
            
            print(f"✅ Fixed lifetime issues in {debug_file}")

if __name__ == "__main__":
    print("🔧 Fixing LLVM integration issues...")
    
    fix_llvm_integration_issues()
    add_missing_methods_to_llvm_generator()
    add_dummy_types()
    fix_debug_builder_error()
    fix_debug_lifetime_issues()
    
    print("✅ LLVM integration fixes completed!")
