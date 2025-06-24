// Minimal runtime module for CURSED minimal build

pub mod stack;
pub mod value;

pub use stack::RuntimeStack;
pub use value::{ValueManager, CursedValue};
