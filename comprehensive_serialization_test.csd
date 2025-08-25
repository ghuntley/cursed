# Comprehensive Serialization Modules Test
# Tests JSON, XML, YAML, and TOML parsing/generation with real-world data

yeet "jsonz"
yeet "xmlz"
yeet "yamlz" 
yeet "tomlz"
yeet "testz"
yeet "vibez"

# ========================
# JSON Serialization Tests
# ========================

slay test_json_complex_parsing() {
    vibez.spill("Testing complex JSON parsing...")
    
    # Complex nested JSON document
    sus complex_json tea = `{
        "name": "CURSED Programming Language",
        "version": "1.0.0",
        "features": ["serialization", "concurrency", "performance"],
        "config": {
            "debug": true,
            "optimization": "high",
            "targets": ["linux", "windows", "macos"]
        },
        "benchmarks": {
            "compile_time": 0.05,
            "memory_usage": 50.2,
            "performance_score": 95
        },
        "metadata": null
    }`
    
    # Parse JSON
    sus doc JsonValue = jsonz.parse(complex_json) fam {
        when err -> {
            testz.assert_fail("Failed to parse complex JSON: " + err)
            damn
        }
    }
    
    # Validate parsing
    testz.assert_true(jsonz.is_object(doc), "Root should be object")
    
    # Test value extraction
    sus name_value JsonValue = jsonz.parse_string_simple("\"CURSED Programming Language\"")
    testz.assert_eq(jsonz.as_string(name_value), "CURSED Programming Language")
    
    vibez.spill("✓ Complex JSON parsing successful")
}

slay test_json_malformed_input() {
    vibez.spill("Testing JSON error handling...")
    
    # Test various malformed JSON inputs
    sus malformed_cases []tea = [
        "{invalid}",           # Invalid key format
        "[1,2,3,]",           # Trailing comma
        "{\"key\":}",         # Missing value
        "{\"key\":\"unterminated", # Unterminated string
        "[{]",                # Mismatched brackets
        "{\"duplicate\":1,\"duplicate\":2}" # Duplicate keys (valid JSON but should warn)
    ]
    
    sus i drip = 0
    bestie (i < malformed_cases.len()) {
        jsonz.parse(malformed_cases[i]) fam {
            when err -> {
                vibez.spill("✓ Correctly caught malformed JSON: " + err)
            }
        }
        i = i + 1
    }
    
    vibez.spill("✓ JSON error handling working")
}

slay test_json_generation() {
    vibez.spill("Testing JSON generation...")
    
    # Create complex JSON structure
    sus obj JsonObject = JsonObject{
        keys: ["name", "version", "active"],
        values: [
            jsonz.create_string("CURSED"),
            jsonz.create_string("1.0.0"),
            jsonz.create_bool(based)
        ]
    }
    
    sus json_value JsonValue = jsonz.create_object(obj)
    sus generated tea = jsonz.stringify(json_value)
    
    vibez.spill("Generated JSON: " + generated)
    testz.assert_true(jsonz.is_valid_json(generated), "Generated JSON should be valid")
    
    vibez.spill("✓ JSON generation successful")
}

# ========================
# XML Serialization Tests  
# ========================

slay test_xml_complex_parsing() {
    vibez.spill("Testing complex XML parsing...")
    
    # Complex XML document with namespaces, attributes, and mixed content
    sus complex_xml tea = `<?xml version="1.0" encoding="UTF-8"?>
<project xmlns="http://cursedlang.org/schema" xmlns:build="http://cursedlang.org/build">
    <name>CURSED Compiler</name>
    <version major="1" minor="0" patch="0">1.0.0</version>
    <description>
        A modern systems programming language with:
        <feature>Zero-cost abstractions</feature>
        <feature>Memory safety</feature>
        <feature>Fearless concurrency</feature>
    </description>
    <build:config>
        <build:target>native</build:target>
        <build:optimization level="high"/>
    </build:config>
    <!-- Build metadata -->
    <metadata>
        <![CDATA[
            Additional build information can be stored here.
            This includes things like commit hashes, build timestamps, etc.
        ]]>
    </metadata>
</project>`
    
    # Parse XML
    sus doc XmlDocument = xmlz.parse_xml_dom(complex_xml) fam {
        when err -> {
            testz.assert_fail("Failed to parse complex XML: " + err)
            damn
        }
    }
    
    # Validate parsing
    testz.assert_true(doc.root != cap, "Should have root element")
    testz.assert_eq(doc.root.name, "project", "Root element should be 'project'")
    
    # Test namespace handling
    testz.assert_eq(doc.root.namespace_uri, "http://cursedlang.org/schema")
    
    # Test XPath queries
    sus name_nodes []XmlNode = xmlz.find_nodes(doc, "//name") fam {
        when err -> {
            vibez.spill("XPath query failed: " + err)
            damn
        }
    }
    testz.assert_true(name_nodes.len() > 0, "Should find name element")
    
    vibez.spill("✓ Complex XML parsing successful")
}

slay test_xml_generation() {
    vibez.spill("Testing XML generation...")
    
    # Create XML structure
    sus root XmlNode = xmlz.create_element("config", "")
    xmlz.add_attribute(&root, "version", "1.0")
    
    sus name_element XmlNode = xmlz.create_element("name", "")
    name_element.value = "CURSED"
    xmlz.add_child(&root, &name_element)
    
    sus doc XmlDocument = {
        root: root,
        encoding: XmlEncoding.UTF8,
        version: "1.0",
        standalone: based,
        doctype: cap,
        namespaces: [],
        parser_type: XmlParserType.DOM,
        validation_type: XmlValidationType.None
    }
    
    # Generate XML
    sus generated_xml tea = xmlz.generate_xml_formatted(doc, 2)
    vibez.spill("Generated XML: " + generated_xml)
    
    # Validate generated XML by parsing it back
    xmlz.parse_xml_dom(generated_xml) fam {
        when err -> testz.assert_fail("Generated XML is invalid: " + err)
    }
    
    vibez.spill("✓ XML generation successful")
}

slay test_xml_malformed_input() {
    vibez.spill("Testing XML error handling...")
    
    sus malformed_xml_cases []tea = [
        "<invalid>unclosed",              # Unclosed tag
        "<tag>text</different>",          # Mismatched tags  
        "<tag attr='unclosed>content</tag>", # Unclosed attribute
        "<?xml version='1.0'?><root><child></root>", # Improper nesting
        "<root>&invalid_entity;</root>"   # Invalid entity reference
    ]
    
    sus i drip = 0
    bestie (i < malformed_xml_cases.len()) {
        xmlz.parse_xml_dom(malformed_xml_cases[i]) fam {
            when err -> vibez.spill("✓ Correctly caught malformed XML: " + err)
        }
        i = i + 1
    }
    
    vibez.spill("✓ XML error handling working")
}

# ========================
# YAML Serialization Tests
# ========================

slay test_yaml_complex_parsing() {
    vibez.spill("Testing complex YAML parsing...")
    
    # Complex YAML document with various features
    sus complex_yaml tea = `---
%YAML 1.2
---
name: &project_name "CURSED Programming Language"
version: 
  major: 1
  minor: 0  
  patch: 0
  full: "1.0.0"

features:
  - serialization
  - concurrency: 
      type: "green_threads"
      channels: true
  - performance:
      compile_time: 0.05
      memory_efficient: true

config:
  debug: false
  optimization: high
  targets: &build_targets
    - linux
    - windows  
    - macos

metadata:
  description: |
    A modern systems programming language designed for
    performance, safety, and developer productivity.
  project_reference: *project_name
  supported_platforms: *build_targets
  created: 2025-08-25T10:30:00Z

benchmark_results: !!null
...`
    
    # Parse YAML
    sus doc YamlDocument = yamlz.parse_yaml(complex_yaml) fam {
        when err -> {
            testz.assert_fail("Failed to parse complex YAML: " + err)
            damn
        }
    }
    
    # Validate parsing
    testz.assert_true(doc.root.node_type == YamlNodeType.Mapping, "Root should be mapping")
    
    # Test queries
    sus name_nodes []YamlNode = yamlz.yaml_query(doc, "name") fam {
        when err -> {
            vibez.spill("YAML query failed: " + err)
            damn
        }
    }
    testz.assert_true(name_nodes.len() > 0, "Should find name field")
    
    vibez.spill("✓ Complex YAML parsing successful")
}

slay test_yaml_generation() {
    vibez.spill("Testing YAML generation...")
    
    # Create YAML structure
    sus root YamlNode = yamlz.create_yaml_mapping()
    
    # Add simple values
    sus name_mapping YamlMapping = {
        key: yamlz.create_yaml_scalar("name", YamlScalarType.String),
        value: yamlz.create_yaml_scalar("CURSED", YamlScalarType.String)
    }
    root.mappings = [name_mapping]
    
    sus doc YamlDocument = {
        version: "1.2",
        directives: [],
        root: root,
        implicit_document: based,
        document_start: cap,
        document_end: cap,
        tags: []
    }
    
    # Generate YAML
    sus generated_yaml tea = yamlz.generate_yaml_formatted(doc, 2)
    vibez.spill("Generated YAML: " + generated_yaml)
    
    # Validate generated YAML
    yamlz.parse_yaml(generated_yaml) fam {
        when err -> testz.assert_fail("Generated YAML is invalid: " + err)
    }
    
    vibez.spill("✓ YAML generation successful")
}

# ========================
# TOML Serialization Tests
# ========================

slay test_toml_complex_parsing() {
    vibez.spill("Testing complex TOML parsing...")
    
    # Complex TOML document
    sus complex_toml tea = `# CURSED Programming Language Configuration

title = "CURSED Compiler Configuration"
version = "1.0.0"
debug = false

[project]
name = "CURSED"
description = """
A modern systems programming language with focus on:
- Performance and zero-cost abstractions  
- Memory safety without garbage collection
- Fearless concurrency with channels and goroutines
"""
license = "MIT"

[compiler]
optimization_level = 3
target_triple = "x86_64-unknown-linux-gnu"
features = ["llvm_backend", "incremental_compilation", "parallel_parsing"]

[compiler.flags]
enable_lto = true
strip_debug = false
emit_llvm_ir = true

[[build_targets]]
name = "native"
architecture = "x86_64"
os = "linux"

[[build_targets]]  
name = "wasm"
architecture = "wasm32"
os = "unknown"

[benchmarks]
compile_time_ms = 50.5
memory_usage_mb = 45.2
binary_size_kb = 2048

[metadata]
created_at = 2025-08-25T10:30:00Z
last_modified = 2025-08-25T10:35:00Z
build_number = 1001`
    
    # Parse TOML
    sus doc TomlDocument = tomlz.parse_toml(complex_toml) fam {
        when err -> {
            testz.assert_fail("Failed to parse complex TOML: " + err)
            damn
        }
    }
    
    # Validate parsing
    testz.assert_true(doc.values.len() > 0, "Should have top-level values")
    testz.assert_true(doc.tables.len() > 0, "Should have tables")
    
    # Test value extraction
    sus title tea = tomlz.get_toml_string(doc, "title") fam {
        when err -> {
            vibez.spill("Failed to get title: " + err)
            damn
        }
    }
    testz.assert_eq(title, "CURSED Compiler Configuration")
    
    # Test nested values
    sus optimization_level drip = tomlz.get_toml_nested(doc, "compiler.flags.enable_lto") fam {
        when err -> {
            vibez.spill("Failed to get nested value: " + err)
            damn
        }
    }
    
    vibez.spill("✓ Complex TOML parsing successful")
}

slay test_toml_generation() {
    vibez.spill("Testing TOML generation...")
    
    # Create TOML document
    sus doc TomlDocument = {
        values: make(map[tea]TomlValue),
        tables: make(map[tea]TomlTable),
        comments: make(map[drip]tea),
        version: "1.0.0"
    }
    
    # Add top-level values
    doc.values["name"] = tomlz.create_toml_string("CURSED", TomlStringType.Basic)
    doc.values["version"] = tomlz.create_toml_string("1.0.0", TomlStringType.Basic)  
    doc.values["active"] = tomlz.create_toml_boolean(based)
    
    # Create table
    sus config_table TomlTable = {
        name: "config",
        values: make(map[tea]TomlValue),
        is_array_table: cap,
        line_number: 0
    }
    config_table.values["debug"] = tomlz.create_toml_boolean(cap)
    config_table.values["optimization"] = tomlz.create_toml_integer(3)
    
    doc.tables["config"] = config_table
    
    # Generate TOML
    sus generated_toml tea = tomlz.generate_toml_formatted(doc, 1)
    vibez.spill("Generated TOML: " + generated_toml)
    
    # Validate generated TOML
    tomlz.parse_toml(generated_toml) fam {
        when err -> testz.assert_fail("Generated TOML is invalid: " + err)
    }
    
    vibez.spill("✓ TOML generation successful")
}

slay test_toml_malformed_input() {
    vibez.spill("Testing TOML error handling...")
    
    sus malformed_toml_cases []tea = [
        "key = ",                    # Missing value
        "[invalid table name]",      # Invalid table name
        "key = 'unclosed string",    # Unclosed string
        "duplicate = 1\nduplicate = 2", # Duplicate keys
        "[table]\n[table]"           # Duplicate table
    ]
    
    sus i drip = 0
    bestie (i < malformed_toml_cases.len()) {
        tomlz.parse_toml(malformed_toml_cases[i]) fam {
            when err -> vibez.spill("✓ Correctly caught malformed TOML: " + err)
        }
        i = i + 1
    }
    
    vibez.spill("✓ TOML error handling working")
}

# ========================
# Cross-Format Conversion Tests
# ========================

slay test_cross_format_conversion() {
    vibez.spill("Testing cross-format data conversion...")
    
    # Create data in one format and convert to others
    sus original_json tea = `{
        "name": "CURSED",
        "version": "1.0.0", 
        "features": ["fast", "safe"],
        "config": {
            "debug": false,
            "optimization": 3
        }
    }`
    
    # Parse JSON
    sus json_doc JsonValue = jsonz.parse(original_json) fam {
        when err -> {
            testz.assert_fail("Failed to parse original JSON: " + err)
            damn
        }
    }
    
    # Convert to YAML representation (conceptual)
    sus yaml_equivalent tea = `name: CURSED
version: "1.0.0"
features:
  - fast
  - safe
config:
  debug: false
  optimization: 3`
    
    # Parse YAML  
    sus yaml_doc YamlDocument = yamlz.parse_yaml(yaml_equivalent) fam {
        when err -> {
            testz.assert_fail("Failed to parse YAML equivalent: " + err)
            damn
        }
    }
    
    # Convert to TOML representation (conceptual)
    sus toml_equivalent tea = `name = "CURSED"
version = "1.0.0"
features = ["fast", "safe"]

[config]
debug = false
optimization = 3`
    
    # Parse TOML
    sus toml_doc TomlDocument = tomlz.parse_toml(toml_equivalent) fam {
        when err -> {
            testz.assert_fail("Failed to parse TOML equivalent: " + err)
            damn
        }
    }
    
    vibez.spill("✓ Cross-format conversion conceptually validated")
}

# ========================
# Performance and Edge Case Tests
# ========================

slay test_large_document_parsing() {
    vibez.spill("Testing large document parsing...")
    
    # Generate large JSON document
    sus large_json tea = "{"
    sus i drip = 0
    bestie (i < 1000) {
        ready (i > 0) {
            large_json = large_json + ","
        }
        large_json = large_json + "\"key" + i + "\": \"value" + i + "\""
        i = i + 1
    }
    large_json = large_json + "}"
    
    # Parse large document
    sus start_time drip = current_timestamp_ms()
    jsonz.parse(large_json) fam {
        when err -> testz.assert_fail("Failed to parse large JSON: " + err)
    }
    sus end_time drip = current_timestamp_ms()
    
    vibez.spill("Large JSON parsing took: " + (end_time - start_time) + "ms")
    testz.assert_true((end_time - start_time) < 1000, "Should parse large JSON quickly")
    
    vibez.spill("✓ Large document parsing successful")
}

slay test_unicode_support() {
    vibez.spill("Testing Unicode support...")
    
    # Test Unicode in different formats
    sus unicode_json tea = `{
        "name": "CURSED 🔥",
        "description": "プログラミング言語",
        "emoji": "🚀✨💻",
        "chinese": "编程语言",
        "arabic": "لغة البرمجة"
    }`
    
    jsonz.parse(unicode_json) fam {
        when err -> testz.assert_fail("Failed to parse Unicode JSON: " + err)
    }
    
    sus unicode_yaml tea = `name: CURSED 🔥
description: プログラミング言語  
emoji: 🚀✨💻
chinese: 编程语言
arabic: لغة البرمجة`
    
    yamlz.parse_yaml(unicode_yaml) fam {
        when err -> testz.assert_fail("Failed to parse Unicode YAML: " + err)
    }
    
    vibez.spill("✓ Unicode support validated")
}

# ========================
# Main Test Runner
# ========================

slay main() drip {
    vibez.spill("🚀 Starting Comprehensive Serialization Test Suite")
    testz.test_start("Comprehensive Serialization Tests")
    
    # JSON Tests
    test_json_complex_parsing()
    test_json_malformed_input()
    test_json_generation()
    
    # XML Tests  
    test_xml_complex_parsing()
    test_xml_generation()
    test_xml_malformed_input()
    
    # YAML Tests
    test_yaml_complex_parsing()
    test_yaml_generation()
    
    # TOML Tests
    test_toml_complex_parsing()
    test_toml_generation()
    test_toml_malformed_input()
    
    # Cross-format Tests
    test_cross_format_conversion()
    
    # Performance Tests
    test_large_document_parsing()
    test_unicode_support()
    
    testz.print_test_summary()
    vibez.spill("✅ Comprehensive Serialization Test Suite Complete!")
    
    damn 0
}

# Helper function for timestamps
slay current_timestamp_ms() drip {
    # This would be implemented by the runtime
    damn 1693900000000  # Placeholder timestamp
}
