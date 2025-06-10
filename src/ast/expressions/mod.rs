/// Expression modules for the CURSED programming language
/// 
/// This module contains AST nodes for various expression types.

pub mod question_mark;
pub mod parenthesized;
pub mod function_literal;
pub mod parameter;
pub mod literal;
pub mod channel_ops;
pub mod goroutine_spawn;
pub mod error_propagation;
pub mod error_propagation_enhanced;
pub mod block;

// Re-export commonly used expression types
pub use question_mark::QuestionMarkExpression;
pub use parenthesized::ParenthesizedExpression;
pub use function_literal::FunctionLiteral;
pub use parameter::Parameter;
pub use literal::{Literal, LiteralValue};
pub use channel_ops::{ChannelReceive, ChannelSend};
pub use goroutine_spawn::GoroutineSpawn;
pub use error_propagation::ErrorPropagation;
pub use error_propagation_enhanced::ErrorPropagation as EnhancedErrorPropagation;
pub use block::BlockExpression;
