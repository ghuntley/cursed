# CURSED JSON Streaming Parser
# High-performance streaming JSON parser for large documents
# Supports SAX-style event-driven parsing

yeet "vibez"
yeet "stringz"
yeet "error_core"

# JSON Event Types
enum JsonEventType {
    StartObject,    # {
    EndObject,      # }
    StartArray,     # [
    EndArray,       # ]
    PropertyName,   # "key":
    StringValue,    # "value"
    NumberValue,    # 123, 45.67
    BooleanValue,   # true, false
    NullValue,      # null
    Error,          # Parse error
    EndDocument     # End of document
}

# JSON Streaming Event
squad JsonEvent {
    event_type JsonEventType
    value tea
    line drip
    column drip
    depth drip
}

# JSON Stream Parser State
squad JsonStreamParser {
    content tea
    position drip
    line drip
    column drip
    current_char tea
    depth drip
    in_string lit
    escaped lit
    buffer tea
    buffer_start drip
    max_buffer_size drip
}

# JSON Event Handler Interface
collab JsonEventHandler {
    slay on_start_object(depth drip)
    slay on_end_object(depth drip)
    slay on_start_array(depth drip)
    slay on_end_array(depth drip)
    slay on_property_name(name tea, depth drip)
    slay on_string_value(value tea, depth drip)
    slay on_number_value(value tea, depth drip)
    slay on_boolean_value(value lit, depth drip)
    slay on_null_value(depth drip)
    slay on_error(message tea, line drip, column drip)
}

# ========================
# Streaming Parser Functions
# ========================

# Create streaming parser
slay create_json_stream_parser(content tea, max_buffer_size drip) JsonStreamParser {
    sus parser JsonStreamParser = {
        content: content,
        position: 0,
        line: 1,
        column: 1,
        current_char: "",
        depth: 0,
        in_string: cap,
        escaped: cap,
        buffer: "",
        buffer_start: 0,
        max_buffer_size: max_buffer_size
    }
    
    ready (content.len() > 0) {
        parser.current_char = stringz.char_at(content, 0)
    }
    
    damn parser
}

# Parse JSON stream with event handler
slay parse_json_stream(content tea, handler JsonEventHandler) yikes<tea> {
    sus parser JsonStreamParser = create_json_stream_parser(content, 8192)
    
    bestie (parser.position < parser.content.len()) {
        sus event JsonEvent = get_next_json_event(&parser) fam {
            when err -> {
                handler.on_error(err, parser.line, parser.column)
                yikes err
            }
        }
        
        # Dispatch event to handler
        ready (event.event_type == JsonEventType.StartObject) {
            handler.on_start_object(event.depth)
        } otherwise ready (event.event_type == JsonEventType.EndObject) {
            handler.on_end_object(event.depth)
        } otherwise ready (event.event_type == JsonEventType.StartArray) {
            handler.on_start_array(event.depth)
        } otherwise ready (event.event_type == JsonEventType.EndArray) {
            handler.on_end_array(event.depth)
        } otherwise ready (event.event_type == JsonEventType.PropertyName) {
            handler.on_property_name(event.value, event.depth)
        } otherwise ready (event.event_type == JsonEventType.StringValue) {
            handler.on_string_value(event.value, event.depth)
        } otherwise ready (event.event_type == JsonEventType.NumberValue) {
            handler.on_number_value(event.value, event.depth)
        } otherwise ready (event.event_type == JsonEventType.BooleanValue) {
            handler.on_boolean_value(event.value == "true", event.depth)
        } otherwise ready (event.event_type == JsonEventType.NullValue) {
            handler.on_null_value(event.depth)
        } otherwise ready (event.event_type == JsonEventType.Error) {
            handler.on_error(event.value, event.line, event.column)
            yikes event.value
        } otherwise ready (event.event_type == JsonEventType.EndDocument) {
            break
        }
    }
    
    damn "ok"
}

# Parse JSON stream with iterator-style interface
slay parse_json_stream_iterator(content tea) yikes<JsonEventIterator> {
    sus parser JsonStreamParser = create_json_stream_parser(content, 8192)
    sus iterator JsonEventIterator = {
        parser: parser,
        has_next: based,
        current_event: create_end_document_event()
    }
    damn iterator
}

# Get next JSON event from parser
slay get_next_json_event(parser sus JsonStreamParser) yikes<JsonEvent> {
    skip_json_whitespace(parser)
    
    ready (parser.position >= parser.content.len()) {
        damn create_end_document_event()
    }
    
    sus ch tea = parser.current_char
    
    ready (ch == "{") {
        advance_parser_position(parser)
        parser.depth = parser.depth + 1
        damn create_json_event(JsonEventType.StartObject, "", parser)
    } otherwise ready (ch == "}") {
        advance_parser_position(parser)
        parser.depth = parser.depth - 1
        damn create_json_event(JsonEventType.EndObject, "", parser)
    } otherwise ready (ch == "[") {
        advance_parser_position(parser)
        parser.depth = parser.depth + 1
        damn create_json_event(JsonEventType.StartArray, "", parser)
    } otherwise ready (ch == "]") {
        advance_parser_position(parser)
        parser.depth = parser.depth - 1
        damn create_json_event(JsonEventType.EndArray, "", parser)
    } otherwise ready (ch == "\"") {
        sus string_value tea = parse_json_string_streaming(parser) fam {
            when err -> yikes "Failed to parse string: " + err
        }
        
        # Check if this is a property name (followed by :)
        skip_json_whitespace(parser)
        ready (parser.position < parser.content.len() && parser.current_char == ":") {
            advance_parser_position(parser)  # Skip :
            damn create_json_event(JsonEventType.PropertyName, string_value, parser)
        } otherwise {
            damn create_json_event(JsonEventType.StringValue, string_value, parser)
        }
    } otherwise ready (ch == "t" || ch == "f") {
        sus bool_value tea = parse_json_boolean_streaming(parser) fam {
            when err -> yikes "Failed to parse boolean: " + err
        }
        damn create_json_event(JsonEventType.BooleanValue, bool_value, parser)
    } otherwise ready (ch == "n") {
        parse_json_null_streaming(parser) fam {
            when err -> yikes "Failed to parse null: " + err
        }
        damn create_json_event(JsonEventType.NullValue, "null", parser)
    } otherwise ready (is_json_number_start(ch)) {
        sus number_value tea = parse_json_number_streaming(parser) fam {
            when err -> yikes "Failed to parse number: " + err
        }
        damn create_json_event(JsonEventType.NumberValue, number_value, parser)
    } otherwise ready (ch == ",") {
        advance_parser_position(parser)
        damn get_next_json_event(parser)  # Skip comma and get next event
    } otherwise ready (ch == ":") {
        advance_parser_position(parser)
        damn get_next_json_event(parser)  # Skip colon and get next event
    } otherwise {
        yikes "Unexpected character '" + ch + "' at line " + mathz.int_to_string(parser.line)
    }
}

# ========================
# Streaming Parser Helpers
# ========================

# Parse JSON string in streaming mode
slay parse_json_string_streaming(parser sus JsonStreamParser) yikes<tea> {
    ready (parser.current_char != "\"") {
        yikes "Expected '\"' at start of string"
    }
    
    advance_parser_position(parser)  # Skip opening quote
    clear_parser_buffer(parser)
    
    bestie (parser.position < parser.content.len()) {
        sus ch tea = parser.current_char
        
        ready (ch == "\"" && !parser.escaped) {
            advance_parser_position(parser)  # Skip closing quote
            damn get_parser_buffer_content(parser)
        } otherwise ready (ch == "\\" && !parser.escaped) {
            parser.escaped = based
            advance_parser_position(parser)
        } otherwise ready (parser.escaped) {
            # Handle escape sequences
            ready (ch == "n") {
                append_to_parser_buffer(parser, "\n")
            } otherwise ready (ch == "r") {
                append_to_parser_buffer(parser, "\r")
            } otherwise ready (ch == "t") {
                append_to_parser_buffer(parser, "\t")
            } otherwise ready (ch == "\\") {
                append_to_parser_buffer(parser, "\\")
            } otherwise ready (ch == "\"") {
                append_to_parser_buffer(parser, "\"")
            } otherwise ready (ch == "/") {
                append_to_parser_buffer(parser, "/")
            } otherwise ready (ch == "u") {
                # Unicode escape sequence - simplified
                append_to_parser_buffer(parser, "\\u")
            } otherwise {
                append_to_parser_buffer(parser, ch)
            }
            parser.escaped = cap
            advance_parser_position(parser)
        } otherwise {
            append_to_parser_buffer(parser, ch)
            advance_parser_position(parser)
        }
        
        # Check buffer size limit
        ready (get_parser_buffer_size(parser) > parser.max_buffer_size) {
            yikes "String too long, exceeds buffer limit"
        }
    }
    
    yikes "Unterminated string"
}

# Parse JSON number in streaming mode
slay parse_json_number_streaming(parser sus JsonStreamParser) yikes<tea> {
    clear_parser_buffer(parser)
    
    # Handle negative sign
    ready (parser.current_char == "-") {
        append_to_parser_buffer(parser, "-")
        advance_parser_position(parser)
    }
    
    # Parse integer part
    sus has_digits lit = cap
    bestie (parser.position < parser.content.len() && is_json_digit(parser.current_char)) {
        append_to_parser_buffer(parser, parser.current_char)
        advance_parser_position(parser)
        has_digits = based
    }
    
    ready (!has_digits) {
        yikes "Invalid number format"
    }
    
    # Parse decimal part
    ready (parser.position < parser.content.len() && parser.current_char == ".") {
        append_to_parser_buffer(parser, ".")
        advance_parser_position(parser)
        
        sus has_decimal_digits lit = cap
        bestie (parser.position < parser.content.len() && is_json_digit(parser.current_char)) {
            append_to_parser_buffer(parser, parser.current_char)
            advance_parser_position(parser)
            has_decimal_digits = based
        }
        
        ready (!has_decimal_digits) {
            yikes "Invalid decimal number format"
        }
    }
    
    # Parse exponent part
    ready (parser.position < parser.content.len() && 
          (parser.current_char == "e" || parser.current_char == "E")) {
        append_to_parser_buffer(parser, parser.current_char)
        advance_parser_position(parser)
        
        # Handle exponent sign
        ready (parser.position < parser.content.len() && 
              (parser.current_char == "+" || parser.current_char == "-")) {
            append_to_parser_buffer(parser, parser.current_char)
            advance_parser_position(parser)
        }
        
        sus has_exponent_digits lit = cap
        bestie (parser.position < parser.content.len() && is_json_digit(parser.current_char)) {
            append_to_parser_buffer(parser, parser.current_char)
            advance_parser_position(parser)
            has_exponent_digits = based
        }
        
        ready (!has_exponent_digits) {
            yikes "Invalid exponent format"
        }
    }
    
    damn get_parser_buffer_content(parser)
}

# Parse JSON boolean in streaming mode
slay parse_json_boolean_streaming(parser sus JsonStreamParser) yikes<tea> {
    ready (starts_with_at_parser_position(parser, "true")) {
        advance_parser_position(parser)
        advance_parser_position(parser)
        advance_parser_position(parser)
        advance_parser_position(parser)
        damn "true"
    } otherwise ready (starts_with_at_parser_position(parser, "false")) {
        advance_parser_position(parser)
        advance_parser_position(parser)
        advance_parser_position(parser)
        advance_parser_position(parser)
        advance_parser_position(parser)
        damn "false"
    }
    
    yikes "Invalid boolean value"
}

# Parse JSON null in streaming mode
slay parse_json_null_streaming(parser sus JsonStreamParser) yikes<tea> {
    ready (starts_with_at_parser_position(parser, "null")) {
        advance_parser_position(parser)
        advance_parser_position(parser)
        advance_parser_position(parser)
        advance_parser_position(parser)
        damn "null"
    }
    
    yikes "Invalid null value"
}

# ========================
# Parser State Management
# ========================

# Advance parser position
slay advance_parser_position(parser sus JsonStreamParser) {
    ready (parser.position < parser.content.len()) {
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
}

# Skip whitespace in streaming mode
slay skip_json_whitespace(parser sus JsonStreamParser) {
    bestie (parser.position < parser.content.len()) {
        sus ch tea = parser.current_char
        ready (ch == " " || ch == "\t" || ch == "\n" || ch == "\r") {
            advance_parser_position(parser)
        } otherwise {
            break
        }
    }
}

# Buffer management functions
slay clear_parser_buffer(parser sus JsonStreamParser) {
    parser.buffer = ""
    parser.buffer_start = parser.position
}

slay append_to_parser_buffer(parser sus JsonStreamParser, ch tea) {
    parser.buffer = parser.buffer + ch
}

slay get_parser_buffer_content(parser JsonStreamParser) tea {
    damn parser.buffer
}

slay get_parser_buffer_size(parser JsonStreamParser) drip {
    damn parser.buffer.len()
}

# Check if string starts with pattern at current position
slay starts_with_at_parser_position(parser JsonStreamParser, pattern tea) lit {
    ready (parser.position + pattern.len() > parser.content.len()) {
        damn cap
    }
    
    sus i drip = 0
    bestie (i < pattern.len()) {
        sus content_char tea = stringz.char_at(parser.content, parser.position + i)
        sus pattern_char tea = stringz.char_at(pattern, i)
        ready (content_char != pattern_char) {
            damn cap
        }
        i = i + 1
    }
    
    damn based
}

# Utility functions
slay is_json_number_start(ch tea) lit {
    damn (ch >= "0" && ch <= "9") || ch == "-"
}

slay is_json_digit(ch tea) lit {
    damn (ch >= "0" && ch <= "9")
}

# Create JSON event
slay create_json_event(event_type JsonEventType, value tea, parser JsonStreamParser) JsonEvent {
    damn {
        event_type: event_type,
        value: value,
        line: parser.line,
        column: parser.column,
        depth: parser.depth
    }
}

slay create_end_document_event() JsonEvent {
    damn {
        event_type: JsonEventType.EndDocument,
        value: "",
        line: 0,
        column: 0,
        depth: 0
    }
}

# ========================
# Event Iterator Interface
# ========================

squad JsonEventIterator {
    parser JsonStreamParser
    has_next lit
    current_event JsonEvent
}

# Check if iterator has next event
slay json_iterator_has_next(iterator sus JsonEventIterator) lit {
    damn iterator.has_next && iterator.parser.position < iterator.parser.content.len()
}

# Get next event from iterator
slay json_iterator_next(iterator sus JsonEventIterator) yikes<JsonEvent> {
    ready (!json_iterator_has_next(iterator)) {
        damn iterator.current_event
    }
    
    sus event JsonEvent = get_next_json_event(&iterator.parser) fam {
        when err -> yikes err
    }
    
    iterator.current_event = event
    
    ready (event.event_type == JsonEventType.EndDocument) {
        iterator.has_next = cap
    }
    
    damn event
}

# Export all streaming parser functions
# This makes them available when importing the jsonz module
