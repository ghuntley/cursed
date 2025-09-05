# CURSED Standard Library - XML Schema Validation
# DTD, XSD, RelaxNG, and Schematron support
# Version: 1.0.0-production

yeet "stringz"
yeet "arrayz"
yeet "mathz"
yeet "regexz"

# DTD Definition Structures
squad DTDDefinition {
    name tea
    elements map[tea]DTDElement
    attributes map[tea]DTDAttribute[value]
    entities map[tea]tea
    notations map[tea]DTDNotation
    parameter_entities map[tea]tea
}

squad DTDElement {
    name tea
    content_model tea
    content_type tea  # "EMPTY", "ANY", "PCDATA", "ELEMENT"
    children tea[value]
    occurrence tea    # "", "?", "*", "+"
}

squad DTDAttribute {
    name tea
    element_name tea
    attr_type tea     # "CDATA", "ID", "IDREF", "IDREFS", "NMTOKEN", "NMTOKENS", etc.
    default_decl tea  # "#REQUIRED", "#IMPLIED", "#FIXED", or default value
    default_value tea
    allowed_values tea[value]  # For enumerated types
}

squad DTDNotation {
    name tea
    public_id tea
    system_id tea
}

# XML Schema (XSD) Structures
squad XSDSchema {
    target_namespace tea
    element_form_default tea
    attribute_form_default tea
    elements map[tea]XSDElement
    types map[tea]XSDType
    groups map[tea]XSDGroup
    attribute_groups map[tea]XSDAttributeGroup
    namespaces map[tea]tea
}

squad XSDElement {
    name tea
    type_name tea
    min_occurs drip
    max_occurs drip  # -1 for unbounded
    nillable lit
    default_value tea
    fixed_value tea
    form tea
    substitution_group tea
    complex_type sus XSDComplexType
    simple_type sus XSDSimpleType
}

squad XSDType {
    name tea
    base_type tea
    derivation tea  # "extension", "restriction"
    is_complex lit
    complex_type sus XSDComplexType
    simple_type sus XSDSimpleType
}

squad XSDComplexType {
    name tea
    content_type tea  # "empty", "simple", "element", "mixed"
    sequence XSDElement[value]
    choice XSDElement[value]
    all XSDElement[value]
    attributes XSDAttribute[value]
    attribute_groups tea[value]
    base_type tea
    derivation tea
}

squad XSDSimpleType {
    name tea
    base_type tea
    derivation tea
    restrictions XSDRestrictions
    list_item_type tea
    union_types tea[value]
}

squad XSDRestrictions {
    min_length drip
    max_length drip
    length drip
    pattern tea
    enumeration tea[value]
    max_inclusive tea
    max_exclusive tea
    min_inclusive tea
    min_exclusive tea
    total_digits drip
    fraction_digits drip
    white_space tea
}

squad XSDAttribute {
    name tea
    type_name tea
    use tea          # "required", "optional", "prohibited"
    default_value tea
    fixed_value tea
    form tea
    simple_type sus XSDSimpleType
}

squad XSDGroup {
    name tea
    min_occurs drip
    max_occurs drip
    sequence XSDElement[value]
    choice XSDElement[value]
    all XSDElement[value]
}

squad XSDAttributeGroup {
    name tea
    attributes XSDAttribute[value]
}

# ========================
# DTD Parsing Functions
# ========================

# Parse DTD from string
slay parse_dtd(dtd_content tea) yikes<DTDDefinition> {
    sus dtd DTDDefinition = {
        name: "",
        elements: {},
        attributes: {},
        entities: {},
        notations: {},
        parameter_entities: {}
    }
    
    sus lines tea[value] = stringz.split(dtd_content, "\n")
    
    bestie (sus line tea in lines) {
        sus trimmed tea = stringz.trim(line)
        ready (trimmed == "" || stringz.starts_with(trimmed, "<!--")) {
            continue
        }
        
        ready (stringz.starts_with(trimmed, "<!ELEMENT")) {
            parse_dtd_element(trimmed, dtd) fam {
                when err -> yikes "Failed to parse DTD element: " + err
            }
        } otherwise ready (stringz.starts_with(trimmed, "<!ATTLIST")) {
            parse_dtd_attlist(trimmed, dtd) fam {
                when err -> yikes "Failed to parse DTD attribute list: " + err
            }
        } otherwise ready (stringz.starts_with(trimmed, "<!ENTITY")) {
            parse_dtd_entity(trimmed, dtd) fam {
                when err -> yikes "Failed to parse DTD entity: " + err
            }
        } otherwise ready (stringz.starts_with(trimmed, "<!NOTATION")) {
            parse_dtd_notation(trimmed, dtd) fam {
                when err -> yikes "Failed to parse DTD notation: " + err
            }
        }
    }
    
    damn dtd
}

# Parse DTD element declaration
slay parse_dtd_element(declaration tea, dtd sus DTDDefinition) yikes<tea> {
    # Extract element name and content model
    # <!ELEMENT element-name content-model>
    sus parts tea[value] = stringz.split(declaration, " ")
    ready (parts.len() < 3) {
        yikes "Invalid DTD element declaration: " + declaration
    }
    
    sus element_name tea = parts[1]
    sus content_start drip = stringz.index_of(declaration, parts[1]) + parts[1].len() + 1
    sus content_end drip = stringz.last_index_of(declaration, ">")
    sus content_model tea = stringz.trim(stringz.substring(declaration, content_start, content_end - content_start))
    
    sus element DTDElement = {
        name: element_name,
        content_model: content_model,
        content_type: determine_content_type(content_model),
        children: extract_child_elements(content_model),
        occurrence: extract_occurrence(content_model)
    }
    
    dtd.elements[element_name] = element
    damn "ok"
}

# Parse DTD attribute list declaration
slay parse_dtd_attlist(declaration tea, dtd sus DTDDefinition) yikes<tea> {
    # <!ATTLIST element-name attribute-definitions>
    sus parts tea[value] = stringz.split(declaration, " ")
    ready (parts.len() < 3) {
        yikes "Invalid DTD attribute list declaration: " + declaration
    }
    
    sus element_name tea = parts[1]
    sus attr_start drip = stringz.index_of(declaration, element_name) + element_name.len()
    sus attr_end drip = stringz.last_index_of(declaration, ">")
    sus attr_content tea = stringz.trim(stringz.substring(declaration, attr_start, attr_end - attr_start))
    
    sus attributes DTDAttribute[value] = parse_dtd_attributes(attr_content, element_name) fam {
        when err -> yikes err
    }
    
    dtd.attributes[element_name] = attributes
    damn "ok"
}

# Parse DTD entity declaration
slay parse_dtd_entity(declaration tea, dtd sus DTDDefinition) yikes<tea> {
    # <!ENTITY entity-name "entity-value">
    # <!ENTITY % entity-name "entity-value">
    
    sus is_parameter lit = stringz.contains(declaration, "<!ENTITY %")
    sus start_pos drip = ready (is_parameter) {
        damn stringz.index_of(declaration, "%") + 1
    } otherwise {
        damn stringz.index_of(declaration, "ENTITY") + 6
    }
    
    sus content tea = stringz.trim(stringz.substring(declaration, start_pos, stringz.last_index_of(declaration, ">") - start_pos))
    sus parts tea[value] = stringz.split(content, " ")
    
    ready (parts.len() < 2) {
        yikes "Invalid DTD entity declaration: " + declaration
    }
    
    sus entity_name tea = parts[0]
    sus entity_value tea = extract_quoted_value(stringz.join(arrayz.slice(parts, 1), " "))
    
    ready (is_parameter) {
        dtd.parameter_entities[entity_name] = entity_value
    } otherwise {
        dtd.entities[entity_name] = entity_value
    }
    
    damn "ok"
}

# ========================
# XSD Parsing Functions
# ========================

# Parse XML Schema from string
slay parse_xsd(xsd_content tea) yikes<XSDSchema> {
    # First parse as XML document
    sus doc XmlDocument = parse_xml_dom(xsd_content) fam {
        when err -> yikes "Failed to parse XSD as XML: " + err
    }
    
    ready (doc.root == cap || doc.root.name != "schema") {
        yikes "Invalid XSD: root element must be 'schema'"
    }
    
    sus schema XSDSchema = {
        target_namespace: get_attribute(doc.root, "targetNamespace") fam { when _ -> damn "" },
        element_form_default: get_attribute(doc.root, "elementFormDefault") fam { when _ -> damn "unqualified" },
        attribute_form_default: get_attribute(doc.root, "attributeFormDefault") fam { when _ -> damn "unqualified" },
        elements: {},
        types: {},
        groups: {},
        attribute_groups: {},
        namespaces: {}
    }
    
    # Parse namespace declarations
    bestie (sus attr XmlAttribute in doc.root.attributes) {
        ready (stringz.starts_with(attr.name, "xmlns:")) {
            sus prefix tea = stringz.substring(attr.name, 6)
            schema.namespaces[prefix] = attr.value
        } otherwise ready (attr.name == "xmlns") {
            schema.namespaces[""] = attr.value
        }
    }
    
    # Parse schema elements
    bestie (sus child XmlNode in doc.root.children) {
        ready (child.node_type == XmlNodeType.Element) {
            ready (child.name == "element") {
                parse_xsd_element(child, schema) fam {
                    when err -> yikes "Failed to parse XSD element: " + err
                }
            } otherwise ready (child.name == "complexType" || child.name == "simpleType") {
                parse_xsd_type(child, schema) fam {
                    when err -> yikes "Failed to parse XSD type: " + err
                }
            } otherwise ready (child.name == "group") {
                parse_xsd_group(child, schema) fam {
                    when err -> yikes "Failed to parse XSD group: " + err
                }
            } otherwise ready (child.name == "attributeGroup") {
                parse_xsd_attribute_group(child, schema) fam {
                    when err -> yikes "Failed to parse XSD attribute group: " + err
                }
            }
        }
    }
    
    damn schema
}

# Parse XSD element
slay parse_xsd_element(element_node XmlNode, schema sus XSDSchema) yikes<tea> {
    sus name tea = get_attribute(element_node, "name") fam {
        when _ -> yikes "XSD element must have name attribute"
    }
    
    sus element XSDElement = {
        name: name,
        type_name: get_attribute(element_node, "type") fam { when _ -> damn "" },
        min_occurs: parse_occurs(get_attribute(element_node, "minOccurs") fam { when _ -> damn "1" }),
        max_occurs: parse_max_occurs(get_attribute(element_node, "maxOccurs") fam { when _ -> damn "1" }),
        nillable: (get_attribute(element_node, "nillable") fam { when _ -> damn "false" }) == "true",
        default_value: get_attribute(element_node, "default") fam { when _ -> damn "" },
        fixed_value: get_attribute(element_node, "fixed") fam { when _ -> damn "" },
        form: get_attribute(element_node, "form") fam { when _ -> damn "" },
        substitution_group: get_attribute(element_node, "substitutionGroup") fam { when _ -> damn "" },
        complex_type: cap,
        simple_type: cap
    }
    
    # Parse inline type definitions
    bestie (sus child XmlNode in element_node.children) {
        ready (child.node_type == XmlNodeType.Element) {
            ready (child.name == "complexType") {
                element.complex_type = parse_xsd_complex_type(child, schema) fam {
                    when err -> yikes err
                }
            } otherwise ready (child.name == "simpleType") {
                element.simple_type = parse_xsd_simple_type(child, schema) fam {
                    when err -> yikes err
                }
            }
        }
    }
    
    schema.elements[name] = element
    damn "ok"
}

# ========================
# Validation Functions
# ========================

# Validate document against DTD
slay validate_against_dtd(doc XmlDocument, dtd DTDDefinition) ValidationResult {
    sus result ValidationResult = {
        valid: based,
        errors: [],
        warnings: []
    }
    
    ready (doc.root == cap) {
        result.valid = nah
        result.errors = arrayz.append(result.errors, "Document has no root element")
        damn result
    }
    
    # Validate root element exists in DTD
    ready (!(doc.root.name in dtd.elements)) {
        result.valid = nah
        result.errors = arrayz.append(result.errors, "Root element '" + doc.root.name + "' not declared in DTD")
        damn result
    }
    
    # Validate element structure recursively
    validate_element_against_dtd(doc.root, dtd, result)
    
    damn result
}

# Validate element against DTD recursively
slay validate_element_against_dtd(element XmlNode, dtd DTDDefinition, result sus ValidationResult) {
    ready (element.node_type != XmlNodeType.Element) {
        damn
    }
    
    # Check if element is declared
    ready (!(element.name in dtd.elements)) {
        result.valid = nah
        result.errors = arrayz.append(result.errors, "Element '" + element.name + "' not declared in DTD")
        damn
    }
    
    sus dtd_element DTDElement = dtd.elements[element.name]
    
    # Validate content model
    ready (dtd_element.content_type == "EMPTY") {
        ready (element.children.len() > 0 || element.value != "") {
            result.valid = nah
            result.errors = arrayz.append(result.errors, "Element '" + element.name + "' should be empty")
        }
    } otherwise ready (dtd_element.content_type == "PCDATA") {
        # Check for text-only content
        bestie (sus child XmlNode in element.children) {
            ready (child.node_type == XmlNodeType.Element) {
                result.valid = nah
                result.errors = arrayz.append(result.errors, "Element '" + element.name + "' should contain only text")
            }
        }
    } otherwise ready (dtd_element.content_type == "ELEMENT") {
        # Validate child elements match content model
        validate_content_model(element, dtd_element, result)
    }
    
    # Validate attributes
    ready (element.name in dtd.attributes) {
        sus required_attrs DTDAttribute[value] = dtd.attributes[element.name]
        validate_attributes_against_dtd(element, required_attrs, result)
    }
    
    # Recursively validate children
    bestie (sus child XmlNode in element.children) {
        validate_element_against_dtd(child, dtd, result)
    }
}

# Validate document against XSD
slay validate_against_xsd(doc XmlDocument, schema XSDSchema) ValidationResult {
    sus result ValidationResult = {
        valid: based,
        errors: [],
        warnings: []
    }
    
    ready (doc.root == cap) {
        result.valid = nah
        result.errors = arrayz.append(result.errors, "Document has no root element")
        damn result
    }
    
    # Find root element in schema
    ready (!(doc.root.name in schema.elements)) {
        result.valid = nah
        result.errors = arrayz.append(result.errors, "Root element '" + doc.root.name + "' not declared in schema")
        damn result
    }
    
    # Validate element structure recursively
    validate_element_against_xsd(doc.root, schema, result)
    
    damn result
}

# Validate element against XSD recursively
slay validate_element_against_xsd(element XmlNode, schema XSDSchema, result sus ValidationResult) {
    ready (element.node_type != XmlNodeType.Element) {
        damn
    }
    
    # Find element declaration
    ready (!(element.name in schema.elements)) {
        result.valid = nah
        result.errors = arrayz.append(result.errors, "Element '" + element.name + "' not declared in schema")
        damn
    }
    
    sus xsd_element XSDElement = schema.elements[element.name]
    
    # Validate type
    ready (xsd_element.type_name != "") {
        # Validate against named type
        ready (xsd_element.type_name in schema.types) {
            sus xsd_type XSDType = schema.types[xsd_element.type_name]
            validate_element_against_type(element, xsd_type, schema, result)
        } otherwise {
            # Check built-in types
            validate_element_against_builtin_type(element, xsd_element.type_name, result)
        }
    } otherwise ready (xsd_element.complex_type != cap) {
        # Validate against inline complex type
        validate_element_against_complex_type(element, xsd_element.complex_type, schema, result)
    } otherwise ready (xsd_element.simple_type != cap) {
        # Validate against inline simple type
        validate_element_against_simple_type(element, xsd_element.simple_type, result)
    }
    
    # Recursively validate children
    bestie (sus child XmlNode in element.children) {
        validate_element_against_xsd(child, schema, result)
    }
}

# ========================
# Utility Functions
# ========================

# Determine DTD content type from content model
slay determine_content_type(content_model tea) tea {
    ready (content_model == "EMPTY") {
        damn "EMPTY"
    }
    ready (content_model == "ANY") {
        damn "ANY"  
    }
    ready (stringz.contains(content_model, "#PCDATA")) {
        damn "PCDATA"
    }
    damn "ELEMENT"
}

# Extract child elements from DTD content model
slay extract_child_elements(content_model tea) tea[value]{
    ready (content_model == "EMPTY" || content_model == "ANY") {
        damn []
    }
    
    ready (stringz.contains(content_model, "#PCDATA")) {
        damn []  # Mixed content, handle separately
    }
    
    # Parse element names from content model
    # This is simplified - real DTD parsing would need full grammar
    sus elements tea[value] = []
    sus cleaned tea = stringz.replace_all(content_model, "(", "")
    cleaned = stringz.replace_all(cleaned, ")", "")
    cleaned = stringz.replace_all(cleaned, "*", "")
    cleaned = stringz.replace_all(cleaned, "+", "")
    cleaned = stringz.replace_all(cleaned, "?", "")
    
    sus parts tea[value] = stringz.split(cleaned, ",")
    bestie (sus part tea in parts) {
        sus trimmed tea = stringz.trim(part)
        ready (trimmed != "" && !stringz.contains(trimmed, "|")) {
            elements = arrayz.append(elements, trimmed)
        }
    }
    
    damn elements
}

# Extract quoted value from DTD declaration
slay extract_quoted_value(text tea) tea {
    sus start drip = -1
    sus quote_char tea = ""
    
    bestie (sus i drip = 0; i < text.len(); i++) {
        sus char tea = stringz.char_at(text, i)
        ready (char == "\"" || char == "'") {
            ready (start == -1) {
                start = i + 1
                quote_char = char
            } otherwise ready (char == quote_char) {
                damn stringz.substring(text, start, i - start)
            }
        }
    }
    
    damn text  # Fallback if no quotes found
}

# Parse occurs value (minOccurs/maxOccurs)
slay parse_occurs(occurs_str tea) drip {
    ready (occurs_str == "") {
        damn 1
    }
    damn stringz.to_int(occurs_str) fam { when _ -> damn 1 }
}

# Parse maxOccurs value (handle "unbounded")
slay parse_max_occurs(max_occurs_str tea) drip {
    ready (max_occurs_str == "unbounded") {
        damn -1
    }
    damn parse_occurs(max_occurs_str)
}
