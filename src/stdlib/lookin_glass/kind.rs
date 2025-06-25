/// Kind represents the specific kind of type that a Type represents
use std::fmt;

/// Describes the specific kind of type that a Type represents
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Kind {
impl Kind {
    /// Get the string representation of the kind
    pub fn as_str(&self) -> &'static str {
        match self {
        }
    }

    /// Check if this kind represents a basic type
    pub fn is_basic(&self) -> bool {
            Kind::Bool | Kind::Int | Kind::Int8 | Kind::Int16 | Kind::Int32 | Kind::Int64 |
            Kind::Uint | Kind::Uint8 | Kind::Uint16 | Kind::Uint32 | Kind::Uint64 | Kind::Uintptr |
            Kind::Float32 | Kind::Float64 | Kind::Complex64 | Kind::Complex128 | Kind::String
        )
    /// Check if this kind represents a signed integer
    pub fn is_signed_int(&self) -> bool {
        matches!(self, Kind::Int | Kind::Int8 | Kind::Int16 | Kind::Int32 | Kind::Int64)
    /// Check if this kind represents an unsigned integer
    pub fn is_unsigned_int(&self) -> bool {
        matches!(self, Kind::Uint | Kind::Uint8 | Kind::Uint16 | Kind::Uint32 | Kind::Uint64 | Kind::Uintptr)
    /// Check if this kind represents an integer (signed or unsigned)
    pub fn is_integer(&self) -> bool {
        self.is_signed_int() || self.is_unsigned_int()
    /// Check if this kind represents a floating point number
    pub fn is_float(&self) -> bool {
        matches!(self, Kind::Float32 | Kind::Float64)
    /// Check if this kind represents a complex number
    pub fn is_complex(&self) -> bool {
        matches!(self, Kind::Complex64 | Kind::Complex128)
    /// Check if this kind represents a numeric type
    pub fn is_numeric(&self) -> bool {
        self.is_integer() || self.is_float() || self.is_complex()
    /// Check if this kind represents a composite type
    pub fn is_composite(&self) -> bool {
        matches!(self, Kind::Array | Kind::Slice | Kind::Map | Kind::Struct | Kind::Interface)
    /// Check if this kind represents a reference type
    pub fn is_reference(&self) -> bool {
        matches!(self, Kind::Pointer | Kind::Chan | Kind::Func | Kind::Map | Kind::Slice | Kind::Interface)
    /// Check if this kind can be compared for equality
    pub fn is_comparable(&self) -> bool {
        !matches!(self, Kind::Slice | Kind::Map | Kind::Func)
    /// Check if this kind can be ordered (supports comparison operators)
    pub fn is_ordered(&self) -> bool {
            Kind::Int | Kind::Int8 | Kind::Int16 | Kind::Int32 | Kind::Int64 |
            Kind::Uint | Kind::Uint8 | Kind::Uint16 | Kind::Uint32 | Kind::Uint64 | Kind::Uintptr |
            Kind::Float32 | Kind::Float64 | Kind::String
        )
    /// Get the size in bytes for this kind (for basic types)
    pub fn size(&self) -> Option<usize> {
        match self {
            _ => None, // Composite types have variable size
        }
    }

    /// Get the alignment requirement for this kind
    pub fn align(&self) -> Option<usize> {
        match self {
            Kind::Complex128 => Some(8), // Complex128 aligns to its component size
            _ => None, // Composite types have variable alignment
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
        }
    }
