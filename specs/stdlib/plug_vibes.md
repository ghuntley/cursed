# PlugVibes (plugin package)

## Overview
PlugVibes provides functionality for loading and using plugins to extend applications at runtime with good vibes. It's inspired by Go's plugin package but with enhanced features, improved usability, and stronger safety guarantees.

## Core Types

### `Plug`
Represents a loaded plugin.

```go
type Plug struct {}

// Load a plugin from a file path
func Load(path string) (*Plug, error)

// Load a plugin with specific options
func LoadWithOptions(path string, opts LoadOptions) (*Plug, error)

// Methods for interacting with a plugin
func (p *Plug) Lookup(symbolName string) (interface{}, error)
func (p *Plug) LookupFunc(funcName string) (func(...interface{}) []interface{}, error)
func (p *Plug) LookupSymbol(symbolName string, symbol interface{}) error
func (p *Plug) Close() error
func (p *Plug) Path() string
func (p *Plug) Symbols() []string
func (p *Plug) Info() PlugInfo
```

### `LoadOptions`
Options for loading plugins.

```go
type LoadOptions struct {
    VersionCheck    bool   // Verify plugin is compatible with host version
    VerifySignature bool   // Verify plugin signature
    Isolation       bool   // Load plugin in isolated context
    Sandbox         bool   // Run plugin in sandboxed environment
    Timeout         time.Duration // Timeout for plugin initialization
    Dependencies    []string // Additional plugin dependencies
    AllowedImports  []string // Whitelist of packages the plugin can import
    Logger          Logger // Logger for plugin loading/running
}
```

### `PlugInfo`
Information about a loaded plugin.

```go
type PlugInfo struct {
    Name            string
    Version         string
    API             string
    Author          string
    Description     string
    BuildTime       time.Time
    Dependencies    []string
    Capabilities    []string
    Imports         []string
    Exports         []string
    Signature       string
    IsVerified      bool
    IsCompatible    bool
}
```

## Plugin Registry

```go
type PlugRegistry struct {}

// Constructor
func NewPlugRegistry() *PlugRegistry

// Methods
func (r *PlugRegistry) Register(name string, plug *Plug) error
func (r *PlugRegistry) Unregister(name string) error
func (r *PlugRegistry) Get(name string) (*Plug, bool)
func (r *PlugRegistry) List() []string
func (r *PlugRegistry) LoadAndRegister(path, name string) (*Plug, error)
func (r *PlugRegistry) LoadAll(directory string) (map[string]*Plug, error)
func (r *PlugRegistry) Close() error
```

## Plugin Manager

```go
type PlugManager struct {}

// Constructor
func NewPlugManager(opts PlugManagerOptions) *PlugManager

type PlugManagerOptions struct {
    PluginDir       string
    AutoLoad        bool
    AutoReload      bool
    WatchInterval   time.Duration
    LoadOptions     LoadOptions
    Registry        *PlugRegistry
    HotReload       bool
    OnPluginLoad    func(name string, plug *Plug) error
    OnPluginUnload  func(name string, plug *Plug) error
    OnPluginError   func(name string, err error)
}

// Methods
func (m *PlugManager) Start() error
func (m *PlugManager) Stop() error
func (m *PlugManager) LoadPlugin(path string) (*Plug, error)
func (m *PlugManager) UnloadPlugin(name string) error
func (m *PlugManager) ReloadPlugin(name string) error
func (m *PlugManager) GetPlugin(name string) (*Plug, bool)
func (m *PlugManager) ListPlugins() []PlugInfo
func (m *PlugManager) InstallPlugin(src string) (*Plug, error)
func (m *PlugManager) EnablePlugin(name string) error
func (m *PlugManager) DisablePlugin(name string) error
```

## Plugin Development

```go
// Plugin manifest function that must be exported by all plugins
func PlugManifest() PlugInfo

// Plugin initialization function that will be called when loaded
func Init() error

// Plugin cleanup function that will be called when unloaded
func Cleanup() error

// Plugin capabilities interface
type PlugCapabilities interface {
    Capabilities() []string
    HasCapability(name string) bool
}

// Helper functions for plugin developers
func IsRunningAsPlugin() bool
func GetHostInfo() HostInfo
func GetPluginAPI() string
func RegisterExport(name string, value interface{})
func RegisterHook(name string, callback func(...interface{}) []interface{})
```

## Host Application Integration

```go
type PlugHook struct {}

// Constructor
func NewPlugHook(name string) *PlugHook

// Methods
func (h *PlugHook) Register(plug *Plug, priority int) error
func (h *PlugHook) Unregister(plug *Plug) error
func (h *PlugHook) Call(args ...interface{}) []interface{}
func (h *PlugHook) CallUntilTrue(args ...interface{}) (interface{}, bool)
func (h *PlugHook) CallUntilError(args ...interface{}) (interface{}, error)

// Plugin extension points for host applications
type ExtensionPoint interface {
    Name() string
    Register(extension interface{}) error
    Unregister(extension interface{}) error
    GetExtensions() []interface{}
}

func NewExtensionPoint(name string, extensionType interface{}) ExtensionPoint
```

## Enhanced Features

### Plugin Sandboxing

```go
type Sandbox struct {}

// Constructor
func NewSandbox(options SandboxOptions) *Sandbox

type SandboxOptions struct {
    MemoryLimit     uint64 // Max memory in bytes
    CPULimit       float64 // CPU usage limit (0.0-1.0)
    TimeLimit      time.Duration // Max execution time
    FileAccess     bool // Allow file access
    NetworkAccess  bool // Allow network access
    AllowedPaths   []string // Allowed file paths
    AllowedHosts   []string // Allowed network hosts
}

// Methods
func (s *Sandbox) LoadPlugin(path string) (*Plug, error)
func (s *Sandbox) ExecuteFunc(plug *Plug, funcName string, args ...interface{}) ([]interface{}, error)
func (s *Sandbox) Release() error
```

### Plugin Versioning

```go
type Version struct {
    Major      int
    Minor      int
    Patch      int
    PreRelease string
}

// Methods
func ParseVersion(v string) (Version, error)
func (v Version) String() string
func (v Version) Compatible(other Version) bool
func (v Version) GreaterThan(other Version) bool
func (v Version) LessThan(other Version) bool
func (v Version) Equal(other Version) bool
```

### Plugin Security

```go
func VerifyPluginSignature(path, pubKey string) (bool, error)
func SignPlugin(path, privateKey string) error
func GeneratePluginKeyPair() (privateKey, publicKey string, err error)
```

### Plugin Distribution

```go
type PluginPackage struct {}

// Methods for creating packages
func PackPlugin(dir, output string) error
func UnpackPlugin(pkgPath, outputDir string) error
func VerifyPackage(pkgPath string) (bool, error)

// Repository integration
func ListRemotePlugins(repoURL string) ([]PlugInfo, error)
func DownloadPlugin(repoURL, pluginName string, version Version) (string, error)
func PublishPlugin(repoURL, pkgPath string, auth AuthInfo) error
```

## Usage Example

```go
// Host application loading a plugin
func main() {
    // Create a plugin manager
    manager := plug_vibes.NewPlugManager(plug_vibes.PlugManagerOptions{
        PluginDir:     "./plugins",
        AutoLoad:      true,
        HotReload:     true,
        WatchInterval: 5 * time.Second,
        LoadOptions: plug_vibes.LoadOptions{
            VersionCheck:    true,
            VerifySignature: true,
            Isolation:       true,
        },
        OnPluginLoad: func(name string, plug *plug_vibes.Plug) error {
            vibez.spill("Loaded plugin:", name)
            info := plug.Info()
            vibez.spill("  Version:", info.Version)
            vibez.spill("  Author:", info.Author)
            vibez.spill("  Description:", info.Description)
            return nil
        },
        OnPluginError: func(name string, err error) {
            vibez.spill("Plugin error:", name, err)
        },
    })
    
    // Start the manager
    if err := manager.Start(); err != nil {
        vibez.spill("Failed to start plugin manager:", err)
        return
    }
    defer manager.Stop()
    
    // Find and use a plugin function
    mathPlug, found := manager.GetPlugin("math-tools")
    if !found {
        vibez.spill("Math plugin not found")
        return
    }
    
    // Look up a function by name
    calculateFunc, err := mathPlug.LookupFunc("Calculate")
    if err != nil {
        vibez.spill("Function not found:", err)
        return
    }
    
    // Call the function
    result := calculateFunc("square-root", 16.0)[0].(float64)
    vibez.spill("Square root result:", result) // 4.0
    
    // Using hooks for extensibility
    filterHook := plug_vibes.NewPlugHook("filter_content")
    
    // Register the hook with a plugin that implements it
    contentPlug, found := manager.GetPlugin("content-filter")
    if found {
        filterHook.Register(contentPlug, 10)
    }
    
    // Call the hook to filter content
    content := "Hello, this is some text to filter."
    results := filterHook.Call(content)
    if len(results) > 0 {
        filteredContent := results[0].(string)
        vibez.spill("Filtered content:", filteredContent)
    }
    
    // Install a new plugin
    newPlug, err := manager.InstallPlugin("https://example.com/plugins/image-effects.plug")
    if err != nil {
        vibez.spill("Failed to install plugin:", err)
    } else {
        vibez.spill("Installed new plugin:", newPlug.Info().Name)
    }
}
```

## Example Plugin Code

```go
// math-tools plugin
package main

import "math"

// PlugManifest provides information about this plugin
func PlugManifest() plug_vibes.PlugInfo {
    return plug_vibes.PlugInfo{
        Name:        "math-tools",
        Version:     "1.0.0",
        API:         "1.0",
        Author:      "Plugin Developer",
        Description: "Mathematical utility functions",
        Capabilities: []string{
            "square-root",
            "cube-root",
            "power",
            "logarithm",
        },
    }
}

// Init is called when the plugin is loaded
func Init() error {
    // Register all exported functions
    plug_vibes.RegisterExport("Calculate", Calculate)
    return nil
}

// Cleanup is called when the plugin is unloaded
func Cleanup() error {
    // Perform any necessary cleanup
    return nil
}

// Calculate performs various mathematical operations
func Calculate(operation string, value float64, args ...float64) float64 {
    switch operation {
    case "square-root":
        return math.Sqrt(value)
    case "cube-root":
        return math.Cbrt(value)
    case "power":
        if len(args) > 0 {
            return math.Pow(value, args[0])
        }
        return math.Pow(value, 2) // Square by default
    case "logarithm":
        if len(args) > 0 {
            return math.Log(value) / math.Log(args[0]) // Log with custom base
        }
        return math.Log(value) // Natural logarithm by default
    default:
        return value // Return input value unchanged
    }
}
```

## Implementation Guidelines
1. Ensure strong security measures for plugin loading and execution
2. Implement proper sandboxing to prevent plugins from affecting host stability
3. Provide clear error messages for plugin load failures and API mismatches
4. Support hot reloading for development efficiency
5. Include comprehensive versioning to handle API changes
6. Enable robust extensibility through hooks and extension points
7. Maintain backward compatibility with Go's plugin package
8. Include tools for building, validating, and distributing plugins