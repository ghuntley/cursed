//! LLVM type system for CURSED

use crate::error_types::Result;

/// LLVM Context for managing compilation state
#[derive(Debug)]
pub struct LlvmContext {
    pub target_triple: String,
    pub modules: Vec<LlvmModule>,
}

impl LlvmContext {
    fn detect_target_triple() -> String {
        // Check for environment variable first
        if let Ok(target) = std::env::var("TARGET") {
            return target;
        }
        
        // Detect current platform with proper target triples for arm64 and x86_64
        cfg_if::cfg_if! {
            if #[cfg(all(target_arch = "aarch64", target_os = "macos"))] {
                "aarch64-apple-darwin".to_string()
            } else if #[cfg(all(target_arch = "aarch64", target_os = "linux"))] {
                "aarch64-unknown-linux-gnu".to_string()
            } else if #[cfg(all(target_arch = "x86_64", target_os = "macos"))] {
                "x86_64-apple-darwin".to_string()
            } else if #[cfg(all(target_arch = "x86_64", target_os = "linux"))] {
                "x86_64-unknown-linux-gnu".to_string()
            } else if #[cfg(all(target_arch = "x86_64", target_os = "windows"))] {
                "x86_64-pc-windows-msvc".to_string()
            } else if #[cfg(all(target_arch = "aarch64", target_os = "windows"))] {
                "aarch64-pc-windows-msvc".to_string()
            } else {
                // Generic fallback for other architectures
                format!("{}-unknown-{}", 
                    std::env::consts::ARCH, 
                    std::env::consts::OS
                )
            }
        }
    }

    pub fn new() -> Self {
        Self {
            target_triple: Self::detect_target_triple(),
            modules: Vec::new(),
        }
    }

    pub fn create_module(&mut self, name: String) -> &mut LlvmModule {
        let module = LlvmModule::new(name);
        self.modules.push(module);
        self.modules.last_mut().unwrap()
    }
}

#[derive(Debug, Clone)]
pub enum LlvmType {
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Bool,
    Pointer(Box<LlvmType>),
    Array(Box<LlvmType>, usize),
    Struct(Vec<LlvmType>),
    Function(Vec<LlvmType>, Box<LlvmType>),
    Void,
    // TestResult type system
    TestResult,
    TestStatus,
    TestSuite,
    TestReport,
}

#[derive(Debug, Clone)]
pub struct LlvmValue {
    pub value_type: LlvmType,
    pub name: String,
}

impl LlvmValue {
    pub fn new(value_type: LlvmType, name: String) -> Self {
        Self { value_type, name }
    }
}

#[derive(Debug, Clone)]
pub struct LlvmFunction {
    pub name: String,
    pub return_type: LlvmType,
    pub parameters: Vec<LlvmType>,
    pub body: Vec<String>, // Simplified body representation
}

impl LlvmFunction {
    pub fn new(name: String, return_type: LlvmType, parameters: Vec<LlvmType>) -> Self {
        Self {
            name,
            return_type,
            parameters,
            body: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct LlvmModule {
    pub name: String,
    pub functions: Vec<LlvmFunction>,
    pub globals: Vec<LlvmValue>,
}

impl LlvmModule {
    pub fn new(name: String) -> Self {
        Self {
            name,
            functions: Vec::new(),
            globals: Vec::new(),
        }
    }

    pub fn add_function(&mut self, function: LlvmFunction) {
        self.functions.push(function);
    }

    pub fn add_global(&mut self, global: LlvmValue) {
        self.globals.push(global);
    }
}

impl LlvmType {
    /// Convert AST Type to LLVM Type
    pub fn from_ast_type(ast_type: &crate::ast::Type) -> Self {
        match ast_type {
            crate::ast::Type::Integer => LlvmType::I32,
            crate::ast::Type::Float => LlvmType::F64,
            crate::ast::Type::String => LlvmType::Pointer(Box::new(LlvmType::I8)),
            crate::ast::Type::Boolean => LlvmType::Bool,
            crate::ast::Type::Void => LlvmType::Void,
            crate::ast::Type::Normie => LlvmType::I32,
            crate::ast::Type::Tea => LlvmType::Pointer(Box::new(LlvmType::I8)),
            crate::ast::Type::Lit => LlvmType::Bool,
            crate::ast::Type::Sip => LlvmType::I8,
            crate::ast::Type::Smol => LlvmType::I8,
            crate::ast::Type::Mid => LlvmType::I16,
            crate::ast::Type::Thicc => LlvmType::I64,
            crate::ast::Type::Snack => LlvmType::F32,
            crate::ast::Type::Meal => LlvmType::F64,
            crate::ast::Type::Byte => LlvmType::I8,
            crate::ast::Type::Rune => LlvmType::I32,
            crate::ast::Type::Extra => LlvmType::Struct(vec![LlvmType::F64, LlvmType::F64]),
            crate::ast::Type::TestResult => LlvmType::TestResult,
            crate::ast::Type::TestStatus => LlvmType::TestStatus,
            crate::ast::Type::TestSuite => LlvmType::TestSuite,
            crate::ast::Type::TestReport => LlvmType::TestReport,
            crate::ast::Type::Array(inner, _) => {
                LlvmType::Array(Box::new(LlvmType::from_ast_type(inner)), 0)
            }
            crate::ast::Type::Pointer(inner) => {
                LlvmType::Pointer(Box::new(LlvmType::from_ast_type(inner)))
            }
            crate::ast::Type::Custom(_) => LlvmType::Pointer(Box::new(LlvmType::I8)),
            _ => LlvmType::Void,
        }
    }
    
    /// Get the LLVM type representation as a string
    pub fn to_llvm_string(&self) -> String {
        match self {
            LlvmType::I8 => "i8".to_string(),
            LlvmType::I16 => "i16".to_string(),
            LlvmType::I32 => "i32".to_string(),
            LlvmType::I64 => "i64".to_string(),
            LlvmType::F32 => "float".to_string(),
            LlvmType::F64 => "double".to_string(),
            LlvmType::Bool => "i1".to_string(),
            LlvmType::Void => "void".to_string(),
            LlvmType::Pointer(inner) => format!("{}*", inner.to_llvm_string()),
            LlvmType::Array(inner, size) => format!("[{} x {}]", size, inner.to_llvm_string()),
            LlvmType::Struct(fields) => {
                let field_types: Vec<String> = fields.iter().map(|f| f.to_llvm_string()).collect();
                format!("{{{}}}", field_types.join(", "))
            }
            LlvmType::Function(params, ret) => {
                let param_types: Vec<String> = params.iter().map(|p| p.to_llvm_string()).collect();
                format!("{} ({})", ret.to_llvm_string(), param_types.join(", "))
            }
            LlvmType::TestResult => "%struct.TestResult".to_string(),
            LlvmType::TestStatus => "i32".to_string(), // enum represented as i32
            LlvmType::TestSuite => "%struct.TestSuite".to_string(),
            LlvmType::TestReport => "%struct.TestReport".to_string(),
        }
    }
    
    /// Get the size of the type in bytes
    pub fn size_in_bytes(&self) -> usize {
        match self {
            LlvmType::I8 => 1,
            LlvmType::I16 => 2,
            LlvmType::I32 => 4,
            LlvmType::I64 => 8,
            LlvmType::F32 => 4,
            LlvmType::F64 => 8,
            LlvmType::Bool => 1,
            LlvmType::Void => 0,
            LlvmType::Pointer(_) => 8, // 64-bit pointer
            LlvmType::Array(inner, size) => inner.size_in_bytes() * size,
            LlvmType::Struct(fields) => fields.iter().map(|f| f.size_in_bytes()).sum(),
            LlvmType::Function(_, _) => 8, // function pointer
            LlvmType::TestResult => 80, // estimated size for TestResult struct
            LlvmType::TestStatus => 4,  // enum as i32
            LlvmType::TestSuite => 120, // estimated size for TestSuite struct
            LlvmType::TestReport => 160, // estimated size for TestReport struct
        }
    }
}

#[cfg(test)]
mod target_triple_tests {
    use super::*;
    
    #[test]
    fn test_target_triple_detection() {
        let context = LlvmContext::new();
        let target_triple = &context.target_triple;
        
        // Verify we get a valid target triple
        assert!(!target_triple.is_empty(), "Target triple should not be empty");
        
        // Check for expected arm64 target triples
        if cfg!(all(target_arch = "aarch64", target_os = "macos")) {
            assert_eq!(target_triple, "aarch64-apple-darwin", 
                "Expected aarch64-apple-darwin for arm64 macOS");
        } else if cfg!(all(target_arch = "aarch64", target_os = "linux")) {
            assert_eq!(target_triple, "aarch64-unknown-linux-gnu", 
                "Expected aarch64-unknown-linux-gnu for arm64 Linux");
        }
        
        // Ensure target triple contains architecture info
        assert!(target_triple.contains("aarch64") || target_triple.contains("x86_64"),
            "Target triple should contain architecture info");
            
        println!("Detected target triple: {}", target_triple);
    }
    
    #[test] 
    fn test_arm64_macos_detection() {
        // This test specifically validates arm64 macOS detection
        #[cfg(all(target_arch = "aarch64", target_os = "macos"))]
        {
            let context = LlvmContext::new();
            assert_eq!(context.target_triple, "aarch64-apple-darwin");
        }
    }
    
    #[test]
    fn test_arm64_linux_detection() {
        // This test validates arm64 Linux detection
        #[cfg(all(target_arch = "aarch64", target_os = "linux"))]
        {
            let context = LlvmContext::new();
            assert_eq!(context.target_triple, "aarch64-unknown-linux-gnu");
        }
    }
}
