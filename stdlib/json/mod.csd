// CURSED JSON Library
// Production-ready JSON parsing and manipulation

yeet "string"
yeet "collections"

// ================================
// JSON Parsing Functions
// ================================

slay parse(json_string tea) map {
    damn json_parse(json_string);
}

slay parse_array(json_string tea) [extra] {
    damn json_parse_array(json_string);
}

slay parse_value(json_string tea) extra {
    damn json_parse_value(json_string);
}

slay parse_object(json_string tea) map {
    damn json_parse_object(json_string);
}

// ================================
// JSON Serialization Functions
// ================================

slay stringify(data map) tea {
    damn json_stringify(data);
}

slay stringify_array(data [extra]) tea {
    damn json_stringify_array(data);
}

slay stringify_value(value extra) tea {
    damn json_stringify_value(value);
}

slay stringify_object(data map) tea {
    damn json_stringify_object(data);
}

// ================================
// JSON Validation Functions
// ================================

slay validate(json_string tea) lit {
    damn json_validate(json_string);
}

slay validate_syntax(json_string tea) lit {
    damn json_validate_syntax(json_string);
}

slay is_valid_json(json_string tea) lit {
    damn json_is_valid(json_string);
}

// ================================
// JSON Formatting Functions
// ================================

slay pretty_print(json_string tea) tea {
    damn json_pretty_print(json_string);
}

slay pretty_print_indent(json_string tea, indent_size normie) tea {
    damn json_pretty_print_indent(json_string, indent_size);
}

slay minify(json_string tea) tea {
    damn json_minify(json_string);
}

slay format(json_string tea, compact lit) tea {
    bestie compact {
        damn json_minify(json_string);
    } else {
        damn json_pretty_print(json_string);
    }
}

// ================================
// JSON Utility Functions
// ================================

slay get_value(json_map map, key tea) tea {
    damn json_get_value(json_map, key);
}

slay get_value_or_default(json_map map, key tea, default_value tea) tea {
    damn json_get_value_or_default(json_map, key, default_value);
}

slay set_value(json_map map, key tea, value tea) map {
    json_set_value(json_map, key, value);
    damn json_map;
}

slay has_key(json_map map, key tea) lit {
    damn json_has_key(json_map, key);
}

slay remove_key(json_map map, key tea) map {
    json_remove_key(json_map, key);
    damn json_map;
}

slay get_keys(json_map map) [tea] {
    damn json_get_keys(json_map);
}

slay get_values(json_map map) [tea] {
    damn json_get_values(json_map);
}

slay get_entries(json_map map) [tea] {
    damn json_get_entries(json_map);
}

// ================================
// JSON Array Functions
// ================================

slay get_array_value(json_array [extra], index normie) tea {
    damn json_get_array_value(json_array, index);
}

slay set_array_value(json_array [extra], index normie, value tea) [extra] {
    json_set_array_value(json_array, index, value);
    damn json_array;
}

slay push_array_value(json_array [extra], value tea) [extra] {
    json_push_array_value(json_array, value);
    damn json_array;
}

slay pop_array_value(json_array [extra]) tea {
    damn json_pop_array_value(json_array);
}

slay array_length(json_array [extra]) normie {
    damn json_array_length(json_array);
}

// ================================
// JSON Path Functions
// ================================

slay get_path(json_map map, path tea) tea {
    damn json_get_path(json_map, path);
}

slay set_path(json_map map, path tea, value tea) map {
    json_set_path(json_map, path, value);
    damn json_map;
}

slay has_path(json_map map, path tea) lit {
    damn json_has_path(json_map, path);
}

slay remove_path(json_map map, path tea) map {
    json_remove_path(json_map, path);
    damn json_map;
}

// ================================
// JSON Type Functions
// ================================

slay get_type(json_value extra) tea {
    damn json_get_type(json_value);
}

slay is_object(json_value extra) lit {
    damn json_is_object(json_value);
}

slay is_array(json_value extra) lit {
    damn json_is_array(json_value);
}

slay is_string(json_value extra) lit {
    damn json_is_string(json_value);
}

slay is_number(json_value extra) lit {
    damn json_is_number(json_value);
}

slay is_boolean(json_value extra) lit {
    damn json_is_boolean(json_value);
}

slay is_null(json_value extra) lit {
    damn json_is_null(json_value);
}

// ================================
// JSON Conversion Functions
// ================================

slay to_string(json_value extra) tea {
    damn json_to_string(json_value);
}

slay to_number(json_value extra) meal {
    damn json_to_number(json_value);
}

slay to_integer(json_value extra) normie {
    damn json_to_integer(json_value);
}

slay to_boolean(json_value extra) lit {
    damn json_to_boolean(json_value);
}

slay to_array(json_value extra) [extra] {
    damn json_to_array(json_value);
}

slay to_map(json_value extra) map {
    damn json_to_map(json_value);
}

// ================================
// JSON Merge Functions
// ================================

slay merge(json1 map, json2 map) map {
    damn json_merge(json1, json2);
}

slay merge_deep(json1 map, json2 map) map {
    damn json_merge_deep(json1, json2);
}

slay merge_arrays(arr1 [extra], arr2 [extra]) [extra] {
    damn json_merge_arrays(arr1, arr2);
}

// ================================
// JSON Comparison Functions
// ================================

slay equals(json1 extra, json2 extra) lit {
    damn json_equals(json1, json2);
}

slay deep_equals(json1 extra, json2 extra) lit {
    damn json_deep_equals(json1, json2);
}

slay compare(json1 extra, json2 extra) normie {
    damn json_compare(json1, json2);
}

// ================================
// JSON Schema Functions
// ================================

slay validate_schema(json_data map, schema map) lit {
    damn json_validate_schema(json_data, schema);
}

slay get_schema_errors(json_data map, schema map) [tea] {
    damn json_get_schema_errors(json_data, schema);
}

// ================================
// JSON Utility Functions
// ================================

slay size(json_value extra) normie {
    damn json_size(json_value);
}

slay is_empty(json_value extra) lit {
    damn json_is_empty(json_value);
}

slay copy(json_value extra) extra {
    damn json_copy(json_value);
}

slay deep_copy(json_value extra) extra {
    damn json_deep_copy(json_value);
}

slay hash(json_value extra) normie {
    damn json_hash(json_value);
}

// ================================
// JSON Streaming Functions
// ================================

slay parse_stream(json_stream tea) [extra] {
    damn json_parse_stream(json_stream);
}

slay stringify_stream(data [extra]) tea {
    damn json_stringify_stream(data);
}

// ================================
// JSON Error Functions
// ================================

slay get_last_error() tea {
    damn json_get_last_error();
}

slay clear_errors() {
    json_clear_errors();
}

slay has_errors() lit {
    damn json_has_errors();
}

// ================================
// JSON Escaping Functions
// ================================

slay escape_string(str tea) tea {
    damn json_escape_string(str);
}

slay unescape_string(str tea) tea {
    damn json_unescape_string(str);
}

slay escape_unicode(str tea) tea {
    damn json_escape_unicode(str);
}

slay unescape_unicode(str tea) tea {
    damn json_unescape_unicode(str);
}
