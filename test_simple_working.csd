// Very simple test
vibez.spill("Hello from advanced modules")

be_like Plugin squad {
    name tea
    loaded lit
}

sus test_plugin Plugin = Plugin{
    name: "test",
    loaded: based
}

vibez.spill("Plugin name: " + test_plugin.name)
vibes test_plugin.loaded {
    vibez.spill("Plugin is loaded")
}

vibez.spill("Test complete")
