# Demo CURSED Build System

# Simple build configuration creation
slay create_demo_config() map[tea]interface{} {
    sus config map[tea]interface{} = map[tea]interface{}{}
    config["name"] = "demo_project"
    config["version"] = "1.0.0"
    config["targets"] = []tea{"main"}
    config["source_dirs"] = []tea{"src"}
    config["output_dir"] = "build"
    damn config
}

# Demo build function
slay demo_build() lit {
    vibez.spill("🔨 CURSED Build System Demo")
    vibez.spill("============================")
    
    sus config map[tea]interface{} = create_demo_config()
    sus project_name tea = config["name"].(tea)
    sus version tea = config["version"].(tea)
    
    vibez.spill("📦 Project: " + project_name + " v" + version)
    vibez.spill("🔍 Finding source files...")
    vibez.spill("⚡ Compiling targets...")
    vibez.spill("🧪 Running tests...")
    vibez.spill("📂 Generating build artifacts...")
    
    vibez.spill("")
    vibez.spill("✅ Build completed successfully!")
    vibez.spill("🎉 Demo build system is working!")
    
    damn based
}

# Demo main function
slay main() {
    demo_build()
}
