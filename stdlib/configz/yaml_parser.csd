fr fr CONFIGZ YAML PARSER - Advanced YAML Processing
fr fr Comprehensive YAML parsing with nested structures and arrays

yeet "stringz"
yeet "vibez"

fr fr ===== YAML PARSER STRUCTURES =====

squad YamlNode {
    sus type tea                    fr fr "scalar", "sequence", "mapping"
    sus scalar_value tea
    sus sequence_items []YamlNode
    sus mapping_keys []tea
    sus mapping_values []YamlNode
    sus indent_level drip
    sus line_number drip
}

squad YamlParser {
    sus content tea
    sus lines []tea
    sus current_line drip
    sus total_lines drip
    sus indent_stack []drip
    sus key_stack []tea
    sus error_message tea
    sus has_error lit
}

fr fr ===== YAML PARSING FUNCTIONS =====

slay yaml_parse(yaml_content tea) YamlNode {
    fr fr Parse YAML content into structured nodes
    sus parser YamlParser = YamlParser{}
    parser.content = yaml_content
    parser.lines = split_lines(yaml_content)
    parser.current_line = 0
    parser.total_lines = array_length(parser.lines)
    parser.indent_stack = []
    parser.key_stack = []
    parser.has_error = cringe
    parser.error_message = ""
    
    sus root_node YamlNode = parse_yaml_document(parser)
    
    ready (parser.has_error) {
        vibez.spill("YAML Parse Error: " + parser.error_message)
        sus empty_node YamlNode = create_empty_yaml_node()
        damn empty_node
    }
    
    damn root_node
}

slay parse_yaml_document(parser YamlParser) YamlNode {
    fr fr Parse complete YAML document
    sus root_node YamlNode = YamlNode{}
    root_node.type = "mapping"
    root_node.mapping_keys = []
    root_node.mapping_values = []
    root_node.indent_level = 0
    root_node.line_number = 1
    
    bestie (parser.current_line < parser.total_lines) {
        sus line tea = parser.lines[parser.current_line]
        sus trimmed_line tea = trim_yaml_line(line)
        
        fr fr Skip empty lines and comments
        ready (trimmed_line == "" || starts_with(trimmed_line, "#")) {
            parser.current_line = parser.current_line + 1
            continue
        }
        
        fr fr Process mapping entry
        ready (is_yaml_mapping_line(trimmed_line)) {
            sus key_value YamlKeyValue = parse_yaml_mapping_line(trimmed_line)
            sus node YamlNode = parse_yaml_value(parser, key_value.value, get_line_indent(line))
            
            sus key_count drip = array_length(root_node.mapping_keys)
            root_node.mapping_keys[key_count] = key_value.key
            root_node.mapping_values[key_count] = node
        }
        
        parser.current_line = parser.current_line + 1
    }
    
    damn root_node
}

slay parse_yaml_value(parser YamlParser, value_str tea, indent drip) YamlNode {
    fr fr Parse YAML value (scalar, sequence, or mapping)
    sus node YamlNode = YamlNode{}
    node.indent_level = indent
    node.line_number = parser.current_line + 1
    
    ready (value_str == "" || value_str == "~") {
        fr fr Empty value or null
        node.type = "scalar"
        node.scalar_value = ""
    } otherwise ready (starts_with(value_str, "-")) {
        fr fr Inline sequence
        node = parse_inline_yaml_sequence(value_str)
    } otherwise ready (starts_with(value_str, "{")) {
        fr fr Inline mapping
        node = parse_inline_yaml_mapping(value_str)
    } otherwise ready (starts_with(value_str, "[")) {
        fr fr Inline array
        node = parse_inline_yaml_array(value_str)
    } otherwise ready (value_str == "|" || value_str == ">") {
        fr fr Block scalar (literal or folded)
        node = parse_yaml_block_scalar(parser, value_str)
    } otherwise {
        fr fr Regular scalar value
        node.type = "scalar"
        node.scalar_value = parse_yaml_scalar_value(value_str)
    }
    
    fr fr Check for nested structure
    ready (value_str == "" && has_nested_content(parser, indent)) {
        node = parse_nested_yaml_structure(parser, indent)
    }
    
    damn node
}

slay parse_nested_yaml_structure(parser YamlParser, parent_indent drip) YamlNode {
    fr fr Parse nested YAML structure (mapping or sequence)
    sus peek_line drip = parser.current_line + 1
    
    ready (peek_line >= parser.total_lines) {
        sus empty_node YamlNode = create_empty_yaml_node()
        damn empty_node
    }
    
    sus next_line tea = parser.lines[peek_line]
    sus next_trimmed tea = trim_yaml_line(next_line)
    sus next_indent drip = get_line_indent(next_line)
    
    ready (next_indent <= parent_indent) {
        sus empty_node YamlNode = create_empty_yaml_node()
        damn empty_node
    }
    
    fr fr Determine if it's a sequence or mapping
    ready (starts_with(next_trimmed, "- ")) {
        damn parse_yaml_sequence(parser, parent_indent)
    } otherwise ready (contains_string(next_trimmed, ":")) {
        damn parse_yaml_mapping(parser, parent_indent)
    } otherwise {
        sus scalar_node YamlNode = YamlNode{}
        scalar_node.type = "scalar"
        scalar_node.scalar_value = next_trimmed
        damn scalar_node
    }
}

slay parse_yaml_sequence(parser YamlParser, parent_indent drip) YamlNode {
    fr fr Parse YAML sequence (array)
    sus sequence_node YamlNode = YamlNode{}
    sequence_node.type = "sequence"
    sequence_node.sequence_items = []
    sequence_node.indent_level = parent_indent
    
    sus item_count drip = 0
    
    bestie (parser.current_line + 1 < parser.total_lines) {
        sus next_line tea = parser.lines[parser.current_line + 1]
        sus next_trimmed tea = trim_yaml_line(next_line)
        sus next_indent drip = get_line_indent(next_line)
        
        fr fr Break if we've moved past this sequence
        ready (next_indent <= parent_indent && next_trimmed != "") {
            break
        }
        
        fr fr Skip empty lines
        ready (next_trimmed == "") {
            parser.current_line = parser.current_line + 1
            continue
        }
        
        fr fr Process sequence item
        ready (starts_with(next_trimmed, "- ")) {
            parser.current_line = parser.current_line + 1
            sus item_value tea = substring(next_trimmed, 2, string_length(next_trimmed) - 2)
            sus item_node YamlNode = parse_yaml_value(parser, trim_yaml_line(item_value), next_indent)
            sequence_node.sequence_items[item_count] = item_node
            item_count = item_count + 1
        } otherwise {
            break
        }
    }
    
    damn sequence_node
}

slay parse_yaml_mapping(parser YamlParser, parent_indent drip) YamlNode {
    fr fr Parse YAML mapping (object)
    sus mapping_node YamlNode = YamlNode{}
    mapping_node.type = "mapping"
    mapping_node.mapping_keys = []
    mapping_node.mapping_values = []
    mapping_node.indent_level = parent_indent
    
    sus pair_count drip = 0
    
    bestie (parser.current_line + 1 < parser.total_lines) {
        sus next_line tea = parser.lines[parser.current_line + 1]
        sus next_trimmed tea = trim_yaml_line(next_line)
        sus next_indent drip = get_line_indent(next_line)
        
        fr fr Break if we've moved past this mapping
        ready (next_indent <= parent_indent && next_trimmed != "") {
            break
        }
        
        fr fr Skip empty lines and comments
        ready (next_trimmed == "" || starts_with(next_trimmed, "#")) {
            parser.current_line = parser.current_line + 1
            continue
        }
        
        fr fr Process mapping pair
        ready (is_yaml_mapping_line(next_trimmed) && next_indent > parent_indent) {
            parser.current_line = parser.current_line + 1
            sus key_value YamlKeyValue = parse_yaml_mapping_line(next_trimmed)
            sus value_node YamlNode = parse_yaml_value(parser, key_value.value, next_indent)
            
            mapping_node.mapping_keys[pair_count] = key_value.key
            mapping_node.mapping_values[pair_count] = value_node
            pair_count = pair_count + 1
        } otherwise {
            break
        }
    }
    
    damn mapping_node
}

fr fr ===== YAML PARSING UTILITIES =====

squad YamlKeyValue {
    sus key tea
    sus value tea
}

slay parse_yaml_mapping_line(line tea) YamlKeyValue {
    fr fr Parse key: value line
    sus colon_pos drip = find_char(line, ":")
    sus result YamlKeyValue = YamlKeyValue{}
    
    ready (colon_pos > 0) {
        result.key = trim_yaml_line(substring(line, 0, colon_pos))
        sus after_colon tea = substring(line, colon_pos + 1, string_length(line) - colon_pos - 1)
        result.value = trim_yaml_line(after_colon)
        
        fr fr Remove quotes from key if present
        ready (starts_with(result.key, "\"") && ends_with(result.key, "\"")) {
            result.key = substring(result.key, 1, string_length(result.key) - 2)
        }
        ready (starts_with(result.key, "'") && ends_with(result.key, "'")) {
            result.key = substring(result.key, 1, string_length(result.key) - 2)
        }
    } otherwise {
        result.key = trim_yaml_line(line)
        result.value = ""
    }
    
    damn result
}

slay parse_yaml_scalar_value(value_str tea) tea {
    fr fr Parse scalar YAML value with proper type conversion
    sus trimmed tea = trim_yaml_line(value_str)
    
    fr fr Handle quoted strings
    ready (starts_with(trimmed, "\"") && ends_with(trimmed, "\"")) {
        damn substring(trimmed, 1, string_length(trimmed) - 2)
    }
    
    ready (starts_with(trimmed, "'") && ends_with(trimmed, "'")) {
        damn substring(trimmed, 1, string_length(trimmed) - 2)
    }
    
    fr fr Handle special YAML values
    ready (trimmed == "null" || trimmed == "~" || trimmed == "NULL") {
        damn ""
    }
    
    ready (trimmed == "true" || trimmed == "True" || trimmed == "TRUE") {
        damn "true"
    }
    
    ready (trimmed == "false" || trimmed == "False" || trimmed == "FALSE") {
        damn "false"
    }
    
    fr fr Return as-is for other values
    damn trimmed
}

slay parse_inline_yaml_sequence(sequence_str tea) YamlNode {
    fr fr Parse inline YAML sequence like "- item1 - item2"
    sus node YamlNode = YamlNode{}
    node.type = "sequence"
    node.sequence_items = []
    
    fr fr Simple parsing for inline sequences
    sus items []tea = split_yaml_inline_sequence(sequence_str)
    sus item_count drip = array_length(items)
    
    sus i drip = 0
    bestie (i < item_count) {
        sus item_node YamlNode = YamlNode{}
        item_node.type = "scalar"
        item_node.scalar_value = parse_yaml_scalar_value(items[i])
        node.sequence_items[i] = item_node
        i = i + 1
    }
    
    damn node
}

slay parse_inline_yaml_mapping(mapping_str tea) YamlNode {
    fr fr Parse inline YAML mapping like "{key: value, key2: value2}"
    sus node YamlNode = YamlNode{}
    node.type = "mapping"
    node.mapping_keys = []
    node.mapping_values = []
    
    fr fr Remove braces and split by commas
    sus inner tea = substring(mapping_str, 1, string_length(mapping_str) - 2)
    sus pairs []tea = split_string(inner, ",", 0)
    sus pair_count drip = array_length(pairs)
    
    sus i drip = 0
    bestie (i < pair_count) {
        sus pair tea = trim_yaml_line(pairs[i])
        sus colon_pos drip = find_char(pair, ":")
        
        ready (colon_pos > 0) {
            sus key tea = trim_yaml_line(substring(pair, 0, colon_pos))
            sus value tea = trim_yaml_line(substring(pair, colon_pos + 1, string_length(pair) - colon_pos - 1))
            
            node.mapping_keys[i] = parse_yaml_scalar_value(key)
            
            sus value_node YamlNode = YamlNode{}
            value_node.type = "scalar"
            value_node.scalar_value = parse_yaml_scalar_value(value)
            node.mapping_values[i] = value_node
        }
        
        i = i + 1
    }
    
    damn node
}

slay parse_inline_yaml_array(array_str tea) YamlNode {
    fr fr Parse inline YAML array like "[item1, item2, item3]"
    sus node YamlNode = YamlNode{}
    node.type = "sequence"
    node.sequence_items = []
    
    fr fr Remove brackets and split by commas
    sus inner tea = substring(array_str, 1, string_length(array_str) - 2)
    sus items []tea = split_string(inner, ",", 0)
    sus item_count drip = array_length(items)
    
    sus i drip = 0
    bestie (i < item_count) {
        sus item tea = trim_yaml_line(items[i])
        sus item_node YamlNode = YamlNode{}
        item_node.type = "scalar"
        item_node.scalar_value = parse_yaml_scalar_value(item)
        node.sequence_items[i] = item_node
        i = i + 1
    }
    
    damn node
}

slay parse_yaml_block_scalar(parser YamlParser, indicator tea) YamlNode {
    fr fr Parse YAML block scalar (| for literal, > for folded)
    sus node YamlNode = YamlNode{}
    node.type = "scalar"
    
    sus block_content tea = ""
    sus base_indent drip = -1
    
    bestie (parser.current_line + 1 < parser.total_lines) {
        parser.current_line = parser.current_line + 1
        sus line tea = parser.lines[parser.current_line]
        
        ready (trim_yaml_line(line) == "") {
            ready (indicator == "|") {
                block_content = block_content + "\n"
            } otherwise {
                block_content = block_content + " "
            }
            continue
        }
        
        sus line_indent drip = get_line_indent(line)
        ready (base_indent == -1) {
            base_indent = line_indent
        }
        
        ready (line_indent < base_indent) {
            parser.current_line = parser.current_line - 1
            break
        }
        
        sus content tea = substring(line, base_indent, string_length(line) - base_indent)
        ready (block_content != "") {
            ready (indicator == "|") {
                block_content = block_content + "\n" + content
            } otherwise {
                block_content = block_content + " " + content
            }
        } otherwise {
            block_content = content
        }
    }
    
    node.scalar_value = block_content
    damn node
}

fr fr ===== YAML UTILITY FUNCTIONS =====

slay is_yaml_mapping_line(line tea) lit {
    fr fr Check if line contains a mapping (key: value)
    sus colon_pos drip = find_char(line, ":")
    ready (colon_pos <= 0) {
        damn cringe
    }
    
    fr fr Make sure colon is not in quotes
    sus before_colon tea = substring(line, 0, colon_pos)
    sus quote_count drip = count_unescaped_quotes(before_colon)
    damn (quote_count % 2 == 0)  fr fr Even number means colon is outside quotes
}

slay has_nested_content(parser YamlParser, current_indent drip) lit {
    fr fr Check if there's nested content at a higher indent level
    sus peek_line drip = parser.current_line + 1
    
    ready (peek_line >= parser.total_lines) {
        damn cringe
    }
    
    sus next_line tea = parser.lines[peek_line]
    ready (trim_yaml_line(next_line) == "") {
        damn cringe  fr fr Empty line doesn't count as nested content
    }
    
    sus next_indent drip = get_line_indent(next_line)
    damn (next_indent > current_indent)
}

slay get_line_indent(line tea) drip {
    fr fr Count leading whitespace characters
    sus length drip = string_length(line)
    sus indent drip = 0
    
    sus i drip = 0
    bestie (i < length) {
        sus char tea = substring(line, i, 1)
        ready (char == " ") {
            indent = indent + 1
        } otherwise ready (char == "\t") {
            indent = indent + 8  fr fr Tab counts as 8 spaces
        } otherwise {
            break
        }
        i = i + 1
    }
    
    damn indent
}

slay trim_yaml_line(line tea) tea {
    fr fr Trim whitespace and handle YAML-specific trimming
    sus trimmed tea = trim_string(line)
    
    fr fr Remove trailing comments (but not those in quotes)
    sus comment_pos drip = find_unquoted_char(trimmed, "#")
    ready (comment_pos >= 0) {
        trimmed = trim_string(substring(trimmed, 0, comment_pos))
    }
    
    damn trimmed
}

slay find_unquoted_char(line tea, char tea) drip {
    fr fr Find character that's not inside quotes
    sus length drip = string_length(line)
    sus in_single_quotes lit = cringe
    sus in_double_quotes lit = cringe
    sus escaped lit = cringe
    
    sus i drip = 0
    bestie (i < length) {
        sus current tea = substring(line, i, 1)
        
        ready (escaped) {
            escaped = cringe
            i = i + 1
            continue
        }
        
        ready (current == "\\") {
            escaped = based
        } otherwise ready (current == "'" && !in_double_quotes) {
            in_single_quotes = !in_single_quotes
        } otherwise ready (current == "\"" && !in_single_quotes) {
            in_double_quotes = !in_double_quotes
        } otherwise ready (current == char && !in_single_quotes && !in_double_quotes) {
            damn i
        }
        
        i = i + 1
    }
    
    damn -1
}

slay count_unescaped_quotes(str tea) drip {
    fr fr Count unescaped quote characters
    sus length drip = string_length(str)
    sus count drip = 0
    sus escaped lit = cringe
    
    sus i drip = 0
    bestie (i < length) {
        sus char tea = substring(str, i, 1)
        
        ready (escaped) {
            escaped = cringe
        } otherwise ready (char == "\\") {
            escaped = based
        } otherwise ready (char == "\"" || char == "'") {
            count = count + 1
        }
        
        i = i + 1
    }
    
    damn count
}

slay split_yaml_inline_sequence(sequence_str tea) []tea {
    fr fr Split inline sequence by dashes
    sus items []tea = []
    sus current_item tea = ""
    sus item_count drip = 0
    sus length drip = string_length(sequence_str)
    
    sus i drip = 0
    bestie (i < length) {
        sus char tea = substring(str, i, 1)
        
        ready (char == "-" && (i == 0 || substring(sequence_str, i - 1, 1) == " ")) {
            ready (current_item != "") {
                items[item_count] = trim_yaml_line(current_item)
                item_count = item_count + 1
                current_item = ""
            }
        } otherwise {
            current_item = current_item + char
        }
        
        i = i + 1
    }
    
    ready (current_item != "") {
        items[item_count] = trim_yaml_line(current_item)
    }
    
    damn items
}

slay create_empty_yaml_node() YamlNode {
    fr fr Create empty YAML node
    sus node YamlNode = YamlNode{}
    node.type = "scalar"
    node.scalar_value = ""
    node.sequence_items = []
    node.mapping_keys = []
    node.mapping_values = []
    node.indent_level = 0
    node.line_number = 0
    damn node
}

fr fr ===== YAML TO CONFIG CONVERSION =====

slay yaml_node_to_config_values(node YamlNode, prefix tea, config ConfigManager) ConfigManager {
    fr fr Convert YAML node to configuration values
    ready (node.type == "scalar") {
        sus config_value ConfigValue = ConfigValue{}
        config_value.type = "string"
        config_value.string_value = node.scalar_value
        config_value.source = "file"
        config_value = auto_detect_type(config_value)
        
        map_set_string(config.values, prefix, config_value)
    } otherwise ready (node.type == "sequence") {
        sus config_value ConfigValue = ConfigValue{}
        config_value.type = "array"
        config_value.source = "file"
        config_value.array_values = []
        
        sus item_count drip = array_length(node.sequence_items)
        sus i drip = 0
        bestie (i < item_count) {
            sus item_config ConfigValue = yaml_node_to_config_value(node.sequence_items[i])
            config_value.array_values[i] = item_config
            i = i + 1
        }
        
        map_set_string(config.values, prefix, config_value)
    } otherwise ready (node.type == "mapping") {
        sus key_count drip = array_length(node.mapping_keys)
        sus i drip = 0
        bestie (i < key_count) {
            sus key tea = node.mapping_keys[i]
            sus value_node YamlNode = node.mapping_values[i]
            
            sus full_key tea
            ready (prefix == "") {
                full_key = key
            } otherwise {
                full_key = prefix + "." + key
            }
            
            config = yaml_node_to_config_values(value_node, full_key, config)
            i = i + 1
        }
    }
    
    damn config
}

slay yaml_node_to_config_value(node YamlNode) ConfigValue {
    fr fr Convert single YAML node to configuration value
    sus config_value ConfigValue = ConfigValue{}
    config_value.source = "file"
    
    ready (node.type == "scalar") {
        config_value.type = "string"
        config_value.string_value = node.scalar_value
        config_value = auto_detect_type(config_value)
    } otherwise {
        config_value.type = "string"
        config_value.string_value = ""
    }
    
    damn config_value
}

fr fr ===== YAML GENERATION =====

slay yaml_generate(node YamlNode, indent_level drip) tea {
    fr fr Generate YAML string from node
    ready (node.type == "scalar") {
        damn yaml_format_scalar(node.scalar_value)
    } otherwise ready (node.type == "sequence") {
        damn yaml_generate_sequence(node, indent_level)
    } otherwise ready (node.type == "mapping") {
        damn yaml_generate_mapping(node, indent_level)
    } otherwise {
        damn ""
    }
}

slay yaml_generate_mapping(node YamlNode, indent_level drip) tea {
    fr fr Generate YAML mapping
    sus result tea = ""
    sus indent_str tea = repeat_string(" ", indent_level)
    sus key_count drip = array_length(node.mapping_keys)
    
    sus i drip = 0
    bestie (i < key_count) {
        sus key tea = node.mapping_keys[i]
        sus value_node YamlNode = node.mapping_values[i]
        
        ready (i > 0) {
            result = result + "\n"
        }
        
        result = result + indent_str + key + ":"
        
        ready (value_node.type == "scalar") {
            result = result + " " + yaml_generate(value_node, 0)
        } otherwise {
            result = result + "\n" + yaml_generate(value_node, indent_level + 2)
        }
        
        i = i + 1
    }
    
    damn result
}

slay yaml_generate_sequence(node YamlNode, indent_level drip) tea {
    fr fr Generate YAML sequence
    sus result tea = ""
    sus indent_str tea = repeat_string(" ", indent_level)
    sus item_count drip = array_length(node.sequence_items)
    
    sus i drip = 0
    bestie (i < item_count) {
        sus item_node YamlNode = node.sequence_items[i]
        
        ready (i > 0) {
            result = result + "\n"
        }
        
        result = result + indent_str + "- "
        
        ready (item_node.type == "scalar") {
            result = result + yaml_generate(item_node, 0)
        } otherwise {
            result = result + "\n" + yaml_generate(item_node, indent_level + 2)
        }
        
        i = i + 1
    }
    
    damn result
}

slay yaml_format_scalar(value tea) tea {
    fr fr Format scalar value for YAML output
    ready (value == "") {
        damn "null"
    }
    
    ready (contains_string(value, "\n") || contains_string(value, ":") || 
           contains_string(value, "#") || contains_string(value, "[")) {
        damn "\"" + escape_yaml_string(value) + "\""
    }
    
    damn value
}

slay escape_yaml_string(str tea) tea {
    fr fr Escape special characters in YAML string
    sus result tea = ""
    sus length drip = string_length(str)
    
    sus i drip = 0
    bestie (i < length) {
        sus char tea = substring(str, i, 1)
        
        ready (char == "\"") {
            result = result + "\\\""
        } otherwise ready (char == "\\") {
            result = result + "\\\\"
        } otherwise ready (char == "\n") {
            result = result + "\\n"
        } otherwise ready (char == "\r") {
            result = result + "\\r"
        } otherwise ready (char == "\t") {
            result = result + "\\t"
        } otherwise {
            result = result + char
        }
        
        i = i + 1
    }
    
    damn result
}

slay repeat_string(str tea, count drip) tea {
    fr fr Repeat string count times
    sus result tea = ""
    sus i drip = 0
    bestie (i < count) {
        result = result + str
        i = i + 1
    }
    damn result
}
