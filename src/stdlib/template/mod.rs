/// CURSED Template Engine - Full-featured templating system with Gen Z slang
/// 
/// A comprehensive templating engine that provides:
/// - CURSED-native syntax with Gen Z slang keywords
/// - Template parsing, compilation, and AST generation
/// - Variable interpolation and expression evaluation
/// - Control flow constructs (loops, conditionals)
/// - Template inheritance and composition
/// - Filters and functions for data transformation
/// - HTML/XML template support with auto-escaping
/// - Custom template formats (email, markdown, configuration files)
/// - Template caching and performance optimization
/// - Web framework integration

pub mod template_core;
pub mod template_syntax;
pub mod template_render;
pub mod template_filters;
pub mod template_html;
pub mod template_formats;
pub mod template_cache;
pub mod template_web;
pub mod template_security;
pub mod template_streaming;
pub mod template_bundler;

// Use modules to avoid ambiguous glob re-exports
pub mod core {
    pub use super::template_core::*;
}
pub mod syntax {
    pub use super::template_syntax::*;
}
pub mod render {
    pub use super::template_render::*;
}
pub mod filters {
    pub use super::template_filters::*;
}
pub mod html {
    pub use super::template_html::*;
}
pub mod formats {
    pub use super::template_formats::*;
}
pub mod cache {
    pub use super::template_cache::*;
}
pub mod web {
    pub use super::template_web::*;
}
pub mod security {
    pub use super::template_security::*;
}
pub mod streaming {
    pub use super::template_streaming::*;
}
pub mod bundler {
    pub use super::template_bundler::*;
}
