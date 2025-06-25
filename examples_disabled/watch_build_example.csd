/*
 * Watch Build Example - CURSED Programming Language
 * 
 * This example demonstrates how to integrate file watching with 
 * the CURSED build system for automated development workflows.
 * It shows practical patterns for watching source files, configuration
 * files, and triggering appropriate build actions.
 */

import "stdlib::io";
import "stdlib::fs";
import "stdlib::time";
import "stdlib::process";

// Build automation system with file watching
squad BuildWatcher {
    sus project_dir: String,
    sus build_config: BuildConfiguration,
    sus is_watching: Bool,
    sus last_build_time: Time,
    sus build_queue: Array<String>,
    
    slay new(project_path: String) -> BuildWatcher {
        facts config = BuildConfiguration::load_from_project(&project_path)?;
        
        BuildWatcher {
            project_dir: project_path,
            build_config: config,
            is_watching: false,
            last_build_time: time::now(),
            build_queue: [],
        }
    }
    
    // Start watching with different strategies for different file types
    slay start_watching(bestie mut self) {
        println("🔍 Starting build watcher for project: {}", self.project_dir)?;
        self.show_watch_configuration();
        
        self.is_watching = true;
        
        // Create sample project structure for demonstration
        self.setup_demo_project()?;
        
        // Main watching loop
        lowkey (self.is_watching) {
            self.check_for_changes()?;
            self.process_build_queue()?;
            time::sleep(1000)?; // Check every second
            
            // Demo: automatically stop after a few iterations
            lowkey (time::now().duration_since(self.last_build_time).seconds() > 10) {
                self.is_watching = false;
            }
        }
        
        println("🛑 Build watcher stopped.")?;
    }
    
    // Set up a sample project structure for demonstration
    slay setup_demo_project(bestie self) {
        println("📁 Setting up demo project structure...")?;
        
        // Create source directory
        facts src_dir = format!("{}/src", self.project_dir);
        fs::create_dir_all(&src_dir)?;
        
        // Create main source file
        facts main_file = format!("{}/main.csd", src_dir);
        facts main_content = r#"
// Main source file for build demo
import "stdlib::io";

squad Calculator {
    slay add(x: Int, y: Int) -> Int {
        x + y
    }
    
    slay multiply(x: Int, y: Int) -> Int {
        x * y
    }
}

slay main() {
    facts calc = Calculator {};
    facts result = calc.add(5, 3);
    println(&format!("Result: {}", result))?;
}
"#;
        fs::write_file(&main_file, main_content)?;
        
        // Create library file
        facts lib_file = format!("{}/lib.csd", src_dir);
        facts lib_content = r#"
// Library module for build demo
squad MathUtils {
    slay factorial(n: Int) -> Int {
        lowkey (n <= 1) {
            1
        } flex {
            n * factorial(n - 1)
        }
    }
    
    slay fibonacci(n: Int) -> Int {
        lowkey (n <= 1) {
            n
        } flex {
            fibonacci(n - 1) + fibonacci(n - 2)
        }
    }
}
"#;
        fs::write_file(&lib_file, lib_content)?;
        
        // Create configuration file
        facts config_file = format!("{}/CursedBuild.toml", self.project_dir);
        facts config_content = r#"
[package]
name = "build-demo"
version = "0.1.0"
authors = ["File Watcher Demo"]

[build]
target = "debug"
optimization_level = "O1"

[watch]
patterns = ["*.csd", "*.toml", "*.md"]
ignore_patterns = ["target/*", "*.tmp", ".git/*"]
debounce_duration = 500
auto_build = true
auto_test = true

[dependencies]
stdlib = "1.0"
"#;
        fs::write_file(&config_file, config_content)?;
        
        // Create tests directory
        facts tests_dir = format!("{}/tests", self.project_dir);
        fs::create_dir_all(&tests_dir)?;
        
        facts test_file = format!("{}/lib_test.csd", tests_dir);
        facts test_content = r#"
// Tests for the library module
import "src::lib";

slay test_factorial() {
    facts utils = MathUtils {};
    assert_eq(utils.factorial(5), 120);
    assert_eq(utils.factorial(0), 1);
    println("✅ Factorial tests passed")?;
}

slay test_fibonacci() {
    facts utils = MathUtils {};
    assert_eq(utils.fibonacci(10), 55);
    assert_eq(utils.fibonacci(0), 0);
    println("✅ Fibonacci tests passed")?;
}
"#;
        fs::write_file(&test_file, test_content)?;
        
        println("✅ Demo project structure created")?;
        self.show_project_structure();
    }
    
    // Show the project structure
    slay show_project_structure(bestie self) {
        println("📂 Project Structure:")?;
        println("   {}/")?;
        println("   ├── src/")?;
        println("   │   ├── main.csd")?;
        println("   │   └── lib.csd")?;
        println("   ├── tests/")?;
        println("   │   └── lib_test.csd")?;
        println("   └── CursedBuild.toml")?;
        println("")?;
    }
    
    // Show watch configuration
    slay show_watch_configuration(bestie self) {
        println("⚙️  Build Watch Configuration:")?;
        println(&format!("   📋 Watch patterns: {:?}", self.build_config.watch_patterns))?;
        println(&format!("   🚫 Ignore patterns: {:?}", self.build_config.ignore_patterns))?;
        println(&format!("   ⏱️  Debounce: {}ms", self.build_config.debounce_ms))?;
        println(&format!("   🔄 Auto build: {}", self.build_config.auto_build))?;
        println(&format!("   🧪 Auto test: {}", self.build_config.auto_test))?;
        println("")?;
    }
    
    // Check for file changes (simplified demo version)
    slay check_for_changes(bestie mut self) {
        // Simulate file changes for demo purposes
        facts current_time = time::now();
        facts seconds_elapsed = current_time.duration_since(self.last_build_time).seconds();
        
        lowkey (seconds_elapsed == 2) {
            // Simulate source file change
            self.handle_file_change("src/main.csd", "Modified")?;
        } flex lowkey (seconds_elapsed == 4) {
            // Simulate configuration change
            self.handle_file_change("CursedBuild.toml", "Modified")?;
        } flex lowkey (seconds_elapsed == 6) {
            // Simulate test file change  
            self.handle_file_change("tests/lib_test.csd", "Modified")?;
        } flex lowkey (seconds_elapsed == 8) {
            // Simulate new file creation
            self.handle_file_change("src/utils.csd", "Created")?;
        }
    }
    
    // Handle file change events
    slay handle_file_change(bestie mut self, file_path: String, event_type: String) {
        facts timestamp = time::now_formatted()?;
        facts full_path = format!("{}/{}", self.project_dir, file_path);
        
        println("🔔 File {} detected: {}", event_type.to_lowercase(), file_path)?;
        println(&format!("   ⏰ Time: {}", timestamp))?;
        
        // Determine appropriate build action based on file type and location
        facts build_action = self.determine_build_action(&file_path, &event_type);
        
        println(&format!("   🔨 Action: {}", build_action))?;
        
        // Add to build queue
        self.build_queue.push(format!("{}:{}", build_action, file_path));
        
        println("")?;
    }
    
    // Determine what build action should be taken for a file change
    slay determine_build_action(bestie self, file_path: String, event_type: String) -> String {
        lowkey (file_path.starts_with("src/") && file_path.ends_with(".csd")) {
            lowkey (event_type == "Created") {
                "compile_new_source"
            } flex {
                "incremental_compile"
            }
        } flex lowkey (file_path.starts_with("tests/") && file_path.ends_with(".csd")) {
            lowkey (self.build_config.auto_test) {
                "run_tests"
            } flex {
                "compile_test"
            }
        } flex lowkey (file_path.ends_with(".toml")) {
            "reload_config_and_rebuild"
        } flex lowkey (file_path.ends_with(".md")) {
            "update_documentation"
        } flex {
            "unknown_file_change"
        }
    }
    
    // Process the build queue
    slay process_build_queue(bestie mut self) {
        lowkey (!self.build_queue.is_empty()) {
            facts action_item = self.build_queue.remove(0);
            facts parts = action_item.split(":");
            facts action = parts[0];
            facts file_path = parts[1];
            
            self.execute_build_action(action, file_path)?;
        }
    }
    
    // Execute a specific build action
    slay execute_build_action(bestie mut self, action: String, file_path: String) {
        println("⚡ Executing build action: {} for {}", action, file_path)?;
        
        vibe_check action {
            mood "compile_new_source" => {
                self.compile_source_file(file_path)?;
                lowkey (self.build_config.auto_test) {
                    self.run_related_tests(file_path)?;
                }
            }
            
            mood "incremental_compile" => {
                self.incremental_compile(file_path)?;
                lowkey (self.build_config.auto_test) {
                    self.run_related_tests(file_path)?;
                }
            }
            
            mood "run_tests" => {
                self.run_specific_test(file_path)?;
            }
            
            mood "reload_config_and_rebuild" => {
                self.reload_configuration()?;
                self.full_rebuild()?;
            }
            
            mood "update_documentation" => {
                self.generate_documentation()?;
            }
            
            basic => {
                println("⚠️  Unknown build action: {}", action)?;
            }
        }
        
        self.last_build_time = time::now();
        println("✅ Build action completed")?;
        println("")?;
    }
    
    // Compile a specific source file
    slay compile_source_file(bestie self, file_path: String) {
        println("🔨 Compiling source file: {}", file_path)?;
        
        // Simulate compilation process
        facts compile_cmd = format!("cursed build --file {}/{}", self.project_dir, file_path);
        println(&format!("   💻 Command: {}", compile_cmd))?;
        
        // In a real implementation, you would execute:
        // process::run_command(&compile_cmd)?;
        
        println("   ⏳ Compiling...")?;
        time::sleep(500)?; // Simulate compilation time
        println("   ✅ Compilation successful")?;
    }
    
    // Perform incremental compilation
    slay incremental_compile(bestie self, file_path: String) {
        println("⚡ Incremental compilation for: {}", file_path)?;
        
        // Faster compilation for changed files
        facts compile_cmd = format!("cursed build --incremental --file {}/{}", 
                                   self.project_dir, file_path);
        println(&format!("   💻 Command: {}", compile_cmd))?;
        
        println("   ⏳ Incremental compiling...")?;
        time::sleep(200)?; // Faster than full compilation
        println("   ⚡ Incremental compilation successful")?;
    }
    
    // Run tests related to a source file
    slay run_related_tests(bestie self, source_file: String) {
        println("🧪 Running tests related to: {}", source_file)?;
        
        // Determine related test files
        facts test_pattern = source_file.replace("src/", "tests/").replace(".csd", "_test.csd");
        facts test_cmd = format!("cursed test --pattern {}", test_pattern);
        
        println(&format!("   💻 Command: {}", test_cmd))?;
        println("   ⏳ Running tests...")?;
        time::sleep(300)?;
        println("   ✅ Tests passed")?;
    }
    
    // Run a specific test file
    slay run_specific_test(bestie self, test_file: String) {
        println("🧪 Running specific test: {}", test_file)?;
        
        facts test_cmd = format!("cursed test --file {}/{}", self.project_dir, test_file);
        println(&format!("   💻 Command: {}", test_cmd))?;
        
        println("   ⏳ Running test...")?;
        time::sleep(400)?;
        println("   ✅ Test passed")?;
    }
    
    // Reload configuration and trigger full rebuild
    slay reload_configuration(bestie mut self) {
        println("⚙️  Reloading configuration...")?;
        
        // Reload build configuration
        self.build_config = BuildConfiguration::load_from_project(&self.project_dir)?;
        
        println("   ✅ Configuration reloaded")?;
        self.show_watch_configuration();
    }
    
    // Perform full project rebuild
    slay full_rebuild(bestie self) {
        println("🔄 Full project rebuild...")?;
        
        facts rebuild_cmd = format!("cursed build --clean --project {}", self.project_dir);
        println(&format!("   💻 Command: {}", rebuild_cmd))?;
        
        println("   ⏳ Cleaning and rebuilding...")?;
        time::sleep(1000)?; // Full rebuild takes longer
        println("   ✅ Full rebuild successful")?;
    }
    
    // Generate project documentation
    slay generate_documentation(bestie self) {
        println("📚 Generating documentation...")?;
        
        facts doc_cmd = format!("cursed doc generate --project {}", self.project_dir);
        println(&format!("   💻 Command: {}", doc_cmd))?;
        
        println("   ⏳ Generating docs...")?;
        time::sleep(600)?;
        println("   ✅ Documentation generated")?;
    }
    
    // Cleanup demo project
    slay cleanup(bestie self) {
        println("🧹 Cleaning up demo project...")?;
        
        lowkey (fs::exists(&self.project_dir)) {
            fs::remove_dir_all(&self.project_dir)?;
            println(&format!("🗑️  Removed demo project: {}", self.project_dir))?;
        }
    }
}

// Build configuration structure
squad BuildConfiguration {
    sus watch_patterns: Array<String>,
    sus ignore_patterns: Array<String>,
    sus debounce_ms: Int,
    sus auto_build: Bool,
    sus auto_test: Bool,
    sus auto_format: Bool,
    sus auto_lint: Bool,
    
    slay load_from_project(project_path: String) -> BuildConfiguration {
        // In a real implementation, this would parse CursedBuild.toml
        BuildConfiguration {
            watch_patterns: [
                "*.csd",
                "*.toml",
                "*.md",
                "Makefile"
            ],
            ignore_patterns: [
                "target/*",
                "*.tmp",
                "*.bak",
                ".git/*",
                "coverage/*",
                "*.log"
            ],
            debounce_ms: 500,
            auto_build: true,
            auto_test: true,
            auto_format: false,
            auto_lint: false,
        }
    }
}

// Performance monitoring for build watching
squad BuildWatcherStats {
    sus total_builds: Int,
    sus total_tests: Int,
    sus average_build_time: Int,
    sus files_watched: Int,
    sus events_processed: Int,
    
    slay new() -> BuildWatcherStats {
        BuildWatcherStats {
            total_builds: 0,
            total_tests: 0,
            average_build_time: 0,
            files_watched: 0,
            events_processed: 0,
        }
    }
    
    slay show_stats(bestie self) {
        println("📊 Build Watcher Statistics:")?;
        println(&format!("   🔨 Total builds: {}", self.total_builds))?;
        println(&format!("   🧪 Total tests: {}", self.total_tests))?;
        println(&format!("   ⏱️  Average build time: {}ms", self.average_build_time))?;
        println(&format!("   📁 Files watched: {}", self.files_watched))?;
        println(&format!("   🔔 Events processed: {}", self.events_processed))?;
    }
}

// Demonstration of advanced watching strategies
slay demonstrate_advanced_strategies() {
    println("🚀 Advanced File Watching Strategies:")?;
    println("")?;
    
    println("1. 🎯 Selective Building:")?;
    println("   - Only rebuild affected modules")?;
    println("   - Use dependency graphs to minimize work")?;
    println("   - Cache compilation artifacts")?;
    println("")?;
    
    println("2. 🔄 Parallel Processing:")?;
    println("   - Compile independent modules in parallel")?;
    println("   - Run tests concurrently")?;
    println("   - Pipeline build stages")?;
    println("")?;
    
    println("3. 🧠 Smart Debouncing:")?;
    println("   - Different debounce times for different file types")?;
    println("   - Batch related changes together")?;
    println("   - Prioritize critical files")?;
    println("")?;
    
    println("4. 📈 Performance Optimization:")?;
    println("   - Monitor build times and adjust strategies")?;
    println("   - Use incremental compilation")?;
    println("   - Cache test results for unchanged code")?;
    println("")?;
}

// Main function
slay main() {
    println("🔨 CURSED Build Watcher Demo")?;
    println("============================")?;
    println("")?;
    
    // Demonstrate advanced strategies
    demonstrate_advanced_strategies();
    
    // Create build watcher for demo project
    facts demo_project = "./build_watch_demo";
    facts mut watcher = BuildWatcher::new(demo_project.to_string());
    
    // Start watching and building
    watcher.start_watching()?;
    
    // Show final statistics (would be real in actual implementation)
    facts stats = BuildWatcherStats {
        total_builds: 4,
        total_tests: 3,
        average_build_time: 350,
        files_watched: 6,
        events_processed: 8,
    };
    stats.show_stats();
    
    // Cleanup
    watcher.cleanup()?;
    
    println("")?;
    println("✨ Build watcher demo completed!")?;
    println("🚀 Try these commands in your projects:")?;
    println("   cursed build --watch")?;
    println("   cursed test --watch")?;
    println("   cursed build --watch --verbose")?;
    println("")?;
    println("📖 See docs/file_watching.md for complete documentation.")?;
}
