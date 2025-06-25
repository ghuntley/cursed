// LLVM-based code generation for CURSED
use crate::error_types::CursedError;

pub mod performance_monitor;
pub mod template;
pub mod optimization;

// Re-export main types
pub use performance_monitor::{
    PerformanceMonitor, MonitoringConfig, CodeMetrics, BaselineMetrics, PerformanceReport
};
pub use template::{
    CompiledTemplate, CompiledTemplateMetadata, TemplateCompilationStats, 
    TemplateCompilationError, declare_template_runtime_functions, 
    register_standard_filters, runtime
};
pub use optimization::{LlvmOptimizer, LlvmPassManager, LlvmPassManagerConfig};

/// LLVM type system for code generation
#[derive(Debug, Clone, PartialEq)]
pub enum LlvmType {
    Void,
    Bool,
    Int(u32),      // bit width
    Float(u32),    // bit width
    Pointer(Box<LlvmType>),
    Array(Box<LlvmType>, usize),
    Struct(Vec<LlvmType>),
    Function {
        params: Vec<LlvmType>,
        return_type: Box<LlvmType>,
        is_var_arg: bool,
    },
}

impl LlvmType {
    pub fn i1() -> Self {
        Self::Bool
    }

    pub fn i8() -> Self {
        Self::Int(8)
    }

    pub fn i16() -> Self {
        Self::Int(16)
    }

    pub fn i32() -> Self {
        Self::Int(32)
    }

    pub fn i64() -> Self {
        Self::Int(64)
    }

    pub fn f32() -> Self {
        Self::Float(32)
    }

    pub fn f64() -> Self {
        Self::Float(64)
    }

    pub fn ptr(inner: LlvmType) -> Self {
        Self::Pointer(Box::new(inner))
    }

    pub fn void_ptr() -> Self {
        Self::Pointer(Box::new(Self::Void))
    }

    pub fn array(element_type: LlvmType, size: usize) -> Self {
        Self::Array(Box::new(element_type), size)
    }

    pub fn function(params: Vec<LlvmType>, return_type: LlvmType) -> Self {
        Self::Function {
            params,
            return_type: Box::new(return_type),
            is_var_arg: false,
        }
    }

    pub fn var_arg_function(params: Vec<LlvmType>, return_type: LlvmType) -> Self {
        Self::Function {
            params,
            return_type: Box::new(return_type),
            is_var_arg: true,
        }
    }

    pub fn to_llvm_string(&self) -> String {
        match self {
            LlvmType::Void => "void".to_string(),
            LlvmType::Bool => "i1".to_string(),
            LlvmType::Int(bits) => format!("i{}", bits),
            LlvmType::Float(32) => "float".to_string(),
            LlvmType::Float(64) => "double".to_string(),
            LlvmType::Float(bits) => format!("f{}", bits),
            LlvmType::Pointer(inner) => format!("{}*", inner.to_llvm_string()),
            LlvmType::Array(element_type, size) => format!("[{} x {}]", size, element_type.to_llvm_string()),
            LlvmType::Struct(fields) => {
                let field_types: Vec<String> = fields.iter().map(|t| t.to_llvm_string()).collect();
                format!("{{ {} }}", field_types.join(", "))
            }
            LlvmType::Function { params, return_type, is_var_arg } => {
                let param_types: Vec<String> = params.iter().map(|t| t.to_llvm_string()).collect();
                let var_arg_suffix = if *is_var_arg { ", ..." } else { "" };
                format!("{}({}{})", return_type.to_llvm_string(), param_types.join(", "), var_arg_suffix)
            }
        }
    }

    pub fn size_in_bits(&self) -> Option<u32> {
        match self {
            LlvmType::Void => Some(0),
            LlvmType::Bool => Some(1),
            LlvmType::Int(bits) => Some(*bits),
            LlvmType::Float(bits) => Some(*bits),
            LlvmType::Pointer(_) => Some(64), // Assume 64-bit pointers
            LlvmType::Array(element_type, size) => {
                element_type.size_in_bits().map(|elem_size| elem_size * (*size as u32))
            }
            LlvmType::Struct(fields) => {
                let mut total_size = 0u32;
                for field in fields {
                    if let Some(field_size) = field.size_in_bits() {
                        total_size += field_size;
                    } else {
                        return None; // Unknown size
                    }
                }
                Some(total_size)
            }
            LlvmType::Function { .. } => Some(64), // Function pointer size
        }
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, LlvmType::Bool | LlvmType::Int(_))
    }

    pub fn is_floating_point(&self) -> bool {
        matches!(self, LlvmType::Float(_))
    }

    pub fn is_pointer(&self) -> bool {
        matches!(self, LlvmType::Pointer(_))
    }

    pub fn is_aggregate(&self) -> bool {
        matches!(self, LlvmType::Array(_, _) | LlvmType::Struct(_))
    }
}

/// Expression compiler for LLVM
pub mod expression_compiler {
    use super::*;

    pub use super::LlvmType;

    /// LLVM expression compiler
    #[derive(Debug)]
    pub struct LlvmExpressionCompiler {
        pub target_triple: String,
        pub optimization_level: u8,
    }

    impl LlvmExpressionCompiler {
        pub fn new() -> Self {
            Self {
                target_triple: "x86_64-unknown-linux-gnu".to_string(),
                optimization_level: 0,
            }
        }

        pub fn compile_expression(&self, _expr: &str) -> crate::error_types::Result<String> {
            // TODO: Implement expression compilation
            Ok("%0 = add i32 0, 0".to_string())
        }
    }

    impl Default for LlvmExpressionCompiler {
        fn default() -> Self {
            Self::new()
        }
    }
}

/// Main LLVM code generator
#[derive(Debug)]
pub struct LlvmCodeGenerator {
    pub target_triple: String,
    pub optimization_level: u8,
    pub performance_monitor: PerformanceMonitor,
    pub template_system: template::runtime::TemplateRuntime,
}

impl LlvmCodeGenerator {
    pub fn new() -> crate::error_types::Result<Self> {
        Ok(Self {
            target_triple: "x86_64-unknown-linux-gnu".to_string(),
            optimization_level: 0,
            performance_monitor: PerformanceMonitor::new(),
            template_system: template::runtime::TemplateRuntime::new(),
        })
    }

    pub fn compile(&mut self, source: &str) -> crate::error_types::Result<String> {
        self.performance_monitor.start();
        
        // TODO: Implement actual compilation
        let ir = format!(
            r#"target triple = "{}"

define i32 @main() {{
entry:
  ret i32 0
}}
"#,
            self.target_triple
        );

        Ok(ir)
    }

    pub fn enable_debug_optimizations(&mut self) -> crate::error_types::Result<()> {
        self.optimization_level = 1;
        Ok(())
    }

    pub fn enable_release_optimizations(&mut self) -> crate::error_types::Result<()> {
        self.optimization_level = 2;
        Ok(())
    }
}

impl Default for LlvmCodeGenerator {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
