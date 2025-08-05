use swissarmyhammer_cli::ui::config::{ColorOutputMode, UiPreferences};
use swissarmyhammer_cli::ui::{
    icon, utils::*, Color, Icon, Style, StyledText, Theme, UiConfig, UiContext,
};

#[test]
fn test_color_creation() {
    let color = Color::new(255, 128, 0);
    assert_eq!(color.r, 255);
    assert_eq!(color.g, 128);
    assert_eq!(color.b, 0);
}

#[test]
fn test_color_to_hex() {
    let color = Color::new(255, 128, 0);
    assert_eq!(color.to_hex(), "#ff8000");

    let black = Color::new(0, 0, 0);
    assert_eq!(black.to_hex(), "#000000");

    let white = Color::new(255, 255, 255);
    assert_eq!(white.to_hex(), "#ffffff");
}

#[test]
fn test_color_to_ansi_256() {
    let red = Color::new(255, 0, 0);
    assert_eq!(red.to_ansi_256(), 196);

    let gray = Color::new(128, 128, 128);
    let gray_ansi = gray.to_ansi_256();
    assert!(gray_ansi >= 232);
}

#[test]
fn test_color_to_ansi_16() {
    let red = Color::new(255, 0, 0);
    assert_eq!(red.to_ansi_16(), 9); // Bright red

    let dark_red = Color::new(128, 0, 0);
    assert_eq!(dark_red.to_ansi_16(), 1); // Red

    let white = Color::new(255, 255, 255);
    assert_eq!(white.to_ansi_16(), 7); // White

    let black = Color::new(0, 0, 0);
    assert_eq!(black.to_ansi_16(), 0); // Black
}

#[test]
fn test_theme_light() {
    let theme = Theme::light();
    assert_eq!(theme.name, "Light");
    assert!(!theme.is_dark);
    assert_eq!(theme.colors.background, Color::new(255, 255, 255));
    assert_eq!(theme.colors.foreground, Color::new(33, 33, 33));
}

#[test]
fn test_theme_dark() {
    let theme = Theme::dark();
    assert_eq!(theme.name, "Dark");
    assert!(theme.is_dark);
    assert_eq!(theme.colors.background, Color::new(18, 18, 18));
    assert_eq!(theme.colors.foreground, Color::new(238, 238, 238));
}

#[test]
fn test_styled_text_creation() {
    let text = StyledText::plain("Hello, World!");
    assert!(text.render().to_string().contains("Hello, World!"));
}

#[test]
fn test_styled_text_decorations() {
    let text = StyledText::plain("Test").bold().underline().dimmed();

    let rendered = text.render();
    let rendered_str = format!("{rendered}");
    assert!(rendered_str.contains("Test"));
}

#[test]
fn test_style_semantic_colors() {
    let theme = Theme::dark();
    let style = Style::new(std::sync::Arc::new(theme));

    let primary = style.primary("Primary");
    let error = style.error("Error");
    let success = style.success("Success");
    let warning = style.warning("Warning");
    let info = style.info("Info");

    assert!(primary.render().to_string().contains("Primary"));
    assert!(error.render().to_string().contains("Error"));
    assert!(success.render().to_string().contains("Success"));
    assert!(warning.render().to_string().contains("Warning"));
    assert!(info.render().to_string().contains("Info"));
}

#[test]
fn test_ui_context_creation() {
    let context = UiContext::default();
    assert!(context.theme().name == "Light" || context.theme().name == "Dark");
}

#[test]
fn test_ui_context_with_theme() {
    let theme = Theme::light();
    let context = UiContext::with_theme(theme);
    assert_eq!(context.theme().name, "Light");
}

#[test]
fn test_ui_context_helpers() {
    let context = UiContext::default();

    let primary = context.primary("Test");
    let error = context.error("Error");
    let success = context.success("Success");
    let header = context.header("Header");

    assert!(primary.render().to_string().contains("Test"));
    assert!(error.render().to_string().contains("Error"));
    assert!(success.render().to_string().contains("Success"));
    assert!(header.render().to_string().contains("Header"));
}

#[test]
fn test_icon_emoji_mode() {
    assert_eq!(icon::get(Icon::Success, true), "✓");
    assert_eq!(icon::get(Icon::Error, true), "✗");
    assert_eq!(icon::get(Icon::Warning, true), "⚠");
    assert_eq!(icon::get(Icon::Info, true), "ℹ");
    assert_eq!(icon::get(Icon::Search, true), "🔍");
    assert_eq!(icon::get(Icon::Rocket, true), "🚀");
}

#[test]
fn test_icon_ascii_mode() {
    assert_eq!(icon::get(Icon::Success, false), "[OK]");
    assert_eq!(icon::get(Icon::Error, false), "[X]");
    assert_eq!(icon::get(Icon::Warning, false), "[!]");
    assert_eq!(icon::get(Icon::Info, false), "[i]");
    assert_eq!(icon::get(Icon::Search, false), "[S]");
    assert_eq!(icon::get(Icon::Rocket, false), "[^]");
}

#[test]
fn test_ui_config_default() {
    let config = UiConfig::default();
    assert_eq!(config.preferences.theme, "dark");
    assert!(config.preferences.use_emojis);
    assert_eq!(config.preferences.color_output, ColorOutputMode::Auto);
    assert!(config.custom_themes.is_empty());
}

#[test]
fn test_text_utilities() {
    let truncated = truncate_with_ellipsis("This is a very long text", 10);
    assert_eq!(truncated, "This is...");

    let short = truncate_with_ellipsis("Short", 10);
    assert_eq!(short, "Short");

    let centered = center_text("Hi", 10);
    assert_eq!(centered, "    Hi    ");

    let wrapped = wrap_text("This is a long text that needs wrapping", 10);
    assert_eq!(wrapped.len(), 4);
    assert_eq!(wrapped[0], "This is a");
    assert_eq!(wrapped[1], "long text");
    assert_eq!(wrapped[2], "that needs");
    assert_eq!(wrapped[3], "wrapping");
}

#[test]
fn test_color_output_mode() {
    let mut config = UiConfig::default();

    config.preferences.color_output = ColorOutputMode::Always;
    assert!(config.should_use_color());

    config.preferences.color_output = ColorOutputMode::Never;
    assert!(!config.should_use_color());
}

#[test]
fn test_styled_text_chaining() {
    let styled = StyledText::plain("Test")
        .fg(Color::new(255, 0, 0))
        .bg(Color::new(0, 0, 255))
        .bold()
        .underline()
        .italic();

    let rendered = styled.render().to_string();
    assert!(rendered.contains("Test"));
}

#[test]
fn test_theme_provider() {
    use swissarmyhammer_cli::ui::theme::{BuiltinThemeProvider, ThemeProvider};

    let provider = BuiltinThemeProvider;

    assert!(provider.get_theme("light").is_some());
    assert!(provider.get_theme("dark").is_some());
    assert!(provider.get_theme("unknown").is_none());

    let themes = provider.list_themes();
    assert!(themes.contains(&"light".to_string()));
    assert!(themes.contains(&"dark".to_string()));
}

#[test]
fn test_ui_preferences_serialization() {
    let prefs = UiPreferences {
        theme: "custom".to_string(),
        use_emojis: false,
        color_output: ColorOutputMode::Never,
    };

    let yaml = serde_yaml::to_string(&prefs).unwrap();
    let deserialized: UiPreferences = serde_yaml::from_str(&yaml).unwrap();

    assert_eq!(deserialized.theme, "custom");
    assert!(!deserialized.use_emojis);
    assert_eq!(deserialized.color_output, ColorOutputMode::Never);
}

#[test]
fn test_theme_serialization() {
    let theme = Theme::light();
    let json = serde_json::to_string(&theme).unwrap();
    let deserialized: Theme = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.name, theme.name);
    assert_eq!(deserialized.is_dark, theme.is_dark);
    assert_eq!(deserialized.colors.primary, theme.colors.primary);
}
