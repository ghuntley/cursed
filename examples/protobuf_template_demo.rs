use cursed::stdlib::template::template_formats::ModuleHandler;
use std::collections::HashMap;

fn main() {
    println!("🔨 Protocol Buffers Template Renderer Demo");
    
    let renderer = ModuleHandler::new();
    
    // Create a simple protobuf schema representation as a string
    let protobuf_schema = r#"
syntax = "proto3";

package com.example.api;

import "google/protobuf/timestamp.proto";
import "google/protobuf/empty.proto";

option java_package = "com.example.api";
option go_package = "github.com/example/api";
option csharp_namespace = "Example.Api";

enum Status {
  UNKNOWN = 0;
  ACTIVE = 1;
  INACTIVE = 2;
}

message User {
  string name = 1;
  int32 age = 2;
  repeated string tags = 3;
}

service UserService {
  rpc GetUser(GetUserRequest) returns (User);
  rpc ListUsers(ListUsersRequest) returns (stream ListUsersResponse);
}
"#;
    
    // Process the protobuf schema through the template renderer
    match renderer.process(protobuf_schema) {
        Ok(proto_output) => {
            println!("\n✅ Generated Protocol Buffers Definition:");
            println!("{}", "-".repeat(60));
            println!("{}", proto_output);
            println!("{}", "-".repeat(60));
            
            // Validate the output contains expected elements
            assert!(proto_output.contains("proto3"));
            assert!(proto_output.contains("package com.example.api"));
            assert!(proto_output.contains("import \"google/protobuf/timestamp.proto\""));
            assert!(proto_output.contains("option java_package = \"com.example.api\""));
            assert!(proto_output.contains("enum Status"));
            assert!(proto_output.contains("UNKNOWN = 0"));
            assert!(proto_output.contains("ACTIVE = 1"));
            assert!(proto_output.contains("message User"));
            assert!(proto_output.contains("string name = 1"));
            assert!(proto_output.contains("int32 age = 2"));
            assert!(proto_output.contains("repeated string tags = 3"));
            assert!(proto_output.contains("service UserService"));
            assert!(proto_output.contains("rpc GetUser(GetUserRequest) returns (User)"));
            assert!(proto_output.contains("rpc ListUsers(ListUsersRequest) returns (stream ListUsersResponse)"));
            
            println!("\n🎉 All validation checks passed!");
        }
        Err(e) => {
            eprintln!("❌ Error rendering protobuf: {:?}", e);
            std::process::exit(1);
        }
    }
}
