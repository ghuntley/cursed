# Path Operations (path)

The `path` module provides cross-platform file system path manipulation and utilities for CURSED programs.

## Purpose

This module implements comprehensive path operations including joining, splitting, normalization, and file system queries with cross-platform compatibility.

## Main Functions

### Path Construction
- `path.join(parts...)` - Join path components
- `path.normalize(path)` - Normalize path (resolve . and ..)
- `path.absolute(path)` - Convert to absolute path
- `path.relative(from, to)` - Get relative path between two paths
- `path.resolve(path)` - Resolve path to canonical form

### Path Analysis
- `path.basename(path)` - Get filename with extension
- `path.dirname(path)` - Get directory name
- `path.extension(path)` - Get file extension
- `path.stem(path)` - Get filename without extension
- `path.split(path)` - Split into directory and filename
- `path.parts(path)` - Split into all path components

### Path Properties
- `path.exists(path)` - Check if path exists
- `path.is_file(path)` - Check if path is a file
- `path.is_dir(path)` - Check if path is a directory
- `path.is_symlink(path)` - Check if path is a symbolic link
- `path.is_absolute(path)` - Check if path is absolute
- `path.is_relative(path)` - Check if path is relative

### Platform Utilities
- `path.separator()` - Get platform path separator
- `path.to_platform(path)` - Convert to platform-specific format
- `path.from_platform(path)` - Convert from platform-specific format
- `path.expanduser(path)` - Expand ~ to home directory

## Usage Examples

### Basic Path Operations

```cursed
yeet "path"

fr fr Join path components
sus config_path = path.join("home", "user", ".config", "app", "settings.json")
vibez.spillf("Config path: {}", config_path)

fr fr Extract path components
vibez.spillf("Directory: {}", path.dirname(config_path))
vibez.spillf("Filename: {}", path.basename(config_path))
vibez.spillf("Extension: {}", path.extension(config_path))
vibez.spillf("Stem: {}", path.stem(config_path))
```

### Path Normalization

```cursed
yeet "path"

sus messy_path = "/home/user/../user/./documents/../downloads/file.txt"
sus clean_path = path.normalize(messy_path)
vibez.spillf("Original: {}", messy_path)
vibez.spillf("Normalized: {}", clean_path)

fr fr Convert relative to absolute
sus relative_path = "docs/readme.md"
sus absolute_path = path.absolute(relative_path)
vibez.spillf("Relative: {}", relative_path)
vibez.spillf("Absolute: {}", absolute_path)
```

### File System Queries

```cursed
yeet "path"

sus paths []tea = [
    "/home",
    "/tmp",
    "/etc/passwd",
    "/nonexistent",
    "."
]

bestie p in paths {
    vibez.spillf("Path: {}", p)
    vibez.spillf("  Exists: {}", path.exists(p))
    if path.exists(p) {
        vibez.spillf("  Is file: {}", path.is_file(p))
        vibez.spillf("  Is directory: {}", path.is_dir(p))
        vibez.spillf("  Is symlink: {}", path.is_symlink(p))
    }
    vibez.spillf("  Is absolute: {}", path.is_absolute(p))
}
```

### Path Splitting and Analysis

```cursed
yeet "path"

sus file_path = "/home/user/documents/project/src/main.rs"

fr fr Split into components
sus parts = path.parts(file_path)
vibez.spill("Path components:")
bestie part in parts {
    vibez.spillf("  '{}'", part)
}

fr fr Split into directory and filename
sus (dir, filename) = path.split(file_path)
vibez.spillf("Directory: {}", dir)
vibez.spillf("Filename: {}", filename)

fr fr Analyze file extension
match path.extension(file_path) {
    ".rs" => vibez.spill("Rust source file"),
    ".py" => vibez.spill("Python source file"),
    ".js" => vibez.spill("JavaScript file"),
    "" => vibez.spill("No extension"),
    ext => vibez.spillf("Unknown extension: {}", ext)
}
```

### Relative Path Calculations

```cursed
yeet "path"

sus base_dir = "/home/user/projects"
sus target_file = "/home/user/projects/myapp/src/main.💀"

sus relative = path.relative(base_dir, target_file)
vibez.spillf("From {} to {}", base_dir, target_file)
vibez.spillf("Relative path: {}", relative)

fr fr Reverse calculation
sus back_to_base = path.relative(target_file, base_dir)
vibez.spillf("Back to base: {}", back_to_base)
```

### Cross-Platform Path Handling

```cursed
yeet "path"
yeet "env"

fr fr Platform-specific operations
match env.platform() {
    "windows" => {
        sus win_path = "C:\\Users\\user\\Documents\\file.txt"
        sus normalized = path.normalize(win_path)
        vibez.spillf("Windows path: {}", normalized)
    },
    _ => {
        sus unix_path = "/home/user/documents/file.txt"
        sus normalized = path.normalize(unix_path)
        vibez.spillf("Unix path: {}", normalized)
    }
}

fr fr Get platform separator
sus sep = path.separator()
vibez.spillf("Platform separator: '{}'", sep)

fr fr Expand user home directory
sus home_file = "~/documents/important.txt"
sus expanded = path.expanduser(home_file)
vibez.spillf("Expanded: {}", expanded)
```

### Path Building Utilities

```cursed
yeet "path"

slay build_project_structure(project_name tea) {
    sus base = path.join("projects", project_name)
    
    sus directories []tea = [
        path.join(base, "src"),
        path.join(base, "tests"),
        path.join(base, "docs"),
        path.join(base, "assets"),
        path.join(base, "build")
    ]
    
    vibez.spillf("Project structure for '{}':", project_name)
    bestie dir in directories {
        vibez.spillf("  {}", dir)
    }
    
    sus files []tea = [
        path.join(base, "README.md"),
        path.join(base, "Cargo.toml"),
        path.join(base, "src", "main.💀"),
        path.join(base, "tests", "integration_test.💀")
    ]
    
    vibez.spill("Key files:")
    bestie file in files {
        vibez.spillf("  {}", file)
    }
}

build_project_structure("my_cursed_app")
```

### Safe Path Operations

```cursed
yeet "path"
yeet "error_drip"

slay safe_resolve_path(user_path tea, base_dir tea) Result<tea, tea> {
    fr fr Normalize and resolve the path
    sus normalized = path.normalize(user_path)
    sus resolved = path.resolve(normalized)
    
    fr fr Check if it's within the allowed base directory
    sus base_resolved = path.resolve(base_dir)
    if !resolved.starts_with(base_resolved) {
        damn error_drip.new_error("Path outside allowed directory")
    }
    
    damn error_drip.ok(resolved)
}

sus base = "/safe/directory"
sus user_input = "../../../etc/passwd"  # Potential directory traversal

match safe_resolve_path(user_input, base) {
    error_drip.Error(e) => vibez.spillf("Security error: {}", e),
    error_drip.Ok(safe_path) => vibez.spillf("Safe path: {}", safe_path)
}
```

## Compilation Examples

### Interpretation Mode
```bash
echo 'yeet "path"
sus joined = path.join("home", "user", "file.txt")
vibez.spillf("Joined path: {}", joined)' > path_test.💀

./cursed-unified path_test.💀
```

### Compilation Mode
```bash
./cursed-unified --compile path_test.💀
./path_test
```

## Advanced Examples

### Path Walker

```cursed
yeet "path"

slay walk_directory(dir tea, pattern tea) []tea {
    sus matches []tea = []
    fr fr This would integrate with fs module for actual directory walking
    fr fr For now, simulate with known paths
    
    sus test_files []tea = [
        path.join(dir, "file1.txt"),
        path.join(dir, "subdir", "file2.rs"),
        path.join(dir, "another", "file3.py")
    ]
    
    bestie file in test_files {
        if path.extension(file) == pattern {
            matches.push(file)
        }
    }
    
    damn matches
}

sus rust_files = walk_directory("/src", ".rs")
vibez.spillf("Found {} Rust files", rust_files.len())
```

### Path Template System

```cursed
yeet "path"

squad PathTemplate {
    spill template tea
}

flex PathTemplate {
    slay new(template tea) PathTemplate {
        damn PathTemplate{template: template}
    }
    
    slay render(self, vars map[tea]tea) tea {
        sus result = self.template
        bestie (key, value) in vars {
            sus placeholder = "{" + key + "}"
            result = result.replace(placeholder, value)
        }
        damn result
    }
}

sus template = PathTemplate.new("{base}/{project}/src/{module}.{ext}")
sus vars = {
    "base": "/home/user/projects",
    "project": "myapp", 
    "module": "main",
    "ext": "csd"
}

sus rendered = template.render(vars)
vibez.spillf("Rendered path: {}", rendered)
```

## Implementation Notes

- Cross-platform compatibility (Windows, macOS, Linux)
- Unicode support for international file names
- Efficient string operations for path manipulation
- Security considerations for path traversal
- Pure CURSED implementation

## Dependencies

- `string_simple` - For string operations
- `env` - For environment variable access
- Core file system integration
- No external dependencies

## Security Considerations

1. **Validate user-provided paths** to prevent directory traversal
2. **Normalize paths** before security checks
3. **Use allowlists** for permitted directories
4. **Check permissions** before file operations
5. **Sanitize file names** from user input

## Performance Considerations

- Efficient string operations for path manipulation
- Minimal file system calls for existence checks
- Caching of expensive operations like `resolve()`
- Optimized path normalization algorithms

## Best Practices

1. **Always normalize** user-provided paths
2. **Use absolute paths** for configuration
3. **Check existence** before file operations
4. **Handle platform differences** appropriately
5. **Use path templates** for complex path generation
6. **Validate file extensions** for security
7. **Use relative paths** for portability when appropriate
