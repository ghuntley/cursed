// Simple template engine test
vibez.spill("Testing template engine")

// Template context structure
be_like TemplateContext squad {
    variables map[tea]tea
}

// Template engine structure
be_like TemplateEngine squad {
    context TemplateContext
    escape_html lit
}

// Create template engine
slay create_template_engine() TemplateEngine {
    sus context TemplateContext = TemplateContext{
        variables: {}
    }
    sus engine TemplateEngine = TemplateEngine{
        context: context,
        escape_html: based
    }
    damn engine
}

// Set template variable
slay set_variable(engine TemplateEngine, name tea, value tea) TemplateEngine {
    engine.context.variables[name] = value
    damn engine
}

// Get template variable
slay get_variable(engine TemplateEngine, name tea) tea {
    bestie var_name tea, var_value tea := range engine.context.variables {
        vibes var_name == name {
            damn var_value
        }
    }
    damn ""
}

// Test template engine creation
sus engine TemplateEngine = create_template_engine()
vibez.spill("✓ Created template engine")

// Test variable setting
engine = set_variable(engine, "name", "CURSED")
engine = set_variable(engine, "version", "1.0")
vibez.spill("✓ Set template variables")

// Test variable retrieval
sus name_value tea = get_variable(engine, "name")
sus version_value tea = get_variable(engine, "version")
vibez.spill("✓ Retrieved variables: " + name_value + " v" + version_value)

// Test missing variable
sus missing_value tea = get_variable(engine, "missing")
vibes missing_value == "" {
    vibez.spill("✓ Missing variable returns empty string")
} nah {
    vibez.spill("✗ Missing variable should return empty string")
}

vibez.spill("✅ Template engine basic functionality working!")
