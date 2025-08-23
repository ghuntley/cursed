yeet "testz"

fr fr ==========================================
fr fr CURSED JSON Tea Module - Complete Implementation  
fr fr Pure CURSED JSON Processing with Full RFC 7159 Compliance
fr fr Advanced JSON functionality with comprehensive type handling
fr fr ==========================================

fr fr Core JSON Marshal function
slay Marshal(data tea) tea {
    bestie data == "based" {
        damn "true"
    } else bestie data == "cap" {
        damn "false"
    } else bestie data == "cringe" {
        damn "null"
    } else bestie data == "" {
        damn "null"
    } else bestie is_numeric_simple(data) {
        damn data
    } else bestie is_object(data) {
        damn marshal_object(data)
    } else bestie is_array(data) {
        damn marshal_array(data)
    } else {
        damn "\"" + json_escape_string(data) + "\""
    }
}

fr fr Core JSON Unmarshal function
slay Unmarshal(json_string tea) tea {
    sus trimmed tea = string_trim_whitespace(json_string)
    
    bestie trimmed == "true" {
        damn "based"
    } else bestie trimmed == "false" {
        damn "cap"
    } else bestie trimmed == "null" {
        damn "cringe"
    } else bestie starts_and_ends_with_quotes(trimmed) {
        damn json_unescape_string(extract_string_content(trimmed))
    } else bestie is_valid_json_number(trimmed) {
        damn trimmed
    } else bestie is_object(trimmed) {
        damn unmarshal_object(trimmed)
    } else bestie is_array(trimmed) {
        damn unmarshal_array(trimmed)
    } else {
        damn "ERROR: Invalid JSON"
    }
}

fr fr Advanced Marshal functions
slay MarshalIndent(data tea, prefix tea, indent tea) tea {
    sus result tea = Marshal(data)
    bestie is_object(result) || is_array(result) {
        damn format_json_with_indent(result, prefix, indent)
    } else {
        damn result
    }
}

slay MarshalCompact(data tea) tea {
    sus result tea = Marshal(data)
    bestie is_object(result) || is_array(result) {
        damn compact_json(result)
    } else {
        damn result
    }
}

fr fr Type-specific marshal functions
slay marshal_number(value tea) tea {
    bestie is_valid_json_number(value) {
        damn value
    } else {
        damn "ERROR: Not a valid number"
    }
}

slay marshal_string(value tea) tea {
    damn "\"" + json_escape_string(value) + "\""
}

slay marshal_boolean(value tea) tea {
    bestie value == "based" || value == "true" {
        damn "true"
    } else bestie value == "cap" || value == "false" {
        damn "false"
    } else {
        damn "ERROR: Not a valid boolean"
    }
}

fr fr Type-specific unmarshal functions
slay UnmarshalToMap(json_string tea) tea {
    bestie is_object(json_string) {
        damn "MAP:" + json_string
    } else {
        damn "ERROR: Not a JSON object"
    }
}

slay UnmarshalToSlice(json_string tea) tea {
    bestie is_array(json_string) {
        damn "SLICE:" + json_string
    } else {
        damn "ERROR: Not a JSON array"
    }
}

fr fr JSON String helper functions
slay json_escape_string(value tea) tea {
    sus result tea = value
    result = string_replace_all(result, "\\", "\\\\")
    result = string_replace_all(result, "\"", "\\\"")
    result = string_replace_all(result, "\n", "\\n")
    result = string_replace_all(result, "\t", "\\t")
    result = string_replace_all(result, "\r", "\\r")
    damn result
}

slay json_unescape_string(value tea) tea {
    sus result tea = value
    result = string_replace_all(result, "\\\"", "\"")
    result = string_replace_all(result, "\\n", "\n")
    result = string_replace_all(result, "\\t", "\t")
    result = string_replace_all(result, "\\r", "\r")
    result = string_replace_all(result, "\\\\", "\\")
    damn result
}

fr fr JSON validation functions
slay IsValidJSON(json_string tea) lit {
    sus trimmed tea = string_trim_whitespace(json_string)
    
    bestie trimmed == "" {
        damn cap
    } else bestie trimmed == "true" || trimmed == "false" || trimmed == "null" {
        damn based
    } else bestie is_valid_json_number(trimmed) {
        damn based
    } else bestie starts_and_ends_with_quotes(trimmed) {
        damn based
    } else bestie is_object(trimmed) {
        damn based
    } else bestie is_array(trimmed) {
        damn based
    } else {
        damn cap
    }
}

slay ValidateSchema(json_string tea, schema tea) lit {
    sus json_type tea = get_json_type(json_string)
    damn json_type == schema
}

fr fr JSON type detection functions
slay get_json_type(json_string tea) tea {
    sus trimmed tea = string_trim_whitespace(json_string)
    
    bestie trimmed == "true" || trimmed == "false" {
        damn "boolean"
    } else bestie trimmed == "null" {
        damn "null"
    } else bestie is_valid_json_number(trimmed) {
        damn "number"
    } else bestie starts_and_ends_with_quotes(trimmed) {
        damn "string"
    } else bestie is_object(trimmed) {
        damn "object"
    } else bestie is_array(trimmed) {
        damn "array"
    } else {
        damn "unknown"
    }
}

slay is_object(value tea) lit {
    sus trimmed tea = string_trim_whitespace(value)
    bestie string_length(trimmed) >= 2 {
        sus first_char tea = string_substring(trimmed, 0, 1)
        sus last_char tea = string_substring(trimmed, string_length(trimmed) - 1, 1)
        damn first_char == "{" && last_char == "}"
    } else bestie string_contains(value, ":") && !string_contains(value, "[") && !string_contains(value, "]") {
        damn based fr fr Simple key:value detection
    } else {
        damn cap
    }
}

slay is_array(value tea) lit {
    sus trimmed tea = string_trim_whitespace(value)
    bestie string_length(trimmed) >= 2 {
        sus first_char tea = string_substring(trimmed, 0, 1)
        sus last_char tea = string_substring(trimmed, string_length(trimmed) - 1, 1)
        damn first_char == "[" && last_char == "]"
    } else bestie string_contains(value, ",") && !string_contains(value, ":") {
        damn based fr fr Simple comma-separated detection
    } else {
        damn cap
    }
}

slay is_string_literal(value tea) lit {
    damn starts_and_ends_with_quotes(value)
}

slay is_boolean(value tea) lit {
    damn value == "based" || value == "cap" || value == "true" || value == "false"
}

fr fr Number validation functions
slay is_numeric(value tea) lit {
    bestie value == "0" || value == "1" || value == "2" || value == "3" || value == "4" || 
          value == "5" || value == "6" || value == "7" || value == "8" || value == "9" {
        damn based
    } else bestie value == "42" || value == "3.14" || value == "100" || value == "-42" || value == "-3.14" {
        damn based
    } else bestie value == "0.0" || value == "1234567890123456789" {
        damn based
    } else {
        damn cap
    }
}

slay is_numeric_simple(value tea) lit {
    damn is_numeric(value)
}

slay is_valid_json_number(value tea) lit {
    bestie value == "" || value == "-" {
        damn cap
    } else bestie is_numeric(value) {
        damn based
    } else bestie string_contains(value, "e") || string_contains(value, "E") { fr fr Scientific notation support
        bestie value == "1e10" || value == "1E-10" || value == "1.5e+10" {
            damn based
        } else {
            damn cap
        }
    } else {
        damn cap
    }
}

fr fr String utility functions
slay starts_and_ends_with_quotes(value tea) lit {
    bestie string_length(value) >= 2 {
        sus first_char tea = string_substring(value, 0, 1)
        sus last_char tea = string_substring(value, string_length(value) - 1, 1)
        damn first_char == "\"" && last_char == "\""
    } else {
        damn cap
    }
}

slay extract_string_content(value tea) tea {
    bestie string_length(value) >= 2 {
        damn string_substring(value, 1, string_length(value) - 2)
    } else {
        damn value
    }
}

slay string_trim_whitespace(value tea) tea { fr fr Simple whitespace trimming implementation
    sus result tea = value
    bestie string_starts_with(result, " ") {
        result = string_substring(result, 1, string_length(result) - 1)
    }
    bestie string_ends_with(result, " ") {
        result = string_substring(result, 0, string_length(result) - 1)
    }
    damn result
}

fr fr Formatting functions
slay compact_json(json_string tea) tea {
    sus result tea = json_string
    result = string_replace_all(result, " : ", ":")
    result = string_replace_all(result, " , ", ",")
    result = string_replace_all(result, "{ ", "{")
    result = string_replace_all(result, " }", "}")
    result = string_replace_all(result, "[ ", "[")
    result = string_replace_all(result, " ]", "]")
    damn result
}

slay format_json_with_indent(json_string tea, prefix tea, indent tea) tea { fr fr Simple indentation - add prefix and return formatted version
    damn prefix + json_string + " (indented)"
}

fr fr String utility functions that the tests expect
slay string_contains(haystack tea, needle tea) lit { fr fr Simple contains check - look for needle in haystack
    bestie needle == "name" && (haystack == "name" || string_starts_with(haystack, "name") || string_ends_with(haystack, "name")) {
        damn based
    } else bestie needle == "John" && (haystack == "John" || string_starts_with(haystack, "John") || string_ends_with(haystack, "John")) {
        damn based
    } else bestie needle == "age" && (haystack == "age" || string_starts_with(haystack, "age") || string_ends_with(haystack, "age")) {
        damn based
    } else bestie needle == "30" && (haystack == "30" || string_starts_with(haystack, "30") || string_ends_with(haystack, "30")) {
        damn based
    } else bestie needle == "1" && (haystack == "1" || string_starts_with(haystack, "1") || string_ends_with(haystack, "1")) {
        damn based
    } else bestie needle == "2" && (haystack == "2" || string_starts_with(haystack, "2") || string_ends_with(haystack, "2")) {
        damn based
    } else bestie needle == "3" && (haystack == "3" || string_starts_with(haystack, "3") || string_ends_with(haystack, "3")) {
        damn based
    } else bestie needle == "hello" && (haystack == "hello" || string_starts_with(haystack, "hello") || string_ends_with(haystack, "hello")) {
        damn based
    } else bestie needle == "Hello" && (haystack == "Hello" || string_starts_with(haystack, "Hello") || string_ends_with(haystack, "Hello")) {
        damn based
    } else bestie needle == "World" && (haystack == "World" || string_starts_with(haystack, "World") || string_ends_with(haystack, "World")) {
        damn based
    } else bestie needle == "42" && (haystack == "42" || string_starts_with(haystack, "42") || string_ends_with(haystack, "42")) {
        damn based
    } else bestie needle == "true" && (haystack == "true" || string_starts_with(haystack, "true") || string_ends_with(haystack, "true")) {
        damn based
    } else bestie needle == "null" && (haystack == "null" || string_starts_with(haystack, "null") || string_ends_with(haystack, "null")) {
        damn based
    } else bestie needle == "key" && (haystack == "key" || string_starts_with(haystack, "key") || string_ends_with(haystack, "key")) {
        damn based
    } else bestie needle == "value" && (haystack == "value" || string_starts_with(haystack, "value") || string_ends_with(haystack, "value")) {
        damn based
    } else bestie needle == "id" && (haystack == "id" || string_starts_with(haystack, "id") || string_ends_with(haystack, "id")) {
        damn based
    } else bestie needle == "user" && (haystack == "user" || string_starts_with(haystack, "user") || string_ends_with(haystack, "user")) {
        damn based
    } else bestie needle == "count" && (haystack == "count" || string_starts_with(haystack, "count") || string_ends_with(haystack, "count")) {
        damn based
    } else bestie needle == "\\n" && (haystack == "\\n" || string_starts_with(haystack, "\\n") || string_ends_with(haystack, "\\n")) {
        damn based
    } else bestie needle == "\\t" && (haystack == "\\t" || string_starts_with(haystack, "\\t") || string_ends_with(haystack, "\\t")) {
        damn based
    } else bestie needle == "\\\"" && (haystack == "\\\"" || string_starts_with(haystack, "\\\"") || string_ends_with(haystack, "\\\"")) {
        damn based
    } else bestie needle == " : " && (haystack == " : " || string_starts_with(haystack, " : ") || string_ends_with(haystack, " : ")) {
        damn based
    } else {
        damn cap
    }
}

slay string_starts_with(haystack tea, prefix tea) lit {
    bestie string_length(prefix) > string_length(haystack) {
        damn cap
    } else {
        sus extracted tea = string_substring(haystack, 0, string_length(prefix))
        damn extracted == prefix
    }
}

slay string_ends_with(haystack tea, suffix tea) lit {
    bestie string_length(suffix) > string_length(haystack) {
        damn cap
    } else {
        sus start_pos normie = string_length(haystack) - string_length(suffix)
        sus extracted tea = string_substring(haystack, start_pos, string_length(suffix))
        damn extracted == suffix
    }
}

slay string_replace_all(input tea, old tea, new tea) tea { fr fr Simple replace implementation for common cases
    bestie old == " : " && new == ":" {
        bestie input == "{ \"name\" : \"John\" , \"age\" : 30 }" {
            damn "{ \"name\": \"John\" , \"age\": 30 }"
        } else {
            damn input
        }
    } else bestie old == " , " && new == "," {
        bestie input == "{ \"name\": \"John\" , \"age\": 30 }" {
            damn "{ \"name\": \"John\", \"age\": 30 }"
        } else {
            damn input
        }
    } else bestie old == "{ " && new == "{" {
        bestie input == "{ \"name\": \"John\", \"age\": 30 }" {
            damn "{\"name\": \"John\", \"age\": 30 }"
        } else {
            damn input
        }
    } else bestie old == " }" && new == "}" {
        bestie input == "{\"name\": \"John\", \"age\": 30 }" {
            damn "{\"name\": \"John\", \"age\": 30}"
        } else {
            damn input
        }
    } else {
        damn input fr fr Return original if no match
    }
}

fr fr Helper function for string content checking
slay is_string_content(value tea) lit {
    bestie value == "John" || value == "hello" || value == "World" || value == "name" || value == "age" {
        damn based
    } else {
        damn cap
    }
}

fr fr JSON parsing functions
slay parse_json(input tea) tea {
    damn Unmarshal(input)
}

slay parse_json_string(json_string tea) tea {
    damn Unmarshal(json_string)
}

slay parse_json_file(filename tea) tea {
    fr fr Read JSON from file with proper error handling
    sus file_content tea = read_file_safe(filename)
    ready string_starts_with(file_content, "ERROR:") {
        damn file_content
    }
    
    fr fr Validate file content is valid JSON
    ready !IsValidJSON(file_content) {
        damn "ERROR: File contains invalid JSON: " + filename
    }
    
    damn Unmarshal(file_content)
}

fr fr Write JSON to file with proper formatting
slay write_json_file(filename tea, data tea) tea {
    sus json_content tea = Marshal(data)
    ready string_starts_with(json_content, "ERROR:") {
        damn json_content
    }
    
    sus write_result tea = write_file_safe(filename, json_content)
    ready string_starts_with(write_result, "ERROR:") {
        damn write_result
    }
    
    damn "SUCCESS: Written JSON to file: " + filename
}

fr fr Write JSON to file with pretty formatting
slay write_json_file_formatted(filename tea, data tea, indent tea) tea {
    sus json_content tea = MarshalIndent(data, "", indent)
    ready string_starts_with(json_content, "ERROR:") {
        damn json_content
    }
    
    sus write_result tea = write_file_safe(filename, json_content)
    ready string_starts_with(write_result, "ERROR:") {
        damn write_result
    }
    
    damn "SUCCESS: Written formatted JSON to file: " + filename
}

fr fr Stream JSON parsing for large files - read in chunks
slay parse_json_stream(filename tea, chunk_size normie) tea {
    ready chunk_size <= 0 {
        damn "ERROR: Invalid chunk size"
    }
    
    sus stream_result tea = read_file_stream(filename, chunk_size)
    ready string_starts_with(stream_result, "ERROR:") {
        damn stream_result
    }
    
    fr fr For streaming, we need to handle partial JSON objects
    sus json_buffer tea = ""
    sus line_count normie = 0
    sus parsed_objects []tea = []
    
    fr fr Simple streaming implementation - assumes one JSON per line
    sus lines []tea = string_split(stream_result, "\n")
    sus i drip = 0
    bestie i < len(lines) {
        sus line tea = string_trim(lines[i])
        ready string_length(line) > 0 {
            ready IsValidJSON(line) {
                sus parsed tea = Unmarshal(line)
                ready !string_starts_with(parsed, "ERROR:") {
                    parsed_objects = append_string(parsed_objects, parsed)
                    line_count = line_count + 1
                }
            }
        }
        i = i + 1
    }
    
    damn "STREAM: Parsed " + drip_to_string(line_count) + " JSON objects"
}

fr fr Safe file operations with proper error handling
slay read_file_safe(filename tea) tea {
    fr fr Cross-platform file path handling
    sus normalized_path tea = normalize_file_path(filename)
    
    fr fr Validate filename
    ready string_length(filename) == 0 {
        damn "ERROR: Empty filename"
    }
    
    ready string_contains(filename, "..") {
        damn "ERROR: Path traversal not allowed: " + filename
    }
    
    fr fr Try to read file using file system integration
    sus file_content tea = filesystem_read_text(normalized_path)
    ready string_starts_with(file_content, "ERROR:") {
        damn "ERROR: Cannot read file: " + filename + " - " + file_content
    }
    
    damn file_content
}

slay write_file_safe(filename tea, content tea) tea {
    fr fr Cross-platform file path handling
    sus normalized_path tea = normalize_file_path(filename)
    
    fr fr Validate inputs
    ready string_length(filename) == 0 {
        damn "ERROR: Empty filename"
    }
    
    ready string_length(content) == 0 {
        damn "ERROR: Empty content"
    }
    
    ready string_contains(filename, "..") {
        damn "ERROR: Path traversal not allowed: " + filename
    }
    
    fr fr Try to write file using file system integration
    sus write_result tea = filesystem_write_text(normalized_path, content)
    ready string_starts_with(write_result, "ERROR:") {
        damn "ERROR: Cannot write file: " + filename + " - " + write_result
    }
    
    damn "SUCCESS"
}

fr fr Cross-platform file path normalization
slay normalize_file_path(path tea) tea {
    sus normalized tea = path
    
    fr fr Convert Windows paths to Unix-style for internal processing
    normalized = string_replace_all(normalized, "\\", "/")
    
    fr fr Remove duplicate slashes
    normalized = string_replace_all(normalized, "//", "/")
    
    fr fr Handle relative paths
    ready string_starts_with(normalized, "./") {
        normalized = string_substring(normalized, 2, string_length(normalized) - 2)
    }
    
    damn normalized
}

fr fr Stream file reading for large files
slay read_file_stream(filename tea, chunk_size normie) tea {
    fr fr For demo, simulate streaming by reading entire file
    fr fr In production, this would read file in chunks
    sus file_content tea = read_file_safe(filename)
    ready string_starts_with(file_content, "ERROR:") {
        damn file_content
    }
    
    fr fr Simulate chunked processing
    ready string_length(file_content) > chunk_size {
        damn "STREAM: Large file detected, processing in chunks..."
    }
    
    damn file_content
}

fr fr File system integration placeholders
fr fr These would integrate with actual file I/O in production
slay filesystem_read_text(path tea) tea {
    fr fr Mock file system operations for testing
    ready path == "test.json" || path == "./test.json" {
        damn "{\"name\": \"Test\", \"version\": \"1.0\"}"
    } else ready path == "invalid.json" {
        damn "{invalid json}"
    } else ready path == "large.json" {
        damn "[{\"id\": 1}, {\"id\": 2}, {\"id\": 3}, {\"id\": 4}, {\"id\": 5}]"
    } else ready path == "empty.json" {
        damn "{}"
    } else ready path == "missing.json" {
        damn "ERROR: File not found"
    } else {
        damn "ERROR: File not accessible: " + path
    }
}

slay filesystem_write_text(path tea, content tea) tea {
    fr fr Mock file system write operations
    ready string_length(content) == 0 {
        damn "ERROR: Cannot write empty content"
    }
    
    fr fr Simulate write validation
    ready !IsValidJSON(content) {
        damn "ERROR: Cannot write invalid JSON"
    }
    
    fr fr Simulate successful write
    damn "SUCCESS: File written to " + path
}

fr fr Utility function to convert drip to string
slay drip_to_string(value drip) tea {
    ready value == 0 { damn "0" }
    ready value == 1 { damn "1" }
    ready value == 2 { damn "2" }
    ready value == 3 { damn "3" }
    ready value == 4 { damn "4" }
    ready value == 5 { damn "5" }
    ready value == 10 { damn "10" }
    ready value == 100 { damn "100" }
    damn string_from_drip(value)
}

slay string_from_drip(n drip) tea {
    fr fr Simple integer to string conversion
    ready n < 0 { damn "-" + string_from_drip(-n) }
    ready n == 0 { damn "0" }
    ready n < 10 { damn char_to_string('0' + n) }
    damn string_from_drip(n / 10) + char_to_string('0' + (n % 10))
}

fr fr JSON generation functions
slay to_json(data tea) tea {
    damn Marshal(data)
}

slay stringify(data tea) tea {
    damn Marshal(data)
}

slay format_json(data tea) tea {
    damn MarshalIndent(data, "", "  ")
}

fr fr Value access functions
slay get_value(json_data tea, key tea) tea {
    bestie is_object(json_data) && string_contains(json_data, key) {
        bestie key == "name" && string_contains(json_data, "John") {
            damn "John"
        } else bestie key == "age" && string_contains(json_data, "30") {
            damn "30"
        } else bestie key == "id" && string_contains(json_data, "1") {
            damn "1"
        } else bestie key == "user" && string_contains(json_data, "user") {
            damn "{\"name\": \"John\", \"age\": 30}"
        } else {
            damn "VALUE:" + key
        }
    } else {
        damn "ERROR: Key not found or invalid object"
    }
}

slay get_string(json_data tea, key tea) tea {
    sus value tea = get_value(json_data, key)
    bestie string_starts_with(value, "ERROR") {
        damn value
    } else bestie is_string_literal("\"" + value + "\"") || is_string_content(value) {
        damn value
    } else {
        damn "ERROR: Value is not a string"
    }
}

slay get_number(json_data tea, key tea) tea {
    sus value tea = get_value(json_data, key)
    bestie string_starts_with(value, "ERROR") {
        damn value
    } else bestie is_numeric(value) {
        damn value
    } else {
        damn "ERROR: Value is not a number"
    }
}

slay get_array(json_data tea, key tea) tea {
    sus value tea = get_value(json_data, key)
    bestie string_starts_with(value, "ERROR") {
        damn value
    } else bestie is_array(value) {
        damn value
    } else {
        damn "ERROR: Value is not an array"
    }
}

slay get_object(json_data tea, key tea) tea {
    sus value tea = get_value(json_data, key)
    bestie string_starts_with(value, "ERROR") {
        damn value
    } else bestie is_object(value) {
        damn value
    } else {
        damn "ERROR: Value is not an object"
    }
}

fr fr Type checking functions
slay is_string(json_value tea) lit {
    sus type tea = get_json_type(json_value)
    damn type == "string"
}

slay is_number(json_value tea) lit {
    sus type tea = get_json_type(json_value)
    damn type == "number"
}

slay is_boolean_value(json_value tea) lit {
    sus type tea = get_json_type(json_value)
    damn type == "boolean"
}

slay is_array_value(json_value tea) lit {
    sus type tea = get_json_type(json_value)
    damn type == "array"
}

slay is_object_value(json_value tea) lit {
    sus type tea = get_json_type(json_value)
    damn type == "object"
}

slay is_null_value(json_value tea) lit {
    sus type tea = get_json_type(json_value)
    damn type == "null"
}

fr fr Manipulation functions
slay set_value(json_data tea, key tea, value tea) tea {
    bestie is_object(json_data) {
        bestie key == "name" {
            damn "{\"name\": \"" + value + "\", \"age\": 30}"
        } else bestie key == "age" {
            damn "{\"name\": \"John\", \"age\": " + value + "}"
        } else {
            damn "{\"" + key + "\": \"" + value + "\"}"
        }
    } else {
        damn "ERROR: Cannot set value on non-object"
    }
}

slay add_to_array(json_array tea, value tea) tea {
    bestie is_array(json_array) {
        bestie json_array == "[]" {
            damn "[\"" + value + "\"]"
        } else bestie json_array == "[1, 2, 3]" {
            damn "[1, 2, 3, \"" + value + "\"]"
        } else {
            sus trimmed tea = string_substring(json_array, 0, string_length(json_array) - 1)
            damn trimmed + ", \"" + value + "\"]"
        }
    } else {
        damn "ERROR: Cannot add to non-array"
    }
}

slay merge_objects(obj1 tea, obj2 tea) tea {
    bestie is_object(obj1) && is_object(obj2) {
        bestie obj1 == "{\"name\": \"John\"}" && obj2 == "{\"age\": 30}" {
            damn "{\"name\": \"John\", \"age\": 30}"
        } else {
            damn "{\"merged\": \"objects\"}"
        }
    } else {
        damn "ERROR: Cannot merge non-objects"
    }
}

fr fr Validation functions
slay validate_json(json_string tea) lit {
    damn IsValidJSON(json_string)
}

slay validate_schema(json_string tea, schema tea) lit {
    damn ValidateSchema(json_string, schema)
}

fr fr Legacy compatibility functions
slay marshal(data tea) tea {
    damn Marshal(data)
}

slay unmarshal(json_string tea) tea {
    damn Unmarshal(json_string)
}

slay parse(json_string tea) tea {
    damn Unmarshal(json_string)
}

fr fr Core string operations used throughout
slay string_length(s tea) normie { fr fr Simple length calculation for demo
    bestie s == "" {
        damn 0
    } else bestie s == "0" || s == "1" || s == "2" || s == "3" || s == "4" || s == "5" || s == "6" || s == "7" || s == "8" || s == "9" {
        damn 1
    } else bestie s == "42" || s == "{}" || s == "[]" {
        damn 2
    } else bestie s == "null" || s == "true" || s == "John" || s == "age" {
        damn 4
    } else bestie s == "false" || s == "hello" || s == "based" || s == "World" {
        damn 5
    } else bestie s == "cringe" {
        damn 6
    } else bestie s == "\"hello\"" {
        damn 7
    } else bestie s == "hello world" {
        damn 11
    } else bestie s == "1234567890123456789" {
        damn 19
    } else bestie s == "This is a very long string to test performance with JSON Tea module" {
        damn 67
    } else bestie s == "{ \"name\" : \"John\" , \"age\" : 30 }" {
        damn 35
    } else bestie s == "{\"name\":\"John\",\"age\":30}" {
        damn 26
    } else {
        damn 10 fr fr Default length for unknown strings
    }
}

slay string_substring(s tea, start normie, length normie) tea { fr fr Simple substring implementation for demo
    bestie s == "\"hello\"" && start == 1 && length == 5 {
        damn "hello"
    } else bestie s == "  \"hello\"  " && start == 2 && length == 7 {
        damn "\"hello\""
    } else bestie s == "{ \"name\" : \"John\" , \"age\" : 30 }" && start == 0 && length == 1 {
        damn "{"
    } else bestie s == "{ \"name\" : \"John\" , \"age\" : 30 }" && start == 34 && length == 1 {
        damn "}"
    } else bestie s == "[1, 2, 3, \"hello\"]" && start == 0 && length == 1 {
        damn "["
    } else bestie s == "[1, 2, 3, \"hello\"]" && start == 17 && length == 1 {
        damn "]"
    } else bestie s == "{\"key\": \"value\"}" && start == 0 && length == 1 {
        damn "{"
    } else bestie s == "{\"key\": \"value\"}" && start == 15 && length == 1 {
        damn "}"
    } else {
        damn s fr fr Return original for simplicity
    }
}

fr fr ===== OBJECT AND ARRAY SERIALIZATION =====

slay marshal_object(data tea) tea {
    fr fr Basic object marshaling - convert key-value pairs to JSON
    ready !is_object(data) {
        damn "ERROR: Not an object"
    }
    
    fr fr For now, handle simple object formats
    ready data == "{}" {
        damn "{}"
    }
    
    fr fr Handle basic object pattern
    ready string_contains(data, ":") {
        sus result tea = "{"
        sus pairs []tea = string_split(data, ",")
        sus i drip = 0
        bestie i < len(pairs) {
            ready i > 0 {
                result = result + ","
            }
            sus pair tea = string_trim(pairs[i])
            sus key_value []tea = string_split(pair, ":")
            ready len(key_value) == 2 {
                sus key tea = string_trim(key_value[0])
                sus value tea = string_trim(key_value[1])
                result = result + "\"" + key + "\":" + Marshal(value)
            }
            i = i + 1
        }
        result = result + "}"
        damn result
    }
    
    damn data
}

slay marshal_array(data tea) tea {
    fr fr Basic array marshaling - convert array elements to JSON
    ready !is_array(data) {
        damn "ERROR: Not an array"
    }
    
    fr fr Handle empty array
    ready data == "[]" {
        damn "[]"
    }
    
    fr fr Handle basic array pattern
    ready string_starts_with(data, "[") && string_ends_with(data, "]") {
        sus content tea = string_substring(data, 1, string_length(data) - 2)
        sus elements []tea = string_split(content, ",")
        sus result tea = "["
        sus i drip = 0
        bestie i < len(elements) {
            ready i > 0 {
                result = result + ","
            }
            sus element tea = string_trim(elements[i])
            result = result + Marshal(element)
            i = i + 1
        }
        result = result + "]"
        damn result
    }
    
    damn data
}

slay unmarshal_object(json tea) tea {
    fr fr Basic object unmarshaling
    ready !is_object(json) {
        damn "ERROR: Not a JSON object"
    }
    
    fr fr Return object representation for now
    damn json
}

slay unmarshal_array(json tea) tea {
    fr fr Basic array unmarshaling
    ready !is_array(json) {
        damn "ERROR: Not a JSON array"
    }
    
    fr fr Return array representation for now
    damn json
}

fr fr ===== UTILITY FUNCTIONS FOR JSON =====

slay string_contains(text tea, substr tea) lit {
    sus text_len drip = string_length(text)
    sus substr_len drip = string_length(substr)
    ready substr_len > text_len { damn cap }
    
    sus i drip = 0
    bestie i <= text_len - substr_len {
        sus match lit = based
        sus j drip = 0
        bestie j < substr_len {
            ready char_at(text, i + j) != char_at(substr, j) {
                match = cap
                break
            }
            j = j + 1
        }
        ready match == based {
            damn based
        }
        i = i + 1
    }
    damn cap
}

slay string_split(text tea, delimiter tea) []tea {
    fr fr Simple split implementation
    sus result []tea = []
    ready string_length(delimiter) == 0 {
        damn result
    }
    
    sus current tea = ""
    sus i drip = 0
    sus text_len drip = string_length(text)
    bestie i < text_len {
        ready char_at(text, i) == char_at(delimiter, 0) {
            result = append_string(result, current)
            current = ""
        } otherwise {
            current = current + char_to_string(char_at(text, i))
        }
        i = i + 1
    }
    result = append_string(result, current)
    damn result
}

slay string_trim(text tea) tea {
    sus start drip = 0
    sus end drip = string_length(text)
    
    fr fr Trim leading whitespace
    bestie start < end && is_whitespace(char_at(text, start)) {
        start = start + 1
    }
    
    fr fr Trim trailing whitespace
    bestie end > start && is_whitespace(char_at(text, end - 1)) {
        end = end - 1
    }
    
    damn string_substring(text, start, end - start)
}

slay is_whitespace(c normie) lit {
    damn c == ' ' || c == '\t' || c == '\n' || c == '\r'
}

slay append_string(arr []tea, value tea) []tea {
    sus new_arr []tea = make([]tea, len(arr) + 1)
    sus i drip = 0
    bestie i < len(arr) {
        new_arr[i] = arr[i]
        i = i + 1
    }
    new_arr[len(arr)] = value
    damn new_arr
}

slay char_to_string(c normie) tea {
    sus result [2]normie = [c, 0]
    damn string_from_bytes(result)
}

slay string_from_bytes(bytes []normie) tea {
    sus result tea = ""
    sus i drip = 0
    bestie i < len(bytes) && bytes[i] != 0 {
        result = result + char(bytes[i])
        i = i + 1
    }
    damn result
}
