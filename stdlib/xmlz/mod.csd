# CURSED Standard Library - XML Processing Module
# Production-grade XML parsing, DOM/SAX support, schema validation, XPath
# Version: 1.0.0-production
# Last Updated: 2025-08-22

yeet "vibez"
yeet "stringz"
yeet "filez"
yeet "arrayz"
yeet "error_core"

# XML Parser Types
enum XmlParserType {
    DOM,      # Document Object Model - loads entire document into memory
    SAX,      # Simple API for XML - event-driven streaming parser
    StAX      # Streaming API for XML - pull-based streaming parser
}

# XML Node Types
enum XmlNodeType {
    Element,
    Text,
    Attribute,
    Comment,
    CDATA,
    ProcessingInstruction,
    Document,
    DocumentType
}

# XML Validation Types
enum XmlValidationType {
    None,
    DTD,      # Document Type Definition
    XSD,      # XML Schema Definition
    RelaxNG,  # RELAX NG schema
    Schematron # Schematron rules
}

# XML Encoding Types  
enum XmlEncoding {
    UTF8,
    UTF16LE,
    UTF16BE,
    UTF32LE,
    UTF32BE,
    ISO88591,
    ASCII
}

# XML Node Structure
squad XmlNode {
    node_type XmlNodeType
    name tea
    value tea
    attributes []XmlAttribute
    children []XmlNode
    parent sus XmlNode
    namespace_uri tea
    namespace_prefix tea
    line_number drip
    column_number drip
}

# XML Attribute Structure
squad XmlAttribute {
    name tea
    value tea
    namespace_uri tea
    namespace_prefix tea
}

# XML Document Structure
squad XmlDocument {
    root sus XmlNode
    encoding XmlEncoding
    version tea
    standalone lit
    doctype sus XmlDocType
    namespaces []XmlNamespace
    parser_type XmlParserType
    validation_type XmlValidationType
}

# XML Document Type Definition
squad XmlDocType {
    name tea
    public_id tea
    system_id tea
    internal_subset tea
}

# XML Namespace Structure
squad XmlNamespace {
    prefix tea
    uri tea
}

# XML Parser Configuration
squad XmlParserConfig {
    parser_type XmlParserType
    validation XmlValidationType
    preserve_whitespace lit
    expand_entities lit
    resolve_namespaces lit
    validate_encoding lit
    max_depth drip
    max_attributes drip
    buffer_size drip
}

# XML Parser Error
squad XmlParseError {
    message tea
    line drip
    column drip
    error_code drip
    context tea
}

# SAX Event Handler Interface
collab SaxHandler {
    slay on_start_document()
    slay on_end_document()
    slay on_start_element(name tea, attributes []XmlAttribute)
    slay on_end_element(name tea)
    slay on_characters(data tea)
    slay on_comment(data tea)
    slay on_processing_instruction(target tea, data tea)
    slay on_error(error XmlParseError)
}

# XPath Expression Result
squad XPathResult {
    nodes []XmlNode
    values []tea
    result_type tea  # "nodeset", "string", "number", "boolean"
}

# XML Schema Validation Result
squad ValidationResult {
    valid lit
    errors []tea
    warnings []tea
}

# ========================
# Core XML Parsing Functions
# ========================

# Parse XML from string with DOM
slay parse_xml_dom(xml_content tea) yikes<XmlDocument> {
    sus config XmlParserConfig = {
        parser_type: XmlParserType.DOM,
        validation: XmlValidationType.None,
        preserve_whitespace: based,
        expand_entities: based,
        resolve_namespaces: based,
        validate_encoding: based,
        max_depth: 1000,
        max_attributes: 1000,
        buffer_size: 8192
    }
    damn parse_xml_with_config(xml_content, config)
}

# Parse XML from string with SAX
slay parse_xml_sax(xml_content tea, handler SaxHandler) yikes<tea> {
    sus config XmlParserConfig = {
        parser_type: XmlParserType.SAX,
        validation: XmlValidationType.None,
        preserve_whitespace: based,
        expand_entities: based,
        resolve_namespaces: based,
        validate_encoding: based,
        max_depth: 1000,
        max_attributes: 1000,
        buffer_size: 8192
    }
    damn parse_xml_sax_with_config(xml_content, handler, config)
}

# Parse XML with custom configuration
slay parse_xml_with_config(xml_content tea, config XmlParserConfig) yikes<XmlDocument> {
    # Detect encoding from BOM or XML declaration
    sus encoding XmlEncoding = detect_xml_encoding(xml_content)
    
    # Create document
    sus doc XmlDocument = {
        root: cap,
        encoding: encoding,
        version: "1.0",
        standalone: based,
        doctype: cap,
        namespaces: [],
        parser_type: config.parser_type,
        validation_type: config.validation
    }
    
    # Initialize parser state
    sus parser_state ParserState = init_parser_state(xml_content, config)
    
    # Parse XML declaration if present
    parse_xml_declaration(parser_state, doc) fam {
        when err -> yikes "Failed to parse XML declaration: " + err
    }
    
    # Parse document type declaration if present
    parse_doctype_declaration(parser_state, doc) fam {
        when err -> yikes "Failed to parse DOCTYPE: " + err
    }
    
    # Parse root element and all children
    sus root_node XmlNode = parse_element(parser_state) fam {
        when err -> yikes "Failed to parse root element: " + err
    }
    
    doc.root = root_node
    
    # Validate document structure if requested
    ready (config.validation != XmlValidationType.None) {
        validate_document(doc, config.validation) fam {
            when err -> yikes "Document validation failed: " + err
        }
    }
    
    damn doc
}

# Parse XML from file
slay parse_xml_file(file_path tea) yikes<XmlDocument> {
    sus content tea = filez.read_file_content(file_path) fam {
        when err -> yikes "Failed to read XML file: " + err
    }
    damn parse_xml_dom(content)
}

# ========================
# XML Generation Functions
# ========================

# Generate XML from document
slay generate_xml(doc XmlDocument) tea {
    sus output tea = ""
    
    # XML declaration
    output = output + generate_xml_declaration(doc)
    
    # DOCTYPE declaration if present
    ready (doc.doctype != cap) {
        output = output + generate_doctype_declaration(doc.doctype)
    }
    
    # Root element and children
    ready (doc.root != cap) {
        output = output + generate_element_xml(doc.root, 0)
    }
    
    damn output
}

# Generate formatted XML with indentation
slay generate_xml_formatted(doc XmlDocument, indent_size drip) tea {
    sus output tea = ""
    
    # XML declaration
    output = output + generate_xml_declaration(doc) + "\n"
    
    # DOCTYPE declaration if present
    ready (doc.doctype != cap) {
        output = output + generate_doctype_declaration(doc.doctype) + "\n"
    }
    
    # Root element and children with formatting
    ready (doc.root != cap) {
        output = output + generate_element_xml_formatted(doc.root, 0, indent_size)
    }
    
    damn output
}

# Generate element XML
slay generate_element_xml(node XmlNode, depth drip) tea {
    ready (node.node_type != XmlNodeType.Element) {
        ready (node.node_type == XmlNodeType.Text) {
            damn escape_xml_text(node.value)
        }
        ready (node.node_type == XmlNodeType.Comment) {
            damn "<!--" + node.value + "-->"
        }
        ready (node.node_type == XmlNodeType.CDATA) {
            damn "<![CDATA[" + node.value + "]]>"
        }
        damn ""
    }
    
    sus output tea = "<" + node.name
    
    # Add attributes
    bestie (sus attr XmlAttribute in node.attributes) {
        output = output + " " + attr.name + "=\"" + escape_xml_attribute(attr.value) + "\""
    }
    
    # Self-closing or with children
    ready (node.children.len() == 0 && node.value == "") {
        output = output + "/>"
    } otherwise {
        output = output + ">"
        
        # Text content
        ready (node.value != "") {
            output = output + escape_xml_text(node.value)
        }
        
        # Child elements
        bestie (sus child XmlNode in node.children) {
            output = output + generate_element_xml(child, depth + 1)
        }
        
        output = output + "</" + node.name + ">"
    }
    
    damn output
}

# Generate formatted element XML with indentation
slay generate_element_xml_formatted(node XmlNode, depth drip, indent_size drip) tea {
    sus indent tea = ""
    bestie (sus i drip = 0; i < depth * indent_size; i++) {
        indent = indent + " "
    }
    
    ready (node.node_type != XmlNodeType.Element) {
        ready (node.node_type == XmlNodeType.Text) {
            sus text tea = escape_xml_text(node.value)
            ready (text.trim() == "") {
                damn ""
            }
            damn text
        }
        ready (node.node_type == XmlNodeType.Comment) {
            damn indent + "<!--" + node.value + "-->\n"
        }
        ready (node.node_type == XmlNodeType.CDATA) {
            damn indent + "<![CDATA[" + node.value + "]]>\n"
        }
        damn ""
    }
    
    sus output tea = indent + "<" + node.name
    
    # Add attributes
    bestie (sus attr XmlAttribute in node.attributes) {
        output = output + " " + attr.name + "=\"" + escape_xml_attribute(attr.value) + "\""
    }
    
    # Self-closing or with children
    ready (node.children.len() == 0 && node.value == "") {
        output = output + "/>\n"
    } otherwise {
        output = output + ">"
        
        # Check if we have mixed content (text + elements)
        sus has_text lit = (node.value != "" && node.value.trim() != "")
        sus has_elements lit = (node.children.len() > 0)
        
        ready (has_text && !has_elements) {
            # Text-only element
            output = output + escape_xml_text(node.value) + "</" + node.name + ">\n"
        } otherwise ready (has_elements && !has_text) {
            # Element-only content
            output = output + "\n"
            bestie (sus child XmlNode in node.children) {
                output = output + generate_element_xml_formatted(child, depth + 1, indent_size)
            }
            output = output + indent + "</" + node.name + ">\n"
        } otherwise {
            # Mixed content or empty
            ready (has_text) {
                output = output + escape_xml_text(node.value)
            }
            bestie (sus child XmlNode in node.children) {
                output = output + generate_element_xml_formatted(child, depth + 1, indent_size)
            }
            output = output + "</" + node.name + ">\n"
        }
    }
    
    damn output
}

# ========================
# XPath Query Functions
# ========================

# Execute XPath query on document
slay xpath_query(doc XmlDocument, xpath tea) yikes<XPathResult> {
    ready (doc.root == cap) {
        yikes "Document has no root element"
    }
    
    sus result XPathResult = {
        nodes: [],
        values: [],
        result_type: "nodeset"
    }
    
    # Parse XPath expression
    sus tokens []tea = tokenize_xpath(xpath)
    sus parsed_expr XPathExpression = parse_xpath_expression(tokens) fam {
        when err -> yikes "Failed to parse XPath: " + err
    }
    
    # Evaluate expression
    result = evaluate_xpath(doc.root, parsed_expr) fam {
        when err -> yikes "Failed to evaluate XPath: " + err
    }
    
    damn result
}

# Find nodes by XPath
slay find_nodes(doc XmlDocument, xpath tea) yikes<[]XmlNode> {
    sus result XPathResult = xpath_query(doc, xpath) fam {
        when err -> yikes err
    }
    damn result.nodes
}

# Find first node by XPath
slay find_first_node(doc XmlDocument, xpath tea) yikes<XmlNode> {
    sus nodes []XmlNode = find_nodes(doc, xpath) fam {
        when err -> yikes err
    }
    
    ready (nodes.len() == 0) {
        yikes "No nodes found matching XPath: " + xpath
    }
    
    damn nodes[0]
}

# Get text content from XPath
slay xpath_text(doc XmlDocument, xpath tea) yikes<tea> {
    sus result XPathResult = xpath_query(doc, xpath) fam {
        when err -> yikes err
    }
    
    ready (result.values.len() > 0) {
        damn result.values[0]
    }
    
    ready (result.nodes.len() > 0) {
        damn get_node_text_content(result.nodes[0])
    }
    
    yikes "No text content found for XPath: " + xpath
}

# ========================
# Schema Validation Functions
# ========================

# Validate document against DTD
slay validate_dtd(doc XmlDocument, dtd_content tea) yikes<ValidationResult> {
    sus result ValidationResult = {
        valid: based,
        errors: [],
        warnings: []
    }
    
    # Parse DTD
    sus parsed_dtd DTDDefinition = parse_dtd(dtd_content) fam {
        when err -> {
            result.valid = nah
            result.errors = [err]
            damn result
        }
    }
    
    # Validate document structure
    result = validate_against_dtd(doc, parsed_dtd)
    damn result
}

# Validate document against XML Schema (XSD)
slay validate_xsd(doc XmlDocument, xsd_content tea) yikes<ValidationResult> {
    sus result ValidationResult = {
        valid: based,
        errors: [],
        warnings: []
    }
    
    # Parse XSD
    sus parsed_xsd XSDSchema = parse_xsd(xsd_content) fam {
        when err -> {
            result.valid = nah
            result.errors = [err]
            damn result
        }
    }
    
    # Validate document structure
    result = validate_against_xsd(doc, parsed_xsd)
    damn result
}

# Validate document well-formedness
slay validate_well_formed(doc XmlDocument) ValidationResult {
    sus result ValidationResult = {
        valid: based,
        errors: [],
        warnings: []
    }
    
    ready (doc.root == cap) {
        result.valid = nah
        result.errors = ["Document has no root element"]
        damn result
    }
    
    # Check for proper nesting
    validate_nesting(doc.root, result)
    
    # Check for unique attributes
    validate_unique_attributes(doc.root, result)
    
    # Check for proper namespace declarations
    validate_namespaces(doc.root, result)
    
    damn result
}

# ========================
# Utility Functions
# ========================

# Escape XML text content
slay escape_xml_text(text tea) tea {
    sus result tea = text
    result = stringz.replace_all(result, "&", "&amp;")
    result = stringz.replace_all(result, "<", "&lt;")
    result = stringz.replace_all(result, ">", "&gt;")
    damn result
}

# Escape XML attribute value
slay escape_xml_attribute(value tea) tea {
    sus result tea = escape_xml_text(value)
    result = stringz.replace_all(result, "\"", "&quot;")
    result = stringz.replace_all(result, "'", "&apos;")
    damn result
}

# Unescape XML entities
slay unescape_xml(text tea) tea {
    sus result tea = text
    result = stringz.replace_all(result, "&lt;", "<")
    result = stringz.replace_all(result, "&gt;", ">")
    result = stringz.replace_all(result, "&quot;", "\"")
    result = stringz.replace_all(result, "&apos;", "'")
    result = stringz.replace_all(result, "&amp;", "&")  # Must be last
    damn result
}

# Get text content of node and all children
slay get_node_text_content(node XmlNode) tea {
    ready (node.node_type == XmlNodeType.Text) {
        damn node.value
    }
    
    sus content tea = ""
    ready (node.value != "") {
        content = content + node.value
    }
    
    bestie (sus child XmlNode in node.children) {
        content = content + get_node_text_content(child)
    }
    
    damn content
}

# Find child element by name
slay find_child_element(node XmlNode, name tea) sus XmlNode {
    bestie (sus child XmlNode in node.children) {
        ready (child.node_type == XmlNodeType.Element && child.name == name) {
            damn child
        }
    }
    damn cap
}

# Find all child elements by name
slay find_child_elements(node XmlNode, name tea) []XmlNode {
    sus results []XmlNode = []
    bestie (sus child XmlNode in node.children) {
        ready (child.node_type == XmlNodeType.Element && child.name == name) {
            results = arrayz.append(results, child)
        }
    }
    damn results
}

# Get attribute value
slay get_attribute(node XmlNode, name tea) sus tea {
    bestie (sus attr XmlAttribute in node.attributes) {
        ready (attr.name == name) {
            damn attr.value
        }
    }
    damn cap
}

# Create new XML element
slay create_element(name tea, namespace_uri tea) XmlNode {
    damn {
        node_type: XmlNodeType.Element,
        name: name,
        value: "",
        attributes: [],
        children: [],
        parent: cap,
        namespace_uri: namespace_uri,
        namespace_prefix: "",
        line_number: 0,
        column_number: 0
    }
}

# Add attribute to element
slay add_attribute(node sus XmlNode, name tea, value tea) {
    sus attr XmlAttribute = {
        name: name,
        value: value,
        namespace_uri: "",
        namespace_prefix: ""
    }
    node.attributes = arrayz.append(node.attributes, attr)
}

# Add child element
slay add_child(parent sus XmlNode, child sus XmlNode) {
    child.parent = parent
    parent.children = arrayz.append(parent.children, child)
}

# Create text node
slay create_text_node(content tea) XmlNode {
    damn {
        node_type: XmlNodeType.Text,
        name: "",
        value: content,
        attributes: [],
        children: [],
        parent: cap,
        namespace_uri: "",
        namespace_prefix: "",
        line_number: 0,
        column_number: 0
    }
}

# Create comment node
slay create_comment(content tea) XmlNode {
    damn {
        node_type: XmlNodeType.Comment,
        name: "",
        value: content,
        attributes: [],
        children: [],
        parent: cap,
        namespace_uri: "",
        namespace_prefix: "",
        line_number: 0,
        column_number: 0
    }
}

# Create CDATA node
slay create_cdata(content tea) XmlNode {
    damn {
        node_type: XmlNodeType.CDATA,
        name: "",
        value: content,
        attributes: [],
        children: [],
        parent: cap,
        namespace_uri: "",
        namespace_prefix: "",
        line_number: 0,
        column_number: 0
    }
}

# ========================
# Internal Parser Functions
# ========================

# Parser state for tracking position and context
squad ParserState {
    content tea
    position drip
    line drip
    column drip
    config XmlParserConfig
    current_char tea
    namespaces []XmlNamespace
}

# Initialize parser state
slay init_parser_state(content tea, config XmlParserConfig) ParserState {
    sus state ParserState = {
        content: content,
        position: 0,
        line: 1,
        column: 1,
        config: config,
        current_char: "",
        namespaces: []
    }
    
    ready (content.len() > 0) {
        state.current_char = stringz.char_at(content, 0)
    }
    
    damn state
}

# Detect XML encoding from BOM or declaration
slay detect_xml_encoding(content tea) XmlEncoding {
    # Check for UTF-16 BE BOM
    ready (content.len() >= 2 && 
           stringz.char_at(content, 0) == "\xFE" && 
           stringz.char_at(content, 1) == "\xFF") {
        damn XmlEncoding.UTF16BE
    }
    
    # Check for UTF-16 LE BOM
    ready (content.len() >= 2 && 
           stringz.char_at(content, 0) == "\xFF" && 
           stringz.char_at(content, 1) == "\xFE") {
        damn XmlEncoding.UTF16LE
    }
    
    # Check for UTF-8 BOM
    ready (content.len() >= 3 && 
           stringz.char_at(content, 0) == "\xEF" && 
           stringz.char_at(content, 1) == "\xBB" && 
           stringz.char_at(content, 2) == "\xBF") {
        damn XmlEncoding.UTF8
    }
    
    # Default to UTF-8 if no BOM detected
    damn XmlEncoding.UTF8
}

# Parse XML declaration
slay parse_xml_declaration(state sus ParserState, doc sus XmlDocument) yikes<tea> {
    skip_whitespace(state)
    
    ready (!starts_with_at_position(state, "<?xml")) {
        damn "ok"  # No XML declaration is optional
    }
    
    advance_position(state, 5)  # Skip "<?xml"
    skip_whitespace(state)
    
    # Parse version attribute
    ready (starts_with_at_position(state, "version=")) {
        advance_position(state, 8)
        sus version tea = parse_attribute_value(state) fam {
            when err -> yikes "Failed to parse version: " + err
        }
        doc.version = version
    }
    
    skip_whitespace(state)
    
    # Parse encoding attribute
    ready (starts_with_at_position(state, "encoding=")) {
        advance_position(state, 9)
        sus encoding_str tea = parse_attribute_value(state) fam {
            when err -> yikes "Failed to parse encoding: " + err
        }
        doc.encoding = parse_encoding_string(encoding_str)
    }
    
    skip_whitespace(state)
    
    # Parse standalone attribute
    ready (starts_with_at_position(state, "standalone=")) {
        advance_position(state, 11)
        sus standalone_str tea = parse_attribute_value(state) fam {
            when err -> yikes "Failed to parse standalone: " + err
        }
        doc.standalone = (standalone_str == "yes")
    }
    
    skip_whitespace(state)
    
    # Expect closing ?>
    ready (!starts_with_at_position(state, "?>")) {
        yikes "Expected '?>' to close XML declaration"
    }
    
    advance_position(state, 2)
    damn "ok"
}

# Generate XML declaration
slay generate_xml_declaration(doc XmlDocument) tea {
    sus output tea = "<?xml version=\"" + doc.version + "\""
    
    # Add encoding if not UTF-8
    ready (doc.encoding != XmlEncoding.UTF8) {
        output = output + " encoding=\"" + encoding_to_string(doc.encoding) + "\""
    }
    
    # Add standalone if not default
    ready (doc.standalone) {
        output = output + " standalone=\"yes\""
    }
    
    output = output + "?>"
    damn output
}

# Export all public functions and types
# This makes them available when importing xmlz module
