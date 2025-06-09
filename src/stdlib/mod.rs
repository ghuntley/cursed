/// Standard library for CURSED
pub mod dot_registry;
pub mod packages;
pub mod web_vibez;
pub mod http_core;
pub mod database;

// Database package re-exports for easy access
pub use database::llvm_integration::{
    DatabaseLLVMIntegration, DatabaseLLVMIntegrationImpl, 
    register_database_functions
};

// Package re-exports
pub use packages::{
    db_core, db_pool, sql_vibes, db_migrate, db_orm, db_nosql, db_query
};

pub use dot_registry::DOT_REGISTRY;
