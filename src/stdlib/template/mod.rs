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

pub use template_core::*;
pub use template_syntax::*;
pub use template_render::*;
pub use template_filters::*;
pub use template_html::*;
pub use template_formats::*;
pub use template_cache::*;
pub use template_web::*;
pub use template_security::*;
pub use template_streaming::*;
pub use template_bundler::*;
