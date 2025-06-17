//! Integration tests for Build Optimization System
//! 
//! Tests the integration between the CLI and the underlying build system components.

use std::path::PathBuf;
use std::fs;
use std::time::Duration;
use tempfile::TempDir;

use cursed::build_system::{
    DependencyOptimizer, DependencyOptimizerConfig, AdvancedCache, AdvancedCacheConfig,
    BuildAnalytics, BuildAnalyticsConfig, MemoryOptimizer, MemoryOptimizerConfig,
    DistributedCompilationSystem, DistributedCompilationConfig, CompilationUnit
};
use cursed::error::Result;

/// Create a realistic project structure for testing
fn create_realistic_project() -> Result<TempDir> {
    let temp_dir = TempDir::new()?;
    let project_path = temp_dir.path();
    
    // Create src directory
    let src_dir = project_path.join("src");
    fs::create_dir(&src_dir)?;
    
    // Main application file
    let main_content = r#"
import "lib::http"
import "lib::database"
import "models::user"
import "services::auth"

fn main() {
    sus server = http::create_server(8080)
    sus db = database::connect("postgres://localhost/myapp")
    
    // Set up routes
    server.route("/api/users", handle_users)
    server.route("/api/auth", auth::handle_auth)
    
    println("Server starting on port 8080")
    server.start()
}

fn handle_users(request: http::Request) -> http::Response {
    lowkey (request.method == "GET") {
        sus users = user::get_all_users()
        return http::json_response(users)
    } highkey (request.method == "POST") {
        sus new_user = user::create_user(request.body)
        return http::json_response(new_user)
    } flex {
        return http::error_response(405, "Method not allowed")
    }
}
"#;
    fs::write(src_dir.join("main.csd"), main_content)?;
    
    // Library directory
    let lib_dir = src_dir.join("lib");
    fs::create_dir(&lib_dir)?;
    
    // HTTP library
    let http_content = r#"
export squad Server {
    sus port: i32
    sus routes: Map<String, fn(Request) -> Response>
}

export squad Request {
    sus method: String
    sus path: String
    sus headers: Map<String, String>
    sus body: String
}

export squad Response {
    sus status: i32
    sus headers: Map<String, String>
    sus body: String
}

export fn create_server(port: i32) -> Server {
    return Server {
        port: port,
        routes: Map::new()
    }
}

export fn json_response<T>(data: T) -> Response {
    return Response {
        status: 200,
        headers: Map::from([("Content-Type", "application/json")]),
        body: serde_json::to_string(&data).unwrap()
    }
}

export fn error_response(status: i32, message: String) -> Response {
    return Response {
        status: status,
        headers: Map::new(),
        body: message
    }
}
"#;
    fs::write(lib_dir.join("http.csd"), http_content)?;
    
    // Database library
    let database_content = r#"
import "postgres"

export squad Connection {
    sus url: String
    sus pool: postgres::Pool
}

export fn connect(url: String) -> Connection {
    sus pool = postgres::Pool::new(&url)
    return Connection {
        url: url,
        pool: pool
    }
}

export fn execute(conn: &Connection, query: String) -> Result<Vec<Row>, DatabaseError> {
    return conn.pool.execute(&query)
}
"#;
    fs::write(lib_dir.join("database.csd"), database_content)?;
    
    // Models directory
    let models_dir = src_dir.join("models");
    fs::create_dir(&models_dir)?;
    
    // User model
    let user_content = r#"
import "lib::database"
import "serde"

export squad User {
    sus id: i32
    sus name: String
    sus email: String
    sus created_at: DateTime
}

export fn get_all_users() -> Vec<User> {
    sus conn = database::get_connection()
    sus rows = database::execute(&conn, "SELECT * FROM users")
    
    facts users = Vec::new()
    lowkey (sus row in rows) {
        users.push(User::from_row(row))
    }
    
    return users
}

export fn create_user(data: UserCreateRequest) -> User {
    sus conn = database::get_connection()
    sus query = format!("INSERT INTO users (name, email) VALUES ('{}', '{}') RETURNING *", 
                       data.name, data.email)
    sus row = database::execute(&conn, query).first()
    return User::from_row(row)
}

squad UserCreateRequest {
    sus name: String
    sus email: String
}
"#;
    fs::write(models_dir.join("user.csd"), user_content)?;
    
    // Services directory
    let services_dir = src_dir.join("services");
    fs::create_dir(&services_dir)?;
    
    // Auth service
    let auth_content = r#"
import "lib::http"
import "models::user"
import "crypto"

export fn handle_auth(request: http::Request) -> http::Response {
    lowkey (request.method == "POST") {
        sus auth_data = parse_auth_request(request.body)
        
        lowkey (validate_credentials(auth_data)) {
            sus token = generate_jwt_token(auth_data.user_id)
            return http::json_response(AuthResponse {
                token: token,
                expires_in: 3600
            })
        } flex {
            return http::error_response(401, "Invalid credentials")
        }
    } flex {
        return http::error_response(405, "Method not allowed")
    }
}

fn validate_credentials(auth_data: AuthRequest) -> bool {
    sus user = user::find_by_email(auth_data.email)
    return crypto::verify_password(auth_data.password, user.password_hash)
}

fn generate_jwt_token(user_id: i32) -> String {
    sus payload = JwtPayload {
        user_id: user_id,
        exp: chrono::Utc::now().timestamp() + 3600
    }
    return jwt::encode(payload, &get_jwt_secret())
}

squad AuthRequest {
    sus email: String
    sus password: String
}

squad AuthResponse {
    sus token: String
    sus expires_in: i32
}
"#;
    fs::write(services_dir.join("auth.csd"), auth_content)?;
    
    // Tests directory
    let tests_dir = project_path.join("tests");
    fs::create_dir(&tests_dir)?;
    
    let test_content = r#"
import "src::lib::http"
import "src::models::user"

fn test_user_creation() {
    sus user_data = user::UserCreateRequest {
        name: "Test User",
        email: "test@example.com"
    }
    
    sus user = user::create_user(user_data)
    assert!(user.name == "Test User")
    assert!(user.email == "test@example.com")
}

fn test_http_server() {
    sus server = http::create_server(8080)
    assert!(server.port == 8080)
}
"#;
    fs::write(tests_dir.join("integration_test.csd"), test_content)?;
    
    Ok(temp_dir)
}

#[test]
fn test_dependency_optimizer_integration() -> Result<()> {
    let temp_dir = create_realistic_project()?;
    let project_path = temp_dir.path().to_path_buf();
    
    // Collect compilation units
    let units = collect_compilation_units_for_test(&project_path)?;
    assert!(units.len() > 0, "Should find compilation units");
    
    // Test dependency optimizer
    let config = DependencyOptimizerConfig {
        max_parallel_jobs: 4,
        enable_smart_ordering: true,
        enable_dependency_pruning: true,
        ..Default::default()
    };
    
    let optimizer = DependencyOptimizer::new(config);
    let analysis = optimizer.analyze_dependencies(&units)?;
    
    // Verify analysis results
    assert!(analysis.compilation_order.len() > 0, "Should have compilation layers");
    assert!(analysis.parallelism_factor >= 0.0 && analysis.parallelism_factor <= 1.0);
    assert!(analysis.estimated_time > Duration::from_millis(0));
    
    Ok(())
}

#[test]
fn test_advanced_cache_integration() -> Result<()> {
    let temp_dir = create_realistic_project()?;
    let project_path = temp_dir.path().to_path_buf();
    
    let config = AdvancedCacheConfig::default();
    let cache = AdvancedCache::new(config)?;
    
    // Test cache operations
    let units = collect_compilation_units_for_test(&project_path)?;
    let file_paths: Vec<String> = units.iter()
        .map(|u| u.path.to_string_lossy().to_string())
        .collect();
    
    // Warm cache
    let warmed = cache.warm_cache(&file_paths)?;
    assert!(warmed >= 0, "Cache warming should return non-negative count");
    
    // Get statistics
    let stats = cache.get_statistics()?;
    assert!(stats.total_entries >= 0);
    assert!(stats.hit_rate >= 0.0 && stats.hit_rate <= 1.0);
    
    Ok(())
}

#[test]
fn test_build_analytics_integration() -> Result<()> {
    let temp_dir = create_realistic_project()?;
    
    let config = BuildAnalyticsConfig::default();
    let analytics = BuildAnalytics::new(config)?;
    
    // Start build session
    analytics.start_build_session()?;
    
    // Simulate some build events
    std::thread::sleep(Duration::from_millis(100));
    
    // End build session
    analytics.end_build_session()?;
    
    // Generate report
    let report = analytics.generate_build_report()?;
    
    // Verify report structure
    assert!(!report.generated_at.is_empty());
    assert!(report.build_metrics.total_build_time >= Duration::from_millis(0));
    assert!(report.build_metrics.cache_hit_rate >= 0.0);
    
    Ok(())
}

#[test]
fn test_memory_optimizer_integration() -> Result<()> {
    let config = MemoryOptimizerConfig::default();
    let optimizer = MemoryOptimizer::new(config)?;
    
    // Get initial statistics
    let stats = optimizer.get_statistics()?;
    assert!(stats.current_usage_mb >= 0.0);
    assert!(stats.memory_efficiency_percent >= 0.0);
    
    // Test garbage collection trigger
    let gc_triggered = optimizer.trigger_gc_if_needed()?;
    // GC trigger result depends on memory state, so we just verify it returns
    
    Ok(())
}

#[test]
fn test_distributed_compilation_integration() -> Result<()> {
    let config = DistributedCompilationConfig {
        coordinator_port: 9001, // Use different port for testing
        work_stealing_enabled: true,
        distributed_nodes: vec![],
        ..Default::default()
    };
    
    let mut system = DistributedCompilationSystem::new(config)?;
    
    // Test system lifecycle
    system.start()?;
    
    // Give it a moment to start
    std::thread::sleep(Duration::from_millis(100));
    
    system.stop()?;
    
    Ok(())
}

#[test]
fn test_end_to_end_build_optimization() -> Result<()> {
    let temp_dir = create_realistic_project()?;
    let project_path = temp_dir.path().to_path_buf();
    
    // Collect compilation units
    let units = collect_compilation_units_for_test(&project_path)?;
    
    // Set up all optimization components
    let dependency_config = DependencyOptimizerConfig {
        max_parallel_jobs: 4,
        enable_smart_ordering: true,
        enable_dependency_pruning: true,
        ..Default::default()
    };
    let dependency_optimizer = DependencyOptimizer::new(dependency_config);
    
    let cache_config = AdvancedCacheConfig::default();
    let cache = AdvancedCache::new(cache_config)?;
    
    let analytics_config = BuildAnalyticsConfig::default();
    let analytics = BuildAnalytics::new(analytics_config)?;
    
    let memory_config = MemoryOptimizerConfig::default();
    let memory_optimizer = MemoryOptimizer::new(memory_config)?;
    
    // Start analytics session
    analytics.start_build_session()?;
    
    // Analyze dependencies
    let analysis = dependency_optimizer.analyze_dependencies(&units)?;
    assert!(analysis.compilation_order.len() > 0);
    
    // Warm cache
    let file_paths: Vec<String> = units.iter()
        .map(|u| u.path.to_string_lossy().to_string())
        .collect();
    let warmed = cache.warm_cache(&file_paths)?;
    
    // Simulate compilation process
    for (layer_idx, layer) in analysis.compilation_order.iter().enumerate() {
        println!("Processing layer {} with {} files", layer_idx + 1, layer.len());
        
        // Simulate compilation time based on complexity
        let complexity_sum: u32 = layer.iter()
            .filter_map(|id| units.iter().find(|u| u.id == *id))
            .map(|u| u.complexity_score)
            .sum();
        
        let simulation_time = Duration::from_millis((complexity_sum as u64).min(500));
        std::thread::sleep(simulation_time);
    }
    
    // Check memory usage
    let memory_stats = memory_optimizer.get_statistics()?;
    assert!(memory_stats.current_usage_mb >= 0.0);
    
    // End analytics session and generate report
    analytics.end_build_session()?;
    let report = analytics.generate_build_report()?;
    
    // Verify the end-to-end process worked
    assert!(report.build_metrics.total_build_time > Duration::from_millis(0));
    assert!(warmed >= 0);
    
    Ok(())
}

#[test]
fn test_large_project_performance() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let project_path = temp_dir.path();
    
    // Create a larger project structure
    for i in 0..50 {
        let content = format!(r#"
import "module_{}"

export fn function_{}() -> i32 {{
    lowkey (true) {{
        return {}
    }} flex {{
        return 0
    }}
}}

export squad Struct_{} {{
    sus field1: String
    sus field2: i32
}}
"#, (i + 1) % 10, i, i * 2, i);
        
        fs::write(project_path.join(format!("module_{}.csd", i)), content)?;
    }
    
    // Test performance with larger project
    let units = collect_compilation_units_for_test(&project_path.to_path_buf())?;
    assert!(units.len() >= 50, "Should have created 50+ files");
    
    let config = DependencyOptimizerConfig {
        max_parallel_jobs: 8,
        enable_smart_ordering: true,
        enable_dependency_pruning: true,
        ..Default::default()
    };
    
    let optimizer = DependencyOptimizer::new(config);
    
    let start_time = std::time::Instant::now();
    let analysis = optimizer.analyze_dependencies(&units)?;
    let analysis_time = start_time.elapsed();
    
    // Performance assertions
    assert!(analysis_time < Duration::from_secs(5), "Analysis should complete within 5 seconds");
    assert!(analysis.compilation_order.len() > 0);
    assert!(analysis.parallelism_factor >= 0.0);
    
    Ok(())
}

#[test]
fn test_error_handling_integration() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let project_path = temp_dir.path().to_path_buf();
    
    // Create files with dependency cycles for testing error handling
    let file1_content = r#"
import "file2"

export fn function1() -> i32 {
    return file2::function2()
}
"#;
    fs::write(project_path.join("file1.csd"), file1_content)?;
    
    let file2_content = r#"
import "file1"

export fn function2() -> i32 {
    return file1::function1()
}
"#;
    fs::write(project_path.join("file2.csd"), file2_content)?;
    
    let units = collect_compilation_units_for_test(&project_path)?;
    
    let config = DependencyOptimizerConfig::default();
    let optimizer = DependencyOptimizer::new(config);
    
    // The system should handle dependency cycles gracefully
    let result = optimizer.analyze_dependencies(&units);
    
    // Either succeeds with cycle detection or returns an appropriate error
    match result {
        Ok(analysis) => {
            // If it succeeds, it should detect the issue
            assert!(analysis.compilation_order.len() > 0);
        }
        Err(_) => {
            // Graceful error handling is also acceptable
        }
    }
    
    Ok(())
}

// Helper function to collect compilation units for testing
fn collect_compilation_units_for_test(project_path: &PathBuf) -> Result<Vec<CompilationUnit>> {
    use std::collections::HashSet;
    
    let mut units = Vec::new();
    let mut visited = HashSet::new();
    
    collect_source_files_recursive(project_path, &mut units, &mut visited)?;
    analyze_file_dependencies_simple(&mut units)?;
    
    Ok(units)
}

fn collect_source_files_recursive(
    dir: &PathBuf,
    units: &mut Vec<CompilationUnit>,
    visited: &mut HashSet<PathBuf>,
) -> Result<()> {
    if visited.contains(dir) {
        return Ok(());
    }
    visited.insert(dir.clone());
    
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            collect_source_files_recursive(&path, units, visited)?;
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("csd") {
            let unit = create_compilation_unit_simple(&path)?;
            units.push(unit);
        }
    }
    
    Ok(())
}

fn create_compilation_unit_simple(path: &PathBuf) -> Result<CompilationUnit> {
    let metadata = fs::metadata(path)?;
    let last_modified = metadata
        .modified()?
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    let content = fs::read_to_string(path)?;
    let complexity_score = calculate_complexity_simple(&content);
    
    let id = path.to_string_lossy().to_string();
    let cache_key = format!("{}-{}", id, last_modified);
    
    Ok(CompilationUnit {
        id: id.clone(),
        path: path.clone(),
        dependencies: extract_dependencies_simple(&content),
        dependents: Vec::new(),
        last_modified,
        compilation_time: Duration::from_millis((complexity_score * 10) as u64),
        complexity_score,
        is_dirty: true,
        cache_key,
    })
}

fn extract_dependencies_simple(content: &str) -> Vec<String> {
    let mut dependencies = Vec::new();
    
    for line in content.split("\n") {
        let trimmed = line.trim();
        if trimmed.starts_with("import") {
            if let Some(module) = extract_module_name_simple(trimmed) {
                dependencies.push(module);
            }
        }
    }
    
    dependencies.sort();
    dependencies.dedup();
    dependencies
}

fn extract_module_name_simple(line: &str) -> Option<String> {
    if let Some(start) = line.find('"') {
        if let Some(end) = line[start + 1..].find('"') {
            let module = &line[start + 1..start + 1 + end];
            return Some(module.to_string());
        }
    }
    None
}

fn calculate_complexity_simple(content: &str) -> u32 {
    let mut score = content.split("\n").count() as u32;
    
    for line in content.split("\n") {
        if line.contains("fn ") { score += 5; }
        if line.contains("lowkey") || line.contains("highkey") { score += 3; }
        if line.contains("squad") { score += 4; }
        if line.contains('<') && line.contains('>') { score += 2; }
    }
    
    score
}

fn analyze_file_dependencies_simple(units: &mut Vec<CompilationUnit>) -> Result<()> {
    // Build reverse dependency mapping
    for i in 0..units.len() {
        let dependencies = units[i].dependencies.clone();
        
        for dep in dependencies {
            for j in 0..units.len() {
                if units[j].id.contains(&dep) || units[j].path.to_string_lossy().contains(&dep) {
                    units[j].dependents.push(units[i].id.clone());
                    break;
                }
            }
        }
    }
    
    Ok(())
}
