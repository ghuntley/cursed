yeet "testz"
yeet "runtime_core"

fr fr ================================
fr fr CURSED Enhanced String Library v1.0
fr fr Compiler runtime optimized strings
fr fr Pure CURSED implementation
fr fr ================================

fr fr Import basic string operations
yeet "string"

fr fr Advanced string operations for compiler runtime

fr fr String interning for identifier optimization
squad StringIntern {
    spill strings RuntimeHashMap<tea, normie>
    spill pool RuntimeVec<tea>
    spill next_id normie
}

slay StringIntern_new() StringIntern {
    damn StringIntern{
        strings: RuntimeHashMap_new<tea, normie>(),
        pool: RuntimeVec_new<tea>(),
        next_id: 0
    }
}

slay StringIntern_intern(intern StringIntern, str tea) (normie, StringIntern) {
    (id, found) := RuntimeHashMap_get(intern.strings, str)
    vibes found {
        damn (id, intern)
    }
    
    fr fr Add new string to pool
    sus new_id normie = intern.next_id
    intern.pool = RuntimeVec_push(intern.pool, str)
    intern.strings = RuntimeHashMap_insert(intern.strings, str, new_id)
    intern.next_id = intern.next_id + 1
    
    damn (new_id, intern)
}

slay StringIntern_get(intern StringIntern, id normie) tea {
    vibes id >= 0 && id < RuntimeVec_len(intern.pool) {
        damn RuntimeVec_get(intern.pool, id)
    }
    damn ""
}

fr fr String scanning for lexer operations
squad StringScanner {
    spill source tea
    spill position normie
    spill line normie
    spill column normie
    spill length normie
}

slay StringScanner_new(source tea) StringScanner {
    damn StringScanner{
        source: source,
        position: 0,
        line: 1,
        column: 1,
        length: string_length(source)
    }
}

slay StringScanner_current_char(scanner StringScanner) sip {
    vibes scanner.position >= scanner.length {
        damn '\0'
    }
    damn string_char_at(scanner.source, scanner.position)
}

slay StringScanner_peek_char(scanner StringScanner, offset normie) sip {
    sus pos normie = scanner.position + offset
    vibes pos >= scanner.length {
        damn '\0'
    }
    damn string_char_at(scanner.source, pos)
}

slay StringScanner_advance(scanner StringScanner) StringScanner {
    vibes scanner.position < scanner.length {
        sus ch sip = StringScanner_current_char(scanner)
        scanner.position = scanner.position + 1
        
        vibes ch == '\n' {
            scanner.line = scanner.line + 1
            scanner.column = 1
        } nah {
            scanner.column = scanner.column + 1
        }
    }
    damn scanner
}

slay StringScanner_skip_whitespace(scanner StringScanner) StringScanner {
    bestie scanner.position < scanner.length {
        sus ch sip = StringScanner_current_char(scanner)
        vibes ch == ' ' || ch == '\t' || ch == '\r' || ch == '\n' {
            scanner = StringScanner_advance(scanner)
        } nah {
            break
        }
    }
    damn scanner
}

slay StringScanner_read_while(scanner StringScanner, predicate tea) (tea, StringScanner) {
    sus start normie = scanner.position
    
    bestie scanner.position < scanner.length {
        sus ch sip = StringScanner_current_char(scanner)
        vibes StringScanner_char_matches_predicate(ch, predicate) {
            scanner = StringScanner_advance(scanner)
        } nah {
            break
        }
    }
    
    sus result tea = string_substring(scanner.source, start, scanner.position)
    damn (result, scanner)
}

slay StringScanner_char_matches_predicate(ch sip, predicate tea) lit {
    vibes predicate == "alpha" {
        damn StringScanner_is_alpha(ch)
    } elseif predicate == "digit" {
        damn StringScanner_is_digit(ch)
    } elseif predicate == "alnum" {
        damn StringScanner_is_alpha(ch) || StringScanner_is_digit(ch)
    } elseif predicate == "ident" {
        damn StringScanner_is_alpha(ch) || StringScanner_is_digit(ch) || ch == '_'
    }
    damn cringe
}

slay StringScanner_is_alpha(ch sip) lit {
    damn (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z')
}

slay StringScanner_is_digit(ch sip) lit {
    damn ch >= '0' && ch <= '9'
}

slay StringScanner_is_at_end(scanner StringScanner) lit {
    damn scanner.position >= scanner.length
}

slay StringScanner_remaining(scanner StringScanner) tea {
    vibes scanner.position >= scanner.length {
        damn ""
    }
    damn string_substring(scanner.source, scanner.position, scanner.length)
}

fr fr String formatting for code generation
slay format_function_signature(name tea, params []tea, return_type tea) tea {
    sus sb RuntimeStringBuilder = RuntimeStringBuilder_new()
    sb = RuntimeStringBuilder_append(sb, "slay ")
    sb = RuntimeStringBuilder_append(sb, name)
    sb = RuntimeStringBuilder_append_char(sb, '(')
    
    bestie i := 0; i < len(params); i = i + 1 {
        vibes i > 0 {
            sb = RuntimeStringBuilder_append(sb, ", ")
        }
        sb = RuntimeStringBuilder_append(sb, params[i])
    }
    
    sb = RuntimeStringBuilder_append_char(sb, ')')
    vibes return_type != "" {
        sb = RuntimeStringBuilder_append_char(sb, ' ')
        sb = RuntimeStringBuilder_append(sb, return_type)
    }
    
    damn RuntimeStringBuilder_to_string(sb)
}

slay format_variable_declaration(name tea, type tea, value tea) tea {
    sus sb RuntimeStringBuilder = RuntimeStringBuilder_new()
    sb = RuntimeStringBuilder_append(sb, "sus ")
    sb = RuntimeStringBuilder_append(sb, name)
    vibes type != "" {
        sb = RuntimeStringBuilder_append_char(sb, ' ')
        sb = RuntimeStringBuilder_append(sb, type)
    }
    vibes value != "" {
        sb = RuntimeStringBuilder_append(sb, " = ")
        sb = RuntimeStringBuilder_append(sb, value)
    }
    damn RuntimeStringBuilder_to_string(sb)
}

slay format_array_type(element_type tea) tea {
    damn "[" + element_type + "]"
}

slay format_function_call(name tea, args []tea) tea {
    sus sb RuntimeStringBuilder = RuntimeStringBuilder_new()
    sb = RuntimeStringBuilder_append(sb, name)
    sb = RuntimeStringBuilder_append_char(sb, '(')
    
    bestie i := 0; i < len(args); i = i + 1 {
        vibes i > 0 {
            sb = RuntimeStringBuilder_append(sb, ", ")
        }
        sb = RuntimeStringBuilder_append(sb, args[i])
    }
    
    sb = RuntimeStringBuilder_append_char(sb, ')')
    damn RuntimeStringBuilder_to_string(sb)
}

fr fr Identifier validation for compiler
slay is_valid_identifier(name tea) lit {
    vibes string_length(name) == 0 {
        damn cringe
    }
    
    fr fr First character must be letter or underscore
    sus first sip = string_char_at(name, 0)
    vibes !StringScanner_is_alpha(first) && first != '_' {
        damn cringe
    }
    
    fr fr Remaining characters must be alphanumeric or underscore
    bestie i := 1; i < string_length(name); i = i + 1 {
        sus ch sip = string_char_at(name, i)
        vibes !StringScanner_is_alpha(ch) && !StringScanner_is_digit(ch) && ch != '_' {
            damn cringe
        }
    }
    
    damn based
}

slay is_cursed_keyword(word tea) lit {
    vibes word == "slay" || word == "sus" || word == "damn" || word == "vibes" {
        damn based
    } elseif word == "bestie" || word == "lowkey" || word == "yeet" || word == "stan" {
        damn based
    } elseif word == "squad" || word == "collab" || word == "flex" || word == "spill" {
        damn based
    } elseif word == "facts" || word == "tea" || word == "normie" || word == "lit" {
        damn based
    } elseif word == "based" || word == "cringe" || word == "cap" || word == "extra" {
        damn based
    } elseif word == "meal" || word == "sip" || word == "drip" || word == "thicc" {
        damn based
    } elseif word == "smol" || word == "fr" || word == "nah" || word == "elseif" {
        damn based
    } elseif word == "vibez" || word == "break" || word == "continue" || word == "match" {
        damn based
    }
    damn cringe
}

fr fr String escape handling for string literals
slay escape_string_literal(str tea) tea {
    sus sb RuntimeStringBuilder = RuntimeStringBuilder_new()
    sb = RuntimeStringBuilder_append_char(sb, '"')
    
    bestie i := 0; i < string_length(str); i = i + 1 {
        sus ch sip = string_char_at(str, i)
        vibes ch == '"' {
            sb = RuntimeStringBuilder_append(sb, "\\\"")
        } elseif ch == '\\' {
            sb = RuntimeStringBuilder_append(sb, "\\\\")
        } elseif ch == '\n' {
            sb = RuntimeStringBuilder_append(sb, "\\n")
        } elseif ch == '\t' {
            sb = RuntimeStringBuilder_append(sb, "\\t")
        } elseif ch == '\r' {
            sb = RuntimeStringBuilder_append(sb, "\\r")
        } nah {
            sb = RuntimeStringBuilder_append_char(sb, ch)
        }
    }
    
    sb = RuntimeStringBuilder_append_char(sb, '"')
    damn RuntimeStringBuilder_to_string(sb)
}

slay unescape_string_literal(str tea) tea {
    vibes string_length(str) < 2 {
        damn str
    }
    
    fr fr Remove surrounding quotes
    sus content tea = string_substring(str, 1, string_length(str) - 1)
    sus sb RuntimeStringBuilder = RuntimeStringBuilder_new()
    
    sus i normie = 0
    bestie i < string_length(content) {
        sus ch sip = string_char_at(content, i)
        vibes ch == '\\' && i + 1 < string_length(content) {
            sus next sip = string_char_at(content, i + 1)
            vibes next == 'n' {
                sb = RuntimeStringBuilder_append_char(sb, '\n')
            } elseif next == 't' {
                sb = RuntimeStringBuilder_append_char(sb, '\t')
            } elseif next == 'r' {
                sb = RuntimeStringBuilder_append_char(sb, '\r')
            } elseif next == '\\' {
                sb = RuntimeStringBuilder_append_char(sb, '\\')
            } elseif next == '"' {
                sb = RuntimeStringBuilder_append_char(sb, '"')
            } nah {
                sb = RuntimeStringBuilder_append_char(sb, ch)
                sb = RuntimeStringBuilder_append_char(sb, next)
            }
            i = i + 2
        } nah {
            sb = RuntimeStringBuilder_append_char(sb, ch)
            i = i + 1
        }
    }
    
    damn RuntimeStringBuilder_to_string(sb)
}

fr fr Path manipulation for module resolution
slay normalize_module_path(path tea) tea {
    vibes string_contains(path, "..") {
        fr fr Remove parent directory references
        damn string_replace_all(path, "../", "")
    }
    damn path
}

slay module_path_to_file_path(module_path tea) tea {
    sus file_path tea = string_replace_all(module_path, "::", "/")
    damn file_path + ".csd"
}

slay file_path_to_module_path(file_path tea) tea {
    sus module_path tea = file_path
    vibes string_ends_with(module_path, ".csd") {
        module_path = string_substring(module_path, 0, string_length(module_path) - 4)
    }
    damn string_replace_all(module_path, "/", "::")
}

fr fr Indentation management for code generation
squad IndentationManager {
    spill level normie
    spill indent_string tea
}

slay IndentationManager_new(indent_string tea) IndentationManager {
    damn IndentationManager{
        level: 0,
        indent_string: indent_string
    }
}

slay IndentationManager_increase(manager IndentationManager) IndentationManager {
    manager.level = manager.level + 1
    damn manager
}

slay IndentationManager_decrease(manager IndentationManager) IndentationManager {
    vibes manager.level > 0 {
        manager.level = manager.level - 1
    }
    damn manager
}

slay IndentationManager_current_indent(manager IndentationManager) tea {
    sus sb RuntimeStringBuilder = RuntimeStringBuilder_new()
    bestie i := 0; i < manager.level; i = i + 1 {
        sb = RuntimeStringBuilder_append(sb, manager.indent_string)
    }
    damn RuntimeStringBuilder_to_string(sb)
}

slay IndentationManager_indent_line(manager IndentationManager, line tea) tea {
    sus indent tea = IndentationManager_current_indent(manager)
    damn indent + line
}

fr fr Case conversion utilities
slay to_snake_case(str tea) tea {
    sus sb RuntimeStringBuilder = RuntimeStringBuilder_new()
    
    bestie i := 0; i < string_length(str); i = i + 1 {
        sus ch sip = string_char_at(str, i)
        vibes ch >= 'A' && ch <= 'Z' {
            vibes i > 0 {
                sb = RuntimeStringBuilder_append_char(sb, '_')
            }
            sus lower_ch sip = sip(char_to_int(ch) + 32)
            sb = RuntimeStringBuilder_append_char(sb, lower_ch)
        } nah {
            sb = RuntimeStringBuilder_append_char(sb, ch)
        }
    }
    
    damn RuntimeStringBuilder_to_string(sb)
}

slay to_pascal_case(str tea) tea {
    sus sb RuntimeStringBuilder = RuntimeStringBuilder_new()
    sus next_upper lit = based
    
    bestie i := 0; i < string_length(str); i = i + 1 {
        sus ch sip = string_char_at(str, i)
        vibes ch == '_' {
            next_upper = based
        } elseif next_upper && ch >= 'a' && ch <= 'z' {
            sus upper_ch sip = sip(char_to_int(ch) - 32)
            sb = RuntimeStringBuilder_append_char(sb, upper_ch)
            next_upper = cringe
        } nah {
            sb = RuntimeStringBuilder_append_char(sb, ch)
            next_upper = cringe
        }
    }
    
    damn RuntimeStringBuilder_to_string(sb)
}

fr fr Utility functions
slay char_to_int(ch sip) normie {
    damn runtime_char_to_ascii(ch)
}

slay len(arr []tea) normie {
    fr fr Runtime-provided array length
    damn runtime_slice_length(arr)
}

vibez.spill("🚀 CURSED Enhanced String Library v1.0 Loaded")
vibez.spill("✅ String interning, scanning, formatting")
vibez.spill("🔧 Identifier validation and escape handling")
vibez.spill("⚡ Optimized for compiler runtime operations")
