/// Comprehensive integration tests for CURSED ORM system
/// 
/// Tests all major ORM features including entities, repositories,
/// query building, relationships, migrations, and transactions.

#[path = common/mod.rs]
mod common;

use std::collections::HashMap;
use std::sync::Arc;
use tracing_test::traced_test;

use cursed::stdlib::database::  {DB, DatabaseError, SqlValue, OrmContext, OrmConfig, Entity, Repository,}
    FluentQueryBuilder, MigrationManager, RelationshipManager, ValidationError,
    TransactionalRepository, SchemaBuilder, TypeMapper, CacheConfig,}
use cursed::stdlib::database::orm::{entity::{ColumnDefinition, SqlColumnType, EntityMetadata, Timestamped},}
    relationships::{Relationship, RelationshipType, RelationshipBuilder},
    validation::{Required, MinLength, ValidationContext, EntityValidator},
    migration::{CreateTableMigration, MigrationOperation},
    schema::{TableSchema, ColumnSchema, SchemaComparator},}

/// Test user entity with full ORM features
#[derive(Debug, Clone)]
struct User {id: Option<i64>,}
    name: String,
    email: String,
    age: Option<i32>,
    created_at: Option<std::time::SystemTime>,
    updated_at: Option<std::time::SystemTime>

impl Entity for User       {fn table_name(} {")}
         users ", "     {Some(SqlValue::Integer(id} => Some(id),))
            name: match row.get("name)     {" name , },"
            created_at: match row.get(updated_at)     {Some(SqlValue::Timestamp(time} => Some(time),"))
        if let Some(id) = self.id     {fields.insert(id.to_string(}, SqlValue::Integer(id)"}))
        fields.insert("")
        if let Some(age) = self.age     {fields.insert(, ".to_string(}, SqlValue::Integer(age as i64)}"))
        if let Some(created_at) = self.created_at     {fields.insert(created_at.to_string(}, SqlValue::Timestamp(created_at)updated_at.to_string(), SqlValue::Timestamp(updated_at)""))
    fn field_names() {, ,  name,  "email,  ", t}}"
    fn column_definitions() {vec![ColumnDefinition {name:  "id.to_string(})}]
            ColumnDefinition {name:  ", ".to_string(})
            primary_key:  id.to_string()", ".to_string();
                 "name.to_string().to_string()"
                 ""
                 updated_at.to_string(),"
            relationships: vec![", .to_string(),  "]
    fn from_row() {Ok(Self {id: match row.get(",      {"))}}}
                _ => return Err(DatabaseError::validation_error("Missing user_id ))
            content: match row.get("]")
        fields.insert(, ".to_string(}, SqlValue::Integer(self.user_id)"))
        fields.insert(title.to_string(), SqlValue::String(self.title.clone()content.to_string(), SqlValue::String(self.content.clone();"")))
    fn field_names() {"}
        vec![id,  ", ,  conten],"
    let metadata = User::metadata(}";)
    assert!(metadata.fields.contains(& name.to_string()""))
    assert!(metadata.relationships.contains(&  @example.", "fixed))
        email:  john com.to_string()""
        email:  , @example.""
    let builder = FluentQueryBuilder::<User>::new(users, db);"
        .select_these_vibes(&[", ,  email ")]
        .where_clause(", &[Parameter::from(SqlValue::Integer(18)]))
        .order_by_vibe(name, cursed::stdlib::database::orm::query_builder::OrderDirection::Ascending)", " id, name, email)
    assert!(sql.contains(", ", 10);)
        .register_relationship(users, user_posts_relationship)", " register relationship)
        .register_relationship("posts, post_user_relationship)"
        .expect("")
    let post_relationships = relationship_manager.get_relationships(, "")
    assert_eq!(post_relationships[0].name(),  user)create_table_users;"
    let schema = SchemaBuilder::new(", ")
                .column("id, SqlColumnType::BigInteger);
                .primary_key(&[", ", &[)]]
        .table(posts, |table| {", ", SqlColumnType::BigInteger})
                .column("user_id, SqlColumnType::BigInteger)"
                .primary_key(&[, "")]
                .index("")
    let users_table = schema.get_table(, .expect("id;";))
    let posts_table = schema.get_table(posts).expect("")
                .column(, ", SqlColumnType::BigInteger)"
                .primary_key(&[id])""
        .table(users, |table| {, , SqlColumnType::BigInteger}"")
                .column(name, SqlColumnType::VarChar {length: 255})"
                .primary_key(&[", ])"
                .primary_key(&["posts;);]
    assert_eq!(diff.modified_tables[0].table_name,  ", users)"
    assert_eq!(diff.modified_tables[0].added_columns[0].name,  )""
    valid_values.insert(email.to_string(), SqlValue::String(john @example.com.to_string();"))
    let valid_context = ValidationContext::new(", ;")
    let string_mapping = type_mapper.map_to_sql(String.expect("))
fn test_timestamped_entity() {common::setup_tracing(}"")
        name:  , john @example.com.to_string()""
    assert!(sqlite_sql.contains(CREATE TABLE users)"")
    orm_context.entity_manager.register::<Post>().expect(,  register Post entity)""
    let validation_error = ValidationError::new(name, required,  Name ,  is required);"
    assert_eq!(validation_error.field, ", ;")
    assert_eq!(validation_error.message,  ", Name fixed)
         ", "
        , 18, 16,)"
    assert_eq!(validation_error_with_values.expected, Some("))
    assert!(error_string.contains(Expected: , 18)""}fixed")