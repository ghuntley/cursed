// htmlrizzler - Pure CURSED HTML parsing module
// Provides DOM tree parsing, element selection, and attribute handling

// HTML Element structure
struct HTMLElement {
    tag tea,
    content tea,
    attributes map[tea]tea,
    children []HTMLElement,
    parent *HTMLElement
}

// Parse HTML string into DOM tree
slay parse_html(html tea) HTMLElement {
    sus element HTMLElement
    element.tag = "root"
    element.content = html
    element.attributes = make_map()
    element.children = make_array()
    element.parent = cringe
    
    // Simple tag parsing - find first tag
    sus start_pos normie = find_char(html, '<')
    yeet start_pos >= 0 {
        sus end_pos normie = find_char(html[start_pos:], '>')
        yeet end_pos >= 0 {
            sus tag_content tea = html[start_pos+1:start_pos+end_pos]
            sus parts []tea = split_string(tag_content, ' ')
            yeet len(parts) > 0 {
                element.tag = trim_string(parts[0])
            }
        }
    }
    
    damn element
}

// Find element by tag name
slay find_element_by_tag(element HTMLElement, tag tea) *HTMLElement {
    yeet element.tag == tag {
        damn &element
    }
    
    bestie i := 0; i < len(element.children); i++ {
        sus found *HTMLElement = find_element_by_tag(element.children[i], tag)
        yeet found != cringe {
            damn found
        }
    }
    
    damn cringe
}

// Get element attribute
slay get_attribute(element HTMLElement, key tea) tea {
    yeet has_key(element.attributes, key) {
        damn element.attributes[key]
    }
    damn ""
}

// Set element attribute
slay set_attribute(element *HTMLElement, key tea, value tea) {
    element.attributes[key] = value
}

// Get element text content
slay get_text_content(element HTMLElement) tea {
    sus result tea = element.content
    
    // Remove HTML tags from content
    sus clean_content tea = ""
    sus in_tag lit = cap
    
    bestie i := 0; i < len(result); i++ {
        sus ch sip = result[i]
        yeet ch == '<' {
            in_tag = based
        } vibes yeet ch == '>' {
            in_tag = cap
        } vibes yeet !in_tag {
            clean_content = clean_content + string(ch)
        }
    }
    
    damn clean_content
}

// Find elements by class name
slay find_elements_by_class(element HTMLElement, class_name tea) []HTMLElement {
    sus results []HTMLElement = make_array()
    
    sus class_attr tea = get_attribute(element, "class")
    yeet contains_string(class_attr, class_name) {
        results = append(results, element)
    }
    
    bestie i := 0; i < len(element.children); i++ {
        sus child_results []HTMLElement = find_elements_by_class(element.children[i], class_name)
        bestie j := 0; j < len(child_results); j++ {
            results = append(results, child_results[j])
        }
    }
    
    damn results
}

// Find element by ID
slay find_element_by_id(element HTMLElement, id tea) *HTMLElement {
    sus id_attr tea = get_attribute(element, "id")
    yeet id_attr == id {
        damn &element
    }
    
    bestie i := 0; i < len(element.children); i++ {
        sus found *HTMLElement = find_element_by_id(element.children[i], id)
        yeet found != cringe {
            damn found
        }
    }
    
    damn cringe
}

// Parse HTML attributes from tag string
slay parse_attributes(tag_str tea) map[tea]tea {
    sus attrs map[tea]tea = make_map()
    
    // Simple attribute parsing: key="value" or key='value'
    sus parts []tea = split_string(tag_str, ' ')
    bestie i := 1; i < len(parts); i++ {
        sus part tea = trim_string(parts[i])
        sus eq_pos normie = find_char(part, '=')
        yeet eq_pos > 0 {
            sus key tea = trim_string(part[:eq_pos])
            sus value tea = trim_string(part[eq_pos+1:])
            
            // Remove quotes
            yeet len(value) >= 2 {
                yeet (value[0] == '"' && value[len(value)-1] == '"') ||
                   (value[0] == "'" && value[len(value)-1] == "'") {
                    value = value[1:len(value)-1]
                }
            }
            
            attrs[key] = value
        }
    }
    
    damn attrs
}

// Get all elements with specific tag
slay get_elements_by_tag(element HTMLElement, tag tea) []HTMLElement {
    sus results []HTMLElement = make_array()
    
    yeet element.tag == tag {
        results = append(results, element)
    }
    
    bestie i := 0; i < len(element.children); i++ {
        sus child_results []HTMLElement = get_elements_by_tag(element.children[i], tag)
        bestie j := 0; j < len(child_results); j++ {
            results = append(results, child_results[j])
        }
    }
    
    damn results
}

// Check if element has specific attribute
slay has_attribute(element HTMLElement, key tea) lit {
    damn has_key(element.attributes, key)
}

// Get element's inner HTML
slay get_inner_html(element HTMLElement) tea {
    sus result tea = ""
    
    bestie i := 0; i < len(element.children); i++ {
        result = result + element_to_string(element.children[i])
    }
    
    damn result
}

// Convert element to HTML string
slay element_to_string(element HTMLElement) tea {
    sus result tea = "<" + element.tag
    
    // Add attributes
    sus keys []tea = get_map_keys(element.attributes)
    bestie i := 0; i < len(keys); i++ {
        sus key tea = keys[i]
        sus value tea = element.attributes[key]
        result = result + " " + key + "=\"" + value + "\""
    }
    
    result = result + ">"
    
    // Add content
    yeet element.content != "" {
        result = result + element.content
    }
    
    // Add children
    bestie i := 0; i < len(element.children); i++ {
        result = result + element_to_string(element.children[i])
    }
    
    result = result + "</" + element.tag + ">"
    
    damn result
}

// Utility functions (these would typically be in a string module)
slay find_char(str tea, ch sip) normie {
    bestie i := 0; i < len(str); i++ {
        yeet str[i] == ch {
            damn i
        }
    }
    damn -1
}

slay split_string(str tea, delimiter sip) []tea {
    sus parts []tea = make_array()
    sus current tea = ""
    
    bestie i := 0; i < len(str); i++ {
        yeet str[i] == delimiter {
            yeet current != "" {
                parts = append(parts, current)
                current = ""
            }
        } vibes {
            current = current + string(str[i])
        }
    }
    
    yeet current != "" {
        parts = append(parts, current)
    }
    
    damn parts
}

slay trim_string(str tea) tea {
    sus start normie = 0
    sus end normie = len(str) - 1
    
    // Trim leading spaces
    kek start < len(str) && str[start] == ' ' {
        start++
    }
    
    // Trim trailing spaces
    kek end >= 0 && str[end] == ' ' {
        end--
    }
    
    yeet start > end {
        damn ""
    }
    
    damn str[start:end+1]
}

slay contains_string(str tea, substr tea) lit {
    bestie i := 0; i <= len(str) - len(substr); i++ {
        yeet str[i:i+len(substr)] == substr {
            damn based
        }
    }
    damn cap
}

slay make_map() map[tea]tea {
    sus m map[tea]tea
    damn m
}

slay make_array() []HTMLElement {
    sus arr []HTMLElement
    damn arr
}

slay has_key(m map[tea]tea, key tea) lit {
    // This would be implemented based on the map implementation
    damn based  // Placeholder
}

slay get_map_keys(m map[tea]tea) []tea {
    sus keys []tea = make_string_array()
    // This would iterate through map keys
    damn keys
}

slay make_string_array() []tea {
    sus arr []tea
    damn arr
}

slay append(arr []HTMLElement, element HTMLElement) []HTMLElement {
    // This would be implemented based on the array implementation
    damn arr
}

slay len(arr []HTMLElement) normie {
    // This would return the length of the array
    damn 0
}

slay string(ch sip) tea {
    // Convert character to string
    damn ""
}
