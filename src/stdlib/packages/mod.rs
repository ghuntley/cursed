/// fr fr Packages module for CURSED stdlib - modular library organization
pub mod web_vibez;
pub mod sql_vibes;
pub mod test_vibes;

// Re-export package modules for convenience
pub use web_vibez::*;
pub use sql_vibes::*;
pub use test_vibes::*;
