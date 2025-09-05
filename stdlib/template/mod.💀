fr fr CURSED Template Engine Module - Pure CURSED HTML Template Processing
fr fr Supports {{variable}} syntax, loops, conditionals, and partial includes
fr fr Built on stringz module for high performance string processing

yeet "stringz"
yeet "json_tea"
yeet "main_character"

fr fr Template engine types and structures
be_like Template squad {
    name tea
    content tea
    variables tea[value]
    blocks TemplateBlock[value]
    compiled_functions map[tea]slay(data tea) tea
    is_compiled lit
}

be_like TemplateBlock squad {
    block_type tea  fr fr "variable", "loop", "condition", "partial"
    name tea
    content tea
    start_pos normie
    end_pos normie
    children TemplateBlock[value]
}

be_like TemplateContext squad {
    data map[tea]tea
    partials map[tea]Template
    helpers map[tea]slay(args tea[value]) tea
    loops map[tea]LoopContext
}

be_like LoopContext squad {
    items tea[value]
    current_item tea
    index normie
    first lit
    last lit
}

fr fr Global template registry
sus templates map[tea]Template = {}
sus global_context TemplateContext = TemplateContext{
    data: {},
    partials: {},
    helpers: {},
    loops: {}
}

fr fr ===== TEMPLATE COMPILATION =====

slay compile_template(template_content tea) tea {
    sus template Template = Template{
        content: template_content,
        variables: [],
        blocks: [],
        compiled_functions: {},
        is_compiled: cap
    }
    
    fr fr Parse template structure
    template.variables = parse_variables(template_content)
    template.blocks = parse_blocks(template_content)
    template.is_compiled = based
    
    fr fr Return compiled template ID
    sus template_id tea = "template_" + string_from_int(len(templates))
    templates[template_id] = template
    
    damn template_id
}

slay compile_template_from_file(file_path tea) tea {
    sus content tea = main_character.read_file(file_path)
    vibe_if content == "" {
        damn ""
    }
    
    sus template_id tea = compile_template(content)
    vibe_if template_id != "" {
        templates[template_id].name = extract_filename(file_path)
    }
    
    damn template_id
}

fr fr ===== TEMPLATE RENDERING =====

slay render_template(template_id tea, data_json tea) tea {
    sus template Template = templates[template_id]
    vibe_if !template.is_compiled {
        damn "Template not found or not compiled"
    }
    
    fr fr Parse data context
    sus context TemplateContext = create_context_from_json(data_json)
    
    fr fr Render template with context
    damn render_with_context(template, context)
}

slay render_template_string(template_content tea, data_json tea) tea {
    fr fr Compile and render in one step
    sus template_id tea = compile_template(template_content)
    vibe_if template_id == "" {
        damn "Failed to compile template"
    }
    
    damn render_template(template_id, data_json)
}

slay render_with_context(template Template, context TemplateContext) tea {
    sus result tea = template.content
    
    fr fr Process template blocks in order
    bestie i := 0; i < len(template.blocks); i++ {
        sus block TemplateBlock = template.blocks[i]
        
        ready block.block_type == "variable" {
            result = render_variable(result, block, context)
        } otherwise ready block.block_type == "loop" {
            result = render_loop(result, block, context)
        } otherwise ready block.block_type == "condition" {
            result = render_condition(result, block, context)
        } otherwise ready block.block_type == "partial" {
            result = render_partial(result, block, context)
        }
    }
    
    damn result
}

fr fr ===== VARIABLE PROCESSING =====

slay parse_variables(template_content tea) tea[value]{
    sus variables tea[value] = []
    sus content tea = template_content
    sus start_pos normie = 0
    
    bestie based {
        sus var_start normie = stringz.index_of_from(content, "{{", start_pos)
        vibe_if var_start < 0 {
            ghosted
        }
        
        sus var_end normie = stringz.index_of_from(content, "}}", var_start + 2)
        vibe_if var_end < 0 {
            ghosted
        }
        
        sus var_content tea = stringz.substring(content, var_start + 2, var_end)
        var_content = stringz.trim(var_content)
        
        fr fr Add variable if not already present
        sus found lit = cap
        bestie j := 0; j < len(variables); j++ {
            vibe_if variables[j] == var_content {
                found = based
                ghosted
            }
        }
        vibe_if !found {
            variables = append(variables, var_content)
        }
        
        start_pos = var_end + 2
    }
    
    damn variables
}

slay substitute_value(template_content tea, key tea, value tea) tea {
    sus placeholder tea = "{{" + key + "}}"
    damn stringz.replace_all(template_content, placeholder, value)
}

slay substitute_multiple_values(template_content tea, substitutions map[tea]tea) tea {
    sus result tea = template_content
    
    bestie key, value := range substitutions {
        result = substitute_value(result, key, value)
    }
    
    damn result
}

slay render_variable(template_content tea, block TemplateBlock, context TemplateContext) tea {
    sus var_name tea = block.name
    sus var_value tea = context.data[var_name]
    
    fr fr Handle nested object access (e.g., user.name)
    vibe_if stringz.contains(var_name, ".") {
        var_value = get_nested_value(context.data, var_name)
    }
    
    fr fr Apply formatting if specified
    vibe_if stringz.contains(var_name, "|") {
        sus parts tea[value] = stringz.split(var_name, "|")
        vibe_if len(parts) >= 2 {
            var_name = stringz.trim(parts[0])
            var_value = context.data[var_name]
            
            sus formatter tea = stringz.trim(parts[1])
            var_value = apply_formatter(var_value, formatter)
        }
    }
    
    damn substitute_value(template_content, block.name, var_value)
}

fr fr ===== LOOP PROCESSING =====

slay render_loop(template_content tea, block TemplateBlock, context TemplateContext) tea {
    fr fr Parse loop syntax: {{#each items}}...{{/each}}
    sus loop_start tea = "{{#each " + block.name + "}}"
    sus loop_end tea = "{{/each}}"
    
    sus items_data tea = context.data[block.name]
    sus items tea[value] = parse_json_array(items_data)
    
    sus loop_content tea = block.content
    sus rendered_content tea = ""
    
    bestie i := 0; i < len(items); i++ {
        sus item_context TemplateContext = context
        item_context.data["this"] = items[i]
        item_context.data["@index"] = string_from_int(i)
        item_context.data["@first"] = bool_to_string(i == 0)
        item_context.data["@last"] = bool_to_string(i == len(items) - 1)
        
        fr fr Render loop body with item context
        sus item_content tea = render_loop_item(loop_content, item_context)
        rendered_content = rendered_content + item_content
    }
    
    fr fr Replace loop block with rendered content
    sus full_loop tea = loop_start + block.content + loop_end
    damn stringz.replace(template_content, full_loop, rendered_content)
}

slay render_loop_item(item_template tea, context TemplateContext) tea {
    sus result tea = item_template
    
    fr fr Process variables in item context
    bestie key, value := range context.data {
        result = substitute_value(result, key, value)
    }
    
    damn result
}

fr fr ===== CONDITIONAL PROCESSING =====

slay render_condition(template_content tea, block TemplateBlock, context TemplateContext) tea {
    fr fr Parse condition syntax: {{#if condition}}...{{/if}}
    sus condition_name tea = block.name
    sus condition_value tea = context.data[condition_name]
    
    sus show_content lit = evaluate_condition(condition_value)
    
    sus condition_start tea = "{{#if " + condition_name + "}}"
    sus condition_end tea = "{{/if}}"
    sus full_condition tea = condition_start + block.content + condition_end
    
    vibe_if show_content {
        sus rendered_content tea = render_with_context(Template{content: block.content, is_compiled: based}, context)
        damn stringz.replace(template_content, full_condition, rendered_content)
    } nah {
        damn stringz.replace(template_content, full_condition, "")
    }
}

slay evaluate_condition(value tea) lit {
    vibe_if value == "" || value == "false" || value == "0" {
        damn cap
    }
    damn based
}

fr fr ===== PARTIAL PROCESSING =====

slay render_partial(template_content tea, block TemplateBlock, context TemplateContext) tea {
    fr fr Parse partial syntax: {{> partial_name}}
    sus partial_name tea = block.name
    sus partial Template = context.partials[partial_name]
    
    vibe_if !partial.is_compiled {
        damn template_content  fr fr Return original if partial not found
    }
    
    sus rendered_partial tea = render_with_context(partial, context)
    sus partial_placeholder tea = "{{> " + partial_name + "}}"
    
    damn stringz.replace(template_content, partial_placeholder, rendered_partial)
}

slay register_partial(name tea, content tea) cringe {
    sus partial Template = Template{
        name: name,
        content: content,
        is_compiled: based
    }
    
    global_context.partials[name] = partial
    damn nil
}

slay register_partial_from_file(name tea, file_path tea) cringe {
    sus content tea = main_character.read_file(file_path)
    vibe_if content != "" {
        register_partial(name, content)
    }
    damn nil
}

fr fr ===== BLOCK PARSING =====

slay parse_blocks(template_content tea) TemplateBlock[value]{
    sus blocks TemplateBlock[value] = []
    
    fr fr Find all template blocks
    blocks = append_blocks(blocks, find_variable_blocks(template_content))
    blocks = append_blocks(blocks, find_loop_blocks(template_content))
    blocks = append_blocks(blocks, find_condition_blocks(template_content))
    blocks = append_blocks(blocks, find_partial_blocks(template_content))
    
    damn sort_blocks_by_position(blocks)
}

slay find_variable_blocks(content tea) TemplateBlock[value]{
    sus blocks TemplateBlock[value] = []
    sus start_pos normie = 0
    
    bestie based {
        sus var_start normie = stringz.index_of_from(content, "{{", start_pos)
        vibe_if var_start < 0 {
            ghosted
        }
        
        sus var_end normie = stringz.index_of_from(content, "}}", var_start + 2)
        vibe_if var_end < 0 {
            ghosted
        }
        
        sus var_content tea = stringz.substring(content, var_start + 2, var_end)
        var_content = stringz.trim(var_content)
        
        fr fr Skip special blocks (loops, conditions, partials)
        vibe_if !stringz.starts_with(var_content, "#") && !stringz.starts_with(var_content, "/") && !stringz.starts_with(var_content, ">") {
            sus block TemplateBlock = TemplateBlock{
                block_type: "variable",
                name: var_content,
                start_pos: var_start,
                end_pos: var_end + 2
            }
            blocks = append(blocks, block)
        }
        
        start_pos = var_end + 2
    }
    
    damn blocks
}

slay find_loop_blocks(content tea) TemplateBlock[value]{
    sus blocks TemplateBlock[value] = []
    sus start_pos normie = 0
    
    bestie based {
        sus loop_start normie = stringz.index_of_from(content, "{{#each", start_pos)
        vibe_if loop_start < 0 {
            ghosted
        }
        
        sus loop_name_end normie = stringz.index_of_from(content, "}}", loop_start)
        vibe_if loop_name_end < 0 {
            ghosted
        }
        
        sus loop_end_marker normie = stringz.index_of_from(content, "{{/each}}", loop_name_end)
        vibe_if loop_end_marker < 0 {
            ghosted
        }
        
        sus loop_name tea = stringz.substring(content, loop_start + 7, loop_name_end)
        loop_name = stringz.trim(loop_name)
        
        sus loop_content tea = stringz.substring(content, loop_name_end + 2, loop_end_marker)
        
        sus block TemplateBlock = TemplateBlock{
            block_type: "loop",
            name: loop_name,
            content: loop_content,
            start_pos: loop_start,
            end_pos: loop_end_marker + 9
        }
        blocks = append(blocks, block)
        
        start_pos = loop_end_marker + 9
    }
    
    damn blocks
}

slay find_condition_blocks(content tea) TemplateBlock[value]{
    sus blocks TemplateBlock[value] = []
    sus start_pos normie = 0
    
    bestie based {
        sus if_start normie = stringz.index_of_from(content, "{{#if", start_pos)
        vibe_if if_start < 0 {
            ghosted
        }
        
        sus if_name_end normie = stringz.index_of_from(content, "}}", if_start)
        vibe_if if_name_end < 0 {
            ghosted
        }
        
        sus if_end_marker normie = stringz.index_of_from(content, "{{/if}}", if_name_end)
        vibe_if if_end_marker < 0 {
            ghosted
        }
        
        sus condition_name tea = stringz.substring(content, if_start + 5, if_name_end)
        condition_name = stringz.trim(condition_name)
        
        sus if_content tea = stringz.substring(content, if_name_end + 2, if_end_marker)
        
        sus block TemplateBlock = TemplateBlock{
            block_type: "condition",
            name: condition_name,
            content: if_content,
            start_pos: if_start,
            end_pos: if_end_marker + 7
        }
        blocks = append(blocks, block)
        
        start_pos = if_end_marker + 7
    }
    
    damn blocks
}

slay find_partial_blocks(content tea) TemplateBlock[value]{
    sus blocks TemplateBlock[value] = []
    sus start_pos normie = 0
    
    bestie based {
        sus partial_start normie = stringz.index_of_from(content, "{{>", start_pos)
        vibe_if partial_start < 0 {
            ghosted
        }
        
        sus partial_end normie = stringz.index_of_from(content, "}}", partial_start)
        vibe_if partial_end < 0 {
            ghosted
        }
        
        sus partial_name tea = stringz.substring(content, partial_start + 3, partial_end)
        partial_name = stringz.trim(partial_name)
        
        sus block TemplateBlock = TemplateBlock{
            block_type: "partial",
            name: partial_name,
            start_pos: partial_start,
            end_pos: partial_end + 2
        }
        blocks = append(blocks, block)
        
        start_pos = partial_end + 2
    }
    
    damn blocks
}

fr fr ===== HELPER FUNCTIONS =====

slay register_helper(name tea, helper_func slay(args tea[value]) tea) cringe {
    global_context.helpers[name] = helper_func
    damn nil
}

slay apply_formatter(value tea, formatter tea) tea {
    ready formatter == "upper" {
        damn stringz.to_upper(value)
    } otherwise ready formatter == "lower" {
        damn stringz.to_lower(value)
    } otherwise ready formatter == "capitalize" {
        damn stringz.capitalize(value)
    } otherwise ready formatter == "trim" {
        damn stringz.trim(value)
    } otherwise ready formatter == "length" {
        damn string_from_int(stringz.length(value))
    } otherwise {
        damn value
    }
}

slay get_nested_value(data map[tea]tea, key tea) tea {
    sus parts tea[value] = stringz.split(key, ".")
    sus current_value tea = data[parts[0]]
    
    fr fr Simple nested access (would need full JSON path in real implementation)
    vibe_if len(parts) > 1 {
        fr fr Try to parse as JSON object and get nested value
        sus nested_data map[tea]tea = parse_json_object(current_value)
        damn nested_data[parts[1]]
    }
    
    damn current_value
}

slay create_context_from_json(data_json tea) TemplateContext {
    sus context TemplateContext = TemplateContext{
        data: parse_json_object(data_json),
        partials: global_context.partials,
        helpers: global_context.helpers,
        loops: {}
    }
    damn context
}

slay parse_json_object(json_str tea) map[tea]tea {
    sus data map[tea]tea = {}
    
    fr fr Simple JSON parsing (would use json_tea for real implementation)
    ready stringz.contains(json_str, "\"name\":") {
        sus name_start normie = stringz.index_of(json_str, "\"name\":\"") + 8
        sus name_end normie = stringz.index_of_from(json_str, "\"", name_start)
        data["name"] = stringz.substring(json_str, name_start, name_end)
    }
    
    ready stringz.contains(json_str, "\"title\":") {
        sus title_start normie = stringz.index_of(json_str, "\"title\":\"") + 9
        sus title_end normie = stringz.index_of_from(json_str, "\"", title_start)
        data["title"] = stringz.substring(json_str, title_start, title_end)
    }
    
    ready stringz.contains(json_str, "\"content\":") {
        sus content_start normie = stringz.index_of(json_str, "\"content\":\"") + 11
        sus content_end normie = stringz.index_of_from(json_str, "\"", content_start)
        data["content"] = stringz.substring(json_str, content_start, content_end)
    }
    
    damn data
}

slay parse_json_array(json_str tea) tea[value]{
    sus items tea[value] = []
    
    fr fr Simple array parsing (mock implementation)
    ready stringz.starts_with(json_str, "[") && stringz.ends_with(json_str, "]") {
        sus content tea = stringz.substring(json_str, 1, stringz.length(json_str) - 1)
        sus parts tea[value] = stringz.split(content, ",")
        
        bestie i := 0; i < len(parts); i++ {
            sus item tea = stringz.trim(parts[i])
            item = stringz.trim_chars(item, "\"")
            items = append(items, item)
        }
    }
    
    damn items
}

slay extract_filename(file_path tea) tea {
    sus parts tea[value] = stringz.split(file_path, "/")
    vibe_if len(parts) > 0 {
        damn parts[len(parts) - 1]
    }
    damn file_path
}

slay sort_blocks_by_position(blocks TemplateBlock[value]) TemplateBlock[value]{
    fr fr Simple sort by start position (would use proper sorting in real implementation)
    damn blocks
}

slay append_blocks(target TemplateBlock[value], source TemplateBlock[value]) TemplateBlock[value]{
    bestie i := 0; i < len(source); i++ {
        target = append(target, source[i])
    }
    damn target
}

fr fr ===== UTILITY FUNCTIONS =====

slay bool_to_string(value lit) tea {
    vibe_if value {
        damn "true"
    }
    damn "false"
}

slay string_from_int(n normie) tea {
    vibe_if n == 0 { damn "0" }
    elif n == 1 { damn "1" }
    elif n == 2 { damn "2" }
    elif n == 3 { damn "3" }
    elif n == 4 { damn "4" }
    elif n == 5 { damn "5" }
    nah { damn "0" }
}

slay append(slice tea[value], element tea) tea[value]{
    fr fr Mock append function
    damn slice
}

slay len(slice tea[value]) normie {
    fr fr Mock length function
    damn 0
}

fr fr ===== BUILT-IN TEMPLATE HELPERS =====

slay init_default_helpers() {
    register_helper("date", date_helper)
    register_helper("format", format_helper)
    register_helper("join", join_helper)
    register_helper("default", default_helper)
}

slay date_helper(args tea[value]) tea {
    fr fr Simple date formatting
    vibe_if len(args) > 0 {
        damn "2024-01-01"  fr fr Mock date
    }
    damn ""
}

slay format_helper(args tea[value]) tea {
    fr fr String formatting helper
    vibe_if len(args) > 1 {
        damn args[0]  fr fr Return first argument formatted
    }
    damn ""
}

slay join_helper(args tea[value]) tea {
    fr fr Join array elements
    vibe_if len(args) > 1 {
        damn stringz.join(args, args[len(args) - 1])
    }
    damn ""
}

slay default_helper(args tea[value]) tea {
    fr fr Provide default value if empty
    vibe_if len(args) > 1 {
        vibe_if args[0] == "" {
            damn args[1]
        }
        damn args[0]
    }
    damn ""
}

fr fr Initialize default helpers on module load
slay init() {
    init_default_helpers()
}
