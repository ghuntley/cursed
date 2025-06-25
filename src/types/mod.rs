// Types module containing Result, Option, and error pattern utilities
pub mod result;
pub mod literal;
pub mod module;
pub mod line_ending;

// Re-export public types for easier access
pub use result::{
    Result, Option, result_utils, error_patterns
// };
pub use literal::Literal;
pub use module::Module;
pub use line_ending::LineEnding;
