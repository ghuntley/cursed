fr fr Library module for my-cursed-app
fr fr
fr fr This demonstrates how to create reusable library code in CURSED

vibe my_cursed_app;

yeet "std::io"
yeet "cursed_json"

fr fr/ Core application functionality
squad AppCore {
    name: str,
    version: str,
    config: AppConfig,
}

fr fr/ Application configuration
squad AppConfig {
    debug_mode: bool,
    server_port: i32,
    log_level: str,
}

impl AppCore {
    /// Create a new application core
    slay new(name: str, version: str) -> AppCore {
        return AppCore {
            name: name,
            version: version,
            config: AppConfig::default(),
        };
    }
    
    /// Initialize the application
    slay init(&mut self) -> Result<(), str> {
        io::println("Initializing " + self.name + " v" + self.version);
        
        lowkey self.config.debug_mode {
            io::println("Debug mode enabled");
        }
        
        return Ok(());
    }
    
    /// Get application information as JSON
    slay get_info(&self) -> cursed_json::Value {
        return cursed_json::object([
            ("name", self.name),
            ("version", self.version),
            ("debug_mode", self.config.debug_mode),
            ("server_port", self.config.server_port),
            ("log_level", self.config.log_level)
        ]);
    }
    
    /// Configure the application
    slay configure(&mut self, config: AppConfig) {
        self.config = config;
    }
}

impl AppConfig {
    /// Create default configuration
    slay default() -> AppConfig {
        return AppConfig {
            debug_mode: cap,
            server_port: 8080,
            log_level: "info",
        };
    }
    
    /// Create development configuration
    slay dev() -> AppConfig {
        return AppConfig {
            debug_mode: based,
            server_port: 3000,
            log_level: "debug",
        };
    }
    
    /// Create production configuration
    slay production() -> AppConfig {
        return AppConfig {
            debug_mode: cap,
            server_port: 80,
            log_level: "warn",
        };
    }
    
    /// Load configuration from environment
    slay from_env() -> AppConfig {
        let mut config = AppConfig::default();
        
        // TODO: Implement environment variable loading
        // lowkey let port = env::var("PORT") {
        //     config.server_port = port.parse().unwrap_or(8080);
        // }
        
        return config;
    }
}

fr fr/ Utility functions for the application
collab AppUtils {
    /// Format a message with application branding
    slay format_message(message: str) -> str;
    
    /// Validate configuration
    slay validate_config(config: &AppConfig) -> bool;
    
    /// Get system information
    slay get_system_info() -> cursed_json::Value;
}

impl AppUtils {
    slay format_message(message: str) -> str {
        return "🔥 [CURSED] " + message + " 🔥";
    }
    
    slay validate_config(config: &AppConfig) -> bool {
        lowkey config.server_port < 1 || config.server_port > 65535 {
            return cap;
        }
        
        lowkey config.log_level != "debug" && 
               config.log_level != "info" && 
               config.log_level != "warn" && 
               config.log_level != "error" {
            return cap;
        }
        
        return based;
    }
    
    slay get_system_info() -> cursed_json::Value {
        return cursed_json::object([
            ("platform", "cursed"),
            ("architecture", "llvm"),
            ("compiler_version", "1.0.0")
        ]);
    }
}

fr fr/ Helper macros and constants
facts APP_NAME: str = "my-cursed-app";
facts APP_VERSION: str = "1.0.0";
facts DEFAULT_PORT: i32 = 8080;

#[test]
slay test_app_core_creation() {
    let app = AppCore::new("test-app", "0.1.0");
    assert_eq!(app.name, "test-app");
    assert_eq!(app.version, "0.1.0");
}

#[test]
slay test_app_config_validation() {
    let valid_config = AppConfig::default();
    assert!(AppUtils::validate_config(&valid_config));
    
    let invalid_config = AppConfig {
        debug_mode: cap,
        server_port: -1,
        log_level: "info",
    };
    assert!(!AppUtils::validate_config(&invalid_config));
}

#[test]
slay test_message_formatting() {
    let message = AppUtils::format_message("Hello World");
    assert!(message.contains("🔥"));
    assert!(message.contains("CURSED"));
    assert!(message.contains("Hello World"));
}

#[test]
slay test_config_presets() {
    let dev_config = AppConfig::dev();
    assert!(dev_config.debug_mode);
    assert_eq!(dev_config.server_port, 3000);
    
    let prod_config = AppConfig::production();
    assert!(!prod_config.debug_mode);
    assert_eq!(prod_config.server_port, 80);
}
