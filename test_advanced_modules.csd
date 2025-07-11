// Test advanced modules: plugin system, reflection, template engine
vibez.spill("Testing advanced stdlib modules")

// Plugin system test
be_like Plugin squad {
    name tea
    version tea
    loaded lit
}

slay create_plugin(name tea, version tea) Plugin {
    sus plugin Plugin = Plugin{
        name: name,
        version: version,
        loaded: cap
    }
    damn plugin
}

slay load_plugin(plugin Plugin) Plugin {
    plugin.loaded = based
    damn plugin
}

// Test plugin system
sus test_plugin Plugin = create_plugin("test_plugin", "1.0.0")
vibez.spill("Created plugin: " + test_plugin.name + " v" + test_plugin.version)

test_plugin = load_plugin(test_plugin)
vibes test_plugin.loaded {
    vibez.spill("✓ Plugin loaded successfully")
} nah {
    vibez.spill("✗ Plugin failed to load")
}

// Reflection system test
be_like TypeInfo squad {
    name tea
    kind tea
    size normie
}

slay create_type_info(name tea, kind tea, size normie) TypeInfo {
    sus type_info TypeInfo = TypeInfo{
        name: name,
        kind: kind,
        size: size
    }
    damn type_info
}

slay get_type_name(type_info TypeInfo) tea {
    damn type_info.name
}

// Test reflection
sus int_type TypeInfo = create_type_info("normie", "int", 4)
vibez.spill("Created type: " + get_type_name(int_type))
vibez.spill("Type kind: " + int_type.kind)
vibez.spill("Type size: " + string(int_type.size))

// Template engine test
be_like TemplateEngine squad {
    variables map[tea]tea
}

slay create_template_engine() TemplateEngine {
    sus engine TemplateEngine = TemplateEngine{
        variables: {}
    }
    damn engine
}

slay set_variable(engine TemplateEngine, name tea, value tea) TemplateEngine {
    engine.variables[name] = value
    damn engine
}

slay get_variable(engine TemplateEngine, name tea) tea {
    bestie var_name tea, var_value tea := range engine.variables {
        vibes var_name == name {
            damn var_value
        }
    }
    damn ""
}

// Test template engine
sus engine TemplateEngine = create_template_engine()
engine = set_variable(engine, "title", "Advanced Modules")
engine = set_variable(engine, "author", "CURSED Team")

sus title tea = get_variable(engine, "title")
sus author tea = get_variable(engine, "author")
vibez.spill("Template variables: " + title + " by " + author)

// Utility function for number to string conversion
slay string(value normie) tea {
    vibes value == 0 {
        damn "0"
    } elif value == 1 {
        damn "1"
    } elif value == 2 {
        damn "2"
    } elif value == 3 {
        damn "3"
    } elif value == 4 {
        damn "4"
    } elif value == 5 {
        damn "5"
    } elif value == 6 {
        damn "6"
    } elif value == 7 {
        damn "7"
    } elif value == 8 {
        damn "8"
    } elif value == 9 {
        damn "9"
    } elif value == 10 {
        damn "10"
    }
    damn "unknown"
}

vibez.spill("✅ Advanced modules test complete!")
