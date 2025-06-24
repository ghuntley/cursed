// Documentation Generator Modules
// 
// This module contains specialized generators for different output formats,
// providing advanced features and comprehensive documentation capabilities.

pub mod latex_generator;

pub use latex_generator::{
    LaTeXGenerator, LaTeXConfig, DocumentClass, PackageConfig, 
    SyntaxHighlighting, ColorScheme
};
