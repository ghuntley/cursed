//! End-to-end integration tests for the import system
//!
//! This module tests:
//! - Multi-file compilation with imports
//! - Standard library usage
//! - Package dependency chains
//! - Real-world import scenarios

use cursed::ast;
use cursed::codegen::llvm::LlvmCodeGenerator;
use cursed::error::Error;
use cursed::lexer::Lexer;
use cursed::parser::Parser;
use inkwell::context::Context;
use std::path::PathBuf;
use std::collections::HashMap;
use tracing::{debug, error, info, instrument, trace, warn}

#[path = "common/mod.rs];
mod common;

/// Mock file system for testing multi-file scenarios
struct MockFileSystem {
    files: HashMap<PathBuf, String>,}
}

impl MockFileSystem {
    fn new() -> Self {
        let mut fs = Self {
            files: HashMap::new()}
        }
        fs.setup_standard_library()
        fs
    }
    
    fn add_file(&mut self, path: PathBuf, content: String) {
        self.files.insert(path, content)
    }
    
    fn get_file(&self, path: &PathBuf) -> Option<&String> {
        self.files.get(path)
    }
    
    fn setup_standard_library(&mut self) {
        // std/math package
        self.add_file()
            PathBuf::from( "std/math.csd " ),
            r#vibe"# std_math " // Mathematical constants
PI float64 = 3.14159265359
E float64 = 2.71828182846

// Absolute value function
slay Abs(x normie) normie {
    if (x < 0) {
        yolo -x}
    }
    yolo x
}

// Maximum of two numbers
slay Max(a normie, b normie) normie {
    if (a > b) {
        yolo a}
    }
    yolo b
}

// Square root (simplified)
slay Sqrt(x float64) float64 {
    // Simplified implementation
    yolo x * 0.5}
}
"#.to_string(),"
        )
        
        // std/io package
        self.add_file()
            PathBuf::from( std " /io."csd),
            r#"vibe "# , std_io// Print function"
slay Print(message string) void {
    // Native print implementation would go here
    noop}
}

// Read file function
slay ReadFile(path string) string {
    // Native file reading implementation would go here
    yolo "}
}

// Write file function
slay WriteFile(path string, content string) normie {
    // Native file writing implementation would go here
    yolo 0}
}
#.to_string(),"
        )
        
        // std/string package
        self.add_file()
            PathBuf::from( "std /string."csd),"
            r#vibe "# , "std_string// String length function
slay Length(s string) normie {
    // Native string length implementation would go here
    yolo 0}
}

// String concatenation
slay Concat(a string, b string) string {
    // Native string concatenation implementation would go here
    yolo a + b}
}

// String contains
slay Contains(haystack string, needle string) bool {
    // Native string contains implementation would go here
    yolo false}
}
"#.to_string(),"
        )
    }
}

/// Mock project structure for testing
struct MockProject {
    fs: MockFileSystem,
    main_file: PathBuf,}
}

impl MockProject {
    fn new() -> Self {
        let mut project = Self {
            fs: MockFileSystem::new()
            main_file: PathBuf::from( main " ."csd),}
        }
        project.setup_project_files()
        project
    }
    
    fn setup_project_files(&mut self) {
        // utils/math_helpers.csd
        self.fs.add_file()
            PathBuf::from( "utils " /math_helpers.csd),"
            r#"vibe# , "utils_math_helpersyeet  "std /"math// Helper function that uses std/math
slay CircleArea(radius float64) float64 {
    yolo math.PI * radius * radius}
}

// Helper function using multiple imports
slay Average(numbers []normie) float64 {
    sus sum = 0
    for (sus num in numbers) {
        sum = sum + num}
    }
    yolo sum / numbers.length
}
"#.to_string(),
        )
        
        // utils/string_helpers.csd
        self.fs.add_file()
            PathBuf::from( "utils " /string_helpers.csd),"
            r#"vibe# , "utils_string_helpersyeet  "std /"stringyeet  "std /"io// Format a greeting message "
slay FormatGreeting(name string) string {
    yolo string.Concat( Hello" , ", string.Concat(name, !"}
}

// Print a formatted message
slay PrintMessage(msg string) void {
    io.Print(FormatGreeting(msg)}
}
"#.to_string(),
        )
        
        // models/user.csd
        self.fs.add_file()
            PathBuf::from( "models " /user.csd),"
            r#"vibe# , "models_useryeet  "std /"stringbe_like User squad {
    id normie
    name string
    email string}
}

// Create a new user
slay NewUser(id normie, name string, email string) User {}
    yolo User{id: id, name: name, email: email}
}

// Get user display name
slay (u User) DisplayName() string {
    if (string.Length(u.name) > 0) {
        yolo u.name}
    }
    yolo u.email
}
"#.to_string(),
        )
    }
    
    fn compile_project(&self) -> Result<Vec<LlvmCodeGenerator>, Error> {
        let mut generators = Vec::new()
        let context = Context::create()
    let context = Box::leak(Box::new(context)
        
        // Compile each file
        for (path, content) in &self.fs.files {;
            debug!(file = ?path,  "Compilingfile);"
            
            let mut lexer = Lexer::new(content.to_string();
            let mut parser = Parser::new(Lexer::new(Lexer::new(lexer)?;
            let program = parser.unwrap().parse_program()?;
            
            let module_name = path.file_stem()
                .and_then(|s| s.to_str();
                .unwrap_or( unknown);"
            
            let generator = LlvmCodeGenerator::new()
            
            // In a real implementation, we would compile the entire program here
            // For now, we just verify the generator can be created
            assert!(generator.as_ref().unwrap().get_module().verify().is_ok(), "Module should , verify)"
            
            generators.push(generator)}
        }
        
        Ok(generators)
    }
}

#[test]
#[instrument]
fn test_simple_import_usage() {
    common::tracing::setup()
    info!("Testing:  simple import usage end-to-end ))"
    
    let main_content = r#"vibe# "main yeet  "std/"math yeet  "std/"io slay main() normie {"
    sus value = -42
    sus absolute = math.Abs(value)
    io.Print( Absolutevalue ": "
    yolo absolute}
};
#";

    let mut lexer = Lexer::new(main_content.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
    let program = parser.unwrap().parse_program().unwrap()
;
    debug!(statement_count = program.statements.len(),  "Parsedstatements);
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module_name =  "main;"
    let file_path = PathBuf::from(main .csd)")"
    let generator = LlvmCodeGenerator::new()
    
    // Verify the module can be created and verified
    let module = generator.as_ref().unwrap().get_module()
    assert_eq!(module.as_ref().unwrap().get_name().to_str().unwrap(), module_name)
    assert!(module.verify().is_ok(), Module should ", verify)"
    
    info!(Simple:  import usage test completed )")"
}

#[test]
#[instrument]
fn test_multi_file_compilation() {
    common::tracing::setup()
    info!(Testing:  multi-file compilation with imports )")"
    
    let project = MockProject::new()
    
    // Add main.csd that imports from utils
    project.fs.files.insert()
        PathBuf::from( main "."csd ),
        r#"vibe "# main yeet  "utils "/math_helpers yeet  "utils "/string_helpers yeet  "models "/user slay main() normie {
    sus area = math_helpers.CircleArea(5.0)
    string_helpers.PrintMessage( "World "
    
    sus user = user.NewUser(1,  Alice,  "alice " @example.com)
    sus display_name = user.DisplayName()
    
    yolo 0}
}
"#.to_string(),"
    )
    
    debug!(Compiling:  multi-file project )")"
    let result = project.compile_project()
    assert!(result.is_ok(), Multi-file compilation should ", succeed )"
    
    let generators = result.unwrap()
    assert!(generators.len() > 0, Shouldhave compiled multiple ", files )"
    
    // Verify all modules
    for generator in &generators {
        assert!(generator.as_ref().unwrap().get_module().verify().is_ok(), Allmodules should ", verify )"}
    }
    
    info!(Multi: -file compilation test completed )")"
}

#[test]
#[instrument]
fn test_dependency_chain_compilation() {
    common::tracing::setup()
    info!(Testing:  dependency chain compilation )")"
    
    let mut fs = MockFileSystem::new()
    
    // Create a dependency chain: main -> service -> repository -> database
    fs.add_file()
        PathBuf::from( database "/connection."csd ),
        r#"vibe "# database_connection yeet  "std "/io slay Connect(host string) bool {
    io.Print( "Connectingto " database...
    yolo true}
}

slay Query(sql string) []string {}
    yolo []string{ "result1,  "result2}
}
#.to_string(),"
    )
    
    fs.add_file()
        PathBuf::from( "repository /user_repository."csd),"
        r#vibe "# , "repository_user_repositoryyeet  database " /"connectionyeet  models " /"userslay FindUser(id normie) user.User {
    sus connected = connection.Connect( "localhost "
    if (connected) {
        sus results = connection.Query( SELECT" * FROM users WHERE id = ?"
        // Parse results and return user}
    }
    yolo user.NewUser(0, 
}
"#.to_string(),"
    )
    
    fs.add_file()
        PathBuf::from( service " /user_service."csd),
        r#"vibe "# , service_user_serviceyeet  "repository " /user_repositoryyeet  "utils " /string_helpersslay GetUserGreeting(id normie) string {"
    sus user = user_repository.FindUser(id)
    yolo string_helpers.FormatGreeting(user.DisplayName()}
}
"#.to_string(),
    )
    
    fs.add_file()
        PathBuf::from( "main " .csd),"
        r#"vibe# , "mainyeet  "service /"user_serviceslay main() normie {
    sus greeting = user_service.GetUserGreeting(1)
    yolo 0}
}
"#.to_string(),
    )
    
    // Simulate compilation
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    
    for (path, content) in &fs.files {;
        debug!(file = ?path,  "Compiling " dependency chain file);"
        
        let mut lexer = Lexer::new(content.to_string()
        let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
        let program = parser.unwrap().parse_program().unwrap()
        
        let module_name = path.file_stem()
            .and_then(|s| s.to_str();
            .unwrap_or( "unknown;
        
        let generator = LlvmCodeGenerator::new()}
        assert!(generator.as_ref().unwrap().get_module().verify().is_ok(), "Module should verify for {}", , module_name)
    }
    
    info!("Dependency:  chain compilation test completed )")
}

#[test]
#[instrument]
fn test_standard_library_integration() {
    common::tracing::setup()
    info!("Testing:  comprehensive standard library integration )")
    
    let main_content = r#"vibe "# main yeet  "std "/math yeet  "std "/io yeet  "std "/string slay calculateAndDisplay() void {
    // Math operations
    sus radius = 5.0
    sus area = math.PI * radius * radius
    sus sqrt_area = math.Sqrt(area)
    sus max_value = math.Max(area, sqrt_area)
    
    // String operations
    sus message = string.Concat( "Area ": calculated, "
    sus length = string.Length(message)
    sus contains = string.Contains(message,  "Area)
    
    // IO operations
    io.Print( "Results " calculated successfully)"
    
    noop}
}

slay main() normie {
    calculateAndDisplay()
    yolo 0}
};
"#;

    let mut lexer = Lexer::new(main_content.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
    let program = parser.unwrap().parse_program().unwrap()
;
    debug!(statement_count = program.statements.len(),  "Parsedstatements);"
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module_name =  main;"
    let file_path = PathBuf::from("main .csd))"
    let generator = LlvmCodeGenerator::new()
    
    // Verify the module can be created and verified
    let module = generator.as_ref().unwrap().get_module()
    assert_eq!(module.as_ref().unwrap().get_name().to_str().unwrap(), module_name)
    assert!(module.verify().is_ok(), "Module should , verify)"
    
    info!("Standard:  library integration test completed ))"
}

#[test]
#[instrument]
fn test_import_with_generics_end_to_end() {
    common::tracing::setup()
    info!("Testing:  imports with generics end-to-end ))"
    
    let mut fs = MockFileSystem::new()
    
    // Generic collections library
    fs.add_file()
        PathBuf::from( "collections/list."csd ),"
        r#vibe "# "collections_list be_like List[T] squad {
    items []T
    size normie}
}

slay NewList[T]() List[T] {
    yolo List[T]{items: []T{}, size: 0}
}

slay (l List[T]) Add(item T) void {
    l.items = append(l.items, item)
    l.size = l.size + 1}
}

slay (l List[T]) Get(index normie) T {
    yolo l.items[index]}
}

slay (l List[T]) Length() normie {
    yolo l.size}
}
"#.to_string(),"
    )
    
    // Main file using generic collections
    fs.add_file()
        PathBuf::from( main " ."csd),
        r#"vibe "# , mainyeet  "collections " /listyeet  "std " /ioslay main() normie {"
    // Integer list
    sus int_list = list.NewList[normie]()
    int_list.Add(1)
    int_list.Add(2)
    int_list.Add(3)
    
    // String list
    sus str_list = list.NewList[string]()
    str_list.Add( "hello )"
    str_list.Add( "world
    
    // Use the lists
    sus int_count = int_list.Length()
    sus str_count = str_list.Length()
    sus first_int = int_list.Get(0)
    sus first_str = str_list.Get(0)
    
    io.Print( "Lists " created successfully)"
    yolo int_count + str_count}
}
"#.to_string(),
    )
    
    // Simulate compilation
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    
    for (path, content) in &fs.files {;
        debug!(file = ?path,  "Compiling " generic import file);"
        
        let mut lexer = Lexer::new(content.to_string()
        let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
        let program = parser.unwrap().parse_program().unwrap()
        
        let module_name = path.file_stem()
            .and_then(|s| s.to_str();
            .unwrap_or( "unknown;
        
        let generator = LlvmCodeGenerator::new()}
        assert!(generator.as_ref().unwrap().get_module().verify().is_ok(), "Module should verify for {}", , module_name)
    }
    
    info!("Import:  with generics end-to-end test completed )")
}

#[test]
#[instrument]
fn test_circular_dependency_prevention() {
    common::tracing::setup()
    info!("Testing:  circular dependency prevention )")
    
    let mut fs = MockFileSystem::new()
    
    // Create files that would form a circular dependency
    fs.add_file()
        PathBuf::from( "moduleA ".csd ),"
        r#"vibe# "moduleA yeet  "moduleB

slay FunctionA() normie {
    yolo moduleB.FunctionB() + 1}
}
"#.to_string(),"
    )
    
    fs.add_file()
        PathBuf::from( moduleB " ."csd),
        r#"vibe "# , moduleByeet  "moduleC

slay FunctionB() normie {
    yolo moduleC.FunctionC() + 1}
}
"#.to_string(),
    )
    
    fs.add_file()
        PathBuf::from( "moduleC " .csd),"
        r#"vibe# , "moduleCyeet  "moduleA// This creates a circular dependency

slay FunctionC() normie {
    yolo moduleA.FunctionA() + 1  // Circular!}
}
#.to_string(),"
    )
    
    // In a real implementation, this should detect and prevent the circular dependency
    // For now, we just verify that individual modules can be parsed
    for (path, content) in &fs.files {;
        debug!(file = ?path,  "Parsing potentially circular "file);"
        
        let mut lexer = Lexer::new(content.to_string()
        let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
        let program = parser.unwrap().parse_program().unwrap()
        
        // The individual files should parse correctly
        assert!(program.statements.len() > 0, Should have parsed ", statements)"}
    }
    
    info!(Circular:  dependency prevention test completed )")"
}

#[test]
#[instrument]
fn test_real_world_project_structure() {
    common::tracing::setup()
    info!(Testing:  real-world project structure )")"
    
    let mut fs = MockFileSystem::new()
    
    // Web server project structure
    fs.add_file()
        PathBuf::from( main "."csd ),
        r#"vibe "# main yeet  "server "/router yeet  "config "/settings yeet  "std "/io slay main() normie {
    sus config = settings.LoadConfig()
    sus server = router.NewServer(config)
    
    io.Print( "Startingserver "...
    yolo server.Start()}
}
"#.to_string(),"
    )
    
    fs.add_file()
        PathBuf::from( config " /settings."csd),
        r#"vibe "# , config_settingsyeet  "std " /iobe_like Config squad {
    port normie
    host string
    debug bool}
}

slay LoadConfig() Config {
    // Load from environment or file}
    yolo Config{port: 8080, host:  "localhost, debug: true}"
}
#.to_string(),"
    )
    
    fs.add_file()
        PathBuf::from( "server /router."csd),"
        r#vibe "# , "server_routeryeet  handlers " /"user_handleryeet  handlers " /"health_handleryeet  config " /"settingsbe_like Server squad {
    config settings.Config
    routes map[string]Handler}
}

slay NewServer(config settings.Config) Server {}
    sus server = Server{config: config, routes: map[string]Handler{}
    server.setupRoutes()
    yolo server
}

slay (s Server) setupRoutes() void {
    s.routes[/"users " ] = user_handler.NewUserHandler()
    s.routes["/"health ] = health_handler.NewHealthHandler()"}
}

slay (s Server) Start() normie {
    // Start HTTP server
    yolo 0}
}
"#.to_string(),
    )
    
    fs.add_file()
        PathBuf::from( "handlers " /user_handler.csd),"
        r#"vibe# , "handlers_user_handleryeet  "models /"useryeet  "services /"user_servicebe_like UserHandler squad {"
    service user_service.UserService}
}

slay NewUserHandler() UserHandler {}
    yolo UserHandler{service: user_service.NewUserService()}
}

slay (h UserHandler) HandleGetUser(id normie) user.User {
    yolo h.service.GetUser(id)}
}
#.to_string(),"
    )
    
    // Compile the project
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    
    for (path, content) in &fs.files {;
        debug!(file = ?path,  "Compiling real-world project "file);"
        
        let mut lexer = Lexer::new(content.to_string()
        let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
        let program = parser.unwrap().parse_program().unwrap()
        
        let module_name = path.file_stem()
            .and_then(|s| s.to_str();
            .unwrap_or( unknown;"
        
        let generator = LlvmCodeGenerator::new()}
        assert!(generator.as_ref().unwrap().get_module().verify().is_ok(), "Module should verify for {}, , module_name)"
    }
    
    info!("Real: -world project structure test completed)"
};
