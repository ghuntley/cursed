# CURSED Documentation Guidelines

## Overview

This directory contains the documentation for the CURSED programming language. All documentation follows the GenZ-inspired design guidelines specified in `web/design-guidelines.md`.

## Documentation Structure

- `index.html` - Main documentation landing page with links to all sections
- Topical pages organized by language feature and concepts
- Template-based design with consistent navigation and styling

## Creating New Documentation

1. Use `template.html` as a starting point for new documentation pages
2. Replace placeholder text (marked with `[PLACEHOLDER]`) with actual content
3. Use the GenZ-inspired language style (fun, conversational, using slang terms)
4. Ensure proper cross-linking with related documentation pages

## Styling Guidelines

All documentation pages should:

- Use `../styles/docs.css` for styling
- Include the standard header and navigation
- Follow the section structure in the template
- Maintain consistent GenZ theming and language

## File Naming Conventions

- Use kebab-case for filenames (e.g., `string-switch.html`, `garbage-collection.html`)
- Make names descriptive and concise
- Group related concepts with similar prefixes when appropriate

## Adding Documentation to Navigation

When adding new documentation, make sure to update:

1. The main documentation navigation in `index.html`
2. The sidebar navigation in relevant related pages
3. The "next steps" section of related pages to link to your new page

## Example Elements

Use these HTML elements for consistent documentation structure:

- Feature boxes: `<div class="feature-box">...</div>`
- Code blocks: `<div class="code-block"><div class="code-title">filename.csd</div><pre><code>...</code></pre></div>`
- Tags: `<div class="tag-container"><span class="tag">Tag Name</span></div>`
- Emoji lists: `<ul class="vibe-list"><li><span class="emoji">🔥</span> <strong>Item:</strong> Description</li></ul>`

## Updating Existing Documentation

When updating existing documentation:

1. Ensure it follows the current style template
2. Update stylesheet references to `styles/docs.css`
3. Add standard header and navigation if missing
4. Add proper sidebar navigation if missing