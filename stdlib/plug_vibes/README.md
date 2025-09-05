# PlugVibes Module

## Overview
PlugVibes provides functionality for loading and using plugins to extend applications at runtime with good vibes. It's inspired by Go's plugin package but with enhanced features, improved usability, and stronger safety guarantees.

## Core Types

### `Plug`
Represents a loaded plugin.
- **path** - File path of the plugin
- **symbols** - Map of exported symbols
- **info** - Plugin metadata
- **loaded** - Loading status

### `LoadOptions`
Options for loading plugins with various security and isolation settings.
- **VersionCheck** - Verify plugin compatibility
- **VerifySignature** - Verify plugin signature
- **Isolation** - Load plugin in isolated context
- **Sandbox** - Run plugin in sandboxed environment
- **Timeout** - Timeout for plugin initialization
- **Dependencies** - Additional plugin dependencies
- **AllowedImports** - Whitelist of packages

### `PlugInfo`
Information about a loaded plugin.
- **Name** - Plugin name
- **Version** - Plugin version
- **API** - API version
- **Author** - Plugin author
- **Description** - Plugin description
- **BuildTime** - Build timestamp
- **Dependencies** - Required dependencies
- **Capabilities** - Plugin capabilities
- **Signature** - Security signature
- **IsVerified** - Signature verification status
- **IsCompatible** - Compatibility status

### `PlugRegistry`
Registry for managing multiple plugins.
- **plugins** - Map of registered plugins
- **loadOrder** - Order of plugin loading

### `PlugManager`
High-level plugin management with automatic loading and lifecycle management.

## Core Functions

### Plugin Loading
- **Load(path tea) (*Plug, tea)** - Load plugin from file path
- **LoadWithOptions(path tea, opts LoadOptions) (*Plug, tea)** - Load with specific options

### Plugin Operations
- **Lookup(symbolName tea) (interface{}, tea)** - Look up symbol by name
- **LookupFunc(funcName tea) (slay(interface{}) interface{}, tea)** - Look up function
- **LookupSymbol(symbolName tea, symbol interface{}) tea** - Look up and assign symbol
- **Close() tea** - Close and unload plugin
- **Path() tea** - Get plugin file path
- **Symbols() []tea** - Get list of exported symbols
- **Info() PlugInfo** - Get plugin information

## Plugin Registry

### Registry Operations
- **NewPlugRegistry() *PlugRegistry** - Create new registry
- **Register(name tea, plug *Plug) tea** - Register plugin
- **Unregister(name tea) tea** - Unregister plugin
- **Get(name tea) (*Plug, lit)** - Get plugin by name
- **List() []tea** - List registered plugin names
- **LoadAndRegister(path, name tea) (*Plug, tea)** - Load and register plugin
- **LoadAll(directory tea) (map[tea]*Plug, tea)** - Load all plugins from directory
- **Close() tea** - Close all plugins

## Plugin Manager

### Manager Operations
- **NewPlugManager(opts PlugManagerOptions) *PlugManager** - Create plugin manager
- **Start() tea** - Start plugin manager
- **Stop() tea** - Stop plugin manager
- **LoadPlugin(path tea) (*Plug, tea)** - Load single plugin
- **UnloadPlugin(name tea) tea** - Unload plugin
- **ReloadPlugin(name tea) tea** - Reload plugin
- **GetPlugin(name tea) (*Plug, lit)** - Get plugin by name
- **ListPlugins() []PlugInfo** - List all plugin information
- **InstallPlugin(src tea) (*Plug, tea)** - Install plugin from source
- **EnablePlugin(name tea) tea** - Enable plugin
- **DisablePlugin(name tea) tea** - Disable plugin

## Hook System

### Plugin Hooks
- **NewPlugHook(name tea) *PlugHook** - Create new hook
- **Register(plug *Plug, priority normie) tea** - Register plugin with hook
- **Unregister(plug *Plug) tea** - Unregister plugin from hook
- **Call(args interface{}) []interface{}** - Call all registered hooks
- **CallUntilTrue(args interface{}) (interface{}, lit)** - Call until one returns true
- **CallUntilError(args interface{}) (interface{}, tea)** - Call until error occurs

### Extension Points
- **NewExtensionPoint(name tea, extensionType interface{}) ExtensionPointStruct** - Create extension point
- **Register(extension interface{}) tea** - Register extension
- **Unregister(extension interface{}) tea** - Unregister extension
- **GetExtensions() []interface{}** - Get all extensions

## Sandboxing

### Sandbox Creation
- **NewSandbox(options SandboxOptions) *Sandbox** - Create sandbox
- **LoadPlugin(path tea) (*Plug, tea)** - Load plugin in sandbox
- **ExecuteFunc(plug *Plug, funcName tea, args interface{}) (interface{}, tea)** - Execute function safely
- **Release() tea** - Release sandbox resources

### Sandbox Options
- **MemoryLimit** - Maximum memory usage
- **CPULimit** - CPU usage limit (0.0-1.0)
- **TimeLimit** - Maximum execution time
- **FileAccess** - Allow file system access
- **NetworkAccess** - Allow network access
- **AllowedPaths** - Whitelisted file paths
- **AllowedHosts** - Whitelisted network hosts

## Version Management

### Version Operations
- **ParseVersion(v tea) (Version, tea)** - Parse version string
- **String() tea** - Convert version to string
- **Compatible(other Version) lit** - Check compatibility
- **GreaterThan(other Version) lit** - Compare versions
- **LessThan(other Version) lit** - Compare versions
- **Equal(other Version) lit** - Check equality

## Security Functions

### Plugin Security
- **VerifyPluginSignature(path, pubKey tea) (lit, tea)** - Verify plugin signature
- **SignPlugin(path, privateKey tea) tea** - Sign plugin
- **GeneratePluginKeyPair() (tea, tea, tea)** - Generate key pair

## Distribution Functions

### Plugin Packaging
- **PackPlugin(dir, output tea) tea** - Package plugin
- **UnpackPlugin(pkgPath, outputDir tea) tea** - Unpack plugin
- **VerifyPackage(pkgPath tea) (lit, tea)** - Verify package integrity

### Remote Repository
- **ListRemotePlugins(repoURL tea) ([]PlugInfo, tea)** - List remote plugins
- **DownloadPlugin(repoURL, pluginName tea, version Version) (tea, tea)** - Download plugin
- **PublishPlugin(repoURL, pkgPath tea, auth AuthInfo) tea** - Publish plugin

## Development Functions

### Plugin Development
- **IsRunningAsPlugin() lit** - Check if running as plugin
- **GetHostInfo() HostInfo** - Get host application info
- **GetPluginAPI() tea** - Get plugin API version
- **RegisterExport(name tea, value interface{})** - Register exported symbol
- **RegisterHook(name tea, callback slay(interface{}) interface{})** - Register hook callback

## Usage Examples

### Basic Plugin Loading
```cursed
yeet "plug_vibes"

fr fr Load a plugin
sus plugin, err := plug_vibes.Load("math_plugin.💀")
if err != "" {
    vibez.spill("Failed to load plugin:", err)
    yolo
}

fr fr Get plugin information
sus info := plugin.Info()
vibez.spill("Plugin:", info.Name)
vibez.spill("Version:", info.Version)
vibez.spill("Author:", info.Author)

fr fr Look up a function
sus calcFunc, lookupErr := plugin.Lookup("Calculate")
if lookupErr == "" {
    vibez.spill("Found Calculate function")
}

fr fr Close plugin when done
plugin.Close()
```

### Plugin Manager Usage
```cursed
fr fr Create plugin manager
sus options := plug_vibes.PlugManagerOptions{
    PluginDir: "./plugins",
    AutoLoad: based,
    HotReload: based,
    OnPluginLoad: slay(name tea, plug *plug_vibes.Plug) tea {
        vibez.spill("Loaded plugin:", name)
        damn ""
    },
    OnPluginError: slay(name tea, err tea) {
        vibez.spill("Plugin error:", name, err)
    }
}

sus manager := plug_vibes.NewPlugManager(options)

fr fr Start manager
sus startErr := manager.Start()
if startErr == "" {
    vibez.spill("Plugin manager started")
}

fr fr Load specific plugin
sus mathPlugin, loadErr := manager.LoadPlugin("math_tools.💀")
if loadErr == "" {
    vibez.spill("Math plugin loaded")
}

fr fr List all plugins
sus plugins := manager.ListPlugins()
for i := 0; i < len(plugins); i++ {
    vibez.spill("Plugin:", plugins[i].Name)
}

fr fr Stop manager
manager.Stop()
```

### Plugin Registry
```cursed
fr fr Create registry
sus registry := plug_vibes.NewPlugRegistry()

fr fr Load and register plugins
sus plugin1, err1 := registry.LoadAndRegister("plugin1.💀", "plugin1")
sus plugin2, err2 := registry.LoadAndRegister("plugin2.💀", "plugin2")

if err1 == "" && err2 == "" {
    vibez.spill("Both plugins loaded")
}

fr fr Get plugin by name
sus retrievedPlugin, found := registry.Get("plugin1")
if found {
    vibez.spill("Found plugin1")
}

fr fr Load all plugins from directory
sus allPlugins, loadAllErr := registry.LoadAll("./plugin_directory")
if loadAllErr == "" {
    vibez.spill("Loaded", len(allPlugins), "plugins")
}

fr fr Clean up
registry.Close()
```

### Hook System
```cursed
fr fr Create hook for content filtering
sus hook := plug_vibes.NewPlugHook("filter_content")

fr fr Register plugins with hook
sus filterPlugin, filterErr := plug_vibes.Load("content_filter.💀")
if filterErr == "" {
    hook.Register(filterPlugin, 10)  fr fr Priority 10
}

sus cleanupPlugin, cleanupErr := plug_vibes.Load("cleanup_filter.💀")
if cleanupErr == "" {
    hook.Register(cleanupPlugin, 5)  fr fr Priority 5
}

fr fr Call hooks
sus content := "Some content to filter"
sus results := hook.Call(content)
vibez.spill("Hook results:", len(results))

fr fr Call until one returns true
sus result, success := hook.CallUntilTrue(content)
if success {
    vibez.spill("Hook succeeded with result:", result)
}
```

### Sandboxed Execution
```cursed
fr fr Create sandbox with restrictions
sus sandboxOpts := plug_vibes.SandboxOptions{
    MemoryLimit: 1024000,    fr fr 1MB memory limit
    CPULimit: 0.5,           fr fr 50% CPU limit
    TimeLimit: 30,           fr fr 30 second timeout
    FileAccess: cap,         fr fr No file access
    NetworkAccess: cap,      fr fr No network access
    AllowedPaths: []tea{"/tmp"},
    AllowedHosts: []tea{"localhost"}
}

sus sandbox := plug_vibes.NewSandbox(sandboxOpts)

fr fr Load plugin in sandbox
sus safePlugin, safeErr := sandbox.LoadPlugin("untrusted_plugin.💀")
if safeErr == "" {
    fr fr Execute function safely
    sus result, execErr := sandbox.ExecuteFunc(safePlugin, "ProcessData", "input data")
    if execErr == "" {
        vibez.spill("Safe execution result:", result)
    }
}

fr fr Clean up sandbox
sandbox.Release()
```

### Version Management
```cursed
fr fr Parse and compare versions
sus version1, v1Err := plug_vibes.ParseVersion("1.2.3")
sus version2, v2Err := plug_vibes.ParseVersion("2.0.0")

if v1Err == "" && v2Err == "" {
    vibez.spill("Version 1:", version1.String())
    vibez.spill("Version 2:", version2.String())
    
    if version1.Compatible(version2) {
        vibez.spill("Versions are compatible")
    }
    
    if version2.GreaterThan(version1) {
        vibez.spill("Version 2 is newer")
    }
}
```

### Security and Distribution
```cursed
fr fr Generate key pair for signing
sus privateKey, publicKey, keyErr := plug_vibes.GeneratePluginKeyPair()
if keyErr == "" {
    fr fr Sign plugin
    sus signErr := plug_vibes.SignPlugin("my_plugin.💀", privateKey)
    if signErr == "" {
        fr fr Verify signature
        sus verified, verifyErr := plug_vibes.VerifyPluginSignature("my_plugin.💀", publicKey)
        if verifyErr == "" && verified {
            vibez.spill("Plugin signature verified")
        }
    }
}

fr fr Package plugin for distribution
sus packErr := plug_vibes.PackPlugin("./my_plugin_source", "my_plugin.plug")
if packErr == "" {
    vibez.spill("Plugin packaged successfully")
    
    fr fr Verify package
    sus packageOK, packageErr := plug_vibes.VerifyPackage("my_plugin.plug")
    if packageErr == "" && packageOK {
        vibez.spill("Package verification passed")
    }
}

fr fr Remote operations
sus remotePlugins, remoteErr := plug_vibes.ListRemotePlugins("https://plugins.example.com")
if remoteErr == "" {
    vibez.spill("Found", len(remotePlugins), "remote plugins")
    
    fr fr Download plugin
    if len(remotePlugins) > 0 {
        sus firstPlugin := remotePlugins[0]
        sus pluginVersion, parseErr := plug_vibes.ParseVersion(firstPlugin.Version)
        if parseErr == "" {
            sus downloadPath, downloadErr := plug_vibes.DownloadPlugin(
                "https://plugins.example.com", 
                firstPlugin.Name, 
                pluginVersion
            )
            if downloadErr == "" {
                vibez.spill("Downloaded to:", downloadPath)
            }
        }
    }
}
```

### Extension Points
```cursed
fr fr Create extension point for image filters
sus imageFilters := plug_vibes.NewExtensionPoint("image_filter", "ImageFilterInterface")

fr fr Register filter extensions
sus blurFilter := "BlurFilterImplementation"
sus sharpenFilter := "SharpenFilterImplementation"

imageFilters.Register(blurFilter)
imageFilters.Register(sharpenFilter)

fr fr Get all registered filters
sus allFilters := imageFilters.GetExtensions()
vibez.spill("Registered filters:", len(allFilters))

fr fr Unregister filter
imageFilters.Unregister(blurFilter)
sus remainingFilters := imageFilters.GetExtensions()
vibez.spill("Remaining filters:", len(remainingFilters))
```

## Implementation Features

1. **Pure CURSED Implementation** - No FFI dependencies
2. **Comprehensive Security** - Sandboxing, signature verification, isolation
3. **Flexible Loading** - Multiple loading options and configurations
4. **Hot Reloading** - Runtime plugin reloading support
5. **Hook System** - Extensible hook and extension point system
6. **Version Management** - Semantic versioning with compatibility checks
7. **Distribution Support** - Packaging, signing, and remote repository integration
8. **Error Handling** - Comprehensive error reporting and recovery
9. **Resource Management** - Automatic cleanup and resource limits

## Error Handling

All functions return error messages as strings:
- Empty string ("") indicates success
- Non-empty string contains error description

Common errors:
- "plugin path cannot be empty" - Invalid path parameter
- "plugin not loaded" - Operation on unloaded plugin
- "symbol not found" - Symbol lookup failed
- "plugin cannot be nil" - Invalid plugin parameter

## Security Considerations

1. **Signature Verification** - Verify plugin authenticity
2. **Sandboxing** - Isolate plugin execution
3. **Resource Limits** - Memory, CPU, and time restrictions
4. **Access Control** - File and network access restrictions
5. **Version Checking** - Ensure API compatibility
6. **Package Verification** - Validate package integrity

## Implementation Notes

This is a pure CURSED implementation that provides comprehensive plugin management functionality without external dependencies. The implementation includes:

- Complete plugin lifecycle management
- Security features with sandboxing and signatures
- Flexible hook and extension systems
- Version management and compatibility checking
- Remote repository integration
- Development and distribution tools

The module focuses on:
- Plugin safety and security
- Extensible architecture
- Easy integration
- Developer-friendly APIs
- Production-ready features

For production use, this module provides a robust foundation for building extensible applications with a comprehensive plugin ecosystem.
