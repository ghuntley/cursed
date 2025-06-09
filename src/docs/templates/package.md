# {{package_name}} Documentation

{{#if description}}
{{description}}
{{/if}}

## Table of Contents

{{#if include_toc}}
{{table_of_contents}}
{{/if}}

## Package Overview

{{#if package_description}}
{{package_description}}
{{/if}}

### Statistics

- **Modules**: {{module_count}}
- **Total Items**: {{total_items}}
{{#each type_counts}}
- **{{@key}}s**: {{this}}
{{/each}}

## Modules

{{#each modules}}
### Module: {{name}}

{{#if description}}
{{description}}
{{/if}}

{{#if include_source_links}}
**Source**: [{{file_path}}]({{../base_url}}/{{file_path}})
{{/if}}

{{#each items_by_type}}
#### {{@key}}s

{{#each this}}
##### {{item_type}} `{{name}}`

{{#if is_deprecated}}
> ⚠️ **Deprecated**: This item is deprecated and may be removed in future versions.
{{/if}}

{{#if description}}
{{description}}
{{/if}}

{{#if signature}}
**Signature:**

```cursed
{{signature}}
```
{{/if}}

{{#if parameters}}
**Parameters:**

{{#each parameters}}
- `{{name}}` ({{param_type}}): {{description}}
{{/each}}
{{/if}}

{{#if return_type}}
**Returns:** `{{return_type}}`{{#if return_description}} - {{return_description}}{{/if}}
{{/if}}

{{#if fields}}
**Fields:**

| Field | Type | Visibility | Description |
|-------|------|------------|-------------|
{{#each fields}}
| `{{name}}` | `{{field_type}}` | {{visibility}} | {{description}} |
{{/each}}
{{/if}}

{{#if examples}}
**Examples:**

{{#each examples}}
```cursed
{{this}}
```
{{/each}}
{{/if}}

{{#if include_source_links}}
*Source: Line {{line}} in [source]({{../../base_url}}#L{{line}})*
{{/if}}

---

{{/each}}
{{/each}}
{{/each}}

## Index

| Item | Type | Module | Description |
|------|------|--------|-------------|
{{#each all_items}}
| [`{{name}}`](#{{anchor}}) | {{item_type}} | {{module_name}} | {{short_description}} |
{{/each}}
