# CURSED Standard Library - SAX (Simple API for XML) Parser
# Event-driven streaming XML parser for large documents
# Version: 1.0.0-production

yeet "stringz"
yeet "arrayz"
yeet "vibez"

# SAX Parser Implementation
squad SaxParser {
    content tea
    position drip
    line drip
    column drip
    handler SaxHandler
    config XmlParserConfig
    namespace_stack NamespaceContext[value]
    element_stack tea[value]
    current_char tea
    buffer tea
    state SaxParserState
}

# SAX Parser States
enum SaxParserState {
    Start,
    InXmlDeclaration,
    InDoctype,
    InElement,
    InStartTag,
    InEndTag,
    InAttribute,
    InAttributeValue,
    InText,
    InComment,
    InCDATA,
    InProcessingInstruction,
    Error,
    End
}

# Namespace Context for SAX parsing
squad NamespaceContext {
    prefix tea
    uri tea
}

# SAX Content Handler Implementation
squad DefaultSaxHandler collab SaxHandler {
    # Default implementations of SAX events
    slay on_start_document() {
        vibez.spill("SAX: Document started")
    }
    
    slay on_end_document() {
        vibez.spill("SAX: Document ended")
    }
    
    slay on_start_element(name tea, attributes XmlAttribute[value]) {
        vibez.spill("SAX: Start element: " + name)
        bestie (sus attr XmlAttribute in attributes) {
            vibez.spill("  Attribute: " + attr.name + "=" + attr.value)
        }
    }
    
    slay on_end_element(name tea) {
        vibez.spill("SAX: End element: " + name)
    }
    
    slay on_characters(data tea) {
        ready (stringz.trim(data) != "") {
            vibez.spill("SAX: Characters: " + data)
        }
    }
    
    slay on_comment(data tea) {
        vibez.spill("SAX: Comment: " + data)
    }
    
    slay on_processing_instruction(target tea, data tea) {
        vibez.spill("SAX: PI: " + target + " " + data)
    }
    
    slay on_error(error XmlParseError) {
        vibez.spill("SAX: Error: " + error.message + " at line " + stringz.from_int(error.line))
    }
}

# ========================
# SAX Parser Functions
# ========================

# Parse XML with SAX handler and configuration
slay parse_xml_sax_with_config(xml_content tea, handler SaxHandler, config XmlParserConfig) yikes<tea> {
    sus parser SaxParser = {
        content: xml_content,
        position: 0,
        line: 1,
        column: 1,
        handler: handler,
        config: config,
        namespace_stack: [],
        element_stack: [],
        current_char: "",
        buffer: "",
        state: SaxParserState.Start
    }
    
    # Initialize current character
    ready (xml_content.len() > 0) {
        parser.current_char = stringz.char_at(xml_content, 0)
    }
    
    # Start parsing
    handler.on_start_document()
    
    sus result tea = parse_sax_document(parser) fam {
        when err -> {
            sus error XmlParseError = {
                message: err,
                line: parser.line,
                column: parser.column,
                error_code: 1,
                context: parser.buffer
            }
            handler.on_error(error)
            yikes err
        }
    }
    
    handler.on_end_document()
    damn result
}

# Parse entire SAX document
slay parse_sax_document(parser sus SaxParser) yikes<tea> {
    bestie (parser.position < parser.content.len() && parser.state != SaxParserState.Error) {
        ready (parser.state == SaxParserState.Start) {
            parse_sax_prolog(parser) fam {
                when err -> yikes err
            }
        } otherwise ready (parser.state == SaxParserState.InElement) {
            parse_sax_element_content(parser) fam {
                when err -> yikes err
            }
        } otherwise ready (parser.state == SaxParserState.InText) {
            parse_sax_character_data(parser) fam {
                when err -> yikes err
            }
        } otherwise ready (parser.state == SaxParserState.InComment) {
            parse_sax_comment(parser) fam {
                when err -> yikes err
            }
        } otherwise ready (parser.state == SaxParserState.InCDATA) {
            parse_sax_cdata(parser) fam {
                when err -> yikes err
            }
        } otherwise ready (parser.state == SaxParserState.InProcessingInstruction) {
            parse_sax_processing_instruction(parser) fam {
                when err -> yikes err
            }
        } otherwise {
            # Default: look for next markup
            parse_sax_next_token(parser) fam {
                when err -> yikes err
            }
        }
    }
    
    ready (parser.state == SaxParserState.Error) {
        yikes "SAX parser error"
    }
    
    damn "ok"
}

# Parse XML prolog (declaration, DTD, etc.)
slay parse_sax_prolog(parser sus SaxParser) yikes<tea> {
    sax_skip_whitespace(parser)
    
    # Check for XML declaration
    ready (sax_starts_with(parser, "<?xml")) {
        parse_sax_xml_declaration(parser) fam {
            when err -> yikes err
        }
        sax_skip_whitespace(parser)
    }
    
    # Check for DOCTYPE declaration
    ready (sax_starts_with(parser, "<!DOCTYPE")) {
        parse_sax_doctype(parser) fam {
            when err -> yikes err
        }
        sax_skip_whitespace(parser)
    }
    
    # Skip any processing instructions or comments
    bestie (sax_starts_with(parser, "<?") || sax_starts_with(parser, "<!--")) {
        ready (sax_starts_with(parser, "<?")) {
            parser.state = SaxParserState.InProcessingInstruction
            parse_sax_processing_instruction(parser) fam {
                when err -> yikes err
            }
        } otherwise {
            parser.state = SaxParserState.InComment
            parse_sax_comment(parser) fam {
                when err -> yikes err
            }
        }
        sax_skip_whitespace(parser)
    }
    
    # Ready for root element
    parser.state = SaxParserState.InElement
    damn "ok"
}

# Parse XML declaration
slay parse_sax_xml_declaration(parser sus SaxParser) yikes<tea> {
    parser.state = SaxParserState.InXmlDeclaration
    
    # Skip "<?xml"
    sax_advance(parser, 5)
    sax_skip_whitespace(parser)
    
    # Parse attributes (version, encoding, standalone)
    bestie (!sax_starts_with(parser, "?>")) {
        ready (sax_is_at_end(parser)) {
            yikes "Unexpected end of XML declaration"
        }
        
        # Skip attribute parsing for now - just advance to end
        sax_advance_char(parser)
    }
    
    # Skip "?>"
    ready (sax_starts_with(parser, "?>")) {
        sax_advance(parser, 2)
    } otherwise {
        yikes "Expected '?>' to close XML declaration"
    }
    
    parser.state = SaxParserState.Start
    damn "ok"
}

# Parse DOCTYPE declaration
slay parse_sax_doctype(parser sus SaxParser) yikes<tea> {
    parser.state = SaxParserState.InDoctype
    
    # Skip to end of DOCTYPE (simplified)
    sus depth drip = 0
    bestie (parser.position < parser.content.len()) {
        ready (parser.current_char == "<") {
            depth = depth + 1
        } otherwise ready (parser.current_char == ">") {
            depth = depth - 1
            ready (depth == 0) {
                sax_advance_char(parser)
                break
            }
        }
        sax_advance_char(parser)
    }
    
    parser.state = SaxParserState.Start
    damn "ok"
}

# Parse element content (main parsing loop)
slay parse_sax_element_content(parser sus SaxParser) yikes<tea> {
    sax_skip_whitespace(parser)
    
    ready (sax_is_at_end(parser)) {
        parser.state = SaxParserState.End
        damn "ok"
    }
    
    ready (parser.current_char == "<") {
        # Start of markup
        ready (sax_starts_with(parser, "<!--")) {
            parser.state = SaxParserState.InComment
            parse_sax_comment(parser) fam {
                when err -> yikes err
            }
        } otherwise ready (sax_starts_with(parser, "<![CDATA[")) {
            parser.state = SaxParserState.InCDATA
            parse_sax_cdata(parser) fam {
                when err -> yikes err
            }
        } otherwise ready (sax_starts_with(parser, "<?")) {
            parser.state = SaxParserState.InProcessingInstruction
            parse_sax_processing_instruction(parser) fam {
                when err -> yikes err
            }
        } otherwise ready (sax_starts_with(parser, "</")) {
            # End tag
            parse_sax_end_tag(parser) fam {
                when err -> yikes err
            }
        } otherwise {
            # Start tag
            parse_sax_start_tag(parser) fam {
                when err -> yikes err
            }
        }
    } otherwise {
        # Character data
        parser.state = SaxParserState.InText
        parse_sax_character_data(parser) fam {
            when err -> yikes err
        }
    }
    
    # Continue parsing
    parser.state = SaxParserState.InElement
    damn "ok"
}

# Parse start tag
slay parse_sax_start_tag(parser sus SaxParser) yikes<tea> {
    parser.state = SaxParserState.InStartTag
    
    # Skip '<'
    sax_advance_char(parser)
    
    # Parse element name
    sus element_name tea = parse_sax_name(parser) fam {
        when err -> yikes "Failed to parse element name: " + err
    }
    
    # Parse attributes
    sus attributes XmlAttribute[value] = []
    sax_skip_whitespace(parser)
    
    bestie (parser.current_char != ">" && parser.current_char != "/" && !sax_is_at_end(parser)) {
        sus attr XmlAttribute = parse_sax_attribute(parser) fam {
            when err -> yikes "Failed to parse attribute: " + err
        }
        attributes = arrayz.append(attributes, attr)
        sax_skip_whitespace(parser)
    }
    
    # Check for self-closing tag
    sus is_empty lit = nah
    ready (parser.current_char == "/") {
        is_empty = based
        sax_advance_char(parser)
    }
    
    # Expect '>'
    ready (parser.current_char != ">") {
        yikes "Expected '>' after element name"
    }
    sax_advance_char(parser)
    
    # Fire start element event
    parser.handler.on_start_element(element_name, attributes)
    
    ready (is_empty) {
        # Self-closing tag - fire end element event immediately
        parser.handler.on_end_element(element_name)
    } otherwise {
        # Push element onto stack
        parser.element_stack = arrayz.append(parser.element_stack, element_name)
    }
    
    parser.state = SaxParserState.InElement
    damn "ok"
}

# Parse end tag
slay parse_sax_end_tag(parser sus SaxParser) yikes<tea> {
    parser.state = SaxParserState.InEndTag
    
    # Skip "</"
    sax_advance(parser, 2)
    
    # Parse element name
    sus element_name tea = parse_sax_name(parser) fam {
        when err -> yikes "Failed to parse end tag name: " + err
    }
    
    # Validate element stack
    ready (parser.element_stack.len() == 0) {
        yikes "Unexpected end tag: " + element_name
    }
    
    sus expected tea = parser.element_stack[parser.element_stack.len() - 1]
    ready (element_name != expected) {
        yikes "Mismatched end tag: expected '" + expected + "', got '" + element_name + "'"
    }
    
    # Pop from stack
    parser.element_stack = arrayz.slice(parser.element_stack, 0, parser.element_stack.len() - 1)
    
    sax_skip_whitespace(parser)
    
    # Expect '>'
    ready (parser.current_char != ">") {
        yikes "Expected '>' after end tag name"
    }
    sax_advance_char(parser)
    
    # Fire end element event
    parser.handler.on_end_element(element_name)
    
    # Check if we've closed the root element
    ready (parser.element_stack.len() == 0) {
        parser.state = SaxParserState.End
    } otherwise {
        parser.state = SaxParserState.InElement
    }
    
    damn "ok"
}

# Parse attribute
slay parse_sax_attribute(parser sus SaxParser) yikes<XmlAttribute> {
    parser.state = SaxParserState.InAttribute
    
    # Parse attribute name
    sus attr_name tea = parse_sax_name(parser) fam {
        when err -> yikes err
    }
    
    sax_skip_whitespace(parser)
    
    # Expect '='
    ready (parser.current_char != "=") {
        yikes "Expected '=' after attribute name"
    }
    sax_advance_char(parser)
    
    sax_skip_whitespace(parser)
    
    # Parse attribute value
    sus attr_value tea = parse_sax_attribute_value(parser) fam {
        when err -> yikes err
    }
    
    damn {
        name: attr_name,
        value: attr_value,
        namespace_uri: "",
        namespace_prefix: ""
    }
}

# Parse attribute value (quoted string)
slay parse_sax_attribute_value(parser sus SaxParser) yikes<tea> {
    parser.state = SaxParserState.InAttributeValue
    
    # Expect quote
    ready (parser.current_char != "\"" && parser.current_char != "'") {
        yikes "Expected quote to start attribute value"
    }
    
    sus quote tea = parser.current_char
    sax_advance_char(parser)
    
    sus value tea = ""
    bestie (parser.current_char != quote && !sax_is_at_end(parser)) {
        # Handle entity references
        ready (parser.current_char == "&") {
            sus entity tea = parse_sax_entity_reference(parser) fam {
                when err -> yikes err
            }
            value = value + entity
        } otherwise {
            value = value + parser.current_char
            sax_advance_char(parser)
        }
    }
    
    # Expect closing quote
    ready (parser.current_char != quote) {
        yikes "Expected closing quote for attribute value"
    }
    sax_advance_char(parser)
    
    damn value
}

# Parse character data (text content)
slay parse_sax_character_data(parser sus SaxParser) yikes<tea> {
    parser.state = SaxParserState.InText
    
    sus text tea = ""
    bestie (parser.current_char != "<" && !sax_is_at_end(parser)) {
        ready (parser.current_char == "&") {
            # Entity reference
            sus entity tea = parse_sax_entity_reference(parser) fam {
                when err -> yikes err
            }
            text = text + entity
        } otherwise {
            text = text + parser.current_char
            sax_advance_char(parser)
        }
    }
    
    # Fire characters event if we have non-empty text
    ready (text != "") {
        parser.handler.on_characters(text)
    }
    
    parser.state = SaxParserState.InElement
    damn "ok"
}

# Parse comment
slay parse_sax_comment(parser sus SaxParser) yikes<tea> {
    parser.state = SaxParserState.InComment
    
    # Skip "<!--"
    sax_advance(parser, 4)
    
    sus comment tea = ""
    bestie (!sax_starts_with(parser, "-->") && !sax_is_at_end(parser)) {
        comment = comment + parser.current_char
        sax_advance_char(parser)
    }
    
    # Skip "-->"
    ready (sax_starts_with(parser, "-->")) {
        sax_advance(parser, 3)
    } otherwise {
        yikes "Expected '-->' to close comment"
    }
    
    # Fire comment event
    parser.handler.on_comment(comment)
    
    parser.state = SaxParserState.InElement
    damn "ok"
}

# Parse CDATA section
slay parse_sax_cdata(parser sus SaxParser) yikes<tea> {
    parser.state = SaxParserState.InCDATA
    
    # Skip "<![CDATA["
    sax_advance(parser, 9)
    
    sus cdata tea = ""
    bestie (!sax_starts_with(parser, "]]>") && !sax_is_at_end(parser)) {
        cdata = cdata + parser.current_char
        sax_advance_char(parser)
    }
    
    # Skip "]]>"
    ready (sax_starts_with(parser, "]]>")) {
        sax_advance(parser, 3)
    } otherwise {
        yikes "Expected ']]>' to close CDATA section"
    }
    
    # Fire characters event with CDATA content
    parser.handler.on_characters(cdata)
    
    parser.state = SaxParserState.InElement
    damn "ok"
}

# Parse processing instruction
slay parse_sax_processing_instruction(parser sus SaxParser) yikes<tea> {
    parser.state = SaxParserState.InProcessingInstruction
    
    # Skip "<?"
    sax_advance(parser, 2)
    
    # Parse target
    sus target tea = parse_sax_name(parser) fam {
        when err -> yikes "Failed to parse PI target: " + err
    }
    
    sax_skip_whitespace(parser)
    
    # Parse data (everything until "?>")
    sus data tea = ""
    bestie (!sax_starts_with(parser, "?>") && !sax_is_at_end(parser)) {
        data = data + parser.current_char
        sax_advance_char(parser)
    }
    
    # Skip "?>"
    ready (sax_starts_with(parser, "?>")) {
        sax_advance(parser, 2)
    } otherwise {
        yikes "Expected '?>' to close processing instruction"
    }
    
    # Fire PI event
    parser.handler.on_processing_instruction(target, stringz.trim(data))
    
    parser.state = SaxParserState.InElement
    damn "ok"
}

# ========================
# SAX Parser Utility Functions
# ========================

# Parse XML name (element/attribute name)
slay parse_sax_name(parser sus SaxParser) yikes<tea> {
    sus name tea = ""
    
    # First character must be letter or underscore
    ready (!sax_is_name_start_char(parser.current_char)) {
        yikes "Invalid character to start XML name: " + parser.current_char
    }
    
    bestie (sax_is_name_char(parser.current_char) && !sax_is_at_end(parser)) {
        name = name + parser.current_char
        sax_advance_char(parser)
    }
    
    ready (name == "") {
        yikes "Empty XML name"
    }
    
    damn name
}

# Parse entity reference
slay parse_sax_entity_reference(parser sus SaxParser) yikes<tea> {
    ready (parser.current_char != "&") {
        yikes "Expected '&' to start entity reference"
    }
    
    sax_advance_char(parser)  # Skip '&'
    
    sus entity_name tea = ""
    bestie (parser.current_char != ";" && !sax_is_at_end(parser)) {
        entity_name = entity_name + parser.current_char
        sax_advance_char(parser)
    }
    
    ready (parser.current_char != ";") {
        yikes "Expected ';' to end entity reference"
    }
    sax_advance_char(parser)  # Skip ';'
    
    # Resolve standard entities
    ready (entity_name == "amp") {
        damn "&"
    } otherwise ready (entity_name == "lt") {
        damn "<"
    } otherwise ready (entity_name == "gt") {
        damn ">"
    } otherwise ready (entity_name == "quot") {
        damn "\""
    } otherwise ready (entity_name == "apos") {
        damn "'"
    }
    
    # For now, return the entity reference as-is for custom entities
    damn "&" + entity_name + ";"
}

# Advance parser position by one character
slay sax_advance_char(parser sus SaxParser) {
    ready (parser.position >= parser.content.len()) {
        damn
    }
    
    ready (parser.current_char == "\n") {
        parser.line = parser.line + 1
        parser.column = 1
    } otherwise {
        parser.column = parser.column + 1
    }
    
    parser.position = parser.position + 1
    
    ready (parser.position < parser.content.len()) {
        parser.current_char = stringz.char_at(parser.content, parser.position)
    } otherwise {
        parser.current_char = ""
    }
}

# Advance parser position by multiple characters
slay sax_advance(parser sus SaxParser, count drip) {
    bestie (sus i drip = 0; i < count; i++) {
        sax_advance_char(parser)
    }
}

# Skip whitespace characters
slay sax_skip_whitespace(parser sus SaxParser) {
    bestie (sax_is_whitespace(parser.current_char) && !sax_is_at_end(parser)) {
        sax_advance_char(parser)
    }
}

# Check if parser is at end of content
slay sax_is_at_end(parser SaxParser) lit {
    damn (parser.position >= parser.content.len())
}

# Check if content starts with string at current position
slay sax_starts_with(parser SaxParser, prefix tea) lit {
    ready (parser.position + prefix.len() > parser.content.len()) {
        damn nah
    }
    
    bestie (sus i drip = 0; i < prefix.len(); i++) {
        ready (stringz.char_at(parser.content, parser.position + i) != stringz.char_at(prefix, i)) {
            damn nah
        }
    }
    
    damn based
}

# Check if character is whitespace
slay sax_is_whitespace(char tea) lit {
    damn (char == " " || char == "\t" || char == "\n" || char == "\r")
}

# Check if character can start XML name
slay sax_is_name_start_char(char tea) lit {
    # Simplified - should include full XML name character ranges
    damn (char >= "A" && char <= "Z") || (char >= "a" && char <= "z") || char == "_" || char == ":"
}

# Check if character can be in XML name
slay sax_is_name_char(char tea) lit {
    damn sax_is_name_start_char(char) || (char >= "0" && char <= "9") || char == "-" || char == "."
}

# Parse next token (dispatch function)
slay parse_sax_next_token(parser sus SaxParser) yikes<tea> {
    sax_skip_whitespace(parser)
    
    ready (sax_is_at_end(parser)) {
        parser.state = SaxParserState.End
        damn "ok"
    }
    
    ready (parser.current_char == "<") {
        # Determine what kind of markup this is
        ready (sax_starts_with(parser, "<!--")) {
            parser.state = SaxParserState.InComment
        } otherwise ready (sax_starts_with(parser, "<![CDATA[")) {
            parser.state = SaxParserState.InCDATA
        } otherwise ready (sax_starts_with(parser, "<?")) {
            parser.state = SaxParserState.InProcessingInstruction
        } otherwise ready (sax_starts_with(parser, "</")) {
            parser.state = SaxParserState.InEndTag
        } otherwise {
            parser.state = SaxParserState.InStartTag
        }
    } otherwise {
        # Character data
        parser.state = SaxParserState.InText
    }
    
    damn "ok"
}
