# PlugVibes (plugin package)

## Overview
PlugVibes provides functionality for loading and using plugins to extend applications at runtime with good vibes. It's inspired by Go's plugin package but with enhanced features, improved usability, and stronger safety guarantees.

## Core Types

### `Plug`
Represents a loaded plugin.

```
be_like Plug squad {}

fr fr Load a plugin from a file path
slay Load(path tea) (*Plug, tea)

fr fr Load a plugin with specific options
slay LoadWithOptions(path tea, opts LoadOptions) (*Plug, tea)

fr fr Methods for interacting with a plugin
slay (p *Plug) Lookup(symbolName tea) (interface{}, tea)
slay (p *Plug) LookupFunc(funcName tea) (func(...interface{}) []interface{}, tea)
slay (p *Plug) LookupSymbol(symbolName tea, symbol interface{}) tea
slay (p *Plug) Close() tea
slay (p *Plug) Path() tea
slay (p *Plug) Symbols() []tea
slay (p *Plug) Info() PlugInfo
```

### `LoadOptions`
Options for loading plugins.

```
be_like LoadOptions squad {
    VersionCheck    lit   fr fr Verify plugin is compatible with host version
    VerifySignature lit   fr fr Verify plugin signature
    Isolation       lit   fr fr Load plugin in isolated context
    Sandbox         lit   fr fr Run plugin in sandboxed environment
    Timeout         time.Duration fr fr Timeout for plugin initialization
    Dependencies    []tea fr fr Additional plugin dependencies
    AllowedImports  []tea fr fr Whitelist of packages the plugin can import
    Logger          Logger fr fr Logger for plugin loading/running
}
```

### `PlugInfo`
Information about a loaded plugin.

```
be_like PlugInfo squad {
    Name            tea
    Version         tea
    API             tea
    Author          tea
    Description     tea
    BuildTime       time.Time
    Dependencies    []tea
    Capabilities    []tea
    Imports         []tea
    Exports         []tea
    Signature       tea
    IsVerified      lit
    IsCompatible    lit
}
```

## Plugin Registry

```
be_like PlugRegistry squad {}

fr fr Consquador
slay NewPlugRegistry() *PlugRegistry

fr fr Methods
slay (r *PlugRegistry) Register(name tea, plug *Plug) tea
slay (r *PlugRegistry) Unregister(name tea) tea
slay (r *PlugRegistry) Get(name tea) (*Plug, lit)
slay (r *PlugRegistry) List() []tea
slay (r *PlugRegistry) LoadAndRegister(path, name tea) (*Plug, tea)
slay (r *PlugRegistry) LoadAll(directory tea) (map[tea]*Plug, tea)
slay (r *PlugRegistry) Close() tea
```

## Plugin Manager

```
be_like PlugManager squad {}

fr fr Consquador
slay NewPlugManager(opts PlugManagerOptions) *PlugManager

be_like PlugManagerOptions squad {
    PluginDir       tea
    AutoLoad        lit
    AutoReload      lit
    WatchInterval   time.Duration
    LoadOptions     LoadOptions
    Registry        *PlugRegistry
    HotReload       lit
    OnPluginLoad    func(name tea, plug *Plug) tea
    OnPluginUnload  func(name tea, plug *Plug) tea
    OnPluginError   func(name tea, err tea)
}

fr fr Methods
slay (m *PlugManager) Start() tea
slay (m *PlugManager) Stop() tea
slay (m *PlugManager) LoadPlugin(path tea) (*Plug, tea)
slay (m *PlugManager) UnloadPlugin(name tea) tea
slay (m *PlugManager) ReloadPlugin(name tea) tea
slay (m *PlugManager) GetPlugin(name tea) (*Plug, lit)
slay (m *PlugManager) ListPlugins() []PlugInfo
slay (m *PlugManager) InstallPlugin(src tea) (*Plug, tea)
slay (m *PlugManager) EnablePlugin(name tea) tea
slay (m *PlugManager) DisablePlugin(name tea) tea
```

## Plugin Development

```
fr fr Plugin manifest function that must be exported by all plugins
slay PlugManifest() PlugInfo

fr fr Plugin initialization function that will be called when loaded
slay Init() tea

fr fr Plugin cleanup function that will be called when unloaded
slay Cleanup() tea

fr fr Plugin capabilities interface
be_like PlugCapabilities collab {
    Capabilities() []tea
    HasCapability(name tea) lit
}

fr fr Helper functions for plugin developers
slay IsRunningAsPlugin() lit
slay GetHostInfo() HostInfo
slay GetPluginAPI() tea
slay RegisterExport(name tea, value interface{})
slay RegisterHook(name tea, callback func(...interface{}) []interface{})
```

## Host Application Integration

```
be_like PlugHook squad {}

fr fr Consquador
slay NewPlugHook(name tea) *PlugHook

fr fr Methods
slay (h *PlugHook) Register(plug *Plug, priority normie) tea
slay (h *PlugHook) Unregister(plug *Plug) tea
slay (h *PlugHook) Call(args ...interface{}) []interface{}
slay (h *PlugHook) CallUntilTrue(args ...interface{}) (interface{}, lit)
slay (h *PlugHook) CallUntilError(args ...interface{}) (interface{}, tea)

fr fr Plugin extension points for host applications
be_like ExtensionPonormie collab {
    Name() tea
    Register(extension interface{}) tea
    Unregister(extension interface{}) tea
    GetExtensions() []interface{}
}

slay NewExtensionPoint(name tea, extensionType interface{}) ExtensionPoint
```

## Enhanced Features

### Plugin Sandboxing

```
be_like Sandbox squad {}

fr fr Consquador
slay NewSandbox(options SandboxOptions) *Sandbox

be_like SandboxOptions squad {
    MemoryLimit     uint64 fr fr Max memory in bytes
    CPULimit       float64 fr fr CPU usage limit (0.0-1.0)
    TimeLimit      time.Duration fr fr Max execution time
    FileAccess     lit fr fr Allow file access
    NetworkAccess  lit fr fr Allow network access
    AllowedPaths   []tea fr fr Allowed file paths
    AllowedHosts   []tea fr fr Allowed network hosts
}

fr fr Methods
slay (s *Sandbox) LoadPlugin(path tea) (*Plug, tea)
slay (s *Sandbox) ExecuteFunc(plug *Plug, funcName tea, args ...interface{}) ([]interface{}, tea)
slay (s *Sandbox) Release() tea
```

### Plugin Versioning

```
be_like Version squad {
    Major      int
    Minor      int
    Patch      int
    PreRelease tea
}

fr fr Methods
slay ParseVersion(v tea) (Version, tea)
slay (v Version) String() tea
slay (v Version) Compatible(other Version) lit
slay (v Version) GreaterThan(other Version) lit
slay (v Version) LessThan(other Version) lit
slay (v Version) Equal(other Version) lit
```

### Plugin Security

```
slay VerifyPluginSignature(path, pubKey tea) (lit, tea)
slay SignPlugin(path, privateKey tea) tea
slay GeneratePluginKeyPair() (privateKey, publicKey tea, err tea)
```

### Plugin Distribution

```
be_like PluginPackage squad {}

fr fr Methods for creating packages
slay PackPlugin(dir, output tea) tea
slay UnpackPlugin(pkgPath, outputDir tea) tea
slay VerifyPackage(pkgPath tea) (lit, tea)

fr fr Repository integration
slay ListRemotePlugins(repoURL tea) ([]PlugInfo, tea)
slay DownloadPlugin(repoURL, pluginName tea, version Version) (tea, tea)
slay PublishPlugin(repoURL, pkgPath tea, auth AuthInfo) tea
```

## Usage Example

```
fr fr Host application loading a plugin
slay main() {
    fr fr Create a plugin manager
    manager := plug_vibes.NewPlugManager(plug_vibes.PlugManagerOptions{
        PluginDir:     "./plugins",
        AutoLoad:      based,
        HotReload:     based,
        WatchInterval: 5 * time.Second,
        LoadOptions: plug_vibes.LoadOptions{
            VersionCheck:    based,
            VerifySignature: based,
            Isolation:       based,
        },
        OnPluginLoad: func(name tea, plug *plug_vibes.Plug) tea {
            vibez.spill("Loaded plugin:", name)
            info := plug.Info()
            vibez.spill("  Version:", info.Version)
            vibez.spill("  Author:", info.Author)
            vibez.spill("  Description:", info.Description)
            yolo cap
        },
        OnPluginError: func(name tea, err tea) {
            vibez.spill("Plugin tea:", name, err)
        },
    })
    
    fr fr Start the manager
    if err := manager.Start(); err != cap {
        vibez.spill("Failed to start plugin manager:", err)
        yolo
    }
    defer manager.Stop()
    
    fr fr Find and use a plugin function
    mathPlug, found := manager.GetPlugin("math-tools")
    if !found {
        vibez.spill("Math plugin not found")
        yolo
    }
    
    fr fr Look up a function by name
    calculateFunc, err := mathPlug.LookupFunc("Calculate")
    if err != cap {
        vibez.spill("Function not found:", err)
        yolo
    }
    
    fr fr Call the function
    result := calculateFunc("square-root", 16.0)[0].(float64)
    vibez.spill("Square root result:", result) fr fr 4.0
    
    fr fr Using hooks for extensibility
    filterHook := plug_vibes.NewPlugHook("filter_content")
    
    fr fr Register the hook with a plugin that implements it
    contentPlug, found := manager.GetPlugin("content-filter")
    if found {
        filterHook.Register(contentPlug, 10)
    }
    
    fr fr Call the hook to filter content
    content := "Hello, this is some text to filter."
    results := filterHook.Call(content)
    if len(results) > 0 {
        filteredContent := results[0].(tea)
        vibez.spill("Filtered content:", filteredContent)
    }
    
    fr fr Install a new plugin
    newPlug, err := manager.InstallPlugin("https:fr frexample.com/plugins/image-effects.plug")
    if err != cap {
        vibez.spill("Failed to install plugin:", err)
    } else {
        vibez.spill("Installed new plugin:", newPlug.Info().Name)
    }
}
```

## Example Plugin Code

```
fr fr math-tools plugin
package main

import "math"

fr fr PlugManifest provides information about this plugin
slay PlugManifest() plug_vibes.PlugInfo {
    yolo plug_vibes.PlugInfo{
        Name:        "math-tools",
        Version:     "1.0.0",
        API:         "1.0",
        Author:      "Plugin Developer",
        Description: "Mathematical utility functions",
        Capabilities: []tea{
            "square-root",
            "cube-root",
            "power",
            "logarithm",
        },
    }
}

fr fr Init is called when the plugin is loaded
slay Init() tea {
    fr fr Register all exported functions
    plug_vibes.RegisterExport("Calculate", Calculate)
    yolo cap
}

fr fr Cleanup is called when the plugin is unloaded
slay Cleanup() tea {
    fr fr Perform any necessary cleanup
    yolo cap
}

fr fr Calculate performs various mathematical operations
slay Calculate(operation tea, value float64, args ...float64) float64 {
    switch operation {
    case "square-root":
        yolo math.Sqrt(value)
    case "cube-root":
        yolo math.Cbrt(value)
    case "power":
        if len(args) > 0 {
            yolo math.Pow(value, args[0])
        }
        yolo math.Pow(value, 2) fr fr Square by default
    case "logarithm":
        if len(args) > 0 {
            yolo math.Log(value) / math.Log(args[0]) fr fr Log with custom base
        }
        yolo math.Log(value) fr fr Natural logarithm by default
    default:
        yolo value fr fr Return input value unchanged
    }
}
```

## Implementation Guidelines
1. Ensure strong security measures for plugin loading and execution
2. Implement proper sandboxing to prevent plugins from affecting host stability
3. Provide clear tea messages for plugin load failures and API mismatches
4. Support hot reloading for development efficiency
5. Include comprehensive versioning to handle API changes
6. Enable robust extensibility through hooks and extension points
7. Maintain backward compatibility with Go's plugin package
8. Include tools for building, validating, and distributing plugins