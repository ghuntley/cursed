# {{package_name}} ✨

{{#if description}}
{{description}}
{{/if}}

{{#if include_quickstart}}
## Quick Start 🚀

```bash
# Build the project
make build

# Run tests
make test

# Format code (keep it aesthetic!)
make fmt

# Run the CURSED compiler
make run ARGS="path/to/your/file.csd"
```
{{/if}}

{{#if include_examples}}
## Examples 💡

{{#each example_modules}}
### {{module_name}}

{{#each items}}
#### {{name}}

{{#if description}}
{{description}}
{{/if}}

{{#each examples}}
```cursed
{{this}}
```
{{/each}}
{{/each}}
{{/each}}
{{/if}}

## API Overview 📚

{{#each modules}}
### {{name}}

{{#if description}}
{{description}}
{{/if}}

| Item | Type | Description |
|------|------|-------------|
{{#each public_items}}
| `{{name}}` | {{item_type}} | {{short_description}} |
{{/each}}
{{/each}}

## Features ✨

{{#each features}}
- **{{name}}**: {{description}}
{{/each}}

## Installation 📦

{{installation_instructions}}

## Contributing 👯‍♀️

{{contributing_guidelines}}

## License 📜

{{license_info}}

---

Made with 💅 by the CURSED community
