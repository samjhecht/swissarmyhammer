use swissarmyhammer_cli::ui::{Icon, Theme, UiContext};

fn main() {
    // Create UI context with default theme (detects terminal)
    let ui = UiContext::default();

    println!();
    println!("{}", ui.header("SwissArmyHammer UI Demo"));
    println!();

    // Demonstrate semantic colors
    println!(
        "{} {}",
        ui.icon(Icon::Info),
        ui.info("This is an info message")
    );
    println!(
        "{} {}",
        ui.icon(Icon::Success),
        ui.success("Operation completed successfully!")
    );
    println!(
        "{} {}",
        ui.icon(Icon::Warning),
        ui.warning("Warning: Check your configuration")
    );
    println!(
        "{} {}",
        ui.icon(Icon::Error),
        ui.error("Error: Something went wrong")
    );
    println!();

    // Demonstrate primary/secondary colors
    println!("{}", ui.primary("Primary text for main content"));
    println!("{}", ui.secondary("Secondary text for supporting content"));
    println!("{}", ui.muted("Muted text for less important information"));
    println!();

    // Demonstrate with light theme
    let light_ui = UiContext::with_theme(Theme::light());
    println!("{}", light_ui.header("Light Theme Example"));
    println!(
        "{} {}",
        light_ui.icon(Icon::Star),
        light_ui.accent("Featured content")
    );
    println!();

    // Demonstrate with dark theme
    let dark_ui = UiContext::with_theme(Theme::dark());
    println!("{}", dark_ui.header("Dark Theme Example"));
    println!(
        "{} {}",
        dark_ui.icon(Icon::Rocket),
        dark_ui.primary("Ready for launch!")
    );
    println!();

    // Demonstrate text utilities
    use swissarmyhammer_cli::ui::utils::{center_text, truncate_with_ellipsis, wrap_text};

    println!("{}", ui.header("Text Utilities"));
    println!();

    let long_text = "This is a very long text that needs to be truncated";
    println!(
        "Truncated: {}",
        ui.muted(truncate_with_ellipsis(long_text, 30))
    );

    let centered = center_text("Centered Text", 50);
    println!("Centered: {}", ui.primary(&centered));

    let wrapped = wrap_text(
        "This is a long text that needs to be wrapped across multiple lines for better readability",
        40,
    );
    println!("\nWrapped text:");
    for line in wrapped {
        println!("  {}", ui.muted(&line));
    }
}
