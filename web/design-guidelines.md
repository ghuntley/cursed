# CURSED Website Design Guidelines

## Brand Overview

The CURSED programming language website follows a GenZ-inspired aesthetic, combining modern web design with youth culture elements. The design emphasizes high contrast, vibrant colors, playful interactions, and a mix of minimalist and maximalist design elements.

## Color Palette

### Primary Colors

- Primary Pink: `#ff3366` - Used for primary buttons, important highlights, and branded elements
- Secondary Purple: `#6600ff` - Used for secondary actions and complementary elements
- Accent Blue: `#33ccff` - Used for interactive elements and complementary accents
- Neon Green: `#39ff14` - Used for eye-catching elements and highlights
- Hot Pink: `#ff00ff` - Used for special emphasis

### Neutral Colors

- Dark: `#121212` - Primary background for dark sections
- Light: `#f5f5f5` - Primary background for light sections
- Gray: `#666666` - Used for non-emphasized text

### Gradients

- Main Gradient: `linear-gradient(90deg, var(--color-primary), var(--color-secondary))` - Primary brand gradient
- Vibe Gradient: `linear-gradient(90deg, var(--color-tertiary), var(--color-accent))` - Secondary brand gradient
- Fire Gradient: `linear-gradient(120deg, var(--color-primary), var(--color-pop))` - Accent gradient
- Neon Gradient: `linear-gradient(45deg, var(--color-neon), var(--color-tertiary))` - Special accent gradient
- Y2K Gradient: `linear-gradient(90deg, var(--color-pink), var(--color-accent))` - Nostalgic accent gradient

## Typography

### Primary Fonts

- **Primary Font**: 'Outfit' - A modern, rounded sans-serif font with a friendly feel
- **Fallback Fonts**: 'Inter', system fonts

### Code Font

- **Code Font**: 'JetBrains Mono' - A modern monospace font with excellent readability
- **Fallback Fonts**: 'Fira Code', 'Cascadia Code'

### Font Sizes

- Hero Heading: 3.5rem / 56px
- Section Headings: 2.5rem / 40px
- Subheadings: 1.5rem / 24px
- Body Text: 1rem / 16px
- Small Text: 0.9rem / 14.4px

### Font Weights

- Normal Body: 400
- Medium: 500
- Semi-Bold: 600
- Bold: 700
- Extra Bold: 800

## Design Elements

### Animations & Effects

- **Hover Animations**: Use subtle scale and translate transformations (5-10px) on hover
- **Transitions**: All transitions should be between 0.2s and 0.3s with ease timing
- **Glowing Effects**: Used for important CTA elements with `box-shadow` and `text-shadow`
- **Subtle Background Patterns**: Very light background patterns or gradient overlays
- **Floating Animation**: Subtle up-and-down animations for decorative elements

### Shadows

- **Button Shadows**: `0 4px 14px rgba(0, 0, 0, 0.1)`
- **Card Shadows**: `0 10px 30px rgba(0, 0, 0, 0.05)`
- **Glow Effects**: `0 0 10px rgba(255, 51, 102, 0.5)` (adapt color as needed)

### Borders & Shapes

- **Border Radius** (Small): 8px
- **Border Radius** (Medium): 16px
- **Border Radius** (Large): 24px
- **Button Radius**: 30px (pill-shaped)
- **Border Accents**: 1px borders with low opacity (0.05-0.1) for subtle division

### Layout 

- **Section Spacing**: 4-6rem (64-96px) vertical spacing between major sections
- **Grid System**: Responsive grid with 2-3 columns on desktop, single column on mobile
- **Container Width**: Maximum 1200px for main content container
- **Section Transitions**: Angled section dividers with `transform: skewY(-5deg)`

## UI Components

### Buttons

#### Primary Button
- Background: White or `--color-primary`
- Text Color: `--color-secondary` or White
- Border: None
- Box Shadow: `0 4px 14px rgba(0, 0, 0, 0.1)`
- Hover: Scale and translate up slightly

#### Secondary Button
- Background: Transparent
- Border: 2px solid white or accent color
- Text Color: White or accent color
- Hover: Semi-transparent background

#### Glassmorphism Button
- Background: Semi-transparent (rgba(255, 255, 255, 0.1))
- Backdrop Filter: blur(10px)
- Border: 1px solid rgba(255, 255, 255, 0.2)

### Cards

- Background: White or dark background
- Border Radius: var(--border-radius-md)
- Box Shadow: `0 10px 30px rgba(0, 0, 0, 0.05)`
- Border: 1px solid rgba(0, 0, 0, 0.05) or rgba(255, 255, 255, 0.1)
- Padding: 2rem
- Hover Effect: translateY(-5px) and increased shadow

### Code Blocks

- Background: #1a1a1a
- Title Bar: #ff1744 (vibrant red)
- Border: 1px solid #444
- Syntax Highlighting: Keywords #ff5588, Strings #b7f259, Comments #7a88c4
- Font: JetBrains Mono

### Tags & Badges

- Background: Semi-transparent or gradient
- Border Radius: 30px (pill-shaped)
- Padding: Small (0.3rem 0.8rem)
- Animation: Subtle floating animation
- Text: Bold, sometimes with text-shadow

## Response Design

### Breakpoints

- Mobile: < 480px
- Tablet: 481px - 768px
- Desktop Small: 769px - 992px
- Desktop Large: > 992px

### Mobile Adaptations

- Stack columns vertically
- Reduce font sizes by 20-30%
- Ensure touch targets are at least 44px × 44px
- Simplify animations and effects
- Adjust padding and margins for smaller screens

## Accessibility

- Maintain color contrast ratio of at least 4.5:1 for normal text
- Ensure all interactive elements have focus states
- Provide alternatives to animation/motion when possible
- Support keyboard navigation
- Use semantic HTML structures

## Design Principles

1. **Bold & Vibrant**: Use high-contrast colors and bold typography
2. **Playful & Energetic**: Incorporate animations and interactive elements
3. **Authentic Voice**: Use GenZ slang and expressions judiciously
4. **Balance**: Mix minimalist layouts with maximalist details
5. **Performance**: Ensure animations and effects don't harm page performance

## Implementation Notes

- Use CSS variables for all colors and spacing
- Favor flexbox and grid for layouts
- Use reasonable animation performance techniques
- Always test on mobile and low-powered devices

---

This design system aims to create a cohesive, recognizable brand identity for the CURSED programming language that appeals to a young, tech-savvy audience while maintaining usability and performance.