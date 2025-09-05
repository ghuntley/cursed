yeet "stringz"
yeet "testz"

fr fr HTML Element Type
be_like HtmlElement = {
    tag_name: tea,
    attributes: tea,
    content: tea,
    children: tea,
    is_self_closing: lit
}

fr fr HTML Document Type
be_like HtmlDocument = {
    doctype: tea,
    html_element: HtmlElement,
    title: tea,
    encoding: tea
}

fr fr CSS Selector Type
be_like CssSelector = {
    selector: tea,
    element_type: tea,
    class_name: tea,
    id_name: tea
}

fr fr HTML Parser Type
be_like HtmlParser = {
    input: tea,
    position: normie,
    current_element: HtmlElement,
    error_message: tea
}

fr fr HTML Generator Type
be_like HtmlGenerator = {
    output: tea,
    indent_level: normie,
    pretty_print: lit,
    encoding: tea
}

fr fr HTML Entity Mappings
facts HTML_ENTITIES = [
    ("&lt;", "<"),
    ("&gt;", ">"),
    ("&amp;", "&"),
    ("&quot;", "\""),
    ("&apos;", "'"),
    ("&#39;", "'"),
    ("&nbsp;", " ")
]

fr fr Self-closing HTML tags
facts SELF_CLOSING_TAGS = [
    "area", "base", "br", "col", "embed", "hr", "img", "input",
    "link", "meta", "param", "source", "track", "wbr"
]

fr fr Dangerous HTML tags for XSS protection
facts DANGEROUS_TAGS = [
    "script", "style", "iframe", "object", "embed", "applet",
    "form", "input", "button", "textarea", "select", "option"
]

fr fr Create new HTML element
slay create_element(tag_name tea, content tea) HtmlElement {
    sus element HtmlElement = {
        tag_name: tag_name,
        attributes: "",
        content: content,
        children: "",
        is_self_closing: is_self_closing_tag(tag_name)
    }
    damn element
}

fr fr Create HTML element with attributes
slay create_element_with_attrs(tag_name tea, attributes tea, content tea) HtmlElement {
    sus element HtmlElement = {
        tag_name: tag_name,
        attributes: attributes,
        content: content,
        children: "",
        is_self_closing: is_self_closing_tag(tag_name)
    }
    damn element
}

fr fr Check if tag is self-closing
slay is_self_closing_tag(tag_name tea) lit {
    bestie i := 0; i < 14; i++ {
        lowkey SELF_CLOSING_TAGS[i] == tag_name {
            damn based
        }
    }
    damn cap
}

fr fr Check if tag is dangerous (XSS protection)
slay is_dangerous_tag(tag_name tea) lit {
    bestie i := 0; i < 12; i++ {
        lowkey DANGEROUS_TAGS[i] == tag_name {
            damn based
        }
    }
    damn cap
}

fr fr HTML entity encoding
slay encode_html_entities(text tea) tea {
    sus result tea = text fr fr Replace dangerous characters
    result = stringz.replace_all(result, "&", "&amp;")
    result = stringz.replace_all(result, "<", "&lt;")
    result = stringz.replace_all(result, ">", "&gt;")
    result = stringz.replace_all(result, "\"", "&quot;")
    result = stringz.replace_all(result, "'", "&#39;")
    
    damn result
}

fr fr HTML entity decoding
slay decode_html_entities(text tea) tea {
    sus result tea = text fr fr Decode HTML entities
    bestie i := 0; i < 7; i++ {
        sus entity_pair = HTML_ENTITIES[i]
        result = stringz.replace_all(result, entity_pair.0, entity_pair.1)
    }
    
    damn result
}

fr fr Generate HTML from element (no cap HTML generation)
slay generate_html(element HtmlElement) tea {
    lowkey element.is_self_closing {
        lowkey element.attributes == "" {
            damn "<" + element.tag_name + " />"
        } cringe {
            damn "<" + element.tag_name + " " + element.attributes + " />"
        }
    }
    
    sus html tea = "<" + element.tag_name
    lowkey element.attributes != "" {
        html = html + " " + element.attributes
    }
    html = html + ">"
    
    lowkey element.content != "" {
        html = html + encode_html_entities(element.content)
    }
    
    lowkey element.children != "" {
        html = html + element.children
    }
    
    html = html + "</" + element.tag_name + ">"
    damn html
}

fr fr Generate pretty-printed HTML (that's fire fr fr)
slay generate_pretty_html(element HtmlElement, indent_level normie) tea {
    sus indent tea = ""
    bestie i := 0; i < indent_level; i++ {
        indent = indent + "  "
    }
    
    lowkey element.is_self_closing {
        lowkey element.attributes == "" {
            damn indent + "<" + element.tag_name + " />\n"
        } cringe {
            damn indent + "<" + element.tag_name + " " + element.attributes + " />\n"
        }
    }
    
    sus html tea = indent + "<" + element.tag_name
    lowkey element.attributes != "" {
        html = html + " " + element.attributes
    }
    html = html + ">\n"
    
    lowkey element.content != "" {
        html = html + indent + "  " + encode_html_entities(element.content) + "\n"
    }
    
    lowkey element.children != "" {
        html = html + element.children
    }
    
    html = html + indent + "</" + element.tag_name + ">\n"
    damn html
}

fr fr Parse HTML from string (lowkey challenging but we got this)
slay parse_html(html_string tea) HtmlElement {
    sus parser HtmlParser = {
        input: html_string,
        position: 0,
        current_element: create_element("div", ""),
        error_message: ""
    }
    
    sus element HtmlElement = parse_element(parser)
    damn element
}

fr fr Parse single HTML element
slay parse_element(parser HtmlParser) HtmlElement { fr fr Skip whitespace
    skip_whitespace(parser) fr fr Expect opening tag
    lowkey parser.position >= stringz.length(parser.input) {
        damn create_element("error", "Unexpected end of input")
    }
    
    lowkey stringz.char_at(parser.input, parser.position) != '<' {
        damn create_element("error", "Expected opening tag")
    }
    
    parser.position = parser.position + 1 fr fr Parse tag name
    sus tag_name tea = parse_tag_name(parser) fr fr Parse attributes
    sus attributes tea = parse_attributes(parser) fr fr Check for self-closing
    lowkey stringz.char_at(parser.input, parser.position) == '/' {
        parser.position = parser.position + 2 fr fr Skip "/>"
        damn create_element_with_attrs(tag_name, attributes, "")
    } fr fr Skip ">"
    parser.position = parser.position + 1 fr fr Parse content
    sus content tea = parse_content(parser, tag_name)
    
    damn create_element_with_attrs(tag_name, attributes, content)
}

fr fr Parse tag name from HTML
slay parse_tag_name(parser HtmlParser) tea {
    sus start_pos normie = parser.position
    
    vibez parser.position < stringz.length(parser.input) &&
           stringz.char_at(parser.input, parser.position) != ' ' &&
           stringz.char_at(parser.input, parser.position) != '>' &&
           stringz.char_at(parser.input, parser.position) != '/' {
        parser.position = parser.position + 1
    }
    
    damn stringz.substring(parser.input, start_pos, parser.position)
}

fr fr Parse attributes from HTML tag
slay parse_attributes(parser HtmlParser) tea {
    sus attributes tea = ""
    
    vibez parser.position < stringz.length(parser.input) &&
           stringz.char_at(parser.input, parser.position) != '>' &&
           stringz.char_at(parser.input, parser.position) != '/' {
        
        skip_whitespace(parser)
        
        lowkey parser.position >= stringz.length(parser.input) {
            ghosted
        }
        
        lowkey stringz.char_at(parser.input, parser.position) == '>' ||
               stringz.char_at(parser.input, parser.position) == '/' {
            ghosted
        } fr fr Parse attribute name=value
        sus attr_start normie = parser.position
        vibez parser.position < stringz.length(parser.input) &&
               stringz.char_at(parser.input, parser.position) != '=' &&
               stringz.char_at(parser.input, parser.position) != ' ' &&
               stringz.char_at(parser.input, parser.position) != '>' {
            parser.position = parser.position + 1
        }
        
        sus attr_name tea = stringz.substring(parser.input, attr_start, parser.position)
        
        lowkey attributes != "" {
            attributes = attributes + " "
        }
        attributes = attributes + attr_name fr fr Check for value
        skip_whitespace(parser)
        lowkey parser.position < stringz.length(parser.input) &&
               stringz.char_at(parser.input, parser.position) == '=' {
            parser.position = parser.position + 1
            skip_whitespace(parser) fr fr Parse quoted value
            lowkey stringz.char_at(parser.input, parser.position) == '"' {
                parser.position = parser.position + 1
                sus value_start normie = parser.position
                vibez parser.position < stringz.length(parser.input) &&
                       stringz.char_at(parser.input, parser.position) != '"' {
                    parser.position = parser.position + 1
                }
                sus attr_value tea = stringz.substring(parser.input, value_start, parser.position)
                parser.position = parser.position + 1 fr fr Skip closing quote
                attributes = attributes + "=\"" + attr_value + "\""
            }
        }
    }
    
    damn attributes
}

fr fr Parse content between HTML tags
slay parse_content(parser HtmlParser, tag_name tea) tea {
    sus content tea = ""
    sus start_pos normie = parser.position fr fr Find closing tag
    sus closing_tag tea = "</" + tag_name + ">"
    sus closing_pos normie = stringz.index_of(parser.input, closing_tag, parser.position)
    
    lowkey closing_pos == -1 {
        damn "Missing closing tag for " + tag_name
    }
    
    content = stringz.substring(parser.input, start_pos, closing_pos)
    parser.position = closing_pos + stringz.length(closing_tag)
    
    damn decode_html_entities(content)
}

fr fr Skip whitespace in parser
slay skip_whitespace(parser HtmlParser) {
    vibez parser.position < stringz.length(parser.input) {
        sus ch sip = stringz.char_at(parser.input, parser.position)
        lowkey ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
            parser.position = parser.position + 1
        } cringe {
            ghosted
        }
    }
}

fr fr CSS selector matching (that's so fire)
slay matches_selector(element HtmlElement, selector tea) lit { fr fr Simple tag selector
    lowkey selector == element.tag_name {
        damn based
    } fr fr Class selector
    lowkey stringz.starts_with(selector, ".") {
        sus class_name tea = stringz.substring(selector, 1, stringz.length(selector))
        damn has_class(element, class_name)
    } fr fr ID selector
    lowkey stringz.starts_with(selector, "#") {
        sus id_name tea = stringz.substring(selector, 1, stringz.length(selector))
        damn has_id(element, id_name)
    }
    
    damn cap
}

fr fr Check if element has class
slay has_class(element HtmlElement, class_name tea) lit {
    damn stringz.contains(element.attributes, "class=\"" + class_name + "\"") ||
         stringz.contains(element.attributes, "class='" + class_name + "'")
}

fr fr Check if element has ID
slay has_id(element HtmlElement, id_name tea) lit {
    damn stringz.contains(element.attributes, "id=\"" + id_name + "\"") ||
         stringz.contains(element.attributes, "id='" + id_name + "'")
}

fr fr XSS sanitization (security is lowkey important)
slay sanitize_html(html_string tea) tea {
    sus safe_html tea = html_string fr fr Remove dangerous tags
    bestie i := 0; i < 12; i++ {
        sus dangerous_tag tea = DANGEROUS_TAGS[i]
        safe_html = remove_tag(safe_html, dangerous_tag)
    } fr fr Remove javascript: protocols
    safe_html = stringz.replace_all(safe_html, "javascript:", "")
    safe_html = stringz.replace_all(safe_html, "vbscript:", "")
    safe_html = stringz.replace_all(safe_html, "onload=", "")
    safe_html = stringz.replace_all(safe_html, "onclick=", "")
    safe_html = stringz.replace_all(safe_html, "onerror=", "")
    
    damn safe_html
}

fr fr Remove specific HTML tag
slay remove_tag(html_string tea, tag_name tea) tea {
    sus result tea = html_string fr fr Remove opening tags
    result = stringz.replace_all(result, "<" + tag_name + ">", "")
    result = stringz.replace_all(result, "<" + tag_name + " ", "<removed ") fr fr Remove closing tags
    result = stringz.replace_all(result, "</" + tag_name + ">", "")
    
    damn result
}

fr fr HTML validation (making sure it's valid fr)
slay validate_html(html_string tea) lit {
    sus tag_stack normie = 0
    sus position normie = 0
    
    vibez position < stringz.length(html_string) {
        lowkey stringz.char_at(html_string, position) == '<' {
            position = position + 1 fr fr Check for closing tag
            lowkey position < stringz.length(html_string) &&
                   stringz.char_at(html_string, position) == '/' {
                tag_stack = tag_stack - 1
                lowkey tag_stack < 0 {
                    damn cap fr fr Unmatched closing tag
                }
            } cringe { fr fr Opening tag (check if self-closing)
                sus tag_start normie = position
                vibez position < stringz.length(html_string) &&
                       stringz.char_at(html_string, position) != '>' {
                    position = position + 1
                }
                
                sus tag_content tea = stringz.substring(html_string, tag_start, position)
                lowkey !stringz.ends_with(tag_content, "/") {
                    tag_stack = tag_stack + 1
                }
            }
        }
        position = position + 1
    }
    
    damn tag_stack == 0
}

fr fr Create complete HTML document (that's actually fire)
slay create_html_document(title tea, body_content tea) tea {
    sus html tea = "<!DOCTYPE html>\n"
    html = html + "<html lang=\"en\">\n"
    html = html + "<head>\n"
    html = html + "  <meta charset=\"UTF-8\">\n"
    html = html + "  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n"
    html = html + "  <title>" + encode_html_entities(title) + "</title>\n"
    html = html + "</head>\n"
    html = html + "<body>\n"
    html = html + "  " + body_content + "\n"
    html = html + "</body>\n"
    html = html + "</html>"
    damn html
}

fr fr Gen Z Enhanced APIs (these are straight fire no cap)

fr fr Create a div that's absolutely sending (popular element)
slay create_sending_div(content tea, classes tea) HtmlElement {
    sus element HtmlElement = create_element_with_attrs("div", "class=\"" + classes + "\"", content)
    damn element
}

fr fr Create a button that's no cap interactive
slay create_fire_button(text tea, action tea) HtmlElement {
    sus attrs tea = "type=\"button\" onclick=\"" + encode_html_entities(action) + "\""
    sus element HtmlElement = create_element_with_attrs("button", attrs, text)
    damn element
}

fr fr Create an input that's lowkey necessary
slay create_lowkey_input(input_type tea, placeholder tea, name tea) HtmlElement {
    sus attrs tea = "type=\"" + input_type + "\" placeholder=\"" + encode_html_entities(placeholder) + "\" name=\"" + name + "\""
    sus element HtmlElement = create_element_with_attrs("input", attrs, "")
    damn element
}

fr fr Create a link that's actually goated
slay create_goated_link(url tea, text tea, target tea) HtmlElement {
    sus safe_url tea = encode_html_entities(url)
    sus attrs tea = "href=\"" + safe_url + "\" target=\"" + target + "\""
    sus element HtmlElement = create_element_with_attrs("a", attrs, text)
    damn element
}

fr fr Create an image that's absolutely iconic
slay create_iconic_image(src tea, alt tea, classes tea) HtmlElement {
    sus safe_src tea = encode_html_entities(src)
    sus safe_alt tea = encode_html_entities(alt)
    sus attrs tea = "src=\"" + safe_src + "\" alt=\"" + safe_alt + "\" class=\"" + classes + "\""
    sus element HtmlElement = create_element_with_attrs("img", attrs, "")
    damn element
}

fr fr Create a form that's actually valid
slay create_valid_form(action tea, method tea, content tea) HtmlElement {
    sus safe_action tea = encode_html_entities(action)
    sus attrs tea = "action=\"" + safe_action + "\" method=\"" + method + "\""
    sus element HtmlElement = create_element_with_attrs("form", attrs, content)
    damn element
}

fr fr Create table that's organized fr
slay create_organized_table(headers tea, rows tea, classes tea) HtmlElement {
    sus table_content tea = "<thead><tr>" fr fr Add headers
    table_content = table_content + "<th>" + headers + "</th>"
    table_content = table_content + "</tr></thead><tbody>" fr fr Add rows
    table_content = table_content + "<tr><td>" + rows + "</td></tr>"
    table_content = table_content + "</tbody>"
    
    sus attrs tea = "class=\"" + classes + "\""
    sus element HtmlElement = create_element_with_attrs("table", attrs, table_content)
    damn element
}

fr fr Minify HTML (performance is key)
slay minify_html(html_string tea) tea {
    sus minified tea = html_string fr fr Remove extra whitespace
    minified = stringz.replace_all(minified, "\n", "")
    minified = stringz.replace_all(minified, "\r", "")
    minified = stringz.replace_all(minified, "\t", "") fr fr Remove space between tags
    minified = stringz.replace_all(minified, "> <", "><") fr fr Remove multiple spaces
    vibez stringz.contains(minified, "  ") {
        minified = stringz.replace_all(minified, "  ", " ")
    }
    
    damn minified
}

fr fr Extract text content from HTML
slay extract_text_content(html_string tea) tea {
    sus text tea = ""
    sus in_tag lit = cap
    sus position normie = 0
    
    vibez position < stringz.length(html_string) {
        sus ch sip = stringz.char_at(html_string, position)
        
        lowkey ch == '<' {
            in_tag = based
        } cringe lowkey ch == '>' {
            in_tag = cap
        } cringe lowkey !in_tag {
            text = text + stringz.char_to_string(ch)
        }
        
        position = position + 1
    }
    
    damn decode_html_entities(text)
}

fr fr Count HTML elements
slay count_elements(html_string tea) normie {
    sus count normie = 0
    sus position normie = 0
    
    vibez position < stringz.length(html_string) {
        lowkey stringz.char_at(html_string, position) == '<' &&
               position + 1 < stringz.length(html_string) &&
               stringz.char_at(html_string, position + 1) != '/' {
            count = count + 1
        }
        position = position + 1
    }
    
    damn count
}
