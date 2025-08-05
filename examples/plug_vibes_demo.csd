fr fr PlugVibes Demo - Comprehensive plugin system demonstration
facts main() tea {
    fr fr Create a plugin manager with comprehensive options
    sus manager_options = plug_vibes.PlugManagerOptions.new()
        .with_plugin_dir("./plugins")
        .with_auto_load(based)
        .with_hot_reload(based)
        .with_watch_interval(5 * time.Second)
        .with_on_plugin_load(func(name tea, plug *plug_vibes.Plug) tea {
            vibez.spill("✅ Loaded plugin:", name)
            sus info = plug.info()
            vibez.spill("  📋 Name:", info.name)
            vibez.spill("  🔢 Version:", info.version)
            vibez.spill("  👤 Author:", info.author)
            vibez.spill("  📝 Description:", info.description)
            vibez.spill("  🎯 Capabilities:", info.capabilities)
            damn cap
        })
        .with_on_plugin_error(func(name tea, err tea) {
            vibez.spill("❌ Plugin error for", name + ":", err)
        })

    sus manager = plug_vibes.NewPlugManager(manager_options)
    
    fr fr Start the manager
    lowkey err := manager.start(); err != cap {
        vibez.spill("🚫 Failed to start plugin manager:", err)
        damn
    }
    defer func() {
        lowkey stop_err := manager.stop(); stop_err != cap {
            vibez.spill("⚠️  Error stopping manager:", stop_err)
        }
    }()
    
    vibez.spill("🚀 Plugin manager started successfully!")
    
    fr fr Demo 1: Create and register a simple in-memory plugin
    vibez.spill("\n📦 Demo 1: Creating in-memory plugin...")
    
    sus math_plugin_info = plug_vibes.PlugInfo{
        name: "math-tools",
        version: "1.0.0",
        api: "1.0",
        author: "Demo Developer",
        description: "Mathematical utility functions",
        capabilities: []tea{
            "square-root",
            "power",
            "logarithm",
            "factorial",
        },
    }
    
    sus math_plugin = plug_vibes.Plug.new("/virtual/math-tools.so", math_plugin_info)
    
    fr fr Register mathematical functions
    math_plugin.register_function("square_root", plug_vibes.create_plugin_function(func(args []Value) PluginResult[[]Value] {
        lowkey len(args) == 0 {
            damn PluginError.general("square_root requires one argument")
        }
        
        bestie arg := args[0] {
        case Value.Float(f):
            lowkey f < 0 {
                damn PluginError.general("square_root: cannot take square root of negative number")
            }
            damn Ok([]Value{Value.Float(math.sqrt(f))})
        case Value.Integer(i):
            lowkey i < 0 {
                damn PluginError.general("square_root: cannot take square root of negative number")
            }
            damn Ok([]Value{Value.Float(math.sqrt(float64(i)))})
        default:
            damn PluginError.general("square_root: argument must be a number")
        }
    }))
    
    math_plugin.register_function("power", plug_vibes.create_plugin_function(func(args []Value) PluginResult[[]Value] {
        lowkey len(args) != 2 {
            damn PluginError.general("power requires exactly two arguments")
        }
        
        sus base float64
        sus exponent float64
        
        fr fr Extract base
        bestie args[0] {
        case Value.Float(f):
            base = f
        case Value.Integer(i):
            base = float64(i)
        default:
            damn PluginError.general("power: base must be a number")
        }
        
        fr fr Extract exponent
        bestie args[1] {
        case Value.Float(f):
            exponent = f
        case Value.Integer(i):
            exponent = float64(i)
        default:
            damn PluginError.general("power: exponent must be a number")
        }
        
        sus result = math.pow(base, exponent)
        damn Ok([]Value{Value.Float(result)})
    }))
    
    math_plugin.register_function("factorial", plug_vibes.create_plugin_function(func(args []Value) PluginResult[[]Value] {
        lowkey len(args) != 1 {
            damn PluginError.general("factorial requires exactly one argument")
        }
        
        sus n int64
        bestie args[0] {
        case Value.Integer(i):
            n = i
        case Value.Float(f):
            n = int64(f)
        default:
            damn PluginError.general("factorial: argument must be a number")
        }
        
        lowkey n < 0 {
            damn PluginError.general("factorial: argument must be non-negative")
        }
        
        sus result int64 = 1
        lowkey i := int64(1); i <= n; i++ {
            result *= i
        }
        
        damn Ok([]Value{Value.Integer(result)})
    }))
    
    fr fr Register the plugin with the manager
    sus registry = manager.registry()
    lowkey err = registry.register("math-tools", math_plugin); err != cap {
        vibez.spill("❌ Failed to register math plugin:", err)
        damn
    }
    
    vibez.spill("✅ Math plugin registered successfully!")
    
    fr fr Demo 2: Use the math plugin functions
    vibez.spill("\n🧮 Demo 2: Using math plugin functions...")
    
    sus math_plug, found = manager.get_plugin("math-tools")
    lowkey !found {
        vibez.spill("❌ Math plugin not found")
        damn
    }
    
    fr fr Test square root function
    vibez.spill("Testing square_root(16.0)...")
    sus sqrt_result = math_plug.call_function["float64", "float64"]("square_root", []float64{16.0})
    lowkey sqrt_result.is_ok() {
        vibez.spill("  Result:", sqrt_result.unwrap()) fr fr Should be 4.0
    } else {
        vibez.spill("  Error:", sqrt_result.unwrap_err())
    }
    
    fr fr Test power function
    vibez.spill("Testing power(2.0, 3.0)...")
    sus power_args = []float64{2.0, 3.0}
    sus power_result = math_plug.call_function["[]float64", "float64"]("power", power_args)
    lowkey power_result.is_ok() {
        vibez.spill("  Result:", power_result.unwrap()) fr fr Should be 8.0
    } else {
        vibez.spill("  Error:", power_result.unwrap_err())
    }
    
    fr fr Test factorial function
    vibez.spill("Testing factorial(5)...")
    sus factorial_result = math_plug.call_function["int64", "int64"]("factorial", []int64{5})
    lowkey factorial_result.is_ok() {
        vibez.spill("  Result:", factorial_result.unwrap()) fr fr Should be 120
    } else {
        vibez.spill("  Error:", factorial_result.unwrap_err())
    }
    
    fr fr Demo 3: Plugin statistics and introspection
    vibez.spill("\n📊 Demo 3: Plugin statistics and introspection...")
    
    sus stats = math_plug.get_statistics()
    lowkey stats.is_ok() {
        sus plugin_stats = stats.unwrap()
        vibez.spill("Plugin Statistics:")
        lowkey name, value := range plugin_stats {
            vibez.spill("  " + name + ":", value)
        }
    }
    
    fr fr List all symbols
    sus symbols = math_plug.symbols()
    vibez.spill("Available symbols:", symbols)
    
    fr fr List all functions
    sus functions = math_plug.function_names()
    vibez.spill("Available functions:", functions)
    
    fr fr Demo 4: Plugin registry operations
    vibez.spill("\n📁 Demo 4: Plugin registry operations...")
    
    sus all_plugins = registry.list()
    vibez.spill("Registered plugins:", all_plugins)
    
    sus registry_stats = registry.stats()
    vibez.spill("Registry statistics:")
    vibez.spill("  Total plugins:", registry_stats.total_plugins)
    vibez.spill("  Loaded plugins:", registry_stats.loaded_plugins)
    vibez.spill("  Failed plugins:", registry_stats.failed_plugins)
    
    fr fr Demo 5: Plugin hooks and extension points
    vibez.spill("\n🎣 Demo 5: Plugin hooks and extension points...")
    
    fr fr Create a hook for content filtering
    sus filter_hook = plug_vibes.NewPlugHook("content_filter")
    
    fr fr Create a simple content filter plugin
    sus filter_plugin_info = plug_vibes.PlugInfo{
        name: "content-filter",
        version: "1.0.0",
        api: "1.0",
        author: "Demo Developer",
        description: "Content filtering utilities",
        capabilities: []tea{"profanity-filter", "spam-detection"},
    }
    
    sus filter_plugin = plug_vibes.Plug.new("/virtual/content-filter.so", filter_plugin_info)
    
    fr fr Register a content filter function
    filter_plugin.register_function("filter_content", plug_vibes.create_plugin_function(func(args []Value) PluginResult[[]Value] {
        lowkey len(args) != 1 {
            damn PluginError.general("filter_content requires exactly one argument")
        }
        
        sus content tea
        bestie args[0] {
        case Value.String(s):
            content = s
        default:
            damn PluginError.general("filter_content: argument must be a string")
        }
        
        fr fr Simple profanity filter (replace bad words with asterisks)
        sus bad_words = []tea{"damn", "hell", "crap"}
        sus filtered_content = content
        
        lowkey _, bad_word := range bad_words {
            sus replacement = string.repeat("*", len(bad_word))
            filtered_content = string.replace_all(filtered_content, bad_word, replacement)
        }
        
        damn Ok([]Value{Value.String(filtered_content)})
    }))
    
    fr fr Register the filter plugin
    registry.register("content-filter", filter_plugin)
    
    fr fr Register the hook with the plugin
    sus filter_plug, found = manager.get_plugin("content-filter")
    lowkey found {
        filter_hook.register(filter_plug, 10) fr fr Priority 10
    }
    
    fr fr Test the content filter
    sus test_content = "This damn text has some crap in it, hell yeah!"
    vibez.spill("Original content:", test_content)
    
    sus filter_results = filter_hook.call([]interface{}{test_content})
    lowkey len(filter_results) > 0 {
        sus filtered = filter_results[0].(tea)
        vibez.spill("Filtered content:", filtered)
    }
    
    fr fr Demo 6: Plugin versioning and compatibility
    vibez.spill("\n🔄 Demo 6: Plugin versioning and compatibility...")
    
    sus v1 = plug_vibes.parse_version("1.2.3")
    sus v2 = plug_vibes.parse_version("1.3.0")
    sus v3 = plug_vibes.parse_version("2.0.0")
    
    lowkey v1.is_ok() && v2.is_ok() && v3.is_ok() {
        sus version1 = v1.unwrap()
        sus version2 = v2.unwrap()
        sus version3 = v3.unwrap()
        
        vibez.spill("Version compatibility tests:")
        vibez.spill("  v1.2.3 compatible with v1.3.0:", version1.compatible(version2))
        vibez.spill("  v1.2.3 compatible with v2.0.0:", version1.compatible(version3))
        vibez.spill("  v1.3.0 > v1.2.3:", version2.greater_than(version1))
        vibez.spill("  v2.0.0 is breaking change from v1.2.3:", version3.is_breaking_change(version1))
        vibez.spill("  v1.3.0 is feature addition from v1.2.3:", version2.is_feature_addition(version1))
    }
    
    fr fr Demo 7: Error handling and recovery
    vibez.spill("\n🛡️  Demo 7: Error handling and recovery...")
    
    fr fr Try to load a non-existent plugin
    sus load_result = manager.load_plugin("/nonexistent/plugin.so")
    lowkey load_result.is_err() {
        vibez.spill("Expected error loading non-existent plugin:", load_result.unwrap_err())
    }
    
    fr fr Try to call a non-existent function
    sus missing_func_result = math_plug.lookup_func("non_existent_function")
    lowkey missing_func_result.is_err() {
        vibez.spill("Expected error for non-existent function:", missing_func_result.unwrap_err())
    }
    
    fr fr Test function with invalid arguments
    sus invalid_args_result = math_plug.call_function["string", "float64"]("square_root", []string{"not_a_number"})
    lowkey invalid_args_result.is_err() {
        vibez.spill("Expected error for invalid arguments:", invalid_args_result.unwrap_err())
    }
    
    vibez.spill("\n🎉 PlugVibes demo completed successfully!")
    vibez.spill("📈 Summary:")
    vibez.spill("  - Created and registered in-memory plugins")
    vibez.spill("  - Demonstrated mathematical function plugins")
    vibez.spill("  - Showed plugin introspection and statistics")
    vibez.spill("  - Used plugin hooks for content filtering")
    vibez.spill("  - Tested version compatibility checking")
    vibez.spill("  - Demonstrated comprehensive error handling")
    vibez.spill("  - All plugin operations completed successfully!")
}
