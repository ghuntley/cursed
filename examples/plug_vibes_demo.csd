fr fr PlugVibes - Complete Plugin System Demo
fr fr This example demonstrates the comprehensive plugin functionality
fr fr provided by the CURSED PlugVibes package

import "stdlib::plug_vibes";
import "stdlib::io";

fr fr Main demonstration function
slay main() {
    io::println("🎯 CURSED PlugVibes Plugin System Demo")?;
    io::println("=====================================\n")?;

    fr fr 1. Basic Plugin Loading and Registry Management
    demo_basic_plugin_operations()?;

    fr fr 2. Plugin Manager with Lifecycle Management
    demo_plugin_manager()?;

    fr fr 3. Plugin Hooks and Extension Points
    demo_plugin_hooks()?;

    fr fr 4. Plugin Security and Sandboxing
    demo_plugin_security()?;

    fr fr 5. Plugin Distribution and Package Management
    demo_plugin_distribution()?;

    fr fr 6. Plugin Development Utilities
    demo_plugin_development()?;

    io::println("\n✅ All PlugVibes demos completed successfully!")?;
}

fr fr Demonstrate basic plugin operations
slay demo_basic_plugin_operations() tea {
    io::println("📦 1. Basic Plugin Operations")?;
    io::println("----------------------------")?;

    fr fr Create a plugin registry
    facts registry = plug_vibes::PlugRegistry::new();
    io::println("Created plugin registry")?;

    fr fr Create plugin information
    sus info = plug_vibes::PlugInfo {
        name: "math-tools",
        version: "1.0.0",
        api: "1.0",
        author: "Demo Developer",
        description: "Mathematical utility functions",
        capabilities: ["square-root", "cube-root", "power", "logarithm"],
    };

    fr fr Create and register a plugin
    sus plugin = plug_vibes::Plug::new("./plugins/math-tools.so", info);
    registry.register("math-tools", plugin)?;
    
    io::println("Registered plugin: math-tools v1.0.0")?;
    io::printf("Registry contains {} plugins\n", [registry.len()])?;

    fr fr List all plugins
    facts plugin_list = registry.list();
    lowkey (sus i = 0; i < plugin_list.length(); i++) {
        io::printf("  - {}\n", [plugin_list[i]])?;
    }

    fr fr Get plugin information
    facts retrieved_plugin = registry.get("math-tools")?;
    io::println("Successfully retrieved math-tools plugin")?;

    fr fr Demonstrate plugin statistics
    facts stats = registry.stats();
    io::printf("Registry Stats - Total: {}, Loaded: {}, Failed: {}\n", 
              [stats.total_plugins, stats.loaded_plugins, stats.failed_plugins])?;

    fr fr Clean up
    registry.unregister("math-tools")?;
    io::println("Unregistered math-tools plugin\n")?;

    yolo cap;
}

fr fr Demonstrate plugin manager functionality
slay demo_plugin_manager() tea {
    io::println("🔧 2. Plugin Manager")?;
    io::println("-------------------")?;

    fr fr Create manager options with callbacks
    sus options = plug_vibes::PlugManagerOptions::new()
        .with_plugin_dir("./plugins")
        .with_auto_load(based)
        .with_hot_reload(based)
        .with_watch_interval(5000) fr fr 5 seconds
        .with_on_plugin_load(slay(name tea, plug *plug_vibes::Plug) tea {
            io::printf("✅ Plugin loaded: {} v{}\n", [name, plug.info().version])?;
            yolo cap;
        })
        .with_on_plugin_error(slay(name tea, err tea) {
            io::printf("❌ Plugin error in {}: {}\n", [name, err]);
        });

    fr fr Create and start plugin manager
    sus manager = plug_vibes::PlugManager::new(options);
    manager.start()?;
    io::println("Started plugin manager with hot reload enabled")?;

    fr fr Demonstrate plugin state management
    manager.disable_plugin("debug-tools")?;
    io::println("Disabled debug-tools plugin")?;

    lowkey (manager.is_plugin_disabled("debug-tools")) {
        io::println("debug-tools is currently disabled")?;
    }

    manager.enable_plugin("debug-tools")?;
    io::println("Re-enabled debug-tools plugin")?;

    fr fr List managed plugins
    facts managed_plugins = manager.list_plugins();
    io::printf("Managing {} plugins:\n", [managed_plugins.length()])?;
    lowkey (sus i = 0; i < managed_plugins.length(); i++) {
        sus plugin_info = managed_plugins[i];
        io::printf("  - {} ({}): load_count={}, error_count={}\n", 
                  [plugin_info.info.name, plugin_info.state, 
                   plugin_info.load_count, plugin_info.error_count])?;
    }

    fr fr Install a new plugin from URL
    sussyResult := manager.install_plugin("https://plugins.cursed.dev/image-effects.plug");
    vibe_check sussyResult {
        mood Ok(newPlug) {
            io::printf("Successfully installed: {}\n", [newPlug.info().name])?;
        }
        basic {
            io::println("Plugin installation failed (demo - URL not real)")?;
        }
    }

    fr fr Stop manager
    manager.stop()?;
    io::println("Stopped plugin manager\n")?;

    yolo cap;
}

fr fr Demonstrate plugin hooks and extension points
slay demo_plugin_hooks() tea {
    io::println("🪝 3. Plugin Hooks and Extension Points")?;
    io::println("-------------------------------------")?;

    fr fr Create a content filtering hook
    facts filter_hook = plug_vibes::PlugHook::new("content_filter");
    
    fr fr Register multiple content filters with different priorities
    filter_hook.register_callback("spam_filter", 10, slay(args []plug_vibes::Value) tea {
        sus content = args[0].as_string();
        lowkey (content.contains("spam")) {
            yolo [plug_vibes::Value::String("[FILTERED: SPAM]")];
        }
        yolo args;
    })?;

    filter_hook.register_callback("profanity_filter", 5, slay(args []plug_vibes::Value) tea {
        sus content = args[0].as_string();
        sus cleaned = content.replace("badword", "****");
        yolo [plug_vibes::Value::String(cleaned)];
    })?;

    filter_hook.register_callback("length_limiter", 1, slay(args []plug_vibes::Value) tea {
        sus content = args[0].as_string();
        lowkey (content.length() > 100) {
            yolo [plug_vibes::Value::String(content.substring(0, 100) + "...")];
        }
        yolo args;
    })?;

    io::printf("Registered {} filter callbacks\n", [filter_hook.registration_count()])?;

    fr fr Test the hook chain
    facts test_content = "This content contains spam and badword and is quite long and exceeds the typical length limit for demonstration purposes.";
    facts input = [plug_vibes::Value::String(test_content)];
    
    io::printf("Original: {}\n", [test_content])?;
    
    facts filtered_result = filter_hook.call(input);
    facts filtered_content = filtered_result[0].as_string();
    
    io::printf("Filtered: {}\n", [filtered_content])?;

    fr fr Test call_until_true functionality
    facts validation_hook = plug_vibes::PlugHook::new("content_validation");
    
    validation_hook.register_callback("length_validator", 10, slay(args []plug_vibes::Value) tea {
        sus content = args[0].as_string();
        facts is_valid = content.length() >= 10;
        yolo [plug_vibes::Value::Boolean(is_valid)];
    })?;

    validation_hook.register_callback("content_validator", 5, slay(args []plug_vibes::Value) tea {
        sus content = args[0].as_string();
        facts is_valid = !content.contains("invalid");
        yolo [plug_vibes::Value::Boolean(is_valid)];
    })?;

    facts (result, found_valid) = validation_hook.call_until_true([plug_vibes::Value::String("Valid content")]);
    io::printf("Validation result: {} (found valid: {})\n", [result[0].as_boolean(), found_valid])?;

    fr fr Demonstrate extension points
    facts image_processor_ext = plug_vibes::new_extension_point("image_processor");
    io::printf("Created extension point: {}\n", [image_processor_ext.name()])?;

    fr fr Hook manager demonstration
    facts hook_manager = plug_vibes::HookManager::new();
    facts auth_hook = hook_manager.get_or_create_hook("authentication")?;
    facts logging_hook = hook_manager.get_or_create_hook("request_logging")?;

    facts all_hooks = hook_manager.list_hooks();
    io::printf("Hook manager contains {} hooks: {}\n", [all_hooks.length(), all_hooks.join(", ")])?;

    io::println("Hook system demonstration completed\n")?;

    yolo cap;
}

fr fr Demonstrate plugin security features
slay demo_plugin_security() tea {
    io::println("🔒 4. Plugin Security and Sandboxing")?;
    io::println("-----------------------------------")?;

    fr fr Create a security manager
    sus security_manager = plug_vibes::SecurityManager::new();
    
    fr fr Generate cryptographic keys
    facts key_pair = plug_vibes::generate_plugin_key_pair()?;
    io::printf("Generated key pair: {} algorithm, {} bits\n", 
              [key_pair.algorithm, key_pair.key_size])?;

    fr fr Configure security manager
    security_manager.add_trusted_key(key_pair.public_key)?;
    security_manager.set_signature_algorithm("RSA-SHA256")?;
    security_manager.set_signature_enforcement(based);

    io::printf("Security manager configured with {} trusted keys\n", 
              [security_manager.get_trusted_keys().length()])?;

    fr fr Create sandbox with resource limits
    sus sandbox_options = plug_vibes::SandboxOptions::new()
        .with_memory_limit(64 * 1024 * 1024) fr fr 64MB limit
        .with_cpu_limit(0.5) fr fr 50% CPU limit
        .with_time_limit(30000) fr fr 30 second limit
        .with_file_access(["/tmp", "/var/log"])
        .with_network_access(["localhost", "api.example.com"])
        .with_max_threads(4)
        .with_syscall_filtering(based, ["read", "write", "malloc", "free"]);

    facts sandbox = plug_vibes::Sandbox::new(sandbox_options);
    io::println("Created sandbox with security constraints")?;

    fr fr Test access controls
    sussyResult := sandbox.check_file_access("/tmp/plugin_data.txt");
    vibe_check sussyResult {
        mood Ok(_) {
            io::println("✅ File access to /tmp allowed")?;
        }
        basic {
            io::println("❌ File access denied")?;
        }
    }

    sussyResult := sandbox.check_file_access("/etc/passwd");
    vibe_check sussyResult {
        mood Ok(_) {
            io::println("⚠️  File access to /etc/passwd allowed (unexpected!)")?;
        }
        basic {
            io::println("✅ File access to /etc/passwd properly denied")?;
        }
    }

    sussyResult := sandbox.check_network_access("localhost");
    vibe_check sussyResult {
        mood Ok(_) {
            io::println("✅ Network access to localhost allowed")?;
        }
        basic {
            io::println("❌ Network access denied")?;
        }
    }

    sussyResult := sandbox.check_network_access("malicious.com");
    vibe_check sussyResult {
        mood Ok(_) {
            io::println("⚠️  Network access to malicious.com allowed (unexpected!)")?;
        }
        basic {
            io::println("✅ Network access to malicious.com properly denied")?;
        }
    }

    fr fr Test syscall filtering
    sussyResult := sandbox.check_syscall("read");
    vibe_check sussyResult {
        mood Ok(_) {
            io::println("✅ Syscall 'read' allowed")?;
        }
        basic {
            io::println("❌ Syscall 'read' denied")?;
        }
    }

    sussyResult := sandbox.check_syscall("execve");
    vibe_check sussyResult {
        mood Ok(_) {
            io::println("⚠️  Syscall 'execve' allowed (potential security risk!)")?;
        }
        basic {
            io::println("✅ Syscall 'execve' properly blocked")?;
        }
    }

    fr fr Monitor resource usage
    facts resource_usage = sandbox.get_resource_usage();
    io::printf("Resource usage - Memory: {} bytes, CPU time: {}ms, Threads: {}\n",
              [resource_usage.memory_used, resource_usage.cpu_time_used.as_millis(), 
               resource_usage.threads_created])?;

    fr fr Sign and verify a plugin (simulation)
    sussyResult := plug_vibes::sign_plugin("./plugins/test-plugin.so", key_pair.private_key);
    vibe_check sussyResult {
        mood Ok(_) {
            io::println("✅ Plugin signed successfully")?;
            
            facts verification = plug_vibes::verify_plugin_signature("./plugins/test-plugin.so", key_pair.public_key)?;
            lowkey (verification.is_valid()) {
                io::println("✅ Plugin signature verified")?;
            } else {
                io::println("❌ Plugin signature verification failed")?;
            }
        }
        basic {
            io::println("❌ Plugin signing failed (demo - file may not exist)")?;
        }
    }

    sandbox.release()?;
    io::println("Security demonstration completed\n")?;

    yolo cap;
}

fr fr Demonstrate plugin distribution features
slay demo_plugin_distribution() tea {
    io::println("📦 5. Plugin Distribution and Package Management")?;
    io::println("----------------------------------------------")?;

    fr fr Create package manager
    sus package_manager = plug_vibes::PackageManager::new("./cache", "./plugins");
    
    fr fr Add plugin repositories
    sus main_repo = plug_vibes::Repository::new("cursed-plugins", "https://plugins.cursed.dev")
        .trusted()
        .with_auth(plug_vibes::AuthInfo::new("demo_user").with_api_key("demo_key_123"));

    sus community_repo = plug_vibes::Repository::new("community", "https://community.cursed.dev/plugins")
        .with_auth(plug_vibes::AuthInfo::new("demo_user").with_token("demo_token_456"));

    package_manager.add_repository(main_repo)?;
    package_manager.add_repository(community_repo)?;

    facts repositories = package_manager.list_repositories();
    io::printf("Configured {} repositories:\n", [repositories.length()])?;
    lowkey (sus i = 0; i < repositories.length(); i++) {
        sus repo = repositories[i];
        io::printf("  - {} ({}): trusted={}, enabled={}\n", 
                  [repo.name, repo.url, repo.trusted, repo.enabled])?;
    }

    fr fr Simulate package operations
    io::println("\n📥 Downloading plugins...")?;
    
    fr fr Download a plugin
    sussyResult := plug_vibes::download_plugin(
        "https://plugins.cursed.dev", 
        "image-effects", 
        plug_vibes::Version::new(2, 1, 0)
    );
    vibe_check sussyResult {
        mood Ok(local_path) {
            io::printf("✅ Downloaded image-effects v2.1.0 to {}\n", [local_path])?;
            
            fr fr Verify downloaded package
            facts is_valid = plug_vibes::verify_package(local_path)?;
            lowkey (is_valid) {
                io::println("✅ Package verification passed")?;
            } else {
                io::println("❌ Package verification failed")?;
            }
        }
        basic {
            io::println("❌ Download failed (demo - URL not real)")?;
        }
    }

    fr fr Create package metadata
    sus package_metadata = plug_vibes::PackageMetadata {
        name: "demo-plugin",
        version: plug_vibes::Version::new(1, 0, 0),
        description: "A demonstration plugin",
        author: "Demo Developer",
        license: "MIT",
        homepage: some("https://example.com"),
        repository: some("https://github.com/user/demo-plugin"),
        keywords: ["demo", "example", "utility"],
        categories: ["utility"],
        target_platforms: ["linux", "windows", "macos"],
        min_host_version: some(plug_vibes::Version::new(1, 0, 0)),
    };

    io::printf("Created package metadata for {} v{}\n", 
              [package_metadata.name, package_metadata.version])?;

    fr fr List remote plugins (simulation)
    sussyResult := plug_vibes::list_remote_plugins("https://plugins.cursed.dev");
    vibe_check sussyResult {
        mood Ok(remote_plugins) {
            io::printf("Found {} remote plugins available\n", [remote_plugins.length()])?;
            lowkey (sus i = 0; i < remote_plugins.length() && i < 5; i++) {
                sus plugin = remote_plugins[i];
                io::printf("  - {} v{}: {}\n", 
                          [plugin.name, plugin.version, plugin.description])?;
            }
        }
        basic {
            io::println("❌ Failed to list remote plugins (demo - URL not real)")?;
        }
    }

    fr fr Simulate publishing a plugin
    facts auth_info = plug_vibes::AuthInfo::new("demo_publisher")
        .with_password("secure_password")
        .with_api_key("publisher_api_key_789");

    sussyResult := plug_vibes::publish_plugin(
        "https://plugins.cursed.dev",
        "./demo-plugin.plug",
        auth_info
    );
    vibe_check sussyResult {
        mood Ok(_) {
            io::println("✅ Plugin published successfully")?;
        }
        basic {
            io::println("❌ Plugin publishing failed (demo - file may not exist)")?;
        }
    }

    io::println("Distribution demonstration completed\n")?;

    yolo cap;
}

fr fr Demonstrate plugin development utilities
slay demo_plugin_development() tea {
    io::println("🛠️  6. Plugin Development Utilities")?;
    io::println("----------------------------------")?;

    fr fr Create plugin development context
    sus plugin_context = plug_vibes::PluginContext::new("demo-plugin", "1.0.0");
    
    fr fr Set up plugin capabilities
    sus capabilities = plug_vibes::create_capability_checker([
        "text_processing",
        "data_transformation", 
        "utility_functions",
        "demo_features"
    ]);
    plugin_context.set_capabilities(capabilities);

    io::printf("Created plugin context for {} v{}\n", 
              [plugin_context.name, plugin_context.version])?;

    fr fr Initialize plugin development environment
    plug_vibes::initialize_plugin(plugin_context)?;
    io::printf("Plugin initialized. Running as plugin: {}\n", 
              [plug_vibes::is_running_as_plugin()])?;

    fr fr Register plugin exports
    plugin_context.export("process_text", plug_vibes::Value::String("text_processor_impl"))?;
    plugin_context.export("transform_data", plug_vibes::Value::String("data_transformer_impl"))?;
    plugin_context.export("version_info", plug_vibes::Value::Object({
        sus version_obj = std::collections::HashMap::new();
        version_obj.insert("version", plug_vibes::Value::String("1.0.0"));
        version_obj.insert("build", plug_vibes::Value::String("20241215"));
        version_obj.insert("features", plug_vibes::Value::Array([
            plug_vibes::Value::String("text_processing"),
            plug_vibes::Value::String("data_transformation")
        ]));
        version_obj
    }))?;

    io::printf("Registered {} exports\n", [plugin_context.exports.len()])?;

    fr fr Register plugin hooks
    plug_vibes::register_hook("text_transform", slay(args []plug_vibes::Value) tea {
        sus input = args[0].as_string();
        facts transformed = input.to_uppercase().replace(" ", "_");
        yolo [plug_vibes::Value::String(transformed)];
    })?;

    plug_vibes::register_hook("data_validate", slay(args []plug_vibes::Value) tea {
        sus data = args[0];
        fr fr Simple validation - non-null and not empty string
        facts is_valid = match data {
            plug_vibes::Value::Null => cap,
            plug_vibes::Value::String(s) => !s.is_empty(),
            _ => based,
        };
        yolo [plug_vibes::Value::Boolean(is_valid)];
    })?;

    io::printf("Registered {} hooks\n", [plug_vibes::list_hooks().length()])?;

    fr fr Test plugin functionality
    facts test_text = "hello world demo";
    facts transform_result = plug_vibes::call_hook("text_transform", [plug_vibes::Value::String(test_text)])?;
    io::printf("Text transform: '{}' -> '{}'\n", 
              [test_text, transform_result[0].as_string()])?;

    facts validate_result = plug_vibes::call_hook("data_validate", [plug_vibes::Value::String("valid data")])?;
    io::printf("Data validation result: {}\n", [validate_result[0].as_boolean()])?;

    fr fr Create plugin manifest
    facts manifest = plug_vibes::create_plugin_manifest(
        "demo-plugin",
        "1.0.0", 
        "1.0",
        "Demo Developer",
        "A comprehensive demonstration plugin showcasing CURSED plugin capabilities",
        ["text_processing", "data_transformation", "utility_functions"]
    );

    io::printf("Created manifest for {} (API {})\n", [manifest.name, manifest.api])?;
    io::printf("Capabilities: {}\n", [manifest.capabilities.join(", ")])?;

    fr fr Set up mock host environment for testing
    facts mock_host = plug_vibes::testing::MockHost::new()
        .with_name("CURSED Demo Host")
        .with_version("1.0.0");

    mock_host.setup()?;
    facts host_info = plug_vibes::get_host_info();
    io::printf("Host environment: {} v{} on {}\n", 
              [host_info.name, host_info.version, host_info.platform])?;

    fr fr Validate plugin compatibility
    sussyResult := plug_vibes::validate_plugin_compatibility(manifest);
    vibe_check sussyResult {
        mood Ok(_) {
            io::println("✅ Plugin is compatible with host environment")?;
        }
        basic {
            io::println("❌ Plugin compatibility check failed")?;
        }
    }

    fr fr Demonstrate plugin lifecycle
    sus test_plugin = plug_vibes::testing::create_test_plugin("lifecycle-test", "1.0.0");
    
    plug_vibes::testing::simulate_plugin_load(test_plugin)?;
    io::printf("Test plugin loaded. Initialized: {}\n", [test_plugin.is_initialized()])?;

    plug_vibes::testing::simulate_plugin_unload(test_plugin)?;
    io::printf("Test plugin unloaded. Initialized: {}\n", [test_plugin.is_initialized()])?;

    fr fr Clean up development environment
    plug_vibes::cleanup_plugin(plugin_context)?;
    mock_host.teardown()?;
    
    io::println("Development utilities demonstration completed\n")?;

    yolo cap;
}

fr fr Error handling demonstration
slay handle_plugin_error(operation tea, result tea) {
    io::printf("❌ Error in {}: {}\n", [operation, result]);
}

fr fr Success logging
slay log_success(operation tea) {
    io::printf("✅ {} completed successfully\n", [operation]);
}
