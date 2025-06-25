/// Test for real database execution functionality in CURSED ORM
/// 
/// This test validates that the ORM can perform real CRUD operations
/// against an SQLite database with proper transaction handling.

use std::collections::HashMap;
use std::sync::Arc;
use cursed::stdlib::database::{
    DB, DatabaseError, SqlValue, VibeContext,
    orm::{Entity, Repository, OrmContext, OrmConfig, EntityMetadata, 
          ColumnDefinition, FluentQueryBuilder},
};
use tracing_test::traced_test;

/// Test entity for demonstrating ORM functionality
#[derive(Debug, Clone)]
struct TestUser {
    pub id: Option<i64>,
    pub name: String,
    pub email: String,
    pub age: i32,
}

impl Entity for TestUser {
    fn table_name() -> &'static str {
        "users"
    }

    fn primary_key_name() -> &'static str {
        "id"
    }

    fn primary_key_value(&self) -> Option<SqlValue> {
        self.id.map(SqlValue::Integer)
    }

    fn set_primary_key_value(&mut self, value: SqlValue) {
        if let SqlValue::Integer(id) = value {
            self.id = Some(id);
        }
    }

    fn from_row(row: &HashMap<String, SqlValue>) -> Result<Self, DatabaseError> {
        let id = match row.get("id") {
            Some(SqlValue::Integer(i)) => Some(*i),
            _ => None,
        };

        let name = match row.get("name") {
            Some(SqlValue::String(s)) => s.clone(),
            _ => return Err(DatabaseError::validation_error("Missing or invalid name field")),
        };

        let email = match row.get("email") {
            Some(SqlValue::String(s)) => s.clone(),
            _ => return Err(DatabaseError::validation_error("Missing or invalid email field")),
        };

        let age = match row.get("age") {
            Some(SqlValue::Integer(i)) => *i as i32,
            _ => return Err(DatabaseError::validation_error("Missing or invalid age field")),
        };

        Ok(TestUser { id, name, email, age })
    }

    fn to_fields(&self) -> HashMap<String, SqlValue> {
        let mut fields = HashMap::new();
        
        if let Some(id) = self.id {
            fields.insert("id".to_string(), SqlValue::Integer(id));
        }
        fields.insert("name".to_string(), SqlValue::String(self.name.clone()));
        fields.insert("email".to_string(), SqlValue::String(self.email.clone()));
        fields.insert("age".to_string(), SqlValue::Integer(self.age as i64));
        
        fields
    }

    fn field_names() -> Vec<&'static str> {
        vec!["id", "name", "email", "age"]
    }

    fn column_definitions() -> Vec<ColumnDefinition> {
        vec![]
    }

    fn metadata() -> EntityMetadata {
        EntityMetadata {
            table_name: "users".to_string(),
            primary_key: "id".to_string(),
            fields: vec!["id".to_string(), "name".to_string(), "email".to_string(), "age".to_string()],
            relationships: vec![],
            validation_rules: vec![],
            indexes: vec![],
            version: 1,
        }
    }
}

/// Create a test database with users table
async fn create_test_database() -> Result<Arc<DB>, DatabaseError> {
    let db = Arc::new(DB::open("sqlite".to_string(), ":memory:".to_string())?);
    
    // Create users table
    let create_table_sql = r#"
        CREATE TABLE users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE,
            age INTEGER NOT NULL
        )
    "#;
    
    db.exec(create_table_sql.to_string(), vec![])?;
    
    Ok(db)
}

#[traced_test]
#[tokio::test]
async fn test_real_entity_creation() -> Result<(), Box<dyn std::error::Error>> {
    let db = create_test_database().await?;
    let config = OrmConfig::default();
    let orm = OrmContext::new(db.clone(), config);
    let repository = orm.repository::<TestUser>();

    // Create a new user entity
    let user = TestUser {
        id: None,
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        age: 30,
    };

    // Save the entity - this should execute real INSERT SQL
    let saved_user = repository.save_it(&user).await?;
    
    // Verify the user got an ID assigned
    assert!(saved_user.id.is_some());
    assert_eq!(saved_user.name, "John Doe");
    assert_eq!(saved_user.email, "john@example.com");
    assert_eq!(saved_user.age, 30);

    println!("✅ Successfully created user with ID: {:?}", saved_user.id);
    Ok(())
}

#[traced_test]
#[tokio::test]
async fn test_real_entity_finding() -> Result<(), Box<dyn std::error::Error>> {
    let db = create_test_database().await?;
    let config = OrmConfig::default();
    let orm = OrmContext::new(db.clone(), config);
    let repository = orm.repository::<TestUser>();

    // Create test data
    let user = TestUser {
        id: None,
        name: "Jane Smith".to_string(),
        email: "jane@example.com".to_string(),
        age: 25,
    };

    let saved_user = repository.save_it(&user).await?;
    let user_id = saved_user.id.unwrap();

    // Find the user by ID - this should execute real SELECT SQL
    let found_user = repository.find_by_vibe(SqlValue::Integer(user_id)).await?;
    
    assert!(found_user.is_some());
    let found_user = found_user.unwrap();
    assert_eq!(found_user.id, Some(user_id));
    assert_eq!(found_user.name, "Jane Smith");
    assert_eq!(found_user.email, "jane@example.com");

    println!("✅ Successfully found user: {:?}", found_user);
    Ok(())
}

#[traced_test]
#[tokio::test]
async fn test_real_entity_updating() -> Result<(), Box<dyn std::error::Error>> {
    let db = create_test_database().await?;
    let config = OrmConfig::default();
    let orm = OrmContext::new(db.clone(), config);
    let repository = orm.repository::<TestUser>();

    // Create and save a user
    let mut user = TestUser {
        id: None,
        name: "Bob Wilson".to_string(),
        email: "bob@example.com".to_string(),
        age: 35,
    };

    let saved_user = repository.save_it(&user).await?;
    user.id = saved_user.id;

    // Update the user's age
    user.age = 36;
    let updated_user = repository.save_it(&user).await?;

    // Verify the update
    assert_eq!(updated_user.age, 36);
    assert_eq!(updated_user.id, saved_user.id);

    println!("✅ Successfully updated user age to: {}", updated_user.age);
    Ok(())
}

#[traced_test]
#[tokio::test]
async fn test_real_entity_deletion() -> Result<(), Box<dyn std::error::Error>> {
    let db = create_test_database().await?;
    let config = OrmConfig::default();
    let orm = OrmContext::new(db.clone(), config);
    let repository = orm.repository::<TestUser>();

    // Create and save a user
    let user = TestUser {
        id: None,
        name: "Alice Brown".to_string(),
        email: "alice@example.com".to_string(),
        age: 28,
    };

    let saved_user = repository.save_it(&user).await?;
    let user_id = saved_user.id.unwrap();

    // Delete the user - this should execute real DELETE SQL
    let deleted = repository.delete_sus(&saved_user).await?;
    assert!(deleted);

    // Verify the user is gone
    let found_user = repository.find_by_vibe(SqlValue::Integer(user_id)).await?;
    assert!(found_user.is_none());

    println!("✅ Successfully deleted user with ID: {}", user_id);
    Ok(())
}

#[traced_test]
#[tokio::test]
async fn test_real_query_builder_execution() -> Result<(), Box<dyn std::error::Error>> {
    let db = create_test_database().await?;
    let config = OrmConfig::default();
    let orm = OrmContext::new(db.clone(), config);
    let repository = orm.repository::<TestUser>();

    // Create some test data
    let users = vec![
        TestUser {
            id: None,
            name: "User One".to_string(),
            email: "user1@example.com".to_string(),
            age: 20,
        },
        TestUser {
            id: None,
            name: "User Two".to_string(),
            email: "user2@example.com".to_string(),
            age: 30,
        },
        TestUser {
            id: None,
            name: "User Three".to_string(),
            email: "user3@example.com".to_string(),
            age: 40,
        },
    ];

    // Bulk insert the users
    let saved_users = repository.bulk_insert_vibes(&users).await?;
    assert_eq!(saved_users.len(), 3);

    // Test real query builder execution with WHERE clause
    let query = repository.query()
        .where_clause("age > $1", vec![SqlValue::Integer(25)]);
    
    let results = query.execute().await?;
    assert_eq!(results.len(), 2); // Should find users with age 30 and 40

    println!("✅ Query builder found {} users over 25", results.len());

    // Test real count query
    let count_query = repository.query()
        .where_clause("age < $1", vec![SqlValue::Integer(35)]);
    
    let count = count_query.count_the_vibes().await?;
    assert_eq!(count, 2); // Should count users with age 20 and 30

    println!("✅ Count query found {} users under 35", count);
    Ok(())
}

#[traced_test]
#[tokio::test]
async fn test_real_bulk_operations() -> Result<(), Box<dyn std::error::Error>> {
    let db = create_test_database().await?;
    let config = OrmConfig::default();
    let orm = OrmContext::new(db.clone(), config);
    let repository = orm.repository::<TestUser>();

    // Test bulk insert with transaction
    let users: Vec<TestUser> = (1..=5).map(|i| TestUser {
        id: None,
        name: format!("Bulk User {}", i),
        email: format!("bulk{}@example.com", i),
        age: 20 + i,
    }).collect();

    let saved_users = repository.bulk_insert_vibes(&users).await?;
    assert_eq!(saved_users.len(), 5);

    // Verify all users have IDs
    for user in &saved_users {
        assert!(user.id.is_some());
    }

    println!("✅ Successfully bulk inserted {} users", saved_users.len());
    Ok(())
}
