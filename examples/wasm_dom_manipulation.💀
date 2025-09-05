// CURSED WebAssembly DOM Manipulation Example
// Demonstrates browser DOM interaction through WASM

yeet "vibez"
yeet "domz"  // Browser DOM integration module

// Interactive web application entry point
slay main_character() {
    vibez.spill("🌐 CURSED DOM Manipulation Demo Starting...")
    
    // Create dynamic page content
    setup_page()
    
    // Setup event handlers
    setup_interactive_elements()
    
    vibez.spill("✅ CURSED DOM app initialized!")
    damn 0
}

// Setup basic page structure
slay setup_page() {
    // Create main container
    sus container domz.Element = domz.create_element("div")
    domz.set_id(container, "cursed-app")
    domz.set_style(container, "padding", "20px")
    domz.set_style(container, "max-width", "600px")
    domz.set_style(container, "margin", "20px auto")
    domz.set_style(container, "background", "linear-gradient(45deg, #667eea, #764ba2)")
    domz.set_style(container, "border-radius", "15px")
    domz.set_style(container, "color", "white")
    
    // Create title
    sus title domz.Element = domz.create_element("h1")
    domz.set_text(title, "🚀 CURSED WASM Interactive Demo")
    domz.set_style(title, "text-align", "center")
    domz.set_style(title, "margin-bottom", "30px")
    
    // Create input section
    sus input_section domz.Element = create_input_section()
    
    // Create output area
    sus output domz.Element = domz.create_element("div")
    domz.set_id(output, "output")
    domz.set_style(output, "background", "rgba(0,0,0,0.3)")
    domz.set_style(output, "padding", "20px")
    domz.set_style(output, "border-radius", "8px")
    domz.set_style(output, "margin-top", "20px")
    domz.set_style(output, "min-height", "200px")
    domz.set_style(output, "font-family", "monospace")
    
    // Assemble the page
    domz.append_child(container, title)
    domz.append_child(container, input_section)
    domz.append_child(container, output)
    domz.append_child(domz.get_body(), container)
}

// Create interactive input section
slay create_input_section() domz.Element {
    sus section domz.Element = domz.create_element("div")
    domz.set_style(section, "background", "rgba(255,255,255,0.1)")
    domz.set_style(section, "padding", "20px")
    domz.set_style(section, "border-radius", "10px")
    domz.set_style(section, "margin-bottom", "20px")
    
    // Number input for calculations
    sus label1 domz.Element = domz.create_element("label")
    domz.set_text(label1, "Enter a number for CURSED calculations:")
    domz.set_style(label1, "display", "block")
    domz.set_style(label1, "margin-bottom", "10px")
    
    sus number_input domz.Element = domz.create_element("input")
    domz.set_attribute(number_input, "type", "number")
    domz.set_attribute(number_input, "id", "number-input")
    domz.set_attribute(number_input, "placeholder", "42")
    domz.set_style(number_input, "width", "100%")
    domz.set_style(number_input, "padding", "10px")
    domz.set_style(number_input, "margin-bottom", "15px")
    domz.set_style(number_input, "border-radius", "5px")
    domz.set_style(number_input, "border", "none")
    
    // Text input for string manipulation
    sus label2 domz.Element = domz.create_element("label")
    domz.set_text(label2, "Enter text to make it more cursed:")
    domz.set_style(label2, "display", "block")
    domz.set_style(label2, "margin-bottom", "10px")
    
    sus text_input domz.Element = domz.create_element("input")
    domz.set_attribute(text_input, "type", "text")
    domz.set_attribute(text_input, "id", "text-input")
    domz.set_attribute(text_input, "placeholder", "Hello World")
    domz.set_style(text_input, "width", "100%")
    domz.set_style(text_input, "padding", "10px")
    domz.set_style(text_input, "margin-bottom", "15px")
    domz.set_style(text_input, "border-radius", "5px")
    domz.set_style(text_input, "border", "none")
    
    // Buttons
    sus button_container domz.Element = domz.create_element("div")
    domz.set_style(button_container, "text-align", "center")
    
    sus calc_btn domz.Element = create_button("Calculate Magic", "calc-btn")
    sus text_btn domz.Element = create_button("Cursify Text", "text-btn")
    sus clear_btn domz.Element = create_button("Clear Output", "clear-btn")
    
    // Assemble input section
    domz.append_child(section, label1)
    domz.append_child(section, number_input)
    domz.append_child(section, label2)
    domz.append_child(section, text_input)
    
    domz.append_child(button_container, calc_btn)
    domz.append_child(button_container, text_btn)
    domz.append_child(button_container, clear_btn)
    
    domz.append_child(section, button_container)
    
    damn section
}

// Create styled button helper
slay create_button(text tea, id tea) domz.Element {
    sus button domz.Element = domz.create_element("button")
    domz.set_text(button, text)
    domz.set_id(button, id)
    domz.set_style(button, "background", "linear-gradient(45deg, #ff6b6b, #ee5a24)")
    domz.set_style(button, "color", "white")
    domz.set_style(button, "border", "none")
    domz.set_style(button, "padding", "10px 20px")
    domz.set_style(button, "margin", "0 10px")
    domz.set_style(button, "border-radius", "20px")
    domz.set_style(button, "cursor", "pointer")
    domz.set_style(button, "font-size", "14px")
    domz.set_style(button, "transition", "transform 0.2s")
    
    damn button
}

// Setup event handlers for interactive elements
slay setup_interactive_elements() {
    // Calculate button handler
    domz.add_event_listener("calc-btn", "click", calculate_magic)
    
    // Text transformation button handler  
    domz.add_event_listener("text-btn", "click", cursify_text)
    
    // Clear button handler
    domz.add_event_listener("clear-btn", "click", clear_output)
    
    // Enter key handlers for inputs
    domz.add_event_listener("number-input", "keypress", handle_enter_calculate)
    domz.add_event_listener("text-input", "keypress", handle_enter_text)
}

// Calculate mathematical operations
slay calculate_magic() {
    sus input_element domz.Element = domz.get_element_by_id("number-input")
    sus value_str tea = domz.get_value(input_element)
    sus number drip = parse_int(value_str)
    
    ready (number == 0) {
        append_output("❌ Please enter a valid number!")
        damn
    }
    
    append_output("🔢 CURSED Mathematical Magic:")
    append_output("Input number: " + value_str)
    append_output("Squared: " + int_to_string(number * number))
    append_output("Factorial: " + int_to_string(factorial(number)))
    append_output("Fibonacci: " + int_to_string(fibonacci(number)))
    append_output("Is Prime: " + (is_prime(number) ? "Yes" : "No"))
    append_output("Binary: " + to_binary(number))
    append_output("Hex: " + to_hex(number))
    append_output("---")
}

// Transform text with CURSED operations
slay cursify_text() {
    sus input_element domz.Element = domz.get_element_by_id("text-input")
    sus text tea = domz.get_value(input_element)
    
    ready (text == "") {
        append_output("❌ Please enter some text!")
        damn
    }
    
    append_output("✨ CURSED Text Transformations:")
    append_output("Original: " + text)
    append_output("Reversed: " + reverse_string(text))
    append_output("Uppercase: " + to_uppercase(text))
    append_output("Lowercase: " + to_lowercase(text))
    append_output("Length: " + int_to_string(string_length(text)))
    append_output("Vowel count: " + int_to_string(count_vowels(text)))
    append_output("Cursed style: " + make_cursed(text))
    append_output("---")
}

// Clear output area
slay clear_output() {
    sus output domz.Element = domz.get_element_by_id("output")
    domz.set_text(output, "")
    append_output("🧹 Output cleared - ready for more CURSED magic!")
}

// Handle Enter key for number input
slay handle_enter_calculate(event domz.Event) {
    ready (domz.get_key_code(event) == 13) { // Enter key
        calculate_magic()
    }
}

// Handle Enter key for text input
slay handle_enter_text(event domz.Event) {
    ready (domz.get_key_code(event) == 13) { // Enter key
        cursify_text()
    }
}

// Helper function to append to output
slay append_output(message tea) {
    sus output domz.Element = domz.get_element_by_id("output")
    sus current tea = domz.get_text(output)
    domz.set_text(output, current + message + "\n")
}

// Mathematical helper functions
slay factorial(n drip) drip {
    ready (n <= 1) damn 1
    damn n * factorial(n - 1)
}

slay fibonacci(n drip) drip {
    ready (n <= 1) damn n
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

slay is_prime(n drip) lit {
    ready (n < 2) damn cringe
    sus i drip = 2
    bestie (i * i <= n) {
        ready (n % i == 0) damn cringe
        i = i + 1
    }
    damn based
}

// String helper functions
slay reverse_string(str tea) tea {
    // Simplified implementation
    damn "gnirts desrever" // Placeholder
}

slay count_vowels(str tea) drip {
    // Simplified implementation
    damn 3 // Placeholder
}

slay make_cursed(str tea) tea {
    // Add cursed styling
    damn "🔥" + str + "🔥"
}

// Utility functions (simplified implementations)
slay parse_int(str tea) drip { damn 42 }
slay int_to_string(n drip) tea { damn "42" }
slay to_binary(n drip) tea { damn "101010" }
slay to_hex(n drip) tea { damn "2A" }
slay to_uppercase(str tea) tea { damn "UPPERCASE" }
slay to_lowercase(str tea) tea { damn "lowercase" }
slay string_length(str tea) drip { damn 5 }

// Export main for WASM entry point
export slay main_character() drip {
    main()
    damn 0
}
