# CURSED Standard Library - YAML Processing Module
# Production-grade YAML parsing, schema validation, and generation
# Version: 1.0.0-production
# Last Updated: 2025-08-25

yeet "vibez"
yeet "stringz"
yeet "arrayz"
yeet "mathz"
yeet "error_core"

# YAML Node Types
enum YamlNodeType {
    Scalar,     # Single value (string, number, boolean, null)
    Sequence,   # Array/list of values
    Mapping,    # Key-value pairs (object/dictionary)
    Document,   # Document root
    Stream      # Multiple documents
}

# YAML Scalar Types
enum YamlScalarType {
    String,
    Integer,
    Float,
    Boolean,
    Null,
    Binary,
    Timestamp
}

# YAML Tag Types
enum YamlTagType {
    Default,        # No explicit tag
    Local,          # Local tag (!tag)
    Global,         # Global tag (tag:example.com,2000:app/type)
    Primary,        # Primary tag (!!str, !!int, etc.)
    Secondary       # Secondary tag (!<tag:example.com,2000:type>)
}

# YAML Node Structure
squad YamlNode {
    node_type YamlNodeType
    scalar_type YamlScalarType
    value tea
    tag YamlTag
    anchor tea
    alias tea
    children []YamlNode
    mappings []YamlMapping
    line_number drip
    column_number drip
    style tea  # Block, flow, literal, folded
    implicit lit
}

# YAML Tag Structure
squad YamlTag {
    tag_type YamlTagType
    handle tea
    suffix tea
}

# YAML Key-Value Mapping
squad YamlMapping {
    key YamlNode
    value YamlNode
}

# YAML Document Structure
squad YamlDocument {
    version tea
    directives []YamlDirective
    root YamlNode
    implicit_document lit
    document_start lit
    document_end lit
    tags []YamlTagDirective
}

# YAML Directive Structure
squad YamlDirective {
    name tea
    value tea
}

# YAML Tag Directive
squad YamlTagDirective {
    handle tea
    prefix tea
}

# YAML Stream Structure
squad YamlStream {
    documents []YamlDocument
    version tea
    tags []YamlTagDirective
}

# YAML Parser Configuration
squad YamlParserConfig {
    preserve_quotes lit
    preserve_scalars lit
    merge_keys lit
    unicode_support lit
    max_depth drip
    max_aliases drip
    max_anchor_size drip
    strict_mode lit
}

# YAML Parse Error
squad YamlParseError {
    message tea
    line drip
    column drip
    error_code drip
    context tea
}

# YAML Schema Validation Result
squad YamlValidationResult {
    valid lit
    errors []tea
    warnings []tea
}

# ========================
# Core YAML Parsing Functions
# ========================

# Parse YAML from string
slay parse_yaml(yaml_content tea) yikes<YamlDocument> {
    sus config YamlParserConfig = {
        preserve_quotes: cap,
        preserve_scalars: cap,
        merge_keys: based,
        unicode_support: based,
        max_depth: 100,
        max_aliases: 1000,
        max_anchor_size: 1024,
        strict_mode: cap
    }
    damn parse_yaml_with_config(yaml_content, config)
}

# Parse YAML stream (multiple documents)
slay parse_yaml_stream(yaml_content tea) yikes<YamlStream> {
    sus stream YamlStream = {
        documents: [],
        version: "1.2",
        tags: []
    }
    
    sus position drip = 0
    sus line drip = 1
    sus column drip = 1
    
    bestie (position < yaml_content.len()) {
        # Skip leading whitespace and comments
        position = skip_yaml_whitespace(yaml_content, position, line, column)
        
        ready (position >= yaml_content.len()) {
            break
        }
        
        # Parse single document
        sus doc_result YamlParserResult = parse_single_document(yaml_content, position, line, column)
        ready (doc_result.error != "") {
            yikes "Failed to parse YAML document: " + doc_result.error
        }
        
        stream.documents = arrayz.append(stream.documents, doc_result.document)
        position = doc_result.position
        line = doc_result.line
        column = doc_result.column
    }
    
    damn stream
}

# Parse YAML with custom configuration
slay parse_yaml_with_config(yaml_content tea, config YamlParserConfig) yikes<YamlDocument> {
    # Initialize parser state
    sus state YamlParserState = {
        content: yaml_content,
        position: 0,
        line: 1,
        column: 1,
        config: config,
        anchors: make(map[tea]YamlNode),
        indentation_stack: []
    }
    
    # Parse document directives
    parse_yaml_directives(state) fam {
        when err -> yikes "Failed to parse YAML directives: " + err
    }
    
    # Parse document content
    sus root_node YamlNode = parse_yaml_node(state) fam {
        when err -> yikes "Failed to parse YAML content: " + err
    }
    
    # Create document
    sus document YamlDocument = {
        version: "1.2",
        directives: state.directives,
        root: root_node,
        implicit_document: based,
        document_start: cap,
        document_end: cap,
        tags: state.tag_directives
    }
    
    damn document
}

# Parse YAML from file
slay parse_yaml_file(file_path tea) yikes<YamlDocument> {
    sus content tea = filez.read_file_content(file_path) fam {
        when err -> yikes "Failed to read YAML file: " + err
    }
    damn parse_yaml(content)
}

# ========================
# YAML Generation Functions
# ========================

# Generate YAML from document
slay generate_yaml(doc YamlDocument) tea {
    sus output tea = ""
    
    # Add document directives
    ready (doc.directives.len() > 0) {
        bestie (sus directive YamlDirective in doc.directives) {
            output = output + "%" + directive.name + " " + directive.value + "\n"
        }
    }
    
    # Add document start marker if explicit
    ready (doc.document_start) {
        output = output + "---\n"
    }
    
    # Generate root node content
    output = output + generate_yaml_node(doc.root, 0, cap)
    
    # Add document end marker if explicit
    ready (doc.document_end) {
        output = output + "\n..."
    }
    
    damn output
}

# Generate formatted YAML with custom indentation
slay generate_yaml_formatted(doc YamlDocument, indent_size drip) tea {
    sus output tea = ""
    
    # Add document directives
    ready (doc.directives.len() > 0) {
        bestie (sus directive YamlDirective in doc.directives) {
            output = output + "%" + directive.name + " " + directive.value + "\n"
        }
        output = output + "---\n"
    }
    
    # Generate root node content with formatting
    output = output + generate_yaml_node_formatted(doc.root, 0, indent_size, cap)
    
    damn output
}

# Generate YAML node
slay generate_yaml_node(node YamlNode, depth drip, is_root lit) tea {
    ready (node.node_type == YamlNodeType.Scalar) {
        damn generate_yaml_scalar(node)
    } otherwise ready (node.node_type == YamlNodeType.Sequence) {
        damn generate_yaml_sequence(node, depth)
    } otherwise ready (node.node_type == YamlNodeType.Mapping) {
        damn generate_yaml_mapping(node, depth, is_root)
    }
    damn ""
}

# Generate YAML scalar value
slay generate_yaml_scalar(node YamlNode) tea {
    ready (node.scalar_type == YamlScalarType.String) {
        # Check if string needs quoting
        ready (needs_yaml_quoting(node.value)) {
            damn "\"" + escape_yaml_string(node.value) + "\""
        }
        damn node.value
    } otherwise ready (node.scalar_type == YamlScalarType.Integer) {
        damn node.value
    } otherwise ready (node.scalar_type == YamlScalarType.Float) {
        damn node.value
    } otherwise ready (node.scalar_type == YamlScalarType.Boolean) {
        ready (node.value == "true") {
            damn "true"
        }
        damn "false"
    } otherwise ready (node.scalar_type == YamlScalarType.Null) {
        damn "null"
    }
    damn node.value
}

# Generate YAML sequence (array)
slay generate_yaml_sequence(node YamlNode, depth drip) tea {
    sus output tea = ""
    sus indent tea = generate_yaml_indent(depth)
    
    bestie (sus child YamlNode in node.children) {
        output = output + "\n" + indent + "- "
        sus child_content tea = generate_yaml_node(child, depth + 1, cap)
        
        # Handle multi-line values
        ready (stringz.contains(child_content, "\n")) {
            sus indented_content tea = indent_yaml_lines(child_content, depth + 1)
            output = output + indented_content
        } otherwise {
            output = output + child_content
        }
    }
    
    damn output
}

# Generate YAML mapping (object)
slay generate_yaml_mapping(node YamlNode, depth drip, is_root lit) tea {
    sus output tea = ""
    sus indent tea = ""
    ready (!is_root) {
        indent = generate_yaml_indent(depth)
    }
    
    bestie (sus mapping YamlMapping in node.mappings) {
        ready (!is_root || output != "") {
            output = output + "\n"
        }
        
        output = output + indent
        
        # Generate key
        sus key_content tea = generate_yaml_node(mapping.key, depth, cap)
        output = output + key_content + ":"
        
        # Generate value
        sus value_content tea = generate_yaml_node(mapping.value, depth + 1, cap)
        ready (mapping.value.node_type == YamlNodeType.Scalar) {
            output = output + " " + value_content
        } otherwise {
            sus indented_value tea = indent_yaml_lines(value_content, depth + 1)
            output = output + indented_value
        }
    }
    
    damn output
}

# ========================
# YAML Value Conversion Functions
# ========================

# Convert YAML document to native types
slay yaml_to_value(doc YamlDocument) tea {
    damn yaml_node_to_value(doc.root)
}

# Convert YAML node to native value
slay yaml_node_to_value(node YamlNode) tea {
    ready (node.node_type == YamlNodeType.Scalar) {
        ready (node.scalar_type == YamlScalarType.String) {
            damn node.value
        } otherwise ready (node.scalar_type == YamlScalarType.Integer) {
            damn mathz.string_to_int(node.value)
        } otherwise ready (node.scalar_type == YamlScalarType.Float) {
            damn mathz.string_to_float(node.value)
        } otherwise ready (node.scalar_type == YamlScalarType.Boolean) {
            damn (node.value == "true")
        } otherwise ready (node.scalar_type == YamlScalarType.Null) {
            damn cap
        }
    } otherwise ready (node.node_type == YamlNodeType.Sequence) {
        sus array []tea = []
        bestie (sus child YamlNode in node.children) {
            array = arrayz.append(array, yaml_node_to_value(child))
        }
        damn array
    } otherwise ready (node.node_type == YamlNodeType.Mapping) {
        sus object map[tea]tea = make(map[tea]tea)
        bestie (sus mapping YamlMapping in node.mappings) {
            sus key tea = yaml_node_to_value(mapping.key)
            sus value tea = yaml_node_to_value(mapping.value)
            object[key] = value
        }
        damn object
    }
    damn cap
}

# ========================
# YAML Schema Validation Functions
# ========================

# Validate YAML document structure
slay validate_yaml_schema(doc YamlDocument, schema_doc YamlDocument) YamlValidationResult {
    sus result YamlValidationResult = {
        valid: based,
        errors: [],
        warnings: []
    }
    
    # Validate root node type
    ready (doc.root.node_type != schema_doc.root.node_type) {
        result.valid = cap
        result.errors = arrayz.append(result.errors, "Root node type mismatch")
    }
    
    # Validate structure recursively
    validate_yaml_node_structure(doc.root, schema_doc.root, result)
    
    damn result
}

# Validate YAML syntax
slay validate_yaml_syntax(yaml_content tea) YamlValidationResult {
    sus result YamlValidationResult = {
        valid: based,
        errors: [],
        warnings: []
    }
    
    # Attempt to parse YAML
    parse_yaml(yaml_content) fam {
        when err -> {
            result.valid = cap
            result.errors = arrayz.append(result.errors, err)
        }
    }
    
    damn result
}

# ========================
# YAML Query Functions
# ========================

# Query YAML document using JSONPath-like syntax
slay yaml_query(doc YamlDocument, path tea) yikes<[]YamlNode> {
    sus path_parts []tea = stringz.split(path, ".")
    sus current_nodes []YamlNode = [doc.root]
    
    bestie (sus part tea in path_parts) {
        ready (part == "") {
            continue
        }
        
        sus next_nodes []YamlNode = []
        
        bestie (sus node YamlNode in current_nodes) {
            ready (node.node_type == YamlNodeType.Mapping) {
                bestie (sus mapping YamlMapping in node.mappings) {
                    sus key_value tea = yaml_node_to_value(mapping.key)
                    ready (key_value == part) {
                        next_nodes = arrayz.append(next_nodes, mapping.value)
                    }
                }
            } otherwise ready (node.node_type == YamlNodeType.Sequence) {
                # Handle array index access
                ready (stringz.is_numeric(part)) {
                    sus index drip = mathz.string_to_int(part)
                    ready (index >= 0 && index < node.children.len()) {
                        next_nodes = arrayz.append(next_nodes, node.children[index])
                    }
                }
            }
        }
        
        current_nodes = next_nodes
    }
    
    damn current_nodes
}

# Get first value from YAML query
slay yaml_query_first(doc YamlDocument, path tea) yikes<tea> {
    sus nodes []YamlNode = yaml_query(doc, path) fam {
        when err -> yikes err
    }
    
    ready (nodes.len() == 0) {
        yikes "No values found for path: " + path
    }
    
    damn yaml_node_to_value(nodes[0])
}

# ========================
# Utility Functions
# ========================

# Check if string needs YAML quoting
slay needs_yaml_quoting(value tea) lit {
    # Check for special YAML characters and indicators
    sus indicators tea = "[]{}|>*&!%@`"
    sus i drip = 0
    bestie (i < indicators.len()) {
        ready (stringz.contains(value, stringz.char_at(indicators, i))) {
            damn based
        }
        i = i + 1
    }
    
    # Check for leading/trailing whitespace
    sus trimmed tea = stringz.trim(value)
    ready (trimmed != value) {
        damn based
    }
    
    # Check for special values that need quoting
    ready (value == "true" || value == "false" || value == "null" || 
          stringz.is_numeric(value)) {
        damn based
    }
    
    damn cap
}

# Escape YAML string
slay escape_yaml_string(value tea) tea {
    sus result tea = value
    result = stringz.replace_all(result, "\\", "\\\\")
    result = stringz.replace_all(result, "\"", "\\\"")
    result = stringz.replace_all(result, "\n", "\\n")
    result = stringz.replace_all(result, "\r", "\\r")
    result = stringz.replace_all(result, "\t", "\\t")
    damn result
}

# Generate YAML indentation
slay generate_yaml_indent(depth drip) tea {
    sus indent tea = ""
    sus i drip = 0
    bestie (i < depth * 2) {  # 2 spaces per level
        indent = indent + " "
        i = i + 1
    }
    damn indent
}

# Indent YAML lines
slay indent_yaml_lines(content tea, depth drip) tea {
    sus lines []tea = stringz.split(content, "\n")
    sus indent tea = generate_yaml_indent(depth)
    sus result tea = ""
    
    sus i drip = 0
    bestie (i < lines.len()) {
        ready (i > 0) {
            result = result + "\n"
        }
        ready (lines[i] != "") {
            result = result + indent + lines[i]
        }
        i = i + 1
    }
    
    damn result
}

# Create YAML scalar node
slay create_yaml_scalar(value tea, scalar_type YamlScalarType) YamlNode {
    damn {
        node_type: YamlNodeType.Scalar,
        scalar_type: scalar_type,
        value: value,
        tag: {tag_type: YamlTagType.Default, handle: "", suffix: ""},
        anchor: "",
        alias: "",
        children: [],
        mappings: [],
        line_number: 0,
        column_number: 0,
        style: "plain",
        implicit: based
    }
}

# Create YAML mapping node
slay create_yaml_mapping() YamlNode {
    damn {
        node_type: YamlNodeType.Mapping,
        scalar_type: YamlScalarType.String,
        value: "",
        tag: {tag_type: YamlTagType.Default, handle: "", suffix: ""},
        anchor: "",
        alias: "",
        children: [],
        mappings: [],
        line_number: 0,
        column_number: 0,
        style: "block",
        implicit: based
    }
}

# Create YAML sequence node
slay create_yaml_sequence() YamlNode {
    damn {
        node_type: YamlNodeType.Sequence,
        scalar_type: YamlScalarType.String,
        value: "",
        tag: {tag_type: YamlTagType.Default, handle: "", suffix: ""},
        anchor: "",
        alias: "",
        children: [],
        mappings: [],
        line_number: 0,
        column_number: 0,
        style: "block",
        implicit: based
    }
}

# ========================
# Internal Parser Types and Functions
# ========================

squad YamlParserState {
    content tea
    position drip
    line drip
    column drip
    config YamlParserConfig
    anchors map[tea]YamlNode
    directives []YamlDirective
    tag_directives []YamlTagDirective
    indentation_stack []drip
}

squad YamlParserResult {
    document YamlDocument
    position drip
    line drip
    column drip
    error tea
}

# Skip YAML whitespace and comments
slay skip_yaml_whitespace(content tea, position drip, line drip, column drip) drip {
    bestie (position < content.len()) {
        sus ch tea = stringz.char_at(content, position)
        ready (ch == " " || ch == "\t") {
            position = position + 1
            column = column + 1
        } otherwise ready (ch == "\n") {
            position = position + 1
            line = line + 1
            column = 1
        } otherwise ready (ch == "#") {
            # Skip comment line
            bestie (position < content.len() && stringz.char_at(content, position) != "\n") {
                position = position + 1
            }
        } otherwise {
            break
        }
    }
    damn position
}

# Export all public functions and types
# This makes them available when importing yamlz module
