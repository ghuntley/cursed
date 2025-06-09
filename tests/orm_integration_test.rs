/// Comprehensive integration tests for CURSED ORM system
/// 
/// Tests all major ORM features including entities, repositories,
/// query building, relationships, migrations, and transactions.

#[path = "common/mod.rs"]
mod common;

use std::collections::HashMap;
use std::sync::Arc;
use tracing_test::traced_test;

use cursed::stdlib::database::{
    DB, DatabaseError, SqlValue, OrmContext, OrmConfig, Entity, Repository,
    FluentQueryBuilder, MigrationManager, RelationshipManager, ValidationError,
    TransactionalRepository, SchemaBuilder, TypeMapper, CacheConfig,
};
use cursed::stdlib::database::orm::{
    entity::{ColumnDefinition, SqlColumnType, EntityMetadata, Timestamped},
    relationships::{Relationship, RelationshipType, RelationshipBuilder},
    validation::{Required, MinLength, ValidationContext, EntityValidator},
    migration::{CreateTableMigration, MigrationOperation},
    schema::{TableSchema, ColumnSchema, SchemaComparator},
};

/// Test user entity with full ORM features
#[derive(Debug, Clone)]
struct User {
    id: Option<i64>,
    name: String,
    email: String,
    age: Option<i32>,
    created_at: Option<std::time::SystemTime>,
    updated_at: Option<std::time::SystemTime>,
}

impl Entity for User {
    fn table_name() -> &'static str {
        "users"
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
        Ok(Self {
            id: match row.get("id") {
                Some(SqlValue::Integer(id)) => Some(*id),
                _ => None,
            },
            name: match row.get("name") {
                Some(SqlValue::String(name)) => name.clone(),
                _ => return Err(DatabaseError::validation_error("Missing name field")),
            },
            email: match row.get("email") {
                Some(SqlValue::String(email)) => email.clone(),
                _ => String::new(),
            },
            age: match row.get("age") {
                Some(SqlValue::Integer(age)) => Some(*age as i32),
                _ => None,
            },
            created_at: match row.get("created_at") {
                Some(SqlValue::Timestamp(time)) => Some(*time),
                _ => None,
            },
            updated_at: match row.get("updated_at") {
                Some(SqlValue::Timestamp(time)) => Some(*time),
                _ => None,
            },
        })
    }

    fn to_fields(&self) -> HashMap<String, SqlValue> {
        let mut fields = HashMap::new();
        
        if let Some(id) = self.id {
            fields.insert("id".to_string(), SqlValue::Integer(id));
        }
        fields.insert("name".to_string(), SqlValue::String(self.name.clone()));
        fields.insert("email".to_string(), SqlValue::String(self.email.clone()));
        if let Some(age) = self.age {
            fields.insert("age".to_string(), SqlValue::Integer(age as i64));
        }
        if let Some(created_at) = self.created_at {
            fields.insert("created_at".to_string(), SqlValue::Timestamp(created_at));
        }
        if let Some(updated_at) = self.updated_at {
            fields.insert("updated_at".to_string(), SqlValue::Timestamp(updated_at));
        }
        
        fields
    }

    fn field_names() -> Vec<&'static str> {
        vec!["id", "name", "email", "age", "created_at", "updated_at"]
    }

    fn column_definitions() -> Vec<ColumnDefinition> {
        vec![
            ColumnDefinition {
                name: "id".to_string(),
                sql_type: SqlColumnType::BigInteger,
                nullable: false,
                default: None,
                primary_key: true,
                foreign_key: None,
                constraints: vec![],
            },
            ColumnDefinition {
                name: "name".to_string(),
                sql_type: SqlColumnType::VarChar { length: 255 },
                nullable: false,
                default: None,
                primary_key: false,
                foreign_key: None,
                constraints: vec![],
            },
            ColumnDefinition {
                name: "email".to_string(),
                sql_type: SqlColumnType::VarChar { length: 255 },
                nullable: true,
                default: None,
                primary_key: false,
                foreign_key: None,
                constraints: vec![],
            },
            ColumnDefinition {
                name: "age".to_string(),
                sql_type: SqlColumnType::Integer,
                nullable: true,
                default: None,
                primary_key: false,
                foreign_key: None,
                constraints: vec![],
            },
        ]
    }

    fn metadata() -> EntityMetadata {
        EntityMetadata {
            table_name: "users".to_string(),
            primary_key: "id".to_string(),
            fields: vec![
                "id".to_string(),
                "name".to_string(),
                "email".to_string(),
                "age".to_string(),
                "created_at".to_string(),
                "updated_at".to_string(),
            ],
            relationships: vec!["posts".to_string()],
            validation_rules: vec!["name_required".to_string(), "name_min_length".to_string()],
            indexes: vec![],
            version: 1,
        }
    }

    fn validate(&self) -> Result<(), DatabaseError> {
        if self.name.is_empty() {
            return Err(DatabaseError::validation_error("Name cannot be empty"));
        }
        if self.name.len() < 2 {
            return Err(DatabaseError::validation_error("Name must be at least 2 characters"));
        }
        Ok(())
    }

    fn relationships() -> Vec<cursed::stdlib::database::orm::relationships::Relationship> {
        vec![
            RelationshipBuilder::new("posts", "users", "posts")
                .has_many("user_id"),
        ]
    }

    fn as_timestamped_mut(&mut self) -> Option<&mut dyn Timestamped> {
        Some(self)
    }
}

impl Timestamped for User {
    fn created_at(&self) -> Option<std::time::SystemTime> {
        self.created_at
    }

    fn updated_at(&self) -> Option<std::time::SystemTime> {
        self.updated_at
    }

    fn touch_created_at(&mut self) {
        self.created_at = Some(std::time::SystemTime::now());
    }

    fn touch_updated_at(&mut self) {
        self.updated_at = Some(std::time::SystemTime::now());
    }
}

/// Test post entity for relationship testing
#[derive(Debug, Clone)]
struct Post {
    id: Option<i64>,
    user_id: i64,
    title: String,
    content: String,
}

impl Entity for Post {
    fn table_name() -> &'static str {
        "posts"
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
        Ok(Self {
            id: match row.get("id") {
                Some(SqlValue::Integer(id)) => Some(*id),
                _ => None,
            },
            user_id: match row.get("user_id") {
                Some(SqlValue::Integer(user_id)) => *user_id,
                _ => return Err(DatabaseError::validation_error("Missing user_id field")),
            },
            title: match row.get("title") {
                Some(SqlValue::String(title)) => title.clone(),
                _ => String::new(),
            },
            content: match row.get("content") {
                Some(SqlValue::String(content)) => content.clone(),
                _ => String::new(),
            },
        })
    }

    fn to_fields(&self) -> HashMap<String, SqlValue> {
        let mut fields = HashMap::new();
        if let Some(id) = self.id {
            fields.insert("id".to_string(), SqlValue::Integer(id));
        }
        fields.insert("user_id".to_string(), SqlValue::Integer(self.user_id));
        fields.insert("title".to_string(), SqlValue::String(self.title.clone()));
        fields.insert("content".to_string(), SqlValue::String(self.content.clone()));
        fields
    }

    fn field_names() -> Vec<&'static str> {
        vec!["id", "user_id", "title", "content"]
    }

    fn column_definitions() -> Vec<ColumnDefinition> {
        vec![]
    }

    fn metadata() -> EntityMetadata {
        EntityMetadata {
            table_name: "posts".to_string(),
            primary_key: "id".to_string(),
            fields: vec![
                "id".to_string(),
                "user_id".to_string(),
                "title".to_string(),
                "content".to_string(),
            ],
            relationships: vec!["user".to_string()],
            validation_rules: vec![],
            indexes: vec![],
            version: 1,
        }
    }

    fn relationships() -> Vec<cursed::stdlib::database::orm::relationships::Relationship> {
        vec![
            RelationshipBuilder::new("user", "posts", "users")
                .belongs_to("user_id"),
        ]
    }
}

fn create_test_db() -> Arc<DB> {
    Arc::new(DB::new("test_orm").expect("Failed to create test database"))
}

fn create_test_orm_config() -> OrmConfig {
    OrmConfig {
        cache_config: CacheConfig {
            max_size: 1000,
            default_ttl: std::time::Duration::from_secs(300),
            enable_query_cache: true,
            enable_entity_cache: true,
        },
        query_timeout: std::time::Duration::from_secs(30),
        enable_query_logging: true,
        pool_config: cursed::stdlib::database::orm::PoolConfig::default(),
        migration_config: cursed::stdlib::database::orm::MigrationConfig::default(),
    }
}

#[traced_test]
#[test]
fn test_orm_context_creation() {
    common::setup_tracing();
    
    let db = create_test_db();
    let config = create_test_orm_config();
    let orm_context = OrmContext::new(db, config);
    
    let stats = orm_context.stats();
    assert_eq!(stats.cache_stats.hits, 0);
    assert_eq!(stats.entity_stats.total_queries, 0);
}

#[traced_test]
#[test]
fn test_entity_metadata() {
    common::setup_tracing();
    
    let metadata = User::metadata();
    assert_eq!(metadata.table_name, "users");
    assert_eq!(metadata.primary_key, "id");
    assert_eq!(metadata.fields.len(), 6);
    assert!(metadata.fields.contains(&"name".to_string()));
    assert!(metadata.relationships.contains(&"posts".to_string()));
    assert_eq!(metadata.version, 1);
}

#[traced_test]
#[test]
fn test_entity_validation() {
    common::setup_tracing();
    
    // Valid user
    let valid_user = User {
        id: None,
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        age: Some(30),
        created_at: None,
        updated_at: None,
    };
    assert!(valid_user.validate().is_ok());
    
    // Invalid user - empty name
    let invalid_user = User {
        id: None,
        name: "".to_string(),
        email: "john@example.com".to_string(),
        age: Some(30),
        created_at: None,
        updated_at: None,
    };
    assert!(invalid_user.validate().is_err());
    
    // Invalid user - short name
    let short_name_user = User {
        id: None,
        name: "A".to_string(),
        email: "john@example.com".to_string(),
        age: Some(30),
        created_at: None,
        updated_at: None,
    };
    assert!(short_name_user.validate().is_err());
}

#[traced_test]
#[tokio::test]
async fn test_repository_operations() {
    common::setup_tracing();
    
    let db = create_test_db();
    let config = create_test_orm_config();
    let orm_context = OrmContext::new(db, config);
    let repository = orm_context.repository::<User>();
    
    // Test finding by primary key
    let user = repository.find_by_vibe(SqlValue::Integer(1)).await;
    assert!(user.is_ok());
    
    // Test finding with conditions
    let conditions = vec![("name", SqlValue::String("John".to_string()))];
    let users = repository.find_where_its_at(&conditions).await;
    assert!(users.is_ok());
}

#[traced_test]
#[test]
fn test_fluent_query_builder() {
    common::setup_tracing();
    
    let db = create_test_db();
    let builder = FluentQueryBuilder::<User>::new("users", db);
    
    let sql_query = builder
        .select_these_vibes(&["id", "name", "email"])
        .where_clause("age > ?", vec![SqlValue::Integer(18)])
        .order_by_vibe("name", cursed::stdlib::database::orm::query_builder::OrderDirection::Ascending)
        .limit(10)
        .build_sql();
    
    assert!(sql_query.is_ok());
    let sql = sql_query.unwrap();
    assert!(sql.contains("SELECT id, name, email"));
    assert!(sql.contains("FROM users"));
    assert!(sql.contains("WHERE age > ?"));
    assert!(sql.contains("ORDER BY name ASC"));
    assert!(sql.contains("LIMIT 10"));
}

#[traced_test]
#[test]
fn test_relationship_management() {
    common::setup_tracing();
    
    let relationship_manager = RelationshipManager::new();
    
    // Register user -> posts relationship
    let user_posts_relationship = RelationshipBuilder::new("posts", "users", "posts")
        .has_many("user_id");
    
    relationship_manager
        .register_relationship("users", user_posts_relationship)
        .expect("Should register relationship");
    
    // Register post -> user relationship
    let post_user_relationship = RelationshipBuilder::new("user", "posts", "users")
        .belongs_to("user_id");
    
    relationship_manager
        .register_relationship("posts", post_user_relationship)
        .expect("Should register relationship");
    
    // Test relationship retrieval
    let user_relationships = relationship_manager.get_relationships("users");
    assert_eq!(user_relationships.len(), 1);
    assert_eq!(user_relationships[0].name(), "posts");
    
    let post_relationships = relationship_manager.get_relationships("posts");
    assert_eq!(post_relationships.len(), 1);
    assert_eq!(post_relationships[0].name(), "user");
}

#[traced_test]
#[test]
fn test_migration_generation() {
    common::setup_tracing();
    
    let db = create_test_db();
    let migration_manager = MigrationManager::new(db);
    
    // Generate create table migration
    let create_migration = migration_manager
        .generate_migration::<User>(MigrationOperation::CreateTable)
        .expect("Should generate create table migration");
    
    assert_eq!(create_migration.name(), "create_table_users");
    assert!(!create_migration.version().is_empty());
    
    // Generate add column migration
    let column = ColumnDefinition {
        name: "phone".to_string(),
        sql_type: SqlColumnType::VarChar { length: 20 },
        nullable: true,
        default: None,
        primary_key: false,
        foreign_key: None,
        constraints: vec![],
    };
    
    let add_column_migration = migration_manager
        .generate_migration::<User>(MigrationOperation::AddColumn { column })
        .expect("Should generate add column migration");
    
    assert_eq!(add_column_migration.name(), "add_column_users");
}

#[traced_test]
#[test]
fn test_schema_building() {
    common::setup_tracing();
    
    let schema = SchemaBuilder::new("test_schema")
        .table("users", |table| {
            table
                .column("id", SqlColumnType::BigInteger)
                .auto_increment()
                .not_null()
                .end_column()
                .column("name", SqlColumnType::VarChar { length: 255 })
                .not_null()
                .end_column()
                .column("email", SqlColumnType::VarChar { length: 255 })
                .end_column()
                .primary_key(&["id"])
                .index("idx_users_email", &["email"])
        })
        .table("posts", |table| {
            table
                .column("id", SqlColumnType::BigInteger)
                .auto_increment()
                .not_null()
                .end_column()
                .column("user_id", SqlColumnType::BigInteger)
                .not_null()
                .end_column()
                .column("title", SqlColumnType::VarChar { length: 255 })
                .not_null()
                .end_column()
                .primary_key(&["id"])
                .index("idx_posts_user_id", &["user_id"])
        })
        .build();
    
    assert_eq!(schema.name, "test_schema");
    assert_eq!(schema.tables.len(), 2);
    
    let users_table = schema.get_table("users").expect("Should have users table");
    assert_eq!(users_table.columns.len(), 3);
    assert_eq!(users_table.primary_keys, vec!["id"]);
    assert_eq!(users_table.indexes.len(), 1);
    
    let posts_table = schema.get_table("posts").expect("Should have posts table");
    assert_eq!(posts_table.columns.len(), 3);
    assert_eq!(posts_table.indexes.len(), 1);
}

#[traced_test]
#[test]
fn test_schema_comparison() {
    common::setup_tracing();
    
    // Create current schema
    let current_schema = SchemaBuilder::new("current")
        .table("users", |table| {
            table
                .column("id", SqlColumnType::BigInteger)
                .auto_increment()
                .not_null()
                .end_column()
                .column("name", SqlColumnType::VarChar { length: 255 })
                .not_null()
                .end_column()
                .primary_key(&["id"])
        })
        .build();
    
    // Create target schema with additional column and table
    let target_schema = SchemaBuilder::new("target")
        .table("users", |table| {
            table
                .column("id", SqlColumnType::BigInteger)
                .auto_increment()
                .not_null()
                .end_column()
                .column("name", SqlColumnType::VarChar { length: 255 })
                .not_null()
                .end_column()
                .column("email", SqlColumnType::VarChar { length: 255 })
                .end_column()
                .primary_key(&["id"])
        })
        .table("posts", |table| {
            table
                .column("id", SqlColumnType::BigInteger)
                .auto_increment()
                .not_null()
                .end_column()
                .primary_key(&["id"])
        })
        .build();
    
    let diff = SchemaComparator::compare(&current_schema, &target_schema);
    
    assert_eq!(diff.added_tables.len(), 1);
    assert_eq!(diff.added_tables[0].name, "posts");
    
    assert_eq!(diff.modified_tables.len(), 1);
    assert_eq!(diff.modified_tables[0].table_name, "users");
    assert_eq!(diff.modified_tables[0].added_columns.len(), 1);
    assert_eq!(diff.modified_tables[0].added_columns[0].name, "email");
}

#[traced_test]
#[test]
fn test_validation_framework() {
    common::setup_tracing();
    
    let mut entity_validator = EntityValidator::new();
    
    // Add validation rules
    entity_validator.add_field_rule("name", Box::new(Required));
    entity_validator.add_field_rule("name", Box::new(MinLength { min: 2 }));
    
    // Test valid entity
    let mut valid_values = HashMap::new();
    valid_values.insert("name".to_string(), SqlValue::String("John Doe".to_string()));
    valid_values.insert("email".to_string(), SqlValue::String("john@example.com".to_string()));
    
    let valid_context = ValidationContext::new("user", valid_values);
    let validation_result = entity_validator.validate(&valid_context);
    assert!(validation_result.is_ok());
    
    // Test invalid entity
    let mut invalid_values = HashMap::new();
    invalid_values.insert("name".to_string(), SqlValue::String("J".to_string()));
    
    let invalid_context = ValidationContext::new("user", invalid_values);
    let validation_result = entity_validator.validate(&invalid_context);
    assert!(validation_result.is_err());
    
    if let Err(errors) = validation_result {
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].rule, "min_length");
    }
}

#[traced_test]
#[test]
fn test_type_mapping() {
    common::setup_tracing();
    
    let type_mapper = TypeMapper::new();
    
    // Test built-in mappings
    let i64_mapping = type_mapper.map_to_sql("i64").expect("Should map i64");
    assert!(matches!(
        i64_mapping,
        cursed::stdlib::database::orm::mapping::SqlTypeMapping::Simple {
            sql_type: SqlColumnType::BigInteger,
            nullable: false
        }
    ));
    
    let string_mapping = type_mapper.map_to_sql("String").expect("Should map String");
    assert!(matches!(
        string_mapping,
        cursed::stdlib::database::orm::mapping::SqlTypeMapping::Simple {
            sql_type: SqlColumnType::Text,
            nullable: false
        }
    ));
    
    // Test Option type mapping
    let option_string_mapping = type_mapper.map_to_sql("Option<String>").expect("Should map Option<String>");
    assert!(matches!(
        option_string_mapping,
        cursed::stdlib::database::orm::mapping::SqlTypeMapping::Simple {
            sql_type: SqlColumnType::Text,
            nullable: true
        }
    ));
}

#[traced_test]
#[test]
fn test_timestamped_entity() {
    common::setup_tracing();
    
    let mut user = User {
        id: Some(1),
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        age: Some(30),
        created_at: None,
        updated_at: None,
    };
    
    // Test timestamp methods
    user.touch_created_at();
    user.touch_updated_at();
    
    assert!(user.created_at().is_some());
    assert!(user.updated_at().is_some());
    
    // Test timestamped trait via entity
    if let Some(timestamped) = user.as_timestamped_mut() {
        timestamped.touch_updated_at();
        assert!(timestamped.updated_at().is_some());
    }
}

#[traced_test]
#[test]
fn test_sql_generation() {
    common::setup_tracing();
    
    let table_schema = TableSchema::from_entity::<User>().expect("Should create table schema from entity");
    
    // Test PostgreSQL SQL generation
    let postgres_sql = table_schema.to_create_sql("postgresql");
    assert!(postgres_sql.contains("CREATE TABLE users"));
    assert!(postgres_sql.contains("id BIGINT"));
    assert!(postgres_sql.contains("name VARCHAR(255)"));
    assert!(postgres_sql.contains("PRIMARY KEY (id)"));
    
    // Test SQLite SQL generation
    let sqlite_sql = table_schema.to_create_sql("sqlite");
    assert!(sqlite_sql.contains("CREATE TABLE users"));
    assert!(sqlite_sql.contains("id INTEGER"));
    assert!(sqlite_sql.contains("name TEXT"));
}

#[traced_test]
#[tokio::test]
async fn test_full_orm_workflow() {
    common::setup_tracing();
    
    // Create ORM context
    let db = create_test_db();
    let config = create_test_orm_config();
    let orm_context = OrmContext::new(db, config);
    
    // Register entities
    orm_context.entity_manager.register::<User>().expect("Should register User entity");
    orm_context.entity_manager.register::<Post>().expect("Should register Post entity");
    
    // Get repository
    let user_repository = orm_context.repository::<User>();
    
    // Create a test user
    let mut user = User {
        id: None,
        name: "Jane Doe".to_string(),
        email: "jane@example.com".to_string(),
        age: Some(25),
        created_at: None,
        updated_at: None,
    };
    
    // Test save operation (would create in real implementation)
    let save_result = user_repository.save_it(&user).await;
    assert!(save_result.is_ok());
    
    // Test query building
    let query_result = user_repository.query()
        .where_clause("age > ?", vec![SqlValue::Integer(18)])
        .asc_vibes("name")
        .limit(10)
        .execute()
        .await;
    assert!(query_result.is_ok());
    
    // Test relationship loading (simplified)
    let relationships = user_repository.relationship_manager.get_relationships("users");
    assert_eq!(relationships.len(), 1);
    assert_eq!(relationships[0].name(), "posts");
    
    // Get ORM statistics
    let stats = orm_context.stats();
    assert_eq!(stats.entity_stats.total_queries, 2); // Registered User and Post
}

#[traced_test]
#[test]
fn test_cache_operations() {
    common::setup_tracing();
    
    let cache_config = CacheConfig {
        max_size: 100,
        default_ttl: std::time::Duration::from_secs(60),
        enable_query_cache: true,
        enable_entity_cache: true,
    };
    
    let mut query_cache = cursed::stdlib::database::orm::cache::QueryCache::new(cache_config);
    
    // Test cache set and get
    query_cache.set(
        "test_key".to_string(),
        "test_value".to_string(),
        std::time::Duration::from_secs(60),
    );
    
    let cached_value: Option<String> = query_cache.get("test_key");
    assert_eq!(cached_value, Some("test_value".to_string()));
    
    // Test cache stats
    let stats = query_cache.stats();
    assert_eq!(stats.hits, 1);
    assert_eq!(stats.sets, 1);
    assert_eq!(stats.current_size, 1);
}

#[traced_test]
#[test]
fn test_error_handling() {
    common::setup_tracing();
    
    // Test validation error
    let validation_error = ValidationError::new("name", "required", "Name is required");
    assert_eq!(validation_error.field, "name");
    assert_eq!(validation_error.rule, "required");
    assert_eq!(validation_error.message, "Name is required");
    
    // Test validation error with values
    let validation_error_with_values = ValidationError::with_values(
        "age",
        "min_value",
        "Age must be at least 18",
        "18",
        "16",
    );
    assert_eq!(validation_error_with_values.expected, Some("18".to_string()));
    assert_eq!(validation_error_with_values.actual, Some("16".to_string()));
    
    // Test error display
    let error_string = validation_error_with_values.to_string();
    assert!(error_string.contains("min_value"));
    assert!(error_string.contains("Expected: 18"));
    assert!(error_string.contains("Actual: 16"));
}
