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
use tracing::  {debug, error, info, instrument, trace, warn}

#[path = "common/mod.rs"# std_math " // Mathematical constants
PI float64 = 3.14159265359
E float64 = 2.71828182846

// Absolute value function
slay Abs(x normie) normie {if (x < 0)     {yolo -x}
    yolo x}

// Maximum of two numbers
slay Max(a normie, b normie) normie {if (a > b)     {yolo a}
    yolo b}

// Square root (simplified)
slay Sqrt(x float64) float64 {// Simplified implementation
    yolo x * 0.5}
#.to_string(),)
        
        // std/io package
        self.add_file()
            PathBuf::from(std  /io.csd),
            r#"# , std_io// Print function"#
slay Print(message string) void {// Native print implementation would go here
    noop}

// Read file function
slay ReadFile(path string) string {// Native file reading implementation would go here
    yolo}

// Write file function
slay WriteFile(path string, content string) normie {// Native file writing implementation would go here
    yolo 0}
#.to_string(),)
        
        // std/string package
        self.add_file()
            PathBuf::from(std /string.csd),"
            r#vibe "std_string// String length function
slay Length(s string) normie {// Native string length implementation would go here
    yolo 0}

// String concatenation
slay Concat(a string, b string) string {// Native string concatenation implementation would go here
    yolo a + b}

// String contains
slay Contains(haystack string, needle string) bool {// Native string contains implementation would go here
    yolo false}
#.to_string(),)}

/// Mock project structure for testing
struct MockProject {fs: MockFileSystem,
    main_file: PathBuf}

impl MockProject     {fn new() {let mut project = Self {fs: MockFileSystem::new()
            main_file: PathBuf::from(main  .csd)}
        project.setup_project_files()
        project}
    
    fn setup_project_files() {// utils/math_helpers.csd
        self.fs.add_file()
            PathBuf::from(utils  /math_helpers.csd),"
            r#"utils_math_helpersyeet  "std /"
            r#"vibe# , "std /"stringyeet  "io// Format a greeting message 
slay FormatGreeting(name string) string {yolo string.Concat(Hello, ", string.Concat(name, !"
            r#"vibe# , "std /"stringbe_like User squad {id normie
    name string
    email string}

// Create a new user
slay NewUser(id normie, name string, email string) User {}
    yolo User{id: id, name: name, email: email}

// Get user display name
slay (u User) DisplayName() string {if (string.Length(u.name) > 0)     {yolo u.name}
    yolo u.email}
#.to_string(),)}
    
    fn compile_project() {let mut generators = Vec::new()
        let context = Context::create()
    let context = Box::leak(Box::new(context)
        
        // Compile each file
        for (path, content) in &self.fs.files    {debug!(file = ?path,  Compilingfile);
            
            let mut lexer = Lexer::new(content.to_string();
            let mut parser = Parser::new(Lexer::new(Lexer::new(lexer)?;
            let program = parser.unwrap().parse_program()?;
            
            let module_name = path.file_stem()
                .and_then(|s| s.to_str();
                .unwrap_or(unknown);"Testing:  simple import usage end-to-end);
    
    let main_content = r#"main yeet  "std/"std/"io slay main() normie {": 
    yolo absolute};
#"Parsedstatements);
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module_name =  "main;")
    let generator = LlvmCodeGenerator::new()
    // Verify the module can be created and verified
    let module = generator.as_ref().unwrap().get_module()
    assert_eq!(module.as_ref().unwrap().get_name().to_str().unwrap(), module_name)
    assert!(module.verify().is_ok(), Module should , verify)
    
    info!(Simple:  import usage test completed)"}
#[test]
#[instrument]
fn test_multi_file_compilation() {common::tracing::setup()
    info!(Testing:  multi-file compilation with imports)")"vibe "# main yeet  "/math_helpers yeet  "utils "models "/user slay main() normie {sus area = math_helpers.CircleArea(5.0)
    string_helpers.PrintMessage(
    
    sus user = user.NewUser(1,  Alice,  "alice "#.to_string(),")
    
    debug!(Compiling:  multi-file project)
    let result = project.compile_project()
    assert!(result.is_ok(), Multi-file compilation should ", succeed)", files)
    
    // Verify all modules
    for generator in &generators   {assert!(generator.as_ref().unwrap().get_module().verify().is_ok(), Allmodules should , verify)}
    
    info!(Multi: -file compilation test completed)"}
#[test]
#[instrument]
fn test_dependency_chain_compilation() {common::tracing::setup()
    info!(Testing:  dependency chain compilation)")"vibe "# database_connection yeet  "/io slay Connect(host string) bool   {io.Print("Connectingto "result1,  "result2}
#.to_string(),"repository /user_repository."csd),"# , "repository_user_repositoryyeet  database "connectionyeet  models " /"localhost "
    if (connected)     {sus results = connection.Query(SELECT
        // Parse results and return user}
    yolo user.NewUser(0,}
#.to_string(),)
    
    fs.add_file()
        PathBuf::from(service " /user_service."vibe "# , service_user_serviceyeet  " /user_repositoryyeet  "utils 
    sus user = user_repository.FindUser(id)
    yolo string_helpers.FormatGreeting(user.DisplayName()}
"#.to_string(),)
    fs.add_file()
        PathBuf::from(" .csd),"
        r#"mainyeet  "service /"#.to_string(),)
    // Simulate compilation
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    
    for (path, content) in &fs.files    {debug!(file = ?path,  Compiling  dependency chain file);
        
        let mut lexer = Lexer::new(content.to_string()
        let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
        let program = parser.unwrap().parse_program().unwrap()
        
        let module_name = path.file_stem()
            .and_then(|s| s.to_str();
            .unwrap_or("Module should verify for   {}, , module_name)}
    
    info!("Dependency:  chain compilation test completed)"Testing:  comprehensive standard library integration)")
    
    let main_content = r#"# main yeet  "std "std "/io yeet  "/string slay calculateAndDisplay() void {// Math operations
    sus radius = 5.0
    sus area = math.PI * radius * radius
    sus sqrt_area = math.Sqrt(area)
    sus max_value = math.Max(area, sqrt_area)
    
    // String operations
    sus message = string.Concat(Area : calculated, 
    sus length = string.Length(message)
    sus contains = string.Contains(message,  
    
    noop}

slay main() normie {calculateAndDisplay()
    yolo 0};
"#;
    let mut lexer = Lexer::new(main_content.to_string()
    let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
    let program = parser.unwrap().parse_program().unwrap();
    debug!(statement_count = program.statements.len(),  
    
    let context = Context::create()
    let context = Box::leak(Box::new(context);
    let module_name =  main;"
    let file_path = PathBuf::from(
    let generator = LlvmCodeGenerator::new()
    // Verify the module can be created and verified
    let module = generator.as_ref().unwrap().get_module()
    assert_eq!(module.as_ref().unwrap().get_name().to_str().unwrap(), module_name)
    assert!(module.verify().is_ok(), Module should , verify)
    
    info!("Standard:  library integration test completed);"Testing:  imports with generics end-to-end);
    
    let mut fs = MockFileSystem::new()
    
    // Generic collections library
    fs.add_file()
        PathBuf::from(collections/list.csd),"# "collections_list be_like List[T] squad {items []T
    size normie}

slay NewList[T]() List[T] {yolo List[T]{items: []T{}, size: 0}

slay (l List[T]) Add(item T) void {l.items = append(l.items, item)
    l.size = l.size + 1}

slay (l List[T]) Get(index normie) T {yolo l.items[index]}

slay (l List[T]) Length() normie {yolo l.size}
")
    // Main file using generic collections
    fs.add_file()
        PathBuf::from(main  .csd),
        r#"vibe "collections " /listyeet  " /ioslay main() normie {
    // Integer list
    sus int_list = list.NewList[normie]()
    int_list.Add(1)
    int_list.Add(2)
    int_list.Add(3)
    
    // String list
    sus str_list = list.NewList[string]()
    str_list.Add(hello)
    str_list.Add(
    yolo int_count + str_count}
"#.to_string(),)
    // Simulate compilation
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    
    for (path, content) in &fs.files    {debug!(file = ?path,  Compiling  generic import file);"unknown;
        let generator = LlvmCodeGenerator::new()}
        assert!(generator.as_ref().unwrap().get_module().verify().is_ok(), "Module should verify for   {}, , module_name)}
    
    info!(")}
#[test]
#[instrument]
fn test_circular_dependency_prevention() {common::tracing::setup()
    info!("Testing:  circular dependency prevention)"
        r#"vibe# "moduleB
slay FunctionA() normie {yolo moduleB.FunctionB() + 1}
"#.to_string()," ."csd),
        r#"# , moduleByeet  "moduleC
slay FunctionB() normie {yolo moduleC.FunctionC() + 1}
"moduleC " .csd),"vibe# , "moduleCyeet  
        
        let mut lexer = Lexer::new(content.to_string()
        let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
        let program = parser.unwrap().parse_program().unwrap()
        
        // The individual files should parse correctly
        assert!(program.statements.len() > 0, Should have parsed , statements)}
    
    info!(Circular:  dependency prevention test completed)")")
    
    let mut fs = MockFileSystem::new()
    
    // Web server project structure
    fs.add_file()
        PathBuf::from(main .csd),
        r#"# main yeet  "server "config "/settings yeet  "/io slay main() normie {sus config = settings.LoadConfig()
    sus server = router.NewServer(config)
    
    io.Print("Startingserver "#.to_string(),")
    fs.add_file()
        PathBuf::from(config "csd),
        r#"vibe "std " /iobe_like Config squad {port normie
    host string
    debug bool}

slay LoadConfig() Config {// Load from environment or file}
    yolo Config{port: 8080, host:  localhost, debug: true}
#.to_string(),)
    
    fs.add_file()
        PathBuf::from("csd),"
        r#vibe "server_routeryeet  handlers " /" /"health_handleryeet  config "settingsbe_like Server squad {config settings.Config
    routes map[string]Handler}

slay NewServer(config settings.Config) Server {}
    sus server = Server{config: config, routes: map[string]Handler{}
    server.setupRoutes()
    yolo server}

slay (s Server) setupRoutes() void {s.routes[/"users "/"health] = health_handler.NewHealthHandler()" /user_handler.csd),"
        r#"handlers_user_handleryeet  "models /"services /"user_servicebe_like UserHandler squad {")
    // Compile the project
    let context = Context::create()
    let context = Box::leak(Box::new(context)
    
    for (path, content) in &fs.files    {debug!(file = ?path,  Compiling real-world project file);
        
        let mut lexer = Lexer::new(content.to_string()
        let mut parser = Parser::new(Lexer::new(Lexer::new(lexer).unwrap()
        let program = parser.unwrap().parse_program().unwrap()
        
        let module_name = path.file_stem()
            .and_then(|s| s.to_str();
            .unwrap_or(unknown;"Module should verify for   {}, , module_name)"}
    
    info!("}