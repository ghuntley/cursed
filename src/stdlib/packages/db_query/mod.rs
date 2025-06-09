/// fr fr Database query builder package - building queries with style periodt
///
/// This package provides advanced query building, query optimization,
/// and query execution planning. Query like a pro bestie!

// Re-export from db_core and db_sql
pub use crate::stdlib::packages::db_core::query::*;
pub use crate::stdlib::packages::db_sql::builder::*;

/// slay Initialize the db_query package
pub fn init_db_query() -> crate::stdlib::packages::db_core::error::DatabaseResult<()> {
    println!("🔍 db_query package initialized - query building ready bestie!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_db_query() {
        assert!(init_db_query().is_ok());
    }
}
