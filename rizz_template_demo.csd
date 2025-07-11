yeet "rizz_template"

vibez.spill("🎨 RizzTemplate Demo - CURSED Template Engine")
vibez.spill("=" * 50)

# Demo 1: Basic Variable Interpolation
vibez.spill("\n📝 Demo 1: Basic Variable Interpolation")
sus basic_template RizzTemplate = rizz_template_new("Hello {{name}}, welcome to {{place}}!")
rizz_template_set_var(&basic_template, "name", "Developer")
rizz_template_set_var(&basic_template, "place", "CURSED")
sus basic_result tea = rizz_template_render(&basic_template)
vibez.spill("Template: Hello {{name}}, welcome to {{place}}!")
vibez.spill("Result: " + basic_result)

# Demo 2: Conditional Rendering
vibez.spill("\n🔀 Demo 2: Conditional Rendering")
sus cond_template RizzTemplate = rizz_template_new("{{if is_premium}}🌟 Premium User: {{name}}{{endif}}")
rizz_template_set_var(&cond_template, "is_premium", "true")
rizz_template_set_var(&cond_template, "name", "John")
sus cond_result tea = rizz_template_render(&cond_template)
vibez.spill("Template: {{if is_premium}}🌟 Premium User: {{name}}{{endif}}")
vibez.spill("Result: " + cond_result)

# Demo 3: Loop Processing
vibez.spill("\n🔄 Demo 3: Loop Processing")
sus loop_template RizzTemplate = rizz_template_new("Shopping List:\n{{for item in items}}• {{item}}\n{{endfor}}")
rizz_template_set_var(&loop_template, "items", "Apples,Bananas,Oranges,Milk")
sus loop_result tea = rizz_template_render(&loop_template)
vibez.spill("Template: Shopping List:\\n{{for item in items}}• {{item}}\\n{{endfor}}")
vibez.spill("Result:\n" + loop_result)

# Demo 4: Complex Template with All Features
vibez.spill("\n🎭 Demo 4: Complex Template")
sus complex_content tea = `
User Profile: {{username}}
{{if is_active}}
Status: ✅ Active
{{if subscription == "premium"}}
Plan: 🌟 Premium
{{endif}}
{{endif}}

Recent Activities:
{{for activity in activities}}
- {{activity}}
{{endfor}}

{{if show_footer}}
Thanks for using CURSED!
{{endif}}
`

sus complex_template RizzTemplate = rizz_template_new(complex_content)
rizz_template_set_var(&complex_template, "username", "Alice")
rizz_template_set_var(&complex_template, "is_active", "true")
rizz_template_set_var(&complex_template, "subscription", "premium")
rizz_template_set_var(&complex_template, "activities", "Logged in,Updated profile,Viewed dashboard")
rizz_template_set_var(&complex_template, "show_footer", "true")

sus complex_result tea = rizz_template_render(&complex_template)
vibez.spill("Complex template result:")
vibez.spill(complex_result)

# Demo 5: Layout System
vibez.spill("\n🏗️ Demo 5: Layout System")
sus page_template RizzTemplate = rizz_template_new("Welcome to {{page_title}}! This is the main content about {{topic}}.")
rizz_template_set_var(&page_template, "page_title", "My Website")
rizz_template_set_var(&page_template, "topic", "CURSED Templates")

sus layout_content tea = `
<!DOCTYPE html>
<html>
<head>
    <title>{{page_title}}</title>
    <meta charset="UTF-8">
</head>
<body>
    <header>
        <h1>{{page_title}}</h1>
        <nav>Navigation here</nav>
    </header>
    <main>
        {{content}}
    </main>
    <footer>
        <p>© 2025 CURSED Templates</p>
    </footer>
</body>
</html>
`

sus layout_result tea = rizz_template_render_with_layout(&page_template, layout_content)
vibez.spill("Layout rendering result:")
vibez.spill(layout_result)

# Demo 6: Template Validation
vibez.spill("\n✅ Demo 6: Template Validation")
sus valid_template tea = "Hello {{name}}!"
sus invalid_template tea = "Hello {{name}!"  # Missing closing brace

sus is_valid lit
sus error_msg tea
(is_valid, error_msg) = rizz_template_validate(valid_template)
vibez.spill("Valid template check: " + (is_valid ? "✅ Valid" : "❌ Invalid"))

(is_valid, error_msg) = rizz_template_validate(invalid_template)
vibez.spill("Invalid template check: " + (is_valid ? "✅ Valid" : "❌ Invalid - " + error_msg))

# Demo 7: Performance Test
vibez.spill("\n⚡ Demo 7: Performance Test")
sus perf_template RizzTemplate = rizz_template_new("{{for num in numbers}}Number {{num}} is {{num == \"5\" ? \"special\" : \"normal\"}}\\n{{endfor}}")
rizz_template_set_var(&perf_template, "numbers", "1,2,3,4,5,6,7,8,9,10")
sus perf_result tea = rizz_template_render(&perf_template)
vibez.spill("Performance test with 10 items:")
vibez.spill(perf_result)

vibez.spill("\n🎉 RizzTemplate Demo Complete!")
vibez.spill("The template engine supports:")
vibez.spill("• Variable interpolation with {{variable}} syntax")
vibez.spill("• Conditional rendering with {{if condition}}...{{endif}}")
vibez.spill("• Loop processing with {{for item in items}}...{{endfor}}")
vibez.spill("• Layout templates with content injection")
vibez.spill("• Template validation and error handling")
vibez.spill("• Include system for template composition")
vibez.spill("• Performance optimization with compilation")
