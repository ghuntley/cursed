/// Comprehensive tests for GraphQL template format rendering
use std::collections::HashMap;
use cursed::object::Object as CursedObject;
use cursed::stdlib::template::template_formats::{TemplateFormatRenderer, TemplateFormat, ApiFormat};

#[test]
fn test_graphql_basic_schema() {
    let renderer = TemplateFormatRenderer::new(TemplateFormat::Api(ApiFormat::GraphQL));
    
    // Basic schema with Query type
    let mut schema_def = HashMap::new();
    schema_def.insert("query".to_string(), CursedObject::String("Query".to_string()));
    
    let mut user_field = HashMap::new();
    user_field.insert("name".to_string(), CursedObject::String("user".to_string()));
    user_field.insert("type".to_string(), CursedObject::String("User".to_string()));
    user_field.insert("description".to_string(), CursedObject::String("Get a user by ID".to_string()));
    
    let mut id_arg = HashMap::new();
    id_arg.insert("name".to_string(), CursedObject::String("id".to_string()));
    id_arg.insert("type".to_string(), CursedObject::String("ID!".to_string()));
    
    user_field.insert("arguments".to_string(), CursedObject::Array(vec![CursedObject::Map(id_arg)]));
    
    let mut query_type = HashMap::new();
    query_type.insert("kind".to_string(), CursedObject::String("object".to_string()));
    query_type.insert("name".to_string(), CursedObject::String("Query".to_string()));
    query_type.insert("fields".to_string(), CursedObject::Array(vec![CursedObject::Map(user_field)]));
    
    let mut user_type = HashMap::new();
    user_type.insert("kind".to_string(), CursedObject::String("object".to_string()));
    user_type.insert("name".to_string(), CursedObject::String("User".to_string()));
    
    let mut name_field = HashMap::new();
    name_field.insert("name".to_string(), CursedObject::String("name".to_string()));
    name_field.insert("type".to_string(), CursedObject::String("String!".to_string()));
    
    let mut email_field = HashMap::new();
    email_field.insert("name".to_string(), CursedObject::String("email".to_string()));
    email_field.insert("type".to_string(), CursedObject::String("String".to_string()));
    
    user_type.insert("fields".to_string(), CursedObject::Array(vec![
        CursedObject::Map(name_field),
        CursedObject::Map(email_field),
    ]));
    
    let mut schema = HashMap::new();
    schema.insert("schema".to_string(), CursedObject::Map(schema_def));
    schema.insert("types".to_string(), CursedObject::Array(vec![
        CursedObject::Map(query_type),
        CursedObject::Map(user_type),
    ]));
    
    let data = CursedObject::Map(schema);
    let result = renderer.render(&data).unwrap();
    
    assert!(result.contains("schema {"));
    assert!(result.contains("query: Query"));
    assert!(result.contains("type Query {"));
    assert!(result.contains("user(id: ID!): User"));
    assert!(result.contains("type User {"));
    assert!(result.contains("name: String!"));
    assert!(result.contains("email: String"));
}

#[test]
fn test_graphql_interface_and_union() {
    let renderer = TemplateFormatRenderer::new(TemplateFormat::Api(ApiFormat::GraphQL));
    
    // Interface type
    let mut node_interface = HashMap::new();
    node_interface.insert("kind".to_string(), CursedObject::String("interface".to_string()));
    node_interface.insert("name".to_string(), CursedObject::String("Node".to_string()));
    node_interface.insert("description".to_string(), CursedObject::String("An object with an ID".to_string()));
    
    let mut id_field = HashMap::new();
    id_field.insert("name".to_string(), CursedObject::String("id".to_string()));
    id_field.insert("type".to_string(), CursedObject::String("ID!".to_string()));
    id_field.insert("description".to_string(), CursedObject::String("The ID of the object".to_string()));
    
    node_interface.insert("fields".to_string(), CursedObject::Array(vec![CursedObject::Map(id_field)]));
    
    // Union type
    let mut search_result = HashMap::new();
    search_result.insert("kind".to_string(), CursedObject::String("union".to_string()));
    search_result.insert("name".to_string(), CursedObject::String("SearchResult".to_string()));
    search_result.insert("types".to_string(), CursedObject::Array(vec![
        CursedObject::String("User".to_string()),
        CursedObject::String("Post".to_string()),
    ]));
    
    let mut schema = HashMap::new();
    schema.insert("types".to_string(), CursedObject::Array(vec![
        CursedObject::Map(node_interface),
        CursedObject::Map(search_result),
    ]));
    
    let data = CursedObject::Map(schema);
    let result = renderer.render(&data).unwrap();
    
    assert!(result.contains("interface Node {"));
    assert!(result.contains("An object with an ID"));
    assert!(result.contains("id: ID!"));
    assert!(result.contains("union SearchResult = User | Post"));
}

#[test]
fn test_graphql_enum_and_input() {
    let renderer = TemplateFormatRenderer::new(TemplateFormat::Api(ApiFormat::GraphQL));
    
    // Enum type
    let mut status_enum = HashMap::new();
    status_enum.insert("kind".to_string(), CursedObject::String("enum".to_string()));
    status_enum.insert("name".to_string(), CursedObject::String("Status".to_string()));
    
    let mut active_value = HashMap::new();
    active_value.insert("name".to_string(), CursedObject::String("ACTIVE".to_string()));
    active_value.insert("description".to_string(), CursedObject::String("User is active".to_string()));
    
    let mut inactive_value = HashMap::new();
    inactive_value.insert("name".to_string(), CursedObject::String("INACTIVE".to_string()));
    
    status_enum.insert("values".to_string(), CursedObject::Array(vec![
        CursedObject::Map(active_value),
        CursedObject::Map(inactive_value),
    ]));
    
    // Input type
    let mut user_input = HashMap::new();
    user_input.insert("kind".to_string(), CursedObject::String("input".to_string()));
    user_input.insert("name".to_string(), CursedObject::String("UserInput".to_string()));
    
    let mut name_field = HashMap::new();
    name_field.insert("name".to_string(), CursedObject::String("name".to_string()));
    name_field.insert("type".to_string(), CursedObject::String("String!".to_string()));
    
    let mut status_field = HashMap::new();
    status_field.insert("name".to_string(), CursedObject::String("status".to_string()));
    status_field.insert("type".to_string(), CursedObject::String("Status".to_string()));
    status_field.insert("defaultValue".to_string(), CursedObject::String("ACTIVE".to_string()));
    
    user_input.insert("fields".to_string(), CursedObject::Array(vec![
        CursedObject::Map(name_field),
        CursedObject::Map(status_field),
    ]));
    
    let mut schema = HashMap::new();
    schema.insert("types".to_string(), CursedObject::Array(vec![
        CursedObject::Map(status_enum),
        CursedObject::Map(user_input),
    ]));
    
    let data = CursedObject::Map(schema);
    let result = renderer.render(&data).unwrap();
    
    assert!(result.contains("enum Status {"));
    assert!(result.contains("ACTIVE"));
    assert!(result.contains("INACTIVE"));
    assert!(result.contains("User is active"));
    assert!(result.contains("input UserInput {"));
    assert!(result.contains("name: String!"));
    assert!(result.contains("status: Status = \"ACTIVE\""));
}

#[test]
fn test_graphql_directives() {
    let renderer = TemplateFormatRenderer::new(TemplateFormat::Api(ApiFormat::GraphQL));
    
    // Custom directive definition
    let mut auth_directive = HashMap::new();
    auth_directive.insert("name".to_string(), CursedObject::String("auth".to_string()));
    auth_directive.insert("description".to_string(), CursedObject::String("Requires authentication".to_string()));
    
    let mut role_arg = HashMap::new();
    role_arg.insert("name".to_string(), CursedObject::String("role".to_string()));
    role_arg.insert("type".to_string(), CursedObject::String("String".to_string()));
    
    auth_directive.insert("arguments".to_string(), CursedObject::Array(vec![CursedObject::Map(role_arg)]));
    auth_directive.insert("locations".to_string(), CursedObject::Array(vec![
        CursedObject::String("FIELD_DEFINITION".to_string()),
        CursedObject::String("OBJECT".to_string()),
    ]));
    
    // Type with directive usage
    let mut user_type = HashMap::new();
    user_type.insert("kind".to_string(), CursedObject::String("object".to_string()));
    user_type.insert("name".to_string(), CursedObject::String("User".to_string()));
    
    let mut auth_directive_usage = HashMap::new();
    auth_directive_usage.insert("name".to_string(), CursedObject::String("auth".to_string()));
    
    let mut role_args = HashMap::new();
    role_args.insert("role".to_string(), CursedObject::String("admin".to_string()));
    auth_directive_usage.insert("arguments".to_string(), CursedObject::Map(role_args));
    
    user_type.insert("directives".to_string(), CursedObject::Array(vec![CursedObject::Map(auth_directive_usage)]));
    
    let mut name_field = HashMap::new();
    name_field.insert("name".to_string(), CursedObject::String("name".to_string()));
    name_field.insert("type".to_string(), CursedObject::String("String!".to_string()));
    
    user_type.insert("fields".to_string(), CursedObject::Array(vec![CursedObject::Map(name_field)]));
    
    let mut schema = HashMap::new();
    schema.insert("directives".to_string(), CursedObject::Array(vec![CursedObject::Map(auth_directive)]));
    schema.insert("types".to_string(), CursedObject::Array(vec![CursedObject::Map(user_type)]));
    
    let data = CursedObject::Map(schema);
    let result = renderer.render(&data).unwrap();
    
    assert!(result.contains("directive @auth"));
    assert!(result.contains("role: String"));
    assert!(result.contains("on FIELD_DEFINITION | OBJECT"));
    assert!(result.contains("type User @auth(role: \"admin\") {"));
    assert!(result.contains("name: String!"));
}

#[test]
fn test_graphql_federation_directives() {
    let renderer = TemplateFormatRenderer::new(TemplateFormat::Api(ApiFormat::GraphQL));
    
    // Federation directive usage
    let mut user_type = HashMap::new();
    user_type.insert("kind".to_string(), CursedObject::String("object".to_string()));
    user_type.insert("name".to_string(), CursedObject::String("User".to_string()));
    
    let mut key_directive = HashMap::new();
    key_directive.insert("name".to_string(), CursedObject::String("key".to_string()));
    
    let mut fields_args = HashMap::new();
    fields_args.insert("fields".to_string(), CursedObject::String("id".to_string()));
    key_directive.insert("arguments".to_string(), CursedObject::Map(fields_args));
    
    user_type.insert("directives".to_string(), CursedObject::Array(vec![CursedObject::Map(key_directive)]));
    
    let mut id_field = HashMap::new();
    id_field.insert("name".to_string(), CursedObject::String("id".to_string()));
    id_field.insert("type".to_string(), CursedObject::String("ID!".to_string()));
    
    let mut external_directive = HashMap::new();
    external_directive.insert("name".to_string(), CursedObject::String("external".to_string()));
    
    let mut name_field = HashMap::new();
    name_field.insert("name".to_string(), CursedObject::String("name".to_string()));
    name_field.insert("type".to_string(), CursedObject::String("String".to_string()));
    name_field.insert("directives".to_string(), CursedObject::Array(vec![CursedObject::Map(external_directive)]));
    
    user_type.insert("fields".to_string(), CursedObject::Array(vec![
        CursedObject::Map(id_field),
        CursedObject::Map(name_field),
    ]));
    
    let mut schema = HashMap::new();
    schema.insert("types".to_string(), CursedObject::Array(vec![CursedObject::Map(user_type)]));
    
    let data = CursedObject::Map(schema);
    let result = renderer.render(&data).unwrap();
    
    assert!(result.contains("type User @key(fields: \"id\") {"));
    assert!(result.contains("id: ID!"));
    assert!(result.contains("name: String @external"));
}

#[test]
fn test_graphql_complete_schema() {
    let renderer = TemplateFormatRenderer::new(TemplateFormat::Api(ApiFormat::GraphQL));
    
    // Complete schema example
    let mut schema_def = HashMap::new();
    schema_def.insert("query".to_string(), CursedObject::String("Query".to_string()));
    schema_def.insert("mutation".to_string(), CursedObject::String("Mutation".to_string()));
    schema_def.insert("subscription".to_string(), CursedObject::String("Subscription".to_string()));
    
    // Custom scalar
    let mut date_scalar = HashMap::new();
    date_scalar.insert("name".to_string(), CursedObject::String("Date".to_string()));
    date_scalar.insert("description".to_string(), CursedObject::String("ISO 8601 date string".to_string()));
    
    let mut schema = HashMap::new();
    schema.insert("description".to_string(), CursedObject::String("A complete example schema".to_string()));
    schema.insert("schema".to_string(), CursedObject::Map(schema_def));
    schema.insert("scalars".to_string(), CursedObject::Array(vec![CursedObject::Map(date_scalar)]));
    
    let data = CursedObject::Map(schema);
    let result = renderer.render(&data).unwrap();
    
    assert!(result.contains("A complete example schema"));
    assert!(result.contains("schema {"));
    assert!(result.contains("query: Query"));
    assert!(result.contains("mutation: Mutation"));
    assert!(result.contains("subscription: Subscription"));
    assert!(result.contains("scalar Date"));
    assert!(result.contains("ISO 8601 date string"));
}
