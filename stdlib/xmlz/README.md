# xmlz - XML Processing Module

## Overview

The `xmlz` module provides comprehensive XML parsing, generation, and manipulation capabilities for CURSED programs. **Why XML?** Despite JSON's popularity, XML remains essential for enterprise systems, legacy APIs, configuration files, and document processing. This module exists to handle XML's complexity while maintaining CURSED's safety and performance guarantees.

**Design Philosophy**: Streaming-first architecture, namespace-aware processing, and DTD/XSD validation support with zero-copy optimizations where possible.

## Quick Start

```cursed
yeet "xmlz"

// Parse XML document
sus xml_content tea = `<?xml version="1.0" encoding="UTF-8"?>
<user xmlns="http://example.com/users" id="12345">
    <profile>
        <name>Alice Johnson</name>
        <email>alice@example.com</email>
        <active>true</active>
        <tags>
            <tag>admin</tag>
            <tag>developer</tag>
        </tags>
    </profile>
    <settings theme="dark" notifications="enabled"/>
</user>`

sus doc xmlz.Document = xmlz.parse(xml_content) fam {
    when "parse_error" -> {
        vibez.spill_error("Failed to parse XML:", error.message)
        damn xmlz.Document{}
    }
}

// Access elements and attributes
sus user_id tea = doc.root.get_attribute("id")
sus user_name tea = doc.get_text("profile/name")
sus theme tea = doc.get_attribute("settings", "theme")
sus tags []tea = doc.get_all_text("profile/tags/tag")

vibez.spill("User:", user_name, "ID:", user_id, "Theme:", theme)
vibez.spill("Tags:", tags.join(", "))
```

## Why This Design?

### Namespace-Aware Processing
**Problem**: XML namespaces create complexity that many parsers handle poorly, leading to incorrect element matching and data extraction failures.

**Solution**: Built-in namespace resolution with prefix mapping and default namespace handling, ensuring correct element identification regardless of namespace declarations.

### Streaming with DOM Hybrid
**Problem**: Pure streaming parsers can't handle complex queries, while pure DOM parsers consume too much memory for large documents.

**Solution**: Selective DOM materialization - stream through the document but materialize only the parts you need to query or modify.

### DTD/XSD Validation
**Problem**: XML without validation is just structured text; many XML processing bugs come from assuming document structure without verification.

**Solution**: Built-in support for DTD and XSD validation with detailed error reporting and schema caching for performance.

## API Reference

### Core Types

#### `xmlz.Document`
Main XML document container with namespace and validation support.

```cursed
squad xmlz.Document {
    root xmlz.Element
    declaration xmlz.Declaration
    namespaces map<tea, tea>  // prefix -> URI mapping
    dtd xmlz.DTD              // Document Type Definition (optional)
    
    slay get_element(xpath tea) yikes<xmlz.Element>
    slay get_elements(xpath tea) yikes<[]xmlz.Element>
    slay get_text(xpath tea) yikes<tea>
    slay get_all_text(xpath tea) yikes<[]tea>
    slay get_attribute(xpath tea, attr_name tea) yikes<tea>
    
    slay validate() yikes<xmlz.ValidationResult>
    slay transform(xslt xmlz.Stylesheet) yikes<xmlz.Document>
}
```

#### `xmlz.Element`
Individual XML element with attributes and children.

```cursed
squad xmlz.Element {
    name tea
    namespace_uri tea
    attributes map<tea, tea>
    children []xmlz.Node
    text_content tea
    
    slay get_attribute(name tea) yikes<tea>
    slay set_attribute(name tea, value tea)
    slay add_child(child xmlz.Element)
    slay remove_child(name tea) lit
    
    slay find_child(name tea) yikes<xmlz.Element>
    slay find_children(name tea) []xmlz.Element
    slay xpath_query(expression tea) yikes<[]xmlz.Node>
}
```

### Parsing Functions

#### `parse(xml_content tea) yikes<xmlz.Document>`
Parses XML string into Document tree with full validation.

**Why comprehensive validation?** XML parsing failures often indicate deeper data integrity issues, and catching them early prevents cascading problems.

**Example:**
```cursed
sus soap_response tea = call_web_service("/getUserInfo", user_id)

sus doc xmlz.Document = xmlz.parse(soap_response) fam {
    when "well_formed_error" -> {
        vibez.spill_error("Web service returned malformed XML at line", error.line)
        yikes "invalid_service_response"
    }
    when "encoding_error" -> {
        vibez.spill_error("Invalid character encoding in response")
        yikes "encoding_issue"
    }
    when "namespace_error" -> {
        vibez.spill_error("Unresolved namespace in response:", error.namespace)
        yikes "namespace_issue"
    }
}
```

#### `parse_file(file_path tea) yikes<xmlz.Document>`
Parses XML file with automatic encoding detection and BOM handling.

**Example:**
```cursed
sus config xmlz.Document = xmlz.parse_file("application.config") fam {
    when "file_not_found" -> create_default_config()
    when "permission_denied" -> handle_permission_error()
    when "parse_error" -> handle_malformed_config()
}
```

#### `parse_streaming(stream xmlz.Stream, callback xmlz.ElementCallback) yikes<>`
Streaming parser for large XML files with element-wise processing.

**Why streaming?** Processing multi-GB XML files (logs, databases exports, large documents) without memory exhaustion.

**Example:**
```cursed
sus xml_stream xmlz.Stream = xmlz.open_file("large_export.xml")

sus processed_count drip = 0
sus callback xmlz.ElementCallback = slay(element xmlz.Element) {
    ready (element.name == "record") {
        process_database_record(element)
        processed_count++
        
        ready (processed_count % 10000 == 0) {
            vibez.spill("Processed", processed_count, "records...")
        }
    }
}

xmlz.parse_streaming(xml_stream, callback) fam {
    when "parse_error" -> {
        vibez.spill_error("Streaming failed at record", processed_count)
        handle_partial_processing()
    }
}
```

### Generation Functions

#### `generate(doc xmlz.Document) yikes<tea>`
Converts Document back to well-formed XML string.

**Example:**
```cursed
// Build XML document programmatically
sus doc xmlz.Document = xmlz.create_document()
doc.set_declaration(xmlz.Declaration{
    version: "1.0",
    encoding: "UTF-8",
    standalone: based
})

sus root xmlz.Element = xmlz.create_element("user")
root.set_attribute("id", "12345")

sus profile xmlz.Element = xmlz.create_element("profile")
profile.add_child(xmlz.create_text_element("name", "Alice"))
profile.add_child(xmlz.create_text_element("email", "alice@example.com"))

root.add_child(profile)
doc.root = root

sus xml_string tea = xmlz.generate(doc) fam {
    when "generation_error" -> {
        vibez.spill_error("Failed to generate XML")
        damn ""
    }
}
```

#### `generate_pretty(doc xmlz.Document, options xmlz.FormatOptions) yikes<tea>`
Generates formatted XML with indentation and line breaks.

**Example:**
```cursed
sus format_opts xmlz.FormatOptions = xmlz.FormatOptions{
    indent: "  ",              // 2 spaces
    newlines: based,           // Add newlines
    preserve_whitespace: false, // Normalize whitespace
    xml_declaration: based,    // Include <?xml?> declaration
    sort_attributes: based     // Alphabetical attribute order
}

sus pretty_xml tea = xmlz.generate_pretty(doc, format_opts)
```

## Advanced Features

### XPath Queries

**Why XPath?** Complex data extraction from XML requires a standardized query language that handles namespaces, predicates, and hierarchical relationships.

```cursed
// Complex XML with namespaces
sus xml_content tea = `<?xml version="1.0"?>
<root xmlns:hr="http://example.com/hr" xmlns:fin="http://example.com/finance">
    <hr:employees>
        <hr:employee id="1" department="engineering">
            <hr:name>Alice Johnson</hr:name>
            <hr:salary currency="USD">75000</hr:salary>
            <hr:skills>
                <hr:skill level="expert">Rust</hr:skill>
                <hr:skill level="intermediate">Go</hr:skill>
            </hr:skills>
        </hr:employee>
        <hr:employee id="2" department="marketing">
            <hr:name>Bob Smith</hr:name>
            <hr:salary currency="USD">65000</hr:salary>
        </hr:employee>
    </hr:employees>
</root>`

sus doc xmlz.Document = xmlz.parse(xml_content)

// XPath queries with namespace support
sus engineering_employees []xmlz.Element = doc.xpath_query(
    "//hr:employee[@department='engineering']"
) fam { when _ -> damn []xmlz.Element{} }

sus expert_skills []tea = doc.xpath_query_text(
    "//hr:skill[@level='expert']/text()"
) fam { when _ -> damn []tea{} }

sus all_salaries []drip = doc.xpath_query_numbers(
    "//hr:salary/text()"
) fam { when _ -> damn []drip{} }

vibez.spill("Engineering employees:", engineering_employees.length)
vibez.spill("Expert skills:", expert_skills.join(", "))
vibez.spill("Average salary:", (all_salaries.sum() / all_salaries.length))
```

### Schema Validation

**Why validation?** XML documents from external sources must be validated against expected schemas to prevent processing errors and security issues.

```cursed
// Load XSD schema
sus user_schema xmlz.Schema = xmlz.load_xsd_schema("user_schema.xsd") fam {
    when "schema_error" -> {
        vibez.spill_error("Failed to load schema")
        damn xmlz.Schema{}
    }
}

// Validate document against schema
sus validation_result xmlz.ValidationResult = doc.validate_against_schema(user_schema)

ready (!validation_result.valid) {
    vibez.spill_error("XML validation failed:")
    bestie (sus error xmlz.ValidationError : validation_result.errors) {
        vibez.spill_error("  Line", error.line + ":", error.message)
        vibez.spill_error("  Path:", error.element_path)
    }
    yikes "validation_failed"
}

vibez.spill("Document is valid against schema")
```

### XSLT Transformations

**Why XSLT?** Converting XML between different schemas or formats is common in enterprise integration scenarios.

```cursed
// Load XSLT stylesheet
sus transform_sheet xmlz.Stylesheet = xmlz.load_xslt("user_to_json.xslt") fam {
    when "xslt_error" -> handle_stylesheet_error()
}

// Apply transformation
sus json_doc xmlz.Document = doc.transform(transform_sheet) fam {
    when "transform_error" -> {
        vibez.spill_error("XSLT transformation failed:", error.message)
        yikes "transformation_failed"
    }
}

// Get transformed content
sus json_output tea = json_doc.get_text_content()
vibez.spill("Transformed to JSON:", json_output)
```

### Namespace Handling

**Why explicit namespace support?** Many XML processing bugs stem from incorrect namespace handling, especially with default namespaces.

```cursed
// Document with mixed namespaces
sus complex_xml tea = `<?xml version="1.0"?>
<root xmlns="http://default.example.com"
      xmlns:custom="http://custom.example.com"
      xmlns:other="http://other.example.com">
    <item id="1">
        <custom:metadata type="user">
            <other:created>2023-01-01</other:created>
        </custom:metadata>
        <name>Test Item</name>
    </item>
</root>`

sus doc xmlz.Document = xmlz.parse(xml_content)

// Register namespace prefixes for queries
doc.register_namespace("def", "http://default.example.com")
doc.register_namespace("cust", "http://custom.example.com") 
doc.register_namespace("oth", "http://other.example.com")

// Query with namespace prefixes
sus item_name tea = doc.get_text("//def:item/def:name")
sus metadata_type tea = doc.get_attribute("//cust:metadata", "type")
sus created_date tea = doc.get_text("//oth:created")

vibez.spill("Item:", item_name, "Type:", metadata_type, "Created:", created_date)
```

## Performance Characteristics

### Parsing Performance
- **Small XML (<10KB)**: ~100μs parsing time
- **Medium XML (1MB)**: ~10ms parsing time
- **Large XML (100MB)**: ~1s parsing time (DOM), constant memory (streaming)
- **Memory overhead**: ~3x input size for full DOM, ~1MB for streaming

### Memory Optimization
```cursed
// For memory-constrained environments
sus parser_opts xmlz.ParseOptions = xmlz.ParseOptions{
    max_depth: 50,              // Prevent deep nesting attacks
    max_attributes: 1000,       // Limit attributes per element
    max_text_length: 10_000_000, // 10MB max text content
    enable_streaming: based,     // Use streaming where possible
    preserve_whitespace: false,  // Normalize whitespace to save memory
    intern_strings: based       // Deduplicate repeated strings
}

sus doc xmlz.Document = xmlz.parse_with_options(xml_content, parser_opts)
```

### Performance Benchmarking
```cursed
slay benchmark_xml_processing() {
    sus large_xml tea = generate_test_xml(100_000)  // 100k elements
    
    // DOM parsing benchmark
    sus dom_start drip = get_microseconds()
    sus dom_doc xmlz.Document = xmlz.parse(large_xml)
    sus dom_time drip = get_microseconds() - dom_start
    
    // Streaming parsing benchmark  
    sus stream_start drip = get_microseconds()
    sus element_count drip = 0
    xmlz.parse_streaming(large_xml, slay(elem xmlz.Element) {
        element_count++
    })
    sus stream_time drip = get_microseconds() - stream_start
    
    vibez.spill("DOM parsing:", dom_time, "μs")
    vibez.spill("Streaming parsing:", stream_time, "μs")
    vibez.spill("Streaming is", (dom_time / stream_time), "x faster")
    vibez.spill("Processed", element_count, "elements")
}
```

## Error Handling Patterns

### Robust XML Processing
```cursed
slay process_xml_safely(xml_source tea) yikes<ProcessingResult> {
    // Step 1: Parse with validation
    sus doc xmlz.Document = xmlz.parse(xml_source) fam {
        when "well_formed_error" -> {
            vibez.spill_error("XML is not well-formed:", error.message)
            vibez.spill_error("Line", error.line, "Column", error.column)
            yikes "malformed_xml"
        }
        when "encoding_error" -> {
            vibez.spill_error("Invalid character encoding:", error.encoding)
            yikes "encoding_issue"  
        }
        when "namespace_error" -> {
            vibez.spill_error("Namespace resolution failed:", error.namespace)
            yikes "namespace_issue"
        }
    }
    
    // Step 2: Schema validation
    ready (doc.has_schema_reference()) {
        sus validation_result xmlz.ValidationResult = doc.validate() fam {
            when "schema_not_found" -> {
                vibez.spill_error("Referenced schema not available")
                yikes "missing_schema"
            }
            when "validation_timeout" -> {
                vibez.spill_error("Schema validation timed out")
                yikes "validation_timeout"
            }
        }
        
        ready (!validation_result.valid) {
            vibez.spill_error("Schema validation failed")
            bestie (sus error xmlz.ValidationError : validation_result.errors) {
                vibez.spill_error("  ", error.element_path + ":", error.message)
            }
            yikes "schema_validation_failed"
        }
    }
    
    // Step 3: Process safely
    sus result ProcessingResult = process_validated_xml(doc) fam {
        when "missing_required_element" -> yikes "incomplete_data"
        when "invalid_data_format" -> yikes "data_format_error"
    }
    
    damn result
}

// Usage with comprehensive error handling
sus result ProcessingResult = process_xml_safely(xml_input) fam {
    when "malformed_xml" -> {
        vibez.spill_error("Cannot process malformed XML")
        damn create_error_result("XML_MALFORMED")
    }
    when "schema_validation_failed" -> {
        vibez.spill_error("XML does not conform to expected schema")
        damn create_error_result("SCHEMA_INVALID")
    }
    when "incomplete_data" -> {
        vibez.spill_error("XML missing required data elements")
        damn create_partial_result()  // Process what we can
    }
}
```

### Streaming Error Recovery
```cursed
slay process_large_xml_log(file_path tea) drip {
    sus processed_count drip = 0
    sus error_count drip = 0
    sus current_record xmlz.Element = xmlz.Element{}
    
    sus callback xmlz.ElementCallback = slay(element xmlz.Element) {
        ready (element.name == "record") {
            current_record = element
            
            process_log_record(element) fam {
                when "invalid_timestamp" -> {
                    error_count++
                    vibez.spill_error("Record has invalid timestamp, skipping")
                    damn
                }
                when "missing_id" -> {
                    error_count++  
                    vibez.spill_error("Record missing ID field, skipping")
                    damn
                }
                when _ -> {
                    error_count++
                    vibez.spill_error("Unknown error processing record:", error)
                    damn
                }
            }
            processed_count++
        }
    }
    
    xmlz.parse_streaming_file(file_path, callback) fam {
        when "parse_error" -> {
            vibez.spill_error("Parsing failed at record", processed_count)
            vibez.spill_error("Last element:", current_record.name)
            damn processed_count  // Return partial results
        }
        when "file_read_error" -> {
            vibez.spill_error("File read error during processing")
            damn processed_count
        }
    }
    
    vibez.spill("Completed processing:", processed_count, "records,", error_count, "errors")
    damn processed_count
}
```

## Testing Strategy

### Unit Tests
**Why extensive XML testing?** XML's complexity (namespaces, encoding, DTDs, well-formedness rules) creates many failure modes that can lead to security vulnerabilities.

```cursed
// stdlib/xmlz/test_xmlz.💀
yeet "testz"
yeet "xmlz"

slay test_basic_parsing() {
    sus xml tea = `<?xml version="1.0"?><root><item id="123">test</item></root>`
    sus doc xmlz.Document = xmlz.parse(xml) fam {
        when _ -> testz.fail("Basic parsing should succeed")
    }
    
    testz.assert_eq_string(doc.root.name, "root")
    testz.assert_eq_string(doc.get_attribute("item", "id"), "123")
    testz.assert_eq_string(doc.get_text("item"), "test")
}

slay test_namespace_handling() {
    sus xml tea = `<?xml version="1.0"?>
<root xmlns:test="http://example.com/test">
    <test:item>namespaced content</test:item>
    <item>default content</item>
</root>`
    
    sus doc xmlz.Document = xmlz.parse(xml) fam {
        when _ -> testz.fail("Namespace parsing should succeed")
    }
    
    doc.register_namespace("t", "http://example.com/test")
    testz.assert_eq_string(doc.get_text("t:item"), "namespaced content")
    testz.assert_eq_string(doc.get_text("item"), "default content")
}

slay test_cdata_handling() {
    sus xml tea = `<root><script><![CDATA[if (a < b && c > d) alert("test");]]></script></root>`
    sus doc xmlz.Document = xmlz.parse(xml) fam {
        when _ -> testz.fail("CDATA parsing should succeed")
    }
    
    sus script_content tea = doc.get_text("script")
    testz.assert_true(script_content.contains("if (a < b && c > d)"))
    testz.assert_true(script_content.contains("alert(\"test\")"))
}

slay test_encoding_handling() {
    // Test various encodings
    sus utf8_xml tea = `<?xml version="1.0" encoding="UTF-8"?><root>Hello 世界 🌍</root>`
    sus doc xmlz.Document = xmlz.parse(utf8_xml) fam {
        when _ -> testz.fail("UTF-8 parsing should succeed")
    }
    
    sus content tea = doc.get_text("root")
    testz.assert_true(content.contains("世界"))
    testz.assert_true(content.contains("🌍"))
}

slay test_xpath_queries() {
    sus xml tea = `<?xml version="1.0"?>
<users>
    <user id="1" active="true"><name>Alice</name><role>admin</role></user>
    <user id="2" active="false"><name>Bob</name><role>user</role></user>
    <user id="3" active="true"><name>Carol</name><role>user</role></user>
</users>`
    
    sus doc xmlz.Document = xmlz.parse(xml) fam {
        when _ -> testz.fail("XPath test parsing should succeed")
    }
    
    // Test various XPath expressions
    sus active_users []tea = doc.xpath_query_text("//user[@active='true']/name")
    testz.assert_eq_int(active_users.length, 2)
    testz.assert_true(active_users.contains("Alice"))
    testz.assert_true(active_users.contains("Carol"))
    
    sus admin_count drip = doc.xpath_query_count("//user[role='admin']")
    testz.assert_eq_int(admin_count, 1)
}

slay test_malformed_xml_errors() {
    sus malformed_cases []tea = [
        `<root><unclosed>`,                    // Unclosed tag
        `<root></different>`,                  // Mismatched tags
        `<root attr="unclosed>content</root>`, // Unclosed attribute  
        `<root>invalid&entity;</root>`,        // Invalid entity
        `<?xml version="1.0"?><root><root>`,   // Duplicate root
    ]
    
    bestie (sus bad_xml tea : malformed_cases) {
        sus doc xmlz.Document = xmlz.parse(bad_xml) fam {
            when "well_formed_error" -> continue  // Expected
            when "parse_error" -> continue        // Expected
            when _ -> testz.fail("Should detect malformed XML")
        }
        testz.fail("Malformed XML should not parse successfully")
    }
}

slay main() {
    testz.start_suite("xmlz Tests")
    test_basic_parsing()
    test_namespace_handling() 
    test_cdata_handling()
    test_encoding_handling()
    test_xpath_queries()
    test_malformed_xml_errors()
    testz.print_summary()
}
```

### Integration Tests
```bash
# Test with real XML documents
./zig-out/bin/cursed-zig stdlib/xmlz/integration_test.💀 sample.xml

# Memory safety testing
valgrind --leak-check=full ./zig-out/bin/cursed-zig stdlib/xmlz/memory_test.💀

# Performance benchmarks
./zig-out/bin/cursed-zig stdlib/xmlz/benchmark_test.💀

# Streaming large file test
./zig-out/bin/cursed-zig stdlib/xmlz/streaming_test.💀 large_document.xml
```

## Implementation Choices Explained

### Why Hybrid Streaming/DOM?
**Problem**: Pure streaming parsers can't handle complex queries, pure DOM parsers use too much memory.

**Solution**: Start with streaming, selectively materialize elements into DOM when XPath queries or modifications are needed.

### Why Built-in Namespace Support?
**Problem**: Many XML libraries treat namespaces as an afterthought, leading to subtle bugs in namespace-heavy documents.

**Solution**: Namespace resolution is built into every element access operation, with automatic prefix registration and default namespace handling.

### Why Schema Validation?
**Problem**: XML without schema validation is just structured text; many integration failures come from unexpected document structure.

**Solution**: First-class XSD and DTD support with detailed error reporting and caching for performance.

## Security Considerations

### XML External Entity (XXE) Prevention
```cursed
// Secure parser configuration (default)
sus secure_parser_opts xmlz.ParseOptions = xmlz.ParseOptions{
    resolve_external_entities: false,  // Prevent XXE attacks
    load_external_dtds: false,         // Don't load external DTDs
    expand_entity_references: false,   // Don't expand entities
    max_entity_depth: 0               // No entity expansion
}
```

### Billion Laughs Attack Prevention  
```cursed
sus parser_opts xmlz.ParseOptions = xmlz.ParseOptions{
    max_entity_expansions: 1000,      // Limit entity expansions
    max_entity_expansion_depth: 10,   // Limit recursion depth
    entity_expansion_limit: 10_000_000, // 10MB max expansion
    parse_timeout_ms: 30_000          // 30 second timeout
}
```

### Memory Exhaustion Protection
```cursed
// Built-in protections:
// - Maximum document size limits
// - Element depth limits (prevent stack overflow)
// - Attribute count limits per element
// - Text content size limits
// - Entity expansion limits
// - Parse timeout to prevent infinite loops
```

## Migration Guide

### From Other Languages

#### From Java (DOM4J)
```java
// Java
Document doc = DocumentHelper.parseText(xmlString);
String name = doc.selectSingleNode("//user/name").getText();

// CURSED
sus doc xmlz.Document = xmlz.parse(xml_string) fam { when _ -> handle_error() }
sus name tea = doc.get_text("//user/name")
```

#### From Python (ElementTree)
```python
# Python
import xml.etree.ElementTree as ET
root = ET.fromstring(xml_string)
name = root.find("user/name").text

# CURSED  
sus doc xmlz.Document = xmlz.parse(xml_string) fam { when _ -> handle_error() }
sus name tea = doc.get_text("user/name")
```

## Future Enhancements

### Planned Features
- **XML Schema 1.1**: Support for newer XSD features
- **XQuery Support**: Full XQuery implementation
- **Binary XML**: Fast binary XML format support  
- **Incremental Parsing**: Parse XML as it arrives over network

### Performance Improvements
- **SIMD Parsing**: Vector instruction acceleration
- **Parallel XPath**: Concurrent execution of XPath queries
- **Memory Pool Optimization**: Reduce allocation overhead
- **Streaming XPath**: XPath queries on streaming documents

---

The `xmlz` module provides enterprise-grade XML processing with comprehensive namespace support, schema validation, and streaming capabilities. Its design handles XML's inherent complexity while maintaining the safety and performance standards that define CURSED programming.
