yeet "rizz_template"

# Rizz Template Engine Demo
vibez.spill("=== Rizz Template Engine Demo ===")

# Test basic variable substitution
sus template tea = "Hello {{name}}, welcome to {{site}}!"
sus result tea = rizz_template.rizz_parse_template(template, "name", "Chad")
vibez.spill("Template result:")
vibez.spill(result)

# Test HTML escaping
sus dangerous tea = "<script>alert('XSS')</script>"
sus escaped tea = rizz_template.rizz_escape_html(dangerous)
vibez.spill("Escaped HTML:")
vibez.spill(escaped)

# Test Gen Z APIs
sus gen_z_template tea = "This is {{vibe}} fr fr!"
sus bussin_result tea = rizz_template.rizz_template_bussin(gen_z_template, "vibe", "bussin")
vibez.spill("Gen Z API result:")
vibez.spill(bussin_result)

# Test security validation
sus safe_template tea = "Hello {{user}}!"
sus is_safe lit = rizz_template.rizz_validate_template(safe_template)
vibez.spill("Template is safe:")
vibez.spill(is_safe)

sus unsafe_template tea = "<script>alert('hack')</script>"
sus is_unsafe lit = rizz_template.rizz_validate_template(unsafe_template)
vibez.spill("Dangerous template is safe:")
vibez.spill(is_unsafe)

# Test filters
sus filter_result tea = rizz_template.rizz_apply_filter("cursed", "upper")
vibez.spill("Filter result:")
vibez.spill(filter_result)

vibez.spill("=== Demo Complete ===")
