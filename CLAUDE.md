# CLAUDE.md - samutils

## Project Overview

**samutils** is a dioxus based collection of standalone web-based utility tools packaged as a single-page application launcher. Each tool is mostly self-contained, designed with a minimalist aesthetic and no external dependencies if possible. It uses `dx bundle --web --ssg --release` to publish to cloudflare pages, where no backend is required. 

## Architecture

### Design Philosophy

0. **Placement**: User controls are on the bottom half of the screen and displays are on the top half of the screen
1. **Minimalist UI**: Clean, simple interfaces with essential functionality only
2. **Theme Support**: Light/dark mode toggle with localStorage persistence
3. **Responsive**: Mobile-first design with viewport meta tags

## Technical Details

### Common Patterns

**Styling:**
- CSS reset: `* { margin: 0; padding: 0; box-sizing: border-box; }`
- System fonts: `-apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif`
- Centered layouts using flexbox
- 1px solid borders for minimalist aesthetic
- Hover states: typically `#888` background with white text
- Transitions: 0.3s for smooth state changes

**Theme Implementation:**
```javascript
const themeKey = "samutil-theme";
// Uses localStorage for persistence
// Body class toggles between "light" and "dark"
```

**Color Scheme:**
Pallette: #000, #888, #fff only

- Light mode: White background (#fff), black text (#000)
- Dark mode: Black background (#000), white text (#fff)
- Accent/hover: Gray (#888)
- Borders: Match text color (currentColor or explicit)

### Build System

**Justfile commands:**
- `just build`: Creates zip archive in `target/archive.zip`
  - Excludes hidden files, macOS metadata
- `just clean`: Removes target directory

The build creates a deployable archive of all source files, suitable for hosting or distribution.
