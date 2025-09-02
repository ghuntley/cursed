yeet "testz"
yeet "plug_vibes"

test_start("plug_vibes basic plugin loading")

fr fr Test basic plugin loading
sus plug, err := plug_vibes.Load("test_plugin.csd")
assert_eq_string(err, "")
assert_true(plug != cringe)
assert_true(plug.Info().Name != "")
assert_eq_string(plug.Info().Version, "1.0.0")
assert_eq_string(plug.Info().API, "1.0")

fr fr Test plugin info
sus info := plug.Info()
assert_eq_string(info.Name, "test_plugin")
assert_eq_string(info.Author, "Unknown")
assert_eq_string(info.Description, "A CURSED plugin")
assert_true(info.IsVerified)
assert_true(info.IsCompatible)

fr fr Test plugin path and symbols
assert_eq_string(plug.Path(), "test_plugin.csd")
sus symbols := plug.Symbols()
assert_true(len(symbols) > 0)

test_start("plug_vibes plugin loading with options")

fr fr Test loading with custom options
sus opts := plug_vibes.LoadOptions{
    VersionCheck: based,
    VerifySignature: based,
    Isolation: based,
    Sandbox: cap,
    Timeout: 60,
    Dependencies: tea[value]{"math", "string"},
    AllowedImports: tea[value]{"stdlib"},
    LogLevel: 2
}

sus plugWithOpts, optsErr := plug_vibes.LoadWithOptions("advanced_plugin.csd", opts)
assert_eq_string(optsErr, "")
assert_true(plugWithOpts != cringe)

fr fr Test empty path error
sus emptyPlug, emptyErr := plug_vibes.Load("")
assert_true(emptyErr != "")
assert_true(emptyPlug == cringe)

test_start("plug_vibes symbol lookup")

fr fr Test symbol lookup
sus manifestSymbol, manifestErr := plug.Lookup("PlugManifest")
assert_eq_string(manifestErr, "")
assert_true(manifestSymbol != cringe)

sus initSymbol, initErr := plug.Lookup("Init")
assert_eq_string(initErr, "")
assert_true(initSymbol != cringe)

sus calcSymbol, calcErr := plug.Lookup("Calculate")
assert_eq_string(calcErr, "")
assert_true(calcSymbol != cringe)

fr fr Test non-existent symbol
sus missing, missingErr := plug.Lookup("NonExistent")
assert_true(missingErr != "")
assert_true(missing == cringe)

test_start("plug_vibes function lookup")

fr fr Test function lookup
sus calcFunc, funcErr := plug.LookupFunc("Calculate")
assert_eq_string(funcErr, "")
assert_true(calcFunc != cringe)

fr fr Test non-existent function
sus missingFunc, missingFuncErr := plug.LookupFunc("MissingFunction")
assert_true(missingFuncErr != "")
assert_true(missingFunc == cringe)

test_start("plug_vibes plugin registry")

fr fr Test plugin registry
sus registry := plug_vibes.NewPlugRegistry()
assert_true(registry != cringe)

fr fr Test plugin registration
sus regErr := registry.Register("test-plugin", plug)
assert_eq_string(regErr, "")

fr fr Test plugin retrieval
sus retrievedPlug, found := registry.Get("test-plugin")
assert_true(found)
assert_true(retrievedPlug == plug)

fr fr Test plugin listing
sus pluginList := registry.List()
assert_eq_int(len(pluginList), 1)
assert_eq_string(pluginList[0], "test-plugin")

fr fr Test load and register
sus newPlug, loadRegErr := registry.LoadAndRegister("another_plugin.csd", "another-plugin")
assert_eq_string(loadRegErr, "")
assert_true(newPlug != cringe)

sus updatedList := registry.List()
assert_eq_int(len(updatedList), 2)

test_start("plug_vibes plugin unregistration")

fr fr Test plugin unregistration
sus unregErr := registry.Unregister("test-plugin")
assert_eq_string(unregErr, "")

sus afterUnreg, stillFound := registry.Get("test-plugin")
assert_false(stillFound)
assert_true(afterUnreg == cringe)

fr fr Test unregistering non-existent plugin
sus notFoundErr := registry.Unregister("non-existent")
assert_true(notFoundErr != "")

test_start("plug_vibes plugin manager")

fr fr Test plugin manager creation
sus managerOpts := plug_vibes.PlugManagerOptions{
    PluginDir: "./plugins",
    AutoLoad: based,
    AutoReload: cap,
    WatchInterval: 5,
    Registry: plug_vibes.NewPlugRegistry(),
    HotReload: based
}

sus manager := plug_vibes.NewPlugManager(managerOpts)
assert_true(manager != cringe)

fr fr Test manager start and stop
sus startErr := manager.Start()
assert_eq_string(startErr, "")

sus stopErr := manager.Stop()
assert_eq_string(stopErr, "")

test_start("plug_vibes manager plugin operations")

fr fr Create new manager for testing
sus testManager := plug_vibes.NewPlugManager(plug_vibes.PlugManagerOptions{
    PluginDir: "./test_plugins",
    Registry: plug_vibes.NewPlugRegistry()
})

fr fr Test loading plugin through manager
sus loadedPlug, loadErr := testManager.LoadPlugin("manager_test.csd")
assert_eq_string(loadErr, "")
assert_true(loadedPlug != cringe)

fr fr Test getting plugin from manager
sus managerPlug, managerFound := testManager.GetPlugin("manager_test")
assert_true(managerFound)
assert_true(managerPlug != cringe)

fr fr Test listing plugins through manager
sus managerPlugins := testManager.ListPlugins()
assert_eq_int(len(managerPlugins), 1)
assert_eq_string(managerPlugins[0].Name, "manager_test")

fr fr Test plugin enable/disable
sus enableErr := testManager.EnablePlugin("manager_test")
assert_eq_string(enableErr, "")

sus disableErr := testManager.DisablePlugin("manager_test")
assert_eq_string(disableErr, "")

fr fr Test unloading plugin
sus unloadErr := testManager.UnloadPlugin("manager_test")
assert_eq_string(unloadErr, "")

sus afterUnload, stillExists := testManager.GetPlugin("manager_test")
assert_false(stillExists)
assert_true(afterUnload == cringe)

test_start("plug_vibes hook system")

fr fr Test plugin hook creation
sus hook := plug_vibes.NewPlugHook("filter_content")
assert_true(hook != cringe)
assert_eq_string(hook.name, "filter_content")

fr fr Create test plugin for hook
sus hookPlug, hookErr := plug_vibes.Load("hook_test.csd")
assert_eq_string(hookErr, "")

fr fr Test hook registration
sus hookRegErr := hook.Register(hookPlug, 10)
assert_eq_string(hookRegErr, "")

fr fr Test hook calling
sus results := hook.Call("test content")
assert_true(len(results) >= 0)

fr fr Test hook call until true
sus result, success := hook.CallUntilTrue("test")
if success {
    assert_true(result != cringe)
}

fr fr Test hook unregistration
sus hookUnregErr := hook.Unregister(hookPlug)
assert_eq_string(hookUnregErr, "")

test_start("plug_vibes extension points")

fr fr Test extension point creation
sus extPoint := plug_vibes.NewExtensionPoint("image_filter", "FilterInterface")
assert_eq_string(extPoint.Name(), "image_filter")

fr fr Test extension registration
sus filter := "dummy_filter"
sus extRegErr := extPoint.Register(filter)
assert_eq_string(extRegErr, "")

sus extensions := extPoint.GetExtensions()
assert_eq_int(len(extensions), 1)

fr fr Test extension unregistration
sus extUnregErr := extPoint.Unregister(filter)
assert_eq_string(extUnregErr, "")

sus afterUnreg := extPoint.GetExtensions()
assert_eq_int(len(afterUnreg), 0)

test_start("plug_vibes sandboxing")

fr fr Test sandbox creation
sus sandboxOpts := plug_vibes.SandboxOptions{
    MemoryLimit: 1024000,
    CPULimit: 0.5,
    TimeLimit: 30,
    FileAccess: cap,
    NetworkAccess: cap,
    AllowedPaths: tea[value]{"/tmp", "/var/tmp"},
    AllowedHosts: tea[value]{"localhost", "127.0.0.1"}
}

sus sandbox := plug_vibes.NewSandbox(sandboxOpts)
assert_true(sandbox != cringe)
assert_eq_int(sandbox.memoryLimit, 1024000)
assert_eq_int(sandbox.timeLimit, 30)
assert_false(sandbox.fileAccess)
assert_false(sandbox.networkAccess)

fr fr Test loading plugin in sandbox
sus sandboxPlug, sandboxErr := sandbox.LoadPlugin("sandbox_test.csd")
assert_eq_string(sandboxErr, "")
assert_true(sandboxPlug != cringe)

fr fr Test executing function in sandbox
sus execResult, execErr := sandbox.ExecuteFunc(sandboxPlug, "TestFunction", "test_args")
assert_eq_string(execErr, "")
assert_true(execResult != cringe)

fr fr Test sandbox release
sus releaseErr := sandbox.Release()
assert_eq_string(releaseErr, "")

test_start("plug_vibes version management")

fr fr Test version parsing
sus version1, v1Err := plug_vibes.ParseVersion("1.0.0")
assert_eq_string(v1Err, "")
assert_eq_int(version1.Major, 1)
assert_eq_int(version1.Minor, 0)
assert_eq_int(version1.Patch, 0)

sus version2, v2Err := plug_vibes.ParseVersion("2.1.3")
assert_eq_string(v2Err, "")
assert_eq_int(version2.Major, 2)
assert_eq_int(version2.Minor, 1)
assert_eq_int(version2.Patch, 3)

fr fr Test version string representation
assert_eq_string(version1.String(), "1.0.0")
assert_eq_string(version2.String(), "2.1.3")

fr fr Test version comparisons
assert_true(version1.Compatible(version1))
assert_false(version1.Compatible(version2))

assert_false(version1.GreaterThan(version2))
assert_true(version2.GreaterThan(version1))

assert_true(version1.LessThan(version2))
assert_false(version2.LessThan(version1))

assert_true(version1.Equal(version1))
assert_false(version1.Equal(version2))

test_start("plug_vibes security functions")

fr fr Test plugin signature verification
sus verified, verifyErr := plug_vibes.VerifyPluginSignature("test.csd", "public_key")
assert_eq_string(verifyErr, "")
assert_true(verified)

fr fr Test plugin signing
sus signErr := plug_vibes.SignPlugin("test.csd", "private_key")
assert_eq_string(signErr, "")

fr fr Test key pair generation
sus privateKey, publicKey, keyErr := plug_vibes.GeneratePluginKeyPair()
assert_eq_string(keyErr, "")
assert_eq_string(privateKey, "private_key")
assert_eq_string(publicKey, "public_key")

test_start("plug_vibes distribution functions")

fr fr Test plugin packaging
sus packErr := plug_vibes.PackPlugin("./my_plugin", "my_plugin.plug")
assert_eq_string(packErr, "")

fr fr Test plugin unpacking
sus unpackErr := plug_vibes.UnpackPlugin("my_plugin.plug", "./extracted")
assert_eq_string(unpackErr, "")

fr fr Test package verification
sus packageOK, packageErr := plug_vibes.VerifyPackage("my_plugin.plug")
assert_eq_string(packageErr, "")
assert_true(packageOK)

test_start("plug_vibes remote operations")

fr fr Test listing remote plugins
sus remotePlugins, remoteErr := plug_vibes.ListRemotePlugins("https://plugins.example.com")
assert_eq_string(remoteErr, "")
assert_eq_int(len(remotePlugins), 2)
assert_eq_string(remotePlugins[0].Name, "remote-plugin-1")
assert_eq_string(remotePlugins[1].Name, "remote-plugin-2")

fr fr Test downloading plugin
sus downloadPath, downloadErr := plug_vibes.DownloadPlugin("https://plugins.example.com", "math-utils", version1)
assert_eq_string(downloadErr, "")
assert_true(len(downloadPath) > 0)

fr fr Test publishing plugin
sus auth := plug_vibes.AuthInfo{
    Username: "developer",
    Token: "auth_token",
    APIKey: "api_key"
}
sus publishErr := plug_vibes.PublishPlugin("https://plugins.example.com", "my_plugin.plug", auth)
assert_eq_string(publishErr, "")

test_start("plug_vibes development functions")

fr fr Test plugin development utilities
assert_false(plug_vibes.IsRunningAsPlugin())

sus hostInfo := plug_vibes.GetHostInfo()
assert_eq_string(hostInfo.Version, "1.0.0")
assert_eq_string(hostInfo.Platform, "CURSED")
assert_eq_int(len(hostInfo.Features), 3)

sus apiVersion := plug_vibes.GetPluginAPI()
assert_eq_string(apiVersion, "1.0")

fr fr Test export registration
plug_vibes.RegisterExport("MyFunction", "function_implementation")

fr fr Test hook registration
plug_vibes.RegisterHook("my_hook", slay(args interface{}) interface{} {
    damn "hook_result"
})

test_start("plug_vibes plugin closing")

fr fr Test plugin closing
sus closeErr := plug.Close()
assert_eq_string(closeErr, "")
assert_false(plug.loaded)

fr fr Test operations on closed plugin
sus closedSymbol, closedErr := plug.Lookup("Calculate")
assert_true(closedErr != "")
assert_true(closedSymbol == cringe)

print_test_summary()
