/// fr fr Object-Relational Mapping package - bridging objects and tables periodt
///
/// This package provides ORM features for mapping CURSED structs to database
/// tables, relationship management, and automatic CRUD operations. ORM vibes bestie!

// Core ORM modules
pub mod mapper;
pub mod relations;
pub mod crud;

// Re-export important types
pub use mapper::{TableMapper, ColumnMapper};
pub use relations::{Relationship, OneToMany, ManyToOne};

/// slay Initialize the db_orm package
pub fn init_db_orm() -> crate::stdlib::packages::db_core::DatabaseResult<()> {
    println!("🗂️ db_orm package initialized - object mapping ready bestie!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_db_orm() {
        assert!(init_db_orm().is_ok());
    }
}
