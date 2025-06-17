/// Kind represents the specific kind of type that a Type represents
use std::fmt;

/// Describes the specific kind of type that a Type represents
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Kind {
    Invalid = 0,
    Bool,
    Int,
    Int8,
    Int16,
    Int32,
    Int64,
    Uint,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Uintptr,
    Float32,
    Float64,
    Complex64,
    Complex128,
    Array,
    Chan,
    Func,
    Interface,
    Map,
    Pointer,
    Slice,
    String,
    Struct,
    UnsafePointer,
}

impl Kind {
    /// Get the string representation of the kind
    pub fn as_str(&self) -> &'static str {
        match self {
            Kind::Invalid => "invalid",
            Kind::Bool => "bool",
            Kind::Int => "int",
            Kind::Int8 => "int8",
            Kind::Int16 => "int16",
            Kind::Int32 => "int32",
            Kind::Int64 => "int64",
            Kind::Uint => "uint",
            Kind::Uint8 => "uint8",
            Kind::Uint16 => "uint16",
            Kind::Uint32 => "uint32",
            Kind::Uint64 => "uint64",
            Kind::Uintptr => "uintptr",
            Kind::Float32 => "float32",
            Kind::Float64 => "float64",
            Kind::Complex64 => "complex64",
            Kind::Complex128 => "complex128",
            Kind::Array => "array",
            Kind::Chan => "chan",
            Kind::Func => "func",
            Kind::Interface => "interface",
            Kind::Map => "map",
            Kind::Pointer => "pointer",
            Kind::Slice => "slice",
            Kind::String => "string",
            Kind::Struct => "struct",
            Kind::UnsafePointer => "unsafe_pointer",
        }
    }

    /// Check if this kind represents a basic type
    pub fn is_basic(&self) -> bool {
        matches!(self, 
            Kind::Bool | Kind::Int | Kind::Int8 | Kind::Int16 | Kind::Int32 | Kind::Int64 |
            Kind::Uint | Kind::Uint8 | Kind::Uint16 | Kind::Uint32 | Kind::Uint64 | Kind::Uintptr |
            Kind::Float32 | Kind::Float64 | Kind::Complex64 | Kind::Complex128 | Kind::String
        )
    }

    /// Check if this kind represents a signed integer
    pub fn is_signed_int(&self) -> bool {
        matches!(self, Kind::Int | Kind::Int8 | Kind::Int16 | Kind::Int32 | Kind::Int64)
    }

    /// Check if this kind represents an unsigned integer
    pub fn is_unsigned_int(&self) -> bool {
        matches!(self, Kind::Uint | Kind::Uint8 | Kind::Uint16 | Kind::Uint32 | Kind::Uint64 | Kind::Uintptr)
    }

    /// Check if this kind represents an integer (signed or unsigned)
    pub fn is_integer(&self) -> bool {
        self.is_signed_int() || self.is_unsigned_int()
    }

    /// Check if this kind represents a floating point number
    pub fn is_float(&self) -> bool {
        matches!(self, Kind::Float32 | Kind::Float64)
    }

    /// Check if this kind represents a complex number
    pub fn is_complex(&self) -> bool {
        matches!(self, Kind::Complex64 | Kind::Complex128)
    }

    /// Check if this kind represents a numeric type
    pub fn is_numeric(&self) -> bool {
        self.is_integer() || self.is_float() || self.is_complex()
    }

    /// Check if this kind represents a composite type
    pub fn is_composite(&self) -> bool {
        matches!(self, Kind::Array | Kind::Slice | Kind::Map | Kind::Struct | Kind::Interface)
    }

    /// Check if this kind represents a reference type
    pub fn is_reference(&self) -> bool {
        matches!(self, Kind::Pointer | Kind::Chan | Kind::Func | Kind::Map | Kind::Slice | Kind::Interface)
    }

    /// Check if this kind can be compared for equality
    pub fn is_comparable(&self) -> bool {
        !matches!(self, Kind::Slice | Kind::Map | Kind::Func)
    }

    /// Check if this kind can be ordered (supports comparison operators)
    pub fn is_ordered(&self) -> bool {
        matches!(self, 
            Kind::Int | Kind::Int8 | Kind::Int16 | Kind::Int32 | Kind::Int64 |
            Kind::Uint | Kind::Uint8 | Kind::Uint16 | Kind::Uint32 | Kind::Uint64 | Kind::Uintptr |
            Kind::Float32 | Kind::Float64 | Kind::String
        )
    }

    /// Get the size in bytes for this kind (for basic types)
    pub fn size(&self) -> Option<usize> {
        match self {
            Kind::Bool => Some(1),
            Kind::Int8 | Kind::Uint8 => Some(1),
            Kind::Int16 | Kind::Uint16 => Some(2),
            Kind::Int32 | Kind::Uint32 | Kind::Float32 => Some(4),
            Kind::Int64 | Kind::Uint64 | Kind::Float64 | Kind::Complex64 => Some(8),
            Kind::Complex128 => Some(16),
            Kind::Int | Kind::Uint | Kind::Uintptr => Some(std::mem::size_of::<isize>()),
            Kind::String => Some(std::mem::size_of::<String>()),
            Kind::Pointer | Kind::UnsafePointer => Some(std::mem::size_of::<*const u8>()),
            _ => None, // Composite types have variable size
        }
    }

    /// Get the alignment requirement for this kind
    pub fn align(&self) -> Option<usize> {
        match self {
            Kind::Bool => Some(1),
            Kind::Int8 | Kind::Uint8 => Some(1),
            Kind::Int16 | Kind::Uint16 => Some(2),
            Kind::Int32 | Kind::Uint32 | Kind::Float32 => Some(4),
            Kind::Int64 | Kind::Uint64 | Kind::Float64 | Kind::Complex64 => Some(8),
            Kind::Complex128 => Some(8), // Complex128 aligns to its component size
            Kind::Int | Kind::Uint | Kind::Uintptr => Some(std::mem::align_of::<isize>()),
            Kind::String => Some(std::mem::align_of::<String>()),
            Kind::Pointer | Kind::UnsafePointer => Some(std::mem::align_of::<*const u8>()),
            _ => None, // Composite types have variable alignment
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<u8> for Kind {
    fn from(value: u8) -> Self {
        match value {
            0 => Kind::Invalid,
            1 => Kind::Bool,
            2 => Kind::Int,
            3 => Kind::Int8,
            4 => Kind::Int16,
            5 => Kind::Int32,
            6 => Kind::Int64,
            7 => Kind::Uint,
            8 => Kind::Uint8,
            9 => Kind::Uint16,
            10 => Kind::Uint32,
            11 => Kind::Uint64,
            12 => Kind::Uintptr,
            13 => Kind::Float32,
            14 => Kind::Float64,
            15 => Kind::Complex64,
            16 => Kind::Complex128,
            17 => Kind::Array,
            18 => Kind::Chan,
            19 => Kind::Func,
            20 => Kind::Interface,
            21 => Kind::Map,
            22 => Kind::Pointer,
            23 => Kind::Slice,
            24 => Kind::String,
            25 => Kind::Struct,
            26 => Kind::UnsafePointer,
            _ => Kind::Invalid,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kind_string_representation() {
        assert_eq!(Kind::Bool.as_str(), "bool");
        assert_eq!(Kind::Int32.as_str(), "int32");
        assert_eq!(Kind::String.as_str(), "string");
        assert_eq!(Kind::Struct.as_str(), "struct");
    }

    #[test]
    fn test_kind_properties() {
        assert!(Kind::Int32.is_basic());
        assert!(Kind::Int32.is_signed_int());
        assert!(Kind::Int32.is_integer());
        assert!(Kind::Int32.is_numeric());
        assert!(Kind::Int32.is_comparable());
        assert!(Kind::Int32.is_ordered());

        assert!(Kind::Uint64.is_unsigned_int());
        assert!(Kind::Float64.is_float());
        assert!(Kind::Complex128.is_complex());
        
        assert!(Kind::Struct.is_composite());
        assert!(Kind::Pointer.is_reference());
        
        assert!(!Kind::Slice.is_comparable());
        assert!(!Kind::Map.is_comparable());
        assert!(!Kind::Func.is_comparable());
    }

    #[test]
    fn test_kind_size_and_alignment() {
        assert_eq!(Kind::Bool.size(), Some(1));
        assert_eq!(Kind::Int32.size(), Some(4));
        assert_eq!(Kind::Int64.size(), Some(8));
        assert_eq!(Kind::Complex128.size(), Some(16));
        
        assert_eq!(Kind::Bool.align(), Some(1));
        assert_eq!(Kind::Int32.align(), Some(4));
        assert_eq!(Kind::Int64.align(), Some(8));
        
        // Composite types return None for size/alignment
        assert_eq!(Kind::Struct.size(), None);
        assert_eq!(Kind::Array.size(), None);
    }

    #[test]
    fn test_kind_from_u8() {
        assert_eq!(Kind::from(1), Kind::Bool);
        assert_eq!(Kind::from(5), Kind::Int32);
        assert_eq!(Kind::from(24), Kind::String);
        assert_eq!(Kind::from(255), Kind::Invalid);
    }

    #[test]
    fn test_kind_display() {
        assert_eq!(format!("{}", Kind::Bool), "bool");
        assert_eq!(format!("{}", Kind::Struct), "struct");
    }
}
