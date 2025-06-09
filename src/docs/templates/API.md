# {{package_name}} API Reference

{{#if description}}
{{description}}
{{/if}}

{{#if group_by_type}}
{{#each item_types}}
## {{@key}}s

{{#each this}}
### {{name}} {#{{anchor}}}

{{#if is_deprecated}}
> ⚠️ **Deprecated**: This item is deprecated and may be removed in future versions.
{{/if}}

{{#if visibility}}
**Visibility**: {{visibility}}
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

{{#if generics}}
**Generic Parameters:**

{{#each generics}}
- `{{this}}`
{{/each}}
{{/if}}

{{#if parameters}}
**Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
{{#each parameters}}
| `{{name}}` | `{{param_type}}` | {{description}} |
{{/each}}
{{/if}}

{{#if return_type}}
**Returns:** `{{return_type}}`

{{#if return_description}}
{{return_description}}
{{/if}}
{{/if}}

{{#if fields}}
**Fields:**

| Field | Type | Visibility | Description |
|-------|------|------------|-------------|
{{#each fields}}
| `{{name}}` | `{{field_type}}` | {{visibility}} | {{description}} |
{{/each}}
{{/if}}

{{#if methods}}
**Methods:**

{{#each methods}}
#### {{name}} {#{{../name}}-{{anchor}}}

{{#if description}}
{{description}}
{{/if}}

{{#if signature}}
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

{{#if examples}}
**Example:**

{{#each examples}}
```cursed
{{this}}
```
{{/each}}
{{/if}}

---

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
**Source:** [Line {{line}}]({{base_url}}#L{{line}})
{{/if}}

---

{{/each}}
{{/each}}
{{else}}
{{#each modules}}
## Module: {{name}}

{{#if description}}
{{description}}
{{/if}}

{{#each items}}
### {{name}} {#{{anchor}}}

{{#if is_deprecated}}
> ⚠️ **Deprecated**: This item is deprecated and may be removed in future versions.
{{/if}}

**Type:** {{item_type}}
{{#if visibility}}
**Visibility:** {{visibility}}
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

| Parameter | Type | Description |
|-----------|------|-------------|
{{#each parameters}}
| `{{name}}` | `{{param_type}}` | {{description}} |
{{/each}}
{{/if}}

{{#if return_type}}
**Returns:** `{{return_type}}`

{{#if return_description}}
{{return_description}}
{{/if}}
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
**Source:** [Line {{line}}]({{base_url}}#L{{line}})
{{/if}}

---

{{/each}}
{{/each}}
{{/if}}

## Quick Reference

### Functions

| Function | Module | Description |
|----------|--------|-------------|
{{#each all_functions}}
| [`{{name}}`](#{{anchor}}) | {{module_name}} | {{short_description}} |
{{/each}}

### Types

| Type | Module | Description |
|------|--------|-------------|
{{#each all_types}}
| [`{{name}}`](#{{anchor}}) | {{module_name}} | {{short_description}} |
{{/each}}

### Constants

| Constant | Module | Description |
|----------|--------|-------------|
{{#each all_constants}}
| [`{{name}}`](#{{anchor}}) | {{module_name}} | {{short_description}} |
{{/each}}
