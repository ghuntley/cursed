/// Example entity implementation demonstrating the enhanced CURSED ORM
/// 
/// This shows how to implement the Entity trait for a User model
/// with proper serialization and deserialization.

use std::collections::HashMap;
use super::{Entity, DatabaseError};
use crate::stdlib::database::{SqlValue, DatabaseErrorKind};
use crate::stdlib::database::orm::entity::{ColumnDefinition, EntityMetadata, PrimaryKey};

/// Example User entity for demonstration
#[derive(Debug, Clone)]
pub struct User {
    pub id: Option<i64>,
    pub username: String,
    pub email: String,
    pub created_at: Option<String>,
}

impl User {
    /// Create a new user (without ID for insert)
    pub fn new(username: String, email: String) -> Self {
        Self {
            id: None,
            username,
            email,
            created_at: None,
        }
    }
    
    /// Create a user with an existing ID
    pub fn with_id(id: i64, username: String, email: String) -> Self {
        Self {
            id: Some(id),
            username,
            email,
            created_at: None,
        }
    }
}

impl Entity for User {
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
        if let Some(id) = value.as_i64() {
            self.id = Some(id);
        }
    }
    
    fn from_row(row: &HashMap<String, SqlValue>) -> Result<Self, DatabaseError> {
        let id = row.get("id")
            .and_then(|v| v.as_i64());
            
        let username = row.get("username")
            .and_then(|v| v.as_str())
            .ok_or_else(|| DatabaseError::mapping("Missing or invalid username field"))?
            .to_string();
            
        let email = row.get("email")
            .and_then(|v| v.as_str())
            .ok_or_else(|| DatabaseError::mapping("Missing or invalid email field"))?
            .to_string();
            
        let created_at = row.get("created_at")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
            
        Ok(User {
            id,
            username,
            email,
            created_at,
        })
    }
    
    fn to_fields(&self) -> HashMap<String, SqlValue> {
        let mut fields = HashMap::new();
        
        if let Some(id) = self.id {
            fields.insert("id".to_string(), SqlValue::Integer(id));
        }
        
        fields.insert("username".to_string(), SqlValue::String(self.username.clone()));
        fields.insert("email".to_string(), SqlValue::String(self.email.clone()));
        
        if let Some(ref created_at) = self.created_at {
            fields.insert("created_at".to_string(), SqlValue::String(created_at.clone()));
        }
        
        fields
    }
    
    fn field_names() -> Vec<&'static str> {
        vec!["id", "username", "email", "created_at"]
    }
    
    fn column_definitions() -> Vec<ColumnDefinition> {
        vec![
            ColumnDefinition::new("id", "INTEGER").primary_key(),
            ColumnDefinition::new("username", "VARCHAR(255)"),
            ColumnDefinition::new("email", "VARCHAR(255)"),
            ColumnDefinition::new("created_at", "TIMESTAMP")
                .nullable()
                .default_value(SqlValue::String("CURRENT_TIMESTAMP".to_string())),
        ]
    }
    
    fn metadata() -> EntityMetadata {
        EntityMetadata {
            table_name: "users".to_string(),
            primary_key: "id".to_string(),
            fields: vec!["id".to_string(), "username".to_string(), "email".to_string(), "created_at".to_string()],
            relationships: vec![],
            validation_rules: vec![
                "UNIQUE(username)".to_string(),
                "UNIQUE(email)".to_string(),
            ],
            indexes: vec![
                "idx_users_username".to_string(),
                "idx_users_email".to_string(),
            ],
            version: 1,
        }
    }
}

/// Example Product entity demonstrating different field types
#[derive(Debug, Clone)]
pub struct Product {
    pub id: Option<i64>,
    pub name: String,
    pub price: f64,
    pub in_stock: bool,
    pub description: Option<String>,
}

impl Product {
    pub fn new(name: String, price: f64, in_stock: bool) -> Self {
        Self {
            id: None,
            name,
            price,
            in_stock,
            description: None,
        }
    }
}

impl Entity for Product {
    fn table_name() -> &'static str {
        "products"
    }
    
    fn primary_key_name() -> &'static str {
        "id"
    }
    
    fn primary_key_value(&self) -> Option<SqlValue> {
        self.id.map(SqlValue::Integer)
    }
    
    fn set_primary_key_value(&mut self, value: SqlValue) {
        if let Some(id) = value.as_i64() {
            self.id = Some(id);
        }
    }
    
    fn from_row(row: &HashMap<String, SqlValue>) -> Result<Self, DatabaseError> {
        let id = row.get("id").and_then(|v| v.as_i64());
        
        let name = row.get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| DatabaseError::mapping("Missing or invalid name field"))?
            .to_string();
            
        let price = row.get("price")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| DatabaseError::mapping("Missing or invalid price field"))?;
            
        let in_stock = row.get("in_stock")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
            
        let description = row.get("description")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
            
        Ok(Product {
            id,
            name,
            price,
            in_stock,
            description,
        })
    }
    
    fn to_fields(&self) -> HashMap<String, SqlValue> {
        let mut fields = HashMap::new();
        
        if let Some(id) = self.id {
            fields.insert("id".to_string(), SqlValue::Integer(id));
        }
        
        fields.insert("name".to_string(), SqlValue::String(self.name.clone()));
        fields.insert("price".to_string(), SqlValue::Float(self.price));
        fields.insert("in_stock".to_string(), SqlValue::Bool(self.in_stock));
        
        if let Some(ref description) = self.description {
            fields.insert("description".to_string(), SqlValue::String(description.clone()));
        }
        
        fields
    }
    
    fn field_names() -> Vec<&'static str> {
        vec!["id", "name", "price", "in_stock", "description"]
    }
    
    fn column_definitions() -> Vec<ColumnDefinition> {
        vec![
            ColumnDefinition::new("id", "INTEGER").primary_key(),
            ColumnDefinition::new("name", "VARCHAR(255)"),
            ColumnDefinition::new("price", "DECIMAL(10,2)"),
            ColumnDefinition::new("in_stock", "BOOLEAN")
                .default_value(SqlValue::Bool(true)),
            ColumnDefinition::new("description", "TEXT").nullable(),
        ]
    }
    
    fn metadata() -> EntityMetadata {
        EntityMetadata {
            table_name: "products".to_string(),
            primary_key: "id".to_string(),
            fields: vec!["id".to_string(), "name".to_string(), "price".to_string(), "in_stock".to_string(), "description".to_string()],
            relationships: vec![],
            validation_rules: vec![
                "CHECK(price >= 0)".to_string(),
            ],
            indexes: vec![
                "idx_products_name".to_string(),
                "idx_products_price".to_string(),
            ],
            version: 1,
        }
    }
}

/// Test the enhanced ORM functionality
pub async fn test_enhanced_orm() -> Result<(), DatabaseError> {
    use crate::stdlib::database::connection::{InMemoryDatabase, DatabaseConnection};
    use std::sync::Arc;
    use super::Repository;
    
    // Create in-memory database for testing
    let db = InMemoryDatabase::new();
    let connection: Arc<dyn DatabaseConnection> = Arc::new(db);
    
    // Create user repository
    let user_repo: Repository<User> = Repository::new(connection.clone());
    
    // Test creating and saving a user
    let mut user = User::new("alice".to_string(), "alice@example.com".to_string());
    user_repo.save(&mut user).await?;
    
    println!("✅ Enhanced ORM: User created with ID: {:?}", user.id);
    
    // Test finding user by ID
    if let Some(id) = user.id {
        if let Some(found_user) = user_repo.find_by_id(&SqlValue::Integer(id)).await? {
            println!("✅ Enhanced ORM: Found user: {}", found_user.username);
        }
    }
    
    // Test counting users
    let count = user_repo.count().await?;
    println!("✅ Enhanced ORM: Total users: {}", count);
    
    // Test finding all users
    let all_users = user_repo.find_all().await?;
    println!("✅ Enhanced ORM: Found {} users", all_users.len());
    
    println!("🎉 Enhanced ORM functionality test completed successfully!");
    Ok(())
}
