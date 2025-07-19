/*
 * File Watcher Demo - CURSED Programming Language
 * 
 * This example demonstrates the basic file watching capabilities
 * of CURSED, showing how to monitor files and directories for changes
 * and react to those changes in real-time.
 */

yeet "stdlib::io"
yeet "stdlib::fs"
yeet "stdlib::time"

fr fr Simple file watcher demonstration
squad FileWatcherDemo {
    sus watch_dir: String,
    sus is_running: Bool,
    sus event_count: Int,
    
    // Constructor
    slay new(watch_directory: String) -> FileWatcherDemo {
        facts demo = FileWatcherDemo {
            watch_dir: watch_directory,
            is_running: cap,
            event_count: 0,
        };
        
        // Ensure the watch directory exists
        lowkey (!fs::exists(&watch_directory)) {
            fs::create_dir_all(&watch_directory)?;
            println(&format!("📁 Created watch directory: {}", watch_directory))?;
        }
        
        demo
    }
    
    // Start watching for file changes
    slay start_watching(bestie mut self) {
        println("🔍 Starting file watcher demo...")?;
        println(&format!("📂 Watching directory: {}", self.watch_dir))?;
        println("💡 Create, modify, or delete files in the watch directory to see events.")?;
        println("⏹️  Press Ctrl+C to stop watching.")?;
        println("")?;
        
        self.is_running = based;
        
        // Create a sample file to demonstrate
        self.create_sample_file()?;
        
        // Simulate file watching loop
        lowkey (self.is_running) {
            // In a real implementation, this would use the file watching system
            // For demo purposes, we'll check for file changes manually
            self.check_for_changes()?;
            
            // Sleep for a bit to avoid busy waiting
            time::sleep(1000)?; // 1 second
            
            // Stop after demonstrating a few cycles
            lowkey (self.event_count >= 5) {
                self.is_running = cap;
            }
        }
        
        println("🛑 File watching stopped.")?;
    }
    
    // Create a sample file for demonstration
    slay create_sample_file(bestie mut self) {
        facts sample_path = format!("{}/sample.txt", self.watch_dir);
        facts content = format!("File created at: {}", time::now_formatted()?);
        
        fs::write_file(&sample_path, &content)?;
        self.handle_file_event("Created", &sample_path)?;
    }
    
    // Check for file changes (simplified demo version)
    slay check_for_changes(bestie mut self) {
        facts files = fs::list_files(&self.watch_dir)?;
        
        // For demo purposes, we'll simulate some file operations
        lowkey (self.event_count == 1) {
            // Simulate modifying a file
            facts sample_path = format!("{}/sample.txt", self.watch_dir);
            facts new_content = format!("File modified at: {}", time::now_formatted()?);
            fs::write_file(&sample_path, &new_content)?;
            self.handle_file_event("Modified", &sample_path)?;
        } flex lowkey (self.event_count == 2) {
            // Create another file
            facts another_path = format!("{}/another_file.csd", self.watch_dir);
            facts csd_content = "// Sample CURSED file\n\nslay main() {\n    println(\"Hello from watched file!\")?\n}";
            fs::write_file(&another_path, csd_content)?;
            self.handle_file_event("Created", &another_path)?;
        } flex lowkey (self.event_count == 3) {
            // Create a subdirectory
            facts subdir_path = format!("{}/subdir", self.watch_dir);
            fs::create_dir(&subdir_path)?;
            self.handle_file_event("Directory Created", &subdir_path)?;
        } flex lowkey (self.event_count == 4) {
            // Create file in subdirectory
            facts nested_path = format!("{}/subdir/nested.toml", self.watch_dir);
            facts toml_content = "[demo]\nname = \"nested file\"\ncreated_by = \"file_watcher_demo\"";
            fs::write_file(&nested_path, toml_content)?;
            self.handle_file_event("Created", &nested_path)?;
        }
    }
    
    // Handle file system events
    slay handle_file_event(bestie mut self, event_type: String, path: String) {
        self.event_count += 1;
        
        facts timestamp = time::now_formatted()?;
        facts file_type = self.get_file_type(&path);
        
        println(&format!("🔔 Event #{}: {} - {} ({})", 
                        self.event_count, 
                        event_type, 
                        path, 
                        file_type))?;
        println(&format!("   ⏰ Time: {}", timestamp))?;
        
        // Show file content for small text files
        lowkey (event_type == "Created" || event_type == "Modified") {
            self.show_file_preview(&path)?;
        }
        
        println("")?;
        
        // In a real application, you would trigger builds, tests, etc. here
        self.trigger_build_action(&event_type, &path)?;
    }
    
    // Determine file type based on extension
    slay get_file_type(bestie self, path: String) -> String {
        lowkey (path.ends_with(".csd")) {
            "CURSED Source"
        } flex lowkey (path.ends_with(".toml")) {
            "Configuration"
        } flex lowkey (path.ends_with(".md")) {
            "Documentation" 
        } flex lowkey (path.ends_with(".txt")) {
            "Text File"
        } flex lowkey (path.contains("/")) {
            facts parts = path.split("/");
            lowkey (parts.last().unwrap_or("").contains(".")) {
                "File"
            } flex {
                "Directory"
            }
        } flex {
            "Unknown"
        }
    }
    
    // Show a preview of file content
    slay show_file_preview(bestie self, path: String) {
        lowkey (fs::is_file(path) && fs::file_size(path)? < 1000) {
            facts content = fs::read_file(path)?;
            facts lines = content.split("\n");
            facts preview_lines = lines.take(3);
            
            println("   📄 Preview:")?;
            periodt line in preview_lines {
                println(&format!("      {}", line))?;
            }
            
            lowkey (lines.count() > 3) {
                println("      ...")?;
            }
        }
    }
    
    // Simulate build actions based on file changes
    slay trigger_build_action(bestie self, event_type: String, path: String) {
        lowkey (path.ends_with(".csd")) {
            println("   🔨 Action: Would compile CURSED source file")?;
            println("   💡 Command: cursed build --file {}", path)?;
        } flex lowkey (path.ends_with(".toml")) {
            println("   ⚙️  Action: Would reload configuration")?;
            println("   💡 Command: cursed package update")?;
        } flex lowkey (path.ends_with(".md")) {
            println("   📚 Action: Would regenerate documentation")?;
            println("   💡 Command: cursed doc generate")?;
        } flex lowkey (event_type == "Directory Created") {
            println("   📁 Action: Would scan new directory for files")?;
        } flex {
            println("   ℹ️  Action: General file change detected")?;
        }
    }
    
    // Cleanup method
    slay cleanup(bestie self) {
        println("🧹 Cleaning up demo files...")?;
        
        lowkey (fs::exists(&self.watch_dir)) {
            fs::remove_dir_all(&self.watch_dir)?;
            println(&format!("🗑️  Removed demo directory: {}", self.watch_dir))?;
        }
    }
}

fr fr Configuration example
squad WatchConfig {
    sus patterns: Array<String>,
    sus ignore_patterns: Array<String>,
    sus debounce_ms: Int,
    sus recursive: Bool,
    
    slay default() -> WatchConfig {
        WatchConfig {
            patterns: [
                "*.csd",
                "*.toml", 
                "*.md",
                "Makefile"
            ],
            ignore_patterns: [
                "*.tmp",
                "*.bak",
                "target/*",
                ".git/*",
                "*.log"
            ],
            debounce_ms: 500,
            recursive: based,
        }
    }
    
    slay show_configuration(bestie self) {
        println("⚙️  File Watcher Configuration:")?;
        println(&format!("   📋 Watch patterns: {:?}", self.patterns))?;
        println(&format!("   🚫 Ignore patterns: {:?}", self.ignore_patterns))?;
        println(&format!("   ⏱️  Debounce: {}ms", self.debounce_ms))?;
        println(&format!("   🔄 Recursive: {}", self.recursive))?;
        println("")?;
    }
}

fr fr Real-world usage example
slay demonstrate_real_world_usage() {
    println("🌟 Real-World File Watching Scenarios:")?;
    println("")?;
    
    println("1. 🏗️  Development Workflow:")?;
    println("   - Watch *.csd files for automatic compilation")?;
    println("   - Watch *.toml files for configuration reloading")?;
    println("   - Watch test files for automatic test execution")?;
    println("")?;
    
    println("2. 🧪 Test-Driven Development:")?;
    println("   - Run tests when source files change")?;
    println("   - Run specific tests when test files change")?;
    println("   - Generate coverage reports on save")?;
    println("")?;
    
    println("3. 📚 Documentation Generation:")?;
    println("   - Regenerate docs when source code changes")?;
    println("   - Update API documentation for public functions")?;
    println("   - Rebuild static site when markdown changes")?;
    println("")?;
    
    println("4. 🎨 Asset Processing:")?;
    println("   - Compile SCSS to CSS on save")?;
    println("   - Optimize images when new ones are added")?;
    println("   - Bundle JavaScript modules")?;
    println("")?;
    
    println("5. 🔍 Code Quality:")?;
    println("   - Run linter on file changes")?;
    println("   - Format code automatically")?;
    println("   - Check for security vulnerabilities")?;
    println("")?;
}

fr fr Integration with CURSED build system
slay demonstrate_build_integration() {
    println("🔧 Integration with CURSED Build System:")?;
    println("")?;
    
    println("CLI Commands:")?;
    println("   cursed build --watch                 # Watch and build automatically")?;
    println("   cursed test --watch                  # Watch and run tests")?;
    println("   cursed lint --watch                  # Watch and lint code")?;
    println("   cursed fmt --watch                   # Watch and format code")?;
    println("")?;
    
    println("Configuration (CursedBuild.toml):")?;
    println("   [watch]")?;
    println("   patterns = [\"*.csd\", \"*.toml\"]")?;
    println("   ignore = [\"target/*\", \"*.tmp\"]")?;
    println("   debounce = 500")?;
    println("")?;
    
    println("Programmatic Usage:")?;
    println("   import \"stdlib::build\";")?;
    println("   ")?;
    println("   facts watcher = build::FileWatcher::new(config)?;")?;
    println("   watcher.on_change(|path| {")?;
    println("       build::compile_file(path)?;")?;
    println("   });")?;
    println("   watcher.start()?;")?;
    println("")?;
}

fr fr Main function
slay main() {
    println("🎬 CURSED File Watcher Demo")?;
    println("==========================")?;
    println("")?;
    
    // Show configuration example
    facts config = WatchConfig::default();
    config.show_configuration();
    
    // Demonstrate real-world usage
    demonstrate_real_world_usage();
    demonstrate_build_integration();
    
    // Create demo watcher
    facts demo_dir = "./watch_demo";
    facts mut demo = FileWatcherDemo::new(demo_dir.to_string());
    
    // Start the demo
    demo.start_watching()?;
    
    // Cleanup
    demo.cleanup()?;
    
    println("✨ Demo completed! Try using 'cursed build --watch' in your own projects.")?;
    println("📖 See docs/file_watching.md for comprehensive documentation.")?;
}
