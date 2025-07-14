yeet "htmlrizzler"

vibez.spill("🔥 HTML Rizzler Demo - This module is absolutely fire! 🔥")

# Create a complete webpage
sus page_title = "My Fire Website"

# Create header section
sus header_content = htmlrizzler.generate_html(
    htmlrizzler.create_element("h1", "Welcome to the CURSED Web!")
)

# Create a sending div with premium content
sus hero_section = htmlrizzler.generate_html(
    htmlrizzler.create_sending_div("This website is absolutely sending! No cap! 🔥", "hero premium-content")
)

# Create some interactive elements
sus fire_button = htmlrizzler.generate_html(
    htmlrizzler.create_fire_button("Subscribe Now!", "handleSubscribe()")
)

sus contact_form = htmlrizzler.generate_html(
    htmlrizzler.create_lowkey_input("email", "Enter your email", "email")
) + htmlrizzler.generate_html(
    htmlrizzler.create_fire_button("Join the Community", "submitEmail()")
)

sus contact_section = htmlrizzler.generate_html(
    htmlrizzler.create_valid_form("/signup", "POST", contact_form)
)

# Create a goated link
sus github_link = htmlrizzler.generate_html(
    htmlrizzler.create_goated_link("https://github.com/cursed-lang", "Check out our GitHub", "_blank")
)

# Create an iconic image
sus logo_image = htmlrizzler.generate_html(
    htmlrizzler.create_iconic_image("logo.png", "CURSED Language Logo", "logo responsive")
)

# Combine all content
sus body_content = header_content + hero_section + logo_image + fire_button + contact_section + github_link

# Create the complete HTML document
sus complete_webpage = htmlrizzler.create_html_document(page_title, body_content)

vibez.spill("Generated HTML Document:")
vibez.spill("=" * 50)
vibez.spill(complete_webpage)
vibez.spill("=" * 50)

# Test security features
vibez.spill("\n🛡️ Security Demo:")
sus malicious_input = "<script>alert('This would be dangerous!')</script><p>But this is safe</p>"
sus sanitized_output = htmlrizzler.sanitize_html(malicious_input)
vibez.spill("Original: " + malicious_input)
vibez.spill("Sanitized: " + sanitized_output)

# Test HTML validation
vibez.spill("\n✅ Validation Demo:")
sus valid_html = "<div><p>This is valid HTML</p></div>"
sus invalid_html = "<div><p>This is invalid HTML</div></p>"

lowkey htmlrizzler.validate_html(valid_html) {
    vibez.spill("✅ Valid HTML passed validation")
} cringe {
    vibez.spill("❌ Valid HTML failed validation")
}

lowkey htmlrizzler.validate_html(invalid_html) {
    vibez.spill("❌ Invalid HTML passed validation (this shouldn't happen)")
} cringe {
    vibez.spill("✅ Invalid HTML correctly rejected")
}

# Test CSS selector matching
vibez.spill("\n🎯 CSS Selector Demo:")
sus test_element = htmlrizzler.create_element_with_attrs("div", "class=\"premium fire\" id=\"main-content\"", "Test content")

lowkey htmlrizzler.matches_selector(test_element, "div") {
    vibez.spill("✅ Element matches tag selector 'div'")
}

lowkey htmlrizzler.matches_selector(test_element, ".premium") {
    vibez.spill("✅ Element matches class selector '.premium'")
}

lowkey htmlrizzler.matches_selector(test_element, "#main-content") {
    vibez.spill("✅ Element matches ID selector '#main-content'")
}

# Test HTML utilities
vibez.spill("\n🔧 Utility Functions Demo:")
sus unminified = "  <div>  \n  <p>Hello World</p>  \n  </div>  "
sus minified = htmlrizzler.minify_html(unminified)
vibez.spill("Original size: " + stringz.length(unminified))
vibez.spill("Minified size: " + stringz.length(minified))
vibez.spill("Space saved: " + (stringz.length(unminified) - stringz.length(minified)) + " characters")

sus html_with_content = "<div><h1>Title</h1><p>Some <strong>bold</strong> text here</p></div>"
sus text_only = htmlrizzler.extract_text_content(html_with_content)
vibez.spill("HTML: " + html_with_content)
vibez.spill("Text only: " + text_only)

vibez.spill("\n🎉 HTML Rizzler Demo Complete! This module is lowkey the best HTML library ever! 💯")
