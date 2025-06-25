// Type system utilities and definitions
use crate::error_types::CursedError;

/// Module for result types
pub mod result {
    use super::*;

    /// Result type expression for type system
    #[derive(Debug, Clone, PartialEq)]
    pub enum ResultTypeExpression {
        Ok(Box<TypeExpression>),
        Err(Box<TypeExpression>),
        Result {
            ok_type: Box<TypeExpression>,
            err_type: Box<TypeExpression>,
        },
    }

    impl ResultTypeExpression {
        pub fn new_ok(ok_type: TypeExpression) -> Self {
            Self::Ok(Box::new(ok_type))
        }

        pub fn new_err(err_type: TypeExpression) -> Self {
            Self::Err(Box::new(err_type))
        }

        pub fn new_result(ok_type: TypeExpression, err_type: TypeExpression) -> Self {
            Self::Result {
                ok_type: Box::new(ok_type),
                err_type: Box::new(err_type),
            }
        }

        pub fn get_ok_type(&self) -> Option<&TypeExpression> {
            match self {
                Self::Ok(t) => Some(t),
                Self::Result { ok_type, .. } => Some(ok_type),
                _ => None,
            }
        }

        pub fn get_err_type(&self) -> Option<&TypeExpression> {
            match self {
                Self::Err(t) => Some(t),
                Self::Result { err_type, .. } => Some(err_type),
                _ => None,
            }
        }
    }

    /// Option type expression for type system
    #[derive(Debug, Clone, PartialEq)]
    pub enum OptionTypeExpression {
        Some(Box<TypeExpression>),
        None,
        Option(Box<TypeExpression>),
    }

    impl OptionTypeExpression {
        pub fn new_some(inner_type: TypeExpression) -> Self {
            Self::Some(Box::new(inner_type))
        }

        pub fn new_none() -> Self {
            Self::None
        }

        pub fn new_option(inner_type: TypeExpression) -> Self {
            Self::Option(Box::new(inner_type))
        }

        pub fn get_inner_type(&self) -> Option<&TypeExpression> {
            match self {
                Self::Some(t) => Some(t),
                Self::Option(t) => Some(t),
                _ => None,
            }
        }

        pub fn is_some(&self) -> bool {
            matches!(self, Self::Some(_))
        }

        pub fn is_none(&self) -> bool {
            matches!(self, Self::None)
        }
    }
}

/// Generic type expression
#[derive(Debug, Clone, PartialEq)]
pub enum TypeExpression {
    // Primitive types
    Bool,
    Int(IntSize),
    UInt(IntSize),
    Float(FloatSize),
    Char,
    String,

    // Compound types
    Array(Box<TypeExpression>, Option<usize>),
    Tuple(Vec<TypeExpression>),
    Struct(String, Vec<(String, TypeExpression)>),
    Enum(String, Vec<(String, Option<TypeExpression>)>),
    Union(String, Vec<(String, TypeExpression)>),

    // Function types
    Function {
        params: Vec<TypeExpression>,
        return_type: Box<TypeExpression>,
    },

    // Generic and template types
    Generic(String, Vec<TypeExpression>),
    TypeVariable(String),
    Constraint(String, Box<TypeExpression>),

    // Special types
    Void,
    Never,
    Any,
    Unknown,

    // Reference types
    Reference(Box<TypeExpression>),
    MutableReference(Box<TypeExpression>),
    Pointer(Box<TypeExpression>),

    // Result and Option types
    Result(result::ResultTypeExpression),
    Option(result::OptionTypeExpression),
}

impl TypeExpression {
    pub fn is_primitive(&self) -> bool {
        matches!(self, 
            TypeExpression::Bool |
            TypeExpression::Int(_) |
            TypeExpression::UInt(_) |
            TypeExpression::Float(_) |
            TypeExpression::Char |
            TypeExpression::String
        )
    }

    pub fn is_numeric(&self) -> bool {
        matches!(self, 
            TypeExpression::Int(_) |
            TypeExpression::UInt(_) |
            TypeExpression::Float(_)
        )
    }

    pub fn is_reference(&self) -> bool {
        matches!(self, 
            TypeExpression::Reference(_) |
            TypeExpression::MutableReference(_) |
            TypeExpression::Pointer(_)
        )
    }

    pub fn size_hint(&self) -> Option<usize> {
        match self {
            TypeExpression::Bool => Some(1),
            TypeExpression::Int(size) => Some(size.byte_size()),
            TypeExpression::UInt(size) => Some(size.byte_size()),
            TypeExpression::Float(size) => Some(size.byte_size()),
            TypeExpression::Char => Some(4), // UTF-32
            TypeExpression::Reference(_) | 
            TypeExpression::MutableReference(_) | 
            TypeExpression::Pointer(_) => Some(8), // 64-bit pointer
            TypeExpression::Void => Some(0),
            _ => None, // Variable size or unknown
        }
    }
}

/// Integer size specifications
#[derive(Debug, Clone, PartialEq)]
pub enum IntSize {
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize, // Platform-dependent
}

impl IntSize {
    pub fn byte_size(&self) -> usize {
        match self {
            IntSize::I8 => 1,
            IntSize::I16 => 2,
            IntSize::I32 => 4,
            IntSize::I64 => 8,
            IntSize::I128 => 16,
            IntSize::ISize => 8, // Assume 64-bit platform
        }
    }

    pub fn bit_size(&self) -> usize {
        self.byte_size() * 8
    }
}

/// Float size specifications
#[derive(Debug, Clone, PartialEq)]
pub enum FloatSize {
    F32,
    F64,
    F128,
}

impl FloatSize {
    pub fn byte_size(&self) -> usize {
        match self {
            FloatSize::F32 => 4,
            FloatSize::F64 => 8,
            FloatSize::F128 => 16,
        }
    }

    pub fn bit_size(&self) -> usize {
        self.byte_size() * 8
    }
}

/// Type compatibility checker
#[derive(Debug)]
pub struct TypeChecker {
    pub strict_mode: bool,
    pub allow_implicit_conversions: bool,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            strict_mode: false,
            allow_implicit_conversions: true,
        }
    }

    pub fn strict() -> Self {
        Self {
            strict_mode: true,
            allow_implicit_conversions: false,
        }
    }

    pub fn are_compatible(&self, left: &TypeExpression, right: &TypeExpression) -> bool {
        if left == right {
            return true;
        }

        if !self.allow_implicit_conversions {
            return false;
        }

        // Check for implicit conversions
        self.can_implicitly_convert(left, right) || self.can_implicitly_convert(right, left)
    }

    fn can_implicitly_convert(&self, from: &TypeExpression, to: &TypeExpression) -> bool {
        match (from, to) {
            // Numeric promotions
            (TypeExpression::Int(from_size), TypeExpression::Int(to_size)) => {
                from_size.byte_size() <= to_size.byte_size()
            }
            (TypeExpression::UInt(from_size), TypeExpression::UInt(to_size)) => {
                from_size.byte_size() <= to_size.byte_size()
            }
            (TypeExpression::Float(from_size), TypeExpression::Float(to_size)) => {
                from_size.byte_size() <= to_size.byte_size()
            }
            (TypeExpression::Int(_), TypeExpression::Float(_)) => true,
            (TypeExpression::UInt(_), TypeExpression::Float(_)) => true,

            // Reference conversions
            (TypeExpression::Reference(inner), to) => self.can_implicitly_convert(inner, to),
            (from, TypeExpression::Reference(inner)) => self.can_implicitly_convert(from, inner),

            // Any type conversions
            (_, TypeExpression::Any) => true,
            (TypeExpression::Unknown, _) => true,

            _ => false,
        }
    }

    pub fn unify(&self, left: &TypeExpression, right: &TypeExpression) -> Option<TypeExpression> {
        if left == right {
            return Some(left.clone());
        }

        match (left, right) {
            // Numeric unification
            (TypeExpression::Int(l), TypeExpression::Int(r)) => {
                let size = if l.byte_size() >= r.byte_size() { l } else { r };
                Some(TypeExpression::Int(size.clone()))
            }
            (TypeExpression::UInt(l), TypeExpression::UInt(r)) => {
                let size = if l.byte_size() >= r.byte_size() { l } else { r };
                Some(TypeExpression::UInt(size.clone()))
            }
            (TypeExpression::Float(l), TypeExpression::Float(r)) => {
                let size = if l.byte_size() >= r.byte_size() { l } else { r };
                Some(TypeExpression::Float(size.clone()))
            }

            // Mixed numeric types promote to float
            (TypeExpression::Int(_) | TypeExpression::UInt(_), TypeExpression::Float(size)) => {
                Some(TypeExpression::Float(size.clone()))
            }
            (TypeExpression::Float(size), TypeExpression::Int(_) | TypeExpression::UInt(_)) => {
                Some(TypeExpression::Float(size.clone()))
            }

            // Reference unification
            (TypeExpression::Reference(l), TypeExpression::Reference(r)) => {
                self.unify(l, r).map(|t| TypeExpression::Reference(Box::new(t)))
            }

            // Fallback to Any type
            _ if !self.strict_mode => Some(TypeExpression::Any),
            
            _ => None,
        }
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}
