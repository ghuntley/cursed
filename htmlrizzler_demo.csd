// htmlrizzler module demonstration
vibez.spill("=== htmlrizzler HTML Parsing Module Demo ===")

// Simple HTML element structure demonstration
vibez.spill("Creating HTML element...")
sus element struct {
    tag tea,
    content tea,
    id tea
}

element.tag = "div"
element.content = "Hello World"
element.id = "main-content"

vibez.spill("Element tag: " + element.tag)
vibez.spill("Element content: " + element.content)
vibez.spill("Element ID: " + element.id)

// Simple HTML parsing demonstration
vibez.spill("\nHTML parsing example:")
sus html_string tea = "<p>This is a paragraph</p>"
vibez.spill("Original HTML: " + html_string)

// Mock parsing result
sus parsed_element struct {
    tag tea,
    content tea
}
parsed_element.tag = "p"
parsed_element.content = "This is a paragraph"

vibez.spill("Parsed tag: " + parsed_element.tag)
vibez.spill("Parsed content: " + parsed_element.content)

// String utility demonstration
vibez.spill("\nString utility functions:")
sus test_string tea = "Hello World"
vibez.spill("Test string: " + test_string)

// Simple character finding
sus found_w lit = cap
bestie i := 0; i < 11; i++ {
    lowkey test_string[i] == "W" {
        found_w = based
        vibez.spill("Found 'W' at position: " + tea(i))
    }
}

lowkey !found_w {
    vibez.spill("'W' not found")
}

// HTML generation demonstration  
vibez.spill("\nHTML generation example:")
sus link_element struct {
    tag tea,
    href tea,
    text tea
}

link_element.tag = "a"
link_element.href = "https://example.com"
link_element.text = "Click here"

sus generated_html tea = "<" + link_element.tag + " href=\"" + link_element.href + "\">" + link_element.text + "</" + link_element.tag + ">"
vibez.spill("Generated HTML: " + generated_html)

vibez.spill("\n=== htmlrizzler demo completed! ===")
