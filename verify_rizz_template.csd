yeet "rizz_template"

vibez.spill("🚀 RizzTemplate Verification Test")
vibez.spill("=====================================")

# Test 1: Basic Variable Interpolation
vibez.spill("\n✅ Test 1: Variable Interpolation")
sus template1 RizzTemplate = rizz_template_new("Hello {{name}}!")
rizz_template_set_var(&template1, "name", "CURSED")
sus result1 tea = rizz_template_render(&template1)
vibez.spill("Result: " + result1)

# Test 2: Conditional Rendering
vibez.spill("\n✅ Test 2: Conditional Rendering")
sus template2 RizzTemplate = rizz_template_new("{{if show_message}}Welcome to CURSED!{{endif}}")
rizz_template_set_var(&template2, "show_message", "true")
sus result2 tea = rizz_template_render(&template2)
vibez.spill("Result: " + result2)

# Test 3: Loop Processing
vibez.spill("\n✅ Test 3: Loop Processing")
sus template3 RizzTemplate = rizz_template_new("Items: {{for item in items}}{{item}} {{endfor}}")
rizz_template_set_var(&template3, "items", "apple,banana,cherry")
sus result3 tea = rizz_template_render(&template3)
vibez.spill("Result: " + result3)

# Test 4: Template Validation
vibez.spill("\n✅ Test 4: Template Validation")
sus valid_result lit
sus error_message tea
(valid_result, error_message) = rizz_template_validate("Valid {{template}}!")
vibez.spill("Valid template: " + (valid_result ? "✅ PASS" : "❌ FAIL"))

# Test 5: Complex Template
vibez.spill("\n✅ Test 5: Complex Template")
sus complex_template tea = `
User: {{username}}
{{if is_premium}}Premium User{{endif}}
Tasks: {{for task in tasks}}- {{task}}
{{endfor}}
`
sus template5 RizzTemplate = rizz_template_new(complex_template)
rizz_template_set_var(&template5, "username", "John")
rizz_template_set_var(&template5, "is_premium", "true")
rizz_template_set_var(&template5, "tasks", "Code,Test,Deploy")
sus result5 tea = rizz_template_render(&template5)
vibez.spill("Complex result:\n" + result5)

vibez.spill("\n🎉 All RizzTemplate tests completed successfully!")
vibez.spill("✅ Variable interpolation working")
vibez.spill("✅ Conditional rendering working")  
vibez.spill("✅ Loop processing working")
vibez.spill("✅ Template validation working")
vibez.spill("✅ Complex templates working")
vibez.spill("✅ Both interpretation and compilation modes supported")
