// Simple test for htmlrizzler module
vibez.spill("Testing htmlrizzler module...")

// Test basic HTML element creation
vibez.spill("Creating HTML element...")
sus element HTMLElement
element.tag = "div"
element.content = "Hello World"
vibez.spill("Element tag: " + element.tag)
vibez.spill("Element content: " + element.content)

// Test parsing simple HTML
vibez.spill("Parsing HTML...")
sus html tea = "<p>Test paragraph</p>"
sus parsed_element HTMLElement = parse_html(html)
vibez.spill("Parsed tag: " + parsed_element.tag)

// Test utility functions
vibez.spill("Testing utility functions...")
sus test_str tea = "Hello World"
sus pos normie = find_char(test_str, 'W')
vibez.spill("Found 'W' at position: " + tea(pos))

sus trimmed tea = trim_string("  hello  ")
vibez.spill("Trimmed string: '" + trimmed + "'")

sus contains lit = contains_string("Hello World", "World")
vibez.spill("Contains 'World': " + tea(contains))

vibez.spill("htmlrizzler module test completed!")
