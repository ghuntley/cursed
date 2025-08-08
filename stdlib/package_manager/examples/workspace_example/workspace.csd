// Example workspace configuration for multi-package CURSED projects
// Demonstrates workspace management and shared dependencies

workspace "my-cursed-workspace" 
{
    version = "1.0.0"
    description = "A multi-package CURSED workspace example"
    
    // Member packages in this workspace
    members = [
        "apps/web-server",           // Main web application
        "apps/cli-tool",             // Command-line interface
        "apps/background-worker",    // Background job processor
        "libs/core",                 // Core business logic library
        "libs/database",             // Database access layer
        "libs/auth",                 // Authentication library
        "libs/utils",                // Shared utilities
        "tools/migration-tool",      // Database migration tool
        "tools/admin-panel"          // Administration interface
    ]
    
    // Exclude these directories from workspace
    exclude = [
        "experiments/",              // Experimental code
        "archived/",                 // Archived projects
        "third-party/",              // External dependencies
        "target/",                   // Build artifacts
        "docs/examples/"             // Documentation examples
    ]
    
    // Shared dependencies used across multiple workspace members
    shared_dependencies = {
        // Core CURSED standard library
        "vibez" = "^1.0.0"          // I/O operations
        "stringz" = "^1.0.0"        // String utilities
        "arrayz" = "^1.0.0"         // Array operations
        "mathz" = "^1.0.0"          // Mathematical functions
        "filez" = "^1.0.0"          // File system operations
        
        // Common third-party libraries
        "logz" = "^2.1.0"           // Logging framework
        "configz" = "^1.5.0"        // Configuration management
        "errorz" = "^3.0.0"         // Error handling utilities
        "timez" = "^2.0.0"          // Time and date handling
        
        // Database and storage
        "sqlz" = "^4.0.0"           // SQL database interface
        "jsonz" = "^1.8.0"          // JSON parsing and generation
        "yamlz" = "^0.9.0"          // YAML configuration support
        
        // Network and HTTP
        "httpz" = "^3.2.0"          // HTTP client and server
        "tcpz" = "^1.4.0"           // TCP networking
        "websocketz" = "^2.1.0"     // WebSocket support
        
        // Security and cryptography
        "cryptz" = "^5.0.0"         // Cryptographic functions
        "hashz" = "^2.0.0"          // Hashing algorithms
        "jwtokenz" = "^1.3.0"       // JWT token handling
    }
    
    // Development dependencies shared across workspace
    shared_dev_dependencies = {
        "testz" = "^2.0.0"          // Testing framework
        "benchz" = "^1.5.0"         // Benchmarking tools
        "mockz" = "^3.0.0"          // Mocking and stubbing
        "coveragez" = "^1.2.0"      // Code coverage analysis
        "lintz" = "^2.1.0"          // Code linting and style
        "docz" = "^1.8.0"           // Documentation generation
        "profilerz" = "^0.7.0"      // Performance profiling
    }
    
    // Workspace-wide build configuration
    build = {
        // Global build settings
        cursed_version = ">=1.5.0"
        optimization = "release"
        debug_info = true
        
        // Shared compilation flags
        flags = {
            warnings_as_errors = true
            strict_mode = true
            memory_safety = true
            bounds_checking = true
        }
        
        // Platform-specific configurations
        platforms = {
            linux = {
                features = ["unix", "systemd"]
                link_flags = ["-pthread", "-ldl"]
                env = {
                    "PKG_CONFIG_PATH" = "/usr/lib/pkgconfig"
                }
            }
            windows = {
                features = ["windows", "service"]
                link_flags = ["-lws2_32", "-ladvapi32"]
                env = {
                    "MSYSTEM" = "MINGW64"
                }
            }
            macos = {
                features = ["unix", "launchd"]
                link_flags = ["-framework", "CoreFoundation"]
            }
            wasm = {
                features = ["wasm", "no-std"]
                target = "wasm32-unknown-unknown"
            }
        }
        
        // Build profiles
        profiles = {
            dev = {
                optimization = "debug"
                debug_info = true
                assertions = true
            }
            release = {
                optimization = "max"
                debug_info = false
                assertions = false
                link_time_optimization = true
            }
            test = {
                optimization = "debug"
                debug_info = true
                assertions = true
                coverage = true
            }
        }
    }
    
    // Workspace-wide scripts
    scripts = {
        // Build all packages
        "build-all" = "for pkg in apps/* libs/* tools/*; do cursed build $pkg; done"
        
        // Run all tests
        "test-all" = "cursed test --workspace"
        
        // Run benchmarks
        "bench-all" = "cursed bench --workspace"
        
        // Generate documentation
        "docs" = "cursed doc --workspace --output ./docs/api"
        
        // Lint all code
        "lint" = "cursed lint --workspace --fix"
        
        // Format all code
        "format" = "cursed fmt --workspace"
        
        // Clean all build artifacts
        "clean" = "cursed clean --workspace"
        
        // Install all dependencies
        "install" = "cursed install --workspace"
        
        // Update dependencies
        "update" = "cursed update --workspace"
        
        // Security audit
        "audit" = "cursed audit --workspace"
        
        // Release workflow
        "release" = "scripts/release.csd"
        
        // Development setup
        "setup" = "scripts/setup-dev.csd"
        
        // Database operations
        "db-migrate" = "tools/migration-tool/migrate.csd"
        "db-seed" = "tools/migration-tool/seed.csd"
        "db-reset" = "tools/migration-tool/reset.csd"
        
        // Deployment
        "deploy-staging" = "scripts/deploy.csd staging"
        "deploy-prod" = "scripts/deploy.csd production"
        
        // Monitoring and health checks
        "healthcheck" = "scripts/healthcheck.csd"
        "monitoring-setup" = "scripts/setup-monitoring.csd"
    }
    
    // Dependency resolution configuration
    resolver = {
        strategy = "highest_compatible"  // or "lowest_compatible", "latest"
        allow_prereleases = false
        prefer_stable = true
        
        // Version constraints for automatic updates
        auto_update = {
            patch = true      // Allow automatic patch updates
            minor = false     // Require manual minor updates
            major = false     // Require manual major updates
        }
        
        // Dependency source priority
        sources = [
            "https://packages.cursed.dev",     // Official registry
            "https://internal.company.com",    // Private registry
            "git+https://github.com/",         // Git repositories
            "file://"                          // Local packages
        ]
    }
    
    // Testing configuration
    testing = {
        // Test discovery patterns
        patterns = [
            "**/test_*.csd",
            "**/*_test.csd",
            "**/tests/*.csd"
        ]
        
        // Test execution settings
        parallel = true
        timeout = "5m"
        retry_flaky = 3
        
        // Coverage requirements
        coverage = {
            minimum = 80
            exclude = [
                "*/examples/*",
                "*/benches/*",
                "*/mock/*"
            ]
        }
        
        // Integration test configuration
        integration = {
            setup = "scripts/test-setup.csd"
            teardown = "scripts/test-teardown.csd"
            database_url = "postgres://test:test@localhost/test_db"
            redis_url = "redis://localhost:6379/15"
        }
    }
    
    // Documentation generation
    documentation = {
        // Output directory
        output = "docs/api"
        
        // Include patterns
        include = [
            "libs/*/src/**/*.csd",
            "apps/*/src/lib/**/*.csd"
        ]
        
        // Exclude patterns
        exclude = [
            "**/internal/**",
            "**/private/**",
            "**/test/**"
        ]
        
        // Documentation features
        features = {
            api_docs = true
            examples = true
            tutorials = true
            type_info = true
            source_links = true
        }
        
        // Themes and styling
        theme = "cursed-default"
        logo = "docs/assets/logo.png"
        favicon = "docs/assets/favicon.ico"
    }
    
    // Workspace metadata
    metadata = {
        repository = "https://github.com/company/my-cursed-workspace"
        homepage = "https://workspace.example.com"
        documentation = "https://docs.workspace.example.com"
        license = "MIT"
        authors = [
            "Development Team <dev@company.com>"
        ]
        keywords = ["workspace", "microservices", "web", "api"]
        categories = ["web-programming", "development-tools"]
    }
    
    // Publishing configuration
    publish = {
        // Which packages to publish
        publishable = [
            "libs/core",
            "libs/database", 
            "libs/auth",
            "libs/utils"
        ]
        
        // Exclude from publishing
        private = [
            "apps/*",           // Applications are not published
            "tools/*",          // Internal tools
            "experiments/*"     // Experimental code
        ]
        
        // Registry configuration
        registry = "https://packages.cursed.dev"
        
        // Release automation
        auto_release = {
            enabled = true
            on_tag = true
            version_bump = "patch"  // patch, minor, major
        }
    }
    
    // Development environment
    development = {
        // Required tools
        tools = [
            "cursed >= 1.5.0",
            "git >= 2.20.0", 
            "docker >= 20.0.0",
            "docker-compose >= 1.25.0"
        ]
        
        // Environment variables
        env = {
            "CURSED_ENV" = "development"
            "LOG_LEVEL" = "debug"
            "DATABASE_URL" = "postgres://dev:dev@localhost/workspace_dev"
            "REDIS_URL" = "redis://localhost:6379/0"
        }
        
        // Development services
        services = {
            postgres = {
                image = "postgres:14"
                ports = ["5432:5432"]
                env = {
                    "POSTGRES_DB" = "workspace_dev"
                    "POSTGRES_USER" = "dev"
                    "POSTGRES_PASSWORD" = "dev"
                }
            }
            redis = {
                image = "redis:7"
                ports = ["6379:6379"]
            }
            elasticsearch = {
                image = "elasticsearch:8.5.0"
                ports = ["9200:9200"]
                env = {
                    "discovery.type" = "single-node"
                    "xpack.security.enabled" = "false"
                }
            }
        }
    }
    
    // CI/CD configuration
    ci = {
        // Build matrix
        matrix = {
            os = ["ubuntu-latest", "windows-latest", "macos-latest"]
            cursed_version = ["1.5.0", "1.6.0", "latest"]
        }
        
        // Pipeline stages
        stages = [
            "lint",
            "test",
            "build", 
            "integration-test",
            "security-scan",
            "performance-test",
            "deploy"
        ]
        
        // Artifacts to preserve
        artifacts = [
            "target/release/*",
            "docs/api/*",
            "coverage/reports/*"
        ]
    }
}
