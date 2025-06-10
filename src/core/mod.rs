/// Core language functionality
pub mod type_checker;
pub mod char;

pub use type_checker::{Type, TypeChecker};
pub use char::{CharMethods, CharObject};
