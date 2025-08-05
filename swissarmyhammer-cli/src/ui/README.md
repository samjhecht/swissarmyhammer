# SwissArmyHammer UI Module

This module provides a comprehensive theming and styling system for the SwissArmyHammer CLI, offering consistent visual presentation across all commands and output.

## Features

- **Theme System**: Built-in light and dark themes with automatic terminal detection
- **Semantic Colors**: Consistent color palette for different message types (success, error, warning, info)
- **Text Styling**: Fluent API for applying colors and decorations (bold, italic, underline, etc.)
- **Icon Support**: Semantic icons with automatic fallback to ASCII for non-Unicode terminals
- **User Preferences**: Persistent configuration for theme selection and emoji usage
- **Text Utilities**: Helper functions for text formatting (truncation, centering, wrapping)

## Usage

### Basic Usage

```rust
use swissarmyhammer_cli::ui::{UiContext, Icon};

// Create UI context (automatically detects terminal theme)
let ui = UiContext::default();

// Use semantic colors
println!("{}", ui.success("Operation completed!"));
println!("{}", ui.error("Something went wrong"));
println!("{}", ui.warning("Please check your configuration"));
println!("{}", ui.info("Processing files..."));

// Use icons
println!("{} {}", ui.icon(Icon::Success), ui.success("All tests passed"));
println!("{} {}", ui.icon(Icon::Search), ui.muted("Searching..."));
```

### Custom Themes

```rust
use swissarmyhammer_cli::ui::{Theme, UiContext};

// Use a specific theme
let ui = UiContext::with_theme(Theme::light());
println!("{}", ui.primary("Light theme text"));

// Create custom theme
let mut config = UiConfig::load()?;
config.custom_themes.push(custom_theme);
config.save()?;
```

### Configuration

User preferences are stored in `~/.swissarmyhammer/ui.yaml`:

```yaml
preferences:
  theme: dark
  use_emojis: true
  color_output: auto
```

Environment variables:
- `SAH_THEME`: Override theme selection
- `SAH_USE_EMOJIS`: Enable/disable emoji usage
- `NO_COLOR`: Disable all color output
- `FORCE_COLOR`: Force color output even in non-TTY

### Migration from `colored`

Replace direct `colored` usage with semantic UI methods:

```rust
// Old way
println!("{}", "Error!".red());
println!("{}", "Success!".green());

// New way
let ui = UiContext::default();
println!("{}", ui.error("Error!"));
println!("{}", ui.success("Success!"));
```

## Architecture

- `theme.rs`: Color definitions and theme management
- `style.rs`: Text styling and rendering
- `config.rs`: User preferences and persistence
- `utils.rs`: Terminal detection and text utilities
- `mod.rs`: Public API and UiContext

## Testing

Run the test suite:
```bash
cargo test --package swissarmyhammer-cli --test ui_tests
```

Run the demo:
```bash
cargo run --example ui_demo
```