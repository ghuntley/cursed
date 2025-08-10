/// fr fr Database migration system - evolving schemas like a boss periodt
///
/// This package provides database schema migration, version management,
/// and rollback capabilities. Schema evolution bestie!

// Core migration modules
pub mod migration;
pub mod runner;
pub mod version;

// Re-export important types
pub use migration::{Migration, MigrationScript};
pub use runner::{MigrationRunner, MigrationStatus};
pub use version::{SchemaVersion, VersionManager};

/// slay Initialize the db_migrate package
pub fn init_db_migrate() -> Result<(), String> {
    println!("🔄 db_migrate package initialized - schema migrations ready bestie!");
    Ok(())
}
