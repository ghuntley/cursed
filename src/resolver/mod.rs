//! Package Resolution System
//! 
//! This module provides package dependency resolution, import path resolution,
//! and package loading for the CURSED language.

pub mod package_resolver;
pub mod dependency_tracker;
pub mod package_loader;
pub mod standard_library;
pub mod symbol_table;
pub mod errors;

pub use package_resolver::PackageResolver;
pub use dependency_tracker::DependencyTracker;
pub use package_loader::PackageLoader;
pub use standard_library::StandardLibraryResolver;
pub use symbol_table::{PackageSymbolTable, Symbol, SymbolKind};
pub use errors::{ResolverError, ResolverResult};
