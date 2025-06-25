use cursed::stdlib::template::template_formats::{TemplateFormatRenderer, TemplateFormat, ApiFormat};
use cursed::object::Object as CursedObject;
use std::collections::HashMap;

fn main() {
    println!("🔨 Protocol Buffers Template Renderer Demo");
    
    let renderer = TemplateFormatRenderer::new(
        TemplateFormat::Api(ApiFormat::Protobuf)
    );
    
    // Create a comprehensive protobuf schema
    let mut user_fields = Vec::new();
    
    // User name field
    let mut name_field = HashMap::new();
    name_field.insert("number".to_string(), CursedObject::Integer(1));
    name_field.insert("type".to_string(), CursedObject::String("string".to_string()));
    name_field.insert("name".to_string(), CursedObject::String("name".to_string()));
    user_fields.push(CursedObject::Map(name_field));
    
    // User age field with options
    let mut age_field = HashMap::new();
    age_field.insert("number".to_string(), CursedObject::Integer(2));
    age_field.insert("type".to_string(), CursedObject::String("int32".to_string()));
    age_field.insert("name".to_string(), CursedObject::String("age".to_string()));
    let mut age_options = HashMap::new();
    age_options.insert("deprecated".to_string(), CursedObject::Boolean(false));
    age_field.insert("options".to_string(), CursedObject::Map(age_options));
    user_fields.push(CursedObject::Map(age_field));
    
    // Tags field (repeated)
    let mut tags_field = HashMap::new();
    tags_field.insert("number".to_string(), CursedObject::Integer(3));
    tags_field.insert("type".to_string(), CursedObject::String("string".to_string()));
    tags_field.insert("name".to_string(), CursedObject::String("tags".to_string()));
    tags_field.insert("repeated".to_string(), CursedObject::Boolean(true));
    user_fields.push(CursedObject::Map(tags_field));
    
    // User message
    let mut user_message = HashMap::new();
    user_message.insert("name".to_string(), CursedObject::String("User".to_string()));
    user_message.insert("fields".to_string(), CursedObject::Array(user_fields));
    
    // Status enum
    let mut enum_values = Vec::new();
    
    let mut unknown_value = HashMap::new();
    unknown_value.insert("name".to_string(), CursedObject::String("UNKNOWN".to_string()));
    unknown_value.insert("number".to_string(), CursedObject::Integer(0));
    enum_values.push(CursedObject::Map(unknown_value));
    
    let mut active_value = HashMap::new();
    active_value.insert("name".to_string(), CursedObject::String("ACTIVE".to_string()));
    active_value.insert("number".to_string(), CursedObject::Integer(1));
    enum_values.push(CursedObject::Map(active_value));
    
    let mut inactive_value = HashMap::new();
    inactive_value.insert("name".to_string(), CursedObject::String("INACTIVE".to_string()));
    inactive_value.insert("number".to_string(), CursedObject::Integer(2));
    enum_values.push(CursedObject::Map(inactive_value));
    
    let mut status_enum = HashMap::new();
    status_enum.insert("name".to_string(), CursedObject::String("Status".to_string()));
    status_enum.insert("values".to_string(), CursedObject::Array(enum_values));
    
    // RPC methods for UserService
    let mut get_user_method = HashMap::new();
    get_user_method.insert("name".to_string(), CursedObject::String("GetUser".to_string()));
    get_user_method.insert("input_type".to_string(), CursedObject::String("GetUserRequest".to_string()));
    get_user_method.insert("output_type".to_string(), CursedObject::String("User".to_string()));
    
    let mut list_users_method = HashMap::new();
    list_users_method.insert("name".to_string(), CursedObject::String("ListUsers".to_string()));
    list_users_method.insert("input_type".to_string(), CursedObject::String("ListUsersRequest".to_string()));
    list_users_method.insert("output_type".to_string(), CursedObject::String("ListUsersResponse".to_string()));
    list_users_method.insert("server_streaming".to_string(), CursedObject::Boolean(true));
    
    let mut user_service = HashMap::new();
    user_service.insert("name".to_string(), CursedObject::String("UserService".to_string()));
    user_service.insert("methods".to_string(), CursedObject::Array(vec![
        CursedObject::Map(get_user_method),
        CursedObject::Map(list_users_method)
    ]));
    
    // File-level options
    let mut options = HashMap::new();
    options.insert("java_package".to_string(), CursedObject::String("com.example.api".to_string()));
    options.insert("go_package".to_string(), CursedObject::String("github.com/example/api".to_string()));
    options.insert("csharp_namespace".to_string(), CursedObject::String("Example.Api".to_string()));
    
    // Complete schema
    let mut schema = HashMap::new();
    schema.insert("syntax".to_string(), CursedObject::String("proto3".to_string()));
    schema.insert("package".to_string(), CursedObject::String("com.example.api".to_string()));
    schema.insert("imports".to_string(), CursedObject::Array(vec![
        CursedObject::String("google/protobuf/timestamp.proto".to_string()),
        CursedObject::String("google/protobuf/empty.proto".to_string())
    ]));
    schema.insert("options".to_string(), CursedObject::Map(options));
    schema.insert("enums".to_string(), CursedObject::Array(vec![CursedObject::Map(status_enum)]));
    schema.insert("messages".to_string(), CursedObject::Array(vec![CursedObject::Map(user_message)]));
    schema.insert("services".to_string(), CursedObject::Array(vec![CursedObject::Map(user_service)]));
    
    let data = CursedObject::Map(schema);
    
    match renderer.render(&data) {
        Ok(proto_output) => {
            println!("\n✅ Generated Protocol Buffers Definition:");
            println!("{}", "-".repeat(60));
            println!("{}", proto_output);
            println!("{}", "-".repeat(60));
            
            // Validate the output contains expected elements
            assert!(proto_output.contains("syntax = \"proto3\";"));
            assert!(proto_output.contains("package com.example.api;"));
            assert!(proto_output.contains("import \"google/protobuf/timestamp.proto\";"));
            assert!(proto_output.contains("option java_package = \"com.example.api\";"));
            assert!(proto_output.contains("enum Status {"));
            assert!(proto_output.contains("UNKNOWN = 0;"));
            assert!(proto_output.contains("ACTIVE = 1;"));
            assert!(proto_output.contains("message User {"));
            assert!(proto_output.contains("string name = 1;"));
            assert!(proto_output.contains("int32 age = 2"));
            assert!(proto_output.contains("repeated string tags = 3;"));
            assert!(proto_output.contains("service UserService {"));
            assert!(proto_output.contains("rpc GetUser(GetUserRequest) returns (User);"));
            assert!(proto_output.contains("rpc ListUsers(ListUsersRequest) returns (stream ListUsersResponse);"));
            
            println!("\n🎉 All validation checks passed!");
        }
        Err(e) => {
            eprintln!("❌ Error rendering protobuf: {:?}", e);
            std::process::exit(1);
        }
    }
}
