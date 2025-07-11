yeet "testz"

# RizzTemplate - A powerful template engine for CURSED
# Supports variable interpolation, conditionals, loops, and filters

struct TemplateContext {
    variables map[tea]tea
    filters map[tea]slay
}

struct RizzTemplate {
    template_content tea
    context TemplateContext
}

# Core template functions
slay rizz_template_new(content tea) RizzTemplate {
    sus ctx TemplateContext
    ctx.variables = make(map[tea]tea)
    ctx.filters = make(map[tea]slay)
    
    sus template RizzTemplate
    template.template_content = content
    template.context = ctx
    damn template
}

slay rizz_template_set_var(template *RizzTemplate, key tea, value tea) {
    template.context.variables[key] = value
}

slay rizz_template_get_var(template *RizzTemplate, key tea) tea {
    damn template.context.variables[key]
}

slay rizz_template_render(template *RizzTemplate) tea {
    sus result tea = template.template_content
    
    # Variable interpolation: {{variable}}
    result = rizz_interpolate_variables(result, template.context.variables)
    
    # Conditional rendering: {{if condition}}...{{endif}}
    result = rizz_process_conditionals(result, template.context.variables)
    
    # Loop rendering: {{for item in items}}...{{endfor}}
    result = rizz_process_loops(result, template.context.variables)
    
    damn result
}

# Variable interpolation engine
slay rizz_interpolate_variables(content tea, variables map[tea]tea) tea {
    sus result tea = content
    sus i normie = 0
    
    bestie i < len(result) {
        if result[i] == '{' && i + 1 < len(result) && result[i + 1] == '{' {
            sus end_pos normie = rizz_find_closing_braces(result, i)
            if end_pos > i {
                sus var_name tea = result[i+2:end_pos-2]
                var_name = rizz_trim_whitespace(var_name)
                
                if rizz_has_key(variables, var_name) {
                    sus value tea = variables[var_name]
                    result = result[:i] + value + result[end_pos:]
                    i = i + len(value)
                } else {
                    i = end_pos
                }
            } else {
                i++
            }
        } else {
            i++
        }
    }
    
    damn result
}

# Conditional processing engine
slay rizz_process_conditionals(content tea, variables map[tea]tea) tea {
    sus result tea = content
    sus i normie = 0
    
    bestie i < len(result) {
        if rizz_starts_with_at(result, i, "{{if ") {
            sus endif_pos normie = rizz_find_endif(result, i)
            if endif_pos > i {
                sus condition_end normie = rizz_find_closing_braces(result, i)
                sus condition tea = result[i+5:condition_end-2]
                condition = rizz_trim_whitespace(condition)
                
                sus content_start normie = condition_end
                sus content_end normie = endif_pos
                sus block_content tea = result[content_start:content_end]
                
                if rizz_evaluate_condition(condition, variables) {
                    result = result[:i] + block_content + result[content_end+9:]
                    i = i + len(block_content)
                } else {
                    result = result[:i] + result[content_end+9:]
                }
            } else {
                i++
            }
        } else {
            i++
        }
    }
    
    damn result
}

# Loop processing engine
slay rizz_process_loops(content tea, variables map[tea]tea) tea {
    sus result tea = content
    sus i normie = 0
    
    bestie i < len(result) {
        if rizz_starts_with_at(result, i, "{{for ") {
            sus endfor_pos normie = rizz_find_endfor(result, i)
            if endfor_pos > i {
                sus loop_def_end normie = rizz_find_closing_braces(result, i)
                sus loop_def tea = result[i+6:loop_def_end-2]
                loop_def = rizz_trim_whitespace(loop_def)
                
                sus content_start normie = loop_def_end
                sus content_end normie = endfor_pos
                sus loop_content tea = result[content_start:content_end]
                
                sus expanded_content tea = rizz_expand_loop(loop_def, loop_content, variables)
                result = result[:i] + expanded_content + result[content_end+10:]
                i = i + len(expanded_content)
            } else {
                i++
            }
        } else {
            i++
        }
    }
    
    damn result
}

# Helper functions
slay rizz_find_closing_braces(content tea, start normie) normie {
    sus depth normie = 0
    sus i normie = start
    
    bestie i < len(content) - 1 {
        if content[i] == '{' && content[i+1] == '{' {
            depth++
            i += 2
        } else if content[i] == '}' && content[i+1] == '}' {
            depth--
            if depth == 0 {
                damn i + 2
            }
            i += 2
        } else {
            i++
        }
    }
    
    damn -1
}

slay rizz_find_endif(content tea, start normie) normie {
    sus i normie = start
    sus depth normie = 0
    
    bestie i < len(content) - 8 {
        if rizz_starts_with_at(content, i, "{{if ") {
            depth++
            i += 5
        } else if rizz_starts_with_at(content, i, "{{endif}}") {
            depth--
            if depth == 0 {
                damn i
            }
            i += 9
        } else {
            i++
        }
    }
    
    damn -1
}

slay rizz_find_endfor(content tea, start normie) normie {
    sus i normie = start
    sus depth normie = 0
    
    bestie i < len(content) - 10 {
        if rizz_starts_with_at(content, i, "{{for ") {
            depth++
            i += 6
        } else if rizz_starts_with_at(content, i, "{{endfor}}") {
            depth--
            if depth == 0 {
                damn i
            }
            i += 10
        } else {
            i++
        }
    }
    
    damn -1
}

slay rizz_starts_with_at(content tea, pos normie, prefix tea) lit {
    if pos + len(prefix) > len(content) {
        damn cap
    }
    
    sus i normie = 0
    bestie i < len(prefix) {
        if content[pos + i] != prefix[i] {
            damn cap
        }
        i++
    }
    
    damn based
}

slay rizz_trim_whitespace(s tea) tea {
    sus start normie = 0
    sus end normie = len(s)
    
    bestie start < end && (s[start] == ' ' || s[start] == '\t' || s[start] == '\n') {
        start++
    }
    
    bestie end > start && (s[end-1] == ' ' || s[end-1] == '\t' || s[end-1] == '\n') {
        end--
    }
    
    damn s[start:end]
}

slay rizz_has_key(m map[tea]tea, key tea) lit {
    sus _ tea
    sus ok lit
    (_, ok) = m[key]
    damn ok
}

slay rizz_evaluate_condition(condition tea, variables map[tea]tea) lit {
    condition = rizz_trim_whitespace(condition)
    
    # Simple variable existence check
    if rizz_has_key(variables, condition) {
        sus value tea = variables[condition]
        damn value != "" && value != "0" && value != "false"
    }
    
    # Simple equality check: var == "value"
    if rizz_contains(condition, " == ") {
        sus parts []tea = rizz_split(condition, " == ")
        if len(parts) == 2 {
            sus left tea = rizz_trim_whitespace(parts[0])
            sus right tea = rizz_trim_whitespace(parts[1])
            right = rizz_remove_quotes(right)
            
            if rizz_has_key(variables, left) {
                damn variables[left] == right
            }
        }
    }
    
    damn cap
}

slay rizz_expand_loop(loop_def tea, content tea, variables map[tea]tea) tea {
    # Parse "item in items" format
    sus parts []tea = rizz_split(loop_def, " in ")
    if len(parts) != 2 {
        damn ""
    }
    
    sus item_var tea = rizz_trim_whitespace(parts[0])
    sus items_var tea = rizz_trim_whitespace(parts[1])
    
    if !rizz_has_key(variables, items_var) {
        damn ""
    }
    
    sus items_value tea = variables[items_var]
    sus items []tea = rizz_split(items_value, ",")
    sus result tea = ""
    
    sus i normie = 0
    bestie i < len(items) {
        sus item tea = rizz_trim_whitespace(items[i])
        
        # Create temporary context with item variable
        sus temp_vars map[tea]tea = make(map[tea]tea)
        
        # Copy existing variables
        for key, value := range variables {
            temp_vars[key] = value
        }
        
        # Add current item
        temp_vars[item_var] = item
        
        # Process content with item variable
        sus expanded_content tea = rizz_interpolate_variables(content, temp_vars)
        result = result + expanded_content
        
        i++
    }
    
    damn result
}

slay rizz_contains(s tea, substr tea) lit {
    sus i normie = 0
    bestie i <= len(s) - len(substr) {
        if s[i:i+len(substr)] == substr {
            damn based
        }
        i++
    }
    damn cap
}

slay rizz_split(s tea, sep tea) []tea {
    sus result []tea
    sus start normie = 0
    sus i normie = 0
    
    bestie i <= len(s) - len(sep) {
        if s[i:i+len(sep)] == sep {
            result = append(result, s[start:i])
            start = i + len(sep)
            i = start
        } else {
            i++
        }
    }
    
    if start < len(s) {
        result = append(result, s[start:])
    }
    
    damn result
}

slay rizz_remove_quotes(s tea) tea {
    if len(s) >= 2 && s[0] == '"' && s[len(s)-1] == '"' {
        damn s[1:len(s)-1]
    }
    damn s
}

# Advanced features
slay rizz_template_add_filter(template *RizzTemplate, name tea, filter slay) {
    template.context.filters[name] = filter
}

slay rizz_template_render_with_layout(template *RizzTemplate, layout_content tea) tea {
    sus rendered tea = rizz_template_render(template)
    
    sus layout_template RizzTemplate = rizz_template_new(layout_content)
    rizz_template_set_var(&layout_template, "content", rendered)
    
    # Copy variables from main template
    for key, value := range template.context.variables {
        rizz_template_set_var(&layout_template, key, value)
    }
    
    damn rizz_template_render(&layout_template)
}

slay rizz_template_include(template *RizzTemplate, include_content tea) tea {
    sus include_template RizzTemplate = rizz_template_new(include_content)
    
    # Copy variables from main template
    for key, value := range template.context.variables {
        rizz_template_set_var(&include_template, key, value)
    }
    
    damn rizz_template_render(&include_template)
}

# Template compilation for better performance
slay rizz_template_compile(template *RizzTemplate) lit {
    # Pre-process template for faster rendering
    # This would analyze the template and create optimized render instructions
    damn based
}

# Error handling
slay rizz_template_validate(content tea) (lit, tea) {
    sus errors []tea
    
    # Check for unmatched braces
    sus brace_count normie = 0
    sus i normie = 0
    bestie i < len(content) - 1 {
        if content[i] == '{' && content[i+1] == '{' {
            brace_count++
            i += 2
        } else if content[i] == '}' && content[i+1] == '}' {
            brace_count--
            i += 2
        } else {
            i++
        }
    }
    
    if brace_count != 0 {
        errors = append(errors, "Unmatched template braces")
    }
    
    # Check for unmatched if/endif
    sus if_count normie = 0
    i = 0
    bestie i < len(content) - 5 {
        if rizz_starts_with_at(content, i, "{{if ") {
            if_count++
            i += 5
        } else if rizz_starts_with_at(content, i, "{{endif}}") {
            if_count--
            i += 9
        } else {
            i++
        }
    }
    
    if if_count != 0 {
        errors = append(errors, "Unmatched if/endif blocks")
    }
    
    # Check for unmatched for/endfor
    sus for_count normie = 0
    i = 0
    bestie i < len(content) - 6 {
        if rizz_starts_with_at(content, i, "{{for ") {
            for_count++
            i += 6
        } else if rizz_starts_with_at(content, i, "{{endfor}}") {
            for_count--
            i += 10
        } else {
            i++
        }
    }
    
    if for_count != 0 {
        errors = append(errors, "Unmatched for/endfor blocks")
    }
    
    if len(errors) > 0 {
        sus error_msg tea = ""
        sus j normie = 0
        bestie j < len(errors) {
            error_msg = error_msg + errors[j]
            if j < len(errors) - 1 {
                error_msg = error_msg + "; "
            }
            j++
        }
        damn cap, error_msg
    }
    
    damn based, ""
}
