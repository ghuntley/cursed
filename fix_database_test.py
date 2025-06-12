#!/usr/bin/env python3

def fix_database_test():
    """Fix the database integration test to match the actual API."""
    
    with open('tests/database_integration_tests.rs', 'r') as f:
        content = f.read()
    
    # Update imports to match actual database module exports
    content = content.replace(
        'use cursed::stdlib::database::{Database, Connection, QueryResult};',
        'use cursed::stdlib::database::{DB, Conn, QueryResult, SqliteDriver, DriverConn};'
    )
    
    content = content.replace(
        'use cursed::stdlib::database::drivers::{SqliteDriver, PostgresDriver};',
        'use cursed::stdlib::database::{SqliteDriver, DatabaseError};'
    )
    
    # Fix error type usage
    content = content.replace(
        'Err(CursedError::Database("Failed to connect".to_string()))',
        'Err(DatabaseError::connection_error("Failed to connect".to_string()))'
    )
    
    # Add import for CursedError
    content = content.replace(
        'use cursed::error::CursedError;',
        'use cursed::error::CursedError;\nuse cursed::stdlib::database::DatabaseError;'
    )
    
    # Fix the type annotation issue by removing the generic
    content = content.replace(
        'let driver_clone: Arc<T, A> = Arc::clone(&driver);',
        'let driver_clone = driver.clone();'
    )
    
    # Fix the Arc::clone usage
    content = content.replace(
        'let driver_clone = Arc::clone(&driver);',
        'let driver_clone = driver.clone();'
    )
    
    # Write the fixed content
    with open('tests/database_integration_tests.rs', 'w') as f:
        f.write(content)
    
    print("Fixed database_integration_tests.rs")

if __name__ == "__main__":
    fix_database_test()
