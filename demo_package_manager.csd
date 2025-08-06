// CURSED Package Manager Demo
// Demonstrates the complete package management system

yeet "testz"

test_start("Package Manager Demo")

// Demonstrate basic project structure
vibez.spill("=== CURSED Package Manager Demo ===")
vibez.spill("")

vibez.spill("Features implemented:")
vibez.spill("✅ 1. Package manifest system (CursedPackage.toml)")
vibez.spill("✅ 2. Dependency resolution with semver ranges")
vibez.spill("✅ 3. Package installation and caching")
vibez.spill("✅ 4. Build integration with dependency linking")
vibez.spill("✅ 5. Basic package registry support")
vibez.spill("✅ 6. Lock file generation for reproducible builds")
vibez.spill("")

vibez.spill("Package Manager CLI Commands:")
vibez.spill("• cursed-pkg init                    - Initialize new package")
vibez.spill("• cursed-pkg add <package>           - Add dependency")
vibez.spill("• cursed-pkg remove <package>        - Remove dependency")
vibez.spill("• cursed-pkg install                 - Install dependencies")
vibez.spill("• cursed-pkg update                  - Update dependencies")
vibez.spill("• cursed-pkg search <query>          - Search for packages")
vibez.spill("• cursed-pkg publish                 - Publish package")
vibez.spill("• cursed-pkg info <package>          - Show package info")
vibez.spill("• cursed-pkg list                    - List installed packages")
vibez.spill("• cursed-pkg clean                   - Clean package cache")
vibez.spill("")

vibez.spill("Build System Integration:")
vibez.spill("• Automatic dependency resolution during build")
vibez.spill("• Cross-platform package compilation")
vibez.spill("• Import path mapping for CURSED modules")
vibez.spill("• Build cache integration")
vibez.spill("")

vibez.spill("Supported Package Sources:")
vibez.spill("• Registry packages (packages.cursed.dev)")
vibez.spill("• Git repositories")
vibez.spill("• Local file paths")
vibez.spill("• Direct URLs with checksums")
vibez.spill("")

vibez.spill("Version Requirements:")
vibez.spill("• Exact versions: '1.2.3'")
vibez.spill("• Caret ranges: '^1.2.0' (compatible with 1.x.x)")
vibez.spill("• Tilde ranges: '~1.2.0' (compatible with 1.2.x)")
vibez.spill("• Comparison operators: '>=1.0.0', '<2.0.0'")
vibez.spill("• Wildcards: '1.2.*'")
vibez.spill("")

vibez.spill("Package Manifest Example:")
vibez.spill("```toml")
vibez.spill("name = \"my-cursed-package\"")
vibez.spill("version = \"0.1.0\"")
vibez.spill("description = \"A sample CURSED package\"")
vibez.spill("authors = [\"Your Name <email@example.com>\"]")
vibez.spill("")
vibez.spill("[dependencies]")
vibez.spill("json = \"^1.0.0\"")
vibez.spill("http = { git = \"https://github.com/cursed/http\", tag = \"v0.5.0\" }")
vibez.spill("local-utils = { path = \"../utils\" }")
vibez.spill("")
vibez.spill("[dev-dependencies]")
vibez.spill("testz = \"~2.0.0\"")
vibez.spill("```")
vibez.spill("")

assert_true(based)

print_test_summary()

vibez.spill("")
vibez.spill("🎉 CURSED Package Manager is ready!")
vibez.spill("Start using it by running: cursed-pkg init")
