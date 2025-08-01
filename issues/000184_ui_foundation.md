# CLI UX Improvement: UI Foundation Module and Theme System

## Overview

Create the foundational UI module structure and implement a theme system that will support the modern, beautiful CLI experience described in the specification. This establishes the base architecture for all future UI components.

## Goals

1. Create a well-organized UI module structure
2. Implement a flexible theme system with light/dark modes
3. Define semantic color schemes for consistent visual language
4. Establish styling primitives for UI components
5. Create theme configuration and persistence

## Technical Design

### Module Structure

```
swissarmyhammer-cli/src/ui/
├── mod.rs           # Module root with public API
├── theme.rs         # Theme system and color definitions
├── style.rs         # Styling primitives and helpers
├── config.rs        # UI configuration and preferences
└── utils.rs         # Common UI utilities
```

### Implementation Steps

1. Create the `ui` module directory structure
2. Implement the `Color` and `ColorPalette` types with RGB support
3. Create the `Theme` struct with predefined light and dark themes
4. Implement the `Style` API for applying colors and text styles
5. Create the `UiConfig` system for persistence
6. Build the `UiContext` manager for theme selection
7. Add terminal dark mode detection
8. Implement emoji/icon support with fallbacks
9. Add unit tests for theme system
10. Create integration with existing CLI code

## Success Criteria

- [ ] UI module structure created and organized
- [ ] Theme system supports light and dark themes
- [ ] Semantic color system implemented
- [ ] Styling API provides consistent interface
- [ ] Configuration persists user preferences
- [ ] Terminal detection works correctly
- [ ] Emoji support with text fallbacks
- [ ] All tests pass
- [ ] No performance regression
- [ ] Documentation complete